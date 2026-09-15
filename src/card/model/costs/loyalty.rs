/// Evaluate the loyalty cost shapes supported at activation announcement.
/// The value remains a loyalty cost; evaluating −X never turns it into an
/// ordinary counter-removal cost or narrows X to a fixed-cost integer type.
#[must_use]
pub(crate) fn loyalty_change(value: ValueDef, x: u16) -> Option<i32> {
    match value {
        ValueDef::Constant(change) if u16::try_from(change.unsigned_abs()).is_ok() => Some(change),
        ValueDef::Negate(&ValueDef::ChosenX) => Some(-i32::from(x)),
        _ => None,
    }
}

#[must_use]
pub(crate) const fn loyalty_uses_x(value: ValueDef) -> bool {
    matches!(value, ValueDef::Negate(&ValueDef::ChosenX))
}
