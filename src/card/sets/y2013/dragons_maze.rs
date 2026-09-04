//! Dragon's Maze card records used by the built-in ISD–M14 Standard decks.

use super::{CardRecord, PrintingRecord, gatecrash};
use crate::card::sets::y2012::return_to_ravnica;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AggregateOperationDef,
    AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    CardTypeSet, ChoiceVisibilityDef, ChooseDef, ColorSet, ComparisonDef, ControlDurationDef,
    CopyAbilityDef, CopyExceptionsDef, CounterKind, CreatureTypeSetDef, DamageEventMatcherDef,
    DamagePreventionDef, DamageRecipientMatcherDef, DiscardSelectionDef, EffectDef,
    EffectPaymentDef, EffectRecipientDef, LikelihoodDef, ManaColor, ObjectChoiceBindingDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, ObjectValueAggregateDef,
    ObjectValueDef, PayOrDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    ResolvedEffectDurationDef, SacrificedAmountDef, TriggerConditionDef, TriggerEventDef,
    TurnStepDef, ValueComparisonDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::{ParentBinding, TargetIndex};
use crate::mana_cost;

static MULTICOLORED: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::ColorCount(2),
    ObjectPredicateDef::ColorCount(3),
    ObjectPredicateDef::ColorCount(4),
    ObjectPredicateDef::ColorCount(5),
]);

static TWO_GATES_CONDITION: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::Subtype("Gate"),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 2,
};

static INSTANT_OR_SORCERY_YOU_CAST: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
    ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Instant),
        ObjectPredicateDef::HasType(CardType::Sorcery),
    ]),
]);

static ANY_PLAYER_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

static CREATURE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

const fn cluestone_rules(abilities: &'static [AbilityDef]) -> CardRules {
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(abilities)
}

// DGM 1 — Boros Mastiff
pub(in crate::card::sets) static BOROS_MASTIFF: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Boros Mastiff",
    "27a3bfb6-3843-4bda-bbcb-905e4b351dea",
    "Kev Walker",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Dog"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Battalion — Whenever this creature and at least two other creatures attack, this creature gains lifelink until end of turn.",
            TriggerEventDef::attacks_in_declaration(ObjectPredicateDef::Source, 3, None),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// DGM 2 — Haazda Snare Squad
pub(in crate::card::sets) static HAAZDA_SNARE_SQUAD: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Haazda Snare Squad",
    "85d3c012-f356-424d-a960-60e95f395134",
    "David Palumbo",
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
    CardSet::DragonsMaze,
    "Lyev Decree",
    "773cf2aa-4337-4d14-8a8e-ff8b1fdec1b5",
    "Kev Walker",
    CardRules::new_sorcery(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Detain up to two target creatures your opponents control.",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            },
            2,
        )],
        EffectDef::Detain {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )),
);

// DGM 4 — Maze Sentinel
pub(in crate::card::sets) static MAZE_SENTINEL: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Maze Sentinel",
    "7a977e2d-a2bc-42d1-be7d-36a822c6a66e",
    "Yeong-Hao Han",
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
    CardSet::DragonsMaze,
    "Renounce the Guilds",
    "bc9acc14-24e0-4c03-a09a-2afee351f2cc",
    "Daarken",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell(
        "Each player sacrifices a multicolored permanent of their choice.",
        EffectDef::SacrificeOfChoice {
            count: ValueDef::Constant(1),
            player: EffectRecipientDef::EachPlayer,
            object: MULTICOLORED,
            then: None,
            amount: SacrificedAmountDef::Power,
            otherwise: None,
            optional: false,
        },
    )),
);

// DGM 6 — Riot Control
pub(in crate::card::sets) static RIOT_CONTROL: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Riot Control",
    "d7886607-86db-4221-8752-296104aaaef2",
    "Slawomir Maniak",
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell(
        "You gain 1 life for each creature your opponents control. Prevent all damage that would be dealt to you this turn.",
        EffectDef::Sequence(&[
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                )),
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
// Audit: unsupported — Needs an enters-trigger condition that remembers whether the permanent was cast from hand, plus populate's token-copy choice.
pub(in crate::card::sets) static SCION_OF_VITU_GHAZI: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Scion of Vitu-Ghazi",
    "3cd20865-0a9a-4a72-92f9-77c8d6384b46",
    "Willian Murai",
    crate::card::CardRules::unsupported(),
);

// DGM 8 — Steeple Roc
pub(in crate::card::sets) static STEEPLE_ROC: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Steeple Roc",
    "5fecafab-97f4-40ed-bc43-d186eb2f3af6",
    "David Palumbo",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Bird"], 3, 1)
        .with_abilities(&[abilities::flying(), abilities::first_strike()]),
);

// DGM 9 — Sunspire Gatekeepers
pub(in crate::card::sets) static SUNSPIRE_GATEKEEPERS: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Sunspire Gatekeepers",
    "0a3bc6b9-475b-4257-a3bc-1a0b70d45f79",
    "Chippy",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Soldier"], 2, 4).with_ability(
        AbilityDef::triggered_if(
            "When this creature enters, if you control two or more Gates, create a 2/2 white Knight creature token with vigilance.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
            &TWO_GATES_CONDITION,
            EffectDef::create_creature_token(&["Knight"], &[ManaColor::White], 2, 2).with_abilities(&[abilities::vigilance()]).with_art(CardArt::new("67d3d039-248a-4eb8-be5c-12959b458fea", "Matt Stewart")),
        ),
    ),
);

// DGM 10 — Wake the Reflections
pub(in crate::card::sets) static WAKE_THE_REFLECTIONS: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Wake the Reflections",
    "3db0074c-95cf-4d15-8fe1-7282803ec757",
    "Cynthia Sheppard",
    // Populate and nothing else, so a board with no creature token makes
    // this a blank rather than an illegal cast.
    CardRules::new_sorcery(mana_cost!("{W}"))
        .with_ability(AbilityDef::spell("Populate.", abilities::populate())),
);

// DGM 11 — Aetherling
pub(in crate::card::sets) static AETHERLING: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Aetherling",
    "9c93313b-cf43-47e9-a911-717b4d14b0b5",
    "Tyler Jacobson",
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
            abilities::exile_until_next_end_step(EffectRecipientDef::Source),
        ),
        AbilityDef::activated(
            "{U}: This creature can't be blocked this turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
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
// Audit: unsupported — Needs tap-or-untap choices made independently on resolution and cipher's encoded-card link, combat-damage trigger, and free-copy casting permission.
pub(in crate::card::sets) static HIDDEN_STRINGS: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Hidden Strings",
    "216e8047-6f54-49ce-bf86-27dc8fc8c8f7",
    "Daarken",
    crate::card::CardRules::unsupported(),
);

// DGM 13 — Maze Glider
pub(in crate::card::sets) static MAZE_GLIDER: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Maze Glider",
    "d1d20281-49c0-4fd0-91f2-390506ac33f6",
    "Yeong-Hao Han",
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
    CardSet::DragonsMaze,
    "Mindstatic",
    "55d3fad5-a12a-4b41-9c7b-c1af5e0b5ca8",
    "Johann Bodin",
    CardRules::new_instant(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell unless its controller pays {6}.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        abilities::counter_target_unless_paid(ValueDef::Constant(6)),
    )),
);

// DGM 15 — Murmuring Phantasm
pub(in crate::card::sets) static MURMURING_PHANTASM: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Murmuring Phantasm",
    "9752644c-7c43-429e-a79c-1239b9a0bc8a",
    "Peter Mohrbacher",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Spirit"], 0, 5)
        .with_ability(abilities::defender()),
);

// DGM 16 — Opal Lake Gatekeepers
pub(in crate::card::sets) static OPAL_LAKE_GATEKEEPERS: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Opal Lake Gatekeepers",
    "f43ac38f-5cd0-46cf-8623-d82cb8fb719b",
    "Seb McKinnon",
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
pub(in crate::card::sets) static RUNNERS_BANE: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Runner's Bane",
    "4696b5a6-edfd-445e-ac80-64c1be94fbfc",
    "Karl Kopinski",
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
            ),
            abilities::enters_trigger(
                "When this Aura enters, tap enchanted creature.",
                EffectDef::Tap {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature doesn't untap during its controller's untap step.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                },
            ),
        ]),
);

// DGM 18 — Trait Doctoring
// Audit: unsupported — Needs duration-scoped color-word text changes and cipher's encoded-card link, combat-damage trigger, and free-copy casting permission.
pub(in crate::card::sets) static TRAIT_DOCTORING: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Trait Doctoring",
    "e21a7981-5940-4b75-907f-7600a742f946",
    "Clint Cearley",
    crate::card::CardRules::unsupported(),
);

// DGM 19 — Uncovered Clues
// Audit: unsupported — Needs a top-four selection constrained to up to two instant or sorcery followed by ordering the unselected cards on the library bottom.
pub(in crate::card::sets) static UNCOVERED_CLUES: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Uncovered Clues",
    "9dd24556-994f-4480-835e-11d4443f0700",
    "Jaime Jones",
    crate::card::CardRules::unsupported(),
);

// DGM 20 — Wind Drake (reprint)
const WIND_DRAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::WIND_DRAKE,
    "86ea454f-b640-4a89-937f-bae05556292a",
    "John Severin Brassell",
);

// DGM 21 — Bane Alley Blackguard
pub(in crate::card::sets) static BANE_ALLEY_BLACKGUARD: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Bane Alley Blackguard",
    "15fcad03-4567-4f96-976e-01a07d8ab050",
    "Mike Bierek",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Rogue"], 1, 3),
);

