struct BitString {
    start_address: usize,
    start_bit: usize,
    num_of_bits: usize,
}

impl BitString {
    fn new(start_address: usize, start_bit: usize, num_of_bits: usize) -> Self {
        Self {
            start_address,
            start_bit,
            num_of_bits,
        }
    }
}
