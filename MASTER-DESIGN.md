# AVID DESIGN MASTER PROMPT

## 0. PURPOSE

You are designing and implementing the complete visual system for AVID.

AVID stands for:

AI Video Intelligence & Direction.

AVID is a professional desktop video editor with an AI Director.

It is not an AI SaaS dashboard.

It is not a chatbot wrapped around a video editor.

It is not an AI video generator.

It is not a collection of marketing cards.

It is a real creative workstation with AI integrated directly into the editing workflow.

The two primary surfaces covered by this specification are:

1. The AVID landing website
2. The AVID desktop application / harness

These two surfaces must feel like the same product, but they must not have identical layouts.

The website is cinematic and expressive.

The application is dense, precise and operational.

Both share the same underlying design language.

Do not improvise a different visual identity for individual pages.

Do not make aesthetic decisions that contradict this document.

If a requirement is unclear, preserve the principles in this document rather than inventing a trendy UI pattern.

---

# 1. CORE DESIGN PHILOSOPHY

AVID should feel like software made by people who understand professional creative tools.

Reference the visual discipline of products such as:

* Figma
* Final Cut Pro
* DaVinci Resolve
* Adobe Premiere Pro
* professional developer tools
* T3 Code
* OpenCode

Do not copy their interfaces.

Study the principles behind them.

Professional creative software gives the user's work the majority of the visual attention.

Figma's interface, for example, is organized around a central canvas with navigation, toolbars and property panels surrounding it rather than turning every capability into a marketing card.

AVID should follow the same principle:

THE WORK IS THE INTERFACE.

The video, timeline, transcript, media, edit decisions and AI operations are more important than decorative UI.

---

# 2. ANTI-SLOP DIRECTIVE

This is a hard requirement.

Do not generate generic AI startup design.

Do not use:

* purple AI gradients
* blue/purple gradient text
* glowing orbs
* floating AI brains
* robot illustrations
* excessive glassmorphism
* giant rounded cards
* excessive shadows
* giant gradient blobs
* excessive blur
* "magic" sparkles
* generic 3D AI objects
* stock photos of people pointing at laptops
* generic smiling creator photography
* fake testimonials
* fake metrics
* fake customer logos
* fake social proof
* fake awards
* fake statistics
* fake product claims
* "revolutionize your workflow"
* "unlock your creativity"
* "the future of..."
* "next-generation..."
* "supercharge..."
* "10x..."
* "seamlessly..."
* "powerful AI..."
* vague feature descriptions
* meaningless buzzwords
* generic SaaS pricing-card layouts unless actually required
* decorative UI that does not communicate product behavior

Do not add an element simply because an AI-generated website normally has it.

Every visual element must have a reason.

Every animation must communicate state, hierarchy, transition, causality or product behavior.

Every sentence must communicate something specific about AVID.

If a section can be removed without losing product understanding, question whether it belongs.

---

# 3. AVID VISUAL IDENTITY

The base identity is MONOCHROME.

This is non-negotiable.

The core AVID identity should work entirely in grayscale.

Primary palette:

BLACK / NEAR BLACK
CHARCOAL
GRAPHITE
DARK GRAY
MID GRAY
LIGHT GRAY
WHITE

Do not make amber the brand color.

Amber is removed from the core identity.

Do not hard-code electric blue as the permanent brand color either.

Electric blue may exist as one optional accent theme.

The brand itself must remain recognizable without color.

The logo, typography, spacing, geometry, motion language and UI structure are the brand.

---

# 4. COLOR ARCHITECTURE

The application must have a real theme system.

Do not build the UI around hard-coded colors.

Every visual color must reference semantic design tokens.

Example token architecture:

--background
--background-subtle
--surface
--surface-raised
--surface-hover
--surface-active
--surface-selected
--border
--border-subtle
--border-strong
--text
--text-secondary
--text-muted
--text-disabled
--accent
--accent-hover
--accent-active
--accent-contrast
--success
--warning
--error
--info
--selection
--focus
--timeline-playhead
--ai-generated
--ai-suggested
--audio
--video
--caption

Never use raw colors directly inside components.

Users must be able to customize the theme without modifying code.

---

# 5. THEME CUSTOMIZATION

Theme customization is a first-class AVID feature.

The user must be able to customize:

* interface background
* panel background
* elevated surfaces
* borders
* primary text
* secondary text
* muted text
* accent color
* accent hover
* accent active
* selection color
* focus color
* timeline playhead color
* AI activity color
* waveform color
* timeline clip colors
* success
* warning
* error
* informational states

The user should be able to start from presets and then customize individual values.

Provide several professionally designed presets.

Examples:

AVID Monochrome
AVID Graphite
AVID Blue
AVID Violet
AVID Green
AVID Red
AVID Solar
AVID High Contrast
System

These are examples, not a limitation.

The system should support arbitrary user-created themes.

Users should be able to save a custom theme.

Users should be able to export/import a theme.

Theme definitions should be serializable.

Theme changes should update the interface immediately.

Do not make customization a gimmick.

The resulting interface must remain readable and professional regardless of accent choice.

---

# 6. COLOR USAGE RULE

Color communicates state.

Color does not decorate the application.

In monochrome mode, the UI should remain fully usable.

Accent colors should primarily communicate:

* selection
* focus
* active tools
* playhead
* current operation
* AI state
* links
* interactive emphasis

Do not color every button.

Do not color every heading.

Do not color every card.

Do not make the application look like a rainbow.

A user changing the accent color should not destroy the hierarchy.

---

# 7. LOGO

AVID requires a real visual identity.

The logo must not simply be the word "AVID" typed in a font.

Create a unique geometric symbol.

The symbol should be derived from the concepts of:

* editing
* frames
* direction
* timeline
* intelligence
* convergence
* an edit point
* movement through media

Do not use:

* play button
* film strip
* camera
* robot
* brain
* sparkle
* generic AI star
* infinity symbol
* generic lightning bolt

The symbol must work:

* at favicon size
* in the desktop application
* in the website header
* in the application title bar
* inside the command palette
* as an app icon
* as a monochrome mark
* as a single-color mark
* on dark backgrounds
* on light backgrounds

The logo must remain recognizable without gradients.

Do not overcomplicate the mark.

It should feel like a real software company created it, not an AI logo generator.

---

# 8. TYPOGRAPHY

Typography must feel professional and technical.

Do not introduce an unusual decorative font.

Use the native/default professional font approach used by modern developer tools such as T3 Code and OpenCode as the baseline.

The font must be customizable.

The application should expose:

Interface Font
Monospace Font

The default interface font should use the platform/product default font stack rather than forcing a novelty typeface.

The default must feel similar in spirit to modern developer tooling:

* clean
* neutral
* highly legible
* excellent at small sizes
* excellent numerals
* strong punctuation
* professional

Use a monospace face for:

* timecodes
* frame numbers
* technical metadata
* file paths
* code
* keyboard shortcuts where appropriate
* AI operations
* diagnostic output
* render information

Typography hierarchy must be subtle.

Do not use giant text simply to create visual drama.

---

# 9. SPACING

Use a consistent spacing system.

Base unit: 4px.

Preferred spacing values:

4
8
12
16
20
24
32
40
48
64
80
96

Avoid arbitrary spacing.

The application should feel dense but breathable.

The website should feel spacious but not empty.

---

# 10. BORDER RADIUS

Do not make everything pill-shaped.

Do not make everything a rounded card.

Default radius:

4px

Secondary:

6px

Large structural surfaces:

8px maximum unless there is a clear reason.

Buttons may use small radii.

Inputs may use small radii.

Menus may use small radii.

Cards should generally not have visible rounded containers unless grouping genuinely benefits from one.

Professional editing software tends to rely heavily on alignment, separators and hierarchy rather than rounded cards.

AVID should do the same.

---

# 11. DEPTH AND SHADOWS

Depth should come from:

* contrast
* borders
* elevation
* spacing
* layering

not excessive shadows.

Avoid:

* huge shadows
* glowing borders
* neon outlines
* glass blur everywhere

Panels should feel physically separated without looking like floating web cards.

---

# 12. MOTION LANGUAGE

Motion is one of AVID's defining characteristics.

But motion must be intentional.

Never add animation because "the page needs more animation."

Every animation must communicate one of:

1. arrival
2. departure
3. transformation
4. hierarchy
5. progress
6. causality
7. interaction
8. state
9. continuity

The motion language should feel like editing.

AVID is a video editor.

Therefore transitions should often behave like:

* clips moving
* tracks shifting
* frames entering
* panels sliding into place
* timelines extending
* playheads travelling
* media revealing itself
* cuts happening
* layers stacking
* information resolving

Do not use random floating animations.

---

# 13. MOTION TIMING

Use a small, consistent motion system.

Fast interaction:

120–180ms

Standard UI:

180–280ms

Panel transitions:

250–400ms

Major website transitions:

400–800ms

Cinematic transitions:

600–1200ms

Do not make basic buttons take 700ms to animate.

Do not make every element animate independently.

Use staggered motion only when it communicates hierarchy.

Use easing that feels physical and controlled.

Avoid excessive spring physics.

Avoid bouncy UI.

AVID should feel precise, not playful.

---

# 14. LANDING PAGE PHILOSOPHY

The landing site is not a conventional SaaS marketing page.

It is a product demonstration.

The visitor should understand AVID before they finish the first major viewport.

The website should answer:

What is AVID?

What does it actually do?

How does AI interact with editing?

What does the editor look like?

Why is this different from an AI video generator?

Can I bring my own models?

Can I actually edit the result?

The site should demonstrate these answers visually.

---

# 15. LANDING PAGE STRUCTURE

The landing page should be a continuous visual narrative.

Do not build:

Hero
Features
Testimonials
Pricing
FAQ
CTA

as disconnected cards.

Build a sequence.

Recommended structure:

1. Arrival
2. Product reveal
3. Live editor demonstration
4. Recordly product film
5. Script → timeline
6. Footage → understanding
7. Explain → visualize
8. AI Director
9. Editable AI
10. Bring Your Own Model
11. Professional editor
12. Final CTA

Sections should transition into one another.

---

# 16. LANDING PAGE HEADER

Extremely minimal.

Left:

AVID logo + wordmark

Center or right:

Product
Download
Docs
GitHub

Right:

Launch AVID

Do not create a giant navigation bar.

Do not put ten links in the header.

The header should become visually quieter as the hero becomes dominant.

Use a subtle sticky header.

When scrolling, the header can transition from transparent/low-contrast to a solid surface.

---

# 17. HERO

The hero should not begin with a paragraph.

Begin with the product.

Recommended sequence:

AVID mark

Short statement

Interactive editor demonstration

Primary action

Secondary action

The first screen should visually contain:

* actual AVID UI
* timeline
* video viewer
* AI Director
* media
* motion

The product itself is the hero.

Do not use a generic abstract animation.

---

# 18. HERO COPY

Copy must be specific.

Never use generic AI marketing language.

Possible copy direction:

"Video editing, directed."

"Turn footage into an edit."

"Give AVID the footage. Tell it what to change."

"Start with a script. End with a timeline."

These are directions to explore, not mandatory final copy.

Copy should be short.

The interface should do most of the explaining.

---

# 19. HERO PRODUCT ANIMATION

The editor should assemble itself.

Sequence:

1. Empty dark canvas.
2. AVID mark appears.
3. Main editor frame fades in.
4. Media browser enters from the left.
5. Viewer enters.
6. Inspector enters from the right.
7. Timeline expands upward from the bottom.
8. AI Director appears.
9. Media thumbnails populate.
10. Timeline clips populate.
11. Playhead appears.
12. Video begins playing.
13. AI Director receives an instruction.
14. A proposed edit appears.
15. Timeline changes.
16. Preview updates.
17. User reviews the change.
18. Change is accepted.

The visitor should understand the application without reading a paragraph.

---

# 20. RECORDLY PRODUCT VIDEO

The landing page must have a real product demonstration.

Once the AVID harness is functional, record a polished walkthrough using Recordly.

