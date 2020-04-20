use crate::bit_string;

pub enum Operation {
    Set,
    Clear,
}

type DestPtr = *mut u128;
pub type SrcVal = u128;

impl Operation {
    fn edit(&self, dest: DestPtr, bit_mask: SrcVal) -> () {
        match self {
            Self::Set => unsafe { *dest |= bit_mask },
            Self::Clear => unsafe { *dest &= !bit_mask },
        }
    }
}

pub const NUM_OF_BITS: usize = 128;

pub trait EditBitFunc = Fn(DestPtr, SrcVal) -> ();

pub fn bit_operation(bit_string: bit_string::BitString, operation: Operation) -> () {
    for i in 0..bit_string.num_of_sections() {
        operation.edit(
            bit_string.get_address_of_section(i) as DestPtr,
            bit_string.bits_at_section(i),
        );
    }
}
