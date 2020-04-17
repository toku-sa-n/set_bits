#![feature(trait_alias)]

mod bit_operation;
mod bit_string;

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
    bit_operation::bit_operation(
        bit_string::BitString::new(address, start_bit, num_of_bits),
        bit_operation::Operation::Set,
    );
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
    bit_operation::bit_operation(
        bit_string::BitString::new(address, start_bit, num_of_bits),
        bit_operation::Operation::Clear,
    );
}
