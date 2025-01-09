use crate::analyzer::Property;

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

    pub fn fill_properties(&self, properties: &mut Vec<Property>) {
        properties.push(Property::new("version", self.version, None::<String>));
        properties.push(Property::new(
            "flags",
            format!("0x{:06x}", self.flags),
            None::<String>,
        ));
        properties.push(Property::new(
            "entry_count",
            self.entries.len(),
            Some(format!("{} entries", self.entries.len())),
        ));

        // 只显示前几个条目
        for (i, entry) in self.entries.iter().take(5).enumerate() {
            match entry {
                DataEntryBox::Url {
                    flags, location, ..
                } => {
                    properties.push(Property::new(
                        &format!("url[{}]", i),
                        format!("flags=0x{:06x}", flags),
                        Some(format!("URL: {}", location)),
                    ));
                }
                DataEntryBox::Urn {
                    flags,
                    name,
                    location,
                    ..
                } => {
                    properties.push(Property::new(
                        &format!("urn[{}]", i),
                        format!("flags=0x{:06x}", flags),
                        Some(format!("URN: {} -> {}", name, location)),
                    ));
                }
            }
        }

        if self.entries.len() > 5 {
            properties.push(Property::new(
                "...",
                format!("{} more entries", self.entries.len() - 5),
                None::<String>,
            ));
        }
    }
}