Do not create fake UI footage.

Do not create a cinematic fake product demo.

Show the real application.

The video should demonstrate:

* importing media
* transcript generation
* AI Director
* editing
* timeline changes
* visual generation
* captions
* preview
* export

The video should be integrated directly into the landing page narrative.

Do not label it simply:

"Watch Demo"

Instead, introduce it through product behavior.

Example:

"Give AVID the footage."

Then the real footage begins.

---

# 21. VIDEO PLAYER DESIGN

The player should feel native to the site.

Controls should be minimal.

Avoid a giant YouTube-style embedded player.

Use:

* play/pause
* timeline
* volume
* fullscreen
* playback speed if needed

Controls should fade when inactive.

The video can expand during scroll.

The video may transition from a framed product demonstration into full-width cinematic footage.

---

# 22. SCRIPT → TIMELINE SECTION

This is a signature interaction.

Show a real script.

As the visitor scrolls:

Script text appears.

AVID analyzes it.

Sections become visually identified.

Visual opportunities appear.

Then the script transforms into a storyboard.

Then the storyboard becomes a timeline.

Then the timeline becomes playable footage.

The transition must feel continuous.

Do not use three disconnected screenshots.

It should visually communicate:

Idea
→ Script
→ Storyboard
→ Assets
→ Timeline
→ Video

---

# 23. EXPLAIN → VISUALIZE

This is one of AVID's signature capabilities.

Show a spoken explanation.

Example:

"Kafka has three partitions..."

As the sentence progresses:

Kafka diagram appears.

Partitions appear.

Messages move.

Labels appear.

Connections animate.

The website should demonstrate that AVID can understand an abstract concept and turn it into an editable visual.

Do not use a generic AI image.

Use AVID's deterministic visual system.

---

# 24. AI DIRECTOR SECTION

Do not present the AI Director as a generic chatbot.

It is a production control surface.

Show:

AI DIRECTOR

Project
Current task
Understanding
Plan
Changes
Review

Example:

UNDERSTANDING
✓ Transcript
✓ Scenes
✓ Speakers
✓ Existing assets

PROPOSED EDIT

01 Remove repeated intro
02 Tighten opening
03 Add diagram
04 Insert product footage
05 Reframe for 9:16

[ Review Changes ]

Then visually show the timeline responding.

---

# 25. AI CHANGE VISUALIZATION

When AI proposes an edit, the website should visually demonstrate:

Before

↓

Proposed change

↓

After

The user should understand that AI edits are structured and reversible.

Use:

* diff-like highlighting
* timeline changes
* before/after preview
* operation labels

Do not represent AI as magic.

Represent AI as a powerful production tool.

---

# 26. MODEL PROVIDER SECTION

AVID is model-agnostic.

Show official provider identities where permitted and appropriate.

Examples:

Claude
Gemini
OpenAI
Ollama
Whisper
local OpenAI-compatible models

Do not create fake provider logos.

Do not alter official marks.

Do not make the provider logos enormous.

Present them as part of the AVID toolchain.

The message is:

Use the models you want.

Not:

AVID is powered by one magical model.

---

# 27. HARNESS DESIGN

The AVID harness is the control center for development and agent-assisted production.

It is NOT the video editor.

The harness should feel like an engineering control room.

The editor should feel like a creative workstation.

They share the same design language.

---

# 28. HARNESS CORE LAYOUT

Desktop layout:

LEFT:
Navigation / workspaces

CENTER:
Primary working surface

RIGHT:
Context / inspector / task details

BOTTOM:
Events / terminal / activity / logs when required

The center should always receive the largest visual area.

Never allow sidebars to consume the majority of the screen.

Use resizable panels.

Allow panels to collapse.

Remember the Figma principle:

the work should have the largest available surface.

---

# 29. HARNESS PRIMARY NAVIGATION

Navigation should be narrow.

Possible areas:

Home
Projects
Agents
Tasks
Research
Timeline
Files
Tests
Git
Settings

Use icons plus labels where necessary.

Allow compact icon-only mode.

Do not use giant navigation cards.

---

# 30. HARNESS HOME SCREEN

The home screen should immediately communicate current work.

Show:

Recent projects

Active sessions

Recent agent activity

Pending tasks

Recent edits

Git state

Build state

Test state

No generic dashboard charts.

No meaningless analytics.

No fake productivity metrics.

The dashboard exists to help the user resume work.

---

# 31. PROJECT SCREEN

A project should have:

Project identity

Current branch

Current task

Recent activity

Agent sessions

Research

Architecture

Tests

Git status

Documentation

Timeline/history

The project should feel like a workspace rather than a database record.

---

# 32. AGENT SCREEN

Agents should be presented as tools, not cartoon characters.

No robot avatars.

No glowing AI faces.

No AI orb.

Agent rows should contain:

Agent name
Role
Model
Status
Current task
Elapsed time
Token/context information when useful

Example:

ARCHITECT

Claude

Planning

2m 14s

Do not overdecorate agent status.

---

# 33. CURRENT MODEL SIGNIFIER

The current model indicator needs a major redesign.

It must look intentional and premium.

Do not use a generic pill saying:

"Claude 4"

inside a rounded purple bubble.

Instead create a compact model control.

Example:

[ provider mark ] Claude
Sonnet
↓

or:

MODEL
[ Claude icon ] Sonnet

The provider icon should be recognizable but subtle.

The model name should have hierarchy.

Provider:

small

Model:

primary

Context/status:

secondary

Clicking it should open a model selector.

The selector should allow:

Provider
Model
Local/cloud
Context
Capabilities
Cost information where available

Do not make the model indicator visually dominate the application.

---

# 34. AGENT SESSION SCREEN

Structure:

TOP:
Session identity + model + status

CENTER:
Conversation / agent output / tool activity

RIGHT:
Context / files / task

BOTTOM:
Input / command

Agent output should not look like a chat app.

Tool execution should be visually distinct.

Example:

READ
src/avid-core/timeline.rs

EDIT
apps/desktop/src/...

TEST
cargo test

PASS

COMMIT
...

Use monospace selectively.

---

# 35. TASK SCREEN

Tasks should look operational.

Each task contains:

Title

Description

Status

Owner/agent

Files

Plan

Progress

Checks

Review

Changes

The user should always understand:

What is happening?

Why is it happening?

What changed?

What remains?

---

# 36. RESEARCH SCREEN

Research should feel like a working notebook.

Not a generic web browser.

Sections:

Question

Sources

Findings

Decisions

Open questions

Related architecture

Evidence

The interface should support source-to-decision traceability.

---

# 37. TERMINAL / EVENT SCREEN

The terminal should be compact and professional.

Use monospace.

Do not style it like a hacker movie.

No green Matrix text.

No unnecessary glowing effects.

Events should be structured.

Example:

14:32:04  BUILD     cargo build
14:32:11  TEST      42 passed
14:32:14  AGENT     architecture complete
14:32:19  GIT       3 files changed

Allow filtering.

Allow search.

Allow copying.

Allow expanding an event.

---

# 38. GIT SCREEN

Git should show:

branch
status
changed files
diff
commits
history

Use a clean diff interface.

Do not invent a special visual language when established developer-tool patterns work better.

---

# 39. SETTINGS

Settings must be comprehensive.

Sections:

General
Appearance
Themes
Typography
Models
Providers
Agents
Keyboard
Editor
Media
Audio
Rendering
Storage
Privacy
Network
Performance
Accessibility
Developer
Experimental

Settings must not be one enormous scrolling form.

Use categorized navigation.

---

# 40. APPEARANCE SETTINGS

Appearance must allow:

Theme
Accent
Light/dark/system where supported
Density
Panel opacity where appropriate
UI scale
Animation level
Reduced motion
Sidebar behavior
Timeline density

The user should be able to preview changes immediately.

---

# 41. THEME EDITOR

Build a proper theme editor.

Show:

Theme preview

Color tokens

Interface preview

Timeline preview

Editor preview

Buttons

Inputs

AI states

Selection states

The user should see what they are changing.

Allow:

Save theme
Duplicate theme
Reset
Export
Import

Do not expose only raw hex inputs.

Provide semantic controls first.

Advanced users may access raw values.

---

# 42. CUSTOMIZATION MUST NOT BREAK BRAND

Even with a custom theme, AVID should remain recognizable through:

* logo
* typography
* iconography
* spacing
* layout
* motion
* component geometry
* terminology

Color is customizable.

Identity is not.

---

# 43. BUTTON SYSTEM

Buttons must be restrained.

Primary button:

solid accent or high-contrast foreground

Secondary:

subtle surface

Tertiary:

text / minimal

Destructive:

semantic red only when necessary

Ghost:

transparent with hover state

Do not make every button filled.

Button heights should be consistent.

Approximate sizes:

Small: 28px
Default: 32px
Large: 36–40px

Avoid oversized 48–56px buttons inside the application.

Landing-page CTAs may be larger.

---

# 44. BUTTON MOTION

Hover:

subtle background change

Active:

slight contrast shift

Focus:

visible focus ring

Loading:

preserve button dimensions

Success:

brief state transition

Never use bouncing.

Never use exaggerated scaling.

Never make buttons fly around.

---

# 45. INPUTS

Inputs should be compact and professional.

Clear focus state.

Visible placeholder distinction.

Keyboard-friendly.

Search inputs should support shortcuts where appropriate.

Do not create giant rounded search bars unless they are genuinely the primary interaction.

---

# 46. MENUS

Menus:

* compact
* aligned
* keyboard navigable
* clear selection
* subtle shadow
* small radius
* strong hierarchy

Use separators sparingly.

Do not turn menus into cards.

---

# 47. COMMAND PALETTE

AVID should have a command palette.

Shortcut:

Cmd/Ctrl + K

It should search:

commands
projects
files
media
scenes
agents
models
settings
timeline operations

The command palette should feel like a core AVID interaction.

Fast.

Minimal.

Keyboard-first.

---

# 48. ICONOGRAPHY

Use one coherent icon system.

Do not mix icon libraries randomly.

Icons should be:

* simple
* geometric
* consistent stroke width
* visually quiet

Do not use emoji as UI icons.

Do not use colorful icons unless they communicate external provider identity.

---

# 49. AI STATE LANGUAGE

AI states must be visually consistent.

Suggested states:

IDLE
ANALYZING
PLANNING
PROPOSING
RUNNING
WAITING
COMPLETE
FAILED
CANCELLED

AI-generated content should have subtle provenance.

AI-suggested content should be visually distinguishable from committed user content.

Do not make all AI-generated content glow.

---

# 50. TIMELINE VISUAL LANGUAGE

The timeline is one of the most important AVID surfaces.

It must look like a real professional video editor.

Show:

tracks
clips
waveforms
thumbnails
playhead
time ruler
markers
selection
transitions
captions
keyframes where supported

The timeline should be dense but readable.

It must support:

zoom
scroll
selection
dragging
trimming
splitting
snapping
multi-select

Do not simplify it into a fake marketing timeline.

---

# 51. VIDEO VIEWER

The viewer is a primary surface.

Controls:

play
pause
step
timecode
volume
fullscreen
zoom
fit
aspect ratio where applicable

Overlay controls should remain minimal.

The video itself must dominate.

---

# 52. MEDIA BROWSER

The media browser should support:

grid/list
thumbnail previews
metadata
duration
resolution
FPS
audio presence
selection
search
filter
folders/collections

Use real thumbnails when available.

Avoid placeholder gradients.

---

# 53. INSPECTOR

The inspector should feel like professional creative software.

Group properties:

Transform
Crop
Audio
Speed
Color
Captions
Effects
AI metadata
Motion
Advanced

Properties should be editable directly.

Avoid excessive cards.

Use labels and dividers.

---

# 54. RESPONSIVE BEHAVIOR

The desktop app is the primary product.

Do not force web-style responsive behavior onto it.

Instead:

Panels resize.

Panels collapse.

Toolbar compresses.

Navigation minimizes.

Inspector can become an overlay.

Timeline can expand.

