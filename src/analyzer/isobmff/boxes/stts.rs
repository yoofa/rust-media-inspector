#[derive(Debug)]
pub struct TimeToSampleBox {
    version: u8,
    flags: u32,
    entries: Vec<TimeToSampleEntry>,
}

#[derive(Debug)]
pub struct TimeToSampleEntry {
    sample_count: u32,    // Number of consecutive samples with this duration
    sample_delta: u32,    // Duration of each sample
}

impl TimeToSampleBox {
    pub fn new(version: u8, flags: u32, entries: Vec<TimeToSampleEntry>) -> Self {
        Self {
            version,
            flags,
            entries,
        }
    }

    pub fn description(&self) -> &str {
        "Time To Sample Box"
    }

    pub fn fill_properties(&self, properties: &mut Vec<(String, String)>) {
        properties.push(("version".to_string(), self.version.to_string()));
        properties.push(("flags".to_string(), format!("0x{:06x}", self.flags)));
        properties.push(("entry_count".to_string(), self.entries.len().to_string()));

        for (i, entry) in self.entries.iter().enumerate() {
            properties.push((
                format!("entry[{}].sample_count", i),
                entry.sample_count.to_string(),
            ));
            properties.push((
                format!("entry[{}].sample_delta", i),
                entry.sample_delta.to_string(),
            ));
        }
    }
}

impl TimeToSampleEntry {
    pub fn new(sample_count: u32, sample_delta: u32) -> Self {
        Self {
            sample_count,
            sample_delta,
        }
    }
} 