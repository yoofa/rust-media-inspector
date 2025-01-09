use crate::analyzer::Property;

#[derive(Debug)]
pub struct PropChunk {
    max_bit_rate: u32,
    avg_bit_rate: u32,
    max_packet_size: u32,
    avg_packet_size: u32,
    num_packets: u32,
    duration: u32,
    preroll: u32,
    index_offset: u32,
    data_offset: u32,
    num_streams: u16,
    flags: u16,
}

impl PropChunk {
    pub fn new(
        max_bit_rate: u32,
        avg_bit_rate: u32,
        max_packet_size: u32,
        avg_packet_size: u32,
        num_packets: u32,
        duration: u32,
        preroll: u32,
        index_offset: u32,
        data_offset: u32,
        num_streams: u16,
        flags: u16,
    ) -> Self {
        Self {
            max_bit_rate,
            avg_bit_rate,
            max_packet_size,
            avg_packet_size,
            num_packets,
            duration,
            preroll,
            index_offset,
            data_offset,
            num_streams,
            flags,
        }
    }

    pub fn description(&self) -> &str {
        "File Properties"
    }

    pub fn fill_properties(&self, properties: &mut Vec<Property>) {
        properties.push(Property::new(
            "max_bit_rate",
            self.max_bit_rate,
            Some(format!("{} bps", self.max_bit_rate)),
        ));
        properties.push(Property::new(
            "avg_bit_rate",
            self.avg_bit_rate,
            Some(format!("{} bps", self.avg_bit_rate)),
        ));
        properties.push(Property::new(
            "max_packet_size",
            self.max_packet_size,
            Some(format!("{} bytes", self.max_packet_size)),
        ));
        properties.push(Property::new(
            "avg_packet_size",
            self.avg_packet_size,
            Some(format!("{} bytes", self.avg_packet_size)),
        ));
        properties.push(Property::new(
            "num_packets",
            self.num_packets,
            Some(format!("{} packets", self.num_packets)),
        ));
        properties.push(Property::new(
            "duration",
            self.duration,
            Some(format!("{} ms", self.duration)),
        ));
        properties.push(Property::new(
            "preroll",
            self.preroll,
            Some(format!("{} ms", self.preroll)),
        ));
        properties.push(Property::new(
            "index_offset",
            self.index_offset,
            Some(format!("0x{:08x}", self.index_offset)),
        ));
        properties.push(Property::new(
            "data_offset",
            self.data_offset,
            Some(format!("0x{:08x}", self.data_offset)),
        ));
        properties.push(Property::new(
            "num_streams",
            self.num_streams,
            Some(format!("{} streams", self.num_streams)),
        ));
        properties.push(Property::new(
            "flags",
            format!("0x{:04x}", self.flags),
            None::<String>,
        ));
    }
}
