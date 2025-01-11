use crate::analyzer::Property;

#[derive(Debug)]
pub struct DataInformationBox {}

impl DataInformationBox {
    pub fn new() -> Self {
        Self {}
    }

    pub fn description(&self) -> &str {
        "Data Information Box"
    }

    pub fn fill_properties(&self, _properties: &mut Vec<Property>) {}
}
