//! Worldwake cards cataloged for the Vintage Cube.

use super::CardRecord;
use super::PrintingRecord;
use crate::AdditionalCostIndex;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardChoiceSourceDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::ColorSet;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DestroyFollowUpDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PayOrDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SpellCastQueryDef;
use crate::card::SubtypeDef;
use crate::card::TapEventMatcherDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "WWK",
    slug: "worldwake",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// WWK 20 — Stoneforge Mystic
pub(in crate::card::sets) static STONEFORGE_MYSTIC: CardRecord = CardRecord::new(
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
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Equipment")),
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
                    CostDef::Mana(mana_cost!("{1}{W}")),
                    CostDef::TapSource,
                ],
                // The second half of the card, and the reason the first half is worth
                // finding: a minimum of zero is the printed "you may", and with no
                // Equipment in hand the choice is never offered at all.
                EffectDef::ChooseCards {
                    player: EffectRecipientDef::Controller,
                    sources: &[CardChoiceSourceDef::Zone(ZoneKind::Hand)],
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Equipment")),
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
                    &[CostDef::Loyalty(2)],
                    &A_PLAYER,
                    abilities::fateseal(
                        PlayerRefDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(1),
                    ),
                ),
                AbilityDef::activated(
                    "0: Draw three cards, then put two cards from your hand on top of your library in any \
                     order.",
                    &[CostDef::Loyalty(0)],
                    abilities::brainstorm(),
                ),
                AbilityDef::activated_with_targets(
                    "−1: Return target creature to its owner's hand.",
                    &[CostDef::Loyalty(-1)],
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                ),
                AbilityDef::activated_with_targets(
                    "−12: Exile all cards from target player's library, then that player shuffles their hand \
                     into their library.",
                    &[CostDef::Loyalty(-12)],
                    &A_PLAYER,
                    EffectDef::Sequence(&[
                        EffectDef::move_to_zone(
                            EffectRecipientDef::objects(crate::card::ObjectSetDef::Query(
                                // Their whole library, named by owner rather than by relation: the ultimate
                                // points at a player and empties that one.
                                ObjectQueryDef::owned_by(
                                    ObjectPredicateDef::Any,
                                    &[ZoneKind::Library],
                                    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                                ),
                            )),
                            ZoneKind::Exile,
                            ZonePlacement::Top,
                        ),
                        EffectDef::move_to_zone(
                            EffectRecipientDef::objects(crate::card::ObjectSetDef::Query(
                                ObjectQueryDef::owned_by(
                                    ObjectPredicateDef::Any,
                                    &[ZoneKind::Hand],
                                    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                                ),
                            )),
                            ZoneKind::Library,
                            ZonePlacement::Top,
                        ),
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
pub(in crate::card::sets) static BRINK_OF_DISASTER: CardRecord = CardRecord::new(
    "Brink of Disaster",
    "0c841c3e-e0d1-49d7-bcec-3c45f73c13c5",
    "Alex Horley-Orlandelli",
    CardRules::new_enchantment(mana_cost!("{2}{B}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature or land",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Land),
                    ]),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::triggered(
                "When enchanted permanent becomes tapped, destroy it.",
                TriggerEventDef::Tapped(TapEventMatcherDef::any(
                    ObjectPredicateDef::AttachedToSource,
                )),
                EffectDef::Destroy {
                    object: EffectRecipientDef::TriggeringObject,
                    then: None,
                },
            ),
        ]),
);

// WWK 59 — Kalastria Highborn
pub(in crate::card::sets) static KALASTRIA_HIGHBORN: CardRecord = CardRecord::new(
    "Kalastria Highborn",
    "f1efd1dd-903c-47a0-b746-5571a3ea1755",
    "D. Alexander Gregory",
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Vampire", "Shaman"], 2, 2).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever this creature or another Vampire you control dies, \
             you may pay {B}. If you do, target player loses 2 life and \
             you gain 2 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vampire")),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::Mana(mana_cost!("{B}"))],
                &EffectDef::Sequence(&[
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(2),
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                    },
                ]),
            )),
        ),
    ]),
);

// WWK 62 — Pulse Tracker
pub(in crate::card::sets) static PULSE_TRACKER: CardRecord = CardRecord::new(
    "Pulse Tracker",
    "4604a63c-ebe0-420f-968e-3ffc7641ce22",
    "Andrew Robinson",
    CardRules::new_creature(mana_cost!("{B}"), &["Vampire", "Rogue"], 1, 1).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks, each opponent loses 1 life.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// WWK 81 — Dragonmaster Outcast
pub(in crate::card::sets) static DRAGONMASTER_OUTCAST: CardRecord = CardRecord::new(
    "Dragonmaster Outcast",
    "c2297a4e-3c19-4748-9150-efbd2513066a",
    "Raymond Swanland",
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Shaman"], 1, 1).with_abilities(&[
        AbilityDef::triggered_if(
            "At the beginning of your upkeep, if you control six or more \
             lands, create a 5/5 red Dragon creature token with flying.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 6,
            },
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Dragon"], &[ManaColor::Red], 5, 5)
                    .with_abilities(&[abilities::flying()]),
            ))),
        ),
    ]),
);

