mod dinf;
mod dref;
mod edts;
mod elst;
mod ftyp;
mod hdlr;
mod mdat;
mod mdhd;
mod mdia;
mod minf;
mod moov;
mod mvhd;
mod smhd;
mod stbl;
mod stco;
mod stsc;
mod stsd;
mod stsz;
mod stts;
mod tkhd;
mod trak;
mod vmhd;

use crate::analyzer::isobmff::types::BoxType;

/// Common interface for all box types
pub trait BoxInfo {
    fn box_type(&self) -> BoxType;
    fn size(&self) -> u64;
    fn offset(&self) -> u64;
    fn description(&self) -> &str;
    fn fill_properties(&self, properties: &mut Vec<(String, String)>);
    fn children(&self) -> &[Box];
}

/// Generic box structure
#[derive(Debug)]
pub struct Box {
    box_type: BoxType,
    size: u64,
    offset: u64,
    data: BoxData,
    children: Vec<Box>,
}

impl Box {
    pub fn new(box_type: BoxType, size: u64, offset: u64, data: BoxData) -> Self {
        Self {
            box_type,
            size,
            offset,
            data,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Box) {
        self.children.push(child);
    }
}

impl BoxInfo for Box {
    fn box_type(&self) -> BoxType {
        self.box_type.clone()
    }

    fn size(&self) -> u64 {
        self.size
    }

    fn offset(&self) -> u64 {
        self.offset
    }

    fn description(&self) -> &str {
        self.data.description()
    }

    fn fill_properties(&self, properties: &mut Vec<(String, String)>) {
        self.data.fill_properties(properties);
    }

    fn children(&self) -> &[Box] {
        &self.children
    }
}

pub use dinf::DataInformationBox;
pub use dref::DataEntryBox;
pub use dref::DataReferenceBox;
pub use edts::EditBox;
pub use elst::{EditListBox, EditListEntry};
pub use ftyp::FileTypeBox;
pub use hdlr::HandlerBox;
pub use mdat::MediaDataBox;
pub use mdhd::MediaHeaderBox;
pub use mdia::MediaBox;
pub use minf::MediaInfoBox;
pub use moov::MovieBox;
pub use mvhd::MovieHeaderBox;
pub use smhd::SoundMediaHeaderBox;
pub use stbl::SampleTableBox;
pub use stco::ChunkOffsetBox;
pub use stsc::SampleToChunkBox;
pub use stsc::SampleToChunkEntry;
pub use stsd::SampleDescriptionBox;
pub use stsd::SampleEntry;
pub use stsz::SampleSizeBox;
pub use stts::TimeToSampleBox;
pub use stts::TimeToSampleEntry;
pub use tkhd::TrackHeaderBox;
pub use trak::TrackBox;
pub use vmhd::VideoMediaHeaderBox;

#[derive(Debug)]
pub enum BoxData {
    FileType(FileTypeBox),
    Movie(MovieBox),
    MovieHeader(MovieHeaderBox),
    TrackHeader(TrackHeaderBox),
    Track(TrackBox),
    Edit(EditBox),
    EditList(EditListBox),
    Media(MediaBox),
    MediaHeader(MediaHeaderBox),
    Handler(HandlerBox),
    MediaInfo(MediaInfoBox),
    VideoMediaHeader(VideoMediaHeaderBox),
    SoundMediaHeader(SoundMediaHeaderBox),
    MediaData(MediaDataBox),
    SampleTable(SampleTableBox),
    SampleDescription(SampleDescriptionBox),
    TimeToSample(TimeToSampleBox),
    SampleToChunk(SampleToChunkBox),
    SampleSize(SampleSizeBox),
    ChunkOffset(ChunkOffsetBox),
    DataInformation(DataInformationBox),
    DataReference(DataReferenceBox),
    Unknown,
}

impl BoxData {
    fn description(&self) -> &str {
        match self {
            BoxData::FileType(b) => b.description(),
            BoxData::Movie(b) => b.description(),
            BoxData::MovieHeader(b) => b.description(),
            BoxData::TrackHeader(b) => b.description(),
            BoxData::Track(b) => b.description(),
            BoxData::Edit(b) => b.description(),
            BoxData::EditList(b) => b.description(),
            BoxData::Media(b) => b.description(),
            BoxData::MediaHeader(b) => b.description(),
            BoxData::Handler(b) => b.description(),
            BoxData::MediaInfo(b) => b.description(),
            BoxData::VideoMediaHeader(b) => b.description(),
            BoxData::SoundMediaHeader(b) => b.description(),
            BoxData::MediaData(b) => b.description(),
            BoxData::SampleTable(b) => b.description(),
            BoxData::SampleDescription(b) => b.description(),
            BoxData::TimeToSample(b) => b.description(),
            BoxData::SampleToChunk(b) => b.description(),
            BoxData::SampleSize(b) => b.description(),
            BoxData::ChunkOffset(b) => b.description(),
            BoxData::DataInformation(b) => b.description(),
            BoxData::DataReference(b) => b.description(),
            BoxData::Unknown => "Unknown box type",
        }
    }

    fn fill_properties(&self, properties: &mut Vec<(String, String)>) {
        match self {
            BoxData::FileType(b) => b.fill_properties(properties),
            BoxData::Movie(b) => b.fill_properties(properties),
            BoxData::MovieHeader(b) => b.fill_properties(properties),
            BoxData::TrackHeader(b) => b.fill_properties(properties),
            BoxData::Track(b) => b.fill_properties(properties),
            BoxData::Edit(b) => b.fill_properties(properties),
            BoxData::EditList(b) => b.fill_properties(properties),
            BoxData::Media(b) => b.fill_properties(properties),
            BoxData::MediaHeader(b) => b.fill_properties(properties),
            BoxData::Handler(b) => b.fill_properties(properties),
            BoxData::MediaInfo(b) => b.fill_properties(properties),
            BoxData::VideoMediaHeader(b) => b.fill_properties(properties),
            BoxData::SoundMediaHeader(b) => b.fill_properties(properties),
            BoxData::MediaData(b) => b.fill_properties(properties),
            BoxData::SampleTable(b) => b.fill_properties(properties),
            BoxData::SampleDescription(b) => b.fill_properties(properties),
            BoxData::TimeToSample(b) => b.fill_properties(properties),
            BoxData::SampleToChunk(b) => b.fill_properties(properties),
            BoxData::SampleSize(b) => b.fill_properties(properties),
            BoxData::ChunkOffset(b) => b.fill_properties(properties),
            BoxData::DataInformation(b) => b.fill_properties(properties),
            BoxData::DataReference(b) => b.fill_properties(properties),
            BoxData::Unknown => {}
        }
    }
}
