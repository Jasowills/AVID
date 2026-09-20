# Tauri 2.x architecture research (Phase 0.3)

> Researched 2026-09-20. Primary sources: `v2.tauri.app` docs, `github.com/tauri-apps/tauri`.
> Decision recorded in `docs/decisions/ADR-001-tauri.md`.

## 1. Current state (2026)

- Stable line: **Tauri 2.11.x** (2.11.5, Jul 2026; 2.11.x since Apr 2026). MSRV discussion targets ~1.85–1.90 for 2.12; Rust 1.77.2 minimum for shell plugin today. Windows 7 support dropped in 2.12.
- Platforms: Windows 7+ (WebView2; 2.12 drops Win7), macOS 10.15+, Linux via webkit2gtk 4.1 (Ubuntu 22.04+), plus iOS/Android.
- Rendering: OS webview via WRY/TAO — WebView2 (Win), WKWebView (macOS), WebKitGTK (Linux). No bundled Chromium.
- System deps: macOS Xcode/CLI tools; Windows MSVC + WebView2 (+ VBScript feature for MSI); Linux webkit2gtk, libayatana, per-distro list on the prerequisites page.
- Bundles (`tauri.conf.json → bundle.targets`): `nsis` (.exe), `msi`, `dmg`, `app`, `deb`, `rpm`, `appimage`, `updater`, `all`. Typical app binary 2–15 MB; AppImage ~70+ MB (bundled deps) vs deb/rpm ~2–6 MB. Cross-compile: only NSIS via cargo-xwin; .msi/.dmg need native OS → matrix CI (`tauri-action`).
- Auto-updater (`tauri-plugin-updater`): Ed25519/Minisign signatures mandatory. `createUpdaterArtifacts: true`, `TAURI_SIGNING_PRIVATE_KEY` at build, `pubkey` + `endpoints` with `{{target}}/{{arch}}/{{current_version}}` templates. No rollback, no hosting — static JSON (GitHub/S3) or dynamic endpoint.
- Permissions/capabilities (replaces v1 allowlist): `src-tauri/capabilities/*.json` grant `plugin:permission` per window/webview (`core:default`, `fs:…`, `shell:allow-spawn`, `updater:default`) with allow/deny scopes. App commands allowed by default; lock via `AppManifest::commands`.
- Asset protocol (`convertFileSrc`): gated by `app.security.assetProtocol { enable, scope }` with `FsScope` globs; `deny` wins; dot-dirs need literal segment or `requireLiteralLeadingDot: false`. Runtime user-picked paths need `tauri-plugin-persisted-scope` (`protocol-asset` feature).

## 2. React ↔ Rust IPC patterns

Three primitives:

| Primitive | Use for | Notes |
|---|---|---|
| Commands + `invoke()` | **The only mutation path.** Typed request/response (`invoke('cmd', { camelCaseArgs })`). `async fn`, `Result<T,E>` (E must Serialize — thiserror → `{kind,message}`), `State`, `AppHandle`, `Channel`, `Request` (raw body). | Sync commands run on main thread; `async fn` / `#[tauri::command(async)]` → `async_runtime::spawn`. |
| Events (`emit`/`listen`) | Global toasts, job-center fan-out | Fire-and-forget JSON, no return values, no payload scoping, ordering not guaranteed for rapid listeners. |
| Channels (`Channel<T>`) | **All progress streams** (`{jobId, kind, payload}`) | Ordered, invocation-scoped, auto-cleanup. Small JSON (<8 KB) via eval; large payloads via internal fetch queue. |

Binary/large data: return `tauri::ipc::Response::new(Vec<u8>)` (not JSON `Vec<u8>`); receive raw via `Request` + `InvokeBody::Raw`; stream chunks over `Channel<&[u8]>`. **For media files don't IPC the bytes at all** — serve via `convertFileSrc`/custom protocol, pass `{mediaId, assetUrl}`.

Rules: coarse typed commands (`probe_media(path) → MediaInfo`); progress via Channel; notifications via events; payloads ~tens of KB max. Never: video frames/waveforms/whole files through JSON invoke or per-frame events; heavy work on main thread; `webview.eval()` per update; leaked `listen()` without `unlisten()` in React effects.

