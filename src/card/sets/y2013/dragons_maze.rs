//! Dragon's Maze card records used by the built-in ISD–DGM Standard decks.

use super::{CardRecord, PrintingRecord, gatecrash};
use crate::ManaCost;
use crate::card::sets::y2012::return_to_ravnica;
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AppliedEffectDef, AppliedRuleDef, CardArt, CardBehavior, CardComposition, CardEffectStatus,
    CardPart, CardRules, CardSet, CardStructure, CardSupertype, CardType, CardTypeSet,
    ChoiceVisibilityDef, ChooseDef, ColorSet, ComparisonDef, ControlDurationDef, CounterKind,
    CreatureTypeSetDef, DamageEventMatcherDef, DamagePreventionDef, DiscardSelectionDef, EffectDef,
    EffectPaymentDef, EffectRecipientDef, InstalledTriggerDef, ManaColor, ObjectChoiceBindingDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, PayOrDef, PlayOptionDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef, SpellForm,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement,
    abilities, cards,
};
use crate::ids::{CardPartId, ObjectBindingIndex, PlayOptionId, TargetIndex};
use crate::mana_cost;

static MULTICOLORED: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::ColorCount(2),
    ObjectPredicateDef::ColorCount(3),
    ObjectPredicateDef::ColorCount(4),
    ObjectPredicateDef::ColorCount(5),
]);

static BATTALION_CONDITION: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::Attacking,
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 3,
};

static TWO_GATES_CONDITION: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::Subtype("Gate"),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 2,
};

// DGM 1 — Boros Mastiff
// Audit: partial — Battalion's attack-count restriction is rechecked on resolution as though it were an intervening-if condition.
pub(in crate::card::sets) static BOROS_MASTIFF: CardRecord = CardRecord::new(
    cards::BOROS_MASTIFF,
    "Boros Mastiff",
    CardArt::new("27a3bfb6-3843-4bda-bbcb-905e4b351dea", "Kev Walker"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Dog"], 2, 2).with_ability(
        AbilityDef::triggered_if(
            "Battalion — Whenever this creature and at least two other creatures attack, this creature gains lifelink until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &BATTALION_CONDITION,
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "The attack-count condition is incorrectly rechecked when the trigger resolves.",
        )),
    ),
);

// DGM 2 — Haazda Snare Squad
pub(in crate::card::sets) static HAAZDA_SNARE_SQUAD: CardRecord = CardRecord::new(
    cards::HAAZDA_SNARE_SQUAD,
    "Haazda Snare Squad",
    CardArt::new(
        "85d3c012-f356-424d-a960-60e95f395134",
        "David Palumbo",
    ),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 1, 4).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, you may pay {W}. If you do, tap target creature an opponent controls.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef::mana(
                    PlayerSetDef::Related(PlayerRelation::You),
                    mana_cost!("{W}"),
                ),
                &EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            )),
        ),
    ),
);

// DGM 3 — Lyev Decree
pub(in crate::card::sets) static LYEV_DECREE: CardRecord = CardRecord::new(
    cards::LYEV_DECREE,
    "Lyev Decree",
    CardArt::new("773cf2aa-4337-4d14-8a8e-ff8b1fdec1b5", "Kev Walker"),
    CardSet::DragonsMaze,
    CardRules::new_sorcery(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Detain up to two target creatures your opponents control.",
        &LYEV_DECREE_TARGETS,
        EffectDef::Detain {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )),
);

static LYEV_DECREE_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::Opponent),
        owner: None,
    },
    2,
)];

// DGM 4 — Maze Sentinel
pub(in crate::card::sets) static MAZE_SENTINEL: CardRecord = CardRecord::new(
    cards::MAZE_SENTINEL,
    "Maze Sentinel",
    CardArt::new("7a977e2d-a2bc-42d1-be7d-36a822c6a66e", "Yeong-Hao Han"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{5}{W}"), &["Elemental"], 3, 6).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::static_ability(
            "Multicolored creatures you control have vigilance.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    MULTICOLORED,
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::vigilance()),
            },
        ),
    ]),
);

// DGM 5 — Renounce the Guilds
pub(in crate::card::sets) static RENOUNCE_THE_GUILDS: CardRecord = CardRecord::new(
    cards::RENOUNCE_THE_GUILDS,
    "Renounce the Guilds",
    CardArt::new("bc9acc14-24e0-4c03-a09a-2afee351f2cc", "Daarken"),
    CardSet::DragonsMaze,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell(
        "Each player sacrifices a multicolored permanent of their choice.",
        EffectDef::SacrificeOfChoice {
            player: EffectRecipientDef::EachPlayer,
            object: MULTICOLORED,
            then: None,
            optional: false,
        },
    )),
);

static RIOT_CONTROL_OPPONENT_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::Opponent,
);

// DGM 6 — Riot Control
pub(in crate::card::sets) static RIOT_CONTROL: CardRecord = CardRecord::new(
    cards::RIOT_CONTROL,
    "Riot Control",
    CardArt::new("d7886607-86db-4221-8752-296104aaaef2", "Slawomir Maniak"),
    CardSet::DragonsMaze,
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell(
        "You gain 1 life for each creature your opponents control. Prevent all damage that would be dealt to you this turn.",
        EffectDef::Sequence(&[
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::CountMatchingObjects(&RIOT_CONTROL_OPPONENT_CREATURES),
            },
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::to(
                    EffectRecipientDef::Controller,
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// DGM 7 — Scion of Vitu-Ghazi
// Audit: blocked — Needs an enters-trigger condition that remembers whether the permanent was cast from hand, plus populate's token-copy choice.

// DGM 8 — Steeple Roc
pub(in crate::card::sets) static STEEPLE_ROC: CardRecord = CardRecord::new(
    cards::STEEPLE_ROC,
    "Steeple Roc",
    CardArt::new("5fecafab-97f4-40ed-bc43-d186eb2f3af6", "David Palumbo"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Bird"], 3, 1)
        .with_abilities(&[abilities::flying(), abilities::first_strike()]),
);

// DGM 9 — Sunspire Gatekeepers
pub(in crate::card::sets) static SUNSPIRE_GATEKEEPERS: CardRecord = CardRecord::new(
    cards::SUNSPIRE_GATEKEEPERS,
    "Sunspire Gatekeepers",
    CardArt::new("0a3bc6b9-475b-4257-a3bc-1a0b70d45f79", "Chippy"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Soldier"], 2, 4).with_ability(
        AbilityDef::triggered_if(
            "When this creature enters, if you control two or more Gates, create a 2/2 white Knight creature token with vigilance.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
            &TWO_GATES_CONDITION,
            EffectDef::CreateToken {
                token: cards::KNIGHT_TOKEN_2_2_WHITE,
                count: ValueDef::Constant(1),
                tapped: false,
            },
        ),
    ),
);

// DGM 10 — Wake the Reflections
// Audit: blocked — Needs populate's choice of a controlled creature token and token-copy creation.

// DGM 11 — Aetherling
pub(in crate::card::sets) static AETHERLING: CardRecord = CardRecord::new(
    cards::AETHERLING,
    "Aetherling",
    CardArt::new("9c93313b-cf43-47e9-a911-717b4d14b0b5", "Tyler Jacobson"),
    CardSet::DragonsMaze,
    CardRules::new_creature(
        mana_cost!("{4}{U}{U}"),
        &["Shapeshifter"],
        4,
        5,
    )
    .with_abilities(&[
        AbilityDef::activated(
            "{U}: Exile this creature. Return it to the battlefield under its owner's control at the beginning of the next end step.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Sequence(&[
                EffectDef::ExileLinkedToSource {
                    object: EffectRecipientDef::Source,
                },
                // The next end step belongs to whoever's turn it is, which
                // may well be the opponent.
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "At the beginning of the next end step, return the exiled cards to the battlefield under their owner's control.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::End,
                        player: PlayerRelation::Any,
                    },
                    EffectDef::ReturnLinkedExiles {
                        zone: ZoneKind::Battlefield,
                        grant: None,
                    },
                ))),
            ]),
        ),
        AbilityDef::activated(
            "{U}: This creature can't be blocked this turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlockedBy(
                    ObjectPredicateDef::Any,
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{1}: This creature gets +1/-1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(-1)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{1}: This creature gets -1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(-1), ValueDef::Constant(1)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DGM 12 — Hidden Strings
// Audit: blocked — Needs tap-or-untap choices made independently on resolution and cipher's encoded-card link, combat-damage trigger, and free-copy casting permission.

// DGM 13 — Maze Glider
pub(in crate::card::sets) static MAZE_GLIDER: CardRecord = CardRecord::new(
    cards::MAZE_GLIDER,
    "Maze Glider",
    CardArt::new("d1d20281-49c0-4fd0-91f2-390506ac33f6", "Yeong-Hao Han"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{5}{U}"), &["Elemental"], 3, 5).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "Multicolored creatures you control have flying.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    MULTICOLORED,
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
            },
        ),
    ]),
);

// DGM 14 — Mindstatic
pub(in crate::card::sets) static MINDSTATIC: CardRecord = CardRecord::new(
    cards::MINDSTATIC,
    "Mindstatic",
    CardArt::new("55d3fad5-a12a-4b41-9c7b-c1af5e0b5ca8", "Johann Bodin"),
    CardSet::DragonsMaze,
    CardRules::new_instant(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell unless its controller pays {6}.",
        &[AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Any)],
        abilities::counter_target_unless_paid(ValueDef::Constant(6)),
    )),
);

// DGM 15 — Murmuring Phantasm
pub(in crate::card::sets) static MURMURING_PHANTASM: CardRecord = CardRecord::new(
    cards::MURMURING_PHANTASM,
    "Murmuring Phantasm",
    CardArt::new("9752644c-7c43-429e-a79c-1239b9a0bc8a", "Peter Mohrbacher"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Spirit"], 0, 5)
        .with_ability(abilities::defender()),
);

// DGM 16 — Opal Lake Gatekeepers
pub(in crate::card::sets) static OPAL_LAKE_GATEKEEPERS: CardRecord = CardRecord::new(
    cards::OPAL_LAKE_GATEKEEPERS,
    "Opal Lake Gatekeepers",
    CardArt::new("f43ac38f-5cd0-46cf-8623-d82cb8fb719b", "Seb McKinnon"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Vedalken", "Soldier"], 2, 4).with_ability(
        AbilityDef::triggered_if(
            "When this creature enters, if you control two or more Gates, you may draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TWO_GATES_CONDITION,
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            },
        ),
    ),
);

