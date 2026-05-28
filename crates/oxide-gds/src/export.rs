use oxide_db::cell::LayoutView;
use std::path::Path;

pub fn export_gds(_layout: &LayoutView, _path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // v0.2: implement GDSII export via gds21
    Ok(())
}
