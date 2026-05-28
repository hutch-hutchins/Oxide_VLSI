# Oxide VLSI

## Concept

**Oxide VLSI** is a proposed open-source, Rust-based, teaching-first VLSI design environment inspired by the role Electric VLSI has historically played in university courses.

The goal is not to replace Cadence Virtuoso, Synopsys Custom Compiler, or other commercial signoff tools. The goal is to provide a clean, modern, cross-platform environment for students and instructors who need to teach and learn transistor-level CMOS design without relying on expensive commercial EDA licenses or clunky legacy tooling.

A concise project description:

> Oxide VLSI is a lightweight, open-source IC design environment for teaching digital VLSI design. It provides schematic capture, transistor-level layout, design-rule checking, SPICE simulation export, standard-cell exercises, and GDS export through a modern Rust GUI.

The intended role is similar to what OARS does for RISC-V assembly education: take a workflow that exists, remove the worst usability friction, and make the tool approachable enough for students while still being technically meaningful.

---

## Core Motivation

Many schools cannot afford full Cadence/Synopsys flows for every student or every course. Even when commercial tools are available, they are often too heavy for early undergraduate VLSI education.

Open-source tools exist and are powerful, but the ecosystem is fragmented:

- **Electric VLSI** is free and historically useful for teaching, but the interface feels old and clunky.
- **Magic** is powerful and historically important, but not beginner-friendly.
- **KLayout** is excellent for GDS viewing, layout inspection, scripting, DRC, and related workflows, but it is not primarily a teaching-first design environment.
- **OpenROAD/OpenLane** provide RTL-to-GDS flows, but they are aimed more at digital implementation than transistor-level educational layout.
- **Ngspice** provides circuit simulation, but students still need an environment that helps them generate, run, and interpret simulations.
- **GDS/PDK/open-source ASIC flows** are available, but using them effectively requires substantial toolchain knowledge.

The gap is not “there are no VLSI tools.”

The gap is:

> There is no modern, friendly, classroom-oriented tool that lets students draw CMOS layouts, understand design rules, extract connectivity, compare layout to schematic intent, simulate simple cells, export reports, and learn why their designs are wrong.

That is the opportunity.

---

## Project Positioning

### Good Positioning

> A modern, open-source VLSI teaching tool focused on CMOS layout, verification, and student-friendly workflows.

> An Electric-inspired educational VLSI environment written in Rust.

> A lightweight CMOS cell design studio for universities, hobbyists, and open-source silicon learners.

> A teaching-first layout and verification tool for transistor-level VLSI labs.

### Bad Positioning

> A Cadence replacement.

> A Synopsys replacement.

> A full commercial ASIC signoff tool.

> A complete replacement for Magic, KLayout, OpenROAD, and Electric on day one.

This project should be ambitious, but the messaging needs to be precise. The tool is for education, small cells, labs, reports, and open-source workflows. It is not intended for production signoff.

---

## Target Users

### Primary Users

- Undergraduate digital VLSI students
- Graduate students learning IC layout basics
- Instructors teaching CMOS design
- Schools without access to commercial EDA licenses
- Students doing small layout/report-based labs

### Secondary Users

- Hobbyist open-source silicon learners
- Small research groups
- People experimenting with Sky130 or similar educational PDKs
- Instructors building lab handouts and grading flows
- Open-source hardware contributors needing simple visual explanations

---

## What This Should Replace

The classroom pain of:

```text
Install Electric
Fight Java or old UI issues
Learn a dated interface
Draw transistors and cells
Run checks with limited feedback
Export files for reports
Try to connect the work to modern open-source flows
Explain DRC/LVS concepts manually
Grade layout submissions by hand
```

with:

```text
Install one app
Open a lab template
Draw schematic
Draw layout
Run DRC
Extract connectivity
Compare layout to schematic
Run or export SPICE simulation
Export GDS/SVG/PNG/report artifacts
Get understandable feedback
```

---

## Design Philosophy

Oxide VLSI should follow the same philosophy that made OARS compelling:

1. **Teaching first**
   - Every error should help the student understand the underlying hardware/layout concept.
   - The tool should explain, not merely complain.

