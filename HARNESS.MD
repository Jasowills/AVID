# AVID: Diffusion Studio Fork & Product Transformation

You are working on AVID, an AI-native professional video editor.

AVID stands for:

AI Video Intelligence & Direction

IMPORTANT:

We are NOT building a video editor from scratch.

We are forking Diffusion Studio and using its existing open-source editor as the foundation for AVID.

The goal is to preserve the quality, interaction model, layout, editing workflows, composition system, and professional feel of the existing Diffusion Studio editor, then progressively transform it into AVID.

Do NOT invent a new editor UI.
Do NOT recreate the timeline from scratch.
Do NOT replace working Diffusion functionality with simplified mockups.
Do NOT build a generic AI SaaS dashboard.
Do NOT use the previous AVID "harness" UI concept.

The Diffusion Studio editor is now the visual and architectural baseline.

---

# 1. FIRST PRINCIPLE

Before changing anything, understand the existing Diffusion Studio repository.

You must inspect:

- repository structure
- package structure
- applications
- desktop application
- web editor
- timeline implementation
- composition system
- canvas/viewer
- media system
- asset system
- inspector
- project model
- rendering/encoding
- state management
- keyboard shortcuts
- commands/actions
- agent integration
- MCP integration
- persistence
- routing
- styling/design system
- tests
- build system

Do not begin by redesigning anything.

First produce an architecture map.

Create:

docs/diffusion-architecture.md

Document:

- major packages
- responsibilities
- important components
- state flow
- project flow
- timeline flow
- rendering flow
- asset flow
- agent flow
- desktop/web relationship
- extension points
- areas that should not be modified unnecessarily

Also identify:

- reusable Diffusion functionality
- functionality AVID needs to replace
- functionality AVID needs to extend
- functionality AVID needs to build from scratch

---

# 2. LICENSING

Respect the existing Diffusion Studio license.

The code is open source, but do not assume every asset is licensed identically.

Do not copy proprietary Diffusion branding, logos, icons, trademarks, or protected brand assets into AVID.

Replace the product identity with AVID's own identity.

Preserve required license notices and attribution where required.

Create:

docs/licensing.md

Document what was inherited, what was modified, and what licensing obligations apply.

---

# 3. AVID PRODUCT PHILOSOPHY

AVID is not:

- an AI video generator
- a ChatGPT wrapper
- a CapCut clone
- a generic Electron editor
- an AI SaaS dashboard
- a chatbot with a timeline attached
- a template marketplace

AVID is:

An AI-native professional video editor that understands footage, scripts, compositions and timelines, and lets users direct the editing process using natural language while keeping every result editable.

Core idea:

    Human direction
          ↓
    AI understanding
          ↓
    AI planning
          ↓
    Proposed edit
          ↓
    Editable composition
          ↓
    Professional timeline
          ↓
    Preview
          ↓
    Export

AI must operate on the actual project.

It must not simply generate a video externally and drop an opaque rendered file into the timeline.

---

# 4. PRESERVE THE DIFFUSION EDITOR

The existing Diffusion editor is our baseline.

Preserve its:

- overall layout
- timeline interaction model
- canvas/viewer
- media browser
- inspector
- project navigation
- editing controls
- composition behavior
- keyboard shortcuts
- drag/drop behavior
- resizing
- selection
- snapping
- trimming
- splitting
- playback
- preview
- rendering
- project persistence
- professional density

Do not redesign these systems merely for aesthetic reasons.

If something already works well, keep it.

AVID should feel like a natural evolution of the existing editor, not a completely different application.

---

# 5. VISUAL IDENTITY

Replace Diffusion branding with AVID.

AVID's visual identity should be:

- professional
- monochrome-first
- restrained
- technical
- cinematic
- precise
- dense where appropriate
- highly customizable

Do NOT use:

- purple AI gradients
- excessive glassmorphism
- giant rounded cards
- neon everywhere
- futuristic AI clichés
- robot imagery
- sparkle icons
- generic dashboard cards
- excessive pills
- huge empty spaces

The application should feel like professional creative software.

Think:

professional video editor
+
developer tool
+
AI production system

Color should be themeable.

The default AVID theme should work entirely in grayscale.

Allow users to customize the accent color later through semantic theme tokens.

Never hardcode product-critical colors throughout the application.

---

# 6. AVID APPLICATION STRUCTURE

Do NOT introduce the old AVID harness navigation.

The video editor itself is the primary application.

Use the existing Diffusion editor structure as the foundation.

AVID-specific functionality should integrate naturally into it.

The core workspace should remain centered around:

    Media / Assets
          │
          ▼
    ┌───────────────┐
    │               │
    │    VIEWER     │
    │               │
    └───────────────┘
          │
    ┌───────────────┐
    │   TIMELINE    │
    └───────────────┘