// DGM 22 — Blood Scrivener
// Audit: unsupported — Needs a draw-event replacement that checks an empty hand and replaces one draw with two cards plus one life loss.
pub(in crate::card::sets) static BLOOD_SCRIVENER: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Blood Scrivener",
    "9ea8179a-d3c9-4cdc-a5b5-68cc73279050",
    "Peter Mohrbacher",
    crate::card::CardRules::unsupported(),
);

// DGM 23 — Crypt Incursion
// Audit: unsupported — Needs the number of cards actually exiled by a graveyard sweep to feed one life-gain event after replacements are applied.
pub(in crate::card::sets) static CRYPT_INCURSION: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Crypt Incursion",
    "c3b71cc5-0a81-4cab-bae3-49335c04aaaa",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

// DGM 24 — Fatal Fumes
pub(in crate::card::sets) static FATAL_FUMES: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Fatal Fumes",
    "967aa636-a11d-4c5c-ba85-648734b295c2",
    "Kev Walker",
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
// Audit: unsupported — Needs revealing a random card from the targeted opponent's hand after the life-loss effect.
pub(in crate::card::sets) static HIRED_TORTURER: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Hired Torturer",
    "62e9f79e-6606-4c9b-838c-eda5d8cc612c",
    "Winona Nelson",
    crate::card::CardRules::unsupported(),
);

// DGM 26 — Maze Abomination
pub(in crate::card::sets) static MAZE_ABOMINATION: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Maze Abomination",
    "dd84659f-4209-42a2-800a-61706470ce54",
    "Yeong-Hao Han",
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
pub(in crate::card::sets) static PONTIFF_OF_BLIGHT: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Pontiff of Blight",
    "72e5291f-9281-4cb7-9158-54b7cb336b93",
    "Seb McKinnon",
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Zombie", "Cleric"], 2, 7).with_abilities(
        &[
            abilities::extort(),
            AbilityDef::static_ability(
                "Other creatures you control have extort.",
                // Each granted copy is its own instance, so one spell offers one payment per
                // creature rather than a single drain for the board.
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::extort()),
                },
            ),
        ],
    ),
);

// DGM 28 — Rakdos Drake
pub(in crate::card::sets) static RAKDOS_DRAKE: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Rakdos Drake",
    "b9c1bfd7-b8b2-4db7-9ea7-a2d643a83589",
    "Karl Kopinski",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Drake"], 1, 2).with_abilities(&[
        abilities::flying(),
        abilities::unleash(),
        abilities::unleash_counter(),
    ]),
);

// DGM 29 — Sinister Possession
// Audit: unsupported — Needs an Aura to observe both attack and block events from its attached creature and make that creature's controller lose life.
pub(in crate::card::sets) static SINISTER_POSSESSION: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Sinister Possession",
    "7f54c15b-fec0-49a6-8a49-d1af4eeee40e",
    "Anthony Palumbo",
    crate::card::CardRules::unsupported(),
);

// DGM 30 — Ubul Sar Gatekeepers
pub(in crate::card::sets) static UBUL_SAR_GATEKEEPERS: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Ubul Sar Gatekeepers",
    "f5b2e327-adfd-459b-8d18-faa39d88b5de",
    "Volkan Baǵa",
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
pub(in crate::card::sets) static AWE_FOR_THE_GUILDS: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Awe for the Guilds",
    "ec644ac3-07a2-43de-8173-9cc18e2ea2d9",
    "Mathias Kollros",
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell(
        "Monocolored creatures can't block this turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ColorCount(1),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// DGM 32 — Clear a Path
pub(in crate::card::sets) static CLEAR_A_PATH: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Clear a Path",
    "8a8f904b-a9a3-4bae-9284-4e9cbe7592ee",
    "Karl Kopinski",
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
    CardSet::DragonsMaze,
    "Maze Rusher",
    "864d2eb8-e27f-4f84-9725-d2ae6446e217",
    "Yeong-Hao Han",
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
// Audit: unsupported — Needs spell-type-aware library reveal-until, free casting of the found card, and random ordering of the linked exiled cards.
pub(in crate::card::sets) static POSSIBILITY_STORM: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Possibility Storm",
    "858aa831-b491-4f1e-bb56-33eeca14771d",
    "Jason Felix",
    crate::card::CardRules::unsupported(),
);

// DGM 35 — Punish the Enemy
pub(in crate::card::sets) static PUNISH_THE_ENEMY: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Punish the Enemy",
    "4179a72b-8482-46ec-9815-f5d6d94b5aa5",
    "Slawomir Maniak",
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
// Audit: unsupported — Needs one grouped combat-damage event and a paid trigger from the graveyard.
pub(in crate::card::sets) static PYREWILD_SHAMAN: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Pyrewild Shaman",
    "8c6f6e45-f613-420d-83d2-d93c643265ee",
    "Lucas Graciano",
    CardRules::unsupported(),
);

// DGM 37 — Riot Piker
pub(in crate::card::sets) static RIOT_PIKER: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Riot Piker",
    "4daaccd2-733c-4b3b-aa3f-cc825bcc3e53",
    "Christopher Moeller",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Berserker"], 2, 1).with_abilities(
        &[
            abilities::first_strike(),
            abilities::attacks_each_combat_if_able("This creature attacks each combat if able."),
        ],
    ),
);

// DGM 38 — Rubblebelt Maaka
pub(in crate::card::sets) static RUBBLEBELT_MAAKA: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Rubblebelt Maaka",
    "bc802d62-6559-45b9-ad11-de5887aece2b",
    "Eric Velhagen",
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
    CardSet::DragonsMaze,
    "Smelt-Ward Gatekeepers",
    "8237b11f-36d2-4624-a0ef-520663385891",
    "Daarken",
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
                    controller: PlayerRefDef::EffectController,
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

// DGM 40 — Weapon Surge
static WEAPON_SURGE_PUMP: AppliedEffectDef = AppliedEffectDef::Composite(&[
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(0)),
    AppliedEffectDef::add_ability(&abilities::first_strike()),
]);

pub(in crate::card::sets) static WEAPON_SURGE: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Weapon Surge",
    "f28df164-8bff-4428-b7dd-2974c288f1d3",
    "Jason Felix",
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
pub(in crate::card::sets) static BATTERING_KRASIS: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Battering Krasis",
    "5d9aa740-9adf-412a-b6ec-0b9bb1b4618b",
    "Jack Wang",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Shark", "Beast"], 2, 1)
        .with_abilities(&[abilities::trample(), abilities::evolve()]),
);

// DGM 42 — Kraul Warrior
pub(in crate::card::sets) static KRAUL_WARRIOR: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Kraul Warrior",
    "f71da8cc-8773-4dcb-aca8-50a000142218",
    "David Rapoza",
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
    CardSet::DragonsMaze,
    "Maze Behemoth",
    "0a7c9678-dea7-4219-bac0-9e1cef531f54",
    "Yeong-Hao Han",
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
    CardSet::DragonsMaze,
    "Mending Touch",
    "c042c7ee-0e74-4ca5-bbb9-2898b0576f0a",
    "Karla Ortiz",
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
// Audit: unsupported — Needs a target predicate for a +1/+1 counter and the simultaneous fight damage procedure.
pub(in crate::card::sets) static MUTANT_S_PREY: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Mutant's Prey",
    "d9e32d47-2796-4eac-b373-a93506d8d6b7",
    "Ryan Barger",
    crate::card::CardRules::unsupported(),
);

// DGM 46 — Phytoburst
pub(in crate::card::sets) static PHYTOBURST: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Phytoburst",
    "7507afc4-f504-4eb2-a86d-f99bc2860838",
    "Izzy",
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
// Audit: unsupported — Needs evolve's characteristic comparison and an evolve event that can drive the counter sweep.
pub(in crate::card::sets) static RENEGADE_KRASIS: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Renegade Krasis",
    "23b68921-0c34-4d92-83c3-21542f62c7f6",
    "Howard Lyon",
    crate::card::CardRules::unsupported(),
);

// DGM 48 — Saruli Gatekeepers
pub(in crate::card::sets) static SARULI_GATEKEEPERS: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Saruli Gatekeepers",
    "471a5b1d-e2e5-4d90-b72a-ffae81ad6602",
    "Chris Rahn",
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
    CardSet::DragonsMaze,
    "Skylasher",
    "4f4c2069-deb1-4e56-8069-170c4f495944",
    "Dan Murayama Scott",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Insect"], 2, 2).with_abilities(&[
        abilities::flash(),
        abilities::cannot_be_countered(),
        abilities::reach(),
        abilities::protection_from_color(ManaColor::Blue),
    ]),
);

// DGM 50 — Thrashing Mossdog
pub(in crate::card::sets) static THRASHING_MOSSDOG: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Thrashing Mossdog",
    "ffd0d63a-d947-4ce4-8e34-5c1521955b18",
    "Ryan Barger",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Plant", "Dog"], 3, 3).with_abilities(&[
        abilities::reach(),
        abilities::scavenge(
            mana_cost!("{4}{G}{G}"),
            "Scavenge {4}{G}{G} ({4}{G}{G}, Exile this card from your graveyard: Put a number \
             of +1/+1 counters equal to this card's power on target creature. Scavenge only as \
             a sorcery.)",
        ),
    ]),
);