// DGM 17 — Runner's Bane
// Audit: partial — Power-based Aura legality ignores continuous static power modifiers when offering and rechecking the enchanted creature.
pub(in crate::card::sets) static RUNNERS_BANE: CardRecord = CardRecord::new(
    cards::RUNNERS_BANE,
    "Runner's Bane",
    CardArt::new("4696b5a6-edfd-445e-ac80-64c1be94fbfc", "Karl Kopinski"),
    CardSet::DragonsMaze,
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature with power 3 or less",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::PowerAtLeast(4)),
                    ]),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            )
            .with_coverage(AbilityCoverageDef::partial(
                "Power-based target and attachment legality ignores continuous static power modifiers.",
            )),
            AbilityDef::triggered(
                "When this Aura enters, tap enchanted creature.",
                TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
                EffectDef::Tap {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature doesn't untap during its controller's untap step.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(
                        AppliedRuleDef::DoesNotUntapDuringUntapStep,
                    ),
                },
            ),
        ]),
);

// DGM 18 — Trait Doctoring
// Audit: blocked — Needs duration-scoped color-word text changes and cipher's encoded-card link, combat-damage trigger, and free-copy casting permission.

// DGM 19 — Uncovered Clues
// Audit: blocked — Needs a top-four selection constrained to up to two instant or sorcery cards, followed by ordering the unselected cards on the library bottom.

// DGM 21 — Bane Alley Blackguard
pub(in crate::card::sets) static BANE_ALLEY_BLACKGUARD: CardRecord = CardRecord::new(
    cards::BANE_ALLEY_BLACKGUARD,
    "Bane Alley Blackguard",
    CardArt::new("15fcad03-4567-4f96-976e-01a07d8ab050", "Mike Bierek"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Rogue"], 1, 3),
);

// DGM 22 — Blood Scrivener
// Audit: blocked — Needs a draw-event replacement that checks an empty hand and replaces one draw with two cards plus one life loss.

// DGM 23 — Crypt Incursion
// Audit: blocked — Needs the number of cards actually exiled by a graveyard sweep to feed one life-gain event after replacements are applied.

// DGM 24 — Fatal Fumes
pub(in crate::card::sets) static FATAL_FUMES: CardRecord = CardRecord::new(
    cards::FATAL_FUMES,
    "Fatal Fumes",
    CardArt::new("967aa636-a11d-4c5c-ba85-648734b295c2", "Kev Walker"),
    CardSet::DragonsMaze,
    CardRules::new_instant(mana_cost!("{3}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets -4/-2 until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-4),
                ValueDef::Constant(-2),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// DGM 25 — Hired Torturer
// Audit: blocked — Needs revealing a random card from the targeted opponent's hand after the life-loss effect.

// DGM 26 — Maze Abomination
pub(in crate::card::sets) static MAZE_ABOMINATION: CardRecord = CardRecord::new(
    cards::MAZE_ABOMINATION,
    "Maze Abomination",
    CardArt::new("dd84659f-4209-42a2-800a-61706470ce54", "Yeong-Hao Han"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{5}{B}"), &["Elemental"], 4, 5).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::static_ability(
            "Multicolored creatures you control have deathtouch.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    MULTICOLORED,
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::deathtouch()),
            },
        ),
    ]),
);

// DGM 27 — Pontiff of Blight
// Audit: blocked — Needs extort's per-spell optional hybrid-mana payment, opponent life loss, matched life gain, and external ability grant.

// DGM 28 — Rakdos Drake
pub(in crate::card::sets) static RAKDOS_DRAKE: CardRecord = CardRecord::new(
    cards::RAKDOS_DRAKE,
    "Rakdos Drake",
    CardArt::new("b9c1bfd7-b8b2-4db7-9ea7-a2d643a83589", "Karl Kopinski"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Drake"], 1, 2).with_abilities(&[
        abilities::flying(),
        abilities::unleash(),
        abilities::unleash_counter(),
    ]),
);

// DGM 29 — Sinister Possession
// Audit: blocked — Needs an Aura to observe both attack and block events from its attached creature and make that creature's controller lose life.

// DGM 30 — Ubul Sar Gatekeepers
pub(in crate::card::sets) static UBUL_SAR_GATEKEEPERS: CardRecord = CardRecord::new(
    cards::UBUL_SAR_GATEKEEPERS,
    "Ubul Sar Gatekeepers",
    CardArt::new("f5b2e327-adfd-459b-8d18-faa39d88b5de", "Volkan Baǵa"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Zombie", "Soldier"], 2, 4).with_ability(
        AbilityDef::triggered_if_with_targets(
            "When this creature enters, if you control two or more Gates, target creature an opponent controls gets -2/-2 until end of turn.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
            &TWO_GATES_CONDITION,
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(-2), ValueDef::Constant(-2)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// DGM 31 — Awe for the Guilds
// Audit: blocked — Needs a turn-long restriction preventing every monocolored creature from blocking.

// DGM 32 — Clear a Path
pub(in crate::card::sets) static CLEAR_A_PATH: CardRecord = CardRecord::new(
    cards::CLEAR_A_PATH,
    "Clear a Path",
    CardArt::new("8a8f904b-a9a3-4bae-9284-4e9cbe7592ee", "Karl Kopinski"),
    CardSet::DragonsMaze,
    CardRules::new_sorcery(mana_cost!("{R}")).with_ability(AbilityDef::destroy_target(
        "Destroy target creature with defender.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Defender),
        ])),
        true,
    )),
);

// DGM 33 — Maze Rusher
pub(in crate::card::sets) static MAZE_RUSHER: CardRecord = CardRecord::new(
    cards::MAZE_RUSHER,
    "Maze Rusher",
    CardArt::new("864d2eb8-e27f-4f84-9725-d2ae6446e217", "Yeong-Hao Han"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{5}{R}"), &["Elemental"], 6, 3).with_abilities(&[
        abilities::haste(),
        AbilityDef::static_ability(
            "Multicolored creatures you control have haste.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    MULTICOLORED,
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
            },
        ),
    ]),
);

// DGM 34 — Possibility Storm
// Audit: blocked — Needs spell-type-aware library reveal-until, free casting of the found card, and random ordering of the linked exiled cards.

// DGM 35 — Punish the Enemy
pub(in crate::card::sets) static PUNISH_THE_ENEMY: CardRecord = CardRecord::new(
    cards::PUNISH_THE_ENEMY,
    "Punish the Enemy",
    CardArt::new("4179a72b-8482-46ec-9815-f5d6d94b5aa5", "Slawomir Maniak"),
    CardSet::DragonsMaze,
    CardRules::new_instant(mana_cost!("{4}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Punish the Enemy deals 3 damage to target player or planeswalker and 3 damage to target creature.",
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::PlayerOrPlaneswalker(
                    PlayerRelation::Any,
                )),
                AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
            ],
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(3),
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex(1)),
                    amount: ValueDef::Constant(3),
                },
            ]),
        ),
    ),
);

// DGM 36 — Pyrewild Shaman
// Audit: partial — Bloodrush is implemented, but combat damage from multiple creatures is captured as separate events instead of one “one or more” event for the graveyard return trigger.
pub(in crate::card::sets) static PYREWILD_SHAMAN: CardRecord = CardRecord::new(
    cards::PYREWILD_SHAMAN,
    "Pyrewild Shaman",
    CardArt::new("8c6f6e45-f613-420d-83d2-d93c643265ee", "Lucas Graciano"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Shaman"], 3, 1).with_abilities(&[
        abilities::bloodrush(
            mana_cost!("{1}{R}"),
            "Bloodrush — {1}{R}, Discard this card: Target attacking creature gets +3/+1 until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Attacking,
            ]))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(3), ValueDef::Constant(1)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::not_implemented(
            "Whenever one or more creatures you control deal combat damage to a player, if this card is in your graveyard, you may pay {3}. If you do, return this card to your hand.",
            "Combat damage from multiple creatures is captured as separate events instead of one ‘one or more’ event, and the combined graveyard payment continuation is unavailable.",
        ),
    ]),
);

// DGM 37 — Riot Piker
pub(in crate::card::sets) static RIOT_PIKER: CardRecord = CardRecord::new(
    cards::RIOT_PIKER,
    "Riot Piker",
    CardArt::new(
        "4daaccd2-733c-4b3b-aa3f-cc825bcc3e53",
        "Christopher Moeller",
    ),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Berserker"], 2, 1).with_abilities(
        &[
            abilities::first_strike(),
            abilities::attacks_each_combat_if_able("This creature attacks each combat if able."),
        ],
    ),
);

// DGM 38 — Rubblebelt Maaka
pub(in crate::card::sets) static RUBBLEBELT_MAAKA: CardRecord = CardRecord::new(
    cards::RUBBLEBELT_MAAKA,
    "Rubblebelt Maaka",
    CardArt::new("bc802d62-6559-45b9-ad11-de5887aece2b", "Eric Velhagen"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Cat"], 3, 3).with_ability(
        abilities::bloodrush(
            mana_cost!("{R}"),
            "Bloodrush — {R}, Discard this card: Target attacking creature gets +3/+3 until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Attacking,
            ]))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(3), ValueDef::Constant(3)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// DGM 39 — Smelt-Ward Gatekeepers
pub(in crate::card::sets) static SMELT_WARD_GATEKEEPERS: CardRecord = CardRecord::new(
    cards::SMELT_WARD_GATEKEEPERS,
    "Smelt-Ward Gatekeepers",
    CardArt::new("8237b11f-36d2-4624-a0ef-520663385891", "Daarken"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Warrior"], 2, 4).with_ability(
        AbilityDef::triggered_if_with_targets(
            "When this creature enters, if you control two or more Gates, gain control of target creature an opponent controls until end of turn. Untap that creature. It gains haste until end of turn.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
            &TWO_GATES_CONDITION,
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::GainControl {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    duration: ControlDurationDef::UntilEndOfTurn,
                },
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
    ),
);

static WEAPON_SURGE_PUMP: AppliedEffectDef = AppliedEffectDef::Composite(&[
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(0)),
    AppliedEffectDef::add_ability(&abilities::first_strike()),
]);

