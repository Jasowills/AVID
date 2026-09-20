//! Seekable media streaming for `<video>` preview (Phase 2 spike resolution).
//!
//! Tauri's `asset://` protocol has no HTTP Range support, so seeking breaks
//! on macOS/Linux webviews. `stream://project/<relative>` serves project
//! files and `stream://cache/<relative>` serves app-cache files with full
//! single-range semantics (206 Partial Content). The byte-serving core is
//! pure and unit-tested; the Tauri handler below is thin glue.

use std::path::{Path, PathBuf};

/// A parsed single byte range (end inclusive, like HTTP).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RangeRequest {
    pub start: u64,
    pub end: Option<u64>,
}

/// Range parse failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeError {
    /// Malformed header (not `bytes=` single range).
    Invalid,
    /// Well-formed but unsatisfiable (start beyond EOF, start > end).
    Unsatisfiable,
}

/// Parse one `Range` header value (`bytes=<start>-<end?>`, `bytes=-<suffix>`).
/// Multi-range headers are rejected (video elements never send them).
pub fn parse_range(header: &str, total: u64) -> Result<RangeRequest, RangeError> {
    let spec = header
        .trim()
        .strip_prefix("bytes=")
        .ok_or(RangeError::Invalid)?;
    if spec.contains(',') {
        return Err(RangeError::Invalid);
    }
    let (start_part, end_part) = spec.split_once('-').ok_or(RangeError::Invalid)?;
    if start_part.is_empty() {
        // Suffix range: last N bytes.
        let suffix: u64 = end_part.trim().parse().map_err(|_| RangeError::Invalid)?;
        if suffix == 0 || total == 0 {
            return Err(RangeError::Unsatisfiable);
        }
        let start = total.saturating_sub(suffix);
        return Ok(RangeRequest { start, end: None });
    }
    let start: u64 = start_part.trim().parse().map_err(|_| RangeError::Invalid)?;
    if start >= total {
        return Err(RangeError::Unsatisfiable);
    }
    if end_part.trim().is_empty() {
        return Ok(RangeRequest { start, end: None });
    }
    let end: u64 = end_part.trim().parse().map_err(|_| RangeError::Invalid)?;
    if end < start {
        return Err(RangeError::Unsatisfiable);
    }
    Ok(RangeRequest {
        start,
        end: Some(end.min(total.saturating_sub(1))),
    })
}

/// Served byte range with HTTP semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeResponse {
    /// 200 (full), 206 (partial), or 416 (unsatisfiable).
    pub status: u16,
    /// `Content-Range` value for 206/416 (`bytes */total` for 416).
    pub content_range: Option<String>,
    pub content_type: &'static str,
    pub body: Vec<u8>,
}

/// Serve bytes with range semantics (pure — the webview contract in tests).
pub fn serve_range(
    data: &[u8],
    range: Option<RangeRequest>,
    content_type: &'static str,
) -> RangeResponse {
    let total = data.len() as u64;
    let Some(range) = range else {
        return RangeResponse {
            status: 200,
            content_range: None,
            content_type,
            body: data.to_vec(),
        };
    };
    if range.start >= total {
        return RangeResponse {
            status: 416,
            content_range: Some(format!("bytes */{total}")),
            content_type,
            body: vec![],
        };
    }
    let end = range
        .end
        .unwrap_or(total.saturating_sub(1))
        .min(total.saturating_sub(1));
    if end < range.start {
        return RangeResponse {
            status: 416,
            content_range: Some(format!("bytes */{total}")),
            content_type,
            body: vec![],
        };
    }
    #[allow(clippy::cast_possible_truncation)]
    let (start, end) = (range.start as usize, end as usize);
    RangeResponse {
        status: 206,
        content_range: Some(format!("bytes {start}-{end}/{total}")),
        content_type,
        body: data[start..=end].to_vec(),
    }
}

/// URL resolution failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamError {
    /// Unknown scope or malformed URL.
    NotFound,
    /// Traversal outside the allowed roots.
    Forbidden,
}

