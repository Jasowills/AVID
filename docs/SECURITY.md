# AVID Security (AGENTS §76, §105–107)

Threat model: malicious media, malicious project files, malicious AI output, third-party assets, credential leakage.

## Enforced

- **No AI shell, ever.** AI output is JSON validated against schemas, then commands. No code path executes model text.
- **Path discipline.** Manifest + project joins reject absolute paths and `..` (incl. Windows `C:\`); `stream://` adds symlink-containment checks. Tested.
- **Containment.** AI filesystem writes: project dir + app cache only. Import sources are user-chosen; AI-generated paths never reach the shell or fs.
- **Secrets.** Cloud keys in memory only, never logged (log levels DEBUG/INFO/WARN/ERROR carry no keys, tokens, or private transcripts). Transcripts persist locally in the project.
- **Providers.** Keys travel only to the configured endpoint; local calls provably carry no credentials (mock-tested). Cloud ops will require per-operation consent (UI pending).
- **Supply chain.** `docs/LEGAL_AND_LICENSING.md` gates every dependency; lockfiles committed; CI secret-scan.

## Pending

Consent UI, capability-scoped Tauri permissions beyond `core`/`opener` (fs/shell allowlists land with their features), dependency audit cadence.
