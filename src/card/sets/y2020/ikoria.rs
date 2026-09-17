//! Ikoria: Lair of Behemoths cards and complete paper-printing inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityPredicateDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AggregateOperationDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BattlefieldArrivalDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BlockRestrictionDef;
use crate::card::CardArt;
use crate::card::CardNameDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseCardsFromCollectionDef;
use crate::card::ChooseDef;
use crate::card::CollectionInspectionDef;
use crate::card::ColorSet;
use crate::card::ComparisonDef;
use crate::card::ControlDurationDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CountConditionDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatedTokensDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DamageEventMatcherDef;
use crate::card::DamageKindDef;
use crate::card::DamagePreventionDef;
use crate::card::DamageRecipientMatcherDef;
use crate::card::DamageSourceMatcherDef;
use crate::card::DiscardFollowUpDef;
use crate::card::DiscardSelectionDef;
use crate::card::DrawEventMatcherDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::EmblemCharacteristics;
use crate::card::IfNoObjectsDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ManaRestrictionDef;
use crate::card::MoveObjectsDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectValueAggregateDef;
use crate::card::ObjectValueDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayPermissionDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SpellResolutionDestinationDef;
use crate::card::SubtypeDef;
use crate::card::SumValueDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenCopyDef;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::{
    CardProperty, CardRequirement, CompanionDef, DeckCards, DeckRequirementDef,
    DeclarativeAbilityDef,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Companion keeps its requirement in the printed ability clause. Selection,
/// revelation, and the once-per-game special action belong to the mechanic.
pub(crate) const fn companion(text: &'static str, requirement: DeckRequirementDef) -> AbilityDef {
    AbilityDef::defined(
        text,
        DeclarativeAbilityDef::Companion(CompanionDef { requirement }),
        EffectDef::None,
    )
}

/// A triome is a tapped land with three basic land types and cycling, and
/// nothing else. Its printed mana ability is reminder text for what the
/// subtypes already grant, so it is not restated as a clause.
const TRIOME_ABILITIES: &[AbilityDef] = &[
    abilities::enters_tapped(CardType::Land),
    abilities::cycling!(
        "Cycling {3} ({3}, Discard this card: Draw a card.)",
        &[CostDef::Mana(mana_cost!("{3}"))],
    ),
];

const fn triome(types: &'static [&'static str]) -> CardRules {
    CardRules::new_land(types).with_abilities(TRIOME_ABILITIES)
}

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "IKO",
    slug: "ikoria",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const HUMAN_SOLDIER_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Human", "Soldier"], &[ManaColor::White], 1, 1).with_art(
        CardArt::new("90de6d1e-e654-4f4a-8014-061ce69e540f", "Manuel Castañón"),
    );

// IKO 1 — Adaptive Shimmerer
pub(in crate::card::sets) static ADAPTIVE_SHIMMERER: CardRecord = CardRecord::new(
    "Adaptive Shimmerer",
    "d8a2e243-e446-46c6-8a37-e26620951c41",
    "Jason Felix",
    CardRules::new_creature(mana_cost!("{5}"), &["Insect"], 0, 0).with_abilities(&[
        abilities::flash(),
        AbilityDef::as_enters(
            "This creature enters with three +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 3,
                },
            ),
        ),
    ]),
);

// IKO 2 — Farfinder
pub(in crate::card::sets) static FARFINDER: CardRecord = CardRecord::new(
    "Farfinder",
    "53f63f40-46f1-4b9e-b447-ff9274f2b926",
    "Leesha Hannigan",
    CardRules::new_creature(mana_cost!("{3}"), &["Fox"], 1, 1).with_abilities(&[
        abilities::vigilance(),
        abilities::enters_trigger(
            "When this creature enters, you may search your library for a \
             basic land card, reveal it, put it into your hand, then \
             shuffle.",
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Hand,
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

// IKO 3 — Mysterious Egg
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static MYSTERIOUS_EGG: CardRecord = CardRecord::new(
    "Mysterious Egg",
    "cdfac3c3-7f38-49d6-bd88-dc0f5001116f",
    "Volkan Baǵa",
    CardRules::unsupported(),
);

// IKO 4 — Blade Banish
pub(in crate::card::sets) static BLADE_BANISH: CardRecord = CardRecord::new(
    "Blade Banish",
    "35a30091-7f68-4a67-b47e-f44318fc93b2",
    "Lie Setiawan",
    CardRules::new_instant(mana_cost!("{3}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Exile target creature with power 4 or greater.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::PowerAtLeast(4),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::move_to_zone(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ZoneKind::Exile,
            ZonePlacement::Top,
        ),
    )]),
);

// IKO 5 — Checkpoint Officer
pub(in crate::card::sets) static CHECKPOINT_OFFICER: CardRecord = CardRecord::new(
    "Checkpoint Officer",
    "4a0e12f5-7b15-4a8c-b045-35bcbf1fbb90",
    "Manuel Castañón",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 1, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}{W}, {T}: Tap target creature.",
            &[CostDef::Mana(mana_cost!("{1}{W}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// IKO 6 — Coordinated Charge
pub(in crate::card::sets) static COORDINATED_CHARGE: CardRecord = CardRecord::new(
    "Coordinated Charge",
    "129c404b-2c1e-4f0b-bb4e-7e8e627a69a8",
    "Zoltan Boros",
    CardRules::new_instant(mana_cost!("{4}{W}")).with_abilities(&[
        AbilityDef::spell(
            "Creatures you control get +2/+1 until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
    ]),
);

// IKO 7 — Cubwarden
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static CUBWARDEN: CardRecord = CardRecord::new(
    "Cubwarden",
    "cc322744-5d8f-448d-b868-381bb86b68f9",
    "Kekai Kotaki",
    CardRules::unsupported(),
);

// IKO 8 — Daysquad Marshal
pub(in crate::card::sets) static DAYSQUAD_MARSHAL: CardRecord = CardRecord::new(
    "Daysquad Marshal",
    "17d566bd-f272-49d1-bffb-588f2a42046a",
    "Micah Epstein",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Soldier"], 3, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 white Human Soldier \
             creature token.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(HUMAN_SOLDIER_TOKEN))),
        ),
    ]),
);

// IKO 9 — Divine Arrow (reprint)
const DIVINE_ARROW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::war_of_the_spark::DIVINE_ARROW,
    "5b29acc2-5749-4e8a-924d-3c82406e6393",
    "Slawomir Maniak",
);

// IKO 10 — Drannith Healer
// Audit: unsupported — Needs a battlefield listener for another card being cycled; DiscardedToActivate only visits
// the discarded card itself.
pub(in crate::card::sets) static DRANNITH_HEALER: CardRecord = CardRecord::new(
    "Drannith Healer",
    "ff5a821c-eaec-4f69-97c7-8299cdebc2f4",
    "Scott Murphy",
    CardRules::unsupported(),
);

// IKO 11 — Drannith Magistrate
pub(in crate::card::sets) static DRANNITH_MAGISTRATE: CardRecord = CardRecord::new(
    "Drannith Magistrate",
    "98b0a4a8-9319-451b-9b79-b0bca7a41e91",
    "Kieran Yanner",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Wizard"], 1, 3).with_abilities(&[
        AbilityDef::static_ability(
            "Your opponents can't cast spells from anywhere other than \
             their hands.",
            EffectDef::Sequence(&[
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Opponent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                        PlayRestrictionDef::new(
                            PlayActionMatcherDef::CastSpell,
                            ObjectPredicateDef::Any,
                        )
                        .from_zone(ZoneKind::Library),
                    )),
                },
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Opponent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                        PlayRestrictionDef::new(
                            PlayActionMatcherDef::CastSpell,
                            ObjectPredicateDef::Any,
                        )
                        .from_zone(ZoneKind::Battlefield),
                    )),
                },
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Opponent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                        PlayRestrictionDef::new(
                            PlayActionMatcherDef::CastSpell,
                            ObjectPredicateDef::Any,
                        )
                        .from_zone(ZoneKind::Graveyard),
                    )),
                },
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Opponent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                        PlayRestrictionDef::new(
                            PlayActionMatcherDef::CastSpell,
                            ObjectPredicateDef::Any,
                        )
                        .from_zone(ZoneKind::Stack),
                    )),
                },
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Opponent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                        PlayRestrictionDef::new(
                            PlayActionMatcherDef::CastSpell,
                            ObjectPredicateDef::Any,
                        )
                        .from_zone(ZoneKind::Exile),
                    )),
                },
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Opponent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                        PlayRestrictionDef::new(
                            PlayActionMatcherDef::CastSpell,
                            ObjectPredicateDef::Any,
                        )
                        .from_zone(ZoneKind::Command),
                    )),
                },
            ]),
        ),
    ]),
);

// IKO 12 — Fight as One
pub(in crate::card::sets) static FIGHT_AS_ONE: CardRecord = CardRecord::new(
    "Fight as One",
    "5c62ea13-9bd5-46ce-a861-68cf9a2c6f8c",
    "Bryan Sola",
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one or both —",
        &[
            AbilityDef::spell_with_targets(
                "Target Human creature you control gets +1/+1 and gains \
                 indestructible until end of turn.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::from_name("Human")),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
            AbilityDef::spell_with_targets(
                "Target non-Human creature you control gets +1/+1 and gains \
                 indestructible until end of turn.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(
                                SubtypeDef::from_name("Human"),
                            )),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
        ],
    )
    .with_mode_selection(1, 2, false)]),
);

// IKO 13 — Flourishing Fox
// Audit: unsupported — Needs a battlefield listener for another card being cycled; DiscardedToActivate only visits
// the discarded card itself.
pub(in crate::card::sets) static FLOURISHING_FOX: CardRecord = CardRecord::new(
    "Flourishing Fox",
    "78b81339-746d-4f71-9943-01c0f6a5683a",
    "Ilse Gort",
    CardRules::unsupported(),
);

// IKO 14 — Garrison Cat
pub(in crate::card::sets) static GARRISON_CAT: CardRecord = CardRecord::new(
    "Garrison Cat",
    "e7ff8345-227c-43b4-bed5-af3a34c0a990",
    "Sidharth Chaturvedi",
    CardRules::new_creature(mana_cost!("{W}"), &["Cat"], 1, 1).with_abilities(&[
        abilities::dies_trigger(
            "When this creature dies, create a 1/1 white Human Soldier \
             creature token.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(HUMAN_SOLDIER_TOKEN))),
        ),
    ]),
);

// IKO 15 — Helica Glider
// Audit: unsupported — Needs a mandatory choice between keyword-counter entry modifications before the permanent
// enters.
pub(in crate::card::sets) static HELICA_GLIDER: CardRecord = CardRecord::new(
    "Helica Glider",
    "8fc88fe0-2424-49a7-9abf-4773a547d010",
    "Jehan Choo",
    CardRules::unsupported(),
);

// IKO 16 — Huntmaster Liger
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static HUNTMASTER_LIGER: CardRecord = CardRecord::new(
    "Huntmaster Liger",
    "058d6a7d-12b0-4d82-a5bd-91fa44bbc5e1",
    "Leesha Hannigan",
    CardRules::unsupported(),
);

// IKO 17 — Imposing Vantasaur
pub(in crate::card::sets) static IMPOSING_VANTASAUR: CardRecord = CardRecord::new(
    "Imposing Vantasaur",
    "b6fa5feb-f5e9-4079-acc9-84e458044769",
    "Jonathan Kuo",
    CardRules::new_creature(mana_cost!("{5}{W}"), &["Dinosaur"], 3, 6).with_abilities(&[
        abilities::vigilance(),
        abilities::cycling!("Cycling {1}", &[CostDef::Mana(mana_cost!("{1}"))]),
    ]),
);

// IKO 18 — Keensight Mentor
pub(in crate::card::sets) static KEENSIGHT_MENTOR: CardRecord = CardRecord::new(
    "Keensight Mentor",
    "a0583065-9a6c-4ea4-b6dd-2edfbaed1181",
    "Yongjae Choi",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Cleric"], 1, 4).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, put a vigilance counter on target \
             non-Human creature you control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(
                            SubtypeDef::from_name("Human"),
                        )),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::Vigilance,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "{1}{W}, {T}: Put a +1/+1 counter on each creature you \
             control with vigilance.",
            &[CostDef::Mana(mana_cost!("{1}{W}")), CostDef::TapSource],
            EffectDef::AddCounters {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Vigilance),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// IKO 19 — Lavabrink Venturer
// Audit: unsupported — Needs an odd/even entry designation and protection matching the chosen mana-value parity.
pub(in crate::card::sets) static LAVABRINK_VENTURER: CardRecord = CardRecord::new(
    "Lavabrink Venturer",
    "38c0d9d6-c9c7-4169-8106-400445643a1a",
    "Zoltan Boros",
    CardRules::unsupported(),
);

// IKO 20 — Light of Hope
pub(in crate::card::sets) static LIGHT_OF_HOPE: CardRecord = CardRecord::new(
    "Light of Hope",
    "bcb00599-e082-49b0-88f3-ef91b75595e4",
    "Kimonas Theodossiou",
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "You gain 4 life.",
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(4),
                },
            ),
            AbilityDef::spell_with_targets(
                "Destroy target enchantment.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Enchantment),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::destroy_target(TargetIndex::PRIMARY),
            ),
            AbilityDef::spell_with_targets(
                "Put a +1/+1 counter on target creature.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ],
    )]),
);

// IKO 21 — Luminous Broodmoth
pub(in crate::card::sets) static LUMINOUS_BROODMOTH: CardRecord = CardRecord::new(
    "Luminous Broodmoth",
    "bb65df55-d6a6-4a57-a903-e5eb17637982",
    "Lie Setiawan",
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Insect"], 3, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever a creature you control without flying dies, return \
             it to the battlefield under its owner's control with a \
             flying counter on it.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                            KeywordAbility::Flying,
                        )),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::WithBattlefieldArrival {
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::TriggeringZoneChangeResult,
                    ZoneKind::Battlefield,
                    ZonePlacement::Top,
                ),
                arrival: BattlefieldArrivalDef {
                    controller: None,
                    modifications: &[BattlefieldEntryModificationDef::AddCounters {
                        kind: CounterKind::Flying,
                        amount: 1,
                    }],
                    attachment: None,
                    counters: None,
                },
            },
        ),
    ]),
);

// IKO 22 — Majestic Auricorn
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static MAJESTIC_AURICORN: CardRecord = CardRecord::new(
    "Majestic Auricorn",
    "8df320c9-449a-457e-8688-f611aceb3352",
    "Jonathan Kuo",
    CardRules::unsupported(),
);

// IKO 23 — Maned Serval
pub(in crate::card::sets) static MANED_SERVAL: CardRecord = CardRecord::new(
    "Maned Serval",
    "5ac51e35-e2c5-4457-981c-e59894584288",
    "Jonathan Kuo",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Cat"], 1, 4)
        .with_abilities(&[abilities::vigilance()]),
);

// IKO 24 — Mythos of Snapdax
// Audit: unsupported — Needs the colors actually spent on this spell and replacement of each player's selection
// with the caster's choices.
pub(in crate::card::sets) static MYTHOS_OF_SNAPDAX: CardRecord = CardRecord::new(
    "Mythos of Snapdax",
    "2712a1a3-dd28-44c8-a661-5bcf68d3acaa",
    "Seb McKinnon",
    CardRules::unsupported(),
);

// IKO 25 — Pacifism (reprint)
const PACIFISM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::mirage::PACIFISM,
    "47adc9fc-d620-4cb7-a03c-8d9578992249",
    "Kev Walker",
);

// IKO 26 — Patagia Tiger
pub(in crate::card::sets) static PATAGIA_TIGER: CardRecord = CardRecord::new(
    "Patagia Tiger",
    "be398100-89e2-432b-8017-74fb7e4dbc26",
    "Micah Epstein",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Cat"], 3, 4).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, target Human you control gets \
             +2/+2 until end of turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::from_name("Human")),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// IKO 27 — Perimeter Sergeant
pub(in crate::card::sets) static PERIMETER_SERGEANT: CardRecord = CardRecord::new(
    "Perimeter Sergeant",
    "2a174587-d964-48b3-8f69-fa576c1b8bba",
    "Stepan Alekseev",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 3, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks, other Humans you control get \
             +1/+0 until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::from_name("Human")),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// IKO 28 — Sanctuary Lockdown
// Audit: unsupported — Needs joint payment planning for a mana cost combined with tapping multiple creatures.
pub(in crate::card::sets) static SANCTUARY_LOCKDOWN: CardRecord = CardRecord::new(
    "Sanctuary Lockdown",
    "1387b7be-f69e-4919-9070-a89654c656f3",
    "Lius Lasahido",
    CardRules::unsupported(),
);

// IKO 29 — Savai Sabertooth
pub(in crate::card::sets) static SAVAI_SABERTOOTH: CardRecord = CardRecord::new(
    "Savai Sabertooth",
    "d46702b0-7e10-462a-9aac-7564efe91804",
    "Ilse Gort",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Cat"], 3, 1),
);

// IKO 30 — Snare Tactician
// Audit: unsupported — Needs a battlefield listener for another card being cycled; DiscardedToActivate only visits
// the discarded card itself.
pub(in crate::card::sets) static SNARE_TACTICIAN: CardRecord = CardRecord::new(
    "Snare Tactician",
    "5f640e0c-298d-4a5f-94d0-d839cf06ca86",
    "Micah Epstein",
    CardRules::unsupported(),
);

// IKO 31 — Solid Footing
pub(in crate::card::sets) static SOLID_FOOTING: CardRecord = CardRecord::new(
    "Solid Footing",
    "0900a8d1-a1b1-48c9-8af1-726a623c7d74",
    "Wisnu Tan",
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::static_ability(
                "As long as enchanted creature has vigilance, it assigns \
                 combat damage equal to its toughness rather than its power.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::AttachedPermanentMatches {
                        object: ObjectPredicateDef::HasKeyword(KeywordAbility::Vigilance),
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::Rule(
                            AppliedRuleDef::AssignsCombatDamageEqualToToughness,
                        ),
                    },
                },
            ),
        ]),
);

// IKO 32 — Splendor Mare
pub(in crate::card::sets) static SPLENDOR_MARE: CardRecord = CardRecord::new(
    "Splendor Mare",
    "535ccc75-b24e-4071-b6a0-fc5267a454e3",
    "Lucas Graciano",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Elk", "Unicorn"], 3, 3).with_abilities(&[
        abilities::lifelink(),
        abilities::cycling!("Cycling {1}{W}", &[CostDef::Mana(mana_cost!("{1}{W}"))]),
        AbilityDef::triggered_with_targets(
            "When you cycle this card, put a lifelink counter on target \
             creature you control.",
            TriggerEventDef::DiscardedToActivate(abilities::CYCLING),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::Lifelink,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// IKO 33 — Spontaneous Flight
pub(in crate::card::sets) static SPONTANEOUS_FLIGHT: CardRecord = CardRecord::new(
    "Spontaneous Flight",
    "3dc24dd4-d259-4687-8247-f56aa7abb5b9",
    "Gabor Szikszai",
    CardRules::new_instant(mana_cost!("{2}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +2/+2 until end of turn. Put a flying \
         counter on it.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::Flying,
                amount: ValueDef::Constant(1),
            },
        ]),
    )]),
);

// IKO 34 — Stormwild Capridor
// Audit: unsupported — Needs a damage-prevention follow-up that puts counters equal to damage actually prevented;
// current prevention follow-ups only gain life.
pub(in crate::card::sets) static STORMWILD_CAPRIDOR: CardRecord = CardRecord::new(
    "Stormwild Capridor",
    "2afded81-2fc1-4285-bf48-d25f72e72138",
    "Dmitry Burmak",
    CardRules::unsupported(),
);

// IKO 35 — Swallow Whole
pub(in crate::card::sets) static SWALLOW_WHOLE: CardRecord = CardRecord::new(
    "Swallow Whole",
    "c3b9140b-848f-4886-8c28-eccd3829a607",
    "Svetlin Velinov",
    CardRules::new_sorcery(mana_cost!("{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "As an additional cost to cast this spell, tap an untapped \
         creature you control.\nExile target tapped creature. Put a \
         +1/+1 counter on the creature tapped to pay this spell's \
         additional cost.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Tapped,
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::object(ObjectRefDef::AdditionalCostObject(
                    crate::AdditionalCostObjectIndex::PRIMARY,
                )),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ]),
    )
    .with_spell_additional_cost(&CostDef::TapPermanents {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        controller: PlayerRelation::You,
        count: 1,
    })]),
);

// IKO 36 — Valiant Rescuer
// Audit: unsupported — Needs global cycling events and the ordinal of cycling another card this turn, including
// cycles before this permanent entered.
pub(in crate::card::sets) static VALIANT_RESCUER: CardRecord = CardRecord::new(
    "Valiant Rescuer",
    "a2b13ff6-67e9-4760-aef4-a8548ea44344",
    "Bram Sels",
    CardRules::unsupported(),
);

// IKO 37 — Vulpikeet
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static VULPIKEET: CardRecord = CardRecord::new(
    "Vulpikeet",
    "07bce816-3d18-4ca1-b88f-7307ec7c345d",
    "Leesha Hannigan",
    CardRules::unsupported(),
);

// IKO 38 — Will of the All-Hunter
pub(in crate::card::sets) static WILL_OF_THE_ALL_HUNTER: CardRecord = CardRecord::new(
    "Will of the All-Hunter",
    "aa7acbfa-ce3b-424d-886d-6865bc5fc085",
    "Viktor Titov",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature gets +2/+2 until end of turn. If it's \
             blocking, instead put two +1/+1 counters on it.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::Blocking,
                },
                then: &EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(2),
                },
                otherwise: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            },
        ),
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
    ]),
);

// IKO 39 — Aegis Turtle
pub(in crate::card::sets) static AEGIS_TURTLE: CardRecord = CardRecord::new(
    "Aegis Turtle",
    "e433e7f0-7417-4dfe-a7a4-3f222b0a835f",
    "Milivoj Ćeran",
    CardRules::new_creature(mana_cost!("{U}"), &["Turtle"], 0, 5),
);

// IKO 40 — Anticipate (reprint)
const ANTICIPATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::dragons_of_tarkir::ANTICIPATE,
    "46b6dfb2-ffe4-44fb-bfd9-df11bc3193df",
    "Kieran Yanner",
);

// IKO 41 — Archipelagore
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static ARCHIPELAGORE: CardRecord = CardRecord::new(
    "Archipelagore",
    "f24b1030-64d5-4c94-a04c-1d9520bfddab",
    "Svetlin Velinov",
    CardRules::unsupported(),
);

// IKO 42 — Avian Oddity
pub(in crate::card::sets) static AVIAN_ODDITY: CardRecord = CardRecord::new(
    "Avian Oddity",
    "f325873b-97de-4701-910f-ec5cdb66de33",
    "Simon Dominic",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Bird"], 2, 4).with_abilities(&[
        abilities::flying(),
        abilities::cycling!("Cycling {2}{U}", &[CostDef::Mana(mana_cost!("{2}{U}"))]),
        AbilityDef::triggered_with_targets(
            "When you cycle this card, put a flying counter on target \
             creature you control.",
            TriggerEventDef::DiscardedToActivate(abilities::CYCLING),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::Flying,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// IKO 43 — Boon of the Wish-Giver
pub(in crate::card::sets) static BOON_OF_THE_WISH_GIVER: CardRecord = CardRecord::new(
    "Boon of the Wish-Giver",
    "0e790851-f0f7-4f1a-80e6-94be649499b6",
    "Chris Rahn",
    CardRules::new_sorcery(mana_cost!("{4}{U}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Draw four cards.",
            abilities::draw_cards(ValueDef::Constant(4)),
        ),
        abilities::cycling!("Cycling {1}", &[CostDef::Mana(mana_cost!("{1}"))]),
    ]),
);

// IKO 44 — Capture Sphere (reprint)
const CAPTURE_SPHERE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::guilds_of_ravnica::CAPTURE_SPHERE,
    "3c1d2d9d-6eee-4334-bbbe-87e026fa6fc0",
    "Jakub Kasper",
);

// IKO 45 — Convolute (reprint)
const CONVOLUTE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::ravnica_city_of_guilds::CONVOLUTE,
    "3fd8e607-8179-4ae8-ba7f-f5f22649dc18",
    "Jason Rainville",
);

