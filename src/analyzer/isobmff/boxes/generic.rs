use crate::analyzer::isobmff::types::BoxType;
use crate::analyzer::Property;

/// Represents a generic box whose type is known but contents are not specifically parsed
#[derive(Debug, Clone)]
pub struct GenericBox {
    /// The box type (4CC code)
    box_type: BoxType,
}

impl GenericBox {
    /// Creates a new generic box with the specified type
    pub fn new(box_type: BoxType) -> Self {
        Self { box_type }
    }

    /// Returns the box type
    pub fn box_type(&self) -> &BoxType {
        &self.box_type
    }

    pub fn description(&self) -> &str {
        "Unknown Generic Box"
    }

    pub fn fill_properties(&self, properties: &mut Vec<Property>) {
        properties.push(Property::new(
            "box type",
            self.box_type.as_str().to_string(),
            None::<String>,
        ));
    }
}

/// Implementation for displaying generic box information
impl std::fmt::Display for GenericBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Generic Box ({})", self.box_type.as_str())
    }
}
