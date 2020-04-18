pub struct BitString {
    pub start_address: usize,
    pub start_bit: usize,
    pub num_of_bits: usize,
}

use crate::bit_operation::SrcVal;
use crate::bit_operation::NUM_OF_BITS;

impl BitString {
    pub fn new(start_address: usize, start_bit: usize, num_of_bits: usize) -> Self {
        Self {
            start_address,
            start_bit,
            num_of_bits,
        }
    }

    fn len_in_bit(&self) -> usize {
        self.num_of_bits
    }

    pub fn len_in_byte(&self) -> usize {
        if self.num_of_bits == 0 {
            0
        } else {
            (self.start_bit + self.num_of_bits - 1) / NUM_OF_BITS + 1
        }
    }

    fn does_straddle_byte_boundary(&self) -> bool {
        self.len_in_bit() != 0 && self.head_byte_index() != self.tail_byte_index()
    }

    fn head_byte_index(&self) -> usize {
        self.start_bit / NUM_OF_BITS
    }

    fn tail_byte_index(&self) -> usize {
        (self.start_bit + self.num_of_bits - 1) / NUM_OF_BITS
    }

    fn get_head_byte(&self) -> SrcVal {
        (if self.does_straddle_byte_boundary()
            || self.start_bit % NUM_OF_BITS + self.num_of_bits == NUM_OF_BITS
        {
            !0
        } else {
            (1 << (self.start_bit % NUM_OF_BITS + self.num_of_bits)) - 1
        }) - ((1 << (self.start_bit % NUM_OF_BITS)) - 1)
    }

    fn get_tail_byte(&self) -> SrcVal {
        let bits_in_byte: SrcVal = (1 << (self.start_bit + self.num_of_bits) % NUM_OF_BITS) - 1;

        if bits_in_byte == 0 {
            !0
        } else {
            bits_in_byte
        }
    }

    pub fn bits_at_section(&self, idx: usize) -> SrcVal {
        // sec: abbr of section
        // 00000000__00011111__11111111__11111111__11000000
        // ^         ^^^^^^^^  ^^^^^^^^^^^^^^^^^   ^^^^^^^^
        //  start     head           body           tail
        //  bit       sec            sec            sec
        //
        //  If bit string fits in a section , there's no body and tail sections.
        //  Only head section exists.
        //
        //  00000000__00111000__00000000
        //            ^^^^^^^^
        //              head
        //              sec
        if idx == self.head_byte_index() {
            self.get_head_byte()
        } else if idx == self.tail_byte_index() {
            self.get_tail_byte()
        } else if idx > self.head_byte_index() && idx < self.tail_byte_index() {
            !0
        } else {
            0
        }
    }

    pub fn get_address_of_byte(&self, idx: usize) -> usize {
        self.start_address + idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod bits_at_section {
        extern crate std;
        use super::*;
        use std::boxed::Box;

        fn common(start_bit: usize, num_of_bits: usize, idx: usize, correct_value: SrcVal) -> () {
            struct Heap {
                _heap: [SrcVal; 4],
            };

            let heap: Box<Heap> = Box::new(Heap { _heap: [0; 4] });
            let ptr = Box::into_raw(heap);

            let bit_string = BitString::new(ptr as usize, start_bit, num_of_bits);
            assert_eq!(bit_string.bits_at_section(idx), correct_value);

            unsafe {
                let _heap: Box<Heap> = Box::from_raw(ptr);
            }
        }

        mod within_a_section {
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
                common(0, NUM_OF_BITS, 0, !0);
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
                common(128, 128, 0, 0);
            }
        }
        mod more_than_a_section {
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

            #[test]
            fn tail_all_bits() -> () {
                common(0, 32, 3, 0xFF);
            }

            #[test]
            fn body_byte_1() -> () {
                common(3, 20, 1, 0xFF);
            }

            #[test]
            fn body_byte_2() -> () {
                common(0, 32, 1, 0xFF);
                common(0, 32, 2, 0xFF);
            }

            #[test]
            fn index_below_range() -> () {
                common(8, 16, 0, 0);
            }
        }
    }
}