The user should be able to create more canvas/timeline space.

---

# 55. LANDING PAGE RESPONSIVENESS

The website must work on:

desktop
tablet
mobile

But mobile should not simply be a shrunk desktop editor.

On mobile:

product demonstrations become focused compositions.

Timeline details simplify.

Video becomes primary.

Navigation collapses.

Animations remain performant.

---

# 56. ACCESSIBILITY

All interactive controls require:

keyboard navigation
visible focus
ARIA labels where appropriate
sufficient contrast
reduced-motion support
screen-reader semantics where applicable

Never communicate state through color alone.

---

# 57. REDUCED MOTION

If the user enables reduced motion:

* disable cinematic transitions
* reduce panel movement
* remove parallax
* reduce stagger
* replace complex transformations with fades
* preserve functional state changes

Do not disable usability.

---

# 58. PERFORMANCE

Animations must not destroy performance.

Prefer:

transform
opacity
compositor-friendly properties

Avoid expensive layout thrashing.

Do not animate huge DOM trees unnecessarily.

Do not continuously animate decorative objects.

Landing page motion must be lazy-loaded where appropriate.

Video must not block page interaction.

---

# 59. LANDING PAGE SCROLL BEHAVIOR

Scrolling should feel like moving through an edit.

Use:

* pinned sections
* controlled horizontal movement
* timeline progression
* viewport transformations
* clip reveals
* panel expansion
* before/after transitions

But use these only when they explain the product.

Do not create scroll hijacking that makes normal navigation frustrating.

Normal browser scrolling must remain understandable.

---

# 60. SECTION TRANSITIONS

Every major website section should transition intentionally.

Examples:

EDITOR → VIDEO

Timeline expands until it becomes the video's progress bar.

SCRIPT → STORYBOARD

Text lines separate into visual blocks.

STORYBOARD → TIMELINE

Blocks move vertically into timeline tracks.

TIMELINE → FINAL VIDEO

Timeline recedes while the viewer expands.

AI DIRECTOR → EDIT

AI plan items become actual timeline operations.

MODEL SECTION → HARNESS

Provider controls transition into the model selector inside the application.

These transitions are examples of required behavior.

Do not replace them with generic fade-up animations.

---

# 61. WEBSITE MICROINTERACTIONS

Use small details:

* cursor changes over interactive editor elements
* timeline hover previews
* clip selection
* model selector opening
* subtle waveform movement
* playhead movement
* text selection
* panel resizing
* command palette animation
* hover state on provider logos

Microinteractions should make the site feel like software.

---

# 62. LANDING PAGE CTA

Do not finish with:

"Ready to transform your workflow?"

Instead:

"Open AVID."

or:

"Start editing."

or:

"Try the editor."

The CTA should describe the actual action.

---

# 63. COPYWRITING SYSTEM

All copy must be written as if a thoughtful product manager and product designer wrote it.

Short.

Specific.

Concrete.

No corporate language.

No AI buzzword soup.

No fake enthusiasm.

No exaggerated promises.

No unnecessary adjectives.

Avoid:

"powerful"

"revolutionary"

"seamless"

"intelligent"

"next-generation"

"game-changing"

unless the word has a specific factual purpose.

Describe what AVID actually does.

---

# 64. PRODUCT LANGUAGE

Prefer:

"Edit"

"Review"

"Direct"

"Plan"

"Analyze"

"Find"

"Trim"

"Visualize"

"Generate"

"Replace"

"Approve"

"Undo"

"Export"

Avoid:

"Magic"

"Enhance"

"Supercharge"

"Unleash"

"Transform"

"Elevate"

---

# 65. EMPTY STATES

Empty states should be useful.

Example:

NO MEDIA

"Import footage to start building the edit."

Actions:

Import media
Start from script

Do not use:

"Nothing here yet! ✨"

---

# 66. LOADING STATES

Loading states must communicate what AVID is doing.

Bad:

"Loading..."

Better:

"Analyzing 24 clips"

"Transcribing interview"

"Finding visual opportunities"

"Building edit plan"

"Rendering preview"

Use progress when measurable.

---

# 67. ERROR STATES

Errors must explain:

What happened?

Why?

What can the user do?

Example:

"Transcription failed because the local model is unavailable."

[Retry] [Choose model]

Do not show raw stack traces to normal users.

Developer details can be expandable.

---

# 68. SUCCESS STATES

Do not use giant confetti.

Use subtle confirmation.

Example:

"Edit plan applied"

with:

Undo

or:

"Export complete"

[Open file]

---

# 69. WEBSITE CONTENT RULE

Every section must demonstrate a real capability.

Do not create a feature section simply because competitors have one.

The website should be possible to understand without reading every word.

Visual proof > claims.

Real product > mockup.

Real footage > stock photography.

Actual model logos > invented logos.

Actual UI > fake dashboard.

Actual workflow > marketing illustration.

---

# 70. LANDING PAGE AS A PRODUCT TOUR

The visitor should effectively experience a miniature AVID session:

Enter project

Import footage

Understand footage

Give direction

AI creates plan

Review changes

Visualize concept

Edit timeline

Export

This is the story.

---

# 71. DESIGN SYSTEM COMPONENTS

Build reusable primitives before building individual pages.

Required primitives:

Button
IconButton
Input
Select
Dropdown
Tooltip
Popover
Dialog
Tabs
SegmentedControl
CommandPalette
Panel
ResizablePanel
Sidebar
Toolbar
Inspector
Timeline
TimelineTrack
TimelineClip
VideoViewer
MediaCard
MediaGrid
Waveform
Transcript
AIPlan
AIStatus
ModelSelector
ProviderBadge
AgentStatus
Terminal
EventLog
Toast
Progress
Slider
Toggle
ColorPicker
ThemeEditor
KeyboardShortcut
EmptyState
ErrorState

Every primitive must consume semantic tokens.

---

# 72. MODEL PROVIDER BADGES

Create one consistent provider identity component.

It should support:

provider icon
provider name
model name
local/cloud indicator
status

Examples:

Claude / Sonnet
Gemini / Pro
OpenAI / ...
Ollama / Qwen
Whisper / Local

Do not create a different badge design for each provider.

---

# 73. BRAND VS USER THEME

This distinction is critical.

BRAND LAYER:

Logo
Wordmark
Typography
Iconography
Spacing
Geometry
Motion
Terminology

USER THEME LAYER:

Accent
Background
Surface
Text
Selection
Semantic colors
Timeline colors
AI colors

The user may radically change colors.

The interface must still look unmistakably like AVID.

---

# 74. DEFAULT AVID THEME

The default AVID theme should be monochrome.

Suggested direction:

Near-black background

Graphite panels

Soft-white text

Gray secondary text

Very subtle borders

One restrained accent available for active state.

Do not finalize the exact accent until visual exploration has been performed.

Electric blue is an allowed AVID preset, not a mandatory permanent identity color.

---

# 75. OPTIONAL ACCENT PALETTE

Include professionally designed presets such as:

Electric Blue
Ice
Violet
Emerald
Red
Orange
Yellow
Pink

These should all work against the same monochrome base.

Do not change the entire application into a colorful theme.

Accent means accent.

---

# 76. DESIGN EXPLORATION RULE

Before implementing a major visual direction, generate several internal explorations.

Do not immediately settle on the first AI-generated design.

Compare:

Density
Typography
Panel proportions
Logo
Accent treatment
Motion
Timeline
AI Director
Model selector

Choose based on coherence with the product.

---

# 77. DO NOT LET AI GENERATE RANDOM UI

When implementing with OpenCode:

Do not ask:

"Make the AVID website look modern."

Do not ask:

"Make a beautiful dashboard."

Do not ask:

"Make a futuristic AI video editor."

Those prompts invite slop.

Instead implement directly from this specification.

Every page must have an explicit layout.

Every major animation must have explicit behavior.

Every component must use the design system.

---

# 78. OPEN CODE IMPLEMENTATION DIRECTIVE

Before writing UI code:

1. Inspect the repository.
2. Find existing design tokens.
3. Find existing components.
4. Find existing routes.
5. Find existing theme implementation.
6. Find existing fonts.
7. Find existing animations.
8. Find existing layout primitives.
9. Reuse before replacing.
10. Do not create duplicate components.

Then produce a design implementation plan.

Do not immediately code.

---

# 79. IMPLEMENTATION ORDER

Implement in this order:

PHASE 1
Design tokens

PHASE 2
Typography

PHASE 3
Theme engine

PHASE 4
Icon system

PHASE 5
Buttons / inputs / controls

PHASE 6
Panels / navigation

PHASE 7
Command palette

PHASE 8
Model selector

PHASE 9
Harness shell

PHASE 10
Editor shell

PHASE 11
Timeline

PHASE 12
AI Director

PHASE 13
Landing-page system

PHASE 14
Landing-page motion

PHASE 15
Recordly integration

PHASE 16
Responsive behavior

PHASE 17
Accessibility

PHASE 18
Performance

PHASE 19
Visual QA

Do not build the landing page on top of an unstable design system.

---

# 80. VISUAL QA

Every implementation must be visually inspected.

Check:

Alignment
Spacing
Typography
Contrast
Panel proportions
Button sizes
Icon consistency
Theme behavior
Dark/light behavior
Focus states
Hover states
Loading states
Error states
Empty states
Animations
Reduced motion
Mobile behavior

Do not mark a UI task complete simply because TypeScript compiles.

---

# 81. ANTI-REGRESSION

Do not fix one screen by introducing inconsistency elsewhere.

If a component changes:

Check every screen using it.

If a token changes:

Check all themes.

If typography changes:

Check every major layout.

If an animation changes:

Check reduced-motion behavior.

---

# 82. NO FAKE FUNCTIONALITY

Never create fake interactions purely for screenshots.

If a button exists:

It must work.

If a model selector exists:

It must represent the actual model system or be explicitly marked as a placeholder.

If a timeline clip moves:

It must use the actual timeline state.

If the website demonstrates an editor operation:

Prefer the real product recording over fake simulated behavior.

---

# 83. LANDING PAGE VIDEO RULE

When the real harness is ready:

Record the actual workflow.

Use Recordly or another recording tool.

The final landing page should feature the real application.

Do not replace the real recording with a synthetic recreation unless there is a technical reason.

---

# 84. FINAL DESIGN TEST

Ask these questions:

Can someone identify AVID as a video editor within five seconds?

Does the landing page show the product instead of merely describing it?

Does the application look like professional creative software?

Does the harness look like a serious engineering tool?

Does AI feel integrated rather than bolted on?

Does the model selector feel intentional?

Can users customize colors without destroying the brand?

Does the monochrome theme work by itself?

Does the UI remain coherent with Electric Blue?

Does the logo work at tiny sizes?

Does the website motion explain product behavior?

Does every animation have a purpose?

Does every sentence say something concrete?

Could this page have been generated by a generic AI website generator?

If the answer to the last question is even slightly yes, redesign it.

---

# 85. THE NORTH STAR

AVID should feel like:

A real video editor
that happens to have an AI Director.

Not:

An AI application
that happens to contain a video editor.

The landing page should feel like:

A product film
that happens to be a website.

Not:

A SaaS website
with product screenshots.

The harness should feel like:

A serious production and engineering environment.

Not:

A dashboard with AI agents.

The brand should feel:

Quiet.
Precise.
Technical.
Cinematic.
Customizable.
Distinct.

The product should do the talking.

The interface should prove the claims.

The motion should explain the product.

The typography should establish confidence.

The monochrome foundation should establish identity.

Color should belong to the user.

And nothing should be added merely because modern AI products usually have it.

--PART 2---

# AVID DESIGN MASTER PROMPT

## UX, LAYOUT, INTERACTION, STATES AND MICRO-DETAIL SPECIFICATION

This document supplements the AVID visual design master specification.

This is not a suggestion list.

These are implementation rules.

Do not infer missing UX patterns from generic SaaS conventions.

Do not invent layouts.