// DGM 51 — Advent of the Wurm
pub(in crate::card::sets) static ADVENT_OF_THE_WURM: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Advent of the Wurm",
    "f40284e6-01a1-4372-a92c-940e5732607e",
    "Lucas Graciano",
    CardRules::new_instant(mana_cost!("{1}{G}{G}{W}")).with_ability(AbilityDef::spell(
        "Create a 5/5 green Wurm creature token with trample.",
        EffectDef::create_creature_token(&["Wurm"], &[ManaColor::Green], 5, 5)
            .with_abilities(&[abilities::trample()])
            .with_art(CardArt::new(
                "33ee3f6c-5df6-4271-b2f9-86b9afffab7b",
                "Anthony Palumbo",
            )),
    )),
);

// DGM 52 — Armored Wolf-Rider
pub(in crate::card::sets) static ARMORED_WOLF_RIDER: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Armored Wolf-Rider",
    "e43d959f-6055-4578-a69a-0ec93e993e21",
    "Matt Stewart",
    CardRules::new_creature(mana_cost!("{3}{G}{W}"), &["Elf", "Knight"], 4, 6),
);

// DGM 53 — Ascended Lawmage
pub(in crate::card::sets) static ASCENDED_LAWMAGE: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Ascended Lawmage",
    "b1f00799-80ce-431e-97bb-8bb4e0e8ba49",
    "Ryan Yee",
    CardRules::new_creature(mana_cost!("{2}{W}{U}"), &["Vedalken", "Wizard"], 3, 2)
        .with_abilities(&[abilities::flying(), abilities::hexproof()]),
);

// DGM 54 — Beetleform Mage
pub(in crate::card::sets) static BEETLEFORM_MAGE: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Beetleform Mage",
    "1e2f7d7f-4097-419b-8de0-b7bf28fc3a4b",
    "Marco Nelor",
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
                    AppliedEffectDef::add_ability(&abilities::flying()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .once_each_turn(),
    ),
);

// DGM 55 — Blast of Genius
// Audit: unsupported — Needs a discard choice whose chosen card's mana value feeds the later damage effect.
pub(in crate::card::sets) static BLAST_OF_GENIUS: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Blast of Genius",
    "b2ff592c-bd35-4947-ba17-8b6170d5388e",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// DGM 56 — Blaze Commando
// Audit: unsupported — Needs a damage event that groups all damage dealt by one instant or sorcery before creating the two tokens.
pub(in crate::card::sets) static BLAZE_COMMANDO: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Blaze Commando",
    "5e179f0d-2965-44e4-8483-67b330a8608c",
    "James Ryman",
    crate::card::CardRules::unsupported(),
);

// DGM 57 — Blood Baron of Vizkopa
pub(in crate::card::sets) static BLOOD_BARON_OF_VIZKOPA: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Blood Baron of Vizkopa",
    "e4edad09-bf7b-40e9-ac2a-100da8a43274",
    "Anthony Palumbo",
    CardRules::new_creature(
        mana_cost!("{3}{W}{B}"),
        &["Vampire"],
        4,
        4,
    )
    .with_abilities(&[
        abilities::lifelink(),
        abilities::protection_from_color(ManaColor::White),
        abilities::protection_from_color(ManaColor::Black),
        AbilityDef::static_ability(
            "As long as you have 30 or more life and an opponent has 10 or less life, this creature gets +6/+6 and has flying.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::All(&[
                    TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::LifeTotal(PlayerRelation::You),
                        comparison: ComparisonDef::GreaterOrEqual,
                        right: ValueDef::Constant(30),
                    }),
                    TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::LifeTotal(PlayerRelation::Opponent),
                        comparison: ComparisonDef::LessOrEqual,
                        right: ValueDef::Constant(10),
                    }),
                ]),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(ValueDef::Constant(6), ValueDef::Constant(6)),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                },
            },
        ),
    ]),
);

// DGM 58 — Boros Battleshaper
// Audit: unsupported — Needs beginning-of-combat targets that impose positive and negative attack-or-block requirements for that combat.
pub(in crate::card::sets) static BOROS_BATTLESHAPER: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Boros Battleshaper",
    "6c43e449-acf2-4e94-b7cf-8c84d70191da",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// DGM 59 — Bred for the Hunt
// Audit: unsupported — Needs a combat-damage source predicate that tests for a +1/+1 counter on the dealing creature.
pub(in crate::card::sets) static BRED_FOR_THE_HUNT: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Bred for the Hunt",
    "4258a536-2275-45e8-8833-e921ca15c5a7",
    "Karl Kopinski",
    crate::card::CardRules::unsupported(),
);

// DGM 60 — Bronzebeak Moa
pub(in crate::card::sets) static BRONZEBEAK_MOA: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Bronzebeak Moa",
    "291c0ebc-d489-42c7-8d8a-9216c333412f",
    "James Ryman",
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
// Audit: unsupported — Needs a blocking event that identifies each blocker and regeneration shields for the activated ability.
pub(in crate::card::sets) static CARNAGE_GLADIATOR: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Carnage Gladiator",
    "20bde6c1-917c-4860-a8d0-a9d7c461f8d2",
    "Ryan Barger",
    crate::card::CardRules::unsupported(),
);

// DGM 62 — Council of the Absolute
// Audit: unsupported — Needs a stored noncreature, nonland card-name choice that both prohibits opponents' matching spells and reduces matching spells you cast.
pub(in crate::card::sets) static COUNCIL_OF_THE_ABSOLUTE: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Council of the Absolute",
    "da18a6a5-0042-40ae-bd33-a6d5a65a9944",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// DGM 63 — Deadbridge Chant
// Audit: unsupported — Needs a random graveyard-card choice followed by a card-type-dependent destination.
pub(in crate::card::sets) static DEADBRIDGE_CHANT: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Deadbridge Chant",
    "26417a58-b0c9-49fa-956c-794ee1c09a4f",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// DGM 64 — Debt to the Deathless
// Audit: unsupported — Needs arithmetic values for twice X and one life-gain event equal to the life actually lost by all opponents.
pub(in crate::card::sets) static DEBT_TO_THE_DEATHLESS: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Debt to the Deathless",
    "610e5a91-857b-4121-8b75-dbbea27aa0aa",
    "Seb McKinnon",
    crate::card::CardRules::unsupported(),
);

// DGM 65 — Deputy of Acquittals
pub(in crate::card::sets) static DEPUTY_OF_ACQUITTALS: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Deputy of Acquittals",
    "4b555888-21b1-4c45-966d-d98f32460d4e",
    "James Ryman",
    CardRules::new_creature(mana_cost!("{W}{U}"), &["Human", "Wizard"], 2, 2).with_abilities(&[
        abilities::flash(),
        abilities::enters_trigger_with_targets("When this creature enters, you may return another target creature you control to its owner's hand.", &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )], EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
},
            }),
    ]),
);

// DGM 66 — Dragonshift
// Audit: unsupported — Needs its targeted and overload programs migrated to one composite type, color, ability, power/toughness, and flying effect.
pub(in crate::card::sets) static DRAGONSHIFT: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Dragonshift",
    "6c046e4e-810c-4123-bb1a-4f97e0cd43d1",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

// DGM 67 — Drown in Filth
static DROWN_IN_FILTH_PENALTY: ValueDef =
    ValueDef::Negate(&ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
        ObjectPredicateDef::HasType(CardType::Land),
        &[ZoneKind::Graveyard],
        PlayerRelation::You,
    )));

pub(in crate::card::sets) static DROWN_IN_FILTH: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Drown in Filth",
    "22feacda-01e0-4f0d-a3c7-a22e3d40bf4e",
    "Seb McKinnon",
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
pub(in crate::card::sets) static EMMARA_TANDRIS: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Emmara Tandris",
    "c7c91a0a-2f14-4131-8ca7-1d0046a8edd2",
    "Mark Winters",
    // All damage, not just combat damage, and only to tokens -- Emmara
    // herself is a card, so she takes hers.
    CardRules::new_creature(mana_cost!("{5}{G}{W}"), &["Elf", "Shaman"], 5, 7)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::static_ability(
            "Prevent all damage that would be dealt to creature tokens you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Token,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                // A shield installed on each token rather than one rule watching the board,
                // so a token that arrives later is covered and one that leaves is not.
                effect: AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(
                    DamageEventMatcherDef {
                        recipient: DamageRecipientMatcherDef::AffectedObject,
                        ..DamageEventMatcherDef::ANY
                    },
                )),
            },
        )),
);

// DGM 69 — Exava, Rakdos Blood Witch
pub(in crate::card::sets) static EXAVA_RAKDOS_BLOOD_WITCH: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Exava, Rakdos Blood Witch",
    "6cb72a64-89e7-4b0e-a3d3-1309829071d2",
    "Aleksi Briclot",
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
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasCounter(CounterKind::PlusOnePlusOne),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                },
            ),
        ]),
);

// DGM 70 — Feral Animist (reprint)
const FERAL_ANIMIST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2006::guildpact::FERAL_ANIMIST,
    "108a9ef2-c74a-450b-8148-4fdf9f09843f",
    "Dave Kendall",
);