2. **Small, focused, and useful**
   - Start with transistor-level CMOS cells.
   - Avoid trying to become a complete ASIC flow immediately.

3. **Cross-platform and easy to install**
   - Prefer a single binary where possible.
   - Avoid complex first-run setup.

4. **Modern UI**
   - Clean canvas.
   - Clear layer palette.
   - Useful error panels.
   - Simple project navigation.
   - Good zoom/pan/selection behavior.

5. **Interoperable**
   - Export GDSII.
   - Export SPICE.
   - Export SVG/PNG for reports.
   - Eventually integrate with KLayout, Magic, Ngspice, OpenROAD, and OpenLane.

6. **Instructor-friendly**
   - Built-in labs.
   - Lab definition files.
   - Grading mode.
   - Report export.
   - Deterministic checks.

7. **Avoid vendor-tool complexity**
   - Do not clone Virtuoso.
   - Build the subset that makes sense for education.

---

## Proposed Product Name Options

Possible names:

- Oxide VLSI
- Silicon Studio
- OpenCell Studio
- ForgeVLSI
- RustIC
- Lambda Studio
- Transistor Lab
- ChipForge
- CellForge
- Oxide Cell Studio

The strongest names are probably:

1. **Oxide VLSI**
2. **Silicon Studio**
3. **OpenCell Studio**
4. **ChipForge**

**Oxide VLSI** fits especially well if the broader ecosystem includes OARS or other Rust/engineering education tools.

---

## Main Workspaces

The application should be organized around four major workspaces.

---

# 1. Schematic Workspace

The schematic workspace allows students to create transistor-level and simple gate-level schematics.

## Supported Early Primitives

- nMOS transistor
- pMOS transistor
- resistor
- capacitor
- voltage source
- current source
- ground
- VDD
- input pin
- output pin
- bidirectional pin
- labels
- wires
- hierarchical symbols

## Useful Built-In Symbols

- Inverter
- NAND
- NOR
- Transmission gate
- Tri-state inverter
- D latch
- D flip-flop
- Pull-up/pull-down network symbols

## Outputs

The schematic workspace should support:

- Internal schematic representation
- SPICE netlist export
- Schematic image export
- Hierarchical cell definitions
- Netlist comparison against extracted layout

## Educational Role

Students should be able to see the intended circuit before drawing physical layout. This allows the tool to later compare:

```text
Schematic intent
vs.
Extracted layout behavior/connectivity
```

This is the foundation for LVS-style educational feedback.

---

# 2. Layout Workspace

The layout workspace is the heart of the tool.

Students draw physical layout using process layers and snapping rules.

## Early Layout Primitives

- Rectangles
- Paths/wires
- Contacts
- Vias
- Text labels
- Pins
- Instances/cells
- Rulers
- Measurement annotations
- Alignment guides

## Early Educational Layers

For a lambda-based CMOS teaching process:

- n-well
- p-well or substrate marker
- active/diffusion
- n-diffusion
- p-diffusion
- polysilicon
- contact
- metal1
- via1
- metal2
- label/pin layer

The exact layer model can start simple and become more process-aware later.

## Basic Layout Features

- Grid snapping
- Lambda grid
- Pan/zoom
- Rectangle drawing
- Path drawing
- Layer visibility toggles
- Layer locking
- Selection
- Move/resize/delete
- Copy/paste
- Undo/redo
- Ruler/measurement tool
- Net highlighting
- Cell hierarchy

## Required Exports

- Internal project file
- SVG for reports
- PNG for reports
- GDSII for tool interoperability

---

# 3. Verification Workspace

This is where Oxide VLSI becomes more than a drawing program.

The verification workspace should help students answer:

- Did I violate any design rules?
- Are my transistors recognized correctly?
- Are the expected nets present?
- Did I short VDD and GND?
- Does the extracted layout match the schematic?
- Why is the layout wrong?

## Early Checks

### DRC: Design Rule Checking

Initial lambda-rule checks:

- Minimum width
- Minimum spacing
- Minimum enclosure
- Contact size
- Via size
- Metal overlap of contact/via
- Poly spacing
- Poly width
- Active spacing
- Well enclosure
- pMOS must be inside n-well
- nMOS must be in the appropriate substrate/well region
- Missing or invalid contacts
- Obvious shape overlap violations

