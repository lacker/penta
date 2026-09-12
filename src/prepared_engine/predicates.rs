use crate::{BasicLandType, CardSupertype, CardType, ManaColor, ObjectPredicateDef};

/// A complete predicate plan for live battlefield queries. Unsupported leaves
/// reject the whole tree, including otherwise short-circuitable branches.
#[derive(Debug)]
pub(crate) enum PreparedPredicate {
    Leaf(PreparedPredicateLeaf),
    All(Box<[Self]>),
    Any(Box<[Self]>),
    Not(Box<Self>),
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum PreparedPredicateLeaf {
    Constant(bool),
    Source,
    Token,
    Tapped,
    Type(CardType),
    BasicLandTypes(&'static [BasicLandType]),
    Color(ManaColor),
    ColorCount(u8),
    Subtype(&'static str),
    Supertype(CardSupertype),
    ManaValueAtMost(u8),
}

impl PreparedPredicate {
    pub(crate) fn compile(predicate: ObjectPredicateDef) -> Option<Self> {
        use PreparedPredicateLeaf as Leaf;
        Some(match predicate {
            ObjectPredicateDef::All(predicates) => Self::All(
                predicates
                    .iter()
                    .copied()
                    .map(Self::compile)
                    .collect::<Option<_>>()?,
            ),
            ObjectPredicateDef::AnyOf(predicates) => Self::Any(
                predicates
                    .iter()
                    .copied()
                    .map(Self::compile)
                    .collect::<Option<_>>()?,
            ),
            ObjectPredicateDef::Not(predicate) => Self::Not(Box::new(Self::compile(*predicate)?)),
            ObjectPredicateDef::Any => Self::Leaf(Leaf::Constant(true)),
            ObjectPredicateDef::Spell | ObjectPredicateDef::NoncreatureSpell => {
                Self::Leaf(Leaf::Constant(false))
            }
            ObjectPredicateDef::Source => Self::Leaf(Leaf::Source),
            ObjectPredicateDef::Token => Self::Leaf(Leaf::Token),
            ObjectPredicateDef::Tapped => Self::Leaf(Leaf::Tapped),
            ObjectPredicateDef::HasType(kind) => Self::Leaf(Leaf::Type(kind)),
            ObjectPredicateDef::HasAnyBasicLandType(types) => {
                Self::Leaf(Leaf::BasicLandTypes(types))
            }
            ObjectPredicateDef::Color(color) => Self::Leaf(Leaf::Color(color)),
            ObjectPredicateDef::ColorCount(count) => Self::Leaf(Leaf::ColorCount(count)),
            ObjectPredicateDef::Subtype(crate::SubtypeDef::Literal(subtype)) => {
                Self::Leaf(Leaf::Subtype(subtype))
            }
            ObjectPredicateDef::Supertype(supertype) => Self::Leaf(Leaf::Supertype(supertype)),
            ObjectPredicateDef::ManaValueAtMost(limit) => Self::Leaf(Leaf::ManaValueAtMost(limit)),
            _ => return None,
        })
    }

    pub(crate) fn matches(&self, read: &mut impl FnMut(PreparedPredicateLeaf) -> bool) -> bool {
        match self {
            Self::Leaf(leaf) => read(*leaf),
            Self::All(predicates) => predicates.iter().all(|predicate| predicate.matches(read)),
            Self::Any(predicates) => predicates.iter().any(|predicate| predicate.matches(read)),
            Self::Not(predicate) => !predicate.matches(read),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepared_predicate_rejects_an_unsupported_branch_as_a_unit() {
        static MIXED: [ObjectPredicateDef; 2] =
            [ObjectPredicateDef::Any, ObjectPredicateDef::PowerExactly(1)];
        assert!(PreparedPredicate::compile(ObjectPredicateDef::AnyOf(&MIXED)).is_none());
        static BOUND: [ObjectPredicateDef; 2] = [
            ObjectPredicateDef::Any,
            ObjectPredicateDef::Subtype(crate::SubtypeDef::Binding(crate::Binding!("chosen_type"))),
        ];
        assert!(PreparedPredicate::compile(ObjectPredicateDef::AnyOf(&BOUND)).is_none());
    }
}
