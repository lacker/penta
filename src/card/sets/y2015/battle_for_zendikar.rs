//! BFZ card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef, AppliedRuleDef,
    CardArt, CardRules, CardSet, CardType, EffectDef, EffectRecipientDef, ObjectPredicateDef,
    ObjectRefDef, PlayerRelation, ResolvedEffectDurationDef, ZoneKind, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// BFZ 58 — Eldrazi Skyspawner
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ELDRAZI_SKYSPAWNER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c9c1a10-446e-492a-95cc-a459dc6c08a0"),
    "Eldrazi Skyspawner",
    crate::card::CardArt::new("9c9c1a10-446e-492a-95cc-a459dc6c08a0", "Chase Stone"),
    crate::card::CardSet::BattleForZendikar,
    crate::card::CardRules::unsupported(),
);

// BFZ 106 — Carrier Thrall
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CARRIER_THRALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bd2ab895-9225-4eba-90c3-4023db4f8b70"),
    "Carrier Thrall",
    crate::card::CardArt::new("bd2ab895-9225-4eba-90c3-4023db4f8b70", "Lius Lasahido"),
    crate::card::CardSet::BattleForZendikar,
    crate::card::CardRules::unsupported(),
);

// BFZ 168 — Unnatural Aggression
pub(in crate::card::sets) static UNNATURAL_AGGRESSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8293c66d-9a9b-4817-9bc3-ffd57fda290c"),
    "Unnatural Aggression",
    CardArt::new("8293c66d-9a9b-4817-9bc3-ffd57fda290c", "James Ryman"),
    CardSet::BattleForZendikar,
    CardRules::new_instant(mana_cost!("{2}{G}"))
        .printed_colors(&[])
        .with_abilities(&[
            abilities::devoid(),
            AbilityDef::spell_with_targets(
                "Target creature you control fights target creature an opponent controls. If the creature an opponent controls would die this turn, exile it instead.",
                &[
                    AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    }),
                    AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    }),
                ],
                EffectDef::Sequence(&[
                    EffectDef::Fight {
                        first: ObjectRefDef::Target(TargetIndex::PRIMARY),
                        second: ObjectRefDef::Target(TargetIndex(1)),
                        excess: None,
                    },
                    // This sentence is independent of whether the fight dealt damage. If the
                    // opponent's creature remains a legal target, any way it would die later
                    // this turn is replaced with exile.
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex(1)),
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::ExileInsteadOfDying),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&ELDRAZI_SKYSPAWNER, &CARRIER_THRALL, &UNNATURAL_AGGRESSION];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