with contextual:

- inspector
- project/media browser
- AI Director
- transcript
- asset intelligence
- composition/source views

---

# 7. AI DIRECTOR

The AI Director is AVID's primary AI interface.

It is NOT a chatbot.

It is a production control system.

Example:

User:

"Make this explanation of Kafka partitions clearer."

AVID should:

1. inspect the transcript
2. understand the relevant section
3. inspect the current composition
4. inspect available assets
5. identify the visual problem
6. create an edit plan
7. show the proposed changes
8. let the user review them
9. apply them
10. preserve undo/redo

Example UI:

AI DIRECTOR

Make this explanation of Kafka partitions clearer.

Analyzed:
✓ Transcript
✓ Current timeline
✓ Existing visual
✓ Available assets

Proposed changes:

+ Add partition diagram          01:42
+ Highlight partition 0          01:45
+ Animate message flow           01:47
+ Remove repeated sentence       01:51

[Review] [Apply]

Do not make the AI panel enormous.

It should feel like a professional production tool.

---

# 8. AI MUST NEVER DIRECTLY MUTATE THE TIMELINE

AI should produce structured operations.

Example:

{
  "version": 1,
  "operations": [
    {
      "type": "remove_range",
      "start": 42.2,
      "end": 47.1,
      "reason": "repetition"
    },
    {
      "type": "add_visual",
      "start": 84.2,
      "duration": 8,
      "visualType": "diagram",
      "concept": "kafka_partitions"
    }
  ]
}

These operations must pass through a validated command layer.

Every AI operation must be:

- inspectable
- validated
- undoable
- redoable
- serializable
- reviewable
- attributable

AI changes should behave like real editor operations.

---

# 9. COMMAND ARCHITECTURE

Where necessary, establish a typed command system.

Examples:

AddClip
RemoveClip
TrimClip
MoveClip
SplitClip
AddText
AddGraphic
AddDiagram
AddCaption
ChangeCaptionStyle
ChangeTransform
ApplyTemplate
ReplaceAsset
CreateScene
DeleteScene
ModifyScene
AddTransition

Every command should support:

- execute
- undo
- redo
- serialization
- validation

AI should invoke these commands rather than manipulating internal state directly.

---

# 10. FOOTAGE INTELLIGENCE

AVID must understand media before asking AI models to edit it.

Build toward:

    Video
      ↓
    Audio extraction
      ↓
    Transcription
      ↓
    Speaker detection
      ↓
    Scene detection
      ↓
    Object/face detection
      ↓
    Semantic understanding
      ↓
    Metadata
      ↓
    Searchable video index

The goal is to let AVID answer things like:

"Find every time I explain Kafka."

"Show me the best take of the introduction."

"Remove the failed attempts."

"Find the part where I talk about database indexes."

"Create a short from the strongest explanation."

"Where did I mention Postgres?"

The footage intelligence layer must be independent from any particular model provider.

---

# 11. TRANSCRIPT-FIRST EDITING

AVID should support editing through the transcript.

Example:

User deletes a paragraph from the transcript.

AVID should identify the corresponding timeline ranges and propose/remove the associated footage.

Transcript editing must remain synchronized with:

- timeline
- captions
- speaker information
- scene information
- AI analysis

---

# 12. SCRIPT → VIDEO

AVID should support:

Start From Script

Workflow:

    Script
      ↓
    Narrative analysis
      ↓
    Storyboard
      ↓
    Visual opportunities
      ↓
    Asset matching
      ↓
    Scene composition
      ↓
    Timeline
      ↓
    Review
      ↓
    Export

The script analyzer should understand:

- hook
- introduction
- claims
- concepts
- examples
- transitions
- visual opportunities
- B-roll opportunities
- diagrams
- screenshots
- code
- charts
- logos
- captions
- pacing

Different scripts should have different editing grammars.

Examples:

- YouTube explainer
- technical tutorial
- product launch
- documentary
- short-form
- educational video
- startup pitch
- product demo
- developer content

Do not force every script into the same template.

---

# 13. ASSET INTELLIGENCE

AVID should understand the user's existing assets.

Analyze:

- video
- photos
- logos
- screenshots
- screen recordings
- illustrations
- audio
- music
- fonts
- PDFs
- slides
- code
- previous projects

When the user provides an appropriate asset, prefer it over inventing a new one.

AVID must never pretend an asset exists.

If something is needed but unavailable, explicitly represent:

Visual needed

Then offer:

- Use my media
- Generate with AI
- Find from connected asset library
- Create motion graphic
- Skip

---

# 14. EXPLAIN → VISUALIZE