// WWK 87 — Ricochet Trap
pub(in crate::card::sets) static RICOCHET_TRAP: CardRecord = CardRecord::new(
    "Ricochet Trap",
    "5d782375-9192-4ed0-bd79-f3404e5a1b01",
    "Jaime Jones",
CardRules::new_instant(mana_cost!("{3}{R}")).with_subtypes(&["Trap"]).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{R}"))],
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
    "Arbor Elf",
    "6d32a4ed-6b43-4473-91ec-08cd5414f2f0",
    "rk post",
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Druid"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Untap target Forest.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Forest")),
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

// WWK 108 — Nature's Claim
pub(in crate::card::sets) static NATURE_S_CLAIM_108: CardRecord = CardRecord::new(
    "Nature's Claim",
    "64ae5a91-ac54-4222-832e-d7a740a3f7cb",
    "Daarken",
    CardRules::new_instant(mana_cost!("{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target artifact or enchantment. Its controller gains 4 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Enchantment),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::destroy_target(TargetIndex::PRIMARY),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
        ]),
    )]),
);

// WWK 115 — Terastodon
pub(in crate::card::sets) static TERASTODON_115: CardRecord = CardRecord::new(
    "Terastodon",
    "e66d2f62-8a4a-4e8d-93e1-5dc802684106",
    "Lars Grant-West",
    CardRules::new_creature(mana_cost!("{6}{G}{G}"), &["Elephant"], 9, 9).with_abilities(&[
AbilityDef::triggered_with_targets("When this creature enters, you may destroy up to three target noncreature permanents. For each permanent put into a graveyard this way, its controller creates a 3/3 green Elephant creature token.", TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)), &[AbilityTargetDef::up_to(AbilityTargetPredicate::Object { object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)), zones: &[ZoneKind::Battlefield], controller: None, owner: None }, 3)], EffectDef::May { player: EffectRecipientDef::Controller, effect: &EffectDef::Destroy { object: EffectRecipientDef::Target(TargetIndex::PRIMARY), then: Some(DestroyFollowUpDef { binding: Binding!("terastodon_destroyed"), effect: &EffectDef::ForEachInBinding { objects: Binding!("terastodon_destroyed"), binding: Binding!("terastodon_permanent"), effect: &EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::TokenCharacteristics::creature(&["Elephant"], &[ManaColor::Green], 3, 3))).with_controller(PlayerRefDef::ControllerOf(ObjectRefDef::Binding(Binding!("terastodon_permanent"))))) } }) } })
]),
);

// WWK 118 — Wolfbriar Elemental
pub(in crate::card::sets) static WOLFBRIAR_ELEMENTAL: CardRecord = CardRecord::new(
    "Wolfbriar Elemental",
    "35ffbd5e-113a-4f24-baa1-b65a5082d893",
    "Chippy",
CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Elemental"], 4, 4).with_abilities(&[
        abilities::multikicker(
            &[CostDef::Mana(mana_cost!("{G}"))],
        ),
        abilities::enters_trigger(
            "When this creature enters, create a 2/2 green Wolf creature token for each time it was kicked.",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                    &["Wolf"],
                    &[ManaColor::Green],
                    2,
                    2,
                )))
                .with_count(ValueDef::AdditionalCostPayments(
                    AdditionalCostIndex::PRIMARY,
                )),
            ),
        ),
    ]),
);

// WWK 122 — Basilisk Collar
pub(in crate::card::sets) static BASILISK_COLLAR: CardRecord = CardRecord::new(
    "Basilisk Collar",
    "55cdba1b-7a80-435f-9cff-b9365f62e311",
    "Howard Lyon",
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has deathtouch and lifelink. (Any amount of \
                 damage it deals to a creature is enough to destroy it. Damage \
                 dealt by this creature also causes you to gain that much \
                 life.)",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::deathtouch()),
                        AppliedEffectDef::add_ability(&abilities::lifelink()),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// WWK 123 — Everflowing Chalice
pub(in crate::card::sets) static EVERFLOWING_CHALICE: CardRecord = CardRecord::new(
    "Everflowing Chalice",
    "1fdcc0c3-4029-4fc3-a486-5d7f45c910bd",
    "Steve Argyle",
    // A mana rock whose size is chosen as it is cast, which is why it is
    // played on turn two and on turn ten.
    CardRules::new_artifact(mana_cost!("{0}")).with_abilities(&[
        abilities::multikicker(&[CostDef::Mana(mana_cost!("{2}"))]),
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
            &[CostDef::TapSource],
            EffectDef::AddMana(
                AddManaEffectDef::one(ManaColor::Colorless)
                    .with_variable_amount(ValueDef::CountersOnSource(CounterKind::named("charge"))),
            ),
        ),
    ]),
);