// IKO 46 — Crystacean
pub(in crate::card::sets) static CRYSTACEAN: CardRecord = CardRecord::new(
    "Crystacean",
    "32afec8a-dbae-446e-919a-3efb556f5cb1",
    "Mathias Kollros",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Crab"], 1, 6)
        .with_abilities(&[abilities::flash()]),
);

// IKO 47 — Dreamtail Heron
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static DREAMTAIL_HERON: CardRecord = CardRecord::new(
    "Dreamtail Heron",
    "ca777d40-736a-4cd6-85c4-5afd52f2fadd",
    "Caio Monteiro",
    CardRules::unsupported(),
);

// IKO 48 — Escape Protocol
// Audit: unsupported — Needs a battlefield listener for another card being cycled; DiscardedToActivate only visits
// the discarded card itself.
pub(in crate::card::sets) static ESCAPE_PROTOCOL: CardRecord = CardRecord::new(
    "Escape Protocol",
    "2d3eeddd-c86d-4f35-ba57-3caa9565fcb0",
    "Deruchenko Alexander",
    CardRules::unsupported(),
);

// IKO 49 — Essence Scatter (reprint)
const ESSENCE_SCATTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::magic_2010::ESSENCE_SCATTER,
    "5f79c8a0-291e-4e13-b765-4cf8c726cf30",
    "Seb McKinnon",
);

// IKO 50 — Facet Reader
pub(in crate::card::sets) static FACET_READER: CardRecord = CardRecord::new(
    "Facet Reader",
    "b1199596-ae77-4192-ae70-3e2ebd009b64",
    "Matt Stewart",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Wizard"], 1, 2).with_abilities(&[
        AbilityDef::activated(
            "{1}, {T}: Draw a card, then discard a card.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
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
    ]),
);

// IKO 51 — Frost Lynx (reprint)
const FROST_LYNX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::magic_2015::FROST_LYNX,
    "dfcaeccc-fc8c-4a9e-80d5-b48da71d7ff1",
    "Ilse Gort",
);

// IKO 52 — Frostveil Ambush
pub(in crate::card::sets) static FROSTVEIL_AMBUSH: CardRecord = CardRecord::new(
    "Frostveil Ambush",
    "deef1197-e40a-4c9d-919a-213f1bdf3e3e",
    "Johan Grenier",
    CardRules::new_instant(mana_cost!("{3}{U}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Tap up to two target creatures. Those creatures don't untap \
             during their controller's next untap step.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                2,
            )],
            EffectDef::Sequence(&[
                EffectDef::Tap {
                    object: EffectRecipientDef::target_objects(TargetIndex::PRIMARY),
                },
                EffectDef::SkipNextUntapSteps {
                    object: EffectRecipientDef::target_objects(TargetIndex::PRIMARY),
                    count: 1,
                },
            ]),
        ),
        abilities::cycling!("Cycling {1}", &[CostDef::Mana(mana_cost!("{1}"))]),
    ]),
);

// IKO 53 — Glimmerbell
pub(in crate::card::sets) static GLIMMERBELL: CardRecord = CardRecord::new(
    "Glimmerbell",
    "54f71f8a-546a-465e-a254-09a3ae873ef4",
    "Simon Dominic",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Elemental", "Jellyfish"], 1, 3)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::activated(
                "{1}{U}: Untap this creature.",
                &[CostDef::Mana(mana_cost!("{1}{U}"))],
                EffectDef::Untap {
                    object: EffectRecipientDef::Source,
                },
            ),
        ]),
);

// IKO 54 — Gust of Wind
pub(in crate::card::sets) static GUST_OF_WIND: CardRecord = CardRecord::new(
    "Gust of Wind",
    "faabea92-d8cf-4591-b528-3e597d56b31f",
    "Adam Paquette",
    CardRules::new_sorcery(mana_cost!("{3}{U}")).with_abilities(&[
        abilities::this_spell_cost_reduction(
            "This spell costs {2} less to cast if you control a creature \
             with flying.",
            ValueDef::IfMatchingObjectCount(&CountConditionDef {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
                then: ValueDef::Constant(2),
                otherwise: ValueDef::Constant(0),
            }),
        ),
        AbilityDef::spell_with_targets(
            "Return target nonland permanent you don't control to its \
             owner's hand.\nDraw a card.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
                abilities::draw_cards(ValueDef::Constant(1)),
            ]),
        ),
    ]),
);

// IKO 55 — Hampering Snare
pub(in crate::card::sets) static HAMPERING_SNARE: CardRecord = CardRecord::new(
    "Hampering Snare",
    "5f7b331a-bd21-429b-a49f-88da9a31c98e",
    "Svetlin Velinov",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Creatures your opponents control get -2/-0 until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
    ]),
);

// IKO 56 — Keep Safe
pub(in crate::card::sets) static KEEP_SAFE: CardRecord = CardRecord::new(
    "Keep Safe",
    "febfa682-76ae-4979-a40c-c1eae1121f3c",
    "Ekaterina Burmak",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Counter target spell that targets a permanent you control.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Spell,
                    ObjectPredicateDef::TargetsObjectMatching(&ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Any,
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ])),
                ]),
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::counter_target(TargetIndex::PRIMARY),
            abilities::draw_cards(ValueDef::Constant(1)),
        ]),
    )]),
);

// IKO 57 — Mystic Subdual
pub(in crate::card::sets) static MYSTIC_SUBDUAL: CardRecord = CardRecord::new(
    "Mystic Subdual",
    "627c2111-b7d7-40bc-aa30-57d72338b32d",
    "Filip Burburan",
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets -2/-0 and loses all abilities. \
                 (Mutating onto the creature won't give it new abilities. It \
                 can gain abilities in other ways.)",
                EffectDef::Sequence(&[
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(-2),
                            ValueDef::Constant(0),
                        ),
                    },
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
                    },
                ]),
            ),
        ]),
);

// IKO 58 — Mythos of Illuna
// Audit: unsupported — Needs a predicate for the specific colors of mana actually spent to cast this spell,
// including alternative payments.
pub(in crate::card::sets) static MYTHOS_OF_ILLUNA: CardRecord = CardRecord::new(
    "Mythos of Illuna",
    "f1b07082-c5a5-4283-a2f5-5b1e0120d752",
    "Seb McKinnon",
    CardRules::unsupported(),
);

// IKO 59 — Neutralize
pub(in crate::card::sets) static NEUTRALIZE: CardRecord = CardRecord::new(
    "Neutralize",
    "0430da3c-9460-4b62-ae28-2e7e6f4d06a4",
    "Yongjae Choi",
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Counter target spell.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Spell,
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::counter_target(TargetIndex::PRIMARY),
        ),
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
    ]),
);

// IKO 60 — Of One Mind
pub(in crate::card::sets) static OF_ONE_MIND: CardRecord = CardRecord::new(
    "Of One Mind",
    "c95fb136-f21d-4f3a-82b7-bcf490b7e90c",
    "Matt Stewart",
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_abilities(&[
        abilities::this_spell_cost_reduction(
            "This spell costs {2} less to cast if you control a Human \
             creature and a non-Human creature.",
            ValueDef::IfMatchingObjectCount(&CountConditionDef {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::from_name("Human")),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
                then: ValueDef::IfMatchingObjectCount(&CountConditionDef {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(
                                SubtypeDef::from_name("Human"),
                            )),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                    then: ValueDef::Constant(2),
                    otherwise: ValueDef::Constant(0),
                }),
                otherwise: ValueDef::Constant(0),
            }),
        ),
        AbilityDef::spell(
            "Draw two cards.",
            abilities::draw_cards(ValueDef::Constant(2)),
        ),
    ]),
);

// IKO 61 — Ominous Seas
pub(in crate::card::sets) static OMINOUS_SEAS: CardRecord = CardRecord::new(
    "Ominous Seas",
    "ce8965f2-756a-4461-a643-db024a11c2de",
    "Vincent Proce",
    CardRules::new_enchantment(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you draw a card, put a foreshadow counter on this \
             enchantment.",
            TriggerEventDef::DrewCard(DrawEventMatcherDef::any(PlayerRelation::You)),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("foreshadow"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "Remove eight foreshadow counters from this enchantment: \
             Create an 8/8 blue Kraken creature token.",
            &[CostDef::RemoveCountersFromSource {
                kind: CounterKind::named("foreshadow"),
                amount: 8,
            }],
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Kraken"], &[ManaColor::Blue], 8, 8).with_art(
                    CardArt::new("ca17c7b2-180a-4bd1-9ab2-152f8f656dba", "Vincent Proce"),
                ),
            ))),
        ),
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
    ]),
);

// IKO 62 — Phase Dolphin
pub(in crate::card::sets) static PHASE_DOLPHIN: CardRecord = CardRecord::new(
    "Phase Dolphin",
    "88fb4042-e9c1-4b0f-b1f6-4180b0d79663",
    "Lie Setiawan",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Elemental", "Whale"], 1, 4).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, another target attacking \
             creature can't be blocked this turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Attacking,
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// IKO 63 — Pollywog Symbiote
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static POLLYWOG_SYMBIOTE: CardRecord = CardRecord::new(
    "Pollywog Symbiote",
    "77adf375-e7d7-4ebd-8c86-ad7e822a5483",
    "Simon Dominic",
    CardRules::unsupported(),
);

// IKO 64 — Pouncing Shoreshark
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static POUNCING_SHORESHARK: CardRecord = CardRecord::new(
    "Pouncing Shoreshark",
    "c859b339-b55b-41fe-948c-27502e3b3ea8",
    "Dan Murayama Scott",
    CardRules::unsupported(),
);

// IKO 65 — Reconnaissance Mission
pub(in crate::card::sets) static RECONNAISSANCE_MISSION: CardRecord = CardRecord::new(
    "Reconnaissance Mission",
    "ae9f7efa-d125-4f83-825e-172ea099a62a",
    "Johannes Voss",
    CardRules::new_enchantment(mana_cost!("{2}{U}{U}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever a creature you control deals combat damage to a \
             player, you may draw a card.",
            TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                kind: DamageKindDef::Combat,
                source: DamageSourceMatcherDef::Matching(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                recipient: DamageRecipientMatcherDef::Recipients(EffectRecipientDef::EachPlayer),
            }),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &abilities::draw_cards(ValueDef::Constant(1)),
            },
        ),
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
    ]),
);

// IKO 66 — Sea-Dasher Octopus
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static SEA_DASHER_OCTOPUS: CardRecord = CardRecord::new(
    "Sea-Dasher Octopus",
    "a6f59d9e-6c1b-437a-934c-7c776c8dd1d5",
    "Chris Seaman",
    CardRules::unsupported(),
);

// IKO 67 — Shark Typhoon
// Audit: unsupported — Needs the X paid for cycling preserved in the discarded card's trigger; DiscardedToActivate
// currently captures X as zero.
pub(in crate::card::sets) static SHARK_TYPHOON: CardRecord = CardRecord::new(
    "Shark Typhoon",
    "1da4d4f3-b3cb-4b61-81b8-06ae441c41bf",
    "Caio Monteiro",
    CardRules::unsupported(),
);

// IKO 68 — Startling Development
pub(in crate::card::sets) static STARTLING_DEVELOPMENT: CardRecord = CardRecord::new(
    "Startling Development",
    "a12680db-957f-48c1-9062-2edd9115ba26",
    "Simon Dominic",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Until end of turn, target creature becomes a blue Serpent \
             with base power and toughness 4/4.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Blue])),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                        "Serpent",
                    ])),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(4),
                        ValueDef::Constant(4),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
        abilities::cycling!("Cycling {1}", &[CostDef::Mana(mana_cost!("{1}"))]),
    ]),
);

// IKO 69 — Thieving Otter
pub(in crate::card::sets) static THIEVING_OTTER: CardRecord = CardRecord::new(
    "Thieving Otter",
    "07f84b0a-37d9-4b0f-8d75-1fab45a12d44",
    "Jakub Kasper",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Otter"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature deals damage to an opponent, draw a card.",
            TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                kind: DamageKindDef::Any,
                source: DamageSourceMatcherDef::Matching(ObjectPredicateDef::Source),
                recipient: DamageRecipientMatcherDef::Recipients(EffectRecipientDef::Opponent),
            }),
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// IKO 70 — Voracious Greatshark
pub(in crate::card::sets) static VORACIOUS_GREATSHARK: CardRecord = CardRecord::new(
    "Voracious Greatshark",
    "1400155f-8911-45fd-aab2-998c8a28292c",
    "Mathias Kollros",
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Shark"], 5, 4).with_abilities(&[
        abilities::flash(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, counter target artifact or \
             creature spell.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::counter_target(TargetIndex::PRIMARY),
        ),
    ]),
);

// IKO 71 — Wingfold Pteron
// Audit: unsupported — Needs a mandatory choice between keyword-counter entry modifications before the permanent
// enters.
pub(in crate::card::sets) static WINGFOLD_PTERON: CardRecord = CardRecord::new(
    "Wingfold Pteron",
    "11e21a5f-fae7-4488-9463-f0af6e4c5233",
    "Johann Bodin",
    CardRules::unsupported(),
);

// IKO 72 — Wingspan Mentor
pub(in crate::card::sets) static WINGSPAN_MENTOR: CardRecord = CardRecord::new(
    "Wingspan Mentor",
    "89348a02-29f6-40b9-b39f-cb5f8c230a02",
    "Sidharth Chaturvedi",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 1, 3).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, put a flying counter on target \
             non-Human creature you control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(
                            SubtypeDef::from_name("Human"),
                        )),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::Flying,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "{2}{U}, {T}: Put a +1/+1 counter on each creature you \
             control with flying.",
            &[CostDef::Mana(mana_cost!("{2}{U}")), CostDef::TapSource],
            EffectDef::AddCounters {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// IKO 73 — Bastion of Remembrance
pub(in crate::card::sets) static BASTION_OF_REMEMBRANCE: CardRecord = CardRecord::new(
    "Bastion of Remembrance",
    "2dd354dd-939e-4b1a-8ed6-fe89a7fd64bf",
    "Volkan Baǵa",
    CardRules::new_enchantment(mana_cost!("{2}{B}")).with_abilities(&[
        abilities::enters_trigger(
            "When this enchantment enters, create a 1/1 white Human \
             Soldier creature token.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(HUMAN_SOLDIER_TOKEN))),
        ),
        AbilityDef::triggered(
            "Whenever a creature you control dies, each opponent loses 1 \
             life and you gain 1 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
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

// IKO 74 — Blitz Leech
pub(in crate::card::sets) static BLITZ_LEECH: CardRecord = CardRecord::new(
    "Blitz Leech",
    "7d9b5bde-a851-480b-b45e-d384fd1c11bb",
    "Nicholas Gregory",
    CardRules::new_creature(mana_cost!("{5}{B}"), &["Leech"], 5, 2).with_abilities(&[
        abilities::flash(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, target creature an opponent \
             controls gets -2/-2 until end of turn. Remove all counters \
             from that creature.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-2),
                        ValueDef::Constant(-2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::RemoveAllCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: None,
                },
            ]),
        ),
    ]),
);

// IKO 75 — Blood Curdle
pub(in crate::card::sets) static BLOOD_CURDLE: CardRecord = CardRecord::new(
    "Blood Curdle",
    "4184c851-1419-476c-ba9c-9f0cb1137114",
    "Antonio José Manzanedo",
    CardRules::new_instant(mana_cost!("{3}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target creature. Put a menace counter on a creature \
         you control. (It can't be blocked except by two or more \
         creatures.)",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::destroy_target(TargetIndex::PRIMARY),
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 1,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                        "chosen"
                    ))),
                    kind: CounterKind::Menace,
                    amount: ValueDef::Constant(1),
                },
            }),
        ]),
    )]),
);

// IKO 76 — Boot Nipper
// Audit: unsupported — Needs a mandatory choice between keyword-counter entry modifications before the permanent
// enters.
pub(in crate::card::sets) static BOOT_NIPPER: CardRecord = CardRecord::new(
    "Boot Nipper",
    "cff5a5b8-f823-4429-acd8-c4f34a676cb4",
    "Nicholas Gregory",
    CardRules::unsupported(),
);

// IKO 77 — Bushmeat Poacher
pub(in crate::card::sets) static BUSHMEAT_POACHER: CardRecord = CardRecord::new(
    "Bushmeat Poacher",
    "cc01faf4-fabb-40c9-a1d6-359280535f91",
    "Randy Vargas",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Human", "Soldier"], 2, 4).with_abilities(&[
        AbilityDef::activated(
            "{1}, {T}, Sacrifice another creature: You gain life equal to \
             the sacrificed creature's toughness. Draw a card.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::TapSource,
                CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ])),
            ],
            EffectDef::Sequence(&[
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                        objects: ObjectSetDef::One(ObjectRefDef::AdditionalCostObject(
                            crate::AdditionalCostObjectIndex::PRIMARY,
                        )),
                        select: ObjectValueDef::Toughness,
                        operation: AggregateOperationDef::Sum,
                    }),
                },
                abilities::draw_cards(ValueDef::Constant(1)),
            ]),
        ),
    ]),
);

// IKO 78 — Call of the Death-Dweller
// Audit: unsupported — Needs a target-group constraint on total mana value and two independent counter-recipient
// choices among the returned permanents.
pub(in crate::card::sets) static CALL_OF_THE_DEATH_DWELLER: CardRecord = CardRecord::new(
    "Call of the Death-Dweller",
    "37e34017-0aeb-4a93-9edf-596bf3597a0e",
    "Vincent Proce",
    CardRules::unsupported(),
);

// IKO 79 — Cavern Whisperer
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static CAVERN_WHISPERER: CardRecord = CardRecord::new(
    "Cavern Whisperer",
    "64ae7651-4a5c-41f3-b528-41b9b0f2e5d8",
    "Antonio José Manzanedo",
    CardRules::unsupported(),
);

// IKO 80 — Chittering Harvester
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static CHITTERING_HARVESTER: CardRecord = CardRecord::new(
    "Chittering Harvester",
    "d51a0700-d4d3-4b7e-bdb5-6cd913c948d9",
    "Grzegorz Rutkowski",
    CardRules::unsupported(),
);

// IKO 81 — Corpse Churn (reprint)
const CORPSE_CHURN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::oath_of_the_gatewatch::CORPSE_CHURN,
    "a1c0c764-791d-4181-8f7a-147554d97d97",
    "Lars Grant-West",
);

// IKO 82 — Dark Bargain (reprint)
const DARK_BARGAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::dominaria::DARK_BARGAIN,
    "6ef6f861-90ef-4e1d-9f17-1dd306e7d0af",
    "Scott Murphy",
);

// IKO 83 — Dead Weight (reprint)
const DEAD_WEIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::innistrad::DEAD_WEIGHT,
    "ecf18476-f67b-46e6-905c-e6808981c58a",
    "Kev Walker",
);

// IKO 84 — Dirge Bat
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static DIRGE_BAT: CardRecord = CardRecord::new(
    "Dirge Bat",
    "59677c76-8148-4b6f-95b0-0e3ccf137a3f",
    "Paul Scott Canavan",
    CardRules::unsupported(),
);

// IKO 85 — Durable Coilbug
pub(in crate::card::sets) static DURABLE_COILBUG: CardRecord = CardRecord::new(
    "Durable Coilbug",
    "a0cbdfb6-c029-440f-bc07-c95e03c20110",
    "Milivoj Ćeran",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Insect"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{4}{B}: Return this card from your graveyard to your hand.",
            &[CostDef::Mana(mana_cost!("{4}{B}"))],
            EffectDef::move_to_zone(
                EffectRecipientDef::Source,
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// IKO 86 — Duskfang Mentor
pub(in crate::card::sets) static DUSKFANG_MENTOR: CardRecord = CardRecord::new(
    "Duskfang Mentor",
    "fc38fe5d-6c35-4fff-8f1b-726a0685c6f8",
    "Livia Prima",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Cleric"], 1, 3).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, put a lifelink counter on target \
             non-Human creature you control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(
                            SubtypeDef::from_name("Human"),
                        )),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::Lifelink,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "{1}{B}, {T}: Put a +1/+1 counter on each creature you \
             control with lifelink.",
            &[CostDef::Mana(mana_cost!("{1}{B}")), CostDef::TapSource],
            EffectDef::AddCounters {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Lifelink),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// IKO 87 — Easy Prey
pub(in crate::card::sets) static EASY_PREY: CardRecord = CardRecord::new(
    "Easy Prey",
    "312fb6e4-1eb1-4fbb-b7a4-125829a6e96a",
    "Ekaterina Burmak",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target creature with mana value 2 or less.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ManaValueAtMost(2),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY),
        ),
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
    ]),
);

// IKO 88 — Extinction Event
// Audit: unsupported — Needs a resolution-time odd/even choice and a mana-value parity predicate.
pub(in crate::card::sets) static EXTINCTION_EVENT: CardRecord = CardRecord::new(
    "Extinction Event",
    "8725a869-462b-4381-880a-b4bcc63a655b",
    "Filip Burburan",
    CardRules::unsupported(),
);

// IKO 89 — Gloom Pangolin
pub(in crate::card::sets) static GLOOM_PANGOLIN: CardRecord = CardRecord::new(
    "Gloom Pangolin",
    "3f135dd7-2a4f-4c83-9a90-76bcab3cc33d",
    "YW Tang",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Nightmare", "Pangolin"], 1, 5),
);

// IKO 90 — Grimdancer
// Audit: unsupported — Needs selection of two distinct keyword-counter kinds as one battlefield-entry replacement.
pub(in crate::card::sets) static GRIMDANCER: CardRecord = CardRecord::new(
    "Grimdancer",
    "819257c8-9806-48bd-ba49-e117ca31de54",
    "Randy Vargas",
    CardRules::unsupported(),
);

// IKO 91 — Heartless Act
// Audit: unsupported — Needs one up-to-three counter-removal choice distributed among any
// counter kinds on the target; current removal effects specify one kind or remove every
// counter.
pub(in crate::card::sets) static HEARTLESS_ACT: CardRecord = CardRecord::new(
    "Heartless Act",
    "e4e6794a-feeb-4fc8-a2ee-38c75c18aaae",
    "Ryan Pancoast",
    CardRules::unsupported(),
);

// IKO 92 — Hunted Nightmare
pub(in crate::card::sets) static HUNTED_NIGHTMARE: CardRecord = CardRecord::new(
    "Hunted Nightmare",
    "5f2dddce-583e-4af3-b144-a60dd268e84f",
    "Antonio José Manzanedo",
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Nightmare"], 4, 5).with_abilities(&[
        abilities::menace(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, target opponent puts a deathtouch \
             counter on a creature they control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                unchosen: None,
                chooser: PlayerRefDef::Target(TargetIndex::PRIMARY),
                candidates: ObjectSetDef::Query(ObjectQueryDef::controlled_by(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                )),
                exclude: None,
                minimum: 1,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                        "chosen"
                    ))),
                    kind: CounterKind::Deathtouch,
                    amount: ValueDef::Constant(1),
                },
            }),
        ),
    ]),
);

// IKO 93 — Insatiable Hemophage
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static INSATIABLE_HEMOPHAGE: CardRecord = CardRecord::new(
    "Insatiable Hemophage",
    "70419590-2e0d-4232-b596-a5359b284647",
    "Antonio José Manzanedo",
    CardRules::unsupported(),
);

// IKO 94 — Lurking Deadeye
pub(in crate::card::sets) static LURKING_DEADEYE: CardRecord = CardRecord::new(
    "Lurking Deadeye",
    "43925a8d-dd02-4907-929e-c015d678bb49",
    "Livia Prima",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Human", "Assassin"], 4, 2).with_abilities(&[
        abilities::flash(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, destroy target creature that was \
             dealt damage this turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::WasDealtDamageThisTurn,
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY),
        ),
    ]),
);

// IKO 95 — Memory Leak
pub(in crate::card::sets) static MEMORY_LEAK: CardRecord = CardRecord::new(
    "Memory Leak",
    "3b45a493-a1c2-430e-a45c-abc1ba554877",
    "Karl Kopinski",
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target opponent reveals their hand. You choose a nonland \
             card from that player's graveyard or hand and exile it.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::Sequence(&[
                EffectDef::RevealHand {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        &[ZoneKind::Graveyard, ZoneKind::Hand],
                        PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                }),
            ]),
        ),
        abilities::cycling!("Cycling {1}", &[CostDef::Mana(mana_cost!("{1}"))]),
    ]),
);

