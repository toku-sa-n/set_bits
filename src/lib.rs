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
    #[test]
    fn set_bits_within_a_byte() {
        assert_eq!(2 + 2, 4);
    }
}
