//! Coldsnap card records required by supported formats.

use crate::card::AlternativeCastKindDef;
use crate::card::BindObjectsDef;
use crate::card::CardNameDef;
use crate::card::ChangeStackTargetsDef;
use crate::card::CostQuantityDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::RevealObjectsDef;
use crate::card::StackTargetChangeDef;
use crate::card::TurnPhaseDef;
use crate::card::ValueComparisonDef;
use super::CardRecord;
use super::PrintingRecord;
use crate::ParentBinding;
use crate::TargetIndex;
use crate::ZonePlacement;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ColorSet;
use crate::card::ComparisonDef;
use crate::card::ControlDurationDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::ScaledValueDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::card::actions;
use crate::mana_cost;

const AGE_COUNTERS: ValueDef = ValueDef::CountersOnSource(CounterKind::named("age"));

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "CSP",
    slug: "coldsnap",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// CSP 3 — Cover of Winter
// Audit: unsupported — Needs one per-source combat-damage prevention budget
// that can be divided across damage assigned to you and your creatures.
pub(in crate::card::sets) static COVER_OF_WINTER: CardRecord = CardRecord::new(
    "Cover of Winter",
    "91d9bb89-d8f8-4dff-8b94-3f7b8aa8f299",
    "Wayne Reynolds",
    CardRules::unsupported(),
);

// CSP 23 — Wall of Shards
pub(in crate::card::sets) static WALL_OF_SHARDS: CardRecord = CardRecord::new(
    "Wall of Shards",
    "884ee8d8-4c0d-4e44-8321-bccd18195693",
    "Alex Horley-Orlandelli",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Wall"], 1, 8)
        .with_supertype(CardSupertype::Snow)
        .with_abilities(&[
            abilities::defender(),
            abilities::flying(),
            abilities::cumulative_upkeep(&[CostDef::gain_life(PlayerRelation::Opponent, 1)])
                .override_text("Cumulative upkeep—An opponent gains 1 life."),
        ]),
);

// CSP 27 — Arcum Dagsson
pub(in crate::card::sets) static ARCUM_DAGSSON_27: CardRecord = CardRecord::new(
    "Arcum Dagsson",
    "dd9d3ce7-53db-4808-88bc-03c120211f81",
    "Pete Venters",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Human", "Artificer"], 2, 2).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::activated_with_targets("{T}: Target artifact creature's controller sacrifices it. That player may search their library for a noncreature artifact card, put it onto the battlefield, then shuffle.", &[CostDef::TapSource], &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Artifact),ObjectPredicateDef::HasType(CardType::Creature)]))], EffectDef::Sequence(&[EffectDef::sacrifice(EffectRecipientDef::Target(TargetIndex::PRIMARY)), EffectDef::SearchZone { player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY), source: ZoneKind::Library, object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature))]), minimum: 0, maximum: ValueDef::Constant(1), reveal: false, destination: ZoneKind::Battlefield, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None }]))
]),
);

// CSP 29 — Commandeer
pub(in crate::card::sets) static COMMANDEER_29: CardRecord = CardRecord::new(
    "Commandeer",
    "7e6e6204-c622-4efe-ac4f-8195528cec9c",
    "John Matson",
    CardRules::new_instant(mana_cost!("{5}{U}{U}")).with_abilities(&[
AbilityDef::alternative_cast(&[CostDef::exile(ObjectPredicateDef::Color(ManaColor::Blue), ZoneKind::Hand, CostQuantityDef::Fixed(2))], AlternativeCastKindDef::AlternativeCost, Some("You may exile two blue cards from your hand rather than pay this spell’s mana cost."), EffectDef::None),
AbilityDef::spell_with_targets("Gain control of target noncreature spell. You may choose new targets for it. (If that spell is an artifact, enchantment, or planeswalker, the permanent enters under your control.)", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::NoncreatureSpell, zones: &[ZoneKind::Stack], controller: None, owner: None })], EffectDef::Sequence(&[EffectDef::gain_control(EffectRecipientDef::Target(TargetIndex::PRIMARY), PlayerRefDef::EffectController, ControlDurationDef::Indefinitely), EffectDef::ChangeStackTargets(&ChangeStackTargetsDef { object: EffectRecipientDef::Target(TargetIndex::PRIMARY), chooser: PlayerRefDef::EffectController, change: StackTargetChangeDef::ChooseNew { optional: true, restriction: None } })]))
]),
);

