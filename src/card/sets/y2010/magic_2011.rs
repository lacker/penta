//! Magic 2011 cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet, CardType,
    CastTimingPermissionDef, DiscardSelectionDef, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::mana_cost;

// M11 21 — Leyline of Sanctity
// Audit: partial — The opening-hand action is declarative; player hexproof needs a static player-protection effect.
pub(in crate::card::sets) static LEYLINE_OF_SANCTITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("262de9ae-d641-4f0e-af6a-03ce0e1c91d3"),
    "Leyline of Sanctity",
    CardArt::new("262de9ae-d641-4f0e-af6a-03ce0e1c91d3", "Ryan Pancoast"),
    CardSet::Magic2011,
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_abilities(&[
        abilities::begin_game_on_battlefield("If this card is in your opening hand, you may begin the game with it on the battlefield."),
        AbilityDef::not_implemented("You have hexproof. (You can't be the target of spells or abilities your opponents control.)", "Needs a static player-protection effect that grants hexproof to the controller."),
    ]),
);

// M11 30 — Silence
// Audit: metadata-only — Card rules have not been implemented.
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
        abilities::begin_game_on_battlefield("If this card is in your opening hand, you may begin the game with it on the battlefield."),
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
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MERFOLK_SPY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b5ae05cc-116b-4268-ba78-709aeff36ab1"),
    "Merfolk Spy",
    crate::card::CardArt::new(
        "b5ae05cc-116b-4268-ba78-709aeff36ab1",
        "Matt Cavotta & Richard Whitters",
    ),
    crate::card::CardSet::Magic2011,
    crate::card::CardRules::unsupported(),
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
// Audit: metadata-only — Card rules have not been implemented.
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
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYLACTERY_LICH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d088983-92c1-4f4d-8abf-dd20347495b5"),
    "Phylactery Lich",
    crate::card::CardArt::new("9d088983-92c1-4f4d-8abf-dd20347495b5", "Michael Komarck"),
    crate::card::CardSet::Magic2011,
    crate::card::CardRules::unsupported(),
);

// M11 148 — Leyline of Punishment
// Audit: partial — The opening-hand action is declarative; the permanent global life-gain and damage-prevention prohibitions need static player rules.
pub(in crate::card::sets) static LEYLINE_OF_PUNISHMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("51a2eec5-f892-4466-b6c6-960626ba5640"),
    "Leyline of Punishment",
    CardArt::new("51a2eec5-f892-4466-b6c6-960626ba5640", "Charles Urbach"),
    CardSet::Magic2011,
    CardRules::new_enchantment(mana_cost!("{2}{R}{R}")).with_abilities(&[
        abilities::begin_game_on_battlefield("If this card is in your opening hand, you may begin the game with it on the battlefield."),
        AbilityDef::not_implemented("Players can't gain life.\nDamage can't be prevented.", "Needs permanent static rules applying both prohibitions to the whole game."),
    ]),
);

// M11 177 — Garruk's Packleader
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GARRUK_S_PACKLEADER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dfaef299-7879-4f52-8ee4-701ed150b930"),
    "Garruk's Packleader",
    crate::card::CardArt::new("dfaef299-7879-4f52-8ee4-701ed150b930", "Nils Hamm"),
    crate::card::CardSet::Magic2011,
    crate::card::CardRules::unsupported(),
);

// M11 183 — Leyline of Vitality
// Audit: partial — The opening-hand action is declarative; its global toughness boost and optional creature-entry life trigger remain unsupported.
pub(in crate::card::sets) static LEYLINE_OF_VITALITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f5318113-9dfb-492c-9151-de90951d881e"),
    "Leyline of Vitality",
    CardArt::new("f5318113-9dfb-492c-9151-de90951d881e", "Jim Nelson"),
    CardSet::Magic2011,
    CardRules::new_enchantment(mana_cost!("{2}{G}{G}")).with_abilities(&[
        abilities::begin_game_on_battlefield("If this card is in your opening hand, you may begin the game with it on the battlefield."),
        AbilityDef::not_implemented("Creatures you control get +0/+1.\nWhenever a creature you control enters, you may gain 1 life.", "Needs the global static boost plus an optional trigger over other creatures entering."),
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