Do not fill unspecified areas with cards, gradients, decorative illustrations, statistics, testimonials or generic marketing sections.

When implementing AVID, think like a senior product designer designing a real professional application.

The goal is not to produce something that looks impressive in a screenshot.

The goal is to produce an interface that feels deliberately designed after years of use.

Every spacing value, state, transition, button, navigation path and interaction should have an intentional reason.

---

# 1. THE PRODUCT EXPERIENCE

AVID consists of two connected experiences:

1. The landing site
2. The desktop application / harness

The website introduces the product.

The application performs the work.

The transition between them must feel continuous.

A user should never feel like they went from a carefully designed product into a generic dashboard.

---

# 2. DESIGN HIERARCHY

Every screen must have a clear hierarchy.

At any moment the user should be able to identify:

1. Where am I?
2. What am I looking at?
3. What is the primary action?
4. What is currently selected?
5. What changed?
6. What can I do next?
7. How do I go back?

Never present five equally prominent actions.

One action should be visually primary.

Secondary actions should be quieter.

Destructive actions should be clearly distinguished.

---

# 3. PAGE STRUCTURE

Every application screen must have a consistent structural hierarchy.

Preferred structure:

```text
Application chrome
    ↓
Context/navigation
    ↓
Page header
    ↓
Primary workspace
    ↓
Secondary information/actions
```

Do not randomly move headers, controls and navigation between screens.

Users should develop muscle memory.

---

# 4. SCREEN PADDING

Use deliberate screen padding.

Application:

Primary workspace:
16–24px

Dense utility areas:
8–16px

Page-level headers:
16–24px

Large empty states:
32–64px

Website:

Desktop content width:
1200–1440px maximum depending on section

Primary horizontal page padding:
32–64px

Large cinematic sections:
64–120px vertical spacing

Do not use enormous padding simply to make a website appear premium.

Whitespace must establish hierarchy.

---

# 5. SPACING SYSTEM

Use a 4px base unit.

Preferred values:

4px
8px
12px
16px
20px
24px
32px
40px
48px
64px
80px
96px
120px

Do not use arbitrary values such as:

17px
23px
37px
51px

unless required by a specific visual alignment.

Spacing must be consistent across the entire application.

---

# 6. SPACING HIERARCHY

Spacing communicates relationships.

Use:

4–8px:
label → control

8–12px:
related controls

12–16px:
related groups

20–24px:
separate groups

32–48px:
major sections

64px+:
major page-level separation

Do not separate elements that belong together.

Do not crowd unrelated elements together.

---

# 7. ALIGNMENT

Alignment must be intentional.

Primary content should align to a consistent grid.

Labels and controls should share predictable alignment.

Toolbars should align vertically.

Panel headers should align with their content.

Buttons should not randomly shift between screens.

Avoid "almost aligned" elements.

If two components appear related, their edges should usually align.

---

# 8. APPLICATION GRID

Use a consistent desktop grid.

Default conceptual structure:

```text
┌──────┬───────────────────────────────┬────────────┐
│      │                               │            │
│ NAV  │        PRIMARY WORKSPACE     │ INSPECTOR  │
│      │                               │            │
│      │                               │            │
└──────┴───────────────────────────────┴────────────┘
```

Optional bottom region:

```text
┌──────┬───────────────────────────────┬────────────┐
│ NAV  │          WORKSPACE             │ INSPECTOR  │
├──────┴───────────────────────────────┴────────────┤
│                 EVENTS / TERMINAL                  │
└────────────────────────────────────────────────────┘
```

The center is always the primary surface.

---

# 9. RESIZABLE PANELS

Panels must be resizable where their content benefits from resizing.

Resizable boundaries should have:

* subtle hover state
* larger invisible hit target
* visual cursor change
* smooth resize
* minimum width
* maximum width

Do not require pixel-perfect dragging.

Remember panel dimensions between sessions.

Allow reset to default layout.

---

# 10. COLLAPSIBLE PANELS

Users should be able to collapse:

Navigation
Inspector
AI Director
Terminal
Secondary panels

Collapsed states must preserve discoverability.

Never make a panel disappear without leaving an obvious way to restore it.

---

# 11. PANEL DIVIDERS

Panel dividers are structural.

Default:

1px

They should be subtle.

On hover:

increase contrast slightly

On active resize:

increase contrast again

Do not use bright neon borders.

---

# 12. BACK BUTTONS

Back navigation must be deliberate.

Any screen that represents a child context should provide a way back.

Examples:

Projects → Project

Project → Agent Session

Agent Session → Task

Settings → Appearance

Appearance → Theme Editor

Research → Research Item

Download → Release Details

Use a consistent back pattern.

Example:

`← Project`

not merely a mysterious arrow icon.

On compact screens:

`←`

with an accessible tooltip.

Do not place back buttons randomly.

Preferred location:

top-left of the contextual content header.

Do not show a back button when browser/application navigation already clearly communicates the hierarchy and an additional button would be redundant.

---

# 13. BREADCRUMBS

Use breadcrumbs when hierarchy becomes deep.

Example:

Projects / AVID / Agents / Architect

Do not use breadcrumbs for every page.

Do not turn them into giant navigation bars.

They should establish context.

---

# 14. PAGE HEADERS

Every substantial screen should have a clear page header.

Header should contain:

Context

Title

Optional description

Primary action

Secondary actions

Optional status

Example:

```text
← Projects

AVID Editor
Build the next timeline interaction.

[ Open Project ]   [ ... ]
```

Do not create a huge marketing-style heading inside the application.

---

# 15. HEADER HEIGHT

Application page headers should be compact.

Target:

48–64px

Do not consume 120px of vertical space with application chrome.

The work must remain visible.

---

# 16. TOOLBARS

Toolbars must group related actions.

Example:

```text
[ Select ] [ Cut ] [ Trim ] | [ Undo ] [ Redo ] | [ ... ]
```

Use separators between conceptual groups.

Do not put every available action in the primary toolbar.

Rare actions belong in menus or command palette.

---

# 17. TOOLBAR STATES

Every toolbar control must support:

Default
Hover
Active
Pressed
Disabled
Focused
Loading where appropriate

Do not implement only the default state.

---

# 18. HOVER STATES

Hover should provide immediate feedback without becoming distracting.

Preferred behavior:

background contrast changes slightly

or

border becomes slightly more visible

or

icon/text contrast increases

Do not:

scale buttons dramatically

glow

bounce

rotate icons

move layout elements

change colors excessively

---

# 19. ACTIVE STATES

Active state must be visually distinct from hover.

Use:

* stronger contrast
* subtle background
* accent
* inset indicator
* underline where appropriate

Do not rely on color alone.

---

# 20. FOCUS STATES

Keyboard focus must always be visible.

Use a clear focus ring.

Focus should not alter layout.

Focus state must work in every theme.

Do not remove browser/native focus indicators without replacing them with a better accessible equivalent.

---

# 21. DISABLED STATES

Disabled controls:

* reduce contrast
* remain recognizable
* do not disappear
* do not trigger hover effects

Do not make disabled elements completely invisible.

---

# 22. PRESSED STATES

Buttons should communicate when physically pressed.

Use a subtle change in:

background
border
contrast

Do not use dramatic scaling.

---

# 23. LOADING STATES

Loading must preserve layout.

Never allow a button to jump because its text changed.

Example:

`Export`

becomes:

`Exporting…`

inside the same button width where possible.

For longer operations use:

progress
percentage
estimated stage
cancel

---

# 24. SKELETONS

Use skeletons only where content structure is known.

Do not create decorative skeleton loaders everywhere.

Skeletons should resemble the final content.

Do not animate every skeleton aggressively.

---

# 25. EMPTY STATES

Empty states must answer:

Why is this empty?

What should I do?

Example:

```text
No projects yet.

Create a project or import an existing AVID project.

[ New Project ]  [ Open Project ]
```

Never use:

"Nothing here yet ✨"

---

# 26. FIRST-RUN EXPERIENCE

On first launch, AVID should not dump the user into a complicated interface without context.

The first-run experience should establish:

Create project
Open project
Import project

Optional:

Choose theme
Choose model provider
Choose default AI model

Do not create a long onboarding slideshow.

Professional software should get users to the work quickly.

---

# 27. RECENT PROJECTS

Home should prioritize:

Recent projects

Pinned projects

New project

Open project

Import project

Do not prioritize decorative dashboard metrics.

---

# 28. PROJECT CREATION

New project should clearly ask for:

Project name

Location

Resolution

Frame rate

Aspect ratio where necessary

Optional template

Optional model configuration

Defaults should be sensible.

Do not force users through ten setup screens.

---

# 29. CONFIRMATION DIALOGS

Do not ask confirmation for every action.

Use confirmation when an action is:

destructive
difficult to undo
irreversible
potentially data-loss causing

Do not show:

"Are you sure?"

for normal reversible actions.

---

# 30. DIALOG DESIGN

Dialogs must have:

Title
Context
Primary action
Secondary/cancel action

Destructive action should be visually clear.

Do not place two equally strong buttons beside each other when one is clearly the intended action.

Escape closes dismissible dialogs.

Enter activates the primary action where safe.

---

# 31. TOOLTIPS

Tooltips are required for icon-only controls.

Tooltip should include:

Name

Optional shortcut

Example:

`Split Clip`

`Cmd + B`

Do not use tooltips for controls whose purpose is already obvious and labeled.

Tooltip timing should be consistent.

Do not show tooltips immediately when the cursor briefly crosses an icon.

---

# 32. KEYBOARD SHORTCUTS

AVID must be keyboard-first.

Every major operation should have discoverable shortcuts.

Show shortcuts in:

tooltips
menus
command palette
settings

Users must be able to customize shortcuts.

---

# 33. COMMAND PALETTE UX

Cmd/Ctrl + K opens the command palette.

Behavior:

Open
Search
Filter
Keyboard navigate
Enter to execute
Escape to close

Recent commands may appear.

Commands should be grouped by context.

Example:

EDIT

Cut
Split
Trim
Delete

PROJECT

Open
Save
Export

AI

Analyze
Plan Edit
Apply Changes

---

# 34. COMMAND PALETTE VISUAL DESIGN

It should be compact.

Centered or contextually positioned.

Do not make it a giant full-screen modal.

Use a strong search field.

Results should be highly scannable.

Selected result must be obvious.

---

# 35. NOTIFICATIONS

Use toasts for:

successful background operations
completed exports
completed AI operations
non-critical errors

Do not use toasts for information users must act on.

Toasts should not stack infinitely.

Allow dismissal.

Do not interrupt video playback with unnecessary notifications.

---

# 36. TOAST POSITION

Default:

bottom-right

unless that conflicts with important controls.

Toasts should not obscure:

timeline
transport controls
AI Director
primary CTA

---

# 37. CONTEXT MENUS

Right-click menus should expose contextual actions.

Timeline clip:

Cut
Copy
Paste
Split
Trim
Delete
Replace
Properties

Media:

Open
Reveal
Rename
Analyze
Add to timeline

Do not show irrelevant actions.

---

# 38. DRAG AND DROP

Drag interactions must communicate:

valid target
invalid target
active target
drop location

Use a clear insertion indicator.

Do not allow ambiguous drops.

---

# 39. DRAG FEEDBACK

During drag:

show ghost preview where useful

show insertion line

highlight destination

keep original object understandable

Do not make the dragged object disappear.

---

# 40. SELECTION

Selection must be obvious.

Selected timeline clip:

clear border/background distinction

Selected media:

clear selection indicator

Selected transcript segment:

clear highlight

Selection should work in monochrome mode.

---

# 41. MULTI-SELECTION

Support modifier-key selection where appropriate.

Show count where useful:

`4 clips selected`

Do not overwhelm the interface with selection metadata.

---

# 42. UNDO / REDO

Undo is a core AVID interaction.

Cmd/Ctrl + Z

Cmd/Ctrl + Shift + Z

AI operations must participate in undo/redo.

Users must be able to undo an AI edit as a coherent operation where appropriate.

Example:

`Undo AI edit`

