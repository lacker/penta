//! Amonkhet cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ConditionalValueDef;
use crate::card::CostDef;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRelation;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "AKH",
    slug: "amonkhet",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// AKH 24 — Regal Caracal
pub(in crate::card::sets) static REGAL_CARACAL: CardRecord = CardRecord::new(
    "Regal Caracal",
    "cd349f95-3eae-4ef2-abf8-e911bb8e93e5",
    "Filip Burburan",
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Cat"], 3, 3).with_abilities(&[
        AbilityDef::static_ability(
            "Other Cats you control get +1/+1 and have lifelink. (Damage \
             dealt by those creatures also causes you to gain that much \
             life.)",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cat")),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    AppliedEffectDef::add_ability(&abilities::lifelink()),
                ]),
            },
        ),
        abilities::enters_trigger(
            "When this creature enters, create two 1/1 white Cat creature \
             tokens with lifelink.",
            EffectDef::create_creature_token(&["Cat"], &[ManaColor::White], 1, 1)
                .with_count(ValueDef::Constant(2))
                .with_abilities(&[abilities::lifelink()]),
        ),
    ]),
);

// AKH 75 — Vizier of Tumbling Sands
pub(in crate::card::sets) static VIZIER_OF_TUMBLING_SANDS: CardRecord = CardRecord::new(
    "Vizier of Tumbling Sands",
    "ce4ff0f5-abee-4f3e-89ae-1b7ee771ec68",
    "Josu Hernaiz",
    // Two ways to untap something out of one card: the body unlocks a land
    // every turn, and cycling unlocks one the turn you gave up on the body.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Cleric"], 1, 3).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Untap another target permanent.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
        abilities::cycling!(
            "Cycling {1}{U} ({1}{U}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{1}{U}"))],
        ),
        AbilityDef::triggered_with_targets(
            "When you cycle this card, untap target permanent.",
            TriggerEventDef::DiscardedToActivate(crate::card::abilities::CYCLING),
            // No "another" here: the Vizier is in the graveyard by now, so
            // any permanent is a legal choice.
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Any,
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// AKH 81 — Bone Picker
pub(in crate::card::sets) static BONE_PICKER: CardRecord = CardRecord::new(
    "Bone Picker",
    "bdc6a825-43f7-40a4-95f0-335dc538b6cd",
    "Yeong-Hao Han",
    // A one-mana flying deathtouch blocker after any trade, which is why the
    // full four mana is a price the card almost never pays.
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Bird"], 3, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This spell costs {3} less to cast if a creature died this turn.",
            EffectDef::ReduceGenericCostBy(ValueDef::IfCreatureDiedThisTurn(
                &ConditionalValueDef {
                    then: ValueDef::Constant(3),
                    otherwise: ValueDef::Constant(0),
                },
            )),
        )
        // Read while the card is in hand: this prices the spell, so it has
        // to apply from the zone the spell is cast out of.
        .with_source_zones(&[ZoneKind::Hand]),
        abilities::flying(),
        abilities::deathtouch(),
    ]),
);

// AKH 134 — Glorybringer
pub(in crate::card::sets) static GLORYBRINGER: CardRecord = CardRecord::new(
    "Glorybringer",
    "3277ad99-5682-4baa-b106-de15721876a6",
    "Sam Burley",
    // Five mana that attacks the turn it lands for four in the air and kills
    // something on the way in. What exerting costs is the next attack, which
    // is the only thing keeping it honest.
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Dragon"], 4, 4).with_abilities(&[
        abilities::flying(),
        abilities::haste(),
        AbilityDef::triggered_with_targets(
            "You may exert this creature as it attacks. When you do, it deals 4 damage to target \
             non-Dragon creature an opponent controls.",
            TriggerEventDef::Exerted(ObjectPredicateDef::Source),
            // "Target non-Dragon creature an opponent controls." The exclusion is why
            // the card does not simply answer another Glorybringer, which is the whole
            // reason it is printed that way.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                            "Dragon",
                        ))),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(4),
            ),
        ),
    ]),
);

// AKH 192 — Vizier of the Menagerie
// Audit: unsupported — Needs a permanent player permission to spend mana of any type on creature spells, including colorless requirements; current any-color permissions do not express that spell scope and any-type conversion.
pub(in crate::card::sets) static VIZIER_OF_THE_MENAGERIE: CardRecord = CardRecord::new(
    "Vizier of the Menagerie",
    "ca204351-7a7e-4e4b-8c2b-f90fa0f9d724",
    "Victor Adame Minguez",
    CardRules::unsupported(),
);

// AKH 198 — Enigma Drake
pub(in crate::card::sets) static ENIGMA_DRAKE: CardRecord = CardRecord::new(
    "Enigma Drake",
    "66286631-c16e-410c-b963-25cfe8005d8f",
    "Steve Argyle",
    CardRules::new_creature(mana_cost!("{1}{U}{R}"), &["Drake"], 0, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "Enigma Drake's power is equal to the number of instant and \
             sorcery cards in your graveyard.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::define_power(ValueDef::CountMatchingObjects(
                    &ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                )),
            },
        ),
    ]),
);

// AKH 241 — Cradle of the Accursed
pub(in crate::card::sets) static CRADLE_OF_THE_ACCURSED: CardRecord = CardRecord::new(
    "Cradle of the Accursed",
    "41713e82-c3d3-4c2f-b075-f684cbd68ce8",
    "Noah Bradley",
// Untapped and colourless, so the body it eventually becomes costs the
    // deck nothing but the land slot.
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{3}, {T}, Sacrifice this land: Create a 2/2 black Zombie creature token. Activate only as a sorcery.",
            &[
                CostDef::Mana(mana_cost!("{3}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Zombie"], &[ManaColor::Black], 2, 2),
            ))),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &REGAL_CARACAL,
    &VIZIER_OF_TUMBLING_SANDS,
    &BONE_PICKER,
    &GLORYBRINGER,
    &VIZIER_OF_THE_MENAGERIE,
    &ENIGMA_DRAKE,
    &CRADLE_OF_THE_ACCURSED,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