/// Resolve `stream://project/<relative>` / `stream://cache/<relative>` to a
/// file under the given roots. Percent-decoding applied; `..` segments,
/// absolute paths, and unknown scopes are rejected before any filesystem
/// access (AGENTS §106).
pub fn resolve_stream_url(
    url: &str,
    project_dir: &Path,
    cache_dir: &Path,
) -> Result<PathBuf, StreamError> {
    let without_scheme = url.strip_prefix("stream://").ok_or(StreamError::NotFound)?;
    // Tauri passes `stream://<host>/<path>`; our frontend always uses the
    // `localhost` host with `project/` or `cache/` first segments.
    let path = without_scheme
        .strip_prefix("localhost/")
        .unwrap_or(without_scheme);
    let (scope, relative) = path.split_once('/').ok_or(StreamError::NotFound)?;
    let root = match scope {
        "project" => project_dir,
        "cache" => cache_dir,
        _ => return Err(StreamError::NotFound),
    };
    let decoded = percent_decode(relative)?;
    if decoded.is_empty() {
        return Err(StreamError::NotFound);
    }
    let normalized = decoded.replace('\\', "/");
    if normalized.starts_with('/') || normalized.split('/').any(|segment| segment == "..") {
        return Err(StreamError::Forbidden);
    }
    Ok(root.join(normalized))
}

/// Minimal percent-decoding for URL paths (UTF-8 output, `+` kept literal).
fn percent_decode(input: &str) -> Result<String, StreamError> {
    let bytes = input.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            if index + 2 >= bytes.len() {
                return Err(StreamError::NotFound);
            }
            let hex = std::str::from_utf8(&bytes[index + 1..index + 3])
                .map_err(|_| StreamError::NotFound)?;
            let byte = u8::from_str_radix(hex, 16).map_err(|_| StreamError::NotFound)?;
            output.push(byte);
            index += 3;
        } else {
            output.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(output).map_err(|_| StreamError::NotFound)
}

/// MIME type for previewable extensions (safe fallback included).
pub fn mime_for(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or("")
        .to_lowercase()
        .as_str()
    {
        "mp4" | "m4v" => "video/mp4",
        "mov" => "video/quicktime",
        "webm" => "video/webm",
        "mkv" => "video/x-matroska",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "aac" | "m4a" => "audio/aac",
        "ogg" | "oga" => "audio/ogg",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        _ => "application/octet-stream",
    }
}

