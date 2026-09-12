//! Wilds of Eldraine card inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::AdditionalCostIndex;
use crate::CardPartId;
use crate::KeywordAbility;
use crate::PlayOptionId;
use crate::card::AbilityDef;
use crate::card::AbilityPredicateDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AdditionalCostValueDef;
use crate::card::AdditionalTriggerDef;
use crate::card::AlternateSpellKind;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BattlefieldArrivalDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BindObjectsDef;
use crate::card::BlockRestrictionDef;
use crate::card::CardArt;
use crate::card::CardComposition;
use crate::card::CardEffectStatus;
use crate::card::CardNameDef;
use crate::card::CardNameSetDef;
use crate::card::CardPart;
use crate::card::CardRules;
use crate::card::CardStructure;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseCardsFromCollectionDef;
use crate::card::ChooseDef;
use crate::card::ChooseExactDef;
use crate::card::ChooseForEachPlayerDef;
use crate::card::ClassifyObjectsDef;
use crate::card::CollectionInspectionDef;
use crate::card::ColorSet;
use crate::card::ComparisonDef;
use crate::card::ConditionValueDef;
use crate::card::ControlDurationDef;
use crate::card::CopyAbilityDef;
use crate::card::CopyExceptionsDef;
use crate::card::CopyStackObjectDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatedTokensDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DamageDef;
use crate::card::DamageEventMatcherDef;
use crate::card::DamageFollowUpDef;
use crate::card::DamageKindDef;
use crate::card::DamagePreventionDef;
use crate::card::DamageRecipientMatcherDef;
use crate::card::DamageSourceMatcherDef;
use crate::card::DiscardSelectionDef;
use crate::card::DividedTotal;
use crate::card::EffectChoiceDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::GameActionDef;
use crate::card::InstalledTriggerDef;
use crate::card::ManaColor;
use crate::card::MoveObjectsDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetCountConditionDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::ObjectSetPredicateDef;
use crate::card::OptionalAdditionalCostAbilityDef;
use crate::card::OptionalAdditionalCostKindDef;
use crate::card::PayOrDef;
use crate::card::PerPlayerSelectionDef;
use crate::card::PlayOptionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealObjectsDef;
use crate::card::ScaledValueDef;
use crate::card::SpellCastQueryDef;
use crate::card::SpellForm;
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
use crate::ids::TargetIndex;
use crate::mana_cost;

use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1998::portal_second_age as catalog_p02;
use crate::card::sets::y2004::champions_of_kamigawa as catalog_chk;
use crate::card::sets::y2010::rise_of_the_eldrazi as catalog_roe;
use crate::card::sets::y2011::magic_2012 as catalog_m12;
use crate::card::sets::y2014::khans_of_tarkir as catalog_ktk;
use crate::card::sets::y2019::throne_of_eldraine as catalog_eld;
use crate::card::sets::y2020::theros_beyond_death as catalog_thb;
use crate::card::sets::y2022::dominaria_united as catalog_dmu;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(
    &const {
        crate::card::CardSetMetadata {
            code: "WOE",
            slug: "wilds-of-eldraine",
        }
    },
);

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const DEFENSELESS_RAT_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Rat"], &[ManaColor::Black], 1, 1)
        .with_abilities(&[AbilityDef::static_ability(
            "This token can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                    BlockRestrictionDef::CANNOT_BLOCK,
                )),
            },
        )])
        .with_art(CardArt::new(
            "1e0205f2-25c1-403b-b408-56e3f2d63b4d",
            "Kim Sokol",
        ));

const fn bargain() -> AbilityDef {
    AbilityDef::optional_additional_cost(
        "Bargain (You may sacrifice an artifact, enchantment, or token \
as you cast this spell.)",
        OptionalAdditionalCostAbilityDef {
            kind: OptionalAdditionalCostKindDef::Bargain,
            label: OptionalAdditionalCostKindDef::Bargain.label(),
            resolution_destination: SpellResolutionDestinationDef::Graveyard,

            costs: &const {
                [CostDef::Sacrifice {
                    object: ObjectPredicateDef::AnyOf(
                        &const {
                            [
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                ObjectPredicateDef::Token,
                            ]
                        },
                    ),
                    quantity: CostQuantityDef::Fixed(1),
                }]
            },
        },
    )
}
static MONSTER_ROLE: TokenCharacteristics =
    TokenCharacteristics::enchantment(&const { ["Aura", "Role"] }, &const { [] })
        .with_name("Monster Role")
        .enchanting(&const { ObjectPredicateDef::HasType(CardType::Creature) })
        .with_abilities(
            &const {
                [AbilityDef::static_ability(
                    "Enchanted creature gets +1/+1 and has trample.",
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::Composite(
                            &const {
                                [
                                    AppliedEffectDef::modify_power_toughness(
                                        ValueDef::Constant(1),
                                        ValueDef::Constant(1),
                                    ),
                                    AppliedEffectDef::add_ability(&const { abilities::trample() }),
                                ]
                            },
                        ),
                    },
                )]
            },
        );
static ROYAL_ROLE: TokenCharacteristics =
    TokenCharacteristics::enchantment(&const { ["Aura", "Role"] }, &const { [] })
        .with_name("Royal Role")
        .enchanting(&const { ObjectPredicateDef::HasType(CardType::Creature) })
        .with_abilities(
            &const {
                [AbilityDef::static_ability(
                    "Enchanted creature gets +1/+1 and has ward {1}.",
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::Composite(
                            &const {
                                [
                                    AppliedEffectDef::modify_power_toughness(
                                        ValueDef::Constant(1),
                                        ValueDef::Constant(1),
                                    ),
                                    AppliedEffectDef::add_ability(
                                        &const {
                                            abilities::ward(
                                                &const { [CostDef::Mana(mana_cost!("{1}"))] },
                                                "Ward {1}",
                                            )
                                        },
                                    ),
                                ]
                            },
                        ),
                    },
                )]
            },
        );
static CURSED_ROLE: TokenCharacteristics =
    TokenCharacteristics::enchantment(&const { ["Aura", "Role"] }, &const { [] })
        .with_name("Cursed Role")
        .enchanting(&const { ObjectPredicateDef::HasType(CardType::Creature) })
        .with_abilities(
            &const {
                [AbilityDef::static_ability(
                    "Enchanted creature has base power and toughness 1/1.",
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                    },
                )]
            },
        );
static SORCERER_ROLE: TokenCharacteristics =
    TokenCharacteristics::enchantment(&const { ["Aura", "Role"] }, &const { [] })
        .with_name("Sorcerer Role")
        .enchanting(&const { ObjectPredicateDef::HasType(CardType::Creature) })
        .with_abilities(
            &const {
                [AbilityDef::static_ability(
                    "Enchanted creature gets +1/+1 and has \"Whenever this \
creature attacks, scry 1.\"",
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::Composite(
                            &const {
                                [
                                    AppliedEffectDef::modify_power_toughness(
                                        ValueDef::Constant(1),
                                        ValueDef::Constant(1),
                                    ),
                                    AppliedEffectDef::add_ability(
                                        &const {
                                            AbilityDef::triggered(
                                                "Whenever this creature attacks, scry 1.",
                                                TriggerEventDef::attacks(
                                                    ObjectPredicateDef::Source,
                                                ),
                                                abilities::scry(ValueDef::Constant(1)),
                                            )
                                        },
                                    ),
                                ]
                            },
                        ),
                    },
                )]
            },
        );
static WICKED_ROLE: TokenCharacteristics =
    TokenCharacteristics::enchantment(&const { ["Aura", "Role"] }, &const { [] })
        .with_name("Wicked Role")
        .enchanting(&const { ObjectPredicateDef::HasType(CardType::Creature) })
        .with_abilities(
            &const {
                [
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
                    abilities::dies_trigger(
                        "When this token is put into a graveyard, each opponent loses \
1 life.",
                        EffectDef::LoseLife {
                            recipient: EffectRecipientDef::Opponent,
                            amount: ValueDef::Constant(1),
                        },
                    ),
                ]
            },
        );
static YOUNG_HERO_ROLE: TokenCharacteristics =
    TokenCharacteristics::enchantment(&const { ["Aura", "Role"] }, &const { [] })
        .with_name("Young Hero Role")
        .enchanting(&const { ObjectPredicateDef::HasType(CardType::Creature) })
        .with_abilities(
            &const {
                [AbilityDef::static_ability(
                    "Enchanted creature has \"Whenever this creature attacks, if \
its toughness is 3 or less, put a +1/+1 counter on it.\"",
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::add_ability(
                            &const {
                                AbilityDef::triggered_if(
                                    "Whenever this creature attacks, if its toughness is 3 or \
less, put a +1/+1 counter on it.",
                                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                                    &const {
                                        TriggerConditionDef::SourceMatches {
                                            object: ObjectPredicateDef::ToughnessLessThan(
                                                ValueDef::Constant(4),
                                            ),
                                        }
                                    },
                                    EffectDef::AddCounters {
                                        object: EffectRecipientDef::Source,
                                        kind: CounterKind::PlusOnePlusOne,
                                        amount: ValueDef::Constant(1),
                                    },
                                )
                            },
                        ),
                    },
                )]
            },
        );
fn adventure(record: &CardRecord, name: &'static str, alternate: &CardRules) -> CardComposition {
    let primary_name = record
        .name
        .split(" // ")
        .next()
        .expect("Adventure has a primary name");
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, primary_name, record.rules),
            CardPart::new(CardPartId(1), name, *alternate),
        ],
        structure: CardStructure::AlternateSpell {
            main: CardPartId::PRIMARY,
            alternate: CardPartId(1),
            kind: AlternateSpellKind::Adventure,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                primary_name,
                SpellForm::Part(CardPartId::PRIMARY),
                record.rules.mana_cost().expect("printed primary cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                name,
                SpellForm::Part(CardPartId(1)),
                alternate.mana_cost().expect("printed Adventure cost"),
                CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

const FOOD_TOKEN: TokenCharacteristics = crate::card::tokens::food().with_art(CardArt::new(
    "ab4f83dd-94c0-4594-8e72-611eb76fa4e2",
    "Ovidio Cartagena",
));
const TREASURE_TOKEN: TokenCharacteristics = crate::card::tokens::treasure().with_art(
    CardArt::new("73cbf189-a79b-4dca-8b29-33aef6306b5a", "Zezhou Chen"),
);

const HUMAN_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Human"], &[ManaColor::White], 1, 1).with_art(CardArt::new(
        "50240867-3a3e-4a8d-9569-816ab4a3671a",
        "Julia Griffin",
    ));

// WOE 1 — Archon of the Wild Rose
// Audit: unsupported — Needs a host-side predicate for being enchanted by an Aura controlled by a particular player; Enchanted ignores the Aura controller and AttachedTo selects the Aura rather than its host.
pub(in crate::card::sets) static ARCHON_OF_THE_WILD_ROSE: CardRecord = CardRecord::new(
    "Archon of the Wild Rose",
    "00174be7-0dc8-43b9-81b6-f25a8c3fb4eb",
    "Chris Rahn",
    CardRules::unsupported(),
);

// WOE 2 — Archon's Glory
pub(in crate::card::sets) static ARCHON_S_GLORY: CardRecord = CardRecord::new(
    "Archon's Glory",
    "e71768e7-4ef7-4fb2-838b-eb3a7f662d38",
    "Anastasia Ovchinnikova",
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[
        bargain(),
        AbilityDef::spell_with_targets(
            "Target creature gets +2/+2 until end of turn. If this spell \
was bargained, that creature also gains flying and lifelink \
until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
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
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourcePaidAdditionalCost(
                        AdditionalCostIndex::PRIMARY,
                    ),
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::add_ability(&abilities::flying()),
                            AppliedEffectDef::add_ability(&abilities::lifelink()),
                        ]),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                },
            ]),
        ),
    ]),
);

// WOE 3 — Armory Mice
// Audit: unsupported — Needs per-player nonland-permanent entry history for the current turn, including objects that have since left; current battlefield entry timestamps cannot implement celebration.
pub(in crate::card::sets) static ARMORY_MICE: CardRecord = CardRecord::new(
    "Armory Mice",
    "4b041949-6fb6-40a6-9329-f209be537219",
    "Chris Seaman",
    CardRules::unsupported(),
);

// WOE 4 — Besotted Knight // Betroth the Beast
pub(in crate::card::sets) static BESOTTED_KNIGHT: CardRecord = CardRecord::new(
    "Besotted Knight // Betroth the Beast",
    "5980a930-c7f8-45e1-a18a-87734d9ed09e",
    "Andreia Ugrai",
    CardRules::new_creature(mana_cost!("{3}{W}"), &const { ["Human", "Knight"] }, 3, 3),
)
.with_composition(|| {
    adventure(
        &BESOTTED_KNIGHT,
        "Betroth the Beast",
        &CardRules::new_sorcery(mana_cost!("{W}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Create a Royal Role token attached to target creature you \
control. (Enchanted creature gets +1/+1 and has ward {1}.)",
                    &const {
                        [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::HasType(CardType::Creature),
                                zones: &const { [ZoneKind::Battlefield] },
                                controller: Some(PlayerRelation::You),
                                owner: None,
                            },
                        )]
                    },
                    EffectDef::CreateAttachedToken {
                        token: ROYAL_ROLE,
                        host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                    },
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 5 — Break the Spell
// Audit: unsupported — Needs destruction outcomes that include a successful destruction whose graveyard move is replaced by exile; DestroyFollowUpDef currently records only objects moved to the graveyard.
pub(in crate::card::sets) static BREAK_THE_SPELL: CardRecord = CardRecord::new(
    "Break the Spell",
    "f7094fc0-1d26-429c-9b49-37718c4a5c80",
    "Miranda Meeks",
    CardRules::unsupported(),
);

// WOE 6 — Charmed Clothier
pub(in crate::card::sets) static CHARMED_CLOTHIER: CardRecord = CardRecord::new(
    "Charmed Clothier",
    "994f4473-dfc9-45cd-8528-945db3aa6a9a",
    "Winona Nelson",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Faerie", "Advisor"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, create a Royal Role token attached \
to another target creature you control. (If you control \
another Role on it, put that one into the graveyard. \
Enchanted creature gets +1/+1 and has ward {1}.)",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::CreateAttachedToken {
                token: ROYAL_ROLE,
                host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
            },
        ),
    ]),
);

// WOE 7 — Cheeky House-Mouse // Squeak By
pub(in crate::card::sets) static CHEEKY_HOUSE_MOUSE: CardRecord = CardRecord::new(
    "Cheeky House-Mouse // Squeak By",
    "1f3013bf-9647-4bdb-a638-d299ae00f88e",
    "Uriah Voth",
    CardRules::new_creature(mana_cost!("{W}"), &const { ["Mouse"] }, 2, 1),
)
.with_composition(|| {
    adventure(
        &CHEEKY_HOUSE_MOUSE,
        "Squeak By",
        &CardRules::new_sorcery(mana_cost!("{W}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Target creature you control gets +1/+1 until end of turn. It \
can't be blocked by creatures with power 3 or greater this \
turn.",
                    &const {
                        [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::HasType(CardType::Creature),
                                zones: &const { [ZoneKind::Battlefield] },
                                controller: Some(PlayerRelation::You),
                                owner: None,
                            },
                        )]
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::Composite(
                            &const {
                                [
                                    AppliedEffectDef::modify_power_toughness(
                                        ValueDef::Constant(1),
                                        ValueDef::Constant(1),
                                    ),
                                    AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                                        ObjectPredicateDef::PowerAtLeast(3),
                                    )),
                                ]
                            },
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 8 — Cooped Up
pub(in crate::card::sets) static COOPED_UP: CardRecord = CardRecord::new(
    "Cooped Up",
    "8acb8758-09c5-4e19-ada1-904e36ece1fc",
    "Jodie Muir",
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature can't attack or block.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    ]),
                },
            ),
            AbilityDef::activated(
                "{2}{W}: Exile enchanted creature.",
                &[CostDef::Mana(mana_cost!("{2}{W}"))],
                EffectDef::move_to_zone(
                    EffectRecipientDef::AttachedPermanent,
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            ),
        ]),
);

// WOE 9 — Cursed Courtier
pub(in crate::card::sets) static CURSED_COURTIER: CardRecord = CardRecord::new(
    "Cursed Courtier",
    "3d2d5a71-d6e1-4c96-9a53-0e370047a56e",
    "Tuan Duong Chu",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Noble"], 3, 3).with_abilities(&[
        abilities::lifelink(),
        abilities::enters_trigger(
            "When this creature enters, create a Cursed Role token \
attached to it. (Enchanted creature is 1/1.)",
            EffectDef::CreateAttachedToken {
                token: CURSED_ROLE,
                host: Some(EffectRecipientDef::Source),
            },
        ),
    ]),
);

// WOE 10 — Discerning Financier
pub(in crate::card::sets) static DISCERNING_FINANCIER: CardRecord = CardRecord::new(
    "Discerning Financier",
    "584774b5-640f-45e0-810b-f5faf119645b",
    "Wayne Reynolds",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Noble"], 2, 3).with_abilities(&[
        AbilityDef::triggered_if(
            "At the beginning of your upkeep, if an opponent controls more \
lands than you, create a Treasure token. (It's an artifact \
with \"{T}, Sacrifice this token: Add one mana of any \
color.\")",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                )),
                comparison: ComparisonDef::Greater,
                right: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            }),
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
        AbilityDef::activated_with_targets(
            "{2}{W}: Choose another player. That player gains control of \
target Treasure you control. You draw a card.",
            &[CostDef::Mana(mana_cost!("{2}{W}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Treasure")),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Perform(GameActionDef::GainControl {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    controller: PlayerRefDef::Opponent,
                    duration: ControlDurationDef::Indefinitely,
                }),
                abilities::draw_cards(ValueDef::Constant(1)),
            ]),
        ),
    ]),
);

// WOE 11 — Dutiful Griffin
// Audit: unsupported — Needs a graveyard activation to pay a sacrifice-of-enchantments cost; the shared activation payment boundary only supports this cost from the battlefield.
pub(in crate::card::sets) static DUTIFUL_GRIFFIN: CardRecord = CardRecord::new(
    "Dutiful Griffin",
    "b8e40377-990f-41ea-8dd2-62a998dcd128",
    "Ilse Gort",
    CardRules::unsupported(),
);

// WOE 12 — Eerie Interference
pub(in crate::card::sets) static EERIE_INTERFERENCE: CardRecord = CardRecord::new(
    "Eerie Interference",
    "42a74545-75c8-4a6b-bee1-ac5665d9bcf0",
    "Néstor Ossandón Leal",
    CardRules::new_instant(mana_cost!("{2}{W}")).with_abilities(&[AbilityDef::spell(
        "Prevent all damage that would be dealt to you and creatures \
you control this turn by creatures.",
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef {
                kind: DamageKindDef::Any,
                source: DamageSourceMatcherDef::Matching(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
                recipient: DamageRecipientMatcherDef::PlayerAndCreaturesControlledBy(
                    PlayerRefDef::EffectController,
                ),
            }),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// WOE 13 — Expel the Interlopers
pub(in crate::card::sets) static EXPEL_THE_INTERLOPERS: CardRecord = CardRecord::new(
    "Expel the Interlopers",
    "1094eef0-6c57-4bfa-a584-f708b87354fb",
    "Andreas Zafiratos",
    CardRules::new_sorcery(mana_cost!("{3}{W}{W}")).with_abilities(&[AbilityDef::spell(
        "Choose a number between 0 and 10. Destroy all creatures with \
power greater than or equal to the chosen number.",
        EffectDef::ChooseEffect {
            player: EffectRecipientDef::Controller,
            choices: &[
                EffectChoiceDef {
                    label: "0",
                    effect: EffectDef::Destroy {
                        object: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::PowerAtLeast(0),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::Any,
                            ),
                        )),
                        then: None,
                    },
                },
                EffectChoiceDef {
                    label: "1",
                    effect: EffectDef::Destroy {
                        object: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::PowerAtLeast(1),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::Any,
                            ),
                        )),
                        then: None,
                    },
                },
                EffectChoiceDef {
                    label: "2",
                    effect: EffectDef::Destroy {
                        object: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::PowerAtLeast(2),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::Any,
                            ),
                        )),
                        then: None,
                    },
                },
                EffectChoiceDef {
                    label: "3",
                    effect: EffectDef::Destroy {
                        object: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::PowerAtLeast(3),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::Any,
                            ),
                        )),
                        then: None,
                    },
                },
                EffectChoiceDef {
                    label: "4",
                    effect: EffectDef::Destroy {
                        object: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::PowerAtLeast(4),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::Any,
                            ),
                        )),
                        then: None,
                    },
                },
                EffectChoiceDef {
                    label: "5",
                    effect: EffectDef::Destroy {
                        object: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::PowerAtLeast(5),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::Any,
                            ),
                        )),
                        then: None,
                    },
                },
                EffectChoiceDef {
                    label: "6",
                    effect: EffectDef::Destroy {
                        object: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::PowerAtLeast(6),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::Any,
                            ),
                        )),
                        then: None,
                    },
                },
                EffectChoiceDef {
                    label: "7",
                    effect: EffectDef::Destroy {
                        object: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::PowerAtLeast(7),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::Any,
                            ),
                        )),
                        then: None,
                    },
                },
                EffectChoiceDef {
                    label: "8",
                    effect: EffectDef::Destroy {
                        object: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::PowerAtLeast(8),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::Any,
                            ),
                        )),
                        then: None,
                    },
                },
                EffectChoiceDef {
                    label: "9",
                    effect: EffectDef::Destroy {
                        object: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::PowerAtLeast(9),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::Any,
                            ),
                        )),
                        then: None,
                    },
                },
                EffectChoiceDef {
                    label: "10",
                    effect: EffectDef::Destroy {
                        object: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::PowerAtLeast(10),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::Any,
                            ),
                        )),
                        then: None,
                    },
                },
            ],
        },
    )]),
);

// WOE 14 — Frostbridge Guard
pub(in crate::card::sets) static FROSTBRIDGE_GUARD: CardRecord = CardRecord::new(
    "Frostbridge Guard",
    "b9dec431-a20e-4c1c-9855-904779756509",
    "Paul Scott Canavan",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Elemental", "Soldier"], 2, 2).with_abilities(
        &[AbilityDef::activated_with_targets(
            "{2}{W}, {T}: Tap target creature.",
            &[CostDef::Mana(mana_cost!("{2}{W}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        )],
    ),
);

// WOE 15 — Gallant Pie-Wielder
// Audit: unsupported — Needs per-player nonland-permanent entry history for the current turn, including objects that have since left; current battlefield entry timestamps cannot implement celebration.
pub(in crate::card::sets) static GALLANT_PIE_WIELDER: CardRecord = CardRecord::new(
    "Gallant Pie-Wielder",
    "e053d330-d0a2-4468-afba-42bf165b8fbf",
    "Matt Forsyth",
    CardRules::unsupported(),
);

// WOE 16 — Glass Casket (reprint)
const GLASS_CASKET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::GLASS_CASKET,
    "02c5c395-ee8b-47fd-ac52-256354c19cdf",
    "Raoul Vitale",
);

// WOE 17 — Hopeful Vigil
pub(in crate::card::sets) static HOPEFUL_VIGIL: CardRecord = CardRecord::new(
    "Hopeful Vigil",
    "d382fd32-b1f5-4ec7-9d42-fc2915ed3bc9",
    "Jake Murray",
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_abilities(&[
        abilities::enters_trigger(
            "When this enchantment enters, create a 2/2 white Knight \
creature token with vigilance.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Knight"], &[ManaColor::White], 2, 2)
                    .with_abilities(&[abilities::vigilance()]),
            ))),
        ),
        abilities::dies_trigger(
            "When this enchantment is put into a graveyard from the \
battlefield, scry 2.",
            abilities::scry(ValueDef::Constant(2)),
        ),
        AbilityDef::activated(
            "{2}{W}: Sacrifice this enchantment.",
            &[CostDef::Mana(mana_cost!("{2}{W}"))],
            EffectDef::sacrifice(EffectRecipientDef::Source),
        ),
    ]),
);

// WOE 18 — Kellan's Lightblades
pub(in crate::card::sets) static KELLAN_S_LIGHTBLADES: CardRecord = CardRecord::new(
    "Kellan's Lightblades",
    "0cc727a6-f875-49b1-b7a4-67f22fbc3d50",
    "Fajareka Setiawan",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[
        bargain(),
        AbilityDef::spell_with_targets(
            "Kellan's Lightblades deals 3 damage to target attacking or \
blocking creature. If this spell was bargained, destroy that \
creature instead.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Attacking,
                        ObjectPredicateDef::Blocking,
                    ]),
                ]),
            )],
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourcePaidAdditionalCost(
                    AdditionalCostIndex::PRIMARY,
                ),
                then: &EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
                otherwise: &EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(3),
                ),
            },
        ),
    ]),
);

// WOE 19 — Knight of Doves
pub(in crate::card::sets) static KNIGHT_OF_DOVES: CardRecord = CardRecord::new(
    "Knight of Doves",
    "b4f33617-ad19-4d42-ab94-6f21a7fb3dd4",
    "Volkan Baǵa",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Knight"], 1, 3).with_abilities(&[
        AbilityDef::triggered(
            "Whenever an enchantment you control is put into a graveyard \
from the battlefield, create a 1/1 white Bird creature token \
with flying.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Bird"], &[ManaColor::White], 1, 1)
                    .with_abilities(&[abilities::flying()]),
            ))),
        ),
    ]),
);

// WOE 20 — Moment of Valor
pub(in crate::card::sets) static MOMENT_OF_VALOR: CardRecord = CardRecord::new(
    "Moment of Valor",
    "6f257fd6-24a4-4cc1-89e0-99cf5d821e3a",
    "Joshua Cairos",
    CardRules::new_instant(mana_cost!("{2}{W}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Untap target creature. It gets +1/+0 and gains indestructible \
until end of turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Sequence(&[
                    EffectDef::Untap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(1),
                                ValueDef::Constant(0),
                            ),
                            AppliedEffectDef::add_ability(&abilities::indestructible()),
                        ]),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
            AbilityDef::spell_with_targets(
                "Destroy target creature with power 4 or greater.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::PowerAtLeast(4),
                    ]),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
        ],
    )]),
);

// WOE 21 — Moonshaker Cavalry
pub(in crate::card::sets) static MOONSHAKER_CAVALRY: CardRecord = CardRecord::new(
    "Moonshaker Cavalry",
    "092c48bd-b648-4c9e-aa99-cac3c407911d",
    "Aldo Domínguez",
    CardRules::new_creature(mana_cost!("{5}{W}{W}{W}"), &["Spirit", "Knight"], 6, 6)
        .with_abilities(&[
            abilities::flying(),
            abilities::enters_trigger(
                "When this creature enters, creatures you control gain flying \
and get +X/+X until end of turn, where X is the number of \
creatures you control.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::flying()),
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                            ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                        ),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// WOE 22 — Plunge into Winter
pub(in crate::card::sets) static PLUNGE_INTO_WINTER: CardRecord = CardRecord::new(
    "Plunge into Winter",
    "fe4aceba-f386-457c-bc68-6382eda46754",
    "Vincent Christiaens",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Tap up to one target creature. Scry 1, then draw a card.",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            1,
        )],
        EffectDef::Sequence(&[
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            abilities::scry(ValueDef::Constant(1)),
            abilities::draw_cards(ValueDef::Constant(1)),
        ]),
    )]),
);

// WOE 23 — The Princess Takes Flight
pub(in crate::card::sets) static THE_PRINCESS_TAKES_FLIGHT: CardRecord = CardRecord::new(
    "The Princess Takes Flight",
    "dad7bd06-22e4-40f8-bda9-bcdbb2d8f632",
    "Julia Metzger",
    CardRules::new_enchantment(mana_cost!("{2}{W}"))
        .with_subtypes(&["Saga"])
        .with_abilities(&[
            abilities::saga_chapter_with_targets(
                1,
                "I — Exile up to one target creature.",
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::ExileLinkedToSource {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    face_down: false,
                    until_source_leaves: false,
                    then: None,
                },
            ),
            abilities::saga_chapter_with_targets(
                2,
                "II — Target creature you control gets +2/+2 and gains flying \
until end of turn.",
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
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            abilities::saga_chapter(
                3,
                "III — Return the exiled card to the battlefield under its \
owner's control.",
                EffectDef::ReturnLinkedExiles {
                    object: ObjectPredicateDef::Any,
                    zone: ZoneKind::Battlefield,
                    grant: None,
                    counters: None,
                    transformed: false,
                    controller: None,
                },
            ),
        ]),
);

// WOE 24 — Protective Parents
pub(in crate::card::sets) static PROTECTIVE_PARENTS: CardRecord = CardRecord::new(
    "Protective Parents",
    "68cc9653-80ef-4606-a0ec-6d4228fdb118",
    "Matt Stewart",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Peasant"], 3, 2).with_abilities(&[
        abilities::dies_trigger_with_targets(
            "When this creature dies, create a Young Hero Role token \
attached to up to one target creature you control. (If you \
control another Role on it, put that one into the graveyard. \
Enchanted creature has \"Whenever this creature attacks, if \
its toughness is 3 or less, put a +1/+1 counter on it.\")",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
                1,
            )],
            EffectDef::CreateAttachedToken {
                token: YOUNG_HERO_ROLE,
                host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
            },
        ),
    ]),
);

// WOE 25 — Regal Bunnicorn
pub(in crate::card::sets) static REGAL_BUNNICORN: CardRecord = CardRecord::new(
    "Regal Bunnicorn",
    "03c7d409-90e7-44d7-a8c6-4eda35fbcc83",
    "Ilse Gort",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Rabbit", "Unicorn"], 0, 0).with_ability(
        AbilityDef::static_ability(
            "Regal Bunnicorn's power and toughness are each equal to the \
number of nonland permanents you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::define_power_toughness(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                ),
            },
        ),
    ),
);

// WOE 26 — Return Triumphant
pub(in crate::card::sets) static RETURN_TRIUMPHANT: CardRecord = CardRecord::new(
    "Return Triumphant",
    "f5209c13-9591-48eb-8d6c-112b3bdd429a",
    "Will Gist",
    CardRules::new_sorcery(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return target creature card with mana value 3 or less from \
your graveyard to the battlefield. Create a Young Hero Role \
token attached to it. (Enchanted creature has \"Whenever this \
creature attacks, if its toughness is 3 or less, put a +1/+1 \
counter on it.\" If you put another Role on the creature \
later, put this one into the graveyard.)",
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
        EffectDef::WithZoneMoveResult {
            effect: &EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
            binding: crate::Binding!("returned"),
            then: &EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectSetCount(&ObjectSetCountConditionDef {
                    objects: &ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!(
                        "returned"
                    )),
                    predicate: ObjectSetPredicateDef::contains(&ObjectPredicateDef::HasType(
                        CardType::Creature,
                    )),
                }),
                then: &EffectDef::CreateAttachedToken {
                    token: YOUNG_HERO_ROLE,
                    host: Some(EffectRecipientDef::objects(
                        ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!("returned")),
                    )),
                },
            },
        },
    )]),
);

// WOE 27 — Rimefur Reindeer
pub(in crate::card::sets) static RIMEFUR_REINDEER: CardRecord = CardRecord::new(
    "Rimefur Reindeer",
    "60acc0b7-6842-44aa-a7cc-c5d315d90287",
    "Lucas Graciano",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Elk"], 3, 4).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever an enchantment you control enters, tap target \
creature an opponent controls.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// WOE 28 — Savior of the Sleeping
pub(in crate::card::sets) static SAVIOR_OF_THE_SLEEPING: CardRecord = CardRecord::new(
    "Savior of the Sleeping",
    "7b4979d9-fafb-4e0e-868f-f4772109d7a7",
    "Valera Lutfullina",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Knight"], 2, 3).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::triggered(
            "Whenever an enchantment you control is put into a graveyard \
from the battlefield, put a +1/+1 counter on this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// WOE 29 — Slumbering Keepguard
pub(in crate::card::sets) static SLUMBERING_KEEPGUARD: CardRecord = CardRecord::new(
    "Slumbering Keepguard",
    "d7f41ae2-ebc6-439f-95af-c34818a3f4e6",
    "Andreia Ugrai",
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Knight"], 1, 1)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever an enchantment you control enters, scry 1.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            abilities::scry(ValueDef::Constant(1)),
        )])
        .with_ability(AbilityDef::activated(
            "{2}{W}: This creature gets +1/+1 until end of turn for each \
enchantment you control.",
            &[CostDef::Mana(mana_cost!("{2}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )),
);

// WOE 30 — Solitary Sanctuary
// Audit: unsupported — Needs a tap event that records which player caused an opponent's previously untapped creature to become tapped; the existing event retains the tapped object but not the responsible player.
pub(in crate::card::sets) static SOLITARY_SANCTUARY: CardRecord = CardRecord::new(
    "Solitary Sanctuary",
    "d155693c-def0-4290-b662-ab9932e07fe5",
    "Kasia 'Kafis' Zielińska",
    CardRules::unsupported(),
);

// WOE 31 — Spellbook Vendor
// Audit: unsupported — Needs a reflexive trigger with targets or modes chosen after its preceding payment or event, retained even if the source has left the battlefield; choosing them with the original ability changes response timing and target legality.
pub(in crate::card::sets) static SPELLBOOK_VENDOR: CardRecord = CardRecord::new(
    "Spellbook Vendor",
    "4ceac5b5-05eb-4f00-9477-3db490be24dd",
    "Scott Murphy",
    CardRules::unsupported(),
);

// WOE 32 — Stockpiling Celebrant
pub(in crate::card::sets) static STOCKPILING_CELEBRANT: CardRecord = CardRecord::new(
    "Stockpiling Celebrant",
    "0214ebc6-c59f-4170-8dd2-ce07caa6e6ad",
    "Raluca Marinescu",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Dwarf", "Knight"], 3, 2).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, you may return another target \
nonland permanent you control to its owner's hand. If you do, \
scry 2.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::WithZoneMoveResult {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                    binding: crate::Binding!("returned"),
                    then: &EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ObjectSetCount(
                            &ObjectSetCountConditionDef {
                                objects: &ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                    crate::Binding!("returned"),
                                ),
                                predicate: ObjectSetPredicateDef {
                                    filter: None,
                                    comparison: ComparisonDef::GreaterOrEqual,
                                    amount: 1,
                                },
                            },
                        ),
                        then: &abilities::scry(ValueDef::Constant(2)),
                    },
                },
            },
        ),
    ]),
);

// WOE 33 — Stroke of Midnight
pub(in crate::card::sets) static STROKE_OF_MIDNIGHT: CardRecord = CardRecord::new(
    "Stroke of Midnight",
    "289ba7ec-e30e-436f-b8d4-c88b65ecd137",
    "Julia Metzger",
    CardRules::new_instant(mana_cost!("{2}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target nonland permanent. Its controller creates a \
1/1 white Human creature token.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(HUMAN_TOKEN)).with_controller(
                    PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                ),
            ),
        ]),
    )]),
);

// WOE 34 — A Tale for the Ages
pub(in crate::card::sets) static A_TALE_FOR_THE_AGES: CardRecord = CardRecord::new(
    "A Tale for the Ages",
    "ca0c8d3b-ce30-4da5-a6a8-9bdcb3c757f9",
    "Julie Dillon",
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::static_ability(
        "Enchanted creatures you control get +2/+2.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Enchanted,
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ))),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(2),
                ValueDef::Constant(2),
            ),
        },
    )]),
);