// DGM 71 — Fluxcharger
pub(in crate::card::sets) static FLUXCHARGER: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Fluxcharger",
    "7c58f6ed-2544-4b58-8dc0-a0a37b9547e6",
    "Willian Murai",
    // A 1/5 flier that becomes a 5/1 flier on demand, and back again with a
    // second spell: two switches at once cancel.
    CardRules::new_creature(mana_cost!("{2}{U}{R}"), &["Weird"], 1, 5).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever you cast an instant or sorcery spell, you may switch this creature's power and toughness until end of turn.",
            TriggerEventDef::spell_cast(INSTANT_OR_SORCERY_YOU_CAST),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::switch_power_toughness(),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            },
        ),
    ]),
);

// DGM 72 — Gaze of Granite
pub(in crate::card::sets) static GAZE_OF_GRANITE: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Gaze of Granite",
    "96c9ac10-d114-4aa5-87ac-f1069cde8e40",
    "Nils Hamm",
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
            then: None,
        },
    )),
);

// DGM 73 — Gleam of Battle
pub(in crate::card::sets) static GLEAM_OF_BATTLE: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Gleam of Battle",
    "e5f0feef-1a71-4c8c-9fd1-f5cbe718a988",
    "Raymond Swanland",
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
// Audit: unsupported — Needs a uniformly random legal target choice when the activated ability resolves.
pub(in crate::card::sets) static GOBLIN_TEST_PILOT: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Goblin Test Pilot",
    "a8dbb9aa-1bf8-447d-a96c-33e2248bfb01",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

// DGM 75 — Gruul War Chant
pub(in crate::card::sets) static GRUUL_WAR_CHANT: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Gruul War Chant",
    "df383a6a-5eb1-48e8-a5f3-f4731ddb871b",
    "Dave Kendall",
    CardRules::new_enchantment(mana_cost!("{2}{R}{G}")).with_ability(AbilityDef::static_ability(
        "Attacking creatures you control get +1/+0 and have menace.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::Attacking,
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                AppliedEffectDef::add_ability(&abilities::menace()),
            ]),
        },
    )),
);

// DGM 76 — Haunter of Nightveil
pub(in crate::card::sets) static HAUNTER_OF_NIGHTVEIL: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Haunter of Nightveil",
    "438683f5-adfa-42ae-a6fb-c4649a8a30ab",
    "Igor Kieryluk",
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
    CardSet::DragonsMaze,
    "Jelenn Sphinx",
    "533c89eb-d7c6-4945-9689-2f2c0e428b84",
    "Wesley Burt",
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
// Audit: unsupported — Needs removing a +1/+1 counter from a chosen creature, rather than from the ability source, as an activation cost.
pub(in crate::card::sets) static KOROZDA_GORGON: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Korozda Gorgon",
    "7006e5b9-d6a3-43ce-904b-b2ac0fea67e5",
    "Volkan Baǵa",
    crate::card::CardRules::unsupported(),
);

// DGM 79 — Krasis Incubation
// Audit: unsupported — Needs attached-creature attack, block, and activated-ability prohibitions plus returning the Aura as a cost while retaining its former attachment through last-known information.
pub(in crate::card::sets) static KRASIS_INCUBATION: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Krasis Incubation",
    "8da986da-e8ee-4b53-8bbd-9285d0f7f3cb",
    "Marco Nelor",
    crate::card::CardRules::unsupported(),
);

// DGM 80 — Lavinia of the Tenth
// Audit: unsupported — Needs detain's persistent restrictions and a nonland permanent sweep filtered by mana value.
pub(in crate::card::sets) static LAVINIA_OF_THE_TENTH: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Lavinia of the Tenth",
    "813f1967-c048-4e6e-9720-216773fde47e",
    "Willian Murai",
    crate::card::CardRules::unsupported(),
);

// DGM 81 — Legion's Initiative
// Audit: unsupported — Needs a non-choice binding for exactly the creatures exiled together so the installed beginning-of-combat trigger can return and grant haste only to that group.
pub(in crate::card::sets) static LEGION_S_INITIATIVE: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Legion's Initiative",
    "672051a6-d232-4546-842a-369d412c38d2",
    "Jaime Jones",
    crate::card::CardRules::unsupported(),
);

// DGM 82 — Master of Cruelties
// Audit: unsupported — Needs an attack-alone restriction, an unblocked-attacker trigger that sets a player's life total, and suppression of this creature's combat damage.
pub(in crate::card::sets) static MASTER_OF_CRUELTIES: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Master of Cruelties",
    "7b4d8ab5-252c-4727-817d-6f18cbaedd91",
    "Chase Stone",
    crate::card::CardRules::unsupported(),
);

// DGM 83 — Maw of the Obzedat
pub(in crate::card::sets) static MAW_OF_THE_OBZEDAT: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Maw of the Obzedat",
    "cd1131c6-04da-4c4d-ab61-874ac5be7087",
    "Randy Gallegos",
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
// Audit: unsupported — Needs a continuously revealed library top, cast permission from that zone, and copying spells cast from the library with target reselection.
pub(in crate::card::sets) static MELEK_IZZET_PARAGON: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Melek, Izzet Paragon",
    "3e892d86-f443-4846-8049-40ec6b8c22b4",
    "Jason Chan",
    crate::card::CardRules::unsupported(),
);

// DGM 85 — Mirko Vosk, Mind Drinker
// Audit: unsupported — Needs reveal-until-four-matching-cards library traversal and moving the entire revealed group to the graveyard.
pub(in crate::card::sets) static MIRKO_VOSK_MIND_DRINKER: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Mirko Vosk, Mind Drinker",
    "d37cdd3e-4303-4391-aff4-4a543e65a836",
    "Chase Stone",
    crate::card::CardRules::unsupported(),
);

// DGM 86 — Morgue Burst
pub(in crate::card::sets) static MORGUE_BURST: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Morgue Burst",
    "7b3c2909-87ab-4027-9b56-58a2abae3fa3",
    "Raymond Swanland",
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
            EffectDef::WithZoneMoveResult {
                effect: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
                binding: Binding!("returned"),
                then: &EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex(1)),
                    amount: ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                        objects: ObjectSetDef::Binding(Binding!("returned")),
                        select: ObjectValueDef::Power,
                        operation: AggregateOperationDef::Sum,
                    }),
                },
            },
        ),
    ),
);

// DGM 87 — Nivix Cyclops
pub(in crate::card::sets) static NIVIX_CYCLOPS: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Nivix Cyclops",
    "5d7a0e26-8cd4-4f53-8922-93ca28b1879b",
    "Wayne Reynolds",
    CardRules::new_creature(mana_cost!("{1}{U}{R}"), &["Cyclops"], 1, 4).with_abilities(&[
        abilities::defender(),
        AbilityDef::triggered(
            "Whenever you cast an instant or sorcery spell, this creature gets +3/+0 until end of \
             turn and can attack this turn as though it didn't have defender.",
            TriggerEventDef::spell_cast(INSTANT_OR_SORCERY_YOU_CAST),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                // The pump and the permission arrive together and end together, so one
                // Apply carries both. The Cyclops keeps defender throughout; what it gets
                // is leave to ignore it for the turn.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::Rule(AppliedRuleDef::MayAttackDespiteDefender),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DGM 88 — Notion Thief
// Audit: unsupported — Needs a draw-event replacement that recognizes the first draw of each opponent's draw step and redirects every other draw.
pub(in crate::card::sets) static NOTION_THIEF: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Notion Thief",
    "728e660b-ad8b-49d2-a7e5-6588e496519b",
    "Clint Cearley",
    crate::card::CardRules::unsupported(),
);

// DGM 89 — Obzedat's Aid
pub(in crate::card::sets) static OBZEDATS_AID: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Obzedat's Aid",
    "b846ba99-81ba-424a-98eb-f9f69c40f984",
    "Dan Murayama Scott",
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
        },
    )),
);

// DGM 90 — Pilfered Plans
pub(in crate::card::sets) static PILFERED_PLANS: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Pilfered Plans",
    "3475fcc6-ee53-48da-89d2-80685a584e6a",
    "Michael C. Hayes",
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
// Audit: unsupported — Needs a delayed first-main-phase mana effect that lets its controller distribute the countered spell's mana value among any combination of colors.
pub(in crate::card::sets) static PLASM_CAPTURE: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Plasm Capture",
    "0ffe8485-d5fb-47cc-af53-6e0fd062b7a2",
    "Chase Stone",
    crate::card::CardRules::unsupported(),
);

// DGM 92 — Progenitor Mimic
pub(in crate::card::sets) static PROGENITOR_MIMIC: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Progenitor Mimic",
    "3ad76314-b5d5-4353-86aa-e899e0d757a5",
    "Daarken",
    CardRules::new_creature(mana_cost!("{4}{G}{U}"), &["Shapeshifter"], 0, 0).with_ability(
        AbilityDef::replacement(
            "You may have this creature enter as a copy of any creature on the battlefield, except it has \"At the beginning of your upkeep, if this creature isn't a token, create a token that's a copy of this creature.\"",
            crate::card::ReplacementEffectDef::CopyEntering {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                exceptions: CopyExceptionsDef::NONE.with_abilities(&[
                    CopyAbilityDef::Ability(&AbilityDef::triggered_if(
                        "At the beginning of your upkeep, if this creature isn't a token, create a token that's a copy of this creature.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::Upkeep,
                            player: PlayerRelation::You,
                        },
                        &TriggerConditionDef::SourceMatches {
                            object: ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                        },
                        EffectDef::create_token_from_copy(&crate::card::TokenCopyDef {
                            object: &EffectRecipientDef::Source,
                            exceptions: CopyExceptionsDef::NONE,
                        }),
                    )),
                ]),
            },
        ),
    ),
);

