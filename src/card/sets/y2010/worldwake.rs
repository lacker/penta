//! Worldwake cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AlternativeCastKindDef, AppliedEffectDef, AppliedRuleDef, BattlefieldEntryModificationDef,
    CardChoiceSourceDef, CardRules, CardSet, CardSupertype, CardType, CardTypeSet, ColorSet,
    ComparisonDef, CounterKind, CreatureTypeSetDef, EffectDef, EffectRecipientDef, ManaColor,
    ObjectPredicateDef, ObjectQueryDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    ReplacementEffectDef, ResolvedEffectDurationDef, SpellCastQueryDef, TokenCharacteristics,
    TriggerConditionDef, ValueComparisonDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{AdditionalCostIndex, TargetIndex, mana_cost};

// WWK 20 — Stoneforge Mystic
pub(in crate::card::sets) static STONEFORGE_MYSTIC: CardRecord = CardRecord::new(
    CardSet::Worldwake,
    "Stoneforge Mystic",
    "19557351-b65f-4b04-b971-66abdc07000a",
    "Mike Bierek",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Kor", "Artificer"], 1, 2)
        .with_abilities(&[
            abilities::enters_trigger("When this creature enters, you may search your library for an Equipment card, reveal it, put it into your hand, then shuffle.", EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::Subtype("Equipment"),
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
                }),
            AbilityDef::activated(
                "{1}{W}, {T}: You may put an Equipment card from your hand onto the battlefield.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{1}{W}")),
                    AbilityCostDef::TapSource,
                ],
                // The second half of the card, and the reason the first half is worth
                // finding: a minimum of zero is the printed "you may", and with no
                // Equipment in hand the choice is never offered at all.
                EffectDef::ChooseCards {
                    player: EffectRecipientDef::Controller,
                    sources: &[CardChoiceSourceDef::Zone(ZoneKind::Hand)],
                    object: ObjectPredicateDef::Subtype("Equipment"),
                    minimum: 0,
                    maximum: 1,
                    reveal: false,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    // It arrives as itself: nothing about the Equipment changes on the way
                    // down, and it is not attached to anything.
                },
            ),
        ]),
);

// WWK 26 — Dispel
pub(in crate::card::sets) static DISPEL: CardRecord = CardRecord::new(
    CardSet::Worldwake,
    "Dispel",
    "f178d0cc-5dd1-41ab-a2e8-218ece6f2a86",
    "Vance Kovacs",
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::counter_target(
        "Counter target instant spell.",
        &AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::Spell,
                ObjectPredicateDef::HasType(CardType::Instant),
            ]),
            zones: &[ZoneKind::Stack],
            controller: None,
            owner: None,
        }),
    )),
);

// WWK 31 — Jace, the Mind Sculptor
static A_PLAYER: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

pub(in crate::card::sets) static JACE_THE_MIND_SCULPTOR: CardRecord =
    CardRecord::new(
        CardSet::Worldwake,
    "Jace, the Mind Sculptor",
    "0e606072-a3aa-4300-ba90-ec92a721fa76",
    "Jason Chan",
        // Four abilities and three of them matter: the bounce buys the turn, the
        // zero rebuilds the hand, and the fateseal is what a Jace that is not
        // under pressure does forever.
        CardRules::new_planeswalker(mana_cost!("{2}{U}{U}"), &["Jace"], 3)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                AbilityDef::activated_with_targets(
                    "+2: Look at the top card of target player's library. You may put that card on the \
                     bottom of that player's library.",
                    &[AbilityCostDef::Loyalty(2)],
                    &A_PLAYER,
                    abilities::fateseal(
                        PlayerRefDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(1),
                    ),
                ),
                AbilityDef::activated(
                    "0: Draw three cards, then put two cards from your hand on top of your library in any \
                     order.",
                    &[AbilityCostDef::Loyalty(0)],
                    abilities::brainstorm(),
                ),
                AbilityDef::activated_with_targets(
                    "−1: Return target creature to its owner's hand.",
                    &[AbilityCostDef::Loyalty(-1)],
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                    },
                ),
                AbilityDef::activated_with_targets(
                    "−12: Exile all cards from target player's library, then that player shuffles their hand \
                     into their library.",
                    &[AbilityCostDef::Loyalty(-12)],
                    &A_PLAYER,
                    EffectDef::Sequence(&[
                        EffectDef::MoveToZone {
                            object: EffectRecipientDef::objects(crate::card::ObjectSetDef::Query(
                                // Their whole library, named by owner rather than by relation: the ultimate
                                // points at a player and empties that one.
                                ObjectQueryDef::owned_by(
                                    ObjectPredicateDef::Any,
                                    &[ZoneKind::Library],
                                    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                                ),
                            )),
                            zone: ZoneKind::Exile,
                            placement: ZonePlacement::Top,
                        },
                        EffectDef::MoveToZone {
                            object: EffectRecipientDef::objects(crate::card::ObjectSetDef::Query(
                                ObjectQueryDef::owned_by(
                                    ObjectPredicateDef::Any,
                                    &[ZoneKind::Hand],
                                    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                                ),
                            )),
                            zone: ZoneKind::Library,
                            placement: ZonePlacement::Top,
                        },
                        // The shuffle is what leaves them a library at all, so it is the whole
                        // difference between this and drawing from nothing next upkeep.
                        EffectDef::ShuffleLibrary {
                            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        },
                    ]),
                ),
            ]),
    );

