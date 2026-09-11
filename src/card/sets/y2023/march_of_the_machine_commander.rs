//! March of the Machine Commander card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "MOC",
    slug: "march-of-the-machine-commander",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// MOC 30 — Death-Greeter's Champion
/// What backup lends, and what the Champion has printed on it either way: a
/// creature backing itself up gets the counter and nothing else, because
/// double strike is already there.
static CHAMPION_DOUBLE_STRIKE: AbilityDef = abilities::double_strike();

pub(in crate::card::sets) static DEATH_GREETER_S_CHAMPION: CardRecord = CardRecord::new(
    "Death-Greeter's Champion",
    "7cb2b582-1c45-4bb2-8aef-59a71a5a9e94",
    "Jason Rainville",
// Three mana for four damage a turn on its own, and a dash cost for the
    // turns when the double strike is better spent on something already out.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Warrior"], 2, 1)
        .with_abilities(&[
            abilities::dash(
                &[crate::CostDef::Mana(mana_cost!("{3}{R}"))],
                "Dash {3}{R} (You may cast this spell for its dash cost. If you do, it gains haste, and \
                it's returned from the battlefield to its owner's hand at the beginning of the next end \
                step.)",
            ),
            abilities::dashed_haste(),
            abilities::dashed_return(),
            abilities::backup(
                "Backup 1 (When this creature enters, put a +1/+1 counter on target creature. If that's \
                 another creature, it gains the following ability until end of turn.)",
                &abilities::backup_steps(1, &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&CHAMPION_DOUBLE_STRIKE),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                }),
            ),
            CHAMPION_DOUBLE_STRIKE,
        ]),
);

// MOC 34 — Path of the Pyromancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PATH_OF_THE_PYROMANCER_34: CardRecord = CardRecord::new(
    "Path of the Pyromancer",
    "4eeaf326-4521-4508-8032-627677a82dd4",
    "Dominik Mayer",
    crate::card::CardRules::unsupported(),
);

// MOC 118 — Hedron Detonator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEDRON_DETONATOR_118: CardRecord = CardRecord::new(
    "Hedron Detonator",
    "5194978a-ebc3-442f-97f3-012b3edd92da",
    "Caroline Gariba",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &DEATH_GREETER_S_CHAMPION,
    &PATH_OF_THE_PYROMANCER_34,
    &HEDRON_DETONATOR_118,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
