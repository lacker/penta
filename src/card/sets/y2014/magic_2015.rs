//! Magic 2015 cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, CardArt, CardRules, CardSet, CardType, EffectDef,
    EffectRecipientDef, ManaColor, ObjectPredicateDef, ObjectQueryDef, ObjectSetDef,
    PlayerRelation, ResolvedEffectDurationDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind,
    abilities, cards,
};
use crate::mana_cost;

/// "Other Goblin creatures you control": the Rabblemaster is a Goblin too
/// and is not made to attack by its own clause.
static OTHER_GOBLINS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Subtype("Goblin"),
        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

static GOBLINS_MUST_ATTACK: AbilityDef =
    abilities::attacks_each_combat_if_able("This creature attacks each combat if able.");

/// Every other Goblin in the attack, whoever controls it. The count is read
/// as the trigger resolves, so a Goblin that was removed in response is not
/// among them.
static OTHER_ATTACKING_GOBLINS: ObjectQueryDef = ObjectQueryDef::new(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::Subtype("Goblin"),
        ObjectPredicateDef::Attacking,
        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
    ]),
    &[ZoneKind::Battlefield],
);

static GOBLIN_RABBLEMASTER_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::static_ability(
        "Other Goblin creatures you control attack each combat if able.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::objects(ObjectSetDef::Query(OTHER_GOBLINS_YOU_CONTROL)),
            effect: AppliedEffectDef::add_ability(&GOBLINS_MUST_ATTACK),
        },
    ),
    AbilityDef::triggered(
        "At the beginning of combat on your turn, create a 1/1 red Goblin creature token with \
         haste.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::BeginningOfCombat,
            player: PlayerRelation::You,
        },
        EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1)
            .with_abilities(&[abilities::haste()])
            .with_art(CardArt::new(
                "98993a45-4aff-4f9b-a030-7d72fbb4ec6c",
                "Karl Kopinski",
            )),
    ),
    AbilityDef::triggered(
        "Whenever this creature attacks, it gets +1/+0 until end of turn for each other attacking \
         Goblin.",
        TriggerEventDef::attacks(ObjectPredicateDef::Source),
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::CountMatchingObjects(&OTHER_ATTACKING_GOBLINS),
                ValueDef::Constant(0),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    ),
];

// M15 145 — Goblin Rabblemaster
pub(in crate::card::sets) static GOBLIN_RABBLEMASTER: CardRecord = CardRecord::new(
    cards::GOBLIN_RABBLEMASTER,
    "Goblin Rabblemaster",
    CardArt::new("ee9c697e-d2c0-413b-9142-ecf5d7cf5322", "Svetlin Velinov"),
    CardSet::Magic2015,
    // Three mana that makes a Goblin every turn and then sends the whole
    // pile in whether or not that was the plan.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Warrior"], 2, 2)
        .with_abilities(&GOBLIN_RABBLEMASTER_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&GOBLIN_RABBLEMASTER];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
