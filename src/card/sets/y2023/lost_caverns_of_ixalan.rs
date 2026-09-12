//! The Lost Caverns of Ixalan card inventory.

use crate::card::CopyAbilityDef;
use crate::card::ExilePlayDurationDef;
use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityPredicateDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AggregateOperationDef;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BattlefieldArrivalDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BindObjectsDef;
use crate::card::BlockRestrictionDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::CharacteristicOperationDef;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseCardsFromCollectionDef;
use crate::card::ChooseDef;
use crate::card::ChooseForEachPlayerDef;
use crate::card::ChooseObjectOrderDef;
use crate::card::CollectionInspectionDef;
use crate::card::ColorSet;
use crate::card::ComparisonDef;
use crate::card::ConditionDef;
use crate::card::ConditionValueDef;
use crate::card::ControlDurationDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatedTokensDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DamageEventMatcherDef;
use crate::card::DamageKindDef;
use crate::card::DamageRecipientMatcherDef;
use crate::card::DamageSourceMatcherDef;
use crate::card::DiscardFollowUpDef;
use crate::card::DiscardSelectionDef;
use crate::card::DrawEventMatcherDef;
use crate::card::EffectChoiceDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::GameActionDef;
use crate::card::IfNoObjectsDef;
use crate::card::InstalledTriggerDef;
use crate::card::ManaColor;
use crate::card::ManaSelectionDef;
use crate::card::MoveObjectsDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectCountConditionDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::ObjectValueAggregateDef;
use crate::card::ObjectValueDef;
use crate::card::PayOrDef;
use crate::card::PerPlayerSelectionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealAndClassifyCardsDef;
use crate::card::RevealObjectsDef;
use crate::card::ScaledValueDef;
use crate::card::SetOperationDef;
use crate::card::SpellResolutionDestinationDef;
use crate::card::SubtypeDef;
use crate::card::SumValueDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenCopyDef;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::tokens;
use crate::ids::ParentBinding;
use crate::mana_cost;

use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y2011::innistrad as catalog_isd;
use crate::card::sets::y2012::avacyn_restored as catalog_avr;
use crate::card::sets::y2017::hour_of_devastation as catalog_hou;
use crate::card::sets::y2017::ixalan as catalog_xln;
use crate::card::sets::y2018::core_set_2019 as catalog_m19;
use crate::card::sets::y2018::rivals_of_ixalan as catalog_rix;
use crate::card::sets::y2020::ikoria as catalog_iko;
use crate::card::sets::y2023::lost_caverns_of_ixalan_commander as catalog_lcc;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "LCI",
    slug: "lost-caverns-of-ixalan",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const MAP_TOKEN: TokenCharacteristics = tokens::map().with_art(CardArt::new(
    "64839118-09d2-4645-9d3c-f80755ac781f",
    "Francesca Baerald",
));

const TREASURE_TOKEN: TokenCharacteristics = crate::card::tokens::treasure().with_art(
    CardArt::new("3dfaedeb-f8ec-4f0e-b243-c850770a86f2", "Wero Gallo"),
);

const GNOME_TOKEN: TokenCharacteristics =
    TokenCharacteristics::artifact_creature(&["Gnome"], &[], 1, 1).with_art(CardArt::new(
        "6def709a-53b3-4520-9544-74ab6472d256",
        "Racrufi",
    ));
const DINOSAUR_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Dinosaur"], &[ManaColor::Red], 3, 1).with_art(CardArt::new(
        "ee0702f9-769b-40c0-96a7-508dc8f2652c",
        "Francisco Miyara",
    ));

// LCI 1 — Abuelo's Awakening
// Audit: unsupported — Needs a reanimation arrival that applies additional creature types, flying, and base 1/1 characteristics before entry triggers and replacement choices, alongside X additional counters; current arrival modifications carry only tapped state and counters.
pub(in crate::card::sets) static ABUELO_S_AWAKENING: CardRecord = CardRecord::new(
    "Abuelo's Awakening",
    "f93b725e-2b9c-4830-ac54-b2562afe09bb",
    "Eelis Kyttanen",
    CardRules::unsupported(),
);

// LCI 2 — Acrobatic Leap
pub(in crate::card::sets) static ACROBATIC_LEAP: CardRecord = CardRecord::new(
    "Acrobatic Leap",
    "47f82d84-03ad-42dd-80ce-f0ac5e353e46",
    "Fesbra",
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +1/+3 and gains flying until end of \
         turn. Untap it.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(3),
                    ),
                    AppliedEffectDef::add_ability(&abilities::flying()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ]),
    )]),
);

// LCI 3 — Adaptive Gemguard
pub(in crate::card::sets) static ADAPTIVE_GEMGUARD: CardRecord = CardRecord::new(
    "Adaptive Gemguard",
    "83c91e24-b7e8-4040-80cb-d1b375002c10",
    "Anthony Devine",
    CardRules::new_artifact_creature(mana_cost!("{3}{W}"), &["Gnome"], 3, 3).with_abilities(&[
        AbilityDef::activated(
            "Tap two untapped artifacts and/or creatures you control: Put \
             a +1/+1 counter on this creature. Activate only as a sorcery.",
            &[CostDef::TapPermanents {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                controller: PlayerRelation::You,
                count: 2,
            }],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// LCI 4 — Attentive Sunscribe
pub(in crate::card::sets) static ATTENTIVE_SUNSCRIBE: CardRecord = CardRecord::new(
    "Attentive Sunscribe",
    "ad1a0159-7f00-429c-9318-b028b1e03ba8",
    "Devin Platts",
    CardRules::new_artifact_creature(mana_cost!("{1}{W}"), &["Gnome"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature becomes tapped, scry 1. (Look at the \
             top card of your library. You may put that card on the \
             bottom.)",
            TriggerEventDef::tapped(ObjectPredicateDef::Source),
            abilities::scry(ValueDef::Constant(1)),
        ),
    ]),
);

// LCI 5 — Bat Colony
// Audit: unsupported — Needs cast-payment mana provenance that counts mana units produced by sources with the Cave subtype, retained for the enters trigger.
pub(in crate::card::sets) static BAT_COLONY: CardRecord = CardRecord::new(
    "Bat Colony",
    "1c02134c-ec9f-4090-820e-8ba7ae4a8c2b",
    "Cristi Balanescu",
    CardRules::unsupported(),
);

// LCI 6 — Clay-Fired Bricks // Cosmium Kiln
// Audit: unsupported — Needs atomic craft payment selecting and exiling materials across the battlefield and graveyard, together with exiling the source and returning its exact successor transformed. Current activation exile selections cannot span those zones.
pub(in crate::card::sets) static CLAY_FIRED_BRICKS: CardRecord = CardRecord::new(
    "Clay-Fired Bricks // Cosmium Kiln",
    "8aece300-656b-4e0d-a45f-aa7feaff0a4e",
    "Steve Ellis",
    CardRules::unsupported(),
);

// LCI 7 — Cosmium Blast
pub(in crate::card::sets) static COSMIUM_BLAST: CardRecord = CardRecord::new(
    "Cosmium Blast",
    "193d1eac-ede7-4f75-9c74-05133b215f93",
    "Caroline Gariba",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Cosmium Blast deals 4 damage to target attacking or blocking \
         creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Attacking,
                    ObjectPredicateDef::Blocking,
                ]),
            ]),
        )],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::Constant(4),
        ),
    )]),
);

// LCI 8 — Dauntless Dismantler
// Audit: unsupported — Needs activated-cost enumeration for a repeated X mana symbol ({X}{X}{W}); the shared activation boundary supports only one X multiplier.
pub(in crate::card::sets) static DAUNTLESS_DISMANTLER: CardRecord = CardRecord::new(
    "Dauntless Dismantler",
    "3d771631-0aab-4f09-b9a6-49b6b2d8d2aa",
    "Dibujante Nocturno",
    CardRules::unsupported(),
);

// LCI 9 — Deconstruction Hammer
pub(in crate::card::sets) static DECONSTRUCTION_HAMMER: CardRecord = CardRecord::new(
    "Deconstruction Hammer",
    "0c7ba382-18c4-4833-b3d2-bd469ae2ad77",
    "Dibujante Nocturno",
    CardRules::new_artifact(mana_cost!("{W}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 and has \"{3}, {T}, Sacrifice \
                 Deconstruction Hammer: Destroy target artifact or \
                 enchantment.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets(
                            "{3}, {T}, Sacrifice Deconstruction Hammer: Destroy target \
                             artifact or enchantment.",
                            &[
                                CostDef::Mana(mana_cost!("{3}")),
                                CostDef::TapSource,
                                CostDef::SacrificeObject(ObjectRefDef::AbilityGrantSource),
                            ],
                            &[AbilityTargetDef::exactly_one_permanent(
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    ObjectPredicateDef::HasType(CardType::Enchantment),
                                ]),
                            )],
                            EffectDef::destroy_target(TargetIndex::PRIMARY),
                        )),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// LCI 10 — Dusk Rose Reliquary
// Audit: unsupported — Needs an immediate return when the source leaves, without using the stack, for an exile-until duration (CR 610.3). The current linked-exile helper installs a leaves trigger instead, introducing a response window and making the return counterable.
pub(in crate::card::sets) static DUSK_ROSE_RELIQUARY: CardRecord = CardRecord::new(
    "Dusk Rose Reliquary",
    "a5f231ef-4167-4b0a-b54c-a098b2eb2f6f",
    "Samuel Araya",
    CardRules::unsupported(),
);

// LCI 11 — Envoy of Okinec Ahau
pub(in crate::card::sets) static ENVOY_OF_OKINEC_AHAU: CardRecord = CardRecord::new(
    "Envoy of Okinec Ahau",
    "96de0741-9270-4118-bb8e-f3480c75a582",
    "Zoltan Boros",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Cat", "Advisor"], 3, 3).with_abilities(&[
        AbilityDef::activated(
            "{4}{W}: Create a 1/1 colorless Gnome artifact creature token.",
            &[CostDef::Mana(mana_cost!("{4}{W}"))],
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(GNOME_TOKEN))),
        ),
    ]),
);

// LCI 12 — Fabrication Foundry
// Audit: unsupported — Needs cast-or-activate restricted mana and an activation payment that exiles a variable artifact group, binds its total mana value as X, and constrains the graveyard target against that paid total.
pub(in crate::card::sets) static FABRICATION_FOUNDRY: CardRecord = CardRecord::new(
    "Fabrication Foundry",
    "323a05ee-8296-41c0-94ab-00913d9d84f1",
    "Racrufi",
    CardRules::unsupported(),
);

// LCI 13 — Family Reunion
pub(in crate::card::sets) static FAMILY_REUNION: CardRecord = CardRecord::new(
    "Family Reunion",
    "5c2cfbdc-8d40-4abb-afca-6c6d8e36185b",
    "Aurore Folny",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Creatures you control get +1/+1 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell(
                "Creatures you control gain hexproof until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )]),
);

// LCI 14 — Get Lost
pub(in crate::card::sets) static GET_LOST: CardRecord = CardRecord::new(
    "Get Lost",
    "522aa72b-2b8c-484c-872b-f082101cee35",
    "Eli Minaya",
    // Two mana that answers three card types at instant speed, and the two
    // Maps are what it pays for that: real but slow ones.
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature, enchantment, or planeswalker. Its controller creates two Map \
         tokens.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Enchantment),
                ObjectPredicateDef::HasType(CardType::Planeswalker),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            // "Its controller creates two Map tokens." The Maps are theirs, not yours,
            // and the permanent is already destroyed by the time they arrive -- so the
            // player is read from what the target was rather than from where it is.
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(MAP_TOKEN))
                    .with_controller(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                        TargetIndex::PRIMARY,
                    )))
                    .with_amount(2),
            ),
        ]),
    )),
);

// LCI 15 — Glorifier of Suffering
// Audit: unsupported — Needs a payment-result reflexive trigger that survives the source leaving and selects up to two targets only after the optional sacrifice succeeds. OptionalEffectTaken currently requires the source to remain on the battlefield.
pub(in crate::card::sets) static GLORIFIER_OF_SUFFERING: CardRecord = CardRecord::new(
    "Glorifier of Suffering",
    "7580ad36-7362-4dee-9511-d119173b70e8",
    "Lauren K. Cannon",
    CardRules::unsupported(),
);

// LCI 16 — Guardian of the Great Door
pub(in crate::card::sets) static GUARDIAN_OF_THE_GREAT_DOOR: CardRecord = CardRecord::new(
    "Guardian of the Great Door",
    "a86f3cb2-7822-4c19-bd84-cea177e5b6e9",
    "Justyna Dura",
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Angel"], 4, 4).with_abilities(&[
        AbilityDef::spell(
            "As an additional cost to cast this spell, tap four untapped \
             artifacts, creatures, and/or lands you control.",
            EffectDef::None,
        )
        .with_spell_additional_cost(&CostDef::TapPermanents {
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Land),
            ]),
            controller: PlayerRelation::You,
            count: 4,
        }),
        abilities::flying(),
    ]),
);

// LCI 17 — Helping Hand
pub(in crate::card::sets) static HELPING_HAND: CardRecord = CardRecord::new(
    "Helping Hand",
    "b8aa7126-5df3-42dd-b4a3-0d0ea59eeebd",
    "Aldo Domínguez",
    CardRules::new_sorcery(mana_cost!("{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return target creature card with mana value 3 or less from \
         your graveyard to the battlefield tapped.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ManaValueAtMost(3),
                ]),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )],
        EffectDef::WithBattlefieldArrival {
            effect: &EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
            arrival: BattlefieldArrivalDef {
                modifications: &[BattlefieldEntryModificationDef::Tapped],
                ..BattlefieldArrivalDef::DEFAULT
            },
        },
    )]),
);

// LCI 18 — Ironpaw Aspirant
pub(in crate::card::sets) static IRONPAW_ASPIRANT: CardRecord = CardRecord::new(
    "Ironpaw Aspirant",
    "f70689a0-ac69-4052-84fc-9055e9e1c54b",
    "Dan Murayama Scott",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Cat", "Warrior"], 1, 2).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, put a +1/+1 counter on target \
             creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// LCI 19 — Kinjalli's Dawnrunner
pub(in crate::card::sets) static KINJALLI_S_DAWNRUNNER: CardRecord = CardRecord::new(
    "Kinjalli's Dawnrunner",
    "fc4c527c-6963-47f4-bcad-841d06bb2211",
    "Arash Radkia",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Scout"], 1, 1).with_abilities(&[
        abilities::double_strike(),
        abilities::enters_trigger(
            "When this creature enters, it explores. (Reveal the top card \
             of your library. Put that card into your hand if it's a land. \
             Otherwise, put a +1/+1 counter on this creature, then put the \
             card back or put it into your graveyard.)",
            EffectDef::Explore {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// LCI 20 — Kutzil's Flanker
// Audit: unsupported — Needs a controller-relative count of creatures that left the battlefield this turn, retained across zone changes and control changes; the existing revolt flag records only whether a permanent left.
pub(in crate::card::sets) static KUTZIL_S_FLANKER: CardRecord = CardRecord::new(
    "Kutzil's Flanker",
    "d1201811-54ab-4c4e-b6e1-19b0d07e5ede",
    "Michele Giorgi",
    CardRules::unsupported(),
);

// LCI 21 — Malamet War Scribe
pub(in crate::card::sets) static MALAMET_WAR_SCRIBE: CardRecord = CardRecord::new(
    "Malamet War Scribe",
    "b92a6ba0-cea0-4084-92f1-2bd60ea25fb0",
    "Nicholas Gregory",
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Cat", "Warrior"], 4, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, creatures you control get +2/+1 \
             until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LCI 22 — Market Gnome
// Audit: unsupported — Needs battlefield-to-exile event provenance identifying exile paid while activating a craft ability, including exiled material cards rather than only the ability source.
pub(in crate::card::sets) static MARKET_GNOME: CardRecord = CardRecord::new(
    "Market Gnome",
    "36dafcd6-ce4e-4a27-bd8c-fa1a3bffc99e",
    "Gaboleps",
    CardRules::unsupported(),
);

// LCI 23 — Might of the Ancestors
pub(in crate::card::sets) static MIGHT_OF_THE_ANCESTORS: CardRecord = CardRecord::new(
    "Might of the Ancestors",
    "54f82c95-8bc1-4df9-9120-86ffc059c7dd",
    "Aldo Domínguez",
    CardRules::new_enchantment(mana_cost!("{2}{W}")).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "At the beginning of combat on your turn, target creature you \
             control gets +2/+0 and gains vigilance until end of turn.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::vigilance()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LCI 24 — Miner's Guidewing
pub(in crate::card::sets) static MINER_S_GUIDEWING: CardRecord = CardRecord::new(
    "Miner's Guidewing",
    "9048cd9d-df3f-4705-a5f4-e5b09760c631",
    "Allen Douglas",
    // A one-drop flier that pays again when it trades. Vigilance is what
    // makes the trade happen on their turn as well as yours, so the explore
    // is rarely far away.
    CardRules::new_creature(mana_cost!("{W}"), &["Bird"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::vigilance(),
        abilities::dies_trigger_with_targets(
            "When this creature dies, target creature you control explores.",
            // The Bird is already in the graveyard when this resolves, so
            // "creature you control" never includes it: the target is chosen
            // from whatever is left.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Explore {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// LCI 25 — Mischievous Pup
pub(in crate::card::sets) static MISCHIEVOUS_PUP: CardRecord = CardRecord::new(
    "Mischievous Pup",
    "7c219861-f808-456f-b551-37512764062d",
    "Devin Platts",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Dog"], 3, 1).with_abilities(&[
        abilities::flash(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, return up to one other target \
             permanent you control to its owner's hand.",
            &[AbilityTargetDef {
                minimum: 0,
                ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::You),
                    owner: None,
                })
            }],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// LCI 26 — Ojer Taq, Deepest Foundation // Temple of Civilization
// Audit: unsupported — Needs a creature-token creation replacement multiplying the entire prospective token batch by three, and persistent controller attack-count history for the back-face activation.
pub(in crate::card::sets) static OJER_TAQ_DEEPEST_FOUNDATION: CardRecord = CardRecord::new(
    "Ojer Taq, Deepest Foundation // Temple of Civilization",
    "1ca79dd4-67fc-496c-96fc-489b039c4932",
    "Cristi Balanescu",
    CardRules::unsupported(),
);

// LCI 27 — Oltec Archaeologists
pub(in crate::card::sets) static OLTEC_ARCHAEOLOGISTS: CardRecord = CardRecord::new(
    "Oltec Archaeologists",
    "57c21561-2213-4524-89a6-30a305843e5a",
    "Winona Nelson",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Human", "Artificer", "Scout"], 4, 4)
        .with_abilities(&[AbilityDef::modal_triggered(
            "When this creature enters, choose one —",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[
                AbilityDef::spell_with_targets(
                    "Return target artifact card from your graveyard to your hand.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Artifact),
                            zones: &[ZoneKind::Graveyard],
                            controller: None,
                            owner: Some(PlayerRelation::You),
                        },
                    )],
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                ),
                AbilityDef::spell("Scry 3.", abilities::scry(ValueDef::Constant(3))),
            ],
        )]),
);

// LCI 28 — Oltec Cloud Guard
pub(in crate::card::sets) static OLTEC_CLOUD_GUARD: CardRecord = CardRecord::new(
    "Oltec Cloud Guard",
    "02d68a38-2e0b-401b-b67d-a55e2af5b18d",
    "Evyn Fong",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Soldier"], 3, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 colorless Gnome \
             artifact creature token.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(GNOME_TOKEN))),
        ),
    ]),
);

// LCI 29 — Oteclan Landmark // Oteclan Levitator
// Audit: unsupported — Needs atomic craft payment selecting and exiling materials across the battlefield and graveyard, together with exiling the source and returning its exact successor transformed. Current activation exile selections cannot span those zones.
pub(in crate::card::sets) static OTECLAN_LANDMARK: CardRecord = CardRecord::new(
    "Oteclan Landmark // Oteclan Levitator",
    "099a1d1c-72ba-468e-9385-8f93e1fce001",
    "Warren Mahy",
    CardRules::unsupported(),
);

// LCI 30 — Petrify
pub(in crate::card::sets) static PETRIFY: CardRecord = CardRecord::new(
    "Petrify",
    "bbc5f28f-6361-455f-ac82-260a70e59316",
    "Samuel Araya",
    // Two mana that answers a creature or a mana rock, and unlike Pacifism
    // it also turns off the activated ability the creature was played for.
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell(
                "Enchant artifact or creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                )],
            ),
            abilities::enchanted_permanent_subdued(),
        ]),
);

// LCI 31 — Quicksand Whirlpool
// Audit: unsupported — Needs a self spell-cost reduction evaluated against the chosen target's tapped state; ReduceGenericCostBy does not receive target selections.
pub(in crate::card::sets) static QUICKSAND_WHIRLPOOL: CardRecord = CardRecord::new(
    "Quicksand Whirlpool",
    "a74ddccb-ebbd-4fad-a9b6-6b9e9bafae31",
    "L.A. Draws",
    CardRules::unsupported(),
);

// LCI 32 — Resplendent Angel (reprint)
const RESPLENDENT_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m19::RESPLENDENT_ANGEL,
    "59adbd35-9959-4a0f-babf-46ea3c15f018",
    "Victor Adame Minguez",
);

// LCI 33 — Ruin-Lurker Bat
// Audit: unsupported — Needs turn-scoped history of permanent cards entering this controller's graveyard from any zone (excluding tokens), retained even after those cards leave; current graveyard counts and morbid/revolt history do not represent descending.
pub(in crate::card::sets) static RUIN_LURKER_BAT: CardRecord = CardRecord::new(
    "Ruin-Lurker Bat",
    "d6bedf13-c2bc-4e5d-aba3-3c0d5495a9bb",
    "Camille Alquier",
    CardRules::unsupported(),
);

// LCI 34 — Sanguine Evangelist
pub(in crate::card::sets) static SANGUINE_EVANGELIST: CardRecord = CardRecord::new(
    "Sanguine Evangelist",
    "269ddd84-fdc4-4c94-b183-32ecec56967c",
    "Zezhou Chen",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Vampire", "Cleric"], 2, 1).with_abilities(&[
        crate::card::sets::mirrodin_besieged::battle_cry(),
        AbilityDef::triggered(
            "When this creature enters or dies, create a 1/1 black Bat \
             creature token with flying.",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
            ]),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Bat"], &[ManaColor::Black], 1, 1)
                    .with_abilities(&[abilities::flying()]),
            ))),
        ),
    ]),
);

// LCI 35 — Soaring Sandwing
pub(in crate::card::sets) static SOARING_SANDWING: CardRecord = CardRecord::new(
    "Soaring Sandwing",
    "e7833724-aec6-41a4-a7ea-2aa722467732",
    "John Tedrick",
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Dinosaur"], 3, 5).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, you gain 3 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
        abilities::typecycling!(
            "Plainscycling {2}",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Plains"))
        ),
    ]),
);

// LCI 36 — Spring-Loaded Sawblades // Bladewheel Chariot
// Audit: unsupported — Needs atomic craft payment selecting and exiling materials across the battlefield and graveyard, together with exiling the source and returning its exact successor transformed. Current activation exile selections cannot span those zones.
pub(in crate::card::sets) static SPRING_LOADED_SAWBLADES: CardRecord = CardRecord::new(
    "Spring-Loaded Sawblades // Bladewheel Chariot",
    "24417388-f2bb-4783-bdce-264774531838",
    "Wayne Reynolds",
    CardRules::unsupported(),
);

// LCI 37 — Thousand Moons Crackshot
// Audit: unsupported — Needs a payment-result reflexive trigger that survives the source leaving and chooses its creature target only after the optional mana payment succeeds.
pub(in crate::card::sets) static THOUSAND_MOONS_CRACKSHOT: CardRecord = CardRecord::new(
    "Thousand Moons Crackshot",
    "741a7439-965d-49f2-b43e-053f29196e6b",
    "Marie Magny",
    CardRules::unsupported(),
);

// LCI 38 — Thousand Moons Infantry
// Audit: unsupported — Needs an untap-step rule untapping this permanent during other players' untap steps, rather than an upkeep trigger or ordinary activated untap.
pub(in crate::card::sets) static THOUSAND_MOONS_INFANTRY: CardRecord = CardRecord::new(
    "Thousand Moons Infantry",
    "c1974a8c-3328-4ff5-9a00-cb79ebb8ccf6",
    "Manuel Castañón",
    CardRules::unsupported(),
);

// LCI 39 — Thousand Moons Smithy // Barracks of the Thousand
// Audit: unsupported — Needs a cast trigger linked to mana produced by a specific land incarnation, with the mana provenance preserved after the source leaves or transforms.
pub(in crate::card::sets) static THOUSAND_MOONS_SMITHY: CardRecord = CardRecord::new(
    "Thousand Moons Smithy // Barracks of the Thousand",
    "4a6bec46-1acd-4726-b8d9-3045ac6a2ea2",
    "Manuel Castañón",
    CardRules::unsupported(),
);

// LCI 40 — Tinker's Tote
pub(in crate::card::sets) static TINKER_S_TOTE: CardRecord = CardRecord::new(
    "Tinker's Tote",
    "8321857e-7977-46dd-8357-d732312e5261",
    "Julia Metzger",
    CardRules::new_artifact(mana_cost!("{2}{W}")).with_abilities(&[
        abilities::enters_trigger(
            "When this artifact enters, create two 1/1 colorless Gnome \
             artifact creature tokens.",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(GNOME_TOKEN))
                    .with_count(ValueDef::Constant(2)),
            ),
        ),
        AbilityDef::activated(
            "{W}, Sacrifice this artifact: You gain 3 life.",
            &[CostDef::Mana(mana_cost!("{W}")), CostDef::SacrificeSource],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
    ]),
);

// LCI 41 — Unstable Glyphbridge // Sandswirl Wanderglyph
// Audit: unsupported — Needs atomic craft payment selecting and exiling materials across the battlefield and graveyard, together with exiling the source and returning its exact successor transformed. Current activation exile selections cannot span those zones.
pub(in crate::card::sets) static UNSTABLE_GLYPHBRIDGE: CardRecord = CardRecord::new(
    "Unstable Glyphbridge // Sandswirl Wanderglyph",
    "d70f48e7-582c-4dd2-a64e-6fd03fa6b77e",
    "Bastien Grivet",
    CardRules::unsupported(),
);

// LCI 42 — Vanguard of the Rose
pub(in crate::card::sets) static VANGUARD_OF_THE_ROSE: CardRecord = CardRecord::new(
    "Vanguard of the Rose",
    "0200c504-9137-4bc2-9cd4-eaf0643a2855",
    "Alex Brock",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Vampire", "Knight"], 3, 1).with_abilities(&[
        AbilityDef::activated(
            "{1}, Sacrifice another creature or artifact: This creature \
             gains indestructible until end of turn. Tap it.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Artifact),
                    ]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ])),
            ],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Tap {
                    object: EffectRecipientDef::Source,
                },
            ]),
        ),
    ]),
);

// LCI 43 — Warden of the Inner Sky
// Audit: unsupported — Needs a live total of all counters on the source, regardless of counter kind, for the flying/vigilance condition; existing source-counter predicates require one named kind.
pub(in crate::card::sets) static WARDEN_OF_THE_INNER_SKY: CardRecord = CardRecord::new(
    "Warden of the Inner Sky",
    "549fd992-ed37-431d-97cb-9ca017db1d47",
    "Raoul Vitale",
    CardRules::unsupported(),
);

// LCI 44 — Akal Pakal, First Among Equals
// Audit: unsupported — Needs controller-relative artifact-entry history for the current turn, retained if the artifact leaves or changes types/control before the end step.
pub(in crate::card::sets) static AKAL_PAKAL_FIRST_AMONG_EQUALS: CardRecord = CardRecord::new(
    "Akal Pakal, First Among Equals",
    "ab9f6a1b-8467-4584-affd-8c71d3e34d2f",
    "Ryan Pancoast",
    CardRules::unsupported(),
);

// LCI 45 — Ancestral Reminiscence
pub(in crate::card::sets) static ANCESTRAL_REMINISCENCE: CardRecord = CardRecord::new(
    "Ancestral Reminiscence",
    "625690d3-7131-45de-adea-9c927241e661",
    "Artur Treffner",
    CardRules::new_sorcery(mana_cost!("{3}{U}")).with_abilities(&[AbilityDef::spell(
        "Draw three cards, then discard a card.",
        EffectDef::Sequence(&[
            abilities::draw_cards(ValueDef::Constant(3)),
            EffectDef::Discard {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ]),
    )]),
);

// LCI 46 — Brackish Blunder
pub(in crate::card::sets) static BRACKISH_BLUNDER: CardRecord = CardRecord::new(
    "Brackish Blunder",
    "dea3d0f4-76f5-416f-93ba-8b003b967816",
    "Daarken",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return target creature to its owner's hand. If it was tapped, \
         create a Map token. (It's an artifact with \"{1}, {T}, \
         Sacrifice this token: Target creature you control explores. \
         Activate only as a sorcery.\")",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::IfElseCondition {
            condition: &TriggerConditionDef::TargetMatches {
                slot: TargetIndex::PRIMARY,
                object: ObjectPredicateDef::Tapped,
            },
            then: &EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(MAP_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ]),
            otherwise: &EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        },
    )]),
);

// LCI 47 — Braided Net // Braided Quipu
// Audit: unsupported — Needs atomic craft payment selecting and exiling materials across the battlefield and graveyard, together with exiling the source and returning its exact successor transformed. Current activation exile selections cannot span those zones.
pub(in crate::card::sets) static BRAIDED_NET: CardRecord = CardRecord::new(
    "Braided Net // Braided Quipu",
    "68a6ede0-6d57-4e29-9e3b-3569ab7f0bcd",
    "Diego Gisbert",
    CardRules::unsupported(),
);

// LCI 48 — Chart a Course (reprint)
const CHART_A_COURSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_xln::CHART_A_COURSE,
    "233beaac-37a4-4824-8f31-438b6bfe794b",
    "Josu Solano",
);

// LCI 49 — Cogwork Wrestler
pub(in crate::card::sets) static COGWORK_WRESTLER: CardRecord = CardRecord::new(
    "Cogwork Wrestler",
    "a9d329ad-6b19-4aa7-a53f-38c7b76c4c96",
    "Tomek Larek",
    CardRules::new_artifact_creature(mana_cost!("{U}"), &["Gnome"], 1, 2).with_abilities(&[
        abilities::flash(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, target creature an opponent \
             controls gets -2/-0 until end of turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LCI 50 — Confounding Riddle
pub(in crate::card::sets) static CONFOUNDING_RIDDLE: CardRecord = CardRecord::new(
    "Confounding Riddle",
    "f2ae23db-c391-402c-9568-65447cada66e",
    "A. M. Sartor",
    CardRules::new_instant(mana_cost!("{2}{U}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Look at the top four cards of your library. Put one of them \
                 into your hand and the rest into your graveyard.",
                abilities::look_at_top_cards_choose_to_hand_rest_graveyard(
                    ValueDef::Constant(4),
                    ObjectPredicateDef::Any,
                    1,
                    1,
                ),
            ),
            AbilityDef::spell_with_targets(
                "Counter target spell unless its controller pays {4}.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Any,
                        zones: &[ZoneKind::Stack],
                        controller: None,
                        owner: None,
                    },
                )],
                abilities::counter_target_unless_paid(&[CostDef::Mana(mana_cost!("{4}"))]),
            ),
        ],
    )]),
);

