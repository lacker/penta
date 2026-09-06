//! Coldsnap card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef, AppliedEffectDef,
    BattlefieldEntryModificationDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    ColorSet, ComparisonDef, CostDef, CounterKind, EffectDef, EffectRecipientDef,
    InstalledTriggerDef, ManaColor, ObjectPredicateDef, ObjectQueryDef, PlayerRefDef,
    PlayerRelation, PlayerSetDef, ReplacementEffectDef, ResolvedEffectDurationDef, ScaledValueDef,
    TokenCharacteristics, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, abilities,
};
use crate::{TargetIndex, mana_cost};

const AGE_COUNTERS: ValueDef = ValueDef::CountersOnSource(CounterKind::named("age"));

// CSP 3 — Cover of Winter
// Audit: unsupported — Needs one per-source combat-damage prevention budget
// that can be divided across damage assigned to you and your creatures.
pub(in crate::card::sets) static COVER_OF_WINTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("91d9bb89-d8f8-4dff-8b94-3f7b8aa8f299"),
    "Cover of Winter",
    CardArt::new("91d9bb89-d8f8-4dff-8b94-3f7b8aa8f299", "Wayne Reynolds"),
    CardSet::Coldsnap,
    CardRules::unsupported(),
);

// CSP 23 — Wall of Shards
pub(in crate::card::sets) static WALL_OF_SHARDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("884ee8d8-4c0d-4e44-8321-bccd18195693"),
    "Wall of Shards",
    CardArt::new(
        "884ee8d8-4c0d-4e44-8321-bccd18195693",
        "Alex Horley-Orlandelli",
    ),
    CardSet::Coldsnap,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Wall"], 1, 8)
        .with_supertype(CardSupertype::Snow)
        .with_abilities(&[
            abilities::defender(),
            abilities::flying(),
            abilities::cumulative_upkeep(CostDef::gain_life(PlayerRelation::Opponent, 1))
                .override_text("Cumulative upkeep—An opponent gains 1 life."),
        ]),
);

// CSP 50 — Vexing Sphinx
pub(in crate::card::sets) static VEXING_SPHINX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("81cc1248-85c8-428f-ba08-96d188167eaa"),
    "Vexing Sphinx",
    CardArt::new("81cc1248-85c8-428f-ba08-96d188167eaa", "Lars Grant-West"),
    CardSet::Coldsnap,
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Sphinx"], 4, 4).with_abilities(&[
        abilities::flying(),
        abilities::cumulative_upkeep(CostDef::discard_cards(1)),
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
    PrintingAnchor::scryfall("6a52b952-6e3b-403b-b355-2af47a282ab6"),
    "Balduvian Fallen",
    CardArt::new("6a52b952-6e3b-403b-b355-2af47a282ab6", "Dave Kendall"),
    CardSet::Coldsnap,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Zombie"], 3, 5).with_abilities(&[
        abilities::cumulative_upkeep(CostDef::mana(mana_cost!("{1}"))),
        AbilityDef::triggered(
            "Whenever this creature's cumulative upkeep is paid, it gets +1/+0 until end of turn for each {B} or {R} spent this way.",
            TriggerEventDef::CumulativeUpkeepPaid {
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
    PrintingAnchor::scryfall("ad6080b1-b032-4172-8594-4d894a60a80d"),
    "Herald of Leshrac",
    CardArt::new(
        "ad6080b1-b032-4172-8594-4d894a60a80d",
        "Alex Horley-Orlandelli",
    ),
    CardSet::Coldsnap,
    CardRules::new_creature(mana_cost!("{6}{B}"), &["Avatar"], 2, 4).with_abilities(&[
        abilities::flying(),
        abilities::cumulative_upkeep(CostDef::gain_control_permanents(
            ObjectPredicateDef::HasType(CardType::Land),
            1,
        ))
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
    ]),
);

// CSP 78 — Braid of Fire
pub(in crate::card::sets) static BRAID_OF_FIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("41bab8de-6e0f-4ccd-a303-01e9c8c82d3f"),
    "Braid of Fire",
    CardArt::new(
        "41bab8de-6e0f-4ccd-a303-01e9c8c82d3f",
        "Cyril Van Der Haegen",
    ),
    CardSet::Coldsnap,
    CardRules::new_enchantment(mana_cost!("{1}{R}")).with_ability(
        abilities::cumulative_upkeep(CostDef::add_mana(&AddManaEffectDef::one(ManaColor::Red)))
            .override_text("Cumulative upkeep—Add {R}."),
    ),
);