// WOE 35 — Three Blind Mice
pub(in crate::card::sets) static THREE_BLIND_MICE: CardRecord = CardRecord::new(
    "Three Blind Mice",
    "0d2ba371-854c-4529-b72b-e4a1887e33ab",
    "Andrew Mar",
    CardRules::new_enchantment(mana_cost!("{2}{W}"))
        .with_subtypes(&["Saga"])
        .with_abilities(&[
            abilities::saga_chapter(
                1,
                "I — Create a 1/1 white Mouse creature token.",
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Mouse"], &[ManaColor::White], 1, 1),
                ))),
            ),
            AbilityDef::triggered_with_targets(
                "II, III — Create a token that's a copy of target token you \
control.",
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::While {
                        event: &TriggerEventDef::CountersPlaced {
                            object: ObjectPredicateDef::Source,
                            kind: CounterKind::Lore,
                        },
                        condition: &TriggerConditionDef::SourceCounters {
                            kind: CounterKind::Lore,
                            comparison: ComparisonDef::Equal,
                            amount: 2,
                        },
                    },
                    TriggerEventDef::While {
                        event: &TriggerEventDef::CountersPlaced {
                            object: ObjectPredicateDef::Source,
                            kind: CounterKind::Lore,
                        },
                        condition: &TriggerConditionDef::SourceCounters {
                            kind: CounterKind::Lore,
                            comparison: ComparisonDef::Equal,
                            amount: 3,
                        },
                    },
                ]),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Token,
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
            abilities::saga_chapter(
                4,
                "IV — Creatures you control get +1/+1 and gain vigilance until \
end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::vigilance()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// WOE 36 — Tuinvale Guide
// Audit: unsupported — Needs per-player nonland-permanent entry history for the current turn, including objects that have since left; current battlefield entry timestamps cannot implement celebration.
pub(in crate::card::sets) static TUINVALE_GUIDE: CardRecord = CardRecord::new(
    "Tuinvale Guide",
    "01334fed-781e-4864-83fd-d37f787a778b",
    "Anastasia Ovchinnikova",
    CardRules::unsupported(),
);

// WOE 37 — Unassuming Sage
pub(in crate::card::sets) static UNASSUMING_SAGE: CardRecord = CardRecord::new(
    "Unassuming Sage",
    "a66fbaeb-1624-43b3-83e2-a37ce7588a5a",
    "Michele Giorgi",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Peasant", "Wizard"], 2, 2)
        .with_abilities(&[abilities::enters_trigger(
            "When this creature enters, you may pay {2}. If you do, create \
a Sorcerer Role token attached to it. (Enchanted creature \
gets +1/+1 and has \"Whenever this creature attacks, scry \
1.\")",
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::Mana(mana_cost!("{2}"))],
                &EffectDef::CreateAttachedToken {
                    token: SORCERER_ROLE,
                    host: Some(EffectRecipientDef::Source),
                },
            )),
        )]),
);

// WOE 38 — Virtue of Loyalty // Ardenvale Fealty (alternate printing)
const VIRTUE_OF_LOYALTY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VIRTUE_OF_LOYALTY,
    1,
    "ea7e7daf-7c06-4c74-8bcf-e42c1f611861",
    "Piotr Dura",
);

// WOE 39 — Werefox Bodyguard
// Audit: unsupported — Needs an immediate linked return when the source leaves, without a return trigger using the stack; the available exile-until helper installs a triggered return.
pub(in crate::card::sets) static WEREFOX_BODYGUARD: CardRecord = CardRecord::new(
    "Werefox Bodyguard",
    "4494dfa1-1343-417e-b0c5-2b096442dd0e",
    "Néstor Ossandón Leal",
    CardRules::unsupported(),
);

// WOE 40 — Aquatic Alchemist // Bubble Up
pub(in crate::card::sets) static AQUATIC_ALCHEMIST: CardRecord = CardRecord::new(
    "Aquatic Alchemist // Bubble Up",
    "e6f03f21-aeb9-428b-9167-b2604919bdd8",
    "Uriah Voth",
    CardRules::new_creature(mana_cost!("{1}{U}"), &const { ["Elemental"] }, 1, 3).with_abilities(
        &const {
            [AbilityDef::triggered(
                "Whenever you cast your first instant or sorcery spell each \
turn, this creature gets +2/+0 until end of turn.",
                TriggerEventDef::While {
                    event: &const {
                        TriggerEventDef::spell_cast(ObjectPredicateDef::All(
                            &const {
                                [
                                    ObjectPredicateDef::AnyOf(
                                        &const {
                                            [
                                                ObjectPredicateDef::HasType(CardType::Instant),
                                                ObjectPredicateDef::HasType(CardType::Sorcery),
                                            ]
                                        },
                                    ),
                                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                                ]
                            },
                        ))
                    },
                    condition: &const {
                        TriggerConditionDef::ValueComparison(
                            &const {
                                ValueComparisonDef {
                                    left: ValueDef::CountSpellsCastThisTurn(
                                        &const {
                                            SpellCastQueryDef {
                                                player: PlayerRelation::You,
                                                spell: ObjectPredicateDef::AnyOf(
                                                    &const {
                                                        [
                                                            ObjectPredicateDef::HasType(
                                                                CardType::Instant,
                                                            ),
                                                            ObjectPredicateDef::HasType(
                                                                CardType::Sorcery,
                                                            ),
                                                        ]
                                                    },
                                                ),
                                            }
                                        },
                                    ),
                                    comparison: ComparisonDef::Equal,
                                    right: ValueDef::Constant(1),
                                }
                            },
                        )
                    },
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            )]
        },
    ),
)
.with_composition(|| {
    adventure(
        &AQUATIC_ALCHEMIST,
        "Bubble Up",
        &CardRules::new_sorcery(mana_cost!("{2}{U}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Put target instant or sorcery card from your graveyard on top \
of your library. (Then exile this card. You may cast the \
creature later from exile.)",
                    &const {
                        [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::AnyOf(
                                    &const {
                                        [
                                            ObjectPredicateDef::HasType(CardType::Instant),
                                            ObjectPredicateDef::HasType(CardType::Sorcery),
                                        ]
                                    },
                                ),
                                zones: &const { [ZoneKind::Graveyard] },
                                controller: None,
                                owner: Some(PlayerRelation::You),
                            },
                        )]
                    },
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Library,
                        ZonePlacement::Top,
                    ),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 41 — Archive Dragon
pub(in crate::card::sets) static ARCHIVE_DRAGON: CardRecord = CardRecord::new(
    "Archive Dragon",
    "2979104f-7570-487c-8024-131d7ee3ab91",
    "Tyler Walpole",
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Dragon", "Wizard"], 4, 6).with_abilities(
        &[
            abilities::flying(),
            abilities::ward(&[CostDef::Mana(mana_cost!("{2}"))], "Ward {2}"),
            abilities::enters_trigger(
                "When this creature enters, scry 2.",
                abilities::scry(ValueDef::Constant(2)),
            ),
        ],
    ),
);

// WOE 42 — Asinine Antics
// Audit: unsupported — Needs optional additional payment tied to permission to cast at instant speed; current spell alternatives replace costs rather than conditionally purchasing flash.
pub(in crate::card::sets) static ASININE_ANTICS: CardRecord = CardRecord::new(
    "Asinine Antics",
    "50b96a97-0d7d-4e05-9e2f-0b99a039b655",
    "Brent Hollowell",
    CardRules::unsupported(),
);

// WOE 43 — Beluna's Gatekeeper // Entry Denied
pub(in crate::card::sets) static BELUNA_S_GATEKEEPER: CardRecord = CardRecord::new(
    "Beluna's Gatekeeper // Entry Denied",
    "5c1d410e-4237-4963-b015-54d26730e63d",
    "Kai Carpenter",
    CardRules::new_creature(mana_cost!("{5}{U}"), &const { ["Giant", "Soldier"] }, 6, 5),
)
.with_composition(|| {
    adventure(
        &BELUNA_S_GATEKEEPER,
        "Entry Denied",
        &CardRules::new_sorcery(mana_cost!("{1}{U}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Return target creature you don't control with mana value 3 or \
less to its owner's hand. (Then exile this card. You may cast \
the creature later from exile.)",
                    &const {
                        [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::All(
                                    &const {
                                        [
                                            ObjectPredicateDef::HasType(CardType::Creature),
                                            ObjectPredicateDef::ManaValueAtMost(3),
                                        ]
                                    },
                                ),
                                zones: &const { [ZoneKind::Battlefield] },
                                controller: Some(PlayerRelation::Opponent),
                                owner: None,
                            },
                        )]
                    },
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 44 — Bitter Chill
pub(in crate::card::sets) static BITTER_CHILL: CardRecord = CardRecord::new(
    "Bitter Chill",
    "888e3c71-e21d-4e77-b1b6-09769f9cd3d6",
    "Julie Dillon",
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
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
                "Enchanted creature doesn't untap during its controller's \
untap step.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                },
            ),
            abilities::dies_trigger(
                "When this Aura is put into a graveyard from the battlefield, \
you may pay {1}. If you do, scry 1, then draw a card.",
                EffectDef::PayOr(PayOrDef::optional(
                    &[CostDef::Mana(mana_cost!("{1}"))],
                    &EffectDef::Sequence(&[
                        abilities::scry(ValueDef::Constant(1)),
                        abilities::draw_cards(ValueDef::Constant(1)),
                    ]),
                )),
            ),
        ]),
);

// WOE 45 — Chancellor of Tales
pub(in crate::card::sets) static CHANCELLOR_OF_TALES: CardRecord = CardRecord::new(
    "Chancellor of Tales",
    "f67bd5ef-305b-4bf7-990b-3014778b14a0",
    "Joshua Raphael",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Faerie", "Advisor"], 2, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever you cast an Adventure spell, you may copy it. You \
may choose new targets for the copy.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Adventure")),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::CopyStackObject(&CopyStackObjectDef {
                    object: EffectRecipientDef::TriggeringObject,
                    controller: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(1),
                    retarget: true,
                    colors: None,
                }),
            },
        ),
    ]),
);

// WOE 46 — Diminisher Witch
pub(in crate::card::sets) static DIMINISHER_WITCH: CardRecord = CardRecord::new(
    "Diminisher Witch",
    "646d604f-b187-4122-bd4b-67634654b6f1",
    "Fariba Khamseh",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Warlock"], 3, 2).with_abilities(&[
        bargain(),
        AbilityDef::triggered_if_with_targets(
            "When this creature enters, if it was bargained, create a \
Cursed Role token attached to target creature an opponent \
controls. (If you control another Role on it, put that one \
into the graveyard. Enchanted creature is 1/1.)",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::SourcePaidAdditionalCost(AdditionalCostIndex::PRIMARY),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::CreateAttachedToken {
                token: CURSED_ROLE,
                host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
            },
        ),
    ]),
);

// WOE 47 — Disdainful Stroke (reprint)
const DISDAINFUL_STROKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::DISDAINFUL_STROKE,
    "588c6217-c460-417e-98bf-de5475780baf",
    "Eelis Kyttanen",
);

// WOE 48 — Extraordinary Journey
// Audit: unsupported — Needs a batched creature-entry matcher carrying entry-zone and cast-from-exile provenance for each member, plus owner-specific persistent play permissions for its exiled cards.
pub(in crate::card::sets) static EXTRAORDINARY_JOURNEY: CardRecord = CardRecord::new(
    "Extraordinary Journey",
    "a69fb480-a9fc-4f09-ac4e-3ce52c485ea9",
    "Volkan Baǵa",
    CardRules::unsupported(),
);

// WOE 49 — Farsight Ritual
pub(in crate::card::sets) static FARSIGHT_RITUAL: CardRecord = CardRecord::new(
    "Farsight Ritual",
    "c958257e-fa70-4fa3-90a1-0497967abef3",
    "Randy Gallegos",
    CardRules::new_instant(mana_cost!("{2}{U}{U}")).with_abilities(&[
        bargain(),
        AbilityDef::spell(
            "Look at the top four cards of your library. If this spell was \
bargained, look at the top eight cards of your library \
instead. Put two of them into your hand and the rest on the \
bottom of your library in a random order.",
            abilities::look_at_top_cards_choose_to_hand_rest_random_bottom(
                ValueDef::IfAdditionalCostPaid(&AdditionalCostValueDef::new(
                    AdditionalCostIndex::PRIMARY,
                    ValueDef::Constant(8),
                    ValueDef::Constant(4),
                )),
                ObjectPredicateDef::Any,
                2,
                2,
            ),
        ),
    ]),
);

// WOE 50 — Freeze in Place
pub(in crate::card::sets) static FREEZE_IN_PLACE: CardRecord = CardRecord::new(
    "Freeze in Place",
    "1a8bb9c7-2c4b-48a1-806e-742addb72b4b",
    "Leanna Crossan",
    CardRules::new_sorcery(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Tap target creature an opponent controls and put three stun \
counters on it. Scry 2. (If a permanent with a stun counter \
would become untapped, remove one from it instead.)",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
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
                amount: ValueDef::Constant(3),
            },
            abilities::scry(ValueDef::Constant(2)),
        ]),
    )]),
);

// WOE 51 — Gadwick's First Duel
// Audit: unsupported — Needs an installed trigger lifetime that fires once and also expires at end of turn; Once has no turn expiry and ThisTurn does not enforce the nested ability's trigger limit.
pub(in crate::card::sets) static GADWICK_S_FIRST_DUEL: CardRecord = CardRecord::new(
    "Gadwick's First Duel",
    "af07c47f-8b4e-43cb-b469-2efb82aa5590",
    "Chris Seaman",
    CardRules::unsupported(),
);

// WOE 52 — Galvanic Giant // Storm Reading
pub(in crate::card::sets) static GALVANIC_GIANT: CardRecord = CardRecord::new(
    "Galvanic Giant // Storm Reading",
    "60976109-30ad-4f12-99eb-c5ef560fcf1b",
    "Borja Pindado",
    CardRules::new_creature(mana_cost!("{3}{U}"), &const { ["Giant", "Wizard"] }, 3, 3)
        .with_abilities(
            &const {
                [AbilityDef::triggered_with_targets(
                    "Whenever you cast a spell with mana value 5 or greater, tap \
target creature an opponent controls and put a stun counter \
on it.",
                    TriggerEventDef::spell_cast(ObjectPredicateDef::All(
                        &const {
                            [
                                ObjectPredicateDef::Not(
                                    &const { ObjectPredicateDef::ManaValueAtMost(4) },
                                ),
                                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                            ]
                        },
                    )),
                    &const {
                        [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::HasType(CardType::Creature),
                                zones: &const { [ZoneKind::Battlefield] },
                                controller: Some(PlayerRelation::Opponent),
                                owner: None,
                            },
                        )]
                    },
                    EffectDef::Sequence(
                        &const {
                            [
                                EffectDef::Tap {
                                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                },
                                EffectDef::AddCounters {
                                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                    kind: CounterKind::Stun,
                                    amount: ValueDef::Constant(1),
                                },
                            ]
                        },
                    ),
                )]
            },
        ),
)
.with_composition(|| {
    adventure(
        &GALVANIC_GIANT,
        "Storm Reading",
        &CardRules::new_instant(mana_cost!("{5}{U}{U}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell(
                    "Draw four cards, then discard two cards. (Then exile this \
card. You may cast the creature later from exile.)",
                    EffectDef::Sequence(
                        &const {
                            [
                                abilities::draw_cards(ValueDef::Constant(4)),
                                EffectDef::Discard {
                                    recipient: EffectRecipientDef::Controller,
                                    amount: ValueDef::Constant(2),
                                    selection: DiscardSelectionDef::RecipientChooses,
                                    then: None,
                                },
                            ]
                        },
                    ),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 53 — Horned Loch-Whale // Lagoon Breach
// Audit: unsupported — Needs a prospective-entry replacement condition for whether it is its controller's turn; the current entry condition vocabulary does not expose the active player.
pub(in crate::card::sets) static HORNED_LOCH_WHALE: CardRecord = CardRecord::new(
    "Horned Loch-Whale // Lagoon Breach",
    "96a05063-0556-42e4-8d4c-8e92be160ef5",
    "Simon Dominic",
    CardRules::unsupported(),
);

// WOE 54 — Ice Out
// Audit: unsupported — Needs the source spell's generic cost reduction to depend on its selected bargain payment; the self-cost planner does not accept additional-cost payment values.
pub(in crate::card::sets) static ICE_OUT: CardRecord = CardRecord::new(
    "Ice Out",
    "88ffafda-c852-497a-8156-4f759cdf3693",
    "Olivier Bernard",
    CardRules::unsupported(),
);

// WOE 55 — Icewrought Sentry
// Audit: unsupported — Needs a reflexive trigger with targets or modes chosen after its preceding payment or event, retained even if the source has left the battlefield; choosing them with the original ability changes response timing and target legality.
pub(in crate::card::sets) static ICEWROUGHT_SENTRY: CardRecord = CardRecord::new(
    "Icewrought Sentry",
    "859419d3-cd15-4362-98b3-a7ff98e29692",
    "Brian Valeza",
    CardRules::unsupported(),
);

// WOE 56 — Ingenious Prodigy
pub(in crate::card::sets) static INGENIOUS_PRODIGY: CardRecord = CardRecord::new(
    "Ingenious Prodigy",
    "cf224968-b676-40dd-83c1-a9ee2ceba574",
    "Brian Valeza",
    CardRules::new_creature(mana_cost!("{X}{U}"), &["Human", "Wizard"], 0, 1).with_abilities(&[
        AbilityDef::static_ability(
            "Skulk (This creature can't be blocked by creatures with \
greater power.)",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::PowerGreaterThan(ValueDef::SourcePower),
                )),
            },
        ),
        AbilityDef::replacement(
            "This creature enters with X +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCastXCounters {
                    kind: CounterKind::PlusOnePlusOne,
                },
            ),
        ),
        AbilityDef::triggered_if(
            "At the beginning of your upkeep, if this creature has one or \
more +1/+1 counters on it, you may remove a +1/+1 counter \
from it. If you do, draw a card.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::SourceCounters {
                kind: CounterKind::PlusOnePlusOne,
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectSetCount(&ObjectSetCountConditionDef {
                    objects: &ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::Source,
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    )),
                    predicate: ObjectSetPredicateDef::contains(&ObjectPredicateDef::HasCounter(
                        CounterKind::PlusOnePlusOne,
                    )),
                }),
                then: &EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::Sequence(&[
                        EffectDef::RemoveCounters {
                            object: EffectRecipientDef::Source,
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                        abilities::draw_cards(ValueDef::Constant(1)),
                    ]),
                },
            },
        ),
    ]),
);

// WOE 57 — Into the Fae Court
pub(in crate::card::sets) static INTO_THE_FAE_COURT: CardRecord = CardRecord::new(
    "Into the Fae Court",
    "969b13bb-6411-41f9-b6b4-af4ffca62e17",
    "Anna Steinbauer",
    CardRules::new_sorcery(mana_cost!("{3}{U}{U}")).with_abilities(&[AbilityDef::spell(
        "Draw three cards. Create a 1/1 blue Faerie creature token \
with flying and \"This token can block only creatures with \
flying.\"",
        EffectDef::Sequence(&[
            abilities::draw_cards(ValueDef::Constant(3)),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Faerie"], &[ManaColor::Blue], 1, 1)
                    .with_abilities(&[
                        abilities::flying(),
                        AbilityDef::static_ability(
                            "This token can block only creatures with flying.",
                            EffectDef::StaticApply {
                                recipient: EffectRecipientDef::Source,
                                effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                                    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                                )),
                            },
                        ),
                    ]),
            ))),
        ]),
    )]),
);

// WOE 58 — Johann's Stopgap
// Audit: unsupported — Needs the source spell's generic cost reduction to depend on its selected bargain payment; the self-cost planner does not accept additional-cost payment values.
pub(in crate::card::sets) static JOHANN_S_STOPGAP: CardRecord = CardRecord::new(
    "Johann's Stopgap",
    "31408397-36f5-479f-b822-fa97411b7872",
    "Christina Kraus",
    CardRules::unsupported(),
);

// WOE 59 — Living Lectern
pub(in crate::card::sets) static LIVING_LECTERN: CardRecord = CardRecord::new(
    "Living Lectern",
    "169afcaf-7ebf-4590-9e51-2a1a5eb3ac76",
    "Chris Seaman",
    CardRules::new_artifact_creature(mana_cost!("{1}{U}"), &["Construct"], 0, 4).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}, Sacrifice this creature: Draw a card. Create a Sorcerer \
Role token attached to up to one other target creature you \
control. Activate only as a sorcery. (If you control another \
Role on it, put that one into the graveyard. Enchanted \
creature gets +1/+1 and has \"Whenever this creature attacks, \
scry 1.\")",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::SacrificeSource],
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
                1,
            )],
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::CreateAttachedToken {
                    token: SORCERER_ROLE,
                    host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                },
            ]),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// WOE 60 — Merfolk Coralsmith
pub(in crate::card::sets) static MERFOLK_CORALSMITH: CardRecord = CardRecord::new(
    "Merfolk Coralsmith",
    "a7f6a79c-aa2f-47d2-a929-1606a61f9341",
    "Evyn Fong",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk"], 2, 3).with_abilities(&[
        AbilityDef::activated(
            "{1}: This creature gets +1/-1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::dies_trigger(
            "When this creature dies, scry 2.",
            abilities::scry(ValueDef::Constant(2)),
        ),
    ]),
);

// WOE 61 — Misleading Motes
pub(in crate::card::sets) static MISLEADING_MOTES: CardRecord = CardRecord::new(
    "Misleading Motes",
    "4c6c7a43-c3d1-450e-834a-54e1b9def1cd",
    "Raoul Vitale",
    CardRules::new_instant(mana_cost!("{3}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature's owner puts it on their choice of the top or \
bottom of their library.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
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

// WOE 62 — Mocking Sprite
pub(in crate::card::sets) static MOCKING_SPRITE: CardRecord = CardRecord::new(
    "Mocking Sprite",
    "e595014d-4ff4-4561-b7f2-a9bd56300b01",
    "Ben Hill",
    // The discount is read off the battlefield, so an evasive body that
    // survives is what makes it pay -- and flying is why it does.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Faerie", "Rogue"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "Instant and sorcery spells you cast cost {1} less to cast.",
            EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                PlayerRelation::You,
                ValueDef::Constant(1),
            )),
        ),
    ]),
);

// WOE 63 — Obyra's Attendants // Desperate Parry
pub(in crate::card::sets) static OBYRA_S_ATTENDANTS: CardRecord = CardRecord::new(
    "Obyra's Attendants // Desperate Parry",
    "0001e77a-7fff-49d2-a55c-42f6fdf6db08",
    "Andreas Zafiratos",
    CardRules::new_creature(mana_cost!("{4}{U}"), &const { ["Faerie", "Wizard"] }, 3, 4)
        .with_abilities(&const { [abilities::flying()] }),
)
.with_composition(|| {
    adventure(
        &OBYRA_S_ATTENDANTS,
        "Desperate Parry",
        &CardRules::new_instant(mana_cost!("{1}{U}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Target creature gets -4/-0 until end of turn. (Then exile \
this card. You may cast the creature later from exile.)",
                    &const {
                        [AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )]
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(-4),
                            ValueDef::Constant(0),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 64 — Picklock Prankster // Free the Fae
pub(in crate::card::sets) static PICKLOCK_PRANKSTER: CardRecord =
    CardRecord::new(
        "Picklock Prankster // Free the Fae",
        "5ebac73a-1ecf-4e6d-87b1-ea560bfeb064",
        "Iris Compiet",
        CardRules::new_creature(mana_cost!("{1}{U}"), &const { ["Faerie", "Rogue"] }, 1, 3)
            .with_abilities(&const { [abilities::flying(), abilities::vigilance()] }),
    )
    .with_composition(|| {
        adventure(
            &PICKLOCK_PRANKSTER,
            "Free the Fae",
            &CardRules::new_instant(mana_cost!("{1}{U}"))
                .with_subtypes(&const { ["Adventure"] })
                .with_ability(
                    AbilityDef::spell(
                        "Mill four cards. Then put an instant, sorcery, or Faerie card \
from among the milled cards into your hand.",
                        EffectDef::Sequence(
                            &const {
                                [
                                    EffectDef::BindOutput {
                                        binding: crate::Binding!("milled"),
                                        effect: &const {
                                            EffectDef::Mill {
                                                player: EffectRecipientDef::Controller,
                                                amount: ValueDef::Constant(4),
                                            }
                                        },
                                    },
                                    EffectDef::Choose(ChooseDef {
                                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!(
                                            "chosen"
                                        )),
                                        unchosen: None,
                                        chooser: PlayerRefDef::EffectController,
                                        candidates: ObjectSetDef::Matching {
                                            objects: &const {
                                                ObjectSetDef::Binding(crate::Binding!("milled"))
                                            },
                                            object: ObjectSetFilterDef::Predicate(
                                                &const {
                                                    ObjectPredicateDef::AnyOf(
                                                        &const {
                                                            [
                                                                ObjectPredicateDef::AnyOf(
                                                                    &const {
                                                                        [
ObjectPredicateDef::HasType(CardType::Instant),
ObjectPredicateDef::HasType(CardType::Sorcery)]
                                                                    },
                                                                ),
                                                                ObjectPredicateDef::Subtype(
                                                                    SubtypeDef::Literal("Faerie"),
                                                                ),
                                                            ]
                                                        },
                                                    )
                                                },
                                            ),
                                        },
                                        exclude: None,
                                        minimum: 1,
                                        maximum: 1,
                                        visibility: ChoiceVisibilityDef::Public,
                                        then: &const {
                                            EffectDef::move_to_zone(
                                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                                    crate::Binding!("chosen"),
                                                )),
                                                ZoneKind::Hand,
                                                ZonePlacement::Top,
                                            )
                                        },
                                    }),
                                ]
                            },
                        ),
                    )
                    .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
                ),
        )
    });

// WOE 65 — Quick Study
pub(in crate::card::sets) static QUICK_STUDY: CardRecord = CardRecord::new(
    "Quick Study",
    "b78e2bca-bc93-464a-8911-8361abff2ac6",
    "Iris Compiet",
    CardRules::new_instant(mana_cost!("{2}{U}")).with_abilities(&[AbilityDef::spell(
        "Draw two cards.",
        abilities::draw_cards(ValueDef::Constant(2)),
    )]),
);

// WOE 66 — Sleep-Cursed Faerie
pub(in crate::card::sets) static SLEEP_CURSED_FAERIE: CardRecord = CardRecord::new(
    "Sleep-Cursed Faerie",
    "31051436-68f2-457e-8293-2b10ccf7684e",
    "Heonhwa",
    CardRules::new_creature(mana_cost!("{U}"), &["Faerie", "Wizard"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::ward(&[CostDef::Mana(mana_cost!("{2}"))], "Ward {2}"),
        AbilityDef::replacement(
            "This creature enters tapped with three stun counters on it.",
            ReplacementEffectDef::Sequence(&[
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                ),
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCounters {
                        kind: CounterKind::Stun,
                        amount: 3,
                    },
                ),
            ]),
        ),
        AbilityDef::activated(
            "{1}{U}: Untap this creature.",
            &[CostDef::Mana(mana_cost!("{1}{U}"))],
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// WOE 67 — Sleight of Hand (reprint)
const SLEIGHT_OF_HAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_p02::SLEIGHT_OF_HAND,
    "80dea5c0-ada3-488a-9f2b-f895b92c762f",
    "Scott Murphy",
);

// WOE 68 — Snaremaster Sprite
// Audit: unsupported — Needs a reflexive trigger with targets or modes chosen after its preceding payment or event, retained even if the source has left the battlefield; choosing them with the original ability changes response timing and target legality.
pub(in crate::card::sets) static SNAREMASTER_SPRITE: CardRecord = CardRecord::new(
    "Snaremaster Sprite",
    "eaa37390-5c32-46ae-89d1-c094c9aa01e5",
    "Christina Kraus",
    CardRules::unsupported(),
);

// WOE 69 — Spell Stutter
pub(in crate::card::sets) static SPELL_STUTTER: CardRecord = CardRecord::new(
    "Spell Stutter",
    "24447e36-a42f-40a9-ad44-e904b6f9b276",
    "Liiga Smilshkalne",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Counter target spell unless its controller pays {2} plus an \
additional {1} for each Faerie you control.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        abilities::counter_target_unless_paid(&[CostDef::GenericMana(ValueDef::Sum(
            &SumValueDef::new(
                ValueDef::Constant(2),
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Faerie")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            ),
        ))]),
    )]),
);

// WOE 70 — Splashy Spellcaster
pub(in crate::card::sets) static SPLASHY_SPELLCASTER: CardRecord = CardRecord::new(
    "Splashy Spellcaster",
    "73ebd7f0-a54d-43a8-a5ee-9d6835308794",
    "Brian Valeza",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Elemental", "Wizard"], 2, 4).with_abilities(
        &[AbilityDef::triggered_with_targets(
            "Whenever you cast an instant or sorcery spell, create a \
Sorcerer Role token attached to up to one other target \
creature you control. (If you control another Role on it, put \
that one into the graveyard. Enchanted creature gets +1/+1 \
and has \"Whenever this creature attacks, scry 1.\")",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
                1,
            )],
            EffectDef::CreateAttachedToken {
                token: SORCERER_ROLE,
                host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
            },
        )],
    ),
);

// WOE 71 — Stormkeld Prowler
pub(in crate::card::sets) static STORMKELD_PROWLER: CardRecord = CardRecord::new(
    "Stormkeld Prowler",
    "ff065dbf-77e3-45a8-bcca-aff9eaeb151f",
    "Zara Alfonso",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Rogue"], 2, 1).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you cast a spell with mana value 5 or greater, put \
two +1/+1 counters on this creature.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(4)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// WOE 72 — Succumb to the Cold
pub(in crate::card::sets) static SUCCUMB_TO_THE_COLD: CardRecord = CardRecord::new(
    "Succumb to the Cold",
    "c14d9fc0-bfbf-4359-93bf-5e53466965d6",
    "Andrew Mar",
    CardRules::new_instant(mana_cost!("{2}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Tap one or two target creatures an opponent controls. Put a \
stun counter on each of them. (If a permanent with a stun \
counter would become untapped, remove one from it instead.)",
        &[AbilityTargetDef {
            minimum: 1,
            maximum: 2,
            ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &const { [ZoneKind::Battlefield] },
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            })
        }],
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
    )]),
);

// WOE 73 — Talion's Messenger
// Audit: unsupported — Needs a reflexive trigger with targets or modes chosen after its preceding payment or event, retained even if the source has left the battlefield; choosing them with the original ability changes response timing and target legality.
pub(in crate::card::sets) static TALION_S_MESSENGER: CardRecord = CardRecord::new(
    "Talion's Messenger",
    "35fb0640-5b04-4687-b863-46a8b8d36809",
    "Marta Nael",
    CardRules::unsupported(),
);

// WOE 74 — Tenacious Tomeseeker
pub(in crate::card::sets) static TENACIOUS_TOMESEEKER: CardRecord = CardRecord::new(
    "Tenacious Tomeseeker",
    "c40be735-0780-459b-8dd2-a298575beaab",
    "Kai Carpenter",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Knight"], 3, 2).with_abilities(&[
        bargain(),
        AbilityDef::triggered_if_with_targets(
            "When this creature enters, if it was bargained, return target \
instant or sorcery card from your graveyard to your hand.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::SourcePaidAdditionalCost(AdditionalCostIndex::PRIMARY),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
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

// WOE 75 — Vantress Transmuter // Croaking Curse
pub(in crate::card::sets) static VANTRESS_TRANSMUTER: CardRecord = CardRecord::new(
    "Vantress Transmuter // Croaking Curse",
    "11507fa1-ef9e-41c9-b987-be57a03bd0df",
    "Andrey Kuzinskiy",
    CardRules::new_creature(mana_cost!("{3}{U}"), &const { ["Human", "Wizard"] }, 3, 4),
)
.with_composition(|| {
    adventure(
        &VANTRESS_TRANSMUTER,
        "Croaking Curse",
        &CardRules::new_sorcery(mana_cost!("{1}{U}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Tap target creature. Create a Cursed Role token attached to \
it. (Enchanted creature is 1/1.)",
                    &const {
                        [AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )]
                    },
                    EffectDef::Sequence(
                        &const {
                            [
                                EffectDef::Tap {
                                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                },
                                EffectDef::CreateAttachedToken {
                                    token: CURSED_ROLE,
                                    host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                                },
                            ]
                        },
                    ),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 76 — Virtue of Knowledge // Vantress Visions
pub(in crate::card::sets) static VIRTUE_OF_KNOWLEDGE: CardRecord = CardRecord::new(
    "Virtue of Knowledge // Vantress Visions",
    "df606cf5-67dc-46f4-8c79-1d2f1d054391",
    "Piotr Dura",
    CardRules::new_enchantment(mana_cost!("{4}{U}")).with_abilities(
        &const {
            [AbilityDef::static_ability(
                "If a permanent entering causes a triggered ability of a \
permanent you control to trigger, that ability triggers an \
additional time.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::TriggersAnAdditionalTime(
                        &const {
                            AdditionalTriggerDef {
                                entering: ObjectPredicateDef::AnyOf(
                                    &const {
                                        [
                                            ObjectPredicateDef::HasType(CardType::Creature),
                                            ObjectPredicateDef::HasType(CardType::Artifact),
                                            ObjectPredicateDef::HasType(CardType::Enchantment),
                                            ObjectPredicateDef::HasType(CardType::Land),
                                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                                        ]
                                    },
                                ),
                                permanent: ObjectPredicateDef::All(
                                    &const {
                                        [
                                            ObjectPredicateDef::AnyOf(
                                                &const {
                                                    [
                                                        ObjectPredicateDef::HasType(
                                                            CardType::Creature,
                                                        ),
                                                        ObjectPredicateDef::HasType(
                                                            CardType::Artifact,
                                                        ),
                                                        ObjectPredicateDef::HasType(
                                                            CardType::Enchantment,
                                                        ),
                                                        ObjectPredicateDef::HasType(CardType::Land),
                                                        ObjectPredicateDef::HasType(
                                                            CardType::Planeswalker,
                                                        ),
                                                    ]
                                                },
                                            ),
                                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                                        ]
                                    },
                                ),
                            }
                        },
                    )),
                },
            )]
        },
    ),
)
.with_composition(|| {
    adventure(
        &VIRTUE_OF_KNOWLEDGE,
        "Vantress Visions",
        &CardRules::new_instant(mana_cost!("{1}{U}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Copy target activated or triggered ability you control. You \
may choose new targets for the copy.",
                    &const {
                        [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::Ability,
                                zones: &const { [ZoneKind::Stack] },
                                controller: Some(PlayerRelation::You),
                                owner: None,
                            },
                        )]
                    },
                    EffectDef::CopyStackObject(
                        &const {
                            CopyStackObjectDef {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                controller: PlayerRefDef::EffectController,
                                count: ValueDef::Constant(1),
                                retarget: true,
                                colors: None,
                            }
                        },
                    ),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 77 — Water Wings
pub(in crate::card::sets) static WATER_WINGS: CardRecord = CardRecord::new(
    "Water Wings",
    "4ea4993c-d1ba-4b33-955b-e0874fd2132f",
    "Arash Radkia",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Until end of turn, target creature you control has base power \
and toughness 4/4 and gains flying and hexproof. (It can't be \
the target of spells or abilities your opponents control.)",
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
                AppliedEffectDef::set_base_power_toughness(
                    ValueDef::Constant(4),
                    ValueDef::Constant(4),
                ),
                AppliedEffectDef::add_ability(&abilities::flying()),
                AppliedEffectDef::add_ability(&abilities::hexproof()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// WOE 78 — Ashiok, Wicked Manipulator
// Audit: unsupported — Needs replacement of life payments with library exile, including affordability checks and resumable replacement processing, plus current-turn exile history for its Nightmare tokens.
pub(in crate::card::sets) static ASHIOK_WICKED_MANIPULATOR: CardRecord = CardRecord::new(
    "Ashiok, Wicked Manipulator",
    "6c4d0db1-74a5-42c0-ac95-e696585d8022",
    "Raymond Swanland",
    CardRules::unsupported(),
);

// WOE 79 — Ashiok's Reaper
pub(in crate::card::sets) static ASHIOK_S_REAPER: CardRecord = CardRecord::new(
    "Ashiok's Reaper",
    "83957fe0-6500-420c-9b7a-2448a1c1d3b3",
    "Denis Zhbankov",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Nightmare"], 3, 3).with_abilities(&[
        AbilityDef::triggered(
            "Whenever an enchantment you control is put into a graveyard \
from the battlefield, draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// WOE 80 — Back for Seconds
pub(in crate::card::sets) static BACK_FOR_SECONDS: CardRecord = CardRecord::new(
    "Back for Seconds",
    "660845b5-96fa-4484-822b-aa0508801306",
    "Julia Metzger",
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_abilities(&[
        bargain(),
        AbilityDef::spell_with_targets(
            "Return up to two target creature cards from your graveyard to \
your hand. If this spell was bargained, you may put one of \
those cards with mana value 4 or less onto the battlefield \
instead of putting it into your hand.",
            &[AbilityTargetDef {
                minimum: 0,
                maximum: 2,
                ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                })
            }],
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourcePaidAdditionalCost(
                    AdditionalCostIndex::PRIMARY,
                ),
                then: &EffectDef::ClassifyObjects(ClassifyObjectsDef {
                    input: ObjectSetDef::LegalTargets(TargetIndex::PRIMARY),
                    object: ObjectPredicateDef::ManaValueAtMost(4),
                    matching: crate::Binding!("small"),
                    remainder: crate::Binding!("large"),
                    then: &EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("battlefield")),
                        unchosen: Some(crate::Binding!("hand")),
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Binding(crate::Binding!("small")),
                        exclude: None,
                        minimum: 0,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::Sequence(&[
                            EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("battlefield"),
                                )),
                                ZoneKind::Battlefield,
                                ZonePlacement::Top,
                            ),
                            EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Union(&[
                                    ObjectSetDef::Binding(crate::Binding!("hand")),
                                    ObjectSetDef::Binding(crate::Binding!("large")),
                                ])),
                                ZoneKind::Hand,
                                ZonePlacement::Top,
                            ),
                        ]),
                    }),
                }),
                otherwise: &EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            },
        ),
    ]),
);

