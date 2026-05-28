use oxide_drc::violation::DrcViolation;

pub struct MarkdownReport {
    pub cell_name: String,
    pub drc_violations: Vec<DrcViolation>,
    pub exported_files: Vec<String>,
}

impl MarkdownReport {
    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# Oxide VLSI Report — {}\n\n", self.cell_name));

        out.push_str("## DRC\n\n");
        if self.drc_violations.is_empty() {
            out.push_str("**PASS** — No violations found.\n\n");
        } else {
            out.push_str(&format!("**FAIL** — {} violation(s)\n\n", self.drc_violations.len()));
            for (i, v) in self.drc_violations.iter().enumerate() {
                out.push_str(&format!("### {}. {}\n\n", i + 1, v.rule));
                out.push_str(&format!("- **Required:** {}\n", v.required));
                out.push_str(&format!("- **Found:** {}\n", v.found));
                out.push_str(&format!("- **Location:** ({:.1}λ, {:.1}λ)\n", v.location.x, v.location.y));
                out.push_str(&format!("- **Explanation:** {}\n", v.explanation));
                out.push_str(&format!("- **Fix:** {}\n\n", v.fix_hint));
            }
        }

        if !self.exported_files.is_empty() {
            out.push_str("## Exported Files\n\n");
            for f in &self.exported_files {
                out.push_str(&format!("- `{}`\n", f));
            }
            out.push('\n');
        }

        out
    }
}