Example feedback:

```text
DRC Error: Poly width too small

Location:
  x = 14λ, y = 22λ

Required:
  poly width >= 2λ

Found:
  poly width = 1λ

Explanation:
  Polysilicon must be wide enough to reliably fabricate. Increase the
  width of this poly shape or redraw it using the layout grid.
```

### Connectivity Extraction

The tool should infer nets from connected geometry.

Examples:

- Metal1 shapes touching each other are the same net.
- Contact connects metal1 to active.
- Via connects metal1 to metal2.
- Poly crossing active forms a transistor gate.
- Labels assign human-readable net names.
- Power/ground rails can be recognized by labels.

Example feedback:

```text
Connectivity Warning: Floating gate

The gate of transistor M2 is not connected to any named input or internal net.

Likely issue:
  The polysilicon gate is not connected to the input route.
```

### Device Extraction

Early transistor recognition can use simple geometry rules:

```text
poly crossing active = transistor
active type + well context = nMOS or pMOS
active regions on either side = source/drain terminals
poly shape = gate terminal
well/substrate context = body terminal
```

Extracted devices:

```text
M1: pMOS
  gate = A
  source = VDD
  drain = Y
  body = VDD

M2: nMOS
  gate = A
  source = GND
  drain = Y
  body = GND
```

### LVS-Style Comparison

The tool should eventually compare the schematic netlist against the extracted layout netlist.

Example feedback:

```text
LVS Mismatch

Schematic:
  pMOS source connected to VDD
  nMOS source connected to GND
  pMOS and nMOS drains connected to Y
  both gates connected to A

Layout:
  pMOS source connected to VDD
  nMOS source connected to GND
  pMOS drain connected to Y
  nMOS drain connected to GND

Likely issue:
  The nMOS drain is connected to GND instead of the output node Y.
```

This type of explanation is one of the most valuable features.

---

# 4. Flow Workspace

The flow workspace integrates with existing tools instead of replacing them.

Possible backends:

- **Ngspice**
  - SPICE simulation
  - DC transfer curves
  - transient simulations
  - basic waveform data

- **KLayout**
  - GDS viewing
  - DRC scripts
  - layout inspection
  - possible LVS workflows

- **Magic**
  - layout checking
  - extraction
  - interoperability

- **Netgen**
  - LVS comparison

- **OpenROAD/OpenLane**
  - later-stage RTL-to-GDS workflows
  - standard-cell project flow
  - reporting and visualization

The Rust app should act as:

```text
front end
project manager
teaching layer
report generator
flow coordinator
```

not as a complete replacement for every backend.

---

## Example Student Usage

A student opens the application:

```text
Oxide VLSI
New Project
  [CMOS Inverter Lab]
  [2-Input NAND Lab]
  [2-Input NOR Lab]
  [Transmission Gate Lab]
  [Blank Layout Cell]
  [Open Existing Project]
```

They choose:

```text
New Project → CMOS Inverter Lab
```

The app creates:

```text
inverter_lab/
  inverter.oxvlsi
  schematic/
    inv.sch
  layout/
    inv.layout
  sim/
    dc_transfer.spice
  exports/
  report.md
```

The assignment panel says:

```text
Lab 01: CMOS Inverter Layout

Required:
  - One pMOS transistor
  - One nMOS transistor
  - Input pin A
  - Output pin Y
  - VDD rail
  - GND rail
  - DRC-clean layout
  - Extracted netlist matches inverter schematic
```

The main layout view might look like:

```text
┌─────────────────────────────────────────────────────────────────────┐
│ Oxide VLSI                      Select  Draw  DRC  Extract  Export │
├──────────────┬────────────────────────────────────┬────────────────┤
│ Project      │ Layout Canvas                      │ Layers         │
│              │                                    │ [x] n-well     │
│ Schematic    │      VDD metal rail                │ [x] active     │
│  inv.sch     │                                    │ [x] poly       │
│ Layout       │      pMOS layout                   │ [x] contact    │
│  inv.layout  │                                    │ [x] metal1     │
│ Simulation   │      output route                  │ [x] via1       │
│  dc_transfer │                                    │ [x] metal2     │
│              │      nMOS layout                   │                │
├──────────────┴────────────────────────────────────┴────────────────┤
│ DRC | Extraction | LVS | Simulation | Report                         │
│                                                                     │
│ DRC Errors: 2                                                       │
│  1. pMOS active must be inside n-well                               │
│  2. metal1 spacing violation                                        │
└─────────────────────────────────────────────────────────────────────┘
```