// DGM 40 — Weapon Surge
pub(in crate::card::sets) static WEAPON_SURGE: CardRecord = CardRecord::new(
    cards::WEAPON_SURGE,
    "Weapon Surge",
    CardArt::new("f28df164-8bff-4428-b7dd-2974c288f1d3", "Jason Felix"),
    CardSet::DragonsMaze,
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature you control gets +1/+0 and gains first strike until end of turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: WEAPON_SURGE_PUMP,
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::overload(
            mana_cost!("{1}{R}"),
            "Each creature you control gets +1/+0 and gains first strike until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: WEAPON_SURGE_PUMP,
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DGM 41 — Battering Krasis
// Audit: blocked — Needs evolve's intervening power-or-toughness comparison against the entering creature and an evolve-event marker.

// DGM 42 — Kraul Warrior
pub(in crate::card::sets) static KRAUL_WARRIOR: CardRecord = CardRecord::new(
    cards::KRAUL_WARRIOR,
    "Kraul Warrior",
    CardArt::new("f71da8cc-8773-4dcb-aca8-50a000142218", "David Rapoza"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Insect", "Warrior"], 2, 2).with_ability(
        AbilityDef::activated(
            "{5}{G}: This creature gets +3/+3 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{5}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// DGM 43 — Maze Behemoth
pub(in crate::card::sets) static MAZE_BEHEMOTH: CardRecord = CardRecord::new(
    cards::MAZE_BEHEMOTH,
    "Maze Behemoth",
    CardArt::new("0a7c9678-dea7-4219-bac0-9e1cef531f54", "Yeong-Hao Han"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Elemental"], 5, 4).with_abilities(&[
        abilities::trample(),
        AbilityDef::static_ability(
            "Multicolored creatures you control have trample.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    MULTICOLORED,
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::trample()),
            },
        ),
    ]),
);

// DGM 44 — Mending Touch
pub(in crate::card::sets) static MENDING_TOUCH: CardRecord = CardRecord::new(
    cards::MENDING_TOUCH,
    "Mending Touch",
    CardArt::new("c042c7ee-0e74-4ca5-bbb9-2898b0576f0a", "Karla Ortiz"),
    CardSet::DragonsMaze,
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell_with_targets(
        "Regenerate target creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Regenerate {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )),
);

// DGM 45 — Mutant's Prey
// Audit: blocked — Needs a target predicate for a +1/+1 counter and the simultaneous fight damage procedure.

// DGM 46 — Phytoburst
pub(in crate::card::sets) static PHYTOBURST: CardRecord = CardRecord::new(
    cards::PHYTOBURST,
    "Phytoburst",
    CardArt::new("7507afc4-f504-4eb2-a86d-f99bc2860838", "Izzy"),
    CardSet::DragonsMaze,
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +5/+5 until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(5),
                ValueDef::Constant(5),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// DGM 47 — Renegade Krasis
// Audit: blocked — Needs evolve's characteristic comparison and an evolve event that can drive the counter sweep.

// DGM 48 — Saruli Gatekeepers
pub(in crate::card::sets) static SARULI_GATEKEEPERS: CardRecord = CardRecord::new(
    cards::SARULI_GATEKEEPERS,
    "Saruli Gatekeepers",
    CardArt::new("471a5b1d-e2e5-4d90-b72a-ffae81ad6602", "Chris Rahn"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Elf", "Warrior"], 2, 4).with_ability(
        AbilityDef::triggered_if(
            "When this creature enters, if you control two or more Gates, you gain 7 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TWO_GATES_CONDITION,
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(7),
            },
        ),
    ),
);

// DGM 49 — Skylasher
pub(in crate::card::sets) static SKYLASHER: CardRecord = CardRecord::new(
    cards::SKYLASHER,
    "Skylasher",
    CardArt::new("4f4c2069-deb1-4e56-8069-170c4f495944", "Dan Murayama Scott"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Insect"], 2, 2).with_abilities(&[
        abilities::flash(),
        abilities::cannot_be_countered(),
        abilities::reach(),
        abilities::protection_from(ManaColor::Blue),
    ]),
);

// DGM 50 — Thrashing Mossdog
// Audit: blocked — Needs scavenge's graveyard activation timing, source-exile cost, and source-power counter amount.

// DGM 51 — Advent of the Wurm
pub(in crate::card::sets) static ADVENT_OF_THE_WURM: CardRecord = CardRecord::new(
    cards::ADVENT_OF_THE_WURM,
    "Advent of the Wurm",
    CardArt::new("f40284e6-01a1-4372-a92c-940e5732607e", "Lucas Graciano"),
    CardSet::DragonsMaze,
    CardRules::new_instant(mana_cost!("{1}{G}{G}{W}")).with_ability(AbilityDef::spell(
        "Create a 5/5 green Wurm creature token with trample.",
        EffectDef::CreateToken {
            token: cards::WURM_TOKEN_5_5_GREEN,
            count: ValueDef::Constant(1),
            tapped: false,
        },
    )),
);

// DGM 52 — Armored Wolf-Rider
pub(in crate::card::sets) static ARMORED_WOLF_RIDER: CardRecord = CardRecord::new(
    cards::ARMORED_WOLF_RIDER,
    "Armored Wolf-Rider",
    CardArt::new("e43d959f-6055-4578-a69a-0ec93e993e21", "Matt Stewart"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{3}{G}{W}"), &["Elf", "Knight"], 4, 6),
);

// DGM 53 — Ascended Lawmage
pub(in crate::card::sets) static ASCENDED_LAWMAGE: CardRecord = CardRecord::new(
    cards::ASCENDED_LAWMAGE,
    "Ascended Lawmage",
    CardArt::new("b1f00799-80ce-431e-97bb-8bb4e0e8ba49", "Ryan Yee"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{2}{W}{U}"), &["Vedalken", "Wizard"], 3, 2)
        .with_abilities(&[abilities::flying(), abilities::hexproof()]),
);

static BEETLE_WINGS: AbilityDef = abilities::flying();

// DGM 54 — Beetleform Mage
pub(in crate::card::sets) static BEETLEFORM_MAGE: CardRecord = CardRecord::new(
    cards::BEETLEFORM_MAGE,
    "Beetleform Mage",
    CardArt::new("1e2f7d7f-4097-419b-8de0-b7bf28fc3a4b", "Marco Nelor"),
    CardSet::DragonsMaze,
    CardRules::new_creature(
        mana_cost!("{1}{G}{U}"),
        &["Human", "Insect", "Wizard"],
        2,
        2,
    )
    .with_ability(
        AbilityDef::activated(
            "{G}{U}: This creature gets +2/+2 and gains flying until end of turn. \
                 Activate only once each turn.",
            &[AbilityCostDef::Mana(mana_cost!("{G}{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                    AppliedEffectDef::add_ability(&BEETLE_WINGS),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .once_each_turn(),
    ),
);

// DGM 55 — Blast of Genius
// Audit: blocked — Needs a discard choice whose chosen card's mana value feeds the later damage effect.

// DGM 56 — Blaze Commando
// Audit: blocked — Needs a damage event that groups all damage dealt by one instant or sorcery before creating the two tokens.

// DGM 57 — Blood Baron of Vizkopa
pub(in crate::card::sets) static BLOOD_BARON_OF_VIZKOPA: CardRecord = CardRecord::new(
    cards::BLOOD_BARON_OF_VIZKOPA,
    "Blood Baron of Vizkopa",
    CardArt::new("e4edad09-bf7b-40e9-ac2a-100da8a43274", "Anthony Palumbo"),
    CardSet::DragonsMaze,
    CardRules::new_creature(
        mana_cost!("{3}{W}{B}"),
        &["Vampire"],
        4,
        4,
    )
    .with_abilities(&[
        abilities::lifelink(),
        abilities::protection_from(ManaColor::White),
        abilities::protection_from(ManaColor::Black),
        AbilityDef::custom_full(
            "As long as you have 30 or more life and an opponent has 10 or less life, this creature gets +6/+6 and has flying.",
            CardBehavior::BloodBaronOfVizkopa,
            "The conditional power, toughness, and flying effect is implemented by the card-local static-effect hook.",
        ),
    ]),
);

// DGM 58 — Boros Battleshaper
// Audit: blocked — Needs beginning-of-combat targets that impose positive and negative attack-or-block requirements for that combat.

// DGM 59 — Bred for the Hunt
// Audit: blocked — Needs a combat-damage source predicate that tests for a +1/+1 counter on the dealing creature.

// DGM 60 — Bronzebeak Moa
pub(in crate::card::sets) static BRONZEBEAK_MOA: CardRecord = CardRecord::new(
    cards::BRONZEBEAK_MOA,
    "Bronzebeak Moa",
    CardArt::new("291c0ebc-d489-42c7-8d8a-9216c333412f", "James Ryman"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{2}{G}{W}"), &["Bird"], 2, 2)
        .with_ability(AbilityDef::triggered(
        "Whenever another creature you control enters, this creature gets +3/+3 until end of turn.",
        TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            ]), None, Some(ZoneKind::Battlefield)),
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(3), ValueDef::Constant(3)),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// DGM 61 — Carnage Gladiator
// Audit: blocked — Needs a blocking event that identifies each blocker and regeneration shields for the activated ability.

// DGM 62 — Council of the Absolute
// Audit: blocked — Needs a stored noncreature, nonland card-name choice that both prohibits opponents' matching spells and reduces matching spells you cast.

// DGM 63 — Deadbridge Chant
// Audit: blocked — Needs a random graveyard-card choice followed by a card-type-dependent destination.

// DGM 64 — Debt to the Deathless
// Audit: blocked — Needs arithmetic values for twice X and one life-gain event equal to the life actually lost by all opponents.

// DGM 65 — Deputy of Acquittals
pub(in crate::card::sets) static DEPUTY_OF_ACQUITTALS: CardRecord = CardRecord::new(
    cards::DEPUTY_OF_ACQUITTALS,
    "Deputy of Acquittals",
    CardArt::new("4b555888-21b1-4c45-966d-d98f32460d4e", "James Ryman"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{W}{U}"), &["Human", "Wizard"], 2, 2).with_abilities(&[
        abilities::flash(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, you may return another target creature you control to its owner's hand.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    controller: None,
                },
            },
        ),
    ]),
);

// DGM 66 — Dragonshift
// Audit: blocked — Needs its targeted and overload programs migrated to one composite type, color, ability, power/toughness, and flying effect.

static DROWN_IN_FILTH_LANDS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Land),
    &[ZoneKind::Graveyard],
    PlayerRelation::You,
);
static DROWN_IN_FILTH_PENALTY: ValueDef =
    ValueDef::Negate(&ValueDef::CountMatchingObjects(&DROWN_IN_FILTH_LANDS));

// DGM 67 — Drown in Filth
pub(in crate::card::sets) static DROWN_IN_FILTH: CardRecord = CardRecord::new(
    cards::DROWN_IN_FILTH,
    "Drown in Filth",
    CardArt::new("22feacda-01e0-4f0d-a3c7-a22e3d40bf4e", "Seb McKinnon"),
    CardSet::DragonsMaze,
    CardRules::new_sorcery(mana_cost!("{B}{G}")).with_ability(
        AbilityDef::spell_with_targets(
            "Choose target creature. Mill four cards, then that creature gets -1/-1 until end of turn for each land card in your graveyard.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::Mill {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(4),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(DROWN_IN_FILTH_PENALTY, DROWN_IN_FILTH_PENALTY),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ),
);

// DGM 68 — Emmara Tandris
// Audit: blocked — Needs a damage-prevention replacement for every creature token you control.

static EXAVA_OTHER_COUNTERED_CREATURES: [ObjectPredicateDef; 3] = [
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::HasCounter(CounterKind::PlusOnePlusOne),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
];

static EXAVA_HASTE: AbilityDef = abilities::haste();

// DGM 69 — Exava, Rakdos Blood Witch
pub(in crate::card::sets) static EXAVA_RAKDOS_BLOOD_WITCH: CardRecord = CardRecord::new(
    cards::EXAVA_RAKDOS_BLOOD_WITCH,
    "Exava, Rakdos Blood Witch",
    CardArt::new("6cb72a64-89e7-4b0e-a3d3-1309829071d2", "Aleksi Briclot"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{2}{B}{R}"), &["Human", "Cleric"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::first_strike(),
            abilities::haste(),
            abilities::unleash(),
            abilities::unleash_counter(),
            AbilityDef::static_ability(
                "Each other creature you control with a +1/+1 counter on it has haste.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&EXAVA_OTHER_COUNTERED_CREATURES),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&EXAVA_HASTE),
                },
            ),
        ]),
);

// DGM 70 — Feral Animist
pub(in crate::card::sets) static FERAL_ANIMIST: CardRecord = CardRecord::new(
    cards::FERAL_ANIMIST,
    "Feral Animist",
    CardArt::new("108a9ef2-c74a-450b-8148-4fdf9f09843f", "Dave Kendall"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{1}{R}{G}"), &["Goblin", "Shaman"], 2, 1).with_ability(
        AbilityDef::activated(
            "{3}: This creature gets +X/+0 until end of turn, where X is its power.",
            &[AbilityCostDef::Mana(mana_cost!("{3}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::SourcePower,
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// DGM 71 — Fluxcharger
// Audit: blocked — Needs a temporary power-and-toughness exchange effect.

// DGM 72 — Gaze of Granite
pub(in crate::card::sets) static GAZE_OF_GRANITE: CardRecord = CardRecord::new(
    cards::GAZE_OF_GRANITE,
    "Gaze of Granite",
    CardArt::new("96c9ac10-d114-4aa5-87ac-f1069cde8e40", "Nils Hamm"),
    CardSet::DragonsMaze,
    CardRules::new_sorcery(mana_cost!("{X}{B}{B}{G}")).with_ability(AbilityDef::spell(
        "Destroy each nonland permanent with mana value X or less.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    ObjectPredicateDef::ManaValueAtMostValue(ValueDef::ChosenX),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            can_regenerate: true,
        },
    )),
);

// DGM 73 — Gleam of Battle
pub(in crate::card::sets) static GLEAM_OF_BATTLE: CardRecord = CardRecord::new(
    cards::GLEAM_OF_BATTLE,
    "Gleam of Battle",
    CardArt::new("e5f0feef-1a71-4c8c-9fd1-f5cbe718a988", "Raymond Swanland"),
    CardSet::DragonsMaze,
    CardRules::new_enchantment(mana_cost!("{4}{R}{W}")).with_ability(AbilityDef::triggered(
        "Whenever a creature you control attacks, put a +1/+1 counter on it.",
        TriggerEventDef::attacks(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
        ])),
        EffectDef::AddCounters {
            object: EffectRecipientDef::TriggeringObject,
            kind: CounterKind::PlusOnePlusOne,
            amount: ValueDef::Constant(1),
        },
    )),
);

// DGM 74 — Goblin Test Pilot
// Audit: blocked — Needs a uniformly random legal target choice when the activated ability resolves.

// DGM 75 — Gruul War Chant
// Audit: blocked — Needs menace as an executable minimum-blocker constraint and a static grant to attacking creatures.

// DGM 76 — Haunter of Nightveil
pub(in crate::card::sets) static HAUNTER_OF_NIGHTVEIL: CardRecord = CardRecord::new(
    cards::HAUNTER_OF_NIGHTVEIL,
    "Haunter of Nightveil",
    CardArt::new("438683f5-adfa-42ae-a6fb-c4649a8a30ab", "Igor Kieryluk"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{3}{U}{B}"), &["Spirit"], 3, 4).with_ability(
        AbilityDef::static_ability(
            "Creatures your opponents control get -1/-0.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(0),
                ),
            },
        ),
    ),
);

// DGM 77 — Jelenn Sphinx
pub(in crate::card::sets) static JELENN_SPHINX: CardRecord = CardRecord::new(
    cards::JELENN_SPHINX,
    "Jelenn Sphinx",
    CardArt::new("533c89eb-d7c6-4945-9689-2f2c0e428b84", "Wesley Burt"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{3}{W}{U}"), &["Sphinx"], 1, 5).with_abilities(&[
        abilities::flying(),
        abilities::vigilance(),
        AbilityDef::triggered(
            "Whenever this creature attacks, other attacking creatures get +1/+1 until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Attacking,
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]), &[ZoneKind::Battlefield], PlayerRelation::You),
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DGM 78 — Korozda Gorgon
// Audit: blocked — Needs removing a +1/+1 counter from a chosen creature, rather than from the ability source, as an activation cost.

// DGM 79 — Krasis Incubation
// Audit: blocked — Needs attached-creature attack, block, and activated-ability prohibitions plus returning the Aura as a cost while retaining its former attachment through last-known information.

// DGM 80 — Lavinia of the Tenth
// Audit: blocked — Needs detain's persistent restrictions and a nonland permanent sweep filtered by mana value.

// DGM 81 — Legion's Initiative
// Audit: blocked — Needs a non-choice binding for exactly the creatures exiled together so the installed beginning-of-combat trigger can return and grant haste only to that group.

// DGM 82 — Master of Cruelties
// Audit: blocked — Needs an attack-alone restriction, an unblocked-attacker trigger that sets a player's life total, and suppression of this creature's combat damage.

// DGM 83 — Maw of the Obzedat
pub(in crate::card::sets) static MAW_OF_THE_OBZEDAT: CardRecord = CardRecord::new(
    cards::MAW_OF_THE_OBZEDAT,
    "Maw of the Obzedat",
    CardArt::new("cd1131c6-04da-4c4d-ab61-874ac5be7087", "Randy Gallegos"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{3}{W}{B}"), &["Thrull"], 3, 3).with_ability(
        AbilityDef::activated(
            "Sacrifice a creature: Creatures you control get +1/+1 until end of turn.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
            }],
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// DGM 84 — Melek, Izzet Paragon
// Audit: blocked — Needs a continuously revealed library top, cast permission from that zone, and copying spells cast from the library with target reselection.

// DGM 85 — Mirko Vosk, Mind Drinker
// Audit: blocked — Needs reveal-until-four-matching-cards library traversal and moving the entire revealed group to the graveyard.

// DGM 86 — Morgue Burst
// Audit: partial — Returning the graveyard card is implemented, but TargetPower cannot read a card target's power after it moves to hand.
pub(in crate::card::sets) static MORGUE_BURST: CardRecord = CardRecord::new(
    cards::MORGUE_BURST,
    "Morgue Burst",
    CardArt::new("7b3c2909-87ab-4027-9b56-58a2abae3fa3", "Raymond Swanland"),
    CardSet::DragonsMaze,
    CardRules::new_sorcery(mana_cost!("{4}{B}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Return target creature card from your graveyard to your hand. Morgue Burst deals damage to any target equal to the power of the card returned this way.",
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                }),
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget),
            ],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                controller: None,
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "TargetPower reads permanents but not a card target that moved from the graveyard to hand.",
        )),
    ),
);