This is one of AVID's signature capabilities.

AVID should detect when spoken content would benefit from a visual explanation.

Example narration:

"Kafka divides messages across partitions."

AVID should infer:

Concept:
Kafka partitions

Visual type:
Architecture diagram

Elements:

Producer
Kafka topic
Partition 0
Partition 1
Partition 2
Consumer

Animation:

message → partition → consumer

Then create an editable visual composition.

Prefer deterministic graphics for:

- diagrams
- charts
- architecture
- code
- technical explanations
- UI explanations
- data visualizations

Do not use generative video for everything.

The visual should be editable after generation.

---

# 15. VISUAL SCENE SPEC

Create a structured visual scene representation where necessary.

Example:

{
  "type": "diagram",
  "concept": "kafka_partitions",
  "nodes": [
    {
      "id": "producer",
      "label": "Producer"
    },
    {
      "id": "partition-0",
      "label": "Partition 0"
    }
  ],
  "connections": [
    {
      "from": "producer",
      "to": "partition-0"
    }
  ],
  "animation": {
    "type": "flow"
  }
}

The scene should render into the existing composition/timeline system.

Users must be able to modify it manually.

AI should be able to modify the scene without regenerating the entire video.

Example:

"Make the diagram simpler."

"Remove partition 2."

"Move the consumer below the topic."

"Use my company logo."

---

# 16. MODEL PROVIDER ABSTRACTION

AVID must not be tied to one AI provider.

Support an abstraction layer for:

- OpenAI-compatible APIs
- Anthropic
- Gemini
- Ollama
- local models
- other providers
- future providers

Users should eventually be able to choose models for different jobs.

For example:

Video understanding:
local model

Transcription:
Whisper

Edit planning:
Claude

Diagram generation:
local model

Image generation:
Gemini / OpenAI / local model

Do not hardcode provider-specific logic into the editor.

Create a provider/model abstraction.

---

# 17. LOCAL-FIRST

AVID should work locally wherever practical.

The user's footage should not need to be uploaded to a remote server simply to edit it.

Support local:

- media
- projects
- transcription
- analysis
- rendering
- models where available

Cloud models should be optional.

Make privacy explicit in the UI.

---

# 18. AGENTS AND MCP

AVID should eventually be controllable by coding agents.

Examples:

- OpenCode
- Claude Code
- Codex
- Gemini CLI
- other MCP-compatible agents

Agents should be able to:

- inspect project
- inspect media
- inspect transcript
- inspect scenes
- inspect timeline
- search footage
- propose edits
- modify compositions
- render
- inspect render results

The agent must operate through controlled AVID APIs/commands.

Do not expose unrestricted internal state mutation.

Design the project model and command architecture so an MCP layer can be added cleanly.

---

# 19. SOURCE / COMPOSITION MODEL

Study Diffusion's existing source/composition architecture carefully.

Do not unnecessarily replace it.

Determine whether AVID can extend the existing model to support:

- AI-generated scene structures
- diagrams
- motion graphics
- reusable components
- script-linked scenes
- semantic metadata
- provenance
- agent operations

If the existing model already supports the requirement, use it.

If it doesn't, extend it minimally.

Avoid creating a second competing project model.

There must be one canonical source of truth.

---

# 20. AI PROVENANCE

AI-generated or AI-modified elements should have subtle provenance.

Examples:

- AI generated
- AI suggested
- AI modified

Do not plaster "AI GENERATED" labels everywhere.

The user should be able to understand:

- what AI changed
- why it changed it
- which model performed the operation
- what source information was used

Where useful, expose this through the inspector/history rather than permanent visual clutter.

---

# 21. UNDO / REDO

This is mandatory.

Every AI operation must participate in the normal editor history.

Example:

User:

"Remove all filler words."

AVID performs:

- transcript changes
- timeline edits
- caption updates

Undo should revert the entire coherent operation.

The user should not have to manually undo 37 individual internal mutations.

---

# 22. PERFORMANCE

Do not sacrifice editor performance for AI functionality.

AI analysis should run asynchronously.

Long operations must not freeze the editor.

Use:

- background workers
- caching
- incremental analysis
- proxies
- thumbnails
- waveform caching
- memoization
- virtualization where appropriate

The timeline must remain responsive.

Playback must remain responsive.

Opening a project must remain predictable.

---

# 23. STATES

Every new feature must handle:

- loading
- empty
- error
- success
- disabled
- unavailable
- offline
- cancelled
- partial completion
- retry

Never show fake progress.

Never hide failures.

AI failures should explain:

what failed
why it failed when known
what the user can do next

---

# 24. KEYBOARD-FIRST

Preserve existing professional editor shortcuts.

Add AVID shortcuts where appropriate.