not:

`Undo 47 internal mutations`

---

# 43. AI OPERATIONS

AI must never silently modify important user work.

AI should:

Analyze
Plan
Propose
Review
Apply

When appropriate.

The interface should make it clear whether an AI action is:

Suggested
Previewed
Applied

---

# 44. AI CHANGE REVIEW

When AI proposes changes:

show a review state.

Example:

```text
AI DIRECTOR

5 changes proposed

01 Remove 00:42–00:47
02 Tighten opening
03 Add diagram
04 Reframe speaker
05 Add captions

[ Apply all ] [ Review ]
```

Users should be able to inspect individual operations.

---

# 45. AI PROVENANCE

AI-generated material should have subtle provenance.

Do not add giant "AI GENERATED" labels.

Use:

small icon
subtle indicator
metadata
contextual status

The UI should remain clean.

---

# 46. MODEL SELECTOR

The model selector is a high-quality control, not a decorative badge.

Structure:

Provider identity

Model

Availability

Local/cloud state

Optional context/capability information

Example:

```text
[ provider icon ]  Claude
                    Sonnet
                    Cloud
```

Hover:

show additional context.

Click:

open model selector.

---

# 47. MODEL SELECTOR MENU

Group by provider.

Example:

ANTHROPIC

Claude Sonnet
Claude Opus

GOOGLE

Gemini ...

OPENAI

...

LOCAL

Ollama
Local models

Each model may display:

availability
local/cloud
capabilities

Do not overload the menu with technical information users do not need.

---

# 48. PROVIDER ICONS

Use official provider marks when appropriate.

Do not recolor official logos arbitrarily.

Keep them visually restrained.

The provider icon is a recognition aid.

It is not the visual centerpiece.

---

# 49. AGENT STATUS

Agent status must be understandable without color.

Example:

● Running
✓ Complete
○ Idle
! Attention
× Failed

Use shape/icon + text.

Never rely solely on red/green.

---

# 50. AGENT ACTIVITY

Running agents should expose useful information:

Current operation

Files being touched

Elapsed time

Tool activity

Current model

Progress where measurable

Do not display fake progress bars.

---

# 51. TERMINAL STATES

Terminal:

idle
running
success
warning
error

Use semantic color sparingly.

Monospace typography.

Selectable text.

Copy button.

Clear button where appropriate.

Search.

---

# 52. FILE STATES

Files should support:

normal
selected
modified
new
deleted
conflicted
ignored

Use icons and subtle state indicators.

---

# 53. GIT STATES

Git status should be immediately understandable.

Examples:

Clean

Modified

Ahead

Behind

Conflict

Uncommitted changes

Do not make the Git indicator visually dominant unless action is required.

---

# 54. SEARCH UX

Search must:

respond quickly
support keyboard navigation
preserve query
support filtering
show no-result state
show loading state when required

No result:

"No matches for 'kafka'."

Optionally:

Search files
Search commands
Search projects

---

# 55. NAVIGATION PERSISTENCE

Remember:

panel widths
collapsed panels
selected workspace where appropriate
theme
density
keyboard configuration

Do not unexpectedly reset the workspace on every launch.

---

# 56. SCROLL POSITION

Preserve scroll position when navigating back where appropriate.

If the user opens an item and returns, do not always dump them at the top of the previous page.

---

# 57. BACK NAVIGATION BEHAVIOR

Back should return to the previous meaningful context.

Example:

Projects
→ AVID
→ Agent
→ Task
→ Back

returns to Agent.

Not Home.

Navigation should feel hierarchical.

---

# 58. DEEP LINKING

Screens should have stable routes where appropriate.

Example:

/project/avid
/project/avid/agents
/project/avid/agents/architect
/project/avid/tasks/123

This makes navigation predictable.

---

# 59. URL / ROUTE STATES

Refreshing a screen should preserve the current context when technically possible.

Do not make users lose their place simply because the application refreshed.

---

# 60. LANDING PAGE LAYOUT

The landing page should use a strong vertical rhythm.

Recommended structure:

```text
HEADER

HERO

PRODUCT DEMONSTRATION

PRODUCT VIDEO

SCRIPT → STORYBOARD → TIMELINE

UNDERSTANDING FOOTAGE

EXPLAIN → VISUALIZE

AI DIRECTOR

EDITABLE AI

MODEL FREEDOM

PROFESSIONAL EDITING

DOWNLOAD

FINAL CTA

FOOTER
```

Each section should have a distinct purpose.

---

# 61. LANDING PAGE WIDTH

Do not allow text to stretch across the entire screen.

Reading width:

approximately 600–800px

Product demonstrations may use:

1200–1600px

Cinematic video may use:

full viewport width

Use width strategically.

---

# 62. LANDING PAGE SECTION SPACING

Small section:

64–96px

Standard:

120px

Major product demonstration:

160px+

Cinematic transition:

variable based on animation

Do not give every section exactly the same height.

---

# 63. LANDING PAGE HERO HEIGHT

The hero should occupy most of the first viewport.

But the product must appear within the first viewport.

Do not force the user to scroll through 600px of headline text before seeing AVID.

---

# 64. LANDING PAGE HERO MOTION

The hero animation must have a predefined sequence.

Do not allow implementation to invent random entrance animations.

Sequence:

Logo

→ statement

→ editor frame

→ media

→ viewer

→ timeline

→ AI Director

→ model selector

→ playback

→ AI instruction

→ timeline change

→ result

Every transition should visually explain the product.

---

# 65. LANDING PAGE SCROLL INTERACTIONS

Use scroll as a storytelling mechanism.

When a section is pinned:

the content must continue progressing.

Do not create scroll effects that feel like a website fighting the user.

Do not hijack scrolling.

Use natural scroll with controlled animation.

---

# 66. SECTION ENTRY ANIMATION

Do not use the same:

fade up
fade up
fade up
fade up

animation for every section.

This is a major anti-slop rule.

Instead, animation should correspond to the section's concept.

Script:

text transforms

Timeline:

tracks assemble

AI:

operations appear

Video:

viewer expands

Models:

provider marks resolve

Download:

platform options reveal

---

# 67. LANDING PAGE HOVER STATES

Interactive product UI on the website should behave like the real application.

If a timeline clip is hoverable:

show hover state.

If a model is selectable:

show selection.

If a button exists:

show button states.

Do not create fake UI that has no interaction.

---

# 68. LANDING PAGE PRODUCT DEMO

Where possible, use real AVID UI components.

Do not build a completely separate fake editor for the website.

The website demonstration should visually derive from the actual application design system.

---

# 69. DOWNLOAD EXPERIENCE

Downloads are first-class.

Supported platforms:

macOS
Windows
Linux

Header:

Download

Hero:

platform-aware download CTA

Dedicated download page:

`/download`

---

# 70. PLATFORM DETECTION

Detect the user's platform when possible.

Example:

macOS user:

Download for macOS

Windows user:

Download for Windows

Linux user:

Download for Linux

Always provide:

Other platforms

---

# 71. DOWNLOAD PAGE

The download page should clearly communicate:

Platform

Architecture

Minimum supported version

Current version

File size where useful

Release notes

Checksums/signature information when available

Installation instructions

Older releases

Do not make users hunt for the actual download button.

---

# 72. DOWNLOAD BUTTON STATES

Download buttons:

Default
Hover
Pressed
Downloading
Downloaded
Unavailable

If an installer is unavailable:

explain why.

Never display a button that does nothing.

---

# 73. WEBSITE FOOTER

Minimal.

Include:

AVID

Product
Download
Documentation
GitHub
Community where actually available
Privacy
Terms
Contact

Do not create 40 footer links.

---

# 74. MOBILE WEBSITE

On mobile:

Navigation collapses.

Product demonstrations simplify.

Timeline may become a focused crop.

Video remains central.

Text remains readable.

Buttons become thumb-friendly.

Do not attempt to display the entire desktop editor at full complexity.

---

# 75. TOUCH TARGETS

Interactive controls should have sufficiently large hit areas.

Small visual icons may have larger invisible interaction regions.

Do not force users to precisely click tiny icons.

---

# 76. RESPONSIVE BREAKPOINTS

Do not design around arbitrary framework defaults without considering actual layouts.

Define breakpoints based on content.

The interface should change when the content requires it.

---

# 77. TYPOGRAPHY STATES

Typography must communicate hierarchy through:

size
weight
opacity
spacing

Do not use color as the only hierarchy mechanism.

---

# 78. TEXT TRUNCATION

Long names must truncate gracefully.

Example:

`really-long-project-name-that-does...`

Hover or tooltip should reveal the full value.

Never allow long filenames to destroy layout.

---

# 79. MONOSPACE DATA

Use monospace selectively.

Appropriate:

timecodes
file paths
code
terminal
technical identifiers
frame numbers
AI operation IDs

Do not use monospace for all UI text.

---

# 80. ICON BUTTON LABELING

Every icon-only button must have:

accessible name
tooltip
keyboard focus

If the action is destructive or unusual, provide more context.

---

# 81. CONTEXTUAL ACTIONS

Do not expose every action globally.

Actions should appear near the thing they operate on.

Clip actions near clip.

Media actions near media.

Agent actions near agent.

Project actions near project.

This reduces cognitive load.

---

# 82. PROGRESSIVE DISCLOSURE

Show common controls first.

Advanced controls should be discoverable without dominating the interface.

Example:

Inspector:

Transform
Crop
Audio

Advanced:

Keyframes
Metadata
Processing

Do not expose every parameter at once.

---

# 83. DENSITY MODES

Support:

Comfortable

Compact

Potentially:

Ultra Compact

Density should modify:

row height
panel spacing
toolbar spacing
timeline height
text density

Do not alter the fundamental hierarchy.

---

# 84. THEME TRANSITIONS

Changing theme should transition smoothly.

Do not animate every individual element separately.

Use a short global transition.

Respect reduced motion.

---

# 85. ACCENT TRANSITIONS

Changing accent should update semantic tokens consistently.

Timeline

buttons

selection

focus

AI states

links

must all respond.

Do not leave random components with the previous color.

---

# 86. LIGHT MODE

If light mode exists, it must be deliberately designed.

Do not simply invert colors.

Light mode needs:

appropriate borders
surface hierarchy
text contrast
selection
hover
timeline contrast
AI states

Dark mode remains the primary AVID environment.

---

# 87. HIGH CONTRAST

Provide a high-contrast option.

It must preserve usability without destroying the design.

---

# 88. REDUCED MOTION

Respect system-level reduced-motion preferences.

Provide a user setting.

Reduced motion should remove:

parallax
large transforms
complex section transitions
decorative loops

Functional transitions remain.

---

# 89. AUDIO FEEDBACK

Sound should not be required for normal UI comprehension.

If AVID eventually has UI sounds:

They must be:

subtle
optional
configurable

Do not add sounds merely to make the application feel futuristic.

---

# 90. AUTOSAVE

Autosave status should be visible but quiet.

Example:

Saved

Saving...

Saved 4s ago

Do not interrupt the user.

---

# 91. UNSAVED CHANGES

If a screen has unsaved changes:

communicate it.

Example:

Project name *

or:

Unsaved changes

Provide:

Save
Discard
Cancel

Never silently discard meaningful user work.

---

# 92. EXITING / CLOSING

If the user attempts to close with unsaved important changes:

provide a clear confirmation.

Do not trap users unnecessarily.

---

# 93. ERROR RECOVERY

Every recoverable error should offer a recovery path.

Examples:

Retry

Choose another model

Reconnect

Open logs

Undo

Restore previous version

Do not leave users at:

"Something went wrong."

---

# 94. OFFLINE STATE

AVID is local-first.

The interface must communicate when the user is offline only when it matters.

Local editing should remain available.

Cloud model features should indicate unavailable state.

Example:

`Claude · Offline`

`Ollama · Available`

Do not make the entire application look broken because the network is unavailable.

---

# 95. MODEL FAILURE

If a cloud model fails:

Do not simply display an error.

Offer alternatives where appropriate:

