//! Eventide cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AppliedEffectDef, CardArt, CardRules, CardSet, CreatureTypeSetDef,
    EffectDef, EffectRecipientDef, ObjectPredicateDef, ResolvedEffectDurationDef,
    TriggerConditionDef, ValueDef, abilities,
};
use crate::mana_cost;

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

// EVE 139 — Figure of Destiny
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&FIGURE_OF_DESTINY];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