// WWK 52 — Brink of Disaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRINK_OF_DISASTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Worldwake,
    "Brink of Disaster",
    "0c841c3e-e0d1-49d7-bcec-3c45f73c13c5",
    "Alex Horley-Orlandelli",
    crate::card::CardRules::unsupported(),
);

// WWK 87 — Ricochet Trap
pub(in crate::card::sets) static RICOCHET_TRAP: CardRecord = CardRecord::new(
    CardSet::Worldwake,
    "Ricochet Trap",
    "5d782375-9192-4ed0-bd79-f3404e5a1b01",
    "Jaime Jones",
    CardRules::new_instant(mana_cost!("{3}{R}")).with_subtypes(&["Trap"]).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{R}"),
            AlternativeCastKindDef::AlternativeCost,
            Some(
                "If an opponent cast a blue spell this turn, you may pay {R} rather than pay this spell's mana cost.",
            ),
            EffectDef::None,
        )
        .with_alternative_condition(&TriggerConditionDef::ValueComparison(&ValueComparisonDef {
            left: ValueDef::CountSpellsCastThisTurn(&SpellCastQueryDef {
                player: PlayerRelation::Opponent,
                spell: ObjectPredicateDef::Color(ManaColor::Blue),
            }),
            comparison: ComparisonDef::GreaterOrEqual,
            right: ValueDef::Constant(1),
        })),
        AbilityDef::spell_with_targets(
            "Change the target of target spell with a single target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Spell,
                        ObjectPredicateDef::DeclaredTargetCount {
                            minimum: 1,
                            maximum: 1,
                        },
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::ChangeStackTargets(&crate::card::ChangeStackTargetsDef {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                chooser: PlayerRefDef::EffectController,
                change: crate::card::StackTargetChangeDef::ChooseNew {
                    optional: false,
                    restriction: None,
                },
            }),
        ),
    ]),
);

// WWK 95 — Arbor Elf
pub(in crate::card::sets) static ARBOR_ELF: CardRecord = CardRecord::new(
    CardSet::Worldwake,
    "Arbor Elf",
    "6d32a4ed-6b43-4473-91ec-08cd5414f2f0",
    "rk post",
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Druid"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Untap target Forest.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype("Forest"),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// WWK 118 — Wolfbriar Elemental
pub(in crate::card::sets) static WOLFBRIAR_ELEMENTAL: CardRecord = CardRecord::new(
    CardSet::Worldwake,
    "Wolfbriar Elemental",
    "35ffbd5e-113a-4f24-baa1-b65a5082d893",
    "Chippy",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Elemental"], 4, 4).with_abilities(&[
        abilities::multikicker(mana_cost!("{G}")),
        abilities::enters_trigger(
            "When this creature enters, create a 2/2 green Wolf creature token for each time it was kicked.",
            EffectDef::create_token(TokenCharacteristics::creature(
                &["Wolf"],
                &[ManaColor::Green],
                2,
                2,
            ))
            .with_count(ValueDef::AdditionalCostPayments(
                AdditionalCostIndex::PRIMARY,
            )),
        ),
    ]),
);