// WWK 126 — Kitesail
pub(in crate::card::sets) static KITESAIL: CardRecord = CardRecord::new(
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
                &[CostDef::Mana(mana_cost!("{2}"))],
                "Equip {2} ({2}: Attach to target creature you control. Equip only as a \
                 sorcery.)",
            ),
        ]),
);

// WWK 127 — Lodestone Golem
pub(in crate::card::sets) static LODESTONE_GOLEM_127: CardRecord = CardRecord::new(
    "Lodestone Golem",
    "9bb0ee6a-852a-4f1e-8f03-40b6d505bc82",
    "Chris Rahn",
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Golem"], 5, 3).with_abilities(&[
        AbilityDef::static_ability(
            "Nonartifact spells cost {1} more to cast.",
            EffectDef::ModifyCost(CostModificationDef::increase_spell(
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Artifact)),
                PlayerRelation::Any,
                mana_cost!("{1}"),
            )),
        ),
    ]),
);

// WWK 132 — Bojuka Bog
pub(in crate::card::sets) static BOJUKA_BOG_132: CardRecord = CardRecord::new(
    "Bojuka Bog",
    "529c38b3-7397-4dac-9859-acd9cd451c32",
    "Howard Lyon",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::triggered_with_targets(
            "When this land enters, exile target player's graveyard.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::owned_by(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Graveyard],
                    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                ))),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {B}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
    ]),
);

// WWK 133 — Celestial Colonnade
pub(in crate::card::sets) static CELESTIAL_COLONNADE: CardRecord = CardRecord::new(
    "Celestial Colonnade",
    "f6929259-2903-4f6f-9b06-42048fd55c6a",
    "Eric Deschamps",
    // A land that costs you a turn and then wins the game on its own, which
    // is the trade every control deck in the format is happy to make.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
        AbilityDef::activated(
            "{3}{W}{U}: Until end of turn, this land becomes a 4/4 white and blue Elemental \
             creature with flying and vigilance. It's still a land.",
            &[CostDef::Mana(mana_cost!("{3}{W}{U}"))],
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
    "Creeping Tar Pit",
    "0f427f0b-034c-4821-8758-e395c0042d8a",
    "Jason Felix",
CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {B}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Black])),
        ),
        AbilityDef::activated(
            "{1}{U}{B}: Until end of turn, this land becomes a 3/2 blue and black Elemental creature. It's still a land. It can't be blocked this turn.",
            &[CostDef::Mana(mana_cost!("{1}{U}{B}"))],
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

// WWK 145 — Tectonic Edge
pub(in crate::card::sets) static TECTONIC_EDGE_145: CardRecord = CardRecord::new(
    "Tectonic Edge",
    "fdcf5c0f-9d18-406d-a930-c179a781264f",
    "Vincent Proce",
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated_with_targets(
            "{1}, {T}, Sacrifice this land: Destroy target nonbasic land. Activate only \
             if an opponent controls four or more lands.",
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
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY),
        )
        .with_activation_condition(&TriggerConditionDef::ObjectCount {
            query: ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Land),
                &[ZoneKind::Battlefield],
                PlayerRelation::Opponent,
            ),
            comparison: ComparisonDef::GreaterOrEqual,
            amount: 4,
        }),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &STONEFORGE_MYSTIC,
    &DISPEL,
    &JACE_THE_MIND_SCULPTOR,
    &BRINK_OF_DISASTER,
    &KALASTRIA_HIGHBORN,
    &PULSE_TRACKER,
    &DRAGONMASTER_OUTCAST,
    &RICOCHET_TRAP,
    &ARBOR_ELF,
    &NATURE_S_CLAIM_108,
    &TERASTODON_115,
    &WOLFBRIAR_ELEMENTAL,
    &BASILISK_COLLAR,
    &EVERFLOWING_CHALICE,
    &KITESAIL,
    &LODESTONE_GOLEM_127,
    &BOJUKA_BOG_132,
    &CELESTIAL_COLONNADE,
    &CREEPING_TAR_PIT,
    &TECTONIC_EDGE_145,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[QUICKSAND_REPRINT];
