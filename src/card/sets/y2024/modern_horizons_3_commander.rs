//! Modern Horizons 3 Commander cards cataloged for the Vintage Cube pool.

use crate::card::BattlefieldArrivalDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BindObjectsDef;
use crate::card::ComparisonDef;
use crate::card::CreatedTokensDef;
use crate::card::ManaTypeFilterDef;
use crate::card::ManaTypeSetDef;
use crate::card::ManaTypeSourceDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectQueryDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TriggerConditionDef;
use crate::card::TurnStepDef;
use super::super::y2020::theros_beyond_death::escape;
use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::NONBASIC_LAND_SUBTYPES;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::SubtypeDef;
use crate::card::SumValueDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::sets::y2022::commander_legends_baldurs_gate as catalog_clb;
use crate::ids::ParentBinding;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "M3C",
    slug: "modern-horizons-3-commander",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// M3C 4 — Ulalek, Fused Atrocity
// Audit: unsupported — Needs grouped copying of every controlled spell and nonmana stack ability with independent target reselection.
pub(in crate::card::sets) static ULALEK_FUSED_ATROCITY: CardRecord = CardRecord::new(
    "Ulalek, Fused Atrocity",
    "fdad1b0e-d3cc-4d76-ae7e-fee12558cf2c",
    "Alex Konstad",
    CardRules::unsupported(),
);

/// "That number plus 1", shared by both Lhurgoyfs in this set: each counts
/// the card types in every graveyard and is one tougher than it is strong.
static GOYF_TOUGHNESS_IN_ALL_GRAVEYARDS: SumValueDef = SumValueDef::new(
    ValueDef::CardTypesAmongGraveyards(PlayerRelation::Any),
    ValueDef::Constant(1),
);

// M3C 32 — Eldrazi Confluence
pub(in crate::card::sets) static ELDRAZI_CONFLUENCE_32: CardRecord = CardRecord::new(
    "Eldrazi Confluence",
    "78ee2013-29dc-4879-9d59-1b492996d297",
    "Hristo D. Chukov",
    CardRules::new_instant(mana_cost!("{2}{C}{C}")).with_abilities(&[
AbilityDef::modal_spell("Choose three. You may choose the same mode more than once.", &[AbilityDef::spell_with_targets("Target creature gets +3/-3 until end of turn.", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))], EffectDef::Apply { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(3), ValueDef::Constant(-3)), duration: ResolvedEffectDurationDef::UntilEndOfTurn }), AbilityDef::spell_with_targets("Exile target nonland permanent, then return it to the battlefield tapped under its owner's control.", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)))], EffectDef::BindObjects(BindObjectsDef { source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::LegalTargets(TargetIndex::PRIMARY)), binding: Binding!("confluence_blink"), then: &EffectDef::Sequence(&[EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("confluence_blink"))), ZoneKind::Exile, ZonePlacement::Top), EffectDef::WithBattlefieldArrival { effect: &EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::ZoneChangeSuccessorsOfBinding(Binding!("confluence_blink"))), ZoneKind::Battlefield, ZonePlacement::Top), arrival: BattlefieldArrivalDef { controller: None, modifications: &[BattlefieldEntryModificationDef::Tapped], attachment: None, counters: None } }]) })), AbilityDef::spell("Create an Eldrazi Scion token.", EffectDef::create_creature_token(&["Eldrazi", "Scion"], &[], 1, 1).with_abilities(&[AbilityDef::activated_mana("Sacrifice this token: Add {C}.", &[CostDef::SacrificeSource], EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)))]))]).with_mode_selection(3, 3, true)
]),
);

