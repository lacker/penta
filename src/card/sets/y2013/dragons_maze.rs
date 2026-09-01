//! Dragon's Maze card records used by the built-in ISD–M14 Standard decks.

use super::{CardRecord, PrintingAnchor, PrintingRecord, gatecrash};
use crate::card::sets::y2012::return_to_ravnica;
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    CardTypeSet, ChoiceVisibilityDef, ChooseDef, ColorSet, ComparisonDef, ControlDurationDef,
    CopyAbilityDef, CopyExceptionsDef, CounterKind, CreatureTypeSetDef, DamageEventMatcherDef,
    DamagePreventionDef, DamageRecipientMatcherDef, DiscardSelectionDef, EffectDef,
    EffectPaymentDef, EffectRecipientDef, LikelihoodDef, ManaColor, ObjectChoiceBindingDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, PayOrDef, PlayerRefDef,
    PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef, SacrificedAmountDef,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueComparisonDef, ValueDef, ZoneKind,
    ZonePlacement, abilities,
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
pub(in crate::card::sets) static BOROS_MASTIFF: CardRecord = CardRecord::new_with_legacy_id(
    607,
    "Boros Mastiff",
    CardArt::new("27a3bfb6-3843-4bda-bbcb-905e4b351dea", "Kev Walker"),
    CardSet::DragonsMaze,
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
pub(in crate::card::sets) static HAAZDA_SNARE_SQUAD: CardRecord = CardRecord::new_with_legacy_id(
    608,
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
pub(in crate::card::sets) static LYEV_DECREE: CardRecord = CardRecord::new_with_legacy_id(
    1543,
    "Lyev Decree",
    CardArt::new("773cf2aa-4337-4d14-8a8e-ff8b1fdec1b5", "Kev Walker"),
    CardSet::DragonsMaze,
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
pub(in crate::card::sets) static MAZE_SENTINEL: CardRecord = CardRecord::new_with_legacy_id(
    609,
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
pub(in crate::card::sets) static RENOUNCE_THE_GUILDS: CardRecord = CardRecord::new_with_legacy_id(
    610,
    "Renounce the Guilds",
    CardArt::new("bc9acc14-24e0-4c03-a09a-2afee351f2cc", "Daarken"),
    CardSet::DragonsMaze,
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
pub(in crate::card::sets) static RIOT_CONTROL: CardRecord = CardRecord::new_with_legacy_id(
    1499,
    "Riot Control",
    CardArt::new("d7886607-86db-4221-8752-296104aaaef2", "Slawomir Maniak"),
    CardSet::DragonsMaze,
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
// Audit: metadata-only — Needs an enters-trigger condition that remembers whether the permanent was cast from hand, plus populate's token-copy choice.
pub(in crate::card::sets) static SCION_OF_VITU_GHAZI: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3cd20865-0a9a-4a72-92f9-77c8d6384b46"),
    "Scion of Vitu-Ghazi",
    crate::card::CardArt::new("3cd20865-0a9a-4a72-92f9-77c8d6384b46", "Willian Murai"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 8 — Steeple Roc
pub(in crate::card::sets) static STEEPLE_ROC: CardRecord = CardRecord::new_with_legacy_id(
    611,
    "Steeple Roc",
    CardArt::new("5fecafab-97f4-40ed-bc43-d186eb2f3af6", "David Palumbo"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Bird"], 3, 1)
        .with_abilities(&[abilities::flying(), abilities::first_strike()]),
);

// DGM 9 — Sunspire Gatekeepers
pub(in crate::card::sets) static SUNSPIRE_GATEKEEPERS: CardRecord = CardRecord::new_with_legacy_id(
    612,
    "Sunspire Gatekeepers",
    CardArt::new("0a3bc6b9-475b-4257-a3bc-1a0b70d45f79", "Chippy"),
    CardSet::DragonsMaze,
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
pub(in crate::card::sets) static WAKE_THE_REFLECTIONS: CardRecord = CardRecord::new_with_legacy_id(
    1863,
    "Wake the Reflections",
    CardArt::new("3db0074c-95cf-4d15-8fe1-7282803ec757", "Cynthia Sheppard"),
    CardSet::DragonsMaze,
    // Populate and nothing else, so a board with no creature token makes
    // this a blank rather than an illegal cast.
    CardRules::new_sorcery(mana_cost!("{W}"))
        .with_ability(AbilityDef::spell("Populate.", abilities::populate())),
);

// DGM 11 — Aetherling
pub(in crate::card::sets) static AETHERLING: CardRecord = CardRecord::new_with_legacy_id(
    130,
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
// Audit: metadata-only — Needs tap-or-untap choices made independently on resolution and cipher's encoded-card link, combat-damage trigger, and free-copy casting permission.
pub(in crate::card::sets) static HIDDEN_STRINGS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("216e8047-6f54-49ce-bf86-27dc8fc8c8f7"),
    "Hidden Strings",
    crate::card::CardArt::new("216e8047-6f54-49ce-bf86-27dc8fc8c8f7", "Daarken"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 13 — Maze Glider
pub(in crate::card::sets) static MAZE_GLIDER: CardRecord = CardRecord::new_with_legacy_id(
    613,
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
pub(in crate::card::sets) static MINDSTATIC: CardRecord = CardRecord::new_with_legacy_id(
    614,
    "Mindstatic",
    CardArt::new("55d3fad5-a12a-4b41-9c7b-c1af5e0b5ca8", "Johann Bodin"),
    CardSet::DragonsMaze,
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
pub(in crate::card::sets) static MURMURING_PHANTASM: CardRecord = CardRecord::new_with_legacy_id(
    615,
    "Murmuring Phantasm",
    CardArt::new("9752644c-7c43-429e-a79c-1239b9a0bc8a", "Peter Mohrbacher"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Spirit"], 0, 5)
        .with_ability(abilities::defender()),
);

// DGM 16 — Opal Lake Gatekeepers
pub(in crate::card::sets) static OPAL_LAKE_GATEKEEPERS: CardRecord = CardRecord::new_with_legacy_id(
    616,
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
pub(in crate::card::sets) static RUNNERS_BANE: CardRecord = CardRecord::new_with_legacy_id(
    617,
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
            abilities::enters_trigger("When this Aura enters, tap enchanted creature.", EffectDef::Tap {
                    object: EffectRecipientDef::AttachedPermanent,
                }),
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
// Audit: metadata-only — Needs duration-scoped color-word text changes and cipher's encoded-card link, combat-damage trigger, and free-copy casting permission.
pub(in crate::card::sets) static TRAIT_DOCTORING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e21a7981-5940-4b75-907f-7600a742f946"),
    "Trait Doctoring",
    crate::card::CardArt::new("e21a7981-5940-4b75-907f-7600a742f946", "Clint Cearley"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 19 — Uncovered Clues
// Audit: metadata-only — Needs a top-four selection constrained to up to two instant or sorcery followed by ordering the unselected cards on the library bottom.
pub(in crate::card::sets) static UNCOVERED_CLUES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9dd24556-994f-4480-835e-11d4443f0700"),
    "Uncovered Clues",
    crate::card::CardArt::new("9dd24556-994f-4480-835e-11d4443f0700", "Jaime Jones"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 20 — Wind Drake (reprint)

// DGM 21 — Bane Alley Blackguard
pub(in crate::card::sets) static BANE_ALLEY_BLACKGUARD: CardRecord = CardRecord::new_with_legacy_id(
    619,
    "Bane Alley Blackguard",
    CardArt::new("15fcad03-4567-4f96-976e-01a07d8ab050", "Mike Bierek"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Rogue"], 1, 3),
);

// DGM 22 — Blood Scrivener
// Audit: metadata-only — Needs a draw-event replacement that checks an empty hand and replaces one draw with two cards plus one life loss.
pub(in crate::card::sets) static BLOOD_SCRIVENER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9ea8179a-d3c9-4cdc-a5b5-68cc73279050"),
    "Blood Scrivener",
    crate::card::CardArt::new("9ea8179a-d3c9-4cdc-a5b5-68cc73279050", "Peter Mohrbacher"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 23 — Crypt Incursion
// Audit: metadata-only — Needs the number of cards actually exiled by a graveyard sweep to feed one life-gain event after replacements are applied.
pub(in crate::card::sets) static CRYPT_INCURSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c3b71cc5-0a81-4cab-bae3-49335c04aaaa"),
    "Crypt Incursion",
    crate::card::CardArt::new("c3b71cc5-0a81-4cab-bae3-49335c04aaaa", "Svetlin Velinov"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 24 — Fatal Fumes
pub(in crate::card::sets) static FATAL_FUMES: CardRecord = CardRecord::new_with_legacy_id(
    620,
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
// Audit: metadata-only — Needs revealing a random card from the targeted opponent's hand after the life-loss effect.
pub(in crate::card::sets) static HIRED_TORTURER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("62e9f79e-6606-4c9b-838c-eda5d8cc612c"),
    "Hired Torturer",
    crate::card::CardArt::new("62e9f79e-6606-4c9b-838c-eda5d8cc612c", "Winona Nelson"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 26 — Maze Abomination
pub(in crate::card::sets) static MAZE_ABOMINATION: CardRecord = CardRecord::new_with_legacy_id(
    621,
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
pub(in crate::card::sets) static PONTIFF_OF_BLIGHT: CardRecord = CardRecord::new_with_legacy_id(
    1896,
    "Pontiff of Blight",
    CardArt::new("72e5291f-9281-4cb7-9158-54b7cb336b93", "Seb McKinnon"),
    CardSet::DragonsMaze,
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
pub(in crate::card::sets) static RAKDOS_DRAKE: CardRecord = CardRecord::new_with_legacy_id(
    1627,
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
// Audit: metadata-only — Needs an Aura to observe both attack and block events from its attached creature and make that creature's controller lose life.
pub(in crate::card::sets) static SINISTER_POSSESSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f54c15b-fec0-49a6-8a49-d1af4eeee40e"),
    "Sinister Possession",
    crate::card::CardArt::new("7f54c15b-fec0-49a6-8a49-d1af4eeee40e", "Anthony Palumbo"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 30 — Ubul Sar Gatekeepers
pub(in crate::card::sets) static UBUL_SAR_GATEKEEPERS: CardRecord = CardRecord::new_with_legacy_id(
    622,
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
pub(in crate::card::sets) static AWE_FOR_THE_GUILDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ec644ac3-07a2-43de-8173-9cc18e2ea2d9"),
    "Awe for the Guilds",
    crate::card::CardArt::new("ec644ac3-07a2-43de-8173-9cc18e2ea2d9", "Mathias Kollros"),
    crate::card::CardSet::DragonsMaze,
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
pub(in crate::card::sets) static CLEAR_A_PATH: CardRecord = CardRecord::new_with_legacy_id(
    623,
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
pub(in crate::card::sets) static MAZE_RUSHER: CardRecord = CardRecord::new_with_legacy_id(
    624,
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
// Audit: metadata-only — Needs spell-type-aware library reveal-until, free casting of the found card, and random ordering of the linked exiled cards.
pub(in crate::card::sets) static POSSIBILITY_STORM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("858aa831-b491-4f1e-bb56-33eeca14771d"),
    "Possibility Storm",
    crate::card::CardArt::new("858aa831-b491-4f1e-bb56-33eeca14771d", "Jason Felix"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 35 — Punish the Enemy
pub(in crate::card::sets) static PUNISH_THE_ENEMY: CardRecord = CardRecord::new_with_legacy_id(
    625,
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
pub(in crate::card::sets) static PYREWILD_SHAMAN: CardRecord = CardRecord::new_with_legacy_id(
    626,
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
            "The combined graveyard payment continuation is unavailable: the clause asks its controller to pay {3} while this card sits in a graveyard, and a trigger has no way to offer that payment from there.",
        ),
    ]),
);

// DGM 37 — Riot Piker
pub(in crate::card::sets) static RIOT_PIKER: CardRecord = CardRecord::new_with_legacy_id(
    627,
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
pub(in crate::card::sets) static RUBBLEBELT_MAAKA: CardRecord = CardRecord::new_with_legacy_id(
    628,
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
pub(in crate::card::sets) static SMELT_WARD_GATEKEEPERS: CardRecord = CardRecord::new_with_legacy_id(
    629,
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

pub(in crate::card::sets) static WEAPON_SURGE: CardRecord = CardRecord::new_with_legacy_id(
    630,
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
pub(in crate::card::sets) static BATTERING_KRASIS: CardRecord = CardRecord::new_with_legacy_id(
    1897,
    "Battering Krasis",
    CardArt::new("5d9aa740-9adf-412a-b6ec-0b9bb1b4618b", "Jack Wang"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Shark", "Beast"], 2, 1)
        .with_abilities(&[abilities::trample(), abilities::evolve()]),
);

// DGM 42 — Kraul Warrior
pub(in crate::card::sets) static KRAUL_WARRIOR: CardRecord = CardRecord::new_with_legacy_id(
    631,
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
pub(in crate::card::sets) static MAZE_BEHEMOTH: CardRecord = CardRecord::new_with_legacy_id(
    632,
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
pub(in crate::card::sets) static MENDING_TOUCH: CardRecord = CardRecord::new_with_legacy_id(
    1495,
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
// Audit: metadata-only — Needs a target predicate for a +1/+1 counter and the simultaneous fight damage procedure.
pub(in crate::card::sets) static MUTANT_S_PREY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d9e32d47-2796-4eac-b373-a93506d8d6b7"),
    "Mutant's Prey",
    crate::card::CardArt::new("d9e32d47-2796-4eac-b373-a93506d8d6b7", "Ryan Barger"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 46 — Phytoburst
pub(in crate::card::sets) static PHYTOBURST: CardRecord = CardRecord::new_with_legacy_id(
    633,
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
// Audit: metadata-only — Needs evolve's characteristic comparison and an evolve event that can drive the counter sweep.
pub(in crate::card::sets) static RENEGADE_KRASIS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("23b68921-0c34-4d92-83c3-21542f62c7f6"),
    "Renegade Krasis",
    crate::card::CardArt::new("23b68921-0c34-4d92-83c3-21542f62c7f6", "Howard Lyon"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 48 — Saruli Gatekeepers
pub(in crate::card::sets) static SARULI_GATEKEEPERS: CardRecord = CardRecord::new_with_legacy_id(
    634,
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
pub(in crate::card::sets) static SKYLASHER: CardRecord = CardRecord::new_with_legacy_id(
    635,
    "Skylasher",
    CardArt::new("4f4c2069-deb1-4e56-8069-170c4f495944", "Dan Murayama Scott"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Insect"], 2, 2).with_abilities(&[
        abilities::flash(),
        abilities::cannot_be_countered(),
        abilities::reach(),
        abilities::protection_from_color(ManaColor::Blue),
    ]),
);

// DGM 50 — Thrashing Mossdog
pub(in crate::card::sets) static THRASHING_MOSSDOG: CardRecord = CardRecord::new_with_legacy_id(
    1869,
    "Thrashing Mossdog",
    CardArt::new("ffd0d63a-d947-4ce4-8e34-5c1521955b18", "Ryan Barger"),
    CardSet::DragonsMaze,
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
pub(in crate::card::sets) static ADVENT_OF_THE_WURM: CardRecord = CardRecord::new_with_legacy_id(
    636,
    "Advent of the Wurm",
    CardArt::new("f40284e6-01a1-4372-a92c-940e5732607e", "Lucas Graciano"),
    CardSet::DragonsMaze,
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
pub(in crate::card::sets) static ARMORED_WOLF_RIDER: CardRecord = CardRecord::new_with_legacy_id(
    637,
    "Armored Wolf-Rider",
    CardArt::new("e43d959f-6055-4578-a69a-0ec93e993e21", "Matt Stewart"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{3}{G}{W}"), &["Elf", "Knight"], 4, 6),
);

// DGM 53 — Ascended Lawmage
pub(in crate::card::sets) static ASCENDED_LAWMAGE: CardRecord = CardRecord::new_with_legacy_id(
    638,
    "Ascended Lawmage",
    CardArt::new("b1f00799-80ce-431e-97bb-8bb4e0e8ba49", "Ryan Yee"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{2}{W}{U}"), &["Vedalken", "Wizard"], 3, 2)
        .with_abilities(&[abilities::flying(), abilities::hexproof()]),
);

// DGM 54 — Beetleform Mage
pub(in crate::card::sets) static BEETLEFORM_MAGE: CardRecord = CardRecord::new_with_legacy_id(
    1661,
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
                    AppliedEffectDef::add_ability(&abilities::flying()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .once_each_turn(),
    ),
);

// DGM 55 — Blast of Genius
// Audit: metadata-only — Needs a discard choice whose chosen card's mana value feeds the later damage effect.
pub(in crate::card::sets) static BLAST_OF_GENIUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b2ff592c-bd35-4947-ba17-8b6170d5388e"),
    "Blast of Genius",
    crate::card::CardArt::new("b2ff592c-bd35-4947-ba17-8b6170d5388e", "Terese Nielsen"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 56 — Blaze Commando
// Audit: metadata-only — Needs a damage event that groups all damage dealt by one instant or sorcery before creating the two tokens.
pub(in crate::card::sets) static BLAZE_COMMANDO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5e179f0d-2965-44e4-8483-67b330a8608c"),
    "Blaze Commando",
    crate::card::CardArt::new("5e179f0d-2965-44e4-8483-67b330a8608c", "James Ryman"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 57 — Blood Baron of Vizkopa
pub(in crate::card::sets) static BLOOD_BARON_OF_VIZKOPA: CardRecord = CardRecord::new_with_legacy_id(
    142,
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
// Audit: metadata-only — Needs beginning-of-combat targets that impose positive and negative attack-or-block requirements for that combat.
pub(in crate::card::sets) static BOROS_BATTLESHAPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6c43e449-acf2-4e94-b7cf-8c84d70191da"),
    "Boros Battleshaper",
    crate::card::CardArt::new("6c43e449-acf2-4e94-b7cf-8c84d70191da", "Zoltan Boros"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 59 — Bred for the Hunt
// Audit: metadata-only — Needs a combat-damage source predicate that tests for a +1/+1 counter on the dealing creature.
pub(in crate::card::sets) static BRED_FOR_THE_HUNT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4258a536-2275-45e8-8833-e921ca15c5a7"),
    "Bred for the Hunt",
    crate::card::CardArt::new("4258a536-2275-45e8-8833-e921ca15c5a7", "Karl Kopinski"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 60 — Bronzebeak Moa
pub(in crate::card::sets) static BRONZEBEAK_MOA: CardRecord = CardRecord::new_with_legacy_id(
    639,
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
// Audit: metadata-only — Needs a blocking event that identifies each blocker and regeneration shields for the activated ability.
pub(in crate::card::sets) static CARNAGE_GLADIATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("20bde6c1-917c-4860-a8d0-a9d7c461f8d2"),
    "Carnage Gladiator",
    crate::card::CardArt::new("20bde6c1-917c-4860-a8d0-a9d7c461f8d2", "Ryan Barger"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 62 — Council of the Absolute
// Audit: metadata-only — Needs a stored noncreature, nonland card-name choice that both prohibits opponents' matching spells and reduces matching spells you cast.
pub(in crate::card::sets) static COUNCIL_OF_THE_ABSOLUTE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("da18a6a5-0042-40ae-bd33-a6d5a65a9944"),
    "Council of the Absolute",
    crate::card::CardArt::new("da18a6a5-0042-40ae-bd33-a6d5a65a9944", "Zoltan Boros"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 63 — Deadbridge Chant
// Audit: metadata-only — Needs a random graveyard-card choice followed by a card-type-dependent destination.
pub(in crate::card::sets) static DEADBRIDGE_CHANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("26417a58-b0c9-49fa-956c-794ee1c09a4f"),
    "Deadbridge Chant",
    crate::card::CardArt::new("26417a58-b0c9-49fa-956c-794ee1c09a4f", "Zoltan Boros"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 64 — Debt to the Deathless
// Audit: metadata-only — Needs arithmetic values for twice X and one life-gain event equal to the life actually lost by all opponents.
pub(in crate::card::sets) static DEBT_TO_THE_DEATHLESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("610e5a91-857b-4121-8b75-dbbea27aa0aa"),
    "Debt to the Deathless",
    crate::card::CardArt::new("610e5a91-857b-4121-8b75-dbbea27aa0aa", "Seb McKinnon"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 65 — Deputy of Acquittals
pub(in crate::card::sets) static DEPUTY_OF_ACQUITTALS: CardRecord = CardRecord::new_with_legacy_id(
    640,
    "Deputy of Acquittals",
    CardArt::new("4b555888-21b1-4c45-966d-d98f32460d4e", "James Ryman"),
    CardSet::DragonsMaze,
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
// Audit: metadata-only — Needs its targeted and overload programs migrated to one composite type, color, ability, power/toughness, and flying effect.
pub(in crate::card::sets) static DRAGONSHIFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6c046e4e-810c-4123-bb1a-4f97e0cd43d1"),
    "Dragonshift",
    crate::card::CardArt::new("6c046e4e-810c-4123-bb1a-4f97e0cd43d1", "Svetlin Velinov"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 67 — Drown in Filth
static DROWN_IN_FILTH_PENALTY: ValueDef =
    ValueDef::Negate(&ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
        ObjectPredicateDef::HasType(CardType::Land),
        &[ZoneKind::Graveyard],
        PlayerRelation::You,
    )));

pub(in crate::card::sets) static DROWN_IN_FILTH: CardRecord = CardRecord::new_with_legacy_id(
    641,
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
pub(in crate::card::sets) static EMMARA_TANDRIS: CardRecord = CardRecord::new_with_legacy_id(
    1898,
    "Emmara Tandris",
    CardArt::new("c7c91a0a-2f14-4131-8ca7-1d0046a8edd2", "Mark Winters"),
    CardSet::DragonsMaze,
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
pub(in crate::card::sets) static EXAVA_RAKDOS_BLOOD_WITCH: CardRecord =
    CardRecord::new_with_legacy_id(
        1630,
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

// DGM 70 — Feral Animist
pub(in crate::card::sets) static FERAL_ANIMIST: CardRecord = CardRecord::new_with_legacy_id(
    642,
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
pub(in crate::card::sets) static FLUXCHARGER: CardRecord = CardRecord::new_with_legacy_id(
    1993,
    "Fluxcharger",
    CardArt::new("7c58f6ed-2544-4b58-8dc0-a0a37b9547e6", "Willian Murai"),
    CardSet::DragonsMaze,
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
pub(in crate::card::sets) static GAZE_OF_GRANITE: CardRecord = CardRecord::new_with_legacy_id(
    167,
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
            then: None,
        },
    )),
);

// DGM 73 — Gleam of Battle
pub(in crate::card::sets) static GLEAM_OF_BATTLE: CardRecord = CardRecord::new_with_legacy_id(
    643,
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
// Audit: metadata-only — Needs a uniformly random legal target choice when the activated ability resolves.
pub(in crate::card::sets) static GOBLIN_TEST_PILOT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a8dbb9aa-1bf8-447d-a96c-33e2248bfb01"),
    "Goblin Test Pilot",
    crate::card::CardArt::new("a8dbb9aa-1bf8-447d-a96c-33e2248bfb01", "Svetlin Velinov"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 75 — Gruul War Chant
pub(in crate::card::sets) static GRUUL_WAR_CHANT: CardRecord = CardRecord::new_with_legacy_id(
    1759,
    "Gruul War Chant",
    CardArt::new("7c3091d4-d0f8-43d6-9ecb-0fecb32fe698", "Dave Kendall"),
    CardSet::DragonsMaze,
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
pub(in crate::card::sets) static HAUNTER_OF_NIGHTVEIL: CardRecord = CardRecord::new_with_legacy_id(
    644,
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
pub(in crate::card::sets) static JELENN_SPHINX: CardRecord = CardRecord::new_with_legacy_id(
    645,
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
// Audit: metadata-only — Needs removing a +1/+1 counter from a chosen creature, rather than from the ability source, as an activation cost.
pub(in crate::card::sets) static KOROZDA_GORGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7006e5b9-d6a3-43ce-904b-b2ac0fea67e5"),
    "Korozda Gorgon",
    crate::card::CardArt::new("7006e5b9-d6a3-43ce-904b-b2ac0fea67e5", "Volkan Baǵa"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 79 — Krasis Incubation
// Audit: metadata-only — Needs attached-creature attack, block, and activated-ability prohibitions plus returning the Aura as a cost while retaining its former attachment through last-known information.
pub(in crate::card::sets) static KRASIS_INCUBATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8da986da-e8ee-4b53-8bbd-9285d0f7f3cb"),
    "Krasis Incubation",
    crate::card::CardArt::new("8da986da-e8ee-4b53-8bbd-9285d0f7f3cb", "Marco Nelor"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 80 — Lavinia of the Tenth
// Audit: metadata-only — Needs detain's persistent restrictions and a nonland permanent sweep filtered by mana value.
pub(in crate::card::sets) static LAVINIA_OF_THE_TENTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("813f1967-c048-4e6e-9720-216773fde47e"),
    "Lavinia of the Tenth",
    crate::card::CardArt::new("813f1967-c048-4e6e-9720-216773fde47e", "Willian Murai"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 81 — Legion's Initiative
// Audit: metadata-only — Needs a non-choice binding for exactly the creatures exiled together so the installed beginning-of-combat trigger can return and grant haste only to that group.
pub(in crate::card::sets) static LEGION_S_INITIATIVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("672051a6-d232-4546-842a-369d412c38d2"),
    "Legion's Initiative",
    crate::card::CardArt::new("672051a6-d232-4546-842a-369d412c38d2", "Jaime Jones"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 82 — Master of Cruelties
// Audit: metadata-only — Needs an attack-alone restriction, an unblocked-attacker trigger that sets a player's life total, and suppression of this creature's combat damage.
pub(in crate::card::sets) static MASTER_OF_CRUELTIES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7b4d8ab5-252c-4727-817d-6f18cbaedd91"),
    "Master of Cruelties",
    crate::card::CardArt::new("7b4d8ab5-252c-4727-817d-6f18cbaedd91", "Chase Stone"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 83 — Maw of the Obzedat
pub(in crate::card::sets) static MAW_OF_THE_OBZEDAT: CardRecord = CardRecord::new_with_legacy_id(
    646,
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
// Audit: metadata-only — Needs a continuously revealed library top, cast permission from that zone, and copying spells cast from the library with target reselection.
pub(in crate::card::sets) static MELEK_IZZET_PARAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("19b4dc6c-940e-478c-b87f-b3939a30efbd"),
    "Melek, Izzet Paragon",
    crate::card::CardArt::new("3e892d86-f443-4846-8049-40ec6b8c22b4", "Jason Chan"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 85 — Mirko Vosk, Mind Drinker
// Audit: metadata-only — Needs reveal-until-four-matching-cards library traversal and moving the entire revealed group to the graveyard.
pub(in crate::card::sets) static MIRKO_VOSK_MIND_DRINKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d37cdd3e-4303-4391-aff4-4a543e65a836"),
    "Mirko Vosk, Mind Drinker",
    crate::card::CardArt::new("d37cdd3e-4303-4391-aff4-4a543e65a836", "Chase Stone"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 86 — Morgue Burst
// Audit: partial — Returning the graveyard card is implemented, but TargetPower cannot read a card target's power after it moves to hand.
pub(in crate::card::sets) static MORGUE_BURST: CardRecord = CardRecord::new_with_legacy_id(
    647,
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
},
        )
        .with_coverage(AbilityCoverageDef::partial(
            "TargetPower reads permanents but not a card target that moved from the graveyard to hand.",
        )),
    ),
);

// DGM 87 — Nivix Cyclops
pub(in crate::card::sets) static NIVIX_CYCLOPS: CardRecord = CardRecord::new_with_legacy_id(
    1741,
    "Nivix Cyclops",
    CardArt::new("87c6651d-72ca-43b3-94ca-d6c4c6b3ca3b", "Wayne Reynolds"),
    CardSet::DragonsMaze,
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
// Audit: metadata-only — Needs a draw-event replacement that recognizes the first draw of each opponent's draw step and redirects every other draw.
pub(in crate::card::sets) static NOTION_THIEF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("728e660b-ad8b-49d2-a7e5-6588e496519b"),
    "Notion Thief",
    crate::card::CardArt::new("728e660b-ad8b-49d2-a7e5-6588e496519b", "Clint Cearley"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 89 — Obzedat's Aid
pub(in crate::card::sets) static OBZEDATS_AID: CardRecord = CardRecord::new_with_legacy_id(
    648,
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
        },
    )),
);

// DGM 90 — Pilfered Plans
pub(in crate::card::sets) static PILFERED_PLANS: CardRecord = CardRecord::new_with_legacy_id(
    649,
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
// Audit: metadata-only — Needs a delayed first-main-phase mana effect that lets its controller distribute the countered spell's mana value among any combination of colors.
pub(in crate::card::sets) static PLASM_CAPTURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0ffe8485-d5fb-47cc-af53-6e0fd062b7a2"),
    "Plasm Capture",
    crate::card::CardArt::new("0ffe8485-d5fb-47cc-af53-6e0fd062b7a2", "Chase Stone"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 92 — Progenitor Mimic
pub(in crate::card::sets) static PROGENITOR_MIMIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3ad76314-b5d5-4353-86aa-e899e0d757a5"),
    "Progenitor Mimic",
    crate::card::CardArt::new("3ad76314-b5d5-4353-86aa-e899e0d757a5", "Daarken"),
    crate::card::CardSet::DragonsMaze,
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

// DGM 93 — Putrefy
pub(in crate::card::sets) static PUTREFY: CardRecord = CardRecord::new_with_legacy_id(
    198,
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

pub(in crate::card::sets) static RAL_ZAREK: CardRecord = CardRecord::new_with_legacy_id(
    650,
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
// Audit: metadata-only — Needs an X-bounded private-hand choice, same-name searches across three zones, exile of every chosen group, and the final shuffle.
pub(in crate::card::sets) static REAP_INTELLECT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c6297df2-c67a-4054-9617-5c6202c76de8"),
    "Reap Intellect",
    crate::card::CardArt::new("c6297df2-c67a-4054-9617-5c6202c76de8", "Steven Belledin"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 96 — Render Silent
// Audit: metadata-only — Needs a turn-long prohibition on the countered spell's controller casting any spell.
pub(in crate::card::sets) static RENDER_SILENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4514a13f-5eee-49a8-876c-6b4befff4592"),
    "Render Silent",
    crate::card::CardArt::new("e3f3d6e4-0abe-4042-a7f6-0395683e8582", "Matt Stewart"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 97 — Restore the Peace
// Audit: metadata-only — Needs per-turn damage history on creatures and a simultaneous return sweep over every creature that dealt damage.
pub(in crate::card::sets) static RESTORE_THE_PEACE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("105902f6-99d0-4bee-9dfd-87a92ac04d91"),
    "Restore the Peace",
    crate::card::CardArt::new("105902f6-99d0-4bee-9dfd-87a92ac04d91", "Kev Walker"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 98 — Rot Farm Skeleton
// Audit: metadata-only — Needs an executable can't-block restriction and milling cards as an activation cost from the graveyard.
pub(in crate::card::sets) static ROT_FARM_SKELETON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ef5af2dd-75c7-402c-be9a-3d0d4290520c"),
    "Rot Farm Skeleton",
    crate::card::CardArt::new("ef5af2dd-75c7-402c-be9a-3d0d4290520c", "Maciej Kuciara"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 99 — Ruric Thar, the Unbowed
pub(in crate::card::sets) static RURIC_THAR_THE_UNBOWED: CardRecord = CardRecord::new_with_legacy_id(
    206,
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
// Audit: metadata-only — Needs an X-sized battlefield-entry counter replacement and a hybrid-mana activation restricted to sorcery timing.
pub(in crate::card::sets) static SAVAGEBORN_HYDRA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2f2b73cd-6179-4885-9d92-1782d0b492c1"),
    "Savageborn Hydra",
    crate::card::CardArt::new("2f2b73cd-6179-4885-9d92-1782d0b492c1", "Raymond Swanland"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 101 — Scab-Clan Giant
// Audit: metadata-only — Needs a uniformly random legal opponent-creature choice followed by the simultaneous fight damage procedure.
pub(in crate::card::sets) static SCAB_CLAN_GIANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a8e360ae-4c78-47a9-81d4-1849cfa518b7"),
    "Scab-Clan Giant",
    crate::card::CardArt::new("a8e360ae-4c78-47a9-81d4-1849cfa518b7", "Zoltan Boros"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 102 — Showstopper
pub(in crate::card::sets) static SHOWSTOPPER: CardRecord = CardRecord::new_with_legacy_id(
    651,
    "Showstopper",
    CardArt::new("2fd1f68b-3f16-484e-95c9-5cfa8da218c9", "Steve Prescott"),
    CardSet::DragonsMaze,
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
pub(in crate::card::sets) static SIN_COLLECTOR: CardRecord = CardRecord::new_with_legacy_id(
    214,
    "Sin Collector",
    CardArt::new("305a3feb-df49-486c-a3b4-ff2721d60019", "Mike Bierek"),
    CardSet::DragonsMaze,
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
pub(in crate::card::sets) static SIRE_OF_INSANITY: CardRecord = CardRecord::new_with_legacy_id(
    652,
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
                then: None,
            },
        ),
    ),
);

// DGM 105 — Species Gorger
pub(in crate::card::sets) static SPECIES_GORGER: CardRecord = CardRecord::new_with_legacy_id(
    653,
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
pub(in crate::card::sets) static SPIKE_JESTER: CardRecord = CardRecord::new_with_legacy_id(
    654,
    "Spike Jester",
    CardArt::new("cec50499-70d4-4dc1-9cae-abbecfc8e87d", "Ryan Barger"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{B}{R}"), &["Goblin", "Warrior"], 3, 1)
        .with_ability(abilities::haste()),
);

// DGM 107 — Tajic, Blade of the Legion
pub(in crate::card::sets) static TAJIC_BLADE_OF_THE_LEGION: CardRecord = CardRecord::new_with_legacy_id(
    655,
    "Tajic, Blade of the Legion",
    CardArt::new("be5717c1-338e-446c-aa7e-93e79e4abb72", "James Ryman"),
    CardSet::DragonsMaze,
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
// Audit: metadata-only — Needs protection from creatures and a combat-damage trigger that destroys the specific dealing creature before creating a token.
pub(in crate::card::sets) static TEYSA_ENVOY_OF_GHOSTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cbd8183c-6967-4332-b822-02b82c14ef2d"),
    "Teysa, Envoy of Ghosts",
    crate::card::CardArt::new("cbd8183c-6967-4332-b822-02b82c14ef2d", "Karla Ortiz"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 109 — Tithe Drinker
pub(in crate::card::sets) static TITHE_DRINKER: CardRecord = CardRecord::new_with_legacy_id(
    1899,
    "Tithe Drinker",
    CardArt::new("e069aa06-35b0-4af8-89cb-af653708ed32", "Slawomir Maniak"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{W}{B}"), &["Vampire"], 2, 1)
        .with_abilities(&[abilities::lifelink(), abilities::extort()]),
);

// DGM 110 — Trostani's Summoner
pub(in crate::card::sets) static TROSTANIS_SUMMONER: CardRecord = CardRecord::new_with_legacy_id(
    656,
    "Trostani's Summoner",
    CardArt::new("1921fa4e-2256-4ef1-b2fe-874f9fbbcdf3", "Howard Lyon"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{5}{G}{W}"), &["Elf", "Shaman"], 1, 1).with_ability(
        abilities::enters_trigger("When this creature enters, create a 2/2 white Knight creature token with vigilance, a 3/3 green Centaur creature token, and a 4/4 green Rhino creature token with trample.", EffectDef::Sequence(&[
                EffectDef::create_creature_token(&["Knight"], &[ManaColor::White], 2, 2).with_abilities(&[abilities::vigilance()]).with_art(CardArt::new("67d3d039-248a-4eb8-be5c-12959b458fea", "Matt Stewart")),
                EffectDef::create_creature_token(&["Centaur"], &[ManaColor::Green], 3, 3).with_art(CardArt::new("880d5dc1-ceec-4c5f-93c2-c88b7dbfcac2", "Slawomir Maniak")),
                EffectDef::create_creature_token(&["Rhino"], &[ManaColor::Green], 4, 4).with_abilities(&[abilities::trample()]).with_art(CardArt::new("1331008a-ae86-4640-b823-a73be766ac16", "Tomasz Jedruszek")),
            ])),
    ),
);

// DGM 111 — Unflinching Courage
pub(in crate::card::sets) static UNFLINCHING_COURAGE: CardRecord = CardRecord::new_with_legacy_id(
    234,
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
// Audit: metadata-only — Needs granting scavenge to graveyard cards with each card's own mana cost and power, plus regeneration shields.
pub(in crate::card::sets) static VAROLZ_THE_SCAR_STRIPED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4c3ae3db-c14a-4ffc-805c-a3a51da9370d"),
    "Varolz, the Scar-Striped",
    crate::card::CardArt::new("4c3ae3db-c14a-4ffc-805c-a3a51da9370d", "Adam Paquette"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 113 — Viashino Firstblade
pub(in crate::card::sets) static VIASHINO_FIRSTBLADE: CardRecord = CardRecord::new_with_legacy_id(
    657,
    "Viashino Firstblade",
    CardArt::new("1cb0c21c-bdf1-478a-9ad8-6c6bda6ffb0f", "Matt Stewart"),
    CardSet::DragonsMaze,
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

pub(in crate::card::sets) static VOICE_OF_RESURGENCE: CardRecord = CardRecord::new_with_legacy_id(
    238,
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
            TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent)),
            &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
            VOICE_OF_RESURGENCE_TOKEN,
        ),
        abilities::dies_trigger("When this creature dies, create a green and white Elemental creature token with \"This token's power and toughness are each equal to the number of creatures you control.\"", VOICE_OF_RESURGENCE_TOKEN),
    ]),
);

// DGM 115 — Vorel of the Hull Clade
// Audit: metadata-only — Needs an effect that doubles every kind of counter on one targeted artifact, creature, or land.
pub(in crate::card::sets) static VOREL_OF_THE_HULL_CLADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("db0665d4-d974-4d5e-ba29-7bf40cbbe29c"),
    "Vorel of the Hull Clade",
    crate::card::CardArt::new("db0665d4-d974-4d5e-ba29-7bf40cbbe29c", "Mike Bierek"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 116 — Warleader's Helix
pub(in crate::card::sets) static WARLEADERS_HELIX: CardRecord = CardRecord::new_with_legacy_id(
    242,
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
// Audit: metadata-only — Needs a current hand-card count value and its negation to drive the temporary +X/-X effect.
pub(in crate::card::sets) static WARPED_PHYSIQUE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("134802b2-7c5c-4eda-a879-b29bc06faaed"),
    "Warped Physique",
    crate::card::CardArt::new("134802b2-7c5c-4eda-a879-b29bc06faaed", "Karl Kopinski"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 118 — Woodlot Crawler
pub(in crate::card::sets) static WOODLOT_CRAWLER: CardRecord = CardRecord::new_with_legacy_id(
    658,
    "Woodlot Crawler",
    CardArt::new("11f1e6fe-e959-4030-9925-9ccc27040275", "Greg Staples"),
    CardSet::DragonsMaze,
    CardRules::new_creature(mana_cost!("{U}{B}"), &["Insect"], 2, 1).with_abilities(&[
        abilities::forestwalk(),
        abilities::protection_from_color(ManaColor::Green),
    ]),
);

// DGM 119 — Zhur-Taa Ancient
// Audit: metadata-only — Needs mana-production provenance so the trigger can add one mana of a type the tapped land produced.
pub(in crate::card::sets) static ZHUR_TAA_ANCIENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2076308f-0f4e-4b31-9e75-c2965942e7d1"),
    "Zhur-Taa Ancient",
    crate::card::CardArt::new("2076308f-0f4e-4b31-9e75-c2965942e7d1", "Adam Paquette"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 120 — Zhur-Taa Druid
pub(in crate::card::sets) static ZHUR_TAA_DRUID: CardRecord = CardRecord::new_with_legacy_id(
    659,
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
// Audit: metadata-only — Needs fuse spell composition plus a 3/3 green Centaur token and a creature-count life-gain value multiplied by two.
pub(in crate::card::sets) static ALIVE_WELL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("db84415e-048a-4cfc-9121-5ae17a412198"),
    "Alive // Well",
    crate::card::CardArt::new("db84415e-048a-4cfc-9121-5ae17a412198", "Nils Hamm"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 122 — Armed // Dangerous
// Audit: metadata-only — Needs fuse spell composition and a turn-long requirement that every creature able to block the Dangerous target does so.
pub(in crate::card::sets) static ARMED_DANGEROUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ff7f4fc2-6f76-44e7-a30b-7166a0d10d2a"),
    "Armed // Dangerous",
    crate::card::CardArt::new("ff7f4fc2-6f76-44e7-a30b-7166a0d10d2a", "David Palumbo"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 123 — Beck // Call
// Audit: metadata-only — Needs fuse spell composition plus a temporary enters-the-battlefield listener and a 1/1 white flying Bird token.
pub(in crate::card::sets) static BECK_CALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a01d6540-9eaf-4e08-a62d-682551ee78e9"),
    "Beck // Call",
    crate::card::CardArt::new("a01d6540-9eaf-4e08-a62d-682551ee78e9", "Adam Paquette"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 124 — Breaking // Entering
// Audit: metadata-only — Needs fuse spell composition and a nontarget creature-card choice from either graveyard for Entering.
pub(in crate::card::sets) static BREAKING_ENTERING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("50e30e21-0bf7-4d10-b2cc-ed0c52b95955"),
    "Breaking // Entering",
    crate::card::CardArt::new("66724f4e-59dd-4c70-b09b-49947320e6d1", "Mathias Kollros"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 125 — Catch // Release
// Audit: metadata-only — Needs fuse spell composition and one independent permanent choice for each named card type from every player.
pub(in crate::card::sets) static CATCH_RELEASE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29968873-56f3-4528-ab0b-f11dd67dd162"),
    "Catch // Release",
    crate::card::CardArt::new("29968873-56f3-4528-ab0b-f11dd67dd162", "Kev Walker"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 126 — Down // Dirty
pub(in crate::card::sets) static DOWN_DIRTY: CardRecord = CardRecord::new_fuse_with_legacy_id(
    660,
    "Down // Dirty",
    CardArt::new("c35c63c1-6344-4d8c-8f7d-cd253d12f9ae", "Svetlin Velinov"),
    CardSet::DragonsMaze,
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
pub(in crate::card::sets) static FAR_AWAY: CardRecord = CardRecord::new_fuse_with_legacy_id(
    661,
    "Far // Away",
    CardArt::new("d13cdb71-a499-41db-84e6-95f84650c524", "Greg Staples"),
    CardSet::DragonsMaze,
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
// Audit: metadata-only — Needs fuse spell composition and a value carrying the exiled graveyard card's power into Flesh's counter effect.
pub(in crate::card::sets) static FLESH_BLOOD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("02b40fe4-901a-4832-8d52-a6bb5cc07b63"),
    "Flesh // Blood",
    crate::card::CardArt::new("02b40fe4-901a-4832-8d52-a6bb5cc07b63", "Lucas Graciano"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 129 — Give // Take
// Audit: metadata-only — Needs fuse spell composition and removing all +1/+1 counters from the targeted creature while remembering the removed count.
pub(in crate::card::sets) static GIVE_TAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9af07d28-45a2-45d6-b1cb-0858c609a881"),
    "Give // Take",
    crate::card::CardArt::new("9af07d28-45a2-45d6-b1cb-0858c609a881", "Steve Prescott"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 130 — Profit // Loss
pub(in crate::card::sets) static PROFIT_LOSS: CardRecord = CardRecord::new_fuse_with_legacy_id(
    662,
    "Profit // Loss",
    CardArt::new("0eb3ce46-ddd2-43b3-9e45-019ae91df686", "Kev Walker"),
    CardSet::DragonsMaze,
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
pub(in crate::card::sets) static PROTECT_SERVE: CardRecord = CardRecord::new_fuse_with_legacy_id(
    663,
    "Protect // Serve",
    CardArt::new("9b8acd7d-f3e2-4358-91ab-40901b68d64c", "Ryan Barger"),
    CardSet::DragonsMaze,
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
pub(in crate::card::sets) static READY_WILLING: CardRecord = CardRecord::new_fuse_with_legacy_id(
    664,
    "Ready // Willing",
    CardArt::new("22081f95-dc8e-41ed-b609-b6a22ee5428b", "Zoltan Boros"),
    CardSet::DragonsMaze,
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
// Audit: metadata-only — Needs fuse spell composition and a value for the targeted player's current hand size.
pub(in crate::card::sets) static TOIL_TROUBLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("15bb3454-e3bb-4af9-9e93-461e210c26b7"),
    "Toil // Trouble",
    crate::card::CardArt::new("15bb3454-e3bb-4af9-9e93-461e210c26b7", "Nils Hamm"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 134 — Turn // Burn
pub(in crate::card::sets) static TURN_BURN: CardRecord = CardRecord::new_fuse_with_legacy_id(
    230,
    "Turn // Burn",
    CardArt::new("8d7fdd59-6d76-4a0c-ac75-816345ef4a39", "Ryan Barger"),
    CardSet::DragonsMaze,
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
pub(in crate::card::sets) static WEAR_TEAR: CardRecord = CardRecord::new_fuse_with_legacy_id(
    665,
    "Wear // Tear",
    CardArt::new("d169a3b2-18ae-4414-98ef-d879676fdcc0", "Ryan Pancoast"),
    CardSet::DragonsMaze,
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
pub(in crate::card::sets) static AZORIUS_CLUESTONE: CardRecord = CardRecord::new_with_legacy_id(
    666,
    "Azorius Cluestone",
    CardArt::new("09eeb301-bc28-4515-ad69-0b1b5164a5bc", "Raoul Vitale"),
    CardSet::DragonsMaze,
    cluestone_rules(&AZORIUS_CLUESTONE_ABILITIES),
);

// DGM 137 — Boros Cluestone
pub(in crate::card::sets) static BOROS_CLUESTONE: CardRecord = CardRecord::new_with_legacy_id(
    667,
    "Boros Cluestone",
    CardArt::new("87252577-3e7b-4ea2-b0ac-3ba3f0eaac40", "Raoul Vitale"),
    CardSet::DragonsMaze,
    cluestone_rules(&BOROS_CLUESTONE_ABILITIES),
);

// DGM 138 — Dimir Cluestone
pub(in crate::card::sets) static DIMIR_CLUESTONE: CardRecord = CardRecord::new_with_legacy_id(
    668,
    "Dimir Cluestone",
    CardArt::new("0d8ac24f-3309-453a-b2d6-6363df9a1ddd", "Raoul Vitale"),
    CardSet::DragonsMaze,
    cluestone_rules(&DIMIR_CLUESTONE_ABILITIES),
);

// DGM 139 — Golgari Cluestone
pub(in crate::card::sets) static GOLGARI_CLUESTONE: CardRecord = CardRecord::new_with_legacy_id(
    669,
    "Golgari Cluestone",
    CardArt::new("ff77e1ee-7fa3-4370-a0c9-ec008b63302f", "Raoul Vitale"),
    CardSet::DragonsMaze,
    cluestone_rules(&GOLGARI_CLUESTONE_ABILITIES),
);

// DGM 140 — Gruul Cluestone
pub(in crate::card::sets) static GRUUL_CLUESTONE: CardRecord = CardRecord::new_with_legacy_id(
    670,
    "Gruul Cluestone",
    CardArt::new("bc47d1fe-8ab2-42f6-bcab-4bc2084ceba7", "Raoul Vitale"),
    CardSet::DragonsMaze,
    cluestone_rules(&GRUUL_CLUESTONE_ABILITIES),
);

// DGM 141 — Izzet Cluestone
pub(in crate::card::sets) static IZZET_CLUESTONE: CardRecord = CardRecord::new_with_legacy_id(
    671,
    "Izzet Cluestone",
    CardArt::new("8cf63def-e2cc-48c7-8409-c08a36eddf93", "Raoul Vitale"),
    CardSet::DragonsMaze,
    cluestone_rules(&IZZET_CLUESTONE_ABILITIES),
);

// DGM 142 — Orzhov Cluestone
pub(in crate::card::sets) static ORZHOV_CLUESTONE: CardRecord = CardRecord::new_with_legacy_id(
    672,
    "Orzhov Cluestone",
    CardArt::new("4823f904-1c41-42cf-aef7-db0dcf82b10b", "Raoul Vitale"),
    CardSet::DragonsMaze,
    cluestone_rules(&ORZHOV_CLUESTONE_ABILITIES),
);

// DGM 143 — Rakdos Cluestone
pub(in crate::card::sets) static RAKDOS_CLUESTONE: CardRecord = CardRecord::new_with_legacy_id(
    673,
    "Rakdos Cluestone",
    CardArt::new("9ef43817-1813-4608-8e3d-3c14321ab736", "Raoul Vitale"),
    CardSet::DragonsMaze,
    cluestone_rules(&RAKDOS_CLUESTONE_ABILITIES),
);

// DGM 144 — Selesnya Cluestone
pub(in crate::card::sets) static SELESNYA_CLUESTONE: CardRecord = CardRecord::new_with_legacy_id(
    674,
    "Selesnya Cluestone",
    CardArt::new("34ad5631-439a-43e2-b00a-04f78d66b8e6", "Raoul Vitale"),
    CardSet::DragonsMaze,
    cluestone_rules(&SELESNYA_CLUESTONE_ABILITIES),
);

// DGM 145 — Simic Cluestone
pub(in crate::card::sets) static SIMIC_CLUESTONE: CardRecord = CardRecord::new_with_legacy_id(
    675,
    "Simic Cluestone",
    CardArt::new("e3c47552-afed-463d-bd24-13eb1cd724fc", "Raoul Vitale"),
    CardSet::DragonsMaze,
    cluestone_rules(&SIMIC_CLUESTONE_ABILITIES),
);

// DGM 146 — Azorius Guildgate (reprint)

// DGM 147 — Boros Guildgate (reprint)

// DGM 148 — Dimir Guildgate (reprint)

// DGM 149 — Golgari Guildgate (reprint)

// DGM 150 — Gruul Guildgate (reprint)

// DGM 151 — Izzet Guildgate (reprint)

// DGM 152 — Maze's End
// Audit: metadata-only — Needs returning the land as an activation cost, a Gate-specific library search to the battlefield, and the ten-distinct-names win condition.
pub(in crate::card::sets) static MAZE_S_END: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6966ecf0-6dab-4652-bf5b-6e766b8347d0"),
    "Maze's End",
    crate::card::CardArt::new("401f7042-24fd-42a0-ae7c-e6b7de1aa446", "Cliff Childs"),
    crate::card::CardSet::DragonsMaze,
    crate::card::CardRules::unsupported(),
);

// DGM 153 — Orzhov Guildgate (reprint)

// DGM 154 — Rakdos Guildgate (reprint)

// DGM 155 — Selesnya Guildgate (reprint)

// DGM 156 — Simic Guildgate (reprint)

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
    &FERAL_ANIMIST,
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
    &PUTREFY,
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
