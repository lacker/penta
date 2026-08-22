//! Core Set 2020 cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AppliedEffectDef, AppliedRuleDef, CardArt,
    CardRules, CardSet, CardType, EffectDef, EffectRecipientDef, ObjectPredicateDef,
    ResolvedEffectDurationDef,
};
use crate::{TargetIndex, mana_cost};

/// "Another" excludes the Key itself, which is what stops it untapping
/// itself for free every turn.
static ANOTHER_ARTIFACT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Artifact),
        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
    ]),
)];

static A_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

// M20 230 — Manifold Key
pub(in crate::card::sets) static MANIFOLD_KEY: CardRecord = CardRecord::new_with_legacy_id(
    2207,
    "Manifold Key",
    CardArt::new("715e637a-dfd8-45a0-b1ea-53e4abd29307", "Lake Hurwitz"),
    CardSet::Magic2020,
    // One mana that untaps a Mox for profit and, when there is nothing to
    // untap, pushes a creature through instead.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}, {T}: Untap another target artifact.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
            ],
            &ANOTHER_ARTIFACT,
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
        AbilityDef::activated_with_targets(
            "{3}, {T}: Target creature can't be blocked this turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}")),
                AbilityCostDef::TapSource,
            ],
            &A_CREATURE,
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlocked),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&MANIFOLD_KEY];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