// DGM 87 — Nivix Cyclops
// Audit: blocked — Needs a turn-long permission to attack as though defender were absent without actually removing the defender ability.

// DGM 88 — Notion Thief
// Audit: blocked — Needs a draw-event replacement that recognizes the first draw of each opponent's draw step and redirects every other draw.

// DGM 89 — Obzedat's Aid
pub(in crate::card::sets) static OBZEDATS_AID: CardRecord = CardRecord::new(
    cards::OBZEDATS_AID,
    "Obzedat's Aid",
    CardArt::new("b846ba99-81ba-424a-98eb-f9f69c40f984", "Dan Murayama Scott"),
    CardSet::DragonsMaze,
    CardRules::new_sorcery(mana_cost!("{3}{W}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Return target permanent card from your graveyard to the battlefield.",
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
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            controller: None,
        },
    )),
);

// DGM 90 — Pilfered Plans
pub(in crate::card::sets) static PILFERED_PLANS: CardRecord = CardRecord::new(
    cards::PILFERED_PLANS,
    "Pilfered Plans",
    CardArt::new("3475fcc6-ee53-48da-89d2-80685a584e6a", "Michael C. Hayes"),
    CardSet::DragonsMaze,
    CardRules::new_sorcery(mana_cost!("{1}{U}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player mills two cards. Draw two cards.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )),
);

// DGM 91 — Plasm Capture
// Audit: blocked — Needs a delayed first-main-phase mana effect that lets its controller distribute the countered spell's mana value among any combination of colors.

// DGM 92 — Progenitor Mimic
// Audit: blocked — Needs an as-enters copy effect that adds a triggered ability to the copied values and later creates token copies of itself.

// DGM 93 — Putrefy
pub(in crate::card::sets) static PUTREFY: CardRecord = CardRecord::new(
    cards::PUTREFY,
    "Putrefy",
    CardArt::new("0d43a0b6-2a5c-4959-96ee-6e570949dfed", "Igor Kieryluk"),
    CardSet::DragonsMaze,
    CardRules::new_instant(mana_cost!("{1}{B}{G}")).with_ability(AbilityDef::destroy_target(
        "Destroy target artifact or creature. It can't be regenerated.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Creature),
        ])),
        false,
    )),
);

