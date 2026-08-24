//! The Brothers' War cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef,
    CardArt, CardRules, CardSet, CardSupertype, CardType, CharacteristicOperationDef,
    CreatureTypeSetDef, EffectDef, EffectRecipientDef, ObjectPredicateDef, PlayerRelation,
    SetOperationDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// BRO 12 — Loran of the Third Path
/// "Up to one target artifact or enchantment": an Loran with nothing worth
/// answering still arrives as a 2/1 that draws.
static UP_TO_ONE_ARTIFACT_OR_ENCHANTMENT: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Enchantment),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
    1,
)];

static AN_OPPONENT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
)];

/// "You and target opponent each draw a card." Two draws rather than one
/// instruction naming both, because only one of them is targeted: the
/// opponent has to be a legal target and you never are.
static LORAN_DRAWS: [EffectDef; 2] = [
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(1),
    },
];

pub(in crate::card::sets) static LORAN_OF_THE_THIRD_PATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("59faa45d-868b-4bc7-934c-0e077642e129"),
    "Loran of the Third Path",
    CardArt::new("59faa45d-868b-4bc7-934c-0e077642e129", "Steven Belledin"),
    CardSet::TheBrothersWar,
    // Three mana for an answer to an artifact, a body that blocks, and a
    // symmetrical draw nobody else gets to use as often as you do.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Artificer"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::vigilance(),
            AbilityDef::triggered_with_targets(
                "When this creature enters, destroy up to one target artifact or enchantment.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                &UP_TO_ONE_ARTIFACT_OR_ENCHANTMENT,
                EffectDef::destroy_target(TargetIndex::PRIMARY, true),
            ),
            AbilityDef::activated_with_targets(
                "{T}: You and target opponent each draw a card.",
                &[AbilityCostDef::TapSource],
                &AN_OPPONENT,
                EffectDef::Sequence(&LORAN_DRAWS),
            ),
        ]),
);

// BRO 72 — Weakstone's Subjugation
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WEAKSTONE_S_SUBJUGATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ef93ac79-8575-40f8-a222-63c2ffb30f60"),
    "Weakstone's Subjugation",
    crate::card::CardArt::new("ef93ac79-8575-40f8-a222-63c2ffb30f60", "Igor Kieryluk"),
    crate::card::CardSet::TheBrothersWar,
    crate::card::CardRules::unsupported(),
);

// BRO 98 — Gixian Infiltrator
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GIXIAN_INFILTRATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c94a3317-7d1f-4f29-8353-180f1ab48d18"),
    "Gixian Infiltrator",
    crate::card::CardArt::new("c94a3317-7d1f-4f29-8353-180f1ab48d18", "Peter Polach"),
    crate::card::CardSet::TheBrothersWar,
    crate::card::CardRules::unsupported(),
);

// BRO 164 — Scrapwork Mutt
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCRAPWORK_MUTT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4742800a-4872-4c2d-b884-01e0ba16950c"),
    "Scrapwork Mutt",
    crate::card::CardArt::new(
        "4742800a-4872-4c2d-b884-01e0ba16950c",
        "Sidharth Chaturvedi",
    ),
    crate::card::CardSet::TheBrothersWar,
    crate::card::CardRules::unsupported(),
);

// BRO 199 — Haywire Mite
/// "Noncreature artifact or noncreature enchantment." The two types are
/// alternatives and the exclusion applies to both, so it sits outside the
/// choice rather than inside it -- which is what leaves a creature that
/// happens to be an artifact alone.
static A_NONCREATURE_ARTIFACT_OR_ENCHANTMENT: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
            ]),
            zones: &[ZoneKind::Battlefield],
            controller: None,
            owner: None,
        },
    )];

pub(in crate::card::sets) static HAYWIRE_MITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("847a175e-ead1-4596-baf3-5f7f57859e0b"),
    "Haywire Mite",
    CardArt::new("847a175e-ead1-4596-baf3-5f7f57859e0b", "Izzy"),
    CardSet::TheBrothersWar,
    // One mana for a body that is never dead: it answers whichever artifact
    // or enchantment the format is afraid of this week, and every deck can
    // cast it whether or not it can pay the green.
    CardRules::new_artifact_creature(mana_cost!("{1}"), &["Insect"], 1, 1).with_abilities(&[
        AbilityDef::triggered(
            "When this creature dies, you gain 2 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
        AbilityDef::activated_with_targets(
            "{G}, Sacrifice this creature: Exile target noncreature artifact or noncreature \
             enchantment.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}")),
                AbilityCostDef::SacrificeSource,
            ],
            &A_NONCREATURE_ARTIFACT_OR_ENCHANTMENT,
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                counters: None,
            },
        ),
    ]),
);

