use std::rc::Rc;

/// Ordered battlefield positions, valid only during the immutable read that
/// prepared them. Prospective arrivals are deliberately not part of this list.
#[derive(Clone, Debug)]
pub(crate) struct PreparedSourceList(Rc<[usize]>);

impl PreparedSourceList {
    pub(crate) fn compile(supplies: impl Iterator<Item = bool>) -> Self {
        Self(
            supplies
                .enumerate()
                .filter_map(|(index, supplies)| supplies.then_some(index))
                .collect(),
        )
    }

    pub(crate) fn indices(&self) -> &[usize] {
        &self.0
    }
}