// WOE 81 — Barrow Naughty
pub(in crate::card::sets) static BARROW_NAUGHTY: CardRecord = CardRecord::new(
    "Barrow Naughty",
    "67da5ad8-4de2-4bd4-8b95-f2657e1fdee5",
    "Matt Forsyth",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Faerie"], 1, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature has lifelink as long as you control another Faerie.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Faerie")),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                },
            },
        ),
        AbilityDef::activated(
            "{2}{B}: This creature gets +1/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{2}{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// WOE 82 — Beseech the Mirror
// Audit: unsupported — Needs a free-cast offer bounded by the selected spell form's mana value and a face-down searched-card binding; checking the card before spell-form selection is insufficient for Adventures and other alternate spells.
pub(in crate::card::sets) static BESEECH_THE_MIRROR: CardRecord = CardRecord::new(
    "Beseech the Mirror",
    "18c59776-e1f1-4197-a128-db1d603f56b7",
    "Cynthia Sheppard",
    CardRules::unsupported(),
);

// WOE 83 — Candy Grapple
pub(in crate::card::sets) static CANDY_GRAPPLE: CardRecord = CardRecord::new(
    "Candy Grapple",
    "190d97bc-dbef-496d-9bd1-b785bdf8a964",
    "Konstantin Porubov",
    // Two mana kills most of what a limited deck plays, and the Food this
    // set hands out is what turns the rest into targets too.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[
        AbilityDef::optional_additional_cost(
            "Bargain (You may sacrifice an artifact, enchantment, or token \
as you cast this spell.)",
            OptionalAdditionalCostAbilityDef {
                kind: OptionalAdditionalCostKindDef::Bargain,
                label: OptionalAdditionalCostKindDef::Bargain.label(),
                resolution_destination: SpellResolutionDestinationDef::Graveyard,
                costs: &[CostDef::Sacrifice {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::Token,
                    ]),
                    quantity: CostQuantityDef::Fixed(1),
                }],
            },
        ),
        AbilityDef::spell_with_targets(
            "Target creature gets -3/-3 until end of turn. If this spell \
was bargained, that creature gets -5/-5 until end of turn \
instead.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                // One effect reading the payment back, not two: "instead"
                // means the bargained spell never applies the smaller number.
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::IfAdditionalCostPaid(&AdditionalCostValueDef::new(
                        AdditionalCostIndex::PRIMARY,
                        ValueDef::Constant(-5),
                        ValueDef::Constant(-3),
                    )),
                    ValueDef::IfAdditionalCostPaid(&AdditionalCostValueDef::new(
                        AdditionalCostIndex::PRIMARY,
                        ValueDef::Constant(-5),
                        ValueDef::Constant(-3),
                    )),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// WOE 84 — Conceited Witch // Price of Beauty
pub(in crate::card::sets) static CONCEITED_WITCH: CardRecord = CardRecord::new(
    "Conceited Witch // Price of Beauty",
    "f8a0c0f6-fef9-42c5-934d-a2855c11b440",
    "Anna Pavleeva",
    CardRules::new_creature(mana_cost!("{2}{B}"), &const { ["Human", "Warlock"] }, 2, 3)
        .with_abilities(&const { [abilities::menace()] }),
)
.with_composition(|| {
    adventure(
        &CONCEITED_WITCH,
        "Price of Beauty",
        &CardRules::new_sorcery(mana_cost!("{B}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Create a Wicked Role token attached to target creature you \
control. (Then exile this card. You may cast the creature \
later from exile.)",
                    &const {
                        [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::HasType(CardType::Creature),
                                zones: &const { [ZoneKind::Battlefield] },
                                controller: Some(PlayerRelation::You),
                                owner: None,
                            },
                        )]
                    },
                    EffectDef::CreateAttachedToken {
                        token: WICKED_ROLE,
                        host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                    },
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 85 — Dream Spoilers
pub(in crate::card::sets) static DREAM_SPOILERS: CardRecord = CardRecord::new(
    "Dream Spoilers",
    "4efd1963-fe71-42c1-8ad7-53fd80145ca6",
    "Jodie Muir",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Faerie", "Warlock"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "Whenever you cast a spell during an opponent's turn, up to \
one target creature an opponent controls gets -1/-1 until end \
of turn.",
            TriggerEventDef::While {
                event: &TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Any,
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::Opponent),
            },
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
                1,
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// WOE 86 — Ego Drain
pub(in crate::card::sets) static EGO_DRAIN: CardRecord = CardRecord::new(
    "Ego Drain",
    "8faf36da-bbad-4d6e-a530-502d47a2dd23",
    "Valera Lutfullina",
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target opponent reveals their hand. You choose a nonland card \
from it. That player discards that card. If you don't control \
a Faerie, exile a card from your hand.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
        )],
        EffectDef::Sequence(&[
            EffectDef::Sequence(&abilities::reveal_hand_and_discard_chosen_card(
                PlayerRefDef::Target(TargetIndex::PRIMARY),
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            )),
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::Not(&TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Faerie")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                }),
                then: &EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("exiled")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Hand],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Private,
                    then: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "exiled"
                        ))),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                }),
            },
        ]),
    )]),
);

// WOE 87 — The End
// Audit: unsupported — Needs a self spell-cost reduction conditional on the controller's life total; the current self-cost evaluator does not support life-total conditions.
pub(in crate::card::sets) static THE_END: CardRecord = CardRecord::new(
    "The End",
    "b18402dc-c4ab-417c-92d1-5e4d9cfb840d",
    "Donato Giancola",
    CardRules::unsupported(),
);

// WOE 88 — Eriette's Whisper
pub(in crate::card::sets) static ERIETTE_S_WHISPER: CardRecord = CardRecord::new(
    "Eriette's Whisper",
    "dfed2acf-9ac8-447b-94f5-7db3a713c991",
    "Quintin Gleim",
    CardRules::new_sorcery(mana_cost!("{3}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target opponent discards two cards. Create a Wicked Role \
token attached to up to one target creature you control. (If \
you control another Role on it, put that one into the \
graveyard. Enchanted creature gets +1/+1. When this token is \
put into a graveyard, each opponent loses 1 life.)",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Opponent)),
            AbilityTargetDef {
                minimum: 0,
                ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::You),
                    owner: None,
                })
            },
        ],
        EffectDef::Sequence(&[
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
            EffectDef::CreateAttachedToken {
                token: WICKED_ROLE,
                host: Some(EffectRecipientDef::Target(TargetIndex(1))),
            },
        ]),
    )]),
);

// WOE 89 — Faerie Dreamthief
pub(in crate::card::sets) static FAERIE_DREAMTHIEF: CardRecord = CardRecord::new(
    "Faerie Dreamthief",
    "57ca2ec5-442d-4909-be28-93c50fbc5f7a",
    "Randy Vargas",
    CardRules::new_creature(mana_cost!("{B}"), &["Faerie", "Warlock"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, surveil 1. (Look at the top card \
of your library. You may put it into your graveyard.)",
            abilities::surveil(ValueDef::Constant(1)),
        ),
        AbilityDef::activated(
            "{2}{B}, Exile this card from your graveyard: You draw a card \
and you lose 1 life.",
            &[CostDef::Mana(mana_cost!("{2}{B}")), CostDef::ExileSource],
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// WOE 90 — Faerie Fencing
// Audit: unsupported — Needs the casting-time fact that its controller controlled a Faerie preserved on the spell; checking the current battlefield during resolution changes the additional -3/-3.
pub(in crate::card::sets) static FAERIE_FENCING: CardRecord = CardRecord::new(
    "Faerie Fencing",
    "6bdcaf24-4352-47cf-a043-899be47ab1bb",
    "Evyn Fong",
    CardRules::unsupported(),
);

// WOE 91 — Feed the Cauldron
pub(in crate::card::sets) static FEED_THE_CAULDRON: CardRecord = CardRecord::new(
    "Feed the Cauldron",
    "0cfd18a5-e06a-4cb5-b78e-de18ec641321",
    "Marta Nael",
    CardRules::new_instant(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target creature with mana value 3 or less. If it's \
your turn, create a Food token. (It's an artifact with \"{2}, \
{T}, Sacrifice this token: You gain 3 life.\")",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ManaValueAtMost(3),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                then: &EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            },
        ]),
    )]),
);

// WOE 92 — Fell Horseman // Deathly Ride
pub(in crate::card::sets) static FELL_HORSEMAN: CardRecord = CardRecord::new(
    "Fell Horseman // Deathly Ride",
    "43bb3890-4013-48be-8cb5-54fd8fd8ec52",
    "Igor Krstic",
    CardRules::new_creature(mana_cost!("{3}{B}"), &const { ["Zombie", "Knight"] }, 3, 3)
        .with_abilities(
            &const {
                [abilities::dies_trigger(
                    "When this creature dies, put it on the bottom of its owner's \
library.",
                    EffectDef::move_to_zone(
                        EffectRecipientDef::TriggeringZoneChangeResult,
                        ZoneKind::Library,
                        ZonePlacement::Bottom,
                    ),
                )]
            },
        ),
)
.with_composition(|| {
    adventure(
        &FELL_HORSEMAN,
        "Deathly Ride",
        &CardRules::new_sorcery(mana_cost!("{1}{B}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Return target creature card from your graveyard to your hand. \
(Then exile this card. You may cast the creature later from \
exile.)",
                    &const {
                        [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::HasType(CardType::Creature),
                                zones: &const { [ZoneKind::Graveyard] },
                                controller: None,
                                owner: Some(PlayerRelation::You),
                            },
                        )]
                    },
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 93 — Gumdrop Poisoner // Tempt with Treats
pub(in crate::card::sets) static GUMDROP_POISONER: CardRecord = CardRecord::new(
    "Gumdrop Poisoner // Tempt with Treats",
    "5cb01d4d-91c2-41c6-981e-b4135a1e1e36",
    "Brian Valeza",
    CardRules::new_creature(mana_cost!("{2}{B}"), &const { ["Human", "Warlock"] }, 3, 2)
        .with_abilities(
            &const {
                [
                    abilities::lifelink(),
                    abilities::enters_trigger_with_targets(
                        "When this creature enters, up to one target creature gets \
-X/-X until end of turn, where X is the amount of life you \
gained this turn.",
                        &const {
                            [AbilityTargetDef::up_to(
                                AbilityTargetPredicate::Object {
                                    object: ObjectPredicateDef::HasType(CardType::Creature),
                                    zones: &const { [ZoneKind::Battlefield] },
                                    controller: None,
                                    owner: None,
                                },
                                1,
                            )]
                        },
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::modify_power_toughness(
                                ValueDef::Scaled(
                                    &const {
                                        ScaledValueDef::new(
                                            ValueDef::LifeGainedThisTurn(PlayerRelation::You),
                                            -1,
                                        )
                                    },
                                ),
                                ValueDef::Scaled(
                                    &const {
                                        ScaledValueDef::new(
                                            ValueDef::LifeGainedThisTurn(PlayerRelation::You),
                                            -1,
                                        )
                                    },
                                ),
                            ),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ),
                ]
            },
        ),
)
.with_composition(|| {
    adventure(
        &GUMDROP_POISONER,
        "Tempt with Treats",
        &CardRules::new_instant(mana_cost!("{B}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell(
                    "Create a Food token. (Then exile this card. You may cast the \
creature later from exile.)",
                    EffectDef::CreateToken(
                        CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                            .with_count(ValueDef::Constant(1)),
                    ),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 94 — High Fae Negotiator
pub(in crate::card::sets) static HIGH_FAE_NEGOTIATOR: CardRecord = CardRecord::new(
    "High Fae Negotiator",
    "f0fc77e7-154d-4433-93b3-1a1dee34791b",
    "Anna Christenson",
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Faerie", "Warlock"], 3, 5).with_abilities(
        &[
            bargain(),
            abilities::flying(),
            AbilityDef::triggered_if(
                "When this creature enters, if it was bargained, each opponent \
loses 3 life and you gain 3 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                &TriggerConditionDef::SourcePaidAdditionalCost(AdditionalCostIndex::PRIMARY),
                EffectDef::Sequence(&[
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Opponent,
                        amount: ValueDef::Constant(3),
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                ]),
            ),
        ],
    ),
);

// WOE 95 — Hopeless Nightmare
pub(in crate::card::sets) static HOPELESS_NIGHTMARE: CardRecord = CardRecord::new(
    "Hopeless Nightmare",
    "2c2ee817-9ca9-4f09-bc71-7994c19a9470",
    "Dominik Mayer",
    CardRules::new_enchantment(mana_cost!("{B}")).with_abilities(&[
        abilities::enters_trigger(
            "When this enchantment enters, each opponent discards a card \
and loses 2 life.",
            EffectDef::Sequence(&[
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
        abilities::dies_trigger(
            "When this enchantment is put into a graveyard from the \
battlefield, scry 2.",
            abilities::scry(ValueDef::Constant(2)),
        ),
        AbilityDef::activated(
            "{2}{B}: Sacrifice this enchantment.",
            &[CostDef::Mana(mana_cost!("{2}{B}"))],
            EffectDef::sacrifice(EffectRecipientDef::Source),
        ),
    ]),
);

// WOE 96 — Lich-Knights' Conquest
pub(in crate::card::sets) static LICH_KNIGHTS_CONQUEST: CardRecord = CardRecord::new(
    "Lich-Knights' Conquest",
    "59cd67d3-3327-42ad-9db1-50e2f591818c",
    "Denis Zhbankov",
    CardRules::new_sorcery(mana_cost!("{4}{B}")).with_abilities(&[AbilityDef::spell(
        "Sacrifice any number of artifacts, enchantments, and/or \
tokens. Return that many creature cards from your graveyard \
to the battlefield.",
        EffectDef::Choose(ChooseDef {
            binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
            unchosen: None,
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::Token,
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            )),
            exclude: None,
            minimum: 0,
            maximum: usize::MAX,
            visibility: ChoiceVisibilityDef::Public,
            then: &EffectDef::Sequence(&[
                EffectDef::sacrifice(EffectRecipientDef::objects(ObjectSetDef::Binding(
                    crate::Binding!("chosen"),
                ))),
                EffectDef::ChooseExact(ChooseExactDef {
                    binding: crate::Binding!("returned"),
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    amount: ValueDef::BoundObjectCount(crate::Binding!("chosen")),
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "returned"
                        ))),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                }),
            ]),
        }),
    )]),
);

// WOE 97 — Lord Skitter, Sewer King
pub(in crate::card::sets) static LORD_SKITTER_SEWER_KING: CardRecord = CardRecord::new(
    "Lord Skitter, Sewer King",
    "729877be-4894-4ef5-9e60-de8a8fb2bdc0",
    "Jesper Ejsing",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Rat", "Noble"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered_with_targets(
                "Whenever another Rat you control enters, exile up to one \
target card from an opponent's graveyard.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rat")),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                &[AbilityTargetDef {
                    minimum: 0,
                    ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Any,
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::Opponent),
                    })
                }],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            ),
            AbilityDef::triggered(
                "At the beginning of combat on your turn, create a 1/1 black \
Rat creature token with \"This token can't block.\"",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::BeginningOfCombat,
                    player: PlayerRelation::You,
                },
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    DEFENSELESS_RAT_TOKEN,
                ))),
            ),
        ]),
);

// WOE 98 — Lord Skitter's Blessing
pub(in crate::card::sets) static LORD_SKITTER_S_BLESSING: CardRecord = CardRecord::new(
    "Lord Skitter's Blessing",
    "84f343a9-883f-4532-ae66-be6470d67d38",
    "Joseph Weston",
    CardRules::new_enchantment(mana_cost!("{1}{B}")).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this enchantment enters, create a Wicked Role token \
attached to target creature you control. (Enchanted creature \
gets +1/+1. When this token is put into a graveyard, each \
opponent loses 1 life.)",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::CreateAttachedToken {
                token: WICKED_ROLE,
                host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
            },
        ),
        AbilityDef::triggered_if(
            "At the beginning of your draw step, if you control an \
enchanted creature, you lose 1 life and you draw an \
additional card.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Draw,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Enchanted,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                abilities::draw_cards(ValueDef::Constant(1)),
            ]),
        ),
    ]),
);

// WOE 99 — Lord Skitter's Butcher
pub(in crate::card::sets) static LORD_SKITTER_S_BUTCHER: CardRecord = CardRecord::new(
    "Lord Skitter's Butcher",
    "21b31d2b-ef66-4e16-a75e-4e27eb5ebfe9",
    "Leesha Hannigan",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Rat", "Peasant"], 2, 3).with_abilities(&[
        AbilityDef::modal_triggered(
            "When this creature enters, choose one —",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[
                AbilityDef::spell(
                    "Create a 1/1 black Rat creature token with \"This token can't \
block.\"",
                    EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        DEFENSELESS_RAT_TOKEN,
                    ))),
                ),
                AbilityDef::spell(
                    "You may sacrifice another creature. If you do, scry 2, then \
draw a card.",
                    EffectDef::PayOr(PayOrDef::optional(
                        &[CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]))],
                        &EffectDef::Sequence(&[
                            abilities::scry(ValueDef::Constant(2)),
                            abilities::draw_cards(ValueDef::Constant(1)),
                        ]),
                    )),
                ),
                AbilityDef::spell(
                    "Creatures you control gain menace until end of turn.",
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                        )),
                        effect: AppliedEffectDef::add_ability(&abilities::menace()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
            ],
        ),
    ]),
);

// WOE 100 — Mintstrosity
pub(in crate::card::sets) static MINTSTROSITY: CardRecord = CardRecord::new(
    "Mintstrosity",
    "d902f154-6fe8-4b97-aa67-4d4696abf887",
    "Slawomir Maniak",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Horror"], 3, 1).with_abilities(&[
        abilities::dies_trigger(
            "When this creature dies, create a Food token. (It's an \
artifact with \"{2}, {T}, Sacrifice this token: You gain 3 \
life.\")",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
    ]),
);

// WOE 101 — Not Dead After All
pub(in crate::card::sets) static NOT_DEAD_AFTER_ALL: CardRecord = CardRecord::new(
    "Not Dead After All",
    "d01a2b68-efe6-4027-846d-db7b19d9eef6",
    "Randy Vargas",
    CardRules::new_instant(mana_cost!("{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Until end of turn, target creature you control gains \"When \
this creature dies, return it to the battlefield tapped under \
its owner's control, then create a Wicked Role token attached \
to it.\" (Enchanted creature gets +1/+1. When this token is \
put into a graveyard, each opponent loses 1 life.)",
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
            effect: AppliedEffectDef::add_ability(&abilities::dies_trigger(
                "When this creature dies, return it to the battlefield tapped \
under its owner's control, then create a Wicked Role token \
attached to it.",
                EffectDef::WithZoneMoveResult {
                    effect: &EffectDef::WithBattlefieldArrival {
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
                    binding: crate::Binding!("returned"),
                    then: &EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ObjectSetCount(
                            &ObjectSetCountConditionDef {
                                objects: &ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                    crate::Binding!("returned"),
                                ),
                                predicate: ObjectSetPredicateDef::contains(
                                    &ObjectPredicateDef::HasType(CardType::Creature),
                                ),
                            },
                        ),
                        then: &EffectDef::CreateAttachedToken {
                            token: WICKED_ROLE,
                            host: Some(EffectRecipientDef::objects(
                                ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!(
                                    "returned"
                                )),
                            )),
                        },
                    },
                },
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// WOE 102 — Rankle's Prank
pub(in crate::card::sets) static RANKLE_S_PRANK: CardRecord = CardRecord::new(
    "Rankle's Prank",
    "e8a9bdcf-160a-48c9-9750-778c910b805d",
    "Tyler Walpole",
    CardRules::new_sorcery(mana_cost!("{2}{B}{B}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one or more —",
        &[
            AbilityDef::spell(
                "Each player discards two cards.",
                EffectDef::Discard {
                    recipient: EffectRecipientDef::EachPlayer,
                    amount: ValueDef::Constant(2),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ),
            AbilityDef::spell(
                "Each player loses 4 life.",
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::EachPlayer,
                    amount: ValueDef::Constant(4),
                },
            ),
            AbilityDef::spell(
                "Each player sacrifices two creatures of their choice.",
                EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                    player: EffectRecipientDef::EachPlayer,
                    zone: ZoneKind::Battlefield,
                    candidates: ObjectPredicateDef::HasType(CardType::Creature),
                    selection: PerPlayerSelectionDef::Count(ValueDef::Constant(2)),
                    chosen: crate::Binding!("chosen"),
                    unchosen: crate::Binding!("others"),
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::sacrifice(EffectRecipientDef::objects(
                        ObjectSetDef::Binding(crate::Binding!("chosen")),
                    )),
                }),
            ),
        ],
    )
    .with_mode_selection(1, 3, false)]),
);

// WOE 103 — Rat Out
pub(in crate::card::sets) static RAT_OUT: CardRecord = CardRecord::new(
    "Rat Out",
    "f2c42755-bf91-4c75-95c6-d2a60ba3492a",
    "Michele Giorgi",
    CardRules::new_instant(mana_cost!("{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Up to one target creature gets -1/-1 until end of turn. You \
create a 1/1 black Rat creature token with \"This token can't \
block.\"",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            1,
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                DEFENSELESS_RAT_TOKEN,
            ))),
        ]),
    )]),
);

// WOE 104 — Rowan's Grim Search
pub(in crate::card::sets) static ROWAN_S_GRIM_SEARCH: CardRecord = CardRecord::new(
    "Rowan's Grim Search",
    "1be6786e-0569-42dd-b03c-82da7b32a14f",
    "Aurore Folny",
    CardRules::new_instant(mana_cost!("{2}{B}")).with_abilities(&[
        bargain(),
        AbilityDef::spell(
            "If this spell was bargained, look at the top four cards of \
your library, then put up to two of them back on top of your \
library in any order and the rest into your graveyard.\nYou \
draw two cards and you lose 2 life.",
            EffectDef::Sequence(&[
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourcePaidAdditionalCost(
                        AdditionalCostIndex::PRIMARY,
                    ),
                    then: &EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                        source: ObjectCollectionSourceDef::TopCards {
                            player: PlayerRefDef::EffectController,
                            count: ValueDef::Constant(4),
                        },
                        actor: PlayerRefDef::EffectController,
                        inspection: CollectionInspectionDef::Look,
                        object: ObjectPredicateDef::Any,
                        minimum: 0,
                        maximum: 2,
                        chosen: crate::Binding!("kept"),
                        remainder: crate::Binding!("rest"),
                        then: &EffectDef::Sequence(&[
                            EffectDef::MoveObjects(MoveObjectsDef {
                                input: ObjectSetDef::Binding(crate::Binding!("kept")),
                                from: Some(ZoneKind::Library),
                                zone: ZoneKind::Library,
                                placement: ZonePlacement::Top,
                                moved: None,
                                then: &EffectDef::None,
                            }),
                            EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("rest"),
                                )),
                                ZoneKind::Graveyard,
                                ZonePlacement::Top,
                            ),
                        ]),
                    }),
                },
                abilities::draw_cards(ValueDef::Constant(2)),
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
    ]),
);

// WOE 105 — Scream Puff
pub(in crate::card::sets) static SCREAM_PUFF: CardRecord = CardRecord::new(
    "Scream Puff",
    "c62d0ae9-5a82-40bf-b8bb-9c2e2d55d458",
    "Nicholas Gregory",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Horror"], 4, 5).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, \
create a Food token. (It's an artifact with \"{2}, {T}, \
Sacrifice this token: You gain 3 life.\")",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
    ]),
);

// WOE 106 — Shatter the Oath
pub(in crate::card::sets) static SHATTER_THE_OATH: CardRecord = CardRecord::new(
    "Shatter the Oath",
    "cc79f0f7-0a09-4a74-b2b9-cc1ce608d89f",
    "Dominik Mayer",
    CardRules::new_sorcery(mana_cost!("{3}{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target creature or enchantment. Create a Wicked Role \
token attached to up to one target creature you control. (If \
you control another Role on it, put that one into the \
graveyard. Enchanted creature gets +1/+1. When this token is \
put into a graveyard, each opponent loses 1 life.)",
            &[
                AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ])),
                AbilityTargetDef {
                    minimum: 0,
                    ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &const { [ZoneKind::Battlefield] },
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    })
                },
            ],
            EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
                EffectDef::CreateAttachedToken {
                    token: WICKED_ROLE,
                    host: Some(EffectRecipientDef::Target(TargetIndex(1))),
                },
            ]),
        ),
    ]),
);

// WOE 107 — Specter of Mortality
// Audit: unsupported — Needs a reflexive trigger carrying the completed graveyard-exile count even if its source has left the battlefield, with priority between payment and the -X/-X effect.
pub(in crate::card::sets) static SPECTER_OF_MORTALITY: CardRecord = CardRecord::new(
    "Specter of Mortality",
    "5e4c00b5-f6d6-4fbd-828f-ad30321f2cd9",
    "Daarken",
    CardRules::unsupported(),
);

// WOE 108 — Spiteful Hexmage
pub(in crate::card::sets) static SPITEFUL_HEXMAGE: CardRecord = CardRecord::new(
    "Spiteful Hexmage",
    "40c797b2-db51-4a39-b80e-44d58cd7a07c",
    "Anna Steinbauer",
    CardRules::new_creature(mana_cost!("{B}"), &["Human", "Warlock"], 3, 2).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, create a Cursed Role token \
attached to target creature you control. (If you control \
another Role on it, put that one into the graveyard. \
Enchanted creature is 1/1.)",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::CreateAttachedToken {
                token: CURSED_ROLE,
                host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
            },
        ),
    ]),
);

// WOE 109 — Stingblade Assassin
pub(in crate::card::sets) static STINGBLADE_ASSASSIN: CardRecord = CardRecord::new(
    "Stingblade Assassin",
    "b71b7dd9-6a1d-4c71-873b-782a0a2e7d1d",
    "Nicholas Gregory",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Faerie", "Assassin"], 3, 1).with_abilities(&[
        abilities::flash(),
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, destroy target creature an \
opponent controls that was dealt damage this turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::WasDealtDamageThisTurn,
                    ]),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ]),
);

// WOE 110 — Sugar Rush
pub(in crate::card::sets) static SUGAR_RUSH: CardRecord = CardRecord::new(
    "Sugar Rush",
    "a638c9ba-45d6-4b50-a8ab-ef580a0a5d8e",
    "Brent Hollowell",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +3/+0 until end of turn.\nDraw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            abilities::draw_cards(ValueDef::Constant(1)),
        ]),
    )]),
);

// WOE 111 — Sweettooth Witch
pub(in crate::card::sets) static SWEETTOOTH_WITCH: CardRecord = CardRecord::new(
    "Sweettooth Witch",
    "a6bdb984-06f8-4bca-a943-17fe5db97682",
    "Konstantin Porubov",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Warlock"], 3, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a Food token. (It's an \
artifact with \"{2}, {T}, Sacrifice this token: You gain 3 \
life.\")",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
        AbilityDef::activated_with_targets(
            "{2}, Sacrifice a Food: Target player loses 2 life.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::sacrifice_permanent(ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                    "Food",
                ))),
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// WOE 112 — Taken by Nightmares
pub(in crate::card::sets) static TAKEN_BY_NIGHTMARES: CardRecord = CardRecord::new(
    "Taken by Nightmares",
    "f858d83d-a13e-4ffa-a91d-d695e5e5d71a",
    "Artur Treffner",
    CardRules::new_instant(mana_cost!("{2}{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Exile target creature. If you control an enchantment, scry 2.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 1,
                    },
                    then: &abilities::scry(ValueDef::Constant(2)),
                },
            ]),
        ),
    ]),
);

// WOE 113 — Tangled Colony
// Audit: unsupported — Needs total damage dealt to this specific creature during the turn, preserved as last-known information after death; marked damage and player damage history do not provide that total.
pub(in crate::card::sets) static TANGLED_COLONY: CardRecord = CardRecord::new(
    "Tangled Colony",
    "77111ce8-6469-4bf7-882a-4ded1e5d7cad",
    "Filip Burburan",
    CardRules::unsupported(),
);

// WOE 114 — Twisted Sewer-Witch
pub(in crate::card::sets) static TWISTED_SEWER_WITCH: CardRecord = CardRecord::new(
    "Twisted Sewer-Witch",
    "d6e3ddf7-582d-4923-be30-8428e52237e4",
    "Scott Murphy",
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Human", "Warlock"], 3, 4).with_abilities(
        &[abilities::enters_trigger(
            "When this creature enters, create a 1/1 black Rat creature \
token with \"This creature can't block.\" Then for each Rat \
you control, create a Wicked Role token attached to that Rat. \
(If you control another Role on it, put that one into the \
graveyard. Enchanted creature gets +1/+1. When this token is \
put into a graveyard, each opponent loses 1 life.)",
            EffectDef::Sequence(&[
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    DEFENSELESS_RAT_TOKEN,
                ))),
                EffectDef::CreateAttachedToken {
                    token: WICKED_ROLE,
                    host: Some(EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rat")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    ))),
                },
            ]),
        )],
    ),
);