// LCI 51 — Council of Echoes
pub(in crate::card::sets) static COUNCIL_OF_ECHOES: CardRecord = CardRecord::new(
    "Council of Echoes",
    "85ad358d-a520-4d57-82b0-d2297da1fbde",
    "Fariba Khamseh",
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Spirit", "Advisor"], 4, 4).with_abilities(
        &[
            abilities::flying(),
            AbilityDef::triggered_if_with_targets(
                "Descend 4 — When this creature enters, if there are four or \
                 more permanent cards in your graveyard, return up to one \
                 target nonland permanent other than this creature to its \
                 owner's hand.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 4,
                },
                &[AbilityTargetDef {
                    minimum: 0,
                    ..AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    ]))
                }],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
        ],
    ),
);

// LCI 52 — Deeproot Pilgrimage
// Audit: unsupported — Needs one trigger for a simultaneous batch of matching Merfolk becoming tapped; the current tap matcher emits a separate trigger per permanent.
pub(in crate::card::sets) static DEEPROOT_PILGRIMAGE: CardRecord = CardRecord::new(
    "Deeproot Pilgrimage",
    "e2449311-a705-4a31-a345-a36d436ae561",
    "Rémi Jacquot",
    CardRules::unsupported(),
);

// LCI 53 — Didact Echo
pub(in crate::card::sets) static DIDACT_ECHO: CardRecord = CardRecord::new(
    "Didact Echo",
    "3c068b2e-17cc-4fb8-bd79-b775a4713d74",
    "Irina Nordsol",
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Spirit", "Cleric"], 3, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, draw a card.",
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
        AbilityDef::static_ability(
            "Descend 4 — This creature has flying as long as there are \
             four or more permanent cards in your graveyard.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 4,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::flying()),
                },
            },
        ),
    ]),
);

// LCI 54 — Eaten by Piranhas
// Audit: unsupported — Needs a static effect that replaces the attached permanent's complete card-type set, rather than adding types; full card-type replacement is outside the shared static runtime boundary.
pub(in crate::card::sets) static EATEN_BY_PIRANHAS: CardRecord = CardRecord::new(
    "Eaten by Piranhas",
    "b0c504ef-2382-4174-9b1d-5f38e12a28fc",
    "Abz J Harding",
    CardRules::unsupported(),
);

// LCI 55 — The Enigma Jewel // Locus of Enlightenment
// Audit: unsupported — Needs atomic craft payment selecting and exiling materials across the battlefield and graveyard, together with exiling the source and returning its exact successor transformed. Current activation exile selections cannot span those zones.
pub(in crate::card::sets) static THE_ENIGMA_JEWEL: CardRecord = CardRecord::new(
    "The Enigma Jewel // Locus of Enlightenment",
    "2e98970d-06a8-4c91-ba47-4a02c5b949f2",
    "Martin de Diego Sádaba",
    CardRules::unsupported(),
);

// LCI 56 — The Everflowing Well // The Myriad Pools
// Audit: unsupported — Needs a cast trigger linked to mana produced by a specific land incarnation, with the mana provenance preserved after the source leaves or transforms.
pub(in crate::card::sets) static THE_EVERFLOWING_WELL: CardRecord = CardRecord::new(
    "The Everflowing Well // The Myriad Pools",
    "bf573fb7-fa6c-4df7-8e5e-1e071585361e",
    "David Álvarez",
    CardRules::unsupported(),
);

// LCI 57 — Frilled Cave-Wurm
pub(in crate::card::sets) static FRILLED_CAVE_WURM: CardRecord = CardRecord::new(
    "Frilled Cave-Wurm",
    "6f65e1bc-fade-4fdf-a1fd-de068bff9e4c",
    "Aaron Miller",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Salamander", "Wurm"], 2, 5).with_abilities(&[
        AbilityDef::static_ability(
            "Descend 4 — This creature gets +2/+0 as long as there are \
             four or more permanent cards in your graveyard.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 4,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                },
            },
        ),
    ]),
);

// LCI 58 — Hermitic Nautilus
pub(in crate::card::sets) static HERMITIC_NAUTILUS: CardRecord = CardRecord::new(
    "Hermitic Nautilus",
    "545503f8-a8c6-4518-9ad6-76e4996397fc",
    "Logan Feliciano",
    CardRules::new_artifact_creature(mana_cost!("{1}{U}"), &["Nautilus"], 1, 4).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::activated(
            "{1}{U}: This creature gets +3/-3 until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(-3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LCI 59 — Hurl into History
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static HURL_INTO_HISTORY: CardRecord = CardRecord::new(
    "Hurl into History",
    "5946463a-2240-4376-b6f5-fd6e3a9cc51c",
    "Eli Minaya",
    CardRules::unsupported(),
);

// LCI 60 — Inverted Iceberg // Iceberg Titan
// Audit: unsupported — Needs atomic craft payment selecting and exiling materials across the battlefield and graveyard, together with exiling the source and returning its exact successor transformed. Current activation exile selections cannot span those zones.
pub(in crate::card::sets) static INVERTED_ICEBERG: CardRecord = CardRecord::new(
    "Inverted Iceberg // Iceberg Titan",
    "ac5e9a53-cc4f-4ced-8088-5a73d619eae3",
    "Campbell White",
    CardRules::unsupported(),
);

// LCI 61 — Kitesail Larcenist
pub(in crate::card::sets) static KITESAIL_LARCENIST: CardRecord = CardRecord::new(
    "Kitesail Larcenist",
    "03207457-70d8-4462-8c8b-ed39791d56a1",
    "Sidharth Chaturvedi",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Pirate"], 2, 3).with_abilities(&[
        abilities::flying(),
        abilities::ward(&[CostDef::Mana(mana_cost!("{1}"))], "Ward {1}"),
        abilities::enters_trigger_with_targets(
            "When this creature enters, for each player, choose up to one \
             other target artifact or creature that player controls. For \
             as long as this creature remains on the battlefield, the \
             chosen permanents become Treasure artifacts with \"{T}, \
             Sacrifice this artifact: Add one mana of any color\" and lose \
             all other abilities.",
            &[
                AbilityTargetDef {
                    minimum: 0,
                    ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                            ]),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        zones: &const { [ZoneKind::Battlefield] },
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    })
                },
                AbilityTargetDef {
                    minimum: 0,
                    ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                            ]),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        zones: &const { [ZoneKind::Battlefield] },
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    })
                },
            ],
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Union(&[
                    ObjectSetDef::LegalTargets(TargetIndex::PRIMARY),
                    ObjectSetDef::LegalTargets(TargetIndex(1)),
                ])),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::set_card_types(CardTypeSet::single(CardType::Artifact)),
                    AppliedEffectDef::Characteristic(CharacteristicOperationDef::Subtypes(
                        SetOperationDef::Set(&["Treasure"]),
                    )),
                    AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
                    AppliedEffectDef::add_ability(&AbilityDef::activated_mana(
                        "{T}, Sacrifice this artifact: Add one mana of any color.",
                        &[CostDef::TapSource, CostDef::SacrificeSource],
                        EffectDef::AddMana(AddManaEffectDef::any_color()),
                    )),
                ]),
                duration: ResolvedEffectDurationDef::WhileSourceRemains,
            },
        ),
    ]),
);

// LCI 62 — Lodestone Needle // Guidestone Compass
// Audit: unsupported — Needs atomic craft payment selecting and exiling materials across the battlefield and graveyard, together with exiling the source and returning its exact successor transformed. Current activation exile selections cannot span those zones.
pub(in crate::card::sets) static LODESTONE_NEEDLE: CardRecord = CardRecord::new(
    "Lodestone Needle // Guidestone Compass",
    "dedd7a22-92e2-41fd-aa80-944c69653a5e",
    "José Parodi",
    CardRules::unsupported(),
);

// LCI 63 — Malcolm, Alluring Scoundrel
pub(in crate::card::sets) static MALCOLM_ALLURING_SCOUNDREL: CardRecord = CardRecord::new(
    "Malcolm, Alluring Scoundrel",
    "19d6834d-afa3-4747-a62d-0654f4d9729f",
    "Fesbra",
    // Two mana for an evasive body that loots every time it connects, and
    // that turns the loot into a free spell once it has connected four
    // times.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Siren", "Pirate"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flash(),
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever this creature deals combat damage to a player, put a chorus counter on \
                 it. Draw a card, then discard a card. If there are four or more chorus counters \
                 on it, you may cast the discarded card without paying its mana cost.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::named("chorus"),
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: Some(DiscardFollowUpDef {
                            counted: ObjectPredicateDef::Any,
                            bound: Some(ParentBinding),
                            effect: &EffectDef::IfCondition {
                                // Read after the counter has been added, so the connection that makes it
                                // four is itself the one that pays.
                                condition: &TriggerConditionDef::SourceCounters {
                                    kind: CounterKind::named("chorus"),
                                    comparison: ComparisonDef::GreaterOrEqual,
                                    amount: 4,
                                },
                                then: &EffectDef::MayCastTargetWithoutPaying {
                                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        ParentBinding,
                                    )),
                                    // What the fourth connection is worth: the card you just threw away,
                                    // granted a cast with no mana-payment cost. Its destination is unchanged.
                                    ability: &AbilityDef::alternative_cast(
                                        crate::NO_COSTS,
                                        AlternativeCastKindDef::Granted,
                                        Some("Cast without paying its mana cost."),
                                        EffectDef::None,
                                    ),
                                },
                            },
                        }),
                    },
                ]),
            ),
        ]),
);

// LCI 64 — Marauding Brinefang
pub(in crate::card::sets) static MARAUDING_BRINEFANG: CardRecord = CardRecord::new(
    "Marauding Brinefang",
    "78328899-7996-4cfd-bf00-d2a0d0ff3ef8",
    "John Tedrick",
    CardRules::new_creature(mana_cost!("{5}{U}{U}"), &["Dinosaur"], 6, 7).with_abilities(&[
        abilities::ward(&[CostDef::Mana(mana_cost!("{3}"))], "Ward {3}"),
        abilities::typecycling!(
            "Islandcycling {2}",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Island"))
        ),
    ]),
);

// LCI 65 — Merfolk Cave-Diver
// Audit: unsupported — Needs a committed explore event reporting the exploring creature and its controller so another permanent can trigger after each exploration.
pub(in crate::card::sets) static MERFOLK_CAVE_DIVER: CardRecord = CardRecord::new(
    "Merfolk Cave-Diver",
    "6e0fe81c-b8ef-49ff-8743-f03d0220cb9e",
    "Fesbra",
    CardRules::unsupported(),
);

// LCI 66 — Oaken Siren
// Audit: unsupported — Needs restricted ability-activation mana and an OR between casting an artifact spell and activating an artifact ability; the shared mana boundary rejects activation restrictions and combines restrictions conjunctively.
pub(in crate::card::sets) static OAKEN_SIREN: CardRecord = CardRecord::new(
    "Oaken Siren",
    "d7731ef5-da74-4436-8ee7-01c065cbefae",
    "Lars Grant-West",
    CardRules::unsupported(),
);

// LCI 67 — Ojer Pakpatiq, Deepest Epoch // Temple of Cyclical Time
// Audit: unsupported — Needs a shared mana ability that adds mana and removes a time counter as one immediate non-stack resolution; generic mana clauses do not execute arbitrary sibling effects.
pub(in crate::card::sets) static OJER_PAKPATIQ_DEEPEST_EPOCH: CardRecord = CardRecord::new(
    "Ojer Pakpatiq, Deepest Epoch // Temple of Cyclical Time",
    "a9d71007-bc04-4dff-ad3f-e2c0b5b4400e",
    "Chris Rahn",
    CardRules::unsupported(),
);

// LCI 68 — Orazca Puzzle-Door
pub(in crate::card::sets) static ORAZCA_PUZZLE_DOOR: CardRecord = CardRecord::new(
    "Orazca Puzzle-Door",
    "27acf726-52f5-493d-8678-c12485f67747",
    "Cristi Balanescu",
    CardRules::new_artifact(mana_cost!("{U}")).with_abilities(&[AbilityDef::activated(
        "{1}, {T}, Sacrifice this artifact: Look at the top two cards \
         of your library. Put one of those cards into your hand and \
         the other into your graveyard.",
        &[
            CostDef::Mana(mana_cost!("{1}")),
            CostDef::TapSource,
            CostDef::SacrificeSource,
        ],
        abilities::look_at_top_cards_choose_to_hand_rest_graveyard(
            ValueDef::Constant(2),
            ObjectPredicateDef::Any,
            1,
            1,
        ),
    )]),
);

// LCI 69 — Out of Air
// Audit: unsupported — Needs a self spell-cost reduction evaluated against the chosen target spell's creature type; ReduceGenericCostBy does not receive target selections.
pub(in crate::card::sets) static OUT_OF_AIR: CardRecord = CardRecord::new(
    "Out of Air",
    "c263db55-fcac-4b49-b626-7c8092accfcd",
    "Francisco Miyara",
    CardRules::unsupported(),
);

// LCI 70 — Pirate Hat
pub(in crate::card::sets) static PIRATE_HAT: CardRecord = CardRecord::new(
    "Pirate Hat",
    "9d11e3d4-769d-47aa-8d3a-ce1ac60b68b8",
    "Domenico Cava",
    CardRules::new_artifact(mana_cost!("{1}{U}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 and has \"Whenever this creature \
                 attacks, draw a card, then discard a card.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&AbilityDef::triggered(
                            "Whenever this creature attacks, draw a card, then discard a card.",
                            TriggerEventDef::attacks(ObjectPredicateDef::Source),
                            EffectDef::Sequence(&[
                                abilities::draw_cards(ValueDef::Constant(1)),
                                EffectDef::Discard {
                                    recipient: EffectRecipientDef::Controller,
                                    amount: ValueDef::Constant(1),
                                    selection: DiscardSelectionDef::RecipientChooses,
                                    then: None,
                                },
                            ]),
                        )),
                    ]),
                },
            ),
            AbilityDef::activated_with_targets(
                "Equip Pirate {1}",
                &[CostDef::Mana(mana_cost!("{1}"))],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Pirate")),
                        ]),
                        zones: &const { [ZoneKind::Battlefield] },
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed),
            abilities::equip(&[CostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// LCI 71 — Relic's Roar
pub(in crate::card::sets) static RELIC_S_ROAR: CardRecord = CardRecord::new(
    "Relic's Roar",
    "35096eb3-ad7b-4b6e-b799-cb4cc447883e",
    "Nino Vecia",
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Until end of turn, target artifact or creature becomes a \
         Dinosaur artifact creature with base power and toughness 4/3 \
         in addition to its other types.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Creature),
            ]),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::add_card_types(
                    CardTypeSet::single(CardType::Artifact).with(CardType::Creature),
                ),
                AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Dinosaur"])),
                AppliedEffectDef::set_base_power_toughness(
                    ValueDef::Constant(4),
                    ValueDef::Constant(3),
                ),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// LCI 72 — River Herald Scout
pub(in crate::card::sets) static RIVER_HERALD_SCOUT: CardRecord = CardRecord::new(
    "River Herald Scout",
    "8ae8ab66-5840-4b2e-bd4e-94ead0c79b61",
    "Josu Hernaiz",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk", "Scout"], 1, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, it explores. (Reveal the top card \
             of your library. Put that card into your hand if it's a land. \
             Otherwise, put a +1/+1 counter on this creature, then put the \
             card back or put it into your graveyard.)",
            EffectDef::Explore {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// LCI 73 — Sage of Days
pub(in crate::card::sets) static SAGE_OF_DAYS: CardRecord = CardRecord::new(
    "Sage of Days",
    "a6e53495-c76c-4cd4-b93e-b2c491d2c13a",
    "Aldo Domínguez",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 3, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, look at the top three cards of \
             your library. You may put one of those cards back on top of \
             your library. Put the rest into your graveyard.",
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(3),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::Any,
                minimum: 0,
                maximum: 1,
                chosen: crate::Binding!("chosen"),
                remainder: crate::Binding!("rest"),
                then: &EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Library,
                        ZonePlacement::Top,
                    ),
                    EffectDef::MoveObjects(MoveObjectsDef {
                        input: ObjectSetDef::Binding(crate::Binding!("rest")),
                        from: Some(ZoneKind::Library),
                        zone: ZoneKind::Graveyard,
                        placement: ZonePlacement::Top,
                        moved: None,
                        then: &EffectDef::None,
                    }),
                ]),
            }),
        ),
    ]),
);

// LCI 74 — Self-Reflection
pub(in crate::card::sets) static SELF_REFLECTION: CardRecord = CardRecord::new(
    "Self-Reflection",
    "1242203d-c9b5-4ab6-802e-e222f92291e9",
    "Henry Peters",
    CardRules::new_sorcery(mana_cost!("{4}{U}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Create a token that's a copy of target creature you control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Copy(&TokenCopyDef {
                object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                exceptions: CopyExceptionsDef::NONE,
            }))),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{3}{U}"))]),
    ]),
);

// LCI 75 — Shipwreck Sentry
// Audit: unsupported — Needs controller-relative artifact-entry history retained even when the entered artifact leaves the battlefield or changes types/control during the turn.
pub(in crate::card::sets) static SHIPWRECK_SENTRY: CardRecord = CardRecord::new(
    "Shipwreck Sentry",
    "814803bc-72cd-46db-b957-4322f2a7b28a",
    "Ryan Valle",
    CardRules::unsupported(),
);

// LCI 76 — Sinuous Benthisaur
pub(in crate::card::sets) static SINUOUS_BENTHISAUR: CardRecord = CardRecord::new(
    "Sinuous Benthisaur",
    "7a58639c-001f-4cb1-89fd-0a0967b86977",
    "John Tedrick",
    CardRules::new_creature(mana_cost!("{5}{U}"), &["Dinosaur"], 4, 4).with_ability(
        abilities::enters_trigger(
            "When this creature enters, look at the top X cards of your \
             library, where X is the number of Caves you control plus the \
             number of Cave cards in your graveyard. Put two of those \
             cards into your hand and the rest on the bottom of your \
             library in a random order.",
            abilities::look_at_top_cards_choose_to_hand_rest_random_bottom(
                ValueDef::Sum(&SumValueDef::new(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cave")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cave")),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                )),
                ObjectPredicateDef::Any,
                2,
                2,
            ),
        ),
    ),
);

// LCI 77 — Song of Stupefaction
pub(in crate::card::sets) static SONG_OF_STUPEFACTION: CardRecord = CardRecord::new(
    "Song of Stupefaction",
    "ab8f7a75-df5e-43b3-8c33-328ad0dc3c40",
    "Ernanda Souza",
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell(
                "Enchant creature or Vehicle",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                    ]),
                )],
            ),
            abilities::enters_trigger(
                "When this Aura enters, you may mill two cards. (You may put \
                 the top two cards of your library into your graveyard.)",
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::Mill {
                        player: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                    },
                },
            ),
            AbilityDef::static_ability(
                "Fathomless descent — Enchanted permanent gets -X/-0, where X \
                 is the number of permanent cards in your graveyard.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Scaled(&ScaledValueDef::new(
                            ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::HasType(CardType::Enchantment),
                                    ObjectPredicateDef::HasType(CardType::Land),
                                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                                ]),
                                &[ZoneKind::Graveyard],
                                PlayerRelation::You,
                            )),
                            -1,
                        )),
                        ValueDef::Constant(0),
                    ),
                },
            ),
        ]),
);

// LCI 78 — Spyglass Siren
pub(in crate::card::sets) static SPYGLASS_SIREN: CardRecord = CardRecord::new(
    "Spyglass Siren",
    "41e54343-95e5-4dc4-9f18-e4a415fe5e0a",
    "David Astruga",
    CardRules::new_creature(mana_cost!("{U}"), &["Siren", "Pirate"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, create a Map token. (It's an \
             artifact with \"{1}, {T}, Sacrifice this token: Target \
             creature you control explores. Activate only as a sorcery.\")",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(MAP_TOKEN)).with_count(ValueDef::Constant(1)),
            ),
        ),
    ]),
);

// LCI 79 — Staunch Crewmate
pub(in crate::card::sets) static STAUNCH_CREWMATE: CardRecord = CardRecord::new(
    "Staunch Crewmate",
    "143e9845-8a11-4fc4-b116-29a929985146",
    "L.A. Draws",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Pirate"], 2, 1).with_ability(
        abilities::enters_trigger(
            "When this creature enters, look at the top four cards of your \
             library. You may reveal an artifact or Pirate card from among \
             them and put it into your hand. Put the rest on the bottom of \
             your library in a random order.",
            abilities::look_at_top_cards_reveal_choice_to_hand_rest_random_bottom(
                ValueDef::Constant(4),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Pirate")),
                ]),
                0,
                1,
            ),
        ),
    ),
);

// LCI 80 — Subterranean Schooner
// Audit: unsupported — Needs per-Vehicle crew-payment history identifying the creatures that crewed this incarnation this turn, exposed to target legality.
pub(in crate::card::sets) static SUBTERRANEAN_SCHOONER: CardRecord = CardRecord::new(
    "Subterranean Schooner",
    "94b6881a-b00e-4e90-92e6-602ed8e0e090",
    "Svetlin Velinov",
    CardRules::unsupported(),
);

// LCI 81 — Tishana's Tidebinder (alternate printing)
const TISHANA_S_TIDEBINDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TISHANA_S_TIDEBINDER,
    1,
    "907b3d1d-8c85-4707-80b5-c4d832df9846",
    "Nino Vecia",
);

// LCI 82 — Unlucky Drop
pub(in crate::card::sets) static UNLUCKY_DROP: CardRecord = CardRecord::new(
    "Unlucky Drop",
    "76ebb2df-8891-434e-9cd0-f25b848cb754",
    "Patrik Hell",
    CardRules::new_instant(mana_cost!("{3}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target artifact or creature's owner puts it on their choice \
         of the top or bottom of their library.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Creature),
            ]),
        )],
        EffectDef::ChooseEffect {
            player: EffectRecipientDef::player(PlayerRefDef::OwnerOf(ObjectRefDef::Target(
                TargetIndex::PRIMARY,
            ))),
            choices: &[
                EffectChoiceDef {
                    label: "Top",
                    effect: EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Library,
                        ZonePlacement::Top,
                    ),
                },
                EffectChoiceDef {
                    label: "Bottom",
                    effect: EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Library,
                        ZonePlacement::Bottom,
                    ),
                },
            ],
        },
    )]),
);

// LCI 83 — Waterlogged Hulk // Watertight Gondola
// Audit: unsupported — Needs atomic craft payment selecting and exiling materials across the battlefield and graveyard, together with exiling the source and returning its exact successor transformed. Current activation exile selections cannot span those zones.
pub(in crate::card::sets) static WATERLOGGED_HULK: CardRecord = CardRecord::new(
    "Waterlogged Hulk // Watertight Gondola",
    "7ea89d3c-8d47-4bbb-9271-8cdfd9296e2f",
    "Artur Treffner",
    CardRules::unsupported(),
);

// LCI 84 — Waterwind Scout
pub(in crate::card::sets) static WATERWIND_SCOUT: CardRecord = CardRecord::new(
    "Waterwind Scout",
    "8a7738fb-0a1b-4010-b8c0-e1129739c765",
    "Alix Branwyn",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk", "Scout"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, create a Map token. (It's an \
             artifact with \"{1}, {T}, Sacrifice this token: Target \
             creature you control explores. Activate only as a sorcery.\")",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(MAP_TOKEN)).with_count(ValueDef::Constant(1)),
            ),
        ),
    ]),
);

// LCI 85 — Waylaying Pirates
pub(in crate::card::sets) static WAYLAYING_PIRATES: CardRecord = CardRecord::new(
    "Waylaying Pirates",
    "ded2bb06-7163-47fe-bede-9c192c9b8952",
    "Zoltan Boros",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Human", "Pirate"], 3, 3).with_abilities(&[
        AbilityDef::triggered_if_with_targets(
            "When this creature enters, if you control an artifact, tap \
             target artifact or creature an opponent controls and put a \
             stun counter on it. (If a permanent with a stun counter would \
             become untapped, remove one from it instead.)",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::Stun,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// LCI 86 — Zoetic Glyph
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static ZOETIC_GLYPH: CardRecord = CardRecord::new(
    "Zoetic Glyph",
    "a2f498ac-179e-4055-9b83-97bcc5ab1bb9",
    "Yeong-Hao Han",
    CardRules::unsupported(),
);

// LCI 87 — Abyssal Gorestalker
pub(in crate::card::sets) static ABYSSAL_GORESTALKER: CardRecord = CardRecord::new(
    "Abyssal Gorestalker",
    "a559f77f-1f10-475b-9361-7f297d50f254",
    "Maxime Minard",
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Horror"], 6, 6).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, each player sacrifices two \
             creatures of their choice.",
            EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                player: EffectRecipientDef::EachPlayer,
                zone: ZoneKind::Battlefield,
                candidates: ObjectPredicateDef::HasType(CardType::Creature),
                selection: PerPlayerSelectionDef::Count(ValueDef::Constant(2)),
                chosen: crate::Binding!("sacrificed"),
                unchosen: crate::Binding!("remaining"),
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::sacrifice(EffectRecipientDef::objects(ObjectSetDef::Binding(
                    crate::Binding!("sacrificed"),
                ))),
            }),
        ),
    ]),
);

// LCI 88 — Aclazotz, Deepest Betrayal // Temple of the Dead
// Audit: unsupported — Needs a graveyard-to-battlefield return of the exact died card arriving tapped on its transformed face. Generic battlefield arrival modifications have no transformed-face option; linked-exile returns require a different zone path.
pub(in crate::card::sets) static ACLAZOTZ_DEEPEST_BETRAYAL: CardRecord = CardRecord::new(
    "Aclazotz, Deepest Betrayal // Temple of the Dead",
    "627c392c-4d18-4eb2-a4e8-c668f61f5487",
    "Steve Prescott",
    CardRules::unsupported(),
);

// LCI 89 — Acolyte of Aclazotz
pub(in crate::card::sets) static ACOLYTE_OF_ACLAZOTZ: CardRecord = CardRecord::new(
    "Acolyte of Aclazotz",
    "99009400-ffa4-40af-8305-78295ae605ac",
    "Irina Nordsol",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Vampire", "Cleric"], 1, 4).with_abilities(&[
        AbilityDef::activated(
            "{T}, Sacrifice another creature or artifact: Each opponent \
             loses 1 life and you gain 1 life.",
            &[
                CostDef::TapSource,
                CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Artifact),
                    ]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ])),
            ],
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// LCI 90 — Another Chance
pub(in crate::card::sets) static ANOTHER_CHANCE: CardRecord = CardRecord::new(
    "Another Chance",
    "20d9eb9f-2fcc-49f0-99c1-bad748239466",
    "Irina Nordsol",
    CardRules::new_instant(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::spell(
        "You may mill two cards. Then return up to two creature cards \
         from your graveyard to your hand. (To mill two cards, put the \
         top two cards of your library into your graveyard.)",
        EffectDef::Sequence(&[
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Mill {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            },
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(crate::Binding!("returned")),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: 2,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::move_to_zone(
                    EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!("returned"))),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            }),
        ]),
    )]),
);

// LCI 91 — Bitter Triumph
pub(in crate::card::sets) static BITTER_TRIUMPH: CardRecord = CardRecord::new(
    "Bitter Triumph",
    "05bdd22c-3e11-4c29-bdfa-d3dfc0e90a9f",
    "Donato Giancola",
    // Two mana for unconditional removal at instant speed, and the card or
    // the three life is the whole restriction: it answers anything, and it
    // never answers it for free.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, discard a card or pay 3 life.\nDestroy \
             target creature or planeswalker.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            // One cost with two ways to pay it. The life is the way a deck with an
            // empty hand still casts this, which is what keeps it playable late.
            CostDef::choice(&[
                CostDef::discard(ObjectPredicateDef::Any),
                CostDef::pay_life(CostQuantityDef::Fixed(3)),
            ]),
            EffectDef::destroy_target(TargetIndex::PRIMARY),
        ),
    ),
);

// LCI 92 — Bloodletter of Aclazotz
// Audit: unsupported — Needs a prospective life-loss replacement, gated to this controller's turn, that doubles both damage-caused and non-damage life loss; the current replacement vocabulary covers life gain but not life loss.
pub(in crate::card::sets) static BLOODLETTER_OF_ACLAZOTZ: CardRecord = CardRecord::new(
    "Bloodletter of Aclazotz",
    "d4f6027a-003a-4f9d-929a-0b6da1fa42c9",
    "Antonio José Manzanedo",
    CardRules::unsupported(),
);

// LCI 93 — Bloodthorn Flail
// Audit: unsupported — Needs an activated equip cost with a mana-or-discard alternative; CostDef::Choice is supported for casting but not in the shared activated-cost boundary.
pub(in crate::card::sets) static BLOODTHORN_FLAIL: CardRecord = CardRecord::new(
    "Bloodthorn Flail",
    "db38babd-57e8-4e59-9701-11a0682baa77",
    "Igor Kieryluk",
    CardRules::unsupported(),
);

// LCI 94 — Bringer of the Last Gift
// Audit: unsupported — Needs to subtract the actual graveyard successors of the sacrifice batch from each player's post-sacrifice graveyard, including cards moved there during replacement processing. Frozen pre-sacrifice graveyards miss those arrivals; object sets have no collection-difference operation.
pub(in crate::card::sets) static BRINGER_OF_THE_LAST_GIFT: CardRecord = CardRecord::new(
    "Bringer of the Last Gift",
    "19775c18-4cc0-49d3-86e4-0841768cbf4d",
    "Wero Gallo",
    CardRules::unsupported(),
);

// LCI 95 — Broodrage Mycoid
// Audit: unsupported — Needs turn-scoped history of permanent cards entering this controller's graveyard from any zone (excluding tokens), retained even after those cards leave; current graveyard counts and morbid/revolt history do not represent descending.
pub(in crate::card::sets) static BROODRAGE_MYCOID: CardRecord = CardRecord::new(
    "Broodrage Mycoid",
    "08318a16-a9ed-42c8-9433-876b7a72e368",
    "Domenico Cava",
    CardRules::unsupported(),
);

// LCI 96 — Canonized in Blood
// Audit: unsupported — Needs turn-scoped history of permanent cards entering this controller's graveyard from any zone (excluding tokens), retained even after those cards leave; current graveyard counts and morbid/revolt history do not represent descending.
pub(in crate::card::sets) static CANONIZED_IN_BLOOD: CardRecord = CardRecord::new(
    "Canonized in Blood",
    "384b6892-5dfc-4607-b511-cf83544a9357",
    "Antonio José Manzanedo",
    CardRules::unsupported(),
);

