//! Mirrodin cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AppliedEffectDef, CardArt, CardRules, CardSet,
    CardType, EffectDef, EffectRecipientDef, ObjectPredicateDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::{TargetIndex, mana_cost};

static SPELLBOMB_BOUNCE_COST: [AbilityCostDef; 2] = [
    AbilityCostDef::Mana(mana_cost!("{U}")),
    AbilityCostDef::SacrificeSource,
];

static SPELLBOMB_DRAW_COST: [AbilityCostDef; 2] = [
    AbilityCostDef::Mana(mana_cost!("{1}")),
    AbilityCostDef::SacrificeSource,
];

static A_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

// MRD 141 — Aether Spellbomb
pub(in crate::card::sets) static AETHER_SPELLBOMB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f3792e8b-4ad7-4e2d-994c-c4eaac0fa55f"),
    "Aether Spellbomb",
    CardArt::new("f3792e8b-4ad7-4e2d-994c-c4eaac0fa55f", "Jim Nelson"),
    CardSet::Mirrodin,
    // One mana that answers a creature for a turn if it has to and replaces
    // itself if it does not, which is why it costs a deck nothing to play.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{U}, Sacrifice this artifact: Return target creature to its owner's hand.",
            &SPELLBOMB_BOUNCE_COST,
            &A_CREATURE,
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                controller: None,
                arrival_effect: None,
                attachment: None,
            },
        ),
        AbilityDef::activated(
            "{1}, Sacrifice this artifact: Draw a card.",
            &SPELLBOMB_DRAW_COST,
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

static GREAVES_HASTE: AbilityDef = abilities::haste();

static GREAVES_SHROUD: AbilityDef = abilities::shroud();

/// The two halves are why the card is played: haste makes the creature useful
/// the turn it arrives, and shroud makes it hard to answer -- including by
/// its own controller, who cannot target it either.
static GREAVES_GRANTS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::add_ability(&GREAVES_HASTE),
    AppliedEffectDef::add_ability(&GREAVES_SHROUD),
];

// MRD 57 — Barter in Blood
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BARTER_IN_BLOOD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("beccbb2c-ca1d-4b72-9eca-a64a313fd830"),
    "Barter in Blood",
    crate::card::CardArt::new("beccbb2c-ca1d-4b72-9eca-a64a313fd830", "Paolo Parente"),
    crate::card::CardSet::Mirrodin,
    crate::card::CardRules::unsupported(),
);

// MRD 141 — Aether Spellbomb

// MRD 146 — Bonesplitter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BONESPLITTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ae31d513-7412-4467-b497-a7183ff29a42"),
    "Bonesplitter",
    crate::card::CardArt::new("465a7990-c9f9-4716-a833-fd41458b9cee", "Darrell Riche"),
    crate::card::CardSet::Mirrodin,
    crate::card::CardRules::unsupported(),
);

// MRD 152 — Chrome Mox
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHROME_MOX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6a058e68-70af-4a64-859c-c881e5578368"),
    "Chrome Mox",
    crate::card::CardArt::new("6a058e68-70af-4a64-859c-c881e5578368", "Donato Giancola"),
    crate::card::CardSet::Mirrodin,
    crate::card::CardRules::unsupported(),
);

// MRD 199 — Lightning Greaves
pub(in crate::card::sets) static LIGHTNING_GREAVES: CardRecord = CardRecord::new_with_legacy_id(
    2170,
    "Lightning Greaves",
    CardArt::new("61a28870-cf78-4323-9d82-cee764067764", "Jeremy Jarvis"),
    CardSet::Mirrodin,
    // Equipping for nothing is the whole card: the Greaves move to whatever
    // just arrived, every turn, for as long as they are on the battlefield.
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has haste and shroud.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&GREAVES_GRANTS),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{0}"))], "Equip {0}"),
        ]),
);

// MRD 253 — Talisman of Dominance
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TALISMAN_OF_DOMINANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("991037a2-fea2-49f5-8ace-ebbf9f678cff"),
    "Talisman of Dominance",
    crate::card::CardArt::new("991037a2-fea2-49f5-8ace-ebbf9f678cff", "Mike Dringenberg"),
    crate::card::CardSet::Mirrodin,
    crate::card::CardRules::unsupported(),
);

// MRD 256 — Talisman of Progress
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TALISMAN_OF_PROGRESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("41ff849e-2439-4690-8aa4-769039b6da4c"),
    "Talisman of Progress",
    crate::card::CardArt::new("41ff849e-2439-4690-8aa4-769039b6da4c", "Mike Dringenberg"),
    crate::card::CardSet::Mirrodin,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BARTER_IN_BLOOD,
    &AETHER_SPELLBOMB,
    &BONESPLITTER,
    &CHROME_MOX,
    &LIGHTNING_GREAVES,
    &TALISMAN_OF_DOMINANCE,
    &TALISMAN_OF_PROGRESS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
