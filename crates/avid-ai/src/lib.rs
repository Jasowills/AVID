//! `avid-ai`: model-agnostic AI runtime (ADR-005, ADR-007).
//!
//! ```text
//! AI Runtime → Capability Router → Provider Registry → Adapter → Model
//! ```
//!
//! Provider-specific wire logic lives ONLY in adapters. The canonical
//! request shape is OpenAI-compatible, so one code path serves Ollama,
//! LM Studio, generic compat endpoints, and OpenAI cloud. Capabilities are
//! **probed, never assumed** — a model that fails the structured-output
//! probe is reported honestly instead of producing corrupt edit plans.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

/// AI runtime failures.
#[derive(Debug, Error)]
pub enum AiError {
    /// HTTP transport failure (connection refused, timeout, …).
    #[error("provider transport failed: {0}")]
    Transport(String),
    /// Provider returned non-2xx or an unexpected envelope.
    #[error("provider error (status {status}): {body}")]
    Provider {
        /// HTTP status code.
        status: String,
        /// Truncated response body.
        body: String,
    },
    /// Response content was not valid JSON.
    #[error("provider returned invalid JSON: {0}")]
    InvalidJson(String),
    /// Response JSON missed the required shape (no `operations`, …).
    #[error("provider response failed schema check: {0}")]
    SchemaCheck(String),
}

impl AiError {
    /// Stable error code for UI mapping and logs.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::Transport(_) => "AVID_AI_001",
            Self::Provider { .. } => "AVID_AI_002",
            Self::InvalidJson(_) => "AVID_AI_003",
            Self::SchemaCheck(_) => "AVID_AI_004",
        }
    }
}

/// Capabilities a model may offer (AGENTS §16, §79).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    Text,
    StructuredOutput,
    Vision,
    AudioUnderstanding,
    Transcription,
    ImageGeneration,
    Embeddings,
    ToolCalling,
}

/// A registered model with its probed capabilities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelInfo {
    /// e.g. `"qwen2.5:7b"`.
    pub id: String,
    /// e.g. `"ollama"`, `"openai"`.
    pub provider: String,
    pub capabilities: Vec<Capability>,
    /// Context window in tokens, if known.
    pub context_window: Option<u32>,
    /// True for on-machine inference (Ollama, LM Studio, …).
    pub local: bool,
}

impl ModelInfo {
    /// Whether the model offers a capability.
    #[must_use]
    pub fn supports(&self, capability: Capability) -> bool {
        self.capabilities.contains(&capability)
    }
}

/// Registry of configured models + capability routing.
#[derive(Debug, Default)]
pub struct ProviderRegistry {
    models: Vec<ModelInfo>,
}

impl ProviderRegistry {
    /// Register (or replace) a model by `(provider, id)`.
    pub fn register(&mut self, model: ModelInfo) {
        if let Some(existing) = self
            .models
            .iter_mut()
            .find(|m| m.provider == model.provider && m.id == model.id)
        {
            *existing = model;
        } else {
            self.models.push(model);
        }
    }

    /// All registered models.
    #[must_use]
    pub fn models(&self) -> &[ModelInfo] {
        &self.models
    }

    /// Recommend models for a capability: local-first, then others.
    /// Returns references ordered local models first (registration order kept).
    #[must_use]
    pub fn recommend(&self, capability: Capability) -> Vec<&ModelInfo> {
        let mut matches: Vec<&ModelInfo> = self
            .models
            .iter()
            .filter(|m| m.supports(capability))
            .collect();
        matches.sort_by_key(|m| !m.local);
        matches
    }
}

pub mod transcript;
pub mod whisper;

pub use transcript::{Segment as TranscriptSegment, Transcript, Word as TranscriptWord};
pub use whisper::{transcribe_wav, TranscribeError};

/// Minimal HTTP transport (mockable in tests).
pub trait HttpTransport {
    /// POST a JSON body; return `(status_code, body)`.
    fn post_json(&self, url: &str, body: &Value) -> Result<(u16, String), AiError>;
}

/// `ureq`-backed transport for production use.
pub struct UreqTransport {
    agent: ureq::Agent,
}

impl UreqTransport {
    /// Create a transport with default timeouts.
    #[must_use]
    pub fn new() -> Self {
        Self {
            agent: ureq::Agent::new_with_defaults(),
        }
    }
}

