use std::collections::HashMap;
use std::error::Error;

pub mod isobmff;
use isobmff::IsobmffAnalyzer;

#[allow(dead_code)]
pub trait MediaAnalyzer {
    fn analyze(&self, file_path: &str) -> Result<MediaInfo, Box<dyn Error>>;
}

#[allow(dead_code)]
pub struct MediaInfo {
    pub format: String,
    pub duration: Option<f64>,
    pub video_streams: Vec<VideoStream>,
    pub audio_streams: Vec<AudioStream>,
    pub metadata: HashMap<String, String>,
    pub structure: Vec<ElementInfo>,
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct ElementInfo {
    pub name: String,
    pub offset: String,
    pub size: String,
    pub value: String,
    pub children: Vec<ElementInfo>,
    pub properties: Vec<(String, String)>,
}

#[allow(dead_code)]
pub struct VideoStream {
    pub codec: String,
    pub width: u32,
    pub height: u32,
    pub frame_rate: f64,
    pub bit_rate: Option<u64>,
}

#[allow(dead_code)]
pub struct AudioStream {
    pub codec: String,
    pub channels: u8,
    pub sample_rate: u32,
    pub bit_rate: Option<u64>,
}

#[allow(dead_code)]
pub struct DefaultAnalyzer {
    debug: bool,
}

#[allow(dead_code)]
impl DefaultAnalyzer {
    pub fn new(debug: bool) -> Self {
        Self { debug }
    }
}

impl MediaAnalyzer for DefaultAnalyzer {
    fn analyze(&self, file_path: &str) -> Result<MediaInfo, Box<dyn Error>> {
        // Try to detect file format and use appropriate analyzer
        if let Ok(mut analyzer) = IsobmffAnalyzer::new(file_path) {
            analyzer.set_debug(self.debug);
            return Ok(analyzer.analyze()?);
        }
        // TODO: Add more format analyzers here

        Err("Unsupported format".into())
    }
}
