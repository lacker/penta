//! March of the Machine cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, ActivationTimingDef, CardArt, CardRules, CardSet, CardSupertype,
    CardType, CounterKind, DrawEventMatcherDef, EffectDef, EffectRecipientDef,
    ExiledCastPermissionDef, ManaColor, ObjectPredicateDef, ObjectQueryDef, ObjectSetDef,
    PlayerRelation, PlayerSetDef, TokenCountersDef, TriggerEventDef, ValueDef, ZoneKind,
    ZonePlacement, abilities, tokens,
};
use crate::ids::ParentBinding;
use crate::mana_cost;

// MOM 3 — Alabaster Host Intercessor
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ALABASTER_HOST_INTERCESSOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("165357cc-ec74-490f-aec3-7048bb43c8f9"),
    "Alabaster Host Intercessor",
    crate::card::CardArt::new("165357cc-ec74-490f-aec3-7048bb43c8f9", "Konstantin Porubov"),
    crate::card::CardSet::MarchOfTheMachine,
    crate::card::CardRules::unsupported(),
);

// MOM 40 — Sunfall
pub(in crate::card::sets) static SUNFALL: CardRecord = CardRecord::new_with_legacy_id(
    2258,
    "Sunfall",
    CardArt::new(
        "32e29c7d-ed4b-4eff-b3c2-d99e5b63ef8d",
        "Kasia 'Kafis' Zielińska",
    ),
    CardSet::MarchOfTheMachine,
    // A wrath that exiles rather than destroys, and hands the caster the
    // biggest thing on the empty board it just made.
    CardRules::new_sorcery(mana_cost!("{3}{W}{W}")).with_ability(AbilityDef::spell(
        "Exile all creatures. Incubate X, where X is the number of creatures exiled this way. \
         (Create an Incubator token with X +1/+1 counters on it and \"{2}: Transform this \
         token.\" It transforms into a 0/0 Phyrexian artifact creature.)",
        abilities::bind_objects_then(
            // Everyone's, which is what "all creatures" means.
            crate::card::ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Query(
                ObjectQueryDef::new(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                ),
            )),
            // The creatures are bound before they move, because "X, where X is the
            // number of creatures exiled this way" asks about a set the board no longer
            // holds by the time the token is made.
            &EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                },
                // Incubate X. One token however large X is, and X of zero still makes
                // one: the keyword creates the token unconditionally.
                EffectDef::create_token(tokens::incubator())
                    .with_art(CardArt::new(
                        "2c5ed737-657b-43bf-b222-941da7579a4a",
                        "Johann Bodin",
                    ))
                    .with_counters(TokenCountersDef {
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::BoundObjectCount(ParentBinding),
                    }),
            ]),
        ),
    )),
);

// MOM 58 — Faerie Mastermind
pub(in crate::card::sets) static FAERIE_MASTERMIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("52d3005f-a1c7-4ef5-911f-ccc0752f4181"),
    "Faerie Mastermind",
    CardArt::new("52d3005f-a1c7-4ef5-911f-ccc0752f4181", "Joshua Raphael"),
    CardSet::MarchOfTheMachine,
    // A two-mana flash flier that is never a dead card: it taxes every
    // cantrip the other deck was going to cast anyway, and turns into a
    // draw engine once there is nothing else to spend mana on.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Faerie", "Rogue"], 2, 1).with_abilities(&[
        abilities::flash(),
        abilities::flying(),
        // The ordinal is the whole clause: their first card each turn is the one
        // the rules hand them, so this catches the extra one and nothing else.
        AbilityDef::triggered(
            "Whenever an opponent draws their second card each turn, you draw a card.",
            TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(
                PlayerRelation::Opponent,
                2,
            )),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        // Symmetrical on purpose: with the trigger above out, the copy they draw
        // is the one that draws you another.
        AbilityDef::activated(
            "{3}{U}: Each player draws a card.",
            &[AbilityCostDef::Mana(mana_cost!("{3}{U}"))],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::players(PlayerSetDef::All),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// MOM 66 — Meeting of Minds
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MEETING_OF_MINDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("508b8650-c283-4e54-abdc-32ec2fb1ee34"),
    "Meeting of Minds",
    crate::card::CardArt::new("508b8650-c283-4e54-abdc-32ec2fb1ee34", "Milivoj Ćeran"),
    crate::card::CardSet::MarchOfTheMachine,
    crate::card::CardRules::unsupported(),
);

