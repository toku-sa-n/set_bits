pub fn within_a_byte(address: usize, start_bit: usize, num_of_bits: usize) -> () {
    unsafe {
        *(address as *mut u8) |=
            ((1 << (start_bit + num_of_bits)) as u16 - (1 << start_bit) as u16) as u8;
    };
}

fn set_head_byte(address: usize, start_bit: usize) -> () {
    unsafe {
        *(address as *mut u8) |= 0b11111000;
    }
}

pub fn straddling_byte_boundaries(address: usize, start_bit: usize, num_of_bits: usize) -> () {
    unsafe {
        *(address as *mut u16) |= 0b1111111111000;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_head(start_bit: usize, correct_value: u32) -> () {
        let byte: Box<u32> = Box::new(0);
        let ptr = Box::into_raw(byte);

        set_head_byte(ptr as usize, start_bit);
        unsafe {
            assert_eq!(*ptr, correct_value);
        }

        let _byte = unsafe { Box::from_raw(ptr) };
    }

    #[test]
    fn set_head_byte_1() -> () {
        test_head(3, 0b11111000);
    }

    #[test]
    fn set_head_byte_2() -> () {
        test_head(5, 0b11100000);
    }
}