// WOE 115 — Virtue of Persistence // Locthwain Scorn
pub(in crate::card::sets) static VIRTUE_OF_PERSISTENCE: CardRecord = CardRecord::new(
    "Virtue of Persistence // Locthwain Scorn",
    "f1e5cafb-b0e6-4ee5-8c58-6f8e5ef2b9da",
    "Piotr Dura",
    CardRules::new_enchantment(mana_cost!("{5}{B}{B}")).with_abilities(
        &const {
            [AbilityDef::triggered_with_targets(
                "At the beginning of your upkeep, put target creature card \
from a graveyard onto the battlefield under your control.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                &const {
                    [AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &const { [ZoneKind::Graveyard] },
                            controller: None,
                            owner: None,
                        },
                    )]
                },
                EffectDef::WithBattlefieldArrival {
                    effect: &const {
                        EffectDef::move_to_zone(
                            EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            ZoneKind::Battlefield,
                            ZonePlacement::Top,
                        )
                    },
                    arrival: BattlefieldArrivalDef {
                        controller: Some(PlayerRelation::You),
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
            )]
        },
    ),
)
.with_composition(|| {
    adventure(
        &VIRTUE_OF_PERSISTENCE,
        "Locthwain Scorn",
        &CardRules::new_sorcery(mana_cost!("{1}{B}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Target creature gets -3/-3 until end of turn. You gain 2 life.",
                    &const {
                        [AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )]
                    },
                    EffectDef::Sequence(
                        &const {
                            [
                                EffectDef::Apply {
                                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                    effect: AppliedEffectDef::modify_power_toughness(
                                        ValueDef::Constant(-3),
                                        ValueDef::Constant(-3),
                                    ),
                                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                                },
                                EffectDef::GainLife {
                                    recipient: EffectRecipientDef::Controller,
                                    amount: ValueDef::Constant(2),
                                },
                            ]
                        },
                    ),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 116 — Voracious Vermin
pub(in crate::card::sets) static VORACIOUS_VERMIN: CardRecord = CardRecord::new(
    "Voracious Vermin",
    "8059be65-3c73-49bb-a3b6-c346ce2f9fa4",
    "Milivoj Ćeran",
    // The Rat it brings is also the first thing to feed it: a sacrifice
    // outlet turns the token into a counter.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Rat"], 2, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 black Rat creature token with \"This token can't block.\"",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(DEFENSELESS_RAT_TOKEN))),
        ),
        AbilityDef::triggered(
            "Whenever another creature you control dies, put a +1/+1 \
counter on this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// WOE 117 — Warehouse Tabby
pub(in crate::card::sets) static WAREHOUSE_TABBY: CardRecord = CardRecord::new(
    "Warehouse Tabby",
    "d500ad81-9659-4a00-8f99-8be7c23587e8",
    "Steve Prescott",
    CardRules::new_creature(mana_cost!("{B}"), &["Cat"], 1, 1).with_abilities(&[
        AbilityDef::triggered(
            "Whenever an enchantment you control is put into a graveyard \
from the battlefield, create a 1/1 black Rat creature token \
with \"This token can't block.\"",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                DEFENSELESS_RAT_TOKEN,
            ))),
        ),
        AbilityDef::activated(
            "{1}{B}: This creature gains deathtouch until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::deathtouch()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// WOE 118 — Wicked Visitor
pub(in crate::card::sets) static WICKED_VISITOR: CardRecord = CardRecord::new(
    "Wicked Visitor",
    "e26ec4b8-0012-48c4-9ccb-0f062df0c250",
    "Nicholas Gregory",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Nightmare"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever an enchantment you control is put into a graveyard \
from the battlefield, each opponent loses 1 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// WOE 119 — The Witch's Vanity
pub(in crate::card::sets) static THE_WITCH_S_VANITY: CardRecord = CardRecord::new(
    "The Witch's Vanity",
    "47ca4926-b5ac-405a-8b58-f8db6df400ff",
    "Alix Branwyn",
    CardRules::new_enchantment(mana_cost!("{1}{B}"))
        .with_subtypes(&["Saga"])
        .with_abilities(&[
            abilities::saga_chapter_with_targets(
                1,
                "I — Destroy target creature an opponent controls with mana \
value 2 or less.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::ManaValueAtMost(2),
                        ]),
                        zones: &const { [ZoneKind::Battlefield] },
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    },
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
            abilities::saga_chapter(
                2,
                "II — Create a Food token.",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ),
            abilities::saga_chapter_with_targets(
                3,
                "III — Create a Wicked Role token attached to target creature \
you control.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &const { [ZoneKind::Battlefield] },
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::CreateAttachedToken {
                    token: WICKED_ROLE,
                    host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                },
            ),
        ]),
);

// WOE 120 — Belligerent of the Ball
// Audit: unsupported — Needs per-player nonland-permanent entry history for the current turn, including objects that have since left; current battlefield entry timestamps cannot implement celebration.
pub(in crate::card::sets) static BELLIGERENT_OF_THE_BALL: CardRecord = CardRecord::new(
    "Belligerent of the Ball",
    "6658398a-46a5-4f41-9b1b-4a47f2822cf8",
    "Pascal Quidault",
    CardRules::unsupported(),
);

// WOE 121 — Bellowing Bruiser // Beat a Path
pub(in crate::card::sets) static BELLOWING_BRUISER: CardRecord = CardRecord::new(
    "Bellowing Bruiser // Beat a Path",
    "26ece013-f3ef-4c12-9dea-b2789f61f8a0",
    "Kai Carpenter",
    CardRules::new_creature(mana_cost!("{4}{R}"), &const { ["Ogre"] }, 4, 4)
        .with_abilities(&const { [abilities::haste()] }),
)
.with_composition(|| {
    adventure(
        &BELLOWING_BRUISER,
        "Beat a Path",
        &CardRules::new_sorcery(mana_cost!("{2}{R}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Up to two target creatures can't block this turn. (Then exile \
this card. You may cast the creature later from exile.)",
                    &const {
                        [AbilityTargetDef::up_to(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::HasType(CardType::Creature),
                                zones: &const { [ZoneKind::Battlefield] },
                                controller: None,
                                owner: None,
                            },
                            2,
                        )]
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 122 — Bespoke Battlegarb
// Audit: unsupported — Needs per-player nonland-permanent entry history for the current turn, including objects that have since left; current battlefield entry timestamps cannot implement celebration.
pub(in crate::card::sets) static BESPOKE_BATTLEGARB: CardRecord = CardRecord::new(
    "Bespoke Battlegarb",
    "a28ecd59-9166-473c-bafc-cb3c54c21388",
    "Nino Vecia",
    CardRules::unsupported(),
);

// WOE 123 — Boundary Lands Ranger
pub(in crate::card::sets) static BOUNDARY_LANDS_RANGER: CardRecord = CardRecord::new(
    "Boundary Lands Ranger",
    "ccdcbb1d-702e-4832-8eed-7ed3deffefe3",
    "Pascal Quidault",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Ranger"], 2, 2).with_abilities(&[
        AbilityDef::triggered_if(
            "At the beginning of combat on your turn, if you control a \
creature with power 4 or greater, you may discard a card. If \
you do, draw a card.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ObjectCount {
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
            },
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::DiscardCards(1)],
                &abilities::draw_cards(ValueDef::Constant(1)),
            )),
        ),
    ]),
);

// WOE 124 — Charming Scoundrel
pub(in crate::card::sets) static CHARMING_SCOUNDREL: CardRecord = CardRecord::new(
    "Charming Scoundrel",
    "c8090bcf-e17a-4110-a518-77ccd045b18f",
    "Caroline Gariba",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Rogue"], 1, 1).with_abilities(&[
        abilities::haste(),
        AbilityDef::modal_triggered(
            "When this creature enters, choose one —",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[
                AbilityDef::spell(
                    "Discard a card, then draw a card.",
                    EffectDef::Sequence(&[
                        EffectDef::Discard {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(1),
                            selection: DiscardSelectionDef::RecipientChooses,
                            then: None,
                        },
                        abilities::draw_cards(ValueDef::Constant(1)),
                    ]),
                ),
                AbilityDef::spell(
                    "Create a Treasure token.",
                    EffectDef::CreateToken(
                        CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                            .with_count(ValueDef::Constant(1)),
                    ),
                ),
                AbilityDef::spell_with_targets(
                    "Create a Wicked Role token attached to target creature you \
control.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &const { [ZoneKind::Battlefield] },
                            controller: Some(PlayerRelation::You),
                            owner: None,
                        },
                    )],
                    EffectDef::CreateAttachedToken {
                        token: WICKED_ROLE,
                        host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                    },
                ),
            ],
        ),
    ]),
);

// WOE 125 — Cut In
pub(in crate::card::sets) static CUT_IN: CardRecord = CardRecord::new(
    "Cut In",
    "8ea4d40a-7657-4ff8-9fc2-915b99432275",
    "Irina Nordsol",
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Cut In deals 4 damage to target creature.\nCreate a Young \
Hero Role token attached to up to one target creature you \
control. (If you control another Role on it, put that one \
into the graveyard. Enchanted creature has \"Whenever this \
creature attacks, if its toughness is 3 or less, put a +1/+1 \
counter on it.\")",
        &[
            AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Creature,
            )),
            AbilityTargetDef {
                minimum: 0,
                ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::You),
                    owner: None,
                })
            },
        ],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(4),
            ),
            EffectDef::CreateAttachedToken {
                token: YOUNG_HERO_ROLE,
                host: Some(EffectRecipientDef::Target(TargetIndex(1))),
            },
        ]),
    )]),
);

// WOE 126 — Edgewall Pack
pub(in crate::card::sets) static EDGEWALL_PACK: CardRecord = CardRecord::new(
    "Edgewall Pack",
    "acda9d02-00fc-49e2-a9f3-176e9c0a8c5f",
    "Leesha Hannigan",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Dog"], 3, 3).with_abilities(&[
        abilities::menace(),
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 black Rat creature \
token with \"This token can't block.\"",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                DEFENSELESS_RAT_TOKEN,
            ))),
        ),
    ]),
);

// WOE 127 — Embereth Veteran
pub(in crate::card::sets) static EMBERETH_VETERAN: CardRecord = CardRecord::new(
    "Embereth Veteran",
    "bc7130b8-3168-421f-912a-46ed5b769807",
    "Andreia Ugrai",
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Knight"], 2, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}, Sacrifice this creature: Create a Young Hero Role token \
attached to another target creature. (If you control another \
Role on it, put that one into the graveyard. Enchanted \
creature has \"Whenever this creature attacks, if its \
toughness is 3 or less, put a +1/+1 counter on it.\")",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
            )],
            EffectDef::CreateAttachedToken {
                token: YOUNG_HERO_ROLE,
                host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
            },
        ),
    ]),
);

// WOE 128 — Flick a Coin
pub(in crate::card::sets) static FLICK_A_COIN: CardRecord = CardRecord::new(
    "Flick a Coin",
    "673a67b2-fbb0-4be4-9edd-93946a583f23",
    "Andreia Ugrai",
    CardRules::new_instant(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Flick a Coin deals 1 damage to any target. You create a \
Treasure token. (It's an artifact with \"{T}, Sacrifice this \
token: Add one mana of any color.\")\nDraw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
            abilities::draw_cards(ValueDef::Constant(1)),
        ]),
    )]),
);

// WOE 129 — Food Fight
pub(in crate::card::sets) static FOOD_FIGHT: CardRecord = CardRecord::new(
    "Food Fight",
    "1a7cc43c-6e8c-41d2-a885-24604dfc7e7f",
    "Filipe Pagliuso",
    CardRules::new_enchantment(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::static_ability(
        "Artifacts you control have \"{2}, Sacrifice this artifact: It \
deals damage to any target equal to 1 plus the number of \
permanents named Food Fight you control.\"",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Artifact),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ))),
            effect: AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets(
                "{2}, Sacrifice this artifact: It deals damage to any target \
equal to 1 plus the number of permanents named Food Fight you \
control.",
                &[CostDef::Mana(mana_cost!("{2}")), CostDef::SacrificeSource],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Sum(&SumValueDef::new(
                        ValueDef::Constant(1),
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::NameEquals(CardNameDef::Literal("Food Fight")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                    )),
                ),
            )),
        },
    )]),
);

// WOE 130 — Frantic Firebolt
// Audit: unsupported — Needs an object predicate for a card having an Adventure, independently of its currently presented face; the model exposes no Adventure-composition predicate.
pub(in crate::card::sets) static FRANTIC_FIREBOLT: CardRecord = CardRecord::new(
    "Frantic Firebolt",
    "efd85f5a-258b-4ced-bf9e-3abe7fe72395",
    "Olivier Bernard",
    CardRules::unsupported(),
);

// WOE 131 — Gnawing Crescendo
pub(in crate::card::sets) static GNAWING_CRESCENDO: CardRecord = CardRecord::new(
    "Gnawing Crescendo",
    "254fc64a-9734-44a6-8869-ab03512f1a99",
    "Alexey Kruglov",
    // The pump is what wins the combat; the watcher is what stops the
    // opponent from blocking profitably to answer it.
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell(
        "Creatures you control get +2/+0 until end of turn. Whenever a \
nontoken creature you control dies this turn, create a 1/1 \
black Rat creature token with \"This token can't block.\"",
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            // A watcher installed for the rest of the turn rather than a
            // one-shot: every nontoken creature that dies makes its own Rat,
            // and the Rats it makes are excluded from feeding it.
            EffectDef::InstallTrigger(InstalledTriggerDef::this_turn(&AbilityDef::triggered(
                "Whenever a nontoken creature you control dies this turn, \
create a 1/1 black Rat creature token with \"This token can't \
block.\"",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                    ]),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    DEFENSELESS_RAT_TOKEN,
                ))),
            ))),
        ]),
    )),
);

// WOE 132 — Goddric, Cloaked Reveler
// Audit: unsupported — Needs per-player nonland-permanent entry history for the current turn, including objects that have since left; current battlefield entry timestamps cannot implement celebration.
pub(in crate::card::sets) static GODDRIC_CLOAKED_REVELER: CardRecord = CardRecord::new(
    "Goddric, Cloaked Reveler",
    "fe93ef82-51de-40ad-9b52-8f3fd11c144f",
    "Jason A. Engle",
    CardRules::unsupported(),
);

// WOE 133 — Grabby Giant // That's Mine
pub(in crate::card::sets) static GRABBY_GIANT: CardRecord = CardRecord::new(
    "Grabby Giant // That's Mine",
    "fab7646a-61e8-446b-9dba-ac6e0db82f10",
    "Johann Bodin",
    CardRules::new_creature(mana_cost!("{3}{R}"), &const { ["Giant"] }, 4, 3).with_abilities(
        &const {
            [
                abilities::reach(),
                AbilityDef::activated(
                    "{2}{R}, Sacrifice an artifact or land: Draw a card.",
                    &const {
                        [
                            CostDef::Mana(mana_cost!("{2}{R}")),
                            CostDef::sacrifice_permanent(ObjectPredicateDef::AnyOf(
                                &const {
                                    [
                                        ObjectPredicateDef::HasType(CardType::Artifact),
                                        ObjectPredicateDef::HasType(CardType::Land),
                                    ]
                                },
                            )),
                        ]
                    },
                    abilities::draw_cards(ValueDef::Constant(1)),
                ),
            ]
        },
    ),
)
.with_composition(|| {
    adventure(
        &GRABBY_GIANT,
        "That's Mine",
        &CardRules::new_instant(mana_cost!("{1}{R}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell(
                    "Create a Treasure token. (Then exile this card. You may cast \
the creature later from exile.)",
                    EffectDef::CreateToken(
                        CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                            .with_count(ValueDef::Constant(1)),
                    ),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 134 — Grand Ball Guest
// Audit: unsupported — Needs per-player nonland-permanent entry history for the current turn, including objects that have since left; current battlefield entry timestamps cannot implement celebration.
pub(in crate::card::sets) static GRAND_BALL_GUEST: CardRecord = CardRecord::new(
    "Grand Ball Guest",
    "d6e75228-16af-42b0-8441-ed253a660cc9",
    "Leanna Crossan",
    CardRules::unsupported(),
);

// WOE 135 — Harried Spearguard
pub(in crate::card::sets) static HARRIED_SPEARGUARD: CardRecord = CardRecord::new(
    "Harried Spearguard",
    "1db79785-4f55-445f-93f2-14c6e4606fc5",
    "Borja Pindado",
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Soldier"], 1, 1).with_abilities(&[
        abilities::haste(),
        abilities::dies_trigger(
            "When this creature dies, create a 1/1 black Rat creature \
token with \"This token can't block.\"",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                DEFENSELESS_RAT_TOKEN,
            ))),
        ),
    ]),
);

// WOE 136 — Hearth Elemental // Stoke Genius
// Audit: unsupported — Needs an object predicate for a card having an Adventure, independently of its currently presented face; the model exposes no Adventure-composition predicate.
pub(in crate::card::sets) static HEARTH_ELEMENTAL: CardRecord = CardRecord::new(
    "Hearth Elemental // Stoke Genius",
    "a8f5f102-cc75-4cee-a117-4bdaaf86c2e9",
    "Nicholas Gregory",
    CardRules::unsupported(),
);

// WOE 137 — Imodane, the Pyrohammer
// Audit: unsupported — Needs a damage-event matcher that relates the source spell's complete target set to the damaged creature and verifies that it was the spell's sole target.
pub(in crate::card::sets) static IMODANE_THE_PYROHAMMER: CardRecord = CardRecord::new(
    "Imodane, the Pyrohammer",
    "14b44833-0482-4b47-a594-4050bb87f1a5",
    "Chris Rahn",
    CardRules::unsupported(),
);

// WOE 138 — Kindled Heroism
pub(in crate::card::sets) static KINDLED_HEROISM: CardRecord = CardRecord::new(
    "Kindled Heroism",
    "22a6d05e-e566-4a85-bcf8-9d0fbea3dd14",
    "Leanna Crossan",
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +1/+0 and gains first strike until end \
of turn. Scry 1.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::first_strike()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            abilities::scry(ValueDef::Constant(1)),
        ]),
    )]),
);

// WOE 139 — Korvold and the Noble Thief
pub(in crate::card::sets) static KORVOLD_AND_THE_NOBLE_THIEF: CardRecord = CardRecord::new(
    "Korvold and the Noble Thief",
    "a811b2cb-ffc7-4100-ac3e-bc4125842bb2",
    "Ben Hill",
    CardRules::new_enchantment(mana_cost!("{3}{R}"))
        .with_subtypes(&["Saga"])
        .with_abilities(&[
            AbilityDef::triggered(
                "I, II — Create a Treasure token.",
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::While {
                        event: &TriggerEventDef::CountersPlaced {
                            object: ObjectPredicateDef::Source,
                            kind: CounterKind::Lore,
                        },
                        condition: &TriggerConditionDef::SourceCounters {
                            kind: CounterKind::Lore,
                            comparison: ComparisonDef::Equal,
                            amount: 1,
                        },
                    },
                    TriggerEventDef::While {
                        event: &TriggerEventDef::CountersPlaced {
                            object: ObjectPredicateDef::Source,
                            kind: CounterKind::Lore,
                        },
                        condition: &TriggerConditionDef::SourceCounters {
                            kind: CounterKind::Lore,
                            comparison: ComparisonDef::Equal,
                            amount: 2,
                        },
                    },
                ]),
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ),
            abilities::saga_chapter_with_targets(
                3,
                "III — Exile the top three cards of target opponent's library. \
You may play those cards this turn.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::BindObjects(BindObjectsDef {
                    source: ObjectCollectionSourceDef::TopCards {
                        player: PlayerRefDef::Target(TargetIndex::PRIMARY),
                        count: ValueDef::Constant(3),
                    },
                    binding: crate::Binding!("top"),
                    then: &EffectDef::ExileGrantingControllerPlayThisTurn {
                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                            crate::Binding!("top"),
                        )),
                    },
                }),
            ),
        ]),
);

// WOE 140 — Merry Bards
// Audit: unsupported — Needs a reflexive trigger with targets or modes chosen after its preceding payment or event, retained even if the source has left the battlefield; choosing them with the original ability changes response timing and target legality.
pub(in crate::card::sets) static MERRY_BARDS: CardRecord = CardRecord::new(
    "Merry Bards",
    "b0058b9b-e919-45eb-9da0-690f62aa252e",
    "Iris Compiet",
    CardRules::unsupported(),
);

// WOE 141 — Minecart Daredevil // Ride the Rails
pub(in crate::card::sets) static MINECART_DAREDEVIL: CardRecord = CardRecord::new(
    "Minecart Daredevil // Ride the Rails",
    "5b2a02f3-3921-4f40-9ffa-70bc08b052e1",
    "Edgar Sánchez Hidalgo",
    CardRules::new_creature(mana_cost!("{2}{R}"), &const { ["Dwarf", "Knight"] }, 4, 2),
)
.with_composition(|| {
    adventure(
        &MINECART_DAREDEVIL,
        "Ride the Rails",
        &CardRules::new_instant(mana_cost!("{1}{R}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Target creature gets +2/+1 until end of turn. (Then exile \
this card. You may cast the creature later from exile.)",
                    &const {
                        [AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )]
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(1),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 142 — Monstrous Rage
pub(in crate::card::sets) static MONSTROUS_RAGE: CardRecord = CardRecord::new(
    "Monstrous Rage",
    "eef5a0ae-5907-42c9-a097-3f973737e392",
    "Borja Pindado",
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +2/+0 until end of turn. Create a \
Monster Role token attached to it. (If you control another \
Role on it, put that one into the graveyard. Enchanted \
creature gets +1/+1 and has trample.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
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
            EffectDef::CreateAttachedToken {
                token: MONSTER_ROLE,
                host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
            },
        ]),
    )),
);

// WOE 143 — Raging Battle Mouse
// Audit: unsupported — Needs per-player nonland-permanent entry history for the current turn, including objects that have since left; current battlefield entry timestamps cannot implement celebration.
pub(in crate::card::sets) static RAGING_BATTLE_MOUSE: CardRecord = CardRecord::new(
    "Raging Battle Mouse",
    "4d0ab162-540e-4999-902c-9dacd6687aca",
    "Rudy Siswanto",
    CardRules::unsupported(),
);

// WOE 144 — Ratcatcher Trainee // Pest Problem
pub(in crate::card::sets) static RATCATCHER_TRAINEE: CardRecord = CardRecord::new(
    "Ratcatcher Trainee // Pest Problem",
    "7f4c0959-a107-4d61-9e51-256b2955f6ba",
    "Michele Giorgi",
    CardRules::new_creature(mana_cost!("{1}{R}"),
&const {
[
"Human",
"Peasant"]}
,
2,
1).with_abilities(&const {
[
AbilityDef::static_ability("During your turn, this creature has first strike.",
EffectDef::IfCondition {
condition:&const {
TriggerConditionDef::ActivePlayer(PlayerRelation::You)}
,
then:&const {
EffectDef::StaticApply {
recipient:EffectRecipientDef::Source,
effect:AppliedEffectDef::add_ability(&const {
abilities::first_strike()}
)}
}
}
)]}
),

).with_composition(|| adventure(&RATCATCHER_TRAINEE,
"Pest Problem",
&CardRules::new_instant(mana_cost!("{2}{R}")).with_subtypes(&const {
[
"Adventure"]}
).with_ability(AbilityDef::spell("Create two 1/1 black Rat creature tokens with \"This token \
can't block.\" (Then exile this card. You may cast the \
creature later from exile.)",
EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(DEFENSELESS_RAT_TOKEN)).with_count(ValueDef::Constant(2)))).with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure))));

// WOE 145 — Realm-Scorcher Hellkite
pub(in crate::card::sets) static REALM_SCORCHER_HELLKITE: CardRecord = CardRecord::new(
    "Realm-Scorcher Hellkite",
    "845b3c26-05da-4a09-a6c9-4ea4166104a7",
    "Billy Christian",
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Dragon"], 4, 6).with_abilities(&[
        bargain(),
        abilities::flying(),
        abilities::haste(),
        AbilityDef::triggered_if(
            "When this creature enters, if it was bargained, add four mana \
in any combination of colors.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::SourcePaidAdditionalCost(AdditionalCostIndex::PRIMARY),
            EffectDef::AddMana(AddManaEffectDef::combination(
                &[
                    ManaColor::White,
                    ManaColor::Blue,
                    ManaColor::Black,
                    ManaColor::Red,
                    ManaColor::Green,
                ],
                4,
            )),
        ),
        AbilityDef::activated_with_targets(
            "{1}{R}: This creature deals 1 damage to any target.",
            &[CostDef::Mana(mana_cost!("{1}{R}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ]),
);

// WOE 146 — Redcap Gutter-Dweller
pub(in crate::card::sets) static REDCAP_GUTTER_DWELLER: CardRecord = CardRecord::new(
    "Redcap Gutter-Dweller",
    "96bcd5f0-da79-47ab-83cf-976198b458d1",
    "Alexey Kruglov",
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Goblin", "Warrior"], 3, 3).with_abilities(
        &[
            abilities::menace(),
            abilities::enters_trigger(
                "When this creature enters, create two 1/1 black Rat creature \
tokens with \"This token can't block.\"",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(DEFENSELESS_RAT_TOKEN))
                        .with_count(ValueDef::Constant(2)),
                ),
            ),
            AbilityDef::triggered(
                "At the beginning of your upkeep, you may sacrifice another \
creature. If you do, put a +1/+1 counter on this creature and \
exile the top card of your library. You may play that card \
this turn.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::PayOr(PayOrDef::optional(
                    &[CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]))],
                    &EffectDef::Sequence(&[
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::Source,
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                        EffectDef::BindObjects(BindObjectsDef {
                            source: ObjectCollectionSourceDef::TopCards {
                                player: PlayerRefDef::EffectController,
                                count: ValueDef::Constant(1),
                            },
                            binding: crate::Binding!("top"),
                            then: &EffectDef::ExileGrantingControllerPlayThisTurn {
                                object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("top"),
                                )),
                            },
                        }),
                    ]),
                )),
            ),
        ],
    ),
);

// WOE 147 — Redcap Thief
pub(in crate::card::sets) static REDCAP_THIEF: CardRecord = CardRecord::new(
    "Redcap Thief",
    "cda6bdeb-a0d6-46ca-ba8c-317ee0096416",
    "Vincent Christiaens",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Rogue"], 2, 3).with_abilities(&[
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

// WOE 148 — Rotisserie Elemental
pub(in crate::card::sets) static ROTISSERIE_ELEMENTAL: CardRecord = CardRecord::new(
    "Rotisserie Elemental",
    "8d787045-3918-4ed6-85ea-843d1f2356f2",
    "Leonardo Santanna",
    CardRules::new_creature(mana_cost!("{R}"), &["Elemental"], 1, 1).with_abilities(&[
        abilities::menace(),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, put a \
skewer counter on this creature. Then you may sacrifice it. \
If you do, exile the top X cards of your library, where X is \
the number of skewer counters on this creature. You may play \
those cards this turn.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("skewer"),
                    amount: ValueDef::Constant(1),
                },
                EffectDef::PayOr(PayOrDef::optional(
                    &[CostDef::sacrifice_permanent(ObjectPredicateDef::Source)],
                    &EffectDef::BindObjects(BindObjectsDef {
                        source: ObjectCollectionSourceDef::TopCards {
                            player: PlayerRefDef::EffectController,
                            count: ValueDef::CountersOnSource(CounterKind::named("skewer")),
                        },
                        binding: crate::Binding!("top"),
                        then: &EffectDef::ExileGrantingControllerPlayThisTurn {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("top"),
                            )),
                        },
                    }),
                )),
            ]),
        ),
    ]),
);

// WOE 149 — Skewer Slinger
pub(in crate::card::sets) static SKEWER_SLINGER: CardRecord = CardRecord::new(
    "Skewer Slinger",
    "e78e50ca-d27b-45db-91fa-7fef3cad16d0",
    "Edgar Sánchez Hidalgo",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Dwarf", "Knight"], 1, 3).with_abilities(&[
        abilities::reach(),
        AbilityDef::triggered(
            "Whenever this creature blocks or becomes blocked by a \
creature, this creature deals 1 damage to that creature.",
            TriggerEventDef::BlocksOrBecomesBlockedBy {
                creature: ObjectPredicateDef::Source,
                other: ObjectPredicateDef::HasType(CardType::Creature),
            },
            EffectDef::damage(EffectRecipientDef::TriggeringObject, ValueDef::Constant(1)),
        ),
    ]),
);

// WOE 150 — Song of Totentanz
pub(in crate::card::sets) static SONG_OF_TOTENTANZ: CardRecord = CardRecord::new(
    "Song of Totentanz",
    "940e8bb7-0251-4fac-945c-d83618c10447",
    "Randy Gallegos",
    CardRules::new_sorcery(mana_cost!("{X}{R}")).with_abilities(&[AbilityDef::spell(
        "Create X 1/1 black Rat creature tokens with \"This token \
can't block.\" Creatures you control gain haste until end of \
turn.",
        EffectDef::Sequence(&[
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(DEFENSELESS_RAT_TOKEN))
                    .with_count(ValueDef::ChosenX),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )]),
);

// WOE 151 — Stonesplitter Bolt
pub(in crate::card::sets) static STONESPLITTER_BOLT: CardRecord = CardRecord::new(
    "Stonesplitter Bolt",
    "fb22f79c-3075-439d-a072-ceaabe35d76f",
    "Alexandr Leskinen",
    CardRules::new_instant(mana_cost!("{X}{R}")).with_abilities(&[
        bargain(),
        AbilityDef::spell_with_targets(
            "Stonesplitter Bolt deals X damage to target creature or \
planeswalker. If this spell was bargained, it deals twice X \
damage to that permanent instead.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::IfAdditionalCostPaid(&AdditionalCostValueDef::new(
                    AdditionalCostIndex::PRIMARY,
                    ValueDef::Scaled(&ScaledValueDef::new(ValueDef::ChosenX, 2)),
                    ValueDef::ChosenX,
                )),
            ),
        ),
    ]),
);

// WOE 152 — Tattered Ratter
pub(in crate::card::sets) static TATTERED_RATTER: CardRecord = CardRecord::new(
    "Tattered Ratter",
    "30f505b4-d61c-4da8-ab45-37125260d556",
    "Tyler Walpole",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Peasant"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever a Rat you control becomes blocked, it gets +2/+0 \
until end of turn.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::All(&[
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rat")),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Apply {
                recipient: EffectRecipientDef::TriggeringObject,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// WOE 153 — Torch the Tower
pub(in crate::card::sets) static TORCH_THE_TOWER: CardRecord = CardRecord::new(
    "Torch the Tower",
    "b3d6027c-813f-46df-95b4-e2e305a67620",
    "Uriah Voth",
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[
        bargain(),
        AbilityDef::spell_with_targets(
            "Torch the Tower deals 2 damage to target creature or \
planeswalker. If this spell was bargained, instead it deals 3 \
damage to that permanent and you scry 1.\nIf a permanent \
dealt damage by Torch the Tower would die this turn, exile it \
instead.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
            )],
            EffectDef::Sequence(&[
                EffectDef::DealDamage(
                    DamageDef::new(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::IfAdditionalCostPaid(&AdditionalCostValueDef::new(
                            AdditionalCostIndex::PRIMARY,
                            ValueDef::Constant(3),
                            ValueDef::Constant(2),
                        )),
                    )
                    .with_follow_up(DamageFollowUpDef::ApplyToDamaged {
                        effect: &AppliedEffectDef::Rule(AppliedRuleDef::ExileInsteadOfDying),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    }),
                ),
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourcePaidAdditionalCost(
                        AdditionalCostIndex::PRIMARY,
                    ),
                    then: &abilities::scry(ValueDef::Constant(1)),
                },
            ]),
        ),
    ]),
);

// WOE 154 — Twisted Fealty
pub(in crate::card::sets) static TWISTED_FEALTY: CardRecord = CardRecord::new(
    "Twisted Fealty",
    "382d6085-79b9-48f7-8949-9f44dde2c753",
    "Mila Pesic",
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Gain control of target creature until end of turn. Untap that \
creature. It gains haste until end of turn.\nCreate a Wicked \
Role token attached to up to one target creature. (If you \
control another Role on it, put that one into the graveyard. \
Enchanted creature gets +1/+1. When this token is put into a \
graveyard, each opponent loses 1 life.)",
        &[
            AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Creature,
            )),
            AbilityTargetDef {
                minimum: 0,
                ..AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                ))
            },
        ],
        EffectDef::Sequence(&[
            EffectDef::Perform(GameActionDef::GainControl {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                controller: PlayerRefDef::EffectController,
                duration: ControlDurationDef::UntilEndOfTurn,
            }),
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::CreateAttachedToken {
                token: WICKED_ROLE,
                host: Some(EffectRecipientDef::Target(TargetIndex(1))),
            },
        ]),
    )]),
);

// WOE 155 — Two-Headed Hunter // Twice the Rage
pub(in crate::card::sets) static TWO_HEADED_HUNTER: CardRecord = CardRecord::new(
    "Two-Headed Hunter // Twice the Rage",
    "70c12e75-7e65-4706-b976-e47835910928",
    "Filip Burburan",
    CardRules::new_creature(mana_cost!("{4}{R}"), &const { ["Giant"] }, 5, 4)
        .with_abilities(&const { [abilities::menace()] }),
)
.with_composition(|| {
    adventure(
        &TWO_HEADED_HUNTER,
        "Twice the Rage",
        &CardRules::new_instant(mana_cost!("{1}{R}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Target creature gains double strike until end of turn. (Then \
exile this card. You may cast the creature later from exile.)",
                    &const {
                        [AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )]
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_ability(
                            &const { abilities::double_strike() },
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 156 — Unruly Catapult
pub(in crate::card::sets) static UNRULY_CATAPULT: CardRecord = CardRecord::new(
    "Unruly Catapult",
    "3dc275f8-f060-4c15-9b8f-63cf3857eaa7",
    "Vincent Christiaens",
    CardRules::new_artifact_creature(mana_cost!("{2}{R}"), &["Construct"], 0, 4).with_abilities(&[
        abilities::defender(),
        AbilityDef::activated(
            "{T}: This creature deals 1 damage to each opponent.",
            &[CostDef::TapSource],
            EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)),
        ),
        AbilityDef::triggered(
            "Whenever you cast an instant or sorcery spell, untap this \
creature.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// WOE 157 — Virtue of Courage // Embereth Blaze
// Audit: unsupported — Needs a noncombat-only damage-event matcher; the current event vocabulary selects all damage or combat damage, without a noncombat exclusion.
pub(in crate::card::sets) static VIRTUE_OF_COURAGE: CardRecord = CardRecord::new(
    "Virtue of Courage // Embereth Blaze",
    "8b0e6daf-0dec-4718-af79-b7ce137c3135",
    "Piotr Dura",
    CardRules::unsupported(),
);

// WOE 158 — Witch's Mark
pub(in crate::card::sets) static WITCH_S_MARK: CardRecord = CardRecord::new(
    "Witch's Mark",
    "0685afcb-06f6-4d18-b8c2-510764558dc1",
    "Justyna Dura",
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "You may discard a card. If you do, draw two cards.\nCreate a \
Wicked Role token attached to up to one target creature you \
control. (If you control another Role on it, put that one \
into the graveyard. Enchanted creature gets +1/+1. When this \
token is put into a graveyard, each opponent loses 1 life.)",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
            1,
        )],
        EffectDef::Sequence(&[
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::DiscardCards(1)],
                &abilities::draw_cards(ValueDef::Constant(2)),
            )),
            EffectDef::CreateAttachedToken {
                token: WICKED_ROLE,
                host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
            },
        ]),
    )]),
);

// WOE 159 — Witchstalker Frenzy
// Audit: unsupported — Needs the number of creatures that attacked this turn, including creatures that have since left the battlefield; live attacking or attacked-this-turn object queries lose those objects.
pub(in crate::card::sets) static WITCHSTALKER_FRENZY: CardRecord = CardRecord::new(
    "Witchstalker Frenzy",
    "649025a7-79d1-4d7c-b1db-d46bcf5a1ae2",
    "Pascal Quidault",
    CardRules::unsupported(),
);

// WOE 160 — Agatha's Champion
pub(in crate::card::sets) static AGATHA_S_CHAMPION: CardRecord = CardRecord::new(
    "Agatha's Champion",
    "92652c41-1239-4299-a486-11fe1a96e912",
    "Jason A. Engle",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Human", "Knight"], 4, 4).with_abilities(&[
        bargain(),
        abilities::trample(),
        AbilityDef::triggered_if_with_targets(
            "When this creature enters, if it was bargained, it fights up \
to one target creature you don't control. (Each deals damage \
equal to its power to the other.)",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::SourcePaidAdditionalCost(AdditionalCostIndex::PRIMARY),
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
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
    ]),
);

// WOE 161 — Beanstalk Wurm // Plant Beans
// Audit: unsupported — Needs a resolving effect to grant an additional land play for this turn; the shared runtime only supports the corresponding continuous static permission.
pub(in crate::card::sets) static BEANSTALK_WURM: CardRecord = CardRecord::new(
    "Beanstalk Wurm // Plant Beans",
    "19f20c0a-22be-4a9c-96ce-4047f7a2d424",
    "Aldo Domínguez",
    CardRules::unsupported(),
);

// WOE 162 — Bestial Bloodline
pub(in crate::card::sets) static BESTIAL_BLOODLINE: CardRecord = CardRecord::new(
    "Bestial Bloodline",
    "55b9b8e5-1ed8-4be0-aad5-041a599c6841",
    "Mila Pesic",
    CardRules::new_enchantment(mana_cost!("{1}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                },
            ),
            AbilityDef::activated(
                "{4}{G}: Return this card from your graveyard to your hand.",
                &[CostDef::Mana(mana_cost!("{4}{G}"))],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            )
            .with_source_zones(&[ZoneKind::Graveyard]),
        ]),
);

// WOE 163 — Blossoming Tortoise
pub(in crate::card::sets) static BLOSSOMING_TORTOISE: CardRecord = CardRecord::new(
    "Blossoming Tortoise",
    "7811a45d-6bfb-4c2a-b5a2-cccbd8cff186",
    "Simon Dominic",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Turtle"], 3, 3).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature enters or attacks, mill three cards, \
then return a land card from your graveyard to the \
battlefield tapped.",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
            ]),
            EffectDef::Sequence(&[
                EffectDef::Mill {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("land")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::WithBattlefieldArrival {
                        effect: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "land"
                            ))),
                            ZoneKind::Battlefield,
                            ZonePlacement::Top,
                        ),
                        arrival: BattlefieldArrivalDef {
                            modifications: &[BattlefieldEntryModificationDef::Tapped],
                            ..BattlefieldArrivalDef::DEFAULT
                        },
                    },
                }),
            ]),
        ),
        AbilityDef::static_ability(
            "Activated abilities of lands you control cost {1} less to \
activate.",
            EffectDef::ModifyCost(CostModificationDef::AbilityReduction {
                permanent: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                amount: ValueDef::Constant(1),
                minimum: 0,
            }),
        ),
        AbilityDef::static_ability(
            "Land creatures you control get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Land),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
            },
        ),
    ]),
);

