# AVID

## AI Video Intelligence & Direction

### Master Product, UX, Architecture, Engineering, Research, Testing and Delivery Specification

---

# 0. OPENING DIRECTIVE TO THE AGENT

You are the principal engineer, product engineer, UX architect, desktop application engineer, media systems engineer, AI systems engineer, QA engineer, and technical researcher responsible for building AVID.

AVID is not a toy project.

AVID is intended to become a genuinely usable cross-platform desktop video editor that combines traditional non-linear editing with AI-assisted editing, AI-generated visual explanations, templates, local AI models, cloud AI providers, and a model-agnostic architecture.

The final product must feel like software that a real creator could install, open, import footage into, edit, export, and continue using.

Do not build a mockup that merely looks like an editor.

Do not build a collection of disconnected proof-of-concepts.

Do not optimize for impressive screenshots at the expense of actual editing functionality.

Do not create fake AI functionality where buttons merely simulate work.

Do not hard-code one AI provider.

Do not make cloud AI mandatory.

Do not make the application dependent on an internet connection for basic editing.

Do not prematurely implement complex AI features before the underlying timeline, project, media, rendering, and undo/redo systems are reliable.

Build the product in vertical slices.

Every feature must eventually connect to a working end-to-end user workflow.

The goal is:

IMPORT → UNDERSTAND → EDIT → VISUALIZE → REVIEW → MODIFY → EXPORT

with AI assisting at every appropriate stage without taking control away from the user.

---

# 1. PRODUCT IDENTITY

## Product

AVID

## Meaning

AI Video Intelligence & Direction

The name should be used as a product name rather than constantly spelling out the acronym in the UI.

## Product description

AVID is a local-first, cross-platform AI video editor that understands video content, edits through an editable timeline, and can automatically create supporting visuals such as diagrams, text callouts, code snippets, charts, logos, captions, animations, and other contextual elements.

## Core thesis

Traditional video editors make humans manually translate an idea into:

* cuts
* transitions
* captions
* graphics
* diagrams
* B-roll
* animations
* layouts
* audio adjustments
* exports

AVID should let the creator describe what they want while retaining a real editable timeline underneath.

The AI should not simply generate a final video.

The AI should generate and manipulate an editable representation of the video.

Therefore:

AI proposes.

AVID structures.

The user controls.

The renderer executes.

---

# 2. PRODUCT PRINCIPLES

These principles are non-negotiable.

## 2.1 Local-first

AVID must work without cloud AI.

Users should be able to configure local inference where practical.

Examples include:

* Ollama
* OpenAI-compatible local endpoints
* LM Studio-compatible endpoints
* vLLM-compatible endpoints
* local Whisper
* future local multimodal models

Cloud providers are optional.

Possible cloud providers include:

* OpenAI
* Anthropic / Claude
* Google / Gemini
* other compatible providers

The architecture must not contain provider-specific logic throughout the application.

Instead:

AI Provider
→ Provider Adapter
→ Capability Interface
→ AVID AI Runtime

---

# 3. IMPORTANT DISTINCTION

AVID is NOT:

"ChatGPT but for video."

AVID is NOT:

"CapCut with an AI button."

AVID is NOT:

"An automatic video generator."

AVID is:

"A real video editor with an AI director that understands the media and can operate an editable project."

This distinction must influence every architectural decision.

---

# 4. TARGET USERS

Design for several personas.

## Persona A: Technical creator

Examples:

* software engineers
* developer advocates
* educators
* technical YouTubers
* startup founders
* documentation creators

They record:

* coding sessions
* architecture explanations
* tutorials
* product demos
* conference talks

They want AVID to automatically create:

* diagrams
* architecture visuals
* code highlights
* labels
* captions
* chapter markers
* B-roll
* zooms
* explanatory animations

This should be AVID's initial differentiation.

---

## Persona B: YouTube creator

Needs:

* fast rough cuts
* silence removal
* filler removal
* captions
* thumbnails
* chapters
* Shorts
* B-roll
* pacing improvements

---

## Persona C: Educator

Needs:

* diagrams
* equations
* annotations
* visual explanations
* screen recordings
* chapter structure
* captions
* presentation visuals

---

## Persona D: Podcaster

Needs:

* multi-camera editing
* speaker detection
* silence removal
* audio cleanup
* captions
* clips
* Shorts
* speaker layouts

---

## Persona E: General creator

Needs:

* templates
* simple editing
* AI assistance
* social formats
* music
* captions
* easy exports

The UI must remain approachable even though the underlying engine is sophisticated.

---

# 5. USER PAIN POINTS TO SOLVE

Research and continuously validate these.

## Existing editing pain

Video editing often requires creators to:

1. import media
2. organize footage
3. watch everything
4. find good moments
5. cut mistakes
6. remove silence
7. edit audio
8. add captions
9. find B-roll
10. create graphics
11. create diagrams
12. animate graphics
13. resize for platforms
14. create clips
15. export
16. discover mistakes
17. render again
18. repeat

AVID should compress this workflow.

But do not remove user control.

---

# 6. THE CORE USER EXPERIENCE

The ideal workflow should feel like:

## Step 1

"Create Project"

## Step 2

Drop footage into AVID.

## Step 3

AVID automatically analyzes it.

## Step 4

User sees:

* transcript
* speakers
* scenes
* clips
* audio
* detected topics
* possible cuts
* visual opportunities

## Step 5

AVID asks:

"What are you making?"

Examples:

* YouTube tutorial
* Technical explainer
* Podcast
* Short
* Course lesson
* Product demo
* Documentary
* Talking-head video

## Step 6

AVID generates a proposed edit.

## Step 7

User reviews it.

## Step 8

User says:

"Make the Kafka explanation more visual."

AVID creates an editable diagram.

## Step 9

User edits the diagram.

## Step 10

User exports.

---

# 7. UX PHILOSOPHY

AVID should combine:

* professional editing power
* consumer simplicity
* AI conversational control
* local-first transparency

Do not overwhelm new users with every feature.

Progressively reveal complexity.

---

# 8. APPLICATION STRUCTURE

Desktop application.

Primary platform:

* macOS
* Windows
* Linux

Use:

React
TypeScript
Tauri
Rust

The frontend owns interaction.

Rust owns core native functionality.

---

# 9. RECOMMENDED STACK

## Desktop

Tauri 2.x

## UI

React

## Language

TypeScript

## Styling

Tailwind CSS

Use a consistent design system rather than random utility classes everywhere.

## State

Zustand or an equivalent predictable state solution.

Use normalized state where appropriate.

Do not store enormous media objects directly in React state.

---

# 10. RUST CORE

Rust should own:

* project engine
* media metadata
* timeline engine
* command execution
* undo/redo infrastructure
* rendering orchestration
* FFmpeg process management
* filesystem abstraction
* background jobs
* proxy generation
* thumbnails
* waveform generation
* cache management
* media probing
* export orchestration
* native performance-sensitive operations

Do not create Rust simply because "Rust is cool".

