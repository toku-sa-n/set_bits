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
        self.len_in_bit() != 0 && self.head_byte_index() != self.tail_byte_index()
    }

    fn head_byte_index(&self) -> usize {
        self.start_bit / 8
    }

    fn tail_byte_index(&self) -> usize {
        (self.start_bit + self.num_of_bits - 1) / 8
    }

    fn within_a_byte(&self, idx: usize) -> u8 {
        if self.num_of_bits == 0 || idx < self.head_byte_index() || idx > self.tail_byte_index() {
            0
        } else {
            ((1 << (self.start_bit % 8 + self.num_of_bits)) as u16
                - (1 << (self.start_bit % 8)) as u16) as u8
        }
    }

    fn more_than_a_byte(&self, idx: usize) -> u8 {
        0b11111000
    }

    fn get_head_byte(&self) -> u8 {
        ((1 << if self.does_straddle_byte_boundary() {
            8
        } else {
            self.start_bit % 8 + self.num_of_bits
        }) as u16
            - (1 << (self.start_bit % 8)) as u16) as u8
    }

    fn get_tail_byte(&self) -> u8 {
        let bits_in_byte: u8 = (1 << (self.start_bit + self.num_of_bits) % 8) - 1;

        if bits_in_byte == 0 {
            0xFF
        } else {
            bits_in_byte
        }
    }

    fn bits_at_byte(&self, idx: usize) -> u8 {
        if idx == self.head_byte_index() {
            self.get_head_byte()
        } else if idx == self.tail_byte_index() {
            self.get_tail_byte()
        } else if self.does_straddle_byte_boundary() {
            self.more_than_a_byte(idx)
        } else {
            self.within_a_byte(idx)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod bits_at_byte {
        use super::*;

        fn common(start_bit: usize, num_of_bits: usize, idx: usize, correct_value: u8) -> () {
            let heap: Box<u32> = Box::new(0);
            let ptr = Box::into_raw(heap);

            let bit_string = BitString::new(ptr as usize, start_bit, num_of_bits);
            assert_eq!(bit_string.bits_at_byte(idx), correct_value);

            unsafe {
                let _heap: Box<u32> = Box::from_raw(ptr);
            }
        }

        mod within_a_byte {
            use super::*;

            #[test]
            fn common_1() -> () {
                common(3, 2, 0, 0b00011000);
            }

            #[test]
            fn common_2() -> () {
                common(1, 4, 0, 0b00011110);
            }

            #[test]
            fn set_all_bits() -> () {
                common(0, 8, 0, 0xFF);
            }

            #[test]
            fn set_no_bits() -> () {
                common(0, 0, 0, 0);
            }

            #[test]
            fn start_bit_more_than_7() -> () {
                common(11, 2, 1, 0b00011000);
            }

            #[test]
            fn index_over_range() -> () {
                common(0, 8, 1, 0);
            }

            #[test]
            fn index_below_range() -> () {
                common(8, 8, 0, 0);
            }
        }
        mod more_than_a_byte {
            use super::*;

            #[test]
            fn head_byte_1() -> () {
                common(3, 20, 0, 0b11111000);
            }

            #[test]
            fn head_byte_2() -> () {
                common(5, 9, 0, 0b11100000);
            }

            #[test]
            fn head_all_bits() -> () {
                common(0, 32, 0, 0xFF);
            }

            #[test]
            fn tail_byte_1() -> () {
                common(3, 20, 2, 0b01111111);
            }

            #[test]
            fn tail_byte_2() -> () {
                common(5, 9, 1, 0b00111111);
            }
        }
    }
}