// DGM 93 — Putrefy (reprint)
const PUTREFY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2005::ravnica_city_of_guilds::PUTREFY,
    "0d43a0b6-2a5c-4959-96ee-6e570949dfed",
    "Igor Kieryluk",
);

// DGM 94 — Ral Zarek
static RAL_ZAREK_EXTRA_TURN: EffectDef = EffectDef::TakeExtraTurn {
    player: EffectRecipientDef::Controller,
};

static RAL_ZAREK_FLIP_ONE: EffectDef = EffectDef::Randomized {
    likelihood: LikelihoodDef::new(0.5),
    on_success: &RAL_ZAREK_EXTRA_TURN,
    on_failure: &EffectDef::None,
};

static RAL_ZAREK_FLIP_TWO: EffectDef = EffectDef::Randomized {
    likelihood: LikelihoodDef::new(0.5),
    on_success: &EffectDef::Sequence(&[RAL_ZAREK_EXTRA_TURN, RAL_ZAREK_FLIP_ONE]),
    on_failure: &RAL_ZAREK_FLIP_ONE,
};

static RAL_ZAREK_FLIP_THREE: EffectDef = EffectDef::Randomized {
    likelihood: LikelihoodDef::new(0.5),
    on_success: &EffectDef::Sequence(&[RAL_ZAREK_EXTRA_TURN, RAL_ZAREK_FLIP_TWO]),
    on_failure: &RAL_ZAREK_FLIP_TWO,
};

static RAL_ZAREK_FLIP_FOUR: EffectDef = EffectDef::Randomized {
    likelihood: LikelihoodDef::new(0.5),
    on_success: &EffectDef::Sequence(&[RAL_ZAREK_EXTRA_TURN, RAL_ZAREK_FLIP_THREE]),
    on_failure: &RAL_ZAREK_FLIP_THREE,
};

pub(in crate::card::sets) static RAL_ZAREK: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Ral Zarek",
    "fcdbb062-0b0b-4b4c-b4db-dd149f744baa",
    "Eric Deschamps",
    CardRules::new_planeswalker(mana_cost!("{2}{U}{R}"), &["Ral"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "+1: Tap target permanent, then untap another target permanent.",
                &[AbilityCostDef::Loyalty(1)],
                &[
                    AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Any),
                    AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Any).another(),
                ],
                EffectDef::Sequence(&[
                    EffectDef::Tap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::Untap {
                        object: EffectRecipientDef::Target(TargetIndex(1)),
                    },
                ]),
            ),
            AbilityDef::activated_with_targets(
                "−2: Ral Zarek deals 3 damage to any target.",
                &[AbilityCostDef::Loyalty(-2)],
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(3),
                },
            ),
            AbilityDef::activated(
                "−7: Flip five coins. Take an extra turn after this one for each coin that comes up heads.",
                &[AbilityCostDef::Loyalty(-7)],
                EffectDef::Randomized {
                    likelihood: LikelihoodDef::new(0.5),
                    on_success: &EffectDef::Sequence(&[RAL_ZAREK_EXTRA_TURN, RAL_ZAREK_FLIP_FOUR]),
                    on_failure: &RAL_ZAREK_FLIP_FOUR,
                },
            ),
        ]),
);

// DGM 95 — Reap Intellect
// Audit: unsupported — Needs an X-bounded private-hand choice, same-name searches across three zones, exile of every chosen group, and the final shuffle.
pub(in crate::card::sets) static REAP_INTELLECT: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Reap Intellect",
    "c6297df2-c67a-4054-9617-5c6202c76de8",
    "Steven Belledin",
    crate::card::CardRules::unsupported(),
);

// DGM 96 — Render Silent
// Audit: unsupported — Needs a turn-long prohibition on the countered spell's controller casting any spell.
pub(in crate::card::sets) static RENDER_SILENT: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Render Silent",
    "e3f3d6e4-0abe-4042-a7f6-0395683e8582",
    "Matt Stewart",
    crate::card::CardRules::unsupported(),
);

// DGM 97 — Restore the Peace
// Audit: unsupported — Needs per-turn damage history on creatures and a simultaneous return sweep over every creature that dealt damage.
pub(in crate::card::sets) static RESTORE_THE_PEACE: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Restore the Peace",
    "105902f6-99d0-4bee-9dfd-87a92ac04d91",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// DGM 98 — Rot Farm Skeleton
// Audit: unsupported — Needs an executable can't-block restriction and milling cards as an activation cost from the graveyard.
pub(in crate::card::sets) static ROT_FARM_SKELETON: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Rot Farm Skeleton",
    "ef5af2dd-75c7-402c-be9a-3d0d4290520c",
    "Maciej Kuciara",
    crate::card::CardRules::unsupported(),
);

// DGM 99 — Ruric Thar, the Unbowed
pub(in crate::card::sets) static RURIC_THAR_THE_UNBOWED: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Ruric Thar, the Unbowed",
    "84dd3586-7c3b-4f9c-a1eb-7745b75339b0",
    "Tyler Jacobson",
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
            TriggerEventDef::spell_cast(ObjectPredicateDef::NoncreatureSpell),
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
// Audit: unsupported — Needs an X-sized battlefield-entry counter replacement and a hybrid-mana activation restricted to sorcery timing.
pub(in crate::card::sets) static SAVAGEBORN_HYDRA: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Savageborn Hydra",
    "2f2b73cd-6179-4885-9d92-1782d0b492c1",
    "Raymond Swanland",
    crate::card::CardRules::unsupported(),
);

// DGM 101 — Scab-Clan Giant
// Audit: unsupported — Needs a uniformly random legal opponent-creature choice followed by the simultaneous fight damage procedure.
pub(in crate::card::sets) static SCAB_CLAN_GIANT: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Scab-Clan Giant",
    "a8e360ae-4c78-47a9-81d4-1849cfa518b7",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// DGM 102 — Showstopper
pub(in crate::card::sets) static SHOWSTOPPER: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Showstopper",
    "2fd1f68b-3f16-484e-95c9-5cfa8da218c9",
    "Steve Prescott",
    CardRules::new_instant(mana_cost!("{1}{B}{R}")).with_ability(AbilityDef::spell(
        "Until end of turn, creatures you control gain ‘When this creature dies, it deals 2 damage to target creature an opponent controls.’",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You),
            effect: AppliedEffectDef::add_ability(&abilities::dies_trigger_with_targets(
                "When this creature dies, it deals 2 damage to target creature an opponent controls.",
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
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// DGM 103 — Sin Collector
pub(in crate::card::sets) static SIN_COLLECTOR: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Sin Collector",
    "305a3feb-df49-486c-a3b4-ff2721d60019",
    "Mike Bierek",
    CardRules::new_creature(
        mana_cost!("{1}{W}{B}"),
        &["Human", "Cleric"],
        2,
        1,
    )
    .with_abilities(&[abilities::enters_trigger_with_targets("When this creature enters, target opponent reveals their hand. You choose an instant or sorcery card from it and exile that card.", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
        )], EffectDef::Sequence(&abilities::reveal_hand_and_exile_chosen_card(
            PlayerRefDef::Target(TargetIndex::PRIMARY),
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::HasType(CardType::Sorcery),
            ]),
        ))),
    ]),
);

// DGM 104 — Sire of Insanity
pub(in crate::card::sets) static SIRE_OF_INSANITY: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Sire of Insanity",
    "3665cfb7-51b6-4083-8eae-fbd3fa6c3554",
    "Peter Mohrbacher",
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
                then: None,
            },
        ),
    ),
);

// DGM 105 — Species Gorger
pub(in crate::card::sets) static SPECIES_GORGER: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Species Gorger",
    "e0087a98-55cf-4c8b-a180-fb0d9c336eb2",
    "Min Yum",
    CardRules::new_creature(mana_cost!("{3}{G}{U}"), &["Frog", "Beast"], 6, 6).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, return a creature you control to its owner's hand.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Object(ParentBinding),
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
                then: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            }),
        ),
    ),
);

// DGM 106 — Spike Jester
pub(in crate::card::sets) static SPIKE_JESTER: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Spike Jester",
    "cec50499-70d4-4dc1-9cae-abbecfc8e87d",
    "Ryan Barger",
    CardRules::new_creature(mana_cost!("{B}{R}"), &["Goblin", "Warrior"], 3, 1)
        .with_ability(abilities::haste()),
);

// DGM 107 — Tajic, Blade of the Legion
pub(in crate::card::sets) static TAJIC_BLADE_OF_THE_LEGION: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Tajic, Blade of the Legion",
    "be5717c1-338e-446c-aa7e-93e79e4abb72",
    "James Ryman",
    CardRules::new_creature(mana_cost!("{2}{R}{W}"), &["Human", "Soldier"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::indestructible(),
            AbilityDef::triggered(
                "Battalion — Whenever this creature and at least two other creatures attack, this creature gets +5/+5 until end of turn.",
                TriggerEventDef::attacks_in_declaration(ObjectPredicateDef::Source, 3, None),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(5), ValueDef::Constant(5)),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// DGM 108 — Teysa, Envoy of Ghosts
// Audit: unsupported — Needs protection from creatures and a combat-damage trigger that destroys the specific dealing creature before creating a token.
pub(in crate::card::sets) static TEYSA_ENVOY_OF_GHOSTS: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Teysa, Envoy of Ghosts",
    "cbd8183c-6967-4332-b822-02b82c14ef2d",
    "Karla Ortiz",
    crate::card::CardRules::unsupported(),
);

// DGM 109 — Tithe Drinker
pub(in crate::card::sets) static TITHE_DRINKER: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Tithe Drinker",
    "e069aa06-35b0-4af8-89cb-af653708ed32",
    "Slawomir Maniak",
    CardRules::new_creature(mana_cost!("{W}{B}"), &["Vampire"], 2, 1)
        .with_abilities(&[abilities::lifelink(), abilities::extort()]),
);