Every Rust module should have a clear responsibility.

---

# 11. MEDIA ENGINE

Use FFmpeg as the foundational media engine.

AVID should support:

* MP4
* MOV
* WebM
* MKV where practical
* WAV
* MP3
* AAC
* common image formats

Build an abstraction around FFmpeg.

Do not scatter raw FFmpeg command strings throughout the application.

Create:

MediaEngine

with operations such as:

* probe
* transcode
* proxy
* thumbnail
* waveform
* extractAudio
* extractFrame
* render
* concatenate
* composite
* burnCaptions

FFmpeg provides filters for operations such as text drawing and overlays, which should become lower-level primitives rather than direct application logic.

---

# 12. PROJECT FORMAT

AVID projects must be portable.

A project should not become unusable because the application database changes.

Use a versioned project format.

Example:

avid-project/
project.json
media/
proxies/
thumbnails/
audio/
cache/
assets/
generated/
exports/

The project manifest should contain:

* project ID
* version
* metadata
* timeline
* tracks
* clips
* assets
* compositions
* captions
* transcripts
* AI operations
* template references
* render settings

Use migrations when the schema changes.

Never silently break old projects.

---

# 13. TIMELINE ENGINE

The timeline is the heart of AVID.

Do not treat it as a visual UI component.

Build a real timeline model.

Support:

* video tracks
* audio tracks
* text tracks
* graphics tracks
* captions
* overlays
* effects
* transitions
* markers

Every timeline item needs:

* ID
* source
* start
* duration
* in point
* out point
* track
* transform
* opacity
* audio properties
* effects
* metadata

---

# 14. NON-DESTRUCTIVE EDITING

Never destroy original media.

A user's edit should be represented as operations against source media.

This enables:

* undo
* redo
* alternate edits
* AI changes
* templates
* versioning

---

# 15. COMMAND-BASED EDITING ENGINE

Every edit should be represented as a command.

Examples:

AddClipCommand

RemoveClipCommand

TrimClipCommand

MoveClipCommand

SplitClipCommand

AddTextCommand

AddGraphicCommand

AddDiagramCommand

ChangeCaptionStyleCommand

ChangeTransformCommand

ApplyTemplateCommand

Commands must support:

execute()

undo()

redo()

serialize()

This becomes extremely important for AI.

If the AI says:

"Remove the 2.4 seconds of silence at 00:41"

it should produce a normal AVID command.

AI must not mutate internal state directly.

---

# 16. AI ARCHITECTURE

Create an AI Runtime.

Example:

AI Runtime
↓
Capability Router
↓
Provider Registry
↓
Provider Adapter
↓
Model

Capabilities:

* text reasoning
* structured output
* vision
* audio understanding
* transcription
* image generation
* video generation
* embeddings
* tool calling

A provider may support some capabilities and not others.

AVID must detect this.

---

# 17. MODEL PROVIDERS

Provider registry.

Example:

providers/
ollama
openai
anthropic
gemini
openai-compatible
custom

Do not assume every model supports every feature.

For example:

A local text model may be able to:

* understand transcript
* generate edit plan

but not:

* inspect video frames directly
* generate images
* generate video

The UI must communicate capability limitations.

---

# 18. LOCAL AI

Local AI is a first-class feature.

Settings:

AI Providers

Local

Ollama
Connected
Models
Refresh

Custom Local Endpoint
Base URL
API format

Cloud

OpenAI
Anthropic
Gemini
etc.

The application should never secretly send media to cloud providers.

If cloud processing is required:

Show it clearly.

Example:

"Visual analysis requires sending selected frames to Gemini."

Buttons:

[Allow Once]

[Always Allow]

[Cancel]

---

# 19. AI PRIVACY

The privacy model must be understandable.

For every AI operation show:

Processing:

Local

or

Cloud: Anthropic

or

Cloud: Google Gemini

Do not hide this in advanced settings.

---

# 20. TRANSCRIPTION

Transcription should support:

* local Whisper
* timestamps
* word-level timestamps where available
* speaker labels
* confidence where available
* language
* punctuation

Transcript must map back to timeline positions.

This enables text-based editing.

Premiere's current text-based editing workflow demonstrates the value of treating transcript text as an editing surface rather than merely a subtitle output.

AVID should go further.

---

# 21. TRANSCRIPT EDITING

User can:

Select sentence

→ Delete

AVID removes corresponding video/audio.

Select paragraph

→ Keep

AVID creates a clip.

Search:

"Kafka"

AVID jumps to every occurrence.

Select:

"Everything about partitions"

AVID finds relevant transcript sections.

---

# 22. AI ROUGH CUT

AVID should detect:

* long pauses
* filler words
* repeated phrases
* false starts
* obvious mistakes
* dead sections
* duplicated statements
* low-confidence speech
* topic changes

Never automatically destroy footage.

Generate a proposed edit.

Example:

"AVID found 3m 42s of potentially removable material."

[Review]

---

# 23. AI EDIT REVIEW

The user should see:

REMOVE

00:12.4 → 00:16.8

Reason:

Long pause

Confidence:

96%

Buttons:

[Accept]

[Reject]

[Edit]

This creates trust.

---

# 24. AI DIRECTOR

The Director is AVID's primary AI interaction model.

User:

"Turn this into a 7-minute technical explainer."

Director:

1. analyzes transcript
2. identifies sections
3. creates rough structure
4. proposes cuts
5. identifies visual opportunities
6. proposes diagrams
7. creates captions
8. proposes B-roll
9. prepares timeline changes

But the Director must return structured operations.

Not arbitrary code.

---

# 25. EDIT PLAN

Define an Edit Plan schema.

Example:

{
"version": 1,
"goal": "technical_explainer",
"operations": [
{
"type": "remove_range",
"start": 42.2,
"end": 47.1,
"reason": "repetition"
},
{
"type": "add_visual",
"visualType": "diagram",
"start": 84.2,
"duration": 8,
"concept": "kafka_partitions"
}
]
}

Validate the schema before execution.

Invalid AI output must never mutate the timeline.

---

# 26. VISUAL INTELLIGENCE

This is one of AVID's primary differentiators.

AVID should identify statements that benefit from visual support.

Example:

User says:

"Kafka has three partitions and each consumer in a group processes different partitions."

AVID detects:

Concept:

Kafka consumer groups

Visual opportunity:

Architecture diagram

Recommended visual:

Producer
→ Kafka
→ Partition 1
→ Partition 2
→ Partition 3
→ Consumer Group

The system creates the visual.

---

# 27. VISUAL TYPES

AVID should support:

### Text

* titles
* subtitles
* labels
* definitions
* emphasis
* quotes

### Diagrams

* architecture
* flowcharts
* pipelines
* trees
* graphs
* sequences
* timelines
* comparisons

### Code

* syntax-highlighted code
* line highlighting
* terminal output
* diff views

### Data

* bar charts
* line charts
* pie charts where appropriate
* tables
* counters

### Branding

