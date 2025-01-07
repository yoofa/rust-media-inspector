use crate::analyzer::isobmff::types::Mp4DateTime;

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

    pub fn fill_properties(&self, properties: &mut Vec<(String, String)>) {
        properties.push(("version".to_string(), self.version.to_string()));
        properties.push(("flags".to_string(), format!("0x{:06x}", self.flags)));
        properties.push(("creation_time".to_string(), self.creation_time.to_string()));
        properties.push((
            "modification_time".to_string(),
            self.modification_time.to_string(),
        ));
        properties.push(("timescale".to_string(), self.timescale.to_string()));
        properties.push(("duration".to_string(), self.duration.to_string()));
        properties.push(("language".to_string(), self.language.clone()));
    }
}