// WOE 164 — Bramble Familiar // Fetch Quest
pub(in crate::card::sets) static BRAMBLE_FAMILIAR: CardRecord = CardRecord::new(
    "Bramble Familiar // Fetch Quest",
    "475d7e9a-759d-4523-a5cd-2a6e0d1b14ea",
    "Simon Dominic",
    CardRules::new_creature(
        mana_cost!("{1}{G}"),
        &const { ["Elemental", "Raccoon"] },
        2,
        2,
    )
    .with_abilities(
        &const {
            [
                abilities::tap_for(ManaColor::Green),
                AbilityDef::activated(
                    "{1}{G}, {T}, Discard a card: Return this creature to its \
owner's hand.",
                    &const {
                        [
                            CostDef::Mana(mana_cost!("{1}{G}")),
                            CostDef::TapSource,
                            CostDef::discard(ObjectPredicateDef::Any),
                        ]
                    },
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Source,
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                ),
            ]
        },
    ),
)
.with_composition(|| {
    adventure(
        &BRAMBLE_FAMILIAR,
        "Fetch Quest",
        &CardRules::new_sorcery(mana_cost!("{5}{G}{G}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell(
                    "Mill seven cards. Then put a creature, enchantment, or land \
card from among the milled cards onto the battlefield.",
                    EffectDef::Sequence(
                        &const {
                            [
                                EffectDef::BindOutput {
                                    binding: crate::Binding!("milled"),
                                    effect: &const {
                                        EffectDef::Mill {
                                            player: EffectRecipientDef::Controller,
                                            amount: ValueDef::Constant(7),
                                        }
                                    },
                                },
                                EffectDef::Choose(ChooseDef {
                                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!(
                                        "chosen"
                                    )),
                                    unchosen: None,
                                    chooser: PlayerRefDef::EffectController,
                                    candidates: ObjectSetDef::Matching {
                                        objects: &const {
                                            ObjectSetDef::Binding(crate::Binding!("milled"))
                                        },
                                        object: ObjectSetFilterDef::Predicate(
                                            &const {
                                                ObjectPredicateDef::AnyOf(
                                                    &const {
                                                        [
                                                            ObjectPredicateDef::HasType(
                                                                CardType::Creature,
                                                            ),
                                                            ObjectPredicateDef::HasType(
                                                                CardType::Enchantment,
                                                            ),
                                                            ObjectPredicateDef::HasType(
                                                                CardType::Land,
                                                            ),
                                                        ]
                                                    },
                                                )
                                            },
                                        ),
                                    },
                                    exclude: None,
                                    minimum: 1,
                                    maximum: 1,
                                    visibility: ChoiceVisibilityDef::Public,
                                    then: &const {
                                        EffectDef::move_to_zone(
                                            EffectRecipientDef::objects(ObjectSetDef::Binding(
                                                crate::Binding!("chosen"),
                                            )),
                                            ZoneKind::Battlefield,
                                            ZonePlacement::Top,
                                        )
                                    },
                                }),
                            ]
                        },
                    ),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 165 — Brave the Wilds
pub(in crate::card::sets) static BRAVE_THE_WILDS: CardRecord = CardRecord::new(
    "Brave the Wilds",
    "821b9e86-d108-42e5-b642-c5e07ab16c37",
    "Lucas Graciano",
    CardRules::new_sorcery(mana_cost!("{G}")).with_abilities(&[
        bargain(),
        AbilityDef::spell_with_targets(
            "If this spell was bargained, target land you control becomes \
a 3/3 Elemental creature with haste that's still a \
land.\nSearch your library for a basic land card, reveal it, \
put it into your hand, then shuffle.",
            &[AbilityTargetDef {
                exact_count: Some(ValueDef::IfAdditionalCostPaid(
                    &AdditionalCostValueDef::new(
                        AdditionalCostIndex::PRIMARY,
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                )),
                ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::You),
                    owner: None,
                })
            }],
            EffectDef::Sequence(&[
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourcePaidAdditionalCost(
                        AdditionalCostIndex::PRIMARY,
                    ),
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::add_card_types(CardTypeSet::single(
                                CardType::Creature,
                            )),
                            AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                                "Elemental",
                            ])),
                            AppliedEffectDef::set_base_power_toughness(
                                ValueDef::Constant(3),
                                ValueDef::Constant(3),
                            ),
                            AppliedEffectDef::add_ability(&abilities::haste()),
                        ]),
                        duration: ResolvedEffectDurationDef::Permanent,
                    },
                },
                EffectDef::SearchZone {
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
            ]),
        ),
    ]),
);

// WOE 166 — Commune with Nature (reprint)
const COMMUNE_WITH_NATURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_chk::COMMUNE_WITH_NATURE,
    "5224eec3-2941-4d16-a713-099e34e93eee",
    "Jodie Muir",
);

// WOE 167 — Curse of the Werefox
// Audit: unsupported — Needs a reflexive trigger with targets or modes chosen after its preceding payment or event, retained even if the source has left the battlefield; choosing them with the original ability changes response timing and target legality.
pub(in crate::card::sets) static CURSE_OF_THE_WEREFOX: CardRecord = CardRecord::new(
    "Curse of the Werefox",
    "89148458-1fd6-48ef-a2d9-7b434c9723ec",
    "Andrew Mar",
    CardRules::unsupported(),
);

// WOE 168 — Elvish Archivist
pub(in crate::card::sets) static ELVISH_ARCHIVIST: CardRecord = CardRecord::new(
    "Elvish Archivist",
    "0670dbf0-b150-4e8d-bb40-f768b2f06fe5",
    "Mila Pesic",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Artificer"], 0, 1).with_abilities(&[
        AbilityDef::triggered(
            "Whenever one or more artifacts you control enter, put two \
+1/+1 counters on this creature. This ability triggers only \
once each turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(2),
            },
        )
        .triggering_at_most(1),
        AbilityDef::triggered(
            "Whenever one or more enchantments you control enter, draw a \
card. This ability triggers only once each turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            abilities::draw_cards(ValueDef::Constant(1)),
        )
        .triggering_at_most(1),
    ]),
);

// WOE 169 — Feral Encounter
// Audit: unsupported — Needs a delayed trigger that fires once at the next combat this turn and expires if none occurs, plus a bound creature-card exile cast permission; available installed-trigger lifetimes cannot combine once-only firing with end-of-turn expiry.
pub(in crate::card::sets) static FERAL_ENCOUNTER: CardRecord = CardRecord::new(
    "Feral Encounter",
    "de21251f-40bf-4f7e-9e85-9033207a788f",
    "Fajareka Setiawan",
    CardRules::unsupported(),
);

// WOE 170 — Ferocious Werefox // Guard Change
pub(in crate::card::sets) static FEROCIOUS_WEREFOX: CardRecord = CardRecord::new(
    "Ferocious Werefox // Guard Change",
    "ac1907e8-0713-47dd-ac42-bf1323c5bec0",
    "Caroline Gariba",
    CardRules::new_creature(
        mana_cost!("{3}{G}"),
        &const { ["Elf", "Fox", "Warrior"] },
        4,
        3,
    )
    .with_abilities(&const { [abilities::trample()] }),
)
.with_composition(|| {
    adventure(
        &FEROCIOUS_WEREFOX,
        "Guard Change",
        &CardRules::new_instant(mana_cost!("{1}{G}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Create a Monster Role token attached to target creature you \
control. (Enchanted creature gets +1/+1 and has trample.)",
                    &const {
                        [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::HasType(CardType::Creature),
                                zones: &const { [ZoneKind::Battlefield] },
                                controller: Some(PlayerRelation::You),
                                owner: None,
                            },
                        )]
                    },
                    EffectDef::CreateAttachedToken {
                        token: MONSTER_ROLE,
                        host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                    },
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 171 — Graceful Takedown
// Audit: unsupported — Needs a simultaneous damage instruction whose source assignments are built from a variable-size target group, with each source's power frozen for the batch; iterating damage effects would be sequential.
pub(in crate::card::sets) static GRACEFUL_TAKEDOWN: CardRecord = CardRecord::new(
    "Graceful Takedown",
    "83edf626-ed34-417f-818d-597ecf439167",
    "Sidharth Chaturvedi",
    CardRules::unsupported(),
);

// WOE 172 — Gruff Triplets
pub(in crate::card::sets) static GRUFF_TRIPLETS: CardRecord = CardRecord::new(
    "Gruff Triplets",
    "f8760ab9-ac76-4e2e-b82f-0ee2a6dc5634",
    "Fajareka Setiawan",
    CardRules::new_creature(mana_cost!("{3}{G}{G}{G}"), &["Satyr", "Warrior"], 3, 3)
        .with_abilities(&[
            abilities::trample(),
            AbilityDef::triggered_if(
                "When this creature enters, if it isn't a token, create two \
tokens that are copies of it.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                },
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Copy(&TokenCopyDef {
                        object: &EffectRecipientDef::Source,
                        exceptions: CopyExceptionsDef::NONE,
                    }))
                    .with_count(ValueDef::Constant(2)),
                ),
            ),
            abilities::dies_trigger(
                "When this creature dies, put a number of +1/+1 counters equal \
to its power on each creature you control named Gruff \
Triplets.",
                EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::NameEquals(CardNameDef::Literal(
                                    "Gruff Triplets",
                                )),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::SourcePower,
                },
            ),
        ]),
);

// WOE 173 — Hamlet Glutton
// Audit: unsupported — Needs the source spell's generic cost reduction to depend on its selected bargain payment; the self-cost planner does not accept additional-cost payment values.
pub(in crate::card::sets) static HAMLET_GLUTTON: CardRecord = CardRecord::new(
    "Hamlet Glutton",
    "a4ec5544-c138-44bb-a807-5798313c9a50",
    "Edgar Sánchez Hidalgo",
    CardRules::unsupported(),
);

// WOE 174 — Hollow Scavenger // Bakery Raid
pub(in crate::card::sets) static HOLLOW_SCAVENGER: CardRecord = CardRecord::new(
    "Hollow Scavenger // Bakery Raid",
    "0ad345b6-7077-4dd2-b515-c774a3185fe4",
    "Michele Giorgi",
    CardRules::new_creature(mana_cost!("{2}{G}"), &const { ["Wolf"] }, 3, 2).with_abilities(
        &const {
            [AbilityDef::activated(
                "{1}, Sacrifice a Food: This creature gets +2/+2 until end of \
turn. Activate only once each turn.",
                &const {
                    [
                        CostDef::Mana(mana_cost!("{1}")),
                        CostDef::sacrifice_permanent(ObjectPredicateDef::Subtype(
                            SubtypeDef::Literal("Food"),
                        )),
                    ]
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            )
            .once_each_turn()]
        },
    ),
)
.with_composition(|| {
    adventure(
        &HOLLOW_SCAVENGER,
        "Bakery Raid",
        &CardRules::new_sorcery(mana_cost!("{G}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell(
                    "Create a Food token. (Then exile this card. You may cast the \
creature later from exile.)",
                    EffectDef::CreateToken(
                        CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                            .with_count(ValueDef::Constant(1)),
                    ),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 175 — Howling Galefang
// Audit: unsupported — Needs an object predicate for a card having an Adventure, independently of its currently presented face; the model exposes no Adventure-composition predicate.
pub(in crate::card::sets) static HOWLING_GALEFANG: CardRecord = CardRecord::new(
    "Howling Galefang",
    "86311523-d0eb-4db3-b586-8349de9c2d37",
    "Néstor Ossandón Leal",
    CardRules::unsupported(),
);

// WOE 176 — The Huntsman's Redemption
pub(in crate::card::sets) static THE_HUNTSMAN_S_REDEMPTION: CardRecord = CardRecord::new(
    "The Huntsman's Redemption",
    "27003577-e276-4ad5-b3e9-8523b166ad49",
    "Magali Villeneuve",
    CardRules::new_enchantment(mana_cost!("{2}{G}"))
        .with_subtypes(&["Saga"])
        .with_abilities(&[
            abilities::saga_chapter(
                1,
                "I — Create a 3/3 green Beast creature token.",
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Beast"], &[ManaColor::Green], 3, 3),
                ))),
            ),
            abilities::saga_chapter(
                2,
                "II — You may sacrifice a creature. If you do, search your \
library for a creature or basic land card, reveal it, put it \
into your hand, then shuffle.",
                EffectDef::PayOr(PayOrDef::optional(
                    &[CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(
                        CardType::Creature,
                    ))],
                    &EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::Supertype(CardSupertype::Basic),
                            ]),
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
                )),
            ),
            abilities::saga_chapter_with_targets(
                3,
                "III — Up to two target creatures each get +2/+2 and gain \
trample until end of turn.",
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    2,
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// WOE 177 — Leaping Ambush
pub(in crate::card::sets) static LEAPING_AMBUSH: CardRecord = CardRecord::new(
    "Leaping Ambush",
    "2785f716-274c-495c-b4e3-71a72e22f856",
    "Vincent Christiaens",
    CardRules::new_instant(mana_cost!("{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +1/+3 and gains reach until end of turn. \
Untap it.",
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
                    AppliedEffectDef::add_ability(&abilities::reach()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ]),
    )]),
);

// WOE 178 — Night of the Sweets' Revenge
pub(in crate::card::sets) static NIGHT_OF_THE_SWEETS_REVENGE: CardRecord = CardRecord::new(
    "Night of the Sweets' Revenge",
    "27f53bed-7075-4303-aa9e-fcca0a266e19",
    "Leonardo Santanna",
    CardRules::new_enchantment(mana_cost!("{3}{G}")).with_abilities(&[
        abilities::enters_trigger(
            "When this enchantment enters, create a Food token. (It's an \
artifact with \"{2}, {T}, Sacrifice this token: You gain 3 \
life.\")",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
        AbilityDef::static_ability(
            "Foods you control have \"{T}: Add {G}.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Food")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&abilities::tap_for(ManaColor::Green)),
            },
        ),
        AbilityDef::activated(
            "{5}{G}{G}, Sacrifice this enchantment: Creatures you control \
get +X/+X until end of turn, where X is the number of Foods \
you control. Activate only as a sorcery.",
            &[
                CostDef::Mana(mana_cost!("{5}{G}{G}")),
                CostDef::SacrificeSource,
            ],
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Food")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Food")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// WOE 179 — Redtooth Genealogist
pub(in crate::card::sets) static REDTOOTH_GENEALOGIST: CardRecord = CardRecord::new(
    "Redtooth Genealogist",
    "99c81440-66eb-4443-a83f-e2f15cb68a3e",
    "Gaboleps",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Advisor"], 2, 3).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, create a Royal Role token attached \
to another target creature you control. (If you control \
another Role on it, put that one into the graveyard. \
Enchanted creature gets +1/+1 and has ward {1}.)",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::CreateAttachedToken {
                token: ROYAL_ROLE,
                host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
            },
        ),
    ]),
);

// WOE 180 — Redtooth Vanguard
pub(in crate::card::sets) static REDTOOTH_VANGUARD: CardRecord = CardRecord::new(
    "Redtooth Vanguard",
    "55271960-b9bd-4bea-93ca-3321bf30be78",
    "Joshua Cairos",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Warrior"], 3, 1).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "Whenever an enchantment you control enters, you may pay {2}. \
If you do, return this card from your graveyard to your hand.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::Mana(mana_cost!("{2}"))],
                &EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            )),
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// WOE 181 — Return from the Wilds
pub(in crate::card::sets) static RETURN_FROM_THE_WILDS: CardRecord = CardRecord::new(
    "Return from the Wilds",
    "9597d9ea-d9b2-4009-8e7c-02caa3585bc5",
    "Julia Metzger",
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Search your library for a basic land card, put it onto the \
battlefield tapped, then shuffle.",
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
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
                    enters_tapped: true,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
            AbilityDef::spell(
                "Create a 1/1 white Human creature token.",
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(HUMAN_TOKEN))),
            ),
            AbilityDef::spell(
                "Create a Food token.",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ),
        ],
    )
    .with_mode_selection(2, 2, false)]),
);

// WOE 182 — Rootrider Faun
pub(in crate::card::sets) static ROOTRIDER_FAUN: CardRecord = CardRecord::new(
    "Rootrider Faun",
    "4e87db5a-1a70-42df-83a2-c0f71fe8533a",
    "Jesper Ejsing",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Satyr", "Scout"], 1, 3).with_abilities(&[
        abilities::tap_for(ManaColor::Green),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// WOE 183 — Royal Treatment
pub(in crate::card::sets) static ROYAL_TREATMENT: CardRecord = CardRecord::new(
    "Royal Treatment",
    "b6516b8f-ecfb-401e-ba8e-bf561aa2be64",
    "Julia Metzger",
    CardRules::new_instant(mana_cost!("{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control gains hexproof until end of turn. \
Create a Royal Role token attached to that creature. (If you \
control another Role on it, put that one into the graveyard. \
Enchanted creature gets +1/+1 and has ward {1}.)",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &const { [ZoneKind::Battlefield] },
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::CreateAttachedToken {
                token: ROYAL_ROLE,
                host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
            },
        ]),
    )]),
);

// WOE 184 — Sentinel of Lost Lore
// Audit: unsupported — Needs an object predicate for a card having an Adventure, independently of its currently presented face; the model exposes no Adventure-composition predicate.
pub(in crate::card::sets) static SENTINEL_OF_LOST_LORE: CardRecord = CardRecord::new(
    "Sentinel of Lost Lore",
    "f109a5bf-1472-4b87-b3d3-70db0e123693",
    "Cristi Balanescu",
    CardRules::unsupported(),
);

// WOE 185 — Skybeast Tracker
pub(in crate::card::sets) static SKYBEAST_TRACKER: CardRecord = CardRecord::new(
    "Skybeast Tracker",
    "a08da5c6-ebe7-4166-99d5-2aca5b0b529f",
    "Andreas Zafiratos",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Giant", "Archer"], 2, 4).with_abilities(&[
        abilities::reach(),
        AbilityDef::triggered(
            "Whenever you cast a spell with mana value 5 or greater, \
create a Food token. (It's an artifact with \"{2}, {T}, \
Sacrifice this token: You gain 3 life.\")",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(4)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
    ]),
);

// WOE 186 — Spider Food
pub(in crate::card::sets) static SPIDER_FOOD: CardRecord = CardRecord::new(
    "Spider Food",
    "b9fd720b-e9c2-4e82-917e-bab6c544afb0",
    "Mila Pesic",
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy up to one target artifact, enchantment, or creature \
with flying. Create a Food token. (It's an artifact with \
\"{2}, {T}, Sacrifice this token: You gain 3 life.\")",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasAbility(AbilityPredicateDef::Keyword(
                            KeywordAbility::Flying,
                        )),
                    ]),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            1,
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ]),
    )]),
);

// WOE 187 — Stormkeld Vanguard // Bear Down
pub(in crate::card::sets) static STORMKELD_VANGUARD: CardRecord = CardRecord::new(
    "Stormkeld Vanguard // Bear Down",
    "bacb1fe5-0adf-461f-b698-9d09a8728c63",
    "Aldo Domínguez",
    CardRules::new_creature(
        mana_cost!("{4}{G}{G}"),
        &const { ["Giant", "Warrior"] },
        6,
        7,
    )
    .with_abilities(
        &const {
            [AbilityDef::static_ability(
                "This creature can't be blocked by creatures with power 2 or less.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                        ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                    )),
                },
            )]
        },
    ),
)
.with_composition(|| {
    adventure(
        &STORMKELD_VANGUARD,
        "Bear Down",
        &CardRules::new_sorcery(mana_cost!("{1}{G}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Destroy target artifact or enchantment. (Then exile this \
card. You may cast the creature later from exile.)",
                    &const {
                        [AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::AnyOf(
                                &const {
                                    [
                                        ObjectPredicateDef::HasType(CardType::Artifact),
                                        ObjectPredicateDef::HasType(CardType::Enchantment),
                                    ]
                                },
                            ),
                        )]
                    },
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        then: None,
                    },
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 188 — Tanglespan Lookout
pub(in crate::card::sets) static TANGLESPAN_LOOKOUT: CardRecord = CardRecord::new(
    "Tanglespan Lookout",
    "3bc5c32d-be0a-4a5f-a8c7-9767a895bc76",
    "Dmitry Burmak",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Satyr"], 2, 3).with_abilities(&[
        AbilityDef::triggered(
            "Whenever an Aura you control enters, draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Aura")),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// WOE 189 — Territorial Witchstalker
pub(in crate::card::sets) static TERRITORIAL_WITCHSTALKER: CardRecord = CardRecord::new(
    "Territorial Witchstalker",
    "2515d53d-7a50-4da3-980d-91d91fea2020",
    "Ilse Gort",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Wolf"], 2, 3).with_abilities(&[
        abilities::defender(),
        AbilityDef::triggered_if(
            "At the beginning of combat on your turn, if you control a \
creature with power 4 or greater, this creature gets +1/+0 \
until end of turn and can attack this turn as though it \
didn't have defender.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ObjectCount {
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
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::Rule(AppliedRuleDef::MayAttackDespiteDefender),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// WOE 190 — Thunderous Debut
pub(in crate::card::sets) static THUNDEROUS_DEBUT: CardRecord = CardRecord::new(
    "Thunderous Debut",
    "d98f51ec-8eae-434a-ab62-85bdb6586fa2",
    "Aldo Domínguez",
    CardRules::new_sorcery(mana_cost!("{6}{G}{G}")).with_abilities(&[
        bargain(),
        AbilityDef::spell(
            "Look at the top twenty cards of your library. You may reveal \
up to two creature cards from among them. If this spell was \
bargained, put the revealed cards onto the battlefield. \
Otherwise, put the revealed cards into your hand. Then \
shuffle.",
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(20),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::HasType(CardType::Creature),
                minimum: 0,
                maximum: 2,
                chosen: crate::Binding!("chosen"),
                remainder: crate::Binding!("rest"),
                then: &EffectDef::Sequence(&[
                    EffectDef::RevealObjects(RevealObjectsDef {
                        input: ObjectSetDef::Binding(crate::Binding!("chosen")),
                        then: &EffectDef::None,
                    }),
                    EffectDef::IfElseCondition {
                        condition: &TriggerConditionDef::SourcePaidAdditionalCost(
                            AdditionalCostIndex::PRIMARY,
                        ),
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "chosen"
                            ))),
                            ZoneKind::Battlefield,
                            ZonePlacement::Top,
                        ),
                        otherwise: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "chosen"
                            ))),
                            ZoneKind::Hand,
                            ZonePlacement::Top,
                        ),
                    },
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!("rest"))),
                        ZoneKind::Library,
                        ZonePlacement::Top,
                    ),
                    EffectDef::ShuffleLibrary {
                        player: EffectRecipientDef::Controller,
                    },
                ]),
            }),
        ),
    ]),
);

// WOE 191 — Titanic Growth (reprint)
const TITANIC_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m12::TITANIC_GROWTH,
    "46917de3-5e98-4dd6-8950-fc10338515df",
    "Iris Compiet",
);

// WOE 192 — Toadstool Admirer
pub(in crate::card::sets) static TOADSTOOL_ADMIRER: CardRecord = CardRecord::new(
    "Toadstool Admirer",
    "f0a6349a-beff-4abd-b005-86d27c1e2957",
    "Julia Metzger",
    CardRules::new_creature(mana_cost!("{G}"), &["Ouphe"], 1, 1).with_abilities(&[
        abilities::ward(&[CostDef::Mana(mana_cost!("{2}"))], "Ward {2}"),
        AbilityDef::activated(
            "{3}{G}: Put a +1/+1 counter on this creature.",
            &[CostDef::Mana(mana_cost!("{3}{G}"))],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// WOE 193 — Tough Cookie
pub(in crate::card::sets) static TOUGH_COOKIE: CardRecord = CardRecord::new(
    "Tough Cookie",
    "7aa37f85-5336-4372-97a2-9c8b00798c7a",
    "Milivoj Ćeran",
    CardRules::new_artifact_creature(mana_cost!("{1}{G}"), &["Food", "Golem"], 2, 2)
        .with_abilities(&[
            abilities::enters_trigger(
                "When this creature enters, create a Food token. (It's an \
artifact with \"{2}, {T}, Sacrifice this token: You gain 3 \
life.\")",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ),
            AbilityDef::activated_with_targets(
                "{2}{G}: Until end of turn, target noncreature artifact you \
control becomes a 4/4 artifact creature.",
                &[CostDef::Mana(mana_cost!("{2}{G}"))],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                CardType::Creature,
                            )),
                        ]),
                        zones: &const { [ZoneKind::Battlefield] },
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(4),
                            ValueDef::Constant(4),
                        ),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::activated(
                "{2}, {T}, Sacrifice this creature: You gain 3 life.",
                &[
                    CostDef::Mana(mana_cost!("{2}")),
                    CostDef::TapSource,
                    CostDef::SacrificeSource,
                ],
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            ),
        ]),
);

// WOE 194 — Troublemaker Ouphe
pub(in crate::card::sets) static TROUBLEMAKER_OUPHE: CardRecord = CardRecord::new(
    "Troublemaker Ouphe",
    "7f7b2fc0-d3f6-4c4d-a163-986a372e5b12",
    "Jesper Ejsing",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Ouphe"], 2, 2).with_abilities(&[
        bargain(),
        AbilityDef::triggered_if_with_targets(
            "When this creature enters, if it was bargained, exile target \
artifact or enchantment an opponent controls.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::SourcePaidAdditionalCost(AdditionalCostIndex::PRIMARY),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
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

// WOE 195 — Up the Beanstalk
pub(in crate::card::sets) static UP_THE_BEANSTALK: CardRecord = CardRecord::new(
    "Up the Beanstalk",
    "2d5e991f-23b2-4db0-a452-7755125b1fd2",
    "Lucas Graciano",
    CardRules::new_enchantment(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::triggered(
        "When this enchantment enters and whenever you cast a spell \
with mana value 5 or greater, draw a card.",
        TriggerEventDef::AnyOf(&[
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(4)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
        ]),
        abilities::draw_cards(ValueDef::Constant(1)),
    )]),
);

// WOE 196 — Verdant Outrider
pub(in crate::card::sets) static VERDANT_OUTRIDER: CardRecord = CardRecord::new(
    "Verdant Outrider",
    "c34830e4-823a-40fa-ba41-bb2afbf1e499",
    "Taras Susak",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Knight"], 4, 2).with_abilities(&[
        AbilityDef::activated(
            "{1}{G}: This creature can't be blocked by creatures with \
power 2 or less this turn.",
            &[CostDef::Mana(mana_cost!("{1}{G}"))],
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

// WOE 197 — Virtue of Strength // Garenbrig Growth
// Audit: unsupported — Needs a mana-production replacement that triples each component produced by tapping a basic land, preserving its mana types and restrictions; mana production is not a supported replacement event.
pub(in crate::card::sets) static VIRTUE_OF_STRENGTH: CardRecord = CardRecord::new(
    "Virtue of Strength // Garenbrig Growth",
    "ff857d41-767d-4e99-83cc-444738341b92",
    "Piotr Dura",
    CardRules::unsupported(),
);

// WOE 198 — Welcome to Sweettooth
pub(in crate::card::sets) static WELCOME_TO_SWEETTOOTH: CardRecord = CardRecord::new(
    "Welcome to Sweettooth",
    "8e629a31-2e06-4f95-9628-34670dcf68b9",
    "Gaboleps",
    CardRules::new_enchantment(mana_cost!("{1}{G}"))
        .with_subtypes(&["Saga"])
        .with_abilities(&[
            abilities::saga_chapter(
                1,
                "I — Create a 1/1 white Human creature token.",
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(HUMAN_TOKEN))),
            ),
            abilities::saga_chapter(
                2,
                "II — Create a Food token.",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ),
            abilities::saga_chapter_with_targets(
                3,
                "III — Put X +1/+1 counters on target creature you control, \
where X is one plus the number of Foods you control.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &const { [ZoneKind::Battlefield] },
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Sum(&SumValueDef::new(
                        ValueDef::Constant(1),
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Food")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                    )),
                },
            ),
        ]),
);

// WOE 199 — Agatha of the Vile Cauldron
// Audit: unsupported — Needs activated-ability cost reduction by the source's current power; the cost-reduction evaluator rejects source-power values even though the declaration type exposes a dynamic amount.
pub(in crate::card::sets) static AGATHA_OF_THE_VILE_CAULDRON: CardRecord = CardRecord::new(
    "Agatha of the Vile Cauldron",
    "d6c48f07-63b7-4a60-8da6-ce77405abf1e",
    "Jason A. Engle",
    CardRules::unsupported(),
);

// WOE 200 — The Apprentice's Folly
pub(in crate::card::sets) static THE_APPRENTICE_S_FOLLY: CardRecord = CardRecord::new(
    "The Apprentice's Folly",
    "0edb58bb-8ff3-4e34-b3d1-d83b5bd8c178",
    "Tuan Duong Chu",
    CardRules::new_enchantment(mana_cost!("{2}{U}{R}"))
        .with_subtypes(&["Saga"])
        .with_abilities(&[
            AbilityDef::triggered_with_targets(
                "I, II — Choose target nontoken creature you control that \
doesn't have the same name as a token you control. Create a \
token that's a copy of it, except it isn't legendary, is a \
Reflection in addition to its other types, and has haste.",
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::While {
                        event: &TriggerEventDef::CountersPlaced {
                            object: ObjectPredicateDef::Source,
                            kind: CounterKind::Lore,
                        },
                        condition: &TriggerConditionDef::SourceCounters {
                            kind: CounterKind::Lore,
                            comparison: ComparisonDef::Equal,
                            amount: 1,
                        },
                    },
                    TriggerEventDef::While {
                        event: &TriggerEventDef::CountersPlaced {
                            object: ObjectPredicateDef::Source,
                            kind: CounterKind::Lore,
                        },
                        condition: &TriggerConditionDef::SourceCounters {
                            kind: CounterKind::Lore,
                            comparison: ComparisonDef::Equal,
                            amount: 2,
                        },
                    },
                ]),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::NameIn(
                                &CardNameSetDef::NamesOf(&ObjectSetDef::Query(
                                    ObjectQueryDef::matching(
                                        ObjectPredicateDef::Token,
                                        &[ZoneKind::Battlefield],
                                        PlayerRelation::You,
                                    ),
                                )),
                            )),
                        ]),
                        zones: &const { [ZoneKind::Battlefield] },
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Copy(&TokenCopyDef {
                    object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    exceptions: CopyExceptionsDef {
                        removed_supertypes: &[CardSupertype::Legendary],
                        added_creature_types: CreatureTypeSetDef::named(&["Reflection"]),
                        added_abilities: &[CopyAbilityDef::Ability(&abilities::haste())],
                        ..CopyExceptionsDef::NONE
                    },
                }))),
            ),
            abilities::saga_chapter(
                3,
                "III — Sacrifice all Reflections you control.",
                EffectDef::sacrifice(EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Reflection")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                ))),
            ),
        ]),
);

// WOE 201 — Ash, Party Crasher
// Audit: unsupported — Needs per-player nonland-permanent entry history for the current turn, including objects that have since left; current battlefield entry timestamps cannot implement celebration.
pub(in crate::card::sets) static ASH_PARTY_CRASHER: CardRecord = CardRecord::new(
    "Ash, Party Crasher",
    "d51e6610-25b6-4d8e-92d7-c50a2ff844ff",
    "Jason Rainville",
    CardRules::unsupported(),
);

// WOE 202 — Eriette of the Charmed Apple
// Audit: unsupported — Needs a host-side predicate for a creature enchanted by an Aura its ability's controller controls; the existing Enchanted predicate cannot distinguish Aura controllers.
pub(in crate::card::sets) static ERIETTE_OF_THE_CHARMED_APPLE: CardRecord = CardRecord::new(
    "Eriette of the Charmed Apple",
    "ecead4cd-47ae-4c42-b15c-1b29b5caba18",
    "Magali Villeneuve",
    CardRules::unsupported(),
);

// WOE 203 — Faunsbane Troll
pub(in crate::card::sets) static FAUNSBANE_TROLL: CardRecord = CardRecord::new(
    "Faunsbane Troll",
    "2d8bd585-c5ea-46f8-8e11-f33c067f2f8e",
    "Artur Nakhodkin",
    CardRules::new_creature(mana_cost!("{2}{B}{G}"), &["Troll"], 4, 4).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a Monster Role token \
attached to it. (Enchanted creature gets +1/+1 and has \
trample.)",
            EffectDef::CreateAttachedToken {
                token: MONSTER_ROLE,
                host: Some(EffectRecipientDef::Source),
            },
        ),
        AbilityDef::activated_with_targets(
            "{1}, Sacrifice an Aura attached to this creature: This \
creature fights target creature you don't control. If that \
creature would die this turn, exile it instead. Activate only \
as a sorcery.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Aura")),
                    ObjectPredicateDef::AttachedTo(&ObjectPredicateDef::Source),
                ])),
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Fight {
                    first: ObjectRefDef::Source,
                    second: ObjectRefDef::Target(TargetIndex::PRIMARY),
                    excess: None,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::ExileInsteadOfDying),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// WOE 204 — The Goose Mother
// Audit: unsupported — Needs the cast X retained for an enters-trigger value after its source leaves the battlefield; SourceCastX currently reads only a live permanent, so removing Goose Mother before its Food trigger resolves changes the token count.
pub(in crate::card::sets) static THE_GOOSE_MOTHER: CardRecord = CardRecord::new(
    "The Goose Mother",
    "1a55c370-d396-4c73-8ee2-83dc4c124005",
    "Jesper Ejsing",
    CardRules::unsupported(),
);

// WOE 205 — Greta, Sweettooth Scourge
pub(in crate::card::sets) static GRETA_SWEETTOOTH_SCOURGE: CardRecord = CardRecord::new(
    "Greta, Sweettooth Scourge",
    "2cfd365e-34d1-4224-b925-119000311934",
    "Steve Prescott",
    CardRules::new_creature(mana_cost!("{1}{B}{G}"), &["Human", "Warrior"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Greta enters, create a Food token. (It's an artifact \
with \"{2}, {T}, Sacrifice this token: You gain 3 life.\")",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ),
            AbilityDef::activated_with_targets(
                "{G}, Sacrifice a Food: Put a +1/+1 counter on target \
creature. Activate only as a sorcery.",
                &[
                    CostDef::Mana(mana_cost!("{G}")),
                    CostDef::sacrifice_permanent(ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                        "Food",
                    ))),
                ],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed),
            AbilityDef::activated(
                "{1}{B}, Sacrifice a Food: You draw a card and you lose 1 life.",
                &[
                    CostDef::Mana(mana_cost!("{1}{B}")),
                    CostDef::sacrifice_permanent(ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                        "Food",
                    ))),
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
);

// WOE 206 — Hylda of the Icy Crown
// Audit: unsupported — Needs a reflexive trigger with targets or modes chosen after its preceding payment or event, retained even if the source has left the battlefield; choosing them with the original ability changes response timing and target legality.
pub(in crate::card::sets) static HYLDA_OF_THE_ICY_CROWN: CardRecord = CardRecord::new(
    "Hylda of the Icy Crown",
    "ae9231fd-053d-4b84-a7a8-86063465bc49",
    "Ekaterina Burmak",
    CardRules::unsupported(),
);

// WOE 207 — Johann, Apprentice Sorcerer
// Audit: unsupported — Needs once-per-turn usage tracking tied to this source's top-of-library spell permission; the current top-of-library permission has no usage limit.
pub(in crate::card::sets) static JOHANN_APPRENTICE_SORCERER: CardRecord = CardRecord::new(
    "Johann, Apprentice Sorcerer",
    "b88a762d-19ed-451d-a3a9-b3e7eea40f67",
    "Dmitry Burmak",
    CardRules::unsupported(),
);

// WOE 208 — Likeness Looter
pub(in crate::card::sets) static LIKENESS_LOOTER: CardRecord = CardRecord::new(
    "Likeness Looter",
    "2957472a-825e-4904-b7e8-62bef1cb432d",
    "Ben Hill",
    CardRules::new_creature(mana_cost!("{U}{B}"), &["Faerie", "Shapeshifter"], 1, 1)
        .with_abilities(&[
            abilities::flying(),
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
            AbilityDef::activated_with_targets(
                "{X}: This creature becomes a copy of target creature card in \
your graveyard with mana value X, except it has flying and \
this ability. Activate only as a sorcery.",
                &[CostDef::Mana(mana_cost!("{X}"))],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::ManaValueEqualTo(ValueDef::ChosenX),
                        ]),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                )],
                EffectDef::BecomeCopyOf {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    copier: None,
                    exceptions: CopyExceptionsDef::NONE.with_abilities(&[
                        CopyAbilityDef::Ability(&abilities::flying()),
                        CopyAbilityDef::This,
                    ]),
                    duration: None,
                },
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed),
        ]),
);