## 3. State ownership

- **Rust owns**: project/timeline truth, media metadata, job registry, render graph, undo-stack infra, path validation, secrets. Pattern: `app.manage(Mutex<AppState>)`, `State<'_, Mutex<T>>` in commands; std `Mutex` unless held across `.await` (then tokio Mutex). Persist via `tauri-plugin-store` (settings) / SQLite (indexes, transcripts) + `project.json` on disk.
- **Zustand owns**: ephemeral UI (selection, panel, timeline viewport, playback mirror, toasts), optimistic copies. Never authoritative media bytes or job state.
- **Background jobs** (3 options): (1) `async` commands + `tokio::spawn` + Channel progress — preferred for probe/thumbnail/waveform/transcribe; (2) FFmpeg/Whisper sidecars via `externalBin` + `tauri-plugin-shell` (`sidecar().spawn()`, stdout events, stdin-write, kill) with arg allowlists in capabilities; (3) Node sidecar only if needed. Sidecar binaries need `-<target-triple>` suffix. Cancel/job-ids in Rust; mirror `Job[]` to Zustand.

## 4. Alternatives

| | Tauri 2.x | Electron 34+ | Neutralinojs |
|---|---|---|---|
| Runtime | OS webview + Rust | Bundled Chromium + Node | OS webview + thin C++ |
| Hello-world | ~3–15 MB, ~50–100 MB RAM | ~150–300 MB, ~100–300 MB RAM | ~2–5 MB, ~40–80 MB RAM |
| Rendering | varies per OS | identical Chromium | varies per OS |
| Backend | Rust commands/plugins, capabilities sandbox | full Node, large attack surface | small JS API |
| FFmpeg story | sidecar `externalBin` + shell plugin (~25 MB/arch) | npm ffmpeg-static, easiest, huge bundle | custom extension/spawn, thin tooling |
| Updater/ecosystem | built-in signed updater | Forge/builder/updater, massive | manual/basic |

Verdict: Tauri wins on bundle/memory/security/local-first posture + Rust media core. Loses on Chromium uniformity (codec/WebGL/CSS QA burden) and Rust hiring curve. Neutralino too thin for job engine/undo/plugins/updater.

## 5. Risks for a video editor (with mitigations)

1. **Seekable preview breaks** — `asset://`/`convertFileSrc` has no HTTP Range support; `<video>` needs ranges/seek (black screen on macOS WKWebView/Linux WebKitGTK). → Custom `stream://` protocol with Range/206 + MIME/CORS, or `tauri-plugin-localhost`; proxy-first H.264/AAC; per-OS preview test matrix. **#1 risk.**
2. **FS/asset scope denials** ("not configured to allow path", dot-dirs, relink after restart) → narrow project-dir + `$APPCACHE` scopes, `persisted-scope[protocol-asset]`, relink dialog.
3. **Long FFmpeg jobs** freeze/kill/lose progress → sidecar spawn + Channel progress, job-ids + cancel/kill, resume from last segment, async commands only.
4. **WebView divergence** (codecs/CSS/WebGL) → Chromium-safe UI subset, capability banner, proxy-first preview, cross-OS E2E fixtures.
5. **Signing/updater/bundle bloat** (Gatekeeper/SmartScreen, missing .sig, FFmpeg triples × OS) → early signing pipeline, `createUpdaterArtifacts` CI check, per-arch sidecars, NSIS default on Win, deb+AppImage split on Linux.

## Recommendation

- Pin `tauri 2.11.x` (latest 2.11.5), `@tauri-apps/api 2.x`, plugins `2.x` in lockstep, Rust ≥1.77.2 (aim 1.85+), Vite + React 18 + TS 5, Zustand 5.
- IPC: commands = only mutation path; Channels = progress; events = fan-out; never media bytes over IPC.
- Bundle FFmpeg as versioned sidecars per triple; never system ffmpeg.
- CI: `tauri-action` matrix macOS/Win/Linux.

Sources: v2.tauri.app (architecture, process-model, calling-rust, calling-frontend, IPC, state-management, sidecar, shell plugin, updater, capabilities, asset-protocol, config reference, prerequisites, release notes); github.com/tauri-apps/tauri.