// IKO 96 — Mutual Destruction
// Audit: unsupported — Needs conditional intrinsic flash on a nonbattlefield spell, including before casting
// legality is evaluated.
pub(in crate::card::sets) static MUTUAL_DESTRUCTION: CardRecord = CardRecord::new(
    "Mutual Destruction",
    "85ac0b25-80bf-4871-a6f6-5cf4d5b9496e",
    "PINDURSKI",
    CardRules::unsupported(),
);

// IKO 97 — Mythos of Nethroi
// Audit: unsupported — Needs a predicate for the specific colors of mana actually spent to cast this spell,
// including alternative payments.
pub(in crate::card::sets) static MYTHOS_OF_NETHROI: CardRecord = CardRecord::new(
    "Mythos of Nethroi",
    "6abc24e1-e721-471a-9efd-547f320675b0",
    "Seb McKinnon",
    CardRules::unsupported(),
);

// IKO 98 — Nightsquad Commando
// Audit: unsupported — Needs player-level attack history that survives attackers leaving or changing controller;
// querying current attacking permanents is insufficient.
pub(in crate::card::sets) static NIGHTSQUAD_COMMANDO: CardRecord = CardRecord::new(
    "Nightsquad Commando",
    "80306d10-0da7-4ee7-b975-ef6c9b535a2a",
    "Evyn Fong",
    CardRules::unsupported(),
);

// IKO 99 — Serrated Scorpion
pub(in crate::card::sets) static SERRATED_SCORPION: CardRecord = CardRecord::new(
    "Serrated Scorpion",
    "bc8f0242-35e1-4409-9321-56e742e8fef4",
    "Chris Seaman",
    CardRules::new_creature(mana_cost!("{B}"), &["Scorpion"], 1, 2).with_abilities(&[
        abilities::dies_trigger(
            "When this creature dies, it deals 2 damage to each opponent \
             and you gain 2 life.",
            EffectDef::Sequence(&[
                EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(2)),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
    ]),
);

// IKO 100 — Suffocating Fumes
pub(in crate::card::sets) static SUFFOCATING_FUMES: CardRecord = CardRecord::new(
    "Suffocating Fumes",
    "66b562e4-35df-4aee-848d-ceb4204bbe58",
    "Anastasia Ovchinnikova",
    CardRules::new_instant(mana_cost!("{2}{B}")).with_abilities(&[
        AbilityDef::spell(
            "Creatures your opponents control get -1/-1 until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
    ]),
);

// IKO 101 — Unbreakable Bond
pub(in crate::card::sets) static UNBREAKABLE_BOND: CardRecord = CardRecord::new(
    "Unbreakable Bond",
    "c9da02a1-7b39-4a0e-8466-6512a02f3e3b",
    "Randy Vargas",
    CardRules::new_sorcery(mana_cost!("{4}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return target creature card from your graveyard to the \
         battlefield with a lifelink counter on it.",
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
                controller: None,
                modifications: &[BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::Lifelink,
                    amount: 1,
                }],
                attachment: None,
                counters: None,
            },
        },
    )]),
);

// IKO 102 — Unexpected Fangs
pub(in crate::card::sets) static UNEXPECTED_FANGS: CardRecord = CardRecord::new(
    "Unexpected Fangs",
    "aa6494ad-a35e-4b09-8623-7740f3c20b0b",
    "Jesper Ejsing",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Put a +1/+1 counter and a lifelink counter on target creature.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::Lifelink,
                amount: ValueDef::Constant(1),
            },
        ]),
    )]),
);

// IKO 103 — Unlikely Aid (reprint)
const UNLIKELY_AID_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::war_of_the_spark::UNLIKELY_AID,
    "bac3409e-7c24-43b0-9956-5f0cdeae6468",
    "Randy Vargas",
);

// IKO 104 — Void Beckoner
pub(in crate::card::sets) static VOID_BECKONER: CardRecord = CardRecord::new(
    "Void Beckoner",
    "a1523cda-c47d-4419-a5d3-fd6ed9867c56",
    "Daarken",
    CardRules::new_creature(mana_cost!("{6}{B}{B}"), &["Nightmare", "Horror"], 8, 8)
        .with_abilities(&[
            abilities::deathtouch(),
            abilities::cycling!("Cycling {2}{B}", &[CostDef::Mana(mana_cost!("{2}{B}"))]),
            AbilityDef::triggered_with_targets(
                "When you cycle this card, put a deathtouch counter on target \
                 creature you control.",
                TriggerEventDef::DiscardedToActivate(abilities::CYCLING),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::Deathtouch,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// IKO 105 — Whisper Squad
pub(in crate::card::sets) static WHISPER_SQUAD: CardRecord = CardRecord::new(
    "Whisper Squad",
    "097eeb32-cfd5-4adb-ac30-d7762e6ea48f",
    "Stepan Alekseev",
    CardRules::new_creature(mana_cost!("{B}"), &["Human", "Soldier"], 1, 1).with_abilities(&[
        AbilityDef::activated(
            "{1}{B}: Search your library for a card named Whisper Squad, \
             put it onto the battlefield tapped, then shuffle.",
            &[CostDef::Mana(mana_cost!("{1}{B}"))],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::NameEquals(CardNameDef::Literal("Whisper Squad")),
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

// IKO 106 — Zagoth Mamba
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static ZAGOTH_MAMBA: CardRecord = CardRecord::new(
    "Zagoth Mamba",
    "ce945f7e-ded9-4819-9313-d50d3aad40fa",
    "Yeong-Hao Han",
    CardRules::unsupported(),
);

// IKO 107 — Blazing Volley (reprint)
const BLAZING_VOLLEY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::amonkhet::BLAZING_VOLLEY,
    "3adc0288-acdf-4a99-9bfb-919cae1aeb69",
    "Heonhwa",
);

// IKO 108 — Blisterspit Gremlin
pub(in crate::card::sets) static BLISTERSPIT_GREMLIN: CardRecord = CardRecord::new(
    "Blisterspit Gremlin",
    "4ec65b97-d5c0-4609-8a60-3c4daa3e59c1",
    "Simon Dominic",
    CardRules::new_creature(mana_cost!("{R}"), &["Gremlin"], 1, 1).with_abilities(&[
        AbilityDef::activated(
            "{1}, {T}: This creature deals 1 damage to each opponent.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)),
        ),
        AbilityDef::triggered(
            "Whenever you cast a noncreature spell, untap this creature.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::NoncreatureSpell,
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// IKO 109 — Blitz of the Thunder-Raptor
pub(in crate::card::sets) static BLITZ_OF_THE_THUNDER_RAPTOR: CardRecord = CardRecord::new(
    "Blitz of the Thunder-Raptor",
    "a0174556-9f9b-4756-84ca-d14a5a60720c",
    "Zack Stella",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Blitz of the Thunder-Raptor deals damage to target creature \
         or planeswalker equal to the number of instant and sorcery \
         cards in your graveyard. If that creature or planeswalker \
         would die this turn, exile it instead.",
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
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                )),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::ExileInsteadOfDying),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )]),
);

// IKO 110 — Cathartic Reunion (reprint)
const CATHARTIC_REUNION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::kaladesh::CATHARTIC_REUNION,
    "43d93cfd-06d1-4c6a-87f6-9b6eaa2a995f",
    "Jakub Kasper",
);

// IKO 111 — Clash of Titans
pub(in crate::card::sets) static CLASH_OF_TITANS: CardRecord = CardRecord::new(
    "Clash of Titans",
    "ca10e2b8-a317-4c98-a866-43bf274e82cb",
    "Viktor Titov",
    CardRules::new_instant(mana_cost!("{3}{R}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature fights another target creature. (Each deals \
             damage equal to its power to the other.)",
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                }),
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                })
                .another(),
            ],
            EffectDef::Fight {
                first: ObjectRefDef::Target(TargetIndex::PRIMARY),
                second: ObjectRefDef::Target(TargetIndex(1)),
                excess: None,
            },
        ),
    ]),
);

// IKO 112 — Cloudpiercer
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static CLOUDPIERCER: CardRecord = CardRecord::new(
    "Cloudpiercer",
    "3ec5f005-d8fb-48b8-8ac5-74445cc83273",
    "Dan Murayama Scott",
    CardRules::unsupported(),
);

// IKO 113 — Drannith Stinger
// Audit: unsupported — Needs a battlefield listener for another card being cycled; DiscardedToActivate only visits
// the discarded card itself.
pub(in crate::card::sets) static DRANNITH_STINGER: CardRecord = CardRecord::new(
    "Drannith Stinger",
    "612ee4be-e7a2-423c-a37c-7c6ca97f630e",
    "Denman Rooke",
    CardRules::unsupported(),
);

// IKO 114 — Everquill Phoenix
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static EVERQUILL_PHOENIX: CardRecord = CardRecord::new(
    "Everquill Phoenix",
    "3953157f-2deb-4ab7-b4b3-592ebb7c7a9c",
    "Lie Setiawan",
    CardRules::unsupported(),
);

// IKO 115 — Ferocious Tigorilla
// Audit: unsupported — Needs a mandatory choice between keyword-counter entry modifications before the permanent
// enters.
pub(in crate::card::sets) static FEROCIOUS_TIGORILLA: CardRecord = CardRecord::new(
    "Ferocious Tigorilla",
    "5976e8d8-5122-4423-b3eb-61170390ad7a",
    "Antonio José Manzanedo",
    CardRules::unsupported(),
);

// IKO 116 — Fire Prophecy
pub(in crate::card::sets) static FIRE_PROPHECY: CardRecord = CardRecord::new(
    "Fire Prophecy",
    "4c2029e5-cf7d-461f-b7b9-bf96399d8f49",
    "Kieran Yanner",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Fire Prophecy deals 3 damage to target creature. You may put \
         a card from your hand on the bottom of your library. If you \
         do, draw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Private,
                then: &EffectDef::IfNoObjects(IfNoObjectsDef {
                    input: ObjectSetDef::Binding(crate::Binding!("chosen")),
                    if_empty: &EffectDef::None,
                    otherwise: &EffectDef::Sequence(&[
                        EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "chosen"
                            ))),
                            ZoneKind::Library,
                            ZonePlacement::Bottom,
                        ),
                        abilities::draw_cards(ValueDef::Constant(1)),
                    ]),
                }),
            }),
        ]),
    )]),
);

// IKO 117 — Flame Spill
// Audit: unsupported — Needs excess-damage routing for one-sided damage; the existing excess routing is limited to
// fights.
pub(in crate::card::sets) static FLAME_SPILL: CardRecord = CardRecord::new(
    "Flame Spill",
    "b3090004-d7dd-47bc-92e5-977be4fd9ae5",
    "Zoltan Boros",
    CardRules::unsupported(),
);

// IKO 118 — Footfall Crater
pub(in crate::card::sets) static FOOTFALL_CRATER: CardRecord = CardRecord::new(
    "Footfall Crater",
    "f1b75aac-54e3-4181-bac5-5a142757ec05",
    "Ravenna Tran",
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_land(),
            AbilityDef::static_ability(
                "Enchanted land has \"{T}: Target creature gains trample and \
                 haste until end of turn.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets(
                        "{T}: Target creature gains trample and haste until end of turn.",
                        &[CostDef::TapSource],
                        &[AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::HasType(CardType::Creature),
                                zones: &[ZoneKind::Battlefield],
                                controller: None,
                                owner: None,
                            },
                        )],
                        EffectDef::Sequence(&[
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                effect: AppliedEffectDef::add_ability(&abilities::trample()),
                                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                            },
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                            },
                        ]),
                    )),
                },
            ),
            abilities::cycling!("Cycling {1}", &[CostDef::Mana(mana_cost!("{1}"))]),
        ]),
);

// IKO 119 — Forbidden Friendship
pub(in crate::card::sets) static FORBIDDEN_FRIENDSHIP: CardRecord = CardRecord::new(
    "Forbidden Friendship",
    "e634baa3-2cdd-412a-9407-c347fe46f9b8",
    "Jakub Kasper",
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell(
        "Create a 1/1 red Dinosaur creature token with haste and a \
         1/1 white Human Soldier creature token.",
        EffectDef::Sequence(&[
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Dinosaur"], &[ManaColor::Red], 1, 1)
                    .with_abilities(&[abilities::haste()])
                    .with_art(CardArt::new(
                        "f918b740-1984-4090-8886-9e290a698b95",
                        "Jakub Kasper",
                    )),
            ))),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(HUMAN_SOLDIER_TOKEN))),
        ]),
    )]),
);

// IKO 120 — Frenzied Raptor (reprint)
const FRENZIED_RAPTOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::ixalan::FRENZIED_RAPTOR,
    "5fb22ac0-3863-4165-8c93-f2ec1775474f",
    "Jonathan Kuo",
);

// IKO 121 — Frillscare Mentor
pub(in crate::card::sets) static FRILLSCARE_MENTOR: CardRecord = CardRecord::new(
    "Frillscare Mentor",
    "24c7d034-6403-41e2-9e44-4d58ef54cf10",
    "Jason A. Engle",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Warrior"], 3, 2).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, put a menace counter on target \
             non-Human creature you control. (It can't be blocked except \
             by two or more creatures.)",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(
                            SubtypeDef::from_name("Human"),
                        )),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::Menace,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "{2}{R}, {T}: Put a +1/+1 counter on each creature you \
             control with menace.",
            &[CostDef::Mana(mana_cost!("{2}{R}")), CostDef::TapSource],
            EffectDef::AddCounters {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Menace),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// IKO 122 — Go for Blood
pub(in crate::card::sets) static GO_FOR_BLOOD: CardRecord = CardRecord::new(
    "Go for Blood",
    "67315df9-99a1-45ba-8ade-ddc476368a86",
    "Chris Rallis",
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature you control fights target creature you don't \
             control. (Each deals damage equal to its power to the \
             other.)",
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
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                }),
            ],
            EffectDef::Fight {
                first: ObjectRefDef::Target(TargetIndex::PRIMARY),
                second: ObjectRefDef::Target(TargetIndex(1)),
                excess: None,
            },
        ),
        abilities::cycling!("Cycling {1}", &[CostDef::Mana(mana_cost!("{1}"))]),
    ]),
);

// IKO 123 — Heightened Reflexes
pub(in crate::card::sets) static HEIGHTENED_REFLEXES: CardRecord = CardRecord::new(
    "Heightened Reflexes",
    "4f7cc8e7-3002-4ee0-869f-931438d8362d",
    "Caio Monteiro",
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +1/+0 until end of turn. Put a first \
         strike counter on it.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::FirstStrike,
                amount: ValueDef::Constant(1),
            },
        ]),
    )]),
);

// IKO 124 — Lava Serpent
pub(in crate::card::sets) static LAVA_SERPENT: CardRecord = CardRecord::new(
    "Lava Serpent",
    "00ebd57f-7f7c-41b0-aa56-511c1816bc14",
    "Jason A. Engle",
    CardRules::new_creature(mana_cost!("{5}{R}"), &["Elemental", "Serpent"], 5, 5).with_abilities(
        &[
            abilities::haste(),
            abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
        ],
    ),
);

// IKO 125 — Lukka, Coppercoat Outcast
// Audit: unsupported — Needs abilities granted to the exiled cards that permit casting while their owner controls
// a Lukka planeswalker.
pub(in crate::card::sets) static LUKKA_COPPERCOAT_OUTCAST: CardRecord = CardRecord::new(
    "Lukka, Coppercoat Outcast",
    "5798fdf0-d178-43d9-b821-8f3f654654b4",
    "Chris Rallis",
    CardRules::unsupported(),
);

// IKO 126 — Momentum Rumbler
pub(in crate::card::sets) static MOMENTUM_RUMBLER: CardRecord = CardRecord::new(
    "Momentum Rumbler",
    "3587f450-c1ba-4074-84bb-47978a2b6116",
    "Andrey Kuzinskiy",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Dinosaur"], 3, 3).with_abilities(&[
        AbilityDef::triggered_if(
            "Whenever this creature attacks, if it doesn't have first \
             strike, put a first strike counter on it.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &TriggerConditionDef::Not(&TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Source,
                        ObjectPredicateDef::HasKeyword(KeywordAbility::FirstStrike),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            }),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::FirstStrike,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::triggered_if(
            "Whenever this creature attacks, if it has first strike, it \
             gains double strike until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Source,
                        ObjectPredicateDef::HasKeyword(KeywordAbility::FirstStrike),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// IKO 127 — Mythos of Vadrok
// Audit: unsupported — Needs a predicate for the specific colors of mana actually spent to cast this spell,
// including alternative payments.
pub(in crate::card::sets) static MYTHOS_OF_VADROK: CardRecord = CardRecord::new(
    "Mythos of Vadrok",
    "2bbe99a7-3b58-4f23-bf86-e1e35b0bec2e",
    "Seb McKinnon",
    CardRules::unsupported(),
);

// IKO 128 — Porcuparrot
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static PORCUPARROT: CardRecord = CardRecord::new(
    "Porcuparrot",
    "856892c8-ba47-46d0-aec2-0416b55b9e88",
    "Chris Seaman",
    CardRules::unsupported(),
);

// IKO 129 — Prickly Marmoset
// Audit: unsupported — Needs a battlefield listener for another card being cycled; DiscardedToActivate only visits
// the discarded card itself.
pub(in crate::card::sets) static PRICKLY_MARMOSET: CardRecord = CardRecord::new(
    "Prickly Marmoset",
    "bad8512f-31b9-48ba-bb10-1497303dcfba",
    "Simon Dominic",
    CardRules::unsupported(),
);

// IKO 130 — Pyroceratops
pub(in crate::card::sets) static PYROCERATOPS: CardRecord = CardRecord::new(
    "Pyroceratops",
    "f6599645-dbb5-4174-bd26-8556af6d89c3",
    "Jason A. Engle",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Elemental", "Dinosaur"], 2, 3).with_abilities(
        &[
            abilities::trample(),
            AbilityDef::triggered(
                "Whenever you cast a noncreature spell, put a +1/+1 counter \
                 on this creature.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::NoncreatureSpell,
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ],
    ),
);

// IKO 131 — Raking Claws
pub(in crate::card::sets) static RAKING_CLAWS: CardRecord = CardRecord::new(
    "Raking Claws",
    "6eb0d9a2-f9bb-4d8e-a1ca-896c42f8ad56",
    "Slawomir Maniak",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature gains double strike until end of turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
    ]),
);

// IKO 132 — Reptilian Reflection
// Audit: unsupported — Needs a battlefield listener for another card being cycled; DiscardedToActivate only visits
// the discarded card itself.
pub(in crate::card::sets) static REPTILIAN_REFLECTION: CardRecord = CardRecord::new(
    "Reptilian Reflection",
    "7871b4dd-9085-4a3f-a1ae-9a292f73689b",
    "Antonio José Manzanedo",
    CardRules::unsupported(),
);

// IKO 133 — Rooting Moloch
// Audit: unsupported — Needs a targeted exile move granting play permission through the end of the controller's
// next turn; the targeted primitive grants only this turn.
pub(in crate::card::sets) static ROOTING_MOLOCH: CardRecord = CardRecord::new(
    "Rooting Moloch",
    "cc2fd581-5cf8-4154-90dd-922afddcd556",
    "Andrey Kuzinskiy",
    CardRules::unsupported(),
);

// IKO 134 — Rumbling Rockslide
pub(in crate::card::sets) static RUMBLING_ROCKSLIDE: CardRecord = CardRecord::new(
    "Rumbling Rockslide",
    "96f9aaa7-11c7-4cd0-9803-9471c14ab846",
    "Adam Paquette",
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Rumbling Rockslide deals damage to target creature equal to \
         the number of lands you control.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Land),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            )),
        ),
    )]),
);

// IKO 135 — Sanctuary Smasher
pub(in crate::card::sets) static SANCTUARY_SMASHER: CardRecord = CardRecord::new(
    "Sanctuary Smasher",
    "cc634c10-42c5-4bdc-bc22-f862ae285492",
    "Mathias Kollros",
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Rhino", "Beast"], 6, 4).with_abilities(&[
        abilities::first_strike(),
        abilities::cycling!("Cycling {2}{R}", &[CostDef::Mana(mana_cost!("{2}{R}"))]),
        AbilityDef::triggered_with_targets(
            "When you cycle this card, put a first strike counter on \
             target creature you control.",
            TriggerEventDef::DiscardedToActivate(abilities::CYCLING),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::FirstStrike,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// IKO 136 — Shredded Sails
pub(in crate::card::sets) static SHREDDED_SAILS: CardRecord = CardRecord::new(
    "Shredded Sails",
    "8b10219a-aa72-4431-9a6a-984109a605c8",
    "Titus Lunter",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::modal_spell(
            "Choose one —",
            &[
                AbilityDef::spell_with_targets(
                    "Destroy target artifact.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Artifact),
                            zones: &[ZoneKind::Battlefield],
                            controller: None,
                            owner: None,
                        },
                    )],
                    EffectDef::destroy_target(TargetIndex::PRIMARY),
                ),
                AbilityDef::spell_with_targets(
                    "Shredded Sails deals 4 damage to target creature with flying.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                            ]),
                            zones: &[ZoneKind::Battlefield],
                            controller: None,
                            owner: None,
                        },
                    )],
                    EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(4),
                    ),
                ),
            ],
        ),
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
    ]),
);

// IKO 137 — Spelleater Wolverine
pub(in crate::card::sets) static SPELLEATER_WOLVERINE: CardRecord = CardRecord::new(
    "Spelleater Wolverine",
    "a5f03ffd-dcdb-441c-8dfc-4fe06a289b22",
    "Uriah Voth",
    // A vanilla 3/2 until the graveyard fills, then six damage a turn: the
    // threshold is what a spells deck is being paid for.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Wolverine"], 3, 2).with_ability(
        AbilityDef::static_ability(
            "This creature has double strike as long as there are three or more instant and/or \
             sorcery cards in your graveyard.",
            EffectDef::IfCondition {
                // One count over both types rather than two, since the
                // printed clause adds them together.
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 3,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                },
            },
        ),
    ),
);

// IKO 138 — Tentative Connection
pub(in crate::card::sets) static TENTATIVE_CONNECTION: CardRecord = CardRecord::new(
    "Tentative Connection",
    "5c40fd33-bb95-48f3-a10f-a33eb27c48f1",
    "Kieran Yanner",
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_abilities(&[
        abilities::this_spell_cost_reduction(
            "This spell costs {3} less to cast if you control a creature \
             with menace.",
            ValueDef::IfMatchingObjectCount(&CountConditionDef {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Menace),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
                then: ValueDef::Constant(3),
                otherwise: ValueDef::Constant(0),
            }),
        ),
        AbilityDef::spell_with_targets(
            "Gain control of target creature until end of turn. Untap \
             that creature. It gains haste until end of turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::gain_control(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    PlayerRefDef::EffectController,
                    ControlDurationDef::UntilEndOfTurn,
                ),
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// IKO 139 — Unpredictable Cyclone
// Audit: unsupported — Needs draw replacement matched to a cycling ability and access to the cycled card's types
// during reveal-until and free casting.
pub(in crate::card::sets) static UNPREDICTABLE_CYCLONE: CardRecord = CardRecord::new(
    "Unpredictable Cyclone",
    "3a0462f6-a027-43c0-8e01-c0591d9a45d9",
    "Noah Bradley",
    CardRules::unsupported(),
);

// IKO 140 — Weaponize the Monsters
pub(in crate::card::sets) static WEAPONIZE_THE_MONSTERS: CardRecord = CardRecord::new(
    "Weaponize the Monsters",
    "68bba622-a0ab-4c0e-88b1-9120690ea5a0",
    "Magali Villeneuve",
    CardRules::new_enchantment(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{2}, Sacrifice a creature: This enchantment deals 2 damage \
             to any target.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(CardType::Creature)),
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
        ),
    ]),
);