* logos
* icons
* brand colors

### Motion graphics

* arrows
* highlights
* callouts
* zooms
* animated paths
* transitions

### Generated imagery

Optional.

Must remain separate from deterministic diagrams.

---

# 28. DETERMINISTIC VISUAL ENGINE

Do not use image generation for everything.

For technical content:

Prefer structured graphics.

A diagram should be editable.

Example:

User:

"Move the database to the right."

AVID should modify the diagram.

It should not regenerate an image.

---

# 29. VISUAL SCENE SPEC

Create a visual scene specification.

Example:

{
"scene": {
"width": 1920,
"height": 1080,
"duration": 8
},
"elements": [
{
"type": "node",
"id": "producer",
"x": 200,
"y": 450,
"label": "Producer"
},
{
"type": "node",
"id": "kafka",
"x": 800,
"y": 450,
"label": "Kafka"
}
],
"connections": [
{
"from": "producer",
"to": "kafka",
"animation": "flow"
}
]
}

The renderer turns this into actual graphics.

---

# 30. VISUAL EDITING

User must be able to click a generated visual.

Then:

Edit visual

Change:

* text
* colors
* positions
* size
* animation
* duration
* icons
* layout

The AI can modify the scene specification.

---

# 31. TEMPLATE SYSTEM

Templates are a major feature.

Templates should not merely be visual presets.

Templates should encode editing behavior.

Example:

Technical Explainer Template:

* talking head
* automatic captions
* concept detection
* diagram insertion
* code highlighting
* zoom transitions
* chapter cards

---

# 32. TEMPLATE FORMAT

Templates should be data-driven.

Example:

templates/
technical-explainer/
template.json
preview.png
assets/
scenes/
rules/

Template metadata:

* name
* author
* version
* description
* category
* aspect ratios
* supported features
* required assets

---

# 33. INITIAL TEMPLATES

Build at least:

1. Technical Explainer
2. YouTube Talking Head
3. Podcast
4. Podcast Short
5. Educational Lesson
6. Product Demo
7. Documentary
8. Social Short
9. Code Tutorial
10. Architecture Breakdown
11. Product Launch
12. Minimal Caption Video

---

# 34. TEMPLATE EXPERIENCE

Home screen:

Templates

Search:

"technical"

Results:

Technical Explainer

Preview

[Use Template]

Do not immediately destroy the user's creative control.

After selecting:

"Use this template as a starting point"

---

# 35. HOME SCREEN

Home should feel like a real product.

Sections:

Recent Projects

Create New

Templates

Examples

Getting Started

Local AI Status

---

# 36. FIRST-RUN EXPERIENCE

First launch:

Welcome to AVID.

"Where should your AI run?"

Options:

Local

Cloud

Both

Then:

"Choose your editing experience."

Simple

or

Professional

Do not force configuration before allowing the user to explore.

---

# 37. PROJECT CREATION

New Project modal:

Project name

Canvas:

16:9
9:16
1:1
4:5
Custom

Frame rate:

24
25
30
50
60

Resolution:

720p
1080p
4K

[Create Project]

Optional:

Start from template.

---

# 38. MAIN EDITOR UI

Primary layout:

---

## Top bar

## Media | Preview | Inspector

```
          Preview
```

---

## Timeline

Left:

Media / transcript / scenes / AI

Center:

Video preview

Right:

Inspector

Bottom:

Timeline

---

# 39. TOP BAR

Include:

Project name

Undo

Redo

Save status

AI status

Preview quality

Export

Settings

---

# 40. LEFT SIDEBAR

Tabs:

Media

Transcript

Scenes

AI

Templates

Assets

Audio

Captions

---

# 41. MEDIA PANEL

Show:

* imported clips
* images
* audio
* generated assets
* folders
* search
* metadata

Thumbnail grid.

Support list view.

---

# 42. TRANSCRIPT PANEL

Show:

Speaker

Timestamp

Text

Search

Highlight

Select

Edit

AI actions

Example:

"Remove this sentence"

"Create clip"

"Add visual"

---

# 43. AI PANEL

The AI panel should not be a generic chatbot.

It is an editor command center.

Example:

"What would you like to change?"

Suggestions:

* Remove pauses
* Find best moments
* Make this shorter
* Add visuals
* Add captions
* Create Short
* Improve pacing

Then conversational commands.

---

# 44. CONTEXT-AWARE AI

If user selects a clip:

"Make this more dynamic."

AI acts on the selected clip.

If user selects a visual:

"Simplify this diagram."

AI acts on that visual.

If user selects the timeline:

"Turn this into a 60-second Short."

AI acts on the selected range.

Context matters.

---

# 45. TIMELINE UX

Timeline must support:

* zoom
* horizontal scrolling
* track height
* snapping
* split
* trim
* ripple delete
* overwrite
* insert
* magnetic behavior optionally
* markers
* selection
* multi-select
* keyboard shortcuts

Use established editor conventions where they reduce friction.

Do not invent unfamiliar behavior merely to be different.

DaVinci Resolve's emphasis on action-oriented editing and fast review is an important UX reference.

---

# 46. KEYBOARD SHORTCUTS

Provide standard shortcuts.

Examples:

Space

Play/pause

J

Reverse

K

Pause

L

Forward

I

Mark in

O

Mark out

S

Split

Delete

Delete selected

Cmd/Ctrl+Z

Undo

Cmd/Ctrl+Shift+Z

Redo

Cmd/Ctrl+S

Save

E

Export

Shortcuts must be customizable later.

---

# 47. PREVIEW

Preview should be fast.

Do not render the entire video for every timeline change.

Use:

* proxies
* cached frames
* low-resolution preview
* incremental rendering

Preview quality options:

1/4

1/2

Full

Automatic

---

# 48. PROXY WORKFLOW

Large media should automatically offer proxies.

Example:

4K footage

AVID:

"Large media detected."

"Generate proxies for smoother editing?"

[Generate]

[Not Now]

Users can disable automatic proxy generation.

---

# 49. BACKGROUND JOB SYSTEM

Long operations must run asynchronously.

Examples:

* transcription
* proxy generation
* scene detection
* waveform generation
* AI analysis
* rendering
* export

Do not freeze the UI.

Global job indicator:

Processing 3 tasks

---

# 50. JOB CENTER

Users can open:

Jobs

Example:

Transcribing interview.mp4

62%

Generating thumbnails

Complete

Rendering preview

Queued

Users can:

* cancel
* retry
* inspect errors

---

# 51. ERROR UX

Never show:

"Error: subprocess exited 1"

Show:

"AVID couldn't create the proxy."

Possible reason:

FFmpeg could not decode this file.

Try:

* Retry
* Use original media
* Convert media
* View technical details

Technical details should be expandable.

---

# 52. AUTOSAVE

Autosave frequently.

Project recovery.

Crash recovery.

If AVID crashes:

"AVID recovered your project from 2 minutes ago."

Never lose work.

---

# 53. VERSION HISTORY

Eventually support project snapshots.

