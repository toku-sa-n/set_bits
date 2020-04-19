use set_bits;

fn test_general<T: Fn(usize, usize, usize) -> ()>(
    start_bit: usize,
    num_of_bits: usize,
    index: isize,
    correct_value: u128,
    func: T,
) -> () {
    #[repr(C, packed)]
    struct Heap {
        _heap: [u128; 4],
    };

    let byte: Box<Heap> = Box::new(Heap { _heap: [0; 4] });
    let ptr = Box::into_raw(byte);

    func(ptr as usize, start_bit, num_of_bits);
    unsafe {
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
    fn all_bits_of_a_byte_2() -> () {
        test(8, 8, 0, 0xff00);
    }

    #[test]
    fn no_bits_1() -> () {
        test(0, 0, 0, 0);
    }

    #[test]
    fn more_than_a_byte_1() -> () {
        test(3, 10, 0, 0b1111111111000);
    }

    #[test]
    fn more_than_a_byte_2() -> () {
        test(6, 13, 0, 0b1111111111111000000);
    }

    #[test]
    fn all_bits_of_u32() -> () {
        test(0, 32, 0, 0xFFFFFFFF);
    }

    #[test]
    fn no_bits_2() -> () {
        test(8, 0, 0, 0);
    }

    #[test]
    fn start_bit_more_than_7_1() -> () {
        test(10, 3, 0, 0b11100_00000000);
    }

    #[test]
    fn start_bit_more_than_7_2() -> () {
        test(26, 5, 0, 0b01111100_00000000_00000000_00000000);
    }
}

#[cfg(test)]
mod clear {
    use super::*;

    fn test(start_bit: usize, num_of_bits: usize, index: isize, correct_value: u128) -> () {
        let func = |address, start_bit, num_of_bits| {
            unsafe {
                *(address as *mut u128) = !0;
            }
            set_bits::clear(address, start_bit, num_of_bits);
        };

        test_general(start_bit, num_of_bits, index, correct_value, func);
    }

    #[test]
    fn within_a_byte_1() -> () {
        test(2, 3, 0, 0b11111111_11111111_11111111_11100011);
    }

    #[test]
    fn within_a_byte_2() -> () {
        test(1, 4, 0, 0b11111111_11111111_11111111_11100001);
    }

    #[test]
    fn more_than_a_byte_1() -> () {
        test(3, 10, 0, 0b11111111_11111111_11100000_00000111);
    }

    #[test]
    fn more_than_a_byte_2() -> () {
        test(11, 10, 0, 0b11111111_11100000_00000111_11111111);
    }

    #[test]
    fn all_bits_of_u32() -> () {
        test(0, 32, 0, 0);
    }

    #[test]
    fn start_bit_more_than_7_1() -> () {
        test(10, 3, 0, 0b11111111_11111111_11100011_11111111);
    }

    #[test]
    fn all_bits_within_a_byte() -> () {
        test(0, 8, 0, 0xffffff00);
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