// IKO 141 — Yidaro, Wandering Monster (alternate printing)
const YIDARO_WANDERING_MONSTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &YIDARO_WANDERING_MONSTER,
    1,
    "b540c7c6-5b9e-4606-ac84-e583a62a3647",
    "Jesper Ejsing",
);

// IKO 142 — Adventurous Impulse (reprint)
const ADVENTUROUS_IMPULSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::dominaria::ADVENTUROUS_IMPULSE,
    "30811fb2-5767-4106-9a8d-6091f61969c6",
    "Victor Adame Minguez",
);

// IKO 143 — Almighty Brushwagg
pub(in crate::card::sets) static ALMIGHTY_BRUSHWAGG: CardRecord = CardRecord::new(
    "Almighty Brushwagg",
    "71f2b7ac-8742-468d-b6a3-87881cb522ff",
    "Dmitry Burmak",
    CardRules::new_creature(mana_cost!("{G}"), &["Brushwagg"], 1, 1).with_abilities(&[
        abilities::trample(),
        AbilityDef::activated(
            "{3}{G}: This creature gets +3/+3 until end of turn.",
            &[CostDef::Mana(mana_cost!("{3}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// IKO 144 — Auspicious Starrix
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static AUSPICIOUS_STARRIX: CardRecord = CardRecord::new(
    "Auspicious Starrix",
    "a39ae1e4-d4dd-4691-af5a-5fa25ace4ebe",
    "Lucas Graciano",
    CardRules::unsupported(),
);

// IKO 145 — Barrier Breach
pub(in crate::card::sets) static BARRIER_BREACH: CardRecord = CardRecord::new(
    "Barrier Breach",
    "822f8403-77a7-4e75-88af-60d604632f5d",
    "Mathias Kollros",
    CardRules::new_instant(mana_cost!("{2}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Exile up to three target enchantments.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Enchantment),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                3,
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::target_objects(TargetIndex::PRIMARY),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
        ),
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
    ]),
);

// IKO 146 — Bristling Boar (reprint)
const BRISTLING_BOAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::core_set_2019::BRISTLING_BOAR,
    "c3c3243c-e9af-4b7d-8296-f7714436e571",
    "Zezhou Chen",
);

// IKO 147 — Charge of the Forever-Beast
// Audit: unsupported — Needs revealing a creature card as a spell additional cost and retaining its power;
// reveal-from-hand costs currently work only for activated abilities.
pub(in crate::card::sets) static CHARGE_OF_THE_FOREVER_BEAST: CardRecord = CardRecord::new(
    "Charge of the Forever-Beast",
    "4bf147a6-2282-4120-9cbd-d0309fc9b02b",
    "Filip Burburan",
    CardRules::unsupported(),
);

// IKO 148 — Colossification
pub(in crate::card::sets) static COLOSSIFICATION: CardRecord = CardRecord::new(
    "Colossification",
    "7b6e6f2a-5015-44c6-aa8d-85188494d1a6",
    "Johan Grenier",
    CardRules::new_enchantment(mana_cost!("{5}{G}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            abilities::enters_trigger(
                "When this Aura enters, tap enchanted creature.",
                EffectDef::Tap {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +20/+20.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(20),
                        ValueDef::Constant(20),
                    ),
                },
            ),
        ]),
);

// IKO 149 — Essence Symbiote
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static ESSENCE_SYMBIOTE: CardRecord = CardRecord::new(
    "Essence Symbiote",
    "8d09ddf0-91f0-4e76-809f-c39ca7418ed5",
    "Jason Felix",
    CardRules::unsupported(),
);

// IKO 150 — Excavation Mole
pub(in crate::card::sets) static EXCAVATION_MOLE: CardRecord = CardRecord::new(
    "Excavation Mole",
    "3a1712dc-8f4c-407f-83ea-3a1c59c437d1",
    "Lars Grant-West",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Mole"], 3, 3).with_abilities(&[
        abilities::trample(),
        abilities::enters_trigger(
            "When this creature enters, mill three cards.",
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
    ]),
);

// IKO 151 — Exuberant Wolfbear
pub(in crate::card::sets) static EXUBERANT_WOLFBEAR: CardRecord = CardRecord::new(
    "Exuberant Wolfbear",
    "8c8ff0da-fbe3-4c95-bc39-c975c55b6aea",
    "Jesper Ejsing",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Wolf", "Bear"], 4, 4).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, you may change the base \
             power and toughness of target Human you control to this \
             creature's power and toughness until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::from_name("Human")),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::set_base_power_toughness(
                        ValueDef::SourcePower,
                        ValueDef::SourceToughness,
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            },
        ),
    ]),
);

// IKO 152 — Fertilid (reprint)
const FERTILID_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::morningtide::FERTILID,
    "9c6b39b9-0615-4899-8de2-b579d5b12f1c",
    "Nicholas Gregory",
);

// IKO 153 — Flycatcher Giraffid
// Audit: unsupported — Needs a mandatory choice between keyword-counter entry modifications before the permanent
// enters.
pub(in crate::card::sets) static FLYCATCHER_GIRAFFID: CardRecord = CardRecord::new(
    "Flycatcher Giraffid",
    "1988e220-d746-46e4-a534-164203e63c14",
    "Dan Murayama Scott",
    CardRules::unsupported(),
);

// IKO 154 — Fully Grown
pub(in crate::card::sets) static FULLY_GROWN: CardRecord = CardRecord::new(
    "Fully Grown",
    "0b683d3f-025c-4b8d-89d7-3513488649d5",
    "Dmitry Burmak",
    CardRules::new_instant(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +3/+3 until end of turn. Put a trample \
         counter on it.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::Trample,
                amount: ValueDef::Constant(1),
            },
        ]),
    )]),
);

// IKO 155 — Gemrazer
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static GEMRAZER: CardRecord = CardRecord::new(
    "Gemrazer",
    "0095245c-a30e-4e2a-88c9-632c678e9f03",
    "Svetlin Velinov",
    CardRules::unsupported(),
);

// IKO 156 — Glowstone Recluse
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static GLOWSTONE_RECLUSE: CardRecord = CardRecord::new(
    "Glowstone Recluse",
    "06a9698d-400c-4c31-82cd-28d0897fe28c",
    "Yeong-Hao Han",
    CardRules::unsupported(),
);

// IKO 157 — Greater Sandwurm (reprint)
const GREATER_SANDWURM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::amonkhet::GREATER_SANDWURM,
    "d90c5650-7eb2-480a-856e-a04406096830",
    "Grzegorz Rutkowski",
);

// IKO 158 — Honey Mammoth
pub(in crate::card::sets) static HONEY_MAMMOTH: CardRecord = CardRecord::new(
    "Honey Mammoth",
    "84b9bee2-b973-4de7-b72d-7f36f8e8153c",
    "Lars Grant-West",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Elephant"], 6, 6).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, you gain 4 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
        ),
    ]),
);

// IKO 159 — Hornbash Mentor
pub(in crate::card::sets) static HORNBASH_MENTOR: CardRecord = CardRecord::new(
    "Hornbash Mentor",
    "d47d2596-a321-44c0-a9f6-87bf80abc1c4",
    "Wisnu Tan",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Warrior"], 3, 3).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, put a trample counter on target \
             non-Human creature you control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(
                            SubtypeDef::from_name("Human"),
                        )),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::Trample,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "{2}{G}, {T}: Put a +1/+1 counter on each creature you \
             control with trample.",
            &[CostDef::Mana(mana_cost!("{2}{G}")), CostDef::TapSource],
            EffectDef::AddCounters {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Trample),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// IKO 160 — Humble Naturalist
pub(in crate::card::sets) static HUMBLE_NATURALIST: CardRecord = CardRecord::new(
    "Humble Naturalist",
    "8f56705d-eb64-4cef-b716-edbeac60bf79",
    "Matt Stewart",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Druid"], 1, 3).with_abilities(&[
        abilities::tap_for_mana(
            "{T}: Add one mana of any color. Spend this mana only to cast \
             a creature spell.",
            AddManaEffectDef::any_color().with_restrictions(&[ManaRestrictionDef::CastSpell(
                ObjectPredicateDef::HasType(CardType::Creature),
            )]),
        ),
    ]),
);

// IKO 161 — Ivy Elemental (reprint)
const IVY_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::odyssey::IVY_ELEMENTAL,
    "8e8aea71-02aa-4799-86fb-efd2a36efc22",
    "Uriah Voth",
);

// IKO 162 — Kogla, the Titan Ape
pub(in crate::card::sets) static KOGLA_THE_TITAN_APE: CardRecord = CardRecord::new(
    "Kogla, the Titan Ape",
    "3c35ca79-eb72-427a-a8ed-404b2214389a",
    "Chris Rahn",
    CardRules::new_creature(mana_cost!("{3}{G}{G}{G}"), &["Ape"], 7, 6)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When Kogla enters, it fights up to one target creature you \
                 don't control.",
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::NotYou),
                        owner: None,
                    },
                    1,
                )],
                EffectDef::Fight {
                    first: ObjectRefDef::Source,
                    second: ObjectRefDef::Target(TargetIndex::PRIMARY),
                    excess: None,
                },
            ),
            AbilityDef::triggered_with_targets(
                "Whenever Kogla attacks, destroy target artifact or \
                 enchantment defending player controls.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::DefendingPlayer),
                        owner: None,
                    },
                )],
                EffectDef::destroy_target(TargetIndex::PRIMARY),
            ),
            AbilityDef::activated_with_targets(
                "{1}{G}: Return target Human you control to its owner's \
                 hand. Kogla gains indestructible until end of turn.",
                &[CostDef::Mana(mana_cost!("{1}{G}"))],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Subtype(SubtypeDef::from_name("Human")),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
        ]),
);

// IKO 163 — Lead the Stampede (reprint)
const LEAD_THE_STAMPEDE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::mirrodin_besieged::LEAD_THE_STAMPEDE,
    "9e76b676-c7a3-4de6-a78d-3059a0df83f2",
    "Lius Lasahido",
);

// IKO 164 — Migration Path
pub(in crate::card::sets) static MIGRATION_PATH: CardRecord = CardRecord::new(
    "Migration Path",
    "d345ca63-16a3-4988-93f5-d0e00e8d6ab0",
    "Grzegorz Rutkowski",
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_abilities(&[
        AbilityDef::spell(
            "Search your library for up to two basic land cards, put them \
             onto the battlefield tapped, then shuffle.",
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::Supertype(CardSupertype::Basic),
                minimum: 0,
                maximum: ValueDef::Constant(2),
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
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
    ]),
);

// IKO 165 — Migratory Greathorn
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static MIGRATORY_GREATHORN: CardRecord = CardRecord::new(
    "Migratory Greathorn",
    "6a2a287b-b83f-444f-84f7-e388beb616c2",
    "Filip Burburan",
    CardRules::unsupported(),
);

// IKO 166 — Monstrous Step
// Audit: unsupported — Needs a block requirement relating two independently targeted creatures; MustBeBlockedBy
// cannot name a target or bound object.
pub(in crate::card::sets) static MONSTROUS_STEP: CardRecord = CardRecord::new(
    "Monstrous Step",
    "893f300b-a823-4156-90f2-0636e9f499b0",
    "Chris Seaman",
    CardRules::unsupported(),
);

// IKO 167 — Mosscoat Goriak
pub(in crate::card::sets) static MOSSCOAT_GORIAK: CardRecord = CardRecord::new(
    "Mosscoat Goriak",
    "c23139d4-0db5-4683-8d49-f4600fbe29e2",
    "Dan Murayama Scott",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Beast"], 2, 4)
        .with_abilities(&[abilities::vigilance()]),
);

// IKO 168 — Mythos of Brokkos
// Audit: unsupported — Needs a predicate for the specific colors of mana actually spent to cast this spell,
// including alternative payments.
pub(in crate::card::sets) static MYTHOS_OF_BROKKOS: CardRecord = CardRecord::new(
    "Mythos of Brokkos",
    "fa4fc5b1-6666-4c60-898f-f927212c7923",
    "Seb McKinnon",
    CardRules::unsupported(),
);

// IKO 169 — Plummet (reprint)
const PLUMMET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::archenemy::PLUMMET,
    "d884b2f2-946e-4d5d-b8cf-ef035726a188",
    "Sidharth Chaturvedi",
);

// IKO 170 — Ram Through
// Audit: unsupported — Needs excess-damage routing for one-sided damage; the existing excess routing is limited to
// fights.
pub(in crate::card::sets) static RAM_THROUGH: CardRecord = CardRecord::new(
    "Ram Through",
    "ac0b24e7-14e7-45ee-b5d8-bdb8674b669c",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// IKO 171 — Sudden Spinnerets
pub(in crate::card::sets) static SUDDEN_SPINNERETS: CardRecord = CardRecord::new(
    "Sudden Spinnerets",
    "3730996a-96d2-4311-a4dc-0045a2f8ccc9",
    "Nicholas Gregory",
    CardRules::new_instant(mana_cost!("{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +1/+3 until end of turn. Put a reach \
         counter on it. Untap it.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::Reach,
                amount: ValueDef::Constant(1),
            },
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ]),
    )]),
);

// IKO 172 — Survivors' Bond
pub(in crate::card::sets) static SURVIVORS_BOND: CardRecord = CardRecord::new(
    "Survivors' Bond",
    "530ff2a1-6447-4653-a661-d9a39156d6fa",
    "Randy Vargas",
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one or both —",
        &[
            AbilityDef::spell_with_targets(
                "Return target Human creature card from your graveyard to \
                 your hand.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::from_name("Human")),
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
            AbilityDef::spell_with_targets(
                "Return target non-Human creature card from your graveyard to \
                 your hand.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(
                                SubtypeDef::from_name("Human"),
                            )),
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
        ],
    )
    .with_mode_selection(1, 2, false)]),
);

// IKO 173 — Thwart the Enemy
pub(in crate::card::sets) static THWART_THE_ENEMY: CardRecord = CardRecord::new(
    "Thwart the Enemy",
    "a7101f35-d8c9-4320-bffc-8bb25000d103",
    "Chris Rallis",
    CardRules::new_instant(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::spell(
        "Prevent all damage that would be dealt this turn by \
         creatures your opponents control.",
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef {
                kind: DamageKindDef::Any,
                source: DamageSourceMatcherDef::Matching(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                ])),
                recipient: DamageRecipientMatcherDef::Any,
            }),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// IKO 174 — Titanoth Rex
pub(in crate::card::sets) static TITANOTH_REX: CardRecord = CardRecord::new(
    "Titanoth Rex",
    "9d02e1e8-b85b-4e26-8ab8-ca2f49d05b88",
    "Svetlin Velinov",
    CardRules::new_creature(mana_cost!("{7}{G}{G}"), &["Dinosaur", "Beast"], 11, 11)
        .with_abilities(&[
            abilities::trample(),
            abilities::cycling!(
                "Cycling {1}{G} ({1}{G}, Discard this card: Draw a card.)",
                &[CostDef::Mana(mana_cost!("{1}{G}"))]
            ),
            AbilityDef::triggered_with_targets(
                "When you cycle this card, put a trample counter on target creature you control.",
                TriggerEventDef::DiscardedToActivate(abilities::CYCLING),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::Trample,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// IKO 175 — Vivien, Monsters' Advocate
// Audit: unsupported — Needs an installed trigger that expires this turn and is consumed by its first matching
// creature cast, independently for each activation.
pub(in crate::card::sets) static VIVIEN_MONSTERS_ADVOCATE: CardRecord = CardRecord::new(
    "Vivien, Monsters' Advocate",
    "504ebb84-7e7b-4119-a128-a9c183c5d9de",
    "Lius Lasahido",
    CardRules::unsupported(),
);

// IKO 176 — Wilt
pub(in crate::card::sets) static WILT: CardRecord = CardRecord::new(
    "Wilt",
    "55987bab-c24f-4261-b97e-f0ad4474b0a0",
    "Volkan Baǵa",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target artifact or enchantment.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY),
        ),
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
    ]),
);

// IKO 177 — Back for More
pub(in crate::card::sets) static BACK_FOR_MORE: CardRecord = CardRecord::new(
    "Back for More",
    "3fc7210c-da23-4cec-9195-4de75587f40f",
    "Daarken",
    CardRules::new_instant(mana_cost!("{4}{B}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return target creature card from your graveyard to the \
             battlefield. When you do, it fights up to one target \
             creature you don't control. (Each deals damage equal to its \
             power to the other.)",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::MoveObjects(MoveObjectsDef {
                input: ObjectSetDef::LegalTargets(TargetIndex::PRIMARY),
                from: Some(ZoneKind::Graveyard),
                zone: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                moved: Some(crate::Binding!("returned")),
                then: &EffectDef::IfNoObjects(IfNoObjectsDef {
                    input: ObjectSetDef::InZone {
                        objects: &ObjectSetDef::Binding(crate::Binding!("returned")),
                        zone: ZoneKind::Battlefield,
                    },
                    if_empty: &EffectDef::None,
                    otherwise: &EffectDef::ReflexiveTrigger(&AbilityDef::triggered_with_targets(
                        "When you do, it fights up to one target creature you don't \
                         control.",
                        TriggerEventDef::Reflexive,
                        &[AbilityTargetDef::up_to(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::HasType(CardType::Creature),
                                zones: &[ZoneKind::Battlefield],
                                controller: Some(PlayerRelation::NotYou),
                                owner: None,
                            },
                            1,
                        )],
                        EffectDef::ForEachInBinding {
                            objects: crate::Binding!("returned"),
                            binding: crate::Binding!("fighter"),
                            effect: &EffectDef::Fight {
                                first: ObjectRefDef::Binding(crate::Binding!("fighter")),
                                second: ObjectRefDef::Target(TargetIndex::PRIMARY),
                                excess: None,
                            },
                        },
                    )),
                }),
            }),
        ),
    ]),
);

// IKO 178 — Boneyard Lurker
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static BONEYARD_LURKER: CardRecord = CardRecord::new(
    "Boneyard Lurker",
    "37e4df5b-ec53-4f8a-8c26-272b3177c0a6",
    "Nils Hamm",
    CardRules::unsupported(),
);

// IKO 179 — Brokkos, Apex of Forever
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static BROKKOS_APEX_OF_FOREVER: CardRecord = CardRecord::new(
    "Brokkos, Apex of Forever",
    "c9f07625-fbd8-4581-8568-eb3cfb2a4c1e",
    "Filip Burburan",
    CardRules::unsupported(),
);

// IKO 180 — Channeled Force
// Audit: unsupported — Needs X announced for a variable discard additional cost when the printed mana cost has no
// X, with that paid value preserved on the spell.
pub(in crate::card::sets) static CHANNELED_FORCE: CardRecord = CardRecord::new(
    "Channeled Force",
    "8918d24e-8ae8-4028-9f06-ba6fafcc717e",
    "Randy Vargas",
    CardRules::unsupported(),
);

// IKO 181 — Chevill, Bane of Monsters
pub(in crate::card::sets) static CHEVILL_BANE_OF_MONSTERS: CardRecord = CardRecord::new(
    "Chevill, Bane of Monsters",
    "fecbf0a3-ebe1-43b6-a720-462ba19002eb",
    "Yongjae Choi",
    CardRules::new_creature(mana_cost!("{B}{G}"), &["Human", "Rogue"], 1, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::deathtouch(),
            AbilityDef::triggered_if_with_targets(
                "At the beginning of your upkeep, if your opponents control \
                 no permanents with bounty counters on them, put a bounty \
                 counter on target creature or planeswalker an opponent \
                 controls.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                &TriggerConditionDef::Not(&TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::HasCounter(CounterKind::named("bounty")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                }),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    },
                )],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::named("bounty"),
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::triggered(
                "Whenever a permanent an opponent controls with a bounty \
                 counter on it dies, you gain 3 life and draw a card.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                        ObjectPredicateDef::HasCounter(CounterKind::named("bounty")),
                    ]),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::Sequence(&[
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                    abilities::draw_cards(ValueDef::Constant(1)),
                ]),
            ),
        ]),
);

// IKO 182 — Death's Oasis
pub(in crate::card::sets) static DEATH_S_OASIS: CardRecord = CardRecord::new(
    "Death's Oasis",
    "112e2cca-bae7-4296-9514-e6058f4b38a5",
    "Grzegorz Rutkowski",
    CardRules::new_enchantment(mana_cost!("{W}{B}{G}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever a nontoken creature you control dies, mill two \
             cards. Then return a creature card with lesser mana value \
             than the creature that died from your graveyard to your \
             hand.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::Sequence(&[
                EffectDef::Mill {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::ManaValueAtMostValue(ValueDef::Sum(&SumValueDef {
                                left: ValueDef::ObjectManaValue(ObjectRefDef::TriggeringObject),
                                right: ValueDef::Constant(-1),
                            })),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                }),
            ]),
        ),
        AbilityDef::activated(
            "{1}, Sacrifice this enchantment: You gain life equal to the \
             greatest mana value among creatures you control.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::SacrificeSource],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                    objects: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    select: ObjectValueDef::ManaValue,
                    operation: AggregateOperationDef::Maximum,
                }),
            },
        ),
    ]),
);

// IKO 183 — Dire Tactics
pub(in crate::card::sets) static DIRE_TACTICS: CardRecord = CardRecord::new(
    "Dire Tactics",
    "e0e3974e-3753-4f25-8930-6d96b40332ce",
    "Daarken",
    CardRules::new_instant(mana_cost!("{W}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Exile target creature. If you don't control a Human, you \
         lose life equal to that creature's toughness.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::Not(&TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::from_name("Human")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                }),
                then: &EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::TargetToughness(TargetIndex::PRIMARY),
                },
            },
        ]),
    )]),
);

// IKO 184 — Eerie Ultimatum
// Audit: unsupported — Needs a distinct-name constraint across a non-targeted multi-card selection.
pub(in crate::card::sets) static EERIE_ULTIMATUM: CardRecord = CardRecord::new(
    "Eerie Ultimatum",
    "3a9fb2db-228f-4d48-acdf-6330baf356c7",
    "Jason A. Engle",
    CardRules::unsupported(),
);

// IKO 185 — Emergent Ultimatum
// Audit: unsupported — Needs a distinct-name constraint across a non-targeted multi-card selection.
pub(in crate::card::sets) static EMERGENT_ULTIMATUM: CardRecord = CardRecord::new(
    "Emergent Ultimatum",
    "3a6a52ab-6d38-4429-9969-90064e615152",
    "Zack Stella",
    CardRules::unsupported(),
);

// IKO 186 — Frondland Felidar
pub(in crate::card::sets) static FRONDLAND_FELIDAR: CardRecord = CardRecord::new(
    "Frondland Felidar",
    "ab220695-e1a9-45ec-a1b1-5a82c9c90a03",
    "Steve Prescott",
    CardRules::new_creature(mana_cost!("{2}{G}{W}"), &["Cat", "Beast"], 3, 5).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::static_ability(
            "Creatures you control with vigilance have \"{1}, {T}: Tap \
             target creature.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Vigilance),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets(
                    "{1}, {T}: Tap target creature.",
                    &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &[ZoneKind::Battlefield],
                            controller: None,
                            owner: None,
                        },
                    )],
                    EffectDef::Tap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                )),
            },
        ),
    ]),
);

// IKO 187 — General Kudro of Drannith
pub(in crate::card::sets) static GENERAL_KUDRO_OF_DRANNITH: CardRecord = CardRecord::new(
    "General Kudro of Drannith",
    "2c3227ae-0c72-478a-a6dd-661aaf718038",
    "Ryan Pancoast",
    CardRules::new_creature(mana_cost!("{1}{W}{B}"), &["Human", "Soldier"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Other Humans you control get +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::from_name("Human")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::triggered_with_targets(
                "Whenever General Kudro or another Human you control enters, \
                 exile target card from an opponent's graveyard.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Source,
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::from_name("Human")),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
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
            AbilityDef::activated_with_targets(
                "{2}, Sacrifice two Humans: Destroy target creature with \
                 power 4 or greater.",
                &[
                    CostDef::Mana(mana_cost!("{2}")),
                    CostDef::sacrifice_permanents(
                        ObjectPredicateDef::Subtype(SubtypeDef::from_name("Human")),
                        PlayerRelation::You,
                        2,
                    ),
                ],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::PowerAtLeast(4),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::destroy_target(TargetIndex::PRIMARY),
            ),
        ]),
);

