use crate::analyzer::rmff::chunks::ChunkInfo;
use crate::analyzer::{ElementInfo, MediaInfo};
use crate::error::MediaError;
use crate::reader::Reader;
use std::fs::File;
use std::path::Path;

mod chunk_parser;
pub mod chunks;
use chunk_parser::ChunkParser;

pub struct RmffAnalyzer {
    parser: ChunkParser,
}

impl RmffAnalyzer {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, MediaError> {
        let file = File::open(path)?;
        let reader = Reader::new(file);
        let parser = ChunkParser::new(reader);

        Ok(Self { parser })
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.parser.set_debug(debug);
    }

    pub fn analyze(&mut self) -> Result<MediaInfo, MediaError> {
        let chunks = self.parser.parse_chunks()?;

        Ok(MediaInfo {
            format: "RealMedia".to_string(),
            duration: None,
            video_streams: Vec::new(),
            audio_streams: Vec::new(),
            metadata: Default::default(),
            structure: Self::convert_to_elements(&chunks),
        })
    }

    fn convert_to_elements(chunks: &[chunks::Chunk]) -> Vec<ElementInfo> {
        chunks
            .iter()
            .map(|chunk| {
                let mut element = ElementInfo::new(
                    &chunk.chunk_type().to_string(),
                    chunk.offset(),
                    chunk.size(),
                );
                element.readable_value = chunk.description().to_string();
                chunk.fill_properties(&mut element.properties);
                element.children = Self::convert_to_elements(chunk.children());
                element
            })
            .collect()
    }
}
