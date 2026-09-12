//! Amonkhet cards cataloged for the Vintage Cube pool.

use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardSupertype;
use crate::card::CostModificationDef;
use crate::card::PayOrDef;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
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
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Cat"], &[ManaColor::White], 1, 1)
                        .with_abilities(&[abilities::lifelink()]),
                ))
                .with_count(ValueDef::Constant(2)),
            ),
        ),
    ]),
);

// AKH 38 — Vizier of Remedies
// Audit: unsupported — The replacement-event vocabulary has no counter-placement event to reduce the number of -1/-1 counters placed.
pub(in crate::card::sets) static VIZIER_OF_REMEDIES_38: CardRecord = CardRecord::new(
    "Vizier of Remedies",
    "36ab760e-93e0-4dbc-aaa1-02316f62ed3f",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
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

// AKH 107 — Shadow of the Grave
// Audit: unsupported — Turn history does not preserve the identities of cards discarded or cycled this turn through their graveyard incarnations.
pub(in crate::card::sets) static SHADOW_OF_THE_GRAVE_107: CardRecord = CardRecord::new(
    "Shadow of the Grave",
    "9b0205cb-c163-4332-9624-394e1024bf6a",
    "Darek Zabrocki",
    crate::card::CardRules::unsupported(),
);

// AKH 125 — Combat Celebrant
// Audit: unsupported — Exert is implemented, but its untap debt is attached to the permanent rather than the player who exerted it. Borrowed creatures therefore skip the wrong player's untap step; a complete declaration requires that shared exert correction.
pub(in crate::card::sets) static COMBAT_CELEBRANT_125: CardRecord = CardRecord::new(
    "Combat Celebrant",
    "28b63c3d-2e55-4343-b49a-11fa602ec473",
    "Chris Rallis",
    crate::card::CardRules::unsupported(),
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

// AKH 175 — Manglehorn
pub(in crate::card::sets) static MANGLEHORN_175: CardRecord = CardRecord::new(
    "Manglehorn",
    "0aa3a844-97e6-4f5d-a36f-56fea4e06932",
    "Lius Lasahido",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Beast"], 2, 2).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "When this creature enters, you may destroy target artifact.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            },
        ),
        AbilityDef::replacement_for(
            "Artifacts your opponents control enter tapped.",
            ReplacementEventDef::ObjectEntersBattlefield {
                object: ObjectPredicateDef::HasType(CardType::Artifact),
                controller: PlayerRelation::Opponent,
                cast: None,
            },
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
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

// AKH 229 — Hazoret's Monument
pub(in crate::card::sets) static HAZORET_S_MONUMENT_229: CardRecord = CardRecord::new(
    "Hazoret's Monument",
    "7a0a70f2-f2cb-4a08-a1a7-95c8fc3de6e3",
    "Richard Wright",
    CardRules::new_artifact(mana_cost!("{3}")).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::static_ability("Red creature spells you cast cost {1} less to cast.", EffectDef::ModifyCost(CostModificationDef::reduce_spell(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Color(ManaColor::Red)]), PlayerRelation::You, ValueDef::Constant(1)))),
AbilityDef::triggered("Whenever you cast a creature spell, you may discard a card. If you do, draw a card.", TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ControlledBy(PlayerRelation::You)])), EffectDef::PayOr(PayOrDef::optional(&[CostDef::discard(ObjectPredicateDef::Any)], &abilities::draw_cards(ValueDef::Constant(1)))))
]),
);

// AKH 239 — Canyon Slough
pub(in crate::card::sets) static CANYON_SLOUGH_239: CardRecord = CardRecord::new(
    "Canyon Slough",
    "8cb273d9-466d-416d-b27d-d1bc8a249076",
    "Titus Lunter",
    CardRules::new_land(&["Swamp", "Mountain"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
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
    &VIZIER_OF_REMEDIES_38,
    &VIZIER_OF_TUMBLING_SANDS,
    &BONE_PICKER,
    &SHADOW_OF_THE_GRAVE_107,
    &COMBAT_CELEBRANT_125,
    &GLORYBRINGER,
    &MANGLEHORN_175,
    &VIZIER_OF_THE_MENAGERIE,
    &ENIGMA_DRAKE,
    &HAZORET_S_MONUMENT_229,
    &CANYON_SLOUGH_239,
    &CRADLE_OF_THE_ACCURSED,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
