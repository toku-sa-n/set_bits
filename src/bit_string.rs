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

    pub fn len_in_bit(&self) -> usize {
        self.num_of_bits
    }

    pub fn does_straddle_byte_boundary(&self) -> bool {
        self.start_bit / 8 != (self.start_bit + self.num_of_bits - 1) / 8
    }

    fn bits_at_byte(&self, idx: usize) -> u8 {
        0b00011000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bits_at_byte_within_a_byte_1() -> () {
        let heap: Box<u32> = Box::new(0);
        let ptr: *mut u32 = Box::into_raw(heap);

        let bit_string: BitString = BitString::new(ptr as usize, 3, 2);
        assert_eq!(bit_string.bits_at_byte(0), 0b00011000);

        unsafe {
            let _heap: Box<u32> = Box::from_raw(ptr);
        }
    }
}