// LCI 97 — Chupacabra Echo
pub(in crate::card::sets) static CHUPACABRA_ECHO: CardRecord = CardRecord::new(
    "Chupacabra Echo",
    "bdd8eed6-91da-4454-a5e3-7bbfac614f47",
    "Izzy",
    CardRules::new_creature(
        mana_cost!("{2}{B}{B}"),
        &["Beast", "Horror", "Spirit"],
        3,
        2,
    )
    .with_abilities(&[abilities::enters_trigger_with_targets(
        "Fathomless descent — When this creature enters, target \
         creature an opponent controls gets -X/-X until end of turn, \
         where X is the number of permanent cards in your graveyard.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &const { [ZoneKind::Battlefield] },
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            },
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Negate(&ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ))),
                ValueDef::Negate(&ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ))),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// LCI 98 — Corpses of the Lost
// Audit: unsupported — Needs turn-scoped history of permanent cards entering this controller's graveyard from any zone (excluding tokens), retained even after those cards leave; current graveyard counts and morbid/revolt history do not represent descending.
pub(in crate::card::sets) static CORPSES_OF_THE_LOST: CardRecord = CardRecord::new(
    "Corpses of the Lost",
    "5f661095-3645-4e44-ac39-752e417c2174",
    "Izzy",
    CardRules::unsupported(),
);

// LCI 99 — Dead Weight (reprint)
const DEAD_WEIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_isd::DEAD_WEIGHT,
    "82e6b971-3f5b-47e7-8209-98d72ee781fc",
    "Javier Charro",
);

// LCI 100 — Deathcap Marionette
pub(in crate::card::sets) static DEATHCAP_MARIONETTE: CardRecord = CardRecord::new(
    "Deathcap Marionette",
    "ccc9d22c-dfce-4136-aed8-a5fe9bb852f2",
    "Svetlin Velinov",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Fungus"], 1, 1).with_abilities(&[
        abilities::deathtouch(),
        abilities::enters_trigger(
            "When this creature enters, you may mill two cards. (You may \
             put the top two cards of your library into your graveyard.)",
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Mill {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            },
        ),
    ]),
);

// LCI 101 — Deep Goblin Skulltaker
// Audit: unsupported — Needs turn-scoped history of permanent cards entering this controller's graveyard from any zone (excluding tokens), retained even after those cards leave; current graveyard counts and morbid/revolt history do not represent descending.
pub(in crate::card::sets) static DEEP_GOBLIN_SKULLTAKER: CardRecord = CardRecord::new(
    "Deep Goblin Skulltaker",
    "0b24bb7c-6b34-48b6-af94-f312ebdbb759",
    "Cristi Balanescu",
    CardRules::unsupported(),
);

// LCI 102 — Deep-Cavern Bat
// Audit: unsupported — Needs an immediate return when the source leaves, without using the stack, for an exile-until duration (CR 610.3). The current linked-exile helper installs a leaves trigger instead, introducing a response window and making the return counterable.
pub(in crate::card::sets) static DEEP_CAVERN_BAT: CardRecord = CardRecord::new(
    "Deep-Cavern Bat",
    "69c68c95-b788-43b1-9f22-1b22c5a00b25",
    "Campbell White",
    CardRules::unsupported(),
);

// LCI 103 — Defossilize
pub(in crate::card::sets) static DEFOSSILIZE: CardRecord = CardRecord::new(
    "Defossilize",
    "3d9abaec-af72-4399-b162-5e62e7487242",
    "Dibujante Nocturno",
    CardRules::new_sorcery(mana_cost!("{4}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return target creature card from your graveyard to the \
         battlefield. That creature explores, then it explores again. \
         (Reveal the top card of your library. Put that card into your \
         hand if it's a land. Otherwise, put a +1/+1 counter on that \
         creature, then put the card back or put it into your \
         graveyard. Then repeat this process.)",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )],
        EffectDef::WithZoneMoveResult {
            effect: &EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
            binding: crate::Binding!("returned"),
            then: &EffectDef::Sequence(&[
                EffectDef::Explore {
                    object: EffectRecipientDef::objects(
                        ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!("returned")),
                    ),
                },
                EffectDef::Explore {
                    object: EffectRecipientDef::objects(
                        ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!("returned")),
                    ),
                },
            ]),
        },
    )]),
);

// LCI 104 — Echo of Dusk
pub(in crate::card::sets) static ECHO_OF_DUSK: CardRecord = CardRecord::new(
    "Echo of Dusk",
    "319a457c-89b7-47f6-a13c-20bb50d41138",
    "Domenico Cava",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire", "Spirit"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "Descend 4 — As long as there are four or more permanent cards \
             in your graveyard, this creature gets +1/+1 and has lifelink.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 4,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::lifelink()),
                    ]),
                },
            },
        ),
    ]),
);

// LCI 105 — Fanatical Offering
pub(in crate::card::sets) static FANATICAL_OFFERING: CardRecord = CardRecord::new(
    "Fanatical Offering",
    "d896dd52-b134-4b55-ab91-ccb05ecc50f4",
    "Raluca Marinescu",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell(
        "As an additional cost to cast this spell, sacrifice an \
         artifact or creature.\nDraw two cards and create a Map token. \
         (It's an artifact with \"{1}, {T}, Sacrifice this token: \
         Target creature you control explores. Activate only as a \
         sorcery.\")",
        EffectDef::Sequence(&[
            abilities::draw_cards(ValueDef::Constant(2)),
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(MAP_TOKEN)).with_count(ValueDef::Constant(1)),
            ),
        ]),
    )
    .with_spell_additional_cost(&CostDef::sacrifice_permanent(ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::HasType(CardType::Artifact),
    ])))]),
);

// LCI 106 — Fungal Fortitude
pub(in crate::card::sets) static FUNGAL_FORTITUDE: CardRecord = CardRecord::new(
    "Fungal Fortitude",
    "9d2bd0ca-521c-45d9-85ed-2f36f583408e",
    "Izzy",
    CardRules::new_enchantment(mana_cost!("{1}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            abilities::dies_trigger_matching(
                "When enchanted creature dies, return it to the battlefield \
                 tapped under its owner's control.",
                ObjectPredicateDef::AttachedToSource,
                EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::TriggeringZoneChangeResult,
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: BattlefieldArrivalDef {
                        modifications: &[BattlefieldEntryModificationDef::Tapped],
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
            ),
        ]),
);

// LCI 107 — Gargantuan Leech
pub(in crate::card::sets) static GARGANTUAN_LEECH: CardRecord = CardRecord::new(
    "Gargantuan Leech",
    "94724efb-4785-4751-9fe1-07f243dd6008",
    "Piotr Foksowicz",
    CardRules::new_creature(mana_cost!("{7}{B}"), &["Leech"], 5, 5).with_abilities(&[
        AbilityDef::static_ability(
            "This spell costs {1} less to cast for each Cave you control \
             and each Cave card in your graveyard.",
            EffectDef::ReduceGenericCostBy(ValueDef::Sum(&SumValueDef::new(
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cave")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cave")),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                )),
            ))),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        abilities::lifelink(),
    ]),
);

// LCI 108 — Grasping Shadows // Shadows' Lair
pub(in crate::card::sets) static GRASPING_SHADOWS: CardRecord = CardRecord::new_dfc(
    "Grasping Shadows // Shadows' Lair",
    "81b8b9c9-725d-476d-a3cf-55e3dc3e433d",
    "Sam Wolfe Connelly",
    &[
        (
            "Grasping Shadows",
            CardRules::new_enchantment(mana_cost!("{3}{B}")).with_abilities(&[
                AbilityDef::triggered(
                    "Whenever a creature you control attacks alone, it gains \
                     deathtouch and lifelink until end of turn. Put a dread \
                     counter on this enchantment. Then if there are three or more \
                     dread counters on it, transform it.",
                    TriggerEventDef::attacks_in_declaration(
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        1,
                        Some(1),
                    ),
                    EffectDef::Sequence(&[
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::TriggeringObject,
                            effect: AppliedEffectDef::Composite(&[
                                AppliedEffectDef::add_ability(&abilities::deathtouch()),
                                AppliedEffectDef::add_ability(&abilities::lifelink()),
                            ]),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::Source,
                            kind: CounterKind::named("dread"),
                            amount: ValueDef::Constant(1),
                        },
                        EffectDef::IfCondition {
                            condition: &TriggerConditionDef::SourceCounters {
                                kind: CounterKind::named("dread"),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 3,
                            },
                            then: &EffectDef::Transform {
                                object: EffectRecipientDef::Source,
                            },
                        },
                    ]),
                ),
            ]),
        ),
        (
            "Shadows' Lair",
            CardRules::new_land(&["Cave"]).with_abilities(&[
                abilities::tap_for(ManaColor::Black),
                AbilityDef::activated(
                    "{B}, {T}, Remove a dread counter from this land: You draw a \
                     card and you lose 1 life.",
                    &[
                        CostDef::Mana(mana_cost!("{B}")),
                        CostDef::TapSource,
                        CostDef::RemoveCountersFromSource {
                            kind: CounterKind::named("dread"),
                            amount: 1,
                        },
                    ],
                    EffectDef::Sequence(&[
                        abilities::draw_cards(ValueDef::Constant(1)),
                        EffectDef::LoseLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(1),
                        },
                    ]),
                ),
            ]),
        ),
    ],
);

// LCI 109 — Greedy Freebooter
pub(in crate::card::sets) static GREEDY_FREEBOOTER: CardRecord = CardRecord::new(
    "Greedy Freebooter",
    "692fe4e8-13b4-4bec-938e-a8073a7fbd71",
    "David Palumbo",
    CardRules::new_creature(mana_cost!("{B}"), &["Human", "Pirate"], 1, 1).with_abilities(&[
        abilities::dies_trigger(
            "When this creature dies, scry 1 and create a Treasure token. \
             (To scry 1, look at the top card of your library. You may put \
             that card on the bottom. A Treasure token is an artifact with \
             \"{T}, Sacrifice this token: Add one mana of any color.\")",
            EffectDef::Sequence(&[
                abilities::scry(ValueDef::Constant(1)),
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ]),
        ),
    ]),
);

// LCI 110 — Join the Dead
pub(in crate::card::sets) static JOIN_THE_DEAD: CardRecord = CardRecord::new(
    "Join the Dead",
    "b5bf6c25-a7d7-40b6-aa19-f852d348967f",
    "Olivier Bernard",
    CardRules::new_instant(mana_cost!("{1}{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature gets -5/-5 until end of turn.\nDescend 4 — \
             That creature gets -10/-10 until end of turn instead if there \
             are four or more permanent cards in your graveyard.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::IfCondition(&ConditionValueDef {
                        condition: &TriggerConditionDef::ObjectCount {
                            query: ObjectQueryDef::matching(
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::HasType(CardType::Enchantment),
                                    ObjectPredicateDef::HasType(CardType::Land),
                                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                                ]),
                                &[ZoneKind::Graveyard],
                                PlayerRelation::You,
                            ),
                            comparison: ComparisonDef::GreaterOrEqual,
                            amount: 4,
                        },
                        then: ValueDef::Constant(-10),
                        otherwise: ValueDef::Constant(-5),
                    }),
                    ValueDef::IfCondition(&ConditionValueDef {
                        condition: &TriggerConditionDef::ObjectCount {
                            query: ObjectQueryDef::matching(
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::HasType(CardType::Enchantment),
                                    ObjectPredicateDef::HasType(CardType::Land),
                                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                                ]),
                                &[ZoneKind::Graveyard],
                                PlayerRelation::You,
                            ),
                            comparison: ComparisonDef::GreaterOrEqual,
                            amount: 4,
                        },
                        then: ValueDef::Constant(-10),
                        otherwise: ValueDef::Constant(-5),
                    }),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LCI 111 — Malicious Eclipse
// Audit: unsupported — Needs a turn-scoped zone-move replacement restricted to creatures moving specifically from the battlefield to a graveyard, including later arrivals. AnyObjectWouldMove has no source-zone filter, and applying ExileInsteadOfDying freezes only currently present creatures.
pub(in crate::card::sets) static MALICIOUS_ECLIPSE: CardRecord = CardRecord::new(
    "Malicious Eclipse",
    "2796fffa-8cbf-4ec9-91a8-7b6f39fd50ec",
    "Campbell White",
    CardRules::unsupported(),
);

// LCI 112 — Mephitic Draught
pub(in crate::card::sets) static MEPHITIC_DRAUGHT: CardRecord = CardRecord::new(
    "Mephitic Draught",
    "adcae196-dbcc-44d5-b700-c57b0b646483",
    "Lauren K. Cannon",
    CardRules::new_artifact(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::triggered(
        "When this artifact enters or is put into a graveyard from the \
         battlefield, you draw a card and you lose 1 life.",
        TriggerEventDef::AnyOf(&[
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
        ]),
        EffectDef::Sequence(&[
            abilities::draw_cards(ValueDef::Constant(1)),
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )]),
);

// LCI 113 — Preacher of the Schism (alternate printing)
const PREACHER_OF_THE_SCHISM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PREACHER_OF_THE_SCHISM,
    1,
    "89345f55-2b32-4356-945a-d56dded39909",
    "Donato Giancola",
);

// LCI 114 — Primordial Gnawer
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static PRIMORDIAL_GNAWER: CardRecord = CardRecord::new(
    "Primordial Gnawer",
    "a18f5ad0-e9c1-4e45-b245-2946e29baecb",
    "Maxime Minard",
    CardRules::unsupported(),
);

// LCI 115 — Queen's Bay Paladin
pub(in crate::card::sets) static QUEEN_S_BAY_PALADIN: CardRecord = CardRecord::new(
    "Queen's Bay Paladin",
    "d0a3339d-6067-4af5-9536-7e2d5ddd8918",
    "Slawomir Maniak",
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Vampire", "Knight"], 5, 4).with_abilities(
        &[AbilityDef::triggered_with_targets(
            "Whenever this creature enters or attacks, return up to one \
             target Vampire card from your graveyard to the battlefield \
             with a finality counter on it. You lose life equal to its \
             mana value. (If a creature with a finality counter on it \
             would die, exile it instead.)",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
            ]),
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vampire")),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
                1,
            )],
            EffectDef::Sequence(&[
                EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: BattlefieldArrivalDef {
                        modifications: &[BattlefieldEntryModificationDef::AddCountersValue {
                            kind: CounterKind::Finality,
                            amount: ValueDef::Constant(1),
                        }],
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::TargetManaValue(TargetIndex::PRIMARY),
                },
            ]),
        )],
    ),
);

// LCI 116 — Rampaging Spiketail
pub(in crate::card::sets) static RAMPAGING_SPIKETAIL: CardRecord = CardRecord::new(
    "Rampaging Spiketail",
    "aad0adcb-80e5-4c6d-bcdd-9e5d18c017b3",
    "John Tedrick",
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Dinosaur"], 5, 6).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, target creature you control gets \
             +2/+0 and gains indestructible until end of turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::indestructible()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::typecycling!(
            "Swampcycling {2}",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Swamp"))
        ),
    ]),
);

// LCI 117 — Ray of Ruin
pub(in crate::card::sets) static RAY_OF_RUIN: CardRecord = CardRecord::new(
    "Ray of Ruin",
    "d440d90e-ac7e-4715-971a-700c977c7fde",
    "Sam Rowan",
    CardRules::new_sorcery(mana_cost!("{4}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Exile target creature, Vehicle, or nonbasic land. Scry 1. \
         (Look at the top card of your library. You may put that card \
         on the bottom.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(CardSupertype::Basic)),
                ]),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
            abilities::scry(ValueDef::Constant(1)),
        ]),
    )]),
);

// LCI 118 — Screaming Phantom
pub(in crate::card::sets) static SCREAMING_PHANTOM: CardRecord = CardRecord::new(
    "Screaming Phantom",
    "bc2f3efb-1526-48c0-842a-374af63ea467",
    "Halil Ural",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Spirit"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature attacks, mill a card. (Put the top \
             card of your library into your graveyard.)",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// LCI 119 — Skullcap Snail
pub(in crate::card::sets) static SKULLCAP_SNAIL: CardRecord = CardRecord::new(
    "Skullcap Snail",
    "0d96d019-5d80-468a-b891-e3e99346372a",
    "Maxime Minard",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Fungus", "Snail"], 1, 1).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, target opponent exiles a card from \
             their hand.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(crate::Binding!("exiled")),
                unchosen: None,
                chooser: PlayerRefDef::Target(TargetIndex::PRIMARY),
                candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Hand],
                    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                )),
                exclude: None,
                minimum: 1,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Private,
                then: &EffectDef::move_to_zone(
                    EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!("exiled"))),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            }),
        ),
    ]),
);

// LCI 120 — Soulcoil Viper
pub(in crate::card::sets) static SOULCOIL_VIPER: CardRecord = CardRecord::new(
    "Soulcoil Viper",
    "1125fdf2-2dbb-49a6-b76f-8cd6c3d6fab4",
    "Allen Douglas",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Snake"], 2, 3).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{B}, {T}, Sacrifice this creature: Return target creature \
             card from your graveyard to the battlefield with a finality \
             counter on it. Activate only as a sorcery. (If a creature \
             with a finality counter on it would die, exile it instead.)",
            &[
                CostDef::Mana(mana_cost!("{B}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::WithBattlefieldArrival {
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Battlefield,
                    ZonePlacement::Top,
                ),
                arrival: BattlefieldArrivalDef {
                    modifications: &[BattlefieldEntryModificationDef::AddCountersValue {
                        kind: CounterKind::Finality,
                        amount: ValueDef::Constant(1),
                    }],
                    ..BattlefieldArrivalDef::DEFAULT
                },
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// LCI 121 — Souls of the Lost
pub(in crate::card::sets) static SOULS_OF_THE_LOST: CardRecord = CardRecord::new(
    "Souls of the Lost",
    "b5c6f502-f11f-4e41-beb8-0843432fa431",
    "Nils Hamm",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Spirit"], 0, 1).with_abilities(&[
        AbilityDef::spell(
            "As an additional cost to cast this spell, discard a card or \
             sacrifice a permanent.",
            EffectDef::None,
        )
        .with_spell_additional_cost(&CostDef::choice(&[
            CostDef::DiscardCards(1),
            CostDef::sacrifice_permanent(ObjectPredicateDef::Any),
        ])),
        AbilityDef::static_ability(
            "Fathomless descent — Souls of the Lost's power is equal to \
             the number of permanent cards in your graveyard and its \
             toughness is equal to that number plus 1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::define_power_toughness(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                    ValueDef::Sum(&SumValueDef::new(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ]),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        )),
                        ValueDef::Constant(1),
                    )),
                ),
            },
        ),
    ]),
);

// LCI 122 — Stalactite Stalker
// Audit: unsupported — Needs turn-scoped history of permanent cards entering this controller's graveyard from any zone (excluding tokens), retained even after those cards leave; current graveyard counts and morbid/revolt history do not represent descending.
pub(in crate::card::sets) static STALACTITE_STALKER: CardRecord = CardRecord::new(
    "Stalactite Stalker",
    "5319c0b1-de54-492a-bdea-85a5a75d693e",
    "Olivier Bernard",
    CardRules::unsupported(),
);

// LCI 123 — Starving Revenant
pub(in crate::card::sets) static STARVING_REVENANT: CardRecord = CardRecord::new(
    "Starving Revenant",
    "f25ea466-eb48-4c4c-b5d4-35f58e46ebe1",
    "Fesbra",
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Spirit", "Horror"], 4, 4).with_abilities(
        &[
            abilities::enters_trigger(
                "When this creature enters, surveil 2. Then for each card you \
                 put on top of your library, you draw a card and you lose 3 \
                 life.",
                EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                    source: ObjectCollectionSourceDef::TopCards {
                        player: PlayerRefDef::EffectController,
                        count: ValueDef::Constant(2),
                    },
                    actor: PlayerRefDef::EffectController,
                    inspection: CollectionInspectionDef::Look,
                    object: ObjectPredicateDef::Any,
                    minimum: 0,
                    maximum: 2,
                    chosen: crate::Binding!("grave"),
                    remainder: crate::Binding!("kept"),
                    then: &EffectDef::Sequence(&[
                        EffectDef::MoveObjects(MoveObjectsDef {
                            input: ObjectSetDef::Binding(crate::Binding!("grave")),
                            from: Some(ZoneKind::Library),
                            zone: ZoneKind::Graveyard,
                            placement: ZonePlacement::Top,
                            moved: None,
                            then: &EffectDef::None,
                        }),
                        EffectDef::ChooseObjectOrder(ChooseObjectOrderDef {
                            actor: PlayerRefDef::EffectController,
                            input: ObjectSetDef::Binding(crate::Binding!("kept")),
                            ordered: crate::Binding!("ordered"),
                            placement: ZonePlacement::Top,
                            visibility: ChoiceVisibilityDef::Private,
                            then: &EffectDef::MoveObjects(MoveObjectsDef {
                                input: ObjectSetDef::Binding(crate::Binding!("ordered")),
                                from: Some(ZoneKind::Library),
                                zone: ZoneKind::Library,
                                placement: ZonePlacement::Top,
                                moved: Some(crate::Binding!("kept_after_order")),
                                then: &EffectDef::ForEachInBinding {
                                    objects: crate::Binding!("kept_after_order"),
                                    binding: crate::Binding!("kept_card"),
                                    effect: &EffectDef::Sequence(&[
                                        EffectDef::DrawCards {
                                            recipient: EffectRecipientDef::player(
                                                PlayerRefDef::OwnerOf(ObjectRefDef::Binding(
                                                    crate::Binding!("kept_card"),
                                                )),
                                            ),
                                            amount: ValueDef::Constant(1),
                                        },
                                        EffectDef::LoseLife {
                                            recipient: EffectRecipientDef::player(
                                                PlayerRefDef::OwnerOf(ObjectRefDef::Binding(
                                                    crate::Binding!("kept_card"),
                                                )),
                                            ),
                                            amount: ValueDef::Constant(3),
                                        },
                                    ]),
                                },
                            }),
                        }),
                    ]),
                }),
            ),
            AbilityDef::triggered_if_with_targets(
                "Descend 8 — Whenever you draw a card, if there are eight or \
                 more permanent cards in your graveyard, target opponent loses \
                 1 life and you gain 1 life.",
                TriggerEventDef::DrewCard(DrawEventMatcherDef::any(PlayerRelation::You)),
                &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 8,
                },
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::Sequence(&[
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
        ],
    ),
);

// LCI 124 — Stinging Cave Crawler
pub(in crate::card::sets) static STINGING_CAVE_CRAWLER: CardRecord = CardRecord::new(
    "Stinging Cave Crawler",
    "6a230208-84ae-4f48-afbd-a0d50596ad27",
    "Dan Murayama Scott",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Insect", "Horror"], 1, 3).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::triggered_if(
            "Descend 4 — Whenever this creature attacks, if there are four \
             or more permanent cards in your graveyard, you draw a card \
             and you lose 1 life.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 4,
            },
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// LCI 125 — Synapse Necromage
pub(in crate::card::sets) static SYNAPSE_NECROMAGE: CardRecord = CardRecord::new(
    "Synapse Necromage",
    "efeb22a4-37bf-487a-9cf4-74de4cbbc0a3",
    "Piotr Foksowicz",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Fungus", "Wizard"], 3, 1).with_abilities(&[
        abilities::dies_trigger(
            "When this creature dies, create two 1/1 black Fungus creature \
             tokens with \"This token can't block.\"",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Fungus"], &[ManaColor::Black], 1, 1)
                        .with_abilities(&[AbilityDef::static_ability(
                            "This token can't block.",
                            EffectDef::StaticApply {
                                recipient: EffectRecipientDef::Source,
                                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                            },
                        )]),
                ))
                .with_count(ValueDef::Constant(2)),
            ),
        ),
    ]),
);

// LCI 126 — Tarrian's Journal // The Tomb of Aclazotz
// Audit: unsupported — Needs a one-use graveyard casting permission whose resulting permanent enters with a finality counter and an additional Vampire subtype. Existing casting permissions do not carry those prospective arrival modifications.
pub(in crate::card::sets) static TARRIAN_S_JOURNAL: CardRecord = CardRecord::new(
    "Tarrian's Journal // The Tomb of Aclazotz",
    "99255a66-b868-45fc-a2a9-0c89bd851b69",
    "Randy Gallegos",
    CardRules::unsupported(),
);

// LCI 127 — Terror Tide
pub(in crate::card::sets) static TERROR_TIDE: CardRecord = CardRecord::new(
    "Terror Tide",
    "0a281d2c-20b9-4887-a924-9be6c07b7629",
    "Brock Grossman",
    CardRules::new_sorcery(mana_cost!("{2}{B}{B}")).with_abilities(&[AbilityDef::spell(
        "Fathomless descent — All creatures get -X/-X until end of \
         turn, where X is the number of permanent cards in your \
         graveyard.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ))),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Negate(&ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ))),
                ValueDef::Negate(&ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ))),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// LCI 128 — Tithing Blade // Consuming Sepulcher
// Audit: unsupported — Needs atomic craft payment selecting and exiling materials across the battlefield and graveyard, together with exiling the source and returning its exact successor transformed. Current activation exile selections cannot span those zones.
pub(in crate::card::sets) static TITHING_BLADE: CardRecord = CardRecord::new(
    "Tithing Blade",
    "dbaa9a2d-e9fd-4746-a26c-f99ae731f024",
    "Michael Walsh",
    CardRules::unsupported(),
);

// LCI 129 — Visage of Dread // Dread Osseosaur
// Audit: unsupported — Needs atomic craft payment selecting and exiling materials across the battlefield and graveyard, together with exiling the source and returning its exact successor transformed. Current activation exile selections cannot span those zones.
pub(in crate::card::sets) static VISAGE_OF_DREAD: CardRecord = CardRecord::new(
    "Visage of Dread // Dread Osseosaur",
    "3d4b61c6-3e88-49f5-9e16-aa8a59653327",
    "David Auden Nash",
    CardRules::unsupported(),
);

// LCI 130 — Vito's Inquisitor
pub(in crate::card::sets) static VITO_S_INQUISITOR: CardRecord = CardRecord::new(
    "Vito's Inquisitor",
    "9030048a-b866-4a89-8d4a-aa55411463e4",
    "Anna Pavleeva",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Vampire", "Knight"], 3, 3).with_abilities(&[
        AbilityDef::activated(
            "{B}, Sacrifice another creature or artifact: Put a +1/+1 \
             counter on this creature. It gains menace until end of turn.",
            &[
                CostDef::Mana(mana_cost!("{B}")),
                CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Artifact),
                    ]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ])),
            ],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::menace()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// LCI 131 — Abrade (reprint)
const ABRADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hou::ABRADE,
    "47f39b5e-2e85-4f31-bbab-0b0bf58f701d",
    "Bartek Fedyczak",
);

// LCI 132 — Ancestors' Aid
pub(in crate::card::sets) static ANCESTORS_AID: CardRecord = CardRecord::new(
    "Ancestors' Aid",
    "e90d5b7d-e27d-42d6-ba7a-d4a6b0dd8549",
    "Alessandra Pisano",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +2/+0 and gains first strike until end \
         of turn.\nCreate a Treasure token. (It's an artifact with \
         \"{T}, Sacrifice this token: Add one mana of any color.\")",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::first_strike()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ]),
    )]),
);

// LCI 133 — Belligerent Yearling
pub(in crate::card::sets) static BELLIGERENT_YEARLING: CardRecord = CardRecord::new(
    "Belligerent Yearling",
    "0b2debca-8535-4cf6-a461-c268faaacaae",
    "Maxime Minard",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Dinosaur"], 3, 2).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "Whenever another Dinosaur you control enters, you may have \
             this creature's base power become equal to that creature's \
             power until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dinosaur")),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::set_base_power(ValueDef::TriggeringObjectPower),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            },
        ),
    ]),
);

// LCI 134 — Bonehoard Dracosaur
pub(in crate::card::sets) static BONEHOARD_DRACOSAUR: CardRecord = CardRecord::new(
    "Bonehoard Dracosaur",
    "2220ed60-3f8f-4dd2-8319-6a06896a5350",
    "Mark Zug",
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Dinosaur", "Dragon"], 5, 5).with_abilities(
        &[
            abilities::flying(),
            abilities::first_strike(),
            AbilityDef::triggered(
                "At the beginning of your upkeep, exile the top two cards of \
                 your library. You may play them this turn. If you exiled a \
                 land card this way, create a 3/1 red Dinosaur creature token. \
                 If you exiled a nonland card this way, create a Treasure \
                 token.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::BindObjects(BindObjectsDef {
                    source: ObjectCollectionSourceDef::TopCards {
                        player: PlayerRefDef::EffectController,
                        count: ValueDef::Constant(2),
                    },
                    binding: crate::Binding!("exiled_top"),
                    then: &EffectDef::Sequence(&[
                        EffectDef::ExileGrantingControllerPlayThisTurn {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("exiled_top"),
                            )),
                        },
                        EffectDef::IfNoObjects(IfNoObjectsDef {
                            input: ObjectSetDef::Matching {
                                objects: &ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                    crate::Binding!("exiled_top"),
                                ),
                                object: ObjectSetFilterDef::Predicate(
                                    &ObjectPredicateDef::HasType(CardType::Land),
                                ),
                            },
                            if_empty: &EffectDef::None,
                            otherwise: &EffectDef::CreateToken(CreateTokenDef::new(
                                TokenDef::Literal(DINOSAUR_TOKEN),
                            )),
                        }),
                        EffectDef::IfNoObjects(IfNoObjectsDef {
                            input: ObjectSetDef::Matching {
                                objects: &ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                    crate::Binding!("exiled_top"),
                                ),
                                object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::Not(
                                    &ObjectPredicateDef::HasType(CardType::Land),
                                )),
                            },
                            if_empty: &EffectDef::None,
                            otherwise: &EffectDef::CreateToken(
                                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                                    .with_count(ValueDef::Constant(1)),
                            ),
                        }),
                    ]),
                }),
            ),
        ],
    ),
);

// LCI 135 — Brass's Tunnel-Grinder // Tecutlan, the Searing Rift
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static BRASS_S_TUNNEL_GRINDER: CardRecord = CardRecord::new(
    "Brass's Tunnel-Grinder // Tecutlan, the Searing Rift",
    "d61d8895-7f2e-4c77-951f-4f1a49e96f57",
    "Cristi Balanescu",
    CardRules::unsupported(),
);

// LCI 136 — Brazen Blademaster
pub(in crate::card::sets) static BRAZEN_BLADEMASTER: CardRecord = CardRecord::new(
    "Brazen Blademaster",
    "8e9e0c66-f64a-428c-be13-a52691833df1",
    "Jarel Threat",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Orc", "Pirate"], 2, 3).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks while you control two or more \
             artifacts, it gets +2/+1 until end of turn.",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 2,
                },
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LCI 137 — Breeches, Eager Pillager
// Audit: unsupported — Needs per-source, turn-scoped modal-choice history shared across separate attack triggers, excluding previously chosen modes while new triggers are put on the stack.
pub(in crate::card::sets) static BREECHES_EAGER_PILLAGER: CardRecord = CardRecord::new(
    "Breeches, Eager Pillager",
    "aadf5028-8dfe-40d3-89b4-22bd7ed0aae6",
    "Josu Hernaiz",
    CardRules::unsupported(),
);