The student clicks:

```text
[Run DRC]
```

The tool reports:

```text
DRC Errors: 2

1. pMOS active must be inside n-well
   Location: x=20λ, y=40λ
   Fix: Extend the n-well region around the pMOS active area.

2. Metal1 spacing violation
   Location: x=35λ, y=18λ
   Required spacing: 3λ
   Found spacing: 2λ
```

The student fixes the layout and clicks:

```text
[Extract Netlist]
```

The tool reports:

```text
Extracted Devices:
  M1: pMOS, gate=A, source=VDD, drain=Y, body=VDD
  M2: nMOS, gate=A, source=GND, drain=Y, body=GND

Extracted Nets:
  A
  Y
  VDD
  GND

Connectivity:
  PASS
```

Then they click:

```text
[Compare to Schematic]
```

The tool reports:

```text
LVS:
  PASS

The extracted layout matches the schematic inverter.
```

Finally, they export:

```text
Export → Lab Report Package
```

Output:

```text
lab01_report/
  source/
    inv.sch
    inv.layout
  exports/
    inv.gds
    inv.svg
    inv.png
    extracted_netlist.spice
  reports/
    drc_report.txt
    lvs_report.txt
  report.md
  report.pdf
```

---

## Instructor Usage

An instructor should be able to define labs using a simple configuration file.

Example:

```toml
[lab]
name = "CMOS Inverter"
type = "layout"
technology = "lambda_cmos"

[required_pins]
inputs = ["A"]
outputs = ["Y"]
power = ["VDD", "GND"]

[required_devices]
pmos = 1
nmos = 1

[checks]
drc = true
extract = true
lvs = true
max_area_lambda2 = 400

[exports]
required = ["layout_png", "gds", "drc_report", "lvs_report"]
```

The instructor can create a starter lab:

```bash
oxide-vlsi new-lab inverter --from inverter.toml
```

Generated structure:

```text
lab01_inverter/
  starter/
    inverter.oxvlsi
    README.md
  solution/
    inverter_solution.oxvlsi
  tests/
    inverter.toml
  grading/
    rubric.md
```

The instructor distributes the `starter/` folder.

---

## Grading Workflow

The CLI should support batch grading:

```bash
oxide-vlsi grade submissions/lab01 --lab inverter.toml
```

Example output:

```text
Lab 01 - CMOS Inverter

Student_001
  DRC: PASS
  Extraction: PASS
  LVS: PASS
  Area: 312 λ²
  Score: 100

Student_002
  DRC: FAIL
  Extraction: SKIPPED
  LVS: SKIPPED
  Errors:
    - active spacing violation
    - pMOS outside n-well
  Score: 62

Student_003
  DRC: PASS
  Extraction: PASS
  LVS: FAIL
  Errors:
    - output Y shorted to VDD
  Score: 75
```

Generated grading artifacts:

```text
grading_report/
  summary.csv
  summary.html
  student_reports/
    Student_001.pdf
    Student_002.pdf
    Student_003.pdf
```

The student-facing report should explain the problem clearly:

```text
LVS Failure: Output short

Expected:
  Y should connect to the drains of the pMOS and nMOS devices.

Observed:
  Y is connected to VDD.

Likely issue:
  A metal1 shape or contact connects the output node to the VDD rail.
```

---

## Built-In Labs

The first educational release should include a set of small labs.

### Lab 1: CMOS Inverter

Concepts:

- pMOS pull-up
- nMOS pull-down
- VDD/GND rails
- input/output pins
- poly gates
- active regions
- basic DRC
- layout extraction

### Lab 2: 2-Input NAND

Concepts:

- series nMOS pull-down network
- parallel pMOS pull-up network
- transistor sizing discussion
- Euler path introduction

