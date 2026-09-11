//! SPM card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BattlefieldEntryScalarChoiceDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectPaymentDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ReplacementChoiceDef;
use crate::card::ReplacementEffectDef;
use crate::card::ValueDef;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new("SPM", "marvels-spider-man");

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// SPM 93 — Spider-Verse
// Audit: unsupported — Needs a once-each-turn trigger for spells cast outside hand whose optional stack copy can grant haste specifically when it copies a permanent spell.
pub(in crate::card::sets) static SPIDER_VERSE: CardRecord = CardRecord::new(
    "Spider-Verse",
    "f8779eb2-1210-430d-8d42-3077053441ee",
    "Alexander Gering",
    CardRules::unsupported(),
);

// SPM 141 — Rhino's Rampage
// Audit: unsupported — Needs a reflexive excess-damage trigger that chooses its artifact target after the fight.
pub(in crate::card::sets) static RHINOS_RAMPAGE: CardRecord = CardRecord::new(
    "Rhino's Rampage",
    "f668817c-1cab-44c5-b6a8-95113e480d5e",
    "Nino Is",
    CardRules::unsupported(),
);

// SPM 180 — Multiversal Passage
pub(in crate::card::sets) static MULTIVERSAL_PASSAGE: CardRecord = CardRecord::new(
    "Multiversal Passage",
    "f5fb426a-5618-4dd4-9c51-0cc847be8c1d",
    "Pablo Mendoza",
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
                    payment: EffectPaymentDef::new(
                        PlayerSetDef::Related(PlayerRelation::You),
                        &[CostDef::PayLife(2)],
                    ),
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

/// The OM1 cycle of tapped duals that surveil late: lands that differ only
/// in which two colours they make, so the clauses are written once here.
/// Entering tapped is the price of the two colours, and the surveil is what
/// a flooded late game does with the land instead of drawing it.
///
/// `colors` is a promoted literal at each call site, and the abilities are
/// added one at a time in printed order: an array holding the parameterized
/// mana ability could not be given a `'static` lifetime.
///
/// Spectacle Summit prints the same shape but is not in this cycle -- its
/// surveil costs {2}{U}{R} rather than {4}.
const fn surveilling_dual_land(mana_text: &'static str, colors: &'static [ManaColor]) -> CardRules {
    CardRules::new_land(&[])
        .with_ability(abilities::enters_tapped(CardType::Land))
        .with_ability(AbilityDef::activated_mana(
            mana_text,
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(colors)),
        ))
        .with_ability(AbilityDef::activated(
            "{4}, {T}: Surveil 1. (Look at the top card of your library. You may put it into \
             your graveyard.)",
            &[CostDef::Mana(mana_cost!("{4}")), CostDef::TapSource],
            abilities::surveil(ValueDef::Constant(1)),
        ))
}

// SPM 181 — Ominous Asylum
pub(in crate::card::sets) static OMINOUS_ASYLUM: CardRecord = CardRecord::new(
    "Ominous Asylum",
    "4329f94a-9110-4f07-b4a6-f1ccae97ccc9",
    "Pavel Kolomeyets",
    surveilling_dual_land("{T}: Add {B} or {R}.", &[ManaColor::Black, ManaColor::Red]),
);

// SPM 183 — Savage Mansion
pub(in crate::card::sets) static SAVAGE_MANSION: CardRecord = CardRecord::new(
    "Savage Mansion",
    "855f59a5-17a8-4aca-8a4d-f98111eba14c",
    "David Álvarez",
    surveilling_dual_land("{T}: Add {R} or {G}.", &[ManaColor::Red, ManaColor::Green]),
);

// SPM 184 — Sinister Hideout
pub(in crate::card::sets) static SINISTER_HIDEOUT: CardRecord = CardRecord::new(
    "Sinister Hideout",
    "23190d7e-5165-49bd-b307-bf81877d228d",
    "Pavel Kolomeyets",
    surveilling_dual_land("{T}: Add {U} or {B}.", &[ManaColor::Blue, ManaColor::Black]),
);

// SPM 185 — Suburban Sanctuary
pub(in crate::card::sets) static SUBURBAN_SANCTUARY: CardRecord = CardRecord::new(
    "Suburban Sanctuary",
    "467df77a-a99c-4cfd-9af4-502eaa2eb2e3",
    "David Frasheski",
    surveilling_dual_land(
        "{T}: Add {G} or {W}.",
        &[ManaColor::Green, ManaColor::White],
    ),
);

// SPM 186 — University Campus
pub(in crate::card::sets) static UNIVERSITY_CAMPUS: CardRecord = CardRecord::new(
    "University Campus",
    "2752f21c-f535-4772-a8b3-e97e1339e9c9",
    "David Álvarez",
    // A Campus that surveils rather than scries, so it does not share the
    // Strixhaven cycle's clause even though the rest of the card matches.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
        AbilityDef::activated(
            "{4}, {T}: Surveil 1.",
            &[CostDef::Mana(mana_cost!("{4}")), CostDef::TapSource],
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SPIDER_VERSE,
    &RHINOS_RAMPAGE,
    &MULTIVERSAL_PASSAGE,
    &OMINOUS_ASYLUM,
    &SAVAGE_MANSION,
    &SINISTER_HIDEOUT,
    &SUBURBAN_SANCTUARY,
    &UNIVERSITY_CAMPUS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
