use oxide_db::cell::LayoutView;
use std::path::Path;

pub fn import_gds(_path: &Path) -> Result<LayoutView, Box<dyn std::error::Error>> {
    // v0.2: implement GDSII import via gds21
    Ok(LayoutView::default())
}
