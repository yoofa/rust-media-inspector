use crate::analyzer::isobmff::types::{Fixed16_16, Matrix, Mp4DateTime};
use crate::analyzer::Property;
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

    pub fn fill_properties(&self, properties: &mut Vec<Property>) {
        properties.push(Property::new(
            "version",
            format!("{}", self.version),
            None::<String>,
        ));
        properties.push(Property::new(
            "flags",
            format!("0x{:06x}", self.flags),
            None::<String>,
        ));
        properties.push(Property::new(
            "creation time",
            format!("{}", self.creation_time),
            None::<String>,
        ));
        properties.push(Property::new(
            "modification time",
            format!("{}", self.modification_time),
            None::<String>,
        ));
        properties.push(Property::new(
            "timescale",
            format!("{}", self.timescale),
            None::<String>,
        ));
        properties.push(Property::new(
            "duration",
            format!("{}", self.duration),
            None::<String>,
        ));
        properties.push(Property::new(
            "rate",
            format!("{}", self.rate.as_f32()),
            None::<String>,
        ));
        properties.push(Property::new(
            "volume",
            format!("{}", self.volume.as_f32()),
            None::<String>,
        ));
        properties.push(Property::new(
            "matrix",
            format!("{:?}", self.matrix.values),
            None::<String>,
        ));
        properties.push(Property::new(
            "next track ID",
            format!("{}", self.next_track_id),
            None::<String>,
        ));
    }
}
