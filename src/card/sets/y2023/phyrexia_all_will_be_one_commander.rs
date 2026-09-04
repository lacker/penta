//! Phyrexia: All Will Be One Commander cards cataloged for the Vintage Cube
//! pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, BattlefieldEntryModificationDef, CardRules, CardSet, CardSupertype,
    CardType, CounterKind, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    PlayerRelation, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::mana_cost;

// ONC 6 — Glimmer Lens
pub(in crate::card::sets) static GLIMMER_LENS: CardRecord = CardRecord::new(
    CardSet::PhyrexiaAllWillBeOneCommander,
    "Glimmer Lens",
    "c9262000-e6f3-4da1-ad1c-038f65d3bef6",
    "Sidharth Chaturvedi",
    CardRules::new_artifact(mana_cost!("{1}{W}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::for_mirrodin(),
            // "And at least one other creature" is the whole declaration
            // being two or more: the Rebel it brought is one attacker, so
            // the card asks for a second body before it draws.
            AbilityDef::triggered(
                "Whenever equipped creature and at least one other creature attack, draw a card.",
                TriggerEventDef::attacks_in_declaration(
                    ObjectPredicateDef::AttachedToSource,
                    2,
                    None,
                ),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{1}{W}"))],
                "Equip {1}{W}",
            ),
        ]),
);

// ONC 39 — Otharri, Suns' Glory
pub(in crate::card::sets) static OTHARRI_SUNS_GLORY: CardRecord = CardRecord::new(
    CardSet::PhyrexiaAllWillBeOneCommander,
    "Otharri, Suns' Glory",
    "348e0927-1d8f-4723-879d-f7e95ac60c27",
    "Marta Nael",
    // Five mana for a hasty lifelinking flier that pays out more every time
    // it connects, and buys itself back out of the graveyard with what it
    // left behind.
    CardRules::new_creature(mana_cost!("{3}{R}{W}"), &["Phoenix"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::lifelink(),
            abilities::haste(),
            AbilityDef::triggered(
                "Whenever this creature attacks, you get an experience counter. Then create a 2/2 red \
                 Rebel creature token that's tapped and attacking for each experience counter you have.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                // "Then" is the order that matters: the counter is his first, so the
                // attack he arrives on already makes one Rebel.
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Controller,
                        kind: CounterKind::named("experience"),
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::create_creature_token(&["Rebel"], &[ManaColor::Red], 2, 2)
                        .entering_tapped()
                        .entering_attacking()
                        // The counter goes on the player, not on him: it stays through his death
                        // and counts for whatever he comes back to.
                        .with_count(ValueDef::PlayerCounters {
                            player: PlayerRelation::You,
                            kind: CounterKind::named("experience"),
                        }),
                ]),
            ),
            AbilityDef::activated(
                "{2}{R}{W}, Tap an untapped Rebel you control: Return this card from your graveyard to \
                 the battlefield tapped.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{2}{R}{W}")),
                    AbilityCostDef::TapPermanents {
                        object: ObjectPredicateDef::Subtype("Rebel"),
                        controller: PlayerRelation::You,
                        count: 1,
                    },
                ],
                EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::MoveToZone {
                        object: EffectRecipientDef::Source,
                        zone: ZoneKind::Battlefield,
                        placement: ZonePlacement::Top,
                    },
                    arrival: crate::card::BattlefieldArrivalDef {
                        modifications: &[BattlefieldEntryModificationDef::Tapped],
                        ..crate::card::BattlefieldArrivalDef::DEFAULT
                    },
                },
            )
            .with_source_zones(&[ZoneKind::Graveyard]),
        ]),
);

// ONC 48 — Staff of the Storyteller
pub(in crate::card::sets) static STAFF_OF_THE_STORYTELLER: CardRecord = CardRecord::new(
    CardSet::PhyrexiaAllWillBeOneCommander,
    "Staff of the Storyteller",
    "17be11f2-f2db-40c4-8fc1-2ed7173f9a1a",
    "Dan Murayama Scott",
    // Two mana for a flier, and a card for every turn the deck keeps making
    // tokens afterwards.
    CardRules::new_artifact(mana_cost!("{1}{W}")).with_abilities(&[
        abilities::enters_trigger(
            "When this artifact enters, create a 1/1 white Spirit creature token with flying.",
            // The Staff pays for itself the moment it lands: the Spirit it makes is a
            // creature token you created, so its own trigger sees it.
            EffectDef::create_creature_token(&["Spirit"], &[ManaColor::White], 1, 1)
                    .with_abilities(&[abilities::flying()]),
        ),
        AbilityDef::triggered(
            "Whenever you create one or more creature tokens, put a story counter on this artifact.",
            // One instruction, one counter, however many tokens it made -- which is
            // what "one or more" says and what makes a wide token maker no better here
            // than a narrow one.
            TriggerEventDef::TokensCreated {
                player: PlayerRelation::You,
                token: ObjectPredicateDef::HasType(CardType::Creature),
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("story"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "{W}, {T}, Remove a story counter from this artifact: Draw a card.",
            &[
                AbilityCostDef::Mana(mana_cost!("{W}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("story"),
                    amount: 1,
                },
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &GLIMMER_LENS,
    &OTHARRI_SUNS_GLORY,
    &STAFF_OF_THE_STORYTELLER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