// CSP 31 — Counterbalance
pub(in crate::card::sets) static COUNTERBALANCE_31: CardRecord = CardRecord::new(
    "Counterbalance",
    "c329ff2b-0331-4934-a8df-870dd7bf402b",
    "John Zeleznik",
    CardRules::new_enchantment(mana_cost!("{U}{U}")).with_abilities(&[
AbilityDef::triggered("Whenever an opponent casts a spell, you may reveal the top card of your library. If you do, counter that spell if it has the same mana value as the revealed card.", TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent)), EffectDef::May { player: EffectRecipientDef::Controller, effect: &EffectDef::BindObjects(BindObjectsDef { source: ObjectCollectionSourceDef::TopCards { player: PlayerRefDef::EffectController, count: ValueDef::Constant(1) }, binding: Binding!("revealed_top"), then: &EffectDef::Sequence(&[EffectDef::RevealObjects(RevealObjectsDef { input: ObjectSetDef::Binding(Binding!("revealed_top")), then: &EffectDef::None }), EffectDef::ForEachInBinding { objects: Binding!("revealed_top"), binding: Binding!("revealed_card"), effect: &EffectDef::IfCondition { condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::ObjectManaValue(ObjectRefDef::TriggeringObject), comparison: ComparisonDef::Equal, right: ValueDef::ObjectManaValue(ObjectRefDef::Binding(Binding!("revealed_card"))) }), then: &EffectDef::Counter { object: EffectRecipientDef::TriggeringObject, zone: ZoneKind::Graveyard, placement: ZonePlacement::Top } } }]) }) })
]),
);

// CSP 33 — Flashfreeze
pub(in crate::card::sets) static FLASHFREEZE: CardRecord = CardRecord::new(
    "Flashfreeze",
    "cefd9955-a195-4855-a00e-3809b96ca92b",
    "Brian Despain",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target red or green spell.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Spell,
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Color(ManaColor::Red),
                        ObjectPredicateDef::Color(ManaColor::Green),
                    ]),
                ]),
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Counter {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Graveyard,
            placement: ZonePlacement::Top,
        },
    )),
);

// CSP 50 — Vexing Sphinx
pub(in crate::card::sets) static VEXING_SPHINX: CardRecord = CardRecord::new(
    "Vexing Sphinx",
    "81cc1248-85c8-428f-ba08-96d188167eaa",
    "Lars Grant-West",
CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Sphinx"], 4, 4).with_abilities(&[
        abilities::flying(),
        abilities::cumulative_upkeep(&[actions::choose_discard(1).as_cost()])
            .override_text("Cumulative upkeep—Discard a card. (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)"),
        abilities::dies_trigger(
            "When this creature dies, draw a card for each age counter on it.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: AGE_COUNTERS,
            },
        ),
    ]),
);

// CSP 51 — Balduvian Fallen
pub(in crate::card::sets) static BALDUVIAN_FALLEN: CardRecord = CardRecord::new(
    "Balduvian Fallen",
    "6a52b952-6e3b-403b-b355-2af47a282ab6",
    "Dave Kendall",
CardRules::new_creature(mana_cost!("{3}{B}"), &["Zombie"], 3, 5).with_abilities(&[
        abilities::cumulative_upkeep(
            &[CostDef::mana(mana_cost!("{1}"))],
        ),
        AbilityDef::triggered(
            "Whenever this creature's cumulative upkeep is paid, it gets +1/+0 until end of turn for each {B} or {R} spent this way.",
            TriggerEventDef::PaymentPaid { label: crate::card::abilities::CUMULATIVE_UPKEEP,
                mana_colors: ColorSet::from_colors(&[ManaColor::Black, ManaColor::Red]),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::TriggerEventAmount,
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// CSP 54 — Deathmark
pub(in crate::card::sets) static DEATHMARK: CardRecord = CardRecord::new(
    "Deathmark",
    "e72e8728-d0a0-4ee5-87c3-092ca94225e0",
    "Jeremy Jarvis",
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target green or white creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Color(ManaColor::Green),
                    ObjectPredicateDef::Color(ManaColor::White),
                ]),
            ]),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    )),
);