// DGM 94 — Ral Zarek
// Audit: partial — The damage ability is complete, but the tap/untap targets are not constrained to different permanents and coin flips plus extra-turn scheduling are unavailable.
pub(in crate::card::sets) static RAL_ZAREK: CardRecord = CardRecord::new(
    cards::RAL_ZAREK,
    "Ral Zarek",
    CardArt::new("fcdbb062-0b0b-4b4c-b4db-dd149f744baa", "Eric Deschamps"),
    CardSet::DragonsMaze,
    CardRules::new_planeswalker(mana_cost!("{2}{U}{R}"), &["Ral"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "+1: Tap target permanent, then untap another target permanent.",
                &[AbilityCostDef::Loyalty(1)],
                &[
                    AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Any),
                    AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Any),
                ],
                EffectDef::Sequence(&[
                    EffectDef::Tap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::Untap {
                        object: EffectRecipientDef::Target(TargetIndex(1)),
                    },
                ]),
            )
            .with_coverage(AbilityCoverageDef::partial(
                "The target system cannot require the two target slots to name different permanents.",
            )),
            AbilityDef::activated_with_targets(
                "−2: Ral Zarek deals 3 damage to any target.",
                &[AbilityCostDef::Loyalty(-2)],
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(3),
                },
            ),
            AbilityDef::not_implemented(
                "−7: Flip five coins. Take an extra turn after this one for each coin that comes up heads.",
                "Coin flips and extra-turn scheduling are not available declaratively.",
            ),
        ]),
);

// DGM 95 — Reap Intellect
// Audit: blocked — Needs an X-bounded private-hand choice, same-name searches across three zones, exile of every chosen group, and the final shuffle.

// DGM 96 — Render Silent
// Audit: blocked — Needs a turn-long prohibition on the countered spell's controller casting any spell.

// DGM 97 — Restore the Peace
// Audit: blocked — Needs per-turn damage history on creatures and a simultaneous return sweep over every creature that dealt damage.

// DGM 98 — Rot Farm Skeleton
// Audit: blocked — Needs an executable can't-block restriction and milling cards as an activation cost from the graveyard.

// DGM 99 — Ruric Thar, the Unbowed
pub(in crate::card::sets) static RURIC_THAR_THE_UNBOWED: CardRecord = CardRecord::new(
    cards::RURIC_THAR_THE_UNBOWED,
    "Ruric Thar, the Unbowed",
    CardArt::new("84dd3586-7c3b-4f9c-a1eb-7745b75339b0", "Tyler Jacobson"),
    CardSet::DragonsMaze,
    CardRules::new_creature(
        mana_cost!("{4}{R}{G}"),
        &["Ogre", "Warrior"],
        6,
        6,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::vigilance(),
        abilities::reach(),
        abilities::attacks_each_combat_if_able("Ruric Thar attacks each combat if able."),
        AbilityDef::triggered(
            "Whenever a player casts a noncreature spell, Ruric Thar deals 6 damage to that player.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::NoncreatureSpell),
            EffectDef::DealDamage {
                // Whoever cast it, which is what the event names; this hits
                // its own controller too.
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(6),
            },
        ),
    ]),
);

// DGM 100 — Savageborn Hydra
// Audit: blocked — Needs an X-sized battlefield-entry counter replacement and a hybrid-mana activation restricted to sorcery timing.

// DGM 101 — Scab-Clan Giant
// Audit: blocked — Needs a uniformly random legal opponent-creature choice followed by the simultaneous fight damage procedure.

static SHOWSTOPPER_DIES_ABILITY: AbilityDef = AbilityDef::triggered_with_targets(
    "When this creature dies, it deals 2 damage to target creature an opponent controls.",
    TriggerEventDef::zone_changed(
        ObjectPredicateDef::Source,
        Some(ZoneKind::Battlefield),
        Some(ZoneKind::Graveyard),
    ),
    &[AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            zones: &[ZoneKind::Battlefield],
            controller: Some(PlayerRelation::Opponent),
            owner: None,
        },
    )],
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(2),
    },
);

// DGM 102 — Showstopper
pub(in crate::card::sets) static SHOWSTOPPER: CardRecord = CardRecord::new(
    cards::SHOWSTOPPER,
    "Showstopper",
    CardArt::new("2fd1f68b-3f16-484e-95c9-5cfa8da218c9", "Steve Prescott"),
    CardSet::DragonsMaze,
    CardRules::new_instant(mana_cost!("{1}{B}{R}")).with_ability(AbilityDef::spell(
        "Until end of turn, creatures you control gain ‘When this creature dies, it deals 2 damage to target creature an opponent controls.’",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You),
            effect: AppliedEffectDef::add_ability(&SHOWSTOPPER_DIES_ABILITY),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

static SIN_COLLECTOR_EXILE: EffectDef = EffectDef::MoveToZone {
    object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    zone: ZoneKind::Exile,
    placement: ZonePlacement::Top,
    controller: None,
};

static SIN_COLLECTOR_EFFECTS: [EffectDef; 2] = [
    EffectDef::LookAtHand {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::Choose(ChooseDef {
        binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
        chooser: PlayerRefDef::EffectController,
        candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::HasType(CardType::Sorcery),
            ]),
            &[ZoneKind::Hand],
            PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
        )),
        exclude: None,
        minimum: 1,
        maximum: 1,
        visibility: ChoiceVisibilityDef::Public,
        then: &SIN_COLLECTOR_EXILE,
    }),
];

// DGM 103 — Sin Collector
pub(in crate::card::sets) static SIN_COLLECTOR: CardRecord = CardRecord::new(
    cards::SIN_COLLECTOR,
    "Sin Collector",
    CardArt::new("305a3feb-df49-486c-a3b4-ff2721d60019", "Mike Bierek"),
    CardSet::DragonsMaze,
    CardRules::new_creature(
        mana_cost!("{1}{W}{B}"),
        &["Human", "Cleric"],
        2,
        1,
    )
    .with_abilities(&[AbilityDef::triggered_with_targets("When this creature enters, target opponent reveals their hand. You choose an instant or sorcery card from it and exile that card.", TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)), &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
        )], EffectDef::Sequence(&SIN_COLLECTOR_EFFECTS)),
    ]),
);

// DGM 104 — Sire of Insanity
pub(in crate::card::sets) static SIRE_OF_INSANITY: CardRecord = CardRecord::new(
    cards::SIRE_OF_INSANITY,
    "Sire of Insanity",
    CardArt::new("3665cfb7-51b6-4083-8eae-fbd3fa6c3554", "Peter Mohrbacher"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{4}{B}{R}"), &["Demon"], 6, 4).with_ability(
        AbilityDef::triggered(
            "At the beginning of each end step, each player discards their hand.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            EffectDef::Discard {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ValueDef::Constant(i32::MAX),
                selection: DiscardSelectionDef::RecipientChooses,
            },
        ),
    ),
);

// DGM 105 — Species Gorger
// Audit: partial — Needs a mandatory non-target creature choice followed by returning that chosen permanent to its owner's hand.
pub(in crate::card::sets) static SPECIES_GORGER: CardRecord = CardRecord::new(
    cards::SPECIES_GORGER,
    "Species Gorger",
    CardArt::new("e0087a98-55cf-4c8b-a180-fb0d9c336eb2", "Min Yum"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{3}{G}{U}"), &["Frog", "Beast"], 6, 6).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, return a creature you control to its owner's hand.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::Special("Choose a creature you control and return it to its owner's hand"),
        )
        .with_coverage(AbilityCoverageDef::metadata_only(
            "A mandatory non-target object choice followed by a zone move is not available.",
        )),
    ),
);

// DGM 106 — Spike Jester
pub(in crate::card::sets) static SPIKE_JESTER: CardRecord = CardRecord::new(
    cards::SPIKE_JESTER,
    "Spike Jester",
    CardArt::new("cec50499-70d4-4dc1-9cae-abbecfc8e87d", "Ryan Barger"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{B}{R}"), &["Goblin", "Warrior"], 3, 1)
        .with_ability(abilities::haste()),
);

// DGM 107 — Tajic, Blade of the Legion
// Audit: partial — Battalion's attack-count restriction is rechecked on resolution as though it were an intervening-if condition.
pub(in crate::card::sets) static TAJIC_BLADE_OF_THE_LEGION: CardRecord = CardRecord::new(
    cards::TAJIC_BLADE_OF_THE_LEGION,
    "Tajic, Blade of the Legion",
    CardArt::new("be5717c1-338e-446c-aa7e-93e79e4abb72", "James Ryman"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{2}{R}{W}"), &["Human", "Soldier"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::indestructible(),
            AbilityDef::triggered_if(
                "Battalion — Whenever this creature and at least two other creatures attack, this creature gets +5/+5 until end of turn.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                &BATTALION_CONDITION,
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(5), ValueDef::Constant(5)),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            )
            .with_coverage(AbilityCoverageDef::partial(
                "The attack-count condition is incorrectly rechecked when the trigger resolves.",
            )),
        ]),
);

// DGM 108 — Teysa, Envoy of Ghosts
// Audit: blocked — Needs protection from creatures and a combat-damage trigger that destroys the specific dealing creature before creating a token.

// DGM 109 — Tithe Drinker
// Audit: blocked — Needs extort's optional hybrid-mana payment and life-loss-to-life-gain linkage.

