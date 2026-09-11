//! The emptiness tests serde's `skip_serializing_if` needs.
//!
//! Each is taken by reference because that is the signature the attribute
//! requires, and each answers "is this the value a checkpoint written
//! without the member would have meant?".

#[allow(clippy::trivially_copy_pass_by_ref)]
pub(super) fn is_zero_u8(value: &u8) -> bool {
    *value == 0
}

// The same, for the turn a permanent entered on.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(in crate::game::state_checkpoint) fn is_zero_u16(value: &u16) -> bool {
    *value == 0
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub(super) fn is_zero_turn(value: &u32) -> bool {
    *value == 0
}

// And for a per-seat flag nobody has been given yet.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(super) fn is_unset_for_both(value: &[bool; 2]) -> bool {
    !value[0] && !value[1]
}

// And for a per-seat list nobody has anything in: two empty vectors are what
// a checkpoint written without the member meant.
pub(super) fn is_empty_pair_of_vectors<T>(value: &[Vec<T>; 2]) -> bool {
    value[0].is_empty() && value[1].is_empty()
}

// And for a per-seat list of names nobody has attacked with yet.
pub(super) fn is_empty_pair_of_names(value: &[Vec<String>; 2]) -> bool {
    value[0].is_empty() && value[1].is_empty()
}
