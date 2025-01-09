use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use crate::error::MediaError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DetectionStrategy {
    /// 根据文件扩展名检测
    Extension,
    /// 根据文件内容检测
    Content,
    /// 先尝试扩展名,失败后尝试内容检测
    Auto,
}

pub struct FormatDetector {
    strategy: DetectionStrategy,
}

impl FormatDetector {
    pub fn new(strategy: DetectionStrategy) -> Self {
        Self { strategy }
    }

    pub fn detect_format(&self, path: impl AsRef<Path>) -> Result<FileFormat, MediaError> {
        match self.strategy {
            DetectionStrategy::Extension => self.detect_by_extension(path),
            DetectionStrategy::Content => self.detect_by_content(path),
            DetectionStrategy::Auto => {
                // 先尝试扩展名检测
                match self.detect_by_extension(path.as_ref()) {
                    Ok(format) => Ok(format),
                    Err(_) => self.detect_by_content(path), // 失败则尝试内容检测
                }
            }
        }
    }

    fn detect_by_extension(&self, path: impl AsRef<Path>) -> Result<FileFormat, MediaError> {
        let extension = path
            .as_ref()
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase());

        match extension.as_deref() {
            Some("mp4") | Some("mov") | Some("m4a") | Some("m4v") => Ok(FileFormat::Isobmff),
            Some("rm") | Some("rmvb") | Some("ra") => Ok(FileFormat::RealMedia),
            _ => Err(MediaError::UnsupportedFormat(
                "Unknown file extension".to_string(),
            )),
        }
    }

    fn detect_by_content(&self, path: impl AsRef<Path>) -> Result<FileFormat, MediaError> {
        let mut file = File::open(path)?;
        let mut magic = [0u8; 4];
        file.read_exact(&mut magic)?;
        
        // 读完后重置文件指针位置
        file.seek(SeekFrom::Start(0))?;

        match &magic {
            b".RMF" => Ok(FileFormat::RealMedia),
            b"ftyp" | // MP4
            b"moov" | // MOV
            b"mdat" | // MOV/MP4
            b"free" | // MOV/MP4
            b"wide" | // MOV/MP4
            b"skip" => Ok(FileFormat::Isobmff),
            _ => Err(MediaError::UnsupportedFormat(format!(
                "Unknown magic number: {:?}",
                magic
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileFormat {
    Isobmff,
    RealMedia,
} 