impl Default for UreqTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpTransport for UreqTransport {
    fn post_json(&self, url: &str, body: &Value) -> Result<(u16, String), AiError> {
        let mut response = self
            .agent
            .post(url)
            .send_json(body)
            .map_err(|e| AiError::Transport(e.to_string()))?;
        let status = response.status().as_u16();
        let text = response
            .body_mut()
            .read_to_string()
            .map_err(|e| AiError::Transport(e.to_string()))?;
        Ok((status, text))
    }
}

/// OpenAI-compatible chat adapter (Ollama `/v1`, LM Studio, vLLM, OpenAI).
/// One canonical path per ADR-005 — no per-provider call sites elsewhere.
pub struct OpenAiCompatAdapter<T: HttpTransport> {
    transport: T,
    base_url: String,
    model: String,
}

impl<T: HttpTransport> OpenAiCompatAdapter<T> {
    /// Create an adapter for `{base_url}/v1/chat/completions` + `{model}`.
    pub fn new(transport: T, base_url: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            transport,
            base_url: base_url.into(),
            model: model.into(),
        }
    }

    fn endpoint(&self) -> String {
        format!(
            "{}/v1/chat/completions",
            self.base_url.trim_end_matches('/')
        )
    }

    /// Request a structured JSON object conforming to `schema` (JSON Schema).
    /// Temperature is forced to 0 — determinism over creativity for edit plans.
    pub fn complete_json(
        &self,
        system: &str,
        user: &str,
        schema: &Value,
    ) -> Result<Value, AiError> {
        let body = serde_json::json!({
            "model": self.model,
            "temperature": 0,
            "messages": [
                {"role": "system", "content": system},
                {"role": "user", "content": user},
            ],
            "response_format": {
                "type": "json_schema",
                "json_schema": {"name": "avid_result", "strict": true, "schema": schema},
            },
        });
        let (status, text) = self.transport.post_json(&self.endpoint(), &body)?;
        if !(200..300).contains(&status) {
            return Err(AiError::Provider {
                status: status.to_string(),
                body: truncate(&text),
            });
        }
        let envelope: Value =
            serde_json::from_str(&text).map_err(|e| AiError::InvalidJson(e.to_string()))?;
        let content = envelope
            .pointer("/choices/0/message/content")
            .and_then(Value::as_str)
            .ok_or_else(|| AiError::SchemaCheck("missing choices[0].message.content".to_owned()))?;
        serde_json::from_str(content).map_err(|e| AiError::InvalidJson(e.to_string()))
    }

    /// Probe whether the model honors JSON-schema structured output.
    /// `Ok(true)` = usable for edit plans; `Ok(false)` = report honestly
    /// ("responded but lacks structured output"), never silently use prose.
    pub fn probe_structured_output(&self) -> Result<bool, AiError> {
        let schema = serde_json::json!({
            "type": "object",
            "properties": {"ok": {"type": "boolean"}},
            "required": ["ok"],
            "additionalProperties": false,
        });
        match self.complete_json("Reply with JSON only.", "probe", &schema) {
            Ok(value) => Ok(value
                .pointer("/ok")
                .and_then(Value::as_bool)
                .unwrap_or(false)),
            Err(AiError::Provider { .. })
            | Err(AiError::SchemaCheck(_))
            | Err(AiError::InvalidJson(_)) => Ok(false),
            Err(other) => Err(other),
        }
    }
}

