pub mod box_parser;
pub mod boxes;
pub mod types;

use crate::error::MediaError;
use crate::reader::Reader;
pub use box_parser::BoxParser;
use boxes::BoxInfo;

use crate::analyzer::{ElementInfo, MediaInfo};
use std::fs::File;
use std::path::Path;

pub struct IsobmffAnalyzer {
    parser: BoxParser,
}

impl IsobmffAnalyzer {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, MediaError> {
        let file = File::open(path)?;
        let reader = Reader::new(file);
        let parser = BoxParser::new(reader);

        Ok(Self { parser })
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.parser.set_debug(debug);
    }

    pub fn analyze(&mut self) -> Result<MediaInfo, MediaError> {
        let boxes = self.parser.parse_boxes()?;

        Ok(MediaInfo {
            format: "ISOBMFF".to_string(),
            duration: None,
            video_streams: Vec::new(),
            audio_streams: Vec::new(),
            metadata: Default::default(),
            structure: Self::convert_to_elements(&boxes),
        })
    }

    fn convert_to_elements(boxes: &[boxes::Box]) -> Vec<ElementInfo> {
        boxes
            .iter()
            .map(|box_info| {
                let mut element = ElementInfo::new(
                    &box_info.box_type().to_string(),
                    box_info.offset(),
                    box_info.size(),
                );
                element.readable_value = box_info.description().to_string();

                // 让box自己填充属性
                box_info.fill_properties(&mut element.properties);
                element.children = Self::convert_to_elements(box_info.children());
                element
            })
            .collect()
    }
}
