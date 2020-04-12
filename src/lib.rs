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
fn set_bits(start_byte: usize, start_bit: usize, num_of_bits: usize) -> () {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_bits_within_a_byte() {
        let byte: Box<u32> = Box::new(0);
        let ptr = Box::into_raw(byte);

        set_bits(ptr as usize, 3, 2);
        unsafe {
            assert_eq!(*ptr, 0b11000);
        }

        let byte = unsafe { Box::from_raw(ptr) };
    }
}