// DGM 110 — Trostani's Summoner
pub(in crate::card::sets) static TROSTANIS_SUMMONER: CardRecord = CardRecord::new(
    cards::TROSTANIS_SUMMONER,
    "Trostani's Summoner",
    CardArt::new("1921fa4e-2256-4ef1-b2fe-874f9fbbcdf3", "Howard Lyon"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{5}{G}{W}"), &["Elf", "Shaman"], 1, 1).with_ability(
        AbilityDef::triggered(
            "When this creature enters, create a 2/2 white Knight creature token with vigilance, a 3/3 green Centaur creature token, and a 4/4 green Rhino creature token with trample.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
            EffectDef::Sequence(&[
                EffectDef::CreateToken {
                    token: cards::KNIGHT_TOKEN_2_2_WHITE,
                    count: ValueDef::Constant(1),
                    tapped: false,
                },
                EffectDef::CreateToken {
                    token: cards::CENTAUR_TOKEN_3_3_GREEN,
                    count: ValueDef::Constant(1),
                    tapped: false,
                },
                EffectDef::CreateToken {
                    token: cards::RHINO_TOKEN_4_4_GREEN,
                    count: ValueDef::Constant(1),
                    tapped: false,
                },
            ]),
        ),
    ),
);

// DGM 111 — Unflinching Courage
pub(in crate::card::sets) static UNFLINCHING_COURAGE: CardRecord = CardRecord::new(
    cards::UNFLINCHING_COURAGE,
    "Unflinching Courage",
    CardArt::new("35952c24-d728-4ec6-b0d1-b8183a18554a", "Mike Bierek"),
    CardSet::DragonsMaze,
    CardRules::new_enchantment(mana_cost!("{1}{G}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
        AbilityDef::spell_with_targets("Enchant creature", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )], EffectDef::Attach {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            }),
        AbilityDef::static_ability(
            "Enchanted creature gets +2/+2 and has trample and lifelink. (Damage dealt by the creature also causes its controller to gain that much life.)",
            EffectDef::Sequence(&[
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
                },
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                },
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                },
            ]),
        ),
    ]),
);

// DGM 112 — Varolz, the Scar-Striped
// Audit: blocked — Needs granting scavenge to graveyard cards with each card's own mana cost and power, plus regeneration shields.

// DGM 113 — Viashino Firstblade
pub(in crate::card::sets) static VIASHINO_FIRSTBLADE: CardRecord = CardRecord::new(
    cards::VIASHINO_FIRSTBLADE,
    "Viashino Firstblade",
    CardArt::new("1cb0c21c-bdf1-478a-9ad8-6c6bda6ffb0f", "Matt Stewart"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{1}{R}{W}"), &["Lizard", "Soldier"], 2, 2).with_abilities(
        &[
            abilities::haste(),
            AbilityDef::triggered(
                "When this creature enters, it gets +2/+2 until end of turn.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    ),
);

static VOICE_OF_RESURGENCE_DURING_YOUR_TURN: TriggerConditionDef =
    TriggerConditionDef::ActivePlayer(PlayerRelation::You);

static VOICE_OF_RESURGENCE_TOKEN: EffectDef = EffectDef::CreateToken {
    token: cards::ELEMENTAL_TOKEN_GREEN_WHITE,
    count: ValueDef::Constant(1),
    tapped: false,
};

// DGM 114 — Voice of Resurgence
pub(in crate::card::sets) static VOICE_OF_RESURGENCE: CardRecord = CardRecord::new(
    cards::VOICE_OF_RESURGENCE,
    "Voice of Resurgence",
    CardArt::new("07246783-d475-4f61-99ac-e2b574072349", "Winona Nelson"),
    CardSet::DragonsMaze,
    CardRules::new_creature(
        mana_cost!("{G}{W}"),
        &["Elemental"],
        2,
        2,
    )
    // One printed sentence, two separate triggers: the cast one only during
    // your turn, and the death one whenever it happens.
    .with_abilities(&[
        AbilityDef::triggered_if(
            "Whenever an opponent casts a spell during your turn, create a green and white Elemental creature token with \"This token's power and toughness are each equal to the number of creatures you control.\"",
            TriggerEventDef::SpellCast(ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent)),
            &VOICE_OF_RESURGENCE_DURING_YOUR_TURN,
            VOICE_OF_RESURGENCE_TOKEN,
        ),
        AbilityDef::triggered(
            "When this creature dies, create a green and white Elemental creature token with \"This token's power and toughness are each equal to the number of creatures you control.\"",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)),
            VOICE_OF_RESURGENCE_TOKEN,
        ),
    ]),
);

// DGM 115 — Vorel of the Hull Clade
// Audit: blocked — Needs an effect that doubles every kind of counter on one targeted artifact, creature, or land.

// DGM 116 — Warleader's Helix
pub(in crate::card::sets) static WARLEADERS_HELIX: CardRecord = CardRecord::new(
    cards::WARLEADERS_HELIX,
    "Warleader's Helix",
    CardArt::new("81e474ac-54f7-43f9-8af9-2f1adf258b15", "Greg Staples"),
    CardSet::DragonsMaze,
    CardRules::new_instant(mana_cost!("{2}{R}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Warleader's Helix deals 4 damage to any target and you gain 4 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
        ]),
    )),
);

// DGM 117 — Warped Physique
// Audit: blocked — Needs a current hand-card count value and its negation to drive the temporary +X/-X effect.

// DGM 118 — Woodlot Crawler
pub(in crate::card::sets) static WOODLOT_CRAWLER: CardRecord = CardRecord::new(
    cards::WOODLOT_CRAWLER,
    "Woodlot Crawler",
    CardArt::new("11f1e6fe-e959-4030-9925-9ccc27040275", "Greg Staples"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{U}{B}"), &["Insect"], 2, 1).with_abilities(&[
        abilities::forestwalk(),
        abilities::protection_from(ManaColor::Green),
    ]),
);

// DGM 119 — Zhur-Taa Ancient
// Audit: blocked — Needs mana-production provenance so the trigger can add one mana of a type the tapped land produced.

// DGM 120 — Zhur-Taa Druid
pub(in crate::card::sets) static ZHUR_TAA_DRUID: CardRecord = CardRecord::new(
    cards::ZHUR_TAA_DRUID,
    "Zhur-Taa Druid",
    CardArt::new("fd565782-8b2f-4b9f-a62d-4af60af20a82", "Mark Winters"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{R}{G}"), &["Human", "Druid"], 1, 1).with_abilities(&[
        abilities::tap_for(ManaColor::Green),
        AbilityDef::triggered(
            "Whenever you tap this creature for mana, it deals 1 damage to each opponent.",
            TriggerEventDef::tapped_for_mana(ObjectPredicateDef::Source),
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// DGM 121 — Alive // Well
// Audit: blocked — Needs fuse spell composition plus a 3/3 green Centaur token and a creature-count life-gain value multiplied by two.

// DGM 122 — Armed // Dangerous
// Audit: blocked — Needs fuse spell composition and a turn-long requirement that every creature able to block the Dangerous target does so.

// DGM 123 — Beck // Call
// Audit: blocked — Needs fuse spell composition plus a temporary enters-the-battlefield listener and a 1/1 white flying Bird token.

// DGM 124 — Breaking // Entering
// Audit: blocked — Needs fuse spell composition and a nontarget creature-card choice from either graveyard for Entering.

// DGM 125 — Catch // Release
// Audit: blocked — Needs fuse spell composition and one independent permanent choice for each named card type from every player.

static ANY_PLAYER_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];
static CREATURE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];
static OWN_GRAVEYARD_CARD_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Any,
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: Some(PlayerRelation::You),
    },
)];

// DGM 126 — Down // Dirty
pub(in crate::card::sets) static DOWN_DIRTY: CardRecord = CardRecord::new(
    cards::DOWN_DIRTY,
    "Down // Dirty",
    CardArt::new("c35c63c1-6344-4d8c-8f7d-cd253d12f9ae", "Svetlin Velinov"),
    CardSet::DragonsMaze,
    down_rules(),
)
.with_composition(down_dirty_composition);

const fn down_rules() -> CardRules {
    CardRules::new_sorcery(mana_cost!("{3}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player discards two cards.",
        &ANY_PLAYER_TARGET,
        EffectDef::Discard {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
            selection: DiscardSelectionDef::RecipientChooses,
        },
    ))
}

fn down_dirty_composition() -> CardComposition {
    let down = down_rules();
    let dirty =
        CardRules::new_sorcery(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell_with_targets(
            "Return target card from your graveyard to your hand.",
            &OWN_GRAVEYARD_CARD_TARGET,
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                controller: None,
            },
        ));
    split_fuse_composition("Down", down, "Dirty", dirty, mana_cost!("{5}{B}{G}"))
}

// DGM 127 — Far // Away
pub(in crate::card::sets) static FAR_AWAY: CardRecord = CardRecord::new(
    cards::FAR_AWAY,
    "Far // Away",
    CardArt::new("d13cdb71-a499-41db-84e6-95f84650c524", "Greg Staples"),
    CardSet::DragonsMaze,
    far_rules(),
)
.with_composition(far_away_composition);

const fn far_rules() -> CardRules {
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target creature to its owner's hand.",
        &CREATURE_TARGET,
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            controller: None,
        },
    ))
}

fn far_away_composition() -> CardComposition {
    let far = far_rules();
    let away =
        CardRules::new_instant(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell_with_targets(
            "Target player sacrifices a creature of their choice.",
            &ANY_PLAYER_TARGET,
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                object: ObjectPredicateDef::HasType(CardType::Creature),
                then: None,
                optional: false,
            },
        ));
    split_fuse_composition("Far", far, "Away", away, mana_cost!("{3}{U}{B}"))
}

// DGM 128 — Flesh // Blood
// Audit: blocked — Needs fuse spell composition and a value carrying the exiled graveyard card's power into Flesh's counter effect.

// DGM 129 — Give // Take
// Audit: blocked — Needs fuse spell composition and removing all +1/+1 counters from the targeted creature while remembering the removed count.

// DGM 130 — Profit // Loss
pub(in crate::card::sets) static PROFIT_LOSS: CardRecord = CardRecord::new(
    cards::PROFIT_LOSS,
    "Profit // Loss",
    CardArt::new("0eb3ce46-ddd2-43b3-9e45-019ae91df686", "Kev Walker"),
    CardSet::DragonsMaze,
    profit_rules(),
)
.with_composition(profit_loss_composition);