// WOE 209 — Neva, Stalked by Nightmares
pub(in crate::card::sets) static NEVA_STALKED_BY_NIGHTMARES: CardRecord = CardRecord::new(
    "Neva, Stalked by Nightmares",
    "e6f925ab-ade7-4da8-a791-185e098d18f2",
    "Tyler Jacobson",
    CardRules::new_creature(mana_cost!("{2}{W}{B}"), &["Human", "Noble"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::menace(),
            abilities::enters_trigger_with_targets(
                "When Neva enters, return target creature or enchantment card \
from your graveyard to your hand.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
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
            AbilityDef::triggered(
                "Whenever an enchantment you control is put into a graveyard \
from the battlefield, put a +1/+1 counter on Neva, then scry \
1.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    abilities::scry(ValueDef::Constant(1)),
                ]),
            ),
        ]),
);

// WOE 210 — Obyra, Dreaming Duelist
pub(in crate::card::sets) static OBYRA_DREAMING_DUELIST: CardRecord = CardRecord::new(
    "Obyra, Dreaming Duelist",
    "63ce03ee-279b-4955-98e3-9ce1990f8b7b",
    "Evyn Fong",
    CardRules::new_creature(mana_cost!("{U}{B}"), &["Faerie", "Warrior"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flash(),
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever another Faerie you control enters, each opponent \
loses 1 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Faerie")),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// WOE 211 — Rowan, Scion of War
// Audit: unsupported — Needs life-lost-this-turn history and a temporary spell-cost adjustment that freezes that amount at activation resolution.
pub(in crate::card::sets) static ROWAN_SCION_OF_WAR: CardRecord = CardRecord::new(
    "Rowan, Scion of War",
    "4ee179ab-a15b-4bd6-b7f8-1e1abeeb31b7",
    "Magali Villeneuve",
    CardRules::unsupported(),
);

// WOE 212 — Ruby, Daring Tracker
pub(in crate::card::sets) static RUBY_DARING_TRACKER: CardRecord = CardRecord::new(
    "Ruby, Daring Tracker",
    "ffb5786b-6825-4ebf-a1e1-80011340adbb",
    "Ekaterina Burmak",
    CardRules::new_creature(mana_cost!("{R}{G}"), &["Human", "Scout"], 1, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::haste(),
            AbilityDef::triggered(
                "Whenever Ruby attacks while you control a creature with power \
4 or greater, Ruby gets +2/+2 until end of turn.",
                TriggerEventDef::While {
                    event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                    condition: &TriggerConditionDef::ObjectCount {
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
                    },
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            abilities::tap_for_mana(
                "{T}: Add {R} or {G}.",
                AddManaEffectDef::choice(&[ManaColor::Red, ManaColor::Green]),
            ),
        ]),
);

// WOE 213 — Sharae of Numbing Depths
// Audit: unsupported — Needs a player-attributed batch tap event for one or more untapped opposing creatures; the existing per-object tap matcher does not preserve the actor or atomic batch.
pub(in crate::card::sets) static SHARAE_OF_NUMBING_DEPTHS: CardRecord = CardRecord::new(
    "Sharae of Numbing Depths",
    "600bc36a-3ef0-459c-9a93-94ec45b8c3d9",
    "Evyn Fong",
    CardRules::unsupported(),
);

// WOE 214 — Syr Armont, the Redeemer
pub(in crate::card::sets) static SYR_ARMONT_THE_REDEEMER: CardRecord = CardRecord::new(
    "Syr Armont, the Redeemer",
    "85050609-baf0-430a-ab33-83a6ea6d4741",
    "Magali Villeneuve",
    CardRules::new_creature(mana_cost!("{3}{G}{W}"), &["Human", "Knight"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[abilities::enters_trigger_with_targets(
            "When Syr Armont enters, create a Monster Role token attached \
to another target creature you control. (If you control \
another Role on it, put that one into the graveyard. \
Enchanted creature gets +1/+1 and has trample.)",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &const { [ZoneKind::Battlefield] },
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::CreateAttachedToken {
                token: MONSTER_ROLE,
                host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
            },
        )])
        .with_ability(AbilityDef::static_ability(
            "Enchanted creatures you control get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Enchanted,
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
            },
        )),
);

// WOE 215 — Talion, the Kindly Lord
// Audit: unsupported — Needs a stored 1–10 entry choice and spell-event comparisons of mana value, power, or toughness against that chosen number.
pub(in crate::card::sets) static TALION_THE_KINDLY_LORD: CardRecord = CardRecord::new(
    "Talion, the Kindly Lord",
    "62a6b452-c796-45c6-b4d1-0ae3d675e38e",
    "Justyna Dura",
    CardRules::unsupported(),
);

// WOE 216 — Totentanz, Swarm Piper
pub(in crate::card::sets) static TOTENTANZ_SWARM_PIPER: CardRecord = CardRecord::new(
    "Totentanz, Swarm Piper",
    "1422d6db-fe5b-4a89-951a-fbd7985a29fc",
    "Matt Stewart",
    CardRules::new_creature(mana_cost!("{1}{B}{R}"), &["Human", "Warlock", "Bard"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever Totentanz or another nontoken creature you control \
dies, create a 1/1 black Rat creature token with \"This token \
can't block.\"",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Source,
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                            ]),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]),
                    ]),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    DEFENSELESS_RAT_TOKEN,
                ))),
            ),
            AbilityDef::activated_with_targets(
                "{1}{B}: Target attacking Rat you control gains deathtouch \
until end of turn.",
                &[CostDef::Mana(mana_cost!("{1}{B}"))],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rat")),
                            ObjectPredicateDef::Attacking,
                        ]),
                        zones: &const { [ZoneKind::Battlefield] },
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::deathtouch()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// WOE 217 — Troyan, Gutsy Explorer
// Audit: unsupported — Needs restricted mana whose allowed-spell predicate is the union of mana value at least five and an X symbol in the chosen spell form's mana cost.
pub(in crate::card::sets) static TROYAN_GUTSY_EXPLORER: CardRecord = CardRecord::new(
    "Troyan, Gutsy Explorer",
    "eadd2a1a-93a0-4257-897d-1aaee279449f",
    "Jesper Ejsing",
    CardRules::unsupported(),
);

// WOE 218 — Will, Scion of Peace
// Audit: unsupported — Needs a temporary spell-cost adjustment that freezes life gained this turn when the activation resolves and expires at end of turn.
pub(in crate::card::sets) static WILL_SCION_OF_PEACE: CardRecord = CardRecord::new(
    "Will, Scion of Peace",
    "162088ea-5f99-4244-9427-2fdfb2168fc3",
    "Ryan Pancoast",
    CardRules::unsupported(),
);

// WOE 219 — Yenna, Redtooth Regent
pub(in crate::card::sets) static YENNA_REDTOOTH_REGENT: CardRecord = CardRecord::new(
    "Yenna, Redtooth Regent",
    "e635d461-254a-434e-8e5d-dea61dd8ca4f",
    "Justyna Dura",
    CardRules::new_creature(mana_cost!("{2}{G}{W}"), &["Elf", "Noble"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::activated_with_targets(
            "{2}, {T}: Choose target enchantment you control that doesn't \
have the same name as another permanent you control. Create a \
token that's a copy of it, except it isn't legendary. If the \
token is an Aura, untap Yenna, then scry 2. Activate only as \
a sorcery.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::NameIn(
                            &CardNameSetDef::NamesAppearingAtLeast {
                                objects: &ObjectSetDef::Query(ObjectQueryDef::matching(
                                    ObjectPredicateDef::Any,
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                )),
                                count: 2,
                            },
                        )),
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
                        removed_supertypes: &[CardSupertype::Legendary],
                        ..CopyExceptionsDef::NONE
                    },
                }))
                .with_created_tokens(CreatedTokensDef {
                    binding: crate::Binding!("copy"),
                    then: &EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ObjectSetCount(
                            &ObjectSetCountConditionDef {
                                objects: &ObjectSetDef::MatchingBinding {
                                    binding: crate::Binding!("copy"),
                                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                                        "Aura",
                                    )),
                                },
                                predicate: ObjectSetPredicateDef {
                                    filter: None,
                                    comparison: ComparisonDef::GreaterOrEqual,
                                    amount: 1,
                                },
                            },
                        ),
                        then: &EffectDef::Sequence(&[
                            EffectDef::Untap {
                                object: EffectRecipientDef::Source,
                            },
                            abilities::scry(ValueDef::Constant(2)),
                        ]),
                    },
                }),
            ),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed)]),
);

// WOE 220 — Beluna Grandsquall // Seek Thrills
// Audit: unsupported — Needs an object predicate for a card having an Adventure, independently of its currently presented face; the model exposes no Adventure-composition predicate.
pub(in crate::card::sets) static BELUNA_GRANDSQUALL: CardRecord = CardRecord::new(
    "Beluna Grandsquall // Seek Thrills",
    "3f5acc0d-33a6-476f-95ca-a1ad788334dd",
    "Victor Adame Minguez",
    CardRules::unsupported(),
);

// WOE 221 — Callous Sell-Sword // Burn Together
// Audit: unsupported — Needs current-turn creature death counts attributed to their battlefield controller for its entry counters; the available death counter is global.
pub(in crate::card::sets) static CALLOUS_SELL_SWORD: CardRecord = CardRecord::new(
    "Callous Sell-Sword // Burn Together",
    "770ee3da-d33e-466f-9a2e-ad2d08ef5012",
    "Valera Lutfullina",
    CardRules::unsupported(),
);

// WOE 222 — Cruel Somnophage // Can't Wake Up
pub(in crate::card::sets) static CRUEL_SOMNOPHAGE: CardRecord = CardRecord::new(
    "Cruel Somnophage // Can't Wake Up",
    "39b11ff0-9946-4337-86fb-42e967f3d2e4",
    "Jason A. Engle",
    CardRules::new_creature(mana_cost!("{1}{B}"), &const { ["Nightmare"] }, 0, 0).with_ability(
        AbilityDef::static_ability(
            "Cruel Somnophage's power and toughness are each equal to the \
number of creature cards in all graveyards.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::define_power_toughness(
                    ValueDef::CountMatchingObjects(
                        &const {
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &const { [ZoneKind::Graveyard] },
                                PlayerRelation::Any,
                            )
                        },
                    ),
                    ValueDef::CountMatchingObjects(
                        &const {
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &const { [ZoneKind::Graveyard] },
                                PlayerRelation::Any,
                            )
                        },
                    ),
                ),
            },
        ),
    ),
)
.with_composition(|| {
    adventure(
        &CRUEL_SOMNOPHAGE,
        "Can't Wake Up",
        &CardRules::new_sorcery(mana_cost!("{1}{U}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Target player mills four cards. (Then exile this card. You \
may cast the creature later from exile.)",
                    &const {
                        [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Player(PlayerRelation::Any),
                        )]
                    },
                    EffectDef::Mill {
                        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(4),
                    },
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 223 — Decadent Dragon // Expensive Taste
// Audit: unsupported — Needs a persistent controller-owned permission to play specifically bound face-down exiled cards for as long as they remain exiled; the existing controller permission lasts only this turn.
pub(in crate::card::sets) static DECADENT_DRAGON: CardRecord = CardRecord::new(
    "Decadent Dragon // Expensive Taste",
    "315cbbf7-a2ad-4565-9877-1e903d7fd797",
    "Wylie Beckert",
    CardRules::unsupported(),
);

// WOE 224 — Devouring Sugarmaw // Have for Dinner
pub(in crate::card::sets) static DEVOURING_SUGARMAW: CardRecord = CardRecord::new(
    "Devouring Sugarmaw // Have for Dinner",
    "58c7f52e-a97d-4475-ae00-3149991e723e",
    "Nino Vecia",
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &const { ["Horror"] }, 6, 6).with_abilities(
        &const {
            [
                abilities::menace(),
                abilities::trample(),
                AbilityDef::triggered(
                    "At the beginning of your upkeep, you may sacrifice an \
artifact, enchantment, or token. If you don't, tap this \
creature.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::Upkeep,
                        player: PlayerRelation::You,
                    },
                    EffectDef::PayOr(PayOrDef::optional_or(
                        &const {
                            [CostDef::sacrifice_permanent(ObjectPredicateDef::AnyOf(
                                &const {
                                    [
                                        ObjectPredicateDef::HasType(CardType::Artifact),
                                        ObjectPredicateDef::HasType(CardType::Enchantment),
                                        ObjectPredicateDef::Token,
                                    ]
                                },
                            ))]
                        },
                        &EffectDef::None,
                        &const {
                            EffectDef::Tap {
                                object: EffectRecipientDef::Source,
                            }
                        },
                    )),
                ),
            ]
        },
    ),
)
.with_composition(|| {
    adventure(
        &DEVOURING_SUGARMAW,
        "Have for Dinner",
        &CardRules::new_instant(mana_cost!("{1}{W}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell(
                    "Create a 1/1 white Human creature token and a Food token. \
(Then exile this card. You may cast the creature later from \
exile.)",
                    EffectDef::Sequence(
                        &const {
                            [
                                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                                    TokenCharacteristics::creature(
                                        &const { ["Human"] },
                                        &const { [ManaColor::White] },
                                        1,
                                        1,
                                    ),
                                ))),
                                EffectDef::CreateToken(
                                    CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                                        .with_count(ValueDef::Constant(1)),
                                ),
                            ]
                        },
                    ),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 225 — Elusive Otter // Grove's Bounty
pub(in crate::card::sets) static ELUSIVE_OTTER: CardRecord = CardRecord::new(
    "Elusive Otter // Grove's Bounty",
    "bc9bdf96-3e3b-4dca-aae2-e81d4cbeafe8",
    "Christina Kraus",
    CardRules::new_creature(mana_cost!("{U}"), &const { ["Otter"] }, 1, 1).with_abilities(
        &const {
            [
                abilities::prowess(),
                AbilityDef::static_ability(
                    "Creatures with power less than this creature's power can't \
block it.",
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                            ObjectPredicateDef::PowerLessThan(ValueDef::SourcePower),
                        )),
                    },
                ),
            ]
        },
    ),
)
.with_composition(|| {
    adventure(
        &ELUSIVE_OTTER,
        "Grove's Bounty",
        &CardRules::new_sorcery(mana_cost!("{X}{G}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Distribute X +1/+1 counters among any number of target \
creatures you control. (Then exile this card. You may cast \
the creature later from exile.)",
                    &const {
                        [AbilityTargetDef {
                            minimum: 0,
                            maximum: AbilityTargetDef::UNLIMITED,
                            divided_total: Some(DividedTotal::ChosenX),
                            ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::HasType(CardType::Creature),
                                zones: &const { [ZoneKind::Battlefield] },
                                controller: Some(PlayerRelation::You),
                                owner: None,
                            })
                        }]
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::DividedAmongTargets,
                    },
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 226 — Frolicking Familiar // Blow Off Steam
pub(in crate::card::sets) static FROLICKING_FAMILIAR: CardRecord = CardRecord::new(
    "Frolicking Familiar // Blow Off Steam",
    "64c432d5-4f5b-44ac-9d61-891e78460d58",
    "Brian Valeza",
    CardRules::new_creature(mana_cost!("{2}{U}"), &const { ["Otter", "Wizard"] }, 2, 2)
        .with_abilities(
            &const {
                [
                    abilities::flying(),
                    AbilityDef::triggered(
                        "Whenever you cast an instant or sorcery spell, this creature \
gets +1/+1 until end of turn.",
                        TriggerEventDef::spell_cast(ObjectPredicateDef::All(
                            &const {
                                [
                                    ObjectPredicateDef::AnyOf(
                                        &const {
                                            [
                                                ObjectPredicateDef::HasType(CardType::Instant),
                                                ObjectPredicateDef::HasType(CardType::Sorcery),
                                            ]
                                        },
                                    ),
                                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                                ]
                            },
                        )),
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(1),
                                ValueDef::Constant(1),
                            ),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ),
                ]
            },
        ),
)
.with_composition(|| {
    adventure(
        &FROLICKING_FAMILIAR,
        "Blow Off Steam",
        &CardRules::new_instant(mana_cost!("{R}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Blow Off Steam deals 1 damage to any target. (Then exile this \
card. You may cast the creature later from exile.)",
                    &const {
                        [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::AnyTarget,
                        )]
                    },
                    EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(1),
                    ),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 227 — Gingerbread Hunter // Puny Snack
pub(in crate::card::sets) static GINGERBREAD_HUNTER: CardRecord = CardRecord::new(
    "Gingerbread Hunter // Puny Snack",
    "e77a8fd4-af5f-42b3-a87e-788baf2562dd",
    "Milivoj Ćeran",
    CardRules::new_creature(mana_cost!("{4}{G}"), &const { ["Giant"] }, 5, 5).with_abilities(
        &const {
            [abilities::enters_trigger(
                "When this creature enters, create a Food token. (It's an \
artifact with \"{2}, {T}, Sacrifice this token: You gain 3 \
life.\")",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            )]
        },
    ),
)
.with_composition(|| {
    adventure(
        &GINGERBREAD_HUNTER,
        "Puny Snack",
        &CardRules::new_instant(mana_cost!("{2}{B}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Target creature gets -2/-2 until end of turn. (Then exile \
this card. You may cast the creature later from exile.)",
                    &const {
                        [AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )]
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(-2),
                            ValueDef::Constant(-2),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 228 — Heartflame Duelist // Heartflame Slash
// Audit: unsupported — Needs a continuous keyword grant to instant and sorcery spells on the stack; static applied recipients currently do not admit that stack-object group.
pub(in crate::card::sets) static HEARTFLAME_DUELIST: CardRecord = CardRecord::new(
    "Heartflame Duelist // Heartflame Slash",
    "811b283f-22f3-47b1-a802-11dc8c25d0ee",
    "Justyna Dura",
    CardRules::unsupported(),
);

// WOE 229 — Imodane's Recruiter // Train Troops
pub(in crate::card::sets) static IMODANE_S_RECRUITER: CardRecord = CardRecord::new(
    "Imodane's Recruiter // Train Troops",
    "4dbaa855-3f8e-42e6-8ec8-5ffbc5c8acf0",
    "Néstor Ossandón Leal",
    CardRules::new_creature(mana_cost!("{2}{R}"), &const { ["Human", "Knight"] }, 2, 2)
        .with_abilities(
            &const {
                [abilities::enters_trigger(
                    "When this creature enters, creatures you control get +1/+0 \
and gain haste until end of turn.",
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &const { [ZoneKind::Battlefield] },
                                PlayerRelation::You,
                            ),
                        )),
                        effect: AppliedEffectDef::Composite(
                            &const {
                                [
                                    AppliedEffectDef::modify_power_toughness(
                                        ValueDef::Constant(1),
                                        ValueDef::Constant(0),
                                    ),
                                    AppliedEffectDef::add_ability(&const { abilities::haste() }),
                                ]
                            },
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                )]
            },
        ),
)
.with_composition(|| {
    adventure(
        &IMODANE_S_RECRUITER,
        "Train Troops",
        &CardRules::new_sorcery(mana_cost!("{4}{W}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell(
                    "Create two 2/2 white Knight creature tokens with vigilance. \
(Then exile this card. You may cast the creature later from \
exile.)",
                    EffectDef::CreateToken(
                        CreateTokenDef::new(TokenDef::Literal(
                            TokenCharacteristics::creature(
                                &const { ["Knight"] },
                                &const { [ManaColor::White] },
                                2,
                                2,
                            )
                            .with_abilities(&const { [abilities::vigilance()] }),
                        ))
                        .with_count(ValueDef::Constant(2)),
                    ),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 230 — Kellan, the Fae-Blooded // Birthright Boon
pub(in crate::card::sets) static KELLAN_THE_FAE_BLOODED: CardRecord = CardRecord::new(
    "Kellan, the Fae-Blooded // Birthright Boon",
    "ec5e2680-8b42-4571-ab45-4936aec51901",
    "Anna Steinbauer",
    CardRules::new_creature(mana_cost!("{2}{R}"), &const { ["Human", "Faerie"] }, 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(
            &const {
                [
                    abilities::double_strike(),
                    AbilityDef::static_ability(
                        "Other creatures you control get +1/+0 for each Aura and \
Equipment attached to Kellan.",
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                                ObjectQueryDef::matching(
                                    ObjectPredicateDef::All(
                                        &const {
                                            [
                                                ObjectPredicateDef::HasType(CardType::Creature),
                                                ObjectPredicateDef::Not(
                                                    &ObjectPredicateDef::Source,
                                                ),
                                            ]
                                        },
                                    ),
                                    &const { [ZoneKind::Battlefield] },
                                    PlayerRelation::You,
                                ),
                            )),
                            effect: AppliedEffectDef::modify_power_toughness(
                                ValueDef::CountMatchingObjects(
                                    &const {
                                        ObjectQueryDef::matching(
                                            ObjectPredicateDef::All(
                                                &const {
                                                    [
                                                        ObjectPredicateDef::AnyOf(
                                                            &const {
                                                                [
                                                                    ObjectPredicateDef::Subtype(
                                                                        SubtypeDef::Literal("Aura"),
                                                                    ),
                                                                    ObjectPredicateDef::Subtype(
                                                                        SubtypeDef::Literal(
                                                                            "Equipment",
                                                                        ),
                                                                    ),
                                                                ]
                                                            },
                                                        ),
                                                        ObjectPredicateDef::AttachedTo(
                                                            &ObjectPredicateDef::Source,
                                                        ),
                                                    ]
                                                },
                                            ),
                                            &const { [ZoneKind::Battlefield] },
                                            PlayerRelation::Any,
                                        )
                                    },
                                ),
                                ValueDef::Constant(0),
                            ),
                        },
                    ),
                ]
            },
        ),
)
.with_composition(|| {
    adventure(
        &KELLAN_THE_FAE_BLOODED,
        "Birthright Boon",
        &CardRules::new_sorcery(mana_cost!("{1}{W}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell(
                    "Search your library for an Aura or Equipment card, reveal it, \
put it into your hand, then shuffle.",
                    EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::AnyOf(
                            &const {
                                [
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Aura")),
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Equipment")),
                                ]
                            },
                        ),
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
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 231 — Mosswood Dreadknight // Dread Whispers
// Audit: unsupported — Needs a graveyard casting permission restricted to the Adventure spell form through the end of the controller's next turn; existing permissions cannot select that form.
pub(in crate::card::sets) static MOSSWOOD_DREADKNIGHT: CardRecord = CardRecord::new(
    "Mosswood Dreadknight // Dread Whispers",
    "9869ac70-5907-45fa-952c-31aef70c5066",
    "Ryan Pancoast",
    CardRules::unsupported(),
);

// WOE 232 — Picnic Ruiner // Stolen Goodies
pub(in crate::card::sets) static PICNIC_RUINER: CardRecord = CardRecord::new(
    "Picnic Ruiner // Stolen Goodies",
    "66485c3e-3b21-4db4-ac12-af04e35b49b1",
    "Edgar Sánchez Hidalgo",
    CardRules::new_creature(mana_cost!("{1}{R}"), &const { ["Goblin", "Rogue"] }, 2, 2)
        .with_abilities(
            &const {
                [AbilityDef::triggered(
                    "Whenever this creature attacks while you control a creature \
with power 4 or greater, this creature gains double strike \
until end of turn.",
                    TriggerEventDef::While {
                        event: &const { TriggerEventDef::attacks(ObjectPredicateDef::Source) },
                        condition: &const {
                            TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::All(
                                        &const {
                                            [
                                                ObjectPredicateDef::HasType(CardType::Creature),
                                                ObjectPredicateDef::PowerAtLeast(4),
                                            ]
                                        },
                                    ),
                                    &const { [ZoneKind::Battlefield] },
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            }
                        },
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::add_ability(
                            &const { abilities::double_strike() },
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                )]
            },
        ),
)
.with_composition(|| {
    adventure(
        &PICNIC_RUINER,
        "Stolen Goodies",
        &CardRules::new_sorcery(mana_cost!("{3}{G}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Distribute three +1/+1 counters among any number of target \
creatures you control.",
                    &const {
                        [AbilityTargetDef {
                            minimum: 0,
                            maximum: AbilityTargetDef::UNLIMITED,
                            divided_total: Some(DividedTotal::Fixed(3)),
                            ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::HasType(CardType::Creature),
                                zones: &const { [ZoneKind::Battlefield] },
                                controller: Some(PlayerRelation::You),
                                owner: None,
                            })
                        }]
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::DividedAmongTargets,
                    },
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 233 — Pollen-Shield Hare // Hare Raising
pub(in crate::card::sets) static POLLEN_SHIELD_HARE: CardRecord = CardRecord::new(
    "Pollen-Shield Hare // Hare Raising",
    "139f31f5-28f0-4e90-abdf-3f6ed0992dea",
    "Olena Richards",
    CardRules::new_creature(mana_cost!("{1}{W}"), &const { ["Rabbit"] }, 2, 2).with_abilities(
        &const {
            [AbilityDef::static_ability(
                "Creature tokens you control get +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(
                                &const {
                                    [
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        ObjectPredicateDef::Token,
                                    ]
                                },
                            ),
                            &const { [ZoneKind::Battlefield] },
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            )]
        },
    ),
)
.with_composition(|| {
    adventure(
        &POLLEN_SHIELD_HARE,
        "Hare Raising",
        &CardRules::new_sorcery(mana_cost!("{G}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Target creature you control gains vigilance and gets +X/+X \
until end of turn, where X is the number of creatures you \
control.",
                    &const {
                        [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::HasType(CardType::Creature),
                                zones: &const { [ZoneKind::Battlefield] },
                                controller: Some(PlayerRelation::You),
                                owner: None,
                            },
                        )]
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::Composite(
                            &const {
                                [
                                    AppliedEffectDef::add_ability(
                                        &const { abilities::vigilance() },
                                    ),
                                    AppliedEffectDef::modify_power_toughness(
                                        ValueDef::CountMatchingObjects(
                                            &const {
                                                ObjectQueryDef::matching(
                                                    ObjectPredicateDef::HasType(CardType::Creature),
                                                    &const { [ZoneKind::Battlefield] },
                                                    PlayerRelation::You,
                                                )
                                            },
                                        ),
                                        ValueDef::CountMatchingObjects(
                                            &const {
                                                ObjectQueryDef::matching(
                                                    ObjectPredicateDef::HasType(CardType::Creature),
                                                    &const { [ZoneKind::Battlefield] },
                                                    PlayerRelation::You,
                                                )
                                            },
                                        ),
                                    ),
                                ]
                            },
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 234 — Questing Druid // Seek the Beast
// Audit: unsupported — Needs exile-play permission that expires as the controller's next end step begins; the existing UntilYourNextEndStep path shares an expiry representation with end-of-next-turn permissions and remains usable after that end step.
pub(in crate::card::sets) static QUESTING_DRUID: CardRecord = CardRecord::new(
    "Questing Druid // Seek the Beast",
    "72c130e2-1e17-4996-a5ae-231155d68261",
    "Jason A. Engle",
    CardRules::unsupported(),
);

// WOE 235 — Scalding Viper // Steam Clean
pub(in crate::card::sets) static SCALDING_VIPER: CardRecord = CardRecord::new(
    "Scalding Viper // Steam Clean",
    "58e72bfb-6f64-4647-afb6-b5ad4737121c",
    "Andrew Mar",
    CardRules::new_creature(
        mana_cost!("{1}{R}"),
        &const { ["Elemental", "Snake"] },
        2,
        1,
    )
    .with_abilities(
        &const {
            [AbilityDef::triggered(
                "Whenever an opponent casts a spell with mana value 3 or less, \
this creature deals 1 damage to that player.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(
                    &const {
                        [
                            ObjectPredicateDef::ManaValueAtMost(3),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                        ]
                    },
                )),
                EffectDef::damage(
                    EffectRecipientDef::player(PlayerRefDef::EventPlayer),
                    ValueDef::Constant(1),
                ),
            )]
        },
    ),
)
.with_composition(|| {
    adventure(
        &SCALDING_VIPER,
        "Steam Clean",
        &CardRules::new_sorcery(mana_cost!("{1}{U}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Return target nonland permanent to its owner's hand. (Then \
exile this card. You may cast the creature later from exile.)",
                    &const {
                        [AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::Not(
                                &const { ObjectPredicateDef::HasType(CardType::Land) },
                            ),
                        )]
                    },
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 236 — Shrouded Shepherd // Cleave Shadows
pub(in crate::card::sets) static SHROUDED_SHEPHERD: CardRecord = CardRecord::new(
    "Shrouded Shepherd // Cleave Shadows",
    "ab03c342-2bf4-41bf-8bb8-472d978d238a",
    "Randy Vargas",
    CardRules::new_creature(mana_cost!("{1}{W}"), &const { ["Spirit", "Warrior"] }, 2, 2)
        .with_abilities(
            &const {
                [abilities::enters_trigger_with_targets(
                    "When this creature enters, target creature you control gets \
+2/+2 until end of turn.",
                    &const {
                        [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::HasType(CardType::Creature),
                                zones: &const { [ZoneKind::Battlefield] },
                                controller: Some(PlayerRelation::You),
                                owner: None,
                            },
                        )]
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                )]
            },
        ),
)
.with_composition(|| {
    adventure(
        &SHROUDED_SHEPHERD,
        "Cleave Shadows",
        &CardRules::new_sorcery(mana_cost!("{1}{B}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell(
                    "Creatures your opponents control get -1/-1 until end of turn. \
(Then exile this card. You may cast the creature later from \
exile.)",
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &const { [ZoneKind::Battlefield] },
                                PlayerRelation::Opponent,
                            ),
                        )),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(-1),
                            ValueDef::Constant(-1),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 237 — Spellscorn Coven // Take It Back
pub(in crate::card::sets) static SPELLSCORN_COVEN: CardRecord = CardRecord::new(
    "Spellscorn Coven // Take It Back",
    "8c112f62-6034-4636-a75b-4a45bc916a91",
    "Uriah Voth",
    CardRules::new_creature(mana_cost!("{3}{B}"), &const { ["Faerie", "Warlock"] }, 2, 3)
        .with_abilities(
            &const {
                [
                    abilities::flying(),
                    abilities::enters_trigger(
                        "When this creature enters, each opponent discards a card.",
                        EffectDef::Discard {
                            recipient: EffectRecipientDef::Opponent,
                            amount: ValueDef::Constant(1),
                            selection: DiscardSelectionDef::RecipientChooses,
                            then: None,
                        },
                    ),
                ]
            },
        ),
)
.with_composition(|| {
    adventure(
        &SPELLSCORN_COVEN,
        "Take It Back",
        &CardRules::new_instant(mana_cost!("{2}{U}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Return target spell to its owner's hand. (Then exile this \
card. You may cast the creature later from exile.)",
                    &const {
                        [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::Spell,
                                zones: &const { [ZoneKind::Stack] },
                                controller: None,
                                owner: None,
                            },
                        )]
                    },
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 238 — Tempest Hart // Scan the Clouds
pub(in crate::card::sets) static TEMPEST_HART: CardRecord = CardRecord::new(
    "Tempest Hart // Scan the Clouds",
    "559bacc8-facc-4d93-90b5-8ac21d3246f5",
    "Aldo Domínguez",
    CardRules::new_creature(mana_cost!("{3}{G}"), &const { ["Elemental", "Elk"] }, 3, 4)
        .with_abilities(
            &const {
                [
                    abilities::trample(),
                    AbilityDef::triggered(
                        "Whenever you cast a spell with mana value 5 or greater, put a \
+1/+1 counter on this creature.",
                        TriggerEventDef::spell_cast(ObjectPredicateDef::All(
                            &const {
                                [
                                    ObjectPredicateDef::Not(
                                        &const { ObjectPredicateDef::ManaValueAtMost(4) },
                                    ),
                                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                                ]
                            },
                        )),
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::Source,
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                    ),
                ]
            },
        ),
)
.with_composition(|| {
    adventure(
        &TEMPEST_HART,
        "Scan the Clouds",
        &CardRules::new_instant(mana_cost!("{1}{U}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell(
                    "Draw two cards, then discard two cards. (Then exile this \
card. You may cast the creature later from exile.)",
                    EffectDef::Sequence(
                        &const {
                            [
                                abilities::draw_cards(ValueDef::Constant(2)),
                                EffectDef::Discard {
                                    recipient: EffectRecipientDef::Controller,
                                    amount: ValueDef::Constant(2),
                                    selection: DiscardSelectionDef::RecipientChooses,
                                    then: None,
                                },
                            ]
                        },
                    ),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 239 — Threadbind Clique // Rip the Seams
pub(in crate::card::sets) static THREADBIND_CLIQUE: CardRecord = CardRecord::new(
    "Threadbind Clique // Rip the Seams",
    "dd6ed252-c262-4062-97ba-75c50d6b5579",
    "Michal Ivan",
    CardRules::new_creature(mana_cost!("{3}{U}"), &const { ["Faerie"] }, 3, 3)
        .with_abilities(&const { [abilities::flying()] }),
)
.with_composition(|| {
    adventure(
        &THREADBIND_CLIQUE,
        "Rip the Seams",
        &CardRules::new_instant(mana_cost!("{2}{W}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Destroy target tapped creature. (Then exile this card. You \
may cast the creature later from exile.)",
                    &const {
                        [AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::All(
                                &const {
                                    [
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        ObjectPredicateDef::Tapped,
                                    ]
                                },
                            ),
                        )]
                    },
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        then: None,
                    },
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 240 — Twining Twins // Swift Spiral
pub(in crate::card::sets) static TWINING_TWINS: CardRecord = CardRecord::new(
    "Twining Twins // Swift Spiral",
    "043718ea-59f6-4d1a-94c5-271704c1a38a",
    "Fajareka Setiawan",
    CardRules::new_creature(
        mana_cost!("{2}{U}{U}"),
        &const { ["Faerie", "Wizard"] },
        4,
        4,
    )
    .with_abilities(
        &const {
            [
                abilities::flying(),
                abilities::vigilance(),
                abilities::ward(&const { [CostDef::Mana(mana_cost!("{1}"))] }, "Ward {1}"),
            ]
        },
    ),
)
.with_composition(|| {
    adventure(
        &TWINING_TWINS,
        "Swift Spiral",
        &CardRules::new_instant(mana_cost!("{1}{W}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Exile target nontoken creature. Return it to the battlefield \
under its owner's control at the beginning of the next end \
step.",
                    &const {
                        [AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::All(
                                &const {
                                    [
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                                    ]
                                },
                            ),
                        )]
                    },
                    abilities::exile_until_next_end_step(EffectRecipientDef::Target(
                        TargetIndex::PRIMARY,
                    )),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 241 — Woodland Acolyte // Mend the Wilds
pub(in crate::card::sets) static WOODLAND_ACOLYTE: CardRecord = CardRecord::new(
    "Woodland Acolyte // Mend the Wilds",
    "b9f10623-4783-4773-b9c8-a5a2bcfdb5d9",
    "Steve Prescott",
    CardRules::new_creature(mana_cost!("{2}{W}"), &const { ["Human", "Cleric"] }, 2, 2)
        .with_abilities(
            &const {
                [abilities::enters_trigger(
                    "When this creature enters, draw a card.",
                    abilities::draw_cards(ValueDef::Constant(1)),
                )]
            },
        ),
)
.with_composition(|| {
    adventure(
        &WOODLAND_ACOLYTE,
        "Mend the Wilds",
        &CardRules::new_instant(mana_cost!("{G}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Put target permanent card from your graveyard on top of your \
library. (Then exile this card. You may cast the creature \
later from exile.)",
                    &const {
                        [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::AnyOf(
                                    &const {
                                        [
                                            ObjectPredicateDef::HasType(CardType::Creature),
                                            ObjectPredicateDef::HasType(CardType::Artifact),
                                            ObjectPredicateDef::HasType(CardType::Enchantment),
                                            ObjectPredicateDef::HasType(CardType::Land),
                                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                                        ]
                                    },
                                ),
                                zones: &const { [ZoneKind::Graveyard] },
                                controller: None,
                                owner: Some(PlayerRelation::You),
                            },
                        )]
                    },
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Library,
                        ZonePlacement::Top,
                    ),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 242 — Agatha's Soul Cauldron
// Audit: unsupported — Needs a reflexive trigger with targets or modes chosen after its preceding payment or event, retained even if the source has left the battlefield; choosing them with the original ability changes response timing and target legality.
pub(in crate::card::sets) static AGATHAS_SOUL_CAULDRON: CardRecord = CardRecord::new(
    "Agatha's Soul Cauldron",
    "019b51b0-e5c6-4208-922b-7736686dddcd",
    "Jason A. Engle",
    CardRules::unsupported(),
);

// WOE 243 — Candy Trail
pub(in crate::card::sets) static CANDY_TRAIL: CardRecord = CardRecord::new(
    "Candy Trail",
    "1a860925-d912-49e5-9ddc-41ab26916bb3",
    "Alix Branwyn",
    // A one-mana artifact that smooths the draw now and replaces itself
    // later, which is what makes it a fine card in a deck that just wants
    // its land drops.
    CardRules::new_artifact(mana_cost!("{1}"))
        // Food and Clue are printed types here rather than granted rules:
        // the sacrifice ability this card wants is its own, not either
        // token's.
        .with_subtypes(&["Food", "Clue"])
        .with_abilities(&[
            abilities::enters_trigger(
                "When this artifact enters, scry 2.",
                abilities::scry(ValueDef::Constant(2)),
            ),
            AbilityDef::activated(
                "{2}, {T}, Sacrifice this artifact: You gain 3 life and draw a \
card.",
                &[
                    CostDef::Mana(mana_cost!("{2}")),
                    CostDef::TapSource,
                    CostDef::SacrificeSource,
                ],
                EffectDef::Sequence(&[
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
        ]),
);

// WOE 244 — Collector's Vault
pub(in crate::card::sets) static COLLECTOR_S_VAULT: CardRecord = CardRecord::new(
    "Collector's Vault",
    "967a4f27-8cd6-437d-8c05-8aedbc7ddc4b",
    "Artur Nakhodkin",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[AbilityDef::activated(
        "{2}, {T}: Draw a card, then discard a card. Create a Treasure \
token. (It's an artifact with \"{T}, Sacrifice this token: \
Add one mana of any color.\")",
        &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
        EffectDef::Sequence(&[
            abilities::draw_cards(ValueDef::Constant(1)),
            EffectDef::Discard {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ]),
    )]),
);

// WOE 245 — Eriette's Tempting Apple
pub(in crate::card::sets) static ERIETTE_S_TEMPTING_APPLE: CardRecord = CardRecord::new(
    "Eriette's Tempting Apple",
    "5b61711f-8982-4ff0-86ba-01e0125cd705",
    "Alayna Danner",
    CardRules::new_artifact(mana_cost!("{4}"))
        .with_subtypes(&["Food"])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When Eriette's Tempting Apple enters, gain control of target \
creature until end of turn. Untap that creature. It gains \
haste until end of turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Sequence(&[
                    EffectDef::Perform(GameActionDef::GainControl {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        controller: PlayerRefDef::EffectController,
                        duration: ControlDurationDef::UntilEndOfTurn,
                    }),
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
            AbilityDef::activated(
                "{2}, {T}, Sacrifice Eriette's Tempting Apple: You gain 3 life.",
                &[
                    CostDef::Mana(mana_cost!("{2}")),
                    CostDef::TapSource,
                    CostDef::SacrificeSource,
                ],
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            ),
            AbilityDef::activated_with_targets(
                "{2}, {T}, Sacrifice Eriette's Tempting Apple: Target opponent \
loses 3 life.",
                &[
                    CostDef::Mana(mana_cost!("{2}")),
                    CostDef::TapSource,
                    CostDef::SacrificeSource,
                ],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(3),
                },
            ),
        ]),
);

// WOE 246 — Gingerbrute (reprint)
const GINGERBRUTE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::GINGERBRUTE,
    "09a4578a-7dc6-4da3-93ee-913b10be5740",
    "Carlos Palma Cruchaga",
);

// WOE 247 — Hylda's Crown of Winter
pub(in crate::card::sets) static HYLDA_S_CROWN_OF_WINTER: CardRecord = CardRecord::new(
    "Hylda's Crown of Winter",
    "b0d4a6c0-f00e-45a7-9c88-899460007020",
    "Volkan Baǵa",
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "{1}, {T}: Tap target creature. This ability costs {1} less to \
activate during your turn.",
                &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            )
            .with_activation_cost_reduction(
                ValueDef::IfCondition(&ConditionValueDef {
                    condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                    then: ValueDef::Constant(1),
                    otherwise: ValueDef::Constant(0),
                }),
                0,
            ),
            AbilityDef::activated(
                "{3}, Sacrifice Hylda's Crown of Winter: Draw a card for each \
tapped creature your opponents control.",
                &[CostDef::Mana(mana_cost!("{3}")), CostDef::SacrificeSource],
                abilities::draw_cards(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Tapped,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                ))),
            ),
        ]),
);

// WOE 248 — The Irencrag
// Audit: unsupported — Needs persistent name and Equipment-subtype replacement together with executable static-ability grants; the existing persistent grant boundary rejects the equipped-creature bonus.
pub(in crate::card::sets) static THE_IRENCRAG: CardRecord = CardRecord::new(
    "The Irencrag",
    "8051c5ec-54a6-45a8-8945-fb93c5feaa39",
    "Adam Paquette",
    CardRules::unsupported(),
);

// WOE 249 — Prophetic Prism (reprint)
const PROPHETIC_PRISM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_roe::PROPHETIC_PRISM,
    "1fae351c-b918-4648-a361-d5239ae63156",
    "Quintin Gleim",
);

// WOE 250 — Scarecrow Guide
pub(in crate::card::sets) static SCARECROW_GUIDE: CardRecord = CardRecord::new(
    "Scarecrow Guide",
    "3dc08461-bec6-4a18-b782-da4c92abb789",
    "Carlos Palma Cruchaga",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Scarecrow"], 2, 1).with_abilities(&[
        abilities::reach(),
        AbilityDef::activated_mana(
            "{1}: Add one mana of any color. Activate only once each turn.",
            &[CostDef::Mana(mana_cost!("{1}"))],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        )
        .once_each_turn(),
    ]),
);

// WOE 251 — Soul-Guide Lantern (reprint)
const SOUL_GUIDE_LANTERN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_thb::SOUL_GUIDE_LANTERN,
    "13571173-29e7-4915-af5f-05f13b463061",
    "Iris Compiet",
);

// WOE 252 — Syr Ginger, the Meal Ender
pub(in crate::card::sets) static SYR_GINGER_THE_MEAL_ENDER: CardRecord = CardRecord::new(
    "Syr Ginger, the Meal Ender",
    "7fbdba12-1369-41ae-b0e9-c405c0f0a2e5",
    "Michal Ivan",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Food", "Knight"], 3, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Syr Ginger has trample, hexproof, and haste as long as an \
opponent controls a planeswalker.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Opponent,
                        ),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 1,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::add_ability(&abilities::trample()),
                            AppliedEffectDef::add_ability(&abilities::hexproof()),
                            AppliedEffectDef::add_ability(&abilities::haste()),
                        ]),
                    },
                },
            ),
            AbilityDef::triggered(
                "Whenever another artifact you control is put into a graveyard \
from the battlefield, put a +1/+1 counter on Syr Ginger and \
scry 1.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    abilities::scry(ValueDef::Constant(1)),
                ]),
            ),
            AbilityDef::activated(
                "{2}, {T}, Sacrifice Syr Ginger: You gain life equal to its power.",
                &[
                    CostDef::Mana(mana_cost!("{2}")),
                    CostDef::TapSource,
                    CostDef::SacrificeSource,
                ],
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::SourcePower,
                },
            ),
        ]),
);

// WOE 253 — Three Bowls of Porridge
// Audit: unsupported — Needs modal activation history scoped to the source object, preventing reuse of each chosen mode across turns.
pub(in crate::card::sets) static THREE_BOWLS_OF_PORRIDGE: CardRecord = CardRecord::new(
    "Three Bowls of Porridge",
    "a508e040-d1e5-46aa-8404-1adc18f0f8bd",
    "Edgar Sánchez Hidalgo",
    CardRules::unsupported(),
);

// WOE 254 — Crystal Grotto (reprint)
const CRYSTAL_GROTTO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dmu::CRYSTAL_GROTTO,
    "6f6f9d3d-600d-43f3-a915-612e5d53aaa1",
    "Andreas Zafiratos",
);

// WOE 255 — Edgewall Inn
// Audit: unsupported — Needs an object predicate for a card having an Adventure, independently of its currently presented face; the model exposes no Adventure-composition predicate.
pub(in crate::card::sets) static EDGEWALL_INN: CardRecord = CardRecord::new(
    "Edgewall Inn",
    "ec435e54-628a-43bd-8804-cbc37e375bce",
    "Alayna Danner",
    CardRules::unsupported(),
);

// WOE 256 — Evolving Wilds (reprint)
const EVOLVING_WILDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_roe::EVOLVING_WILDS,
    "74f9c819-719a-461b-8e7e-a26c88e8099b",
    "Alayna Danner",
);

// WOE 257 — Restless Bivouac
pub(in crate::card::sets) static RESTLESS_BIVOUAC: CardRecord = CardRecord::new(
    "Restless Bivouac",
    "b85e0aed-bfb2-4aa8-a754-849c4d9a6a58",
    "Sergey Glushakov",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for_mana(
            "{T}: Add {R} or {W}.",
            AddManaEffectDef::choice(&[ManaColor::Red, ManaColor::White]),
        ),
        AbilityDef::activated(
            "{1}{R}{W}: This land becomes a 2/2 red and white Ox creature \
until end of turn. It's still a land.",
            &[CostDef::Mana(mana_cost!("{1}{R}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::set_colors(ColorSet::from_colors(&[
                        ManaColor::Red,
                        ManaColor::White,
                    ])),
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Ox"])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::triggered_with_targets(
            "Whenever this land attacks, put a +1/+1 counter on target \
creature you control.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
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
        ),
    ]),
);

// WOE 258 — Restless Cottage
pub(in crate::card::sets) static RESTLESS_COTTAGE: CardRecord = CardRecord::new(
    "Restless Cottage",
    "787eadf3-5005-4ae5-820f-4012a4d4e1a5",
    "Jesper Ejsing",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for_mana(
            "{T}: Add {B} or {G}.",
            AddManaEffectDef::choice(&[ManaColor::Black, ManaColor::Green]),
        ),
        AbilityDef::activated(
            "{2}{B}{G}: This land becomes a 4/4 black and green Horror \
creature until end of turn. It's still a land.",
            &[CostDef::Mana(mana_cost!("{2}{B}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::set_colors(ColorSet::from_colors(&[
                        ManaColor::Black,
                        ManaColor::Green,
                    ])),
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Horror"])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(4),
                        ValueDef::Constant(4),
                    ),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::triggered_with_targets(
            "Whenever this land attacks, create a Food token and exile up \
to one target card from a graveyard.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef {
                minimum: 0,
                ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                })
            }],
            EffectDef::Sequence(&[
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            ]),
        ),
    ]),
);

// WOE 259 — Restless Fortress
pub(in crate::card::sets) static RESTLESS_FORTRESS: CardRecord = CardRecord::new(
    "Restless Fortress",
    "675213bb-28d7-460c-a4f3-950f5b9090af",
    "Piotr Dura",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for_mana(
            "{T}: Add {W} or {B}.",
            AddManaEffectDef::choice(&[ManaColor::White, ManaColor::Black]),
        ),
        AbilityDef::activated(
            "{2}{W}{B}: This land becomes a 1/4 white and black Nightmare \
creature until end of turn. It's still a land.",
            &[CostDef::Mana(mana_cost!("{2}{W}{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::set_colors(ColorSet::from_colors(&[
                        ManaColor::White,
                        ManaColor::Black,
                    ])),
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Nightmare"])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(4),
                    ),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::triggered(
            "Whenever this land attacks, defending player loses 2 life and \
you gain 2 life.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
    ]),
);

// WOE 260 — Restless Spire
// Audit: unsupported — Needs a temporary grant of an executable static ability whose first-strike condition remains part of the animated creature's abilities; the grant boundary rejects executable statics.
pub(in crate::card::sets) static RESTLESS_SPIRE: CardRecord = CardRecord::new(
    "Restless Spire",
    "66386fe8-9d3c-47f7-9cd3-4cd30051535f",
    "Sergey Glushakov",
    CardRules::unsupported(),
);

// WOE 261 — Restless Vinestalk
pub(in crate::card::sets) static RESTLESS_VINESTALK: CardRecord = CardRecord::new(
    "Restless Vinestalk",
    "e5f3161d-3f69-4b06-ab73-c31fc0c1520c",
    "Sam Burley",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for_mana(
            "{T}: Add {G} or {U}.",
            AddManaEffectDef::choice(&[ManaColor::Green, ManaColor::Blue]),
        ),
        AbilityDef::activated(
            "{3}{G}{U}: Until end of turn, this land becomes a 5/5 green \
and blue Plant creature with trample. It's still a land.",
            &[CostDef::Mana(mana_cost!("{3}{G}{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::set_colors(ColorSet::from_colors(&[
                        ManaColor::Green,
                        ManaColor::Blue,
                    ])),
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Plant"])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(5),
                        ValueDef::Constant(5),
                    ),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::triggered_with_targets(
            "Whenever this land attacks, up to one other target creature \
has base power and toughness 3/3 until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::set_base_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// WOE 262 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "c9cd4d57-8c51-4fcf-8a9f-5d6a61c33e3d",
    "Hari & Deepti",
);

// WOE 263 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "bd4b4da4-83f6-4280-880b-b6033308f2a2",
    "Hari & Deepti",
);

// WOE 264 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "ee68f2cb-851b-4196-ac58-844d72628e6a",
    "Hari & Deepti",
);

// WOE 265 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "8822db23-34dc-452a-92bc-a3ceee4db375",
    "Hari & Deepti",
);

// WOE 266 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "ecd6d8fb-780c-446c-a8bf-93386b22fe95",
    "Hari & Deepti",
);

// WOE 267 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "eed1af19-e075-4e6f-9394-d258143e15a4",
    "Carlos Palma Cruchaga",
);

// WOE 268 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "486fbcf9-3a04-47f6-8927-886c2a454499",
    "Jonas De Ro",
);

// WOE 269 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "6245d2f8-8ef0-4d2a-9abb-99839dc3abf0",
    "Leanna Crossan",
);

// WOE 270 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "1a9798d6-34b3-4438-992d-d3616a7c8536",
    "Sarah Finnigan",
);

// WOE 271 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "d0a801ba-ebf7-4b9e-98c2-db50448845b7",
    "Jonas De Ro",
);

// WOE 272 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "f0de4ea5-77e6-474e-99f9-36192bbd37d5",
    "Julian Kok Joon Wen",
);

// WOE 273 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "8996cd43-5ecd-4a05-a5a9-e49326befaa1",
    "Sarah Finnigan",
);

// WOE 274 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "8e747bba-b521-4f81-9d9a-3e85747cefe9",
    "Julian Kok Joon Wen",
);

// WOE 275 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "a4c99f1b-f304-42d5-bbea-32283b01d43b",
    "Jonas De Ro",
);

// WOE 276 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "1bd51a22-3e0d-4826-aab7-0adbfce4478a",
    "Adam Paquette",
);

