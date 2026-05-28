use crate::checks::{check_enclosures, check_pmos_in_nwell, check_spacing, check_widths};
use crate::violation::DrcViolation;
use oxide_db::cell::LayoutView;
use oxide_tech::tech::Technology;

pub struct DrcEngine<'a> {
    pub tech: &'a Technology,
}

impl<'a> DrcEngine<'a> {
    pub fn new(tech: &'a Technology) -> Self {
        Self { tech }
    }

    /// Run all DRC checks on a layout and return all violations.
    pub fn run(&self, layout: &LayoutView) -> Vec<DrcViolation> {
        let mut violations = Vec::new();

        violations.extend(check_widths(layout, &self.tech.rules.width));
        violations.extend(check_spacing(layout, &self.tech.rules.spacing));
        violations.extend(check_enclosures(layout, &self.tech.rules.enclosure));
        violations.extend(check_pmos_in_nwell(layout));

        violations
    }
}
