use crate::analyzer::isobmff::types::{Fixed16_16, Matrix, Mp4DateTime};
use crate::analyzer::Property;
#[derive(Debug)]
pub struct TrackHeaderBox {
    version: u8,
    flags: u32,
    creation_time: Mp4DateTime,
    modification_time: Mp4DateTime,
    track_id: u32,
    duration: u64,
    layer: i16,
    alternate_group: i16,
    volume: Fixed16_16,
    matrix: Matrix,
    width: Fixed16_16,
    height: Fixed16_16,
}

impl TrackHeaderBox {
    pub fn new(
        version: u8,
        flags: u32,
        creation_time: Mp4DateTime,
        modification_time: Mp4DateTime,
        track_id: u32,
        duration: u64,
        layer: i16,
        alternate_group: i16,
        volume: Fixed16_16,
        matrix: Matrix,
        width: Fixed16_16,
        height: Fixed16_16,
    ) -> Self {
        Self {
            version,
            flags,
            creation_time,
            modification_time,
            track_id,
            duration,
            layer,
            alternate_group,
            volume,
            matrix,
            width,
            height,
        }
    }

    pub fn description(&self) -> &str {
        "Track Header Box"
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
            "track ID",
            format!("{}", self.track_id),
            None::<String>,
        ));
        properties.push(Property::new(
            "duration",
            format!("{}", self.duration),
            None::<String>,
        ));
        properties.push(Property::new(
            "layer",
            format!("{}", self.layer),
            None::<String>,
        ));
        properties.push(Property::new(
            "alternate group",
            format!("{}", self.alternate_group),
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
            "width",
            format!("{}", self.width.as_f32()),
            None::<String>,
        ));
        properties.push(Property::new(
            "height",
            format!("{}", self.height.as_f32()),
            None::<String>,
        ));
    }
}
