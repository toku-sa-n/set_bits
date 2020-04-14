/// Set `num_of_bits` bits from the `start_bit`th bit of address `address`.
///
/// `num_of_bits` may be more than the number of bits a byte has.
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
pub fn set(address: usize, start_bit: usize, num_of_bits: usize) -> () {
    unsafe {
        *((address + start_bit / 8) as *mut u8) |=
            ((1 << (start_bit % 8 + num_of_bits)) as u16 - (1 << (start_bit % 8)) as u16) as u8;
    };
}

/// Clear `num_of_bits` bits from the `start_bit`th bit of address `address`.
///
/// `num_of_bits` may be more than the number of bits a byte has.
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
pub fn clear(address: usize, start_bit: usize, num_of_bits: usize) -> () {
    unsafe {
        *(address as *mut u8) &=
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
        let byte: Box<u32> = Box::new(!0);
        let ptr = Box::into_raw(byte);

        clear(ptr as usize, start_bit, num_of_bits);
        unsafe {
            assert_eq!(*ptr, correct_value);
        }

        let _byte = unsafe { Box::from_raw(ptr) };
    }

    #[test]
    fn set_within_a_byte_1() -> () {
        test_set(3, 2, 0b11000);
    }

    #[test]
    fn set_within_a_byte_2() -> () {
        test_set(1, 4, 0b11110);
    }

    #[test]
    fn set_all_bits_of_a_byte_1() -> () {
        test_set(0, 8, 0b11111111);
    }

    #[test]
    fn set_all_bits_of_a_byte_2() -> () {
        test_set(8, 8, 0xff00);
    }

    #[test]
    fn set_no_bits_1() -> () {
        test_set(0, 0, 0);
    }

    #[test]
    fn set_no_bits_2() -> () {
        test_set(8, 0, 0);
    }

    #[test]
    fn set_start_bit_more_than_7_1() -> () {
        test_set(10, 3, 0b1110000000000);
    }

    #[test]
    fn set_start_bit_more_than_7_2() -> () {
        test_set(26, 5, 0b1111100000000000000000000000000);
    }

    #[test]
    fn clear_within_a_byte_1() -> () {
        test_clear(2, 3, 0b11111111_11111111_11111111_11100011);
    }

    #[test]
    fn clear_within_a_byte_2() -> () {
        test_clear(1, 4, 0b11111111_11111111_11111111_11100001);
    }

    #[test]
    fn clear_start_bit_more_than_7_1() -> () {
        test_clear(10, 3, 0b11111111_11111111_11100011_11111111);
    }

    #[test]
    fn clear_all_bits_within_a_byte() -> () {
        test_clear(0, 8, 0xffffff00);
    }

    #[test]
    fn clear_no_bits() -> () {
        test_clear(0, 0, !0);
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