### Lab 3: 2-Input NOR

Concepts:

- parallel nMOS pull-down network
- series pMOS pull-up network
- layout area comparison with NAND

### Lab 4: Transmission Gate

Concepts:

- complementary control signals
- pass transistor behavior
- bidirectional signal path
- body/source/drain interpretation

### Lab 5: Tri-State Inverter

Concepts:

- output enable
- high impedance behavior
- output contention
- transistor-level implementation

### Lab 6: XOR/XNOR Cell

Concepts:

- more complex CMOS logic
- layout regularity
- transistor arrangement
- area/performance tradeoff

### Lab 7: D Latch

Concepts:

- feedback
- storage nodes
- transmission gates or clocked inverters
- sequential behavior

### Lab 8: D Flip-Flop

Concepts:

- master/slave latch composition
- clocking
- layout hierarchy
- repeated cells

### Lab 9: Standard Cell Layout Rules

Concepts:

- fixed cell height
- power rails
- input/output pin placement
- abutment
- cell boundary
- routing tracks

### Lab 10: Small Combinational Block

Concepts:

- hierarchical layout
- cell instantiation
- routing between cells
- area estimation
- report generation

---

## MVP Scope

The MVP should be intentionally narrow.

### Version 0.1: Lambda CMOS Layout Editor

Student-facing features:

- Create/open project
- Draw basic layout shapes
- Select/move/resize/delete shapes
- Layer palette
- Lambda grid
- Snapping
- Basic cell save/load
- DRC panel
- SVG/PNG export

Technology features:

- Generic lambda CMOS technology
- Basic layer definitions
- Basic design rules

DRC features:

- width checks
- spacing checks
- enclosure checks
- contact/via size checks
- pMOS-in-n-well check
- basic missing label warnings

Instructor features:

- simple lab template
- starter project generation
- report export

This version does not need full schematic capture, full extraction, LVS, or PDK integration.

The goal of v0.1 is:

> Students can draw a CMOS inverter/NAND/NOR layout, run basic design-rule checks, and export a figure/report artifact.

---

## Roadmap

### v0.1: Layout Foundation

- Rust/egui desktop app
- project save/load
- cell/layout database
- lambda-CMOS technology definition
- layer palette
- rectangle/path drawing
- contacts/vias
- text labels
- grid snap
- ruler
- DRC: width, spacing, enclosure
- SVG/PNG export

### v0.2: Extraction and GDS

- GDSII export
- GDSII import, limited
- transistor recognition
- basic net extraction
- net highlighting
- extracted netlist view
- built-in inverter, NAND, NOR labs

### v0.3: Schematic and SPICE

- schematic capture
- SPICE netlist export
- extracted SPICE netlist generation
- Ngspice runner
- DC transfer simulation workflow
- transient simulation workflow
- waveform import/display
- simulation report export

### v0.4: LVS and Grading

- schematic-vs-layout comparison
- LVS-style mismatch reporting
- lab definition files
- instructor grading mode
- CLI batch grader
- student report export
- CSV/HTML/PDF grading reports

### v0.5: Open-Source Flow Integration

- Sky130 educational layer mapping
- KLayout integration
- Magic integration
- Netgen integration
- OpenLane/OpenROAD handoff
- standard-cell library exercises
- flow-stage report parser

### v1.0: Teaching-Ready Release

- stable project format
- core lab set
- reliable DRC/extraction for educational process
- Windows/macOS/Linux builds
- documentation
- instructor guide
- student guide
- example course module
- public demo projects

---

## Internal Architecture

A clean Rust workspace could be organized as:

```text
oxide-vlsi/
  crates/
    oxide-db/          # design database: cells, layers, shapes, nets
    oxide-gds/         # GDSII import/export
    oxide-tech/        # technology/lambda/PDK rule definitions
    oxide-drc/         # design rule checking
    oxide-extract/     # connectivity/device extraction
    oxide-lvs/         # schematic vs layout comparison
    oxide-sch/         # schematic capture model
    oxide-sim/         # SPICE deck generation/ngspice runner
    oxide-report/      # markdown/html/pdf report generation
    oxide-gui/         # egui/wgpu GUI
    oxide-cli/         # command-line grading/export
```