/// Tauri glue: resolve, read, and serve a `stream://` request.
/// Roots: open project dir (`project/…`) and app cache (`cache/…`).
/// Failures map to HTTP statuses, never panics and never path leaks:
/// traversal → 403, unknown/missing → 404, unsatisfiable range → 416.
///
/// Note: files are read fully into memory. Preview targets are proxies
/// (tens of MB); range-sliced file IO is the documented follow-up if
/// originals are ever streamed directly.
pub fn handle_stream_request<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    request: http::Request<Vec<u8>>,
) -> http::Response<Vec<u8>> {
    use tauri::Manager;
    let fail = |status: u16| {
        http::Response::builder()
            .status(status)
            .body(Vec::new())
            .unwrap_or_else(|_| http::Response::new(Vec::new()))
    };
    let url = request.uri().to_string();
    let range_header = request
        .headers()
        .get("range")
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);

    let project_dir = app
        .state::<crate::AppState>()
        .with_session(|session| Ok(session.dir().to_path_buf()));
    let Ok(project_dir) = project_dir else {
        return fail(404);
    };
    let Ok(cache_dir) = app.path().app_cache_dir() else {
        return fail(500);
    };
    let Ok(path) = resolve_stream_url(&url, &project_dir, &cache_dir) else {
        return fail(404);
    };
    // Containment double-check after symlinks resolve.
    let canonical_root = project_dir.canonicalize().unwrap_or(project_dir.clone());
    let canonical_cache = cache_dir.canonicalize().unwrap_or(cache_dir.clone());
    let Ok(canonical) = path.canonicalize() else {
        return fail(404);
    };
    if !canonical.starts_with(&canonical_root) && !canonical.starts_with(&canonical_cache) {
        return fail(403);
    }
    let Ok(data) = std::fs::read(&canonical) else {
        return fail(404);
    };
    let mime = mime_for(&canonical);
    let total = data.len() as u64;
    // Malformed Range headers are ignored per HTTP convention (full 200);
    // unsatisfiable ones are honored (416).
    let (range, unsatisfiable) = match range_header.as_deref() {
        None => (None, false),
        Some(header) => match parse_range(header, total) {
            Ok(request) => (Some(request), false),
            Err(RangeError::Invalid) => (None, false),
            Err(RangeError::Unsatisfiable) => (None, true),
        },
    };
    let served = if unsatisfiable {
        RangeResponse {
            status: 416,
            content_range: Some(format!("bytes */{total}")),
            content_type: mime,
            body: vec![],
        }
    } else {
        serve_range(&data, range, mime)
    };
    let mut builder = http::Response::builder()
        .status(served.status)
        .header("content-type", served.content_type)
        .header("content-length", served.body.len().to_string())
        .header("accept-ranges", "bytes");
    if let Some(content_range) = served.content_range {
        builder = builder.header("content-range", content_range);
    }
    builder.body(served.body).unwrap_or_else(|_| fail(500))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_single_ranges_and_suffixes() {
        assert_eq!(
            parse_range("bytes=0-99", 1000),
            Ok(RangeRequest {
                start: 0,
                end: Some(99)
            })
        );
        assert_eq!(
            parse_range("bytes=500-", 1000),
            Ok(RangeRequest {
                start: 500,
                end: None
            })
        );
        assert_eq!(
            parse_range("bytes=-200", 1000),
            Ok(RangeRequest {
                start: 800,
                end: None
            })
        );
        // End clamps to EOF instead of failing.
        assert_eq!(
            parse_range("bytes=900-9999", 1000),
            Ok(RangeRequest {
                start: 900,
                end: Some(999)
            })
        );
    }

    #[test]
    fn rejects_malformed_and_unsatisfiable_ranges() {
        for bad in ["bytes=0-1,3-4", "items=0-1", "bytes=", "bytes=abc-"] {
            assert_eq!(
                parse_range(bad, 1000),
                Err(RangeError::Invalid),
                "header: {bad}"
            );
        }
        // Zero suffix and out-of-range starts are well-formed but unsatisfiable.
        for bad in [
            "bytes=-0",
            "bytes=1000-",
            "bytes=1500-2000",
            "bytes=500-100",
        ] {
            assert_eq!(
                parse_range(bad, 1000),
                Err(RangeError::Unsatisfiable),
                "header: {bad}"
            );
        }
    }

    #[test]
    fn serves_full_partial_and_empty_ranges() {
        let data: Vec<u8> = (0..100).collect();
        let full = serve_range(&data, None, "video/mp4");
        assert_eq!((full.status, full.body.len()), (200, 100));

        let partial = serve_range(
            &data,
            Some(RangeRequest {
                start: 10,
                end: Some(19),
            }),
            "video/mp4",
        );
        assert_eq!(partial.status, 206);
        assert_eq!(partial.content_range, Some("bytes 10-19/100".to_owned()));
        assert_eq!(partial.body, (10..20).collect::<Vec<_>>());

        let open = serve_range(
            &data,
            Some(RangeRequest {
                start: 90,
                end: None,
            }),
            "video/mp4",
        );
        assert_eq!((open.status, open.body.len()), (206, 10));

        let past_end = serve_range(
            &data,
            Some(RangeRequest {
                start: 100,
                end: None,
            }),
            "video/mp4",
        );
        assert_eq!(past_end.status, 416);
        assert_eq!(past_end.content_range, Some("bytes */100".to_owned()));
    }

    #[test]
    fn resolves_project_and_cache_urls_with_traversal_guard() {
        let project = Path::new("/proj");
        let cache = Path::new("/cache");
        assert_eq!(
            resolve_stream_url("stream://localhost/project/media/a.mp4", project, cache),
            Ok(PathBuf::from("/proj/media/a.mp4"))
        );
        assert_eq!(
            resolve_stream_url("stream://localhost/cache/avid-audio/x.wav", project, cache),
            Ok(PathBuf::from("/cache/avid-audio/x.wav"))
        );
        // Percent-decoding works for spaces.
        assert_eq!(
            resolve_stream_url(
                "stream://localhost/project/media/my%20clip.mp4",
                project,
                cache
            ),
            Ok(PathBuf::from("/proj/media/my clip.mp4"))
        );
        // Traversal, absolute paths, unknown scopes, bad escapes all fail.
        for bad in [
            "stream://localhost/project/../secret.mp4",
            "stream://localhost/project//etc/passwd",
            "stream://localhost/other/a.mp4",
            "http://localhost/project/a.mp4",
            "stream://localhost/project/%zz.mp4",
            "stream://localhost/project/%",
        ] {
            assert!(
                resolve_stream_url(bad, project, cache).is_err(),
                "url: {bad}"
            );
        }
    }

    #[test]
    fn mime_types_cover_preview_formats() {
        assert_eq!(mime_for(Path::new("a.mp4")), "video/mp4");
        assert_eq!(mime_for(Path::new("a.WAV")), "audio/wav");
        assert_eq!(mime_for(Path::new("a.xyz")), "application/octet-stream");
    }
}
