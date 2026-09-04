//! Conflux cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingRecord};
use crate::AbilityTargetPredicate;
use crate::AppliedRuleDef;
use crate::ControlDurationDef;
use crate::DiscardSelectionDef;
use crate::PlayerRefDef;
use crate::PlayerSetDef;
use crate::card::SacrificedAmountDef;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AddManaEffectDef, AppliedEffectDef,
    BasicLandType, CardRules, CardSet, CardSupertype, CardType, EffectDef, EffectRecipientDef,
    ManaColor, ManaTypeSetDef, ObjectPredicateDef, ObjectQueryDef, ObjectSetDef, PlayerRelation,
    ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// CON 3 — Aven Squire
pub(in crate::card::sets) static AVEN_SQUIRE: CardRecord = CardRecord::new(
    CardSet::Conflux,
    "Aven Squire",
    "60301dbd-40d1-4af8-8e2b-797febfa859f",
    "David Palumbo",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Bird", "Soldier"], 1, 1)
        .with_abilities(&[abilities::flying(), abilities::exalted()]),
);

// CON 5 — Celestial Purge
pub(in crate::card::sets) static CELESTIAL_PURGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Conflux,
    "Celestial Purge",
    "31c404e8-1241-4675-b259-fbbf1dba15c4",
    "David Palumbo",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target black or red permanent.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::Color(ManaColor::Black),
                ObjectPredicateDef::Color(ManaColor::Red),
            ]),
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Exile,
            placement: ZonePlacement::Top,
        },
    )),
);

// CON 15 — Path to Exile
pub(in crate::card::sets) static PATH_TO_EXILE: CardRecord = CardRecord::new(
    CardSet::Conflux,
    "Path to Exile",
    "29b7a8b1-b98e-483a-87a4-73bd831c03d4",
    "Todd Lockwood",
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target creature. Its controller may search their library for a basic land card, put that card onto the battlefield tapped, then shuffle.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
            EffectDef::May {
                player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
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
        ]),
    )),
);

// CON 60 — Canyon Minotaur
pub(in crate::card::sets) static CANYON_MINOTAUR: CardRecord = CardRecord::new(
    CardSet::Conflux,
    "Canyon Minotaur",
    "9b200790-43c7-42ae-9edf-89c8198a385b",
    "Steve Prescott",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Minotaur", "Warrior"], 3, 3),
);

// CON 87 — Noble Hierarch
pub(in crate::card::sets) static NOBLE_HIERARCH: CardRecord = CardRecord::new(
    CardSet::Conflux,
    "Noble Hierarch",
    "6adfe928-1305-444d-b709-1e714544daaf",
    "Mark Zug",
    // A one-mana accelerant for three colours whose body is beside the
    // point, except that exalted makes the 0/1 into a real attacker's
    // dividend on any turn nothing else attacks.
    CardRules::new_creature(mana_cost!("{G}"), &["Human", "Druid"], 0, 1).with_abilities(&[
        abilities::exalted(),
        AbilityDef::activated_mana(
            "{T}: Add {G}, {W}, or {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// CON 113 — Knight of the Reliquary
/// Land cards rather than creature cards, and your own graveyard: what she
/// sacrifices to fetch is what makes her bigger, so each activation pays
/// twice.
static RELIQUARY_LAND_CARDS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Land),
    &[ZoneKind::Graveyard],
    PlayerRelation::You,
);

pub(in crate::card::sets) static KNIGHT_OF_THE_RELIQUARY: CardRecord = CardRecord::new(
    CardSet::Conflux,
    "Knight of the Reliquary",
    "ad8b8518-c09e-4cb7-95b2-08e4e370d89c",
    "Michael Komarck",
    // Three mana for a body that grows a point every time it fetches, which
    // is what makes the utility lands in the deck worth a card each.
    CardRules::new_creature(mana_cost!("{1}{G}{W}"), &["Human", "Knight"], 2, 2)
        .with_abilities(&[
            AbilityDef::static_ability(
                "This creature gets +1/+1 for each land card in your graveyard.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountMatchingObjects(&RELIQUARY_LAND_CARDS),
                        ValueDef::CountMatchingObjects(&RELIQUARY_LAND_CARDS),
                    ),
                },
            ),
            AbilityDef::activated(
                "{T}, Sacrifice a Forest or Plains: Search your library for a land card, put it onto the \
                 battlefield, then shuffle.",
                &[
                    AbilityCostDef::TapSource,
                    AbilityCostDef::SacrificePermanent {
                        // A Forest or a Plains by basic land type rather than by name, so a dual
                        // with either type pays for her too.
                        object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest, BasicLandType::Plains]),
                        controller: PlayerRelation::You,
                    },
                ],
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: false,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    // Untapped, unlike the Wight's: the land she finds can be used
                    // the turn it arrives.
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
        ]),
);

