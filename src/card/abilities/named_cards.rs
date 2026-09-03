// Shared card-name choices and the rules that consume them.
//
// Included textually into `abilities.rs`, so the imports here are the parent
// module's. The chosen name always belongs to the permanent carrying the
// clause; every predicate below therefore resolves relative to that source.

use super::model::{
    BattlefieldEntryScalarChoiceDef, PlayActionMatcherDef, PlayRestrictionDef,
    ReplacementChoiceDef,
};

/// A spell or permanent whose effective card-part name is the one this
/// ability's source chose as it entered.
pub const SOURCES_CHOSEN_CARD_NAME: ObjectPredicateDef =
    ObjectPredicateDef::NameEquals(CardNameDef::SourceChoice);

static LOOK_AT_OPPONENT_HAND_THEN_CHOOSE_CARD_NAME: [ReplacementEffectDef; 2] = [
    ReplacementEffectDef::LookAtHand(PlayerRelation::Opponent),
    ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
        BattlefieldEntryScalarChoiceDef::CARD_NAME,
    )),
];

/// Records one catalog-derived card name on a permanent as it enters.
#[must_use]
pub const fn choose_card_name_as_enters(
    text: &'static str,
    choice: BattlefieldEntryScalarChoiceDef,
) -> AbilityDef {
    AbilityDef::as_enters(
        text,
        ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(choice)),
    )
}

/// Records one catalog-derived nonland card name on a permanent as it enters.
#[must_use]
pub const fn choose_nonland_card_name(text: &'static str) -> AbilityDef {
    choose_card_name_as_enters(text, BattlefieldEntryScalarChoiceDef::NONLAND_CARD_NAME)
}

/// Records one catalog-derived land card name on a permanent as it enters.
#[must_use]
pub const fn choose_land_card_name(text: &'static str) -> AbilityDef {
    choose_card_name_as_enters(text, BattlefieldEntryScalarChoiceDef::LAND_CARD_NAME)
}

/// Privately inspects the opponent's hand, then records any card name on the
/// entering permanent. The hand observation is available while the public
/// naming decision is pending.
#[must_use]
pub const fn look_at_opponent_hand_then_choose_card_name_as_enters(
    text: &'static str,
) -> AbilityDef {
    AbilityDef::as_enters(
        text,
        ReplacementEffectDef::Sequence(&LOOK_AT_OPPONENT_HAND_THEN_CHOOSE_CARD_NAME),
    )
}

/// No player can cast spells with the name this permanent chose.
#[must_use]
pub const fn cannot_cast_spells_with_chosen_name(text: &'static str) -> AbilityDef {
    AbilityDef::static_ability(
        text,
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::EachPlayer,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(
                PlayActionMatcherDef::CastSpell,
                SOURCES_CHOSEN_CARD_NAME,
            ))),
        },
    )
}

/// No player can activate nonmana abilities of sources with the name this
/// permanent chose.
#[must_use]
pub const fn cannot_activate_nonmana_abilities_with_chosen_name(
    text: &'static str,
) -> AbilityDef {
    AbilityDef::static_ability(
        text,
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::EachPlayer,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(
                PlayActionMatcherDef::ActivateNonManaAbility,
                SOURCES_CHOSEN_CARD_NAME,
            ))),
        },
    )
}

/// No player can activate any ability of a source with the name this
/// permanent chose, including mana abilities and sources outside the
/// battlefield.
#[must_use]
pub const fn cannot_activate_abilities_with_chosen_name(text: &'static str) -> AbilityDef {
    AbilityDef::static_ability(
        text,
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::EachPlayer,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(
                PlayActionMatcherDef::ActivateAbility,
                SOURCES_CHOSEN_CARD_NAME,
            ))),
        },
    )
}

/// Spells cast by the named relation cost this much more when their name is
/// the one this permanent chose.
#[must_use]
pub const fn chosen_name_spell_cost_increase(
    text: &'static str,
    caster: PlayerRelation,
    amount: ManaCost,
) -> AbilityDef {
    AbilityDef::static_ability(
        text,
        EffectDef::ModifyCost(CostModificationDef::increase_spell(
            SOURCES_CHOSEN_CARD_NAME,
            caster,
            amount,
        )),
    )
}

/// Nonmana activated abilities of sources with the chosen name cost this
/// much more. Mana abilities are enumerated outside the ordinary activated-
/// ability planner and remain exempt.
#[must_use]
pub const fn chosen_name_ability_cost_increase(
    text: &'static str,
    amount: ManaCost,
) -> AbilityDef {
    AbilityDef::static_ability(
        text,
        EffectDef::ModifyCost(CostModificationDef::SourceAbilityIncrease {
            source: SOURCES_CHOSEN_CARD_NAME,
            amount,
        }),
    )
}
