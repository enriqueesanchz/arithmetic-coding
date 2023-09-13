use crate::model::Model;

pub struct Range {
    high: u64,
    low: u64,
    half: u64,
    one_quarter: u64,
    three_quarter: u64
}

impl Range {
    pub fn new(precision_bits: u8) -> Self {
        assert!(precision_bits < 64);

        let high: u64 = 1 << precision_bits;
        let half: u64 = high / 2;
        let one_quarter: u64 = high / 4;
        let three_quarter: u64 = high / 4 * 3;

        Self {
            high,
            low: 0,
            half,
            one_quarter,
            three_quarter
        }
    }

    pub fn scale_upper_half(&mut self) {
        self.low = (self.low - self.half) << 1;
        self.high = (self.high - self.half) << 1;
    }

    pub fn scale_bottom_half(&mut self) {
        self.low = self.low << 1;
        self.high = self.high << 1;
    }

    pub fn scale_middle_half(&mut self) {
        self.low = (self.low - self.one_quarter) << 1;
        self.high = (self.high - self.one_quarter) << 1;
    }

    pub fn in_upper_half(&self) -> bool {
        self.low > self.half
    }

    pub fn in_bottom_half(&self) -> bool {
        self.high < self.half
    }

    pub fn in_middle_half(&self) -> bool {
        self.low > self.one_quarter && self.high < self.three_quarter
    }

    pub const fn in_bottom_quarter(&self) -> bool {
        self.low <= self.one_quarter
    }

    pub fn calculate_range(&self, symbol: u32, model: &Model) -> (u64, u64) {
        let current_width = self.high - self.low;
        let (low, high) = model.prob(symbol);

        (
            (self.low + (current_width as f64 * low) as u64),
            (self.low + (current_width as f64 * high) as u64)
        )
    }

    pub fn update_range(&mut self, (low, high): (u64, u64)) {
        self.low = low;
        self.high = high;
    }

    pub fn half(&self) -> u64 {
        self.half
    }
    
    pub fn quarter(&self) -> u64 {
        self.one_quarter
    }
}

