use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

pub struct Reader {
    file: File,
    position: u64,
}

impl Reader {
    pub fn new(file: File) -> Self {
        Self { file, position: 0 }
    }

    pub fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.file.read_exact(buf)?;
        self.position += buf.len() as u64;
        Ok(())
    }

    pub fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    pub fn read_u16(&mut self) -> io::Result<u16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    pub fn read_u32(&mut self) -> io::Result<u32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    pub fn read_u64(&mut self) -> io::Result<u64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }

    pub fn read_fixed_string(&mut self, len: usize) -> io::Result<String> {
        let mut buf = vec![0; len];
        self.read_exact(&mut buf)?;
        Ok(String::from_utf8_lossy(&buf).to_string())
    }

    pub fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.position = self.file.seek(pos)?;
        Ok(self.position)
    }

    pub fn position(&self) -> u64 {
        self.position
    }

    pub fn read_u24(&mut self) -> io::Result<u32> {
        let mut buf = [0u8; 3];
        self.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes([0, buf[0], buf[1], buf[2]]))
    }

    pub fn read_i32(&mut self) -> io::Result<i32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }

    pub fn skip(&mut self, count: u64) -> io::Result<()> {
        self.position += count;
        self.file.seek(SeekFrom::Current(count as i64))?;
        Ok(())
    }

    pub fn read_i16(&mut self) -> io::Result<i16> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(i16::from_be_bytes(buf))
    }

    #[allow(dead_code)]
    pub fn get_file_size(&mut self) -> io::Result<u64> {
        let current_pos = self.position();
        let size = self.file.seek(SeekFrom::End(0))?;
        self.seek(SeekFrom::Start(current_pos))?;
        Ok(size)
    }

    pub fn read_i64(&mut self) -> io::Result<i64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(i64::from_be_bytes(buf))
    }

    pub fn read_string(&mut self, len: u64) -> Result<String, std::io::Error> {
        let mut buf = vec![0u8; len as usize];
        self.read_exact(&mut buf)?;

        // 移除末尾的空字节
        while let Some(&0) = buf.last() {
            buf.pop();
        }

        String::from_utf8(buf).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    pub fn read_string_until_null(&mut self) -> Result<String, std::io::Error> {
        let mut bytes = Vec::new();
        loop {
            let byte = self.read_u8()?;
            if byte == 0 {
                break;
            }
            bytes.push(byte);
        }

        String::from_utf8(bytes)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}