// WOE 277 — Virtue of Loyalty
/// "Those creatures" is the same set the clause just counted: nothing joins
/// or leaves the battlefield while one effect resolves, so asking twice and
/// binding the first answer come to the same thing.
static YOUR_CREATURES: EffectRecipientDef = EffectRecipientDef::matching_objects(
    ObjectPredicateDef::HasType(CardType::Creature),
    &const { [ZoneKind::Battlefield] },
    PlayerRelation::You,
);

const fn virtue_of_loyalty_rules() -> CardRules {
    CardRules::new_enchantment(mana_cost!("{3}{W}{W}")).with_ability(AbilityDef::triggered(
        "At the beginning of your end step, put a +1/+1 counter on \
each creature you control. Untap those creatures.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::You,
        },
        EffectDef::Sequence(
            &const {
                [
                    EffectDef::AddCounters {
                        object: YOUR_CREATURES,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Untap {
                        object: YOUR_CREATURES,
                    },
                ]
            },
        ),
    ))
}

fn virtue_of_loyalty_composition() -> CardComposition {
    let virtue = virtue_of_loyalty_rules();
    let fealty = const {
        CardRules::new_instant(mana_cost!("{1}{W}"))
            .with_subtypes(&["Adventure"])
            .with_ability(
                AbilityDef::spell(
                    "Create a 2/2 white Knight creature token with vigilance.",
                    EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::creature(&["Knight"], &[ManaColor::White], 2, 2)
                            .with_abilities(&const { [abilities::vigilance()] }),
                    ))),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            )
    };
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Virtue of Loyalty", virtue),
            CardPart::new(CardPartId(1), "Ardenvale Fealty", fealty),
        ],
        structure: CardStructure::AlternateSpell {
            main: CardPartId::PRIMARY,
            alternate: CardPartId(1),
            kind: AlternateSpellKind::Adventure,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Virtue of Loyalty",
                SpellForm::Part(CardPartId::PRIMARY),
                virtue
                    .mana_cost()
                    .expect("the enchantment has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Ardenvale Fealty",
                SpellForm::Part(CardPartId(1)),
                fealty
                    .mana_cost()
                    .expect("the Adventure has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

pub(in crate::card::sets) static VIRTUE_OF_LOYALTY: CardRecord = CardRecord::new(
    "Virtue of Loyalty",
    "9622e597-dc7c-4198-9ce5-4df53bb0c96c",
    "Keith Garletts",
    virtue_of_loyalty_rules(),
)
.with_composition(virtue_of_loyalty_composition);

// WOE 278 — Horned Loch-Whale // Lagoon Breach (alternate printing)
const HORNED_LOCH_WHALE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HORNED_LOCH_WHALE,
    1,
    "af969428-8dd2-47d6-bdbf-65eca632c3d4",
    "Nils Hamm",
);

// WOE 279 — Virtue of Knowledge // Vantress Visions (alternate printing)
const VIRTUE_OF_KNOWLEDGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VIRTUE_OF_KNOWLEDGE,
    1,
    "7cc2173a-b7fb-4bd6-9c8e-73de91c6903a",
    "Josu Hernaiz",
);

// WOE 280 — Gumdrop Poisoner // Tempt with Treats (alternate printing)
const GUMDROP_POISONER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GUMDROP_POISONER,
    1,
    "ec5d0453-ef17-47f0-81d4-13941f1380d5",
    "Allen Williams",
);

// WOE 281 — Virtue of Persistence // Locthwain Scorn (alternate printing)
const VIRTUE_OF_PERSISTENCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VIRTUE_OF_PERSISTENCE,
    1,
    "337393e3-9081-4251-b4ca-afd7ce10dc99",
    "Allen Williams",
);

// WOE 282 — Virtue of Courage // Embereth Blaze (alternate printing)
const VIRTUE_OF_COURAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VIRTUE_OF_COURAGE,
    1,
    "ed585964-68ed-4889-8d20-678b43c46018",
    "Nils Hamm",
);

// WOE 283 — Bramble Familiar // Fetch Quest (alternate printing)
const BRAMBLE_FAMILIAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRAMBLE_FAMILIAR,
    1,
    "1ee74605-63be-41cd-a6ba-1b33f8094ec9",
    "Alayna Danner",
);

// WOE 284 — Virtue of Strength // Garenbrig Growth (alternate printing)
const VIRTUE_OF_STRENGTH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VIRTUE_OF_STRENGTH,
    1,
    "684f8568-390f-426f-ba71-e4be5fdaceee",
    "Danny Schwartz",
);

// WOE 285 — Beluna Grandsquall // Seek Thrills (alternate printing)
const BELUNA_GRANDSQUALL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BELUNA_GRANDSQUALL,
    1,
    "5ed9d78f-a556-435f-b95f-7317ae66e5e3",
    "Kev Walker",
);

// WOE 286 — Cruel Somnophage // Can't Wake Up (alternate printing)
const CRUEL_SOMNOPHAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CRUEL_SOMNOPHAGE,
    1,
    "852d840c-c9b5-4486-b1f5-95ea88577fa0",
    "Kev Walker",
);

// WOE 287 — Decadent Dragon // Expensive Taste (alternate printing)
const DECADENT_DRAGON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DECADENT_DRAGON,
    1,
    "8a717d27-596d-4341-b592-4f9777f778e5",
    "Aaron Miller",
);

// WOE 288 — Devouring Sugarmaw // Have for Dinner (alternate printing)
const DEVOURING_SUGARMAW_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEVOURING_SUGARMAW,
    1,
    "f2848594-1718-43f2-8ccf-34a18a55e4b2",
    "Kev Walker",
);

// WOE 289 — Elusive Otter // Grove's Bounty (alternate printing)
const ELUSIVE_OTTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELUSIVE_OTTER,
    1,
    "c5a61619-f951-42ff-8246-c51ee5dc18c8",
    "Andrea Sipl",
);

// WOE 290 — Heartflame Duelist // Heartflame Slash (alternate printing)
const HEARTFLAME_DUELIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HEARTFLAME_DUELIST,
    1,
    "f54b25e1-193a-41fb-9d24-3147cb381dbe",
    "Allen Williams",
);

// WOE 291 — Kellan, the Fae-Blooded // Birthright Boon (alternate printing)
const KELLAN_THE_FAE_BLOODED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KELLAN_THE_FAE_BLOODED,
    1,
    "e662dfa5-edf8-4e7b-8a29-86935e95ac35",
    "Omar Rayyan",
);

// WOE 292 — Mosswood Dreadknight // Dread Whispers (alternate printing)
const MOSSWOOD_DREADKNIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOSSWOOD_DREADKNIGHT,
    1,
    "2ab293c3-64cf-4d8a-909d-4cf73ffd828a",
    "Keith Garletts",
);

// WOE 293 — Pollen-Shield Hare // Hare Raising (alternate printing)
const POLLEN_SHIELD_HARE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &POLLEN_SHIELD_HARE,
    1,
    "f2d517ad-6df7-4cf9-982f-763379724d24",
    "Greg Hildebrandt",
);

// WOE 294 — Questing Druid // Seek the Beast (alternate printing)
const QUESTING_DRUID_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &QUESTING_DRUID,
    1,
    "c6406eba-da58-4264-a213-20e22c1c3bec",
    "Greg Hildebrandt",
);

// WOE 295 — Scalding Viper // Steam Clean (alternate printing)
const SCALDING_VIPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SCALDING_VIPER,
    1,
    "3014de04-66ce-4641-b36f-6ff1cc1c116a",
    "Omar Rayyan",
);

// WOE 296 — Twining Twins // Swift Spiral (alternate printing)
const TWINING_TWINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TWINING_TWINS,
    1,
    "38708b07-df7d-4ea4-b10e-ead7cd03d849",
    "Alayna Danner",
);

// WOE 297 — Ashiok, Wicked Manipulator (alternate printing)
const ASHIOK_WICKED_MANIPULATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ASHIOK_WICKED_MANIPULATOR,
    1,
    "dd6f57de-21af-4c25-8a21-df3a75157939",
    "Serena Malyon",
);

// WOE 298 — Kellan, the Fae-Blooded // Birthright Boon (alternate printing)
const KELLAN_THE_FAE_BLOODED_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KELLAN_THE_FAE_BLOODED,
    2,
    "aa5b622a-66a6-4097-8478-e22854b9984d",
    "Andreas Zafiratos",
);

// WOE 299 — Eriette of the Charmed Apple (alternate printing)
const ERIETTE_OF_THE_CHARMED_APPLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ERIETTE_OF_THE_CHARMED_APPLE,
    1,
    "487a2cc0-ddbf-46b9-90ed-9455347724d5",
    "Zach Alexander",
);

// WOE 300 — Rowan, Scion of War (alternate printing)
const ROWAN_SCION_OF_WAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROWAN_SCION_OF_WAR,
    1,
    "844e29e9-e870-4433-ad63-0ff918a3e41a",
    "Abigail Larson",
);

// WOE 301 — Talion, the Kindly Lord (alternate printing)
const TALION_THE_KINDLY_LORD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TALION_THE_KINDLY_LORD,
    1,
    "bb1f05ef-7607-4a63-a710-18d0061ec982",
    "Olena Richards",
);

// WOE 302 — Will, Scion of Peace (alternate printing)
const WILL_SCION_OF_PEACE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WILL_SCION_OF_PEACE,
    1,
    "0f25e3db-19ae-4f89-80ec-bf0ad561be39",
    "Matteo Marjoram",
);

// WOE 303 — Restless Bivouac (alternate printing)
const RESTLESS_BIVOUAC_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RESTLESS_BIVOUAC,
    1,
    "901796b2-c567-41e2-a87b-6147a966c9a8",
    "Lucas Graciano",
);

// WOE 304 — Restless Cottage (alternate printing)
const RESTLESS_COTTAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RESTLESS_COTTAGE,
    1,
    "0dcb97ff-9356-4be7-8991-091f950c0b63",
    "Sergey Glushakov",
);

// WOE 305 — Restless Fortress (alternate printing)
const RESTLESS_FORTRESS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RESTLESS_FORTRESS,
    1,
    "8034589b-3b57-49be-b0fd-a306f915d864",
    "Alayna Danner",
);

// WOE 306 — Restless Spire (alternate printing)
const RESTLESS_SPIRE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RESTLESS_SPIRE,
    1,
    "dc42bb4f-475e-4c76-bcbd-a9d8cf03117c",
    "Sergey Glushakov",
);

// WOE 307 — Restless Vinestalk (alternate printing)
const RESTLESS_VINESTALK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RESTLESS_VINESTALK,
    1,
    "a710643a-b9dd-49bf-a375-d9986b05ed7a",
    "Sergey Glushakov",
);

// WOE 308 — Food Coma
// Audit: unsupported — Needs an immediate linked return when the source leaves, without a return trigger using the stack; the available exile-until helper installs a triggered return.
pub(in crate::card::sets) static FOOD_COMA: CardRecord = CardRecord::new(
    "Food Coma",
    "61ba2aed-3514-4db9-8da0-329620b12f63",
    "Iris Compiet",
    CardRules::unsupported(),
);

// WOE 309 — Lady of Laughter
// Audit: unsupported — Needs per-player nonland-permanent entry history for the current turn, including objects that have since left; current battlefield entry timestamps cannot implement celebration.
pub(in crate::card::sets) static LADY_OF_LAUGHTER: CardRecord = CardRecord::new(
    "Lady of Laughter",
    "c26714f9-d42f-4bac-b185-8943d1621444",
    "Kai Carpenter",
    CardRules::unsupported(),
);

// WOE 310 — Pests of Honor
// Audit: unsupported — Needs per-player nonland-permanent entry history for the current turn, including objects that have since left; current battlefield entry timestamps cannot implement celebration.
pub(in crate::card::sets) static PESTS_OF_HONOR: CardRecord = CardRecord::new(
    "Pests of Honor",
    "5671c2a0-51e8-4d5e-8409-87abd6c0a8ab",
    "Quintin Gleim",
    CardRules::unsupported(),
);

// WOE 311 — Faerie Slumber Party
// Audit: unsupported — Needs returned-permanent outcomes grouped by their pre-return controller, including tokens that cease to exist and excluding replacement moves to other zones; existing successor bindings do not preserve that complete outcome group.
pub(in crate::card::sets) static FAERIE_SLUMBER_PARTY: CardRecord = CardRecord::new(
    "Faerie Slumber Party",
    "f8e5de58-cc4c-40b2-94c8-4e4d7cebfaf8",
    "Lie Setiawan",
    CardRules::unsupported(),
);

// WOE 312 — Rowdy Research
// Audit: unsupported — Needs the number of creatures that attacked this turn, including creatures that have since left the battlefield; live attacking or attacked-this-turn object queries lose those objects.
pub(in crate::card::sets) static ROWDY_RESEARCH: CardRecord = CardRecord::new(
    "Rowdy Research",
    "89b7e901-5ba4-4374-9eeb-96354279a123",
    "Bram Sels",
    CardRules::unsupported(),
);

// WOE 313 — Storyteller Pixie
pub(in crate::card::sets) static STORYTELLER_PIXIE: CardRecord = CardRecord::new(
    "Storyteller Pixie",
    "b9f4cf85-2120-4897-aecd-4ee4b02d6f9b",
    "Peter Polach",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Faerie", "Wizard"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever you cast an Adventure spell, draw a card.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Adventure")),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// WOE 314 — Experimental Confectioner
pub(in crate::card::sets) static EXPERIMENTAL_CONFECTIONER: CardRecord = CardRecord::new(
    "Experimental Confectioner",
    "49aaf5db-9418-4b97-97da-736c674905d4",
    "Gaboleps",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Peasant"], 2, 3)
        .with_abilities(&[abilities::enters_trigger(
            "When this creature enters, create a Food token. (It's an \
artifact with \"{2}, {T}, Sacrifice this token: You gain 3 \
life.\")",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        )])
        .with_ability(AbilityDef::triggered(
            "Whenever you sacrifice a Food, create a 1/1 black Rat \
creature token with \"This token can't block.\"",
            TriggerEventDef::Sacrificed {
                object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Food")),
                player: PlayerRelation::You,
            },
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                DEFENSELESS_RAT_TOKEN,
            ))),
        )),
);

// WOE 315 — Malevolent Witchkite
pub(in crate::card::sets) static MALEVOLENT_WITCHKITE: CardRecord = CardRecord::new(
    "Malevolent Witchkite",
    "d6cb3c6d-560d-40ca-b5c0-27322863cead",
    "Donato Giancola",
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Dragon", "Warlock"], 5, 4).with_abilities(
        &[
            abilities::flying(),
            abilities::enters_trigger(
                "When this creature enters, sacrifice any number of artifacts, \
enchantments, and/or tokens, then draw that many cards.",
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::Token,
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 0,
                    maximum: usize::MAX,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::Sequence(&[
                        EffectDef::sacrifice(EffectRecipientDef::objects(ObjectSetDef::Binding(
                            crate::Binding!("chosen"),
                        ))),
                        abilities::draw_cards(ValueDef::BoundObjectCount(crate::Binding!(
                            "chosen"
                        ))),
                    ]),
                }),
            ),
        ],
    ),
);

// WOE 316 — Old Flitterfang
pub(in crate::card::sets) static OLD_FLITTERFANG: CardRecord = CardRecord::new(
    "Old Flitterfang",
    "67c77d6f-de14-423c-bf55-0fb289171004",
    "Chris Seaman",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Rat", "Faerie"], 3, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::triggered_if(
                "At the beginning of each end step, if a creature died this \
turn, create a Food token. (It's an artifact with \"{2}, {T}, \
Sacrifice this token: You gain 3 life.\")",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::Any,
                },
                &TriggerConditionDef::CreatureDiedThisTurn,
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ),
            AbilityDef::activated(
                "{2}{B}, Sacrifice another creature or artifact: Old \
Flitterfang gets +2/+2 until end of turn.",
                &[
                    CostDef::Mana(mana_cost!("{2}{B}")),
                    CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Artifact),
                        ]),
                    ])),
                ],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// WOE 317 — Become Brutes
pub(in crate::card::sets) static BECOME_BRUTES: CardRecord = CardRecord::new(
    "Become Brutes",
    "a154dadc-be5b-4ad6-9946-bdc54c251bff",
    "Julia Griffin",
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "One or two target creatures each gain haste until end of \
turn. For each of those creatures, create a Monster Role \
token attached to it. (If you control another Role on it, put \
that one into the graveyard. Enchanted creature gets +1/+1 \
and has trample.)",
        &[AbilityTargetDef {
            minimum: 1,
            maximum: 2,
            ..AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Creature,
            ))
        }],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::CreateAttachedToken {
                token: MONSTER_ROLE,
                host: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
            },
        ]),
    )]),
);

// WOE 318 — Charging Hooligan
pub(in crate::card::sets) static CHARGING_HOOLIGAN: CardRecord = CardRecord::new(
    "Charging Hooligan",
    "fddc6f47-202d-4764-abc1-ee453a8917c2",
    "Tomas Duchek",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Peasant"], 3, 3).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks, it gets +1/+0 until end of \
turn for each attacking creature. If a Rat is attacking, this \
creature gains trample until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Attacking,
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        )),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rat")),
                                ObjectPredicateDef::Attacking,
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 1,
                    },
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::add_ability(&abilities::trample()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                },
            ]),
        ),
    ]),
);

// WOE 319 — Ogre Chitterlord
pub(in crate::card::sets) static OGRE_CHITTERLORD: CardRecord = CardRecord::new(
    "Ogre Chitterlord",
    "a70b2033-eec5-4c86-9ebe-a68960a86763",
    "Piotr Foksowicz",
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Ogre", "Warrior"], 6, 5).with_abilities(&[
        abilities::menace(),
        AbilityDef::triggered(
            "Whenever this creature enters or attacks, create two 1/1 \
black Rat creature tokens with \"This token can't block.\" \
Then if you control five or more Rats, each Rat you control \
gets +2/+0 until end of turn.",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
            ]),
            EffectDef::Sequence(&[
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(DEFENSELESS_RAT_TOKEN))
                        .with_count(ValueDef::Constant(2)),
                ),
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rat")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 5,
                    },
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rat")),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                        )),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(0),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                },
            ]),
        ),
    ]),
);

