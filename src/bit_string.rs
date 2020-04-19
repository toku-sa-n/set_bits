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

    pub fn num_of_sections(&self) -> usize {
        if self.num_of_bits == 0 {
            0
        } else {
            (self.start_bit + self.num_of_bits - 1) / NUM_OF_BITS + 1
        }
    }

    fn does_straddle_section_boundary(&self) -> bool {
        self.len_in_bit() != 0 && self.head_section_index() != self.tail_section_index()
    }

    fn head_section_index(&self) -> usize {
        self.start_bit / NUM_OF_BITS
    }

    fn tail_section_index(&self) -> usize {
        (self.start_bit + self.num_of_bits - 1) / NUM_OF_BITS
    }

    fn get_head_section(&self) -> SrcVal {
        (if self.does_straddle_section_boundary()
            || self.start_bit % NUM_OF_BITS + self.num_of_bits == NUM_OF_BITS
        {
            !0
        } else {
            (1 << (self.start_bit % NUM_OF_BITS + self.num_of_bits)) - 1
        }) - ((1 << (self.start_bit % NUM_OF_BITS)) - 1)
    }

    fn get_tail_section(&self) -> SrcVal {
        let bits_in_section: SrcVal = (1 << (self.start_bit + self.num_of_bits) % NUM_OF_BITS) - 1;

        if bits_in_section == 0 {
            !0
        } else {
            bits_in_section
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
        if idx == self.head_section_index() {
            self.get_head_section()
        } else if idx == self.tail_section_index() {
            self.get_tail_section()
        } else if idx > self.head_section_index() && idx < self.tail_section_index() {
            !0
        } else {
            0
        }
    }

    pub fn get_address_of_section(&self, idx: usize) -> usize {
        self.start_address + idx * NUM_OF_BITS / 8
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
            fn start_bit_more_than_127() -> () {
                common(131, 2, 1, 0b00011000);
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
            fn head_section_1() -> () {
                common(3, 128, 0, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF8);
            }

            #[test]
            fn head_section_2() -> () {
                common(5, 128, 0, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE0);
            }

            #[test]
            fn head_section_3() -> () {
                common(41, 119, 0, 0xFFFFFFFFFFFFFFFFFFFFFE0000000000);
            }

            #[test]
            fn head_all_bits() -> () {
                common(0, NUM_OF_BITS * 4, 0, !0);
            }

            #[test]
            fn tail_section_1() -> () {
                common(3, 260, 2, 0b1111111);
            }

            #[test]
            fn tail_section_2() -> () {
                common(5, 128, 1, 0b11111);
            }

            #[test]
            fn tail_section_3() -> () {
                common(41, 119, 1, 0xFFFFFFFF);
            }

            #[test]
            fn tail_all_bits() -> () {
                common(0, NUM_OF_BITS * 4, 3, !0);
            }

            #[test]
            fn body_section_1() -> () {
                common(3, 256, 1, !0);
            }

            #[test]
            fn body_section_2() -> () {
                common(0, NUM_OF_BITS * 4, 1, !0);
                common(0, NUM_OF_BITS * 4, 2, !0);
            }

            #[test]
            fn index_over_range() -> () {
                common(0, NUM_OF_BITS * 3, 3, 0);
            }

            #[test]
            fn index_below_range() -> () {
                common(128, 256, 0, 0);
            }
        }
    }
}
