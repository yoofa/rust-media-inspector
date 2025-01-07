use super::Box;

#[derive(Debug)]
#[allow(dead_code)]
pub struct MediaBox {
    children: Vec<Box>,
}

impl MediaBox {
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
        "Media Box"
    }

    pub fn fill_properties(&self, _properties: &mut Vec<(String, String)>) {
        // Media box itself doesn't have properties
    }

    #[allow(dead_code)]
    pub fn children(&self) -> &[Box] {
        &self.children
    }
}
