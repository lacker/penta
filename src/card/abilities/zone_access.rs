//! Shared vocabulary for continuous knowledge and play permissions over zone queries.
use super::{AbilityDef, AppliedEffectDef, AppliedRuleDef, EffectDef, EffectRecipientDef};
use crate::card::{ObjectQueryDef, PlayPermissionDef, PlayRestrictionDef, PlayerSetDef};

#[must_use]
pub const fn cards_known_to(
    text: &'static str,
    cards: ObjectQueryDef,
    viewers: PlayerSetDef,
) -> AbilityDef {
    AbilityDef::static_ability(
        text,
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::players(viewers),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::KnownCards(cards)),
        },
    )
}

#[must_use]
pub const fn play_from_zone(
    cards: ObjectQueryDef,
    text: &'static str,
    restriction: PlayRestrictionDef,
) -> AbilityDef {
    AbilityDef::static_ability(
        text,
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Controller,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlay(PlayPermissionDef::new(
                cards,
                restriction,
            ))),
        },
    )
}