// CON 120 — Nicol Bolas, Planeswalker
pub(in crate::card::sets) static NICOL_BOLAS_PLANESWALKER: CardRecord = CardRecord::new(
    CardSet::Conflux,
    "Nicol Bolas, Planeswalker",
    "48ee3939-bc12-4275-a446-9de36f0b4672",
    "D. Alexander Gregory",
    CardRules::new_planeswalker(mana_cost!("{4}{U}{B}{B}{R}"), &["Bolas"], 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "+3: Destroy target noncreature permanent.",
                &[AbilityCostDef::Loyalty(3)],
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                })],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
            ),
            AbilityDef::activated_with_targets(
                "−2: Gain control of target creature.",
                &[AbilityCostDef::Loyalty(-2)],
                &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))],
                EffectDef::GainControl {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    controller: PlayerRefDef::EffectController,
                    duration: ControlDurationDef::Indefinitely,
                },
            ),
            AbilityDef::activated_with_targets(
                "−9: Nicol Bolas deals 7 damage to target player or planeswalker. That player or that planeswalker's controller discards seven then sacrifices seven permanents of their choice.",
                &[AbilityCostDef::Loyalty(-9)],
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any))],
                EffectDef::Sequence(&[
                    EffectDef::DealDamage {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(7),
                    },
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(7),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                    EffectDef::SacrificeOfChoice {
                        player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                        object: ObjectPredicateDef::Any,
                        count: ValueDef::Constant(7),
                        then: None,
                        amount: SacrificedAmountDef::Power,
                        otherwise: None,
                        optional: false,
                    },
                ]),
            ),
        ]),
);

// CON 142 — Exotic Orchard
pub(in crate::card::sets) static EXOTIC_ORCHARD: CardRecord = CardRecord::new(
    CardSet::Conflux,
    "Exotic Orchard",
    "6aae6480-4e71-4d94-a648-f80d3849d792",
    "Steven Belledin",
    CardRules::new_land(&[]).with_ability(AbilityDef::activated_mana(
        "{T}: Add one mana of any color that a land an opponent controls could produce.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice_from(
            ManaTypeSetDef::could_be_produced_by(ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Land),
                &[ZoneKind::Battlefield],
                PlayerRelation::Opponent,
            )))
            .colors_only(),
        )),
    )),
);

// CON 143 — Reliquary Tower
pub(in crate::card::sets) static RELIQUARY_TOWER: CardRecord = CardRecord::new(
    CardSet::Conflux,
    "Reliquary Tower",
    "c5c0c1a5-dce7-4c7d-8a5b-0bf93ba68ace",
    "Jesper Ejsing",
    // "You", so it does nothing for the opponent, and it is read at cleanup
    // rather than captured -- losing the Tower on your own turn puts the
    // limit straight back.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::static_ability(
            "You have no maximum hand size.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                    crate::card::PlayerRuleDef::NoMaximumHandSize,
                )),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AVEN_SQUIRE,
    &CELESTIAL_PURGE,
    &PATH_TO_EXILE,
    &CANYON_MINOTAUR,
    &NOBLE_HIERARCH,
    &KNIGHT_OF_THE_RELIQUARY,
    &NICOL_BOLAS_PLANESWALKER,
    &EXOTIC_ORCHARD,
    &RELIQUARY_TOWER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