// BRO 223 — Third Path Iconoclast
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THIRD_PATH_ICONOCLAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f1a21287-e244-4960-84fb-c4f6e5c346d9"),
    "Third Path Iconoclast",
    crate::card::CardArt::new("f1a21287-e244-4960-84fb-c4f6e5c346d9", "Manuel Castañón"),
    crate::card::CardSet::TheBrothersWar,
    crate::card::CardRules::unsupported(),
);

// BRO 238 — The Mightstone and Weakstone
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THE_MIGHTSTONE_AND_WEAKSTONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("02aea379-b444-46a3-82f4-3038f698d4f4"),
    "The Mightstone and Weakstone",
    crate::card::CardArt::new("02aea379-b444-46a3-82f4-3038f698d4f4", "Ryan Pancoast"),
    crate::card::CardSet::TheBrothersWar,
    crate::card::CardRules::unsupported(),
);

// BRO 240 — Portal to Phyrexia
/// "It's a Phyrexian in addition to its other types." Added rather than set:
/// what comes back through the Portal keeps whatever it already was, and is
/// a Phyrexian as well.
static AS_A_PHYREXIAN: AppliedEffectDef =
    AppliedEffectDef::Characteristic(CharacteristicOperationDef::CreatureTypes(
        SetOperationDef::Add(CreatureTypeSetDef::named(&["Phyrexian"])),
    ));

/// Any graveyard, not only yours: the Portal is as happy to take back what
/// it made an opponent sacrifice as anything of your own.
static A_CREATURE_CARD_IN_A_GRAVEYARD: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: None,
    },
)];

pub(in crate::card::sets) static PORTAL_TO_PHYREXIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5f608efc-0dbc-4cc3-aadd-ed473bfc29ab"),
    "Portal to Phyrexia",
    CardArt::new("5f608efc-0dbc-4cc3-aadd-ed473bfc29ab", "Svetlin Velinov"),
    CardSet::TheBrothersWar,
    // Nine mana, and the game is over: three of their creatures die on the
    // way in and one comes back for you every upkeep afterwards.
    CardRules::new_artifact(mana_cost!("{9}")).with_abilities(&[
        AbilityDef::triggered(
            "When this artifact enters, each opponent sacrifices three creatures of their choice.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::Opponent,
                object: ObjectPredicateDef::HasType(CardType::Creature),
                count: ValueDef::Constant(3),
                then: None,
                amount: crate::card::SacrificedAmountDef::Power,
                otherwise: None,
                optional: false,
            },
        ),
        AbilityDef::triggered_with_targets(
            "At the beginning of your upkeep, put target creature card from a graveyard onto the \
             battlefield under your control. It's a Phyrexian in addition to its other types.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &A_CREATURE_CARD_IN_A_GRAVEYARD,
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                arrival_effect: Some(&AS_A_PHYREXIAN),
                attachment: None,
                controller: Some(PlayerRelation::You),
            },
        ),
    ]),
);

// BRO 266 — Tocasia's Dig Site
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TOCASIA_S_DIG_SITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("23d4b90c-95b1-4828-bc08-7067da0d5364"),
    "Tocasia's Dig Site",
    crate::card::CardArt::new("23d4b90c-95b1-4828-bc08-7067da0d5364", "Nadia Hurianova"),
    crate::card::CardSet::TheBrothersWar,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &LORAN_OF_THE_THIRD_PATH,
    &WEAKSTONE_S_SUBJUGATION,
    &GIXIAN_INFILTRATOR,
    &SCRAPWORK_MUTT,
    &HAYWIRE_MITE,
    &THIRD_PATH_ICONOCLAST,
    &THE_MIGHTSTONE_AND_WEAKSTONE,
    &PORTAL_TO_PHYREXIA,
    &TOCASIA_S_DIG_SITE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
