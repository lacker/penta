//! Through the Omenpaths cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, BattlefieldEntryModificationDef, BattlefieldEntryScalarChoiceDef,
    CardArt, CardRules, CardSet, EffectDef, EffectPaymentDef, EffectRecipientDef, PlayerRelation,
    PlayerSetDef, ReplacementChoiceDef, ReplacementEffectDef,
};

// OM1 181 — Multiversal Passage
pub(in crate::card::sets) static MULTIVERSAL_PASSAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("21502958-a8e3-494a-9be9-bebbbb1dd9dc"),
    "Multiversal Passage",
    CardArt::new("21502958-a8e3-494a-9be9-bebbbb1dd9dc", "Daren Bader"),
    CardSet::ThroughTheOmenpaths,
    // A shock land that is whichever basic type the hand actually wants,
    // which is a different card in a deck with two colours and in one with
    // five. The mana ability comes from the type rather than a printed
    // clause, so choosing is all there is to it.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::replacement(
            "As this land enters, choose a basic land type. Then you may pay 2 life. If you \
             don't, it enters tapped.",
            ReplacementEffectDef::Sequence(&[
                ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                    BattlefieldEntryScalarChoiceDef::BASIC_LAND_TYPE,
                )),
                ReplacementEffectDef::PayOr {
                    payment: EffectPaymentDef::life(PlayerSetDef::Related(PlayerRelation::You), 2),
                    if_paid: &[],
                    // Declining is what makes it a tapped land, so the branch that pays does
                    // nothing at all and the branch that does not is the whole cost.
                    if_declined: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                        BattlefieldEntryModificationDef::Tapped,
                    )],
                },
            ]),
        ),
        AbilityDef::static_ability(
            "This land is the chosen type.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::set_chosen_basic_land_type(),
            },
        ),
    ]),
);

// OM1 182 — Ominous Asylum
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OMINOUS_ASYLUM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("371b03a1-7707-4a8a-8c0e-0272418c801f"),
    "Ominous Asylum",
    crate::card::CardArt::new("371b03a1-7707-4a8a-8c0e-0272418c801f", "Daniel Ljunggren"),
    crate::card::CardSet::ThroughTheOmenpaths,
    crate::card::CardRules::unsupported(),
);

// OM1 183 — Savage Mansion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAVAGE_MANSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c172cdb5-aa2c-419d-b8ab-4795f4b7e160"),
    "Savage Mansion",
    crate::card::CardArt::new("c172cdb5-aa2c-419d-b8ab-4795f4b7e160", "Vincent Proce"),
    crate::card::CardSet::ThroughTheOmenpaths,
    crate::card::CardRules::unsupported(),
);

// OM1 184 — Sinister Hideout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SINISTER_HIDEOUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c417f8ce-e156-4c9a-af30-792606d861bd"),
    "Sinister Hideout",
    crate::card::CardArt::new(
        "c417f8ce-e156-4c9a-af30-792606d861bd",
        "Julian Kok Joon Wen",
    ),
    crate::card::CardSet::ThroughTheOmenpaths,
    crate::card::CardRules::unsupported(),
);

// OM1 185 — Suburban Sanctuary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUBURBAN_SANCTUARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cabf021b-23e9-404d-90c6-eef629e1283e"),
    "Suburban Sanctuary",
    crate::card::CardArt::new("cabf021b-23e9-404d-90c6-eef629e1283e", "Victor Sales"),
    crate::card::CardSet::ThroughTheOmenpaths,
    crate::card::CardRules::unsupported(),
);

// OM1 186 — University Campus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNIVERSITY_CAMPUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cd4b9fc5-fe3d-41d9-9d0e-77f1aebef618"),
    "University Campus",
    crate::card::CardArt::new("cd4b9fc5-fe3d-41d9-9d0e-77f1aebef618", "Randy Gallegos"),
    crate::card::CardSet::ThroughTheOmenpaths,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &MULTIVERSAL_PASSAGE,
    &OMINOUS_ASYLUM,
    &SAVAGE_MANSION,
    &SINISTER_HIDEOUT,
    &SUBURBAN_SANCTUARY,
    &UNIVERSITY_CAMPUS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
