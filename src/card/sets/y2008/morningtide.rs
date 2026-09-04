//! Morningtide card records.

use super::{CardRecord, PrintingRecord};
use crate::AbilityCostDef;
use crate::AbilityDef;
use crate::AbilityTargetDef;
use crate::AbilityTargetPredicate;
use crate::AppliedEffectDef;
use crate::CardRules;
use crate::CardSet;
use crate::CardType;
use crate::CardTypeSet;
use crate::CreatureTypeSetDef;
use crate::EffectDef;
use crate::EffectRecipientDef;
use crate::ManaColor;
use crate::ObjectPredicateDef;
use crate::ResolvedEffectDurationDef;
use crate::TargetIndex;
use crate::ValueDef;
use crate::ZoneKind;
use crate::ZonePlacement;
use crate::card::abilities;

use crate::mana_cost;

// MOR 31 — Disperse
pub(in crate::card::sets) static DISPERSE: CardRecord = CardRecord::new(
    CardSet::Morningtide,
    "Disperse",
    "0ae239b2-1596-4906-9711-1d180a246d35",
    "Steve Ellis",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target nonland permanent to its owner's hand.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
        },
    )),
);

// MOR 43 — Negate
pub(in crate::card::sets) static NEGATE: CardRecord = CardRecord::new(
    CardSet::Morningtide,
    "Negate",
    "5a501252-e722-4ebf-bcf7-f53a42745fa7",
    "Jeremy Jarvis",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::counter_target(
        "Counter target noncreature spell.",
        &AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::Spell,
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
            ]),
            zones: &[ZoneKind::Stack],
            controller: None,
            owner: None,
        }),
    )),
);

// MOR 92 — Kindled Fury
pub(in crate::card::sets) static KINDLED_FURY: CardRecord = CardRecord::new(
    CardSet::Morningtide,
    "Kindled Fury",
    "993956c9-30d8-41ee-84c2-c06d0512aea4",
    "Shelly Wan",
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +1/+0 and gains first strike until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                AppliedEffectDef::add_ability(&abilities::first_strike()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// MOR 143 — Door of Destinies
// Audit: unsupported — Predicates cannot consume a stored creature-type choice for both spell triggers and a counter-scaled continuous bonus.
pub(in crate::card::sets) static DOOR_OF_DESTINIES: CardRecord = CardRecord::new(
    crate::card::CardSet::Morningtide,
    "Door of Destinies",
    "ac4800be-5f77-42f5-914c-2a8e647e3af5",
    "Larry MacDougall",
    crate::card::CardRules::unsupported(),
);

// MOR 148 — Mutavault
pub(in crate::card::sets) static MUTAVAULT: CardRecord = CardRecord::new(
    CardSet::Morningtide,
    "Mutavault",
    "8ca3c48b-f104-4292-9a4e-2ce87a65893c",
    "Fred Fields",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{1}: This land becomes a 2/2 creature with all creature types until end of turn. It's still a land.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                // The animation keeps the land types Mutavault is printed with, so the
                // creature types are added rather than replacing anything.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::ALL),
                    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &DISPERSE,
    &NEGATE,
    &KINDLED_FURY,
    &DOOR_OF_DESTINIES,
    &MUTAVAULT,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
