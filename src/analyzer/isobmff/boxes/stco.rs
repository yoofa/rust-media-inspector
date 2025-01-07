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

    pub fn fill_properties(&self, properties: &mut Vec<(String, String)>) {
        properties.push(("version".to_string(), self.version.to_string()));
        properties.push(("flags".to_string(), format!("0x{:06x}", self.flags)));
        properties.push(("entry_count".to_string(), self.offsets.len().to_string()));

        // Only show first few offsets to avoid overwhelming output
        for (i, offset) in self.offsets.iter().take(5).enumerate() {
            properties.push((format!("offset[{}]", i), format!("0x{:x}", offset)));
        }
        if self.offsets.len() > 5 {
            properties.push((
                "...".to_string(),
                format!("{} more", self.offsets.len() - 5),
            ));
        }
    }
}
