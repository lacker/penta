//! Eventide cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AppliedEffectDef, CardArt, CardRules, CardSet,
    CreatureTypeSetDef, EffectDef, EffectRecipientDef, ObjectPredicateDef,
    ResolvedEffectDurationDef, TriggerConditionDef, ValueDef, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// EVE 6 — Flickerwisp
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
            // "Another target permanent": his own arrival cannot answer itself, and
            // nothing else is out of reach -- a land is as blinkable as a creature,
            // which is what separates him from every other flicker in white.
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            )],
            abilities::exile_until_next_end_step(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
        ),
    ]),
);

// EVE 41 — Raven's Crime
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAVEN_S_CRIME: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ced5797-5de0-43ca-9dc9-e48912333a70"),
    "Raven's Crime",
    crate::card::CardArt::new("7ced5797-5de0-43ca-9dc9-e48912333a70", "Warren Mahy"),
    crate::card::CardSet::Eventide,
    crate::card::CardRules::unsupported(),
);

// EVE 119 — Desecrator Hag
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESECRATOR_HAG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("74d2e092-c805-447c-b784-1896b69524e0"),
    "Desecrator Hag",
    crate::card::CardArt::new("74d2e092-c805-447c-b784-1896b69524e0", "Fred Harper"),
    crate::card::CardSet::Eventide,
    crate::card::CardRules::unsupported(),
);

// EVE 139 — Figure of Destiny
pub(in crate::card::sets) static FIGURE_OF_DESTINY: CardRecord = CardRecord::new_with_legacy_id(
    2260,
    "Figure of Destiny",
    CardArt::new("0da69523-cece-425a-b08a-fb27fac29374", "Scott M. Fischer"),
    CardSet::Eventide,
    // A one-drop that is never a dead draw: it is a 1/1 on turn one and an
    // 8/8 flier on turn six, and every point of mana in between goes into it.
    CardRules::new_creature(mana_cost!("{R/W}"), &["Kithkin"], 1, 1)
        .with_abilities(&[
            AbilityDef::activated(
                "{R/W}: This creature becomes a Kithkin Spirit with base power and toughness 2/2.",
                &[AbilityCostDef::Mana(mana_cost!("{R/W}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    // Each step repaints the whole creature-type line rather than adding to it,
                    // which is what "becomes a Kithkin Spirit Warrior" says: the types it lists
                    // are the types it has. None of them ends, so every one is permanent and
                    // the next step reads the one before it off the board.
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Kithkin", "Spirit"])),
                        AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
                    ]),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            ),
            AbilityDef::activated(
                "{R/W}{R/W}{R/W}: If this creature is a Spirit, it becomes a Kithkin Spirit Warrior with \
                 base power and toughness 4/4.",
                &[AbilityCostDef::Mana(mana_cost!("{R/W}{R/W}{R/W}"))],
                EffectDef::IfCondition {
                    // "If this creature is a Spirit" is read as the ability resolves, so a
                    // Figure that was answered in response is a 1/1 again and the second
                    // activation does nothing.
                    condition: &TriggerConditionDef::SourceMatches {
                        object: ObjectPredicateDef::Subtype("Spirit"),
                    },
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                                "Kithkin", "Spirit", "Warrior",
                            ])),
                            AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(4), ValueDef::Constant(4)),
                        ]),
                        duration: ResolvedEffectDurationDef::Permanent,
                    },
                },
            ),
            AbilityDef::activated(
                "{R/W}{R/W}{R/W}{R/W}{R/W}{R/W}: If this creature is a Warrior, it becomes a Kithkin \
                 Spirit Warrior Avatar with base power and toughness 8/8, flying, and first strike.",
                &[AbilityCostDef::Mana(mana_cost!(
                    "{R/W}{R/W}{R/W}{R/W}{R/W}{R/W}"
                ))],
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceMatches {
                        object: ObjectPredicateDef::Subtype("Warrior"),
                    },
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                                "Kithkin", "Spirit", "Warrior", "Avatar",
                            ])),
                            AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(8), ValueDef::Constant(8)),
                            AppliedEffectDef::add_ability(&abilities::flying()),
                            AppliedEffectDef::add_ability(&abilities::first_strike()),
                        ]),
                        duration: ResolvedEffectDurationDef::Permanent,
                    },
                },
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &FLICKERWISP,
    &RAVEN_S_CRIME,
    &DESECRATOR_HAG,
    &FIGURE_OF_DESTINY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