fn truncate(s: &str) -> String {
    const LIMIT: usize = 1000;
    if s.len() <= LIMIT {
        s.to_owned()
    } else {
        format!("{}…[truncated]", &s[..LIMIT])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    /// Scripted transport: URL suffix → queued responses.
    type QueuedResponse = Vec<Result<(u16, String), String>>;

    #[derive(Debug, Default, Clone)]
    struct MockTransport {
        responses: Arc<Mutex<HashMap<String, QueuedResponse>>>,
    }

    impl MockTransport {
        fn queue(&self, url_suffix: &str, response: Result<(u16, String), String>) {
            self.responses
                .lock()
                .unwrap()
                .entry(url_suffix.to_owned())
                .or_default()
                .push(response);
        }
    }

    impl HttpTransport for MockTransport {
        fn post_json(&self, url: &str, _body: &Value) -> Result<(u16, String), AiError> {
            let mut guard = self.responses.lock().unwrap();
            let key = guard.keys().find(|k| url.ends_with(k.as_str())).cloned();
            match key.and_then(|k| {
                guard.get_mut(&k).and_then(|q| {
                    if q.is_empty() {
                        None
                    } else {
                        Some(q.remove(0))
                    }
                })
            }) {
                Some(Ok(response)) => Ok(response),
                Some(Err(message)) => Err(AiError::Transport(message)),
                None => Err(AiError::Transport(format!("no mock for {url}"))),
            }
        }
    }

    fn chat_envelope(content: &str) -> String {
        serde_json::json!({"choices": [{"message": {"content": content}}]}).to_string()
    }

    #[test]
    fn registry_prefers_local_models() {
        let mut registry = ProviderRegistry::default();
        registry.register(ModelInfo {
            id: "cloud-x".to_owned(),
            provider: "openai".to_owned(),
            capabilities: vec![Capability::StructuredOutput],
            context_window: Some(128_000),
            local: false,
        });
        registry.register(ModelInfo {
            id: "qwen2.5:7b".to_owned(),
            provider: "ollama".to_owned(),
            capabilities: vec![Capability::Text, Capability::StructuredOutput],
            context_window: Some(32_768),
            local: true,
        });
        let picks = registry.recommend(Capability::StructuredOutput);
        assert_eq!(picks.len(), 2);
        assert!(picks[0].local);
        assert!(picks
            .iter()
            .all(|m| m.supports(Capability::StructuredOutput)));
    }

    #[test]
    fn adapter_returns_parsed_json() {
        let transport = MockTransport::default();
        transport.queue(
            "/v1/chat/completions",
            Ok((200, chat_envelope(r#"{"operations": []}"#))),
        );
        let adapter = OpenAiCompatAdapter::new(transport, "http://localhost:11434", "qwen2.5:7b");
        let schema = serde_json::json!({"type": "object"});
        let value = adapter.complete_json("s", "u", &schema).unwrap();
        assert_eq!(value, serde_json::json!({"operations": []}));
    }

    #[test]
    fn adapter_rejects_prose_and_bad_status() {
        let transport = MockTransport::default();
        transport.queue(
            "/v1/chat/completions",
            Ok((200, chat_envelope("Sure thing!"))),
        );
        let adapter = OpenAiCompatAdapter::new(transport, "http://x", "m");
        let schema = serde_json::json!({"type": "object"});
        assert!(matches!(
            adapter.complete_json("s", "u", &schema),
            Err(AiError::InvalidJson(_))
        ));

        let transport = MockTransport::default();
        transport.queue("/v1/chat/completions", Ok((429, "rate limited".to_owned())));
        let adapter = OpenAiCompatAdapter::new(transport, "http://x", "m");
        assert!(matches!(
            adapter.complete_json("s", "u", &schema),
            Err(AiError::Provider { .. })
        ));
    }

    #[test]
    fn probe_reports_false_on_schema_failure() {
        let transport = MockTransport::default();
        transport.queue("/v1/chat/completions", Ok((200, chat_envelope("not json"))));
        let adapter = OpenAiCompatAdapter::new(transport, "http://x", "m");
        assert!(!adapter.probe_structured_output().unwrap());
    }

    /// LIVE eval: structured-output probe against local Ollama (`qwen2.5:7b`).
    /// Ignored by default (needs a running Ollama); run explicitly:
    /// `cargo test -p avid-ai -- --ignored`. This is the same probe the app
    /// runs at provider setup (AGENTS §147).
    #[test]
    #[ignore]
    fn live_ollama_structured_output_probe() {
        let adapter =
            OpenAiCompatAdapter::new(UreqTransport::new(), "http://localhost:11434", "qwen2.5:7b");
        let supported = adapter.probe_structured_output().expect("ollama reachable");
        assert!(
            supported,
            "qwen2.5:7b must honor json_schema for edit plans"
        );

        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "operations": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "type": {"type": "string"},
                            "start": {"type": "number"},
                            "end": {"type": "number"},
                            "reason": {"type": "string"},
                        },
                        "required": ["type", "start", "end", "reason"],
                        "additionalProperties": false,
                    },
                },
            },
            "required": ["operations"],
            "additionalProperties": false,
        });
        let plan = adapter
            .complete_json(
                "You are AVID's edit planner. Reply with JSON only.",
                "Remove the silence from 10 to 15 seconds (repetition).",
                &schema,
            )
            .expect("structured edit plan");
        let operations = plan
            .pointer("/operations")
            .and_then(Value::as_array)
            .expect("operations array");
        assert!(!operations.is_empty(), "plan = {plan}");
        for operation in operations {
            assert!(operation.get("type").and_then(Value::as_str).is_some());
            assert!(operation.get("start").and_then(Value::as_f64).is_some());
        }
    }
}