Examples:

Version 12

"Before AI rough cut"

Version 13

"AI rough cut"

Version 14

"Added Kafka diagram"

This is particularly important because AI operations can make large changes.

---

# 54. AI CHANGE PREVIEW

Never allow a large AI modification to silently alter the entire timeline.

Before:

12:43

After:

9:31

Changes:

* 14 cuts
* 3 visual insertions
* 2 caption changes

[Review Changes]

[Apply]

[Cancel]

---

# 55. AI DIFF

Create an editing diff.

Example:

AI proposes:

REMOVE

02:14 → 02:19

ADD DIAGRAM

03:42 → 03:50

CHANGE CAPTION STYLE

04:12 → 04:25

The user can accept/reject each operation.

---

# 56. EXAMPLES GALLERY

Ship AVID with example projects.

Examples should demonstrate:

* technical explainer
* podcast
* tutorial
* short
* product demo

The user can open an example and inspect the timeline.

This teaches the product.

---

# 57. EMPTY STATES

Never show empty screens with no guidance.

Example:

No media yet.

"Drop video here or import media."

Buttons:

[Import Media]

[Try an Example]

[Explore Templates]

---

# 58. AI EXAMPLES

The AI panel should show examples based on context.

For a talking head:

"Remove filler words"

"Create a Short"

"Add captions"

For technical content:

"Explain this visually"

"Create architecture diagram"

"Highlight the code"

For podcast:

"Find the strongest moments"

"Create 3 Shorts"

---

# 59. ACCESSIBILITY

Support:

* keyboard navigation
* screen reader labels
* high contrast
* scalable UI
* reduced motion
* captions
* visible focus states

Do not rely on color alone.

---

# 60. DESIGN LANGUAGE

Visual identity:

Dark-first.

Professional.

Calm.

Minimal.

Not overly futuristic.

Avoid:

* excessive gradients
* glowing AI blobs
* pointless glassmorphism
* giant chat UI
* excessive animations

The editor should look like a serious creative tool.

---

# 61. DESIGN SYSTEM

Define:

Colors

Typography

Spacing

Radius

Borders

Shadows

Icons

Buttons

Inputs

Panels

Menus

Dialogs

Toasts

Tooltips

Timeline components

Inspector components

AI components

Use a single design system.

---

# 62. RENDERING ARCHITECTURE

Rendering pipeline:

Project

↓

Timeline Compiler

↓

Render Graph

↓

Media Inputs

↓

Transforms

↓

Effects

↓

Graphics

↓

Audio Mix

↓

Captions

↓

FFmpeg / Renderer

↓

Output

The timeline must compile into a deterministic render representation.

---

# 63. RENDER GRAPH

Create a render graph abstraction.

Example:

SourceClip

→ Trim

→ Transform

→ Color

→ Overlay

→ Caption

→ Output

This will allow future rendering engines without rewriting the editor.

---

# 64. EXPORT

Initial formats:

MP4

H.264

AAC

Presets:

YouTube 1080p

YouTube 4K

Instagram Reel

TikTok

YouTube Short

Instagram Feed

Custom

---

# 65. EXPORT UX

Export dialog:

Preview

Format

Resolution

FPS

Quality

Estimated file size

Estimated render time

Output path

[Export]

After export:

[Open File]

[Show in Folder]

[Create Short]

[Share]

Do not upload anything automatically.

---

# 66. AUDIO

Initial support:

* volume
* mute
* fade
* normalize
* basic noise reduction
* ducking
* music
* voice isolation where available

AI commands:

"Make my voice clearer."

"Lower the music when I speak."

"Remove this background noise."

---

# 67. CAPTIONS

Support:

* automatic captions
* speaker labels
* caption styles
* word highlighting
* position
* font
* size
* background
* animation

Caption styles should be templates.

---

# 68. SMART REFRAMING

For 16:9 → 9:16:

detect:

* face
* speaker
* screen region
* important object

Generate keyframes.

User can manually correct them.

Never lock users into AI decisions.

---

# 69. MULTI-CAMERA

Future phase.

Support:

* synchronized tracks
* audio sync
* camera angle detection
* speaker detection
* AI camera selection

User:

"Use the close-up whenever the speaker makes an important point."

AVID creates proposed cuts.

---

# 70. B-ROLL

Do not automatically insert random stock footage.

B-roll should have a reason.

Example:

Transcript:

"PostgreSQL uses a write-ahead log..."

AVID:

Visual opportunity:

PostgreSQL architecture

Options:

[Generate diagram]

[Search local assets]

[Search stock provider]

[Skip]

Cloud asset searches must be explicit.

---

# 71. LOGOS

AVID should have an asset library.

Users can import:

* logos
* SVGs
* icons
* fonts
* brand colors

AI can recommend a logo already present in the project.

Do not automatically fetch copyrighted logos from the web.

---

# 72. CODE VISUALIZATION

This should be a first-class feature.

User imports:

* source code
* screen recording
* repository

AVID can create:

* syntax-highlighted snippets
* line highlights
* terminal animations
* diff views
* architecture diagrams

Example:

"Show the function I'm talking about."

AVID finds the relevant code and creates a visual.

---

# 73. TECHNICAL CREATOR MODE

Eventually introduce a specialized mode.

Technical Creator

Capabilities:

* code visualization
* architecture diagrams
* terminal capture
* Git diffs
* API diagrams
* database diagrams
* infrastructure diagrams
* Mermaid rendering
* syntax highlighting

This could become AVID's initial wedge.

---

# 74. DIAGRAM ENGINE

Support deterministic diagram rendering.

Potential inputs:

* AVID Scene Spec
* Mermaid
* SVG
* custom nodes

A Mermaid diagram can be converted into an editable visual representation where practical.

---

# 75. AI TOOLING

The AI should have tools such as:

get_project

get_selected_clip

get_timeline

get_transcript

search_transcript

search_media

get_scene

get_audio_analysis

get_project_style

create_edit_plan

create_visual_scene

apply_edit_operations

preview_changes

render_preview

Tools should be permissioned.

---

# 76. AI SAFETY

AI must not:

* delete source media
* overwrite original files
* upload media without consent
* expose API keys
* execute arbitrary shell commands from model output
* modify unrelated files
* silently change project settings

AI operations must go through controlled interfaces.

---

# 77. PROVIDER FAILOVER

If a cloud model fails:

Do not automatically send the data to another provider.

Tell the user.

Option:

"Claude failed. Try another configured provider?"

User chooses.

---

# 78. MODEL ROUTING

Eventually:

Simple task:

Local small model.

Complex reasoning:

Claude/Gemini/OpenAI.

Transcription:

Local Whisper.

Diagram:

Local structured model.

Image generation:

Configured image provider.

This is model orchestration.

---

# 79. MODEL CAPABILITY REGISTRY

Each model declares:

* text
* vision
* audio
* structured output
* tool calling
* image generation
* video generation
* context window
* local/cloud
* latency estimate

The UI can then intelligently recommend models.