// DGM 110 — Trostani's Summoner
pub(in crate::card::sets) static TROSTANIS_SUMMONER: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Trostani's Summoner",
    "1921fa4e-2256-4ef1-b2fe-874f9fbbcdf3",
    "Howard Lyon",
    CardRules::new_creature(mana_cost!("{5}{G}{W}"), &["Elf", "Shaman"], 1, 1).with_ability(
        abilities::enters_trigger("When this creature enters, create a 2/2 white Knight creature token with vigilance, a 3/3 green Centaur creature token, and a 4/4 green Rhino creature token with trample.", EffectDef::Sequence(&[
                EffectDef::create_creature_token(&["Knight"], &[ManaColor::White], 2, 2).with_abilities(&[abilities::vigilance()]).with_art(CardArt::new("67d3d039-248a-4eb8-be5c-12959b458fea", "Matt Stewart")),
                EffectDef::create_creature_token(&["Centaur"], &[ManaColor::Green], 3, 3).with_art(CardArt::new("880d5dc1-ceec-4c5f-93c2-c88b7dbfcac2", "Slawomir Maniak")),
                EffectDef::create_creature_token(&["Rhino"], &[ManaColor::Green], 4, 4).with_abilities(&[abilities::trample()]).with_art(CardArt::new("1331008a-ae86-4640-b823-a73be766ac16", "Tomasz Jedruszek")),
            ])),
    ),
);

// DGM 111 — Unflinching Courage
pub(in crate::card::sets) static UNFLINCHING_COURAGE: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Unflinching Courage",
    "35952c24-d728-4ec6-b0d1-b8183a18554a",
    "Mike Bierek",
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
// Audit: unsupported — Needs granting scavenge to graveyard cards with each card's own mana cost and power, plus regeneration shields.
pub(in crate::card::sets) static VAROLZ_THE_SCAR_STRIPED: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Varolz, the Scar-Striped",
    "4c3ae3db-c14a-4ffc-805c-a3a51da9370d",
    "Adam Paquette",
    crate::card::CardRules::unsupported(),
);

// DGM 113 — Viashino Firstblade
pub(in crate::card::sets) static VIASHINO_FIRSTBLADE: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Viashino Firstblade",
    "1cb0c21c-bdf1-478a-9ad8-6c6bda6ffb0f",
    "Matt Stewart",
    CardRules::new_creature(mana_cost!("{1}{R}{W}"), &["Lizard", "Soldier"], 2, 2).with_abilities(
        &[
            abilities::haste(),
            abilities::enters_trigger(
                "When this creature enters, it gets +2/+2 until end of turn.",
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

// DGM 114 — Voice of Resurgence
static VOICE_OF_RESURGENCE_CREATURES_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

static VOICE_OF_RESURGENCE_TOKEN: EffectDef = EffectDef::create_creature_token(
    &["Elemental"],
    &[ManaColor::Green, ManaColor::White],
    0,
    0,
)
.with_abilities(&[AbilityDef::static_ability(
    "This token's power and toughness are each equal to the number of creatures you control.",
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::modify_power_toughness(
            ValueDef::CountMatchingObjects(&VOICE_OF_RESURGENCE_CREATURES_YOU_CONTROL),
            ValueDef::CountMatchingObjects(&VOICE_OF_RESURGENCE_CREATURES_YOU_CONTROL),
        ),
    },
)])
.with_art(CardArt::new(
    "5bfb1440-d4c1-42cf-a777-ee1644dbbac7",
    "Mark Winters",
));

pub(in crate::card::sets) static VOICE_OF_RESURGENCE: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Voice of Resurgence",
    "07246783-d475-4f61-99ac-e2b574072349",
    "Winona Nelson",
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
            TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent)),
            &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
            VOICE_OF_RESURGENCE_TOKEN,
        ),
        abilities::dies_trigger("When this creature dies, create a green and white Elemental creature token with \"This token's power and toughness are each equal to the number of creatures you control.\"", VOICE_OF_RESURGENCE_TOKEN),
    ]),
);

// DGM 115 — Vorel of the Hull Clade
// Audit: unsupported — Needs an effect that doubles every kind of counter on one targeted artifact, creature, or land.
pub(in crate::card::sets) static VOREL_OF_THE_HULL_CLADE: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Vorel of the Hull Clade",
    "db0665d4-d974-4d5e-ba29-7bf40cbbe29c",
    "Mike Bierek",
    crate::card::CardRules::unsupported(),
);

// DGM 116 — Warleader's Helix
pub(in crate::card::sets) static WARLEADERS_HELIX: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Warleader's Helix",
    "81e474ac-54f7-43f9-8af9-2f1adf258b15",
    "Greg Staples",
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
// Audit: unsupported — Needs a current hand-card count value and its negation to drive the temporary +X/-X effect.
pub(in crate::card::sets) static WARPED_PHYSIQUE: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Warped Physique",
    "134802b2-7c5c-4eda-a879-b29bc06faaed",
    "Karl Kopinski",
    crate::card::CardRules::unsupported(),
);

// DGM 118 — Woodlot Crawler
pub(in crate::card::sets) static WOODLOT_CRAWLER: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Woodlot Crawler",
    "11f1e6fe-e959-4030-9925-9ccc27040275",
    "Greg Staples",
    CardRules::new_creature(mana_cost!("{U}{B}"), &["Insect"], 2, 1).with_abilities(&[
        abilities::forestwalk(),
        abilities::protection_from_color(ManaColor::Green),
    ]),
);

// DGM 119 — Zhur-Taa Ancient
// Audit: unsupported — Needs mana-production provenance so the trigger can add one mana of a type the tapped land produced.
pub(in crate::card::sets) static ZHUR_TAA_ANCIENT: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Zhur-Taa Ancient",
    "2076308f-0f4e-4b31-9e75-c2965942e7d1",
    "Adam Paquette",
    crate::card::CardRules::unsupported(),
);

// DGM 120 — Zhur-Taa Druid
pub(in crate::card::sets) static ZHUR_TAA_DRUID: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Zhur-Taa Druid",
    "fd565782-8b2f-4b9f-a62d-4af60af20a82",
    "Mark Winters",
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
// Audit: unsupported — Needs fuse spell composition plus a 3/3 green Centaur token and a creature-count life-gain value multiplied by two.
pub(in crate::card::sets) static ALIVE_WELL: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Alive // Well",
    "db84415e-048a-4cfc-9121-5ae17a412198",
    "Nils Hamm",
    crate::card::CardRules::unsupported(),
);

// DGM 122 — Armed // Dangerous
// Audit: unsupported — Needs fuse spell composition and a turn-long requirement that every creature able to block the Dangerous target does so.
pub(in crate::card::sets) static ARMED_DANGEROUS: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Armed // Dangerous",
    "ff7f4fc2-6f76-44e7-a30b-7166a0d10d2a",
    "David Palumbo",
    crate::card::CardRules::unsupported(),
);

// DGM 123 — Beck // Call
// Audit: unsupported — Needs fuse spell composition plus a temporary enters-the-battlefield listener and a 1/1 white flying Bird token.
pub(in crate::card::sets) static BECK_CALL: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Beck // Call",
    "a01d6540-9eaf-4e08-a62d-682551ee78e9",
    "Adam Paquette",
    crate::card::CardRules::unsupported(),
);

// DGM 124 — Breaking // Entering
// Audit: unsupported — Needs fuse spell composition and a nontarget creature-card choice from either graveyard for Entering.
pub(in crate::card::sets) static BREAKING_ENTERING: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Breaking // Entering",
    "66724f4e-59dd-4c70-b09b-49947320e6d1",
    "Mathias Kollros",
    crate::card::CardRules::unsupported(),
);

// DGM 125 — Catch // Release
// Audit: unsupported — Needs fuse spell composition and one independent permanent choice for each named card type from every player.
pub(in crate::card::sets) static CATCH_RELEASE: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Catch // Release",
    "29968873-56f3-4528-ab0b-f11dd67dd162",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// DGM 126 — Down // Dirty
pub(in crate::card::sets) static DOWN_DIRTY: CardRecord = CardRecord::new_fuse(
    CardSet::DragonsMaze,
    "Down // Dirty",
    "c35c63c1-6344-4d8c-8f7d-cd253d12f9ae",
    "Svetlin Velinov",
    &[
        (
            "Down",
            CardRules::new_sorcery(mana_cost!("{3}{B}")).with_ability(
                AbilityDef::spell_with_targets(
                    "Target player discards two cards.",
                    &ANY_PLAYER_TARGET,
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(2),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                ),
            ),
        ),
        (
            "Dirty",
            CardRules::new_sorcery(mana_cost!("{2}{G}")).with_ability(
                AbilityDef::spell_with_targets(
                    "Return target card from your graveyard to your hand.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::Any,
                            zones: &[ZoneKind::Graveyard],
                            controller: None,
                            owner: Some(PlayerRelation::You),
                        },
                    )],
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                    },
                ),
            ),
        ),
    ],
    mana_cost!("{5}{B}{G}"),
);