---

## Core Data Model

The internal model should represent libraries, cells, geometry, and hierarchy.

```text
Library
  Cell
    LayoutView
      Shapes
      Instances
      Labels
      Ports
    SchematicView
      Symbols
      Wires
      Ports
    ExtractedView
      Devices
      Nets
      Reports
```

A simplified conceptual model:

```text
Technology
  Layers
  Purposes
  Display styles
  Design rules
  Device recognition rules
  Extraction rules

Cell
  Name
  Layout shapes
  Schematic elements
  Ports
  Child instances
  Metadata

Shape
  Layer
  Geometry
  Net label, optional
  Purpose, optional

Net
  Name
  Connected shapes
  Connected pins
  Connected device terminals

Device
  Type: nMOS, pMOS, resistor, capacitor
  Terminals
  Geometry
  Parameters
```

---

## Technology File Concept

The technology file should describe layers and rules without hard-coding everything into the program.

Example direction:

```toml
[technology]
name = "lambda_cmos"
unit = "lambda"

[[layers]]
name = "nwell"
color = "#b8d7ff"
purpose = "well"

[[layers]]
name = "active"
color = "#7bd88f"
purpose = "diffusion"

[[layers]]
name = "poly"
color = "#ff7070"
purpose = "gate"

[[layers]]
name = "metal1"
color = "#6fa8dc"
purpose = "routing"

[[rules.width]]
layer = "poly"
min = 2

[[rules.spacing]]
layer = "metal1"
min = 3

[[rules.enclosure]]
outer = "metal1"
inner = "contact"
min = 1
```

The first format does not need to be perfect. It just needs to make the rule system data-driven early.

---

## Suggested GUI Layout

A likely main GUI structure:

```text
┌─────────────────────────────────────────────────────────────────────┐
│ Oxide VLSI             File  Edit  View  Draw  Verify  Sim  Export │
├──────────────┬────────────────────────────────────┬────────────────┤
│ Project      │ Canvas                             │ Inspector      │
│              │                                    │                │
│ Cells        │ Layout/Schematic View              │ Selected Shape │
│  inv         │                                    │ Layer          │
│  nand2       │                                    │ Coordinates    │
│  nor2        │                                    │ Net            │
│              │                                    │ DRC status     │
│ Views        │                                    │                │
│  Schematic   │                                    │ Layers         │
│  Layout      │                                    │ [x] nwell      │
│  Extracted   │                                    │ [x] active     │
│              │                                    │ [x] poly       │
│              │                                    │ [x] metal1     │
├──────────────┴────────────────────────────────────┴────────────────┤
│ Problems | DRC | Extraction | LVS | Simulation | Report             │
│                                                                     │
│ DRC Errors: 0                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

Important panels:

- Project tree
- Layer palette
- Properties inspector
- Problems/DRC panel
- Extraction/LVS report panel
- Simulation panel
- Export/report panel

---

## CLI Concept

The GUI should be paired with a CLI.

Example commands:

```bash
oxide-vlsi new inverter --template cmos-inverter
oxide-vlsi check inverter.oxvlsi
oxide-vlsi drc inverter.oxvlsi
oxide-vlsi extract inverter.oxvlsi
oxide-vlsi lvs inverter.oxvlsi
oxide-vlsi export inverter.oxvlsi --format gds,svg,png
oxide-vlsi report inverter.oxvlsi
oxide-vlsi grade submissions/lab01 --lab inverter.toml
```

A student command:

```bash
oxide-vlsi check
```

Example output:

```text
Project: lab01_inverter
Cell: inv

DRC:
  PASS

Extraction:
  PASS

LVS:
  PASS

Exports:
  exports/inv.svg
  exports/inv.gds
```

An instructor command:

```bash
oxide-vlsi grade ./submissions --export csv,html,pdf
```

---

## Example Error Messages

The tool should be judged heavily by error quality.

### DRC Error

```text
Metal1 spacing violation

Location:
  x = 35λ, y = 18λ

Required:
  metal1 spacing >= 3λ

Found:
  metal1 spacing = 2λ

