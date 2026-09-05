//! Through the Omenpaths cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AddManaEffectDef, AppliedEffectDef,
    BattlefieldEntryModificationDef, BattlefieldEntryScalarChoiceDef, CardArt, CardRules, CardSet,
    CardType, EffectDef, EffectPaymentDef, EffectRecipientDef, ManaColor, PlayerRelation,
    PlayerSetDef, ReplacementChoiceDef, ReplacementEffectDef, ValueDef, abilities,
};
use crate::mana_cost;

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
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(colors)),
        ))
        .with_ability(AbilityDef::activated(
            "{4}, {T}: Surveil 1. (Look at the top card of your library. You may put it into \
             your graveyard.)",
            &[
                AbilityCostDef::Mana(mana_cost!("{4}")),
                AbilityCostDef::TapSource,
            ],
            abilities::surveil(ValueDef::Constant(1)),
        ))
}

// OM1 182 — Ominous Asylum
pub(in crate::card::sets) static OMINOUS_ASYLUM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("371b03a1-7707-4a8a-8c0e-0272418c801f"),
    "Ominous Asylum",
    CardArt::new("371b03a1-7707-4a8a-8c0e-0272418c801f", "Daniel Ljunggren"),
    CardSet::ThroughTheOmenpaths,
    surveilling_dual_land("{T}: Add {B} or {R}.", &[ManaColor::Black, ManaColor::Red]),
);

// OM1 183 — Savage Mansion
pub(in crate::card::sets) static SAVAGE_MANSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c172cdb5-aa2c-419d-b8ab-4795f4b7e160"),
    "Savage Mansion",
    CardArt::new("c172cdb5-aa2c-419d-b8ab-4795f4b7e160", "Vincent Proce"),
    CardSet::ThroughTheOmenpaths,
    surveilling_dual_land("{T}: Add {R} or {G}.", &[ManaColor::Red, ManaColor::Green]),
);

// OM1 184 — Sinister Hideout
pub(in crate::card::sets) static SINISTER_HIDEOUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c417f8ce-e156-4c9a-af30-792606d861bd"),
    "Sinister Hideout",
    CardArt::new(
        "c417f8ce-e156-4c9a-af30-792606d861bd",
        "Julian Kok Joon Wen",
    ),
    CardSet::ThroughTheOmenpaths,
    surveilling_dual_land("{T}: Add {U} or {B}.", &[ManaColor::Blue, ManaColor::Black]),
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
pub(in crate::card::sets) static UNIVERSITY_CAMPUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cd4b9fc5-fe3d-41d9-9d0e-77f1aebef618"),
    "University Campus",
    CardArt::new("cd4b9fc5-fe3d-41d9-9d0e-77f1aebef618", "Randy Gallegos"),
    CardSet::ThroughTheOmenpaths,
    // A Campus that surveils rather than scries, so it does not share the
    // Strixhaven cycle's clause even though the rest of the card matches.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
        AbilityDef::activated(
            "{4}, {T}: Surveil 1.",
            &[
                AbilityCostDef::Mana(mana_cost!("{4}")),
                AbilityCostDef::TapSource,
            ],
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
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