Retry

Switch model

Use local model

Cancel

---

# 96. LONG-RUNNING AI TASKS

Long operations must survive navigation when possible.

The user should be able to leave the screen.

Global activity should communicate that the operation continues.

Example:

`AI Director · Analyzing 18 clips`

Clicking opens the operation.

---

# 97. CANCEL

Long-running operations should support cancellation when technically possible.

Cancellation must communicate:

Cancelling...

Cancelled

Do not pretend cancellation is instantaneous if the underlying process cannot stop immediately.

---

# 98. VERSION HISTORY

Important project states should be recoverable.

Version history should show:

timestamp
description
source
AI/user
changes

Do not create an overwhelming Git-style interface for ordinary users.

---

# 99. DEStructive ACTIONS

Destructive operations:

Delete project
Delete media
Discard changes
Remove timeline content

must have clear consequences.

Use confirmation when the action cannot be easily undone.

---

# 100. SUCCESS WITHOUT CELEBRATION

Do not use confetti.

Do not use huge checkmarks.

Success should feel professional.

Small confirmation.

Clear next action.

---

# 101. LANDING PAGE DOWNLOAD SECTION

The website must contain a dedicated download section.

Example structure:

```text
AVID

Available for

macOS        Windows        Linux

[ Download ] [ Download ]  [ Download ]

Apple Silicon
Intel
x64
ARM64
...
```

The actual architectures should reflect the release system.

Do not invent unsupported builds.

---

# 102. DOWNLOAD CTA HIERARCHY

Primary:

Download AVID

Secondary:

View releases

Tertiary:

Read installation guide

Do not make GitHub compete visually with the actual download.

---

# 103. WEBSITE NAVIGATION

Primary:

Product
Download
Docs
GitHub

Secondary:

Theme/demo controls if useful

Primary action:

Download

Do not overload the navigation.

---

# 104. WEBSITE INTERACTION QUALITY

Before calling the landing page complete:

Test:

Navigation

Scroll

Back

Forward

Deep links

Download

Video

Pause

Resume

Mute

Fullscreen

Theme

Mobile menu

Buttons

Hover

Focus

Keyboard

Reduced motion

---

# 105. APPLICATION INTERACTION QUALITY

Before calling the harness complete:

Test:

Navigation

Back

Forward

Panel resizing

Panel collapsing

Keyboard shortcuts

Command palette

Model selection

Theme changes

Agent state

Task state

Terminal

Git

Search

Dialogs

Toasts

Error recovery

Persistence

Restart

---

# 106. VISUAL REGRESSION

Capture screenshots at important states.

Compare:

default theme
custom theme
light theme if supported
high contrast
compact mode
mobile website
desktop website
empty state
loading state
error state
success state

Do not rely only on visual inspection in one state.

---

# 107. PRODUCT FEEL

The tiny details matter.

Examples:

A button should not shift when its label changes.

A panel should remember its width.

A tooltip should not cover the control it explains.

A dropdown should open toward available space.

A modal should trap focus correctly.

A back button should return to the meaningful previous context.

A loading state should explain what is actually happening.

An error should explain what the user can do.

A selected clip should remain visibly selected.

A model change should immediately update the current model indicator.

A theme change should update every relevant surface.

A user should never wonder whether an action succeeded.

These details are not polish.

They are the product.

---

# 108. OPEN CODE IMPLEMENTATION RULE

OpenCode must not make visual or UX assumptions that contradict this specification.

Before implementing any new screen:

1. Identify its place in the application hierarchy.
2. Identify its parent.
3. Identify its navigation path.
4. Identify how the user enters it.
5. Identify how the user leaves it.
6. Define primary action.
7. Define secondary actions.
8. Define empty state.
9. Define loading state.
10. Define error state.
11. Define success state.
12. Define hover states.
13. Define focus states.
14. Define disabled states.
15. Define keyboard interactions.
16. Define responsive behavior.
17. Define persistence requirements.
18. Define accessibility behavior.

Only then implement it.

---

# 109. NEW SCREEN CHECKLIST

A screen is incomplete if any of these are missing:

* layout
* spacing
* hierarchy
* navigation
* back behavior where appropriate
* loading
* empty
* error
* success
* hover
* active
* focus
* disabled
* keyboard
* responsive
* accessibility
* persistence
* animation
* reduced motion

---

# 110. FINAL ANTI-SLOP CHECK

Before accepting any UI implementation, ask:

Does this look like a generic AI-generated SaaS interface?

Does this look like a collection of rounded cards?

Did we add a gradient because the page felt empty?

Did we add a glow because the UI felt too plain?

Did we add an animation that doesn't explain anything?

Did we create fake statistics?

Did we invent fake testimonials?

Did we add unnecessary text?

Did we add a decorative illustration instead of showing the product?

Did we make the AI look magical instead of useful?

Did we make the application look like a dashboard instead of a workstation?

Did we use too many colors?

Did we hard-code colors instead of using theme tokens?

Did we make the interface impossible to customize?

Did we forget keyboard users?

Did we forget empty/error/loading states?

Did we forget how the user gets back?

Did we forget what happens when something fails?

Did we forget what happens when the user reloads?

Did we forget reduced motion?

Did we forget mobile?

Did we forget download states?

If yes to any of these, the implementation is not finished.

---

# 111. FINAL AVID EXPERIENCE PRINCIPLE

AVID should feel like someone cared about the tenth interaction, not just the first screenshot.

The first impression should be beautiful.

The tenth interaction should be predictable.

The hundredth interaction should be fast.

The thousandth interaction should still feel coherent.

That is the standard.

Do not optimize for Dribbble shots.

Do not optimize for AI-generated website aesthetics.

Do not optimize for visual novelty.

Optimize for:

clarity
speed
hierarchy
consistency
precision
control
discoverability
customization
accessibility
professionalism

The interface should disappear into the user's work.

That is the AVID experience.
# AVID DESIGN MASTER PROMPT

## UX, LAYOUT, INTERACTION, STATES AND MICRO-DETAIL SPECIFICATION

This document supplements the AVID visual design master specification.

This is not a suggestion list.

These are implementation rules.

Do not infer missing UX patterns from generic SaaS conventions.

Do not invent layouts.

Do not fill unspecified areas with cards, gradients, decorative illustrations, statistics, testimonials or generic marketing sections.

When implementing AVID, think like a senior product designer designing a real professional application.

The goal is not to produce something that looks impressive in a screenshot.

The goal is to produce an interface that feels deliberately designed after years of use.

Every spacing value, state, transition, button, navigation path and interaction should have an intentional reason.

---

# 1. THE PRODUCT EXPERIENCE

AVID consists of two connected experiences:

1. The landing site
2. The desktop application / harness

The website introduces the product.

The application performs the work.

The transition between them must feel continuous.

A user should never feel like they went from a carefully designed product into a generic dashboard.

---

# 2. DESIGN HIERARCHY

Every screen must have a clear hierarchy.

At any moment the user should be able to identify:

1. Where am I?
2. What am I looking at?
3. What is the primary action?
4. What is currently selected?
5. What changed?
6. What can I do next?
7. How do I go back?

Never present five equally prominent actions.

One action should be visually primary.

Secondary actions should be quieter.

Destructive actions should be clearly distinguished.

---

# 3. PAGE STRUCTURE

Every application screen must have a consistent structural hierarchy.

Preferred structure:

```text
Application chrome
    ↓
Context/navigation
    ↓
Page header
    ↓
Primary workspace
    ↓
Secondary information/actions
```

Do not randomly move headers, controls and navigation between screens.

Users should develop muscle memory.

---

# 4. SCREEN PADDING

Use deliberate screen padding.

Application:

Primary workspace:
16–24px

Dense utility areas:
8–16px

Page-level headers:
16–24px

Large empty states:
32–64px

Website:

Desktop content width:
1200–1440px maximum depending on section

Primary horizontal page padding:
32–64px

Large cinematic sections:
64–120px vertical spacing

Do not use enormous padding simply to make a website appear premium.

Whitespace must establish hierarchy.

---

# 5. SPACING SYSTEM

Use a 4px base unit.

Preferred values:

4px
8px
12px
16px
20px
24px
32px
40px
48px
64px
80px
96px
120px

Do not use arbitrary values such as:

17px
23px
37px
51px

unless required by a specific visual alignment.

Spacing must be consistent across the entire application.

---

# 6. SPACING HIERARCHY

Spacing communicates relationships.

Use:

4–8px:
label → control

8–12px:
related controls

12–16px:
related groups

20–24px:
separate groups

32–48px:
major sections

64px+:
major page-level separation

Do not separate elements that belong together.

Do not crowd unrelated elements together.

---

# 7. ALIGNMENT

Alignment must be intentional.

Primary content should align to a consistent grid.

Labels and controls should share predictable alignment.

Toolbars should align vertically.

Panel headers should align with their content.

Buttons should not randomly shift between screens.

Avoid "almost aligned" elements.

If two components appear related, their edges should usually align.

---

# 8. APPLICATION GRID

Use a consistent desktop grid.

Default conceptual structure:

```text
┌──────┬───────────────────────────────┬────────────┐
│      │                               │            │
│ NAV  │        PRIMARY WORKSPACE     │ INSPECTOR  │
│      │                               │            │
│      │                               │            │
└──────┴───────────────────────────────┴────────────┘
```

Optional bottom region:

```text
┌──────┬───────────────────────────────┬────────────┐
│ NAV  │          WORKSPACE             │ INSPECTOR  │
├──────┴───────────────────────────────┴────────────┤
│                 EVENTS / TERMINAL                  │
└────────────────────────────────────────────────────┘
```

The center is always the primary surface.

---

# 9. RESIZABLE PANELS

Panels must be resizable where their content benefits from resizing.

Resizable boundaries should have:

* subtle hover state
* larger invisible hit target
* visual cursor change
* smooth resize
* minimum width
* maximum width

Do not require pixel-perfect dragging.

Remember panel dimensions between sessions.

Allow reset to default layout.

---

# 10. COLLAPSIBLE PANELS

Users should be able to collapse:

Navigation
Inspector
AI Director
Terminal
Secondary panels

Collapsed states must preserve discoverability.

Never make a panel disappear without leaving an obvious way to restore it.

---

# 11. PANEL DIVIDERS

Panel dividers are structural.

Default:

1px

They should be subtle.

On hover:

increase contrast slightly

On active resize:

increase contrast again

Do not use bright neon borders.

---

# 12. BACK BUTTONS

Back navigation must be deliberate.

Any screen that represents a child context should provide a way back.

Examples:

Projects → Project

Project → Agent Session

Agent Session → Task

Settings → Appearance

Appearance → Theme Editor

Research → Research Item

Download → Release Details

Use a consistent back pattern.

Example:

`← Project`

not merely a mysterious arrow icon.

On compact screens:

`←`

with an accessible tooltip.

Do not place back buttons randomly.

Preferred location:

top-left of the contextual content header.

Do not show a back button when browser/application navigation already clearly communicates the hierarchy and an additional button would be redundant.

---

# 13. BREADCRUMBS

Use breadcrumbs when hierarchy becomes deep.

Example:

Projects / AVID / Agents / Architect

Do not use breadcrumbs for every page.

Do not turn them into giant navigation bars.

They should establish context.

---

# 14. PAGE HEADERS

Every substantial screen should have a clear page header.

Header should contain:

Context

Title

Optional description

Primary action

Secondary actions

Optional status

Example:

```text
← Projects

AVID Editor
Build the next timeline interaction.

[ Open Project ]   [ ... ]
```

Do not create a huge marketing-style heading inside the application.

---

# 15. HEADER HEIGHT

Application page headers should be compact.

Target:

48–64px

Do not consume 120px of vertical space with application chrome.

The work must remain visible.

---

# 16. TOOLBARS

Toolbars must group related actions.

Example:

```text
[ Select ] [ Cut ] [ Trim ] | [ Undo ] [ Redo ] | [ ... ]
```

Use separators between conceptual groups.

