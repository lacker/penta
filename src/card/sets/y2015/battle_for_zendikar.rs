//! BFZ card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::ids::TargetIndex;
use crate::mana_cost;

/// The Scion token that every BFZ Scion-maker prints in full: a 1/1 body
/// whose only job is to be sacrificed for one colourless mana.
static ELDRAZI_SCION_TOKEN: EffectDef =
    EffectDef::create_creature_token(&["Eldrazi", "Scion"], &[], 1, 1).with_abilities(&[
        AbilityDef::activated_mana(
            "Sacrifice this creature: Add {C}.",
            &[CostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
    ]);

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "BFZ",
    slug: "battle-for-zendikar",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// BFZ 58 — Eldrazi Skyspawner
pub(in crate::card::sets) static ELDRAZI_SKYSPAWNER: CardRecord = CardRecord::new(
    "Eldrazi Skyspawner",
    "9c9c1a10-446e-492a-95cc-a459dc6c08a0",
    "Chase Stone",
// Three mana for two bodies and a ritual: the Scion is what turns the
    // flier into a fourth-turn six-drop.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Eldrazi", "Drone"], 2, 1).with_abilities(&[
        abilities::devoid(),
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 colorless Eldrazi Scion creature token. It has \"Sacrifice this token: Add {C}.\"",
            ELDRAZI_SCION_TOKEN,
        ),
    ]),
);

// BFZ 106 — Carrier Thrall
pub(in crate::card::sets) static CARRIER_THRALL: CardRecord = CardRecord::new(
    "Carrier Thrall",
    "bd2ab895-9225-4eba-90c3-4023db4f8b70",
    "Lius Lasahido",
// Two mana that trades and still leaves a mana behind, which is why the
    // body is aggressive and the death trigger is not.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire"], 2, 1).with_abilities(&[
        abilities::devoid(),
        abilities::dies_trigger(
            "When this creature dies, create a 1/1 colorless Eldrazi Scion creature token. It has \"Sacrifice this token: Add {C}.\"",
            ELDRAZI_SCION_TOKEN,
        ),
    ]),
);

// BFZ 168 — Unnatural Aggression
pub(in crate::card::sets) static UNNATURAL_AGGRESSION: CardRecord = CardRecord::new(
    "Unnatural Aggression",
    "8293c66d-9a9b-4817-9bc3-ffd57fda290c",
    "James Ryman",
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
