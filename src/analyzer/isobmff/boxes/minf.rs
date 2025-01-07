use super::Box;

#[derive(Debug)]
#[allow(dead_code)]
pub struct MediaInfoBox {
    children: Vec<Box>,
}

impl MediaInfoBox {
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
        "Media Information Box"
    }

    pub fn fill_properties(&self, _properties: &mut Vec<(String, String)>) {
        // Media info box itself doesn't have properties
    }

    #[allow(dead_code)]
    pub fn children(&self) -> &[Box] {
        &self.children
    }
}
