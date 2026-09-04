//! March of the Machine Commander card records required by supported formats.

use super::{CardRecord, PrintingRecord};
use crate::TargetIndex;
use crate::card::{
    AbilityDef, AppliedEffectDef, CardRules, CardSet, EffectDef, EffectRecipientDef,
    ResolvedEffectDurationDef, abilities,
};
use crate::mana_cost;

// MOC 30 — Death-Greeter's Champion
/// What backup lends, and what the Champion has printed on it either way: a
/// creature backing itself up gets the counter and nothing else, because
/// double strike is already there.
static CHAMPION_DOUBLE_STRIKE: AbilityDef = abilities::double_strike();

pub(in crate::card::sets) static DEATH_GREETER_S_CHAMPION: CardRecord = CardRecord::new(
    CardSet::MarchOfTheMachineCommander,
    "Death-Greeter's Champion",
    "7cb2b582-1c45-4bb2-8aef-59a71a5a9e94",
    "Jason Rainville",
    // Three mana for four damage a turn on its own, and a dash cost for the
    // turns when the double strike is better spent on something already out.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Warrior"], 2, 1)
        .with_abilities(&[
            abilities::dash(
                mana_cost!("{3}{R}"),
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&DEATH_GREETER_S_CHAMPION];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