// MOM 73 — Preening Champion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PREENING_CHAMPION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("44178ece-af31-4a94-88bc-c9ce43bb4573"),
    "Preening Champion",
    crate::card::CardArt::new("44178ece-af31-4a94-88bc-c9ce43bb4573", "Alix Branwyn"),
    crate::card::CardSet::MarchOfTheMachine,
    crate::card::CardRules::unsupported(),
);

// MOM 173 — Wrenn's Resolve
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WRENN_S_RESOLVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9a47999c-12d5-4e1a-a9c1-40a1757007f1"),
    "Wrenn's Resolve",
    crate::card::CardArt::new("9a47999c-12d5-4e1a-a9c1-40a1757007f1", "Viko Menezes"),
    crate::card::CardSet::MarchOfTheMachine,
    crate::card::CardRules::unsupported(),
);

// MOM 298 — Etali, Primal Conqueror // Etali, Primal Sickness
pub(in crate::card::sets) static ETALI_PRIMAL_CONQUEROR: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("3e97c609-3932-4428-96d4-1c97e61f0abb"),
    "Etali, Primal Conqueror // Etali, Primal Sickness",
    CardArt::new("3e97c609-3932-4428-96d4-1c97e61f0abb", "Yeong-Hao Han"),
    CardSet::MarchOfTheMachine,
    // Seven mana that casts the two best cards on the table, and a back face
    // nobody in the cube ever pays for.
    &[
        (
            "Etali, Primal Conqueror",
            const {
                CardRules::new_creature(mana_cost!("{5}{R}{R}"), &const { ["Elder", "Dinosaur"] }, 7, 7)
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&const { [
                    abilities::trample(),
                    abilities::enters_trigger(
                        "When this creature enters, each player exiles cards from the top of their library until \
                         they exile a nonland card. You may cast any number of spells from among the nonland \
                         cards exiled this way without paying their mana costs.",
                        // Both libraries, and the permission is always Etali's controller's: what
                        // their library turned up is yours to cast.
                        //
                        // The printed clause states no duration, which means the casting happens as
                        // the ability resolves: a card left uncast stays in exile uncastable rather
                        // than waiting for later in the turn. Every card it turned up is offered at
                        // once, so the order they are cast in is the caster's.
                        EffectDef::ExileFromTopUntil {
                            player: EffectRecipientDef::EachPlayer,
                            object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                            permission: ExiledCastPermissionDef::FreeWhileResolving,
                        },
                    ),
                    AbilityDef::activated(
                        "{9}{G/P}: Transform this creature. Activate only as a sorcery.",
                        &const { [AbilityCostDef::Mana(mana_cost!("{9}{G/P}"))] },
                        EffectDef::Transform {
                            object: EffectRecipientDef::Source,
                        },
                    )
                    .with_activation_timing(ActivationTimingDef::SorcerySpeed),
                ] })
            },
        ),
        (
            "Etali, Primal Sickness",
            const {
                CardRules::new_creature_without_mana_cost(&const { ["Phyrexian", "Elder", "Dinosaur"] }, 11, 11)
                .with_supertype(CardSupertype::Legendary)
                .printed_colors(&const { [ManaColor::Green] })
                .with_abilities(&const { [
                    abilities::trample(),
                    abilities::indestructible(),
                    AbilityDef::triggered(
                        "Whenever this creature deals combat damage to a player, they get that many poison \
                         counters.",
                        TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                        // "They get that many poison counters": the amount is the damage that was
                        // dealt rather than the creature's power, which is what makes a blocked
                        // trampler give exactly what got through.
                        EffectDef::AddPlayerCounters {
                            recipient: EffectRecipientDef::EventPlayer,
                            kind: CounterKind::Poison,
                            amount: ValueDef::TriggerEventAmount,
                        },
                    ),
                ] })
            },
        ),
    ],
);

// MOM 328 — Zephyr Winder
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ZEPHYR_WINDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("14456a8e-016c-4407-8410-c490db3f5ea9"),
    "Zephyr Winder",
    crate::card::CardArt::new("14456a8e-016c-4407-8410-c490db3f5ea9", "Jana Schirmer"),
    crate::card::CardSet::MarchOfTheMachine,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ALABASTER_HOST_INTERCESSOR,
    &SUNFALL,
    &FAERIE_MASTERMIND,
    &MEETING_OF_MINDS,
    &PREENING_CHAMPION,
    &WRENN_S_RESOLVE,
    &ETALI_PRIMAL_CONQUEROR,
    &ZEPHYR_WINDER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