// M3C 33 — Eldritch Immunity
pub(in crate::card::sets) static ELDRITCH_IMMUNITY_33: CardRecord = CardRecord::new(
    "Eldritch Immunity",
    "64a63b90-dbd6-4b66-8031-a3e230ada5b9",
    "Carlos Palma Cruchaga",
    CardRules::new_instant(mana_cost!("{C}")).with_subtypes(&["Eldrazi"]).with_abilities(&[
AbilityDef::spell_with_targets("Target creature you control gains protection from each color until end of turn.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::HasType(CardType::Creature), zones: &[ZoneKind::Battlefield], controller: Some(PlayerRelation::You), owner: None })], EffectDef::Apply { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(&abilities::protection_from_color(ManaColor::White)), AppliedEffectDef::add_ability(&abilities::protection_from_color(ManaColor::Blue)), AppliedEffectDef::add_ability(&abilities::protection_from_color(ManaColor::Black)), AppliedEffectDef::add_ability(&abilities::protection_from_color(ManaColor::Red)), AppliedEffectDef::add_ability(&abilities::protection_from_color(ManaColor::Green))]), duration: ResolvedEffectDurationDef::UntilEndOfTurn }),
abilities::overload(&[CostDef::Mana(mana_cost!("{4}{C}"))], "Overload {4}{C} (You may cast this spell for its overload cost. If you do, change \"target\" in its text to \"each.\")", EffectDef::Apply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You), effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(&abilities::protection_from_color(ManaColor::White)), AppliedEffectDef::add_ability(&abilities::protection_from_color(ManaColor::Blue)), AppliedEffectDef::add_ability(&abilities::protection_from_color(ManaColor::Black)), AppliedEffectDef::add_ability(&abilities::protection_from_color(ManaColor::Red)), AppliedEffectDef::add_ability(&abilities::protection_from_color(ManaColor::Green))]), duration: ResolvedEffectDurationDef::UntilEndOfTurn })
]),
);

// M3C 50 — Barrowgoyf
/// Where the chosen card is saved, kept apart from the milled pile so that
/// "them" and "the one you took" are two different sets.
pub(in crate::card::sets) static BARROWGOYF: CardRecord = CardRecord::new(
    "Barrowgoyf",
    "f979fc86-2c7e-49b3-965e-607a203cbfb1",
    "Igor Kieryluk",
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
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                            ZoneKind::Hand,
                            ZonePlacement::Top,
                        ),
                    }),
                ]),
            },
        ),
    ]),
);

// M3C 59 — Pyrogoyf
pub(in crate::card::sets) static PYROGOYF: CardRecord = CardRecord::new(
    "Pyrogoyf",
    "f60be310-4461-4b84-95f0-b2095108bd79",
    "Xabi Gaztelua",
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
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Lhurgoyf")),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]), None, Some(ZoneKind::Battlefield)),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                // "That creature" deals it, not Pyrogoyf: the Lhurgoyf that entered
                // is both where the amount is read and what the damage is from, so
                // protection and redirection answer the right object when the one
                // entering is some other Lhurgoyf.
                EffectDef::damage_from(
                    ObjectRefDef::TriggeringObject,
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::TriggeringObjectPower,
                ),
            ),
        ]),
);

// M3C 61 — Siege-Gang Lieutenant
pub(in crate::card::sets) static SIEGE_GANG_LIEUTENANT_61: CardRecord = CardRecord::new(
    "Siege-Gang Lieutenant",
    "2567e5a7-e045-48f1-b749-b1920b948b9b",
    "Warren Mahy",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Goblin"], 2, 2).with_abilities(&[
AbilityDef::triggered_if("Lieutenant — At the beginning of combat on your turn, if you control your commander, create two 1/1 red Goblin creature tokens. Those tokens gain haste until end of turn.", TriggerEventDef::StepBegins { step: TurnStepDef::BeginningOfCombat, player: PlayerRelation::You }, &TriggerConditionDef::ObjectCount { query: ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::Commander, ObjectPredicateDef::OwnedBy(PlayerRelation::You)]), &[ZoneKind::Battlefield], PlayerRelation::You), comparison: ComparisonDef::GreaterOrEqual, amount: 1 }, EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1).with_count(ValueDef::Constant(2)).with_created_tokens(CreatedTokensDef { binding: Binding!("lieutenant_goblins"), then: &EffectDef::Apply { recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("lieutenant_goblins"))), effect: AppliedEffectDef::add_ability(&abilities::haste()), duration: ResolvedEffectDurationDef::UntilEndOfTurn } })),
AbilityDef::activated_with_targets("{2}, Sacrifice a Goblin: This creature deals 1 damage to any target.", &[CostDef::Mana(mana_cost!("{2}")), CostDef::sacrifice_permanent(ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")))], &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)], EffectDef::damage(EffectRecipientDef::Target(TargetIndex::PRIMARY), ValueDef::Constant(1)))
]),
);

