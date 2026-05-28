#[derive(Debug, Clone, PartialEq)]
pub enum ToolMode {
    Select,
    DrawRect { layer: String },
}