// IKO 188 — General's Enforcer
pub(in crate::card::sets) static GENERAL_S_ENFORCER: CardRecord = CardRecord::new(
    "General's Enforcer",
    "f6a45d0e-582a-4ec1-b83b-71b6a0ff6249",
    "Wisnu Tan",
    CardRules::new_creature(mana_cost!("{W}{B}"), &["Human", "Soldier"], 2, 3).with_abilities(&[
        AbilityDef::static_ability(
            "Legendary Humans you control have indestructible.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::from_name("Human")),
                        ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
            },
        ),
        AbilityDef::activated_with_targets(
            "{2}{W}{B}: Exile target card from a graveyard. If it was a \
             creature card, create a 1/1 white Human Soldier creature \
             token.",
            &[CostDef::Mana(mana_cost!("{2}{W}{B}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                },
                then: &EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                    EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        HUMAN_SOLDIER_TOKEN,
                    ))),
                ]),
                otherwise: &EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            },
        ),
    ]),
);

// IKO 189 — Genesis Ultimatum
pub(in crate::card::sets) static GENESIS_ULTIMATUM: CardRecord = CardRecord::new(
    "Genesis Ultimatum",
    "556dfe7a-43e8-4801-ad79-89ab2148eca6",
    "Jason Rainville",
    CardRules::new_sorcery(mana_cost!("{G}{G}{U}{U}{U}{R}{R}")).with_abilities(&[
        AbilityDef::spell(
            "Look at the top five cards of your library. Put any number \
             of permanent cards from among them onto the battlefield and \
             the rest into your hand. Exile Genesis Ultimatum.",
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(5),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
                minimum: 0,
                maximum: 5,
                chosen: crate::Binding!("chosen"),
                remainder: crate::Binding!("rest"),
                then: &EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!("rest"))),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                ]),
            }),
        )
        .with_resolution_destination(SpellResolutionDestinationDef::Exile),
    ]),
);

// IKO 190 — Illuna, Apex of Wishes
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static ILLUNA_APEX_OF_WISHES: CardRecord = CardRecord::new(
    "Illuna, Apex of Wishes",
    "9fc99c81-d112-4811-9bba-77a14d904692",
    "Chris Rahn",
    CardRules::unsupported(),
);

// IKO 191 — Inspired Ultimatum
pub(in crate::card::sets) static INSPIRED_ULTIMATUM: CardRecord = CardRecord::new(
    "Inspired Ultimatum",
    "dd64f064-8f05-41ef-b95b-1b723137f846",
    "Tyler Jacobson",
    CardRules::new_sorcery(mana_cost!("{U}{U}{R}{R}{R}{W}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target player gains 5 life, Inspired Ultimatum deals 5 \
             damage to any target, then you draw five cards.",
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Any)),
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget),
            ],
            EffectDef::Sequence(&[
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(5),
                },
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex(1)),
                    ValueDef::Constant(5),
                ),
                abilities::draw_cards(ValueDef::Constant(5)),
            ]),
        ),
    ]),
);

// IKO 192 — Kinnan, Bonder Prodigy (alternate printing)
const KINNAN_BONDER_PRODIGY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KINNAN_BONDER_PRODIGY,
    1,
    "63cda4a0-0dff-4edb-ae67-a2b7e2971350",
    "Jason Rainville",
);

// IKO 193 — Labyrinth Raptor
// Audit: unsupported — Needs a blocking relationship to the triggering attacker; BlockingSource only relates
// blockers to the ability's source.
pub(in crate::card::sets) static LABYRINTH_RAPTOR: CardRecord = CardRecord::new(
    "Labyrinth Raptor",
    "c608543d-cb60-44c3-a437-e4a18c311420",
    "Daarken",
    CardRules::unsupported(),
);

// IKO 194 — Lore Drakkis
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static LORE_DRAKKIS: CardRecord = CardRecord::new(
    "Lore Drakkis",
    "83e035ca-eccd-4b63-817c-f2c676b9c98d",
    "Lucas Graciano",
    CardRules::unsupported(),
);

// IKO 195 — Narset of the Ancient Way
pub(in crate::card::sets) static NARSET_OF_THE_ANCIENT_WAY: CardRecord = CardRecord::new(
    "Narset of the Ancient Way",
    "fa7b28d8-b835-44a0-978d-cadfd392fff5",
    "Yongjae Choi",
    CardRules::new_planeswalker(mana_cost!("{1}{U}{R}{W}"), &["Narset"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "+1: You gain 2 life. Add {U}, {R}, or {W}. Spend this mana only to \
                    cast a noncreature spell.",
                &[CostDef::Loyalty(ValueDef::Constant(1))],
                EffectDef::Sequence(&[
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                    },
                    EffectDef::AddMana(
                        AddManaEffectDef::choice(&[
                            ManaColor::Blue,
                            ManaColor::Red,
                            ManaColor::White,
                        ])
                        .with_restrictions(&[
                            ManaRestrictionDef::CastSpell(ObjectPredicateDef::NoncreatureSpell),
                        ]),
                    ),
                ]),
            ),
            AbilityDef::activated(
                "−2: Draw a card, then you may discard a card. When you discard a \
                    nonland card this way, Narset deals damage equal to that card's mana \
                    value to target creature or planeswalker.",
                &[CostDef::Loyalty(ValueDef::Constant(-2))],
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(1)),
                    EffectDef::May {
                        player: EffectRecipientDef::Controller,
                        effect: &EffectDef::Discard {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(1),
                            selection: DiscardSelectionDef::RecipientChooses,
                            then: Some(DiscardFollowUpDef {
                                counted: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                    CardType::Land,
                                )),
                                bound: Some(crate::Binding!("discarded")),
                                effect: &EffectDef::IfNoObjects(IfNoObjectsDef {
                                    input: ObjectSetDef::Binding(crate::Binding!("discarded")),
                                    if_empty: &EffectDef::None,
                                    otherwise: &EffectDef::ReflexiveTrigger(
                                        &AbilityDef::triggered_with_targets(
                                            "When you discard a nonland card this way, Narset deals damage equal to \
                                                that card's mana value to target creature or planeswalker.",
                                            TriggerEventDef::Reflexive,
                                            &[AbilityTargetDef::exactly_one(
                                                AbilityTargetPredicate::Object {
                                                    object: ObjectPredicateDef::AnyOf(&[
                                                        ObjectPredicateDef::HasType(
                                                            CardType::Creature,
                                                        ),
                                                        ObjectPredicateDef::HasType(
                                                            CardType::Planeswalker,
                                                        ),
                                                    ]),
                                                    zones: &[ZoneKind::Battlefield],
                                                    controller: None,
                                                    owner: None,
                                                },
                                            )],
                                            EffectDef::damage(
                                                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                                ValueDef::AggregateObjectValues(
                                                    &ObjectValueAggregateDef {
                                                        objects: ObjectSetDef::Binding(
                                                            crate::Binding!("discarded"),
                                                        ),
                                                        select: ObjectValueDef::ManaValue,
                                                        operation: AggregateOperationDef::Sum,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                }),
                            }),
                        },
                    },
                ]),
            ),
            AbilityDef::activated(
                "−6: You get an emblem with \"Whenever you cast a noncreature spell, \
                    this emblem deals 2 damage to any target.\"",
                &[CostDef::Loyalty(ValueDef::Constant(-6))],
                EffectDef::CreateEmblem {
                    creature_type: None,
                    emblem: EmblemCharacteristics::new(
                        "Narset of the Ancient Way emblem",
                        &[AbilityDef::triggered_with_targets(
                            "Whenever you cast a noncreature spell, this emblem deals 2 damage to \
                                any target.",
                            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                                ObjectPredicateDef::NoncreatureSpell,
                                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                            ])),
                            &[AbilityTargetDef::exactly_one(
                                AbilityTargetPredicate::AnyTarget,
                            )],
                            EffectDef::damage(
                                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                ValueDef::Constant(2),
                            ),
                        )],
                    ),
                },
            ),
        ]),
);

// IKO 196 — Necropanther
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static NECROPANTHER: CardRecord = CardRecord::new(
    "Necropanther",
    "2e1ed055-8988-4625-9d57-8ce8a4e04aea",
    "Jason A. Engle",
    CardRules::unsupported(),
);

// IKO 197 — Nethroi, Apex of Death
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static NETHROI_APEX_OF_DEATH: CardRecord = CardRecord::new(
    "Nethroi, Apex of Death",
    "8ca6eb5a-8bc9-4091-bcfb-b207f0afd188",
    "Slawomir Maniak",
    CardRules::unsupported(),
);

// IKO 198 — Offspring's Revenge
pub(in crate::card::sets) static OFFSPRING_S_REVENGE: CardRecord = CardRecord::new(
    "Offspring's Revenge",
    "78619ffb-f514-4f9f-9d77-3ab49e045f7c",
    "Daarken",
    CardRules::new_enchantment(mana_cost!("{2}{R}{W}{B}")).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "At the beginning of combat on your turn, exile target red, \
             white, or black creature card from your graveyard. Create a \
             token that's a copy of that card, except it's 1/1. It gains \
             haste until your next turn.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::Color(ManaColor::Red),
                            ObjectPredicateDef::Color(ManaColor::White),
                            ObjectPredicateDef::Color(ManaColor::Black),
                        ]),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::MoveObjects(MoveObjectsDef {
                input: ObjectSetDef::LegalTargets(TargetIndex::PRIMARY),
                from: Some(ZoneKind::Graveyard),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
                moved: Some(crate::Binding!("exiled")),
                then: &EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Copy(&TokenCopyDef {
                        object: &EffectRecipientDef::objects(ObjectSetDef::Binding(
                            crate::Binding!("exiled"),
                        )),
                        exceptions: CopyExceptionsDef::power_toughness(1, 1),
                    }))
                    .with_created_tokens(CreatedTokensDef {
                        binding: crate::ParentBinding,
                        then: &EffectDef::Apply {
                            recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::ParentBinding,
                            )),
                            effect: AppliedEffectDef::add_ability(&abilities::haste()),
                            duration: ResolvedEffectDurationDef::UntilYourNextTurn,
                        },
                    }),
                ),
            }),
        ),
    ]),
);

// IKO 199 — Parcelbeast
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static PARCELBEAST: CardRecord = CardRecord::new(
    "Parcelbeast",
    "610bb98c-d66a-44cc-92e2-a80d700b59e4",
    "Milivoj Ćeran",
    CardRules::unsupported(),
);

// IKO 200 — Primal Empathy
pub(in crate::card::sets) static PRIMAL_EMPATHY: CardRecord = CardRecord::new(
    "Primal Empathy",
    "034e0afb-7787-4cb8-8bb4-cf6997dbf7e4",
    "Micah Epstein",
    CardRules::new_enchantment(mana_cost!("{1}{G}{U}")).with_abilities(&[AbilityDef::triggered(
        "At the beginning of your upkeep, draw a card if you control \
         a creature with the greatest power among creatures on the \
         battlefield. Otherwise, put a +1/+1 counter on a creature \
         you control.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        EffectDef::IfElseCondition {
            condition: &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::PowerLessThan(
                            ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                                objects: ObjectSetDef::Query(ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::Any,
                                )),
                                select: ObjectValueDef::Power,
                                operation: AggregateOperationDef::Maximum,
                            }),
                        )),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            then: &abilities::draw_cards(ValueDef::Constant(1)),
            otherwise: &EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 1,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                        "chosen"
                    ))),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            }),
        },
    )]),
);

// IKO 201 — Quartzwood Crasher (alternate printing)
const QUARTZWOOD_CRASHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &QUARTZWOOD_CRASHER,
    1,
    "c8e4c609-19c9-433b-a852-7999e375ee4f",
    "Antonio José Manzanedo",
);

// IKO 202 — Regal Leosaur
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static REGAL_LEOSAUR: CardRecord = CardRecord::new(
    "Regal Leosaur",
    "b6fdf313-740b-4976-911f-9fb5eb54afce",
    "Ilse Gort",
    CardRules::unsupported(),
);

// IKO 203 — Rielle, the Everwise
// Audit: unsupported — Needs the first discard-batch event each turn, including discards before Rielle entered; a
// per-ability trigger limit is insufficient.
pub(in crate::card::sets) static RIELLE_THE_EVERWISE: CardRecord = CardRecord::new(
    "Rielle, the Everwise",
    "5f03c944-1929-4cb2-a373-d57eefa29ed1",
    "Yongjae Choi",
    CardRules::unsupported(),
);

// IKO 204 — Ruinous Ultimatum
pub(in crate::card::sets) static RUINOUS_ULTIMATUM: CardRecord = CardRecord::new(
    "Ruinous Ultimatum",
    "50c1d6ca-7789-46b5-bc89-85cc3915cb85",
    "Chase Stone",
    CardRules::new_sorcery(mana_cost!("{R}{R}{W}{W}{W}{B}{B}")).with_abilities(&[
        AbilityDef::spell(
            "Destroy all nonland permanents your opponents control.",
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                ),
                then: None,
            },
        ),
    ]),
);

// IKO 205 — Savai Thundermane
// Audit: unsupported — Needs a battlefield listener for another card being cycled; DiscardedToActivate only visits
// the discarded card itself.
pub(in crate::card::sets) static SAVAI_THUNDERMANE: CardRecord = CardRecord::new(
    "Savai Thundermane",
    "73d3925c-cf7d-4546-bc70-33f04d4b7566",
    "Svetlin Velinov",
    CardRules::unsupported(),
);

// IKO 206 — Skull Prophet
pub(in crate::card::sets) static SKULL_PROPHET: CardRecord = CardRecord::new(
    "Skull Prophet",
    "2f5ec787-79f6-4922-a0f7-debde8f7a4be",
    "Nils Hamm",
    CardRules::new_creature(mana_cost!("{B}{G}"), &["Human", "Druid"], 3, 1).with_abilities(&[
        abilities::tap_for_mana(
            "{T}: Add {B} or {G}.",
            AddManaEffectDef::choice(&[ManaColor::Black, ManaColor::Green]),
        ),
        AbilityDef::activated(
            "{T}: Mill two cards. (Put the top two cards of your library \
             into your graveyard.)",
            &[CostDef::TapSource],
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// IKO 207 — Skycat Sovereign
pub(in crate::card::sets) static SKYCAT_SOVEREIGN: CardRecord = CardRecord::new(
    "Skycat Sovereign",
    "6209dbfd-b765-44c5-b3ad-94f6e0705926",
    "Slawomir Maniak",
    CardRules::new_creature(mana_cost!("{W}{U}"), &["Elemental", "Cat"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature gets +1/+1 for each other creature you control \
             with flying.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                ),
            },
        ),
        AbilityDef::activated(
            "{2}{W}{U}: Create a 1/1 white Cat Bird creature token with \
             flying.",
            &[CostDef::Mana(mana_cost!("{2}{W}{U}"))],
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Cat", "Bird"], &[ManaColor::White], 1, 1)
                    .with_abilities(&[abilities::flying()])
                    .with_art(CardArt::new(
                        "816da759-e224-4960-b2b6-453cadc2faf1",
                        "Leesha Hannigan",
                    )),
            ))),
        ),
    ]),
);

// IKO 208 — Slitherwisp
pub(in crate::card::sets) static SLITHERWISP: CardRecord = CardRecord::new(
    "Slitherwisp",
    "8067f195-8b6e-4329-a34e-43e52f67c571",
    "Yigit Koroglu",
    CardRules::new_creature(mana_cost!("{U}{B}{B}"), &["Elemental", "Nightmare"], 3, 2)
        .with_abilities(&[
            abilities::flash(),
            AbilityDef::triggered(
                "Whenever you cast another spell that has flash, you draw a \
                 card and each opponent loses 1 life.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flash),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(1)),
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Opponent,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
        ]),
);

// IKO 209 — Snapdax, Apex of the Hunt
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static SNAPDAX_APEX_OF_THE_HUNT: CardRecord = CardRecord::new(
    "Snapdax, Apex of the Hunt",
    "51fdeeb7-9868-4fc2-9c53-806244cd5488",
    "Viktor Titov",
    CardRules::unsupported(),
);

// IKO 210 — Song of Creation
pub(in crate::card::sets) static SONG_OF_CREATION: CardRecord = CardRecord::new(
    "Song of Creation",
    "b7320233-1c83-4f61-8c08-cc8361867256",
    "Noah Bradley",
    CardRules::new_enchantment(mana_cost!("{1}{G}{U}{R}")).with_abilities(&[
        AbilityDef::static_ability(
            "You may play an additional land on each of your turns.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayAdditionalLands(1)),
            },
        ),
        AbilityDef::triggered(
            "Whenever you cast a spell, draw two cards.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Any,
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            abilities::draw_cards(ValueDef::Constant(2)),
        ),
        AbilityDef::triggered(
            "At the beginning of your end step, discard your hand.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            EffectDef::Discard {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ]),
);

// IKO 211 — Sprite Dragon
pub(in crate::card::sets) static SPRITE_DRAGON: CardRecord = CardRecord::new(
    "Sprite Dragon",
    "281f6118-adb8-4a7d-9c77-5570f3399e6e",
    "Gabor Szikszai",
    CardRules::new_creature(mana_cost!("{U}{R}"), &["Faerie", "Dragon"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::haste(),
        AbilityDef::triggered(
            "Whenever you cast a noncreature spell, put a +1/+1 counter \
             on this creature.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::NoncreatureSpell,
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// IKO 212 — Titans' Nest
// Audit: unsupported — Needs a mana-spending restriction matching a colored spell whose printed mana cost contains
// no X symbol.
pub(in crate::card::sets) static TITANS_NEST: CardRecord = CardRecord::new(
    "Titans' Nest",
    "75d4e6f3-73cb-4799-bcb9-c1f5bc234a02",
    "Cliff Childs",
    CardRules::unsupported(),
);

// IKO 213 — Trumpeting Gnarr
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static TRUMPETING_GNARR: CardRecord = CardRecord::new(
    "Trumpeting Gnarr",
    "063a95ee-3fda-436f-9ff8-de80cc874dde",
    "Aaron Miller",
    CardRules::unsupported(),
);

// IKO 214 — Vadrok, Apex of Thunder
// Audit: unsupported — Needs merged-permanent representation, mutate casting, and mutation events and history.
pub(in crate::card::sets) static VADROK_APEX_OF_THUNDER: CardRecord = CardRecord::new(
    "Vadrok, Apex of Thunder",
    "ed7b0256-d342-4523-9516-6a007bf51825",
    "Zack Stella",
    CardRules::unsupported(),
);

// IKO 215 — Whirlwind of Thought
pub(in crate::card::sets) static WHIRLWIND_OF_THOUGHT: CardRecord = CardRecord::new(
    "Whirlwind of Thought",
    "d0699cbc-b499-44a6-82e1-631491aaaec6",
    "Bram Sels",
    CardRules::new_enchantment(mana_cost!("{1}{U}{R}{W}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you cast a noncreature spell, draw a card.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::NoncreatureSpell,
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// IKO 216 — Winota, Joiner of Forces
// Audit: unsupported — Needs a frozen top-library choice that puts a selected Human card onto
// the battlefield tapped and attacking, then randomizes the remainder.
pub(in crate::card::sets) static WINOTA_JOINER_OF_FORCES: CardRecord = CardRecord::new(
    "Winota, Joiner of Forces",
    "5dd13a6c-23d3-44ce-a628-cb1c19d777c4",
    "Magali Villeneuve",
    crate::card::CardRules::unsupported(),
);

// IKO 217 — Zenith Flare
pub(in crate::card::sets) static ZENITH_FLARE: CardRecord = CardRecord::new(
    "Zenith Flare",
    "0efac1ed-3f01-487c-86be-8239568b4425",
    "Jonas De Ro",
    CardRules::new_instant(mana_cost!("{2}{R}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Zenith Flare deals X damage to any target and you gain X \
             life, where X is the number of cards with a cycling ability \
             in your graveyard.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::Sequence(&[
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasAbility(AbilityPredicateDef::Label(
                            &abilities::CYCLING,
                        )),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                ),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasAbility(AbilityPredicateDef::Label(
                            &abilities::CYCLING,
                        )),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                },
            ]),
        ),
    ]),
);

// IKO 218 — Alert Heedbonder
pub(in crate::card::sets) static ALERT_HEEDBONDER: CardRecord = CardRecord::new(
    "Alert Heedbonder",
    "15a2ccae-c322-4e99-a094-a68fc5d52ea3",
    "Randy Vargas",
    CardRules::new_creature(mana_cost!("{1}{G/W}{G/W}"), &["Human", "Scout"], 2, 4).with_abilities(
        &[
            abilities::vigilance(),
            AbilityDef::triggered(
                "At the beginning of your end step, you gain 1 life for each \
                 creature you control with vigilance.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::You,
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasKeyword(KeywordAbility::Vigilance),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                },
            ),
        ],
    ),
);

// IKO 219 — Cunning Nightbonder
pub(in crate::card::sets) static CUNNING_NIGHTBONDER: CardRecord = CardRecord::new(
    "Cunning Nightbonder",
    "adb202e7-f9a4-4785-840d-18aa6aa663b4",
    "Ekaterina Burmak",
    CardRules::new_creature(mana_cost!("{U/B}{U/B}"), &["Human", "Rogue"], 2, 2).with_abilities(&[
        abilities::flash(),
        AbilityDef::static_ability(
            "Spells with flash you cast cost {1} less to cast.",
            EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                ObjectPredicateDef::HasKeyword(KeywordAbility::Flash),
                PlayerRelation::You,
                ValueDef::Constant(1),
            )),
        ),
        AbilityDef::static_ability(
            "Spells with flash you cast can't be countered.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Flash),
                    &[ZoneKind::Stack],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered),
            },
        ),
    ]),
);

// IKO 220 — Fiend Artisan
pub(in crate::card::sets) static FIEND_ARTISAN: CardRecord = CardRecord::new(
    "Fiend Artisan",
    "6cd9d800-6d31-42e2-87d2-772db0ff95ed",
    "Yigit Koroglu",
    CardRules::new_creature(mana_cost!("{B/G}{B/G}"), &["Nightmare"], 1, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This creature gets +1/+1 for each creature card in your \
             graveyard.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                ),
            },
        ),
        AbilityDef::activated(
            "{X}{B/G}, {T}, Sacrifice another creature: Search your \
             library for a creature card with mana value X or less, put \
             it onto the battlefield, then shuffle. Activate only as a \
             sorcery.",
            &[
                CostDef::Mana(mana_cost!("{X}{B/G}")),
                CostDef::TapSource,
                CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ])),
            ],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ManaValueAtMostValue(ValueDef::ChosenX),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// IKO 221 — Gyruda, Doom of Depths
// Audit: unsupported — Needs a mana-value parity requirement for the starting deck and a parity predicate
// over the frozen cards each player milled; the shared Companion selection and access are implemented.
pub(in crate::card::sets) static GYRUDA_DOOM_OF_DEPTHS: CardRecord = CardRecord::new(
    "Gyruda, Doom of Depths",
    "97eb1804-6fd8-4917-af36-87fdfce39d3a",
    "Tyler Jacobson",
    CardRules::unsupported(),
);

