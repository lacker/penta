use super::super::{DeclarativeAbilityDef, StackObjectKind};

pub(super) enum StackAbilityCondition {
    Unsupported,
    Supported(Option<&'static crate::card::TriggerConditionDef>),
}

pub(super) fn stack_ability_condition(
    kind: StackObjectKind,
    definition: &crate::card::AbilityDef,
) -> StackAbilityCondition {
    match (kind, definition.definition) {
        (StackObjectKind::ActivatedAbility, DeclarativeAbilityDef::Activated(_)) => {
            StackAbilityCondition::Supported(None)
        }
        (StackObjectKind::TriggeredAbility, DeclarativeAbilityDef::Triggered(triggered)) => {
            StackAbilityCondition::Supported(triggered.condition)
        }
        (
            StackObjectKind::TriggeredAbility,
            DeclarativeAbilityDef::AlternativeCast(alternative),
        ) if definition.is_executable()
            && alternative.kind == crate::card::AlternativeCastKindDef::Miracle =>
        {
            StackAbilityCondition::Supported(None)
        }
        _ => StackAbilityCondition::Unsupported,
    }
}