---

# 80. AI SETTINGS

Users should see:

Default AI

Visual AI

Transcript AI

Image generation

Cloud permissions

Local model directory

Temperature where appropriate

Maximum processing budget

Never make advanced settings necessary.

---

# 81. MCP

Design an AVID MCP server eventually.

Expose project capabilities:

* inspect project
* inspect timeline
* inspect transcript
* add clip
* create visual
* create caption
* export
* render preview

This would allow external agents to interact with AVID.

For example:

OpenCode

→ AVID MCP

→ "Create a technical explainer from this recording."

This aligns naturally with AVID's model-agnostic philosophy.

---

# 82. OPEN SOURCE STRATEGY

Keep core functionality open source if that remains the project's direction.

Potential structure:

avid/
apps/
crates/
packages/
templates/
examples/
docs/
.opencode/
scripts/

---

# 83. DOCUMENTATION

Create:

README.md

ARCHITECTURE.md

CONTRIBUTING.md

DEVELOPMENT.md

AI.md

TIMELINE.md

RENDERING.md

PROJECT_FORMAT.md

PROVIDER_SYSTEM.md

TEMPLATES.md

UX.md

TESTING.md

SECURITY.md

ROADMAP.md

DECISIONS.md

---

# 84. AGENTS.MD

Create a detailed root AGENTS.md.

It must contain:

* project purpose
* architecture
* coding conventions
* commands
* testing requirements
* design principles
* AI rules
* security rules
* media rules
* performance requirements
* current roadmap
* forbidden shortcuts

Keep it updated throughout development.

OpenCode explicitly supports project-level `AGENTS.md` initialization and recommends committing it with the project, so use it as the persistent context layer for the implementation.

---

# 85. OPENCODE AGENTS

Create specialized agents.

## researcher

Researches:

* competing editors
* libraries
* technical constraints
* APIs
* UX patterns

No code changes.

## architect

Reviews:

* architecture
* dependencies
* boundaries
* ADRs

## frontend

Owns React/UI.

## rust

Owns Rust core.

## media

Owns FFmpeg/media.

## ai

Owns AI runtime.

## ux

Reviews user flows and usability.

## qa

Runs tests and investigates failures.

## performance

Finds bottlenecks.

## security

Reviews:

* filesystem
* process execution
* provider credentials
* cloud uploads
* malicious media

## reviewer

Reviews implementation quality.

OpenCode supports project-scoped custom agents and subagents, including permissions and model selection, so configure these rather than expecting one general-purpose agent to handle every concern.

---

# 86. OPENCODE COMMANDS

Create commands such as:

/research

/research-ux

/plan

/test

/test-ui

/test-rust

/test-media

/review

/security-review

/performance-review

/build-release

/update-docs

/check-architecture

/verify-feature

OpenCode supports custom commands through project command files, making these repeatable workflows appropriate for this project.

---

# 87. DEVELOPMENT PROCESS

Never work as:

"Build everything."

Instead:

Research

↓

Plan

↓

Design

↓

Implement

↓

Test

↓

Review

↓

Document

↓

Verify

↓

Next feature

---

# 88. RESEARCH-FIRST RULE

Before implementing a significant subsystem:

Research existing solutions.

For example:

Timeline:

research professional editor timeline behavior.

Transcription:

research Whisper and alternatives.

Rendering:

research FFmpeg limitations.

Desktop:

research Tauri limitations.

AI:

research current provider APIs.

Do not blindly reinvent existing infrastructure.

---

# 89. DEPENDENCY POLICY

Before adding a dependency:

Research:

* maintenance
* license
* platform support
* bundle size
* security
* activity
* alternatives

Document why it was chosen.

Do not install libraries simply because they are popular.

---

# 90. ADR SYSTEM

Create:

docs/decisions/

Every important architectural decision gets an ADR.

Example:

ADR-001 Tauri

ADR-002 Rust Media Core

ADR-003 Project Format

ADR-004 Timeline Model

ADR-005 AI Provider Abstraction

ADR-006 Rendering Architecture

ADR-007 Local-first AI

ADR-008 Command-based Editing

---

# 91. TESTING PHILOSOPHY

Testing is part of implementation.

Not a final phase.

Every feature requires:

Unit tests

Integration tests

UI tests where applicable

End-to-end tests where applicable

Regression tests

---

# 92. RUST TESTING

Test:

* timeline math
* trimming
* splitting
* command execution
* undo
* redo
* serialization
* project migration
* render graph
* media metadata
* path handling

Property tests where useful.

---

# 93. TYPESCRIPT TESTING

Test:

* state transitions
* commands
* AI operation parsing
* UI components
* template loading
* provider registry
* project loading
* error handling

---

# 94. E2E TESTS

Build real workflows.

Example:

Create project

→ import sample video

→ transcribe

→ create timeline

→ trim

→ add caption

→ add visual

→ export

→ verify output exists

---

# 95. MEDIA TEST FIXTURES

Create a small fixture library.

Examples:

* 10-second talking head
* silence
* multiple speakers
* 4K clip
* vertical video
* audio-only
* image
* corrupted media
* unusual codec

Tests should use deterministic fixtures.

---

# 96. RENDER TESTS

For rendering:

Generate known output.

Verify:

* file exists
* codec
* duration
* resolution
* audio stream
* video stream

Where practical, compare rendered frames or perceptual hashes.

Avoid brittle exact byte comparisons because encoders may differ.

---

# 97. AI TESTING

Do not test AI by expecting identical prose.

Test structured output.

Example:

Input:

"Remove the silence from 10 to 15 seconds."

Expected operation:

remove_range

start:

10

end:

15

Validate schema.

---

# 98. AI FAILURE TESTS

Test:

* malformed JSON
* missing fields
* hallucinated clip IDs
* invalid timestamps
* overlapping operations
* nonexistent media
* unsupported capability

AVID must reject invalid operations safely.

---

# 99. AI EVALUATION DATASET

Create fixtures:

prompts/

```
remove_pause.json

create_short.json

explain_kafka.json

add_captions.json

shorten_video.json

create_diagram.json
```

Expected structured outputs.

Run evaluation across providers.

---

# 100. OFFLINE MODE TESTING

Disable network.

AVID must still:

* open projects
* edit
* save
* render
* use configured local models
* manage media

Cloud features should clearly report unavailable.

---

# 101. NETWORK TESTING

Simulate:

* timeout
* rate limit
* disconnect
* invalid credentials
* provider outage

UI must recover gracefully.

---

# 102. CRASH TESTING

Simulate:

* renderer crash
* FFmpeg failure
* AI process crash
* application crash
* power interruption where feasible

Project recovery must work.

---

# 103. PERFORMANCE TARGETS

Initial goals:

Application launches quickly.

UI remains responsive during background work.

Timeline scrolling should remain smooth on reasonable projects.

Do not perform expensive media processing on the UI thread.

Memory usage must be monitored.

Large projects must not require loading every frame into memory.

---