Why this matters:
  Metal wires that are too close together may short or fail during fabrication.

Suggested fix:
  Move one of the metal1 shapes farther away or route through another track.
```

### Extraction Warning

```text
Floating output node

Net:
  Y

Issue:
  The output label Y is present, but it is not connected to any transistor drain.

Likely fix:
  Add a metal/contact connection from the joined pMOS/nMOS drains to the Y label.
```

### LVS Error

```text
LVS mismatch: missing pull-up path

Expected:
  Y should connect to VDD through a pMOS controlled by A.

Observed:
  No pMOS device connects VDD to Y.

Likely issue:
  The pMOS active or contact is disconnected from the output node.
```

### Well Error

```text
pMOS outside n-well

Location:
  x = 20λ, y = 40λ

Issue:
  A pMOS device was detected, but its active region is not enclosed by n-well.

Suggested fix:
  Draw an n-well region around the pMOS active area with the required enclosure.
```

---

## File Formats

### Internal Project File

The internal project format should be stable, text-friendly if possible, and versioned.

Potential options:

- custom `.oxvlsi` project file
- directory-based project with TOML/JSON/RON files
- zipped project folder
- SQLite database, later if needed

Recommended early approach:

```text
project/
  project.toml
  tech.toml
  cells/
    inv/
      layout.ron
      schematic.ron
      metadata.toml
  exports/
  reports/
```

This is easier to debug and better for version control than a single opaque binary file.

### External Formats

Important exports/imports:

- GDSII
- SVG
- PNG
- SPICE
- Markdown report
- HTML report
- CSV grading summary
- PDF report, later

---

## Relationship to Existing Tools

Oxide VLSI should integrate with the ecosystem rather than compete with every tool.

### Electric VLSI

Electric is the inspiration: a free educational VLSI environment. Oxide VLSI would modernize the experience, narrow the initial scope, and focus on clean classroom workflows.

### KLayout

KLayout is excellent for layout viewing, GDS/OASIS work, scripting, DRC, and inspection. Oxide VLSI can export to KLayout and later optionally invoke KLayout for advanced checking.

### Magic

Magic remains important in open-source VLSI flows. Oxide VLSI can export/import where practical and potentially call Magic for extraction or checking in later versions.

### Ngspice

Ngspice should be used for simulation instead of implementing a SPICE engine.

### OpenROAD/OpenLane

OpenROAD/OpenLane should be considered later-stage integration targets for digital implementation flows. They are not the v0.1 target.

### Sky130

Sky130 can be a later educational PDK target. Do not start with full Sky130 support because real PDK rules and layer mappings add substantial complexity.

Start with lambda CMOS. Add Sky130 once the tool model is stable.

---

## What Not to Build First

Avoid these in early versions:

- Full commercial PDK support
- Full custom analog layout flow
- Signoff DRC
- Signoff LVS
- Parasitic extraction
- Timing analysis
- RTL synthesis
- Placement and routing
- Standard-cell auto-generation
- Full OpenLane GUI
- Complete Cadence-like hierarchy and library system
- Autorouter
- Advanced density/fill/antenna checks
- Multi-corner/multi-mode analysis

These may be useful eventually, but they are not needed to make the first version valuable.

---

## Technical Risks

### 1. DRC Complexity

Basic DRC is manageable. Real production DRC is extremely complex.

Mitigation:

- Start with lambda rules.
- Clearly label the technology as educational.
- Keep the DRC engine data-driven.
- Add real-PDK rules only after the internal model is stable.

### 2. Extraction Complexity

Device extraction can become complicated quickly.

Mitigation:

- Start with simple MOS recognition.
- Limit the educational process.
- Require labels for early versions.
- Support only common beginner structures first.

### 3. LVS Complexity

Full LVS is hard.

Mitigation:

- Start with structural comparisons for known labs.
- Compare expected device counts and basic net connectivity.
- Build toward more general graph comparison later.

### 4. UI Complexity

Layout editing requires a good interaction model.

Mitigation:

- Start with rectangles, snapping, selection, and undo/redo.
- Avoid fancy editing features early.
- Make the DRC/report loop strong.

### 5. PDK Scope Creep

Real PDK support can consume the project.

Mitigation:

- Keep v0.1-v0.4 process-independent.
- Treat PDK support as v0.5+.
- Avoid promising production readiness.

### 6. User Expectations

Some users will compare it to Cadence or Magic.

Mitigation:

- State the educational scope clearly.
- Make the tool excellent at beginner workflows.
- Do not overclaim.

---

## Why This May Be More Distinctive Than a VHDL Tool

VHDL Lab Studio is useful, but students already have several ways to simulate HDL using GHDL, NVC, vendor tools, VUnit, OSVVM, and IDE plugins.

A modern teaching-first VLSI layout tool feels more distinctive because the pain is sharper:

- Commercial tools are expensive.
- Open-source tools are fragmented.
- Electric is useful but dated.
- Students struggle to understand layout errors.
- Instructors need report/grading workflows.
- There is room for a clean educational experience.

This makes Oxide VLSI a strong candidate for a serious open-source project.

---

## Initial Development Strategy

The first technical milestone should not be “VLSI tool.”

It should be:

> Draw a CMOS inverter layout, run lambda-rule DRC, recognize the pMOS/nMOS devices, and export a clean report.

That decomposes into concrete tasks:

1. Create project format.
2. Create layer model.
3. Create layout canvas.
4. Draw rectangles.
5. Add grid snapping.
6. Add labels.
7. Add DRC width/spacing checks.
8. Add contact/enclosure checks.
9. Export SVG/PNG.
10. Add simple transistor recognition.
11. Add net extraction.
12. Add inverter lab template.
13. Add report export.

Once that works, expand to NAND and NOR.

---

## Suggested Repository Structure

```text
oxide-vlsi/
  README.md
  LICENSE
  Cargo.toml
  crates/
    oxide-db/
    oxide-tech/
    oxide-drc/
    oxide-extract/
    oxide-gds/
    oxide-sch/
    oxide-sim/
    oxide-report/
    oxide-gui/
    oxide-cli/
  examples/
    labs/
      inverter/
      nand2/
      nor2/
  docs/
    design/
    user-guide/
    instructor-guide/
    file-format/
  tests/
    drc/
    extraction/
    lvs/
