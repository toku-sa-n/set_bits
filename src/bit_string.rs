pub struct BitString {
    pub start_address: usize,
    pub start_bit: usize,
    pub num_of_bits: usize,
}

impl BitString {
    pub fn new(start_address: usize, start_bit: usize, num_of_bits: usize) -> Self {
        Self {
            start_address,
            start_bit,
            num_of_bits,
        }
    }

    pub fn does_straddle_byte_boundary(&self) -> bool {
        self.start_bit / 8 != (self.start_bit + self.num_of_bits - 1) / 8
    }
}
