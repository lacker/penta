//! Magic 2015 cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, CardArt, CardRules, CardSet, CardType, EffectDef,
    EffectRecipientDef, ManaColor, ObjectPredicateDef, ObjectQueryDef, ObjectSetDef,
    PlayerRelation, ResolvedEffectDurationDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind,
    abilities,
};
use crate::mana_cost;

// M15 14 — Heliod's Pilgrim
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HELIOD_S_PILGRIM: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2015,
    "Heliod's Pilgrim",
    "7ea54b97-9182-4d46-9d70-3cc7f9b18ada",
    "Izzy",
    crate::card::CardRules::unsupported(),
);

// M15 40 — Triplicate Spirits
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRIPLICATE_SPIRITS: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2015,
    "Triplicate Spirits",
    "3d6498d3-bf1f-4bf1-a602-7c21fb44c106",
    "Izzy",
    crate::card::CardRules::unsupported(),
);

// M15 142 — Frenzied Goblin (reprint)
const FRENZIED_GOBLIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2005::ravnica_city_of_guilds::FRENZIED_GOBLIN,
    "7ddfe382-3a80-45f3-a022-54739c4b69a6",
    "Carl Critchlow",
);

// M15 145 — Goblin Rabblemaster
pub(in crate::card::sets) static GOBLIN_RABBLEMASTER: CardRecord = CardRecord::new(
    CardSet::Magic2015,
    "Goblin Rabblemaster",
    "ee9c697e-d2c0-413b-9142-ecf5d7cf5322",
    "Svetlin Velinov",
    // Three mana that makes a Goblin every turn and then sends the whole
    // pile in whether or not that was the plan.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Warrior"], 2, 2)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Other Goblin creatures you control attack each combat if able.",
                EffectDef::StaticApply {
                    // "Other Goblin creatures you control": the Rabblemaster is a Goblin too
                    // and is not made to attack by its own clause.
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Goblin"),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ))),
                    effect: AppliedEffectDef::add_ability(&abilities::attacks_each_combat_if_able("This creature attacks each combat if able.")),
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
                        // Every other Goblin in the attack, whoever controls it. The count is read
                        // as the trigger resolves, so a Goblin that was removed in response is not
                        // among them.
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::new(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Subtype("Goblin"),
                                ObjectPredicateDef::Attacking,
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                        )),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&HELIOD_S_PILGRIM, &TRIPLICATE_SPIRITS, &GOBLIN_RABBLEMASTER];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[FRENZIED_GOBLIN_REPRINT];
