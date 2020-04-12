/// Set `num_of_bits` bits from the `start_bit`th bit of address `start_byte`.
///
/// `start_byte` may be more than 7.
///
/// # Examples
///
/// Set 3 bits from the 2nd bit of specified address.
///
/// ```
/// ```
fn set_bits(start_byte: usize, start_bit: usize, num_of_bits: usize) -> () {
    unsafe {
        *(start_byte as *mut u8) = 0b11000;
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_set_bits(start_bit: usize, num_of_bits: usize, correct_value: u32) -> () {
        let byte: Box<u32> = Box::new(0);
        let ptr = Box::into_raw(byte);

        set_bits(ptr as usize, start_bit, num_of_bits);
        unsafe {
            assert_eq!(*ptr, correct_value);
        }

        let byte = unsafe { Box::from_raw(ptr) };
    }

    #[test]
    fn set_bits_within_a_byte() {
        test_set_bits(3, 2, 0b11000);
        test_set_bits(4, 1, 0b11110);
    }
}