// CSP 62 — Herald of Leshrac
const LANDS_YOU_CONTROL_BUT_DONT_OWN: ValueDef = ValueDef::CountMatchingObjects(&ObjectQueryDef {
    object: ObjectPredicateDef::HasType(CardType::Land),
    zones: &[crate::card::ZoneKind::Battlefield],
    related_player: None,
    controller: Some(PlayerSetDef::Related(PlayerRelation::You)),
    owner: Some(PlayerSetDef::Related(PlayerRelation::NotYou)),
    relative_position: None,
    excluding_target: None,
});

pub(in crate::card::sets) static HERALD_OF_LESHRAC: CardRecord = CardRecord::new(
    "Herald of Leshrac",
    "ad6080b1-b032-4172-8594-4d894a60a80d",
    "Alex Horley-Orlandelli",
CardRules::new_creature(mana_cost!("{6}{B}"), &["Avatar"], 2, 4).with_abilities(&[
        abilities::flying(),
        abilities::cumulative_upkeep(&[
            actions::choose_gain_control(1)
                .matching(ObjectPredicateDef::HasType(CardType::Land))
                .as_cost(),
        ])
        .override_text("Cumulative upkeep—Gain control of a land you don't control."),
        AbilityDef::static_ability(
            "This creature gets +1/+1 for each land you control but don't own.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    LANDS_YOU_CONTROL_BUT_DONT_OWN,
                    LANDS_YOU_CONTROL_BUT_DONT_OWN,
                ),
            },
        ),
        AbilityDef::triggered(
            "When this creature leaves the battlefield, each player gains control of each land they own that you control.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, Some(ZoneKind::Battlefield), None),
            abilities::bind_objects_then(
                crate::card::ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Query(ObjectQueryDef {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    zones: &[ZoneKind::Battlefield],
                    related_player: None,
                    controller: Some(PlayerSetDef::Related(PlayerRelation::You)),
                    owner: None,
                    relative_position: None,
                    excluding_target: None,
                })),
                &EffectDef::ForEachInBinding {
                    objects: ParentBinding,
                    binding: ParentBinding,
                    effect: &EffectDef::gain_control(
                        EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
                        PlayerRefDef::OwnerOf(ObjectRefDef::Binding(ParentBinding)),
                        ControlDurationDef::Indefinitely,
                    ),
                },
            ),
        ),
    ]),
);

// CSP 78 — Braid of Fire
pub(in crate::card::sets) static BRAID_OF_FIRE: CardRecord = CardRecord::new(
    "Braid of Fire",
    "41bab8de-6e0f-4ccd-a303-01e9c8c82d3f",
    "Cyril Van Der Haegen",
    CardRules::new_enchantment(mana_cost!("{1}{R}")).with_ability(
        abilities::cumulative_upkeep(&[CostDef::add_mana(&AddManaEffectDef::one(ManaColor::Red))])
            .override_text("Cumulative upkeep—Add {R}."),
    ),
);

// CSP 81 — Fury of the Horde
pub(in crate::card::sets) static FURY_OF_THE_HORDE_81: CardRecord = CardRecord::new(
    "Fury of the Horde",
    "f3223ef9-787a-40ac-8ab2-79ac75664aa2",
    "Stephen Tappin",
    CardRules::new_sorcery(mana_cost!("{5}{R}{R}")).with_abilities(&[
AbilityDef::alternative_cast(&[CostDef::exile(ObjectPredicateDef::Color(ManaColor::Red), ZoneKind::Hand, CostQuantityDef::Fixed(2))], AlternativeCastKindDef::AlternativeCost, Some("You may exile two red cards from your hand rather than pay this spell’s mana cost."), EffectDef::None),
AbilityDef::spell("Untap all creatures that attacked this turn. After this main phase, there is an additional combat phase followed by an additional main phase.", EffectDef::Sequence(&[EffectDef::Untap { object: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::AttackedThisTurn]), &[ZoneKind::Battlefield], PlayerRelation::Any) }, EffectDef::ScheduleTurnPhases(&[TurnPhaseDef::Combat, TurnPhaseDef::PostcombatMain])]))
]),
);

