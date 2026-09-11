//! Magic 2015 cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AppliedEffectDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new("M15", "magic-2015");

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// M15 14 — Heliod's Pilgrim
pub(in crate::card::sets) static HELIOD_S_PILGRIM: CardRecord = CardRecord::new(
    "Heliod's Pilgrim",
    "7ea54b97-9182-4d46-9d70-3cc7f9b18ada",
    "Izzy",
    // The body is beside the point: this is a three-mana tutor that an Aura
    // deck plays for whichever Aura the board asks for.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Cleric"], 1, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, you may search your library for an Aura card, reveal it, \
             put it into your hand, then shuffle.",
            // Two ways to decline: the outer may, and a minimum of zero for a
            // search that finds nothing worth taking.
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::Subtype("Aura"),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            },
        ),
    ),
);

// M15 40 — Triplicate Spirits
pub(in crate::card::sets) static TRIPLICATE_SPIRITS: CardRecord = CardRecord::new(
    "Triplicate Spirits",
    "3d6498d3-bf1f-4bf1-a602-7c21fb44c106",
    "Izzy",
    // Six mana printed, but the tokens it already made are what pay for the
    // next copy, so the real cost falls every time a token deck casts it.
    CardRules::new_sorcery(mana_cost!("{4}{W}{W}")).with_abilities(&[
        abilities::convoke(),
        AbilityDef::spell(
            "Create three 1/1 white Spirit creature tokens with flying.",
            EffectDef::create_creature_token(&["Spirit"], &[ManaColor::White], 1, 1)
                .with_abilities(&[abilities::flying()])
                .with_amount(3),
        ),
    ]),
);

// M15 142 — Frenzied Goblin (reprint)
const FRENZIED_GOBLIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2005::ravnica_city_of_guilds::FRENZIED_GOBLIN,
    "7ddfe382-3a80-45f3-a022-54739c4b69a6",
    "Carl Critchlow",
);

// M15 145 — Goblin Rabblemaster
pub(in crate::card::sets) static GOBLIN_RABBLEMASTER: CardRecord = CardRecord::new(
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
                    effect: AppliedEffectDef::add_ability(&abilities::attacks_each_combat_if_able()),
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
