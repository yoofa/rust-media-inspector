#[derive(Debug)]
pub struct MediaDataBox {
    data_size: u64,
}

impl MediaDataBox {
    pub fn new(data_size: u64) -> Self {
        Self { data_size }
    }

    pub fn description(&self) -> &str {
        "Media Data Box"
    }

    pub fn fill_properties(&self, properties: &mut Vec<(String, String)>) {
        properties.push(("data size".to_string(), format!("{}", self.data_size)));
    }
}
