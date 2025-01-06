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

    pub fn fill_properties(&self, properties: &mut Vec<(String, String)>) {
        properties.push(("version".to_string(), self.version.to_string()));
        properties.push(("flags".to_string(), format!("0x{:06x}", self.flags)));
        properties.push(("handler_type".to_string(), self.handler_type.clone()));
        properties.push(("name".to_string(), self.name.clone()));
    }
}
