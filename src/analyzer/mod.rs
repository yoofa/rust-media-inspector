use std::collections::HashMap;
use std::error::Error;

use detector::{DetectionStrategy, FileFormat, FormatDetector};

pub mod detector;
pub mod isobmff;
pub mod rmff;

use isobmff::IsobmffAnalyzer;
use rmff::RmffAnalyzer;

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

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub readable_value: String,
}

impl Property {
    pub fn new(name: &str, value: impl ToString, readable_value: Option<impl ToString>) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            readable_value: readable_value
                .map(|v| v.to_string())
                .unwrap_or_else(|| value.to_string()),
        }
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct ElementInfo {
    pub name: String,
    pub offset: String,
    pub size: String,
    pub readable_value: String,
    pub children: Vec<ElementInfo>,
    pub properties: Vec<Property>,
}

impl ElementInfo {
    pub fn new(name: &str, offset: impl ToString, size: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            offset: offset.to_string(),
            size: size.to_string(),
            readable_value: String::new(),
            properties: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn add_property(
        &mut self,
        name: &str,
        value: impl ToString,
        readable_value: impl ToString,
    ) {
        self.properties.push(Property {
            name: name.to_string(),
            value: value.to_string(),
            readable_value: readable_value.to_string(),
        });
    }

    pub fn add_child(&mut self, child: ElementInfo) {
        self.children.push(child);
    }
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
    detector: FormatDetector,
}

impl DefaultAnalyzer {
    pub fn new(debug: bool) -> Self {
        Self {
            debug,
            detector: FormatDetector::new(DetectionStrategy::Auto),
        }
    }

    pub fn with_strategy(debug: bool, strategy: DetectionStrategy) -> Self {
        Self {
            debug,
            detector: FormatDetector::new(strategy),
        }
    }
}

impl MediaAnalyzer for DefaultAnalyzer {
    fn analyze(&self, file_path: &str) -> Result<MediaInfo, Box<dyn Error>> {
        let format = self.detector.detect_format(file_path)?;

        match format {
            FileFormat::RealMedia => {
                let mut analyzer = RmffAnalyzer::new(file_path)?;
                analyzer.set_debug(self.debug);
                Ok(analyzer.analyze()?)
            }
            FileFormat::Isobmff => {
                let mut analyzer = IsobmffAnalyzer::new(file_path)?;
                analyzer.set_debug(self.debug);
                Ok(analyzer.analyze()?)
            }
        }
    }
}
