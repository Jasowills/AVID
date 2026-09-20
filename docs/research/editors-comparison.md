# Competitive editor research (Phase 0.2)

> Researched 2026-09-20. `[F]` = documented in official docs. `[A]` = anecdotal (reviews/forums). Distinguish the two when citing.
> Question driving this: what do users like / hate / where do they lose time / where does AVID differentiate?

## 1. DaVinci Resolve (Blackmagic, v20/21)

- Timeline `[F]`: track-based Cut + Edit pages. Ripple/roll/slip/slide, A/B trimmer, ripple-delete, snap. (blackmagicdesign.com/products/davinciresolve)
- Text editing: no full doc-edit. IntelliScript (script + transcription → auto timeline), Animated Subtitles `[F]`.
- AI `[F]`: on-device Neural Engine — Magic Mask v2, Voice Isolation, Dialogue Separator, Music Remixer, SmartSwitch speaker multicam, Audio Assistant auto-mix, SuperScale; Studio-only AutoCaption.
- Like: free tier is a real pro NLE to 4K, no watermark; best color + Fairlight audio; Cut page speed.
- Hate `[A]`: steep learning curve, page sprawl, Studio paywall for captions/NR, GPU hunger.
- Model: $0 + Studio $295 one-time. Local-first desktop.

## 2. Adobe Premiere Pro (v26.5)

- Timeline `[F]`: track-based, patching/targeting, 3-point edits, split/trim/ripple/roll/slip/slide, snap, multicam. (helpx.adobe.com/premiere)
- Text editing `[F]` — **the reference**: Sensei transcribe → timecoded transcript; select/cut/paste/reorder text = timeline ripple; search; bulk pause/filler delete; transcript → captions.
- AI `[F]`: Generative Extend (Firefly cloud, credits, +2s video/+10s audio, dialogue muted, no music); Enhance Speech, Auto Reframe, Media Intelligence, Translate Captions (27+ langs), Object Mask.
- Like: industry standard, AE/Audition/Frame.io ecosystem, fastest speech rough-cut.
- Hate `[A]`: $22.99–34.49/mo subscription, crashes/perf complaints, credit metering, weak multicam color workflow.
- Model: subscription-only. Local edit, cloud AI.

## 3. Final Cut Pro (Apple, v11/12.2)

- Timeline `[F]`: **magnetic**, trackless primary storyline + connected clips; auto-close gaps, Q-connect, roll/slip/slide, Precision Editor, snap. (apple.com/final-cut-pro, support.apple.com/guide/final-cut-pro)
- Text editing: none. Transcribe-to-Captions on-device via Neural Engine (Apple Silicon) `[F]`. The Verge (2024): "wish Apple went further and added text-based editing."
- AI `[F]`: Magnetic Mask, Smart Conform (16:9 → vertical/square), Enhance Light/Color, Edit Detection, Auto Mask.
- Like: speed/battery on Mac, magnetic ease, background render.
- Hate `[A]`: Mac-only, paradigm shock for track editors, thin collaboration, late to AI captions.
- Model: $299.99 one-time or $12.99/mo Creator Studio. Local on-device.

## 4. CapCut (ByteDance)

- Timeline `[F]`: multitrack mobile/desktop/web, split/trim/snap, keyframes, ripple-like close. (capcut.com)
- Transcript-based edit + auto-captions `[F]`.
- AI `[F]`: AutoCut silence remover, filler removal, Auto Reframe, background removal, vocal isolation, Seedance text-to-video, avatars/voice-clone.
- Like: free real editor to 1080p no watermark, templates, fastest TikTok pipeline.
- Hate `[A]`: June 2025 ToS perpetual-license backlash, free→Pro migration, ByteDance privacy scrutiny, weak color/audio/pro management.
- Model: free + Standard ~$10/mo + Pro $19.99/mo + AI credits. Cloud-hybrid.

## 5. Descript (the text-editing inventor)

- Model `[F]`: doc-first, not tracks — composition transcript + Scenes/layouts; limited multitrack Sequences; timeline export XML/FCPXML/AAF to Premiere/FCP/Resolve. (descript.com/underlord, help.descript.com)
- Text editing `[F]`: delete/rearrange text = delete/move media; word-level sync, speaker labels, search.
- AI `[F]`: Underlord agent, Remove Filler Words, Studio Sound, silence/shorten, Create Clips, Regenerate/voice clone, eye-contact, AI B-roll/gen media, translate/dub 30+ langs.
- Like: fastest rough-cut for podcasts/talking-head; non-editors can edit.
- Hate `[A]`: weak multicam export (requested since 2020), no deep color/VFX/compositing, 4K/proxy perf complaints, meters (media hours + AI credits).
- Model: free (1h) / $16–65/mo tiers. Cloud-dependent.

## 6. Runway

- Timeline `[F]`: browser layered timeline, drag, optional snap — help docs say "no longer actively maintained, use local editor for larger projects." (runwayml.com, help.runwayml.com)
- Text editing: none.
- AI: Gen-4/4.5 text/image-to-video, Aleph keyframed edit, Act-One, green screen/inpaint/outpaint, lip-sync, 4K upscale. No silence/rough-cut/reframe NLE tools.
- Like: frontier generation/VFX without skill. Hate `[A]`: credits burn fast, not an NLE, weak long-form.
- Model: free (125 one-time credits) / $15–95/mo. Cloud-only.