// M3C 70 — Bloodbraid Challenger
pub(in crate::card::sets) static BLOODBRAID_CHALLENGER: CardRecord = CardRecord::new(
    "Bloodbraid Challenger",
    "4b39d43d-2a02-4edb-915a-6a7c002c945f",
    "Lie Setiawan",
    // Five mana for a hasty 4/3 and a free spell, and the graveyard keeps
    // handing it back for five more.
    CardRules::new_creature(mana_cost!("{3}{R}{G}"), &["Elf", "Berserker"], 4, 3).with_abilities(
        &[
            abilities::cascade(),
            abilities::haste(),
            escape(&[
                crate::card::CostDef::Mana(mana_cost!("{3}{R}{G}")),
                CostDef::exile(
                    crate::ObjectPredicateDef::Any,
                    crate::ZoneKind::Graveyard,
                    crate::card::CostQuantityDef::Fixed(3),
                ),
            ]),
        ],
    ),
);

// M3C 78 — Horizon of Progress
pub(in crate::card::sets) static HORIZON_OF_PROGRESS_78: CardRecord = CardRecord::new(
    "Horizon of Progress",
    "5ae3a9c8-194e-421b-b77d-9c8784442651",
    "Julian Kok Joon Wen",
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}, Pay 1 life: Add one mana of any type that a land you control could produce.",
            &[CostDef::TapSource, CostDef::PayLife(1)],
            EffectDef::AddMana(AddManaEffectDef::choice_from(ManaTypeSetDef {
                source: ManaTypeSourceDef::CouldBeProducedBy(&ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                filter: ManaTypeFilterDef::AnyType,
            })),
        ),
        AbilityDef::activated(
            "{3}, {T}: You may put a land card from your hand onto the battlefield tapped.",
            &[CostDef::Mana(mana_cost!("{3}")), CostDef::TapSource],
            EffectDef::Choose(ChooseDef {
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: 1,
                binding: ObjectChoiceBindingDef::Objects(Binding!("horizon_land")),
                unchosen: None,
                visibility: ChoiceVisibilityDef::Private,
                then: &EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!(
                            "horizon_land"
                        ))),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: BattlefieldArrivalDef {
                        controller: None,
                        modifications: &[BattlefieldEntryModificationDef::Tapped],
                        attachment: None,
                        counters: None,
                    },
                },
            }),
        ),
        AbilityDef::activated(
            "{1}, {T}, Sacrifice this land: Draw a card.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// M3C 80 — Planar Nexus
pub(in crate::card::sets) static PLANAR_NEXUS: CardRecord = CardRecord::new(
    "Planar Nexus",
    "28603c1c-f9b4-4001-bc56-d1453d5cacf5",
    "Sam Burley",
    CardRules::new_land(NONBASIC_LAND_SUBTYPES).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// M3C 131 — Lazotep Quarry
// Audit: unsupported — CopyExceptionsDef can add creature types but cannot replace the copied creature types with Zombie. Adding Zombie would incorrectly preserve the exiled card's original creature types.
pub(in crate::card::sets) static LAZOTEP_QUARRY_131: CardRecord = CardRecord::new(
    "Lazotep Quarry",
    "656ddd43-c70c-4927-9a02-fef5732708da",
    "Sam Burley",
    crate::card::CardRules::unsupported(),
);

// M3C 134 — Talon Gates of Madara
pub(in crate::card::sets) static TALON_GATES_OF_MADARA: CardRecord = CardRecord::new(
    "Talon Gates of Madara",
    "c565f8fe-acf7-40dd-8100-8f692d1e232c",
    "Steven Belledin",
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
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
        // Activated from hand, which is the only zone the clause names: what it
        // does is move itself, so the land arrives without using a land drop and
        // its enter trigger fires like any other.
        AbilityDef::activated(
            "{4}: Put this card from your hand onto the battlefield.",
            &[CostDef::Mana(mana_cost!("{4}"))],
            EffectDef::move_to_zone(
                EffectRecipientDef::Source,
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
        )
        .with_source_zones(&[ZoneKind::Hand]),
    ]),
);

// M3C 320 — Basilisk Gate (reprint)
const BASILISK_GATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_clb::BASILISK_GATE,
    "935f3dfa-7d8d-459a-8ac2-37892cb9545f",
    "Jorge Jacinto",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ULALEK_FUSED_ATROCITY,
    &ELDRAZI_CONFLUENCE_32,
    &ELDRITCH_IMMUNITY_33,
    &BARROWGOYF,
    &PYROGOYF,
    &SIEGE_GANG_LIEUTENANT_61,
    &BLOODBRAID_CHALLENGER,
    &HORIZON_OF_PROGRESS_78,
    &PLANAR_NEXUS,
    &LAZOTEP_QUARRY_131,
    &TALON_GATES_OF_MADARA,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[BASILISK_GATE_REPRINT];
