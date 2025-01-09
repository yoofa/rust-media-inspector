use crate::analyzer::Property;

#[derive(Debug)]
pub struct ChunkOffsetBox {
    version: u8,
    flags: u32,
    offsets: Vec<u64>, // u32 for stco, u64 for co64
}

impl ChunkOffsetBox {
    pub fn new(version: u8, flags: u32, offsets: Vec<u64>) -> Self {
        Self {
            version,
            flags,
            offsets,
        }
    }

    pub fn description(&self) -> &str {
        "Chunk Offset Box"
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
            "entry_count",
            self.offsets.len().to_string(),
            None::<String>,
        ));

        // Only show first few offsets to avoid overwhelming output
        for (i, offset) in self.offsets.iter().take(5).enumerate() {
            properties.push(Property::new(
                &format!("offset[{}]", i),
                format!("0x{:x}", offset),
                None::<String>,
            ));
        }
        if self.offsets.len() > 5 {
            properties.push(Property::new(
                "...",
                format!("{} more", self.offsets.len() - 5),
                None::<String>,
            ));
        }
    }
}
