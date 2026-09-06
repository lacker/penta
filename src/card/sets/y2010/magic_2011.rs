//! Magic 2011 cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, AppliedRuleDef, BasicLandType, CardArt, CardRules, CardSet,
    CardType, CastTimingPermissionDef, DiscardSelectionDef, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, PlayerRelation, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::mana_cost;

// M11 21 — Leyline of Sanctity
pub(in crate::card::sets) static LEYLINE_OF_SANCTITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("262de9ae-d641-4f0e-af6a-03ce0e1c91d3"),
    "Leyline of Sanctity",
    CardArt::new("262de9ae-d641-4f0e-af6a-03ce0e1c91d3", "Ryan Pancoast"),
    CardSet::Magic2011,
    // Four mana for nothing at all, or nothing at all for a wall the
    // discard and the burn cannot see past.
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_abilities(&[
        abilities::begin_game_on_battlefield(),
        AbilityDef::static_ability(
            "You have hexproof. (You can't be the target of spells or abilities your opponents control.)",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                // The player, not the permanents: what this stops is a spell
                // that names its controller, and nothing that names a
                // creature they control.
                effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                    crate::card::PlayerRuleDef::Hexproof,
                )),
            },
        ),
    ]),
);

// M11 30 — Silence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1559d660-8a9d-422b-95d3-710a046583dd"),
    "Silence",
    crate::card::CardArt::new("37b70d17-e4ec-4731-8892-b444f82be7a2", "Wayne Reynolds"),
    crate::card::CardSet::Magic2011,
    crate::card::CardRules::unsupported(),
);

// M11 61 — Leyline of Anticipation
pub(in crate::card::sets) static LEYLINE_OF_ANTICIPATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d7dbb092-3bb0-445e-ab26-d939cac92a73"),
    "Leyline of Anticipation",
    CardArt::new("d7dbb092-3bb0-445e-ab26-d939cac92a73", "Charles Urbach"),
    CardSet::Magic2011,
    CardRules::new_enchantment(mana_cost!("{2}{U}{U}")).with_abilities(&[
        abilities::begin_game_on_battlefield(),
        AbilityDef::static_ability(
            "You may cast spells as though they had flash.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayCastAsThoughItHadFlash(
                    CastTimingPermissionDef::new(ObjectPredicateDef::Any),
                )),
            },
        ),
    ]),
);

// M11 66 — Merfolk Spy
pub(in crate::card::sets) static MERFOLK_SPY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b5ae05cc-116b-4268-ba78-709aeff36ab1"),
    "Merfolk Spy",
    CardArt::new(
        "b5ae05cc-116b-4268-ba78-709aeff36ab1",
        "Matt Cavotta & Richard Whitters",
    ),
    CardSet::Magic2011,
    // Islandwalk against the deck it most wants to look at, and the reveal
    // is at random, so it reports rather than chooses.
    CardRules::new_creature(mana_cost!("{U}"), &["Merfolk", "Rogue"], 1, 1).with_abilities(&[
        abilities::landwalk(BasicLandType::Island),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, that player reveals a card at random from their hand.",
            TriggerEventDef::CombatDamageDealtToPlayers {
                sources: ObjectPredicateDef::Source,
                players: PlayerRelation::Opponent,
            },
            EffectDef::RevealAtRandomFromHand {
                player: EffectRecipientDef::Opponent,
            },
        ),
    ]),
);

// M11 70 — Preordain
pub(in crate::card::sets) static PREORDAIN: CardRecord = CardRecord::new_with_legacy_id(
    2130,
    "Preordain",
    CardArt::new("e3868c3d-4fcd-444b-866f-0f8e50ce7b67", "Svetlin Velinov"),
    CardSet::Magic2011,
    CardRules::new_sorcery(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Scry 2, then draw a card.",
        EffectDef::Sequence(&[
            abilities::scry(ValueDef::Constant(2)),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// M11 74 — Stormtide Leviathan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORMTIDE_LEVIATHAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0e7f3fb6-93ce-4bc9-8efd-11af5a46218f"),
    "Stormtide Leviathan",
    crate::card::CardArt::new("0e7f3fb6-93ce-4bc9-8efd-11af5a46218f", "Karl Kopinski"),
    crate::card::CardSet::Magic2011,
    crate::card::CardRules::unsupported(),
);

// M11 104 — Liliana's Specter
pub(in crate::card::sets) static LILIANA_S_SPECTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("33122581-39fd-44a0-b928-f73e39a0c0f1"),
    "Liliana's Specter",
    crate::card::CardArt::new("33122581-39fd-44a0-b928-f73e39a0c0f1", "Vance Kovacs"),
    crate::card::CardSet::Magic2011,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Specter"], 2, 1).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, each opponent discards a card.",
            EffectDef::Discard {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ]),
);

// M11 110 — Phylactery Lich
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYLACTERY_LICH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d088983-92c1-4f4d-8abf-dd20347495b5"),
    "Phylactery Lich",
    crate::card::CardArt::new("9d088983-92c1-4f4d-8abf-dd20347495b5", "Michael Komarck"),
    crate::card::CardSet::Magic2011,
    crate::card::CardRules::unsupported(),
);