# 104. LARGE PROJECT TEST

Create a stress project containing:

* hundreds of clips
* multiple tracks
* long transcript
* many captions
* many graphics

Measure:

* load time
* timeline interaction
* memory
* export
* autosave

---

# 105. SECURITY

Threat model:

Local media may be malicious.

Project files may be malicious.

AI output may be malicious.

Third-party assets may be malicious.

Provider credentials are sensitive.

Never execute arbitrary AI-generated shell commands.

Sandbox external processes where practical.

Validate paths.

Prevent path traversal.

Never expose secrets to frontend logs.

---

# 106. FILESYSTEM SAFETY

Never allow an AI operation to write outside authorized project directories without explicit user action.

Normalize paths.

Reject traversal.

Use allowlists.

---

# 107. LOGGING

Logs should be useful.

Levels:

DEBUG

INFO

WARN

ERROR

NEVER log:

API keys

tokens

full private media contents

private transcript contents unnecessarily

---

# 108. TELEMETRY

Default to no invasive telemetry.

If analytics are ever introduced:

* explicit
* documented
* privacy-respecting
* ideally opt-in

Local-first must mean something.

---

# 109. ACCESSIBILITY TESTING

Test:

* keyboard-only editing
* screen reader navigation
* focus order
* reduced motion
* high contrast

---

# 110. INTERNATIONALIZATION

Architect the UI so text can eventually be translated.

Do not hard-code user-facing strings everywhere.

---

# 111. ERROR CODES

Define structured errors.

Example:

AVID_MEDIA_001

Unsupported media

AVID_AI_001

Provider unavailable

AVID_RENDER_001

Render failed

AVID_PROJECT_001

Project migration failed

---

# 112. CLI

Eventually provide:

avid open project.avid

avid render project.avid

avid transcribe video.mp4

avid export project.avid

This should come after the core engine is stable.

---

# 113. PROJECT FILE PORTABILITY

A project created on:

macOS

must open on:

Windows

Linux

where media paths can be resolved.

Use relative project paths whenever possible.

If media is missing:

show:

"Media missing"

[Locate]

[Relink Folder]

---

# 114. MEDIA RELINKING

If files move:

AVID should detect:

filename

duration

size

hash

metadata

and suggest matches.

---

# 115. UNDO/REDO

Undo must be reliable.

Every meaningful edit must be undoable.

AI operations should be grouped.

Example:

Undo:

"AI: Create Technical Explainer"

instead of requiring the user to undo 37 individual operations.

---

# 116. TRANSACTIONAL AI OPERATIONS

AI-generated changes should execute transactionally.

If operation 8 of 12 fails:

rollback the entire AI operation.

Do not leave the project half-modified.

---

# 117. SAVE STATES

Before major AI transformations:

create a lightweight snapshot.

This provides another recovery layer.

---

# 118. USER TRUST

AI should explain what it did.

Example:

"Removed 8 pauses."

"Added 3 diagrams."

"Created 12 caption segments."

"Changed aspect ratio to 9:16."

Do not say:

"Done."

---

# 119. AI CONFIDENCE

Where meaningful:

show confidence.

But do not show fake precision.

Use:

High confidence

Medium confidence

Needs review

rather than:

97.234%.

---

# 120. EXPLAINABILITY

For important AI changes:

"Why?"

Possible response:

"AVID removed this segment because it repeated the previous sentence."

---

# 121. EXAMPLE USER SESSION

User imports a 25-minute recording.

AVID analyzes it.

Shows:

Transcript ready.

18 scenes detected.

3 speakers detected.

11 potential cuts.

7 visual opportunities.

User:

"Make this a technical YouTube video."

AVID proposes:

8:42 final duration.

Then:

* removes repeated sections
* cleans pauses
* adds intro
* creates chapter cards
* adds captions
* identifies 5 concepts
* creates 5 diagrams

User reviews.

User selects the Kafka section.

"Make this visual."

AVID creates:

Producer

↓

Kafka

↓

Partitions

↓

Consumer Group

User says:

"Make the diagram simpler."

AVID modifies the diagram.

User says:

"Create a 60-second Short from this section."

AVID creates a new sequence.

User exports:

YouTube 16:9

and

Short 9:16.

This is the experience the product should ultimately achieve.

---

# 122. INITIAL MVP

Do not attempt the entire vision immediately.

MVP must include:

1. Desktop application
2. Project creation
3. Media import
4. Media preview
5. Timeline
6. Basic editing
7. Undo/redo
8. Autosave
9. FFmpeg rendering
10. Export
11. Local transcription
12. Transcript editing
13. Basic AI provider abstraction
14. Ollama support
15. One cloud provider
16. AI rough-cut proposal
17. Captions
18. Text overlays
19. Basic diagrams
20. Templates
21. Example projects
22. Crash recovery
23. Documentation
24. Tests

---

# 123. MVP EXPLICITLY EXCLUDES

Do not initially build:

* full color grading suite
* advanced VFX
* 3D rendering
* collaborative cloud editing
* stock marketplace
* social network
* cloud storage platform
* complex video generation
* advanced multicam
* mobile application

These can come later.

---

# 124. MVP PHASES

## Phase 0

Research and architecture.

Deliver:

* competitive analysis
* architecture
* ADRs
* AGENTS.md
* project structure
* UX flows
* technical risks

No premature implementation.

---

## Phase 1

Desktop shell.

Deliver:

* Tauri
* React
* Rust
* design system
* routing/navigation
* project creation

---

## Phase 2

Media foundation.

Deliver:

* import
* probe
* thumbnails
* waveform
* playback
* media library

---

## Phase 3

Timeline.

Deliver:

* tracks
* clips
* trim
* split
* move
* delete
* snapping
* undo/redo

---

## Phase 4

Rendering.

Deliver:

* render graph
* FFmpeg
* preview
* export
* presets

---

## Phase 5

Transcription.

Deliver:

* Whisper integration
* transcript
* timestamps
* transcript/timeline synchronization

---

## Phase 6

AI Runtime.

Deliver:

* provider abstraction
* Ollama
* cloud provider
* model registry
* capability detection

---

## Phase 7

AI Editing.

Deliver:

* rough cut
* silence detection
* filler removal
* transcript commands
* AI edit plans
* diff/review

---

## Phase 8

Visual Intelligence.

Deliver:

* concept detection
* visual suggestions
* scene spec
* diagram renderer
* editable diagrams

---

## Phase 9

Templates.

Deliver:

* template system
* initial templates
* template browser
* template preview
* template application

---

## Phase 10

Polish.

Deliver:

* accessibility
* performance
* recovery
* error UX
* onboarding
* documentation
* examples

---

# 125. POST-MVP

After MVP:

* smart reframing
* advanced captions
* code visualization
* Mermaid
* image generation
* B-roll
* multi-camera
* audio enhancement
* automatic Shorts
* plugin system
* MCP server
* CLI
* template marketplace
* community templates

---

# 126. FUTURE AVID AGENT