// IKO 222 — Jegantha, the Wellspring
// Audit: unsupported — Needs a per-card mana-symbol multiplicity requirement and a payment restriction
// forbidding generic-cost payment; the shared Companion selection and access are implemented.
pub(in crate::card::sets) static JEGANTHA_THE_WELLSPRING: CardRecord = CardRecord::new(
    "Jegantha, the Wellspring",
    "1d52e527-3835-4350-8c01-0f2d5d623b9c",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

// IKO 223 — Jubilant Skybonder
// Audit: unsupported — Needs cost-modifier discovery through granted static abilities; the granted tax must follow
// each flying creature and disappear with ability loss.
pub(in crate::card::sets) static JUBILANT_SKYBONDER: CardRecord = CardRecord::new(
    "Jubilant Skybonder",
    "7f8f2d69-fba9-40e5-8ef7-75e14ef4070a",
    "Jesper Ejsing",
    CardRules::unsupported(),
);

// IKO 224 — Kaheera, the Orphanguard
pub(in crate::card::sets) static KAHEERA_THE_ORPHANGUARD: CardRecord = CardRecord::new(
    "Kaheera, the Orphanguard",
    "d4ebed0b-8060-4a7b-a060-5cfcd2172b16",
    "Ryan Pancoast",
    CardRules::new_creature(mana_cost!("{1}{G/W}{G/W}"), &["Cat", "Beast"], 3, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            companion(
                "Companion — Each creature card in your starting deck is a Cat, Elemental, \
                 Nightmare, Dinosaur, or Beast card. (If this card is your chosen companion, \
                 you may put it into your hand from outside the game for {3} as a sorcery.)",
                DeckCards::creatures().all(CardRequirement::HasAnySubtype(&[
                    "Cat",
                    "Elemental",
                    "Nightmare",
                    "Dinosaur",
                    "Beast",
                ])),
            ),
            abilities::vigilance(),
            AbilityDef::static_ability(
                "Each other creature you control that's a Cat, Elemental, Nightmare, Dinosaur, \
                 or Beast gets +1/+1 and has vigilance.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::Subtype(SubtypeDef::from_name("Cat")),
                                ObjectPredicateDef::Subtype(SubtypeDef::from_name("Elemental")),
                                ObjectPredicateDef::Subtype(SubtypeDef::from_name("Nightmare")),
                                ObjectPredicateDef::Subtype(SubtypeDef::from_name("Dinosaur")),
                                ObjectPredicateDef::Subtype(SubtypeDef::from_name("Beast")),
                            ]),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::vigilance()),
                    ]),
                },
            ),
        ]),
);

// IKO 225 — Keruga, the Macrosage
pub(in crate::card::sets) static KERUGA_THE_MACROSAGE: CardRecord = CardRecord::new(
    "Keruga, the Macrosage",
    "a90ee952-de7a-420f-993c-a38db89bc8ac",
    "Dan Murayama Scott",
    CardRules::new_creature(mana_cost!("{3}{G/U}{G/U}"), &["Dinosaur", "Hippo"], 5, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            companion(
                "Companion — Your starting deck contains only cards with mana value 3 or greater \
                 and land cards. (If this card is your chosen companion, you may put it into \
                 your hand from outside the game for {3} as a sorcery.)",
                DeckCards::nonlands().all(CardRequirement::ManaValueAtLeast(3)),
            ),
            abilities::enters_trigger(
                "When Keruga enters, draw a card for each other permanent you control with mana \
                 value 3 or greater.",
                abilities::draw_cards(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(2)),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ))),
            ),
        ]),
);

// IKO 226 — Lurrus of the Dream-Den
pub(in crate::card::sets) static LURRUS_OF_THE_DREAM_DEN: CardRecord = CardRecord::new(
    "Lurrus of the Dream-Den",
    "5ad36fb2-c44e-4085-ba0d-54277841ad3a",
    "Slawomir Maniak",
    // Three mana for a lifelinking body that turns every cheap permanent in
    // the graveyard back into a card, one a turn -- which is why the decks
    // that play him keep their curve at two.
    CardRules::new_creature(mana_cost!("{1}{W/B}{W/B}"), &["Cat", "Nightmare"], 3, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            companion(
                "Companion — Each permanent card in your starting deck has mana value 2 or less. \
                 (If this card is your chosen companion, you may put it into your hand from \
                 outside the game for {3} as a sorcery.)",
                DeckCards::permanents().all(CardRequirement::ManaValueAtMost(2)),
            ),
            abilities::lifelink(),
            AbilityDef::static_ability(
                "Once during each of your turns, you may cast a permanent spell with mana value 2 \
                 or less from your graveyard.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlay(
                        // "A permanent spell with mana value 2 or less." The action is a cast, so a
                        // land card in the graveyard is not among them: lands are played rather
                        // than cast, which is what keeps this from being a Crucible.
                        PlayPermissionDef::once_each_of_your_turns(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::Any,
                                &[ZoneKind::Graveyard],
                                PlayerRelation::You,
                            ),
                            PlayRestrictionDef::new(
                                PlayActionMatcherDef::CastSpell,
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::AnyOf(&[
                                        ObjectPredicateDef::HasType(CardType::Artifact),
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        ObjectPredicateDef::HasType(CardType::Enchantment),
                                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                                    ]),
                                    ObjectPredicateDef::ManaValueAtMost(2),
                                ]),
                            ),
                        ),
                    )),
                },
            ),
        ]),
);

// IKO 227 — Lutri, the Spellchaser (alternate printing)
const LUTRI_THE_SPELLCHASER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LUTRI_THE_SPELLCHASER,
    1,
    "fb1189c9-7842-466e-8238-1e02677d8494",
    "Lie Setiawan",
);

// IKO 228 — Obosh, the Preypiercer
// Audit: unsupported — Needs a mana-value parity requirement for nonlands and damage replacement filtered by
// the source's mana-value parity; the shared Companion selection and access are implemented.
pub(in crate::card::sets) static OBOSH_THE_PREYPIERCER: CardRecord = CardRecord::new(
    "Obosh, the Preypiercer",
    "451507de-9c42-43ee-b9ba-1f69e9aa29d2",
    "Daarken",
    CardRules::unsupported(),
);

// IKO 229 — Proud Wildbonder
// Audit: unsupported — Needs granting an executable static combat-assignment ability; granted static clauses
// currently support only power/toughness effects.
pub(in crate::card::sets) static PROUD_WILDBONDER: CardRecord = CardRecord::new(
    "Proud Wildbonder",
    "f4876e17-a206-4351-9c0b-0845db4569a3",
    "Dmitry Burmak",
    CardRules::unsupported(),
);

// IKO 230 — Sonorous Howlbonder
pub(in crate::card::sets) static SONOROUS_HOWLBONDER: CardRecord = CardRecord::new(
    "Sonorous Howlbonder",
    "595f0b25-6e6e-43c4-a4a0-903920067df1",
    "Kimonas Theodossiou",
    CardRules::new_creature(mana_cost!("{1}{B/R}{B/R}"), &["Human", "Warrior"], 2, 2)
        .with_abilities(&[
            abilities::menace(),
            AbilityDef::static_ability(
                "Each creature you control with menace can't be blocked \
                 except by three or more creatures.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasKeyword(KeywordAbility::Menace),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                        BlockRestrictionDef::MinimumBlockers(3),
                    )),
                },
            ),
        ]),
);

// IKO 231 — Umori, the Collector
// Audit: unsupported — Needs a shared-card-type requirement for nonlands and an entry-time card-type
// designation read by spell pricing; the shared Companion selection and access are implemented.
pub(in crate::card::sets) static UMORI_THE_COLLECTOR: CardRecord = CardRecord::new(
    "Umori, the Collector",
    "75ac31e0-ac70-4ee6-b2b1-cc445ffa1da9",
    "Jehan Choo",
    CardRules::unsupported(),
);

// IKO 232 — Yorion, Sky Nomad
pub(in crate::card::sets) static YORION_SKY_NOMAD: CardRecord = CardRecord::new(
    "Yorion, Sky Nomad",
    "275426c4-c14e-47d0-a9d4-24da7f6f6911",
    "Steven Belledin",
    CardRules::new_creature(mana_cost!("{3}{W/U}{W/U}"), &["Bird", "Serpent"], 4, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            companion(
                "Companion — Your starting deck contains at least twenty cards more than the \
                 minimum deck size. (If this card is your chosen companion, you may put it into \
                 your hand from outside the game for {3} as a sorcery.)",
                DeckRequirementDef::MinimumSizeAboveFormat(20),
            ),
            abilities::flying(),
            abilities::enters_trigger(
                "When Yorion enters, exile any number of other nonland permanents you own and \
                 control. Return those cards to the battlefield at the beginning of the \
                 next end step.",
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("blink")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                            ObjectPredicateDef::OwnedBy(PlayerRelation::You),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 0,
                    maximum: usize::MAX,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::WithZoneMoveResult {
                        effect: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "blink"
                            ))),
                            ZoneKind::Exile,
                            ZonePlacement::Top,
                        ),
                        binding: crate::Binding!("exiled"),
                        then: &EffectDef::InstallTrigger(crate::card::InstalledTriggerDef::once(
                            &AbilityDef::triggered(
                                "At the beginning of the next end step, return those cards \
                                 to the battlefield.",
                                TriggerEventDef::StepBegins {
                                    step: TurnStepDef::End,
                                    player: PlayerRelation::Any,
                                },
                                EffectDef::move_to_zone(
                                    EffectRecipientDef::objects(ObjectSetDef::InZone {
                                        objects: &ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                            crate::Binding!("exiled"),
                                        ),
                                        zone: ZoneKind::Exile,
                                    }),
                                    ZoneKind::Battlefield,
                                    ZonePlacement::Top,
                                ),
                            ),
                        )),
                    },
                }),
            ),
        ]),
);

// IKO 233 — Zirda, the Dawnwaker
pub(in crate::card::sets) static ZIRDA_THE_DAWNWAKER: CardRecord = CardRecord::new(
    "Zirda, the Dawnwaker",
    "1bd8e61c-2ee8-4243-a848-7008810db8a0",
    "Jesper Ejsing",
    // Three mana for a 3/3 that makes every activated ability on the board
    // two cheaper, which is what a deck full of equipment and pingers is
    // waiting for.
    CardRules::new_creature(mana_cost!("{1}{R/W}{R/W}"), &["Elemental", "Fox"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            companion(
                "Companion — Each permanent card in your starting deck has an activated ability. \
                 (If this card is your chosen companion, you may put it into your hand from \
                 outside the game for {3} as a sorcery.)",
                DeckCards::permanents().all(CardRequirement::HasActivatedAbility),
            ),
            AbilityDef::static_ability(
                "Abilities you activate that aren't mana abilities cost {2} less to activate. \
                 This effect can't reduce the mana in that cost to less than one mana.",
                EffectDef::ModifyCost(CostModificationDef::AbilityReduction {
                    target: None,
                    abilities: crate::card::AbilityKindDef::NonManaActivated,
                    // "Abilities you activate", which is wider than the permanents you
                    // control: cycling and the rest of what a card in a hand or a
                    // graveyard prints is an ability you activate too, and the shared
                    // vocabulary reaches those objects with the same predicate. The
                    // ability-kind selector excludes mana abilities while retaining
                    // the shared pricing path.
                    permanent: ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    amount: ValueDef::Constant(2),
                    minimum: 1,
                }),
            ),
            AbilityDef::activated_with_targets(
                "{1}, {T}: Target creature can't block this turn.",
                &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// IKO 234 — Crystalline Giant
// Audit: unsupported — Needs uniform random selection from counter kinds absent on this permanent, followed by
// adding that chosen kind.
pub(in crate::card::sets) static CRYSTALLINE_GIANT: CardRecord = CardRecord::new(
    "Crystalline Giant",
    "1146c418-2176-466e-8a06-f2ef6bf2b1a9",
    "Jason Rainville",
    CardRules::unsupported(),
);

// IKO 235 — Indatha Crystal
pub(in crate::card::sets) static INDATHA_CRYSTAL: CardRecord = CardRecord::new(
    "Indatha Crystal",
    "bdace59f-f025-4717-8eb4-3d1e13b31d2b",
    "Raoul Vitale",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        abilities::tap_for_mana(
            "{T}: Add {W}, {B}, or {G}.",
            AddManaEffectDef::choice(&[ManaColor::White, ManaColor::Black, ManaColor::Green]),
        ),
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
    ]),
);

// IKO 236 — Ketria Crystal
pub(in crate::card::sets) static KETRIA_CRYSTAL: CardRecord = CardRecord::new(
    "Ketria Crystal",
    "7df435cb-3aeb-490a-8fca-91f3b6936965",
    "Yeong-Hao Han",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        abilities::tap_for_mana(
            "{T}: Add {G}, {U}, or {R}.",
            AddManaEffectDef::choice(&[ManaColor::Green, ManaColor::Blue, ManaColor::Red]),
        ),
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
    ]),
);

// IKO 237 — The Ozolith
// Audit: unsupported — Needs last-known per-kind counter snapshots on leaving creatures and moving all counter
// kinds between objects.
pub(in crate::card::sets) static THE_OZOLITH: CardRecord = CardRecord::new(
    "The Ozolith",
    "9341ed06-53db-4604-b60a-3ea9129afbc2",
    "Sam Burley",
    CardRules::unsupported(),
);

// IKO 238 — Raugrin Crystal
pub(in crate::card::sets) static RAUGRIN_CRYSTAL: CardRecord = CardRecord::new(
    "Raugrin Crystal",
    "c5bce0fb-53d6-47ee-8c5a-a99e50f67fc9",
    "Kirsten Zirngibl",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        abilities::tap_for_mana(
            "{T}: Add {U}, {R}, or {W}.",
            AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Red, ManaColor::White]),
        ),
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
    ]),
);

// IKO 239 — Savai Crystal
pub(in crate::card::sets) static SAVAI_CRYSTAL: CardRecord = CardRecord::new(
    "Savai Crystal",
    "6954a5c1-89b1-4edd-814e-8f88fd49cda3",
    "Daniel Ljunggren",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        abilities::tap_for_mana(
            "{T}: Add {R}, {W}, or {B}.",
            AddManaEffectDef::choice(&[ManaColor::Red, ManaColor::White, ManaColor::Black]),
        ),
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
    ]),
);

// IKO 240 — Sleeper Dart
pub(in crate::card::sets) static SLEEPER_DART: CardRecord = CardRecord::new(
    "Sleeper Dart",
    "311f827a-1585-483e-b0fd-3dc05969b485",
    "Sidharth Chaturvedi",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        abilities::enters_trigger(
            "When this artifact enters, draw a card.",
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this artifact: Target creature doesn't untap \
             during its controller's next untap step.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::SkipNextUntapSteps {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                count: 1,
            },
        ),
    ]),
);

// IKO 241 — Springjaw Trap
pub(in crate::card::sets) static SPRINGJAW_TRAP: CardRecord = CardRecord::new(
    "Springjaw Trap",
    "3741de51-ea92-493a-8058-e0f2000e7701",
    "Zoltan Boros",
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        abilities::flash(),
        AbilityDef::activated_with_targets(
            "{4}, {T}, Sacrifice this artifact: It deals 3 damage to any \
             target.",
            &[
                CostDef::Mana(mana_cost!("{4}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
        ),
    ]),
);

// IKO 242 — Zagoth Crystal
pub(in crate::card::sets) static ZAGOTH_CRYSTAL: CardRecord = CardRecord::new(
    "Zagoth Crystal",
    "9138a442-8e8b-465f-bb76-b6af7e6dab6f",
    "Raoul Vitale",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        abilities::tap_for_mana(
            "{T}: Add {B}, {G}, or {U}.",
            AddManaEffectDef::choice(&[ManaColor::Black, ManaColor::Green, ManaColor::Blue]),
        ),
        abilities::cycling!("Cycling {2}", &[CostDef::Mana(mana_cost!("{2}"))]),
    ]),
);

// IKO 243 — Bloodfell Caves (reprint)
const BLOODFELL_CAVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::khans_of_tarkir::BLOODFELL_CAVES,
    "0c8269e6-b30c-4fdc-82a7-d503c133afa6",
    "Titus Lunter",
);

// IKO 244 — Blossoming Sands (reprint)
const BLOSSOMING_SANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::khans_of_tarkir::BLOSSOMING_SANDS,
    "828044d6-0f53-4caf-a581-d71919df4175",
    "Jonas De Ro",
);

// IKO 245 — Bonders' Enclave
pub(in crate::card::sets) static BONDERS_ENCLAVE: CardRecord = CardRecord::new(
    "Bonders' Enclave",
    "4fe9388d-b1ee-4f35-9fbd-5f504528b398",
    "Cliff Childs",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{3}, {T}: Draw a card. Activate only if you control a \
             creature with power 4 or greater.",
            &[CostDef::Mana(mana_cost!("{3}")), CostDef::TapSource],
            abilities::draw_cards(ValueDef::Constant(1)),
        )
        .with_activation_condition(&TriggerConditionDef::ObjectCount {
            query: ObjectQueryDef::matching(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::PowerAtLeast(4),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            comparison: ComparisonDef::GreaterOrEqual,
            amount: 1,
        }),
    ]),
);

// IKO 246 — Dismal Backwater (reprint)
const DISMAL_BACKWATER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::khans_of_tarkir::DISMAL_BACKWATER,
    "9de0981e-54fc-4919-96a4-a9608a5452fc",
    "Eytan Zana",
);

// IKO 247 — Evolving Wilds (reprint)
const EVOLVING_WILDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::rise_of_the_eldrazi::EVOLVING_WILDS,
    "8e3d6745-8904-4905-a98f-d43f18b51d6d",
    "Jonas De Ro",
);

// IKO 248 — Indatha Triome
pub(in crate::card::sets) static INDATHA_TRIOME: CardRecord = CardRecord::new(
    "Indatha Triome",
    "2b74bb81-fb9a-40e5-a941-e517430b52f5",
    "Noah Bradley",
    triome(&["Plains", "Swamp", "Forest"]),
);

// IKO 249 — Jungle Hollow (reprint)
const JUNGLE_HOLLOW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::khans_of_tarkir::JUNGLE_HOLLOW,
    "01926862-bf2d-4a2b-af94-1ea5e4dd7444",
    "Jonas De Ro",
);

// IKO 250 — Ketria Triome
pub(in crate::card::sets) static KETRIA_TRIOME: CardRecord = CardRecord::new(
    "Ketria Triome",
    "a249b1f4-2b22-4b67-a207-e0c4ae95d2e1",
    "Sam Burley",
    triome(&["Forest", "Island", "Mountain"]),
);

// IKO 251 — Raugrin Triome
pub(in crate::card::sets) static RAUGRIN_TRIOME: CardRecord = CardRecord::new(
    "Raugrin Triome",
    "02138fbb-3962-4348-8d31-faaefba0b8b2",
    "Jonas De Ro",
    triome(&["Island", "Mountain", "Plains"]),
);

// IKO 252 — Rugged Highlands (reprint)
const RUGGED_HIGHLANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::khans_of_tarkir::RUGGED_HIGHLANDS,
    "9a69fa8f-1c2b-453d-8d54-2748890ce925",
    "Adam Paquette",
);

// IKO 253 — Savai Triome
pub(in crate::card::sets) static SAVAI_TRIOME: CardRecord = CardRecord::new(
    "Savai Triome",
    "748e6a61-9c1f-4225-9f04-e54002f63ac3",
    "Titus Lunter",
    triome(&["Mountain", "Plains", "Swamp"]),
);

// IKO 254 — Scoured Barrens (reprint)
const SCOURED_BARRENS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::khans_of_tarkir::SCOURED_BARRENS,
    "2282018a-46c8-41ca-ab93-f7dbf32cd295",
    "Cliff Childs",
);

// IKO 255 — Swiftwater Cliffs (reprint)
const SWIFTWATER_CLIFFS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::khans_of_tarkir::SWIFTWATER_CLIFFS,
    "7108c071-5f1c-4cf5-90c6-530cee3f7685",
    "Adam Paquette",
);

// IKO 256 — Thornwood Falls (reprint)
const THORNWOOD_FALLS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::khans_of_tarkir::THORNWOOD_FALLS,
    "8d011b0f-1681-4a51-9f5d-47ad135d03fe",
    "Adam Paquette",
);

// IKO 257 — Tranquil Cove (reprint)
const TRANQUIL_COVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::khans_of_tarkir::TRANQUIL_COVE,
    "366ac097-39cb-4401-87c7-1dd73bf34329",
    "Jonas De Ro",
);

// IKO 258 — Wind-Scarred Crag (reprint)
const WIND_SCARRED_CRAG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::khans_of_tarkir::WIND_SCARRED_CRAG,
    "86367edc-9587-4f3b-aa95-c3a2bfc8c6f4",
    "Titus Lunter",
);

// IKO 259 — Zagoth Triome
pub(in crate::card::sets) static ZAGOTH_TRIOME: CardRecord = CardRecord::new(
    "Zagoth Triome",
    "cc520518-2063-4b57-a0d4-10cf62a7175e",
    "Eytan Zana",
    triome(&["Swamp", "Forest", "Island"]),
);

// IKO 260 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::alpha::PLAINS,
    "16ebbce9-fd10-4c14-b52d-cf82c0c1a58c",
    "Cliff Childs",
);

// IKO 261 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::PLAINS,
    1,
    "e84f2b9b-1412-476f-8739-17d35ea48a51",
    "Alayna Danner",
);

// IKO 262 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::PLAINS,
    2,
    "8a65b379-47a9-48f2-be6e-abb4e7868ad0",
    "Jesper Ejsing",
);

// IKO 263 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::alpha::ISLAND,
    "4b2ad5b3-7257-4521-8916-6b1cbfb89e27",
    "Alayna Danner",
);

// IKO 264 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::ISLAND,
    1,
    "0b9bbc32-a89a-4bed-ab52-ac6569ec74ce",
    "Jesper Ejsing",
);

// IKO 265 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::ISLAND,
    2,
    "be169f8c-6472-40ec-9b4a-0edcb63e9e2f",
    "Nick Southam",
);

// IKO 266 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::alpha::SWAMP,
    "6c8c3f0e-7af4-410b-a675-9ea84f51e812",
    "Alayna Danner",
);

// IKO 267 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::SWAMP,
    1,
    "e1d99025-46bc-4848-bcbc-bee858ed906c",
    "Jesper Ejsing",
);

// IKO 268 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::SWAMP,
    2,
    "45991831-1018-4f98-a4ad-0998a9577d97",
    "Svetlin Velinov",
);

// IKO 269 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::alpha::MOUNTAIN,
    "ae3d2fcd-11e0-4071-8c53-cb3315b7360a",
    "Alayna Danner",
);

// IKO 270 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::MOUNTAIN,
    1,
    "eb1ba09d-ecdd-48c0-af0b-3dc5ef908f9e",
    "Jesper Ejsing",
);

// IKO 271 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::MOUNTAIN,
    2,
    "e609028b-43b8-4aea-9f9a-25aa127482db",
    "Adam Paquette",
);

// IKO 272 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::alpha::FOREST,
    "9c348494-f60c-4bd1-9077-bff24f2e634b",
    "Alayna Danner",
);

// IKO 273 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::FOREST,
    1,
    "eb61e54a-646b-4c0a-88fe-f55d514202aa",
    "Jesper Ejsing",
);

// IKO 274 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::alpha::FOREST,
    2,
    "f7cf3bbf-acc0-4fe7-a631-aca698190ce2",
    "Yeong-Hao Han",
);

// IKO 275 — Zilortha, Strength Incarnate
// Audit: unsupported — Needs a combat lethal-damage rule that substitutes controlled creatures'
// power for toughness.
pub(in crate::card::sets) static ZILORTHA_STRENGTH_INCARNATE: CardRecord = CardRecord::new(
    "Zilortha, Strength Incarnate",
    "9a0639a0-c898-4a07-975c-a02bdd53175b",
    "Antonio José Manzanedo",
    crate::card::CardRules::unsupported(),
);

// IKO 276 — Lukka, Coppercoat Outcast (alternate printing)
const LUKKA_COPPERCOAT_OUTCAST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LUKKA_COPPERCOAT_OUTCAST,
    1,
    "2a8f74c3-c552-4331-a47c-e8155841bb27",
    "Kieran Yanner",
);

// IKO 277 — Vivien, Monsters' Advocate (alternate printing)
const VIVIEN_MONSTERS_ADVOCATE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VIVIEN_MONSTERS_ADVOCATE,
    1,
    "3e7ce2ee-2ebc-4336-a9f3-02380ea9784b",
    "Kev Walker",
);

// IKO 278 — Narset of the Ancient Way (alternate printing)
const NARSET_OF_THE_ANCIENT_WAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NARSET_OF_THE_ANCIENT_WAY,
    1,
    "91ac6589-75ee-4fbf-8056-1860c1482592",
    "Kev Walker",
);

// IKO 279 — Cubwarden (alternate printing)
const CUBWARDEN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CUBWARDEN,
    1,
    "17398d7f-baf3-4605-aced-21582a81c73a",
    "Denis Medri",
);

// IKO 280 — Huntmaster Liger (alternate printing)
const HUNTMASTER_LIGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HUNTMASTER_LIGER,
    1,
    "208084f5-35d4-4042-bdc6-dc2bf14c9990",
    "Steve Ellis",
);