// CSP 86 — Karplusan Minotaur
pub(in crate::card::sets) static KARPLUSAN_MINOTAUR: CardRecord = CardRecord::new(
    "Karplusan Minotaur",
    "963f45d7-ce84-47af-ae1c-727172a31f0f",
    "Wayne England",
CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Minotaur", "Warrior"], 3, 3)
        .with_abilities(&[
            abilities::cumulative_upkeep(
                &[CostDef::flip_coins(1)],
            )
                .override_text("Cumulative upkeep—Flip a coin."),
            AbilityDef::triggered_with_targets(
                "Whenever you win a coin flip, this creature deals 1 damage to any target.",
                TriggerEventDef::CoinFlipWon(PlayerRelation::You),
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(1),
                ),
            ),
            AbilityDef::triggered_with_targets(
                "Whenever you lose a coin flip, this creature deals 1 damage to any target of an opponent's choice.",
                TriggerEventDef::CoinFlipLost(PlayerRelation::You),
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)
                    .chosen_by_opponent()],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(1),
                ),
            ),
        ]),
);

// CSP 96 — Rite of Flame
pub(in crate::card::sets) static RITE_OF_FLAME_96: CardRecord = CardRecord::new(
    "Rite of Flame",
    "c062caf7-f0eb-44db-9f74-e6711a13fada",
    "Dany Orizio",
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[AbilityDef::spell(
        "Add {R}{R}, then add {R} for each card named Rite of Flame in each graveyard.",
        EffectDef::Sequence(&[
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red).with_amount(2)),
            EffectDef::AddManaEqualTo {
                color: ManaColor::Red,
                amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::NameEquals(CardNameDef::Literal("Rite of Flame")),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::Any,
                )),
            },
        ]),
    )]),
);

// CSP 102 — Arctic Nishoba
pub(in crate::card::sets) static ARCTIC_NISHOBA: CardRecord = CardRecord::new(
    "Arctic Nishoba",
    "8da62ada-b7cd-4110-a213-281f00fca3e7",
    "Dave Kendall",
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Cat", "Warrior"], 6, 6).with_abilities(&[
        abilities::trample(),
        abilities::cumulative_upkeep(&[CostDef::mana(mana_cost!("{G/W}"))])
            .override_text("Cumulative upkeep {G} or {W}"),
        abilities::dies_trigger(
            "When this creature dies, you gain 2 life for each age counter on it.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Scaled(&ScaledValueDef::new(AGE_COUNTERS, 2)),
            },
        ),
    ]),
);

// CSP 105 — Boreal Druid
pub(in crate::card::sets) static BOREAL_DRUID_105: CardRecord = CardRecord::new(
    "Boreal Druid",
    "473d3633-6dc7-4026-a50e-3ea76b9e8c20",
    "Dan Dos Santos",
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Druid"], 1, 1)
        .with_abilities(&[abilities::tap_for(ManaColor::Colorless)])
        .with_supertype(CardSupertype::Snow),
);

// CSP 138 — Mishra's Bauble
pub(in crate::card::sets) static MISHRA_S_BAUBLE: CardRecord = CardRecord::new(
    "Mishra's Bauble",
    "8a720448-017f-4f4a-9501-678245eaed17",
    "Chippy",
    // A free artifact that replaces itself a turn later. The looking is
    // incidental; what the card is played for is being an artifact that cost
    // nothing and a card that comes back.
    CardRules::new_artifact(mana_cost!("{0}")).with_ability(AbilityDef::activated_with_targets(
        "{T}, Sacrifice this artifact: Look at the top card of target player's library. Draw a \
         card at the beginning of the next turn's upkeep.",
        &[CostDef::TapSource, CostDef::SacrificeSource],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            abilities::look_at_top_cards(
                PlayerRefDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                "Draw a card at the beginning of the next turn's upkeep.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::Any,
                },
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ))),
        ]),
    )),
);

