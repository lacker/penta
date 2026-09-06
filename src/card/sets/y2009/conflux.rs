//! Conflux cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AddManaEffectDef, AppliedEffectDef, BasicLandType, CardArt,
    CardRules, CardSet, CardSupertype, CardType, CostDef, EffectDef, EffectRecipientDef, ManaColor,
    ManaTypeSetDef, ObjectPredicateDef, ObjectQueryDef, ObjectSetDef, PlayerRelation, ValueDef,
    ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// CON 15 — Path to Exile
pub(in crate::card::sets) static PATH_TO_EXILE: CardRecord = CardRecord::new_with_legacy_id(
    2189,
    "Path to Exile",
    CardArt::new("29b7a8b1-b98e-483a-87a4-73bd831c03d4", "Todd Lockwood"),
    CardSet::Conflux,
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

// CON 87 — Noble Hierarch
pub(in crate::card::sets) static NOBLE_HIERARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6adfe928-1305-444d-b709-1e714544daaf"),
    "Noble Hierarch",
    CardArt::new("6adfe928-1305-444d-b709-1e714544daaf", "Mark Zug"),
    CardSet::Conflux,
    // A one-mana accelerant for three colours whose body is beside the
    // point, except that exalted makes the 0/1 into a real attacker's
    // dividend on any turn nothing else attacks.
    CardRules::new_creature(mana_cost!("{G}"), &["Human", "Druid"], 0, 1).with_abilities(&[
        abilities::exalted(),
        AbilityDef::activated_mana(
            "{T}: Add {G}, {W}, or {U}.",
            &[CostDef::TapSource],
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
    PrintingAnchor::scryfall("ad8b8518-c09e-4cb7-95b2-08e4e370d89c"),
    "Knight of the Reliquary",
    CardArt::new("ad8b8518-c09e-4cb7-95b2-08e4e370d89c", "Michael Komarck"),
    CardSet::Conflux,
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
                    CostDef::TapSource,
                    CostDef::SacrificePermanent {
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

// CON 142 — Exotic Orchard
pub(in crate::card::sets) static EXOTIC_ORCHARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6aae6480-4e71-4d94-a648-f80d3849d792"),
    "Exotic Orchard",
    CardArt::new("6aae6480-4e71-4d94-a648-f80d3849d792", "Steven Belledin"),
    CardSet::Conflux,
    CardRules::new_land(&[]).with_ability(AbilityDef::activated_mana(
        "{T}: Add one mana of any color that a land an opponent controls could produce.",
        &[CostDef::TapSource],
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &PATH_TO_EXILE,
    &NOBLE_HIERARCH,
    &KNIGHT_OF_THE_RELIQUARY,
    &EXOTIC_ORCHARD,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
