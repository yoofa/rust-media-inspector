#[derive(Debug)]
pub struct VideoMediaHeaderBox {
    version: u8,
    flags: u32,
    graphics_mode: u16,
    op_color: [u16; 3], // RGB color for compositing
}

impl VideoMediaHeaderBox {
    pub fn new(version: u8, flags: u32, graphics_mode: u16, op_color: [u16; 3]) -> Self {
        Self {
            version,
            flags,
            graphics_mode,
            op_color,
        }
    }

    pub fn description(&self) -> &str {
        "Video Media Header Box"
    }

    pub fn fill_properties(&self, properties: &mut Vec<(String, String)>) {
        properties.push(("version".to_string(), self.version.to_string()));
        properties.push(("flags".to_string(), format!("0x{:06x}", self.flags)));
        properties.push(("graphics_mode".to_string(), self.graphics_mode.to_string()));
        properties.push((
            "op_color".to_string(),
            format!(
                "[{}, {}, {}]",
                self.op_color[0], self.op_color[1], self.op_color[2]
            ),
        ));
    }
}
