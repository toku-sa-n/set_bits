/// Set `num_of_bits` bits from the `start_bit`th bit of address `start_byte`.
///
/// `start_byte` may be more than 7.
///
/// # Examples
///
/// Set 3 bits from the 2nd bit of specified address.
///
/// ```
/// let byte: Box<u32> = Box::new(0);
/// let ptr = Box::into_raw(byte);
///
/// set_bits::set(ptr as usize, 2, 3);
/// unsafe {
///     assert_eq!(*ptr, 0b11100);
/// }
///
/// let byte = unsafe { Box::from_raw(ptr) };
/// ```
pub fn set(start_byte: usize, start_bit: usize, num_of_bits: usize) -> () {
    unsafe {
        *(start_byte as *mut u8) = (1 << (start_bit + num_of_bits)) - (1 << start_bit);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_set(start_bit: usize, num_of_bits: usize, correct_value: u32) -> () {
        let byte: Box<u32> = Box::new(0);
        let ptr = Box::into_raw(byte);

        set(ptr as usize, start_bit, num_of_bits);
        unsafe {
            assert_eq!(*ptr, correct_value);
        }

        let byte = unsafe { Box::from_raw(ptr) };
    }

    #[test]
    fn set_within_a_byte() {
        test_set(3, 2, 0b11000);
        test_set(1, 4, 0b11110);
    }
}