// WOE 320 — Intrepid Trufflesnout // Go Hog Wild
pub(in crate::card::sets) static INTREPID_TRUFFLESNOUT: CardRecord = CardRecord::new(
    "Intrepid Trufflesnout // Go Hog Wild",
    "4224747e-1dbc-4a29-b5da-5916d8ca2768",
    "Kisung Koh",
    CardRules::new_creature(mana_cost!("{1}{G}"), &const { ["Boar"] }, 3, 1).with_abilities(
        &const {
            [AbilityDef::triggered(
                "Whenever this creature attacks alone, create a Food token. \
(It's an artifact with \"{2}, {T}, Sacrifice this token: You \
gain 3 life.\")",
                TriggerEventDef::attacks_in_declaration(ObjectPredicateDef::Source, 1, Some(1)),
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            )]
        },
    ),
)
.with_composition(|| {
    adventure(
        &INTREPID_TRUFFLESNOUT,
        "Go Hog Wild",
        &CardRules::new_instant(mana_cost!("{1}{G}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Target creature gets +2/+2 until end of turn. (Then exile \
this card. You may cast the creature later from exile.)",
                    &const {
                        [AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )]
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            ),
    )
});

// WOE 321 — Provisions Merchant
pub(in crate::card::sets) static PROVISIONS_MERCHANT: CardRecord = CardRecord::new(
    "Provisions Merchant",
    "a015282d-bcd4-44ab-ae26-41e4e3b23fc0",
    "Raluca Marinescu",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Beast", "Peasant"], 3, 3).with_abilities(
        &[
            abilities::enters_trigger(
                "When this creature enters, create a Food token. (It's an \
artifact with \"{2}, {T}, Sacrifice this token: You gain 3 \
life.\")",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ),
            AbilityDef::triggered(
                "Whenever this creature attacks, you may sacrifice a Food. If \
you do, attacking creatures get +1/+1 and gain trample until \
end of turn.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::PayOr(PayOrDef::optional(
                    &[CostDef::sacrifice_permanent(ObjectPredicateDef::Subtype(
                        SubtypeDef::Literal("Food"),
                    ))],
                    &EffectDef::Apply {
                        recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::Attacking,
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::Any,
                            ),
                        )),
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(1),
                                ValueDef::Constant(1),
                            ),
                            AppliedEffectDef::add_ability(&abilities::trample()),
                        ]),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                )),
            ),
        ],
    ),
);

// WOE 322 — Wildwood Mentor
pub(in crate::card::sets) static WILDWOOD_MENTOR: CardRecord = CardRecord::new(
    "Wildwood Mentor",
    "96c247f4-06cf-4c41-8285-d44d40f4130c",
    "Piotr Foksowicz",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Treefolk"], 1, 1)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever a token you control enters, put a +1/+1 counter on \
this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Token,
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )])
        .with_ability(AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, another target attacking \
creature gets +X/+X until end of turn, where X is this \
creature's power.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ObjectPredicateDef::Attacking,
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::SourcePower,
                    ValueDef::SourcePower,
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )),
);

// WOE 323 — Archon of the Wild Rose (alternate printing)
const ARCHON_OF_THE_WILD_ROSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARCHON_OF_THE_WILD_ROSE,
    1,
    "cd2b19f1-2f92-458d-a658-25481c57dea0",
    "Chris Rahn",
);

// WOE 324 — Expel the Interlopers (alternate printing)
const EXPEL_THE_INTERLOPERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EXPEL_THE_INTERLOPERS,
    1,
    "92f5907a-2d70-46a9-b9f9-0ae7bcf35d48",
    "Andreas Zafiratos",
);

// WOE 325 — Moonshaker Cavalry (alternate printing)
const MOONSHAKER_CAVALRY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOONSHAKER_CAVALRY,
    1,
    "08e06198-b96a-4b52-ba86-3df8278c0785",
    "Aldo Domínguez",
);

// WOE 326 — Regal Bunnicorn (alternate printing)
const REGAL_BUNNICORN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REGAL_BUNNICORN,
    1,
    "6e650b3f-30a7-4943-8829-055e8aa7b9cd",
    "Ilse Gort",
);

// WOE 327 — Spellbook Vendor (alternate printing)
const SPELLBOOK_VENDOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPELLBOOK_VENDOR,
    1,
    "6973853c-2e95-4b5b-9008-4b06ffc8a078",
    "Scott Murphy",
);

// WOE 328 — A Tale for the Ages (alternate printing)
const A_TALE_FOR_THE_AGES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &A_TALE_FOR_THE_AGES,
    1,
    "4fc7d7d8-0965-4097-98ee-e89cfa05fd05",
    "Julie Dillon",
);

// WOE 329 — Werefox Bodyguard (alternate printing)
const WEREFOX_BODYGUARD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WEREFOX_BODYGUARD,
    1,
    "572eff52-c141-4587-857b-e71050899bdb",
    "Néstor Ossandón Leal",
);

// WOE 330 — Asinine Antics (alternate printing)
const ASININE_ANTICS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ASININE_ANTICS,
    1,
    "dba9c281-a6e0-4af9-a217-e6aee7a38ff0",
    "Brent Hollowell",
);

// WOE 331 — Extraordinary Journey (alternate printing)
const EXTRAORDINARY_JOURNEY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EXTRAORDINARY_JOURNEY,
    1,
    "abd423cc-7a86-4b15-b591-85940faa323e",
    "Volkan Baǵa",
);

// WOE 332 — Farsight Ritual (alternate printing)
const FARSIGHT_RITUAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FARSIGHT_RITUAL,
    1,
    "a3f484b9-486f-4aff-8b23-d8e555b476d3",
    "Randy Gallegos",
);

// WOE 333 — Ingenious Prodigy (alternate printing)
const INGENIOUS_PRODIGY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INGENIOUS_PRODIGY,
    1,
    "49bf6c34-429b-4213-856b-e8189276f2db",
    "Brian Valeza",
);

// WOE 334 — Sleep-Cursed Faerie (alternate printing)
const SLEEP_CURSED_FAERIE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SLEEP_CURSED_FAERIE,
    1,
    "5cb94d9f-d679-4eb2-97d4-b519d8a166b0",
    "Heonhwa",
);

// WOE 335 — Talion's Messenger (alternate printing)
const TALION_S_MESSENGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TALION_S_MESSENGER,
    1,
    "b54e69e4-c6c9-4735-ab86-c437eba8d4e2",
    "Marta Nael",
);

// WOE 336 — Beseech the Mirror (alternate printing)
const BESEECH_THE_MIRROR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BESEECH_THE_MIRROR,
    1,
    "5df3d7fa-2b43-4b9e-85eb-ef96537942fb",
    "Cynthia Sheppard",
);

// WOE 337 — The End (alternate printing)
const THE_END_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_END,
    1,
    "79220c46-01df-41c1-877e-577f9ce78593",
    "Donato Giancola",
);

// WOE 338 — Lich-Knights' Conquest (alternate printing)
const LICH_KNIGHTS_CONQUEST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LICH_KNIGHTS_CONQUEST,
    1,
    "edee2cc6-955c-43fb-b359-9d0e261c6925",
    "Denis Zhbankov",
);

// WOE 339 — Lord Skitter, Sewer King (alternate printing)
const LORD_SKITTER_SEWER_KING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LORD_SKITTER_SEWER_KING,
    1,
    "728520b3-fd90-46e4-ad51-3121a2557e86",
    "Jesper Ejsing",
);

// WOE 340 — Lord Skitter's Blessing (alternate printing)
const LORD_SKITTER_S_BLESSING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LORD_SKITTER_S_BLESSING,
    1,
    "77c78dde-4059-4e8d-9feb-f03653773787",
    "Joseph Weston",
);

// WOE 341 — Rankle's Prank (alternate printing)
const RANKLE_S_PRANK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RANKLE_S_PRANK,
    1,
    "c342061c-6992-47d2-b862-db310d389b3d",
    "Tyler Walpole",
);

// WOE 342 — Specter of Mortality (alternate printing)
const SPECTER_OF_MORTALITY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPECTER_OF_MORTALITY,
    1,
    "3026523f-65a0-49a4-a46e-bb91a8988ea4",
    "Daarken",
);

// WOE 343 — Spiteful Hexmage (alternate printing)
const SPITEFUL_HEXMAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPITEFUL_HEXMAGE,
    1,
    "d527d48d-d6f7-4587-955e-de23b80ce0d6",
    "Anna Steinbauer",
);

// WOE 344 — Tangled Colony (alternate printing)
const TANGLED_COLONY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TANGLED_COLONY,
    1,
    "45ba866b-eda5-45c0-8878-235abadc0a3c",
    "Filip Burburan",
);

// WOE 345 — Charming Scoundrel (alternate printing)
const CHARMING_SCOUNDREL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHARMING_SCOUNDREL,
    1,
    "51f94c92-cf88-4770-8ab8-874cd6634510",
    "Caroline Gariba",
);

// WOE 346 — Food Fight (alternate printing)
const FOOD_FIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FOOD_FIGHT,
    1,
    "fef34fc2-8d37-484d-9adf-e3bb70283f33",
    "Filipe Pagliuso",
);

// WOE 347 — Goddric, Cloaked Reveler (alternate printing)
const GODDRIC_CLOAKED_REVELER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GODDRIC_CLOAKED_REVELER,
    1,
    "bb884762-5468-43be-870a-96328f8172c2",
    "Jason A. Engle",
);

// WOE 348 — Imodane, the Pyrohammer (alternate printing)
const IMODANE_THE_PYROHAMMER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &IMODANE_THE_PYROHAMMER,
    1,
    "77bf3c05-4cbb-4525-9724-2d04cb2084f3",
    "Chris Rahn",
);

// WOE 349 — Raging Battle Mouse (alternate printing)
const RAGING_BATTLE_MOUSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAGING_BATTLE_MOUSE,
    1,
    "640cabc6-f3a6-4992-acfc-1bed6bc5f05d",
    "Rudy Siswanto",
);

// WOE 350 — Realm-Scorcher Hellkite (alternate printing)
const REALM_SCORCHER_HELLKITE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REALM_SCORCHER_HELLKITE,
    1,
    "8d2d38fe-7fa5-4b29-b3e2-dd20d5afd127",
    "Billy Christian",
);

// WOE 351 — Redcap Gutter-Dweller (alternate printing)
const REDCAP_GUTTER_DWELLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REDCAP_GUTTER_DWELLER,
    1,
    "59cb9275-07f2-46b1-a64b-2971390a35de",
    "Alexey Kruglov",
);

// WOE 352 — Rotisserie Elemental (alternate printing)
const ROTISSERIE_ELEMENTAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROTISSERIE_ELEMENTAL,
    1,
    "f4a55780-cb3b-429a-b0a8-ac616f85582f",
    "Leonardo Santanna",
);

// WOE 353 — Song of Totentanz (alternate printing)
const SONG_OF_TOTENTANZ_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SONG_OF_TOTENTANZ,
    1,
    "30c2f106-0df8-4d4f-a6ef-7946fe8e47c4",
    "Randy Gallegos",
);

// WOE 354 — Blossoming Tortoise (alternate printing)
const BLOSSOMING_TORTOISE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BLOSSOMING_TORTOISE,
    1,
    "79593c3a-00d9-4d24-a49c-753520a1ce30",
    "Simon Dominic",
);

// WOE 355 — Elvish Archivist (alternate printing)
const ELVISH_ARCHIVIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELVISH_ARCHIVIST,
    1,
    "e7678157-e18e-45bd-a28e-7594a49bf2d7",
    "Mila Pesic",
);

// WOE 356 — Feral Encounter (alternate printing)
const FERAL_ENCOUNTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FERAL_ENCOUNTER,
    1,
    "794c5066-724b-4f94-aa6e-7b0690dd706e",
    "Fajareka Setiawan",
);

// WOE 357 — Gruff Triplets (alternate printing)
const GRUFF_TRIPLETS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GRUFF_TRIPLETS,
    1,
    "f48f7bd4-c12d-48a7-b39f-36033636e5c7",
    "Fajareka Setiawan",
);

// WOE 358 — Sentinel of Lost Lore (alternate printing)
const SENTINEL_OF_LOST_LORE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SENTINEL_OF_LOST_LORE,
    1,
    "a8556d3c-9290-4600-afa5-5928d74e3c96",
    "Cristi Balanescu",
);

// WOE 359 — Thunderous Debut (alternate printing)
const THUNDEROUS_DEBUT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THUNDEROUS_DEBUT,
    1,
    "b742aa20-4b18-4296-9693-c2c5a8ba6c04",
    "Aldo Domínguez",
);

// WOE 360 — Agatha of the Vile Cauldron (alternate printing)
const AGATHA_OF_THE_VILE_CAULDRON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AGATHA_OF_THE_VILE_CAULDRON,
    1,
    "cd5f6c96-698d-4f5c-81ec-91176bd2e6e8",
    "Jason A. Engle",
);

// WOE 361 — Faunsbane Troll (alternate printing)
const FAUNSBANE_TROLL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FAUNSBANE_TROLL,
    1,
    "3bf2ce6a-f2ee-42a4-a026-b1d0c18a3ed4",
    "Artur Nakhodkin",
);

// WOE 362 — The Goose Mother (alternate printing)
const THE_GOOSE_MOTHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_GOOSE_MOTHER,
    1,
    "1024d5f1-69f4-4a09-ba83-6fcc249217fa",
    "Jesper Ejsing",
);

// WOE 363 — Hylda of the Icy Crown (alternate printing)
const HYLDA_OF_THE_ICY_CROWN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HYLDA_OF_THE_ICY_CROWN,
    1,
    "18f5f043-0b59-40ba-bdab-1706adf44075",
    "Ekaterina Burmak",
);

// WOE 364 — Likeness Looter (alternate printing)
const LIKENESS_LOOTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LIKENESS_LOOTER,
    1,
    "5d51232f-ee63-4929-adab-042101c97b23",
    "Ben Hill",
);

// WOE 365 — Yenna, Redtooth Regent (alternate printing)
const YENNA_REDTOOTH_REGENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &YENNA_REDTOOTH_REGENT,
    1,
    "a02f04f7-a345-4aa9-933c-1969f0cdd5c6",
    "Justyna Dura",
);

// WOE 366 — Agatha's Soul Cauldron (alternate printing)
const AGATHAS_SOUL_CAULDRON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AGATHAS_SOUL_CAULDRON,
    1,
    "dec5fc59-73d8-4735-88f1-3dbd1f15a546",
    "Jason A. Engle",
);

// WOE 367 — Hylda's Crown of Winter (alternate printing)
const HYLDA_S_CROWN_OF_WINTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HYLDA_S_CROWN_OF_WINTER,
    1,
    "8141f900-a87b-40ce-8439-02c000aafa30",
    "Volkan Baǵa",
);

// WOE 368 — The Irencrag (alternate printing)
const THE_IRENCRAG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_IRENCRAG,
    1,
    "93c50432-28af-40e8-ae98-d4f33a5a936e",
    "Adam Paquette",
);

// WOE 369 — Syr Ginger, the Meal Ender (alternate printing)
const SYR_GINGER_THE_MEAL_ENDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SYR_GINGER_THE_MEAL_ENDER,
    1,
    "522e43f3-4f9a-4f40-a5f5-d84277172040",
    "Michal Ivan",
);

// WOE 370 — Lady of Laughter (alternate printing)
const LADY_OF_LAUGHTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LADY_OF_LAUGHTER,
    1,
    "10ba4b10-5667-414c-a692-166a19b7d748",
    "Kai Carpenter",
);

// WOE 371 — Faerie Slumber Party (alternate printing)
const FAERIE_SLUMBER_PARTY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FAERIE_SLUMBER_PARTY,
    1,
    "f4d1945f-6462-4ec3-a966-3a8dafc0f5d8",
    "Lie Setiawan",
);

// WOE 372 — Malevolent Witchkite (alternate printing)
const MALEVOLENT_WITCHKITE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MALEVOLENT_WITCHKITE,
    1,
    "58ee6723-9b90-4d39-9cbb-9d3765a4fcb0",
    "Donato Giancola",
);

// WOE 373 — Ogre Chitterlord (alternate printing)
const OGRE_CHITTERLORD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OGRE_CHITTERLORD,
    1,
    "cd8d687a-8935-4907-909f-4cadac6cdf67",
    "Piotr Foksowicz",
);

// WOE 374 — Wildwood Mentor (alternate printing)
const WILDWOOD_MENTOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WILDWOOD_MENTOR,
    1,
    "fb7b456e-e56d-45dd-88b3-1ac58a9bfb5f",
    "Piotr Foksowicz",
);

// WOE 375 — Stroke of Midnight (alternate printing)
const STROKE_OF_MIDNIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STROKE_OF_MIDNIGHT,
    1,
    "73014a95-5251-4404-b3a3-037fb5ca3327",
    "Julia Metzger",
);

// WOE 376 — Sleight of Hand (alternate printing)
const SLEIGHT_OF_HAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_p02::SLEIGHT_OF_HAND,
    1,
    "87c81647-6586-4c9c-93fc-9d5ee40b377b",
    "Scott Murphy",
);

// WOE 377 — Faerie Dreamthief (alternate printing)
const FAERIE_DREAMTHIEF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FAERIE_DREAMTHIEF,
    1,
    "373f235b-ce93-48b0-8e36-71f5bc5efcc2",
    "Randy Vargas",
);

// WOE 378 — Torch the Tower (alternate printing)
const TORCH_THE_TOWER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TORCH_THE_TOWER,
    1,
    "f603675e-84df-4dd2-b45b-47349341f64c",
    "Uriah Voth",
);

// WOE 379 — Tanglespan Lookout (alternate printing)
const TANGLESPAN_LOOKOUT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TANGLESPAN_LOOKOUT,
    1,
    "dca603dc-8a2d-484d-8b95-25dfa39369fd",
    "Dmitry Burmak",
);

// WOE 380 — Lich-Knights' Conquest (alternate printing)
const LICH_KNIGHTS_CONQUEST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LICH_KNIGHTS_CONQUEST,
    2,
    "89ffe314-b66b-48d8-b94d-36c8bb5861e4",
    "Kristina Carroll",
);

// WOE 381 — Expel the Interlopers (alternate printing)
const EXPEL_THE_INTERLOPERS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &EXPEL_THE_INTERLOPERS,
    2,
    "bdaac375-57ce-432b-b718-b547367faec7",
    "Awanqi (Angela Wang)",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARCHON_OF_THE_WILD_ROSE,
    &ARCHON_S_GLORY,
    &ARMORY_MICE,
    &BESOTTED_KNIGHT,
    &BREAK_THE_SPELL,
    &CHARMED_CLOTHIER,
    &CHEEKY_HOUSE_MOUSE,
    &COOPED_UP,
    &CURSED_COURTIER,
    &DISCERNING_FINANCIER,
    &DUTIFUL_GRIFFIN,
    &EERIE_INTERFERENCE,
    &EXPEL_THE_INTERLOPERS,
    &FROSTBRIDGE_GUARD,
    &GALLANT_PIE_WIELDER,
    &HOPEFUL_VIGIL,
    &KELLAN_S_LIGHTBLADES,
    &KNIGHT_OF_DOVES,
    &MOMENT_OF_VALOR,
    &MOONSHAKER_CAVALRY,
    &PLUNGE_INTO_WINTER,
    &THE_PRINCESS_TAKES_FLIGHT,
    &PROTECTIVE_PARENTS,
    &REGAL_BUNNICORN,
    &RETURN_TRIUMPHANT,
    &RIMEFUR_REINDEER,
    &SAVIOR_OF_THE_SLEEPING,
    &SLUMBERING_KEEPGUARD,
    &SOLITARY_SANCTUARY,
    &SPELLBOOK_VENDOR,
    &STOCKPILING_CELEBRANT,
    &STROKE_OF_MIDNIGHT,
    &A_TALE_FOR_THE_AGES,
    &THREE_BLIND_MICE,
    &TUINVALE_GUIDE,
    &UNASSUMING_SAGE,
    &WEREFOX_BODYGUARD,
    &AQUATIC_ALCHEMIST,
    &ARCHIVE_DRAGON,
    &ASININE_ANTICS,
    &BELUNA_S_GATEKEEPER,
    &BITTER_CHILL,
    &CHANCELLOR_OF_TALES,
    &DIMINISHER_WITCH,
    &EXTRAORDINARY_JOURNEY,
    &FARSIGHT_RITUAL,
    &FREEZE_IN_PLACE,
    &GADWICK_S_FIRST_DUEL,
    &GALVANIC_GIANT,
    &HORNED_LOCH_WHALE,
    &ICE_OUT,
    &ICEWROUGHT_SENTRY,
    &INGENIOUS_PRODIGY,
    &INTO_THE_FAE_COURT,
    &JOHANN_S_STOPGAP,
    &LIVING_LECTERN,
    &MERFOLK_CORALSMITH,
    &MISLEADING_MOTES,
    &MOCKING_SPRITE,
    &OBYRA_S_ATTENDANTS,
    &PICKLOCK_PRANKSTER,
    &QUICK_STUDY,
    &SLEEP_CURSED_FAERIE,
    &SNAREMASTER_SPRITE,
    &SPELL_STUTTER,
    &SPLASHY_SPELLCASTER,
    &STORMKELD_PROWLER,
    &SUCCUMB_TO_THE_COLD,
    &TALION_S_MESSENGER,
    &TENACIOUS_TOMESEEKER,
    &VANTRESS_TRANSMUTER,
    &VIRTUE_OF_KNOWLEDGE,
    &WATER_WINGS,
    &ASHIOK_WICKED_MANIPULATOR,
    &ASHIOK_S_REAPER,
    &BACK_FOR_SECONDS,
    &BARROW_NAUGHTY,
    &BESEECH_THE_MIRROR,
    &CANDY_GRAPPLE,
    &CONCEITED_WITCH,
    &DREAM_SPOILERS,
    &EGO_DRAIN,
    &THE_END,
    &ERIETTE_S_WHISPER,
    &FAERIE_DREAMTHIEF,
    &FAERIE_FENCING,
    &FEED_THE_CAULDRON,
    &FELL_HORSEMAN,
    &GUMDROP_POISONER,
    &HIGH_FAE_NEGOTIATOR,
    &HOPELESS_NIGHTMARE,
    &LICH_KNIGHTS_CONQUEST,
    &LORD_SKITTER_SEWER_KING,
    &LORD_SKITTER_S_BLESSING,
    &LORD_SKITTER_S_BUTCHER,
    &MINTSTROSITY,
    &NOT_DEAD_AFTER_ALL,
    &RANKLE_S_PRANK,
    &RAT_OUT,
    &ROWAN_S_GRIM_SEARCH,
    &SCREAM_PUFF,
    &SHATTER_THE_OATH,
    &SPECTER_OF_MORTALITY,
    &SPITEFUL_HEXMAGE,
    &STINGBLADE_ASSASSIN,
    &SUGAR_RUSH,
    &SWEETTOOTH_WITCH,
    &TAKEN_BY_NIGHTMARES,
    &TANGLED_COLONY,
    &TWISTED_SEWER_WITCH,
    &VIRTUE_OF_PERSISTENCE,
    &VORACIOUS_VERMIN,
    &WAREHOUSE_TABBY,
    &WICKED_VISITOR,
    &THE_WITCH_S_VANITY,
    &BELLIGERENT_OF_THE_BALL,
    &BELLOWING_BRUISER,
    &BESPOKE_BATTLEGARB,
    &BOUNDARY_LANDS_RANGER,
    &CHARMING_SCOUNDREL,
    &CUT_IN,
    &EDGEWALL_PACK,
    &EMBERETH_VETERAN,
    &FLICK_A_COIN,
    &FOOD_FIGHT,
    &FRANTIC_FIREBOLT,
    &GNAWING_CRESCENDO,
    &GODDRIC_CLOAKED_REVELER,
    &GRABBY_GIANT,
    &GRAND_BALL_GUEST,
    &HARRIED_SPEARGUARD,
    &HEARTH_ELEMENTAL,
    &IMODANE_THE_PYROHAMMER,
    &KINDLED_HEROISM,
    &KORVOLD_AND_THE_NOBLE_THIEF,
    &MERRY_BARDS,
    &MINECART_DAREDEVIL,
    &MONSTROUS_RAGE,
    &RAGING_BATTLE_MOUSE,
    &RATCATCHER_TRAINEE,
    &REALM_SCORCHER_HELLKITE,
    &REDCAP_GUTTER_DWELLER,
    &REDCAP_THIEF,
    &ROTISSERIE_ELEMENTAL,
    &SKEWER_SLINGER,
    &SONG_OF_TOTENTANZ,
    &STONESPLITTER_BOLT,
    &TATTERED_RATTER,
    &TORCH_THE_TOWER,
    &TWISTED_FEALTY,
    &TWO_HEADED_HUNTER,
    &UNRULY_CATAPULT,
    &VIRTUE_OF_COURAGE,
    &WITCH_S_MARK,
    &WITCHSTALKER_FRENZY,
    &AGATHA_S_CHAMPION,
    &BEANSTALK_WURM,
    &BESTIAL_BLOODLINE,
    &BLOSSOMING_TORTOISE,
    &BRAMBLE_FAMILIAR,
    &BRAVE_THE_WILDS,
    &CURSE_OF_THE_WEREFOX,
    &ELVISH_ARCHIVIST,
    &FERAL_ENCOUNTER,
    &FEROCIOUS_WEREFOX,
    &GRACEFUL_TAKEDOWN,
    &GRUFF_TRIPLETS,
    &HAMLET_GLUTTON,
    &HOLLOW_SCAVENGER,
    &HOWLING_GALEFANG,
    &THE_HUNTSMAN_S_REDEMPTION,
    &LEAPING_AMBUSH,
    &NIGHT_OF_THE_SWEETS_REVENGE,
    &REDTOOTH_GENEALOGIST,
    &REDTOOTH_VANGUARD,
    &RETURN_FROM_THE_WILDS,
    &ROOTRIDER_FAUN,
    &ROYAL_TREATMENT,
    &SENTINEL_OF_LOST_LORE,
    &SKYBEAST_TRACKER,
    &SPIDER_FOOD,
    &STORMKELD_VANGUARD,
    &TANGLESPAN_LOOKOUT,
    &TERRITORIAL_WITCHSTALKER,
    &THUNDEROUS_DEBUT,
    &TOADSTOOL_ADMIRER,
    &TOUGH_COOKIE,
    &TROUBLEMAKER_OUPHE,
    &UP_THE_BEANSTALK,
    &VERDANT_OUTRIDER,
    &VIRTUE_OF_STRENGTH,
    &WELCOME_TO_SWEETTOOTH,
    &AGATHA_OF_THE_VILE_CAULDRON,
    &THE_APPRENTICE_S_FOLLY,
    &ASH_PARTY_CRASHER,
    &ERIETTE_OF_THE_CHARMED_APPLE,
    &FAUNSBANE_TROLL,
    &THE_GOOSE_MOTHER,
    &GRETA_SWEETTOOTH_SCOURGE,
    &HYLDA_OF_THE_ICY_CROWN,
    &JOHANN_APPRENTICE_SORCERER,
    &LIKENESS_LOOTER,
    &NEVA_STALKED_BY_NIGHTMARES,
    &OBYRA_DREAMING_DUELIST,
    &ROWAN_SCION_OF_WAR,
    &RUBY_DARING_TRACKER,
    &SHARAE_OF_NUMBING_DEPTHS,
    &SYR_ARMONT_THE_REDEEMER,
    &TALION_THE_KINDLY_LORD,
    &TOTENTANZ_SWARM_PIPER,
    &TROYAN_GUTSY_EXPLORER,
    &WILL_SCION_OF_PEACE,
    &YENNA_REDTOOTH_REGENT,
    &BELUNA_GRANDSQUALL,
    &CALLOUS_SELL_SWORD,
    &CRUEL_SOMNOPHAGE,
    &DECADENT_DRAGON,
    &DEVOURING_SUGARMAW,
    &ELUSIVE_OTTER,
    &FROLICKING_FAMILIAR,
    &GINGERBREAD_HUNTER,
    &HEARTFLAME_DUELIST,
    &IMODANE_S_RECRUITER,
    &KELLAN_THE_FAE_BLOODED,
    &MOSSWOOD_DREADKNIGHT,
    &PICNIC_RUINER,
    &POLLEN_SHIELD_HARE,
    &QUESTING_DRUID,
    &SCALDING_VIPER,
    &SHROUDED_SHEPHERD,
    &SPELLSCORN_COVEN,
    &TEMPEST_HART,
    &THREADBIND_CLIQUE,
    &TWINING_TWINS,
    &WOODLAND_ACOLYTE,
    &AGATHAS_SOUL_CAULDRON,
    &CANDY_TRAIL,
    &COLLECTOR_S_VAULT,
    &ERIETTE_S_TEMPTING_APPLE,
    &HYLDA_S_CROWN_OF_WINTER,
    &THE_IRENCRAG,
    &SCARECROW_GUIDE,
    &SYR_GINGER_THE_MEAL_ENDER,
    &THREE_BOWLS_OF_PORRIDGE,
    &EDGEWALL_INN,
    &RESTLESS_BIVOUAC,
    &RESTLESS_COTTAGE,
    &RESTLESS_FORTRESS,
    &RESTLESS_SPIRE,
    &RESTLESS_VINESTALK,
    &VIRTUE_OF_LOYALTY,
    &FOOD_COMA,
    &LADY_OF_LAUGHTER,
    &PESTS_OF_HONOR,
    &FAERIE_SLUMBER_PARTY,
    &ROWDY_RESEARCH,
    &STORYTELLER_PIXIE,
    &EXPERIMENTAL_CONFECTIONER,
    &MALEVOLENT_WITCHKITE,
    &OLD_FLITTERFANG,
    &BECOME_BRUTES,
    &CHARGING_HOOLIGAN,
    &OGRE_CHITTERLORD,
    &INTREPID_TRUFFLESNOUT,
    &PROVISIONS_MERCHANT,
    &WILDWOOD_MENTOR,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    GLASS_CASKET_REPRINT,
    VIRTUE_OF_LOYALTY_ALTERNATE_1,
    DISDAINFUL_STROKE_REPRINT,
    SLEIGHT_OF_HAND_REPRINT,
    COMMUNE_WITH_NATURE_REPRINT,
    TITANIC_GROWTH_REPRINT,
    GINGERBRUTE_REPRINT,
    PROPHETIC_PRISM_REPRINT,
    SOUL_GUIDE_LANTERN_REPRINT,
    CRYSTAL_GROTTO_REPRINT,
    EVOLVING_WILDS_REPRINT,
    PLAINS_REPRINT,
    ISLAND_REPRINT,
    SWAMP_REPRINT,
    MOUNTAIN_REPRINT,
    FOREST_REPRINT,
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
    HORNED_LOCH_WHALE_ALTERNATE_1,
    VIRTUE_OF_KNOWLEDGE_ALTERNATE_1,
    GUMDROP_POISONER_ALTERNATE_1,
    VIRTUE_OF_PERSISTENCE_ALTERNATE_1,
    VIRTUE_OF_COURAGE_ALTERNATE_1,
    BRAMBLE_FAMILIAR_ALTERNATE_1,
    VIRTUE_OF_STRENGTH_ALTERNATE_1,
    BELUNA_GRANDSQUALL_ALTERNATE_1,
    CRUEL_SOMNOPHAGE_ALTERNATE_1,
    DECADENT_DRAGON_ALTERNATE_1,
    DEVOURING_SUGARMAW_ALTERNATE_1,
    ELUSIVE_OTTER_ALTERNATE_1,
    HEARTFLAME_DUELIST_ALTERNATE_1,
    KELLAN_THE_FAE_BLOODED_ALTERNATE_1,
    MOSSWOOD_DREADKNIGHT_ALTERNATE_1,
    POLLEN_SHIELD_HARE_ALTERNATE_1,
    QUESTING_DRUID_ALTERNATE_1,
    SCALDING_VIPER_ALTERNATE_1,
    TWINING_TWINS_ALTERNATE_1,
    ASHIOK_WICKED_MANIPULATOR_ALTERNATE_1,
    KELLAN_THE_FAE_BLOODED_ALTERNATE_2,
    ERIETTE_OF_THE_CHARMED_APPLE_ALTERNATE_1,
    ROWAN_SCION_OF_WAR_ALTERNATE_1,
    TALION_THE_KINDLY_LORD_ALTERNATE_1,
    WILL_SCION_OF_PEACE_ALTERNATE_1,
    RESTLESS_BIVOUAC_ALTERNATE_1,
    RESTLESS_COTTAGE_ALTERNATE_1,
    RESTLESS_FORTRESS_ALTERNATE_1,
    RESTLESS_SPIRE_ALTERNATE_1,
    RESTLESS_VINESTALK_ALTERNATE_1,
    ARCHON_OF_THE_WILD_ROSE_ALTERNATE_1,
    EXPEL_THE_INTERLOPERS_ALTERNATE_1,
    MOONSHAKER_CAVALRY_ALTERNATE_1,
    REGAL_BUNNICORN_ALTERNATE_1,
    SPELLBOOK_VENDOR_ALTERNATE_1,
    A_TALE_FOR_THE_AGES_ALTERNATE_1,
    WEREFOX_BODYGUARD_ALTERNATE_1,
    ASININE_ANTICS_ALTERNATE_1,
    EXTRAORDINARY_JOURNEY_ALTERNATE_1,
    FARSIGHT_RITUAL_ALTERNATE_1,
    INGENIOUS_PRODIGY_ALTERNATE_1,
    SLEEP_CURSED_FAERIE_ALTERNATE_1,
    TALION_S_MESSENGER_ALTERNATE_1,
    BESEECH_THE_MIRROR_ALTERNATE_1,
    THE_END_ALTERNATE_1,
    LICH_KNIGHTS_CONQUEST_ALTERNATE_1,
    LORD_SKITTER_SEWER_KING_ALTERNATE_1,
    LORD_SKITTER_S_BLESSING_ALTERNATE_1,
    RANKLE_S_PRANK_ALTERNATE_1,
    SPECTER_OF_MORTALITY_ALTERNATE_1,
    SPITEFUL_HEXMAGE_ALTERNATE_1,
    TANGLED_COLONY_ALTERNATE_1,
    CHARMING_SCOUNDREL_ALTERNATE_1,
    FOOD_FIGHT_ALTERNATE_1,
    GODDRIC_CLOAKED_REVELER_ALTERNATE_1,
    IMODANE_THE_PYROHAMMER_ALTERNATE_1,
    RAGING_BATTLE_MOUSE_ALTERNATE_1,
    REALM_SCORCHER_HELLKITE_ALTERNATE_1,
    REDCAP_GUTTER_DWELLER_ALTERNATE_1,
    ROTISSERIE_ELEMENTAL_ALTERNATE_1,
    SONG_OF_TOTENTANZ_ALTERNATE_1,
    BLOSSOMING_TORTOISE_ALTERNATE_1,
    ELVISH_ARCHIVIST_ALTERNATE_1,
    FERAL_ENCOUNTER_ALTERNATE_1,
    GRUFF_TRIPLETS_ALTERNATE_1,
    SENTINEL_OF_LOST_LORE_ALTERNATE_1,
    THUNDEROUS_DEBUT_ALTERNATE_1,
    AGATHA_OF_THE_VILE_CAULDRON_ALTERNATE_1,
    FAUNSBANE_TROLL_ALTERNATE_1,
    THE_GOOSE_MOTHER_ALTERNATE_1,
    HYLDA_OF_THE_ICY_CROWN_ALTERNATE_1,
    LIKENESS_LOOTER_ALTERNATE_1,
    YENNA_REDTOOTH_REGENT_ALTERNATE_1,
    AGATHAS_SOUL_CAULDRON_ALTERNATE_1,
    HYLDA_S_CROWN_OF_WINTER_ALTERNATE_1,
    THE_IRENCRAG_ALTERNATE_1,
    SYR_GINGER_THE_MEAL_ENDER_ALTERNATE_1,
    LADY_OF_LAUGHTER_ALTERNATE_1,
    FAERIE_SLUMBER_PARTY_ALTERNATE_1,
    MALEVOLENT_WITCHKITE_ALTERNATE_1,
    OGRE_CHITTERLORD_ALTERNATE_1,
    WILDWOOD_MENTOR_ALTERNATE_1,
    STROKE_OF_MIDNIGHT_ALTERNATE_1,
    SLEIGHT_OF_HAND_ALTERNATE_1,
    FAERIE_DREAMTHIEF_ALTERNATE_1,
    TORCH_THE_TOWER_ALTERNATE_1,
    TANGLESPAN_LOOKOUT_ALTERNATE_1,
    LICH_KNIGHTS_CONQUEST_ALTERNATE_2,
    EXPEL_THE_INTERLOPERS_ALTERNATE_2,
];