// IKO 281 — Majestic Auricorn (alternate printing)
const MAJESTIC_AURICORN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MAJESTIC_AURICORN,
    1,
    "3d63b878-4d0d-444c-9db2-05cf5dd3fa8c",
    "Steve Ellis",
);

// IKO 282 — Vulpikeet (alternate printing)
const VULPIKEET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VULPIKEET,
    1,
    "044dae1f-a767-49f8-b4e2-85161f2685d5",
    "Justine Mara Andersen",
);

// IKO 283 — Archipelagore (alternate printing)
const ARCHIPELAGORE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARCHIPELAGORE,
    1,
    "01cda84f-7bd8-4166-b496-a04a4baa9b62",
    "Kev Walker",
);

// IKO 284 — Dreamtail Heron (alternate printing)
const DREAMTAIL_HERON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DREAMTAIL_HERON,
    1,
    "2d1828b1-a6eb-4346-b405-7642e34b3e71",
    "Tomasz Jedruszek",
);

// IKO 285 — Pouncing Shoreshark (alternate printing)
const POUNCING_SHORESHARK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &POUNCING_SHORESHARK,
    1,
    "54428228-83a0-440f-afe9-573c9d8640cc",
    "Daniel Warren Johnson",
);

// IKO 286 — Sea-Dasher Octopus (alternate printing)
const SEA_DASHER_OCTOPUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SEA_DASHER_OCTOPUS,
    1,
    "bd6986a8-e258-46f0-88a9-8f84732f359e",
    "Antonio Bravo",
);

// IKO 287 — Cavern Whisperer (alternate printing)
const CAVERN_WHISPERER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CAVERN_WHISPERER,
    1,
    "fecae13c-fbbf-4199-a1dc-0e14263ed887",
    "Steve Ellis",
);

// IKO 288 — Chittering Harvester (alternate printing)
const CHITTERING_HARVESTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHITTERING_HARVESTER,
    1,
    "dbd157ee-d17e-402c-a97d-206445dcf864",
    "Carl Critchlow",
);

// IKO 289 — Dirge Bat (alternate printing)
const DIRGE_BAT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DIRGE_BAT,
    1,
    "bef6998d-bda7-4847-b3d6-30e1596bf730",
    "Kev Walker",
);

// IKO 290 — Insatiable Hemophage (alternate printing)
const INSATIABLE_HEMOPHAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INSATIABLE_HEMOPHAGE,
    1,
    "ada6588b-3efc-4464-ae83-11586eb54775",
    "Steve Ellis",
);

// IKO 291 — Cloudpiercer (alternate printing)
const CLOUDPIERCER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CLOUDPIERCER,
    1,
    "3db7d906-1794-40eb-9d73-1f2a4db3a13f",
    "Steve Ellis",
);

// IKO 292 — Everquill Phoenix (alternate printing)
const EVERQUILL_PHOENIX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EVERQUILL_PHOENIX,
    1,
    "7fe3c735-d091-4a32-8d10-38a7f23e5c65",
    "Daniel Warren Johnson",
);

// IKO 293 — Porcuparrot (alternate printing)
const PORCUPARROT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PORCUPARROT,
    1,
    "e6373fe1-c834-419e-8a0b-590fb5dc555e",
    "Denis Medri",
);

// IKO 294 — Auspicious Starrix (alternate printing)
const AUSPICIOUS_STARRIX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AUSPICIOUS_STARRIX,
    1,
    "f7b41cfa-b22e-4d34-bfe9-68c9d8740704",
    "Justine Mara Andersen",
);

// IKO 295 — Gemrazer (alternate printing)
const GEMRAZER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GEMRAZER,
    1,
    "d75546a5-81fd-41c1-a081-d8980f6bd60a",
    "Andrew Huerta",
);

// IKO 296 — Glowstone Recluse (alternate printing)
const GLOWSTONE_RECLUSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GLOWSTONE_RECLUSE,
    1,
    "980dcb8f-fc04-49d2-9859-830ca3d12af6",
    "Denis Medri",
);

// IKO 297 — Migratory Greathorn (alternate printing)
const MIGRATORY_GREATHORN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MIGRATORY_GREATHORN,
    1,
    "1e31f56d-bf75-4e14-94de-5c77193abf3a",
    "Kev Walker",
);

// IKO 298 — Boneyard Lurker (alternate printing)
const BONEYARD_LURKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BONEYARD_LURKER,
    1,
    "2e0232c0-0867-4217-8e5d-b3454c0c8dab",
    "Carl Critchlow",
);

// IKO 299 — Brokkos, Apex of Forever (alternate printing)
const BROKKOS_APEX_OF_FOREVER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BROKKOS_APEX_OF_FOREVER,
    1,
    "5ec4dfeb-d2a8-4b34-8605-75a79c706fe0",
    "Daniel Warren Johnson",
);

// IKO 300 — Illuna, Apex of Wishes (alternate printing)
const ILLUNA_APEX_OF_WISHES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ILLUNA_APEX_OF_WISHES,
    1,
    "5c974c68-1a9f-457a-8bcb-265d29fee8e4",
    "Justine Mara Andersen",
);

// IKO 301 — Lore Drakkis (alternate printing)
const LORE_DRAKKIS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LORE_DRAKKIS,
    1,
    "e938fac3-544a-4f27-9726-a67153392031",
    "Andrew Huerta",
);

// IKO 302 — Necropanther (alternate printing)
const NECROPANTHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NECROPANTHER,
    1,
    "a776f88b-65f5-4cac-8207-c7e6b976d206",
    "Kev Walker",
);

// IKO 303 — Nethroi, Apex of Death (alternate printing)
const NETHROI_APEX_OF_DEATH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NETHROI_APEX_OF_DEATH,
    1,
    "ba04c102-e4c1-43a8-9c2f-d1f3c9bf5edf",
    "Tomasz Jedruszek",
);

// IKO 304 — Parcelbeast (alternate printing)
const PARCELBEAST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PARCELBEAST,
    1,
    "f5ac98e5-a22c-41b5-94a9-b37b5aeb124f",
    "Justine Mara Andersen",
);

// IKO 305 — Regal Leosaur (alternate printing)
const REGAL_LEOSAUR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REGAL_LEOSAUR,
    1,
    "cdb8801b-6cfd-4ada-b6cc-bac09117f4d5",
    "Tomasz Jedruszek",
);

// IKO 306 — Snapdax, Apex of the Hunt (alternate printing)
const SNAPDAX_APEX_OF_THE_HUNT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SNAPDAX_APEX_OF_THE_HUNT,
    1,
    "ceffd13b-8039-44ed-bd7f-68d46476dbe7",
    "Andrew Huerta",
);

// IKO 307 — Trumpeting Gnarr (alternate printing)
const TRUMPETING_GNARR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TRUMPETING_GNARR,
    1,
    "2fe88a45-a420-4998-b242-b475c6b5b0bc",
    "Justine Mara Andersen",
);

// IKO 308 — Vadrok, Apex of Thunder (alternate printing)
const VADROK_APEX_OF_THUNDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VADROK_APEX_OF_THUNDER,
    1,
    "658b75a1-c12c-4904-8a65-4eb75d1d32f7",
    "Tomasz Jedruszek",
);

// IKO 309 — Indatha Triome (alternate printing)
const INDATHA_TRIOME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INDATHA_TRIOME,
    1,
    "9bf2b208-79c6-4c4c-bf66-871352ed600f",
    "Robbie Trevino",
);

// IKO 310 — Ketria Triome (alternate printing)
const KETRIA_TRIOME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KETRIA_TRIOME,
    1,
    "f5b5b9bb-ee9f-4a52-bd74-fd2759c8e3d3",
    "Robbie Trevino",
);

// IKO 311 — Raugrin Triome (alternate printing)
const RAUGRIN_TRIOME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAUGRIN_TRIOME,
    1,
    "c303a627-cce3-4045-81f8-fe7427e0a941",
    "Robbie Trevino",
);

// IKO 312 — Savai Triome (alternate printing)
const SAVAI_TRIOME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SAVAI_TRIOME,
    1,
    "d21ef9e6-e2dd-4e0a-a36c-e07034ac4ba3",
    "Robbie Trevino",
);

// IKO 313 — Zagoth Triome (alternate printing)
const ZAGOTH_TRIOME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ZAGOTH_TRIOME,
    1,
    "f809f970-25a7-4c34-b98f-1cb08012080d",
    "Robbie Trevino",
);

// IKO 314 — Drannith Magistrate (alternate printing)
const DRANNITH_MAGISTRATE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DRANNITH_MAGISTRATE,
    1,
    "adaf8f6a-9559-4b47-8e67-2c4c3d6d8c60",
    "Kieran Yanner",
);

// IKO 315 — Lavabrink Venturer (alternate printing)
const LAVABRINK_VENTURER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LAVABRINK_VENTURER,
    1,
    "f2ab4e55-7c55-4480-83df-c9d9fd4df65a",
    "Zoltan Boros",
);

// IKO 316 — Luminous Broodmoth (alternate printing)
const LUMINOUS_BROODMOTH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LUMINOUS_BROODMOTH,
    1,
    "0535c823-f6e9-4a2f-8adf-f69b6f0fea1f",
    "Lie Setiawan",
);

// IKO 317 — Mythos of Snapdax (alternate printing)
const MYTHOS_OF_SNAPDAX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MYTHOS_OF_SNAPDAX,
    1,
    "0a12b361-ed36-4a89-a973-1e4c7fb56fa9",
    "Seb McKinnon",
);

// IKO 318 — Mythos of Illuna (alternate printing)
const MYTHOS_OF_ILLUNA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MYTHOS_OF_ILLUNA,
    1,
    "edf1cdf3-2543-4b91-b6a8-ac0d04a9c045",
    "Seb McKinnon",
);

// IKO 319 — Shark Typhoon (alternate printing)
const SHARK_TYPHOON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SHARK_TYPHOON,
    1,
    "5dfd8d35-3f73-4181-80a3-d4d69badcc47",
    "Caio Monteiro",
);

// IKO 320 — Voracious Greatshark (alternate printing)
const VORACIOUS_GREATSHARK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VORACIOUS_GREATSHARK,
    1,
    "5c5ee3a2-c683-4657-98c8-daebaa7819f1",
    "Mathias Kollros",
);

// IKO 321 — Extinction Event (alternate printing)
const EXTINCTION_EVENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EXTINCTION_EVENT,
    1,
    "3cb788cd-a6ab-44a9-adcf-13bdfc3be2e8",
    "Filip Burburan",
);

// IKO 322 — Hunted Nightmare (alternate printing)
const HUNTED_NIGHTMARE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HUNTED_NIGHTMARE,
    1,
    "e8ee8cfc-5d76-4e30-8b53-6ac6c9637d5d",
    "Antonio José Manzanedo",
);

// IKO 323 — Mythos of Nethroi (alternate printing)
const MYTHOS_OF_NETHROI_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MYTHOS_OF_NETHROI,
    1,
    "4cfb70ec-c9c8-4359-b2d4-e116bf5ad669",
    "Seb McKinnon",
);

// IKO 324 — Mythos of Vadrok (alternate printing)
const MYTHOS_OF_VADROK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MYTHOS_OF_VADROK,
    1,
    "ce59b5b6-97d8-4678-9381-2800667fdb41",
    "Seb McKinnon",
);

// IKO 325 — Unpredictable Cyclone (alternate printing)
const UNPREDICTABLE_CYCLONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNPREDICTABLE_CYCLONE,
    1,
    "84b94e1d-f328-4e4c-b112-45d6c6f98376",
    "Noah Bradley",
);

// IKO 326 — Yidaro, Wandering Monster (alternate printing)
const YIDARO_WANDERING_MONSTER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &YIDARO_WANDERING_MONSTER,
    2,
    "9679442a-ecd4-4220-aae4-01b95dc18486",
    "Jesper Ejsing",
);

// IKO 327 — Colossification (alternate printing)
const COLOSSIFICATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COLOSSIFICATION,
    1,
    "9111b3dc-63da-45c7-bbe6-42a443ac2015",
    "Johan Grenier",
);

// IKO 328 — Kogla, the Titan Ape (alternate printing)
const KOGLA_THE_TITAN_APE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KOGLA_THE_TITAN_APE,
    1,
    "a1a5905f-4d53-4f7e-8af8-3d802978ec03",
    "Chris Rahn",
);

// IKO 329 — Mythos of Brokkos (alternate printing)
const MYTHOS_OF_BROKKOS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MYTHOS_OF_BROKKOS,
    1,
    "d7e3e1ed-eeb6-4d7e-ab50-0b0f104606b9",
    "Seb McKinnon",
);

// IKO 330 — Chevill, Bane of Monsters (alternate printing)
const CHEVILL_BANE_OF_MONSTERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHEVILL_BANE_OF_MONSTERS,
    1,
    "1f90d92a-2297-4a52-80cb-10b56943b828",
    "Yongjae Choi",
);

// IKO 331 — Death's Oasis (alternate printing)
const DEATH_S_OASIS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEATH_S_OASIS,
    1,
    "7b6c4e70-81fb-42b9-9725-4d5206520037",
    "Grzegorz Rutkowski",
);

// IKO 332 — Eerie Ultimatum (alternate printing)
const EERIE_ULTIMATUM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EERIE_ULTIMATUM,
    1,
    "fdbe5a6f-82a2-4c6b-a69d-3a07917e15c9",
    "Jason A. Engle",
);

// IKO 333 — Emergent Ultimatum (alternate printing)
const EMERGENT_ULTIMATUM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EMERGENT_ULTIMATUM,
    1,
    "400d79b0-102a-406c-8369-8d024b18c4e9",
    "Zack Stella",
);

// IKO 334 — Frondland Felidar (alternate printing)
const FRONDLAND_FELIDAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FRONDLAND_FELIDAR,
    1,
    "bd9438c0-080b-4541-bfb7-6214be694584",
    "Steve Prescott",
);

// IKO 335 — General Kudro of Drannith (alternate printing)
const GENERAL_KUDRO_OF_DRANNITH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GENERAL_KUDRO_OF_DRANNITH,
    1,
    "e6eb6b97-8b87-4565-940a-01a7fd79a989",
    "Ryan Pancoast",
);

// IKO 336 — Genesis Ultimatum (alternate printing)
const GENESIS_ULTIMATUM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GENESIS_ULTIMATUM,
    1,
    "3d51e3a2-c188-4212-8cc0-ef2d4722cb84",
    "Jason Rainville",
);

// IKO 337 — Inspired Ultimatum (alternate printing)
const INSPIRED_ULTIMATUM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INSPIRED_ULTIMATUM,
    1,
    "c4e96967-d694-4c26-8e10-d904e1e64c09",
    "Tyler Jacobson",
);

// IKO 338 — Kinnan, Bonder Prodigy
// Audit: unsupported — Needs a mana-tap trigger exposing the actual types produced by the tapped nonland
// permanent, including multitype choices.
pub(in crate::card::sets) static KINNAN_BONDER_PRODIGY: CardRecord = CardRecord::new(
    "Kinnan, Bonder Prodigy",
    "532746e2-f822-4920-ab31-94e0c8baaa84",
    "Jason Rainville",
    crate::card::CardRules::unsupported(),
);

// IKO 339 — Labyrinth Raptor (alternate printing)
const LABYRINTH_RAPTOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LABYRINTH_RAPTOR,
    1,
    "e6b4a7b3-9eb3-4f96-b151-8b1c4c85118b",
    "Daarken",
);

// IKO 340 — Offspring's Revenge (alternate printing)
const OFFSPRING_S_REVENGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OFFSPRING_S_REVENGE,
    1,
    "dbd7462d-0dfb-4f3f-96c4-43c281f0beb5",
    "Daarken",
);

// IKO 341 — Quartzwood Crasher
// Audit: unsupported — Needs combat damage aggregated per damaged player across the matching trampling creatures
// for dynamic token power and toughness.
pub(in crate::card::sets) static QUARTZWOOD_CRASHER: CardRecord = CardRecord::new(
    "Quartzwood Crasher",
    "39e1effa-92a6-4e8c-9cd6-fc57ae7b3cbf",
    "Antonio José Manzanedo",
    crate::card::CardRules::unsupported(),
);

// IKO 342 — Rielle, the Everwise (alternate printing)
const RIELLE_THE_EVERWISE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RIELLE_THE_EVERWISE,
    1,
    "6cbe5ae7-5c50-476f-9022-cfb61ced107b",
    "Yongjae Choi",
);

// IKO 343 — Ruinous Ultimatum (alternate printing)
const RUINOUS_ULTIMATUM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RUINOUS_ULTIMATUM,
    1,
    "8aa531e0-a3e5-4b40-a824-354ed7a13fd5",
    "Chase Stone",
);

// IKO 344 — Skycat Sovereign (alternate printing)
const SKYCAT_SOVEREIGN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SKYCAT_SOVEREIGN,
    1,
    "32b08d66-4168-433d-9655-08f64b72e2cc",
    "Slawomir Maniak",
);

// IKO 345 — Slitherwisp (alternate printing)
const SLITHERWISP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SLITHERWISP,
    1,
    "d2a641ad-4268-46c4-a5fb-c1515e52d648",
    "Yigit Koroglu",
);

// IKO 346 — Song of Creation (alternate printing)
const SONG_OF_CREATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SONG_OF_CREATION,
    1,
    "7e98d2e6-8247-4915-928b-25d6e2efc908",
    "Noah Bradley",
);

// IKO 347 — Titans' Nest (alternate printing)
const TITANS_NEST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TITANS_NEST,
    1,
    "894502fb-1ec7-4520-90e5-5f97de1c9151",
    "Cliff Childs",
);

// IKO 348 — Whirlwind of Thought (alternate printing)
const WHIRLWIND_OF_THOUGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WHIRLWIND_OF_THOUGHT,
    1,
    "b6c48195-074a-4236-a82d-603b5dd1aa66",
    "Bram Sels",
);

// IKO 349 — Winota, Joiner of Forces (alternate printing)
const WINOTA_JOINER_OF_FORCES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WINOTA_JOINER_OF_FORCES,
    1,
    "750c7d63-b07d-456d-95ca-d3c3f346715e",
    "Magali Villeneuve",
);

// IKO 350 — Fiend Artisan (alternate printing)
const FIEND_ARTISAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FIEND_ARTISAN,
    1,
    "653e706a-5506-4efd-a86b-2a25bb66a2ae",
    "Yigit Koroglu",
);

// IKO 351 — Gyruda, Doom of Depths (alternate printing)
const GYRUDA_DOOM_OF_DEPTHS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GYRUDA_DOOM_OF_DEPTHS,
    1,
    "0d59bf12-f898-40ad-a8a9-2f31f733cf15",
    "Tyler Jacobson",
);

// IKO 352 — Jegantha, the Wellspring (alternate printing)
const JEGANTHA_THE_WELLSPRING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JEGANTHA_THE_WELLSPRING,
    1,
    "d5592378-f9e7-4c50-8425-2105744aa85c",
    "Chris Rahn",
);

// IKO 353 — Kaheera, the Orphanguard (alternate printing)
const KAHEERA_THE_ORPHANGUARD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KAHEERA_THE_ORPHANGUARD,
    1,
    "fe646123-c399-4ead-96a9-f7b46215f9d9",
    "Ryan Pancoast",
);

// IKO 354 — Keruga, the Macrosage (alternate printing)
const KERUGA_THE_MACROSAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KERUGA_THE_MACROSAGE,
    1,
    "1206f4fc-6ebf-4205-90d3-1cb3a9caaa82",
    "Dan Murayama Scott",
);

// IKO 355 — Lurrus of the Dream-Den (alternate printing)
const LURRUS_OF_THE_DREAM_DEN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LURRUS_OF_THE_DREAM_DEN,
    1,
    "2c89beb2-3467-4be0-a066-919f54331942",
    "Slawomir Maniak",
);

// IKO 356 — Lutri, the Spellchaser
pub(in crate::card::sets) static LUTRI_THE_SPELLCHASER: CardRecord = CardRecord::new(
    "Lutri, the Spellchaser",
    "12c01a00-2128-4b6c-874f-a206eca3a756",
    "Lie Setiawan",
    // Three mana at instant speed for a body and a copy of whatever you were
    // already casting -- and in a singleton cube the companion clause costs
    // the deck nothing it was not already paying.
    CardRules::new_creature(mana_cost!("{1}{U/R}{U/R}"), &["Elemental", "Otter"], 3, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            companion(
                "Companion — Each nonland card in your starting deck has a \
                 different name. (If this card is your chosen companion, you \
                 may put it into your hand from outside the game for {3} as \
                 a sorcery.)",
                DeckCards::nonlands().distinct_by(CardProperty::Name),
            ),
            abilities::flash(),
            AbilityDef::triggered_if_with_targets(
                "When Lutri enters, if you cast it, copy target instant or \
                 sorcery spell you control. You may choose new targets for \
                 the copy.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                // "If you cast it": a Lutri put onto the battlefield some other way is a
                // 3/2 and nothing else, which is what keeps the trigger honest about being
                // half of a spell rather than half of a creature.
                &TriggerConditionDef::SourceWasCast,
                // Yours rather than anybody's: Lutri copies what you are casting, not what
                // is being cast at you.
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Spell,
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Instant),
                                ObjectPredicateDef::HasType(CardType::Sorcery),
                            ]),
                        ]),
                        zones: &[ZoneKind::Stack],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::CopyStackObject(&crate::card::CopyStackObjectDef {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    controller: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(1),
                    retarget: true,
                    colors: None,
                }),
            ),
        ]),
);

// IKO 357 — Obosh, the Preypiercer (alternate printing)
const OBOSH_THE_PREYPIERCER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OBOSH_THE_PREYPIERCER,
    1,
    "9c8cf901-1452-4664-be10-0bacc5bef83a",
    "Daarken",
);

// IKO 358 — Umori, the Collector (alternate printing)
const UMORI_THE_COLLECTOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UMORI_THE_COLLECTOR,
    1,
    "10576f29-1ab8-4157-b5a0-8ad94a1ff634",
    "Jehan Choo",
);

// IKO 359 — Yorion, Sky Nomad (alternate printing)
const YORION_SKY_NOMAD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &YORION_SKY_NOMAD,
    1,
    "77b05368-1661-42de-9319-bbe987d27327",
    "Steven Belledin",
);

// IKO 360 — Zirda, the Dawnwaker (alternate printing)
const ZIRDA_THE_DAWNWAKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ZIRDA_THE_DAWNWAKER,
    1,
    "ff7301bd-05db-436b-8a49-5840c4cb9995",
    "Jesper Ejsing",
);

// IKO 361 — Crystalline Giant (alternate printing)
const CRYSTALLINE_GIANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CRYSTALLINE_GIANT,
    1,
    "8e5b4c21-342f-4b46-86bb-8e8f38659f82",
    "Jason Rainville",
);

// IKO 362 — The Ozolith (alternate printing)
const THE_OZOLITH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_OZOLITH,
    1,
    "ed3b5f52-ce33-4e67-aca0-6d417fe7f5fa",
    "Sam Burley",
);

// IKO 363 — Bonders' Enclave (alternate printing)
const BONDERS_ENCLAVE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BONDERS_ENCLAVE,
    1,
    "b7a26d0b-cf19-4c3e-bc81-6ffdc21757fa",
    "Cliff Childs",
);

// IKO 364 — Colossification (alternate printing)
const COLOSSIFICATION_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &COLOSSIFICATION,
    2,
    "6faa8a7b-d886-4005-950d-bad589b6e03a",
    "Mathias Kollros",
);

// IKO 365 — Flourishing Fox (alternate printing)
const FLOURISHING_FOX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FLOURISHING_FOX,
    1,
    "5d0b6981-0fd4-4f58-bba4-a7efd4c633d0",
    "Ilse Gort",
);

// IKO 366 — Heartless Act (alternate printing)
const HEARTLESS_ACT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HEARTLESS_ACT,
    1,
    "928b0234-fc62-4ab8-b257-3fddcc52376b",
    "Ryan Pancoast",
);

// IKO 367 — Forbidden Friendship (alternate printing)
const FORBIDDEN_FRIENDSHIP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FORBIDDEN_FRIENDSHIP,
    1,
    "ae39803e-e67a-4d1b-a3fe-2e255d502656",
    "Jakub Kasper",
);

