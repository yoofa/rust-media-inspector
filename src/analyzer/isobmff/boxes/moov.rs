#[derive(Debug)]
pub struct MovieBox;

impl MovieBox {
    pub fn new() -> Self {
        Self
    }

    pub fn description(&self) -> &str {
        "Movie Box"
    }

    pub fn fill_properties(&self, _properties: &mut Vec<(String, String)>) {
        // Movie box itself doesn't have properties, its children contain the actual data
    }
}
