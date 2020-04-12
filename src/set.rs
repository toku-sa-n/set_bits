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

    #[test]
    fn set_head_byte_1() -> () {
        let byte: Box<u32> = Box::new(0);
        let ptr = Box::into_raw(byte);

        set_head_byte(ptr as usize, 3);
        unsafe {
            assert_eq!(*ptr, 0b11111000);
        }

        let _byte = unsafe { Box::from_raw(ptr) };
    }
}