```

---

## Possible README Pitch

```markdown
# Oxide VLSI

Oxide VLSI is a modern, open-source CMOS layout and verification environment for teaching digital VLSI design.

It is not a commercial signoff tool. Instead, it focuses on the classroom workflows that matter most: drawing small CMOS cells, understanding layout layers, running design-rule checks, extracting connectivity, comparing layout to schematic intent, and exporting reports.

The project is inspired by the educational role of Electric VLSI, but aims to provide a cleaner, modern, cross-platform experience built in Rust.
```

---

## Possible Tagline

> CMOS layout education without the commercial EDA wall.

Other options:

- Open-source VLSI layout for the classroom.
- A modern teaching tool for CMOS layout.
- Learn VLSI by drawing, checking, extracting, and understanding.
- An Electric-inspired VLSI lab tool built in Rust.
- From transistors to GDS, without the licensing headache.

---

## Recommended First Deliverable

The first meaningful release should be:

> **Oxide VLSI v0.1: CMOS Cell Studio**

Minimum feature set:

- Create/open project
- Draw lambda-grid CMOS layouts
- Use n-well, active, poly, contact, metal1
- Place labels
- Run simple DRC
- Export SVG/PNG
- Include inverter, NAND, and NOR lab templates
- Generate a basic report

This would already be useful for an introductory VLSI course.

---

## Long-Term Vision

If successful, Oxide VLSI could become part of a larger open-source computer engineering education toolkit:

```text
OARS
  RISC-V assembly, simulation, debugging

VHDL Lab Studio
  VHDL simulation, testing, and grading

Oxide VLSI
  CMOS layout, DRC, extraction, and VLSI labs

Future Tools
  pipeline visualization
  digital logic grading
  embedded lab notebooks
  hardware reverse engineering reports
```

Together, these could form a coherent teaching ecosystem for:

- Digital logic
- Computer architecture
- Embedded systems
- HDL design
- VLSI
- Reverse engineering

That is a strong direction for a serious open-source educational software suite.
