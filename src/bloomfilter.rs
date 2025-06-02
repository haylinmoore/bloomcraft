const BIT_WIDTH: u32 = 64;

#[derive(Debug, Clone)]
pub struct BloomFilter {
    bits: u64,
}

impl BloomFilter {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn add_char(&mut self, c: char) {
        let hash = Self::hash_char(c);
        self.bits = self.bits | (1u64 << hash);
    }

    fn hash_char(c: char) -> u32 {
        let c_lower = c.to_lowercase().next().unwrap_or(c);
        (c_lower as u32) % BIT_WIDTH
    }

    pub fn from_string(s: &str) -> Self {
        let mut filter = Self::new();
        for c in s.chars() {
            filter.add_char(c);
        }
        filter
    }

    pub fn contains(&self, other: &Self) -> bool {
        (self.bits & other.bits) == other.bits
    }
    
    pub fn is_all_ones(&self) -> bool {
        self.bits == u64::MAX
    }
}