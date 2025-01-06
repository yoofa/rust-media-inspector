#[derive(Debug)]
pub struct SampleSizeBox {
    version: u8,
    flags: u32,
    sample_size: u32,
    sample_count: u32,
    entry_sizes: Vec<u32>, // Only used if sample_size == 0
}

impl SampleSizeBox {
    pub fn new(
        version: u8,
        flags: u32,
        sample_size: u32,
        sample_count: u32,
        entry_sizes: Vec<u32>,
    ) -> Self {
        Self {
            version,
            flags,
            sample_size,
            sample_count,
            entry_sizes,
        }
    }

    pub fn description(&self) -> &str {
        "Sample Size Box"
    }

    pub fn fill_properties(&self, properties: &mut Vec<(String, String)>) {
        properties.push(("version".to_string(), self.version.to_string()));
        properties.push(("flags".to_string(), format!("0x{:06x}", self.flags)));
        properties.push(("sample_size".to_string(), self.sample_size.to_string()));
        properties.push(("sample_count".to_string(), self.sample_count.to_string()));

        if self.sample_size == 0 {
            // Only show individual sizes if they're variable
            properties.push((
                "entry_sizes".to_string(),
                format!("{} entries", self.entry_sizes.len()),
            ));
        }
    }
}
