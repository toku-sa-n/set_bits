use set_bits;

fn test_general<T: Fn(usize, usize, usize) -> ()>(
    start_bit: usize,
    num_of_bits: usize,
    index: isize,
    correct_value: u128,
    func: T,
) -> () {
    let byte: Box<[u128; 4]> = Box::new([0; 4]);
    let ptr = Box::into_raw(byte);

    func(ptr as usize, start_bit, num_of_bits);
    unsafe {
        for i in 0..4 {
            println!(
                "{:X}: {:X}",
                (ptr as *const u128).offset(i) as usize,
                *((ptr as *const u128).offset(i))
            );
        }
        assert_eq!(*((ptr as *const u128).offset(index)), correct_value);
    }

    // For automatic cleanup.
    let _byte = unsafe { Box::from_raw(ptr) };
}

#[cfg(test)]
mod set {
    use super::*;
    fn test(start_bit: usize, num_of_bits: usize, index: isize, correct_value: u128) -> () {
        let func = |address, start_bit, num_of_bits| {
            set_bits::set(address, start_bit, num_of_bits);
        };

        test_general(start_bit, num_of_bits, index, correct_value, func);
    }

    #[test]
    fn within_a_section_1() -> () {
        test(3, 2, 0, 0b11000);
    }

    #[test]
    fn within_a_section_2() -> () {
        test(1, 4, 0, 0b11110);
    }

    #[test]
    fn all_bits_of_a_section_1() -> () {
        test(0, 128, 0, !0);
    }

    #[test]
    fn all_bits_of_a_section_2() -> () {
        test(128, 128, 1, !0);
    }

    #[test]
    fn no_bits_1() -> () {
        test(0, 0, 0, 0);
    }

    #[test]
    fn more_than_a_section_1() -> () {
        test(3, 256, 0, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF8);
    }

    #[test]
    fn more_than_a_section_2() -> () {
        test(6, 400, 0, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC0);
    }

    #[test]
    fn no_bits_2() -> () {
        test(128, 0, 0, 0);
    }

    #[test]
    fn start_bit_more_than_127_1() -> () {
        test(138, 3, 1, 0b11100_00000000);
    }

    #[test]
    fn start_bit_more_than_127_2() -> () {
        test(154, 5, 1, 0b01111100_00000000_00000000_00000000);
    }

    #[test]
    fn index_over_range() -> () {
        test(0, 128, 1, 0);
    }

    #[test]
    fn index_below_range() -> () {
        test(128, 128, 0, 0);
    }
}

#[cfg(test)]
mod clear {
    use super::*;

    fn test(start_bit: usize, num_of_bits: usize, index: isize, correct_value: u128) -> () {
        let func = |address, start_bit, num_of_bits| {
            unsafe {
                for i in 0..4 {
                    *((address as *mut u128).offset(i)) = !0;
                }
            }
            set_bits::clear(address, start_bit, num_of_bits);
        };

        test_general(start_bit, num_of_bits, index, correct_value, func);
    }

    #[test]
    fn within_a_section_1() -> () {
        test(2, 3, 0, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE3);
    }

    #[test]
    fn within_a_section_2() -> () {
        test(1, 4, 0, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE1);
    }

    #[test]
    fn more_than_a_section_1() -> () {
        test(64, 256, 0, 0xFFFFFFFFFFFFFFFF);
    }

    #[test]
    fn more_than_a_section_2() -> () {
        test(192, 256, 1, 0xFFFFFFFFFFFFFFFF);
    }

    #[test]
    fn all_bits_within_a_section() -> () {
        test(0, 128, 0, 0);
    }

    #[test]
    fn no_bits() -> () {
        test(0, 0, 0, !0);
    }
}

#[test]
fn set_and_clear_within_a_byte() -> () {
    let byte: Box<u32> = Box::new(0);
    let ptr = Box::into_raw(byte);

    set_bits::set(ptr as usize, 5, 3);
    set_bits::set(ptr as usize, 0, 3);
    set_bits::set(ptr as usize, 3, 2);
    unsafe {
        assert_eq!(*ptr, 0b11111111);
    }

    set_bits::clear(ptr as usize, 5, 3);
    set_bits::clear(ptr as usize, 0, 3);
    set_bits::clear(ptr as usize, 3, 2);
    unsafe {
        assert_eq!(*ptr, 0);
    }

    let _byte = unsafe { Box::from_raw(ptr) };
}

#[test]
fn set_and_clear_more_than_a_byte() -> () {
    let byte: Box<u32> = Box::new(0);
    let ptr = Box::into_raw(byte);

    set_bits::set(ptr as usize, 0, 10);
    set_bits::set(ptr as usize, 10, 10);
    set_bits::set(ptr as usize, 20, 12);
    unsafe {
        assert_eq!(*ptr, !0);
    }

    set_bits::clear(ptr as usize, 0, 10);
    set_bits::clear(ptr as usize, 10, 10);
    set_bits::clear(ptr as usize, 20, 12);
    unsafe {
        assert_eq!(*ptr, 0);
    }

    let _byte = unsafe { Box::from_raw(ptr) };
}
