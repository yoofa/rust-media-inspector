use crate::analyzer::isobmff::types::Fixed16_16;
use crate::analyzer::Property;

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
            "balance",
            format!("{}", self.balance.as_f32()),
            None::<String>,
        ));
    }
}
