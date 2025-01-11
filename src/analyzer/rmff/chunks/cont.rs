use crate::analyzer::Property;

#[derive(Debug)]
pub struct ContChunk {
    title: String,
    author: String,
    copyright: String,
    comment: String,
}

impl ContChunk {
    pub fn new(title: String, author: String, copyright: String, comment: String) -> Self {
        Self {
            title,
            author,
            copyright,
            comment,
        }
    }

    pub fn description(&self) -> &str {
        "Content Description"
    }

    pub fn fill_properties(&self, properties: &mut Vec<Property>) {
        if !self.title.is_empty() {
            properties.push(Property::new("title", &self.title, None::<String>));
        }
        if !self.author.is_empty() {
            properties.push(Property::new("author", &self.author, None::<String>));
        }
        if !self.copyright.is_empty() {
            properties.push(Property::new("copyright", &self.copyright, None::<String>));
        }
        if !self.comment.is_empty() {
            properties.push(Property::new("comment", &self.comment, None::<String>));
        }
    }
}
