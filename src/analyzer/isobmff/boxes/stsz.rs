use crate::analyzer::Property;
#[derive(Debug)]
pub struct SampleSizeBox {
    version: u8,
    flags: u32,
    sample_size: u32,
    sample_count: u32,
    entry_sizes: Vec<u32>, // Only used if sample_size == 0
}

impl SampleSizeBox {
    pub fn new(
        version: u8,
        flags: u32,
        sample_size: u32,
        sample_count: u32,
        entry_sizes: Vec<u32>,
    ) -> Self {
        Self {
            version,
            flags,
            sample_size,
            sample_count,
            entry_sizes,
        }
    }

    pub fn description(&self) -> &str {
        "Sample Size Box"
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
            "sample_size",
            self.sample_size.to_string(),
            None::<String>,
        ));
        properties.push(Property::new(
            "sample_count",
            self.sample_count.to_string(),
            None::<String>,
        ));

        if self.sample_size == 0 {
            // Only show individual sizes if they're variable
            properties.push(Property::new(
                "entry_sizes",
                format!("{} entries", self.entry_sizes.len()),
                None::<String>,
            ));
        }
    }
}