Command palette:

Cmd/Ctrl + K

It should support:

- commands
- projects
- assets
- scenes
- transcript
- AI actions
- models
- settings

Do not turn every action into a visible button.

---

# 25. DESKTOP APPLICATION

The desktop application is the real AVID product.

Support:

- macOS
- Windows
- Linux

Do not make the web application the primary product experience unless the existing architecture requires it.

Preserve native desktop behaviors where appropriate.

Do not assume a particular CPU architecture when implementing downloads/releases.

---

# 26. PROJECT FORMAT

The project must be portable.

It should be possible to:

- save
- reopen
- backup
- inspect
- version
- migrate

Project metadata should be structured and deterministic.

Avoid storing opaque AI state that makes projects impossible to understand or migrate.

---

# 27. DEVELOPMENT PROCESS

Use this workflow for every substantial feature:

RESEARCH
↓
UNDERSTAND EXISTING DIFFUSION IMPLEMENTATION
↓
ARCHITECTURE DECISION
↓
PLAN
↓
IMPLEMENT
↓
TEST
↓
VISUAL VERIFY
↓
REVIEW
↓
FIX
↓
DOCUMENT

Before implementing something, search the repository for existing functionality.

Never duplicate an existing subsystem without a documented reason.

---

# 28. DO NOT MAKE FAKE IMPLEMENTATIONS

Do not create:

- fake timelines
- fake AI responses
- fake generated diagrams
- fake progress
- mock editor functionality presented as real
- placeholder interactions that look complete
- static screenshots pretending to be the application
- hardcoded demo data in production flows

If a feature cannot yet be implemented properly, build the underlying architecture or clearly mark it as incomplete.

Do not claim completion unless the feature actually works.

---

# 29. TESTING

Every substantial change requires appropriate tests.

Use:

- unit tests
- integration tests
- editor interaction tests
- rendering tests
- project persistence tests
- AI command validation tests
- provider tests
- regression tests

For AI-generated operations, test invalid output.

Examples:

- malformed command
- invalid timeline range
- nonexistent asset
- overlapping operations
- missing scene
- unsupported visual type
- unavailable model

The editor must fail safely.

---

# 30. DOCUMENTATION

Maintain:

docs/

At minimum:

docs/diffusion-architecture.md
docs/avid-architecture.md
docs/ai-director.md
docs/project-format.md
docs/ai-protocol.md
docs/model-providers.md
docs/visual-intelligence.md
docs/explain-visualize.md
docs/agent-integration.md
docs/licensing.md

Also maintain:

ROADMAP.md
CHANGELOG.md
AGENTS.md

Update documentation when architecture changes.

---

# 31. ROADMAP

The implementation order should be:

PHASE 0
Diffusion repository audit

PHASE 1
AVID fork builds and runs

PHASE 2
AVID branding and identity

PHASE 3
Preserve and verify the existing editor

PHASE 4
AI provider abstraction

PHASE 5
Transcript and footage intelligence

PHASE 6
AI Director

PHASE 7
Structured AI editing commands

PHASE 8
Script → storyboard → timeline

PHASE 9
Explain → Visualize

PHASE 10
Asset intelligence

PHASE 11
Agent/MCP control

PHASE 12
Local-first model workflows

PHASE 13
Performance and professional polish

Do not jump directly to Phase 9 because it sounds exciting.

The underlying project/editor architecture must be stable first.

---

# 32. FIRST TASK

DO NOT START IMPLEMENTING AVID FEATURES YET.

Your first task is repository reconnaissance.

1. Inspect the entire repository structure.
2. Identify the editor entry points.
3. Identify the desktop entry point.
4. Identify the timeline implementation.
5. Identify the composition model.
6. Identify the project model.
7. Identify the renderer.
8. Identify the media/asset pipeline.
9. Identify state management.
10. Identify existing AI/agent/MCP functionality.
11. Identify styling/design system.
12. Identify tests.
13. Identify build tooling.
14. Identify extension points.
15. Identify licensing boundaries.

Then create:

docs/diffusion-architecture.md

and:

docs/avid-architecture.md

The AVID architecture document should answer:

"What can we inherit?"

"What should we extend?"

"What should we replace?"

"What should we build?"

"What must remain untouched?"

Do not modify the editor UI until this analysis is complete.

After the audit, provide a concise implementation plan with:

- files/packages to modify
- files/packages to create
- dependencies to add
- architecture decisions
- risks
- migration concerns
- first implementation milestone

Then stop and wait for approval before performing major architectural changes.

The objective is not to build another video editor.

The objective is to take a working professional editor and turn it into AVID.

Preserve what is already excellent.

Change only what needs changing.

Build the intelligence layer deeply.