// CSP 86 — Karplusan Minotaur
pub(in crate::card::sets) static KARPLUSAN_MINOTAUR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("963f45d7-ce84-47af-ae1c-727172a31f0f"),
    "Karplusan Minotaur",
    CardArt::new("963f45d7-ce84-47af-ae1c-727172a31f0f", "Wayne England"),
    CardSet::Coldsnap,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Minotaur", "Warrior"], 3, 3)
        .with_abilities(&[
            abilities::cumulative_upkeep(CostDef::flip_coins(1))
                .override_text("Cumulative upkeep—Flip a coin."),
            AbilityDef::triggered_with_targets(
                "Whenever you win a coin flip, this creature deals 1 damage to any target.",
                TriggerEventDef::CoinFlipWon(PlayerRelation::You),
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::triggered_with_targets(
                "Whenever you lose a coin flip, this creature deals 1 damage to any target of an opponent's choice.",
                TriggerEventDef::CoinFlipLost(PlayerRelation::You),
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)
                    .chosen_by_opponent()],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// CSP 102 — Arctic Nishoba
pub(in crate::card::sets) static ARCTIC_NISHOBA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8da62ada-b7cd-4110-a213-281f00fca3e7"),
    "Arctic Nishoba",
    CardArt::new("8da62ada-b7cd-4110-a213-281f00fca3e7", "Dave Kendall"),
    CardSet::Coldsnap,
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Cat", "Warrior"], 6, 6).with_abilities(&[
        abilities::trample(),
        abilities::cumulative_upkeep(CostDef::mana(mana_cost!("{G/W}")))
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

// CSP 138 — Mishra's Bauble
pub(in crate::card::sets) static MISHRA_S_BAUBLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8a720448-017f-4f4a-9501-678245eaed17"),
    "Mishra's Bauble",
    CardArt::new("8a720448-017f-4f4a-9501-678245eaed17", "Chippy"),
    CardSet::Coldsnap,
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
    PrintingAnchor::scryfall("9d4325ea-2e84-4871-a8d6-a42b1d3d6765"),
    "Phyrexian Soulgorger",
    CardArt::new("9d4325ea-2e84-4871-a8d6-a42b1d3d6765", "Brian Snõddy"),
    CardSet::Coldsnap,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Phyrexian", "Construct"], 8, 8)
        .with_supertype(CardSupertype::Snow)
        .with_ability(
            abilities::cumulative_upkeep(CostDef::sacrifice_permanents(
                ObjectPredicateDef::HasType(CardType::Creature),
                PlayerRelation::You,
                1,
            ))
            .override_text("Cumulative upkeep—Sacrifice a creature."),
        ),
);

// CSP 145 — Dark Depths
pub(in crate::card::sets) static DARK_DEPTHS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("92409c3a-fb1a-4205-9fe1-0f5affc7b21d"),
    "Dark Depths",
    CardArt::new("92409c3a-fb1a-4205-9fe1-0f5affc7b21d", "Stephan Martiniere"),
    CardSet::Coldsnap,
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
                        EffectDef::Sacrifice {
                            object: EffectRecipientDef::Source,
                        },
                        // Twenty power for no mana at all, which is what the ten counters are
                        // paying for. Legendary, so a second one is not a plan.
                        EffectDef::create_token(TokenCharacteristics::creature(&["Avatar"], &[ManaColor::Black], 20, 20)
                                .with_supertype(CardSupertype::Legendary)
                                .with_name("Marit Lage")
                                .with_abilities(&[abilities::flying(), abilities::indestructible()])),
                    ]),
                },
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &COVER_OF_WINTER,
    &WALL_OF_SHARDS,
    &VEXING_SPHINX,
    &BALDUVIAN_FALLEN,
    &HERALD_OF_LESHRAC,
    &BRAID_OF_FIRE,
    &KARPLUSAN_MINOTAUR,
    &ARCTIC_NISHOBA,
    &MISHRA_S_BAUBLE,
    &PHYREXIAN_SOULGORGER,
    &DARK_DEPTHS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
