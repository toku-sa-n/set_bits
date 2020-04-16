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
        ((1 << (self.start_bit % 8 + self.num_of_bits)) as u16 - (1 << (self.start_bit % 8)) as u16)
            as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bits_at_byte_within_a_byte(start_bit: usize, num_of_bits: usize, correct_value: u8) -> () {
        let heap: Box<u32> = Box::new(0);
        let ptr = Box::into_raw(heap);

        let bit_string = BitString::new(ptr as usize, start_bit, num_of_bits);
        assert_eq!(bit_string.bits_at_byte(0), correct_value);

        unsafe {
            let _heap: Box<u32> = Box::from_raw(ptr);
        }
    }

    #[test]
    fn bits_at_byte_within_a_byte_1() -> () {
        bits_at_byte_within_a_byte(3, 2, 0b00011000);
    }

    #[test]
    fn bits_at_byte_within_a_byte_2() -> () {
        bits_at_byte_within_a_byte(1, 4, 0b00011110);
    }

    #[test]
    fn bits_at_byte_set_all_bits_within_a_byte() -> () {
        bits_at_byte_within_a_byte(0, 8, 0xFF);
    }

    #[test]
    fn bits_at_byte_set_no_bits_within_a_byte() -> () {
        bits_at_byte_within_a_byte(0, 0, 0);
    }
}