// DGM 127 — Far // Away
pub(in crate::card::sets) static FAR_AWAY: CardRecord = CardRecord::new_fuse(
    CardSet::DragonsMaze,
    "Far // Away",
    "d13cdb71-a499-41db-84e6-95f84650c524",
    "Greg Staples",
    &[
        (
            "Far",
            CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(
                AbilityDef::spell_with_targets(
                    "Return target creature to its owner's hand.",
                    &CREATURE_TARGET,
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                    },
                ),
            ),
        ),
        (
            "Away",
            CardRules::new_instant(mana_cost!("{2}{B}")).with_ability(
                AbilityDef::spell_with_targets(
                    "Target player sacrifices a creature of their choice.",
                    &ANY_PLAYER_TARGET,
                    EffectDef::SacrificeOfChoice {
                        count: ValueDef::Constant(1),
                        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        then: None,
                        amount: SacrificedAmountDef::Power,
                        otherwise: None,
                        optional: false,
                    },
                ),
            ),
        ),
    ],
    mana_cost!("{3}{U}{B}"),
);

// DGM 128 — Flesh // Blood
// Audit: unsupported — Needs fuse spell composition and a value carrying the exiled graveyard card's power into Flesh's counter effect.
pub(in crate::card::sets) static FLESH_BLOOD: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Flesh // Blood",
    "02b40fe4-901a-4832-8d52-a6bb5cc07b63",
    "Lucas Graciano",
    crate::card::CardRules::unsupported(),
);

// DGM 129 — Give // Take
// Audit: unsupported — Needs fuse spell composition and removing all +1/+1 counters from the targeted creature while remembering the removed count.
pub(in crate::card::sets) static GIVE_TAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Give // Take",
    "9af07d28-45a2-45d6-b1cb-0858c609a881",
    "Steve Prescott",
    crate::card::CardRules::unsupported(),
);

// DGM 130 — Profit // Loss
pub(in crate::card::sets) static PROFIT_LOSS: CardRecord = CardRecord::new_fuse(
    CardSet::DragonsMaze,
    "Profit // Loss",
    "0eb3ce46-ddd2-43b3-9e45-019ae91df686",
    "Kev Walker",
    &[
        (
            "Profit",
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
            )),
        ),
        (
            "Loss",
            CardRules::new_instant(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell(
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
            )),
        ),
    ],
    mana_cost!("{3}{W}{B}"),
);

// DGM 131 — Protect // Serve
pub(in crate::card::sets) static PROTECT_SERVE: CardRecord = CardRecord::new_fuse(
    CardSet::DragonsMaze,
    "Protect // Serve",
    "9b8acd7d-f3e2-4358-91ab-40901b68d64c",
    "Ryan Barger",
    &[
        (
            "Protect",
            CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(
                AbilityDef::spell_with_targets(
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
                ),
            ),
        ),
        (
            "Serve",
            CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(
                AbilityDef::spell_with_targets(
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
                ),
            ),
        ),
    ],
    mana_cost!("{3}{W}{U}"),
);

// DGM 132 — Ready // Willing
pub(in crate::card::sets) static READY_WILLING: CardRecord = CardRecord::new_fuse(
    CardSet::DragonsMaze,
    "Ready // Willing",
    "22081f95-dc8e-41ed-b609-b6a22ee5428b",
    "Zoltan Boros",
    &[
        (
            "Ready",
            CardRules::new_instant(mana_cost!("{1}{G}{W}")).with_ability(AbilityDef::spell(
                "Creatures you control gain indestructible until end of turn. Untap each creature you control.",
                EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::Untap {
                        object: EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    },
                ]),
            )),
        ),
        (
            "Willing",
            CardRules::new_instant(mana_cost!("{1}{W}{B}")).with_ability(AbilityDef::spell(
                "Creatures you control gain deathtouch and lifelink until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::deathtouch()),
                        AppliedEffectDef::add_ability(&abilities::lifelink()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            )),
        ),
    ],
    mana_cost!("{2}{W}{W}{B}{G}"),
);

// DGM 133 — Toil // Trouble
// Audit: unsupported — Needs fuse spell composition and a value for the targeted player's current hand size.
pub(in crate::card::sets) static TOIL_TROUBLE: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Toil // Trouble",
    "15bb3454-e3bb-4af9-9e93-461e210c26b7",
    "Nils Hamm",
    crate::card::CardRules::unsupported(),
);

// DGM 134 — Turn // Burn
pub(in crate::card::sets) static TURN_BURN: CardRecord = CardRecord::new_fuse(
    CardSet::DragonsMaze,
    "Turn // Burn",
    "8d7fdd59-6d76-4a0c-ac75-816345ef4a39",
    "Ryan Barger",
    &[
        (
            "Turn",
            CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(
                AbilityDef::spell_with_targets(
                    "Until end of turn, target creature loses all abilities and becomes a red Weird with base power and toughness 0/1.\nFuse (You may cast one or both halves of this card from your hand.)",
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
                        // Turn repaints the characteristics it names while leaving the target's
                        // other card types and subtype categories intact.
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                            AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Weird"])),
                            AppliedEffectDef::remove_abilities(crate::card::AbilityPredicateDef::Any),
                            AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Red])),
                            AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(0), ValueDef::Constant(1)),
                        ]),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
            ),
        ),
        (
            "Burn",
            CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(
                AbilityDef::spell_with_targets(
                    "Burn deals 2 damage to any target.\nFuse (You may cast one or both halves of this card from your hand.)",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::AnyTarget,
                    )],
                    EffectDef::DealDamage {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(2),
                    },
                ),
            ),
        ),
    ],
    mana_cost!("{3}{U}{R}"),
);

// DGM 135 — Wear // Tear
pub(in crate::card::sets) static WEAR_TEAR: CardRecord = CardRecord::new_fuse(
    CardSet::DragonsMaze,
    "Wear // Tear",
    "d169a3b2-18ae-4414-98ef-d879676fdcc0",
    "Ryan Pancoast",
    &[
        (
            "Wear",
            CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::destroy_target(
                "Destroy target artifact.",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Artifact,
                )),
                true,
            )),
        ),
        (
            "Tear",
            CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::destroy_target(
                "Destroy target enchantment.",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Enchantment,
                )),
                true,
            )),
        ),
    ],
    mana_cost!("{1}{R}{W}"),
);

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

// DGM 136 — Azorius Cluestone
pub(in crate::card::sets) static AZORIUS_CLUESTONE: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Azorius Cluestone",
    "09eeb301-bc28-4515-ad69-0b1b5164a5bc",
    "Raoul Vitale",
    cluestone_rules(&AZORIUS_CLUESTONE_ABILITIES),
);

// DGM 137 — Boros Cluestone
pub(in crate::card::sets) static BOROS_CLUESTONE: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Boros Cluestone",
    "87252577-3e7b-4ea2-b0ac-3ba3f0eaac40",
    "Raoul Vitale",
    cluestone_rules(&BOROS_CLUESTONE_ABILITIES),
);

// DGM 138 — Dimir Cluestone
pub(in crate::card::sets) static DIMIR_CLUESTONE: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Dimir Cluestone",
    "0d8ac24f-3309-453a-b2d6-6363df9a1ddd",
    "Raoul Vitale",
    cluestone_rules(&DIMIR_CLUESTONE_ABILITIES),
);

// DGM 139 — Golgari Cluestone
pub(in crate::card::sets) static GOLGARI_CLUESTONE: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Golgari Cluestone",
    "ff77e1ee-7fa3-4370-a0c9-ec008b63302f",
    "Raoul Vitale",
    cluestone_rules(&GOLGARI_CLUESTONE_ABILITIES),
);

// DGM 140 — Gruul Cluestone
pub(in crate::card::sets) static GRUUL_CLUESTONE: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Gruul Cluestone",
    "bc47d1fe-8ab2-42f6-bcab-4bc2084ceba7",
    "Raoul Vitale",
    cluestone_rules(&GRUUL_CLUESTONE_ABILITIES),
);

// DGM 141 — Izzet Cluestone
pub(in crate::card::sets) static IZZET_CLUESTONE: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Izzet Cluestone",
    "8cf63def-e2cc-48c7-8409-c08a36eddf93",
    "Raoul Vitale",
    cluestone_rules(&IZZET_CLUESTONE_ABILITIES),
);

// DGM 142 — Orzhov Cluestone
pub(in crate::card::sets) static ORZHOV_CLUESTONE: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Orzhov Cluestone",
    "4823f904-1c41-42cf-aef7-db0dcf82b10b",
    "Raoul Vitale",
    cluestone_rules(&ORZHOV_CLUESTONE_ABILITIES),
);

// DGM 143 — Rakdos Cluestone
pub(in crate::card::sets) static RAKDOS_CLUESTONE: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Rakdos Cluestone",
    "9ef43817-1813-4608-8e3d-3c14321ab736",
    "Raoul Vitale",
    cluestone_rules(&RAKDOS_CLUESTONE_ABILITIES),
);

// DGM 144 — Selesnya Cluestone
pub(in crate::card::sets) static SELESNYA_CLUESTONE: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Selesnya Cluestone",
    "34ad5631-439a-43e2-b00a-04f78d66b8e6",
    "Raoul Vitale",
    cluestone_rules(&SELESNYA_CLUESTONE_ABILITIES),
);

