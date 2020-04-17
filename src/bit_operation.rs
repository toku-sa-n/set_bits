use crate::bit_string;

pub enum Operation {
    Set,
    Clear,
}

pub trait EditBitFunc = Fn(*mut u8, u8) -> ();

pub fn bit_operation(bit_string: bit_string::BitString, operation: Operation) -> () {
    match operation {
        Operation::Set => {
            let set = |dest: *mut u8, bit_mask| unsafe {
                *dest |= bit_mask;
            };

            edit_bit(bit_string, set);
        }
        Operation::Clear => {
            let clear = |dest: *mut u8, bit_mask: u8| unsafe {
                *dest &= !bit_mask;
            };

            edit_bit(bit_string, clear);
        }
    }
}

fn edit_bit<T>(bit_string: bit_string::BitString, edit_bit: T) -> ()
where
    T: EditBitFunc,
{
    for i in 0..bit_string.len_in_byte() {
        edit_bit(
            bit_string.get_address_of_byte(i) as *mut u8,
            bit_string.bits_at_byte(i),
        );
    }
}