Do not put every available action in the primary toolbar.

Rare actions belong in menus or command palette.

---

# 17. TOOLBAR STATES

Every toolbar control must support:

Default
Hover
Active
Pressed
Disabled
Focused
Loading where appropriate

Do not implement only the default state.

---

# 18. HOVER STATES

Hover should provide immediate feedback without becoming distracting.

Preferred behavior:

background contrast changes slightly

or

border becomes slightly more visible

or

icon/text contrast increases

Do not:

scale buttons dramatically

glow

bounce

rotate icons

move layout elements

change colors excessively

---

# 19. ACTIVE STATES

Active state must be visually distinct from hover.

Use:

* stronger contrast
* subtle background
* accent
* inset indicator
* underline where appropriate

Do not rely on color alone.

---

# 20. FOCUS STATES

Keyboard focus must always be visible.

Use a clear focus ring.

Focus should not alter layout.

Focus state must work in every theme.

Do not remove browser/native focus indicators without replacing them with a better accessible equivalent.

---

# 21. DISABLED STATES

Disabled controls:

* reduce contrast
* remain recognizable
* do not disappear
* do not trigger hover effects

Do not make disabled elements completely invisible.

---

# 22. PRESSED STATES

Buttons should communicate when physically pressed.

Use a subtle change in:

background
border
contrast

Do not use dramatic scaling.

---

# 23. LOADING STATES

Loading must preserve layout.

Never allow a button to jump because its text changed.

Example:

`Export`

becomes:

`Exporting…`

inside the same button width where possible.

For longer operations use:

progress
percentage
estimated stage
cancel

---

# 24. SKELETONS

Use skeletons only where content structure is known.

Do not create decorative skeleton loaders everywhere.

Skeletons should resemble the final content.

Do not animate every skeleton aggressively.

---

# 25. EMPTY STATES

Empty states must answer:

Why is this empty?

What should I do?

Example:

```text
No projects yet.

Create a project or import an existing AVID project.

[ New Project ]  [ Open Project ]
```

Never use:

"Nothing here yet ✨"

---

# 26. FIRST-RUN EXPERIENCE

On first launch, AVID should not dump the user into a complicated interface without context.

The first-run experience should establish:

Create project
Open project
Import project

Optional:

Choose theme
Choose model provider
Choose default AI model

Do not create a long onboarding slideshow.

Professional software should get users to the work quickly.

---

# 27. RECENT PROJECTS

Home should prioritize:

Recent projects

Pinned projects

New project

Open project

Import project

Do not prioritize decorative dashboard metrics.

---

# 28. PROJECT CREATION

New project should clearly ask for:

Project name

Location

Resolution

Frame rate

Aspect ratio where necessary

Optional template

Optional model configuration

Defaults should be sensible.

Do not force users through ten setup screens.

---

# 29. CONFIRMATION DIALOGS

Do not ask confirmation for every action.

Use confirmation when an action is:

destructive
difficult to undo
irreversible
potentially data-loss causing

Do not show:

"Are you sure?"

for normal reversible actions.

---

# 30. DIALOG DESIGN

Dialogs must have:

Title
Context
Primary action
Secondary/cancel action

Destructive action should be visually clear.

Do not place two equally strong buttons beside each other when one is clearly the intended action.

Escape closes dismissible dialogs.

Enter activates the primary action where safe.

---

# 31. TOOLTIPS

Tooltips are required for icon-only controls.

Tooltip should include:

Name

Optional shortcut

Example:

`Split Clip`

`Cmd + B`

Do not use tooltips for controls whose purpose is already obvious and labeled.

Tooltip timing should be consistent.

Do not show tooltips immediately when the cursor briefly crosses an icon.

---

# 32. KEYBOARD SHORTCUTS

AVID must be keyboard-first.

Every major operation should have discoverable shortcuts.

Show shortcuts in:

tooltips
menus
command palette
settings

Users must be able to customize shortcuts.

---

# 33. COMMAND PALETTE UX

Cmd/Ctrl + K opens the command palette.

Behavior:

Open
Search
Filter
Keyboard navigate
Enter to execute
Escape to close

Recent commands may appear.

Commands should be grouped by context.

Example:

EDIT

Cut
Split
Trim
Delete

PROJECT

Open
Save
Export

AI

Analyze
Plan Edit
Apply Changes

---

# 34. COMMAND PALETTE VISUAL DESIGN

It should be compact.

Centered or contextually positioned.

Do not make it a giant full-screen modal.

Use a strong search field.

Results should be highly scannable.

Selected result must be obvious.

---

# 35. NOTIFICATIONS

Use toasts for:

successful background operations
completed exports
completed AI operations
non-critical errors

Do not use toasts for information users must act on.

Toasts should not stack infinitely.

Allow dismissal.

Do not interrupt video playback with unnecessary notifications.

---

# 36. TOAST POSITION

Default:

bottom-right

unless that conflicts with important controls.

Toasts should not obscure:

timeline
transport controls
AI Director
primary CTA

---

# 37. CONTEXT MENUS

Right-click menus should expose contextual actions.

Timeline clip:

Cut
Copy
Paste
Split
Trim
Delete
Replace
Properties

Media:

Open
Reveal
Rename
Analyze
Add to timeline

Do not show irrelevant actions.

---

# 38. DRAG AND DROP

Drag interactions must communicate:

valid target
invalid target
active target
drop location

Use a clear insertion indicator.

Do not allow ambiguous drops.

---

# 39. DRAG FEEDBACK

During drag:

show ghost preview where useful

show insertion line

highlight destination

keep original object understandable

Do not make the dragged object disappear.

---

# 40. SELECTION

Selection must be obvious.

Selected timeline clip:

clear border/background distinction

Selected media:

clear selection indicator

Selected transcript segment:

clear highlight

Selection should work in monochrome mode.

---

# 41. MULTI-SELECTION

Support modifier-key selection where appropriate.

Show count where useful:

`4 clips selected`

Do not overwhelm the interface with selection metadata.

---

# 42. UNDO / REDO

Undo is a core AVID interaction.

Cmd/Ctrl + Z

Cmd/Ctrl + Shift + Z

AI operations must participate in undo/redo.

Users must be able to undo an AI edit as a coherent operation where appropriate.

Example:

`Undo AI edit`

not:

`Undo 47 internal mutations`

---

# 43. AI OPERATIONS

AI must never silently modify important user work.

AI should:

Analyze
Plan
Propose
Review
Apply

When appropriate.

The interface should make it clear whether an AI action is:

Suggested
Previewed
Applied

---

# 44. AI CHANGE REVIEW

When AI proposes changes:

show a review state.

Example:

```text
AI DIRECTOR

5 changes proposed

01 Remove 00:42–00:47
02 Tighten opening
03 Add diagram
04 Reframe speaker
05 Add captions

[ Apply all ] [ Review ]
```

Users should be able to inspect individual operations.

---

# 45. AI PROVENANCE

AI-generated material should have subtle provenance.

Do not add giant "AI GENERATED" labels.

Use:

small icon
subtle indicator
metadata
contextual status

The UI should remain clean.

---

# 46. MODEL SELECTOR

The model selector is a high-quality control, not a decorative badge.

Structure:

Provider identity

Model

Availability

Local/cloud state

Optional context/capability information

Example:

```text
[ provider icon ]  Claude
                    Sonnet
                    Cloud
```

Hover:

show additional context.

Click:

open model selector.

---

# 47. MODEL SELECTOR MENU

Group by provider.

Example:

ANTHROPIC

Claude Sonnet
Claude Opus

GOOGLE

Gemini ...

OPENAI

...

LOCAL

Ollama
Local models

Each model may display:

availability
local/cloud
capabilities

Do not overload the menu with technical information users do not need.

---

# 48. PROVIDER ICONS

Use official provider marks when appropriate.

Do not recolor official logos arbitrarily.

Keep them visually restrained.

The provider icon is a recognition aid.

It is not the visual centerpiece.

---

# 49. AGENT STATUS

Agent status must be understandable without color.

Example:

● Running
✓ Complete
○ Idle
! Attention
× Failed

Use shape/icon + text.

Never rely solely on red/green.

---

# 50. AGENT ACTIVITY

Running agents should expose useful information:

Current operation

Files being touched

Elapsed time

Tool activity

Current model

Progress where measurable

Do not display fake progress bars.

---

# 51. TERMINAL STATES

Terminal:

idle
running
success
warning
error

Use semantic color sparingly.

Monospace typography.

Selectable text.

Copy button.

Clear button where appropriate.

Search.

---

# 52. FILE STATES

Files should support:

normal
selected
modified
new
deleted
conflicted
ignored

Use icons and subtle state indicators.

---

# 53. GIT STATES

Git status should be immediately understandable.

Examples:

Clean

Modified

Ahead

Behind

Conflict

Uncommitted changes

Do not make the Git indicator visually dominant unless action is required.

---

# 54. SEARCH UX

Search must:

respond quickly
support keyboard navigation
preserve query
support filtering
show no-result state
show loading state when required

No result:

"No matches for 'kafka'."

Optionally:

Search files
Search commands
Search projects

---

# 55. NAVIGATION PERSISTENCE

Remember:

panel widths
collapsed panels
selected workspace where appropriate
theme
density
keyboard configuration

Do not unexpectedly reset the workspace on every launch.

---

# 56. SCROLL POSITION

Preserve scroll position when navigating back where appropriate.

If the user opens an item and returns, do not always dump them at the top of the previous page.

---

# 57. BACK NAVIGATION BEHAVIOR

Back should return to the previous meaningful context.

Example:

Projects
→ AVID
→ Agent
→ Task
→ Back

returns to Agent.

Not Home.

Navigation should feel hierarchical.

---

# 58. DEEP LINKING

Screens should have stable routes where appropriate.

Example:

/project/avid
/project/avid/agents
/project/avid/agents/architect
/project/avid/tasks/123

This makes navigation predictable.

---

# 59. URL / ROUTE STATES

Refreshing a screen should preserve the current context when technically possible.

Do not make users lose their place simply because the application refreshed.

---

# 60. LANDING PAGE LAYOUT

The landing page should use a strong vertical rhythm.

Recommended structure:

```text
HEADER

HERO

PRODUCT DEMONSTRATION

PRODUCT VIDEO

SCRIPT → STORYBOARD → TIMELINE

UNDERSTANDING FOOTAGE

EXPLAIN → VISUALIZE

AI DIRECTOR

EDITABLE AI

MODEL FREEDOM

PROFESSIONAL EDITING

DOWNLOAD

FINAL CTA

FOOTER
```

Each section should have a distinct purpose.

---

# 61. LANDING PAGE WIDTH

Do not allow text to stretch across the entire screen.

Reading width:

approximately 600–800px

Product demonstrations may use:

1200–1600px

Cinematic video may use:

full viewport width

Use width strategically.

---

# 62. LANDING PAGE SECTION SPACING

Small section:

64–96px

Standard:

120px

Major product demonstration:

160px+

Cinematic transition:

variable based on animation

Do not give every section exactly the same height.

---

# 63. LANDING PAGE HERO HEIGHT

The hero should occupy most of the first viewport.

But the product must appear within the first viewport.

Do not force the user to scroll through 600px of headline text before seeing AVID.

---

# 64. LANDING PAGE HERO MOTION

The hero animation must have a predefined sequence.

Do not allow implementation to invent random entrance animations.

Sequence:

Logo

→ statement

→ editor frame

→ media

→ viewer

→ timeline

→ AI Director

→ model selector

→ playback

→ AI instruction

→ timeline change

→ result

Every transition should visually explain the product.

---

# 65. LANDING PAGE SCROLL INTERACTIONS

Use scroll as a storytelling mechanism.

When a section is pinned:

the content must continue progressing.

Do not create scroll effects that feel like a website fighting the user.

Do not hijack scrolling.

Use natural scroll with controlled animation.

---

# 66. SECTION ENTRY ANIMATION

Do not use the same:

fade up
fade up
fade up
fade up

animation for every section.

