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

    pub fn fill_properties(&self, properties: &mut Vec<(String, String)>) {
        properties.push(("major brand".to_string(), self.major_brand.clone()));
        properties.push((
            "minor version".to_string(),
            format!("{}", self.minor_version),
        ));
        properties.push((
            "compatible brands".to_string(),
            self.compatible_brands.join(", "),
        ));
    }
}
