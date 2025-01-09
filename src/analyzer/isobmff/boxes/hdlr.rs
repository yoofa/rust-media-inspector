use crate::analyzer::Property;

#[derive(Debug)]
pub struct HandlerBox {
    version: u8,
    flags: u32,
    handler_type: String, // 4 characters
    name: String,         // Handler name
}

impl HandlerBox {
    pub fn new(version: u8, flags: u32, handler_type: String, name: String) -> Self {
        Self {
            version,
            flags,
            handler_type,
            name,
        }
    }

    pub fn description(&self) -> &str {
        "Handler Reference Box"
    }

    pub fn fill_properties(&self, properties: &mut Vec<Property>) {
        properties.push(Property::new(
            "version",
            self.version.to_string(),
            None::<String>,
        ));
        properties.push(Property::new(
            "flags",
            format!("0x{:06x}", self.flags),
            None::<String>,
        ));
        properties.push(Property::new(
            "handler_type",
            self.handler_type.clone(),
            None::<String>,
        ));
        properties.push(Property::new("name", self.name.clone(), None::<String>));
    }
}
