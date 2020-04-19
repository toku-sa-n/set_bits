use crate::bit_string;

pub enum Operation {
    Set,
    Clear,
}

type DestPtr = *mut u128;
pub type SrcVal = u128;
pub const NUM_OF_BITS: usize = 128;

pub trait EditBitFunc = Fn(DestPtr, SrcVal) -> ();

pub fn bit_operation(bit_string: bit_string::BitString, operation: Operation) -> () {
    match operation {
        Operation::Set => {
            let set = |dest: DestPtr, bit_mask| unsafe {
                *dest |= bit_mask;
            };

            edit_bit(bit_string, set);
        }
        Operation::Clear => {
            let clear = |dest: DestPtr, bit_mask: SrcVal| unsafe {
                *dest &= !bit_mask;
            };

            edit_bit(bit_string, clear);
        }
    }
}

fn edit_bit<T>(bit_string: bit_string::BitString, edit_func: T) -> ()
where
    T: EditBitFunc,
{
    for i in 0..bit_string.num_of_sections() {
        edit_func(
            bit_string.get_address_of_section(i) as DestPtr,
            bit_string.bits_at_section(i),
        );
    }
}
