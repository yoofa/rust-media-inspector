use crate::analyzer::Property;

#[derive(Debug)]
pub struct IndexEntry {
    timestamp: u32,
    offset: u32,
    packet_count: u32,
}

impl IndexEntry {
    pub fn new(timestamp: u32, offset: u32, packet_count: u32) -> Self {
        Self {
            timestamp,
            offset,
            packet_count,
        }
    }
}

#[derive(Debug)]
pub struct IndxChunk {
    num_entries: u32,
    stream_number: u16,
    next_index_header: u32,
    entries: Vec<IndexEntry>,
}

impl IndxChunk {
    pub fn new(
        num_entries: u32,
        stream_number: u16,
        next_index_header: u32,
        entries: Vec<IndexEntry>,
    ) -> Self {
        Self {
            num_entries,
            stream_number,
            next_index_header,
            entries,
        }
    }

    pub fn description(&self) -> &str {
        "Index"
    }

    pub fn fill_properties(&self, properties: &mut Vec<Property>) {
        properties.push(Property::new(
            "num_entries",
            self.num_entries,
            Some(format!("{} entries", self.num_entries)),
        ));
        properties.push(Property::new(
            "stream_number",
            self.stream_number,
            Some(format!("Stream #{}", self.stream_number)),
        ));
        properties.push(Property::new(
            "next_index_header",
            format!("0x{:08x}", self.next_index_header),
            None::<String>,
        ));

        // 只显示前几个条目
        for (i, entry) in self.entries.iter().take(5).enumerate() {
            properties.push(Property::new(
                &format!("entry[{}].timestamp", i),
                entry.timestamp,
                Some(format!("{} ms", entry.timestamp)),
            ));
            properties.push(Property::new(
                &format!("entry[{}].offset", i),
                format!("0x{:08x}", entry.offset),
                None::<String>,
            ));
            properties.push(Property::new(
                &format!("entry[{}].packet_count", i),
                entry.packet_count,
                Some(format!("{} packets", entry.packet_count)),
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
