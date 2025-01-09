use crate::analyzer::Property;

#[derive(Debug)]
pub struct MdprChunk {
    stream_number: u16,
    max_bit_rate: u32,
    avg_bit_rate: u32,
    max_packet_size: u32,
    avg_packet_size: u32,
    start_time: u32,
    preroll: u32,
    duration: u32,
    stream_name: String,
    mime_type: String,
    type_specific_len: u32,
    type_specific_data: Vec<u8>,
}

impl MdprChunk {
    pub fn new(
        stream_number: u16,
        max_bit_rate: u32,
        avg_bit_rate: u32,
        max_packet_size: u32,
        avg_packet_size: u32,
        start_time: u32,
        preroll: u32,
        duration: u32,
        stream_name: String,
        mime_type: String,
        type_specific_len: u32,
        type_specific_data: Vec<u8>,
    ) -> Self {
        Self {
            stream_number,
            max_bit_rate,
            avg_bit_rate,
            max_packet_size,
            avg_packet_size,
            start_time,
            preroll,
            duration,
            stream_name,
            mime_type,
            type_specific_len,
            type_specific_data,
        }
    }

    pub fn description(&self) -> &str {
        "Media Properties"
    }

    pub fn fill_properties(&self, properties: &mut Vec<Property>) {
        properties.push(Property::new(
            "stream_number",
            self.stream_number,
            Some(format!("Stream #{}", self.stream_number)),
        ));
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
            "start_time",
            self.start_time,
            Some(format!("{} ms", self.start_time)),
        ));
        properties.push(Property::new(
            "preroll",
            self.preroll,
            Some(format!("{} ms", self.preroll)),
        ));
        properties.push(Property::new(
            "duration",
            self.duration,
            Some(format!("{} ms", self.duration)),
        ));
        if !self.stream_name.is_empty() {
            properties.push(Property::new(
                "stream_name",
                &self.stream_name,
                None::<String>,
            ));
        }
        if !self.mime_type.is_empty() {
            properties.push(Property::new("mime_type", &self.mime_type, None::<String>));
        }
        properties.push(Property::new(
            "type_specific_data",
            format!("{} bytes", self.type_specific_len),
            Some(format!("0x{:02x?}", &self.type_specific_data)),
        ));
    }
}