// M11 148 — Leyline of Punishment
pub(in crate::card::sets) static LEYLINE_OF_PUNISHMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("51a2eec5-f892-4466-b6c6-960626ba5640"),
    "Leyline of Punishment",
    CardArt::new("51a2eec5-f892-4466-b6c6-960626ba5640", "Charles Urbach"),
    CardSet::Magic2011,
    CardRules::new_enchantment(mana_cost!("{2}{R}{R}")).with_abilities(&[
        abilities::begin_game_on_battlefield(),
        AbilityDef::static_ability(
            "Players can't gain life. Damage can't be prevented.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::EachPlayer,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Rule(AppliedRuleDef::CannotGainLife),
                    AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                        crate::card::PlayerRuleDef::DamageCannotBePrevented,
                    )),
                ]),
            },
        ),
    ]),
);

// M11 177 — Garruk's Packleader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GARRUK_S_PACKLEADER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dfaef299-7879-4f52-8ee4-701ed150b930"),
    "Garruk's Packleader",
    crate::card::CardArt::new("dfaef299-7879-4f52-8ee4-701ed150b930", "Nils Hamm"),
    crate::card::CardSet::Magic2011,
    crate::card::CardRules::unsupported(),
);

// M11 183 — Leyline of Vitality
pub(in crate::card::sets) static LEYLINE_OF_VITALITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f5318113-9dfb-492c-9151-de90951d881e"),
    "Leyline of Vitality",
    CardArt::new("f5318113-9dfb-492c-9151-de90951d881e", "Jim Nelson"),
    CardSet::Magic2011,
    CardRules::new_enchantment(mana_cost!("{2}{G}{G}")).with_abilities(&[
        abilities::begin_game_on_battlefield(),
        AbilityDef::static_ability(
            "Creatures you control get +0/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    crate::card::PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
            },
        ),
        AbilityDef::triggered(
            "Whenever a creature you control enters, you may gain 1 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(crate::card::PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            },
        ),
    ]),
);

// M11 192 — Primeval Titan
pub(in crate::card::sets) static PRIMEVAL_TITAN: CardRecord = CardRecord::new_with_legacy_id(
    2128,
    "Primeval Titan",
    CardArt::new("feee9327-b937-46ba-a2aa-6c015ab6cdd5", "Aleksi Briclot"),
    CardSet::Magic2011,
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Giant"], 6, 6).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "Whenever this creature enters or attacks, you may search your library for up to two land cards, put them onto the battlefield tapped, then shuffle.",
            // One printed ability with two ways in, not two abilities: the card says
            // "enters or attacks", and a Titan that does both in a turn triggers twice
            // for the same reason it would have anyway.
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
            ]),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                // Any land card, not just a basic: the two it finds are usually the two the
                // deck was built around.
                effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    minimum: 0,
                    maximum: ValueDef::Constant(2),
                    reveal: false,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: true,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &LEYLINE_OF_SANCTITY,
    &SILENCE,
    &LEYLINE_OF_ANTICIPATION,
    &MERFOLK_SPY,
    &PREORDAIN,
    &STORMTIDE_LEVIATHAN,
    &LILIANA_S_SPECTER,
    &PHYLACTERY_LICH,
    &LEYLINE_OF_PUNISHMENT,
    &GARRUK_S_PACKLEADER,
    &LEYLINE_OF_VITALITY,
    &PRIMEVAL_TITAN,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
