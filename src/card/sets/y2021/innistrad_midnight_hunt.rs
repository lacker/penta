//! Innistrad: Midnight Hunt cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AppliedEffectDef, CardArt, CardRules, CardSet,
    CardSupertype, CardType, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    ObjectQueryDef, PlayerRelation, TriggerEventDef, ValueDef, ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

// MID 1 — Adeline, Resplendent Cathar
static CREATURES_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

/// "Whenever you attack" is one or more creatures you control attacking,
/// counted once for the declaration rather than once per attacker.
static WHENEVER_YOU_ATTACK: TriggerEventDef = TriggerEventDef::attack_declared(
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
    1,
    None,
);

static ADELINE_ABILITIES: [AbilityDef; 3] = [
    abilities::vigilance(),
    AbilityDef::static_ability(
        "Adeline's power is equal to the number of creatures you control.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            // Adeline is a creature you control, so she counts herself, and
            // every token she makes adds one more before damage. The count
            // defines her power rather than adding to it, which is why it
            // also answers in a hand or a graveyard.
            effect: AppliedEffectDef::define_power(ValueDef::CountMatchingObjects(
                &CREATURES_YOU_CONTROL,
            )),
        },
    ),
    // The token was never declared as an attacker, so nothing watching a
    // declaration sees it -- and with two players the one opponent is the
    // only thing it could be attacking.
    AbilityDef::triggered(
        "Whenever you attack, for each opponent, create a 1/1 white Human creature token that's \
         tapped and attacking that player or a planeswalker they control.",
        WHENEVER_YOU_ATTACK,
        EffectDef::create_creature_token(&["Human"], &[ManaColor::White], 1, 1)
            .with_art(CardArt::new(
                "7d13a93a-a43d-4cf5-8300-8341f3b7f1b1",
                "Miguel Mercado",
            ))
            .entering_tapped()
            .entering_attacking(),
    ),
];

pub(in crate::card::sets) static ADELINE_RESPLENDENT_CATHAR: CardRecord =
    CardRecord::new_with_legacy_id(
        2280,
        "Adeline, Resplendent Cathar",
        CardArt::new("18092f68-b96e-4084-9eba-b240d2195d81", "Bryan Sola"),
        CardSet::InnistradMidnightHunt,
        // Three mana that attacks for four the turn after it lands and for more
        // every turn after that, because each token it makes makes it bigger.
        CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Knight"], 0, 4)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&ADELINE_ABILITIES),
    );

// MID 10 — Cathar Commando
static AN_ARTIFACT_OR_ENCHANTMENT: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Enchantment),
        ]),
    )];

pub(in crate::card::sets) static CATHAR_COMMANDO: CardRecord = CardRecord::new_with_legacy_id(
    2273,
    "Cathar Commando",
    CardArt::new("98cbc1c2-b76e-4da3-aa43-00e10b2ce532", "Evyn Fong"),
    CardSet::InnistradMidnightHunt,
    // Flash is what makes the two halves one card: it can be held up as
    // removal and cashed in as a 3/1 when nothing needs killing.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 3, 1).with_abilities(&[
        abilities::flash(),
        AbilityDef::activated_with_targets(
            "{1}, Sacrifice this creature: Destroy target artifact or enchantment.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::SacrificeSource,
            ],
            &AN_ARTIFACT_OR_ENCHANTMENT,
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ]),
);

// MID 24 — Homestead Courage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HOMESTEAD_COURAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("73a9c49f-fcd3-4572-bac7-6eb06fdc0815"),
    "Homestead Courage",
    crate::card::CardArt::new("73a9c49f-fcd3-4572-bac7-6eb06fdc0815", "Colin Boyer"),
    crate::card::CardSet::InnistradMidnightHunt,
    crate::card::CardRules::unsupported(),
);

// MID 32 — Search Party Captain
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SEARCH_PARTY_CAPTAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cb9006c1-2e6f-4bca-a1c4-3cf2a8b6e964"),
    "Search Party Captain",
    crate::card::CardArt::new("cb9006c1-2e6f-4bca-a1c4-3cf2a8b6e964", "Mike Bierek"),
    crate::card::CardSet::InnistradMidnightHunt,
    crate::card::CardRules::unsupported(),
);

// MID 44 — Consider
pub(in crate::card::sets) static CONSIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b3f40a0-5f58-4157-aed9-b1a52e922c3c"),
    "Consider",
    CardArt::new("a211d505-4d40-4914-a9da-220770d6ddbc", "Zezhou Chen"),
    CardSet::InnistradMidnightHunt,
    // One mana to see two cards deep and choose which of them the deck is
    // better off having in the graveyard.
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Surveil 1. (Look at the top card of your library. You may put it into your graveyard.)\n\
         Draw a card.",
        EffectDef::Sequence(&[
            abilities::surveil(ValueDef::Constant(1)),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// MID 96 — Diregraf Horde
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DIREGRAF_HORDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("153be768-ddad-44f2-bcdd-c40353c807d7"),
    "Diregraf Horde",
    crate::card::CardArt::new("153be768-ddad-44f2-bcdd-c40353c807d7", "Alex Negrea"),
    crate::card::CardSet::InnistradMidnightHunt,
    crate::card::CardRules::unsupported(),
);

// MID 100 — Ecstatic Awakener // Awoken Demon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ECSTATIC_AWAKENER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bbdad18e-e262-41f9-b252-1cbdcdd1b5f9"),
    "Ecstatic Awakener",
    crate::card::CardArt::new("bbdad18e-e262-41f9-b252-1cbdcdd1b5f9", "Tuan Duong Chu"),
    crate::card::CardSet::InnistradMidnightHunt,
    crate::card::CardRules::unsupported(),
);

// MID 107 — Infernal Grasp
/// The life is part of the resolution rather than a cost, so a target that
/// survives being destroyed still costs it -- and a Grasp that never
/// resolves at all costs nothing.
static INFERNAL_GRASP_EFFECTS: [EffectDef; 2] = [
    EffectDef::Destroy {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        can_regenerate: true,
        then: None,
    },
    EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    },
];

static A_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

pub(in crate::card::sets) static INFERNAL_GRASP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("17824929-f131-4b8d-addb-66c25323155e"),
    "Infernal Grasp",
    CardArt::new("17824929-f131-4b8d-addb-66c25323155e", "Naomi Baker"),
    CardSet::InnistradMidnightHunt,
    // Two mana, no restriction on what it answers, and the two life is the
    // whole of the price.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature. You lose 2 life.",
        &A_CREATURE,
        EffectDef::Sequence(&INFERNAL_GRASP_EFFECTS),
    )),
);

// MID 128 — Ardent Elementalist
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARDENT_ELEMENTALIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f58592f7-1df5-428d-9dde-e6acd9a5d1d5"),
    "Ardent Elementalist",
    crate::card::CardArt::new("f58592f7-1df5-428d-9dde-e6acd9a5d1d5", "Miguel Mercado"),
    crate::card::CardSet::InnistradMidnightHunt,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ADELINE_RESPLENDENT_CATHAR,
    &CATHAR_COMMANDO,
    &HOMESTEAD_COURAGE,
    &SEARCH_PARTY_CAPTAIN,
    &CONSIDER,
    &DIREGRAF_HORDE,
    &ECSTATIC_AWAKENER,
    &INFERNAL_GRASP,
    &ARDENT_ELEMENTALIST,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
