![Oxide VLSI](assets/Oxide_VLSI_Logo_2172x724.png)

# Oxide VLSI

**CMOS layout education without the commercial EDA wall.**

Oxide VLSI is an open-source desktop VLSI design environment built for undergraduate digital VLSI courses. Students draw CMOS layouts at the lambda level, run design rule checks, extract netlists, and export clean lab reports — all without commercial EDA licenses.

---

## Features (v0.1)

- Lambda CMOS layout editor with pan, zoom, and snap-to-grid
- Layer palette: nwell, active, poly, contact, metal1, via1, metal2
- Design Rule Checker (DRC) with teaching-first error messages
- SVG and PNG export
- Project save/load (directory-based, git-friendly)
- Lab templates: CMOS Inverter, 2-Input NAND, 2-Input NOR
- Command-line grading tool (`Oxide_VLSI_cli`)

## Roadmap

| Version | Milestone |
|---------|-----------|
| v0.1 | Lambda CMOS layout editor, DRC, SVG/PNG export |
| v0.2 | GDSII export/import, connectivity extraction, net highlighting |
| v0.3 | Schematic capture, SPICE simulation, waveform viewer |
| v0.4 | LVS, batch grading infrastructure, per-student PDF reports |
| v0.5 | Sky130 layer mapping, KLayout/Magic/Netgen integration |

---

## Building

### Prerequisites

- [Rust](https://rustup.rs/) 1.75 or later
- A C linker (MSVC on Windows, GCC/Clang on Linux/macOS)

### Build and run

```sh
# Clone the repository
git clone <repo-url>
cd Oxide_VLSI

# Build everything
cargo build --release

# Run the GUI
cargo run -p oxide-gui --release

# Run the CLI
cargo run -p oxide-cli --release -- --help
```

The debug build is also available without `--release` for faster compile times during development.

### Running tests

```sh
cargo test
```

---

## Project structure

```
Oxide_VLSI/
  Cargo.toml          workspace manifest
  assets/             logos and static assets
  crates/
    oxide-tech/       technology model (layers, lambda design rules)
    oxide-db/         design database (cells, shapes, project save/load)
    oxide-drc/        design rule checker
    oxide-extract/    connectivity and transistor extraction
    oxide-gds/        GDSII import/export (v0.2)
    oxide-sch/        schematic capture (v0.3)
    oxide-sim/        SPICE netlist + ngspice runner (v0.3)
    oxide-report/     PDF/HTML grading reports (v0.4)
    oxide-gui/        egui desktop application
    oxide-cli/        command-line tool for batch grading
  examples/
    labs/             starter projects for each lab assignment
```

---

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
