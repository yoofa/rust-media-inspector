use super::Box;
use crate::analyzer::Property;

#[derive(Debug)]
#[allow(dead_code)]
pub struct SampleTableBox {
    children: Vec<Box>,
}

impl SampleTableBox {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_child(&mut self, child: Box) {
        self.children.push(child);
    }

    pub fn description(&self) -> &str {
        "Sample Table Box"
    }

    pub fn fill_properties(&self, _properties: &mut Vec<Property>) {
        // Sample table box itself doesn't have properties
    }

    #[allow(dead_code)]
    pub fn children(&self) -> &[Box] {
        &self.children
    }
}
