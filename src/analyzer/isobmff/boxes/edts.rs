use crate::analyzer::Property;

#[derive(Debug)]
pub struct EditBox {}

impl EditBox {
    pub fn new() -> Self {
        Self {}
    }

    pub fn description(&self) -> &str {
        "Edit Box"
    }

    pub fn fill_properties(&self, _properties: &mut Vec<Property>) {}
}