// LCI 138 — Burning Sun Cavalry
pub(in crate::card::sets) static BURNING_SUN_CAVALRY: CardRecord = CardRecord::new(
    "Burning Sun Cavalry",
    "0c491ac0-4752-47a7-967e-456b6e4245de",
    "Josu Hernaiz",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks or blocks while you control a \
             Dinosaur, this creature gets +1/+1 until end of turn.",
            TriggerEventDef::While {
                event: &TriggerEventDef::AnyOf(&[
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                    TriggerEventDef::Blocks {
                        blocked: ObjectPredicateDef::Any,
                    },
                ]),
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dinosaur")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LCI 139 — Calamitous Cave-In
pub(in crate::card::sets) static CALAMITOUS_CAVE_IN: CardRecord = CardRecord::new(
    "Calamitous Cave-In",
    "8341ddd9-aac1-4773-b8ce-51e35f696263",
    "Diego Gisbert",
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_abilities(&[AbilityDef::spell(
        "Calamitous Cave-In deals X damage to each creature and each \
         planeswalker, where X is the number of Caves you control plus \
         the number of Cave cards in your graveyard.",
        EffectDef::damage(
            EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ))),
            ValueDef::Sum(&SumValueDef::new(
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cave")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cave")),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                )),
            )),
        ),
    )]),
);

// LCI 140 — Child of the Volcano
// Audit: unsupported — Needs turn-scoped history of permanent cards entering this controller's graveyard from any zone (excluding tokens), retained even after those cards leave; current graveyard counts and morbid/revolt history do not represent descending.
pub(in crate::card::sets) static CHILD_OF_THE_VOLCANO: CardRecord = CardRecord::new(
    "Child of the Volcano",
    "e964f026-9cbf-4fa2-acdc-60d19b88f183",
    "Uriah Voth",
    CardRules::unsupported(),
);

// LCI 141 — Curator of Sun's Creation
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static CURATOR_OF_SUN_S_CREATION: CardRecord = CardRecord::new(
    "Curator of Sun's Creation",
    "a144d0b3-678d-4f4c-a9b0-22af19f5cf9f",
    "Javier Charro",
    CardRules::unsupported(),
);

// LCI 142 — Daring Discovery
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static DARING_DISCOVERY: CardRecord = CardRecord::new(
    "Daring Discovery",
    "d95018a4-be10-4c46-b14d-4b1eba838bc0",
    "Michele Giorgi",
    CardRules::unsupported(),
);

// LCI 143 — Diamond Pick-Axe
pub(in crate::card::sets) static DIAMOND_PICK_AXE: CardRecord = CardRecord::new(
    "Diamond Pick-Axe",
    "4ae30fa7-3d1d-417f-80d7-a668236cb2c1",
    "Dibujante Nocturno",
    CardRules::new_artifact(mana_cost!("{R}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::indestructible(),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 and has \"Whenever this creature \
                 attacks, create a Treasure token.\" (It's an artifact with \
                 \"{T}, Sacrifice this token: Add one mana of any color.\")",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&AbilityDef::triggered(
                            "Whenever this creature attacks, create a Treasure token.",
                            TriggerEventDef::attacks(ObjectPredicateDef::Source),
                            EffectDef::CreateToken(
                                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                                    .with_count(ValueDef::Constant(1)),
                            ),
                        )),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// LCI 144 — Dinotomaton
pub(in crate::card::sets) static DINOTOMATON: CardRecord = CardRecord::new(
    "Dinotomaton",
    "9de1a93e-ec71-45fa-b0b8-2c21123e390c",
    "Zoltan Boros",
    CardRules::new_artifact_creature(mana_cost!("{3}{R}"), &["Dinosaur", "Gnome"], 4, 3)
        .with_abilities(&[
            abilities::menace(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, target creature you control gains \
                 menace until end of turn.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &const { [ZoneKind::Battlefield] },
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::menace()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// LCI 145 — Dire Flail // Dire Blunderbuss
// Audit: unsupported — Needs atomic craft payment selecting and exiling materials across the battlefield and graveyard, together with exiling the source and returning its exact successor transformed. Current activation exile selections cannot span those zones.
pub(in crate::card::sets) static DIRE_FLAIL: CardRecord = CardRecord::new(
    "Dire Flail // Dire Blunderbuss",
    "0d2d98ae-fe02-4a86-9e80-7b95e08de21c",
    "Anthony Devine",
    CardRules::unsupported(),
);

// LCI 146 — Dowsing Device // Geode Grotto
pub(in crate::card::sets) static DOWSING_DEVICE: CardRecord = CardRecord::new_dfc(
    "Dowsing Device // Geode Grotto",
    "3d715e9f-223d-462e-8ce3-eebbaf1cd021",
    "Olena Richards",
    &[
        (
            "Dowsing Device",
            CardRules::new_artifact(mana_cost!("{1}{R}")).with_abilities(&[
                AbilityDef::triggered_with_targets(
                    "Whenever this artifact or another artifact you control \
                     enters, up to one target creature you control gets +1/+0 and \
                     gains haste until end of turn. Then transform this artifact \
                     if you control four or more artifacts.",
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]),
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    &[AbilityTargetDef {
                        minimum: 0,
                        ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &const { [ZoneKind::Battlefield] },
                            controller: Some(PlayerRelation::You),
                            owner: None,
                        })
                    }],
                    EffectDef::Sequence(&[
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::Composite(&[
                                AppliedEffectDef::modify_power_toughness(
                                    ValueDef::Constant(1),
                                    ValueDef::Constant(0),
                                ),
                                AppliedEffectDef::add_ability(&abilities::haste()),
                            ]),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                        EffectDef::IfCondition {
                            condition: &TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 4,
                            },
                            then: &EffectDef::Transform {
                                object: EffectRecipientDef::Source,
                            },
                        },
                    ]),
                ),
            ]),
        ),
        (
            "Geode Grotto",
            CardRules::new_land(&["Cave"]).with_abilities(&[
                abilities::tap_for(ManaColor::Red),
                AbilityDef::activated_with_targets(
                    "{2}{R}, {T}: Until end of turn, target creature gains haste \
                     and gets +X/+0, where X is the number of artifacts you \
                     control. Activate only as a sorcery.",
                    &[CostDef::Mana(mana_cost!("{2}{R}")), CostDef::TapSource],
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::add_ability(&abilities::haste()),
                            AppliedEffectDef::modify_power_toughness(
                                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                )),
                                ValueDef::Constant(0),
                            ),
                        ]),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                )
                .with_activation_timing(ActivationTimingDef::SorcerySpeed),
            ]),
        ),
    ],
);

// LCI 147 — Dreadmaw's Ire
pub(in crate::card::sets) static DREADMAW_S_IRE: CardRecord = CardRecord::new(
    "Dreadmaw's Ire",
    "062e00a1-f1dc-4089-b640-800ab781c590",
    "Crystal Sully",
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Until end of turn, target attacking creature gets +2/+2 and \
         gains trample and \"Whenever this creature deals combat \
         damage to a player, destroy target artifact that player \
         controls.\"",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Attacking,
            ]),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                AppliedEffectDef::add_ability(&abilities::trample()),
                AppliedEffectDef::add_ability(&AbilityDef::triggered_with_targets(
                    "Whenever this creature deals combat damage to a player, \
                     destroy target artifact that player controls.",
                    TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Artifact),
                            zones: &const { [ZoneKind::Battlefield] },
                            controller: Some(PlayerRelation::EventPlayer),
                            owner: None,
                        },
                    )],
                    EffectDef::destroy_target(TargetIndex::PRIMARY),
                )),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// LCI 148 — Enterprising Scallywag
// Audit: unsupported — Needs turn-scoped history of permanent cards entering this controller's graveyard from any zone (excluding tokens), retained even after those cards leave; current graveyard counts and morbid/revolt history do not represent descending.
pub(in crate::card::sets) static ENTERPRISING_SCALLYWAG: CardRecord = CardRecord::new(
    "Enterprising Scallywag",
    "44420f52-2ed8-4f81-93e4-5decc77bed01",
    "Denman Rooke",
    CardRules::unsupported(),
);

// LCI 149 — Etali's Favor
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static ETALI_S_FAVOR: CardRecord = CardRecord::new(
    "Etali's Favor",
    "6d4075a8-c15d-4078-bf4b-0f85c03fecec",
    "Matt Stewart",
    CardRules::unsupported(),
);

// LCI 150 — Geological Appraiser
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static GEOLOGICAL_APPRAISER: CardRecord = CardRecord::new(
    "Geological Appraiser",
    "7f9c1a82-695b-4df2-8e51-2d71a62e7baf",
    "Alix Branwyn",
    CardRules::unsupported(),
);

// LCI 151 — Goblin Tomb Raider
pub(in crate::card::sets) static GOBLIN_TOMB_RAIDER: CardRecord = CardRecord::new(
    "Goblin Tomb Raider",
    "018160fe-f602-43f5-8495-241a08eaa69c",
    "Kevin Sidharta",
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Pirate"], 1, 2).with_abilities(&[
        AbilityDef::static_ability(
            "As long as you control an artifact, this creature gets +1/+0 \
             and has haste.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                },
            },
        ),
    ]),
);

// LCI 152 — Goldfury Strider
pub(in crate::card::sets) static GOLDFURY_STRIDER: CardRecord = CardRecord::new(
    "Goldfury Strider",
    "415904fe-b77f-4c1a-850f-688484d629e6",
    "José Parodi",
    CardRules::new_artifact_creature(mana_cost!("{4}{R}"), &["Golem"], 3, 5).with_abilities(&[
        abilities::trample(),
        AbilityDef::activated_with_targets(
            "Tap two untapped artifacts and/or creatures you control: \
             Target creature gets +2/+0 until end of turn. Activate only \
             as a sorcery.",
            &[CostDef::TapPermanents {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                controller: PlayerRelation::You,
                count: 2,
            }],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// LCI 153 — Hit the Mother Lode
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static HIT_THE_MOTHER_LODE: CardRecord = CardRecord::new(
    "Hit the Mother Lode",
    "e75f460c-43e2-4353-8b73-71ff8651a79d",
    "Diego Gisbert",
    CardRules::unsupported(),
);

// LCI 154 — Hotfoot Gnome
pub(in crate::card::sets) static HOTFOOT_GNOME: CardRecord = CardRecord::new(
    "Hotfoot Gnome",
    "87468b6b-6f78-42e4-ab0d-6730aeecdc3f",
    "Svetlin Velinov",
    CardRules::new_artifact_creature(mana_cost!("{2}{R}"), &["Gnome"], 3, 1).with_abilities(&[
        abilities::haste(),
        AbilityDef::activated_with_targets(
            "{T}: Another target creature gains haste until end of turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LCI 155 — Idol of the Deep King // Sovereign's Macuahuitl
// Audit: unsupported — Needs atomic craft payment selecting and exiling materials across the battlefield and graveyard, together with exiling the source and returning its exact successor transformed. Current activation exile selections cannot span those zones.
pub(in crate::card::sets) static IDOL_OF_THE_DEEP_KING: CardRecord = CardRecord::new(
    "Idol of the Deep King // Sovereign's Macuahuitl",
    "d1d8d8ef-c8b2-4e7c-89e4-b381dff20584",
    "Matteo Bassini",
    CardRules::unsupported(),
);

// LCI 156 — Inti, Seneschal of the Sun
// Audit: unsupported — Needs a payment-result reflexive trigger that selects the attacking creature only after the optional discard succeeds and survives Inti leaving. The previous declaration chose the target with the attack trigger, changing response and target timing.
pub(in crate::card::sets) static INTI_SENESCHAL_OF_THE_SUN: CardRecord = CardRecord::new(
    "Inti, Seneschal of the Sun",
    "fa7a55aa-ae61-4933-b7a4-dcc55dac6fcd",
    "Victor Adame Minguez",
    CardRules::unsupported(),
);

// LCI 157 — Magmatic Galleon
// Audit: unsupported — Needs a committed noncombat damage batch reporting excess damage to matching creatures and aggregating one trigger for one or more recipients.
pub(in crate::card::sets) static MAGMATIC_GALLEON: CardRecord = CardRecord::new(
    "Magmatic Galleon",
    "4471a833-11b9-4146-a9c0-84a6896c94d8",
    "Cristi Balanescu",
    CardRules::unsupported(),
);

// LCI 158 — Ojer Axonil, Deepest Might // Temple of Power
// Audit: unsupported — Needs a prospective damage replacement raising matching noncombat damage to this source's power, turn history of noncombat damage from red sources controlled at damage time, and tapped transformed return directly from the graveyard.
pub(in crate::card::sets) static OJER_AXONIL_DEEPEST_MIGHT: CardRecord = CardRecord::new(
    "Ojer Axonil, Deepest Might // Temple of Power",
    "50f8e2b6-98c7-4f28-bb39-e1fbe841f1ee",
    "Victor Adame Minguez",
    CardRules::unsupported(),
);

// LCI 159 — Panicked Altisaur
pub(in crate::card::sets) static PANICKED_ALTISAUR: CardRecord = CardRecord::new(
    "Panicked Altisaur",
    "ad371c97-a266-4a88-9d28-2e889a37ba00",
    "Lars Grant-West",
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Dinosaur"], 4, 5).with_abilities(&[
        abilities::reach(),
        AbilityDef::activated(
            "{T}: This creature deals 2 damage to each opponent.",
            &[CostDef::TapSource],
            EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(2)),
        ),
    ]),
);

// LCI 160 — Plundering Pirate
pub(in crate::card::sets) static PLUNDERING_PIRATE: CardRecord = CardRecord::new(
    "Plundering Pirate",
    "5bb2552f-8370-4931-83e1-93706d51413a",
    "Francisco Miyara",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Orc", "Pirate"], 3, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a Treasure token. (It's an \
             artifact with \"{T}, Sacrifice this token: Add one mana of \
             any color.\")",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
    ]),
);

// LCI 161 — Poetic Ingenuity
// Audit: unsupported — Needs the frozen number of matching Dinosaur attackers from a single declaration. AttackDeclared can filter whether a trigger fires, but its captured amount counts all attackers; live battlefield counts change before resolution.
pub(in crate::card::sets) static POETIC_INGENUITY: CardRecord = CardRecord::new(
    "Poetic Ingenuity",
    "c035250e-6f6e-4d0f-b4fd-2a53d6069aa7",
    "Kieran Yanner",
    CardRules::unsupported(),
);

// LCI 162 — Rampaging Ceratops
pub(in crate::card::sets) static RAMPAGING_CERATOPS: CardRecord = CardRecord::new(
    "Rampaging Ceratops",
    "8bab3f8f-cb06-466d-a35d-0b5e1a2b524c",
    "Nicholas Gregory",
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Dinosaur"], 5, 4).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't be blocked except by three or more creatures.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                    BlockRestrictionDef::MinimumBlockers(3),
                )),
            },
        ),
    ]),
);

// LCI 163 — Rumbling Rockslide (reprint)
const RUMBLING_ROCKSLIDE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_iko::RUMBLING_ROCKSLIDE,
    "4f06b53f-ec82-4d7f-bee3-6ca04583f023",
    "Johann Bodin",
);

// LCI 164 — Saheeli's Lattice // Mastercraft Raptor
// Audit: unsupported — Needs atomic craft payment selecting and exiling materials across the battlefield and graveyard, together with exiling the source and returning its exact successor transformed. Current activation exile selections cannot span those zones.
pub(in crate::card::sets) static SAHEELI_S_LATTICE: CardRecord = CardRecord::new(
    "Saheeli's Lattice // Mastercraft Raptor",
    "0bdc79c6-1193-46bd-931d-2c2a0381e420",
    "Zoltan Boros",
    CardRules::unsupported(),
);

// LCI 165 — Scytheclaw Raptor
pub(in crate::card::sets) static SCYTHECLAW_RAPTOR: CardRecord = CardRecord::new(
    "Scytheclaw Raptor",
    "9436cf62-56d6-4662-9982-72e9be80d25c",
    "John Tedrick",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Dinosaur"], 4, 3).with_abilities(&[
        AbilityDef::triggered(
            "Whenever a player casts a spell, if it's not their turn, this \
             creature deals 4 damage to them.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(
                PlayerRelation::NonactivePlayer,
            )),
            EffectDef::damage(
                EffectRecipientDef::ControllerOfTriggeringObject,
                ValueDef::Constant(4),
            ),
        ),
    ]),
);

// LCI 166 — Seismic Monstrosaur
pub(in crate::card::sets) static SEISMIC_MONSTROSAUR: CardRecord = CardRecord::new(
    "Seismic Monstrosaur",
    "98aeb9dc-18f9-4120-ac3d-226c62a1dc1d",
    "John Tedrick",
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Dinosaur"], 6, 5).with_abilities(&[
        abilities::trample(),
        AbilityDef::activated(
            "{2}{R}, Sacrifice a land: Draw a card.",
            &[
                CostDef::Mana(mana_cost!("{2}{R}")),
                CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(CardType::Land)),
            ],
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
        abilities::typecycling!(
            "Mountaincycling {2}",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mountain"))
        ),
    ]),
);

// LCI 167 — Sunfire Torch
// Audit: unsupported — Needs a payment-result reflexive damage trigger that selects its target only after sacrificing the granting Equipment and retains the attacking creature as damage source after the grant disappears.
pub(in crate::card::sets) static SUNFIRE_TORCH: CardRecord = CardRecord::new(
    "Sunfire Torch",
    "bb040fc7-f728-4482-92af-9a320c03bb54",
    "David Szabo",
    CardRules::unsupported(),
);

// LCI 168 — Sunshot Militia
pub(in crate::card::sets) static SUNSHOT_MILITIA: CardRecord = CardRecord::new(
    "Sunshot Militia",
    "d3114f5c-21e9-43c3-abe3-3cf1da20916f",
    "Torgeir Fjereide",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Soldier"], 1, 3).with_abilities(&[
        AbilityDef::activated(
            "Tap two untapped artifacts and/or creatures you control: This \
             creature deals 1 damage to each opponent. Activate only as a \
             sorcery.",
            &[CostDef::TapPermanents {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                controller: PlayerRelation::You,
                count: 2,
            }],
            EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// LCI 169 — Tectonic Hazard
pub(in crate::card::sets) static TECTONIC_HAZARD: CardRecord = CardRecord::new(
    "Tectonic Hazard",
    "5204c781-f568-4c7f-b3f7-ce4dd678689b",
    "Jarel Threat",
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[AbilityDef::spell(
        "Tectonic Hazard deals 1 damage to each opponent and each \
         creature they control.",
        EffectDef::damage(
            EffectRecipientDef::EachOpponentAndTheirCreatures,
            ValueDef::Constant(1),
        ),
    )]),
);

// LCI 170 — Triumphant Chomp
pub(in crate::card::sets) static TRIUMPHANT_CHOMP: CardRecord = CardRecord::new(
    "Triumphant Chomp",
    "06b28139-efa7-4818-a012-cf8150692b43",
    "Simon Dominic",
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Triumphant Chomp deals damage to target creature equal to 2 \
         or the greatest power among Dinosaurs you control, whichever \
         is greater.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::IfElseCondition {
            condition: &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dinosaur")),
                        ObjectPredicateDef::PowerAtLeast(3),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            then: &EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                    objects: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dinosaur")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    select: ObjectValueDef::Power,
                    operation: AggregateOperationDef::Maximum,
                }),
            ),
            otherwise: &EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
        },
    )]),
);

// LCI 171 — Trumpeting Carnosaur
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static TRUMPETING_CARNOSAUR: CardRecord = CardRecord::new(
    "Trumpeting Carnosaur",
    "edc035ca-f0a3-4814-9405-d6dc6f048315",
    "Lars Grant-West",
    CardRules::unsupported(),
);

// LCI 172 — Volatile Wanderglyph
pub(in crate::card::sets) static VOLATILE_WANDERGLYPH: CardRecord = CardRecord::new(
    "Volatile Wanderglyph",
    "2101921a-d26f-4a00-a0db-1b418f2e6b01",
    "Slawomir Maniak",
    CardRules::new_artifact_creature(mana_cost!("{1}{R}"), &["Golem"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature becomes tapped, you may discard a \
             card. If you do, draw a card.",
            TriggerEventDef::tapped(ObjectPredicateDef::Source),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::DiscardCards(1)],
                &abilities::draw_cards(ValueDef::Constant(1)),
            )),
        ),
    ]),
);

// LCI 173 — Zoyowa's Justice
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static ZOYOWA_S_JUSTICE: CardRecord = CardRecord::new(
    "Zoyowa's Justice",
    "04839717-d2f9-481d-9d13-e4038dbcbb0e",
    "Néstor Ossandón Leal",
    CardRules::unsupported(),
);

// LCI 174 — Armored Kincaller
pub(in crate::card::sets) static ARMORED_KINCALLER: CardRecord = CardRecord::new(
    "Armored Kincaller",
    "b76a46a1-a63e-460a-98c5-699dd1c827aa",
    "John Tedrick",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Dinosaur"], 3, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, you may reveal a Dinosaur card \
             from your hand. If you do or if you control another Dinosaur, \
             you gain 3 life.",
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(crate::Binding!("revealed")),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dinosaur")),
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Private,
                then: &EffectDef::IfNoObjects(IfNoObjectsDef {
                    input: ObjectSetDef::Binding(crate::Binding!("revealed")),
                    if_empty: &EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ObjectCount {
                            query: ObjectQueryDef::matching(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dinosaur")),
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                            comparison: ComparisonDef::GreaterOrEqual,
                            amount: 1,
                        },
                        then: &EffectDef::GainLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(3),
                        },
                    },
                    otherwise: &EffectDef::Sequence(&[
                        EffectDef::RevealObjects(RevealObjectsDef {
                            input: ObjectSetDef::Binding(crate::Binding!("revealed")),
                            then: &EffectDef::None,
                        }),
                        EffectDef::GainLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(3),
                        },
                    ]),
                }),
            }),
        ),
    ]),
);

// LCI 175 — Basking Capybara
pub(in crate::card::sets) static BASKING_CAPYBARA: CardRecord = CardRecord::new(
    "Basking Capybara",
    "ff0a2ba4-dfa8-49d6-95e9-04b7a14d0c6c",
    "Ilse Gort",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Capybara"], 1, 3).with_abilities(&[
        AbilityDef::static_ability(
            "Descend 4 — This creature gets +3/+0 as long as there are \
             four or more permanent cards in your graveyard.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 4,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(0),
                    ),
                },
            },
        ),
    ]),
);

// LCI 176 — Bedrock Tortoise
pub(in crate::card::sets) static BEDROCK_TORTOISE: CardRecord = CardRecord::new(
    "Bedrock Tortoise",
    "701bd623-3100-44dc-adec-53fa3a95ab19",
    "Maxime Minard",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Turtle"], 0, 6).with_abilities(&[
        AbilityDef::static_ability(
            "During your turn, creatures you control have hexproof.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                },
            },
        ),
        AbilityDef::static_ability(
            "Each creature you control with toughness greater than its \
             power assigns combat damage equal to its toughness rather \
             than its power.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::ToughnessGreaterThanItsPower,
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::AssignsCombatDamageEqualToToughness),
            },
        ),
    ]),
);

// LCI 177 — Cavern Stomper
pub(in crate::card::sets) static CAVERN_STOMPER: CardRecord = CardRecord::new(
    "Cavern Stomper",
    "decdfd6b-55ab-47fd-9f98-1845261f1caf",
    "David Szabo",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Dinosaur"], 7, 7).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, scry 2. (Look at the top two cards \
             of your library, then put any number of them on the bottom \
             and the rest on top in any order.)",
            abilities::scry(ValueDef::Constant(2)),
        ),
        AbilityDef::activated(
            "{3}{G}: This creature can't be blocked by creatures with \
             power 2 or less this turn.",
            &[CostDef::Mana(mana_cost!("{3}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LCI 178 — Cenote Scout
pub(in crate::card::sets) static CENOTE_SCOUT: CardRecord = CardRecord::new(
    "Cenote Scout",
    "4ce4b7bc-636b-4723-a7c1-2c859f333492",
    "Caroline Gariba",
    CardRules::new_creature(mana_cost!("{G}"), &["Merfolk", "Scout"], 1, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, it explores. (Reveal the top card \
             of your library. Put that card into your hand if it's a land. \
             Otherwise, put a +1/+1 counter on this creature, then put the \
             card back or put it into your graveyard.)",
            EffectDef::Explore {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// LCI 179 — Coati Scavenger
pub(in crate::card::sets) static COATI_SCAVENGER: CardRecord = CardRecord::new(
    "Coati Scavenger",
    "d2c70d86-1764-487d-a415-15ae79ba570c",
    "Alessandra Pisano",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Raccoon"], 3, 2).with_abilities(&[
        AbilityDef::triggered_if_with_targets(
            "Descend 4 — When this creature enters, if there are four or \
             more permanent cards in your graveyard, return target \
             permanent card from your graveyard to your hand.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 4,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// LCI 180 — Colossadactyl
pub(in crate::card::sets) static COLOSSADACTYL: CardRecord = CardRecord::new(
    "Colossadactyl",
    "8702e776-be2c-48a9-9bc5-bb8ea514333b",
    "Campbell White",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Dinosaur"], 4, 5)
        .with_abilities(&[abilities::reach(), abilities::trample()]),
);

// LCI 181 — Cosmium Confluence
// Audit: unsupported — Needs battlefield provenance for a creature-land chosen during resolution, so its persistent haste grant can pass validation. A chosen binding is treated as potentially nonbattlefield, and the catalog rejects permanent ability grants on that binding.
pub(in crate::card::sets) static COSMIUM_CONFLUENCE: CardRecord = CardRecord::new(
    "Cosmium Confluence",
    "490a5054-0607-4e4a-a0a9-0e9eea7adb00",
    "Kasia 'Kafis' Zielińska",
    CardRules::unsupported(),
);

// LCI 182 — Disturbed Slumber
// Audit: unsupported — Needs an attack requirement satisfied by at least one legal blocker. MustBeBlockedBy requires every matching blocker and would incorrectly impose a Lure effect.
pub(in crate::card::sets) static DISTURBED_SLUMBER: CardRecord = CardRecord::new(
    "Disturbed Slumber",
    "4404b8d6-3673-4d82-b9ed-d28e9b54e1f6",
    "David Palumbo",
    CardRules::unsupported(),
);

// LCI 183 — Earthshaker Dreadmaw
pub(in crate::card::sets) static EARTHSHAKER_DREADMAW: CardRecord = CardRecord::new(
    "Earthshaker Dreadmaw",
    "cdcbba6f-aa54-44be-a3b0-f712fa8bd5ad",
    "Jesper Ejsing",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Dinosaur"], 6, 6).with_abilities(&[
        abilities::trample(),
        abilities::enters_trigger(
            "When this creature enters, draw a card for each other \
             Dinosaur you control.",
            abilities::draw_cards(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dinosaur")),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ))),
        ),
    ]),
);

// LCI 184 — Explorer's Cache
pub(in crate::card::sets) static EXPLORER_S_CACHE: CardRecord = CardRecord::new(
    "Explorer's Cache",
    "86b36214-9c7e-4a24-93d4-17b2d00cbc51",
    "Nereida",
    CardRules::new_artifact(mana_cost!("{1}{G}")).with_abilities(&[
        AbilityDef::as_enters(
            "This artifact enters with two +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 2,
                },
            ),
        ),
        abilities::dies_trigger_matching(
            "Whenever a creature you control with a +1/+1 counter on it \
             dies, put a +1/+1 counter on this artifact.",
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::CounterCount {
                        kind: CounterKind::PlusOnePlusOne,
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 1,
                    },
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ]),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_with_targets(
            "{T}: Move a +1/+1 counter from this artifact onto target \
             creature. Activate only as a sorcery.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: &EffectDef::Sequence(&[
                    EffectDef::RemoveCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// LCI 185 — Ghalta, Stampede Tyrant
pub(in crate::card::sets) static GHALTA_STAMPEDE_TYRANT: CardRecord = CardRecord::new(
    "Ghalta, Stampede Tyrant",
    "72e805e9-69be-45c1-aa04-f460641a0c1e",
    "Lars Grant-West",
    CardRules::new_creature(mana_cost!("{5}{G}{G}{G}"), &["Elder", "Dinosaur"], 12, 12)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::trample(),
            abilities::enters_trigger(
                "When Ghalta enters, put any number of creature cards from \
                 your hand onto the battlefield.",
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("creatures")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Hand],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 0,
                    maximum: usize::MAX,
                    visibility: ChoiceVisibilityDef::Private,
                    then: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "creatures"
                        ))),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                }),
            ),
        ]),
);

// LCI 186 — Glimpse the Core
pub(in crate::card::sets) static GLIMPSE_THE_CORE: CardRecord = CardRecord::new(
    "Glimpse the Core",
    "de7d6aed-3dc8-417d-a190-1660cfc8ee4a",
    "Francisco Miyara",
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Search your library for a basic Forest card, put that card \
                 onto the battlefield tapped, then shuffle.",
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Forest")),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: true,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
            AbilityDef::spell_with_targets(
                "Return target Cave card from your graveyard to the \
                 battlefield tapped.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cave")),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                )],
                EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: BattlefieldArrivalDef {
                        modifications: &[BattlefieldEntryModificationDef::Tapped],
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
            ),
        ],
    )]),
);

// LCI 187 — Glowcap Lantern
// Audit: unsupported — Needs granting an executable static ability to the equipped creature, so the library-looking permission follows that creature and can be removed with its abilities. Granted executable statics are outside the catalog boundary.
pub(in crate::card::sets) static GLOWCAP_LANTERN: CardRecord = CardRecord::new(
    "Glowcap Lantern",
    "bafde87c-743d-4307-93e0-fbd30f5d92f6",
    "Irina Nordsol",
    CardRules::unsupported(),
);

// LCI 188 — Growing Rites of Itlimoc // Itlimoc, Cradle of the Sun (reprint)
const GROWING_RITES_OF_ITLIMOC_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_xln::GROWING_RITES_OF_ITLIMOC,
    "004524bf-b249-4dac-9c10-44d57143feb9",
    "Josu Hernaiz",
);

// LCI 189 — Huatli, Poet of Unity // Roar of the Fifth People
// Audit: unsupported — Needs resolution to grant a permanent a static ability that itself grants mana abilities to a changing set of creatures; this nested static-ability grant is outside the shared resolving-ability boundary.
pub(in crate::card::sets) static HUATLI_POET_OF_UNITY: CardRecord = CardRecord::new(
    "Huatli, Poet of Unity // Roar of the Fifth People",
    "57df2563-18d4-4526-a8bc-0c114e6fd4d9",
    "Tyler Jacobson",
    CardRules::unsupported(),
);

// LCI 190 — Huatli's Final Strike
pub(in crate::card::sets) static HUATLI_S_FINAL_STRIKE: CardRecord = CardRecord::new(
    "Huatli's Final Strike",
    "1247a5fa-b50a-4ab1-96ca-dbeb45bd0b56",
    "Marta Nael",
    CardRules::new_instant(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control gets +1/+0 until end of turn. It \
         deals damage equal to its power to target creature an \
         opponent controls.",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &const { [ZoneKind::Battlefield] },
                controller: Some(PlayerRelation::You),
                owner: None,
            }),
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &const { [ZoneKind::Battlefield] },
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            }),
        ],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::damage_from(
                ObjectRefDef::Target(TargetIndex::PRIMARY),
                EffectRecipientDef::Target(TargetIndex(1)),
                ValueDef::TargetPower(TargetIndex::PRIMARY),
            ),
        ]),
    )]),
);