// CSP 141 — Phyrexian Soulgorger
pub(in crate::card::sets) static PHYREXIAN_SOULGORGER: CardRecord = CardRecord::new(
    "Phyrexian Soulgorger",
    "9d4325ea-2e84-4871-a8d6-a42b1d3d6765",
    "Brian Snõddy",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Phyrexian", "Construct"], 8, 8)
        .with_supertype(CardSupertype::Snow)
        .with_ability(
            abilities::cumulative_upkeep(&[actions::choose_sacrifice(1)
                .matching(ObjectPredicateDef::HasType(CardType::Creature))
                .as_cost()])
            .override_text("Cumulative upkeep—Sacrifice a creature."),
        ),
);

// CSP 145 — Dark Depths
pub(in crate::card::sets) static DARK_DEPTHS: CardRecord = CardRecord::new(
    "Dark Depths",
    "92409c3a-fb1a-4205-9fe1-0f5affc7b21d",
    "Stephan Martiniere",
// Thirty mana the long way round, or none at all if something else takes
    // the counters off.
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_supertype(CardSupertype::Snow)
        .with_abilities(&[
            AbilityDef::as_enters(
                "Dark Depths enters with ten ice counters on it.",
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCounters {
                        kind: CounterKind::named("ice"),
                        amount: 10,
                    },
                ),
            ),
            AbilityDef::activated(
                "{3}: Remove an ice counter from Dark Depths.",
                &[CostDef::Mana(mana_cost!("{3}"))],
                EffectDef::RemoveCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("ice"),
                    amount: ValueDef::Constant(1),
                },
            ),
            // A state trigger (CR 603.8): it has no event, and it fires whenever the
            // counters are gone -- however they went. Removing them all at once is
            // what the deck is really built to do.
            AbilityDef::triggered_if(
                "When Dark Depths has no ice counters on it, sacrifice it. If you do, create Marit Lage, \
                 a legendary 20/20 black Avatar creature token with flying and indestructible.",
                TriggerEventDef::StateCondition,
                &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("ice"),
                    comparison: ComparisonDef::Equal,
                    amount: 0,
                },
                // "Sacrifice it. If you do, create Marit Lage." Nothing stops a player
                // sacrificing their own permanent, so the only way the sacrifice fails is
                // that the land is no longer there to sacrifice -- which is what this asks,
                // and why an answer in response to the trigger denies the token.
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceOnBattlefield,
                    then: &EffectDef::Sequence(&[
                        EffectDef::sacrifice(EffectRecipientDef::Source),
                        // Twenty power for no mana at all, which is what the ten counters are
                        // paying for. Legendary, so a second one is not a plan.
                        EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                            TokenCharacteristics::creature(&["Avatar"], &[ManaColor::Black], 20, 20)
                                .with_supertype(CardSupertype::Legendary)
                                .with_name("Marit Lage")
                                .with_abilities(&[abilities::flying(), abilities::indestructible()]),
                        ))),
                    ]),
                },
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &COVER_OF_WINTER,
    &WALL_OF_SHARDS,
    &ARCUM_DAGSSON_27,
    &COMMANDEER_29,
    &COUNTERBALANCE_31,
    &FLASHFREEZE,
    &VEXING_SPHINX,
    &BALDUVIAN_FALLEN,
    &DEATHMARK,
    &HERALD_OF_LESHRAC,
    &BRAID_OF_FIRE,
    &FURY_OF_THE_HORDE_81,
    &KARPLUSAN_MINOTAUR,
    &RITE_OF_FLAME_96,
    &ARCTIC_NISHOBA,
    &BOREAL_DRUID_105,
    &MISHRA_S_BAUBLE,
    &PHYREXIAN_SOULGORGER,
    &DARK_DEPTHS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
