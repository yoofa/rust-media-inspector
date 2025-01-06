use super::Box;

#[derive(Debug)]
#[allow(dead_code)]
pub struct TrackBox {
    children: Vec<Box>,
}

impl TrackBox {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_child(&mut self, child: Box) {
        self.children.push(child);
    }

    pub fn description(&self) -> &str {
        "Track Box"
    }

    pub fn fill_properties(&self, _properties: &mut Vec<(String, String)>) {
        // Track box itself doesn't have properties, its children contain the actual data
    }

    #[allow(dead_code)]
    pub fn children(&self) -> &[Box] {
        &self.children
    }
}