// LCI 191 — Hulking Raptor
pub(in crate::card::sets) static HULKING_RAPTOR: CardRecord = CardRecord::new(
    "Hulking Raptor",
    "45f763af-5a6a-404c-8e8c-4dbed71277bc",
    "Néstor Ossandón Leal",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Dinosaur"], 5, 3).with_abilities(&[
        abilities::ward(&[CostDef::Mana(mana_cost!("{2}"))], "Ward {2}"),
        AbilityDef::triggered(
            "At the beginning of your first main phase, add {G}{G}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::PrecombatMain,
                player: PlayerRelation::You,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green).with_amount(2)),
        ),
    ]),
);

// LCI 192 — In the Presence of Ages
pub(in crate::card::sets) static IN_THE_PRESENCE_OF_AGES: CardRecord = CardRecord::new(
    "In the Presence of Ages",
    "97b1be22-3177-49af-bb06-42497d717c21",
    "Steve Prescott",
    CardRules::new_instant(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::spell(
        "Reveal the top four cards of your library. You may put a \
         creature card and/or a land card from among them into your \
         hand. Put the rest into your graveyard.",
        EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
            source: ObjectCollectionSourceDef::TopCards {
                player: PlayerRefDef::EffectController,
                count: ValueDef::Constant(4),
            },
            actor: PlayerRefDef::EffectController,
            inspection: CollectionInspectionDef::Reveal,
            object: ObjectPredicateDef::HasType(CardType::Creature),
            minimum: 0,
            maximum: 1,
            chosen: crate::Binding!("creature"),
            remainder: crate::Binding!("first_rest"),
            then: &EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Binding(
                    crate::Binding!("first_rest"),
                )),
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::HasType(CardType::Land),
                minimum: 0,
                maximum: 1,
                chosen: crate::Binding!("land"),
                remainder: crate::Binding!("rest"),
                then: &EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Union(&[
                            ObjectSetDef::Binding(crate::Binding!("creature")),
                            ObjectSetDef::Binding(crate::Binding!("land")),
                        ])),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                    EffectDef::MoveObjects(MoveObjectsDef {
                        input: ObjectSetDef::Binding(crate::Binding!("rest")),
                        from: Some(ZoneKind::Library),
                        zone: ZoneKind::Graveyard,
                        placement: ZonePlacement::Top,
                        moved: None,
                        then: &EffectDef::None,
                    }),
                ]),
            }),
        }),
    )]),
);

// LCI 193 — Intrepid Paleontologist
// Audit: unsupported — Needs linked-exile casting permission that installs a finality counter as a prospective battlefield-entry modification on the resulting creature.
pub(in crate::card::sets) static INTREPID_PALEONTOLOGIST: CardRecord = CardRecord::new(
    "Intrepid Paleontologist",
    "871a164a-0fe6-480e-a1be-cbffce884bd3",
    "Irina Nordsol",
    CardRules::unsupported(),
);

// LCI 194 — Ixalli's Lorekeeper
// Audit: unsupported — Needs restricted ability-activation mana and an OR between casting a Dinosaur spell and activating a Dinosaur ability; the shared mana boundary rejects activation restrictions and combines restrictions conjunctively.
pub(in crate::card::sets) static IXALLI_S_LOREKEEPER: CardRecord = CardRecord::new(
    "Ixalli's Lorekeeper",
    "4bc94ded-458d-4458-9c0b-136d825b885d",
    "Ernanda Souza",
    CardRules::unsupported(),
);

// LCI 195 — Jade Seedstones // Jadeheart Attendant
// Audit: unsupported — Needs atomic craft payment selecting and exiling materials across the battlefield and graveyard, together with exiling the source and returning its exact successor transformed. Current activation exile selections cannot span those zones.
pub(in crate::card::sets) static JADE_SEEDSTONES: CardRecord = CardRecord::new(
    "Jade Seedstones // Jadeheart Attendant",
    "bb95ffc2-ef35-49bf-8211-c5354d176051",
    "Alexandre Honoré",
    CardRules::unsupported(),
);

// LCI 196 — Jadelight Spelunker
// Audit: unsupported — Needs a resumable effect loop that executes explore exactly the source spell's chosen X times; fixed sequences and object-collection iteration cannot represent an unbounded scalar repetition.
pub(in crate::card::sets) static JADELIGHT_SPELUNKER: CardRecord = CardRecord::new(
    "Jadelight Spelunker",
    "68e633d3-47e2-48c5-9be3-574ce5023bf7",
    "Izzy",
    CardRules::unsupported(),
);

// LCI 197 — Kaslem's Stonetree // Kaslem's Strider
// Audit: unsupported — Needs atomic craft payment selecting and exiling materials across the battlefield and graveyard, together with exiling the source and returning its exact successor transformed. Current activation exile selections cannot span those zones.
pub(in crate::card::sets) static KASLEM_S_STONETREE: CardRecord = CardRecord::new(
    "Kaslem's Stonetree // Kaslem's Strider",
    "78b1b412-228a-4e05-a4b3-8159ebf54dc6",
    "Victor Adame Minguez",
    CardRules::unsupported(),
);

// LCI 198 — Malamet Battle Glyph
pub(in crate::card::sets) static MALAMET_BATTLE_GLYPH: CardRecord = CardRecord::new(
    "Malamet Battle Glyph",
    "2259f959-ca97-4df9-8d50-0532090fb967",
    "Andrew Mar",
    CardRules::new_sorcery(mana_cost!("{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Choose target creature you control and target creature you \
         don't control. If the creature you control entered this turn, \
         put a +1/+1 counter on it. Then those creatures fight each \
         other.",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &const { [ZoneKind::Battlefield] },
                controller: Some(PlayerRelation::You),
                owner: None,
            }),
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &const { [ZoneKind::Battlefield] },
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            }),
        ],
        EffectDef::Sequence(&[
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::EnteredThisTurn,
                },
                then: &EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            },
            EffectDef::Fight {
                first: ObjectRefDef::Target(TargetIndex::PRIMARY),
                second: ObjectRefDef::Target(TargetIndex(1)),
                excess: None,
            },
        ]),
    )]),
);

// LCI 199 — Malamet Brawler
pub(in crate::card::sets) static MALAMET_BRAWLER: CardRecord = CardRecord::new(
    "Malamet Brawler",
    "47d2fb65-cc3d-4664-bee0-8cf1cba5d8fa",
    "Oriana Menendez",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Cat", "Warrior"], 2, 2).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, target attacking creature \
             gains trample until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::trample()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LCI 200 — Malamet Scythe
pub(in crate::card::sets) static MALAMET_SCYTHE: CardRecord = CardRecord::new(
    "Malamet Scythe",
    "df6776a4-cc01-46a3-90df-04e1e3ba513c",
    "Hristo D. Chukov",
    CardRules::new_artifact(mana_cost!("{2}{G}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enters_trigger_with_targets(
                "When this Equipment enters, attach it to target creature you \
                 control.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &const { [ZoneKind::Battlefield] },
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +2/+2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{4}"))], "Equip {4}"),
        ]),
);

// LCI 201 — Malamet Veteran
pub(in crate::card::sets) static MALAMET_VETERAN: CardRecord = CardRecord::new(
    "Malamet Veteran",
    "4b9b16b3-1cbd-4b63-85b2-e4053dfc1e93",
    "Steve Prescott",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Cat", "Warrior"], 5, 4).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered_if_with_targets(
            "Descend 4 — Whenever this creature attacks, if there are four \
             or more permanent cards in your graveyard, put a +1/+1 \
             counter on target creature.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 4,
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// LCI 202 — Mineshaft Spider
pub(in crate::card::sets) static MINESHAFT_SPIDER: CardRecord = CardRecord::new(
    "Mineshaft Spider",
    "b75a3639-a917-4700-a093-14a4cda78d9f",
    "Sam Rowan",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Spider"], 3, 4).with_abilities(&[
        abilities::reach(),
        abilities::enters_trigger(
            "When this creature enters, you may mill two cards. (You may \
             put the top two cards of your library into your graveyard.)",
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Mill {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            },
        ),
    ]),
);

// LCI 203 — Nurturing Bristleback
pub(in crate::card::sets) static NURTURING_BRISTLEBACK: CardRecord = CardRecord::new(
    "Nurturing Bristleback",
    "081d5c9e-71da-4454-ae4e-c76cb7790d20",
    "John Tedrick",
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Dinosaur"], 5, 5).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a 3/3 green Dinosaur \
             creature token.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Dinosaur"], &[ManaColor::Green], 3, 3),
            ))),
        ),
        abilities::typecycling!(
            "Forestcycling {2}",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Forest"))
        ),
    ]),
);

// LCI 204 — Ojer Kaslem, Deepest Growth // Temple of Cultivation
// Audit: unsupported — Needs a graveyard-to-battlefield return of the exact died card arriving tapped on its transformed face. Generic battlefield arrival modifications have no transformed-face option; linked-exile returns require a different zone path.
pub(in crate::card::sets) static OJER_KASLEM_DEEPEST_GROWTH: CardRecord = CardRecord::new(
    "Ojer Kaslem, Deepest Growth // Temple of Cultivation",
    "0cbc43a3-8cba-4988-9de1-c89aedd79ada",
    "Ryan Pancoast",
    CardRules::unsupported(),
);

// LCI 205 — Over the Edge
pub(in crate::card::sets) static OVER_THE_EDGE: CardRecord = CardRecord::new(
    "Over the Edge",
    "23dddddb-5409-4c28-bf32-e6473f2cc620",
    "Ryan Valle",
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Destroy target artifact or enchantment.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                )],
                EffectDef::destroy_target(TargetIndex::PRIMARY),
            ),
            AbilityDef::spell_with_targets(
                "Target creature you control explores, then it explores again.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &const { [ZoneKind::Battlefield] },
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Sequence(&[
                    EffectDef::Explore {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::Explore {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                ]),
            ),
        ],
    )]),
);

// LCI 206 — Pathfinding Axejaw
pub(in crate::card::sets) static PATHFINDING_AXEJAW: CardRecord = CardRecord::new(
    "Pathfinding Axejaw",
    "9f619075-ca5d-4e09-bd84-31e6b61eaa7e",
    "Raoul Vitale",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Dinosaur"], 4, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, it explores. (Reveal the top card \
             of your library. Put that card into your hand if it's a land. \
             Otherwise, put a +1/+1 counter on this creature, then put the \
             card back or put it into your graveyard.)",
            EffectDef::Explore {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// LCI 207 — Poison Dart Frog
pub(in crate::card::sets) static POISON_DART_FROG: CardRecord = CardRecord::new(
    "Poison Dart Frog",
    "62c1f09a-9d17-415c-8afa-cd0b62abe48d",
    "Julia Metzger",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Frog"], 1, 1).with_abilities(&[
        abilities::reach(),
        abilities::tap_for_mana(
            "{T}: Add one mana of any color.",
            AddManaEffectDef::any_color(),
        ),
        AbilityDef::activated(
            "{2}: This creature gains deathtouch until end of turn.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::deathtouch()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LCI 208 — Pugnacious Hammerskull
pub(in crate::card::sets) static PUGNACIOUS_HAMMERSKULL: CardRecord = CardRecord::new(
    "Pugnacious Hammerskull",
    "632e5635-a9bc-473a-a885-02e1fd258f7b",
    "Kev Walker",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Dinosaur"], 6, 6).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks while you don't control \
             another Dinosaur, put a stun counter on it. (If a permanent \
             with a stun counter would become untapped, remove one from it \
             instead.)",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dinosaur")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::Equal,
                    amount: 0,
                },
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::Stun,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// LCI 209 — River Herald Guide
pub(in crate::card::sets) static RIVER_HERALD_GUIDE: CardRecord = CardRecord::new(
    "River Herald Guide",
    "9ea7e323-f329-4928-b25a-c0b44c5ac058",
    "David Astruga",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Merfolk", "Scout"], 3, 1).with_abilities(&[
        abilities::vigilance(),
        abilities::enters_trigger(
            "When this creature enters, it explores. (Reveal the top card \
             of your library. Put that card into your hand if it's a land. \
             Otherwise, put a +1/+1 counter on this creature, then put the \
             card back or put it into your graveyard.)",
            EffectDef::Explore {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// LCI 210 — Seeker of Sunlight
pub(in crate::card::sets) static SEEKER_OF_SUNLIGHT: CardRecord = CardRecord::new(
    "Seeker of Sunlight",
    "ffcdb10f-4bad-4f89-8e7c-daa5c0c69230",
    "Randy Vargas",
    CardRules::new_creature(mana_cost!("{G}"), &["Merfolk", "Scout"], 1, 1).with_abilities(&[
        AbilityDef::activated(
            "{2}{G}: This creature explores. Activate only as a sorcery. \
             (Reveal the top card of your library. Put that card into your \
             hand if it's a land. Otherwise, put a +1/+1 counter on this \
             creature, then put the card back or put it into your \
             graveyard.)",
            &[CostDef::Mana(mana_cost!("{2}{G}"))],
            EffectDef::Explore {
                object: EffectRecipientDef::Source,
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// LCI 211 — Sentinel of the Nameless City
pub(in crate::card::sets) static SENTINEL_OF_THE_NAMELESS_CITY: CardRecord = CardRecord::new(
    "Sentinel of the Nameless City",
    "eeeffc0b-dc92-458e-ad58-86ff6077a508",
    "Josu Hernaiz",
    // A 3/4 that blocks and attacks in the same turn, and hands you a Map
    // for doing either.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Merfolk", "Warrior", "Scout"], 3, 4)
        .with_abilities(&[
            abilities::vigilance(),
            AbilityDef::triggered(
                "Whenever this creature enters or attacks, create a Map token.",
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                ]),
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(MAP_TOKEN))),
            ),
        ]),
);

// LCI 212 — The Skullspore Nexus
// Audit: unsupported — Needs a frozen simultaneous-death batch with last-known powers for every matching nontoken creature, so the created token uses their total power, plus greatest-power evaluation in self-cost reduction.
pub(in crate::card::sets) static THE_SKULLSPORE_NEXUS: CardRecord = CardRecord::new(
    "The Skullspore Nexus",
    "b56a7631-5f94-468d-aab7-7e9e129c5f49",
    "Daarken",
    CardRules::unsupported(),
);

// LCI 213 — Spelunking
// Audit: unsupported — Needs an enters-untapped replacement that can override another tapped-entry effect. The battlefield-entry modification vocabulary has tapped state and counters but no untapped operation.
pub(in crate::card::sets) static SPELUNKING: CardRecord = CardRecord::new(
    "Spelunking",
    "d3be4257-2316-4a2e-b347-f71c0368a947",
    "Ernanda Souza",
    CardRules::unsupported(),
);

// LCI 214 — Staggering Size
pub(in crate::card::sets) static STAGGERING_SIZE: CardRecord = CardRecord::new(
    "Staggering Size",
    "e34c4946-6c21-4ae1-9595-35933d38da52",
    "Fajareka Setiawan",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +3/+3 and gains trample until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(3),
                ),
                AppliedEffectDef::add_ability(&abilities::trample()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// LCI 215 — Tendril of the Mycotyrant
pub(in crate::card::sets) static TENDRIL_OF_THE_MYCOTYRANT: CardRecord = CardRecord::new(
    "Tendril of the Mycotyrant",
    "afa464fe-978f-43de-ac35-79be4b12f0d9",
    "Maxime Minard",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Fungus", "Wizard"], 2, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{5}{G}{G}: Put seven +1/+1 counters on target noncreature \
             land you control. It becomes a 0/0 Fungus creature with \
             haste. It's still a land.",
            &[CostDef::Mana(mana_cost!("{5}{G}{G}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    ]),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(7),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                            "Fungus",
                        ])),
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(0),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            ]),
        ),
    ]),
);

// LCI 216 — Thrashing Brontodon (reprint)
const THRASHING_BRONTODON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rix::THRASHING_BRONTODON,
    "d52ef7c1-dacb-4204-b64e-5fa3ae3b1ace",
    "Randy Vargas",
);

// LCI 217 — Twists and Turns // Mycoid Maze
// Audit: unsupported — Needs a prospective explore replacement that scries before the original exploration without recursively replacing that same exploration again.
pub(in crate::card::sets) static TWISTS_AND_TURNS: CardRecord = CardRecord::new(
    "Twists and Turns // Mycoid Maze",
    "3cdf691e-96a5-45c7-9b94-6f04af81c8e4",
    "Deruchenko Alexander",
    CardRules::unsupported(),
);

// LCI 218 — Walk with the Ancestors
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static WALK_WITH_THE_ANCESTORS: CardRecord = CardRecord::new(
    "Walk with the Ancestors",
    "71a6f85b-5ef8-4526-9c86-7cb71508b4c0",
    "Loïc Canavaggia",
    CardRules::unsupported(),
);

// LCI 219 — Abuelo, Ancestral Echo
pub(in crate::card::sets) static ABUELO_ANCESTRAL_ECHO: CardRecord = CardRecord::new(
    "Abuelo, Ancestral Echo",
    "40eee689-2514-421a-8056-eb7668be66ff",
    "Victor Adame Minguez",
    CardRules::new_creature(mana_cost!("{1}{W}{U}"), &["Spirit"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::ward(&[CostDef::Mana(mana_cost!("{2}"))], "Ward {2}"),
            AbilityDef::activated_with_targets(
                "{1}{W}{U}: Exile another target creature or artifact you \
                 control. Return it to the battlefield under its owner's \
                 control at the beginning of the next end step.",
                &[CostDef::Mana(mana_cost!("{1}{W}{U}"))],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Artifact),
                            ]),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        zones: &const { [ZoneKind::Battlefield] },
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                abilities::exile_until_next_end_step(EffectRecipientDef::Target(
                    TargetIndex::PRIMARY,
                )),
            ),
        ]),
);

// LCI 220 — Akawalli, the Seething Tower
pub(in crate::card::sets) static AKAWALLI_THE_SEETHING_TOWER: CardRecord = CardRecord::new(
    "Akawalli, the Seething Tower",
    "3ee62dd1-97d0-4e5d-8937-26c9e51e9414",
    "Simon Dominic",
    CardRules::new_creature(mana_cost!("{1}{B}{G}"), &["Fungus"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Descend 4 — As long as there are four or more permanent cards \
                 in your graveyard, Akawalli gets +2/+2 and has trample.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ]),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        ),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 4,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(2),
                                ValueDef::Constant(2),
                            ),
                            AppliedEffectDef::add_ability(&abilities::trample()),
                        ]),
                    },
                },
            ),
            AbilityDef::static_ability(
                "Descend 8 — As long as there are eight or more permanent \
                 cards in your graveyard, Akawalli gets an additional +2/+2 \
                 and can't be blocked by more than one creature.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ]),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        ),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 8,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(2),
                                ValueDef::Constant(2),
                            ),
                            AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                                BlockRestrictionDef::MaximumBlockers(1),
                            )),
                        ]),
                    },
                },
            ),
        ]),
);

// LCI 221 — Amalia Benavides Aguirre
pub(in crate::card::sets) static AMALIA_BENAVIDES_AGUIRRE: CardRecord = CardRecord::new(
    "Amalia Benavides Aguirre",
    "9acf80a5-f2ca-45b4-aca8-fbc690e35401",
    "Alix Branwyn",
    CardRules::new_creature(mana_cost!("{W}{B}"), &["Vampire", "Scout"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::ward(&[CostDef::PayLife(3)], "Ward—Pay 3 life."),
            AbilityDef::triggered(
                "Whenever you gain life, Amalia Benavides Aguirre explores. \
                 Then destroy all other creatures if its power is exactly 20. \
                 (To have this creature explore, reveal the top card of your \
                 library. Put that card into your hand if it's a land. \
                 Otherwise, put a +1/+1 counter on this creature, then put the \
                 card back or put it into your graveyard.)",
                TriggerEventDef::LifeGained(PlayerRelation::You),
                EffectDef::Sequence(&[
                    EffectDef::Explore {
                        object: EffectRecipientDef::Source,
                    },
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::SourceMatches {
                            object: ObjectPredicateDef::PowerExactly(20),
                        },
                        then: &EffectDef::Destroy {
                            object: EffectRecipientDef::objects(ObjectSetDef::Query(
                                ObjectQueryDef::matching(
                                    ObjectPredicateDef::All(&[
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                    ]),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::Any,
                                ),
                            )),
                            then: None,
                        },
                    },
                ]),
            ),
        ]),
);

// LCI 222 — The Ancient One
// Audit: unsupported — Needs a discard-result reflexive trigger carrying the discarded card's mana value and choosing a player only after the discard, retained even if the ability source leaves.
pub(in crate::card::sets) static THE_ANCIENT_ONE: CardRecord = CardRecord::new(
    "The Ancient One",
    "66dd43d7-76a7-46ea-b431-097fcea417af",
    "Victor Adame Minguez",
    CardRules::unsupported(),
);

// LCI 223 — Anim Pakal, Thousandth Moon
pub(in crate::card::sets) static ANIM_PAKAL_THOUSANDTH_MOON: CardRecord = CardRecord::new(
    "Anim Pakal, Thousandth Moon",
    "868856b7-8875-43c1-8249-0f8fb2c8319b",
    "Chris Rahn",
    CardRules::new_creature(mana_cost!("{1}{R}{W}"), &["Human", "Soldier"], 1, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever you attack with one or more non-Gnome creatures, put \
             a +1/+1 counter on Anim Pakal, then create X 1/1 colorless \
             Gnome artifact creature tokens that are tapped and attacking, \
             where X is the number of +1/+1 counters on Anim Pakal.",
            TriggerEventDef::attack_declared(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                            "Gnome",
                        ))),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                1,
                None,
            ),
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(GNOME_TOKEN))
                        .with_count(ValueDef::CountersOnSource(CounterKind::PlusOnePlusOne))
                        .entering_tapped()
                        .entering_attacking(),
                ),
            ]),
        )]),
);

// LCI 224 — Bartolomé del Presidio
pub(in crate::card::sets) static BARTOLOME_DEL_PRESIDIO: CardRecord = CardRecord::new(
    "Bartolomé del Presidio",
    "690ccdc7-6c43-4902-9d11-2f07b7a36b11",
    "Randy Gallegos",
    CardRules::new_creature(mana_cost!("{W}{B}"), &["Vampire", "Knight"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::activated(
            "Sacrifice another creature or artifact: Put a +1/+1 counter \
             on Bartolomé del Presidio.",
            &[CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Artifact),
                ]),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            ]))],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )]),
);

// LCI 225 — The Belligerent
// Audit: unsupported — Needs a resolved, turn-scoped permission to look at and play the top card of its controller's library. Those rules are supported as permanent static abilities but not as temporary player-applied effects.
pub(in crate::card::sets) static THE_BELLIGERENT: CardRecord = CardRecord::new(
    "The Belligerent",
    "1454af8c-bce9-47d3-890f-283e2fea2cf2",
    "Bruce Brenneise",
    CardRules::unsupported(),
);

// LCI 226 — Caparocti Sunborn
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static CAPAROCTI_SUNBORN: CardRecord = CardRecord::new(
    "Caparocti Sunborn",
    "8ea82964-fd9c-48e3-962f-94954476b31f",
    "Donato Giancola",
    CardRules::unsupported(),
);

// LCI 227 — Captain Storm, Cosmium Raider
pub(in crate::card::sets) static CAPTAIN_STORM_COSMIUM_RAIDER: CardRecord = CardRecord::new(
    "Captain Storm, Cosmium Raider",
    "14c65f5a-10bd-4f9b-b816-46c2240b11ff",
    "Diego Gisbert",
    CardRules::new_creature(mana_cost!("{U}{R}"), &["Human", "Pirate"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered_with_targets(
            "Whenever an artifact you control enters, put a +1/+1 counter \
             on target Pirate you control.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Pirate")),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )]),
);

// LCI 228 — Deepfathom Echo
pub(in crate::card::sets) static DEEPFATHOM_ECHO: CardRecord = CardRecord::new(
    "Deepfathom Echo",
    "c7c7fb87-8448-49f4-a9ed-db97f6a41d98",
    "Matt Stewart",
    CardRules::new_creature(mana_cost!("{2}{G}{U}"), &["Merfolk", "Spirit"], 4, 4).with_abilities(
        &[AbilityDef::triggered(
            "At the beginning of combat on your turn, this creature \
             explores. Then you may have it become a copy of another \
             creature you control until end of turn. (To have this \
             creature explore, reveal the top card of your library. Put \
             that card into your hand if it's a land. Otherwise, put a \
             +1/+1 counter on this creature, then put the card back or put \
             it into your graveyard.)",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            EffectDef::Sequence(&[
                EffectDef::Explore {
                    object: EffectRecipientDef::Source,
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Object(crate::Binding!("copied")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 0,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::BecomeCopyOf {
                        object: EffectRecipientDef::object(ObjectRefDef::Binding(crate::Binding!(
                            "copied"
                        ))),
                        copier: None,
                        exceptions: CopyExceptionsDef::NONE,
                        duration: Some(ResolvedEffectDurationDef::UntilEndOfTurn),
                    },
                }),
            ]),
        )],
    ),
);

// LCI 229 — Gishath, Sun's Avatar (reprint)
const GISHATH_SUN_S_AVATAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_xln::GISHATH_SUN_S_AVATAR,
    "bc4a65de-23b5-48f0-b8b7-94608eaced3e",
    "Zack Stella",
);

// LCI 230 — Itzquinth, Firstborn of Gishath
// Audit: unsupported — Needs a payment-result reflexive trigger that selects both creatures only after the optional mana payment, retained if Itzquinth leaves before payment. OptionalEffectTaken currently requires a battlefield source.
pub(in crate::card::sets) static ITZQUINTH_FIRSTBORN_OF_GISHATH: CardRecord = CardRecord::new(
    "Itzquinth, Firstborn of Gishath",
    "7112c366-b36a-4bc8-aa64-6bad16bebc39",
    "Lars Grant-West",
    CardRules::unsupported(),
);

// LCI 231 — Kellan, Daring Traveler // Journey On
fn kellan_composition() -> crate::card::CardComposition {
    let main = KELLAN_DARING_TRAVELER.rules;
    let journey = const {
        CardRules::new_sorcery(mana_cost!("{G}"))
            .with_subtypes(&["Adventure"])
            .with_ability(
                AbilityDef::spell(
                    "Create X Map tokens, where X is one plus the number of \
                     opponents who control an artifact. (Then exile this card. You \
                     may cast the creature later from exile.)",
                    EffectDef::CreateToken(
                        CreateTokenDef::new(TokenDef::Literal(MAP_TOKEN)).with_count(
                            ValueDef::Sum(
                                &const {
                                    SumValueDef::new(
                                        ValueDef::Constant(1),
                                        ValueDef::AnyMatchingObject(
                                            &const {
                                                ObjectQueryDef::matching(
                                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                                    &[ZoneKind::Battlefield],
                                                    PlayerRelation::Opponent,
                                                )
                                            },
                                        ),
                                    )
                                },
                            ),
                        ),
                    ),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            )
    };
    crate::card::CardComposition {
        parts: vec![
            crate::card::CardPart::new(crate::CardPartId::PRIMARY, "Kellan, Daring Traveler", main),
            crate::card::CardPart::new(crate::CardPartId(1), "Journey On", journey),
        ],

        structure: crate::card::CardStructure::AlternateSpell {
            main: crate::CardPartId::PRIMARY,
            alternate: crate::CardPartId(1),
            kind: crate::card::AlternateSpellKind::Adventure,
        },
        play_options: vec![
            crate::card::PlayOptionDef::cast(
                crate::PlayOptionId::DEFAULT,
                "Kellan, Daring Traveler",
                crate::card::SpellForm::Part(crate::CardPartId::PRIMARY),
                mana_cost!("{1}{W}"),
                crate::card::CardEffectStatus::Implemented,
            ),
            crate::card::PlayOptionDef::cast(
                crate::PlayOptionId(1),
                "Journey On",
                crate::card::SpellForm::Part(crate::CardPartId(1)),
                mana_cost!("{G}"),
                crate::card::CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

pub(in crate::card::sets) static KELLAN_DARING_TRAVELER: CardRecord = CardRecord::new(
    "Kellan, Daring Traveler // Journey On",
    "01739030-c280-492b-a5c9-b3e9f6debc6d",
    "Marta Nael",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Faerie", "Scout"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::triggered(
            "Whenever Kellan attacks, reveal the top card of your library. \
             If it's a creature card with mana value 3 or less, put it \
             into your hand. Otherwise, you may put it into your \
             graveyard.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::RevealAndClassifyCards(RevealAndClassifyCardsDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(1),
                },
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ManaValueAtMost(3),
                ]),
                matching: crate::Binding!("small_creature"),
                remainder: crate::Binding!("other"),
                then: &EffectDef::Sequence(&[
                    EffectDef::MoveObjects(MoveObjectsDef {
                        input: ObjectSetDef::Binding(crate::Binding!("small_creature")),
                        from: Some(ZoneKind::Library),
                        zone: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                        moved: None,
                        then: &EffectDef::None,
                    }),
                    EffectDef::May {
                        player: EffectRecipientDef::Controller,
                        effect: &EffectDef::MoveObjects(MoveObjectsDef {
                            input: ObjectSetDef::Binding(crate::Binding!("other")),
                            from: Some(ZoneKind::Library),
                            zone: ZoneKind::Graveyard,
                            placement: ZonePlacement::Top,
                            moved: None,
                            then: &EffectDef::None,
                        }),
                    },
                ]),
            }),
        )),
)
.with_composition(kellan_composition);

// LCI 232 — Kutzil, Malamet Exemplar
// Audit: unsupported — Needs an object predicate comparing effective power with that object's base power before aggregating a combat-damage event.
pub(in crate::card::sets) static KUTZIL_MALAMET_EXEMPLAR: CardRecord = CardRecord::new(
    "Kutzil, Malamet Exemplar",
    "c9f88a40-a6ed-4c1f-a309-011aca1acddd",
    "Marie Magny",
    CardRules::unsupported(),
);

// LCI 233 — Master's Guide-Mural // Master's Manufactory
// Audit: unsupported — Needs atomic craft payment selecting and exiling materials across the battlefield and graveyard, together with exiling the source and returning its exact successor transformed. Current activation exile selections cannot span those zones.
pub(in crate::card::sets) static MASTER_S_GUIDE_MURAL: CardRecord = CardRecord::new(
    "Master's Guide-Mural // Master's Manufactory",
    "f7a41343-7cdb-49aa-a9d1-7460195355d8",
    "Racrufi",
    CardRules::unsupported(),
);

// LCI 234 — Molten Collapse
// Audit: unsupported — Needs turn-scoped history of permanent cards entering this controller's graveyard from any zone (excluding tokens), retained even after those cards leave; current graveyard counts and morbid/revolt history do not represent descending.
pub(in crate::card::sets) static MOLTEN_COLLAPSE: CardRecord = CardRecord::new(
    "Molten Collapse",
    "2487d124-210b-4808-888c-cd0a78aebd90",
    "Kasia 'Kafis' Zielińska",
    CardRules::unsupported(),
);

