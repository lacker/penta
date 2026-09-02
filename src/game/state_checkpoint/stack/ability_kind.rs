use super::super::{DeclarativeAbilityDef, Game, StackAbilityPayload, StackObjectKind};

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
        ) if alternative.kind == crate::card::AlternativeCastKindDef::Miracle => {
            StackAbilityCondition::Supported(None)
        }
        _ => StackAbilityCondition::Unsupported,
    }
}

pub(super) fn stack_payload_matches(
    payload: &StackAbilityPayload,
    candidate: &crate::card::AbilityDef,
) -> bool {
    if let Some(definition) = payload.definition.as_deref() {
        return definition == candidate;
    }
    let condition = match candidate.definition {
        DeclarativeAbilityDef::Triggered(triggered) => triggered.condition,
        DeclarativeAbilityDef::Replacement(_) => {
            return payload.text == Some(candidate.text) && payload.condition.is_none();
        }
        DeclarativeAbilityDef::AlternativeCast(alternative)
            if alternative.kind == crate::card::AlternativeCastKindDef::Miracle =>
        {
            None
        }
        _ => return false,
    };
    payload.text == Some(candidate.text)
        && payload.condition == condition
        && payload.resolver == Game::ability_resolver(payload.origin, candidate)
}
