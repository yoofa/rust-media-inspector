use crate::analyzer::Property;

#[derive(Debug)]
pub struct DataChunk {
    num_packets: u32,
    next_data_header: u32,
}

impl DataChunk {
    pub fn new(num_packets: u32, next_data_header: u32) -> Self {
        Self {
            num_packets,
            next_data_header,
        }
    }

    pub fn description(&self) -> &str {
        "Media Data"
    }

    pub fn fill_properties(&self, properties: &mut Vec<Property>) {
        properties.push(Property::new(
            "num_packets",
            self.num_packets,
            Some(format!("{} packets", self.num_packets)),
        ));
        properties.push(Property::new(
            "next_data_header",
            format!("0x{:08x}", self.next_data_header),
            None::<String>,
        ));
    }
}
