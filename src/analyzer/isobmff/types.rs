use std::fmt;

/// Box type identifier (4 characters)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct BoxType([u8; 4]);

impl BoxType {
    pub fn new(value: [u8; 4]) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).unwrap_or("????")
    }
}

impl fmt::Display for BoxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Fixed point number types
#[derive(Debug, Clone, Copy)]
pub struct Fixed16_16(i32);

impl Fixed16_16 {
    pub fn new(value: i32) -> Self {
        Self(value)
    }

    pub fn as_f32(&self) -> f32 {
        (self.0 as f32) / 65536.0
    }
}

/// Matrix for video transformation
#[derive(Debug, Clone)]
pub struct Matrix {
    pub values: [i32; 9],
}

impl Matrix {
    pub fn new(values: [i32; 9]) -> Self {
        Self { values }
    }
}

/// Creation and modification times
#[derive(Debug, Clone, Copy)]
pub struct Mp4DateTime(u64);

impl Mp4DateTime {
    pub fn new(seconds_since_1904: u64) -> Self {
        Self(seconds_since_1904)
    }

    #[allow(dead_code)]
    pub fn as_secs(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for Mp4DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Convert from seconds since 1904 to Unix timestamp
        // (subtract seconds between 1904 and 1970)
        let unix_time = self.0.saturating_sub(2082844800);
        write!(f, "{}", unix_time)
    }
}
