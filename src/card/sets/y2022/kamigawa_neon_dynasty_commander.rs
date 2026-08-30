//! Kamigawa: Neon Dynasty Commander cards cataloged for the Vintage Cube
//! pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet,
    CardSupertype, CardType, CounterKind, DiscardSelectionDef, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, PlayerRelation, ResolvedEffectDurationDef, TriggerEventDef, ValueDef,
    ZoneKind, abilities,
};
use crate::mana_cost;

// NEC 14 — Kappa Cannoneer
pub(in crate::card::sets) static KAPPA_CANNONEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("85a89077-b384-4fca-9d26-7297962c1541"),
    "Kappa Cannoneer",
    CardArt::new("85a89077-b384-4fca-9d26-7297962c1541", "Jesper Ejsing"),
    CardSet::KamigawaNeonDynastyCommander,
    // Six mana on paper and rarely six in practice: the artifacts that make
    // it cheap are the same ones that make it bigger and unblockable.
    CardRules::new_artifact_creature(mana_cost!("{5}{U}"), &["Turtle", "Warrior"], 4, 4)
        .with_abilities(&[
            abilities::improvise(),
            abilities::ward(4, "Ward {4}"),
            AbilityDef::triggered(
                "Whenever this creature or another artifact you control enters, put a +1/+1 \
                 counter on this creature. It can't be blocked this turn.",
                // "This creature or another artifact you control": the Cannoneer's own
                // arrival counts, and so does every artifact after it -- including the ones
                // that are not creatures.
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                ]),
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
        ]),
);

// NEC 76 — Shorikai, Genesis Engine
pub(in crate::card::sets) static SHORIKAI_GENESIS_ENGINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0347cf84-42f5-4674-99de-619b0ae51d62"),
    "Shorikai, Genesis Engine",
    CardArt::new("0347cf84-42f5-4674-99de-619b0ae51d62", "Wisnu Tan"),
    CardSet::KamigawaNeonDynastyCommander,
    // Four mana that loots every turn and pays for its own crew while it
    // does it -- the 8/8 is what the Pilots are for rather than the plan.
    CardRules::new_vehicle(mana_cost!("{2}{W}{U}"), 8, 8)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "{1}, {T}: Draw two cards, then discard a card. Create a 1/1 colorless Pilot creature \
                 token with \"This token crews Vehicles as though its power were 2 greater.\"",
                &[
                    AbilityCostDef::Mana(mana_cost!("{1}")),
                    AbilityCostDef::TapSource,
                ],
                EffectDef::Sequence(&[
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                    },
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                    EffectDef::create_creature_token(&["Pilot"], &[], 1, 1)
                        // The Pilot is worth three power to a Vehicle and one to everything else,
                        // so the loot pays for its own crew: three activations put an 8/8 in the
                        // air, and every one of them drew two cards on the way.
                        .with_abilities(&[AbilityDef::static_ability(
                            "This token crews Vehicles as though its power were 2 greater.",
                            EffectDef::StaticApply {
                                recipient: EffectRecipientDef::Source,
                                effect: AppliedEffectDef::Rule(AppliedRuleDef::CrewsAsThoughPowerGreater(2)),
                            },
                        )])
                        .with_art(CardArt::new(
                            "be84f259-2809-48c9-9c70-861437f08c23",
                            "Mila Pesic",
                        )),
                ]),
            ),
            abilities::crew(
                "Crew 8 (Tap any number of creatures you control with total power 8 or more: This \
                 Vehicle becomes an artifact creature until end of turn.)",
                8,
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&KAPPA_CANNONEER, &SHORIKAI_GENESIS_ENGINE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
