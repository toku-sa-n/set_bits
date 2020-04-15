enum Operation {
    Set,
    Clear,
}

pub fn within_a_byte(address: usize, start_bit: usize, num_of_bits: usize) -> () {
    unsafe {
        *((address + start_bit / 8) as *mut u8) |=
            ((1 << (start_bit % 8 + num_of_bits)) as u16 - (1 << (start_bit % 8)) as u16) as u8;
    };
}

fn set_head_byte(address: usize, start_bit: usize) -> () {
    unsafe {
        *((address + start_bit / 8) as *mut u8) |=
            ((1 << 8) as u16 - (1 << (start_bit % 8)) as u16) as u8;
    }
}

fn set_body_byte(address: usize, start_bit: usize, num_of_bits: usize) -> () {
    let first_byte: usize = address + start_bit / 8;
    let last_byte: usize = address + (start_bit + num_of_bits - 1) / 8;

    // Head and tail must exist. However, body may not.
    for ptr in first_byte + 1..last_byte {
        unsafe {
            *(ptr as *mut u8) = 0xFF;
        }
    }
}

fn set_tail_byte(address: usize, start_bit: usize, num_of_bits: usize) -> () {
    let mut bit_mask: u8 = (1 << (start_bit + num_of_bits) % 8) - 1;
    if bit_mask == 0 {
        bit_mask = 0xFF;
    }

    unsafe {
        *((address + (start_bit + num_of_bits - 1) / 8) as *mut u8) |= bit_mask;
    }
}

pub fn straddling_byte_boundaries(address: usize, start_bit: usize, num_of_bits: usize) -> () {
    set_head_byte(address, start_bit);
    set_body_byte(address, start_bit, num_of_bits);
    set_tail_byte(address, start_bit, num_of_bits);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_general<T: Fn(usize, usize, usize) -> ()>(
        start_bit: usize,
        num_of_bits: usize,
        correct_value: u32,
        func: T,
    ) -> () {
        let byte: Box<u32> = Box::new(0);
        let ptr = Box::into_raw(byte);

        func(ptr as usize, start_bit, num_of_bits);
        unsafe {
            assert_eq!(*ptr, correct_value);
        }

        let _byte = unsafe { Box::from_raw(ptr) };
    }

    fn test_head(start_bit: usize, correct_value: u32) -> () {
        let func = |address, start_bit, _| {
            set_head_byte(address, start_bit);
        };

        test_general(start_bit, 0, correct_value, func);
    }

    fn test_tail(start_bit: usize, num_of_bits: usize, correct_value: u32) -> () {
        let func = |address, start_bit, num_of_bits| {
            set_tail_byte(address, start_bit, num_of_bits);
        };

        test_general(start_bit, num_of_bits, correct_value, func);
    }

    fn test_body(start_bit: usize, num_of_bits: usize, correct_value: u32) -> () {
        let func = |address, start_bit, num_of_bits| {
            set_body_byte(address, start_bit, num_of_bits);
        };

        test_general(start_bit, num_of_bits, correct_value, func);
    }

    #[test]
    fn set_head_byte_1() -> () {
        test_head(3, 0b11111000);
    }

    #[test]
    fn set_head_byte_2() -> () {
        test_head(5, 0b11100000);
    }

    #[test]
    fn set_head_start_bit_more_than_7() -> () {
        test_head(10, 0b11111100_00000000);
    }

    #[test]
    fn set_head_byte_fully() -> () {
        test_head(0, 0b11111111);
    }

    #[test]
    fn set_head_byte_fully_start_bit_more_than_7() -> () {
        test_head(8, 0b11111111_00000000);
    }

    #[test]
    fn set_tail_byte_1() -> () {
        test_tail(5, 16, 0b00011111_00000000_00000000);
    }

    #[test]
    fn set_tail_byte_2() -> () {
        test_tail(2, 25, 0b00000111_00000000_00000000_00000000);
    }

    #[test]
    fn set_tail_start_bit_more_than_7() -> () {
        test_tail(13, 16, 0b00011111_00000000_00000000_00000000);
    }

    #[test]
    fn set_tail_byte_fully() -> () {
        test_tail(0, 16, 0b11111111_00000000);
    }

    #[test]
    fn set_tail_byte_fully_start_bit_more_than_7() -> () {
        test_tail(8, 16, 0xff0000);
    }

    #[test]
    fn set_body_byte_1() -> () {
        test_body(2, 25, 0xFFFF00);
    }

    #[test]
    fn set_body_byte_2() -> () {
        test_body(5, 15, 0xFF00);
    }

    #[test]
    fn set_body_byte_fully() -> () {
        test_body(0, 24, 0xFF00);
    }

    #[test]
    fn set_body_byte_start_bit_more_than_7() -> () {
        test_body(13, 15, 0xFF0000);
    }

    #[test]
    fn set_body_byte_fully_start_bit_more_than_7() -> () {
        test_body(8, 24, 0xFF0000);
    }

    #[test]
    fn set_body_sets_no_bit() -> () {
        test_body(3, 10, 0);
    }
}