// LCI 235 — The Mycotyrant
// Audit: unsupported — Needs a per-turn count of permanent cards put into the controller's graveyard from anywhere, excluding tokens and retained after those cards leave the graveyard.
pub(in crate::card::sets) static THE_MYCOTYRANT: CardRecord = CardRecord::new(
    "The Mycotyrant",
    "caef93cc-70d0-4cce-9aaa-13c0931b2ef7",
    "Chase Stone",
    CardRules::unsupported(),
);

// LCI 236 — Nicanzil, Current Conductor
// Audit: unsupported — Needs committed explore events distinguishing land and nonland results and naming the exploring creature's controller.
pub(in crate::card::sets) static NICANZIL_CURRENT_CONDUCTOR: CardRecord = CardRecord::new(
    "Nicanzil, Current Conductor",
    "5e6f4aba-3500-4fb7-ab78-02f63c03778a",
    "Fariba Khamseh",
    CardRules::unsupported(),
);

// LCI 237 — Palani's Hatcher
pub(in crate::card::sets) static PALANI_S_HATCHER: CardRecord = CardRecord::new(
    "Palani's Hatcher",
    "86ff73c7-428c-469c-b564-6aa9f4eeca14",
    "Aaron Miller",
    CardRules::new_creature(mana_cost!("{3}{R}{G}"), &["Dinosaur"], 5, 3).with_abilities(&[
        AbilityDef::static_ability(
            "Other Dinosaurs you control have haste.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dinosaur")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
            },
        ),
        abilities::enters_trigger(
            "When this creature enters, create two 0/1 green Dinosaur Egg \
             creature tokens.",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                    &["Dinosaur", "Egg"],
                    &[ManaColor::Green],
                    0,
                    1,
                )))
                .with_count(ValueDef::Constant(2)),
            ),
        ),
        AbilityDef::triggered_if(
            "At the beginning of combat on your turn, if you control one \
             or more Eggs, sacrifice an Egg, then create a 3/3 green \
             Dinosaur creature token.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Egg")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::Sequence(&[
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("egg")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Egg")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::sacrifice(EffectRecipientDef::objects(
                        ObjectSetDef::Binding(crate::Binding!("egg")),
                    )),
                }),
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(DINOSAUR_TOKEN))),
            ]),
        ),
    ]),
);

// LCI 238 — Quintorius Kand
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static QUINTORIUS_KAND: CardRecord = CardRecord::new(
    "Quintorius Kand",
    "4382fa49-9e34-45b3-8495-4916dcd995ec",
    "Zoltan Boros",
    CardRules::unsupported(),
);

// LCI 239 — Saheeli, the Sun's Brilliance
pub(in crate::card::sets) static SAHEELI_THE_SUN_S_BRILLIANCE: CardRecord = CardRecord::new(
    "Saheeli, the Sun's Brilliance",
    "0ba99b60-c7d0-4041-a065-f2c510745223",
    "Cynthia Sheppard",
    CardRules::new_creature(mana_cost!("{U}{R}"), &["Human", "Artificer"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::activated_with_targets(
            "{U}{R}, {T}: Create a token that's a copy of another target \
             creature or artifact you control, except it's an artifact in \
             addition to its other types. It gains haste. Sacrifice it at \
             the beginning of the next end step.",
            &[CostDef::Mana(mana_cost!("{U}{R}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                        ]),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Copy(&TokenCopyDef {
                    object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    exceptions: CopyExceptionsDef {
                        added_types: CardTypeSet::single(CardType::Artifact),
                        ..CopyExceptionsDef::NONE
                    },
                }))
                .with_created_tokens(CreatedTokensDef {
                    binding: crate::Binding!("copy"),
                    then: &EffectDef::Sequence(&[
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("copy"),
                            )),
                            effect: AppliedEffectDef::add_ability(&abilities::haste()),
                            duration: ResolvedEffectDurationDef::Permanent,
                        },
                        EffectDef::InstallTrigger(InstalledTriggerDef::once(
                            &AbilityDef::triggered(
                                "At the beginning of the next end step, sacrifice it.",
                                TriggerEventDef::StepBegins {
                                    step: TurnStepDef::End,
                                    player: PlayerRelation::Any,
                                },
                                EffectDef::sacrifice(EffectRecipientDef::objects(
                                    ObjectSetDef::Binding(crate::Binding!("copy")),
                                )),
                            ),
                        )),
                    ]),
                }),
            ),
        )]),
);

// LCI 240 — Sovereign Okinec Ahau
// Audit: unsupported — Needs base-power values distinct from effective power, with per-creature subtraction and counter allocation across the creatures controlled on resolution.
pub(in crate::card::sets) static SOVEREIGN_OKINEC_AHAU: CardRecord = CardRecord::new(
    "Sovereign Okinec Ahau",
    "70c75aa7-e2f9-4a69-8086-c982019ca714",
    "Victor Adame Minguez",
    CardRules::unsupported(),
);

// LCI 241 — Squirming Emergence
pub(in crate::card::sets) static SQUIRMING_EMERGENCE: CardRecord = CardRecord::new(
    "Squirming Emergence",
    "8ee16629-f9be-4cdb-bf52-1d640781ee00",
    "Simon Dominic",
    CardRules::new_sorcery(mana_cost!("{1}{B}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Fathomless descent — Return to the battlefield target nonland \
             permanent card in your graveyard with mana value less than or \
             equal to the number of permanent cards in your graveyard.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        ObjectPredicateDef::ManaValueAtMostValue(ValueDef::CountMatchingObjects(
                            &ObjectQueryDef::matching(
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::HasType(CardType::Enchantment),
                                    ObjectPredicateDef::HasType(CardType::Land),
                                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                                ]),
                                &[ZoneKind::Graveyard],
                                PlayerRelation::You,
                            ),
                        )),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// LCI 242 — Uchbenbak, the Great Mistake
pub(in crate::card::sets) static UCHBENBAK_THE_GREAT_MISTAKE: CardRecord = CardRecord::new(
    "Uchbenbak, the Great Mistake",
    "a062202c-f9fb-4dd6-989a-c3083644f1c0",
    "Steven Belledin",
    CardRules::new_creature(mana_cost!("{3}{U}{B}"), &["Skeleton", "Horror"], 6, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::vigilance(),
            abilities::menace(),
            AbilityDef::activated(
                "Descend 8 — {4}{U}{B}: Return this card from your graveyard \
                 to the battlefield with a finality counter on it. Activate \
                 only if there are eight or more permanent cards in your \
                 graveyard and only as a sorcery. (If a creature with a \
                 finality counter on it would die, exile it instead.)",
                &[CostDef::Mana(mana_cost!("{4}{U}{B}"))],
                EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Source,
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: BattlefieldArrivalDef {
                        modifications: &[BattlefieldEntryModificationDef::AddCountersValue {
                            kind: CounterKind::Finality,
                            amount: ValueDef::Constant(1),
                        }],
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed)
            .with_source_zones(&[ZoneKind::Graveyard])
            .with_activation_condition(&TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 8,
            }),
        ]),
);

// LCI 243 — Vito, Fanatic of Aclazotz
// Audit: unsupported — Needs per-incarnation ability-resolution history that persists after Vito leaves the battlefield. SourceResolutionsThisTurn only reads a live permanent, so queued triggers would produce no reward after its departure.
pub(in crate::card::sets) static VITO_FANATIC_OF_ACLAZOTZ: CardRecord = CardRecord::new(
    "Vito, Fanatic of Aclazotz",
    "c4fd9047-df91-4d82-be00-c623acae0f01",
    "Marta Nael",
    CardRules::unsupported(),
);

// LCI 244 — Wail of the Forgotten
pub(in crate::card::sets) static WAIL_OF_THE_FORGOTTEN: CardRecord = CardRecord::new(
    "Wail of the Forgotten",
    "67108256-b7da-44bd-9639-4264931d348f",
    "Ryan Valle",
    CardRules::new_sorcery(mana_cost!("{U}{B}")).with_abilities(&[AbilityDef::modal_spell(
        "Descend 8 — Choose one. If there are eight or more permanent \
         cards in your graveyard as you cast this spell, choose one or \
         more instead.",
        &[
            AbilityDef::spell_with_targets(
                "Return target nonland permanent to its owner's hand.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                )],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
            AbilityDef::spell_with_targets(
                "Target opponent discards a card.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ),
            AbilityDef::spell(
                "Look at the top three cards of your library. Put one of them \
                 into your hand and the rest into your graveyard.",
                abilities::look_at_top_cards_choose_to_hand_rest_graveyard(
                    ValueDef::Constant(3),
                    ObjectPredicateDef::Any,
                    1,
                    1,
                ),
            ),
        ],
    )
    .with_conditional_mode_maximum(
        ConditionDef::ObjectCount(&ObjectCountConditionDef {
            query: ObjectQueryDef::matching(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
                &[ZoneKind::Graveyard],
                PlayerRelation::You,
            ),
            comparison: ComparisonDef::GreaterOrEqual,
            amount: 8,
        }),
        3,
    )]),
);

// LCI 245 — Zoyowa Lava-Tongue
// Audit: unsupported — Needs turn-scoped history of permanent cards entering this controller's graveyard from any zone (excluding tokens), retained even after those cards leave; current graveyard counts and morbid/revolt history do not represent descending.
pub(in crate::card::sets) static ZOYOWA_LAVA_TONGUE: CardRecord = CardRecord::new(
    "Zoyowa Lava-Tongue",
    "c06f0ff2-d42f-4854-bbe5-4b022fb26d7d",
    "Campbell White",
    CardRules::unsupported(),
);

// LCI 246 — Buried Treasure
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static BURIED_TREASURE: CardRecord = CardRecord::new(
    "Buried Treasure",
    "4c9c45b6-dedd-4481-a06a-c83ace2f18fa",
    "Jarel Threat",
    CardRules::unsupported(),
);

// LCI 247 — Careening Mine Cart
pub(in crate::card::sets) static CAREENING_MINE_CART: CardRecord = CardRecord::new(
    "Careening Mine Cart",
    "5a96d8ae-200a-40be-95a8-72b53638a090",
    "Hector Ortiz",
    CardRules::new_vehicle(mana_cost!("{3}"), 3, 3).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this Vehicle attacks, create a Treasure token. (It's \
             an artifact with \"{T}, Sacrifice this token: Add one mana of \
             any color.\")",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
        abilities::crew("Crew 1", 1),
    ]),
);

// LCI 248 — Cartographer's Companion
pub(in crate::card::sets) static CARTOGRAPHER_S_COMPANION: CardRecord = CardRecord::new(
    "Cartographer's Companion",
    "dbd8115e-4cb3-49b6-b74f-8fcfa28c6404",
    "Chuck Lukacs",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Gnome"], 2, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a Map token. (It's an \
             artifact with \"{1}, {T}, Sacrifice this token: Target \
             creature you control explores. Activate only as a sorcery.\")",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(MAP_TOKEN)).with_count(ValueDef::Constant(1)),
            ),
        ),
    ]),
);

// LCI 249 — Chimil, the Inner Sun (reprint)
const CHIMIL_THE_INNER_SUN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lcc::CHIMIL_THE_INNER_SUN,
    "27a1bfb5-ddfc-49cf-baa3-5d1958d2067a",
    "Adam Paquette",
);

// LCI 250 — Compass Gnome
pub(in crate::card::sets) static COMPASS_GNOME: CardRecord = CardRecord::new(
    "Compass Gnome",
    "10e1030e-2b95-4807-9111-e495a2fc8813",
    "David Astruga",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Gnome"], 2, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, you may search your library for a \
             basic land card or Cave card, reveal it, then shuffle and put \
             that card on top.",
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cave")),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Library,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            },
        ),
    ]),
);

// LCI 251 — Contested Game Ball
pub(in crate::card::sets) static CONTESTED_GAME_BALL: CardRecord = CardRecord::new(
    "Contested Game Ball",
    "71cb7776-a6af-4efb-b536-6b9b4f3d3874",
    "Camille Alquier",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you're dealt combat damage, the attacking player \
             gains control of this artifact and untaps it.",
            TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                kind: DamageKindDef::Combat,
                source: DamageSourceMatcherDef::Any,
                recipient: DamageRecipientMatcherDef::Recipients(EffectRecipientDef::Controller),
            }),
            EffectDef::Sequence(&[
                EffectDef::IfElseCondition {
                    condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                    then: &EffectDef::Perform(GameActionDef::GainControl {
                        object: EffectRecipientDef::Source,
                        controller: PlayerRefDef::EffectController,
                        duration: ControlDurationDef::Indefinitely,
                    }),
                    otherwise: &EffectDef::Perform(GameActionDef::GainControl {
                        object: EffectRecipientDef::Source,
                        controller: PlayerRefDef::Opponent,
                        duration: ControlDurationDef::Indefinitely,
                    }),
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Source,
                },
            ]),
        ),
        AbilityDef::activated(
            "{2}, {T}: Draw a card and put a point counter on this \
             artifact. Then if it has five or more point counters on it, \
             sacrifice it and create a Treasure token.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("point"),
                    amount: ValueDef::Constant(1),
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceCounters {
                        kind: CounterKind::named("point"),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 5,
                    },
                    then: &EffectDef::Sequence(&[
                        EffectDef::sacrifice(EffectRecipientDef::Source),
                        EffectDef::CreateToken(
                            CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                                .with_count(ValueDef::Constant(1)),
                        ),
                    ]),
                },
            ]),
        ),
    ]),
);

// LCI 252 — Digsite Conservator
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static DIGSITE_CONSERVATOR: CardRecord = CardRecord::new(
    "Digsite Conservator",
    "dfa3cd5e-b727-479d-9a77-9f320b92f3f2",
    "Racrufi",
    CardRules::unsupported(),
);

// LCI 253 — Disruptor Wanderglyph
pub(in crate::card::sets) static DISRUPTOR_WANDERGLYPH: CardRecord = CardRecord::new(
    "Disruptor Wanderglyph",
    "7802ca61-6fcc-4965-9f8f-dd58fe82c6bf",
    "Izzy",
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Golem"], 3, 4).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, exile target card from an \
             opponent's graveyard.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::Opponent),
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// LCI 254 — Hoverstone Pilgrim
pub(in crate::card::sets) static HOVERSTONE_PILGRIM: CardRecord = CardRecord::new(
    "Hoverstone Pilgrim",
    "af7e12ee-3db6-4df7-9859-7a8dee9171c7",
    "Izzy",
    CardRules::new_artifact_creature(mana_cost!("{5}"), &["Golem"], 2, 5).with_abilities(&[
        abilities::flying(),
        abilities::ward(&[CostDef::Mana(mana_cost!("{2}"))], "Ward {2}"),
        AbilityDef::activated_with_targets(
            "{2}: Put target card from a graveyard on the bottom of its \
             owner's library.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Library,
                ZonePlacement::Bottom,
            ),
        ),
    ]),
);

// LCI 255 — Hunter's Blowgun
pub(in crate::card::sets) static HUNTER_S_BLOWGUN: CardRecord = CardRecord::new(
    "Hunter's Blowgun",
    "3348abe7-6aa3-47f7-8203-a15f75007e33",
    "Hector Ortiz",
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::static_ability(
                "Equipped creature has deathtouch during your turn. Otherwise, \
                 it has reach.",
                EffectDef::IfElseCondition {
                    condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::add_ability(&abilities::deathtouch()),
                    },
                    otherwise: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::add_ability(&abilities::reach()),
                    },
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// LCI 256 — Matzalantli, the Great Door // The Core
pub(in crate::card::sets) static MATZALANTLI_THE_GREAT_DOOR: CardRecord = CardRecord::new_dfc(
    "Matzalantli, the Great Door // The Core",
    "b4c31b29-06ba-436d-a3d9-18f4796c39be",
    "Piotr Dura",
    &[
        (
            "Matzalantli, the Great Door",
            CardRules::new_artifact(mana_cost!("{3}"))
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&[
                    AbilityDef::activated(
                        "{T}: Draw a card, then discard a card.",
                        &[CostDef::TapSource],
                        EffectDef::Sequence(&[
                            abilities::draw_cards(ValueDef::Constant(1)),
                            EffectDef::Discard {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::Constant(1),
                                selection: DiscardSelectionDef::RecipientChooses,
                                then: None,
                            },
                        ]),
                    ),
                    AbilityDef::activated(
                        "{4}, {T}: Transform Matzalantli. Activate only if there are \
                         four or more permanent types among cards in your graveyard. \
                         (Artifact, battle, creature, enchantment, land, and \
                         planeswalker are permanent types.)",
                        &[CostDef::Mana(mana_cost!("{4}")), CostDef::TapSource],
                        EffectDef::Transform {
                            object: EffectRecipientDef::Source,
                        },
                    )
                    .with_activation_condition(&TriggerConditionDef::AnyOf(&[
                        TriggerConditionDef::All(&[
                            TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                            TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                            TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Enchantment),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                            TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Land),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                        ]),
                        TriggerConditionDef::All(&[
                            TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                            TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                            TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Enchantment),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                            TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                        ]),
                        TriggerConditionDef::All(&[
                            TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                            TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                            TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Land),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                            TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                        ]),
                        TriggerConditionDef::All(&[
                            TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                            TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Enchantment),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                            TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Land),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                            TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                        ]),
                        TriggerConditionDef::All(&[
                            TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                            TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Enchantment),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                            TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Land),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                            TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                        ]),
                    ])),
                ]),
        ),
        (
            "The Core",
            CardRules::new_land(&[])
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&[abilities::tap_for_mana(
                    "Fathomless descent — {T}: Add X mana of any one color, where \
                     X is the number of permanent cards in your graveyard.",
                    AddManaEffectDef::any_color().with_variable_amount(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ]),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        )),
                    ),
                )]),
        ),
    ],
);

// LCI 257 — The Millennium Calendar
// Audit: unsupported — Needs an untap-step batch event with the count of permanents actually untapped during that step; ordinary per-permanent untap events do not carry that batch.
pub(in crate::card::sets) static THE_MILLENNIUM_CALENDAR: CardRecord = CardRecord::new(
    "The Millennium Calendar",
    "deabba7f-05ef-41cf-ae3a-d950d051cf1e",
    "Zoltan Boros",
    CardRules::unsupported(),
);

// LCI 258 — Roaming Throne
// Audit: unsupported — Needs duplication of all matching creatures' triggered abilities, not only enters-the-battlefield triggers; the existing additional-trigger rule is limited to entry events.
pub(in crate::card::sets) static ROAMING_THRONE: CardRecord = CardRecord::new(
    "Roaming Throne",
    "32fd8b7c-baf3-4d3d-be6f-044a917b11a0",
    "Cristi Balanescu",
    CardRules::unsupported(),
);

// LCI 259 — Runaway Boulder
pub(in crate::card::sets) static RUNAWAY_BOULDER: CardRecord = CardRecord::new(
    "Runaway Boulder",
    "06d47a3b-dc66-48d8-981b-dde0ccc34ad5",
    "Loïc Canavaggia",
    CardRules::new_artifact(mana_cost!("{6}")).with_abilities(&[
        abilities::flash(),
        abilities::enters_trigger_with_targets(
            "When this artifact enters, it deals 6 damage to target \
             creature an opponent controls.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(6),
            ),
        ),
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
    ]),
);

// LCI 260 — Scampering Surveyor
pub(in crate::card::sets) static SCAMPERING_SURVEYOR: CardRecord = CardRecord::new(
    "Scampering Surveyor",
    "afcb2a90-d4c0-4c83-8ee7-2ac3a23b4402",
    "Xavier Ribeiro",
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Gnome"], 3, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, search your library for a basic \
             land card or Cave card, put it onto the battlefield tapped, \
             then shuffle.",
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cave")),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: true,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
    ]),
);

// LCI 261 — Sorcerous Spyglass (reprint)
const SORCEROUS_SPYGLASS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_xln::SORCEROUS_SPYGLASS,
    "194b7899-6b44-4ecc-8ddc-ec24304eb14c",
    "Tyler Walpole",
);

// LCI 262 — Sunbird Standard // Sunbird Effigy
// Audit: unsupported — Needs atomic craft payment selecting and exiling materials across the battlefield and graveyard, together with exiling the source and returning its exact successor transformed. Current activation exile selections cannot span those zones.
pub(in crate::card::sets) static SUNBIRD_STANDARD: CardRecord = CardRecord::new(
    "Sunbird Standard // Sunbird Effigy",
    "e0b6d40a-fded-4625-b03c-765c88d75766",
    "Aldo Domínguez",
    CardRules::unsupported(),
);

// LCI 263 — Swashbuckler's Whip
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static SWASHBUCKLER_S_WHIP: CardRecord = CardRecord::new(
    "Swashbuckler's Whip",
    "24a85c52-e5b5-4d65-931c-eacc0cf0fb31",
    "Irina Nordsol",
    CardRules::unsupported(),
);

// LCI 264 — Tarrian's Soulcleaver
pub(in crate::card::sets) static TARRIAN_S_SOULCLEAVER: CardRecord = CardRecord::new(
    "Tarrian's Soulcleaver",
    "99413817-1218-4947-b12f-9a952b095a89",
    "Nereida",
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has vigilance.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::vigilance()),
                },
            ),
            abilities::dies_trigger_matching(
                "Whenever another artifact or creature is put into a graveyard \
                 from the battlefield, put a +1/+1 counter on equipped \
                 creature.",
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::AttachedPermanent,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// LCI 265 — Threefold Thunderhulk
pub(in crate::card::sets) static THREEFOLD_THUNDERHULK: CardRecord = CardRecord::new(
    "Threefold Thunderhulk",
    "1917c62f-d463-43eb-87ad-89ffbc88b6fe",
    "Xavier Ribeiro",
    CardRules::new_artifact_creature(mana_cost!("{7}"), &["Gnome"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with three +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 3,
                },
            ),
        ),
        AbilityDef::triggered(
            "Whenever this creature enters or attacks, create a number of \
             1/1 colorless Gnome artifact creature tokens equal to its \
             power.",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
            ]),
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(GNOME_TOKEN))
                    .with_count(ValueDef::SourcePower),
            ),
        ),
        AbilityDef::activated(
            "{2}, Sacrifice another artifact: Put a +1/+1 counter on this \
             creature.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ])),
            ],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// LCI 266 — Throne of the Grim Captain // The Grim Captain
// Audit: unsupported — Needs atomic craft payment selecting and exiling materials across the battlefield and graveyard, together with exiling the source and returning its exact successor transformed. Current activation exile selections cannot span those zones.
pub(in crate::card::sets) static THRONE_OF_THE_GRIM_CAPTAIN: CardRecord = CardRecord::new(
    "Throne of the Grim Captain // The Grim Captain",
    "c13e8e3d-2a6b-4782-a3c9-71af7336a881",
    "Tiffany Turrill",
    CardRules::unsupported(),
);

// LCI 267 — Treasure Map // Treasure Cove (reprint)
const TREASURE_MAP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_xln::TREASURE_MAP,
    "a924fe1e-a85e-4e14-88d2-ac55130638ab",
    "Néstor Ossandón Leal",
);

// LCI 268 — Captivating Cave
pub(in crate::card::sets) static CAPTIVATING_CAVE: CardRecord = CardRecord::new(
    "Captivating Cave",
    "1d1a645e-85c7-4044-b817-6e24744d245e",
    "Lorenzo Lanfranconi",
    CardRules::new_land(&["Cave"]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
        AbilityDef::activated_with_targets(
            "{4}, {T}, Sacrifice this land: Put two +1/+1 counters on \
             target creature. Activate only as a sorcery.",
            &[
                CostDef::Mana(mana_cost!("{4}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(2),
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// LCI 269 — Cavern of Souls (reprint)
const CAVERN_OF_SOULS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_avr::CAVERN_OF_SOULS,
    "3aad15a2-8a1b-4460-9b06-e85863081878",
    "Alayna Danner",
);

// LCI 270 — Cavernous Maw
pub(in crate::card::sets) static CAVERNOUS_MAW: CardRecord = CardRecord::new(
    "Cavernous Maw",
    "2a51ebf6-a465-42e2-82b7-d2cb928ca632",
    "Alfven Ato",
    CardRules::new_land(&["Cave"]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{2}: This land becomes a 3/3 Elemental creature until end of \
             turn. It's still a Cave land. Activate only if the number of \
             other Caves you control plus the number of Cave cards in your \
             graveyard is three or greater.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Elemental"])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(3),
                    ),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .with_activation_condition(&TriggerConditionDef::ValueComparison(
            &ValueComparisonDef {
                left: ValueDef::Sum(&SumValueDef::new(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cave")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cave")),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                )),
                comparison: ComparisonDef::GreaterOrEqual,
                right: ValueDef::Constant(3),
            },
        )),
    ]),
);

// LCI 271 — Echoing Deeps
// Audit: unsupported — Needs an as-enters copy choice over land cards in graveyards with an added Cave land subtype as a copiable exception; current copy exceptions only add creature subtypes.
pub(in crate::card::sets) static ECHOING_DEEPS: CardRecord = CardRecord::new(
    "Echoing Deeps",
    "244c06b3-532d-426e-8bee-ee9461d092a6",
    "Mauricio Calle",
    CardRules::unsupported(),
);

// LCI 272 — Forgotten Monument
pub(in crate::card::sets) static FORGOTTEN_MONUMENT: CardRecord = CardRecord::new(
    "Forgotten Monument",
    "de8c1c02-e533-46b2-a3eb-91dff561854b",
    "Logan Feliciano",
    CardRules::new_land(&["Cave"]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::static_ability(
            "Other Caves you control have \"{T}, Pay 1 life: Add one mana \
             of any color.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cave")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&AbilityDef::activated_mana(
                    "{T}, Pay 1 life: Add one mana of any color.",
                    &[CostDef::TapSource, CostDef::PayLife(1)],
                    EffectDef::AddMana(AddManaEffectDef::any_color()),
                )),
            },
        ),
    ]),
);

// LCI 273 — Hidden Cataract
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static HIDDEN_CATARACT: CardRecord = CardRecord::new(
    "Hidden Cataract",
    "69f317fc-f603-45b5-9208-545be4dcbf36",
    "Josu Solano",
    CardRules::unsupported(),
);

// LCI 274 — Hidden Courtyard
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static HIDDEN_COURTYARD: CardRecord = CardRecord::new(
    "Hidden Courtyard",
    "b8685d46-99fc-44b3-be95-707a4b7b8327",
    "Josu Solano",
    CardRules::unsupported(),
);

// LCI 275 — Hidden Necropolis
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static HIDDEN_NECROPOLIS: CardRecord = CardRecord::new(
    "Hidden Necropolis",
    "f67fd04f-05da-4418-97de-abeb7346cc69",
    "Svetlin Velinov",
    CardRules::unsupported(),
);

// LCI 276 — Hidden Nursery
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static HIDDEN_NURSERY: CardRecord = CardRecord::new(
    "Hidden Nursery",
    "a942939a-c06e-4b90-a404-ae5acfffcff9",
    "Álvaro Calvo Escudero",
    CardRules::unsupported(),
);

// LCI 277 — Hidden Volcano
// Audit: unsupported — Needs a discover cast offer that bounds the mana value of the spell actually cast, returns an uncast discovered card to hand, and randomly bottoms the other exiled cards. Cascade is source-mana-value based and generic free-cast permissions cannot enforce the discovered spell limit.
pub(in crate::card::sets) static HIDDEN_VOLCANO: CardRecord = CardRecord::new(
    "Hidden Volcano",
    "9fa06aed-52c1-48f1-9906-362db12a3cf7",
    "Logan Feliciano",
    CardRules::unsupported(),
);

// LCI 278 — Pit of Offerings
pub(in crate::card::sets) static PIT_OF_OFFERINGS: CardRecord = CardRecord::new(
    "Pit of Offerings",
    "bc7d3957-b483-4a1f-a244-293c90032f5e",
    "Martin de Diego Sádaba",
    CardRules::new_land(&["Cave"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger_with_targets(
            "When this land enters, exile up to three target cards from \
             graveyards.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
                3,
            )],
            EffectDef::ExileLinkedToSource {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                face_down: false,
                until_source_leaves: false,
                then: None,
            },
        ),
        abilities::tap_for(ManaColor::Colorless),
        abilities::tap_for_mana(
            "{T}: Add one mana of any of the exiled cards' colors.",
            AddManaEffectDef {
                mana: ManaSelectionDef::ColorsOfLinkedExiles,
                ..AddManaEffectDef::one(ManaColor::Colorless)
            },
        ),
    ]),
);

// LCI 279 — Promising Vein
pub(in crate::card::sets) static PROMISING_VEIN: CardRecord = CardRecord::new(
    "Promising Vein",
    "e9681a54-6413-4ff4-b6b1-ee4decb25bfa",
    "Julian Kok Joon Wen",
    CardRules::new_land(&["Cave"]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{1}, {T}, Sacrifice this land: Search your library for a \
             basic land card, put it onto the battlefield tapped, then \
             shuffle.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: true,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
    ]),
);

// LCI 280 — Restless Anchorage
pub(in crate::card::sets) static RESTLESS_ANCHORAGE: CardRecord = CardRecord::new(
    "Restless Anchorage",
    "e3cab352-734e-454b-8b58-733165f4b3b3",
    "Leon Tukker",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for_mana(
            "{T}: Add {W} or {U}.",
            AddManaEffectDef::choice(&[ManaColor::White, ManaColor::Blue]),
        ),
        AbilityDef::activated(
            "{1}{W}{U}: Until end of turn, this land becomes a 2/3 white \
             and blue Bird creature with flying. It's still a land.",
            &[CostDef::Mana(mana_cost!("{1}{W}{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::set_colors(ColorSet::from_colors(&[
                        ManaColor::White,
                        ManaColor::Blue,
                    ])),
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Bird"])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(3),
                    ),
                    AppliedEffectDef::add_ability(&abilities::flying()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::triggered(
            "Whenever this land attacks, create a Map token.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(MAP_TOKEN)).with_count(ValueDef::Constant(1)),
            ),
        ),
    ]),
);

// LCI 281 — Restless Prairie
pub(in crate::card::sets) static RESTLESS_PRAIRIE: CardRecord = CardRecord::new(
    "Restless Prairie",
    "f94ef116-6aff-4f53-a7f9-be5e21c7afa4",
    "Randy Gallegos",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for_mana(
            "{T}: Add {G} or {W}.",
            AddManaEffectDef::choice(&[ManaColor::Green, ManaColor::White]),
        ),
        AbilityDef::activated(
            "{2}{G}{W}: This land becomes a 3/3 green and white Llama \
             creature until end of turn. It's still a land.",
            &[CostDef::Mana(mana_cost!("{2}{G}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::set_colors(ColorSet::from_colors(&[
                        ManaColor::Green,
                        ManaColor::White,
                    ])),
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Llama"])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(3),
                    ),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::triggered(
            "Whenever this land attacks, other creatures you control get \
             +1/+1 until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LCI 282 — Restless Reef
pub(in crate::card::sets) static RESTLESS_REEF: CardRecord = CardRecord::new(
    "Restless Reef",
    "8a8121c9-2480-419c-aa9c-5b8b55f65014",
    "Hristo D. Chukov",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for_mana(
            "{T}: Add {U} or {B}.",
            AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Black]),
        ),
        AbilityDef::activated(
            "{2}{U}{B}: Until end of turn, this land becomes a 4/4 blue \
             and black Shark creature with deathtouch. It's still a land.",
            &[CostDef::Mana(mana_cost!("{2}{U}{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::set_colors(ColorSet::from_colors(&[
                        ManaColor::Blue,
                        ManaColor::Black,
                    ])),
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Shark"])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(4),
                        ValueDef::Constant(4),
                    ),
                    AppliedEffectDef::add_ability(&abilities::deathtouch()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::triggered_with_targets(
            "Whenever this land attacks, target player mills four cards.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
        ),
    ]),
);

