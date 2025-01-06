use crate::analyzer::isobmff::types::Fixed16_16;

#[derive(Debug)]
pub struct SoundMediaHeaderBox {
    version: u8,
    flags: u32,
    balance: Fixed16_16, // Audio balance, fixed point 8.8
}

impl SoundMediaHeaderBox {
    pub fn new(version: u8, flags: u32, balance: Fixed16_16) -> Self {
        Self {
            version,
            flags,
            balance,
        }
    }

    pub fn description(&self) -> &str {
        "Sound Media Header Box"
    }

    pub fn fill_properties(&self, properties: &mut Vec<(String, String)>) {
        properties.push(("version".to_string(), self.version.to_string()));
        properties.push(("flags".to_string(), format!("0x{:06x}", self.flags)));
        properties.push(("balance".to_string(), format!("{}", self.balance.as_f32())));
    }
}
