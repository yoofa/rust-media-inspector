use crate::analyzer::isobmff::types::Mp4DateTime;
use crate::analyzer::Property;
#[derive(Debug)]
pub struct MediaHeaderBox {
    version: u8,
    flags: u32,
    creation_time: Mp4DateTime,
    modification_time: Mp4DateTime,
    timescale: u32,
    duration: u64,
    language: String, // ISO-639-2/T language code
}

impl MediaHeaderBox {
    pub fn new(
        version: u8,
        flags: u32,
        creation_time: Mp4DateTime,
        modification_time: Mp4DateTime,
        timescale: u32,
        duration: u64,
        language: String,
    ) -> Self {
        Self {
            version,
            flags,
            creation_time,
            modification_time,
            timescale,
            duration,
            language,
        }
    }

    pub fn description(&self) -> &str {
        "Media Header Box"
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
            "creation_time",
            self.creation_time.to_string(),
            None::<String>,
        ));
        properties.push(Property::new(
            "modification_time",
            self.modification_time.to_string(),
            None::<String>,
        ));
        properties.push(Property::new(
            "timescale",
            self.timescale.to_string(),
            None::<String>,
        ));
        properties.push(Property::new(
            "duration",
            self.duration.to_string(),
            None::<String>,
        ));
        properties.push(Property::new(
            "language",
            self.language.clone(),
            None::<String>,
        ));
    }
}
