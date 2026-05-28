/// A simple SPICE netlist representation.
pub struct SpiceNetlist {
    pub title: String,
    pub lines: Vec<String>,
}

impl SpiceNetlist {
    pub fn to_string(&self) -> String {
        let mut out = format!("* {}\n", self.title);
        for line in &self.lines {
            out.push_str(line);
            out.push('\n');
        }
        out
    }
}