const fn profit_rules() -> CardRules {
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell(
        "Creatures you control get +1/+1 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    ))
}

fn profit_loss_composition() -> CardComposition {
    let profit = profit_rules();
    let loss = CardRules::new_instant(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell(
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
    ));
    split_fuse_composition("Profit", profit, "Loss", loss, mana_cost!("{3}{W}{B}"))
}

// DGM 131 — Protect // Serve
pub(in crate::card::sets) static PROTECT_SERVE: CardRecord = CardRecord::new(
    cards::PROTECT_SERVE,
    "Protect // Serve",
    CardArt::new("9b8acd7d-f3e2-4358-91ab-40901b68d64c", "Ryan Barger"),
    CardSet::DragonsMaze,
    protect_rules(),
)
.with_composition(protect_serve_composition);

const fn protect_rules() -> CardRules {
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +2/+4 until end of turn.",
        &CREATURE_TARGET,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(2),
                ValueDef::Constant(4),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    ))
}

fn protect_serve_composition() -> CardComposition {
    let protect = protect_rules();
    let serve =
        CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
            "Target creature gets -6/-0 until end of turn.",
            &CREATURE_TARGET,
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-6),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ));
    split_fuse_composition("Protect", protect, "Serve", serve, mana_cost!("{3}{W}{U}"))
}

// DGM 132 — Ready // Willing
pub(in crate::card::sets) static READY_WILLING: CardRecord = CardRecord::new(
    cards::READY_WILLING,
    "Ready // Willing",
    CardArt::new("22081f95-dc8e-41ed-b609-b6a22ee5428b", "Zoltan Boros"),
    CardSet::DragonsMaze,
    ready_rules(),
)
.with_composition(ready_willing_composition);

static READY_INDESTRUCTIBLE: AbilityDef = abilities::indestructible();
static READY_EFFECTS: [EffectDef; 2] = [
    EffectDef::Apply {
        recipient: EffectRecipientDef::matching_objects(
            ObjectPredicateDef::HasType(CardType::Creature),
            &[ZoneKind::Battlefield],
            PlayerRelation::You,
        ),
        effect: AppliedEffectDef::add_ability(&READY_INDESTRUCTIBLE),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
    EffectDef::Untap {
        object: EffectRecipientDef::matching_objects(
            ObjectPredicateDef::HasType(CardType::Creature),
            &[ZoneKind::Battlefield],
            PlayerRelation::You,
        ),
    },
];
static WILLING_DEATHTOUCH: AbilityDef = abilities::deathtouch();
static WILLING_LIFELINK: AbilityDef = abilities::lifelink();
static WILLING_KEYWORDS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::add_ability(&WILLING_DEATHTOUCH),
    AppliedEffectDef::add_ability(&WILLING_LIFELINK),
];

const fn ready_rules() -> CardRules {
    CardRules::new_instant(mana_cost!("{1}{G}{W}")).with_ability(AbilityDef::spell(
        "Creatures you control gain indestructible until end of turn. Untap each creature you control.",
        EffectDef::Sequence(&READY_EFFECTS),
    ))
}

fn ready_willing_composition() -> CardComposition {
    let ready = ready_rules();
    let willing = CardRules::new_instant(mana_cost!("{1}{W}{B}")).with_ability(AbilityDef::spell(
        "Creatures you control gain deathtouch and lifelink until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::Composite(&WILLING_KEYWORDS),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    ));
    split_fuse_composition(
        "Ready",
        ready,
        "Willing",
        willing,
        mana_cost!("{2}{W}{W}{B}{G}"),
    )
}

#[allow(clippy::large_types_passed_by_value)]
fn split_fuse_composition(
    first_name: &str,
    first: CardRules,
    second_name: &str,
    second: CardRules,
    fused_cost: ManaCost,
) -> CardComposition {
    let combined_name = format!("{first_name} // {second_name}");
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, first_name, first),
            CardPart::new(CardPartId(1), second_name, second),
        ],
        structure: CardStructure::Split {
            parts: vec![CardPartId::PRIMARY, CardPartId(1)],
            fused: Some(PlayOptionId(2)),
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                first_name,
                SpellForm::Part(CardPartId::PRIMARY),
                first
                    .mana_cost()
                    .expect("a split-card first half has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                second_name,
                SpellForm::Part(CardPartId(1)),
                second
                    .mana_cost()
                    .expect("a split-card second half has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(2),
                combined_name,
                SpellForm::Combined(vec![CardPartId::PRIMARY, CardPartId(1)]),
                fused_cost,
                CardEffectStatus::Implemented,
            )
            .restricted_to_hand(),
        ],
    }
    .with_derived_spell_targets()
}

// DGM 133 — Toil // Trouble
// Audit: blocked — Needs fuse spell composition and a value for the targeted player's current hand size.

/// Turn repaints the characteristics it names while leaving the target's
/// other card types and subtype categories intact.
static TURN_CHARACTERISTICS: [AppliedEffectDef; 5] = [
    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
    AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Weird"])),
    AppliedEffectDef::remove_abilities(crate::card::AbilityPredicateDef::Any),
    AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Red])),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(0), ValueDef::Constant(1)),
];

static TURN_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];
static BURN_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

const fn turn_rules() -> CardRules {
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Until end of turn, target creature loses all abilities and becomes a red Weird with base power and toughness 0/1.\nFuse (You may cast one or both halves of this card from your hand.)",
        &TURN_TARGETS,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&TURN_CHARACTERISTICS),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    ))
}

fn turn_burn_composition() -> CardComposition {
    let turn = turn_rules();
    let burn = CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Burn deals 2 damage to any target.\nFuse (You may cast one or both halves of this card from your hand.)",
            &BURN_TARGETS,
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
    );
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Turn", turn),
            CardPart::new(CardPartId(1), "Burn", burn),
        ],
        structure: CardStructure::Split {
            parts: vec![CardPartId::PRIMARY, CardPartId(1)],
            fused: Some(PlayOptionId(2)),
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Turn",
                SpellForm::Part(CardPartId::PRIMARY),
                turn.mana_cost().expect("Turn has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Burn",
                SpellForm::Part(CardPartId(1)),
                burn.mana_cost().expect("Burn has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(2),
                "Turn // Burn",
                SpellForm::Combined(vec![CardPartId::PRIMARY, CardPartId(1)]),
                mana_cost!("{3}{U}{R}"),
                CardEffectStatus::Implemented,
            )
            .restricted_to_hand(),
        ],
    }
    .with_derived_spell_targets()
}

// DGM 134 — Turn // Burn
pub(in crate::card::sets) static TURN_BURN: CardRecord = CardRecord::new(
    cards::TURN_BURN,
    "Turn // Burn",
    CardArt::new("8d7fdd59-6d76-4a0c-ac75-816345ef4a39", "Ryan Barger"),
    CardSet::DragonsMaze,
    turn_rules(),
)
.with_composition(turn_burn_composition);

// DGM 135 — Wear // Tear
pub(in crate::card::sets) static WEAR_TEAR: CardRecord = CardRecord::new(
    cards::WEAR_TEAR,
    "Wear // Tear",
    CardArt::new("d169a3b2-18ae-4414-98ef-d879676fdcc0", "Ryan Pancoast"),
    CardSet::DragonsMaze,
    wear_rules(),
)
.with_composition(wear_tear_composition);

static ARTIFACT_TARGET: AbilityTargetDef =
    AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Artifact));
static ENCHANTMENT_TARGET: AbilityTargetDef =
    AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Enchantment));

const fn wear_rules() -> CardRules {
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::destroy_target(
        "Destroy target artifact.",
        &ARTIFACT_TARGET,
        true,
    ))
}

fn wear_tear_composition() -> CardComposition {
    let wear = wear_rules();
    let tear = CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::destroy_target(
        "Destroy target enchantment.",
        &ENCHANTMENT_TARGET,
        true,
    ));
    split_fuse_composition("Wear", wear, "Tear", tear, mana_cost!("{1}{R}{W}"))
}