// IKO 368 — Migration Path (alternate printing)
const MIGRATION_PATH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MIGRATION_PATH,
    1,
    "363428be-0f89-4de5-a804-93172da4d99f",
    "Grzegorz Rutkowski",
);

// IKO 369 — Sprite Dragon (alternate printing)
const SPRITE_DRAGON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPRITE_DRAGON,
    1,
    "7172db88-2208-47c4-8226-bc094fd04a9b",
    "Gabor Szikszai",
);

// IKO 370 — Huntmaster Liger (alternate printing)
const HUNTMASTER_LIGER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HUNTMASTER_LIGER,
    2,
    "c5f741e3-82d8-4842-8054-25e20a10dddd",
    "Slawomir Maniak",
);

// IKO 371 — Luminous Broodmoth (alternate printing)
const LUMINOUS_BROODMOTH_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LUMINOUS_BROODMOTH,
    2,
    "4d393cc4-0e6b-44fe-a74a-822c52678ec9",
    "Nick Southam",
);

// IKO 372 — Pollywog Symbiote (alternate printing)
const POLLYWOG_SYMBIOTE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &POLLYWOG_SYMBIOTE,
    1,
    "dd60a4f8-cb75-42e9-8a33-347e0f4bb458",
    "Simon Dominic",
);

// IKO 373 — Void Beckoner (alternate printing)
const VOID_BECKONER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VOID_BECKONER,
    1,
    "1ca7065e-88c1-44bb-ac68-e6e1df9e0726",
    "Zezhou Chen",
);

// IKO 374 — Everquill Phoenix (alternate printing)
const EVERQUILL_PHOENIX_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &EVERQUILL_PHOENIX,
    2,
    "96e93bf3-94e8-408c-94ba-e83796077548",
    "JOZ",
);

// IKO 375 — Yidaro, Wandering Monster
// Audit: unsupported — Needs per-player, per-card-name cycling history for the entire game and the fourth-cycle
// battlefield branch of its graveyard trigger.
pub(in crate::card::sets) static YIDARO_WANDERING_MONSTER: CardRecord = CardRecord::new(
    "Yidaro, Wandering Monster",
    "8bb6b4c7-4f18-4bea-b927-916c7bb987ee",
    "Yigit Koroglu",
    crate::card::CardRules::unsupported(),
);

// IKO 376 — Gemrazer (alternate printing)
const GEMRAZER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GEMRAZER,
    2,
    "c811c0d4-e2fc-45eb-8a76-b89c38a95536",
    "Sam Rowan",
);

// IKO 377 — Titanoth Rex (alternate printing)
const TITANOTH_REX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TITANOTH_REX,
    1,
    "b4817b86-d55a-4334-82ee-603f8c4b3e93",
    "Lius Lasahido",
);

// IKO 378 — Brokkos, Apex of Forever (alternate printing)
const BROKKOS_APEX_OF_FOREVER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BROKKOS_APEX_OF_FOREVER,
    2,
    "4cf6ca85-0ef5-4104-b0e6-1137c4975579",
    "Svetlin Velinov",
);

// IKO 379 — Illuna, Apex of Wishes (alternate printing)
const ILLUNA_APEX_OF_WISHES_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ILLUNA_APEX_OF_WISHES,
    2,
    "bfb1fba6-8eb3-4338-9249-cec888c892dc",
    "Nicholas Gregory",
);

// IKO 380 — Nethroi, Apex of Death (alternate printing)
const NETHROI_APEX_OF_DEATH_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &NETHROI_APEX_OF_DEATH,
    2,
    "b9ebe73b-29d7-4a07-b7dc-a885dad61a88",
    "Torstein Nordstrand",
);

// IKO 381 — Snapdax, Apex of the Hunt (alternate printing)
const SNAPDAX_APEX_OF_THE_HUNT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SNAPDAX_APEX_OF_THE_HUNT,
    2,
    "673d14b1-e93a-4594-b2c9-792696adb991",
    "YW Tang",
);

// IKO 382 — Sprite Dragon (alternate printing)
const SPRITE_DRAGON_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SPRITE_DRAGON,
    2,
    "94d3b231-d15d-4186-826c-452009cd5c5e",
    "Simon Dominic",
);

// IKO 383 — Vadrok, Apex of Thunder (alternate printing)
const VADROK_APEX_OF_THUNDER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &VADROK_APEX_OF_THUNDER,
    2,
    "d3346674-43cf-4006-b829-bf824560f413",
    "Johann Bodin",
);

// IKO 384 — Gyruda, Doom of Depths (alternate printing)
const GYRUDA_DOOM_OF_DEPTHS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GYRUDA_DOOM_OF_DEPTHS,
    2,
    "206739b3-9b6a-4855-a2c3-721879bdbfb7",
    "Jesper Ejsing",
);

// IKO 385 — Mysterious Egg (alternate printing)
const MYSTERIOUS_EGG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MYSTERIOUS_EGG,
    1,
    "f333b41d-e42c-4b12-be08-742eefb4e401",
    "Yoroikoji",
);

// IKO 386 — Dirge Bat (alternate printing)
const DIRGE_BAT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DIRGE_BAT,
    2,
    "fdb53dbd-b35f-414a-9f99-d340ef9398dc",
    "Kohei Hayama",
);

// IKO 387 — Crystalline Giant (alternate printing)
const CRYSTALLINE_GIANT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CRYSTALLINE_GIANT,
    2,
    "b0c2de2f-bb56-4713-861a-34523b331e26",
    "Kotakan",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ADAPTIVE_SHIMMERER,
    &FARFINDER,
    &MYSTERIOUS_EGG,
    &BLADE_BANISH,
    &CHECKPOINT_OFFICER,
    &COORDINATED_CHARGE,
    &CUBWARDEN,
    &DAYSQUAD_MARSHAL,
    &DRANNITH_HEALER,
    &DRANNITH_MAGISTRATE,
    &FIGHT_AS_ONE,
    &FLOURISHING_FOX,
    &GARRISON_CAT,
    &HELICA_GLIDER,
    &HUNTMASTER_LIGER,
    &IMPOSING_VANTASAUR,
    &KEENSIGHT_MENTOR,
    &LAVABRINK_VENTURER,
    &LIGHT_OF_HOPE,
    &LUMINOUS_BROODMOTH,
    &MAJESTIC_AURICORN,
    &MANED_SERVAL,
    &MYTHOS_OF_SNAPDAX,
    &PATAGIA_TIGER,
    &PERIMETER_SERGEANT,
    &SANCTUARY_LOCKDOWN,
    &SAVAI_SABERTOOTH,
    &SNARE_TACTICIAN,
    &SOLID_FOOTING,
    &SPLENDOR_MARE,
    &SPONTANEOUS_FLIGHT,
    &STORMWILD_CAPRIDOR,
    &SWALLOW_WHOLE,
    &VALIANT_RESCUER,
    &VULPIKEET,
    &WILL_OF_THE_ALL_HUNTER,
    &AEGIS_TURTLE,
    &ARCHIPELAGORE,
    &AVIAN_ODDITY,
    &BOON_OF_THE_WISH_GIVER,
    &CRYSTACEAN,
    &DREAMTAIL_HERON,
    &ESCAPE_PROTOCOL,
    &FACET_READER,
    &FROSTVEIL_AMBUSH,
    &GLIMMERBELL,
    &GUST_OF_WIND,
    &HAMPERING_SNARE,
    &KEEP_SAFE,
    &MYSTIC_SUBDUAL,
    &MYTHOS_OF_ILLUNA,
    &NEUTRALIZE,
    &OF_ONE_MIND,
    &OMINOUS_SEAS,
    &PHASE_DOLPHIN,
    &POLLYWOG_SYMBIOTE,
    &POUNCING_SHORESHARK,
    &RECONNAISSANCE_MISSION,
    &SEA_DASHER_OCTOPUS,
    &SHARK_TYPHOON,
    &STARTLING_DEVELOPMENT,
    &THIEVING_OTTER,
    &VORACIOUS_GREATSHARK,
    &WINGFOLD_PTERON,
    &WINGSPAN_MENTOR,
    &BASTION_OF_REMEMBRANCE,
    &BLITZ_LEECH,
    &BLOOD_CURDLE,
    &BOOT_NIPPER,
    &BUSHMEAT_POACHER,
    &CALL_OF_THE_DEATH_DWELLER,
    &CAVERN_WHISPERER,
    &CHITTERING_HARVESTER,
    &DIRGE_BAT,
    &DURABLE_COILBUG,
    &DUSKFANG_MENTOR,
    &EASY_PREY,
    &EXTINCTION_EVENT,
    &GLOOM_PANGOLIN,
    &GRIMDANCER,
    &HEARTLESS_ACT,
    &HUNTED_NIGHTMARE,
    &INSATIABLE_HEMOPHAGE,
    &LURKING_DEADEYE,
    &MEMORY_LEAK,
    &MUTUAL_DESTRUCTION,
    &MYTHOS_OF_NETHROI,
    &NIGHTSQUAD_COMMANDO,
    &SERRATED_SCORPION,
    &SUFFOCATING_FUMES,
    &UNBREAKABLE_BOND,
    &UNEXPECTED_FANGS,
    &VOID_BECKONER,
    &WHISPER_SQUAD,
    &ZAGOTH_MAMBA,
    &BLISTERSPIT_GREMLIN,
    &BLITZ_OF_THE_THUNDER_RAPTOR,
    &CLASH_OF_TITANS,
    &CLOUDPIERCER,
    &DRANNITH_STINGER,
    &EVERQUILL_PHOENIX,
    &FEROCIOUS_TIGORILLA,
    &FIRE_PROPHECY,
    &FLAME_SPILL,
    &FOOTFALL_CRATER,
    &FORBIDDEN_FRIENDSHIP,
    &FRILLSCARE_MENTOR,
    &GO_FOR_BLOOD,
    &HEIGHTENED_REFLEXES,
    &LAVA_SERPENT,
    &LUKKA_COPPERCOAT_OUTCAST,
    &MOMENTUM_RUMBLER,
    &MYTHOS_OF_VADROK,
    &PORCUPARROT,
    &PRICKLY_MARMOSET,
    &PYROCERATOPS,
    &RAKING_CLAWS,
    &REPTILIAN_REFLECTION,
    &ROOTING_MOLOCH,
    &RUMBLING_ROCKSLIDE,
    &SANCTUARY_SMASHER,
    &SHREDDED_SAILS,
    &SPELLEATER_WOLVERINE,
    &TENTATIVE_CONNECTION,
    &UNPREDICTABLE_CYCLONE,
    &WEAPONIZE_THE_MONSTERS,
    &ALMIGHTY_BRUSHWAGG,
    &AUSPICIOUS_STARRIX,
    &BARRIER_BREACH,
    &CHARGE_OF_THE_FOREVER_BEAST,
    &COLOSSIFICATION,
    &ESSENCE_SYMBIOTE,
    &EXCAVATION_MOLE,
    &EXUBERANT_WOLFBEAR,
    &FLYCATCHER_GIRAFFID,
    &FULLY_GROWN,
    &GEMRAZER,
    &GLOWSTONE_RECLUSE,
    &HONEY_MAMMOTH,
    &HORNBASH_MENTOR,
    &HUMBLE_NATURALIST,
    &KOGLA_THE_TITAN_APE,
    &MIGRATION_PATH,
    &MIGRATORY_GREATHORN,
    &MONSTROUS_STEP,
    &MOSSCOAT_GORIAK,
    &MYTHOS_OF_BROKKOS,
    &RAM_THROUGH,
    &SUDDEN_SPINNERETS,
    &SURVIVORS_BOND,
    &THWART_THE_ENEMY,
    &TITANOTH_REX,
    &VIVIEN_MONSTERS_ADVOCATE,
    &WILT,
    &BACK_FOR_MORE,
    &BONEYARD_LURKER,
    &BROKKOS_APEX_OF_FOREVER,
    &CHANNELED_FORCE,
    &CHEVILL_BANE_OF_MONSTERS,
    &DEATH_S_OASIS,
    &DIRE_TACTICS,
    &EERIE_ULTIMATUM,
    &EMERGENT_ULTIMATUM,
    &FRONDLAND_FELIDAR,
    &GENERAL_KUDRO_OF_DRANNITH,
    &GENERAL_S_ENFORCER,
    &GENESIS_ULTIMATUM,
    &ILLUNA_APEX_OF_WISHES,
    &INSPIRED_ULTIMATUM,
    &LABYRINTH_RAPTOR,
    &LORE_DRAKKIS,
    &NARSET_OF_THE_ANCIENT_WAY,
    &NECROPANTHER,
    &NETHROI_APEX_OF_DEATH,
    &OFFSPRING_S_REVENGE,
    &PARCELBEAST,
    &PRIMAL_EMPATHY,
    &REGAL_LEOSAUR,
    &RIELLE_THE_EVERWISE,
    &RUINOUS_ULTIMATUM,
    &SAVAI_THUNDERMANE,
    &SKULL_PROPHET,
    &SKYCAT_SOVEREIGN,
    &SLITHERWISP,
    &SNAPDAX_APEX_OF_THE_HUNT,
    &SONG_OF_CREATION,
    &SPRITE_DRAGON,
    &TITANS_NEST,
    &TRUMPETING_GNARR,
    &VADROK_APEX_OF_THUNDER,
    &WHIRLWIND_OF_THOUGHT,
    &WINOTA_JOINER_OF_FORCES,
    &ZENITH_FLARE,
    &ALERT_HEEDBONDER,
    &CUNNING_NIGHTBONDER,
    &FIEND_ARTISAN,
    &GYRUDA_DOOM_OF_DEPTHS,
    &JEGANTHA_THE_WELLSPRING,
    &JUBILANT_SKYBONDER,
    &KAHEERA_THE_ORPHANGUARD,
    &KERUGA_THE_MACROSAGE,
    &LURRUS_OF_THE_DREAM_DEN,
    &OBOSH_THE_PREYPIERCER,
    &PROUD_WILDBONDER,
    &SONOROUS_HOWLBONDER,
    &UMORI_THE_COLLECTOR,
    &YORION_SKY_NOMAD,
    &ZIRDA_THE_DAWNWAKER,
    &CRYSTALLINE_GIANT,
    &INDATHA_CRYSTAL,
    &KETRIA_CRYSTAL,
    &THE_OZOLITH,
    &RAUGRIN_CRYSTAL,
    &SAVAI_CRYSTAL,
    &SLEEPER_DART,
    &SPRINGJAW_TRAP,
    &ZAGOTH_CRYSTAL,
    &BONDERS_ENCLAVE,
    &INDATHA_TRIOME,
    &KETRIA_TRIOME,
    &RAUGRIN_TRIOME,
    &SAVAI_TRIOME,
    &ZAGOTH_TRIOME,
    &ZILORTHA_STRENGTH_INCARNATE,
    &KINNAN_BONDER_PRODIGY,
    &QUARTZWOOD_CRASHER,
    &LUTRI_THE_SPELLCHASER,
    &YIDARO_WANDERING_MONSTER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    DIVINE_ARROW_REPRINT,
    PACIFISM_REPRINT,
    ANTICIPATE_REPRINT,
    CAPTURE_SPHERE_REPRINT,
    CONVOLUTE_REPRINT,
    ESSENCE_SCATTER_REPRINT,
    FROST_LYNX_REPRINT,
    CORPSE_CHURN_REPRINT,
    DARK_BARGAIN_REPRINT,
    DEAD_WEIGHT_REPRINT,
    UNLIKELY_AID_REPRINT,
    BLAZING_VOLLEY_REPRINT,
    CATHARTIC_REUNION_REPRINT,
    FRENZIED_RAPTOR_REPRINT,
    YIDARO_WANDERING_MONSTER_ALTERNATE_1,
    ADVENTUROUS_IMPULSE_REPRINT,
    BRISTLING_BOAR_REPRINT,
    FERTILID_REPRINT,
    GREATER_SANDWURM_REPRINT,
    IVY_ELEMENTAL_REPRINT,
    LEAD_THE_STAMPEDE_REPRINT,
    PLUMMET_REPRINT,
    KINNAN_BONDER_PRODIGY_ALTERNATE_1,
    QUARTZWOOD_CRASHER_ALTERNATE_1,
    LUTRI_THE_SPELLCHASER_ALTERNATE_1,
    BLOODFELL_CAVES_REPRINT,
    BLOSSOMING_SANDS_REPRINT,
    DISMAL_BACKWATER_REPRINT,
    EVOLVING_WILDS_REPRINT,
    JUNGLE_HOLLOW_REPRINT,
    RUGGED_HIGHLANDS_REPRINT,
    SCOURED_BARRENS_REPRINT,
    SWIFTWATER_CLIFFS_REPRINT,
    THORNWOOD_FALLS_REPRINT,
    TRANQUIL_COVE_REPRINT,
    WIND_SCARRED_CRAG_REPRINT,
    PLAINS_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    ISLAND_REPRINT,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    SWAMP_REPRINT,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    MOUNTAIN_REPRINT,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    FOREST_REPRINT,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
    LUKKA_COPPERCOAT_OUTCAST_ALTERNATE_1,
    VIVIEN_MONSTERS_ADVOCATE_ALTERNATE_1,
    NARSET_OF_THE_ANCIENT_WAY_ALTERNATE_1,
    CUBWARDEN_ALTERNATE_1,
    HUNTMASTER_LIGER_ALTERNATE_1,
    MAJESTIC_AURICORN_ALTERNATE_1,
    VULPIKEET_ALTERNATE_1,
    ARCHIPELAGORE_ALTERNATE_1,
    DREAMTAIL_HERON_ALTERNATE_1,
    POUNCING_SHORESHARK_ALTERNATE_1,
    SEA_DASHER_OCTOPUS_ALTERNATE_1,
    CAVERN_WHISPERER_ALTERNATE_1,
    CHITTERING_HARVESTER_ALTERNATE_1,
    DIRGE_BAT_ALTERNATE_1,
    INSATIABLE_HEMOPHAGE_ALTERNATE_1,
    CLOUDPIERCER_ALTERNATE_1,
    EVERQUILL_PHOENIX_ALTERNATE_1,
    PORCUPARROT_ALTERNATE_1,
    AUSPICIOUS_STARRIX_ALTERNATE_1,
    GEMRAZER_ALTERNATE_1,
    GLOWSTONE_RECLUSE_ALTERNATE_1,
    MIGRATORY_GREATHORN_ALTERNATE_1,
    BONEYARD_LURKER_ALTERNATE_1,
    BROKKOS_APEX_OF_FOREVER_ALTERNATE_1,
    ILLUNA_APEX_OF_WISHES_ALTERNATE_1,
    LORE_DRAKKIS_ALTERNATE_1,
    NECROPANTHER_ALTERNATE_1,
    NETHROI_APEX_OF_DEATH_ALTERNATE_1,
    PARCELBEAST_ALTERNATE_1,
    REGAL_LEOSAUR_ALTERNATE_1,
    SNAPDAX_APEX_OF_THE_HUNT_ALTERNATE_1,
    TRUMPETING_GNARR_ALTERNATE_1,
    VADROK_APEX_OF_THUNDER_ALTERNATE_1,
    INDATHA_TRIOME_ALTERNATE_1,
    KETRIA_TRIOME_ALTERNATE_1,
    RAUGRIN_TRIOME_ALTERNATE_1,
    SAVAI_TRIOME_ALTERNATE_1,
    ZAGOTH_TRIOME_ALTERNATE_1,
    DRANNITH_MAGISTRATE_ALTERNATE_1,
    LAVABRINK_VENTURER_ALTERNATE_1,
    LUMINOUS_BROODMOTH_ALTERNATE_1,
    MYTHOS_OF_SNAPDAX_ALTERNATE_1,
    MYTHOS_OF_ILLUNA_ALTERNATE_1,
    SHARK_TYPHOON_ALTERNATE_1,
    VORACIOUS_GREATSHARK_ALTERNATE_1,
    EXTINCTION_EVENT_ALTERNATE_1,
    HUNTED_NIGHTMARE_ALTERNATE_1,
    MYTHOS_OF_NETHROI_ALTERNATE_1,
    MYTHOS_OF_VADROK_ALTERNATE_1,
    UNPREDICTABLE_CYCLONE_ALTERNATE_1,
    YIDARO_WANDERING_MONSTER_ALTERNATE_2,
    COLOSSIFICATION_ALTERNATE_1,
    KOGLA_THE_TITAN_APE_ALTERNATE_1,
    MYTHOS_OF_BROKKOS_ALTERNATE_1,
    CHEVILL_BANE_OF_MONSTERS_ALTERNATE_1,
    DEATH_S_OASIS_ALTERNATE_1,
    EERIE_ULTIMATUM_ALTERNATE_1,
    EMERGENT_ULTIMATUM_ALTERNATE_1,
    FRONDLAND_FELIDAR_ALTERNATE_1,
    GENERAL_KUDRO_OF_DRANNITH_ALTERNATE_1,
    GENESIS_ULTIMATUM_ALTERNATE_1,
    INSPIRED_ULTIMATUM_ALTERNATE_1,
    LABYRINTH_RAPTOR_ALTERNATE_1,
    OFFSPRING_S_REVENGE_ALTERNATE_1,
    RIELLE_THE_EVERWISE_ALTERNATE_1,
    RUINOUS_ULTIMATUM_ALTERNATE_1,
    SKYCAT_SOVEREIGN_ALTERNATE_1,
    SLITHERWISP_ALTERNATE_1,
    SONG_OF_CREATION_ALTERNATE_1,
    TITANS_NEST_ALTERNATE_1,
    WHIRLWIND_OF_THOUGHT_ALTERNATE_1,
    WINOTA_JOINER_OF_FORCES_ALTERNATE_1,
    FIEND_ARTISAN_ALTERNATE_1,
    GYRUDA_DOOM_OF_DEPTHS_ALTERNATE_1,
    JEGANTHA_THE_WELLSPRING_ALTERNATE_1,
    KAHEERA_THE_ORPHANGUARD_ALTERNATE_1,
    KERUGA_THE_MACROSAGE_ALTERNATE_1,
    LURRUS_OF_THE_DREAM_DEN_ALTERNATE_1,
    OBOSH_THE_PREYPIERCER_ALTERNATE_1,
    UMORI_THE_COLLECTOR_ALTERNATE_1,
    YORION_SKY_NOMAD_ALTERNATE_1,
    ZIRDA_THE_DAWNWAKER_ALTERNATE_1,
    CRYSTALLINE_GIANT_ALTERNATE_1,
    THE_OZOLITH_ALTERNATE_1,
    BONDERS_ENCLAVE_ALTERNATE_1,
    COLOSSIFICATION_ALTERNATE_2,
    FLOURISHING_FOX_ALTERNATE_1,
    HEARTLESS_ACT_ALTERNATE_1,
    FORBIDDEN_FRIENDSHIP_ALTERNATE_1,
    MIGRATION_PATH_ALTERNATE_1,
    SPRITE_DRAGON_ALTERNATE_1,
    HUNTMASTER_LIGER_ALTERNATE_2,
    LUMINOUS_BROODMOTH_ALTERNATE_2,
    POLLYWOG_SYMBIOTE_ALTERNATE_1,
    VOID_BECKONER_ALTERNATE_1,
    EVERQUILL_PHOENIX_ALTERNATE_2,
    GEMRAZER_ALTERNATE_2,
    TITANOTH_REX_ALTERNATE_1,
    BROKKOS_APEX_OF_FOREVER_ALTERNATE_2,
    ILLUNA_APEX_OF_WISHES_ALTERNATE_2,
    NETHROI_APEX_OF_DEATH_ALTERNATE_2,
    SNAPDAX_APEX_OF_THE_HUNT_ALTERNATE_2,
    SPRITE_DRAGON_ALTERNATE_2,
    VADROK_APEX_OF_THUNDER_ALTERNATE_2,
    GYRUDA_DOOM_OF_DEPTHS_ALTERNATE_2,
    MYSTERIOUS_EGG_ALTERNATE_1,
    DIRGE_BAT_ALTERNATE_2,
    CRYSTALLINE_GIANT_ALTERNATE_2,
];
