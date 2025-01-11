use crate::analyzer::Property;

mod cont;
mod data;
mod indx;
mod mdpr;
mod prop;

pub use cont::ContChunk;
pub use data::DataChunk;
pub use indx::{IndexEntry, IndxChunk};
pub use mdpr::MdprChunk;
pub use prop::PropChunk;

/// Common interface for all chunk types
pub trait ChunkInfo {
    fn chunk_type(&self) -> ChunkType;
    fn size(&self) -> u64;
    fn offset(&self) -> u64;
    fn description(&self) -> &str;
    fn fill_properties(&self, properties: &mut Vec<Property>);
    fn children(&self) -> &[Chunk];
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ChunkType([u8; 4]);

impl ChunkType {
    pub fn new(value: [u8; 4]) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).unwrap_or("????")
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Generic chunk structure
#[derive(Debug)]
pub struct Chunk {
    chunk_type: ChunkType,
    size: u64,
    offset: u64,
    data: ChunkData,
    children: Vec<Chunk>,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, size: u64, offset: u64, data: ChunkData) -> Self {
        Self {
            chunk_type,
            size,
            offset,
            data,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Chunk) {
        self.children.push(child);
    }
}

impl ChunkInfo for Chunk {
    fn chunk_type(&self) -> ChunkType {
        self.chunk_type
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

    fn fill_properties(&self, properties: &mut Vec<Property>) {
        self.data.fill_properties(properties);
    }

    fn children(&self) -> &[Chunk] {
        &self.children
    }
}

#[derive(Debug)]
pub enum ChunkData {
    Prop(PropChunk),
    Cont(ContChunk),
    Mdpr(MdprChunk),
    Data(DataChunk),
    Indx(IndxChunk),
    Unknown,
}

impl ChunkData {
    fn description(&self) -> &str {
        match self {
            ChunkData::Prop(chunk) => chunk.description(),
            ChunkData::Cont(chunk) => chunk.description(),
            ChunkData::Mdpr(chunk) => chunk.description(),
            ChunkData::Data(chunk) => chunk.description(),
            ChunkData::Indx(chunk) => chunk.description(),
            ChunkData::Unknown => "Unknown chunk type",
        }
    }

    fn fill_properties(&self, properties: &mut Vec<Property>) {
        match self {
            ChunkData::Prop(chunk) => chunk.fill_properties(properties),
            ChunkData::Cont(chunk) => chunk.fill_properties(properties),
            ChunkData::Mdpr(chunk) => chunk.fill_properties(properties),
            ChunkData::Data(chunk) => chunk.fill_properties(properties),
            ChunkData::Indx(chunk) => chunk.fill_properties(properties),
            ChunkData::Unknown => {}
        }
    }
}
