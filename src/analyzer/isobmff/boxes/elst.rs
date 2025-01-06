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

    pub fn fill_properties(&self, properties: &mut Vec<(String, String)>) {
        properties.push(("version".to_string(), self.version.to_string()));
        properties.push(("flags".to_string(), format!("0x{:06x}", self.flags)));
        properties.push(("entry_count".to_string(), self.entries.len().to_string()));

        // 只显示前几个条目
        for (i, entry) in self.entries.iter().take(5).enumerate() {
            properties.push((
                format!("entry[{}].duration", i),
                entry.segment_duration.to_string(),
            ));
            properties.push((
                format!("entry[{}].media_time", i),
                entry.media_time.to_string(),
            ));
            properties.push((
                format!("entry[{}].media_rate", i),
                format!("{}.{}", entry.media_rate, entry.media_rate_fraction),
            ));
        }

        if self.entries.len() > 5 {
            properties.push((
                "...".to_string(),
                format!("{} more entries", self.entries.len() - 5),
            ));
        }
    }
}
