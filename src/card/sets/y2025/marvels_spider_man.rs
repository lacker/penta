//! SPM card records required by supported formats.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, BattlefieldEntryModificationDef, BattlefieldEntryScalarChoiceDef,
    CardRules, CardSet, EffectDef, EffectPaymentDef, EffectRecipientDef, PlayerRelation,
    PlayerSetDef, ReplacementChoiceDef, ReplacementEffectDef,
};

// SPM 141 — Rhino's Rampage
// Audit: unsupported — Needs a reflexive excess-damage trigger that chooses its artifact target after the fight.
pub(in crate::card::sets) static RHINOS_RAMPAGE: CardRecord = CardRecord::new(
    CardSet::MarvelsSpiderMan,
    "Rhino's Rampage",
    "f668817c-1cab-44c5-b6a8-95113e480d5e",
    "Nino Is",
    CardRules::unsupported(),
);

// SPM 180 — Multiversal Passage
pub(in crate::card::sets) static MULTIVERSAL_PASSAGE: CardRecord = CardRecord::new(
    CardSet::MarvelsSpiderMan,
    "Multiversal Passage",
    "f5fb426a-5618-4dd4-9c51-0cc847be8c1d",
    "Pablo Mendoza",
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

// SPM 181 — Ominous Asylum
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OMINOUS_ASYLUM: CardRecord = CardRecord::new(
    CardSet::MarvelsSpiderMan,
    "Ominous Asylum",
    "4329f94a-9110-4f07-b4a6-f1ccae97ccc9",
    "Pavel Kolomeyets",
    CardRules::unsupported(),
);

// SPM 183 — Savage Mansion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAVAGE_MANSION: CardRecord = CardRecord::new(
    CardSet::MarvelsSpiderMan,
    "Savage Mansion",
    "855f59a5-17a8-4aca-8a4d-f98111eba14c",
    "David Álvarez",
    CardRules::unsupported(),
);

// SPM 184 — Sinister Hideout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SINISTER_HIDEOUT: CardRecord = CardRecord::new(
    CardSet::MarvelsSpiderMan,
    "Sinister Hideout",
    "23190d7e-5165-49bd-b307-bf81877d228d",
    "Pavel Kolomeyets",
    CardRules::unsupported(),
);

// SPM 185 — Suburban Sanctuary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUBURBAN_SANCTUARY: CardRecord = CardRecord::new(
    CardSet::MarvelsSpiderMan,
    "Suburban Sanctuary",
    "467df77a-a99c-4cfd-9af4-502eaa2eb2e3",
    "David Frasheski",
    CardRules::unsupported(),
);

// SPM 186 — University Campus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNIVERSITY_CAMPUS: CardRecord = CardRecord::new(
    CardSet::MarvelsSpiderMan,
    "University Campus",
    "2752f21c-f535-4772-a8b3-e97e1339e9c9",
    "David Álvarez",
    CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &RHINOS_RAMPAGE,
    &MULTIVERSAL_PASSAGE,
    &OMINOUS_ASYLUM,
    &SAVAGE_MANSION,
    &SINISTER_HIDEOUT,
    &SUBURBAN_SANCTUARY,
    &UNIVERSITY_CAMPUS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
