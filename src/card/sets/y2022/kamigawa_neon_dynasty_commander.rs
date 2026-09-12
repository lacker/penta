//! Kamigawa: Neon Dynasty Commander cards cataloged for the Vintage Cube
//! pool.

use crate::card::AbilityTargetDef;
use crate::card::CardTypeSet;
use crate::card::CharacteristicOperationDef;
use crate::card::DeckConstructionDef;
use crate::card::SetOperationDef;
use crate::card::SubtypeDef;
use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "NEC",
    slug: "kamigawa-neon-dynasty-commander",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// NEC 13 — Imposter Mech
// Audit: unsupported — CopyExceptionsDef can add card types but cannot replace the copied creature's card-type set with artifact while retaining its copied rules and applying the Vehicle exception.
pub(in crate::card::sets) static IMPOSTER_MECH_13: CardRecord = CardRecord::new(
    "Imposter Mech",
    "59b7450c-3163-4f12-9af1-2e998a6c36cf",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// NEC 14 — Kappa Cannoneer
pub(in crate::card::sets) static KAPPA_CANNONEER: CardRecord = CardRecord::new(
    "Kappa Cannoneer",
    "85a89077-b384-4fca-9d26-7297962c1541",
    "Jesper Ejsing",
    // Six mana on paper and rarely six in practice: the artifacts that make
    // it cheap are the same ones that make it bigger and unblockable.
    CardRules::new_artifact_creature(mana_cost!("{5}{U}"), &["Turtle", "Warrior"], 4, 4)
        .with_abilities(&[
            abilities::improvise(),
            abilities::ward(&[CostDef::Mana(crate::ManaCost::new(4, 0))], "Ward {4}"),
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

// NEC 45 — Swift Reconfiguration
pub(in crate::card::sets) static SWIFT_RECONFIGURATION_45: CardRecord = CardRecord::new(
    "Swift Reconfiguration",
    "0bd0b431-534d-4ab7-93ed-b9a25259e88e",
    "Nicholas Gregory",
    CardRules::new_enchantment(mana_cost!("{W}")).with_subtypes(&["Aura"]).with_abilities(&[
abilities::flash(),
abilities::aura_spell("Enchant creature or Vehicle", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle"))]))]),
AbilityDef::static_ability("Enchanted permanent is a Vehicle artifact with crew 5 and it loses all other card types. (It's not a creature unless it's crewed.)", EffectDef::StaticApply { recipient: EffectRecipientDef::AttachedPermanent, effect: AppliedEffectDef::Composite(&[AppliedEffectDef::set_card_types(CardTypeSet::single(CardType::Artifact)), AppliedEffectDef::Characteristic(CharacteristicOperationDef::Subtypes(SetOperationDef::Add(&["Vehicle"]))), AppliedEffectDef::add_ability(&abilities::crew("Crew 5", 5))]) })
]),
);

// NEC 46 — Yoshimaru, Ever Faithful
pub(in crate::card::sets) static YOSHIMARU_EVER_FAITHFUL_46: CardRecord = CardRecord::new(
    "Yoshimaru, Ever Faithful",
    "84dcd364-38c1-4987-a066-1c4d4533912e",
    "Ilse Gort",
    CardRules::new_creature(mana_cost!("{W}"), &["Dog"], 1, 1).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::triggered("Whenever another legendary permanent you control enters, put a +1/+1 counter on Yoshimaru.", TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[ObjectPredicateDef::Supertype(CardSupertype::Legendary), ObjectPredicateDef::ControlledBy(PlayerRelation::You), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), None, Some(ZoneKind::Battlefield)), EffectDef::AddCounters { object: EffectRecipientDef::Source, kind: CounterKind::PlusOnePlusOne, amount: ValueDef::Constant(1) }),
AbilityDef::deck_construction("Partner (You can have two commanders if both have partner.)", DeckConstructionDef::Partner, "Both commanders are designated before the game.")
]),
);

// NEC 56 — Ruthless Technomancer
// Audit: unsupported — The activated-cost planner cannot select a positive variable number of artifacts to sacrifice and preserve that X as the target's power limit.
pub(in crate::card::sets) static RUTHLESS_TECHNOMANCER_56: CardRecord = CardRecord::new(
    "Ruthless Technomancer",
    "b6f8e7b9-d90b-40a5-88f4-4edfae0d01f7",
    "PINDURSKI",
    crate::card::CardRules::unsupported(),
);

// NEC 76 — Shorikai, Genesis Engine
pub(in crate::card::sets) static SHORIKAI_GENESIS_ENGINE: CardRecord = CardRecord::new(
    "Shorikai, Genesis Engine",
    "0347cf84-42f5-4674-99de-619b0ae51d62",
    "Wisnu Tan",
// Four mana that loots every turn and pays for its own crew while it
    // does it -- the 8/8 is what the Pilots are for rather than the plan.
    CardRules::new_vehicle(mana_cost!("{2}{W}{U}"), 8, 8)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "{1}, {T}: Draw two cards, then discard a card. Create a 1/1 colorless Pilot creature \
                 token with \"This token crews Vehicles as though its power were 2 greater.\"",
                &[
                    CostDef::Mana(mana_cost!("{1}")),
                    CostDef::TapSource,
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
                    EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::creature(&["Pilot"], &[], 1, 1)
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
                    ))),
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
    &[
    &IMPOSTER_MECH_13,
    &KAPPA_CANNONEER,
    &SWIFT_RECONFIGURATION_45,
    &YOSHIMARU_EVER_FAITHFUL_46,
    &RUTHLESS_TECHNOMANCER_56,
    &SHORIKAI_GENESIS_ENGINE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