// LCI 283 — Restless Ridgeline
pub(in crate::card::sets) static RESTLESS_RIDGELINE: CardRecord = CardRecord::new(
    "Restless Ridgeline",
    "abde5bed-dc4e-4b2b-820c-18d4d0cf8042",
    "Álvaro Calvo Escudero",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for_mana(
            "{T}: Add {R} or {G}.",
            AddManaEffectDef::choice(&[ManaColor::Red, ManaColor::Green]),
        ),
        AbilityDef::activated(
            "{2}{R}{G}: This land becomes a 3/4 red and green Dinosaur \
             creature until end of turn. It's still a land.",
            &[CostDef::Mana(mana_cost!("{2}{R}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::set_colors(ColorSet::from_colors(&[
                        ManaColor::Red,
                        ManaColor::Green,
                    ])),
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Dinosaur"])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(4),
                    ),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::triggered_with_targets(
            "Whenever this land attacks, another target attacking creature \
             gets +2/+0 until end of turn. Untap that creature.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ObjectPredicateDef::Attacking,
                ]),
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ]),
        ),
    ]),
);

// LCI 284 — Restless Vents
pub(in crate::card::sets) static RESTLESS_VENTS: CardRecord = CardRecord::new(
    "Restless Vents",
    "e628e89b-bee9-408d-bb05-1784fda6b8a1",
    "Svetlin Velinov",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for_mana(
            "{T}: Add {B} or {R}.",
            AddManaEffectDef::choice(&[ManaColor::Black, ManaColor::Red]),
        ),
        AbilityDef::activated(
            "{1}{B}{R}: Until end of turn, this land becomes a 2/3 black \
             and red Insect creature with menace. It's still a land.",
            &[CostDef::Mana(mana_cost!("{1}{B}{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::set_colors(ColorSet::from_colors(&[
                        ManaColor::Black,
                        ManaColor::Red,
                    ])),
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Insect"])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(3),
                    ),
                    AppliedEffectDef::add_ability(&abilities::menace()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::triggered(
            "Whenever this land attacks, you may discard a card. If you \
             do, draw a card.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::DiscardCards(1)],
                &abilities::draw_cards(ValueDef::Constant(1)),
            )),
        ),
    ]),
);

// LCI 285 — Sunken Citadel
// Audit: unsupported — Needs mana restricted to activating abilities of land sources; ActivateAbility restrictions remain outside the shared mana execution boundary.
pub(in crate::card::sets) static SUNKEN_CITADEL: CardRecord = CardRecord::new(
    "Sunken Citadel",
    "3e1c9b1a-e306-47bb-9f68-2083660319c0",
    "Matteo Bassini",
    CardRules::unsupported(),
);

// LCI 286 — Volatile Fault
pub(in crate::card::sets) static VOLATILE_FAULT: CardRecord = CardRecord::new(
    "Volatile Fault",
    "9385abf3-b067-4586-bf3d-175526cf8f0a",
    "Andrew Mar",
    CardRules::new_land(&["Cave"]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{1}, {T}, Sacrifice this land: Destroy target nonbasic land \
             an opponent controls. That player may search their library \
             for a basic land card, put it onto the battlefield, then \
             shuffle. You create a Treasure token.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                            CardSupertype::Basic,
                        )),
                    ]),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::destroy_target(TargetIndex::PRIMARY),
                EffectDef::May {
                    player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::Target(TargetIndex::PRIMARY),
                    )),
                    effect: &EffectDef::SearchZone {
                        player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                            ObjectRefDef::Target(TargetIndex::PRIMARY),
                        )),
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ]),
                        minimum: 0,
                        maximum: ValueDef::Constant(1),
                        reveal: true,
                        destination: ZoneKind::Battlefield,
                        placement: ZonePlacement::Top,
                        shuffle: true,
                        enters_tapped: false,
                        attachment: None,
                        binding: None,
                        then: None,
                    },
                },
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ]),
        ),
    ]),
);

// LCI 287 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "b5453630-bfff-4403-a9a9-49f1534e1d42",
    "Olga Tereshenko",
);

// LCI 288 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "338e5b63-1fee-4a7c-af9b-483d383f79b7",
    "WFlemming Illustration",
);

// LCI 289 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "a825ac86-d642-42fd-b6aa-94aa804907d9",
    "Elektrodeko",
);

// LCI 290 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "7cb82fdb-5090-45c0-ae67-4846667c8625",
    "BEMOCS",
);

// LCI 291 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "8c13cafb-3078-4856-a5b0-c38aace8a34a",
    "Matteo Bassini",
);

// LCI 292 — Akal Pakal, First Among Equals (alternate printing)
const AKAL_PAKAL_FIRST_AMONG_EQUALS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AKAL_PAKAL_FIRST_AMONG_EQUALS,
    1,
    "acaf4dbe-5b63-4948-bc85-b5d288378d4e",
    "Alex Negrea",
);

// LCI 293 — Malcolm, Alluring Scoundrel (alternate printing)
const MALCOLM_ALLURING_SCOUNDREL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MALCOLM_ALLURING_SCOUNDREL,
    1,
    "b3ed6341-59f4-44f3-99c9-80f636fa484d",
    "Rafal Wechterowicz (Too Many Skulls)",
);

// LCI 294 — Breeches, Eager Pillager (alternate printing)
const BREECHES_EAGER_PILLAGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BREECHES_EAGER_PILLAGER,
    1,
    "5cb60b23-2397-4897-9462-56b09f45f321",
    "Rafal Wechterowicz (Too Many Skulls)",
);

// LCI 295 — Inti, Seneschal of the Sun (alternate printing)
const INTI_SENESCHAL_OF_THE_SUN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INTI_SENESCHAL_OF_THE_SUN,
    1,
    "0548ddb6-5301-4911-93fd-bb988275f44a",
    "Richard Luong",
);

// LCI 296 — Huatli, Poet of Unity // Roar of the Fifth People (alternate printing)
const HUATLI_POET_OF_UNITY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HUATLI_POET_OF_UNITY,
    1,
    "b94c8a9e-5b9f-4a80-bced-ee99e5ed060a",
    "Anditya Dita",
);

// LCI 297 — Abuelo, Ancestral Echo (alternate printing)
const ABUELO_ANCESTRAL_ECHO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ABUELO_ANCESTRAL_ECHO,
    1,
    "e0bcc6f3-bae4-485c-82da-56c15c97c7f8",
    "Cabrol",
);

// LCI 298 — Akawalli, the Seething Tower (alternate printing)
const AKAWALLI_THE_SEETHING_TOWER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AKAWALLI_THE_SEETHING_TOWER,
    1,
    "88e9dbe6-5685-480d-b0d7-828a7058fb22",
    "rishxxv",
);

// LCI 299 — Amalia Benavides Aguirre (alternate printing)
const AMALIA_BENAVIDES_AGUIRRE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AMALIA_BENAVIDES_AGUIRRE,
    1,
    "9b1ae96c-012c-4bd4-81f0-5944fc03a8a9",
    "Alex Negrea",
);

// LCI 300 — Anim Pakal, Thousandth Moon (alternate printing)
const ANIM_PAKAL_THOUSANDTH_MOON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ANIM_PAKAL_THOUSANDTH_MOON,
    1,
    "7fc97d16-c54c-4a2b-9691-39e8a41c7777",
    "Anditya Dita",
);

// LCI 301 — Bartolomé del Presidio (alternate printing)
const BARTOLOME_DEL_PRESIDIO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BARTOLOME_DEL_PRESIDIO,
    1,
    "f82bb486-7009-4b53-973f-477fc13ad7e7",
    "Pig Hands",
);

// LCI 302 — Caparocti Sunborn (alternate printing)
const CAPAROCTI_SUNBORN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CAPAROCTI_SUNBORN,
    1,
    "380ed1dc-afa2-4b00-8da6-6177295965b5",
    "Cabrol",
);

// LCI 303 — Captain Storm, Cosmium Raider (alternate printing)
const CAPTAIN_STORM_COSMIUM_RAIDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_STORM_COSMIUM_RAIDER,
    1,
    "f5e13dad-e326-4b8e-91c7-dc0d6c3bf51d",
    "rishxxv",
);

// LCI 304 — Kutzil, Malamet Exemplar (alternate printing)
const KUTZIL_MALAMET_EXEMPLAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KUTZIL_MALAMET_EXEMPLAR,
    1,
    "12fede45-8c95-45eb-915c-c3ee5101dfcc",
    "Pig Hands",
);

// LCI 305 — The Mycotyrant (alternate printing)
const THE_MYCOTYRANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_MYCOTYRANT,
    1,
    "b27e6494-6f64-4adf-96ca-65a17100b8fe",
    "Andy Brase",
);

// LCI 306 — Nicanzil, Current Conductor (alternate printing)
const NICANZIL_CURRENT_CONDUCTOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NICANZIL_CURRENT_CONDUCTOR,
    1,
    "6059b64c-3064-46de-af47-6dc6542dca23",
    "Cabrol",
);

// LCI 307 — Quintorius Kand (alternate printing)
const QUINTORIUS_KAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &QUINTORIUS_KAND,
    1,
    "3dfc085d-4c11-416c-ae00-d41d32f32bb4",
    "Cosmin Podar",
);

// LCI 308 — Saheeli, the Sun's Brilliance (alternate printing)
const SAHEELI_THE_SUN_S_BRILLIANCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SAHEELI_THE_SUN_S_BRILLIANCE,
    1,
    "7a16ca40-2210-4f94-bfb1-960b34a7d98e",
    "Richard Luong",
);

// LCI 309 — Sovereign Okinec Ahau (alternate printing)
const SOVEREIGN_OKINEC_AHAU_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SOVEREIGN_OKINEC_AHAU,
    1,
    "5d6e7f53-d8c4-4356-a31d-ed3d5e0627cc",
    "Pig Hands",
);

// LCI 310 — Uchbenbak, the Great Mistake (alternate printing)
const UCHBENBAK_THE_GREAT_MISTAKE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UCHBENBAK_THE_GREAT_MISTAKE,
    1,
    "d30c23d7-8a3b-4162-89dd-5f7895c55625",
    "WolfSkullJack",
);

// LCI 311 — Vito, Fanatic of Aclazotz (alternate printing)
const VITO_FANATIC_OF_ACLAZOTZ_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VITO_FANATIC_OF_ACLAZOTZ,
    1,
    "8f58c496-2b70-48a9-befa-9cc7baef8597",
    "WolfSkullJack",
);

// LCI 312 — Zoyowa Lava-Tongue (alternate printing)
const ZOYOWA_LAVA_TONGUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ZOYOWA_LAVA_TONGUE,
    1,
    "2cfd2cf8-95e4-4df5-a803-66e7c4eb065a",
    "Richard Luong",
);

// LCI 313 — Throne of the Grim Captain // The Grim Captain (alternate printing)
const THRONE_OF_THE_GRIM_CAPTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THRONE_OF_THE_GRIM_CAPTAIN,
    1,
    "4f7a6892-5f6b-49aa-9b4f-3fcf20236306",
    "Rafal Wechterowicz (Too Many Skulls)",
);

// LCI 314 — Ojer Taq, Deepest Foundation // Temple of Civilization (alternate printing)
const OJER_TAQ_DEEPEST_FOUNDATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OJER_TAQ_DEEPEST_FOUNDATION,
    1,
    "e87dff94-71ed-469e-a10e-3ddfbd4a8bec",
    "Clint Lockwood & Josu Hernaiz",
);

// LCI 315 — Ojer Pakpatiq, Deepest Epoch // Temple of Cyclical Time (alternate printing)
const OJER_PAKPATIQ_DEEPEST_EPOCH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OJER_PAKPATIQ_DEEPEST_EPOCH,
    1,
    "a1fbfb8f-600e-4b29-98c5-7c6714d35347",
    "Clint Lockwood & Viko Menezes",
);

// LCI 316 — Aclazotz, Deepest Betrayal // Temple of the Dead (alternate printing)
const ACLAZOTZ_DEEPEST_BETRAYAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ACLAZOTZ_DEEPEST_BETRAYAL,
    1,
    "95775b13-2761-424e-9828-2275ec987368",
    "Clint Lockwood & Viko Menezes",
);

// LCI 317 — Ojer Axonil, Deepest Might // Temple of Power (alternate printing)
const OJER_AXONIL_DEEPEST_MIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OJER_AXONIL_DEEPEST_MIGHT,
    1,
    "5140d5d0-15ad-45b2-8ae8-aa53b1cd8132",
    "Clint Lockwood & Viko Menezes",
);

// LCI 318 — Ojer Kaslem, Deepest Growth // Temple of Cultivation (alternate printing)
const OJER_KASLEM_DEEPEST_GROWTH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OJER_KASLEM_DEEPEST_GROWTH,
    1,
    "d6ed63e0-d6f1-455a-b29c-e2c91dcfcc6e",
    "Clint Lockwood & Eddie Mendoza",
);

// LCI 319 — The Ancient One (alternate printing)
const THE_ANCIENT_ONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_ANCIENT_ONE,
    1,
    "f1ebe99d-4994-450e-9abb-f0bc3750f098",
    "Clint Lockwood",
);

// LCI 320 — Belligerent Yearling (alternate printing)
const BELLIGERENT_YEARLING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BELLIGERENT_YEARLING,
    1,
    "cf4639ca-d4dc-4630-b696-a8107767cf1a",
    "Sidharth Chaturvedi",
);

// LCI 321 — Bonehoard Dracosaur (alternate printing)
const BONEHOARD_DRACOSAUR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BONEHOARD_DRACOSAUR,
    1,
    "22897e93-bb69-4ad3-bdcc-723c7020a378",
    "Sidharth Chaturvedi",
);

// LCI 322 — Rampaging Ceratops (alternate printing)
const RAMPAGING_CERATOPS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAMPAGING_CERATOPS,
    1,
    "4be8d613-7697-413b-a107-0a25c1662835",
    "Sidharth Chaturvedi",
);

// LCI 323 — Scytheclaw Raptor (alternate printing)
const SCYTHECLAW_RAPTOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SCYTHECLAW_RAPTOR,
    1,
    "467ffe37-f466-4eea-8efc-50b53e2c27da",
    "Sidharth Chaturvedi",
);

// LCI 324 — Trumpeting Carnosaur (alternate printing)
const TRUMPETING_CARNOSAUR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TRUMPETING_CARNOSAUR,
    1,
    "35957bfb-c75f-4181-9726-19956992a6e6",
    "Sidharth Chaturvedi",
);

// LCI 325 — Earthshaker Dreadmaw (alternate printing)
const EARTHSHAKER_DREADMAW_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EARTHSHAKER_DREADMAW,
    1,
    "158ad2e4-5e81-439b-9629-a9d217c45d83",
    "Sidharth Chaturvedi",
);

// LCI 326 — Ghalta, Stampede Tyrant (alternate printing)
const GHALTA_STAMPEDE_TYRANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GHALTA_STAMPEDE_TYRANT,
    1,
    "f591b6a9-ac99-42ad-9b01-c09a3e90201b",
    "Sidharth Chaturvedi",
);

// LCI 327 — Hulking Raptor (alternate printing)
const HULKING_RAPTOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HULKING_RAPTOR,
    1,
    "27b95a76-688c-4bd9-a759-f83f1aa18e2a",
    "Sidharth Chaturvedi",
);

// LCI 328 — Pugnacious Hammerskull (alternate printing)
const PUGNACIOUS_HAMMERSKULL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PUGNACIOUS_HAMMERSKULL,
    1,
    "83cb05bc-5137-477e-b7f4-e914f7fa543f",
    "Sidharth Chaturvedi",
);

// LCI 329 — Thrashing Brontodon (alternate printing)
const THRASHING_BRONTODON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_rix::THRASHING_BRONTODON,
    1,
    "4cb1c91a-398c-437a-9a77-5b8515fc3ef2",
    "Sidharth Chaturvedi",
);

// LCI 330 — Gishath, Sun's Avatar (alternate printing)
const GISHATH_SUN_S_AVATAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_xln::GISHATH_SUN_S_AVATAR,
    1,
    "952cef59-bc7a-48e0-b05a-d9bdd6e05927",
    "Sidharth Chaturvedi",
);

// LCI 331 — Itzquinth, Firstborn of Gishath (alternate printing)
const ITZQUINTH_FIRSTBORN_OF_GISHATH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ITZQUINTH_FIRSTBORN_OF_GISHATH,
    1,
    "44f2aa6b-a092-4348-a24f-5edf46544eb7",
    "Sidharth Chaturvedi",
);

// LCI 332 — Palani's Hatcher (alternate printing)
const PALANI_S_HATCHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PALANI_S_HATCHER,
    1,
    "a0817334-d7ba-4de9-ade7-67beef358d1f",
    "Sidharth Chaturvedi",
);

// LCI 333 — Get Lost (alternate printing)
const GET_LOST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GET_LOST,
    1,
    "7d60e0bc-1ce1-4b91-9952-16b394b5a348",
    "Psydrian",
);

// LCI 334 — Resplendent Angel (alternate printing)
const RESPLENDENT_ANGEL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_m19::RESPLENDENT_ANGEL,
    1,
    "e827612d-5a43-4a29-b0c1-ab7f3285ad98",
    "Jorge Gutierrez Garcia",
);

// LCI 335 — Tishana's Tidebinder
pub(in crate::card::sets) static TISHANA_S_TIDEBINDER: CardRecord = CardRecord::new(
    "Tishana's Tidebinder",
    "604e2bfc-655d-4d3e-98aa-374780ca4016",
    "LeDania",
// Three mana at instant speed for a body, an answer, and a permanent
    // that never does anything again.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk", "Wizard"], 3, 2)
        .with_abilities(&[
            abilities::flash(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, counter up to one target activated or triggered ability. If \
                 an ability of an artifact, creature, or planeswalker is countered this way, that \
                 permanent loses all abilities for as long as this creature remains on the battlefield.",
                // An ability and not a spell, and up to one of them: a Tidebinder flashed
                // in with nothing on the stack is still a 3/2. Mana abilities never use the
                // stack, so nothing has to exclude them.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Ability,
                        zones: &[ZoneKind::Stack],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::Sequence(&[
                    EffectDef::Counter {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Graveyard,
                        placement: ZonePlacement::Top,
                    },
                    // The permanent the countered ability came from, read after the counter has
                    // retired it, then narrowed to the types the rider names. A countered
                    // ability whose source was an enchantment binds nothing here, which is the
                    // "if" doing its work.
                    abilities::bind_objects_then(
                        crate::card::ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::One(
                            ObjectRefDef::SourceOfTargetedStackObject(TargetIndex::PRIMARY),
                        )),
                        &EffectDef::Apply {
                            recipient: EffectRecipientDef::objects(ObjectSetDef::MatchingBinding {
                                binding: ParentBinding,
                                // The rider names three permanent types and not the other two: an
                                // enchantment or a land whose ability is countered keeps everything it has.
                                object: ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                                ]),
                            }),
                            effect: AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
                            // Not a turn and not forever: the silence lasts exactly as long as
                            // the Tidebinder is standing there.
                            duration: ResolvedEffectDurationDef::WhileSourceRemains,
                        },
                    ),
                ]),
            ),
        ]),
);

// LCI 336 — Bloodletter of Aclazotz (alternate printing)
const BLOODLETTER_OF_ACLAZOTZ_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BLOODLETTER_OF_ACLAZOTZ,
    1,
    "48c5b98d-73cd-4b69-9933-4867da6e8389",
    "Bene Rohlmann",
);

// LCI 337 — Bringer of the Last Gift (alternate printing)
const BRINGER_OF_THE_LAST_GIFT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRINGER_OF_THE_LAST_GIFT,
    1,
    "09e732f7-85d7-4b2e-b332-9be30bfba3d5",
    "Psydrian",
);

// LCI 338 — Starving Revenant (alternate printing)
const STARVING_REVENANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STARVING_REVENANT,
    1,
    "3e6ff99a-9ecc-4f1b-9c85-e0820f3a5729",
    "Jorge Gutierrez Garcia",
);

// LCI 339 — Huatli, Poet of Unity // Roar of the Fifth People (alternate printing)
const HUATLI_POET_OF_UNITY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HUATLI_POET_OF_UNITY,
    2,
    "3cdeebd8-28d4-42a5-9a99-3c72771359ea",
    "LeDania",
);

// LCI 340 — The Skullspore Nexus (alternate printing)
const THE_SKULLSPORE_NEXUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_SKULLSPORE_NEXUS,
    1,
    "85c4b837-1557-4dda-a324-9646ce702dfd",
    "Mónica Robles Corzo",
);

// LCI 341 — Kellan, Daring Traveler // Journey On (alternate printing)
const KELLAN_DARING_TRAVELER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KELLAN_DARING_TRAVELER,
    1,
    "95a9d11a-ecf0-471e-9dc3-a5baa6725044",
    "Jorge Gutierrez Garcia",
);

// LCI 342 — Molten Collapse (alternate printing)
const MOLTEN_COLLAPSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOLTEN_COLLAPSE,
    1,
    "ecda9470-7150-48bf-8fbf-1cb0cc80d2e1",
    "Mónica Robles Corzo",
);

// LCI 343 — Wail of the Forgotten (alternate printing)
const WAIL_OF_THE_FORGOTTEN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WAIL_OF_THE_FORGOTTEN,
    1,
    "e435c3bf-2b64-4db4-b9a2-322095b566bf",
    "Bene Rohlmann",
);

// LCI 344 — Roaming Throne (alternate printing)
const ROAMING_THRONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROAMING_THRONE,
    1,
    "0ff7858d-f18d-4e5a-9467-ebef3ef53ac1",
    "LeDania",
);

// LCI 345 — Cavern of Souls (alternate printing)
const CAVERN_OF_SOULS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_avr::CAVERN_OF_SOULS,
    1,
    "1d842f91-a04c-4b11-8ac9-0c0536b82d03",
    "Pedro Correa",
);

// LCI 346 — Echoing Deeps (alternate printing)
const ECHOING_DEEPS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ECHOING_DEEPS,
    1,
    "c6392d30-07c4-4c63-acb4-ad12d08c080b",
    "Mónica Robles Corzo",
);

// LCI 347 — Restless Anchorage (alternate printing)
const RESTLESS_ANCHORAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RESTLESS_ANCHORAGE,
    1,
    "0fc7725d-eb04-4778-8f5e-4c1239ed08b8",
    "Tyler Smith",
);

// LCI 348 — Restless Prairie (alternate printing)
const RESTLESS_PRAIRIE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RESTLESS_PRAIRIE,
    1,
    "7802788a-8a40-40e7-9d33-58191d1d678a",
    "Piotr Dura",
);

// LCI 349 — Restless Reef (alternate printing)
const RESTLESS_REEF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RESTLESS_REEF,
    1,
    "8315e121-cf10-4442-91c0-036ca5bbc652",
    "Thomas Stoop",
);

// LCI 350 — Restless Ridgeline (alternate printing)
const RESTLESS_RIDGELINE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RESTLESS_RIDGELINE,
    1,
    "2634eb9f-d797-4af4-90d3-7a39fae8300b",
    "David Frasheski",
);

// LCI 351 — Restless Vents (alternate printing)
const RESTLESS_VENTS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RESTLESS_VENTS,
    1,
    "9200f9b5-20fc-4ff0-97f7-ef22da5acb72",
    "Alexander Kintner",
);

// LCI 352 — Quintorius Kand (alternate printing)
const QUINTORIUS_KAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &QUINTORIUS_KAND,
    2,
    "03c80368-0fe6-481a-bee0-7d9f65afb233",
    "Psydrian",
);

// LCI 353 — Abuelo's Awakening (alternate printing)
const ABUELO_S_AWAKENING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ABUELO_S_AWAKENING,
    1,
    "647d2fe5-674d-4b3d-b658-ae6901e8044f",
    "Eelis Kyttanen",
);

// LCI 354 — Fabrication Foundry (alternate printing)
const FABRICATION_FOUNDRY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FABRICATION_FOUNDRY,
    1,
    "5b5efc5e-1582-4bb7-9ae1-bc4c0487ee94",
    "Racrufi",
);

// LCI 355 — Kutzil's Flanker (alternate printing)
const KUTZIL_S_FLANKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KUTZIL_S_FLANKER,
    1,
    "685b735e-52a6-4fd6-a12e-d059c13d4afc",
    "Michele Giorgi",
);

// LCI 356 — Sanguine Evangelist (alternate printing)
const SANGUINE_EVANGELIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SANGUINE_EVANGELIST,
    1,
    "a046c0df-7dba-4a05-b4b4-a76321b15480",
    "Zezhou Chen",
);

// LCI 357 — Thousand Moons Smithy // Barracks of the Thousand (alternate printing)
const THOUSAND_MOONS_SMITHY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THOUSAND_MOONS_SMITHY,
    1,
    "89a0f7fe-6bff-4c0a-b419-1f4f457c194c",
    "Manuel Castañón",
);

// LCI 358 — Unstable Glyphbridge // Sandswirl Wanderglyph (alternate printing)
const UNSTABLE_GLYPHBRIDGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNSTABLE_GLYPHBRIDGE,
    1,
    "a8a23e3f-57c8-4cbb-bb6c-d5fdf49a2d9c",
    "Bastien Grivet",
);

// LCI 359 — Warden of the Inner Sky (alternate printing)
const WARDEN_OF_THE_INNER_SKY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WARDEN_OF_THE_INNER_SKY,
    1,
    "848b1e97-d1c7-450e-a935-50b386b6b762",
    "Raoul Vitale",
);

// LCI 360 — Braided Net // Braided Quipu (alternate printing)
const BRAIDED_NET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRAIDED_NET,
    1,
    "c8157978-259e-4af1-890e-ea56a55a9997",
    "Diego Gisbert",
);

// LCI 361 — Deeproot Pilgrimage (alternate printing)
const DEEPROOT_PILGRIMAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEEPROOT_PILGRIMAGE,
    1,
    "790d70a9-ff02-4676-a804-ee0e030dfe3b",
    "Rémi Jacquot",
);

// LCI 362 — The Enigma Jewel // Locus of Enlightenment (alternate printing)
const THE_ENIGMA_JEWEL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_ENIGMA_JEWEL,
    1,
    "8d9cbb83-a89f-411b-88c8-c01718f265e0",
    "Martin de Diego Sádaba",
);

// LCI 363 — The Everflowing Well // The Myriad Pools (alternate printing)
const THE_EVERFLOWING_WELL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_EVERFLOWING_WELL,
    1,
    "581327f5-6e79-4720-9695-198b287a92bb",
    "David Álvarez",
);

// LCI 364 — Kitesail Larcenist (alternate printing)
const KITESAIL_LARCENIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KITESAIL_LARCENIST,
    1,
    "27c2a1f9-7846-4f96-b20d-ceb638e9ed42",
    "Sidharth Chaturvedi",
);

// LCI 365 — Subterranean Schooner (alternate printing)
const SUBTERRANEAN_SCHOONER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUBTERRANEAN_SCHOONER,
    1,
    "2c87307e-59f0-4747-b0d2-e8b65f4b260e",
    "Svetlin Velinov",
);

// LCI 366 — Corpses of the Lost (alternate printing)
const CORPSES_OF_THE_LOST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CORPSES_OF_THE_LOST,
    1,
    "6b3eb24e-bba4-463b-ad7e-e3daebda1e74",
    "Izzy",
);

// LCI 367 — Preacher of the Schism
pub(in crate::card::sets) static PREACHER_OF_THE_SCHISM: CardRecord = CardRecord::new(
    "Preacher of the Schism",
    "3a0db433-7ca2-48d6-b60c-0a9a9149378a",
    "Donato Giancola",
    // A 2/4 deathtouch body that punishes whoever is ahead on life, and
    // draws while she is the one ahead.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Vampire", "Cleric"], 2, 4).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::triggered(
            "Whenever this creature attacks the player with the most life or tied for most life, \
             create a 1/1 white Vampire creature token with lifelink.",
            // "Attacks the player with the most life": the condition belongs to the
            // attack rather than being an intervening if, and the player it asks about
            // is the one the attack was aimed at, which the event names. The player
            // themselves -- a planeswalker of theirs is a different thing to attack,
            // whoever ends up being attacked by it.
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks_a_player(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::PlayerHasMostLife(PlayerRelation::EventPlayer),
            },
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Vampire"], &[ManaColor::White], 1, 1)
                    .with_abilities(&[abilities::lifelink()]),
            ))),
        ),
        AbilityDef::triggered(
            "Whenever this creature attacks while you have the most life or are tied for most \
             life, you draw a card and you lose 1 life.",
            // The same attack, asked about his own controller instead. Both clauses
            // read one attack, so a creature attacking the player who is ahead while
            // its controller is also tied for the lead triggers both.
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::PlayerHasMostLife(PlayerRelation::You),
            },
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// LCI 368 — Queen's Bay Paladin (alternate printing)
const QUEEN_S_BAY_PALADIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &QUEEN_S_BAY_PALADIN,
    1,
    "a40cdb01-d303-438c-9c87-ca334ac68582",
    "Slawomir Maniak",
);

// LCI 369 — Souls of the Lost (alternate printing)
const SOULS_OF_THE_LOST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SOULS_OF_THE_LOST,
    1,
    "efc2f3d3-6bac-4ca7-9c42-badf649269f5",
    "Nils Hamm",
);

// LCI 370 — Stalactite Stalker (alternate printing)
const STALACTITE_STALKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STALACTITE_STALKER,
    1,
    "d8754c3b-2deb-45af-9024-1f070496b45b",
    "Olivier Bernard",
);

// LCI 371 — Tarrian's Journal // The Tomb of Aclazotz (alternate printing)
const TARRIAN_S_JOURNAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TARRIAN_S_JOURNAL,
    1,
    "3577ac1a-7c14-4463-a8c0-530f37f3d935",
    "Randy Gallegos",
);

// LCI 372 — Terror Tide (alternate printing)
const TERROR_TIDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TERROR_TIDE,
    1,
    "39776030-4a89-4b22-952b-4a7f33f655b5",
    "Brock Grossman",
);

// LCI 373 — Brass's Tunnel-Grinder // Tecutlan, the Searing Rift (alternate printing)
const BRASS_S_TUNNEL_GRINDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRASS_S_TUNNEL_GRINDER,
    1,
    "db940d06-5e3d-4831-a200-de614c093aca",
    "Cristi Balanescu",
);

