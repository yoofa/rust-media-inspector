use crate::analyzer::Property;
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
            "graphics_mode",
            self.graphics_mode.to_string(),
            None::<String>,
        ));
        properties.push(Property::new(
            "op_color",
            format!(
                "[{}, {}, {}]",
                self.op_color[0], self.op_color[1], self.op_color[2]
            ),
            None::<String>,
        ));
    }
}
