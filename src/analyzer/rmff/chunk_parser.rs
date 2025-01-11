use super::chunks::{Chunk, ChunkData, ChunkType};
use crate::error::MediaError;
use crate::reader::Reader;

pub struct ChunkParser {
    reader: Reader,
    debug: bool,
}

impl ChunkParser {
    pub fn new(reader: Reader) -> Self {
        Self {
            reader,
            debug: true,
        }
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn parse_chunks(&mut self) -> Result<Vec<Chunk>, MediaError> {
        let mut chunks = Vec::new();

        // 首先验证文件头的魔数 ".RMF"
        let mut magic = [0u8; 4];
        self.reader.read_exact(&mut magic)?;
        if &magic != b".RMF" {
            return Err(MediaError::InvalidFormat("Not a RealMedia file".into()));
        }

        // 读取文件头版本
        let version = self.reader.read_u32()?;
        if version != 0 {
            return Err(MediaError::InvalidFormat(format!(
                "Unsupported RealMedia version: {}",
                version
            )));
        }

        // 读取文件大小
        let file_size = self.reader.read_u32()? as u64;

        // 读取所有chunks
        while let Ok((size, chunk_type, offset)) = self.read_chunk_header() {
            if self.debug {
                println!(
                    "Parsing chunk: type={}, offset={}, size={}",
                    chunk_type.as_str(),
                    offset,
                    size
                );
            }

            let data = match chunk_type.as_str() {
                "PROP" => {
                    let max_bit_rate = self.reader.read_u32()?;
                    let avg_bit_rate = self.reader.read_u32()?;
                    let max_packet_size = self.reader.read_u32()?;
                    let avg_packet_size = self.reader.read_u32()?;
                    let num_packets = self.reader.read_u32()?;
                    let duration = self.reader.read_u32()?;
                    let preroll = self.reader.read_u32()?;
                    let index_offset = self.reader.read_u32()?;
                    let data_offset = self.reader.read_u32()?;
                    let num_streams = self.reader.read_u16()?;
                    let flags = self.reader.read_u16()?;

                    ChunkData::Prop(PropChunk::new(
                        max_bit_rate,
                        avg_bit_rate,
                        max_packet_size,
                        avg_packet_size,
                        num_packets,
                        duration,
                        preroll,
                        index_offset,
                        data_offset,
                        num_streams,
                        flags,
                    ))
                }
                "CONT" => {
                    let title_len = self.reader.read_u16()? as u64;
                    let author_len = self.reader.read_u16()? as u64;
                    let copyright_len = self.reader.read_u16()? as u64;
                    let comment_len = self.reader.read_u16()? as u64;

                    let title = self.reader.read_string(title_len)?;
                    let author = self.reader.read_string(author_len)?;
                    let copyright = self.reader.read_string(copyright_len)?;
                    let comment = self.reader.read_string(comment_len)?;

                    ChunkData::Cont(ContChunk::new(title, author, copyright, comment))
                }
                "MDPR" => {
                    let stream_number = self.reader.read_u16()?;
                    let max_bit_rate = self.reader.read_u32()?;
                    let avg_bit_rate = self.reader.read_u32()?;
                    let max_packet_size = self.reader.read_u32()?;
                    let avg_packet_size = self.reader.read_u32()?;
                    let start_time = self.reader.read_u32()?;
                    let preroll = self.reader.read_u32()?;
                    let duration = self.reader.read_u32()?;
                    let stream_name_len = self.reader.read_u8()? as u64;
                    let mime_type_len = self.reader.read_u8()? as u64;
                    let type_specific_len = self.reader.read_u32()?;

                    let stream_name = self.reader.read_string(stream_name_len)?;
                    let mime_type = self.reader.read_string(mime_type_len)?;
                    let mut type_specific_data = vec![0u8; type_specific_len as usize];
                    self.reader.read_exact(&mut type_specific_data)?;

                    ChunkData::Mdpr(MdprChunk::new(
                        stream_number,
                        max_bit_rate,
                        avg_bit_rate,
                        max_packet_size,
                        avg_packet_size,
                        start_time,
                        preroll,
                        duration,
                        stream_name,
                        mime_type,
                        type_specific_len,
                        type_specific_data,
                    ))
                }
                "DATA" => {
                    let num_packets = self.reader.read_u32()?;
                    let next_data_header = self.reader.read_u32()?;

                    // 跳过数据部分
                    self.reader.skip(size - 16)?; // 16 = header(8) + num_packets(4) + next_data_header(4)

                    ChunkData::Data(DataChunk::new(num_packets, next_data_header))
                }
                "INDX" => {
                    let num_entries = self.reader.read_u32()?;
                    let stream_number = self.reader.read_u16()?;
                    let next_index_header = self.reader.read_u32()?;

                    let mut entries = Vec::with_capacity(num_entries as usize);
                    for _ in 0..num_entries {
                        let timestamp = self.reader.read_u32()?;
                        let offset = self.reader.read_u32()?;
                        let packet_count = self.reader.read_u32()?;

                        entries.push(IndexEntry::new(timestamp, offset, packet_count));
                    }

                    ChunkData::Indx(IndxChunk::new(
                        num_entries,
                        stream_number,
                        next_index_header,
                        entries,
                    ))
                }
                _ => {
                    // 跳过未知chunk的数据部分
                    self.reader.skip(size - 8)?; // 8 = header size
                    ChunkData::Unknown
                }
            };

            chunks.push(Chunk::new(chunk_type, size, offset, data));
        }

        Ok(chunks)
    }

    fn read_chunk_header(&mut self) -> Result<(u64, ChunkType, u64), MediaError> {
        let offset = self.reader.position();

        // 读取chunk大小 (u32)
        let size = self.reader.read_u32()? as u64;

        // 读取chunk类型 (4字节)
        let mut type_buf = [0u8; 4];
        self.reader.read_exact(&mut type_buf)?;
        let chunk_type = ChunkType::new(type_buf);

        Ok((size, chunk_type, offset))
    }
}