This is a major anti-slop rule.

Instead, animation should correspond to the section's concept.

Script:

text transforms

Timeline:

tracks assemble

AI:

operations appear

Video:

viewer expands

Models:

provider marks resolve

Download:

platform options reveal

---

# 67. LANDING PAGE HOVER STATES

Interactive product UI on the website should behave like the real application.

If a timeline clip is hoverable:

show hover state.

If a model is selectable:

show selection.

If a button exists:

show button states.

Do not create fake UI that has no interaction.

---

# 68. LANDING PAGE PRODUCT DEMO

Where possible, use real AVID UI components.

Do not build a completely separate fake editor for the website.

The website demonstration should visually derive from the actual application design system.

---

# 69. DOWNLOAD EXPERIENCE

Downloads are first-class.

Supported platforms:

macOS
Windows
Linux

Header:

Download

Hero:

platform-aware download CTA

Dedicated download page:

`/download`

---

# 70. PLATFORM DETECTION

Detect the user's platform when possible.

Example:

macOS user:

Download for macOS

Windows user:

Download for Windows

Linux user:

Download for Linux

Always provide:

Other platforms

---

# 71. DOWNLOAD PAGE

The download page should clearly communicate:

Platform

Architecture

Minimum supported version

Current version

File size where useful

Release notes

Checksums/signature information when available

Installation instructions

Older releases

Do not make users hunt for the actual download button.

---

# 72. DOWNLOAD BUTTON STATES

Download buttons:

Default
Hover
Pressed
Downloading
Downloaded
Unavailable

If an installer is unavailable:

explain why.

Never display a button that does nothing.

---

# 73. WEBSITE FOOTER

Minimal.

Include:

AVID

Product
Download
Documentation
GitHub
Community where actually available
Privacy
Terms
Contact

Do not create 40 footer links.

---

# 74. MOBILE WEBSITE

On mobile:

Navigation collapses.

Product demonstrations simplify.

Timeline may become a focused crop.

Video remains central.

Text remains readable.

Buttons become thumb-friendly.

Do not attempt to display the entire desktop editor at full complexity.

---

# 75. TOUCH TARGETS

Interactive controls should have sufficiently large hit areas.

Small visual icons may have larger invisible interaction regions.

Do not force users to precisely click tiny icons.

---

# 76. RESPONSIVE BREAKPOINTS

Do not design around arbitrary framework defaults without considering actual layouts.

Define breakpoints based on content.

The interface should change when the content requires it.

---

# 77. TYPOGRAPHY STATES

Typography must communicate hierarchy through:

size
weight
opacity
spacing

Do not use color as the only hierarchy mechanism.

---

# 78. TEXT TRUNCATION

Long names must truncate gracefully.

Example:

`really-long-project-name-that-does...`

Hover or tooltip should reveal the full value.

Never allow long filenames to destroy layout.

---

# 79. MONOSPACE DATA

Use monospace selectively.

Appropriate:

timecodes
file paths
code
terminal
technical identifiers
frame numbers
AI operation IDs

Do not use monospace for all UI text.

---

# 80. ICON BUTTON LABELING

Every icon-only button must have:

accessible name
tooltip
keyboard focus

If the action is destructive or unusual, provide more context.

---

# 81. CONTEXTUAL ACTIONS

Do not expose every action globally.

Actions should appear near the thing they operate on.

Clip actions near clip.

Media actions near media.

Agent actions near agent.

Project actions near project.

This reduces cognitive load.

---

# 82. PROGRESSIVE DISCLOSURE

Show common controls first.

Advanced controls should be discoverable without dominating the interface.

Example:

Inspector:

Transform
Crop
Audio

Advanced:

Keyframes
Metadata
Processing

Do not expose every parameter at once.

---

# 83. DENSITY MODES

Support:

Comfortable

Compact

Potentially:

Ultra Compact

Density should modify:

row height
panel spacing
toolbar spacing
timeline height
text density

Do not alter the fundamental hierarchy.

---

# 84. THEME TRANSITIONS

Changing theme should transition smoothly.

Do not animate every individual element separately.

Use a short global transition.

Respect reduced motion.

---

# 85. ACCENT TRANSITIONS

Changing accent should update semantic tokens consistently.

Timeline

buttons

selection

focus

AI states

links

must all respond.

Do not leave random components with the previous color.

---

# 86. LIGHT MODE

If light mode exists, it must be deliberately designed.

Do not simply invert colors.

Light mode needs:

appropriate borders
surface hierarchy
text contrast
selection
hover
timeline contrast
AI states

Dark mode remains the primary AVID environment.

---

# 87. HIGH CONTRAST

Provide a high-contrast option.

It must preserve usability without destroying the design.

---

# 88. REDUCED MOTION

Respect system-level reduced-motion preferences.

Provide a user setting.

Reduced motion should remove:

parallax
large transforms
complex section transitions
decorative loops

Functional transitions remain.

---

# 89. AUDIO FEEDBACK

Sound should not be required for normal UI comprehension.

If AVID eventually has UI sounds:

They must be:

subtle
optional
configurable

Do not add sounds merely to make the application feel futuristic.

---

# 90. AUTOSAVE

Autosave status should be visible but quiet.

Example:

Saved

Saving...

Saved 4s ago

Do not interrupt the user.

---

# 91. UNSAVED CHANGES

If a screen has unsaved changes:

communicate it.

Example:

Project name *

or:

Unsaved changes

Provide:

Save
Discard
Cancel

Never silently discard meaningful user work.

---

# 92. EXITING / CLOSING

If the user attempts to close with unsaved important changes:

provide a clear confirmation.

Do not trap users unnecessarily.

---

# 93. ERROR RECOVERY

Every recoverable error should offer a recovery path.

Examples:

Retry

Choose another model

Reconnect

Open logs

Undo

Restore previous version

Do not leave users at:

"Something went wrong."

---

# 94. OFFLINE STATE

AVID is local-first.

The interface must communicate when the user is offline only when it matters.

Local editing should remain available.

Cloud model features should indicate unavailable state.

Example:

`Claude · Offline`

`Ollama · Available`

Do not make the entire application look broken because the network is unavailable.

---

# 95. MODEL FAILURE

If a cloud model fails:

Do not simply display an error.

Offer alternatives where appropriate:

Retry

Switch model

Use local model

Cancel

---

# 96. LONG-RUNNING AI TASKS

Long operations must survive navigation when possible.

The user should be able to leave the screen.

Global activity should communicate that the operation continues.

Example:

`AI Director · Analyzing 18 clips`

Clicking opens the operation.

---

# 97. CANCEL

Long-running operations should support cancellation when technically possible.

Cancellation must communicate:

Cancelling...

Cancelled

Do not pretend cancellation is instantaneous if the underlying process cannot stop immediately.

---

# 98. VERSION HISTORY

Important project states should be recoverable.

Version history should show:

timestamp
description
source
AI/user
changes

Do not create an overwhelming Git-style interface for ordinary users.

---

# 99. DEStructive ACTIONS

Destructive operations:

Delete project
Delete media
Discard changes
Remove timeline content

must have clear consequences.

Use confirmation when the action cannot be easily undone.

---

# 100. SUCCESS WITHOUT CELEBRATION

Do not use confetti.

Do not use huge checkmarks.

Success should feel professional.

Small confirmation.

Clear next action.

---

# 101. LANDING PAGE DOWNLOAD SECTION

The website must contain a dedicated download section.

Example structure:

```text
AVID

Available for

macOS        Windows        Linux

[ Download ] [ Download ]  [ Download ]

Apple Silicon
Intel
x64
ARM64
...
```

The actual architectures should reflect the release system.

Do not invent unsupported builds.

---

# 102. DOWNLOAD CTA HIERARCHY

Primary:

Download AVID

Secondary:

View releases

Tertiary:

Read installation guide

Do not make GitHub compete visually with the actual download.

---

# 103. WEBSITE NAVIGATION

Primary:

Product
Download
Docs
GitHub

Secondary:

Theme/demo controls if useful

Primary action:

Download

Do not overload the navigation.

---

# 104. WEBSITE INTERACTION QUALITY

Before calling the landing page complete:

Test:

Navigation

Scroll

Back

Forward

Deep links

Download

Video

Pause

Resume

Mute

Fullscreen

Theme

Mobile menu

Buttons

Hover

Focus

Keyboard

Reduced motion

---

# 105. APPLICATION INTERACTION QUALITY

Before calling the harness complete:

Test:

Navigation

Back

Forward

Panel resizing

Panel collapsing

Keyboard shortcuts

Command palette

Model selection

Theme changes

Agent state

Task state

Terminal

Git

Search

Dialogs

Toasts

Error recovery

Persistence

Restart

---

# 106. VISUAL REGRESSION

Capture screenshots at important states.

Compare:

default theme
custom theme
light theme if supported
high contrast
compact mode
mobile website
desktop website
empty state
loading state
error state
success state

Do not rely only on visual inspection in one state.

---

# 107. PRODUCT FEEL

The tiny details matter.

Examples:

A button should not shift when its label changes.

A panel should remember its width.

A tooltip should not cover the control it explains.

A dropdown should open toward available space.

A modal should trap focus correctly.

A back button should return to the meaningful previous context.

A loading state should explain what is actually happening.

An error should explain what the user can do.

A selected clip should remain visibly selected.

A model change should immediately update the current model indicator.

A theme change should update every relevant surface.

A user should never wonder whether an action succeeded.

These details are not polish.

They are the product.

---

# 108. OPEN CODE IMPLEMENTATION RULE

OpenCode must not make visual or UX assumptions that contradict this specification.

Before implementing any new screen:

1. Identify its place in the application hierarchy.
2. Identify its parent.
3. Identify its navigation path.
4. Identify how the user enters it.
5. Identify how the user leaves it.
6. Define primary action.
7. Define secondary actions.
8. Define empty state.
9. Define loading state.
10. Define error state.
11. Define success state.
12. Define hover states.
13. Define focus states.
14. Define disabled states.
15. Define keyboard interactions.
16. Define responsive behavior.
17. Define persistence requirements.
18. Define accessibility behavior.

Only then implement it.

---

# 109. NEW SCREEN CHECKLIST

A screen is incomplete if any of these are missing:

* layout
* spacing
* hierarchy
* navigation
* back behavior where appropriate
* loading
* empty
* error
* success
* hover
* active
* focus
* disabled
* keyboard
* responsive
* accessibility
* persistence
* animation
* reduced motion

---

# 110. FINAL ANTI-SLOP CHECK

Before accepting any UI implementation, ask:

Does this look like a generic AI-generated SaaS interface?

Does this look like a collection of rounded cards?

Did we add a gradient because the page felt empty?

Did we add a glow because the UI felt too plain?

Did we add an animation that doesn't explain anything?

Did we create fake statistics?

Did we invent fake testimonials?

Did we add unnecessary text?

Did we add a decorative illustration instead of showing the product?

Did we make the AI look magical instead of useful?

Did we make the application look like a dashboard instead of a workstation?

Did we use too many colors?

Did we hard-code colors instead of using theme tokens?

Did we make the interface impossible to customize?

Did we forget keyboard users?

Did we forget empty/error/loading states?

Did we forget how the user gets back?

Did we forget what happens when something fails?

Did we forget what happens when the user reloads?

Did we forget reduced motion?

Did we forget mobile?

Did we forget download states?

If yes to any of these, the implementation is not finished.

---

# 111. FINAL AVID EXPERIENCE PRINCIPLE

AVID should feel like someone cared about the tenth interaction, not just the first screenshot.

The first impression should be beautiful.

The tenth interaction should be predictable.

The hundredth interaction should be fast.

The thousandth interaction should still feel coherent.

That is the standard.

Do not optimize for Dribbble shots.

Do not optimize for AI-generated website aesthetics.

Do not optimize for visual novelty.

Optimize for:

clarity
speed
hierarchy
consistency
precision
control
discoverability
customization
accessibility
professionalism

The interface should disappear into the user's work.

That is the AVID experience.