## 7. VEED

- Timeline `[F]`: browser multitrack, split/trim/snap. (veed.io)
- Text editing `[F]`: Edit-by-Script + Magic Cut — transcript edit → timeline, fully editable.
- AI `[F]`: Magic Cut (silence+filler+mistakes, English-only), Silence Remover, Clean Audio, Eye Contact, AI B-roll/auto-edits, auto-subtitles, translate/dub, Subtitle API.
- Like: zero-install, fastest subtitled social clips, brand kits/teams.
- Hate `[A]`: per-seat $20–70, free 10 min/mo watermarked, upload/render dependence.
- Model: cloud-only browser + API.

## 8. Screen Studio

- Timeline `[F]`: macOS recorder + minimal cut/trim/speed/crop — not a full NLE. (screen.studio)
- Text editing: none; AI transcript + subtitles only.
- AI: auto cinematic zoom on cursor, cursor smoothing, noise removal, webcam overlay, auto vertical reframe, 4K60/GIF.
- Like: beautiful demos in minutes; loved by Stripe/Vercel/Google teams.
- Hate `[A]`: Mac Ventura+ only, $29/mo or $108/yr (lifetime goodwill lost), no free export.
- Model: paid-only export. Local record/edit/export.

## 9. Riverside

- Timeline `[F]`: local-capture + browser timeline, color-coded + multitrack separate tracks, trim/cut. (riverside.fm)
- Text editing `[F]`: delete/copy/paste in transcript edits video; 99% claim, 100+ langs.
- AI `[F]`: filler/silence removal, Magic Audio, eye-contact, auto-layout speaker-switch, Correct Speech/VideoDub, prompt B-roll, Magic Clips highlights → Reels/TikTok/Shorts, animated captions, show notes/chapters.
- Like: 4K local tracks immune to internet; record → edit → host/publish end-to-end for podcasts.
- Hate `[A]`: caps on separate-track downloads, extra AI credits, free 720p watermarked.
- Model: free (2h) / $15–34/mo. Local capture, cloud workflow.

## 10. OpusClip

- Model `[F]`: no real NLE — clip-list + trim/extend/caption-style editor + Premiere/Resolve export. (opus.pro)
- Text editing: none; prompt-to-clip, topic search, timeframe slider.
- AI `[F]`: ClipAnything long→short viral detection, ReframeAnything AI tracking 16:9→9:16, Virality Score, Auto Hook, 97% captions 20+ langs, AI B-roll, filler/silence removal, scheduler/auto-post.
- Like: best long→short automation (podcasts/streams), 10M+ creators.
- Hate `[A]`: generic/awkward cuts need review, input-minute billing (pay for feed, not keep), 60-day credit expiry, 3-day free storage.
- Model: free 60 min/mo watermarked / $15–29/mo. Cloud-only.

---

## Synthesis

### Adopt (proven by)

1. **Transcript as edit surface** (Premiere/Descript/Riverside/VEED) — select/delete text = ripple; search-to-jump; pause/filler bulk review. AVID goes further: transcript ops become validated commands.
2. **Magnetic gap-closing + explicit trim modes** (FCP proves beginners need no stranded gaps; Resolve proves pros need roll/ripple/slip/slide modes).
3. **Proposal review, never silent mutation** (Descript/Riverside/VEED accept/reject lists) — builds trust; AVID adds per-op diff + transactional rollback.
4. **One-click captions + templates + auto-reframe with manual override** (CapCut/OpusClip for Shorts) — keep styles editable.
5. **Opinionated automation + proxy/background jobs + Job Center** (Screen Studio auto-zoom; Resolve proxies) — fast preview without UI freeze.

### Avoid

1. **Cloud-mandatory AI** (Runway/VEED/OpusClip) — AVID basic edit/export works offline (AGENTS §100 test).
2. **Credit/meter shock** (Premiere Firefly, Descript hours+credits, Opus input-minute billing) — show cost pre-call, prefer local, explicit Allow Once (AGENTS §144).
3. **Fake/irreversible AI** (Opus awkward cuts, Premiere Extend limits) — schema-validated Edit Plans, rollback, pre-AI snapshot (AGENTS §54–55, §116–117).
4. **ToS/privacy overreach** (CapCut 2025 license) + silent upload — local-first, per-operation consent, BYOM (AGENTS §18–19, §77).
5. **Paradigm lock-in** (FCP magnetic shock, Descript weak multicam round-trip) — standard shortcuts (JKL/I/O/S), tracks for pros + simple mode, FCPXML/XML export path.

### Where AVID differentiates (nobody combines these)

- **Explain → Visualize as editable deterministic diagrams** (Scene Spec / Mermaid / SVG) — Runway/Descript do generative B-roll only, never editable architecture diagrams.
- **BYOM provider adapter + capability registry** (Ollama default) vs Adobe/Apple locked models.
- **Local-first command-based timeline** — AI emits validated `remove_range`/`add_visual` ops with diff/undo, not flattened video.
