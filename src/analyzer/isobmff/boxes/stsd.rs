use crate::analyzer::Property;
#[derive(Debug)]
pub struct SampleDescriptionBox {
    version: u8,
    flags: u32,
    entry_count: u32,
    entries: Vec<SampleEntry>,
}

#[derive(Debug)]
pub struct SampleEntry {
    entry_type: String, // e.g., 'avc1', 'mp4a'
    data_reference_index: u16,
    data: Vec<u8>, // Format-specific data
}

impl SampleDescriptionBox {
    pub fn new(version: u8, flags: u32, entry_count: u32, entries: Vec<SampleEntry>) -> Self {
        Self {
            version,
            flags,
            entry_count,
            entries,
        }
    }

    pub fn description(&self) -> &str {
        "Sample Description Box"
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
            self.entry_count.to_string(),
            None::<String>,
        ));

        for (i, entry) in self.entries.iter().enumerate() {
            properties.push(Property::new(
                &format!("entry[{}].type", i),
                entry.entry_type.clone(),
                None::<String>,
            ));
            properties.push(Property::new(
                &format!("entry[{}].data_reference_index", i),
                entry.data_reference_index.to_string(),
                None::<String>,
            ));
            properties.push(Property::new(
                &format!("entry[{}].data", i),
                format!("{} bytes", entry.data.len()),
                None::<String>,
            ));
        }
    }
}

impl SampleEntry {
    pub fn new(entry_type: String, data_reference_index: u16, data: Vec<u8>) -> Self {
        Self {
            entry_type,
            data_reference_index,
            data,
        }
    }
}
