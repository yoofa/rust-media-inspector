use crate::analyzer::isobmff::types::{Fixed16_16, Matrix, Mp4DateTime};

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
        properties.push(("track ID".to_string(), format!("{}", self.track_id)));
        properties.push(("duration".to_string(), format!("{}", self.duration)));
        properties.push(("layer".to_string(), format!("{}", self.layer)));
        properties.push((
            "alternate group".to_string(),
            format!("{}", self.alternate_group),
        ));
        properties.push(("volume".to_string(), format!("{}", self.volume.as_f32())));
        properties.push(("matrix".to_string(), format!("{:?}", self.matrix.values)));
        properties.push(("width".to_string(), format!("{}", self.width.as_f32())));
        properties.push(("height".to_string(), format!("{}", self.height.as_f32())));
    }
}
