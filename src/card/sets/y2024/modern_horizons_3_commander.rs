//! Modern Horizons 3 Commander cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AddManaEffectDef, AppliedEffectDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    ChoiceVisibilityDef, ChooseDef, EffectDef, EffectRecipientDef, ManaColor,
    ObjectChoiceBindingDef, ObjectPredicateDef, ObjectRefDef, ObjectSetDef, ObjectSetFilterDef,
    PlayerRefDef, PlayerRelation, SumValueDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::ids::ParentBinding;
use crate::{TargetIndex, mana_cost};

use super::super::y2020::theros_beyond_death::escape;

// M3C 4 — Ulalek, Fused Atrocity
// Audit: metadata-only — Its creature body and Devoid are catalog metadata; the mass spell-and-ability copy trigger is not executable.
pub(in crate::card::sets) static ULALEK_FUSED_ATROCITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fdad1b0e-d3cc-4d76-ae7e-fee12558cf2c"),
    "Ulalek, Fused Atrocity",
    CardArt::new("fdad1b0e-d3cc-4d76-ae7e-fee12558cf2c", "Alex Konstad"),
    CardSet::ModernHorizons3Commander,
    CardRules::new_creature(mana_cost!("{C/W}{C/U}{C/B}{C/R}{C/G}"), &["Eldrazi"], 2, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_metadata_only_creature_body()
        .printed_colors(&[])
        .with_abilities(&[
            abilities::devoid().with_coverage(AbilityCoverageDef::metadata_only(
                "Ulalek's colorlessness is represented directly in its printed color metadata.",
            )),
            AbilityDef::not_implemented(
                "Whenever you cast an Eldrazi spell, you may pay {C}{C}. If you do, copy all spells you control, then copy all other activated and triggered abilities you control. You may choose new targets for the copies. (Mana abilities can't be copied.)",
                "Copying every spell and nonmana ability one player controls, while preserving each copy's choices and allowing new targets, is not modeled.",
            ),
        ]),
);

/// "That number plus 1", shared by both Lhurgoyfs in this set: each counts
/// the card types in every graveyard and is one tougher than it is strong.
static GOYF_TOUGHNESS_IN_ALL_GRAVEYARDS: SumValueDef = SumValueDef::new(
    ValueDef::CardTypesAmongGraveyards(PlayerRelation::Any),
    ValueDef::Constant(1),
);

// M3C 50 — Barrowgoyf
/// Where the chosen card is saved, kept apart from the milled pile so that
/// "them" and "the one you took" are two different sets.
pub(in crate::card::sets) static BARROWGOYF: CardRecord = CardRecord::new_with_legacy_id(
    2213,
    "Barrowgoyf",
    CardArt::new("f979fc86-2c7e-49b3-965e-607a203cbfb1", "Igor Kieryluk"),
    CardSet::ModernHorizons3Commander,
    // Deathtouch and lifelink on a body that grows with every graveyard,
    // and every hit digs for the next one.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Lhurgoyf"], 0, 1).with_abilities(&[
        abilities::deathtouch(),
        abilities::lifelink(),
        AbilityDef::static_ability(
            "Barrowgoyf's power is equal to the number of card types among cards in all graveyards and its toughness is equal to that number plus 1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                // Each half is its own amount: the toughness is the count
                // plus one rather than the count applied to a printed body.
                effect: AppliedEffectDef::define_power_toughness(
                    ValueDef::CardTypesAmongGraveyards(PlayerRelation::Any),
                    ValueDef::Sum(&GOYF_TOUGHNESS_IN_ALL_GRAVEYARDS),
                ),
            },
        ),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, you may mill that many cards. If you do, you may put a creature card from among them into your hand.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                    EffectDef::BindOutput {
                        effect: &EffectDef::Mill {
                            player: EffectRecipientDef::Controller,
                            amount: ValueDef::TriggerEventAmount,
                        },
                        binding: Binding!("milled_cards"),
                    },
                    // A minimum of zero is the second "you may": milling and taking nothing is
                    // a legal answer, and a pile with no creature in it never asks.
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        // "From among them" is what the mill just put there, not what the
                        // graveyard already held -- and only a creature card among those.
                        candidates: ObjectSetDef::Matching {
                            objects: &ObjectSetDef::Binding(Binding!("milled_cards")),
                            object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::HasType(
                                CardType::Creature,
                            )),
                        },
                        exclude: None,
                        minimum: 0,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::MoveToZone {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                            zone: ZoneKind::Hand,
                            placement: ZonePlacement::Top,
                        },
                    }),
                ]),
            },
        ),
    ]),
);