// DGM 145 — Simic Cluestone
pub(in crate::card::sets) static SIMIC_CLUESTONE: CardRecord = CardRecord::new(
    CardSet::DragonsMaze,
    "Simic Cluestone",
    "e3c47552-afed-463d-bd24-13eb1cd724fc",
    "Raoul Vitale",
    cluestone_rules(&SIMIC_CLUESTONE_ABILITIES),
);

// DGM 146 — Azorius Guildgate (reprint)
const AZORIUS_GUILDGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &return_to_ravnica::AZORIUS_GUILDGATE,
    "65bbe27d-c92a-4c65-a997-b21536d7667e",
    "Drew Baker",
);

// DGM 147 — Boros Guildgate (reprint)
const BOROS_GUILDGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &gatecrash::BOROS_GUILDGATE,
    "e94cc167-a6da-4404-88aa-61eee8b4b9e8",
    "Noah Bradley",
);

// DGM 148 — Dimir Guildgate (reprint)
const DIMIR_GUILDGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &gatecrash::DIMIR_GUILDGATE,
    "9627f9eb-c3fc-4517-9d65-132fdcc217d7",
    "Cliff Childs",
);

// DGM 149 — Golgari Guildgate (reprint)
const GOLGARI_GUILDGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &return_to_ravnica::GOLGARI_GUILDGATE,
    "0248cc88-e95c-4667-82a2-40e881acabc2",
    "Eytan Zana",
);

// DGM 150 — Gruul Guildgate (reprint)
const GRUUL_GUILDGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &gatecrash::GRUUL_GUILDGATE,
    "24221b9a-5ff1-43f8-b409-c56967f8308d",
    "Randy Gallegos",
);

// DGM 151 — Izzet Guildgate (reprint)
const IZZET_GUILDGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &return_to_ravnica::IZZET_GUILDGATE,
    "165ee8d7-d509-41d4-abd2-298b3db3ca46",
    "Noah Bradley",
);

// DGM 152 — Maze's End
// Audit: unsupported — Needs returning the land as an activation cost, a Gate-specific library search to the battlefield, and the ten-distinct-names win condition.
pub(in crate::card::sets) static MAZE_S_END: CardRecord = CardRecord::new(
    crate::card::CardSet::DragonsMaze,
    "Maze's End",
    "401f7042-24fd-42a0-ae7c-e6b7de1aa446",
    "Cliff Childs",
    crate::card::CardRules::unsupported(),
);

// DGM 153 — Orzhov Guildgate (reprint)
const ORZHOV_GUILDGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &gatecrash::ORZHOV_GUILDGATE,
    "8f4e2006-5bff-4e91-862b-aa76521a99c3",
    "John Avon",
);

// DGM 154 — Rakdos Guildgate (reprint)
const RAKDOS_GUILDGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &return_to_ravnica::RAKDOS_GUILDGATE,
    "1368e7c6-2220-4dad-8129-68336f261af0",
    "Eytan Zana",
);

// DGM 155 — Selesnya Guildgate (reprint)
const SELESNYA_GUILDGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &return_to_ravnica::SELESNYA_GUILDGATE,
    "90198725-0cd3-4650-9575-c22674aa4185",
    "Howard Lyon",
);

// DGM 156 — Simic Guildgate (reprint)
const SIMIC_GUILDGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &gatecrash::SIMIC_GUILDGATE,
    "fee34fcb-0158-4741-9292-513fed9684cb",
    "Svetlin Velinov",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BOROS_MASTIFF,
    &HAAZDA_SNARE_SQUAD,
    &LYEV_DECREE,
    &MAZE_SENTINEL,
    &RENOUNCE_THE_GUILDS,
    &RIOT_CONTROL,
    &SCION_OF_VITU_GHAZI,
    &STEEPLE_ROC,
    &SUNSPIRE_GATEKEEPERS,
    &WAKE_THE_REFLECTIONS,
    &AETHERLING,
    &HIDDEN_STRINGS,
    &MAZE_GLIDER,
    &MINDSTATIC,
    &MURMURING_PHANTASM,
    &OPAL_LAKE_GATEKEEPERS,
    &RUNNERS_BANE,
    &TRAIT_DOCTORING,
    &UNCOVERED_CLUES,
    &BANE_ALLEY_BLACKGUARD,
    &BLOOD_SCRIVENER,
    &CRYPT_INCURSION,
    &FATAL_FUMES,
    &HIRED_TORTURER,
    &MAZE_ABOMINATION,
    &PONTIFF_OF_BLIGHT,
    &RAKDOS_DRAKE,
    &SINISTER_POSSESSION,
    &UBUL_SAR_GATEKEEPERS,
    &AWE_FOR_THE_GUILDS,
    &CLEAR_A_PATH,
    &MAZE_RUSHER,
    &POSSIBILITY_STORM,
    &PUNISH_THE_ENEMY,
    &PYREWILD_SHAMAN,
    &RIOT_PIKER,
    &RUBBLEBELT_MAAKA,
    &SMELT_WARD_GATEKEEPERS,
    &WEAPON_SURGE,
    &BATTERING_KRASIS,
    &KRAUL_WARRIOR,
    &MAZE_BEHEMOTH,
    &MENDING_TOUCH,
    &MUTANT_S_PREY,
    &PHYTOBURST,
    &RENEGADE_KRASIS,
    &SARULI_GATEKEEPERS,
    &SKYLASHER,
    &THRASHING_MOSSDOG,
    &ADVENT_OF_THE_WURM,
    &ARMORED_WOLF_RIDER,
    &ASCENDED_LAWMAGE,
    &BEETLEFORM_MAGE,
    &BLAST_OF_GENIUS,
    &BLAZE_COMMANDO,
    &BLOOD_BARON_OF_VIZKOPA,
    &BOROS_BATTLESHAPER,
    &BRED_FOR_THE_HUNT,
    &BRONZEBEAK_MOA,
    &CARNAGE_GLADIATOR,
    &COUNCIL_OF_THE_ABSOLUTE,
    &DEADBRIDGE_CHANT,
    &DEBT_TO_THE_DEATHLESS,
    &DEPUTY_OF_ACQUITTALS,
    &DRAGONSHIFT,
    &DROWN_IN_FILTH,
    &EMMARA_TANDRIS,
    &EXAVA_RAKDOS_BLOOD_WITCH,
    &FLUXCHARGER,
    &GAZE_OF_GRANITE,
    &GLEAM_OF_BATTLE,
    &GOBLIN_TEST_PILOT,
    &GRUUL_WAR_CHANT,
    &HAUNTER_OF_NIGHTVEIL,
    &JELENN_SPHINX,
    &KOROZDA_GORGON,
    &KRASIS_INCUBATION,
    &LAVINIA_OF_THE_TENTH,
    &LEGION_S_INITIATIVE,
    &MASTER_OF_CRUELTIES,
    &MAW_OF_THE_OBZEDAT,
    &MELEK_IZZET_PARAGON,
    &MIRKO_VOSK_MIND_DRINKER,
    &MORGUE_BURST,
    &NIVIX_CYCLOPS,
    &NOTION_THIEF,
    &OBZEDATS_AID,
    &PILFERED_PLANS,
    &PLASM_CAPTURE,
    &PROGENITOR_MIMIC,
    &RAL_ZAREK,
    &REAP_INTELLECT,
    &RENDER_SILENT,
    &RESTORE_THE_PEACE,
    &ROT_FARM_SKELETON,
    &RURIC_THAR_THE_UNBOWED,
    &SAVAGEBORN_HYDRA,
    &SCAB_CLAN_GIANT,
    &SHOWSTOPPER,
    &SIN_COLLECTOR,
    &SIRE_OF_INSANITY,
    &SPECIES_GORGER,
    &SPIKE_JESTER,
    &TAJIC_BLADE_OF_THE_LEGION,
    &TEYSA_ENVOY_OF_GHOSTS,
    &TITHE_DRINKER,
    &TROSTANIS_SUMMONER,
    &UNFLINCHING_COURAGE,
    &VAROLZ_THE_SCAR_STRIPED,
    &VIASHINO_FIRSTBLADE,
    &VOICE_OF_RESURGENCE,
    &VOREL_OF_THE_HULL_CLADE,
    &WARLEADERS_HELIX,
    &WARPED_PHYSIQUE,
    &WOODLOT_CRAWLER,
    &ZHUR_TAA_ANCIENT,
    &ZHUR_TAA_DRUID,
    &ALIVE_WELL,
    &ARMED_DANGEROUS,
    &BECK_CALL,
    &BREAKING_ENTERING,
    &CATCH_RELEASE,
    &DOWN_DIRTY,
    &FAR_AWAY,
    &FLESH_BLOOD,
    &GIVE_TAKE,
    &PROFIT_LOSS,
    &PROTECT_SERVE,
    &READY_WILLING,
    &TOIL_TROUBLE,
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
    &MAZE_S_END,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    WIND_DRAKE_REPRINT,
    FERAL_ANIMIST_REPRINT,
    PUTREFY_REPRINT,
    AZORIUS_GUILDGATE_REPRINT,
    BOROS_GUILDGATE_REPRINT,
    DIMIR_GUILDGATE_REPRINT,
    GOLGARI_GUILDGATE_REPRINT,
    GRUUL_GUILDGATE_REPRINT,
    IZZET_GUILDGATE_REPRINT,
    ORZHOV_GUILDGATE_REPRINT,
    RAKDOS_GUILDGATE_REPRINT,
    SELESNYA_GUILDGATE_REPRINT,
    SIMIC_GUILDGATE_REPRINT,
];
