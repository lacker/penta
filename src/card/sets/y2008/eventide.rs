//! Eventide cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AppliedEffectDef, CardArt, CardRules, CardSet,
    CreatureTypeSetDef, EffectDef, EffectRecipientDef, InstalledTriggerDef, ObjectPredicateDef,
    ResolvedEffectDurationDef, TriggerConditionDef, ValueDef, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// EVE 6 — Flickerwisp
/// "Another target permanent": his own arrival cannot answer itself, and
/// nothing else is out of reach -- a land is as blinkable as a creature,
/// which is what separates him from every other flicker in white.
static ANOTHER_PERMANENT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
)];

static FLICKERWISP_EXILE: [EffectDef; 2] = [
    EffectDef::ExileLinkedToSource {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::InstallTrigger(InstalledTriggerDef::once(
        &abilities::return_linked_exiles_at_next_end_step(ObjectPredicateDef::Any),
    )),
];

pub(in crate::card::sets) static FLICKERWISP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5bb3cb5c-8d66-4f5e-a9a9-917e6045f024"),
    "Flickerwisp",
    CardArt::new("5bb3cb5c-8d66-4f5e-a9a9-917e6045f024", "Jeremy Enecio"),
    CardSet::Eventide,
    // Three mana for a 3/1 flier that also answers something for a turn:
    // an attacker, a blocker, a land on the turn it matters, or one of your
    // own permanents that would rather enter again.
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Elemental"], 3, 1).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, exile another target permanent. Return that card to the \
             battlefield under its owner's control at the beginning of the next end step.",
            &ANOTHER_PERMANENT,
            EffectDef::Sequence(&FLICKERWISP_EXILE),
        ),
    ]),
);

// EVE 41 — Raven's Crime
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAVEN_S_CRIME: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ced5797-5de0-43ca-9dc9-e48912333a70"),
    "Raven's Crime",
    crate::card::CardArt::new("7ced5797-5de0-43ca-9dc9-e48912333a70", "Warren Mahy"),
    crate::card::CardSet::Eventide,
    crate::card::CardRules::unsupported(),
);

// EVE 119 — Desecrator Hag
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DESECRATOR_HAG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("74d2e092-c805-447c-b784-1896b69524e0"),
    "Desecrator Hag",
    crate::card::CardArt::new("74d2e092-c805-447c-b784-1896b69524e0", "Fred Harper"),
    crate::card::CardSet::Eventide,
    crate::card::CardRules::unsupported(),
);

// EVE 139 — Figure of Destiny
/// Each step repaints the whole creature-type line rather than adding to it,
/// which is what "becomes a Kithkin Spirit Warrior" says: the types it lists
/// are the types it has. None of them ends, so every one is permanent and
/// the next step reads the one before it off the board.
static FIGURE_BECOMES_A_SPIRIT: [AppliedEffectDef; 2] = [
    AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Kithkin", "Spirit"])),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
];

static FIGURE_BECOMES_A_WARRIOR: [AppliedEffectDef; 2] = [
    AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
        "Kithkin", "Spirit", "Warrior",
    ])),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(4), ValueDef::Constant(4)),
];

static FIGURE_BECOMES_AN_AVATAR: [AppliedEffectDef; 4] = [
    AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
        "Kithkin", "Spirit", "Warrior", "Avatar",
    ])),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(8), ValueDef::Constant(8)),
    AppliedEffectDef::add_ability(&FIGURE_FLYING),
    AppliedEffectDef::add_ability(&FIGURE_FIRST_STRIKE),
];

static FIGURE_FLYING: AbilityDef = abilities::flying();

static FIGURE_FIRST_STRIKE: AbilityDef = abilities::first_strike();

/// "If this creature is a Spirit" is read as the ability resolves, so a
/// Figure that was answered in response is a 1/1 again and the second
/// activation does nothing.
static FIGURE_IS_A_SPIRIT: TriggerConditionDef = TriggerConditionDef::SourceMatches {
    object: ObjectPredicateDef::Subtype("Spirit"),
};

static FIGURE_IS_A_WARRIOR: TriggerConditionDef = TriggerConditionDef::SourceMatches {
    object: ObjectPredicateDef::Subtype("Warrior"),
};

static FIGURE_WARRIOR_STEP: EffectDef = EffectDef::Apply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::Composite(&FIGURE_BECOMES_A_WARRIOR),
    duration: ResolvedEffectDurationDef::Permanent,
};

static FIGURE_AVATAR_STEP: EffectDef = EffectDef::Apply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::Composite(&FIGURE_BECOMES_AN_AVATAR),
    duration: ResolvedEffectDurationDef::Permanent,
};

static FIGURE_FIRST_COST: [AbilityCostDef; 1] = [AbilityCostDef::Mana(mana_cost!("{R/W}"))];

static FIGURE_SECOND_COST: [AbilityCostDef; 1] =
    [AbilityCostDef::Mana(mana_cost!("{R/W}{R/W}{R/W}"))];

static FIGURE_THIRD_COST: [AbilityCostDef; 1] = [AbilityCostDef::Mana(mana_cost!(
    "{R/W}{R/W}{R/W}{R/W}{R/W}{R/W}"
))];

static FIGURE_OF_DESTINY_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::activated(
        "{R/W}: This creature becomes a Kithkin Spirit with base power and toughness 2/2.",
        &FIGURE_FIRST_COST,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::Composite(&FIGURE_BECOMES_A_SPIRIT),
            duration: ResolvedEffectDurationDef::Permanent,
        },
    ),
    AbilityDef::activated(
        "{R/W}{R/W}{R/W}: If this creature is a Spirit, it becomes a Kithkin Spirit Warrior with \
         base power and toughness 4/4.",
        &FIGURE_SECOND_COST,
        EffectDef::IfCondition {
            condition: &FIGURE_IS_A_SPIRIT,
            then: &FIGURE_WARRIOR_STEP,
        },
    ),
    AbilityDef::activated(
        "{R/W}{R/W}{R/W}{R/W}{R/W}{R/W}: If this creature is a Warrior, it becomes a Kithkin \
         Spirit Warrior Avatar with base power and toughness 8/8, flying, and first strike.",
        &FIGURE_THIRD_COST,
        EffectDef::IfCondition {
            condition: &FIGURE_IS_A_WARRIOR,
            then: &FIGURE_AVATAR_STEP,
        },
    ),
];

pub(in crate::card::sets) static FIGURE_OF_DESTINY: CardRecord = CardRecord::new_with_legacy_id(
    2260,
    "Figure of Destiny",
    CardArt::new("0da69523-cece-425a-b08a-fb27fac29374", "Scott M. Fischer"),
    CardSet::Eventide,
    // A one-drop that is never a dead draw: it is a 1/1 on turn one and an
    // 8/8 flier on turn six, and every point of mana in between goes into it.
    CardRules::new_creature(mana_cost!("{R/W}"), &["Kithkin"], 1, 1)
        .with_abilities(&FIGURE_OF_DESTINY_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &FLICKERWISP,
    &RAVEN_S_CRIME,
    &DESECRATOR_HAG,
    &FIGURE_OF_DESTINY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
