pub fn within_a_byte(address: usize, start_bit: usize, num_of_bits: usize) -> () {
    unsafe {
        *(address as *mut u8) |=
            ((1 << (start_bit + num_of_bits)) as u16 - (1 << start_bit) as u16) as u8;
    };
}

pub fn straddling_byte_boundaries(address: usize, start_bit: usize, num_of_bits: usize) -> () {
    unsafe {
        *(address as *mut u16) |= 0b1111111111000;
    }
}
