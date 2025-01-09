use crate::analyzer::Property;
#[derive(Debug)]
pub struct SampleToChunkBox {
    version: u8,
    flags: u32,
    entries: Vec<SampleToChunkEntry>,
}

#[derive(Debug)]
pub struct SampleToChunkEntry {
    first_chunk: u32,
    samples_per_chunk: u32,
    sample_description_index: u32,
}

impl SampleToChunkBox {
    pub fn new(version: u8, flags: u32, entries: Vec<SampleToChunkEntry>) -> Self {
        Self {
            version,
            flags,
            entries,
        }
    }

    pub fn description(&self) -> &str {
        "Sample To Chunk Box"
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
            self.entries.len().to_string(),
            None::<String>,
        ));

        for (i, entry) in self.entries.iter().enumerate() {
            properties.push(Property::new(
                &format!("entry[{}].first_chunk", i),
                entry.first_chunk.to_string(),
                None::<String>,
            ));
            properties.push(Property::new(
                &format!("entry[{}].samples_per_chunk", i),
                entry.samples_per_chunk.to_string(),
                None::<String>,
            ));
            properties.push(Property::new(
                &format!("entry[{}].sample_description_index", i),
                entry.sample_description_index.to_string(),
                None::<String>,
            ));
        }
    }
}

impl SampleToChunkEntry {
    pub fn new(first_chunk: u32, samples_per_chunk: u32, sample_description_index: u32) -> Self {
        Self {
            first_chunk,
            samples_per_chunk,
            sample_description_index,
        }
    }
}