Eventually the AI should be able to operate AVID almost like an editor.

User:

"Make this feel like a polished Fireship-style technical explainer."

AVID should:

* analyze footage
* choose pacing
* create graphics
* identify visual opportunities
* add captions
* create transitions
* produce chapters

But always produce editable work.

Do not imitate a specific creator's copyrighted style unless the user has rights/authorization.

Use generic attributes such as:

"fast-paced technical explainer"

---

# 127. TEMPLATE AUTHORING

Eventually users can say:

"Save this as a template."

AVID extracts:

* typography
* colors
* caption style
* transitions
* visual layouts
* timeline structure
* AI rules

Then:

"Technical Explainer v2"

---

# 128. PLUGIN SYSTEM

Future plugins:

* transcription providers
* AI providers
* stock providers
* music libraries
* diagram engines
* render engines
* export targets

Plugin APIs must be versioned.

---

# 129. OPEN FORMAT

Where practical, favor:

* JSON
* SVG
* WebVTT
* SRT
* standard media codecs
* Mermaid
* common project formats

Do not trap users unnecessarily.

---

# 130. RESEARCH REQUIREMENT

For every major subsystem, research at least three existing approaches.

Document:

Option

Advantages

Disadvantages

Decision

Reason

Example:

Timeline engine:

A

B

C

Decision:

why AVID chooses its approach.

Research must be recorded in docs/research/.

---

# 131. COMPETITOR RESEARCH

Continuously study:

* DaVinci Resolve
* Adobe Premiere Pro
* Final Cut Pro
* CapCut
* Descript
* Runway
* VEED
* Screen Studio
* Riverside
* OpusClip
* other emerging AI editors

Do not copy their branding.

Study their workflows.

Identify:

* what users like
* what users hate
* where users lose time
* where AVID can be different

---

# 132. USER RESEARCH

Eventually create:

docs/research/user-interviews/

Record anonymized findings.

Questions:

"What takes longest?"

"What do you hate about editing?"

"What do you do repeatedly?"

"What do you wish AI could do?"

"Would you trust AI to modify your timeline?"

"What must remain under your control?"

Use findings to prioritize features.

---

# 133. PRODUCT METRICS

Do not optimize solely for downloads.

Useful metrics:

Time to first successful export

Time from import → rough cut

Time saved through AI

AI suggestion acceptance rate

Undo rate after AI actions

Export success rate

Crash-free sessions

Project recovery success

Average project completion

But privacy-first.

Do not collect these automatically without an appropriate privacy model.

---

# 134. QUALITY BAR

A feature is not complete because:

* the code compiles
* the UI exists
* a button works

A feature is complete when:

1. UX is understandable
2. happy path works
3. errors are handled
4. state is persisted
5. undo works
6. tests exist
7. documentation exists
8. accessibility is considered
9. performance is acceptable
10. recovery is considered

---

# 135. DEFINITION OF DONE

Every feature must satisfy:

[ ] Requirements documented

[ ] UX designed

[ ] Architecture reviewed

[ ] Implementation complete

[ ] Unit tests

[ ] Integration tests

[ ] E2E where appropriate

[ ] Error states

[ ] Loading states

[ ] Empty states

[ ] Undo/redo

[ ] Persistence

[ ] Accessibility

[ ] Performance check

[ ] Security check

[ ] Documentation

[ ] Regression tests

[ ] Manual verification

Only then mark the feature complete.

---

# 136. NO FAKE COMPLETION

Never tell the user:

"Done"

when:

* tests were not run
* build failed
* functionality is mocked
* export does not work
* AI is hard-coded
* provider is fake
* timeline isn't persistent

If something is incomplete, say so in project documentation.

---

# 137. CONTEXT PRESERVATION

AVID is a long-running project.

OpenCode must preserve context through:

AGENTS.md

docs/

ADRs

ROADMAP.md

TODO.md

research/

changelogs

tests

commit history

Do not rely on chat history alone.

At the beginning of each major task:

1. read AGENTS.md
2. read ROADMAP.md
3. inspect current architecture
4. inspect relevant ADRs
5. inspect current tests
6. inspect git status
7. determine current phase

---

# 138. TASK EXECUTION LOOP

For every task:

### 1. Understand

Read existing code.

### 2. Research

Research external libraries and current best practices if necessary.

### 3. Plan

Create a concise implementation plan.

### 4. Implement

Make changes.

### 5. Test

Run relevant tests.

### 6. Review

Use reviewer agent.

### 7. Fix

Resolve findings.

### 8. Verify

Run complete relevant verification.

### 9. Document

Update docs.

### 10. Commit

Use a meaningful commit.

---

# 139. GIT STRATEGY

Small coherent commits.

Examples:

feat(media): add ffmpeg media probing

feat(timeline): add split command

feat(ai): add provider registry

feat(transcript): synchronize transcript selections

fix(render): preserve audio stream

test(timeline): add trim command coverage

Do not create enormous commits covering unrelated systems.

---

# 140. RELEASE PROCESS

Before release:

1. clean install
2. build
3. test
4. package
5. launch packaged application
6. create project
7. import video
8. edit
9. export
10. reopen project
11. verify output

Test on:

macOS

Windows

Linux

where CI/build infrastructure allows.

---

# 141. CI

CI should run:

* TypeScript checks
* lint
* unit tests
* Rust tests
* integration tests
* build checks

Media-dependent tests should use deterministic fixtures.

---

# 142. RELEASE ARTIFACTS

Eventually produce:

macOS

.dmg

Windows

.exe / installer

Linux

.AppImage

.deb

Potentially .rpm

---

# 143. PERFORMANCE BUDGET

Every major feature must answer:

What does this cost?

CPU

GPU

RAM

Disk

Network

Startup time

Timeline responsiveness

Do not introduce heavy dependencies without measuring them.

---

# 144. AI COST CONTROL

Cloud AI should show users when an operation may incur provider costs where the provider charges for usage.

Local processing should clearly identify itself.

Potential setting:

"Prefer local models even if slower."

---

# 145. LOCAL MODEL UX

Example:

Local AI

Ollama

Connected

Model:

qwen

Status:

Ready

Capabilities:

Text
Structured output

Missing:

Vision

Then:

"AVID will use this model for edit planning but will use another configured model for visual analysis."

---

# 146. PROVIDER CONFIGURATION UX

Never require editing JSON for normal users.

UI:

Add Provider

Provider

Model

Endpoint

API Key

Capabilities

Test Connection

Save

Advanced:

Headers

Timeout

Custom model ID

---

# 147. AI MODEL TEST

After adding provider:

[ Test ]

AVID runs:

simple structured-output test

Then:

"Provider ready."

or:

"Provider responded, but does not support structured output required for AI editing."

---

# 148. PRODUCT LANGUAGE

Avoid AI hype.

Do not use:

"Revolutionary"

"Magic"

"10x your creativity"

"Next-generation"

Instead:

"Remove pauses"

"Create a diagram"

"Find the strongest moments"