macro_rules! cluestone_abilities {
    ($name:ident, $colors:expr, $mana_text:literal, $draw_text:literal, $draw_cost:literal) => {
        static $name: [AbilityDef; 2] = [
            AbilityDef::activated_mana(
                $mana_text,
                &[AbilityCostDef::TapSource],
                EffectDef::AddMana(crate::card::AddManaEffectDef::choice($colors)),
            ),
            AbilityDef::activated(
                $draw_text,
                &[
                    AbilityCostDef::Mana(mana_cost!($draw_cost)),
                    AbilityCostDef::TapSource,
                    AbilityCostDef::SacrificeSource,
                ],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
        ];
    };
}

cluestone_abilities!(
    AZORIUS_CLUESTONE_ABILITIES,
    &[ManaColor::White, ManaColor::Blue],
    "{T}: Add {W} or {U}.",
    "{W}{U}, {T}, Sacrifice this artifact: Draw a card.",
    "{W}{U}"
);
cluestone_abilities!(
    BOROS_CLUESTONE_ABILITIES,
    &[ManaColor::Red, ManaColor::White],
    "{T}: Add {R} or {W}.",
    "{R}{W}, {T}, Sacrifice this artifact: Draw a card.",
    "{R}{W}"
);
cluestone_abilities!(
    DIMIR_CLUESTONE_ABILITIES,
    &[ManaColor::Blue, ManaColor::Black],
    "{T}: Add {U} or {B}.",
    "{U}{B}, {T}, Sacrifice this artifact: Draw a card.",
    "{U}{B}"
);
cluestone_abilities!(
    GOLGARI_CLUESTONE_ABILITIES,
    &[ManaColor::Black, ManaColor::Green],
    "{T}: Add {B} or {G}.",
    "{B}{G}, {T}, Sacrifice this artifact: Draw a card.",
    "{B}{G}"
);
cluestone_abilities!(
    GRUUL_CLUESTONE_ABILITIES,
    &[ManaColor::Red, ManaColor::Green],
    "{T}: Add {R} or {G}.",
    "{R}{G}, {T}, Sacrifice this artifact: Draw a card.",
    "{R}{G}"
);
cluestone_abilities!(
    IZZET_CLUESTONE_ABILITIES,
    &[ManaColor::Blue, ManaColor::Red],
    "{T}: Add {U} or {R}.",
    "{U}{R}, {T}, Sacrifice this artifact: Draw a card.",
    "{U}{R}"
);
cluestone_abilities!(
    ORZHOV_CLUESTONE_ABILITIES,
    &[ManaColor::White, ManaColor::Black],
    "{T}: Add {W} or {B}.",
    "{W}{B}, {T}, Sacrifice this artifact: Draw a card.",
    "{W}{B}"
);
cluestone_abilities!(
    RAKDOS_CLUESTONE_ABILITIES,
    &[ManaColor::Black, ManaColor::Red],
    "{T}: Add {B} or {R}.",
    "{B}{R}, {T}, Sacrifice this artifact: Draw a card.",
    "{B}{R}"
);
cluestone_abilities!(
    SELESNYA_CLUESTONE_ABILITIES,
    &[ManaColor::Green, ManaColor::White],
    "{T}: Add {G} or {W}.",
    "{G}{W}, {T}, Sacrifice this artifact: Draw a card.",
    "{G}{W}"
);
cluestone_abilities!(
    SIMIC_CLUESTONE_ABILITIES,
    &[ManaColor::Green, ManaColor::Blue],
    "{T}: Add {G} or {U}.",
    "{G}{U}, {T}, Sacrifice this artifact: Draw a card.",
    "{G}{U}"
);

const fn cluestone_rules(abilities: &'static [AbilityDef]) -> CardRules {
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(abilities)
}

// DGM 136 — Azorius Cluestone
pub(in crate::card::sets) static AZORIUS_CLUESTONE: CardRecord = CardRecord::new(
    cards::AZORIUS_CLUESTONE,
    "Azorius Cluestone",
    CardArt::new("09eeb301-bc28-4515-ad69-0b1b5164a5bc", "Raoul Vitale"),
    CardSet::DragonsMaze,
    cluestone_rules(&AZORIUS_CLUESTONE_ABILITIES),
);

// DGM 137 — Boros Cluestone
pub(in crate::card::sets) static BOROS_CLUESTONE: CardRecord = CardRecord::new(
    cards::BOROS_CLUESTONE,
    "Boros Cluestone",
    CardArt::new("87252577-3e7b-4ea2-b0ac-3ba3f0eaac40", "Raoul Vitale"),
    CardSet::DragonsMaze,
    cluestone_rules(&BOROS_CLUESTONE_ABILITIES),
);

// DGM 138 — Dimir Cluestone
pub(in crate::card::sets) static DIMIR_CLUESTONE: CardRecord = CardRecord::new(
    cards::DIMIR_CLUESTONE,
    "Dimir Cluestone",
    CardArt::new("0d8ac24f-3309-453a-b2d6-6363df9a1ddd", "Raoul Vitale"),
    CardSet::DragonsMaze,
    cluestone_rules(&DIMIR_CLUESTONE_ABILITIES),
);

// DGM 139 — Golgari Cluestone
pub(in crate::card::sets) static GOLGARI_CLUESTONE: CardRecord = CardRecord::new(
    cards::GOLGARI_CLUESTONE,
    "Golgari Cluestone",
    CardArt::new("ff77e1ee-7fa3-4370-a0c9-ec008b63302f", "Raoul Vitale"),
    CardSet::DragonsMaze,
    cluestone_rules(&GOLGARI_CLUESTONE_ABILITIES),
);

// DGM 140 — Gruul Cluestone
pub(in crate::card::sets) static GRUUL_CLUESTONE: CardRecord = CardRecord::new(
    cards::GRUUL_CLUESTONE,
    "Gruul Cluestone",
    CardArt::new("bc47d1fe-8ab2-42f6-bcab-4bc2084ceba7", "Raoul Vitale"),
    CardSet::DragonsMaze,
    cluestone_rules(&GRUUL_CLUESTONE_ABILITIES),
);

// DGM 141 — Izzet Cluestone
pub(in crate::card::sets) static IZZET_CLUESTONE: CardRecord = CardRecord::new(
    cards::IZZET_CLUESTONE,
    "Izzet Cluestone",
    CardArt::new("8cf63def-e2cc-48c7-8409-c08a36eddf93", "Raoul Vitale"),
    CardSet::DragonsMaze,
    cluestone_rules(&IZZET_CLUESTONE_ABILITIES),
);

// DGM 142 — Orzhov Cluestone
pub(in crate::card::sets) static ORZHOV_CLUESTONE: CardRecord = CardRecord::new(
    cards::ORZHOV_CLUESTONE,
    "Orzhov Cluestone",
    CardArt::new("4823f904-1c41-42cf-aef7-db0dcf82b10b", "Raoul Vitale"),
    CardSet::DragonsMaze,
    cluestone_rules(&ORZHOV_CLUESTONE_ABILITIES),
);

// DGM 143 — Rakdos Cluestone
pub(in crate::card::sets) static RAKDOS_CLUESTONE: CardRecord = CardRecord::new(
    cards::RAKDOS_CLUESTONE,
    "Rakdos Cluestone",
    CardArt::new("9ef43817-1813-4608-8e3d-3c14321ab736", "Raoul Vitale"),
    CardSet::DragonsMaze,
    cluestone_rules(&RAKDOS_CLUESTONE_ABILITIES),
);

// DGM 144 — Selesnya Cluestone
pub(in crate::card::sets) static SELESNYA_CLUESTONE: CardRecord = CardRecord::new(
    cards::SELESNYA_CLUESTONE,
    "Selesnya Cluestone",
    CardArt::new("34ad5631-439a-43e2-b00a-04f78d66b8e6", "Raoul Vitale"),
    CardSet::DragonsMaze,
    cluestone_rules(&SELESNYA_CLUESTONE_ABILITIES),
);

// DGM 145 — Simic Cluestone
pub(in crate::card::sets) static SIMIC_CLUESTONE: CardRecord = CardRecord::new(
    cards::SIMIC_CLUESTONE,
    "Simic Cluestone",
    CardArt::new("e3c47552-afed-463d-bd24-13eb1cd724fc", "Raoul Vitale"),
    CardSet::DragonsMaze,
    cluestone_rules(&SIMIC_CLUESTONE_ABILITIES),
);

// DGM 152 — Maze's End
// Audit: blocked — Needs returning the land as an activation cost, a Gate-specific library search to the battlefield, and the ten-distinct-names win condition.

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BOROS_MASTIFF,
    &HAAZDA_SNARE_SQUAD,
    &LYEV_DECREE,
    &MAZE_SENTINEL,
    &RENOUNCE_THE_GUILDS,
    &RIOT_CONTROL,
    &STEEPLE_ROC,
    &SUNSPIRE_GATEKEEPERS,
    &AETHERLING,
    &MAZE_GLIDER,
    &MINDSTATIC,
    &MURMURING_PHANTASM,
    &OPAL_LAKE_GATEKEEPERS,
    &RUNNERS_BANE,
    &BANE_ALLEY_BLACKGUARD,
    &FATAL_FUMES,
    &MAZE_ABOMINATION,
    &RAKDOS_DRAKE,
    &UBUL_SAR_GATEKEEPERS,
    &CLEAR_A_PATH,
    &MAZE_RUSHER,
    &PUNISH_THE_ENEMY,
    &PYREWILD_SHAMAN,
    &RIOT_PIKER,
    &RUBBLEBELT_MAAKA,
    &SMELT_WARD_GATEKEEPERS,
    &WEAPON_SURGE,
    &KRAUL_WARRIOR,
    &MAZE_BEHEMOTH,
    &MENDING_TOUCH,
    &PHYTOBURST,
    &SARULI_GATEKEEPERS,
    &SKYLASHER,
    &ADVENT_OF_THE_WURM,
    &ARMORED_WOLF_RIDER,
    &ASCENDED_LAWMAGE,
    &BEETLEFORM_MAGE,
    &BLOOD_BARON_OF_VIZKOPA,
    &BRONZEBEAK_MOA,
    &DEPUTY_OF_ACQUITTALS,
    &DROWN_IN_FILTH,
    &EXAVA_RAKDOS_BLOOD_WITCH,
    &FERAL_ANIMIST,
    &GAZE_OF_GRANITE,
    &GLEAM_OF_BATTLE,
    &HAUNTER_OF_NIGHTVEIL,
    &JELENN_SPHINX,
    &MAW_OF_THE_OBZEDAT,
    &MORGUE_BURST,
    &OBZEDATS_AID,
    &PILFERED_PLANS,
    &PUTREFY,
    &RAL_ZAREK,
    &RURIC_THAR_THE_UNBOWED,
    &SHOWSTOPPER,
    &SIN_COLLECTOR,
    &SIRE_OF_INSANITY,
    &SPECIES_GORGER,
    &SPIKE_JESTER,
    &TAJIC_BLADE_OF_THE_LEGION,
    &TROSTANIS_SUMMONER,
    &UNFLINCHING_COURAGE,
    &VIASHINO_FIRSTBLADE,
    &VOICE_OF_RESURGENCE,
    &WARLEADERS_HELIX,
    &WOODLOT_CRAWLER,
    &ZHUR_TAA_DRUID,
    &DOWN_DIRTY,
    &FAR_AWAY,
    &PROFIT_LOSS,
    &PROTECT_SERVE,
    &READY_WILLING,
    &TURN_BURN,
    &WEAR_TEAR,
    &AZORIUS_CLUESTONE,
    &BOROS_CLUESTONE,
    &DIMIR_CLUESTONE,
    &GOLGARI_CLUESTONE,
    &GRUUL_CLUESTONE,
    &IZZET_CLUESTONE,
    &ORZHOV_CLUESTONE,
    &RAKDOS_CLUESTONE,
    &SELESNYA_CLUESTONE,
    &SIMIC_CLUESTONE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::WIND_DRAKE), // DGM 20
    PrintingRecord::reprint(&return_to_ravnica::AZORIUS_GUILDGATE),             // DGM 146
    PrintingRecord::reprint(&gatecrash::BOROS_GUILDGATE),                       // DGM 147
    PrintingRecord::reprint(&gatecrash::DIMIR_GUILDGATE),                       // DGM 148
    PrintingRecord::reprint(&return_to_ravnica::GOLGARI_GUILDGATE),             // DGM 149
    PrintingRecord::reprint(&gatecrash::GRUUL_GUILDGATE),                       // DGM 150
    PrintingRecord::reprint(&return_to_ravnica::IZZET_GUILDGATE),               // DGM 151
    PrintingRecord::reprint(&gatecrash::ORZHOV_GUILDGATE),                      // DGM 153
    PrintingRecord::reprint(&return_to_ravnica::RAKDOS_GUILDGATE),              // DGM 154
    PrintingRecord::reprint(&return_to_ravnica::SELESNYA_GUILDGATE),            // DGM 155
    PrintingRecord::reprint(&gatecrash::SIMIC_GUILDGATE),                       // DGM 156
];