// WWK 123 — Everflowing Chalice
pub(in crate::card::sets) static EVERFLOWING_CHALICE: CardRecord = CardRecord::new(
    crate::card::CardSet::Worldwake,
    "Everflowing Chalice",
    "1fdcc0c3-4029-4fc3-a486-5d7f45c910bd",
    "Steve Argyle",
    // A mana rock whose size is chosen as it is cast, which is why it is
    // played on turn two and on turn ten.
    CardRules::new_artifact(mana_cost!("{0}")).with_abilities(&[
        abilities::multikicker(mana_cost!("{2}")),
        // The counters are the whole card: a Chalice cast for nothing is a nothing
        // that taps for nothing, and every {2} on the way in is a mana every turn
        // afterwards.
        AbilityDef::as_enters(
            "This artifact enters with a charge counter on it for each time it was kicked.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCountersValue {
                    kind: CounterKind::named("charge"),
                    amount: ValueDef::AdditionalCostPayments(AdditionalCostIndex::PRIMARY),
                },
            ),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {C} for each charge counter on this artifact.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(
                AddManaEffectDef::one(ManaColor::Colorless)
                    .with_variable_amount(ValueDef::CountersOnSource(CounterKind::named("charge"))),
            ),
        ),
    ]),
);

// WWK 126 — Kitesail
pub(in crate::card::sets) static KITESAIL: CardRecord = CardRecord::new(
    CardSet::Worldwake,
    "Kitesail",
    "217a05a7-557f-4879-8fd1-d6c003f1751e",
    "Cyril Van Der Haegen",
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+0 and has flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{2}"))],
                "Equip {2} ({2}: Attach to target creature you control. Equip only as a \
                 sorcery.)",
            ),
        ]),
);

// WWK 133 — Celestial Colonnade
pub(in crate::card::sets) static CELESTIAL_COLONNADE: CardRecord = CardRecord::new(
    CardSet::Worldwake,
    "Celestial Colonnade",
    "f6929259-2903-4f6f-9b06-42048fd55c6a",
    "Eric Deschamps",
    // A land that costs you a turn and then wins the game on its own, which
    // is the trade every control deck in the format is happy to make.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
        AbilityDef::activated(
            "{3}{W}{U}: Until end of turn, this land becomes a 4/4 white and blue Elemental \
             creature with flying and vigilance. It's still a land.",
            &[AbilityCostDef::Mana(mana_cost!("{3}{W}{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                // "It's still a land" is the type being added rather than set: everything
                // else about the animation replaces, and the land stays a land.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Elemental"])),
                    AppliedEffectDef::set_colors(ColorSet::from_colors(&[
                        ManaColor::White,
                        ManaColor::Blue,
                    ])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(4),
                        ValueDef::Constant(4),
                    ),
                    AppliedEffectDef::add_ability(&abilities::flying()),
                    AppliedEffectDef::add_ability(&abilities::vigilance()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// WWK 134 — Creeping Tar Pit
pub(in crate::card::sets) static CREEPING_TAR_PIT: CardRecord = CardRecord::new(
    crate::card::CardSet::Worldwake,
    "Creeping Tar Pit",
    "0f427f0b-034c-4821-8758-e395c0042d8a",
    "Jason Felix",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {B}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Black])),
        ),
        AbilityDef::activated(
            "{1}{U}{B}: Until end of turn, this land becomes a 3/2 blue and black Elemental creature. It's still a land. It can't be blocked this turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{U}{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Elemental"])),
                    AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Blue, ManaColor::Black])),
                    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(3), ValueDef::Constant(2)),
                    AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                        ObjectPredicateDef::Any,
                    )),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// WWK 140 — Quicksand (reprint)
const QUICKSAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::visions::QUICKSAND,
    "4e396df7-9931-43f6-b009-27cf93c4a3e5",
    "Matt Stewart",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &STONEFORGE_MYSTIC,
    &DISPEL,
    &JACE_THE_MIND_SCULPTOR,
    &BRINK_OF_DISASTER,
    &RICOCHET_TRAP,
    &ARBOR_ELF,
    &WOLFBRIAR_ELEMENTAL,
    &EVERFLOWING_CHALICE,
    &KITESAIL,
    &CELESTIAL_COLONNADE,
    &CREEPING_TAR_PIT,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[QUICKSAND_REPRINT];
