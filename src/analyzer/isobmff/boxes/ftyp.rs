use crate::analyzer::Property;
#[derive(Debug)]
pub struct FileTypeBox {
    major_brand: String,
    minor_version: u32,
    compatible_brands: Vec<String>,
}

impl FileTypeBox {
    pub fn new(major_brand: String, minor_version: u32, compatible_brands: Vec<String>) -> Self {
        Self {
            major_brand,
            minor_version,
            compatible_brands,
        }
    }

    pub fn description(&self) -> &str {
        "File Type Box"
    }

    pub fn fill_properties(&self, properties: &mut Vec<Property>) {
        properties.push(Property::new(
            "major brand",
            self.major_brand.clone(),
            None::<String>,
        ));
        properties.push(Property::new(
            "minor version",
            format!("{}", self.minor_version),
            None::<String>,
        ));
        properties.push(Property::new(
            "compatible brands",
            self.compatible_brands.join(", "),
            None::<String>,
        ));
    }
}