"Turn this into a Short"

"Generate captions"

The product should explain what it does.

---

# 149. MICROCOPY

Bad:

"AI Magic"

Good:

"Create visual"

Bad:

"Enhance"

Good:

"Clean up audio"

Bad:

"Make it viral"

Good:

"Create a 60-second Short"

---

# 150. USER CONTROL

Every major AI action should have:

Preview

Apply

Undo

Never remove Undo.

---

# 151. AVID'S SIGNATURE FEATURE

The signature workflow should be:

## Explain → Visualize

User speaks.

AVID understands the concept.

AVID identifies the relevant visual.

AVID creates it.

AVID places it at the correct point in the timeline.

The user can edit it.

This is the feature that should make people understand why AVID exists.

---

# 152. EXAMPLE

User says:

"An API gateway sits between clients and our microservices."

AVID creates:

Client

↓
API Gateway

├── Users Service

├── Payments Service

└── Orders Service

Animation:

Client request

→ API Gateway

→ service

→ response

The diagram appears precisely when the user explains it.

That is AVID.

---

# 153. SECOND SIGNATURE FEATURE

## AI-native editable timeline

The AI does not output a flattened video.

It outputs:

clips

cuts

graphics

captions

audio changes

visual scenes

transitions

All remain editable.

---

# 154. THIRD SIGNATURE FEATURE

## Bring Your Own Model

The user owns the intelligence layer.

AVID owns:

* editor
* timeline
* media engine
* visual system
* orchestration
* UX

This protects the product from dependence on one model provider.

OpenCode's current provider architecture is a useful reference for this principle: provider selection and model configuration can remain separate from the application's higher-level workflow.

---

# 155. INITIAL PROJECT DIRECTORY

Use a structure along these lines:

avid/

```
apps/
    desktop/

crates/
    avid-core/
    avid-media/
    avid-timeline/
    avid-render/
    avid-project/
    avid-ai/
    avid-cli/

packages/
    ui/
    design-system/
    ai-protocol/
    templates/
    shared-types/

templates/
    technical-explainer/
    youtube/
    podcast/
    short/
    education/

examples/
    technical-explainer/
    podcast/
    product-demo/

docs/
    architecture/
    research/
    decisions/
    ux/
    testing/

.opencode/
    agents/
    commands/

fixtures/
    media/
    projects/
    ai/

scripts/

tests/

AGENTS.md

README.md

ROADMAP.md

CONTRIBUTING.md

LICENSE
```

---

# 156. INITIAL IMPLEMENTATION ORDER

Do not change this order without an architectural reason.

1. repository
2. AGENTS.md
3. docs
4. Tauri shell
5. React design system
6. Rust core
7. project model
8. media probing
9. media library
10. playback
11. timeline model
12. timeline UI
13. editing commands
14. undo/redo
15. persistence
16. rendering
17. export
18. transcription
19. AI provider system
20. Ollama
21. AI edit plans
22. AI review
23. captions
24. visual scene engine
25. diagrams
26. templates
27. examples
28. polish
29. performance
30. security
31. release packaging

---

# 157. FIRST TASK FOR OPENCODE

Do not start writing application code immediately.

First:

1. inspect the repository
2. determine whether it is empty or partially initialized
3. research current Tauri architecture
4. research current React/Tauri integration
5. research FFmpeg integration approaches
6. research current local AI APIs
7. research Whisper integration
8. research existing timeline/editor libraries
9. research professional editing UX
10. research licensing implications
11. produce architecture proposal
12. produce ADRs
13. create AGENTS.md
14. create ROADMAP.md
15. create initial repository structure
16. create development environment
17. verify clean build

Then begin Phase 1.

Do not skip research simply because the implementation appears obvious.

---

# 158. RESEARCH SOURCES

Prefer primary sources:

* official Tauri documentation
* official FFmpeg documentation
* official Ollama documentation
* official OpenCode documentation
* official Whisper repository/documentation
* official Adobe documentation
* official Blackmagic documentation
* official provider documentation

Use community discussions for UX complaints and real-world pain points, but distinguish anecdotal reports from documented technical facts.

---

# 159. RESEARCH RULE

Current information can become stale.

Before making decisions about:

* model APIs
* provider support
* Tauri capabilities
* FFmpeg behavior
* licensing
* platform support

research the current documentation.

Do not assume an API from memory.

---

# 160. LICENSING

Before integrating:

* codecs
* fonts
* icons
* templates
* stock assets
* models
* model weights
* third-party libraries

verify licensing.

Create:

docs/LEGAL_AND_LICENSING.md

Track:

dependency

license

usage

redistribution restrictions

commercial implications

---

# 161. FINAL PRODUCT EXPERIENCE

A user should be able to install AVID and within minutes:

1. create a project
2. import a video
3. see it in the timeline
4. play it
5. cut it
6. transcribe it
7. edit from transcript
8. ask AI to remove filler
9. review the changes
10. add captions
11. ask AI to visualize a concept
12. receive an editable diagram
13. change the diagram
14. apply a template
15. export the video
16. reopen the project later
17. continue editing

If this workflow does not work reliably, do not move on to advanced AI features.

---

# 162. THE NORTH STAR

AVID should eventually make this possible:

A creator opens a blank project.

They drop in a 30-minute recording.

They say:

"Turn this into a polished 8-minute technical explainer. Keep the important details, remove repetition and dead air, add captions, visualize concepts when I'm explaining them, show relevant code when I mention it, and create a 60-second Short from the strongest section."

AVID analyzes the recording.

It creates an editable proposal.

The creator reviews it.

They change things conversationally.

Every AI decision remains editable.

Their original media remains untouched.

Their footage does not leave their computer unless they explicitly choose a cloud provider.

They export.

That is the product.

Not an AI video generator.

Not a chatbot.

Not a CapCut clone.

A genuine video editor with an AI director, a deterministic visual engine, a model-agnostic AI runtime, and a local-first architecture.

---

# 163. FINAL INSTRUCTION TO OPENCODE

Build AVID as though real users will depend on it.

Prioritize correctness over speed.

Prioritize user experience over technical novelty.

Prioritize editable output over generated output.

Prioritize local-first behavior over cloud dependency.

Prioritize deterministic systems over opaque AI behavior.

Research before architectural decisions.

Test before claiming completion.

Document decisions.

Preserve context.

Never silently compromise user data.

Never fake functionality.

Never mark a feature complete when it is only partially implemented.

When uncertain, investigate.

When a requirement conflicts with the existing architecture, stop and document the conflict before making a destructive decision.

When a dependency is questionable, research alternatives.

When an AI operation can be represented as a normal editing command, represent it as a normal editing command.

When a visual can be generated deterministically, prefer deterministic generation over image generation.

When the user can understand and control an AI action, expose that control.

The end goal is not to produce a technically impressive repository.

The end goal is to produce AVID, a video editor that people genuinely enjoy using.

Build the foundation first.

Build vertically.

Verify every phase.

Then make it exceptional.
