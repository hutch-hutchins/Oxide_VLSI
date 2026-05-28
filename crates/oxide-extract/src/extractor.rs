use crate::device::Device;
use crate::net::Net;
use oxide_db::cell::LayoutView;

pub struct Extractor;

pub struct ExtractionResult {
    pub nets: Vec<Net>,
    pub devices: Vec<Device>,
}

impl Extractor {
    pub fn run(_layout: &LayoutView) -> ExtractionResult {
        // v0.2: implement connectivity and transistor recognition
        ExtractionResult { nets: Vec::new(), devices: Vec::new() }
    }
}
