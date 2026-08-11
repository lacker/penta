use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AppliedEffectDef, CardArt, CardBehavior, CardRules, CardSet, CardType, ComparisonDef,
    EffectDef, EffectDurationDef, EffectExecutionDef, EffectRecipientDef, ObjectPredicateDef,
    ObjectQueryDef, PlayerRelation, TriggerConditionDef, TriggerEventDef, TurnStepDef, ZoneKind,
    abilities, cards,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

pub(in crate::card::sets) static BALL_LIGHTNING: CardRecord = CardRecord::new(
    cards::BALL_LIGHTNING,
    "Ball Lightning",
    CardArt::new("c1ba83ab-83f5-421d-bba1-0f925870b5c8", "Quinton Hoover"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{R}{R}{R}"), &["Elemental"], 6, 1).with_abilities(&[
        abilities::trample(),
        abilities::haste(),
        AbilityDef::triggered(
            "At the beginning of the end step, sacrifice this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

pub(in crate::card::sets) static BLOOD_MOON: CardRecord = CardRecord::new(
    cards::BLOOD_MOON,
    "Blood Moon",
    CardArt::new("78373616-e2d6-4ccf-998f-09f02bea45b4", "Tom Wänerstrand"),
    CardSet::TheDark,
    CardRules::new_enchantment(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::static_ability(
        "Nonbasic lands are Mountains.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                        crate::card::CardSupertype::Basic,
                    )),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            effect: AppliedEffectDef::SetLandTypes(&[crate::card::BasicLandType::Mountain]),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    )]),
);

pub(in crate::card::sets) static GOBLIN_DIGGING_TEAM: CardRecord = CardRecord::new(
    cards::GOBLIN_DIGGING_TEAM,
    "Goblin Digging Team",
    CardArt::new("8a538b9d-351e-40bb-be11-9ba08c16352b", "Ron Spencer"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this creature: Destroy target Wall.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Subtype("Wall"),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        ),
    ]),
);

/// Any Dwarf you control at all, which is why this is a count of at least one
/// rather than an exact number.
static GOBLINS_OF_THE_FLARG_DWARF_CONDITION: TriggerConditionDef =
    TriggerConditionDef::ObjectCount {
        query: ObjectQueryDef {
            object: ObjectPredicateDef::Subtype("Dwarf"),
            zones: &[ZoneKind::Battlefield],
            controller: PlayerRelation::You,
        },
        comparison: ComparisonDef::AtLeast,
        amount: 1,
    };

pub(in crate::card::sets) static GOBLINS_OF_THE_FLARG: CardRecord = CardRecord::new(
    cards::GOBLINS_OF_THE_FLARG,
    "Goblins of the Flarg",
    CardArt::new("fd333b18-b896-4ab8-9c46-eed4efdd94f2", "Tom Wänerstrand"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Warrior"], 1, 1).with_abilities(&[
        abilities::mountainwalk(),
        AbilityDef::triggered_if(
            "When you control a Dwarf, sacrifice this creature.",
            TriggerEventDef::StateCondition,
            &GOBLINS_OF_THE_FLARG_DWARF_CONDITION,
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

pub(in crate::card::sets) static FELLWAR_STONE: CardRecord = CardRecord::new(
    cards::FELLWAR_STONE,
    "Fellwar Stone",
    CardArt::new("dc47e322-f8b8-4685-b035-fda0cc433e6b", "Quinton Hoover"),
    CardSet::TheDark,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[AbilityDef::activated_mana(
        "{T}: Add one mana of any color that a land an opponent controls could produce.",
        &[AbilityCostDef::TapSource],
        EffectDef::Special("Add one mana of a color an opponent's land could produce"),
    )
    .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::FellwarStone))
    .with_coverage(AbilityCoverageDef::explained_complete(
        "The available colors are computed dynamically from an opponent's lands.",
    ))
    .with_legacy_procedure()]),
);

/// The Maze does not remove the creature from combat: it stays an attacker,
/// keeps whatever is blocking it, and simply exchanges no combat damage.
static MAZE_OF_ITH_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Attacking,
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

static MAZE_OF_ITH_EFFECT: [EffectDef; 2] = [
    EffectDef::Untap {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::PreventCombatDamageThisTurn {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
];

pub(in crate::card::sets) static MAZE_OF_ITH: CardRecord = CardRecord::new(
    cards::MAZE_OF_ITH,
    "Maze of Ith",
    CardArt::new("42dcceee-2a47-4eaa-a6a3-2931b3d50244", "Anson Maddocks"),
    CardSet::TheDark,
    CardRules::new_land(&[]).with_abilities(&[AbilityDef::activated_with_targets(
        "{T}: Untap target attacking creature. Prevent all combat damage that would be dealt to and dealt by that creature this turn.",
        &[AbilityCostDef::TapSource],
        &MAZE_OF_ITH_TARGET,
        EffectDef::Sequence(&MAZE_OF_ITH_EFFECT),
    )]),
);

pub(in crate::card::sets) static DUST_TO_DUST: CardRecord = CardRecord::new(
    cards::DUST_TO_DUST,
    "Dust to Dust",
    CardArt::new("ade075fd-73ee-4d12-a2da-48e5938043af", "Drew Tucker"),
    CardSet::TheDark,
    CardRules::new_sorcery(mana_cost!("{1}{W}{W}")).with_abilities(&[AbilityDef::custom_full(
        "Exile two target artifacts.",
        CardBehavior::DustToDust,
        "Artifact targeting and exile are implemented by the legacy spell resolver.",
    )]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BALL_LIGHTNING,
    &BLOOD_MOON,
    &GOBLIN_DIGGING_TEAM,
    &GOBLINS_OF_THE_FLARG,
    &FELLWAR_STONE,
    &MAZE_OF_ITH,
    &DUST_TO_DUST,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
