use super::boxes::{
    Box, BoxData, BoxInfo, ChunkOffsetBox, DataEntryBox, DataInformationBox, DataReferenceBox,
    EditBox, EditListBox, EditListEntry, FileTypeBox, HandlerBox, MediaBox, MediaDataBox,
    MediaHeaderBox, MediaInfoBox, MovieBox, MovieHeaderBox, SampleDescriptionBox, SampleEntry,
    SampleSizeBox, SampleTableBox, SampleToChunkBox, SampleToChunkEntry, SoundMediaHeaderBox,
    TimeToSampleBox, TimeToSampleEntry, TrackBox, TrackHeaderBox, VideoMediaHeaderBox,
};
use super::types::{BoxType, Fixed16_16, Matrix, Mp4DateTime};
use crate::error::MediaError;
use crate::reader::Reader;
use std::io::SeekFrom;

pub struct BoxParser {
    reader: Reader,
    debug: bool,
}

impl BoxParser {
    pub fn new(reader: Reader) -> Self {
        Self {
            reader,
            debug: true,
        }
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn parse_boxes(&mut self) -> Result<Vec<Box>, MediaError> {
        let mut boxes = Vec::new();
        while let Ok(box_info) = self.parse_box() {
            boxes.push(box_info);
        }
        Ok(boxes)
    }

    fn parse_box(&mut self) -> Result<Box, MediaError> {
        let offset = self.reader.position();

        // Read box size
        let size = self.reader.read_u32().map_err(MediaError::from)? as u64;

        // Read box type
        let mut type_buf = [0u8; 4];
        self.reader
            .read_exact(&mut type_buf)
            .map_err(MediaError::from)?;
        let box_type = BoxType::new(type_buf);

        // Handle 64-bit size
        let (actual_size, header_size) = if size == 1 {
            let large_size = self.reader.read_u64().map_err(MediaError::from)?;
            (large_size, 16)
        } else {
            (size as u64, 8)
        };

        if self.debug {
            println!(
                "Parsing box: type={}, offset={}, size={}, header_size={}, data_size={}",
                box_type.as_str(),
                offset,
                actual_size,
                header_size,
                actual_size - header_size
            );
        }

        // Parse box data based on type
        let data = match box_type.as_str() {
            "ftyp" => self.parse_ftyp(actual_size - header_size)?,
            "moov" => BoxData::Movie(self.parse_moov(actual_size - header_size)?),
            "mvhd" => self.parse_mvhd(actual_size - header_size)?,
            "trak" => BoxData::Track(self.parse_trak(actual_size - header_size)?),
            "tkhd" => self.parse_tkhd(actual_size - header_size)?,
            "mdat" => BoxData::MediaData(self.parse_mdat(actual_size - header_size)?),
            "edts" => BoxData::Edit(self.parse_edts(actual_size - header_size)?),
            "elst" => BoxData::EditList(self.parse_elst(actual_size - header_size)?),
            "mdia" => BoxData::Media(self.parse_mdia(actual_size - header_size)?),
            "mdhd" => self.parse_mdhd(actual_size - header_size)?,
            "hdlr" => self.parse_hdlr(actual_size - header_size)?,
            "minf" => BoxData::MediaInfo(self.parse_minf(actual_size - header_size)?),
            "vmhd" => self.parse_vmhd(actual_size - header_size)?,
            "smhd" => self.parse_smhd(actual_size - header_size)?,
            "stbl" => BoxData::SampleTable(self.parse_stbl(actual_size - header_size)?),
            "stsd" => self.parse_stsd(actual_size - header_size)?,
            "stts" => self.parse_stts(actual_size - header_size)?,
            "stsc" => self.parse_stsc(actual_size - header_size)?,
            "stsz" => self.parse_stsz(actual_size - header_size)?,
            "stco" => self.parse_stco(actual_size - header_size)?,
            "dinf" => BoxData::DataInformation(self.parse_dinf(actual_size - header_size)?),
            "dref" => BoxData::DataReference(self.parse_dref(actual_size - header_size)?),
            _ => {
                if self.debug {
                    println!(
                        "Skipping unknown box: type={}, offset={}, size={}, next_offset={}",
                        box_type.as_str(),
                        offset,
                        actual_size,
                        offset + actual_size
                    );
                }
                self.reader.skip(actual_size - header_size)?;
                BoxData::Unknown
            }
        };

        let mut box_info = Box::new(box_type, actual_size, offset, data);

        // Parse children for container boxes
        match box_info.box_type().as_str() {
            "moov" | "trak" | "mdia" | "minf" | "stbl" | "dinf" | "edts" => {
                let end_offset = offset + actual_size;
                if self.debug {
                    println!(
                        "Parsing container box: type={}, start={}, end={}",
                        box_type.as_str(),
                        offset,
                        end_offset
                    );
                }
                while self.reader.position() < end_offset {
                    if let Ok(child) = self.parse_box() {
                        box_info.add_child(child);
                    } else {
                        if self.debug {
                            println!(
                                "Error parsing child box at offset {}, skipping to {}",
                                self.reader.position(),
                                end_offset
                            );
                        }
                        self.reader.seek(SeekFrom::Start(end_offset))?;
                        break;
                    }
                }
            }
            _ => {}
        }

        if self.debug {
            println!(
                "Finished box: type={}, offset={}, size={}, next_offset={}",
                box_type.as_str(),
                offset,
                actual_size,
                offset + actual_size
            );
        }

        Ok(box_info)
    }

    fn parse_ftyp(&mut self, size: u64) -> Result<BoxData, MediaError> {
        // Read major brand (4 bytes)
        let major_brand = self.reader.read_fixed_string(4)?;

        // Read minor version (4 bytes)
        let minor_version = self.reader.read_u32()?;

        // Read compatible brands (remaining bytes)
        let remaining_size = size - 8; // subtract size of major_brand and minor_version
        let num_brands = remaining_size / 4;
        let mut compatible_brands = Vec::with_capacity(num_brands as usize);

        for _ in 0..num_brands {
            compatible_brands.push(self.reader.read_fixed_string(4)?);
        }

        Ok(BoxData::FileType(FileTypeBox::new(
            major_brand,
            minor_version,
            compatible_brands,
        )))
    }

    fn parse_moov(&mut self, _size: u64) -> Result<MovieBox, MediaError> {
        Ok(MovieBox::new())
    }

    fn parse_mvhd(&mut self, size: u64) -> Result<BoxData, MediaError> {
        let start_pos = self.reader.position();

        // Read version and flags
        let version = self.reader.read_u8()?;
        let flags = self.reader.read_u24()?;

        // Read timestamps based on version
        let (creation_time, modification_time, timescale, duration) = if version == 1 {
            (
                Mp4DateTime::new(self.reader.read_u64()?),
                Mp4DateTime::new(self.reader.read_u64()?),
                self.reader.read_u32()?,
                self.reader.read_u64()?,
            )
        } else {
            (
                Mp4DateTime::new(self.reader.read_u32()? as u64),
                Mp4DateTime::new(self.reader.read_u32()? as u64),
                self.reader.read_u32()?,
                self.reader.read_u32()? as u64,
            )
        };

        // Read rate (fixed point 16.16)
        let rate = Fixed16_16::new(self.reader.read_i32()?);

        // Read volume (fixed point 8.8) and reserved
        let volume = Fixed16_16::new((self.reader.read_u16()? as i32) << 16);

        self.reader.skip(2)?; // reserved

        // Skip reserved
        self.reader.skip(8)?; // two uint32 reserved

        // Read matrix
        let mut matrix_values = [0i32; 9];
        for value in &mut matrix_values {
            *value = self.reader.read_i32()?;
        }
        let matrix = Matrix::new(matrix_values);

        let _preview_time = self.reader.read_u32()?;
        let _preview_duration = self.reader.read_u32()?;
        let _poster_time = self.reader.read_u32()?;
        let _selection_time = self.reader.read_u32()?;
        let _selection_duration = self.reader.read_u32()?;
        let _current_time = self.reader.read_u32()?;

        // Read next_track_id
        let next_track_id = self.reader.read_u32()?;

        // Verify we read exactly the right number of bytes
        let bytes_read = self.reader.position() - start_pos;
        if bytes_read != size {
            if self.debug {
                println!(
                    "Warning: mvhd box size mismatch. Expected {} bytes, read {} bytes",
                    size, bytes_read
                );
            }
            // Adjust position if necessary
            if bytes_read < size {
                self.reader.skip(size - bytes_read)?;
            }
        }

        Ok(BoxData::MovieHeader(MovieHeaderBox::new(
            version,
            flags,
            creation_time,
            modification_time,
            timescale,
            duration,
            rate,
            volume,
            matrix,
            next_track_id,
        )))
    }

    fn parse_mdat(&mut self, size: u64) -> Result<MediaDataBox, MediaError> {
        // Just skip the data content
        self.reader.skip(size)?;
        Ok(MediaDataBox::new(size))
    }

    fn parse_tkhd(&mut self, size: u64) -> Result<BoxData, MediaError> {
        let start_pos = self.reader.position();

        // Read version and flags
        let version = self.reader.read_u8()?;
        let flags = self.reader.read_u24()?;

        // Read timestamps based on version
        let (creation_time, modification_time, track_id, duration) = if version == 1 {
            // Version 1: 64-bit timestamps
            let creation = self.reader.read_u64()?;
            let modification = self.reader.read_u64()?;
            let track = self.reader.read_u32()?;
            self.reader.skip(4)?; // reserved
            let dur = self.reader.read_u64()?;
            (
                Mp4DateTime::new(creation),
                Mp4DateTime::new(modification),
                track,
                dur,
            )
        } else {
            // Version 0: 32-bit timestamps
            let creation = self.reader.read_u32()?;
            let modification = self.reader.read_u32()?;
            let track = self.reader.read_u32()?;
            self.reader.skip(4)?; // reserved
            let dur = self.reader.read_u32()?;
            (
                Mp4DateTime::new(creation as u64),
                Mp4DateTime::new(modification as u64),
                track,
                dur as u64,
            )
        };

        self.reader.skip(8)?; // two uint32 reserved

        // Read layer and alternate_group
        let layer = self.reader.read_i16()?;
        let alternate_group = self.reader.read_i16()?;

        // Read volume (fixed point 8.8)
        let volume = Fixed16_16::new(self.reader.read_i16()? as i32);
        self.reader.skip(2)?; // reserved

        // Read matrix
        let mut matrix_values = [0i32; 9];
        for value in &mut matrix_values {
            *value = self.reader.read_i32()?;
        }
        let matrix = Matrix::new(matrix_values);

        // Read width and height (fixed point 16.16)
        let width = Fixed16_16::new(self.reader.read_i32()?);
        let height = Fixed16_16::new(self.reader.read_i32()?);

        // Verify we read exactly the right number of bytes
        let bytes_read = self.reader.position() - start_pos;
        if bytes_read != size {
            if self.debug {
                println!(
                    "Warning: tkhd box size mismatch. Expected {} bytes, read {} bytes",
                    size, bytes_read
                );
            }
            // Adjust position if necessary
            if bytes_read < size {
                self.reader.skip(size - bytes_read)?;
            }
        }

        Ok(BoxData::TrackHeader(TrackHeaderBox::new(
            version,
            flags,
            creation_time,
            modification_time,
            track_id,
            duration,
            layer,
            alternate_group,
            volume,
            matrix,
            width,
            height,
        )))
    }

    fn parse_trak(&mut self, _size: u64) -> Result<TrackBox, MediaError> {
        Ok(TrackBox::new())
    }

    fn parse_edts(&mut self, _size: u64) -> Result<EditBox, MediaError> {
        Ok(EditBox::new())
    }

    fn parse_elst(&mut self, size: u64) -> Result<EditListBox, MediaError> {
        let start_pos = self.reader.position();

        // Read version and flags
        let version = self.reader.read_u8()?;
        let flags = self.reader.read_u24()?;

        // Read entry count
        let entry_count = self.reader.read_u32()?;
        let mut entries = Vec::with_capacity(entry_count as usize);

        // Read entries
        for _ in 0..entry_count {
            let (segment_duration, media_time) = if version == 1 {
                (
                    self.reader.read_u64()? as u32,
                    self.reader.read_i64()? as i32,
                )
            } else {
                (self.reader.read_u32()?, self.reader.read_i32()?)
            };

            let media_rate = self.reader.read_i16()?;
            let media_rate_fraction = self.reader.read_i16()?;

            entries.push(EditListEntry::new(
                segment_duration,
                media_time,
                media_rate,
                media_rate_fraction,
            ));
        }

        // Verify we read exactly the right number of bytes
        let bytes_read = self.reader.position() - start_pos;
        if bytes_read != size {
            if self.debug {
                println!(
                    "Warning: elst box size mismatch. Expected {} bytes, read {} bytes",
                    size, bytes_read
                );
            }
            // Adjust position if necessary
            if bytes_read < size {
                self.reader.skip(size - bytes_read)?;
            }
        }

        Ok(EditListBox::new(version, flags, entries))
    }

    fn parse_mdia(&mut self, _size: u64) -> Result<MediaBox, MediaError> {
        Ok(MediaBox::new())
    }

    fn parse_mdhd(&mut self, size: u64) -> Result<BoxData, MediaError> {
        let start_pos = self.reader.position();

        // Read version and flags
        let version = self.reader.read_u8()?;
        let flags = self.reader.read_u24()?;

        // Read timestamps based on version
        let (creation_time, modification_time, timescale, duration) = if version == 1 {
            (
                Mp4DateTime::new(self.reader.read_u64()?),
                Mp4DateTime::new(self.reader.read_u64()?),
                self.reader.read_u32()?,
                self.reader.read_u64()?,
            )
        } else {
            (
                Mp4DateTime::new(self.reader.read_u32()? as u64),
                Mp4DateTime::new(self.reader.read_u32()? as u64),
                self.reader.read_u32()?,
                self.reader.read_u32()? as u64,
            )
        };

        // Read language (ISO-639-2/T language code)
        let lang_packed = self.reader.read_u16()?;
        let language = format!(
            "{}{}{}",
            ((lang_packed >> 10) & 0x1F + 0x60) as u8 as char,
            ((lang_packed >> 5) & 0x1F + 0x60) as u8 as char,
            (lang_packed & 0x1F + 0x60) as u8 as char
        );

        // Skip pre_defined
        self.reader.skip(2)?;

        // Verify we read exactly the right number of bytes
        let bytes_read = self.reader.position() - start_pos;
        if bytes_read != size {
            if self.debug {
                println!(
                    "Warning: mdhd box size mismatch. Expected {} bytes, read {} bytes",
                    size, bytes_read
                );
            }
            // Adjust position if necessary
            if bytes_read < size {
                self.reader.skip(size - bytes_read)?;
            }
        }

        Ok(BoxData::MediaHeader(MediaHeaderBox::new(
            version,
            flags,
            creation_time,
            modification_time,
            timescale,
            duration,
            language,
        )))
    }

    fn parse_hdlr(&mut self, size: u64) -> Result<BoxData, MediaError> {
        let start_pos = self.reader.position();

        // Read version and flags
        let version = self.reader.read_u8()?;
        let flags = self.reader.read_u24()?;

        // Skip pre_defined
        self.reader.skip(4)?;

        // Read handler type
        let handler_type = self.reader.read_fixed_string(4)?;

        // Skip reserved[3]
        self.reader.skip(12)?;

        // Read name (remaining bytes, null-terminated string)
        let bytes_read = self.reader.position() - start_pos;
        let remaining_size = size - bytes_read;
        let mut name_bytes = vec![0u8; remaining_size as usize];
        self.reader.read_exact(&mut name_bytes)?;

        // Remove trailing null bytes
        while name_bytes.last() == Some(&0) {
            name_bytes.pop();
        }
        let name = String::from_utf8_lossy(&name_bytes).to_string();

        Ok(BoxData::Handler(HandlerBox::new(
            version,
            flags,
            handler_type,
            name,
        )))
    }

    fn parse_minf(&mut self, _size: u64) -> Result<MediaInfoBox, MediaError> {
        Ok(MediaInfoBox::new())
    }

    fn parse_vmhd(&mut self, size: u64) -> Result<BoxData, MediaError> {
        let start_pos = self.reader.position();

        // Read version and flags
        let version = self.reader.read_u8()?;
        let flags = self.reader.read_u24()?;

        // Read graphics mode
        let graphics_mode = self.reader.read_u16()?;

        // Read opcolor values
        let mut op_color = [0u16; 3];
        for value in &mut op_color {
            *value = self.reader.read_u16()?;
        }

        // Verify we read exactly the right number of bytes
        let bytes_read = self.reader.position() - start_pos;
        if bytes_read != size {
            if self.debug {
                println!(
                    "Warning: vmhd box size mismatch. Expected {} bytes, read {} bytes",
                    size, bytes_read
                );
            }
            // Adjust position if necessary
            if bytes_read < size {
                self.reader.skip(size - bytes_read)?;
            }
        }

        Ok(BoxData::VideoMediaHeader(VideoMediaHeaderBox::new(
            version,
            flags,
            graphics_mode,
            op_color,
        )))
    }

    fn parse_smhd(&mut self, size: u64) -> Result<BoxData, MediaError> {
        let start_pos = self.reader.position();

        // Read version and flags
        let version = self.reader.read_u8()?;
        let flags = self.reader.read_u24()?;

        // Read balance (fixed point 8.8)
        let balance = Fixed16_16::new(self.reader.read_i16()? as i32);

        // Skip reserved
        self.reader.skip(2)?;

        // Verify we read exactly the right number of bytes
        let bytes_read = self.reader.position() - start_pos;
        if bytes_read != size {
            if self.debug {
                println!(
                    "Warning: smhd box size mismatch. Expected {} bytes, read {} bytes",
                    size, bytes_read
                );
            }
            // Adjust position if necessary
            if bytes_read < size {
                self.reader.skip(size - bytes_read)?;
            }
        }

        Ok(BoxData::SoundMediaHeader(SoundMediaHeaderBox::new(
            version, flags, balance,
        )))
    }

    fn parse_stbl(&mut self, _size: u64) -> Result<SampleTableBox, MediaError> {
        Ok(SampleTableBox::new())
    }

    fn parse_stsd(&mut self, size: u64) -> Result<BoxData, MediaError> {
        let start_pos = self.reader.position();

        // Read version and flags
        let version = self.reader.read_u8()?;
        let flags = self.reader.read_u24()?;

        // Read entry count
        let entry_count = self.reader.read_u32()?;
        let mut entries = Vec::with_capacity(entry_count as usize);

        // Read sample entries
        for _ in 0..entry_count {
            // Read entry size and type
            let entry_size = self.reader.read_u32()? as u64;
            let entry_type = self.reader.read_fixed_string(4)?;

            // Skip 6 bytes reserved
            self.reader.skip(6)?;

            // Read data reference index
            let data_reference_index = self.reader.read_u16()?;

            // Calculate remaining data size for this entry
            let header_size = 16; // size(4) + type(4) + reserved(6) + data_ref_idx(2)
            let data_size = entry_size - header_size;

            // Read format-specific data
            let mut data = vec![0u8; data_size as usize];
            self.reader.read_exact(&mut data)?;

            entries.push(SampleEntry::new(entry_type, data_reference_index, data));
        }

        // Verify we read exactly the right number of bytes
        let bytes_read = self.reader.position() - start_pos;
        if bytes_read != size {
            if self.debug {
                println!(
                    "Warning: stsd box size mismatch. Expected {} bytes, read {} bytes",
                    size, bytes_read
                );
            }
            // Adjust position if necessary
            if bytes_read < size {
                self.reader.skip(size - bytes_read)?;
            }
        }

        Ok(BoxData::SampleDescription(SampleDescriptionBox::new(
            version,
            flags,
            entry_count,
            entries,
        )))
    }

    fn parse_stts(&mut self, size: u64) -> Result<BoxData, MediaError> {
        let start_pos = self.reader.position();

        // Read version and flags
        let version = self.reader.read_u8()?;
        let flags = self.reader.read_u24()?;

        // Read entry count
        let entry_count = self.reader.read_u32()?;
        let mut entries = Vec::with_capacity(entry_count as usize);

        // Read entries
        for _ in 0..entry_count {
            let sample_count = self.reader.read_u32()?;
            let sample_delta = self.reader.read_u32()?;
            entries.push(TimeToSampleEntry::new(sample_count, sample_delta));
        }

        // Verify we read exactly the right number of bytes
        let bytes_read = self.reader.position() - start_pos;
        if bytes_read != size {
            if self.debug {
                println!(
                    "Warning: stts box size mismatch. Expected {} bytes, read {} bytes",
                    size, bytes_read
                );
            }
            // Adjust position if necessary
            if bytes_read < size {
                self.reader.skip(size - bytes_read)?;
            }
        }

        Ok(BoxData::TimeToSample(TimeToSampleBox::new(
            version, flags, entries,
        )))
    }

    fn parse_stsc(&mut self, size: u64) -> Result<BoxData, MediaError> {
        let start_pos = self.reader.position();

        // Read version and flags
        let version = self.reader.read_u8()?;
        let flags = self.reader.read_u24()?;

        // Read entry count
        let entry_count = self.reader.read_u32()?;
        let mut entries = Vec::with_capacity(entry_count as usize);

        // Read entries
        for _ in 0..entry_count {
            let first_chunk = self.reader.read_u32()?;
            let samples_per_chunk = self.reader.read_u32()?;
            let sample_description_index = self.reader.read_u32()?;
            entries.push(SampleToChunkEntry::new(
                first_chunk,
                samples_per_chunk,
                sample_description_index,
            ));
        }

        // Verify read size
        let bytes_read = self.reader.position() - start_pos;
        if bytes_read != size {
            if self.debug {
                println!(
                    "Warning: stsc box size mismatch. Expected {} bytes, read {} bytes",
                    size, bytes_read
                );
            }
            if bytes_read < size {
                self.reader.skip(size - bytes_read)?;
            }
        }

        Ok(BoxData::SampleToChunk(SampleToChunkBox::new(
            version, flags, entries,
        )))
    }

    fn parse_stsz(&mut self, size: u64) -> Result<BoxData, MediaError> {
        let start_pos = self.reader.position();

        // Read version and flags
        let version = self.reader.read_u8()?;
        let flags = self.reader.read_u24()?;

        // Read sample size and count
        let sample_size = self.reader.read_u32()?;
        let sample_count = self.reader.read_u32()?;

        // Read entry sizes if sample_size is 0
        let entry_sizes = if sample_size == 0 {
            let mut sizes = Vec::with_capacity(sample_count as usize);
            for _ in 0..sample_count {
                sizes.push(self.reader.read_u32()?);
            }
            sizes
        } else {
            Vec::new()
        };

        // Verify read size
        let bytes_read = self.reader.position() - start_pos;
        if bytes_read != size {
            if self.debug {
                println!(
                    "Warning: stsz box size mismatch. Expected {} bytes, read {} bytes",
                    size, bytes_read
                );
            }
            if bytes_read < size {
                self.reader.skip(size - bytes_read)?;
            }
        }

        Ok(BoxData::SampleSize(SampleSizeBox::new(
            version,
            flags,
            sample_size,
            sample_count,
            entry_sizes,
        )))
    }

    fn parse_stco(&mut self, size: u64) -> Result<BoxData, MediaError> {
        let start_pos = self.reader.position();

        // Read version and flags
        let version = self.reader.read_u8()?;
        let flags = self.reader.read_u24()?;

        // Read entry count
        let entry_count = self.reader.read_u32()?;
        let mut offsets = Vec::with_capacity(entry_count as usize);

        // Read offsets
        for _ in 0..entry_count {
            offsets.push(self.reader.read_u32()? as u64);
        }

        // Verify read size
        let bytes_read = self.reader.position() - start_pos;
        if bytes_read != size {
            if self.debug {
                println!(
                    "Warning: stco box size mismatch. Expected {} bytes, read {} bytes",
                    size, bytes_read
                );
            }
            if bytes_read < size {
                self.reader.skip(size - bytes_read)?;
            }
        }

        Ok(BoxData::ChunkOffset(ChunkOffsetBox::new(
            version, flags, offsets,
        )))
    }

    fn parse_dinf(&mut self, _size: u64) -> Result<DataInformationBox, MediaError> {
        Ok(DataInformationBox::new())
    }

    fn parse_dref(&mut self, size: u64) -> Result<DataReferenceBox, MediaError> {
        let start_pos = self.reader.position();

        // Read version and flags
        let version = self.reader.read_u8()?;
        let flags = self.reader.read_u24()?;

        // Read entry count
        let entry_count = self.reader.read_u32()?;
        let mut entries = Vec::with_capacity(entry_count as usize);

        // Read entries
        for _ in 0..entry_count {
            let entry_start = self.reader.position();
            let entry_size = self.reader.read_u32()? as u64;
            let entry_type = self.reader.read_fixed_string(4)?;
            let entry_version = self.reader.read_u8()?;
            let entry_flags = self.reader.read_u24()?;

            let entry = match entry_type.as_str() {
                "url " => {
                    let location = if entry_flags & 0x000001 == 0 {
                        // 如果没有设置 self_contained 标志，读取 URL
                        let remaining = entry_size - (self.reader.position() - entry_start);
                        self.reader.read_string(remaining)?
                    } else {
                        String::new()
                    };
                    DataEntryBox::Url {
                        version: entry_version,
                        flags: entry_flags,
                        location,
                    }
                }
                "urn " => {
                    let name = self.reader.read_string_until_null()?;
                    let location = self.reader.read_string_until_null()?;
                    DataEntryBox::Urn {
                        version: entry_version,
                        flags: entry_flags,
                        name,
                        location,
                    }
                }
                _ => {
                    // Skip unknown entry type
                    let remaining = entry_size - (self.reader.position() - entry_start);
                    self.reader.skip(remaining)?;
                    continue;
                }
            };

            entries.push(entry);
        }

        // Verify read size
        let bytes_read = self.reader.position() - start_pos;
        if bytes_read != size {
            if self.debug {
                println!(
                    "Warning: dref box size mismatch. Expected {} bytes, read {} bytes",
                    size, bytes_read
                );
            }
            if bytes_read < size {
                self.reader.skip(size - bytes_read)?;
            }
        }

        Ok(DataReferenceBox::new(version, flags, entries))
    }
}
