use crate::analyzer::isobmff::types::{Fixed16_16, Matrix, Mp4DateTime};

#[derive(Debug)]
pub struct MovieHeaderBox {
    version: u8,
    flags: u32,
    creation_time: Mp4DateTime,
    modification_time: Mp4DateTime,
    timescale: u32,
    duration: u64,
    rate: Fixed16_16,
    volume: Fixed16_16,
    matrix: Matrix,
    next_track_id: u32,
}

impl MovieHeaderBox {
    pub fn new(
        version: u8,
        flags: u32,
        creation_time: Mp4DateTime,
        modification_time: Mp4DateTime,
        timescale: u32,
        duration: u64,
        rate: Fixed16_16,
        volume: Fixed16_16,
        matrix: Matrix,
        next_track_id: u32,
    ) -> Self {
        Self {
            version,
            flags,
            creation_time,
            modification_time,
            timescale,
            duration,
            rate,
            volume,
            matrix,
            next_track_id,
        }
    }

    pub fn description(&self) -> &str {
        "Movie Header Box"
    }

    pub fn fill_properties(&self, properties: &mut Vec<(String, String)>) {
        properties.push(("version".to_string(), format!("{}", self.version)));
        properties.push(("flags".to_string(), format!("0x{:06x}", self.flags)));
        properties.push((
            "creation time".to_string(),
            format!("{}", self.creation_time),
        ));
        properties.push((
            "modification time".to_string(),
            format!("{}", self.modification_time),
        ));
        properties.push(("timescale".to_string(), format!("{}", self.timescale)));
        properties.push(("duration".to_string(), format!("{}", self.duration)));
        properties.push(("rate".to_string(), format!("{}", self.rate.as_f32())));
        properties.push(("volume".to_string(), format!("{}", self.volume.as_f32())));
        properties.push(("matrix".to_string(), format!("{:?}", self.matrix.values)));
        properties.push((
            "next track ID".to_string(),
            format!("{}", self.next_track_id),
        ));
    }
}
