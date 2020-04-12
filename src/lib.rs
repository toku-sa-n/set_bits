/// Set `num_of_bits` bits from the `start_bit`th bit of address `start_byte`.
///
/// `num_of_bits` may be more than 7.
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
/// // For automatic cleanup.
/// // See the example of into_raw function.
/// // https://doc.rust-lang.org/std/boxed/struct.Box.html
/// let byte = unsafe { Box::from_raw(ptr) };
/// ```
pub fn set(start_byte: usize, start_bit: usize, num_of_bits: usize) -> () {
    unsafe {
        *(start_byte as *mut u8) |=
            ((1 << (start_bit + num_of_bits)) as u16 - (1 << start_bit) as u16) as u8;
    };
}

/// Clear `num_of_bits` bits from the `start_bit`th bit of address `start_byte`.
///
/// `num_of_bits` may be more than 7.
///
/// # Examples
///
/// Clear 3 bits from the 2nd bit of specified address.
///
/// ```
/// let byte: Box<u32> = Box::new(0);
/// let ptr = Box::into_raw(byte);
///
/// set_bits::set(ptr as usize, 0, 8);
/// set_bits::clear(ptr as usize, 2, 3);
/// unsafe {
///     assert_eq!(*ptr, 0b11100011);
/// }
///
/// let byte = unsafe { Box::from_raw(ptr) };
/// ```
pub fn clear(start_byte: usize, start_bit: usize, num_of_bits: usize) -> () {
    unsafe {
        *(start_byte as *mut u8) =
            !((1 << (start_bit + num_of_bits)) as u16 - (1 << start_bit) as u16) as u8;
    }
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

        // For automatic cleanup.
        let _byte = unsafe { Box::from_raw(ptr) };
    }

    fn test_clear(start_bit: usize, num_of_bits: usize, correct_value: u32) -> () {
        let byte: Box<u32> = Box::new(0xFF);
        let ptr = Box::into_raw(byte);

        clear(ptr as usize, start_bit, num_of_bits);
        unsafe {
            assert_eq!(*ptr, correct_value);
        }

        let _byte = unsafe { Box::from_raw(ptr) };
    }

    #[test]
    fn set_within_a_byte() {
        test_set(3, 2, 0b11000);
        test_set(1, 4, 0b11110);
        test_set(0, 8, 0b11111111);
        test_set(0, 0, 0);
    }

    #[test]
    fn clear_within_a_byte() -> () {
        test_clear(2, 3, 0b11100011);
        test_clear(1, 4, 0b11100001);
        test_clear(0, 8, 0);
        test_clear(0, 0, 0b11111111);
    }

    #[test]
    fn set_and_clear_within_a_byte() -> () {
        let byte: Box<u32> = Box::new(0);
        let ptr = Box::into_raw(byte);

        set(ptr as usize, 5, 3);
        set(ptr as usize, 0, 3);
        set(ptr as usize, 3, 2);
        unsafe {
            assert_eq!(*ptr, 0b11111111);
        }

        clear(ptr as usize, 5, 3);
        clear(ptr as usize, 0, 3);
        clear(ptr as usize, 3, 2);
        unsafe {
            assert_eq!(*ptr, 0);
        }

        let _byte = unsafe { Box::from_raw(ptr) };
    }
}
