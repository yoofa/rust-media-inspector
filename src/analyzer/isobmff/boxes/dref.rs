#[derive(Debug)]
pub enum DataEntryBox {
    Url {
        version: u8,
        flags: u32,
        location: String,
    },
    Urn {
        version: u8,
        flags: u32,
        name: String,
        location: String,
    },
}

#[derive(Debug)]
pub struct DataReferenceBox {
    version: u8,
    flags: u32,
    entries: Vec<DataEntryBox>,
}

impl DataReferenceBox {
    pub fn new(version: u8, flags: u32, entries: Vec<DataEntryBox>) -> Self {
        Self {
            version,
            flags,
            entries,
        }
    }

    pub fn description(&self) -> &str {
        "Data Reference Box"
    }

    pub fn fill_properties(&self, properties: &mut Vec<(String, String)>) {
        properties.push(("version".to_string(), self.version.to_string()));
        properties.push(("flags".to_string(), format!("0x{:06x}", self.flags)));
        properties.push(("entry_count".to_string(), self.entries.len().to_string()));

        // 只显示前几个条目
        for (i, entry) in self.entries.iter().take(5).enumerate() {
            match entry {
                DataEntryBox::Url {
                    flags, location, ..
                } => {
                    properties.push((
                        format!("url[{}]", i),
                        format!("flags=0x{:06x}, location={}", flags, location),
                    ));
                }
                DataEntryBox::Urn {
                    flags,
                    name,
                    location,
                    ..
                } => {
                    properties.push((
                        format!("urn[{}]", i),
                        format!(
                            "flags=0x{:06x}, name={}, location={}",
                            flags, name, location
                        ),
                    ));
                }
            }
        }

        if self.entries.len() > 5 {
            properties.push((
                "...".to_string(),
                format!("{} more entries", self.entries.len() - 5),
            ));
        }
    }
}