// LCI 374 — Dire Flail // Dire Blunderbuss (alternate printing)
const DIRE_FLAIL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DIRE_FLAIL,
    1,
    "c3c5e4a1-7668-41a0-acd3-d45809b6a95e",
    "Anthony Devine",
);

// LCI 375 — Hit the Mother Lode (alternate printing)
const HIT_THE_MOTHER_LODE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HIT_THE_MOTHER_LODE,
    1,
    "73e794c1-33a3-462f-b8e5-479c3acf91d3",
    "Diego Gisbert",
);

// LCI 376 — Magmatic Galleon (alternate printing)
const MAGMATIC_GALLEON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MAGMATIC_GALLEON,
    1,
    "3ba4238b-e7fb-4859-87e4-c192e7e6c047",
    "Cristi Balanescu",
);

// LCI 377 — Poetic Ingenuity (alternate printing)
const POETIC_INGENUITY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &POETIC_INGENUITY,
    1,
    "0c9bec4a-084b-4972-9a6b-58050088168b",
    "Kieran Yanner",
);

// LCI 378 — Bedrock Tortoise (alternate printing)
const BEDROCK_TORTOISE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BEDROCK_TORTOISE,
    1,
    "4fea4c57-326e-4783-b5b5-0b28fcf6fe7f",
    "Maxime Minard",
);

// LCI 379 — Cosmium Confluence (alternate printing)
const COSMIUM_CONFLUENCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COSMIUM_CONFLUENCE,
    1,
    "1f8f827d-300a-4d75-b894-c6dd2e5997ce",
    "Kasia 'Kafis' Zielińska",
);

// LCI 380 — Growing Rites of Itlimoc // Itlimoc, Cradle of the Sun (alternate printing)
const GROWING_RITES_OF_ITLIMOC_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_xln::GROWING_RITES_OF_ITLIMOC,
    1,
    "36651c33-d570-4d48-a1d3-bac736d3f043",
    "Josu Hernaiz",
);

// LCI 381 — Intrepid Paleontologist (alternate printing)
const INTREPID_PALEONTOLOGIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INTREPID_PALEONTOLOGIST,
    1,
    "112da23d-3f13-4438-b027-10c5493808bd",
    "Irina Nordsol",
);

// LCI 382 — Jadelight Spelunker (alternate printing)
const JADELIGHT_SPELUNKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JADELIGHT_SPELUNKER,
    1,
    "0987d47f-8a4b-47fc-9c5d-641f57631af0",
    "Izzy",
);

// LCI 383 — Sentinel of the Nameless City (alternate printing)
const SENTINEL_OF_THE_NAMELESS_CITY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SENTINEL_OF_THE_NAMELESS_CITY,
    1,
    "f0563531-ccd5-4dc4-88dd-a1e438507cba",
    "Josu Hernaiz",
);

// LCI 384 — The Belligerent (alternate printing)
const THE_BELLIGERENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_BELLIGERENT,
    1,
    "5a31f6e4-9fc7-46b3-85f8-82025817a35d",
    "Bruce Brenneise",
);

// LCI 385 — Deepfathom Echo (alternate printing)
const DEEPFATHOM_ECHO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEEPFATHOM_ECHO,
    1,
    "64d78301-333a-428e-9af9-588a103cc527",
    "Matt Stewart",
);

// LCI 386 — Squirming Emergence (alternate printing)
const SQUIRMING_EMERGENCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SQUIRMING_EMERGENCE,
    1,
    "d165f708-65e9-4dbb-8a8c-085c5a2881d6",
    "Simon Dominic",
);

// LCI 387 — Matzalantli, the Great Door // The Core (alternate printing)
const MATZALANTLI_THE_GREAT_DOOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MATZALANTLI_THE_GREAT_DOOR,
    1,
    "5e03b08d-f0f1-40e8-af48-1622430bab38",
    "Piotr Dura",
);

// LCI 388 — The Millennium Calendar (alternate printing)
const THE_MILLENNIUM_CALENDAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_MILLENNIUM_CALENDAR,
    1,
    "53a710ed-6a2a-4702-9648-7807e8c24edc",
    "Zoltan Boros",
);

// LCI 389 — Tarrian's Soulcleaver (alternate printing)
const TARRIAN_S_SOULCLEAVER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TARRIAN_S_SOULCLEAVER,
    1,
    "a0d8cca6-d92c-40e8-b657-b708a08b63ed",
    "Nereida",
);

// LCI 390 — Threefold Thunderhulk (alternate printing)
const THREEFOLD_THUNDERHULK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THREEFOLD_THUNDERHULK,
    1,
    "180d3058-cc7d-4bb7-a312-4d2d7365ef19",
    "Xavier Ribeiro",
);

// LCI 391 — Treasure Map // Treasure Cove (alternate printing)
const TREASURE_MAP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_xln::TREASURE_MAP,
    1,
    "2018a8e6-7699-440f-94d5-91cf6ac3edb5",
    "Néstor Ossandón Leal",
);

// LCI 392 — Sunken Citadel (alternate printing)
const SUNKEN_CITADEL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUNKEN_CITADEL,
    1,
    "58f4843c-8033-4775-9ffe-4f3980fabfeb",
    "Matteo Bassini",
);

// LCI 393 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "4716a32c-91a6-470a-a686-d5eb3d27f46e",
    "Carlos Palma Cruchaga",
);

// LCI 394 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "e06c72cb-03ce-48eb-b5dc-b2301c6871a2",
    "Adam Paquette",
);

// LCI 395 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "23e3ed2c-342b-44aa-8e3e-0c53602397c3",
    "Néstor Ossandón Leal",
);

// LCI 396 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "a5f9dab5-4b13-4a98-8a64-76e69f0ba511",
    "Adam Paquette",
);

// LCI 397 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "4f893107-84b0-4e3f-b1f5-b04632f192c5",
    "Logan Feliciano",
);

// LCI 398 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "ed0b0b4f-2b31-4fe1-aedb-be9d34fce688",
    "Adam Paquette",
);

// LCI 399 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "e82fc4bc-ac81-4ad6-8b33-781d047f60d5",
    "Muhammad Firdaus",
);

// LCI 400 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "be8e09c2-3384-4d00-a2d0-65d6b8056e30",
    "Adam Paquette",
);

// LCI 401 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "f954219a-fd4b-4fb1-945f-7950efc1d975",
    "Carlos Palma Cruchaga",
);

// LCI 402 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "1069841a-0642-4fb6-b831-d45ff7fda3af",
    "Adam Paquette",
);

// LCI 403 — Jadelight Spelunker (alternate printing)
const JADELIGHT_SPELUNKER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &JADELIGHT_SPELUNKER,
    2,
    "cee66a2c-0684-4324-82fc-60c13a55602e",
    "Ovidio Cartagena",
);

// LCI 404 — Hit the Mother Lode (alternate printing)
const HIT_THE_MOTHER_LODE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HIT_THE_MOTHER_LODE,
    2,
    "6b3ed0f5-b476-4595-bdff-531488961a87",
    "Bram Sels",
);

// LCI 405 — Spyglass Siren (alternate printing)
const SPYGLASS_SIREN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPYGLASS_SIREN,
    1,
    "24643b00-9062-42bd-9ecd-f6990ddbca91",
    "David Astruga",
);

// LCI 406 — Deep-Cavern Bat (alternate printing)
const DEEP_CAVERN_BAT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEEP_CAVERN_BAT,
    1,
    "444190a9-5ad7-4132-aae4-247ad3d9ea01",
    "Campbell White",
);

// LCI 407 — Geological Appraiser (alternate printing)
const GEOLOGICAL_APPRAISER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GEOLOGICAL_APPRAISER,
    1,
    "9fb48483-5990-46cf-b200-032500c4a6b3",
    "Alix Branwyn",
);

// LCI 408 — Cenote Scout (alternate printing)
const CENOTE_SCOUT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CENOTE_SCOUT,
    1,
    "9f338f13-eac2-4088-a33c-cb389fbe967a",
    "Caroline Gariba",
);

// LCI 409 — Bartolomé del Presidio (alternate printing)
const BARTOLOME_DEL_PRESIDIO_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BARTOLOME_DEL_PRESIDIO,
    2,
    "aade7c6c-fa34-4f69-9b5d-cf57a5341ccc",
    "Randy Gallegos",
);

// LCI 410a — Cavern of Souls (alternate printing)
const CAVERN_OF_SOULS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_avr::CAVERN_OF_SOULS,
    2,
    "a6675d55-5960-4da4-a31c-45654ab9974d",
    "Pedro Correa",
);

// LCI 410b — Cavern of Souls (alternate printing)
const CAVERN_OF_SOULS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_avr::CAVERN_OF_SOULS,
    3,
    "fcc626a1-cfb0-4e1f-80fc-7c1ff07ee424",
    "Pedro Correa",
);

// LCI 410c — Cavern of Souls (alternate printing)
const CAVERN_OF_SOULS_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_avr::CAVERN_OF_SOULS,
    4,
    "09f9b3b8-090e-4152-8d44-52d92375905b",
    "Pedro Correa",
);

// LCI 410d — Cavern of Souls (alternate printing)
const CAVERN_OF_SOULS_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_avr::CAVERN_OF_SOULS,
    5,
    "d89bcbd5-772a-4ff2-b48b-6fea07830578",
    "Pedro Correa",
);

// LCI 410e — Cavern of Souls (alternate printing)
const CAVERN_OF_SOULS_ALTERNATE_6: PrintingRecord = PrintingRecord::alternate(
    &catalog_avr::CAVERN_OF_SOULS,
    6,
    "0b773b4e-7af3-4836-8c7f-3897f968a33e",
    "Pedro Correa",
);

// LCI 410f — Cavern of Souls (alternate printing)
const CAVERN_OF_SOULS_ALTERNATE_7: PrintingRecord = PrintingRecord::alternate(
    &catalog_avr::CAVERN_OF_SOULS,
    7,
    "fde9dbec-ad25-488b-8472-4bd85fd6c757",
    "Pedro Correa",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ABUELO_S_AWAKENING,
    &ACROBATIC_LEAP,
    &ADAPTIVE_GEMGUARD,
    &ATTENTIVE_SUNSCRIBE,
    &BAT_COLONY,
    &CLAY_FIRED_BRICKS,
    &COSMIUM_BLAST,
    &DAUNTLESS_DISMANTLER,
    &DECONSTRUCTION_HAMMER,
    &DUSK_ROSE_RELIQUARY,
    &ENVOY_OF_OKINEC_AHAU,
    &FABRICATION_FOUNDRY,
    &FAMILY_REUNION,
    &GET_LOST,
    &GLORIFIER_OF_SUFFERING,
    &GUARDIAN_OF_THE_GREAT_DOOR,
    &HELPING_HAND,
    &IRONPAW_ASPIRANT,
    &KINJALLI_S_DAWNRUNNER,
    &KUTZIL_S_FLANKER,
    &MALAMET_WAR_SCRIBE,
    &MARKET_GNOME,
    &MIGHT_OF_THE_ANCESTORS,
    &MINER_S_GUIDEWING,
    &MISCHIEVOUS_PUP,
    &OJER_TAQ_DEEPEST_FOUNDATION,
    &OLTEC_ARCHAEOLOGISTS,
    &OLTEC_CLOUD_GUARD,
    &OTECLAN_LANDMARK,
    &PETRIFY,
    &QUICKSAND_WHIRLPOOL,
    &RUIN_LURKER_BAT,
    &SANGUINE_EVANGELIST,
    &SOARING_SANDWING,
    &SPRING_LOADED_SAWBLADES,
    &THOUSAND_MOONS_CRACKSHOT,
    &THOUSAND_MOONS_INFANTRY,
    &THOUSAND_MOONS_SMITHY,
    &TINKER_S_TOTE,
    &UNSTABLE_GLYPHBRIDGE,
    &VANGUARD_OF_THE_ROSE,
    &WARDEN_OF_THE_INNER_SKY,
    &AKAL_PAKAL_FIRST_AMONG_EQUALS,
    &ANCESTRAL_REMINISCENCE,
    &BRACKISH_BLUNDER,
    &BRAIDED_NET,
    &COGWORK_WRESTLER,
    &CONFOUNDING_RIDDLE,
    &COUNCIL_OF_ECHOES,
    &DEEPROOT_PILGRIMAGE,
    &DIDACT_ECHO,
    &EATEN_BY_PIRANHAS,
    &THE_ENIGMA_JEWEL,
    &THE_EVERFLOWING_WELL,
    &FRILLED_CAVE_WURM,
    &HERMITIC_NAUTILUS,
    &HURL_INTO_HISTORY,
    &INVERTED_ICEBERG,
    &KITESAIL_LARCENIST,
    &LODESTONE_NEEDLE,
    &MALCOLM_ALLURING_SCOUNDREL,
    &MARAUDING_BRINEFANG,
    &MERFOLK_CAVE_DIVER,
    &OAKEN_SIREN,
    &OJER_PAKPATIQ_DEEPEST_EPOCH,
    &ORAZCA_PUZZLE_DOOR,
    &OUT_OF_AIR,
    &PIRATE_HAT,
    &RELIC_S_ROAR,
    &RIVER_HERALD_SCOUT,
    &SAGE_OF_DAYS,
    &SELF_REFLECTION,
    &SHIPWRECK_SENTRY,
    &SINUOUS_BENTHISAUR,
    &SONG_OF_STUPEFACTION,
    &SPYGLASS_SIREN,
    &STAUNCH_CREWMATE,
    &SUBTERRANEAN_SCHOONER,
    &UNLUCKY_DROP,
    &WATERLOGGED_HULK,
    &WATERWIND_SCOUT,
    &WAYLAYING_PIRATES,
    &ZOETIC_GLYPH,
    &ABYSSAL_GORESTALKER,
    &ACLAZOTZ_DEEPEST_BETRAYAL,
    &ACOLYTE_OF_ACLAZOTZ,
    &ANOTHER_CHANCE,
    &BITTER_TRIUMPH,
    &BLOODLETTER_OF_ACLAZOTZ,
    &BLOODTHORN_FLAIL,
    &BRINGER_OF_THE_LAST_GIFT,
    &BROODRAGE_MYCOID,
    &CANONIZED_IN_BLOOD,
    &CHUPACABRA_ECHO,
    &CORPSES_OF_THE_LOST,
    &DEATHCAP_MARIONETTE,
    &DEEP_GOBLIN_SKULLTAKER,
    &DEEP_CAVERN_BAT,
    &DEFOSSILIZE,
    &ECHO_OF_DUSK,
    &FANATICAL_OFFERING,
    &FUNGAL_FORTITUDE,
    &GARGANTUAN_LEECH,
    &GRASPING_SHADOWS,
    &GREEDY_FREEBOOTER,
    &JOIN_THE_DEAD,
    &MALICIOUS_ECLIPSE,
    &MEPHITIC_DRAUGHT,
    &PRIMORDIAL_GNAWER,
    &QUEEN_S_BAY_PALADIN,
    &RAMPAGING_SPIKETAIL,
    &RAY_OF_RUIN,
    &SCREAMING_PHANTOM,
    &SKULLCAP_SNAIL,
    &SOULCOIL_VIPER,
    &SOULS_OF_THE_LOST,
    &STALACTITE_STALKER,
    &STARVING_REVENANT,
    &STINGING_CAVE_CRAWLER,
    &SYNAPSE_NECROMAGE,
    &TARRIAN_S_JOURNAL,
    &TERROR_TIDE,
    &TITHING_BLADE,
    &VISAGE_OF_DREAD,
    &VITO_S_INQUISITOR,
    &ANCESTORS_AID,
    &BELLIGERENT_YEARLING,
    &BONEHOARD_DRACOSAUR,
    &BRASS_S_TUNNEL_GRINDER,
    &BRAZEN_BLADEMASTER,
    &BREECHES_EAGER_PILLAGER,
    &BURNING_SUN_CAVALRY,
    &CALAMITOUS_CAVE_IN,
    &CHILD_OF_THE_VOLCANO,
    &CURATOR_OF_SUN_S_CREATION,
    &DARING_DISCOVERY,
    &DIAMOND_PICK_AXE,
    &DINOTOMATON,
    &DIRE_FLAIL,
    &DOWSING_DEVICE,
    &DREADMAW_S_IRE,
    &ENTERPRISING_SCALLYWAG,
    &ETALI_S_FAVOR,
    &GEOLOGICAL_APPRAISER,
    &GOBLIN_TOMB_RAIDER,
    &GOLDFURY_STRIDER,
    &HIT_THE_MOTHER_LODE,
    &HOTFOOT_GNOME,
    &IDOL_OF_THE_DEEP_KING,
    &INTI_SENESCHAL_OF_THE_SUN,
    &MAGMATIC_GALLEON,
    &OJER_AXONIL_DEEPEST_MIGHT,
    &PANICKED_ALTISAUR,
    &PLUNDERING_PIRATE,
    &POETIC_INGENUITY,
    &RAMPAGING_CERATOPS,
    &SAHEELI_S_LATTICE,
    &SCYTHECLAW_RAPTOR,
    &SEISMIC_MONSTROSAUR,
    &SUNFIRE_TORCH,
    &SUNSHOT_MILITIA,
    &TECTONIC_HAZARD,
    &TRIUMPHANT_CHOMP,
    &TRUMPETING_CARNOSAUR,
    &VOLATILE_WANDERGLYPH,
    &ZOYOWA_S_JUSTICE,
    &ARMORED_KINCALLER,
    &BASKING_CAPYBARA,
    &BEDROCK_TORTOISE,
    &CAVERN_STOMPER,
    &CENOTE_SCOUT,
    &COATI_SCAVENGER,
    &COLOSSADACTYL,
    &COSMIUM_CONFLUENCE,
    &DISTURBED_SLUMBER,
    &EARTHSHAKER_DREADMAW,
    &EXPLORER_S_CACHE,
    &GHALTA_STAMPEDE_TYRANT,
    &GLIMPSE_THE_CORE,
    &GLOWCAP_LANTERN,
    &HUATLI_POET_OF_UNITY,
    &HUATLI_S_FINAL_STRIKE,
    &HULKING_RAPTOR,
    &IN_THE_PRESENCE_OF_AGES,
    &INTREPID_PALEONTOLOGIST,
    &IXALLI_S_LOREKEEPER,
    &JADE_SEEDSTONES,
    &JADELIGHT_SPELUNKER,
    &KASLEM_S_STONETREE,
    &MALAMET_BATTLE_GLYPH,
    &MALAMET_BRAWLER,
    &MALAMET_SCYTHE,
    &MALAMET_VETERAN,
    &MINESHAFT_SPIDER,
    &NURTURING_BRISTLEBACK,
    &OJER_KASLEM_DEEPEST_GROWTH,
    &OVER_THE_EDGE,
    &PATHFINDING_AXEJAW,
    &POISON_DART_FROG,
    &PUGNACIOUS_HAMMERSKULL,
    &RIVER_HERALD_GUIDE,
    &SEEKER_OF_SUNLIGHT,
    &SENTINEL_OF_THE_NAMELESS_CITY,
    &THE_SKULLSPORE_NEXUS,
    &SPELUNKING,
    &STAGGERING_SIZE,
    &TENDRIL_OF_THE_MYCOTYRANT,
    &TWISTS_AND_TURNS,
    &WALK_WITH_THE_ANCESTORS,
    &ABUELO_ANCESTRAL_ECHO,
    &AKAWALLI_THE_SEETHING_TOWER,
    &AMALIA_BENAVIDES_AGUIRRE,
    &THE_ANCIENT_ONE,
    &ANIM_PAKAL_THOUSANDTH_MOON,
    &BARTOLOME_DEL_PRESIDIO,
    &THE_BELLIGERENT,
    &CAPAROCTI_SUNBORN,
    &CAPTAIN_STORM_COSMIUM_RAIDER,
    &DEEPFATHOM_ECHO,
    &ITZQUINTH_FIRSTBORN_OF_GISHATH,
    &KELLAN_DARING_TRAVELER,
    &KUTZIL_MALAMET_EXEMPLAR,
    &MASTER_S_GUIDE_MURAL,
    &MOLTEN_COLLAPSE,
    &THE_MYCOTYRANT,
    &NICANZIL_CURRENT_CONDUCTOR,
    &PALANI_S_HATCHER,
    &QUINTORIUS_KAND,
    &SAHEELI_THE_SUN_S_BRILLIANCE,
    &SOVEREIGN_OKINEC_AHAU,
    &SQUIRMING_EMERGENCE,
    &UCHBENBAK_THE_GREAT_MISTAKE,
    &VITO_FANATIC_OF_ACLAZOTZ,
    &WAIL_OF_THE_FORGOTTEN,
    &ZOYOWA_LAVA_TONGUE,
    &BURIED_TREASURE,
    &CAREENING_MINE_CART,
    &CARTOGRAPHER_S_COMPANION,
    &COMPASS_GNOME,
    &CONTESTED_GAME_BALL,
    &DIGSITE_CONSERVATOR,
    &DISRUPTOR_WANDERGLYPH,
    &HOVERSTONE_PILGRIM,
    &HUNTER_S_BLOWGUN,
    &MATZALANTLI_THE_GREAT_DOOR,
    &THE_MILLENNIUM_CALENDAR,
    &ROAMING_THRONE,
    &RUNAWAY_BOULDER,
    &SCAMPERING_SURVEYOR,
    &SUNBIRD_STANDARD,
    &SWASHBUCKLER_S_WHIP,
    &TARRIAN_S_SOULCLEAVER,
    &THREEFOLD_THUNDERHULK,
    &THRONE_OF_THE_GRIM_CAPTAIN,
    &CAPTIVATING_CAVE,
    &CAVERNOUS_MAW,
    &ECHOING_DEEPS,
    &FORGOTTEN_MONUMENT,
    &HIDDEN_CATARACT,
    &HIDDEN_COURTYARD,
    &HIDDEN_NECROPOLIS,
    &HIDDEN_NURSERY,
    &HIDDEN_VOLCANO,
    &PIT_OF_OFFERINGS,
    &PROMISING_VEIN,
    &RESTLESS_ANCHORAGE,
    &RESTLESS_PRAIRIE,
    &RESTLESS_REEF,
    &RESTLESS_RIDGELINE,
    &RESTLESS_VENTS,
    &SUNKEN_CITADEL,
    &VOLATILE_FAULT,
    &TISHANA_S_TIDEBINDER,
    &PREACHER_OF_THE_SCHISM,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    RESPLENDENT_ANGEL_REPRINT,
    CHART_A_COURSE_REPRINT,
    TISHANA_S_TIDEBINDER_ALTERNATE_1,
    DEAD_WEIGHT_REPRINT,
    PREACHER_OF_THE_SCHISM_ALTERNATE_1,
    ABRADE_REPRINT,
    RUMBLING_ROCKSLIDE_REPRINT,
    GROWING_RITES_OF_ITLIMOC_REPRINT,
    THRASHING_BRONTODON_REPRINT,
    GISHATH_SUN_S_AVATAR_REPRINT,
    CHIMIL_THE_INNER_SUN_REPRINT,
    SORCEROUS_SPYGLASS_REPRINT,
    TREASURE_MAP_REPRINT,
    CAVERN_OF_SOULS_REPRINT,
    PLAINS_REPRINT,
    ISLAND_REPRINT,
    SWAMP_REPRINT,
    MOUNTAIN_REPRINT,
    FOREST_REPRINT,
    AKAL_PAKAL_FIRST_AMONG_EQUALS_ALTERNATE_1,
    MALCOLM_ALLURING_SCOUNDREL_ALTERNATE_1,
    BREECHES_EAGER_PILLAGER_ALTERNATE_1,
    INTI_SENESCHAL_OF_THE_SUN_ALTERNATE_1,
    HUATLI_POET_OF_UNITY_ALTERNATE_1,
    ABUELO_ANCESTRAL_ECHO_ALTERNATE_1,
    AKAWALLI_THE_SEETHING_TOWER_ALTERNATE_1,
    AMALIA_BENAVIDES_AGUIRRE_ALTERNATE_1,
    ANIM_PAKAL_THOUSANDTH_MOON_ALTERNATE_1,
    BARTOLOME_DEL_PRESIDIO_ALTERNATE_1,
    CAPAROCTI_SUNBORN_ALTERNATE_1,
    CAPTAIN_STORM_COSMIUM_RAIDER_ALTERNATE_1,
    KUTZIL_MALAMET_EXEMPLAR_ALTERNATE_1,
    THE_MYCOTYRANT_ALTERNATE_1,
    NICANZIL_CURRENT_CONDUCTOR_ALTERNATE_1,
    QUINTORIUS_KAND_ALTERNATE_1,
    SAHEELI_THE_SUN_S_BRILLIANCE_ALTERNATE_1,
    SOVEREIGN_OKINEC_AHAU_ALTERNATE_1,
    UCHBENBAK_THE_GREAT_MISTAKE_ALTERNATE_1,
    VITO_FANATIC_OF_ACLAZOTZ_ALTERNATE_1,
    ZOYOWA_LAVA_TONGUE_ALTERNATE_1,
    THRONE_OF_THE_GRIM_CAPTAIN_ALTERNATE_1,
    OJER_TAQ_DEEPEST_FOUNDATION_ALTERNATE_1,
    OJER_PAKPATIQ_DEEPEST_EPOCH_ALTERNATE_1,
    ACLAZOTZ_DEEPEST_BETRAYAL_ALTERNATE_1,
    OJER_AXONIL_DEEPEST_MIGHT_ALTERNATE_1,
    OJER_KASLEM_DEEPEST_GROWTH_ALTERNATE_1,
    THE_ANCIENT_ONE_ALTERNATE_1,
    BELLIGERENT_YEARLING_ALTERNATE_1,
    BONEHOARD_DRACOSAUR_ALTERNATE_1,
    RAMPAGING_CERATOPS_ALTERNATE_1,
    SCYTHECLAW_RAPTOR_ALTERNATE_1,
    TRUMPETING_CARNOSAUR_ALTERNATE_1,
    EARTHSHAKER_DREADMAW_ALTERNATE_1,
    GHALTA_STAMPEDE_TYRANT_ALTERNATE_1,
    HULKING_RAPTOR_ALTERNATE_1,
    PUGNACIOUS_HAMMERSKULL_ALTERNATE_1,
    THRASHING_BRONTODON_ALTERNATE_1,
    GISHATH_SUN_S_AVATAR_ALTERNATE_1,
    ITZQUINTH_FIRSTBORN_OF_GISHATH_ALTERNATE_1,
    PALANI_S_HATCHER_ALTERNATE_1,
    GET_LOST_ALTERNATE_1,
    RESPLENDENT_ANGEL_ALTERNATE_1,
    BLOODLETTER_OF_ACLAZOTZ_ALTERNATE_1,
    BRINGER_OF_THE_LAST_GIFT_ALTERNATE_1,
    STARVING_REVENANT_ALTERNATE_1,
    HUATLI_POET_OF_UNITY_ALTERNATE_2,
    THE_SKULLSPORE_NEXUS_ALTERNATE_1,
    KELLAN_DARING_TRAVELER_ALTERNATE_1,
    MOLTEN_COLLAPSE_ALTERNATE_1,
    WAIL_OF_THE_FORGOTTEN_ALTERNATE_1,
    ROAMING_THRONE_ALTERNATE_1,
    CAVERN_OF_SOULS_ALTERNATE_1,
    ECHOING_DEEPS_ALTERNATE_1,
    RESTLESS_ANCHORAGE_ALTERNATE_1,
    RESTLESS_PRAIRIE_ALTERNATE_1,
    RESTLESS_REEF_ALTERNATE_1,
    RESTLESS_RIDGELINE_ALTERNATE_1,
    RESTLESS_VENTS_ALTERNATE_1,
    QUINTORIUS_KAND_ALTERNATE_2,
    ABUELO_S_AWAKENING_ALTERNATE_1,
    FABRICATION_FOUNDRY_ALTERNATE_1,
    KUTZIL_S_FLANKER_ALTERNATE_1,
    SANGUINE_EVANGELIST_ALTERNATE_1,
    THOUSAND_MOONS_SMITHY_ALTERNATE_1,
    UNSTABLE_GLYPHBRIDGE_ALTERNATE_1,
    WARDEN_OF_THE_INNER_SKY_ALTERNATE_1,
    BRAIDED_NET_ALTERNATE_1,
    DEEPROOT_PILGRIMAGE_ALTERNATE_1,
    THE_ENIGMA_JEWEL_ALTERNATE_1,
    THE_EVERFLOWING_WELL_ALTERNATE_1,
    KITESAIL_LARCENIST_ALTERNATE_1,
    SUBTERRANEAN_SCHOONER_ALTERNATE_1,
    CORPSES_OF_THE_LOST_ALTERNATE_1,
    QUEEN_S_BAY_PALADIN_ALTERNATE_1,
    SOULS_OF_THE_LOST_ALTERNATE_1,
    STALACTITE_STALKER_ALTERNATE_1,
    TARRIAN_S_JOURNAL_ALTERNATE_1,
    TERROR_TIDE_ALTERNATE_1,
    BRASS_S_TUNNEL_GRINDER_ALTERNATE_1,
    DIRE_FLAIL_ALTERNATE_1,
    HIT_THE_MOTHER_LODE_ALTERNATE_1,
    MAGMATIC_GALLEON_ALTERNATE_1,
    POETIC_INGENUITY_ALTERNATE_1,
    BEDROCK_TORTOISE_ALTERNATE_1,
    COSMIUM_CONFLUENCE_ALTERNATE_1,
    GROWING_RITES_OF_ITLIMOC_ALTERNATE_1,
    INTREPID_PALEONTOLOGIST_ALTERNATE_1,
    JADELIGHT_SPELUNKER_ALTERNATE_1,
    SENTINEL_OF_THE_NAMELESS_CITY_ALTERNATE_1,
    THE_BELLIGERENT_ALTERNATE_1,
    DEEPFATHOM_ECHO_ALTERNATE_1,
    SQUIRMING_EMERGENCE_ALTERNATE_1,
    MATZALANTLI_THE_GREAT_DOOR_ALTERNATE_1,
    THE_MILLENNIUM_CALENDAR_ALTERNATE_1,
    TARRIAN_S_SOULCLEAVER_ALTERNATE_1,
    THREEFOLD_THUNDERHULK_ALTERNATE_1,
    TREASURE_MAP_ALTERNATE_1,
    SUNKEN_CITADEL_ALTERNATE_1,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
    JADELIGHT_SPELUNKER_ALTERNATE_2,
    HIT_THE_MOTHER_LODE_ALTERNATE_2,
    SPYGLASS_SIREN_ALTERNATE_1,
    DEEP_CAVERN_BAT_ALTERNATE_1,
    GEOLOGICAL_APPRAISER_ALTERNATE_1,
    CENOTE_SCOUT_ALTERNATE_1,
    BARTOLOME_DEL_PRESIDIO_ALTERNATE_2,
    CAVERN_OF_SOULS_ALTERNATE_2,
    CAVERN_OF_SOULS_ALTERNATE_3,
    CAVERN_OF_SOULS_ALTERNATE_4,
    CAVERN_OF_SOULS_ALTERNATE_5,
    CAVERN_OF_SOULS_ALTERNATE_6,
    CAVERN_OF_SOULS_ALTERNATE_7,
];
