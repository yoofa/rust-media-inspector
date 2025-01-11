use crate::analyzer::Property;

#[derive(Debug)]
pub struct EditListEntry {
    pub segment_duration: u32,
    pub media_time: i32,
    pub media_rate: i16,
    pub media_rate_fraction: i16,
}

impl EditListEntry {
    pub fn new(
        segment_duration: u32,
        media_time: i32,
        media_rate: i16,
        media_rate_fraction: i16,
    ) -> Self {
        Self {
            segment_duration,
            media_time,
            media_rate,
            media_rate_fraction,
        }
    }
}

#[derive(Debug)]
pub struct EditListBox {
    version: u8,
    flags: u32,
    entries: Vec<EditListEntry>,
}

impl EditListBox {
    pub fn new(version: u8, flags: u32, entries: Vec<EditListEntry>) -> Self {
        Self {
            version,
            flags,
            entries,
        }
    }

    pub fn description(&self) -> &str {
        "Edit List Box"
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
            self.entries.len().to_string(),
            None::<String>,
        ));

        // 只显示前几个条目
        for (i, entry) in self.entries.iter().take(5).enumerate() {
            properties.push(Property::new(
                &format!("entry[{}].duration", i),
                entry.segment_duration.to_string(),
                None::<String>,
            ));
            properties.push(Property::new(
                &format!("entry[{}].media_time", i),
                entry.media_time.to_string(),
                None::<String>,
            ));
            properties.push(Property::new(
                &format!("entry[{}].media_rate", i),
                format!("{}.{}", entry.media_rate, entry.media_rate_fraction),
                None::<String>,
            ));
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