// M3C 59 — Pyrogoyf
pub(in crate::card::sets) static PYROGOYF: CardRecord = CardRecord::new_with_legacy_id(
    2141,
    "Pyrogoyf",
    CardArt::new("f60be310-4461-4b84-95f0-b2095108bd79", "Xabi Gaztelua"),
    CardSet::ModernHorizons3Commander,
    // The printed 0/1 is only what the corner says; the ability below is
    // what it is, wherever it is.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Lhurgoyf"], 0, 1)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Pyrogoyf's power is equal to the number of card types among cards in all graveyards and its toughness is equal to that number plus 1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    // The same shape Barrowgoyf has, counted over the same pile.
                    effect: AppliedEffectDef::define_power_toughness(
                        ValueDef::CardTypesAmongGraveyards(PlayerRelation::Any),
                        ValueDef::Sum(&GOYF_TOUGHNESS_IN_ALL_GRAVEYARDS),
                    ),
                },
            ),
            AbilityDef::triggered_with_targets(
                "Whenever this creature or another Lhurgoyf creature you control enters, that creature deals damage equal to its power to any target.",
                // A Lhurgoyf you control -- this one included, which is what "this creature
                // or another" comes to.
                TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype("Lhurgoyf"),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]), None, Some(ZoneKind::Battlefield)),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                // "That creature" deals it, not Pyrogoyf: the Lhurgoyf that entered
                // is both where the amount is read and what the damage is from, so
                // protection and redirection answer the right object when the one
                // entering is some other Lhurgoyf.
                EffectDef::DealDamageFrom {
                    source: ObjectRefDef::TriggeringObject,
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::TriggeringObjectPower,
                },
            ),
        ]),
);

// M3C 70 — Bloodbraid Challenger
pub(in crate::card::sets) static BLOODBRAID_CHALLENGER: CardRecord = CardRecord::new_with_legacy_id(
    2255,
    "Bloodbraid Challenger",
    CardArt::new("4b39d43d-2a02-4edb-915a-6a7c002c945f", "Lie Setiawan"),
    CardSet::ModernHorizons3Commander,
    // Five mana for a hasty 4/3 and a free spell, and the graveyard keeps
    // handing it back for five more.
    CardRules::new_creature(mana_cost!("{3}{R}{G}"), &["Elf", "Berserker"], 4, 3).with_abilities(
        &[
            abilities::cascade(),
            abilities::haste(),
            escape(
                crate::card::AlternativeCastManaCostDef::Fixed(mana_cost!("{3}{R}{G}")),
                3,
            ),
        ],
    ),
);

// M3C 134 — Talon Gates of Madara
pub(in crate::card::sets) static TALON_GATES_OF_MADARA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c565f8fe-acf7-40dd-8100-8f692d1e232c"),
    "Talon Gates of Madara",
    CardArt::new("c565f8fe-acf7-40dd-8100-8f692d1e232c", "Steven Belledin"),
    CardSet::ModernHorizons3Commander,
    // A land that answers a creature on the way in, and four mana that puts
    // it there on a turn the land drop is already spent.
    CardRules::new_land(&["Gate"]).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this land enters, up to one target creature phases out.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::PhaseOut {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
        // Activated from hand, which is the only zone the clause names: what it
        // does is move itself, so the land arrives without using a land drop and
        // its enter trigger fires like any other.
        AbilityDef::activated(
            "{4}: Put this card from your hand onto the battlefield.",
            &[AbilityCostDef::Mana(mana_cost!("{4}"))],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
            },
        )
        .with_source_zones(&[ZoneKind::Hand]),
    ]),
);

// M3C 320 — Basilisk Gate
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BASILISK_GATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a306025-d429-4006-b7ed-bdb287e83f57"),
    "Basilisk Gate",
    crate::card::CardArt::new("935f3dfa-7d8d-459a-8ac2-37892cb9545f", "Jorge Jacinto"),
    crate::card::CardSet::ModernHorizons3Commander,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ULALEK_FUSED_ATROCITY,
    &BARROWGOYF,
    &PYROGOYF,
    &BLOODBRAID_CHALLENGER,
    &TALON_GATES_OF_MADARA,
    &BASILISK_GATE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
