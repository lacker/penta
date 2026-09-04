//! Onslaught cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::KeywordAbility;
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1994::legends as catalog_leg;
use crate::card::sets::y1996::mirage as catalog_mir;
use crate::card::sets::y1998::urzas_saga as catalog_usg;
use crate::card::sets::y1999::urzas_destiny as catalog_uds;
use crate::card::sets::y1999::urzas_legacy as catalog_ulg;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, BasicLandType, CardRules, CardSet, CardType,
    DamageEventMatcherDef, DamagePreventionDef, DiscardSelectionDef, EffectDef,
    EffectPaymentCostDef, EffectPaymentDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    ObjectQueryDef, ObjectRefDef, PayOrDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    ResolvedEffectDurationDef, ScaledValueDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::{TargetIndex, TurnStepDef, mana_cost};

const fn fetch_land(text: &'static str, land_types: &'static [BasicLandType]) -> CardRules {
    CardRules::new_land(&[]).with_ability(abilities::fetch_land_ability(
        text,
        ObjectPredicateDef::HasAnyBasicLandType(land_types),
    ))
}

// ONS 1 — Akroma's Blessing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AKROMA_S_BLESSING: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Akroma's Blessing",
    "c3710c68-3f71-4d76-8bd2-001f0e8036f5",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// ONS 2 — Akroma's Vengeance
pub(in crate::card::sets) static AKROMAS_VENGEANCE: CardRecord = CardRecord::new(
    CardSet::Onslaught,
    "Akroma's Vengeance",
    "5e33aaf7-7490-4b64-a966-82fbf7ca8686",
    "Greg Hildebrandt & Tim Hildebrandt",
    // Six mana is a lot for a sweeper, and the cycling is what makes it
    // maindeckable anyway: the card is never dead.
    CardRules::new_sorcery(mana_cost!("{4}{W}{W}")).with_abilities(&[
        AbilityDef::spell(
            "Destroy all artifacts, creatures, and enchantments.",
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                can_regenerate: true,
                then: None,
            },
        ),
        abilities::cycling(
            "Cycling {3} ({3}, Discard this card: Draw a card.)",
            mana_cost!("{3}"),
        ),
    ]),
);

// ONS 3 — Ancestor's Prophet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANCESTOR_S_PROPHET: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Ancestor's Prophet",
    "cdee956e-76b1-4ba7-a387-2fbfb853507d",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// ONS 4 — Astral Slide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASTRAL_SLIDE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Astral Slide",
    "d14993b6-ed8d-4b9b-b54c-2837b343a61e",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// ONS 5 — Aura Extraction
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AURA_EXTRACTION: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Aura Extraction",
    "55d16883-5e98-4dd2-92dd-0ba92f1099cb",
    "Luca Zontini",
    crate::card::CardRules::unsupported(),
);

// ONS 6 — Aurification
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AURIFICATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Aurification",
    "93d9e9ea-9f88-4206-8960-b5ebe839ee16",
    "Gary Ruddell",
    crate::card::CardRules::unsupported(),
);

// ONS 7 — Aven Brigadier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVEN_BRIGADIER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Aven Brigadier",
    "da24ef56-8d54-4146-97e9-4abded807545",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// ONS 8 — Aven Soulgazer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVEN_SOULGAZER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Aven Soulgazer",
    "5189f152-f075-4090-97dd-b7686d813865",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// ONS 9 — Battlefield Medic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BATTLEFIELD_MEDIC: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Battlefield Medic",
    "9c444503-42a8-4952-819b-bbca89b06abc",
    "Matt Thompson",
    crate::card::CardRules::unsupported(),
);

// ONS 10 — Catapult Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CATAPULT_MASTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Catapult Master",
    "a74d7aa2-c6ff-432d-b671-cef58c6736c6",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// ONS 11 — Catapult Squad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CATAPULT_SQUAD: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Catapult Squad",
    "75a71d29-29eb-43c4-b0f3-457435e8f629",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// ONS 12 — Chain of Silence
pub(in crate::card::sets) static CHAIN_OF_SILENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Chain of Silence",
    "9a60ac8e-11eb-433f-86f9-8e593b38c617",
    "Randy Gallegos",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Prevent all damage target creature would deal this turn. That creature's controller may sacrifice a land of their choice. If the player does, they may copy this spell and may choose a new target for that copy.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::from(
                    ObjectRefDef::Target(TargetIndex::PRIMARY),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef {
                    payer: PlayerSetDef::One(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                        TargetIndex::PRIMARY,
                    ))),
                    cost: EffectPaymentCostDef::SacrificePermanentMatching(ObjectPredicateDef::HasType(
                        CardType::Land,
                    )),
                },
                &EffectDef::May {
                    player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    effect: &EffectDef::CopyStackObject(&crate::card::CopyStackObjectDef {
                        object: EffectRecipientDef::object(ObjectRefDef::ResolvingObject),
                        controller: PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                        count: ValueDef::Constant(1),
                        retarget: true,
                        colors: None,
                    }),
                },
            )),
        ]),
    )),
);

// ONS 13 — Circle of Solace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CIRCLE_OF_SOLACE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Circle of Solace",
    "07f567dc-8a60-40e1-b947-199872d8df08",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// ONS 14 — Convalescent Care
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONVALESCENT_CARE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Convalescent Care",
    "48f3ad80-d000-496a-b704-d09e07981b6e",
    "Greg Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// ONS 15 — Crowd Favorites
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROWD_FAVORITES: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Crowd Favorites",
    "1038436d-aea5-4508-8b37-c2cfa32c2771",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// ONS 16 — Crown of Awe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROWN_OF_AWE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Crown of Awe",
    "aeaea4bc-dcea-4340-a039-ebc97b944673",
    "Randy Elliott",
    crate::card::CardRules::unsupported(),
);

// ONS 17 — Crude Rampart
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRUDE_RAMPART: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Crude Rampart",
    "af5d1be2-d6ae-4820-aa01-62f261b0f110",
    "Sam Wood",
    crate::card::CardRules::unsupported(),
);

// ONS 18 — Daru Cavalier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARU_CAVALIER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Daru Cavalier",
    "eb2e9b7e-434e-477f-b3e8-e85ceb913650",
    "Dany Orizio",
    crate::card::CardRules::unsupported(),
);

// ONS 19 — Daru Healer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARU_HEALER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Daru Healer",
    "0e4f3eff-ac99-41e2-9003-9630cdb3ae23",
    "Dany Orizio",
    crate::card::CardRules::unsupported(),
);

// ONS 20 — Daru Lancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARU_LANCER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Daru Lancer",
    "cd888ca8-0ebe-46f0-9317-3b193ccc43fb",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// ONS 21 — Daunting Defender
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAUNTING_DEFENDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Daunting Defender",
    "38737f38-26bd-417c-b6b4-53f26e4e8044",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// ONS 22 — Dawning Purist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAWNING_PURIST: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Dawning Purist",
    "b8cb25b0-e4c3-4a4e-b722-ea30e695f917",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// ONS 23 — Defensive Maneuvers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEFENSIVE_MANEUVERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Defensive Maneuvers",
    "58f9eb25-4140-4ecf-bcaa-1b193d884007",
    "Luca Zontini",
    crate::card::CardRules::unsupported(),
);

// ONS 24 — Demystify
pub(in crate::card::sets) static DEMYSTIFY: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Demystify",
    "d0df839f-dc4c-44b0-82c7-cb2037172ac5",
    "Christopher Rush",
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target enchantment.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Enchantment),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
            then: None,
        },
    )),
);

// ONS 25 — Disciple of Grace (reprint)
const DISCIPLE_OF_GRACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::DISCIPLE_OF_GRACE,
    "1d1790cb-34e4-4f23-8a13-1906fd9a956f",
    "Thomas M. Baxa",
);

// ONS 26 — Dive Bomber
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIVE_BOMBER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Dive Bomber",
    "65162b24-8a3b-4b92-a831-6f23f809c76f",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ONS 27 — Doubtless One
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOUBTLESS_ONE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Doubtless One",
    "0dedef8a-5527-40dc-9ad9-bcee4cf30a76",
    "Justin Sweet",
    crate::card::CardRules::unsupported(),
);

// ONS 28 — Exalted Angel
pub(in crate::card::sets) static EXALTED_ANGEL: CardRecord = CardRecord::new(
    CardSet::Onslaught,
    "Exalted Angel",
    "c2213eac-cea4-4dfd-90c4-c1f466967e2e",
    "Michael Sutfin",
    // Six mana is more than a control deck wants to pay on turn four, so it
    // comes down face down on three and stands up on the next turn instead.
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Angel"], 4, 5)
        .with_morph(mana_cost!("{2}{W}{W}"))
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever this creature deals damage, you gain that much life.",
                TriggerEventDef::damage_dealt_by(ObjectPredicateDef::Source),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::TriggerEventAmount,
                },
            ),
            AbilityDef::alternative_cast(
                mana_cost!("{3}"),
                crate::card::face_down::morph_cast(),
                Some(
                    "Morph {2}{W}{W} (You may cast this card face down as a 2/2 creature for {3}. Turn it face up any time for its morph cost.)",
                ),
                EffectDef::None,
            ),
        ]),
);

// ONS 29 — Foothill Guide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOOTHILL_GUIDE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Foothill Guide",
    "409adb7b-6dcb-4e7f-a5dd-c0adf12140a4",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// ONS 30 — Glarecaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLARECASTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Glarecaster",
    "7e505e8e-51aa-4415-81e6-cf022279edb0",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ONS 31 — Glory Seeker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLORY_SEEKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Glory Seeker",
    "9047075e-9fca-484d-bb79-32c0d6821281",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// ONS 32 — Grassland Crusader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRASSLAND_CRUSADER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Grassland Crusader",
    "c129f361-8769-4f9a-9745-eb5d0c085b88",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ONS 33 — Gravel Slinger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAVEL_SLINGER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Gravel Slinger",
    "87551307-6b5f-4f12-aa1f-4beebefad3b3",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// ONS 34 — Gustcloak Harrier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUSTCLOAK_HARRIER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Gustcloak Harrier",
    "b5ff5c7d-7823-4d1e-8abb-77e2d8126996",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ONS 35 — Gustcloak Runner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUSTCLOAK_RUNNER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Gustcloak Runner",
    "eb227f65-9189-41ed-94a0-2aa21cad26f5",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// ONS 36 — Gustcloak Savior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUSTCLOAK_SAVIOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Gustcloak Savior",
    "0e9d6e81-1869-4ab7-8a4e-477d5c4aed6b",
    "Jim Nelson",
    crate::card::CardRules::unsupported(),
);

// ONS 37 — Gustcloak Sentinel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUSTCLOAK_SENTINEL: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Gustcloak Sentinel",
    "b90da5c3-fd8f-445d-809f-e129870d7449",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// ONS 38 — Gustcloak Skirmisher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUSTCLOAK_SKIRMISHER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Gustcloak Skirmisher",
    "cbbff06c-5f92-4320-8b70-df3c8344f600",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ONS 39 — Harsh Mercy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARSH_MERCY: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Harsh Mercy",
    "b6473b4d-1f59-4216-ace9-f3e5306266fb",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// ONS 40 — Improvised Armor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IMPROVISED_ARMOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Improvised Armor",
    "8d7d5d79-73d8-4f1a-9dda-4de5f41539d9",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// ONS 41 — Inspirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSPIRIT: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Inspirit",
    "55e0e300-db79-4328-ba1d-9c3910e47f52",
    "Keith Garletts",
    crate::card::CardRules::unsupported(),
);

// ONS 42 — Ironfist Crusher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRONFIST_CRUSHER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Ironfist Crusher",
    "c7284e32-de54-4c83-a7de-7b249c47319a",
    "Iain McCaig",
    crate::card::CardRules::unsupported(),
);

// ONS 43 — Jareth, Leonine Titan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JARETH_LEONINE_TITAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Jareth, Leonine Titan",
    "65dd1364-ff36-4cb9-ad93-e6fcbcb942cf",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// ONS 44 — Mobilization
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOBILIZATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Mobilization",
    "653cc07b-0f53-4b5b-9c5f-885b8b4a6e5f",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// ONS 45 — Nova Cleric
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOVA_CLERIC: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Nova Cleric",
    "b2048d84-b5e6-405c-9091-1997a0c4e1a5",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// ONS 46 — Oblation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OBLATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Oblation",
    "58561356-4a97-467b-88e5-412e633715fb",
    "Doug Chaffee",
    crate::card::CardRules::unsupported(),
);

// ONS 47 — Pacifism (reprint)
const PACIFISM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1996::mirage::PACIFISM,
    "ee262fde-8df1-431f-9e5c-0cafe9212b49",
    "Matthew D. Wilson",
);

// ONS 48 — Pearlspear Courier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PEARLSPEAR_COURIER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Pearlspear Courier",
    "a1ea7219-6ab6-471a-afe7-d7da1df434c7",
    "Dany Orizio",
    crate::card::CardRules::unsupported(),
);

// ONS 49 — Piety Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PIETY_CHARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Piety Charm",
    "1bc2da43-c0e1-4fbf-b309-a75e105c29c1",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// ONS 50 — Renewed Faith
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RENEWED_FAITH: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Renewed Faith",
    "1ea572b5-ff68-45aa-8200-78ee7f64a0ce",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// ONS 51 — Righteous Cause
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIGHTEOUS_CAUSE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Righteous Cause",
    "b83c6245-4b37-430d-af10-2581804fff08",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// ONS 52 — Sandskin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SANDSKIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Sandskin",
    "80b59844-c9d4-4bc1-86e6-4cc596d9165d",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// ONS 53 — Shared Triumph
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHARED_TRIUMPH: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Shared Triumph",
    "0d07ebe6-76cf-4345-b59b-9954496c44d0",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// ONS 54 — Shieldmage Elder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIELDMAGE_ELDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Shieldmage Elder",
    "efa2d660-7c93-4087-a6e5-49c2ad21eb5a",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// ONS 55 — Sigil of the New Dawn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SIGIL_OF_THE_NEW_DAWN: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Sigil of the New Dawn",
    "ca1babca-b285-4b00-8b46-ed946c9a027f",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// ONS 56 — Sunfire Balm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNFIRE_BALM: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Sunfire Balm",
    "0d563ebb-ecd1-406c-9d69-c101acdeced7",
    "Monte Michael Moore",
    crate::card::CardRules::unsupported(),
);

// ONS 57 — True Believer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRUE_BELIEVER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "True Believer",
    "4289bdcb-6eea-458f-a4eb-89e26264673a",
    "Alex Horley-Orlandelli",
    crate::card::CardRules::unsupported(),
);

// ONS 58 — Unified Strike
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNIFIED_STRIKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Unified Strike",
    "29906eca-0823-4cd6-890f-e5b93cc50a11",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// ONS 59 — Weathered Wayfarer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WEATHERED_WAYFARER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Weathered Wayfarer",
    "f6601ab1-3862-4aff-82be-be15493fe4b0",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// ONS 60 — Whipcorder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHIPCORDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Whipcorder",
    "3bf6987e-a6e4-4a88-af0b-cf3b2d2b80c7",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ONS 61 — Words of Worship
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORDS_OF_WORSHIP: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Words of Worship",
    "0ea5c6e0-8361-4214-997b-32a66b19fae9",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// ONS 62 — Airborne Aid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AIRBORNE_AID: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Airborne Aid",
    "0aaa43b0-601f-4b99-a328-541b04d5696d",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// ONS 63 — Annex
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANNEX: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Annex",
    "c95d5cb7-3121-430b-80c3-84c75e5f869e",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// ONS 64 — Aphetto Alchemist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APHETTO_ALCHEMIST: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Aphetto Alchemist",
    "dfd2628f-63c4-4e19-83ea-26041650faab",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// ONS 65 — Aphetto Grifter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APHETTO_GRIFTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Aphetto Grifter",
    "3a7a7bf3-1b0c-415d-9c57-73ac55b1f915",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// ONS 66 — Arcanis the Omnipotent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCANIS_THE_OMNIPOTENT: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Arcanis the Omnipotent",
    "90865f52-c062-4505-a204-b4d7d4b3fc4c",
    "Justin Sweet",
    crate::card::CardRules::unsupported(),
);

// ONS 67 — Artificial Evolution
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARTIFICIAL_EVOLUTION: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Artificial Evolution",
    "f46894d1-2503-43fa-938e-7bbf19101d13",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// ONS 68 — Ascending Aven
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASCENDING_AVEN: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Ascending Aven",
    "bd8b17df-615c-4cc1-af1a-2fc35a985af9",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ONS 69 — Aven Fateshaper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVEN_FATESHAPER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Aven Fateshaper",
    "7a4b41c4-0d14-4b9c-8e0c-a626ba6b104d",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// ONS 70 — Backslide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BACKSLIDE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Backslide",
    "47c40269-80a5-454f-83dd-dae1c11500c0",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ONS 71 — Blatant Thievery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLATANT_THIEVERY: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Blatant Thievery",
    "8284476c-a7c8-4a6c-8021-ee997e9270ce",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ONS 72 — Callous Oppressor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALLOUS_OPPRESSOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Callous Oppressor",
    "b3dd3ce7-e0e3-4412-9983-ff933584f59b",
    "Justin Sweet",
    crate::card::CardRules::unsupported(),
);

// ONS 73 — Chain of Vapor
pub(in crate::card::sets) static CHAIN_OF_VAPOR: CardRecord = CardRecord::new(
    CardSet::Onslaught,
    "Chain of Vapor",
    "30f6b4a2-5780-46e9-b239-459d2cf37743",
    "Carl Critchlow",
    // One mana to undo anything, and the chain is the opponent's to continue
    // or stop.
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target nonland permanent to its owner's hand. Then that permanent's controller may sacrifice a land of their choice. If the player does, they may copy this spell and may choose a new target for that copy.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        )],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
},
            // A land of their choice, sacrificed by whoever just had a permanent
            // bounced. Paying buys the copy, which is what turns one Chain of Vapor into
            // a board sweep in a deck holding the lands to spend.
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef {
                    payer: PlayerSetDef::One(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                        TargetIndex::PRIMARY,
                    ))),
                    cost: EffectPaymentCostDef::SacrificePermanentMatching(ObjectPredicateDef::HasType(
                        CardType::Land,
                    )),
                },
                &EffectDef::May {
                    player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    effect: &EffectDef::CopyStackObject(&crate::card::CopyStackObjectDef {
                        object: EffectRecipientDef::object(ObjectRefDef::ResolvingObject),
                        controller: PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                        count: ValueDef::Constant(1),
                        retarget: true,
                        colors: None,
                    }),
                },
            )),
        ]),
    )),
);

// ONS 74 — Choking Tethers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHOKING_TETHERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Choking Tethers",
    "d4de14d1-441f-4d65-bd12-df0506530015",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// ONS 75 — Clone (reprint)
const CLONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CLONE,
    "1d513dde-7c5f-46f1-b871-5290595bdbbe",
    "Carl Critchlow",
);

// ONS 76 — Complicate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COMPLICATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Complicate",
    "33f69670-e494-42b8-9148-fe105ec61aa0",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// ONS 77 — Crafty Pathmage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRAFTY_PATHMAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Crafty Pathmage",
    "c5d91378-f831-40ef-a79b-b044af1470e0",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// ONS 78 — Crown of Ascension
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROWN_OF_ASCENSION: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Crown of Ascension",
    "2fe86733-7851-4c2a-8d94-dba6f071b94d",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// ONS 79 — Discombobulate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISCOMBOBULATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Discombobulate",
    "cef584c5-6e2d-419b-9c11-a1b6c9c9ab2a",
    "Alex Horley-Orlandelli",
    crate::card::CardRules::unsupported(),
);

// ONS 80 — Dispersing Orb
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISPERSING_ORB: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Dispersing Orb",
    "69db0298-f6d5-450f-add3-a28c0a43f33f",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// ONS 81 — Disruptive Pitmage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISRUPTIVE_PITMAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Disruptive Pitmage",
    "5b0d9c2f-356c-4f27-8560-8ffceadac31c",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// ONS 82 — Essence Fracture
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ESSENCE_FRACTURE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Essence Fracture",
    "df0b6c7a-0891-492d-8e07-6a198bf2ccc4",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// ONS 83 — Fleeting Aven
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLEETING_AVEN: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Fleeting Aven",
    "246a2758-0096-43b9-8193-d6ae5b41b6e6",
    "Iain McCaig",
    crate::card::CardRules::unsupported(),
);

// ONS 84 — Future Sight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FUTURE_SIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Future Sight",
    "688bd665-4948-4961-aec5-f17782257f9b",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// ONS 85 — Ghosthelm Courier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GHOSTHELM_COURIER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Ghosthelm Courier",
    "cd6cc30a-9ed4-4f36-95cb-6f0a2b8dce02",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// ONS 86 — Graxiplon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAXIPLON: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Graxiplon",
    "0c16e565-0b7f-46b1-a091-64c47c923a9f",
    "Iain McCaig",
    crate::card::CardRules::unsupported(),
);

// ONS 87 — Imagecrafter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IMAGECRAFTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Imagecrafter",
    "91be6441-8a45-43e4-8d12-a886dcaadbd3",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// ONS 88 — Information Dealer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFORMATION_DEALER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Information Dealer",
    "a45ac59c-654d-44de-b266-532d44b34137",
    "Jerry Tiritilli",
    crate::card::CardRules::unsupported(),
);

// ONS 89 — Ixidor, Reality Sculptor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IXIDOR_REALITY_SCULPTOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Ixidor, Reality Sculptor",
    "314d5e89-55f7-42b4-af19-d4d0f499a265",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// ONS 90 — Ixidor's Will
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IXIDOR_S_WILL: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Ixidor's Will",
    "1b713448-853a-41ee-a302-963e9c1c1c65",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// ONS 91 — Mage's Guile
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGE_S_GUILE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Mage's Guile",
    "301cb538-a931-4916-927b-4986046b1158",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// ONS 92 — Meddle (reprint)
const MEDDLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::MEDDLE,
    "685edfe8-9770-47c6-95fb-0816f3126f04",
    "Brian Snõddy",
);

// ONS 93 — Mistform Dreamer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFORM_DREAMER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Mistform Dreamer",
    "ff34e303-c94a-4f5f-b9f6-8d48e6aac383",
    "Matthew Mitchell",
    crate::card::CardRules::unsupported(),
);

// ONS 94 — Mistform Mask
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFORM_MASK: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Mistform Mask",
    "7fbbb075-5795-425f-9e33-70cb922eea16",
    "Monte Michael Moore",
    crate::card::CardRules::unsupported(),
);

// ONS 95 — Mistform Mutant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFORM_MUTANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Mistform Mutant",
    "a25b2697-5d7f-490a-8474-c775096e681e",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// ONS 96 — Mistform Shrieker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFORM_SHRIEKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Mistform Shrieker",
    "1082eea2-5e83-48d4-b02b-a22e7cbe2054",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// ONS 97 — Mistform Skyreaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFORM_SKYREAVER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Mistform Skyreaver",
    "e394e096-ea70-4813-9039-e4bd065d0a17",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// ONS 98 — Mistform Stalker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFORM_STALKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Mistform Stalker",
    "9e80d109-b73f-4b5d-b9e4-534e8d69633f",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ONS 99 — Mistform Wall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFORM_WALL: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Mistform Wall",
    "ebaa7a26-8516-4d71-a524-77b2d3f030d5",
    "Franz Vohwinkel",
    crate::card::CardRules::unsupported(),
);

// ONS 100 — Nameless One
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NAMELESS_ONE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Nameless One",
    "79cf3535-3f80-4b76-aad3-dd851e6885a6",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ONS 101 — Peer Pressure
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PEER_PRESSURE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Peer Pressure",
    "be0110ba-49e4-4729-8a84-4d408b20df53",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// ONS 102 — Psychic Trance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PSYCHIC_TRANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Psychic Trance",
    "d5e55695-16cc-4373-8078-959f1ded4c6d",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// ONS 103 — Quicksilver Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUICKSILVER_DRAGON: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Quicksilver Dragon",
    "e93577bd-2711-443c-aa88-a235345d7800",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ONS 104 — Read the Runes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static READ_THE_RUNES: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Read the Runes",
    "bc148c21-cbe6-4cea-899b-e62501b59a00",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// ONS 105 — Reminisce
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REMINISCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Reminisce",
    "b5f246e3-2193-4820-9c59-07b480300fbe",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// ONS 106 — Riptide Biologist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIPTIDE_BIOLOGIST: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Riptide Biologist",
    "4d399b71-c365-492c-976e-2c79d97d08bc",
    "Justin Sweet",
    crate::card::CardRules::unsupported(),
);

// ONS 107 — Riptide Chronologist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIPTIDE_CHRONOLOGIST: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Riptide Chronologist",
    "3767f568-36b1-4064-835e-4dd7576b7b8b",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// ONS 108 — Riptide Entrancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIPTIDE_ENTRANCER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Riptide Entrancer",
    "2cd9abc9-f289-4294-bc0f-4addc8b92a4e",
    "Scott Hampton",
    crate::card::CardRules::unsupported(),
);

// ONS 109 — Riptide Shapeshifter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIPTIDE_SHAPESHIFTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Riptide Shapeshifter",
    "85be34ac-7bc2-4da2-8d9c-2412b9946073",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// ONS 110 — Rummaging Wizard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUMMAGING_WIZARD: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Rummaging Wizard",
    "ad96e158-bf2b-4f3e-9692-0f79efdd94f5",
    "Jerry Tiritilli",
    crate::card::CardRules::unsupported(),
);

// ONS 111 — Sage Aven
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAGE_AVEN: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Sage Aven",
    "4c03afc5-7ca3-4ac6-a06e-091e2cce13a0",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ONS 112 — Screaming Seahawk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCREAMING_SEAHAWK: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Screaming Seahawk",
    "cc5856ac-e710-44ee-8516-6070f4f31ce5",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ONS 113 — Sea's Claim
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEA_S_CLAIM: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Sea's Claim",
    "fb652a5c-464e-4ba4-a4ab-1181be70cf7a",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// ONS 114 — Slipstream Eel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLIPSTREAM_EEL: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Slipstream Eel",
    "e9d06a1f-00b7-440d-849d-efc466d73f29",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ONS 115 — Spy Network
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPY_NETWORK: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Spy Network",
    "8a4bed3f-845c-4822-b8af-8b511dce6fe2",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// ONS 116 — Standardize
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STANDARDIZE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Standardize",
    "f2c79e64-91bf-4e87-a4fd-3136ea67c5bb",
    "Justin Sweet",
    crate::card::CardRules::unsupported(),
);

// ONS 117 — Supreme Inquisitor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUPREME_INQUISITOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Supreme Inquisitor",
    "867de3d2-2178-4931-823e-ff439e1a45ea",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// ONS 118 — Trade Secrets
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRADE_SECRETS: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Trade Secrets",
    "e92e197e-ef7e-46bb-9533-5f9819d545b2",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// ONS 119 — Trickery Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRICKERY_CHARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Trickery Charm",
    "32a2ee45-7f1d-40a8-82b4-ab3b705417ea",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// ONS 120 — Voidmage Prodigy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOIDMAGE_PRODIGY: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Voidmage Prodigy",
    "7441e7f9-a326-4f61-b7b1-e0dbed06046f",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// ONS 121 — Wheel and Deal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHEEL_AND_DEAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Wheel and Deal",
    "61f50a1a-f3d0-4fcf-bd32-0e173b0d3247",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// ONS 122 — Words of Wind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORDS_OF_WIND: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Words of Wind",
    "5595a57a-a76c-467b-afaf-5affffc24f35",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// ONS 123 — Accursed Centaur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ACCURSED_CENTAUR: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Accursed Centaur",
    "894556d8-6d5c-431b-a45d-26cd37c5f456",
    "Jerry Tiritilli",
    crate::card::CardRules::unsupported(),
);

// ONS 124 — Anurid Murkdiver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANURID_MURKDIVER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Anurid Murkdiver",
    "9e43d62c-488a-4c8d-b193-bacbf8037761",
    "Dany Orizio",
    crate::card::CardRules::unsupported(),
);

// ONS 125 — Aphetto Dredging
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APHETTO_DREDGING: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Aphetto Dredging",
    "c4e7fadf-40f1-45ff-97ef-5830381accc9",
    "Monte Michael Moore",
    crate::card::CardRules::unsupported(),
);

// ONS 126 — Aphetto Vulture
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APHETTO_VULTURE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Aphetto Vulture",
    "107492b9-03a8-4d53-a0cf-4814ffbec409",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// ONS 127 — Blackmail
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLACKMAIL: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Blackmail",
    "9b40f6eb-e2a4-46d2-8822-b0f3dc508b73",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// ONS 128 — Boneknitter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BONEKNITTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Boneknitter",
    "c9d58030-a95a-4221-93bc-30a59344e30b",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ONS 129 — Cabal Archon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CABAL_ARCHON: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Cabal Archon",
    "4bdf6e2a-1bf5-4d63-a58b-883cfb1ea0fa",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ONS 130 — Cabal Executioner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CABAL_EXECUTIONER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Cabal Executioner",
    "cd7727a7-0cdf-4fd5-82b4-e6587c10ca80",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// ONS 131 — Cabal Slaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CABAL_SLAVER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Cabal Slaver",
    "b9c04fd3-021a-4011-be9b-0d268557aa06",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ONS 132 — Chain of Smog
pub(in crate::card::sets) static CHAIN_OF_SMOG: CardRecord = CardRecord::new(
    CardSet::Onslaught,
    "Chain of Smog",
    "6bfe64f9-8b03-41f6-a47b-fade397ad9d1",
    "Greg Staples",
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player discards two cards. That player may copy this spell and may choose a new target for that copy.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
            // The copy costs nothing here, unlike Chain of Vapor's land. Whoever was
            // just hit decides whether to pass it on, and picks the next target -- which
            // is why the chain usually stops at whoever cannot afford to keep it going.
            EffectDef::May {
                player: EffectRecipientDef::player(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                effect: &EffectDef::CopyStackObject(&crate::card::CopyStackObjectDef {
                    object: EffectRecipientDef::object(ObjectRefDef::ResolvingObject),
                    controller: PlayerRefDef::Target(TargetIndex::PRIMARY),
                    count: ValueDef::Constant(1),
                    retarget: true,
                    colors: None,
                }),
            },
        ]),
    )),
);

// ONS 133 — Cover of Darkness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COVER_OF_DARKNESS: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Cover of Darkness",
    "0d6d7d88-d82b-40f4-bf57-ec5d7c480689",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// ONS 134 — Crown of Suspicion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROWN_OF_SUSPICION: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Crown of Suspicion",
    "8953e11b-cc3a-4c8d-9d7e-04bf90c77027",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// ONS 135 — Cruel Revival
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRUEL_REVIVAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Cruel Revival",
    "245aba23-2abb-4084-b4cb-d06e46de2108",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// ONS 136 — Death Match
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_MATCH: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Death Match",
    "143e9057-267a-4c78-b72a-4f8018b627a8",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// ONS 137 — Death Pulse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_PULSE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Death Pulse",
    "524fd470-e535-47ea-98a0-6187e429dfe1",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// ONS 138 — Dirge of Dread
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIRGE_OF_DREAD: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Dirge of Dread",
    "8496e9c2-4c13-4307-bda7-b88512a21a6a",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ONS 139 — Disciple of Malice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISCIPLE_OF_MALICE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Disciple of Malice",
    "74cc7ab0-a5db-4ae9-af9a-89fd5aaaab57",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// ONS 140 — Doomed Necromancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOOMED_NECROMANCER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Doomed Necromancer",
    "3ca3e348-47cc-41d6-999a-60d1206aaf06",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// ONS 141 — Ebonblade Reaper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EBONBLADE_REAPER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Ebonblade Reaper",
    "16ebef2c-8bb2-4816-a628-0062f95e512e",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// ONS 142 — Endemic Plague
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENDEMIC_PLAGUE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Endemic Plague",
    "15326971-a53b-45f2-8f1d-1b82935286e1",
    "Nelson DeCastro",
    crate::card::CardRules::unsupported(),
);

// ONS 143 — Entrails Feaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENTRAILS_FEASTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Entrails Feaster",
    "cdddab92-3e1f-49dc-afd0-8c84d0d952c2",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// ONS 144 — Fade from Memory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FADE_FROM_MEMORY: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Fade from Memory",
    "56b34afa-0183-49aa-aa5f-03e070020136",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// ONS 145 — Fallen Cleric
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FALLEN_CLERIC: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Fallen Cleric",
    "7652dc61-9170-4895-a0bf-c32a1ee0350e",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// ONS 146 — False Cure
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FALSE_CURE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "False Cure",
    "ef397db1-2d99-4cb0-a6e9-6f72d615ebad",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// ONS 147 — Feeding Frenzy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEEDING_FRENZY: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Feeding Frenzy",
    "a6d74c30-ebca-4684-ad84-3ca19193ad88",
    "Nelson DeCastro",
    crate::card::CardRules::unsupported(),
);

// ONS 148 — Festering Goblin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FESTERING_GOBLIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Festering Goblin",
    "e7209cc8-b519-4f27-87d8-b12e239a121f",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// ONS 149 — Frightshroud Courier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FRIGHTSHROUD_COURIER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Frightshroud Courier",
    "4a0fa75a-a82b-44cd-965f-07e0fe7a111a",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// ONS 150 — Gangrenous Goliath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GANGRENOUS_GOLIATH: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Gangrenous Goliath",
    "69b58b6b-24cd-4440-b99c-d88d44b3c41c",
    "Justin Sweet",
    crate::card::CardRules::unsupported(),
);

// ONS 151 — Gluttonous Zombie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLUTTONOUS_ZOMBIE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Gluttonous Zombie",
    "db909e95-7979-41f0-b17a-874c4137fcc1",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// ONS 152 — Gravespawn Sovereign
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAVESPAWN_SOVEREIGN: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Gravespawn Sovereign",
    "e18dc249-a343-4198-bef9-e8092a2bac15",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// ONS 153 — Grinning Demon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRINNING_DEMON: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Grinning Demon",
    "72de2f66-0b86-4c21-b4c8-c2d97e3fd095",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// ONS 154 — Haunted Cadaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAUNTED_CADAVER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Haunted Cadaver",
    "a164420c-3619-4f5e-81cf-2aa5a4553bc3",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ONS 155 — Head Games
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEAD_GAMES: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Head Games",
    "86ecc098-aa2b-4bae-80d5-4d02128ef837",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// ONS 156 — Headhunter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEADHUNTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Headhunter",
    "3cbd82d5-d64f-4833-b1a9-9652fcfa1578",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// ONS 157 — Infest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFEST: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Infest",
    "b7890ba2-aa42-4c8d-bbc1-94fb1d4150fc",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// ONS 158 — Misery Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISERY_CHARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Misery Charm",
    "2be66eaf-222b-4c40-a9fa-aad56b9218e0",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// ONS 159 — Nantuko Husk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NANTUKO_HUSK: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Nantuko Husk",
    "1ff31ece-f132-4107-9415-fcf30e251167",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// ONS 160 — Oversold Cemetery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OVERSOLD_CEMETERY: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Oversold Cemetery",
    "3bbfd715-0772-4516-8cd8-89495dbccf4a",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// ONS 161 — Patriarch's Bidding
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PATRIARCH_S_BIDDING: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Patriarch's Bidding",
    "2deba175-8c02-492d-b404-5d842910c095",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// ONS 162 — Profane Prayers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROFANE_PRAYERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Profane Prayers",
    "bc8320ef-af97-4cf6-9aaf-17818174d842",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// ONS 163 — Prowling Pangolin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROWLING_PANGOLIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Prowling Pangolin",
    "0f037e99-75fb-4a2a-b4c6-448ef21b16a3",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ONS 164 — Rotlung Reanimator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROTLUNG_REANIMATOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Rotlung Reanimator",
    "87b29d1e-9c06-4ad1-8178-b3eaa212f6f1",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// ONS 165 — Screeching Buzzard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCREECHING_BUZZARD: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Screeching Buzzard",
    "1d4b887a-d928-4f6c-aa37-a0b09e87b91e",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ONS 166 — Severed Legion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEVERED_LEGION: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Severed Legion",
    "efe12afd-da41-436e-af84-fa3b36a58030",
    "Dany Orizio",
    crate::card::CardRules::unsupported(),
);

// ONS 167 — Shade's Breath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHADE_S_BREATH: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Shade's Breath",
    "a37be9a8-ef69-4c62-8455-e129e62fe69a",
    "Franz Vohwinkel",
    crate::card::CardRules::unsupported(),
);

// ONS 168 — Shepherd of Rot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHEPHERD_OF_ROT: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Shepherd of Rot",
    "952c021f-74c9-455f-9cd9-f0d354e8bea8",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// ONS 169 — Silent Specter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILENT_SPECTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Silent Specter",
    "bfd891ba-cf6a-4b83-a421-3a7c346ada31",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// ONS 170 — Smother
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SMOTHER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Smother",
    "9a8321af-d667-44e7-8c03-3957286604b9",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// ONS 171 — Soulless One
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOULLESS_ONE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Soulless One",
    "c826d786-0d96-4f77-94ae-6907fbce51e0",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// ONS 172 — Spined Basher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPINED_BASHER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Spined Basher",
    "4d0d666a-8e31-466c-937f-54df910f664e",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// ONS 173 — Strongarm Tactics
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRONGARM_TACTICS: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Strongarm Tactics",
    "57dcf434-5c67-440a-8b67-2df7307e92bd",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// ONS 174 — Swat (reprint)
const SWAT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ulg::SWAT,
    "cec3a260-6c50-401d-a0ff-bf49a973e1a1",
    "rk post",
);

// ONS 175 — Syphon Mind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SYPHON_MIND: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Syphon Mind",
    "0b0d8543-78c9-4d7f-b45e-44ecf023d276",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// ONS 176 — Syphon Soul (reprint)
const SYPHON_SOUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SYPHON_SOUL,
    "3bdaef0f-9965-463b-902d-72ec24b2db7b",
    "Ron Spears",
);

// ONS 177 — Thrashing Mudspawn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRASHING_MUDSPAWN: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Thrashing Mudspawn",
    "da84de0e-a4cd-4dff-8ee3-87c9debf0969",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// ONS 178 — Undead Gladiator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNDEAD_GLADIATOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Undead Gladiator",
    "bbc779d9-3200-4369-9289-1a8e90e243b9",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// ONS 179 — Visara the Dreadful
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VISARA_THE_DREADFUL: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Visara the Dreadful",
    "ce6adcfe-b0f7-4a96-bab2-f76c84ef5ca6",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// ONS 180 — Walking Desecration
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALKING_DESECRATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Walking Desecration",
    "c39f3e91-571a-4990-b1e8-db2a5bac34af",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// ONS 181 — Withering Hex
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WITHERING_HEX: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Withering Hex",
    "9ce4be1e-97dd-45ec-89e5-2fb56145c098",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// ONS 182 — Words of Waste
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORDS_OF_WASTE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Words of Waste",
    "d2dcb8ed-23e7-4cee-9f43-042232c6035a",
    "Jerry Tiritilli",
    crate::card::CardRules::unsupported(),
);

// ONS 183 — Wretched Anurid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WRETCHED_ANURID: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Wretched Anurid",
    "aab525ad-1f62-4d9c-9b74-c7b0048da452",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// ONS 184 — Aether Charge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AETHER_CHARGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Aether Charge",
    "05df2792-4971-49e8-a8f2-17700e247500",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// ONS 185 — Aggravated Assault
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGGRAVATED_ASSAULT: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Aggravated Assault",
    "c99c5707-d5f2-4675-bfca-e801e6b0f627",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// ONS 186 — Airdrop Condor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AIRDROP_CONDOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Airdrop Condor",
    "ec9796ac-11e2-4295-bf00-f684d0111970",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// ONS 187 — Avarax
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVARAX: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Avarax",
    "ae76705f-ec95-48b0-9e26-84ce40c9514b",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// ONS 188 — Battering Craghorn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BATTERING_CRAGHORN: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Battering Craghorn",
    "9ef71f42-87e5-4b1d-aac1-3752b81cee7c",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// ONS 189 — Blistering Firecat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLISTERING_FIRECAT: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Blistering Firecat",
    "e0ddcf4a-1943-49dd-a02c-75804ce4bc3e",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// ONS 190 — Break Open
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BREAK_OPEN: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Break Open",
    "a5ae8050-b644-41db-b1e9-d9bad2173485",
    "Alex Horley-Orlandelli",
    crate::card::CardRules::unsupported(),
);

// ONS 191 — Brightstone Ritual
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRIGHTSTONE_RITUAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Brightstone Ritual",
    "5b08b0a6-c94e-4407-8a24-c8202497b5f2",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// ONS 192 — Butcher Orgg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BUTCHER_ORGG: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Butcher Orgg",
    "7f2a29cf-4b2e-44c0-af73-512d6fed0dae",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// ONS 193 — Chain of Plasma
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAIN_OF_PLASMA: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Chain of Plasma",
    "f94aa774-9036-4016-8880-4bde2710cb90",
    "Gary Ruddell",
    crate::card::CardRules::unsupported(),
);

// ONS 194 — Charging Slateback
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHARGING_SLATEBACK: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Charging Slateback",
    "d2cfff37-655f-4107-abf3-e6f63d0e4de2",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ONS 195 — Commando Raid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COMMANDO_RAID: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Commando Raid",
    "bb237330-ac2e-411d-836c-6628f96f3262",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ONS 196 — Crown of Fury
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROWN_OF_FURY: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Crown of Fury",
    "6caae974-f531-469d-8c6a-2077c4f3294a",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// ONS 197 — Custody Battle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CUSTODY_BATTLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Custody Battle",
    "b72257f5-0cf9-45ca-8dc7-a1a93bd7dd1e",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// ONS 198 — Dragon Roost
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGON_ROOST: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Dragon Roost",
    "95e4f28b-c7a7-4450-b477-73e4559f0276",
    "Luca Zontini",
    crate::card::CardRules::unsupported(),
);

// ONS 199 — Dwarven Blastminer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_BLASTMINER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Dwarven Blastminer",
    "2970831a-738b-476f-9d46-39f10a1f91e7",
    "Gary Ruddell",
    crate::card::CardRules::unsupported(),
);

// ONS 200 — Embermage Goblin (alternate printing)
const EMBERMAGE_GOBLIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EMBERMAGE_GOBLIN,
    1,
    "f50f60a8-e99a-4891-b474-a21abee38970",
    "Pete Venters",
);

// ONS 200★ — Embermage Goblin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMBERMAGE_GOBLIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Embermage Goblin",
    "0ee5aa80-32cc-486e-bbb2-5386eadaf4ca",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ONS 201 — Erratic Explosion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERRATIC_EXPLOSION: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Erratic Explosion",
    "9f608a7e-5555-4554-a6e7-fe00e0bbe753",
    "Gary Ruddell",
    crate::card::CardRules::unsupported(),
);

// ONS 202 — Fever Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEVER_CHARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Fever Charm",
    "830d1980-f460-4be2-9379-c3f74c8318f3",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// ONS 203 — Flamestick Courier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAMESTICK_COURIER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Flamestick Courier",
    "e822161d-0434-4578-aecd-c9ef0b84bd4e",
    "Luca Zontini",
    crate::card::CardRules::unsupported(),
);

// ONS 204 — Goblin Machinist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_MACHINIST: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Goblin Machinist",
    "5874e312-1010-43f2-b330-82bc9fcc9f53",
    "Doug Chaffee",
    crate::card::CardRules::unsupported(),
);

// ONS 205 — Goblin Piledriver
pub(in crate::card::sets) static GOBLIN_PILEDRIVER: CardRecord = CardRecord::new(
    CardSet::Onslaught,
    "Goblin Piledriver",
    "f6c4df1f-f148-42ec-8e22-e7114216927d",
    "Matt Cavotta",
    // Protection from blue is half the card: it walks past the format's
    // blue blockers while the rest of the team makes it enormous.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Warrior"], 1, 2).with_abilities(&[
        abilities::protection_from_color(ManaColor::Blue),
        AbilityDef::triggered(
            "Whenever this creature attacks, it gets +2/+0 until end of turn for each other attacking Goblin.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Scaled(&ScaledValueDef {
                        // "Each other attacking Goblin", so the Piledriver never counts itself and
                        // a lone one gets nothing.
                        value: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Subtype("Goblin"),
                                ObjectPredicateDef::Attacking,
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        )),
                        factor: 2,
                    }),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ONS 206 — Goblin Pyromancer
pub(in crate::card::sets) static GOBLIN_PYROMANCER: CardRecord = CardRecord::new(
    CardSet::Onslaught,
    "Goblin Pyromancer",
    "bb4815b7-fc20-44a4-ad1c-66d92993557f",
    "Edward P. Beard, Jr.",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Goblin", "Wizard"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, Goblin creatures get +3/+0 until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Goblin"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::triggered(
            "At the beginning of the end step, destroy all Goblins.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Subtype("Goblin"),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                can_regenerate: true,
                then: None,
            },
        ),
    ]),
);

// ONS 207 — Goblin Sharpshooter
pub(in crate::card::sets) static GOBLIN_SHARPSHOOTER: CardRecord = CardRecord::new(
    CardSet::Onslaught,
    "Goblin Sharpshooter",
    "7e689df7-b85d-4346-bee8-5e978b5cbbbc",
    "Greg Staples",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin"], 1, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This creature doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::triggered(
            "Whenever a creature dies, untap this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::HasType(CardType::Creature),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 1 damage to any target.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ONS 208 — Goblin Sky Raider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SKY_RAIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Goblin Sky Raider",
    "738cbf9b-e3d3-4568-93ce-7915b248e5b3",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// ONS 209 — Goblin Sledder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SLEDDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Goblin Sledder",
    "3a9a1ecf-29f6-474e-bbcf-3455d388aa94",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ONS 210 — Goblin Taskmaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_TASKMASTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Goblin Taskmaster",
    "feff65ca-aedf-4434-b701-590d600d1a0b",
    "Trevor Hairsine",
    crate::card::CardRules::unsupported(),
);

// ONS 211 — Grand Melee
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAND_MELEE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Grand Melee",
    "9a0d3142-4224-4b51-885d-33c8938418c1",
    "Trevor Hairsine",
    crate::card::CardRules::unsupported(),
);

// ONS 212 — Gratuitous Violence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRATUITOUS_VIOLENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Gratuitous Violence",
    "4b0c5d14-4fab-4034-a2d3-0d851ef67cbd",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// ONS 213 — Insurrection
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSURRECTION: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Insurrection",
    "998bad32-1927-4e12-9527-efa55b86cae0",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// ONS 214 — Kaboom!
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KABOOM: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Kaboom!",
    "1e81e5fc-0e18-4dd8-a505-aa7dba8521a8",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// ONS 215 — Lavamancer's Skill
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAVAMANCER_S_SKILL: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Lavamancer's Skill",
    "0d4dd156-a2c1-4fab-b9f4-3302a4e8835a",
    "Monte Michael Moore",
    crate::card::CardRules::unsupported(),
);

// ONS 216 — Lay Waste (reprint)
const LAY_WASTE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::LAY_WASTE,
    "22061b5e-81d3-4c7f-ab39-7ee719c13cef",
    "Carl Critchlow",
);

// ONS 217 — Lightning Rift
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHTNING_RIFT: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Lightning Rift",
    "d775d729-0ad9-4b14-9d44-6282f6936e07",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// ONS 218 — Mana Echoes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANA_ECHOES: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Mana Echoes",
    "1b15d04c-62cb-4704-8cc7-9842cef27a1b",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// ONS 219 — Menacing Ogre
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MENACING_OGRE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Menacing Ogre",
    "5360a871-6932-45b2-bc94-1bd414e38906",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ONS 220 — Nosy Goblin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOSY_GOBLIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Nosy Goblin",
    "70ea023e-e66d-4049-b7bc-5e660804f088",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// ONS 221 — Pinpoint Avalanche
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PINPOINT_AVALANCHE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Pinpoint Avalanche",
    "d5cf8876-4c7d-4779-9363-d0a58bb7d851",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// ONS 222 — Reckless One
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RECKLESS_ONE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Reckless One",
    "37775f40-10de-4f5d-abb2-c49e682039de",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ONS 223 — Risky Move
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RISKY_MOVE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Risky Move",
    "0b09315c-d6ff-4fdb-8774-c6402b45e959",
    "Jerry Tiritilli",
    crate::card::CardRules::unsupported(),
);

// ONS 224 — Rorix Bladewing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RORIX_BLADEWING: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Rorix Bladewing",
    "7f2caba5-9f30-4b5e-833e-68c85a47ef7c",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// ONS 225 — Searing Flesh
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEARING_FLESH: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Searing Flesh",
    "d83db110-42e7-4823-a686-b83205faf503",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ONS 226 — Shaleskin Bruiser
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHALESKIN_BRUISER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Shaleskin Bruiser",
    "fc2de8a4-0d84-4f7c-bbe4-3a31172186ab",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// ONS 227 — Shock (reprint)
const SHOCK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::stronghold::SHOCK,
    "83c92b5d-103c-4719-a850-690a7010291a",
    "Edward P. Beard, Jr.",
);

// ONS 228 — Skirk Commando
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKIRK_COMMANDO: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Skirk Commando",
    "8c870a66-4cd5-4a8d-9948-feffa7d4ff11",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// ONS 229 — Skirk Fire Marshal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKIRK_FIRE_MARSHAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Skirk Fire Marshal",
    "b71117d0-5cf7-4041-b568-00bd8a975dd8",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// ONS 230 — Skirk Prospector
pub(in crate::card::sets) static SKIRK_PROSPECTOR: CardRecord = CardRecord::new(
    CardSet::Onslaught,
    "Skirk Prospector",
    "eb545dcd-3a7a-46a7-9c35-d28faebc6d17",
    "Doug Chaffee",
    // A one-drop that turns the rest of the board into mana, including
    // itself: the sacrifice names any Goblin, and the Prospector is one.
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "Sacrifice a Goblin: Add {R}.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::Subtype("Goblin"),
                controller: PlayerRelation::You,
            }],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
    ),
);

// ONS 231 — Skittish Valesk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKITTISH_VALESK: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Skittish Valesk",
    "4cc8a6e6-ed62-4784-ba9a-b1f703fc6119",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// ONS 232 — Slice and Dice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLICE_AND_DICE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Slice and Dice",
    "59262684-86e3-4485-9e35-202771c3eaa6",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// ONS 233 — Snapping Thragg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNAPPING_THRAGG: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Snapping Thragg",
    "c8a47d41-b893-46b9-90c9-ccd8f9f78855",
    "Iain McCaig",
    crate::card::CardRules::unsupported(),
);

// ONS 234 — Solar Blast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLAR_BLAST: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Solar Blast",
    "b36fc40c-6a68-4192-91d9-2031c7d32e05",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// ONS 235 — Sparksmith
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPARKSMITH: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Sparksmith",
    "15a4460d-3fe8-4b1f-9990-0a19c3345367",
    "Jim Nelson",
    crate::card::CardRules::unsupported(),
);

// ONS 236 — Spitfire Handler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPITFIRE_HANDLER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Spitfire Handler",
    "efe72820-952f-4c53-9ee7-ea7ea54fc848",
    "Jim Nelson",
    crate::card::CardRules::unsupported(),
);

// ONS 237 — Spurred Wolverine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPURRED_WOLVERINE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Spurred Wolverine",
    "46d7aaea-226b-4820-8db2-89dcdcbcc557",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// ONS 238 — Starstorm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARSTORM: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Starstorm",
    "b54d72ba-05ce-4299-a7c3-a9e9f126fffb",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// ONS 239 — Tephraderm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEPHRADERM: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Tephraderm",
    "41b65eba-140b-4c1d-b796-8134b7c1ede8",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// ONS 240 — Thoughtbound Primoc
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THOUGHTBOUND_PRIMOC: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Thoughtbound Primoc",
    "e89156b5-8bdb-41d1-a7aa-63f770a9b070",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// ONS 241 — Threaten
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THREATEN: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Threaten",
    "de9676b6-6812-44e5-ad70-f498fbad0e18",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// ONS 242 — Thunder of Hooves
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDER_OF_HOOVES: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Thunder of Hooves",
    "9e4f796a-6831-4d83-824d-88fd2148b4c1",
    "Jim Nelson",
    crate::card::CardRules::unsupported(),
);

// ONS 243 — Wave of Indifference
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAVE_OF_INDIFFERENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Wave of Indifference",
    "2c88b942-06d5-45d8-a4d8-6ca864f65516",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// ONS 244 — Words of War
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORDS_OF_WAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Words of War",
    "2593a6a6-dc21-4742-acb8-f7092931b1ce",
    "Justin Sweet",
    crate::card::CardRules::unsupported(),
);

// ONS 245 — Animal Magnetism
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANIMAL_MAGNETISM: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Animal Magnetism",
    "c33db646-b30d-4a15-9f8a-63bda74e2d81",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// ONS 246 — Barkhide Mauler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARKHIDE_MAULER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Barkhide Mauler",
    "b9196ce7-3ff4-4dda-a628-559ada11c9ba",
    "Iain McCaig",
    crate::card::CardRules::unsupported(),
);

// ONS 247 — Biorhythm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BIORHYTHM: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Biorhythm",
    "2a02d6d5-27be-4301-a467-5b49491d0d4f",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// ONS 248 — Birchlore Rangers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BIRCHLORE_RANGERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Birchlore Rangers",
    "8ce3a3a1-3569-4909-a604-f78d4888781e",
    "Dany Orizio",
    crate::card::CardRules::unsupported(),
);

// ONS 249 — Bloodline Shaman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOODLINE_SHAMAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Bloodline Shaman",
    "5fdfc473-8477-4c04-a4e7-ecac1b0a5716",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// ONS 250 — Broodhatch Nantuko
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BROODHATCH_NANTUKO: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Broodhatch Nantuko",
    "38315ba3-57a0-4aa0-b1bc-4b1fcdd763d4",
    "Keith Garletts",
    crate::card::CardRules::unsupported(),
);

// ONS 251 — Centaur Glade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CENTAUR_GLADE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Centaur Glade",
    "1c75f9c8-9640-4f64-b32a-916436e461fc",
    "Alex Horley-Orlandelli",
    crate::card::CardRules::unsupported(),
);

// ONS 252 — Chain of Acid
pub(in crate::card::sets) static CHAIN_OF_ACID: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Chain of Acid",
    "1d47ddca-a363-4ab7-b7f2-d0e0043c9916",
    "Ron Spencer",
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target noncreature permanent. Then that permanent's controller may copy this spell and may choose a new target for that copy.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
            EffectDef::May {
                player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                effect: &EffectDef::CopyStackObject(&crate::card::CopyStackObjectDef {
                    object: EffectRecipientDef::object(ObjectRefDef::ResolvingObject),
                    controller: PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                    count: ValueDef::Constant(1),
                    retarget: true,
                    colors: None,
                }),
            },
        ]),
    )),
);

// ONS 253 — Crown of Vigor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROWN_OF_VIGOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Crown of Vigor",
    "e7e320a6-88e2-4be1-97e2-30e0f3c2e450",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// ONS 254 — Elven Riders (reprint)
const ELVEN_RIDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::ELVEN_RIDERS,
    "f7c1aa30-0271-48d9-b9d0-3b1da26d98bf",
    "Darrell Riche",
);

// ONS 255 — Elvish Guidance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_GUIDANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Elvish Guidance",
    "8698c46b-2628-4482-88f9-e37a01ade274",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// ONS 256 — Elvish Pathcutter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_PATHCUTTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Elvish Pathcutter",
    "c7d810b8-1a15-46cc-9d9d-871ac43b7036",
    "Todd Lockwood",
    crate::card::CardRules::unsupported(),
);

// ONS 257 — Elvish Pioneer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_PIONEER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Elvish Pioneer",
    "7e71fc2d-643b-4fad-89a8-624d330895d6",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// ONS 258 — Elvish Scrapper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_SCRAPPER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Elvish Scrapper",
    "ae85fafb-114b-4fd8-ac4c-5ada57054705",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// ONS 259 — Elvish Vanguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_VANGUARD: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Elvish Vanguard",
    "455c6923-8d0e-4a7f-a5c0-add8db519ee3",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// ONS 260 — Elvish Warrior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_WARRIOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Elvish Warrior",
    "2c6b767b-49e5-4845-9b3f-29540e5fa330",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// ONS 261 — Enchantress's Presence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENCHANTRESS_S_PRESENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Enchantress's Presence",
    "75def198-99d6-4b0a-8878-5151f44bc0a4",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// ONS 262 — Everglove Courier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EVERGLOVE_COURIER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Everglove Courier",
    "13bf5786-e41a-4839-b8a0-5c7a413b23d0",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// ONS 263 — Explosive Vegetation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXPLOSIVE_VEGETATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Explosive Vegetation",
    "da6efd31-ab5e-46ff-80d2-9382438e302c",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// ONS 264 — Gigapede
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIGAPEDE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Gigapede",
    "0a96a608-9237-41c1-824c-89d5fad939ad",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// ONS 265 — Heedless One
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEEDLESS_ONE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Heedless One",
    "ea338499-26a0-44e5-8999-f264644184d1",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// ONS 266 — Hystrodon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HYSTRODON: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Hystrodon",
    "1c964473-7c54-4c2d-a3eb-dba01c842103",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// ONS 267 — Invigorating Boon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INVIGORATING_BOON: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Invigorating Boon",
    "c46f324b-63c6-4fb5-a80a-e9da51c3eb77",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// ONS 268 — Kamahl, Fist of Krosa
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAMAHL_FIST_OF_KROSA: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Kamahl, Fist of Krosa",
    "150d5229-b1a5-42cf-bf6a-04d246f1124f",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// ONS 269 — Kamahl's Summons
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAMAHL_S_SUMMONS: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Kamahl's Summons",
    "0edc37c6-b6a8-424f-95dd-928d03c28542",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// ONS 270 — Krosan Colossus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROSAN_COLOSSUS: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Krosan Colossus",
    "a804f3c0-5ebf-43ca-b200-09f7c1bbe902",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// ONS 271 — Krosan Groundshaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROSAN_GROUNDSHAKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Krosan Groundshaker",
    "82105090-5f71-4690-9ade-187354311ae3",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// ONS 272 — Krosan Tusker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROSAN_TUSKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Krosan Tusker",
    "0b872f85-60c5-44c4-956d-a8aa8132908b",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// ONS 273 — Leery Fogbeast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEERY_FOGBEAST: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Leery Fogbeast",
    "56125660-2307-4270-a947-f1f4ad63841c",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// ONS 274 — Mythic Proportions
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MYTHIC_PROPORTIONS: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Mythic Proportions",
    "829069cf-7e63-4443-b679-65ad15d6ca5e",
    "Jim Nelson",
    crate::card::CardRules::unsupported(),
);

// ONS 275 — Naturalize
pub(in crate::card::sets) static NATURALIZE: CardRecord = CardRecord::new(
    CardSet::Onslaught,
    "Naturalize",
    "c0acc41f-b55b-47cb-8803-d39d72788799",
    "Ron Spears",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::destroy_target(
        "Destroy target artifact or enchantment.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Enchantment),
        ])),
        true,
    )),
);

// ONS 276 — Overwhelming Instinct
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OVERWHELMING_INSTINCT: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Overwhelming Instinct",
    "2d9e3793-7ddc-45c5-b25d-acd5cb96026f",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// ONS 277 — Primal Boost
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRIMAL_BOOST: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Primal Boost",
    "f1b91a5a-9328-4fc6-a2f6-a7879281e145",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// ONS 278 — Ravenous Baloth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAVENOUS_BALOTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Ravenous Baloth",
    "c98182d6-5b25-4493-9286-f29633e1bec4",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// ONS 279 — Run Wild
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUN_WILD: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Run Wild",
    "939a7354-162c-489d-955d-4df17b930e1c",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// ONS 280 — Serpentine Basilisk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERPENTINE_BASILISK: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Serpentine Basilisk",
    "4052a5af-20b2-4817-8c94-78d488ee220f",
    "Franz Vohwinkel",
    crate::card::CardRules::unsupported(),
);

// ONS 281 — Silklash Spider
pub(in crate::card::sets) static SILKLASH_SPIDER: CardRecord = CardRecord::new(
    CardSet::Onslaught,
    "Silklash Spider",
    "e41680e2-6689-4263-a5a3-9fb2e4280d52",
    "Iain McCaig",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Spider"], 2, 7).with_abilities(&[
        abilities::reach(),
        AbilityDef::activated(
            "{X}{G}{G}: This creature deals X damage to each creature with flying.",
            &[AbilityCostDef::Mana(mana_cost!("{X}{G}{G}"))],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                amount: ValueDef::ChosenX,
            },
        ),
    ]),
);

// ONS 282 — Silvos, Rogue Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILVOS_ROGUE_ELEMENTAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Silvos, Rogue Elemental",
    "3e48715c-6ff7-4b0c-aa7e-a2c901215426",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// ONS 283 — Snarling Undorak
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNARLING_UNDORAK: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Snarling Undorak",
    "05788d63-6210-44f2-9ae4-e55e9507a3a9",
    "Justin Sweet",
    crate::card::CardRules::unsupported(),
);

// ONS 284 — Spitting Gourna
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPITTING_GOURNA: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Spitting Gourna",
    "746b98bf-5398-4a00-b4fe-a990ea9cfd77",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ONS 285 — Stag Beetle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STAG_BEETLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Stag Beetle",
    "72cc64b9-f5b9-42d3-9921-564c4c9f2c77",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// ONS 286 — Steely Resolve
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STEELY_RESOLVE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Steely Resolve",
    "b88c530a-abc3-4cc4-8a48-5b76e1504a3c",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// ONS 287 — Symbiotic Beast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SYMBIOTIC_BEAST: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Symbiotic Beast",
    "bb61443d-e47a-4fe1-b777-67a3670a5a56",
    "Franz Vohwinkel",
    crate::card::CardRules::unsupported(),
);

// ONS 288 — Symbiotic Elf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SYMBIOTIC_ELF: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Symbiotic Elf",
    "33af35c6-7802-4366-ad20-1e330b4957ef",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// ONS 289 — Symbiotic Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SYMBIOTIC_WURM: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Symbiotic Wurm",
    "a60313ca-10cc-4c33-a557-1401c5721e3b",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// ONS 290 — Taunting Elf (reprint)
const TAUNTING_ELF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_uds::TAUNTING_ELF,
    "6b24af94-9632-47da-9bf3-e81bb743cd43",
    "Rebecca Guay",
);

// ONS 291 — Tempting Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEMPTING_WURM: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Tempting Wurm",
    "857c2b6c-cfdf-4c88-a334-2937cb7db603",
    "Bob Petillo",
    crate::card::CardRules::unsupported(),
);

// ONS 292 — Towering Baloth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOWERING_BALOTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Towering Baloth",
    "2a8cc948-28ff-4bbe-b8c9-71de37478023",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// ONS 293 — Treespring Lorian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREESPRING_LORIAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Treespring Lorian",
    "f525d7ce-37d3-4989-beb4-173447cb5294",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ONS 294 — Tribal Unity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRIBAL_UNITY: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Tribal Unity",
    "6f7b5ddf-d5a6-42bf-a196-7e834dbdb3dc",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// ONS 295 — Venomspout Brackus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENOMSPOUT_BRACKUS: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Venomspout Brackus",
    "0774771c-5373-4636-9174-d06e7d635183",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ONS 296 — Vitality Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VITALITY_CHARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Vitality Charm",
    "e1abae21-ed8f-4e21-b227-f721b840c11f",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// ONS 297 — Voice of the Woods
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOICE_OF_THE_WOODS: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Voice of the Woods",
    "1ebb4668-eebf-4b7e-ae29-75fff5963868",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ONS 298 — Wall of Mulch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_MULCH: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Wall of Mulch",
    "8b3b4448-50f0-4996-94a1-db9ce356d925",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// ONS 299 — Weird Harvest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WEIRD_HARVEST: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Weird Harvest",
    "3cdfa8b3-393b-4bb6-9265-faa4ab7126d2",
    "Bob Petillo",
    crate::card::CardRules::unsupported(),
);

// ONS 300 — Wellwisher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WELLWISHER: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Wellwisher",
    "be95ab7c-0e77-4293-aa48-ee54902a363f",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// ONS 301 — Wirewood Elf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WIREWOOD_ELF: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Wirewood Elf",
    "10a34e31-97f1-40e8-9d91-a8139af7f096",
    "Jerry Tiritilli",
    crate::card::CardRules::unsupported(),
);

// ONS 302 — Wirewood Herald
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WIREWOOD_HERALD: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Wirewood Herald",
    "35724e9f-efa6-47e7-ab4d-7defe38ba576",
    "Alex Horley-Orlandelli",
    crate::card::CardRules::unsupported(),
);

// ONS 303 — Wirewood Pride
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WIREWOOD_PRIDE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Wirewood Pride",
    "a559e844-06c9-4953-bc2c-a58e4170fe47",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// ONS 304 — Wirewood Savage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WIREWOOD_SAVAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Wirewood Savage",
    "99982622-98bc-45ae-8642-41cd543f32a8",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// ONS 305 — Words of Wilding
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORDS_OF_WILDING: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Words of Wilding",
    "fdb9565f-5b09-4127-b169-3146079dab84",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// ONS 306 — Cryptic Gateway
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRYPTIC_GATEWAY: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Cryptic Gateway",
    "7f379966-6a0a-434c-8682-1cf528a9a4a1",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// ONS 307 — Doom Cannon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOOM_CANNON: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Doom Cannon",
    "4abde0d7-266b-41bd-ade1-c4d93507eb16",
    "Matthew Mitchell",
    crate::card::CardRules::unsupported(),
);

// ONS 308 — Dream Chisel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAM_CHISEL: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Dream Chisel",
    "e89610e9-f1d3-4332-901a-2598bf01d61d",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// ONS 309 — Riptide Replicator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIPTIDE_REPLICATOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Riptide Replicator",
    "41bb314f-237a-43fc-95c8-b26188dc4476",
    "Doug Chaffee",
    crate::card::CardRules::unsupported(),
);

// ONS 310 — Slate of Ancestry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLATE_OF_ANCESTRY: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Slate of Ancestry",
    "ae596e8c-04f5-48b0-b5e2-683c74912e85",
    "Corey D. Macourek",
    crate::card::CardRules::unsupported(),
);

// ONS 311 — Tribal Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRIBAL_GOLEM: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Tribal Golem",
    "6e208be1-8b24-4048-90b2-6389f08043d1",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// ONS 312 — Barren Moor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARREN_MOOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Barren Moor",
    "45be3811-a223-4c45-9b24-0317f2d53c60",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ONS 313 — Bloodstained Mire
pub(in crate::card::sets) static BLOODSTAINED_MIRE: CardRecord = CardRecord::new(
    CardSet::Onslaught,
    "Bloodstained Mire",
    "68c72226-6f52-4322-8b14-18737293dfa0",
    "Rob Alexander",
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Swamp or Mountain card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Swamp, BasicLandType::Mountain],
    ),
);

// ONS 314 — Contested Cliffs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONTESTED_CLIFFS: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Contested Cliffs",
    "8d6363ea-3814-4014-ad9e-1066c72d907c",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// ONS 315 — Daru Encampment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARU_ENCAMPMENT: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Daru Encampment",
    "c5869f08-fac8-44b6-8142-7d7ecccab414",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// ONS 316 — Flooded Strand
pub(in crate::card::sets) static FLOODED_STRAND: CardRecord = CardRecord::new(
    CardSet::Onslaught,
    "Flooded Strand",
    "b4e3d844-d3b4-41d8-921d-c1cb3af343f8",
    "Rob Alexander",
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Plains or Island card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Plains, BasicLandType::Island],
    ),
);

// ONS 317 — Forgotten Cave
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORGOTTEN_CAVE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Forgotten Cave",
    "c5202668-a32c-4473-b272-e86264992576",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// ONS 318 — Goblin Burrows
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_BURROWS: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Goblin Burrows",
    "a5064cd2-8762-4e08-8c3c-be6f31e9ab61",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// ONS 319 — Grand Coliseum
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAND_COLISEUM: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Grand Coliseum",
    "c2dc8061-a855-4a81-9eb7-350b355a9b3f",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// ONS 320 — Lonely Sandbar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LONELY_SANDBAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Lonely Sandbar",
    "d8ddab06-aff7-4c40-bcaa-10cbfe899dd9",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ONS 321 — Polluted Delta
pub(in crate::card::sets) static POLLUTED_DELTA: CardRecord = CardRecord::new(
    CardSet::Onslaught,
    "Polluted Delta",
    "0f7585c8-9e21-4eef-afc1-2852de23db2f",
    "Rob Alexander",
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for an Island or Swamp card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Island, BasicLandType::Swamp],
    ),
);

// ONS 322 — Riptide Laboratory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIPTIDE_LABORATORY: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Riptide Laboratory",
    "d993c973-2eb6-423c-8ee9-10749a751524",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// ONS 323 — Seaside Haven
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEASIDE_HAVEN: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Seaside Haven",
    "9c940a6b-3c5e-4ce2-92b6-63e2cb575c15",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// ONS 324 — Secluded Steppe
pub(in crate::card::sets) static SECLUDED_STEPPE: CardRecord = CardRecord::new(
    CardSet::Onslaught,
    "Secluded Steppe",
    "ea454280-f7f4-4315-bb46-b56050c02c97",
    "Heather Hudson",
    // The tapped land you play on a turn you had nothing to do, or the card
    // you cycle away on a turn you did.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
        ),
        abilities::cycling(
            "Cycling {W} ({W}, Discard this card: Draw a card.)",
            mana_cost!("{W}"),
        ),
    ]),
);

// ONS 325 — Starlit Sanctum
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARLIT_SANCTUM: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Starlit Sanctum",
    "ace5e601-2583-4d9c-8bdf-aa33666c717c",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// ONS 326 — Tranquil Thicket
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRANQUIL_THICKET: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Tranquil Thicket",
    "afcb7cef-8aeb-4c84-88e9-6df17768e292",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ONS 327 — Unholy Grotto
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNHOLY_GROTTO: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Unholy Grotto",
    "52f464a9-586c-4cf3-894b-b407c9f4dcb8",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// ONS 328 — Windswept Heath
pub(in crate::card::sets) static WINDSWEPT_HEATH: CardRecord = CardRecord::new(
    CardSet::Onslaught,
    "Windswept Heath",
    "7a7c5941-9c8a-4a40-9efb-a84f05c58e53",
    "Anthony S. Waters",
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Forest or Plains card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Forest, BasicLandType::Plains],
    ),
);

// ONS 329 — Wirewood Lodge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WIREWOOD_LODGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Onslaught,
    "Wirewood Lodge",
    "3d251490-41bb-4ad3-bfd0-a5e66ee42598",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// ONS 330 — Wooded Foothills
pub(in crate::card::sets) static WOODED_FOOTHILLS: CardRecord = CardRecord::new(
    CardSet::Onslaught,
    "Wooded Foothills",
    "cdad38f7-9dfa-4f1b-9fac-41ab2b253f53",
    "Rob Alexander",
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Mountain or Forest card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Mountain, BasicLandType::Forest],
    ),
);

// ONS 331 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "7bf7d68a-dbd0-45f3-acbb-59ee38e6057e",
    "Rob Alexander",
);

// ONS 332 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "e52ed647-bd30-40a5-b648-0b98d1a3fd4a",
    "Matthew Mitchell",
);

// ONS 333 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "854a255e-fd89-4c5d-b97b-416a9ac70960",
    "David Martin",
);

// ONS 334 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "fd7babbe-f8c1-4e7c-8de2-2224dd357de4",
    "David Day",
);

// ONS 335 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "36e062ec-df51-40c0-ad8a-2ee1cb8f8f17",
    "Tony Szczudlo",
);

// ONS 336 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "6e8c0e52-8482-4c33-bc5d-26eaad922e72",
    "Bradley Williams",
);

// ONS 337 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "1dac3bfe-884b-4875-bc7d-df564eb014cd",
    "Matt Thompson",
);

// ONS 338 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    3,
    "189a09b8-46d2-4ef6-b7cc-9e510d1ea0b8",
    "Randy Elliott",
);

// ONS 339 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "0356ae45-e5ca-46b9-8ebc-42bf4776e89c",
    "Tony Szczudlo",
);

// ONS 340 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "a6285f63-a5d8-4b8b-a6dd-51ce7968fbaf",
    "Doug Chaffee",
);

// ONS 341 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "7aa97b25-1ea0-4351-ab9f-f06c8bb4d044",
    "Dan Frazier",
);

// ONS 342 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    3,
    "8e10b125-eaa6-4630-a6fe-6b1805921f07",
    "Pete Venters",
);

// ONS 343 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "05f9bdca-0d54-46c7-b803-9083dfc9ee24",
    "Tony Szczudlo",
);

// ONS 344 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "b6d39f35-c7b2-43b2-aee3-4ff2cd3e37e7",
    "Sam Wood",
);

// ONS 345 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "e8aade2d-5cf5-44f6-9095-aa3756b1c1dd",
    "David Day",
);

// ONS 346 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    3,
    "fd194fb1-0d3a-4eff-a446-240d18dad43c",
    "Heather Hudson",
);

// ONS 347 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "b361b42d-401f-440a-bae9-35338b5dde0e",
    "John Avon",
);

// ONS 348 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "4d8edfee-7837-450a-bcf3-a7bb25670056",
    "John Matson",
);

// ONS 349 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "7b0af992-80e0-4ac6-a828-5eaac47eaff6",
    "John Avon",
);

// ONS 350 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    3,
    "835a4eed-a308-428d-ac85-e385b5d47d8e",
    "David Martin",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AKROMA_S_BLESSING,
    &AKROMAS_VENGEANCE,
    &ANCESTOR_S_PROPHET,
    &ASTRAL_SLIDE,
    &AURA_EXTRACTION,
    &AURIFICATION,
    &AVEN_BRIGADIER,
    &AVEN_SOULGAZER,
    &BATTLEFIELD_MEDIC,
    &CATAPULT_MASTER,
    &CATAPULT_SQUAD,
    &CHAIN_OF_SILENCE,
    &CIRCLE_OF_SOLACE,
    &CONVALESCENT_CARE,
    &CROWD_FAVORITES,
    &CROWN_OF_AWE,
    &CRUDE_RAMPART,
    &DARU_CAVALIER,
    &DARU_HEALER,
    &DARU_LANCER,
    &DAUNTING_DEFENDER,
    &DAWNING_PURIST,
    &DEFENSIVE_MANEUVERS,
    &DEMYSTIFY,
    &DIVE_BOMBER,
    &DOUBTLESS_ONE,
    &EXALTED_ANGEL,
    &FOOTHILL_GUIDE,
    &GLARECASTER,
    &GLORY_SEEKER,
    &GRASSLAND_CRUSADER,
    &GRAVEL_SLINGER,
    &GUSTCLOAK_HARRIER,
    &GUSTCLOAK_RUNNER,
    &GUSTCLOAK_SAVIOR,
    &GUSTCLOAK_SENTINEL,
    &GUSTCLOAK_SKIRMISHER,
    &HARSH_MERCY,
    &IMPROVISED_ARMOR,
    &INSPIRIT,
    &IRONFIST_CRUSHER,
    &JARETH_LEONINE_TITAN,
    &MOBILIZATION,
    &NOVA_CLERIC,
    &OBLATION,
    &PEARLSPEAR_COURIER,
    &PIETY_CHARM,
    &RENEWED_FAITH,
    &RIGHTEOUS_CAUSE,
    &SANDSKIN,
    &SHARED_TRIUMPH,
    &SHIELDMAGE_ELDER,
    &SIGIL_OF_THE_NEW_DAWN,
    &SUNFIRE_BALM,
    &TRUE_BELIEVER,
    &UNIFIED_STRIKE,
    &WEATHERED_WAYFARER,
    &WHIPCORDER,
    &WORDS_OF_WORSHIP,
    &AIRBORNE_AID,
    &ANNEX,
    &APHETTO_ALCHEMIST,
    &APHETTO_GRIFTER,
    &ARCANIS_THE_OMNIPOTENT,
    &ARTIFICIAL_EVOLUTION,
    &ASCENDING_AVEN,
    &AVEN_FATESHAPER,
    &BACKSLIDE,
    &BLATANT_THIEVERY,
    &CALLOUS_OPPRESSOR,
    &CHAIN_OF_VAPOR,
    &CHOKING_TETHERS,
    &COMPLICATE,
    &CRAFTY_PATHMAGE,
    &CROWN_OF_ASCENSION,
    &DISCOMBOBULATE,
    &DISPERSING_ORB,
    &DISRUPTIVE_PITMAGE,
    &ESSENCE_FRACTURE,
    &FLEETING_AVEN,
    &FUTURE_SIGHT,
    &GHOSTHELM_COURIER,
    &GRAXIPLON,
    &IMAGECRAFTER,
    &INFORMATION_DEALER,
    &IXIDOR_REALITY_SCULPTOR,
    &IXIDOR_S_WILL,
    &MAGE_S_GUILE,
    &MISTFORM_DREAMER,
    &MISTFORM_MASK,
    &MISTFORM_MUTANT,
    &MISTFORM_SHRIEKER,
    &MISTFORM_SKYREAVER,
    &MISTFORM_STALKER,
    &MISTFORM_WALL,
    &NAMELESS_ONE,
    &PEER_PRESSURE,
    &PSYCHIC_TRANCE,
    &QUICKSILVER_DRAGON,
    &READ_THE_RUNES,
    &REMINISCE,
    &RIPTIDE_BIOLOGIST,
    &RIPTIDE_CHRONOLOGIST,
    &RIPTIDE_ENTRANCER,
    &RIPTIDE_SHAPESHIFTER,
    &RUMMAGING_WIZARD,
    &SAGE_AVEN,
    &SCREAMING_SEAHAWK,
    &SEA_S_CLAIM,
    &SLIPSTREAM_EEL,
    &SPY_NETWORK,
    &STANDARDIZE,
    &SUPREME_INQUISITOR,
    &TRADE_SECRETS,
    &TRICKERY_CHARM,
    &VOIDMAGE_PRODIGY,
    &WHEEL_AND_DEAL,
    &WORDS_OF_WIND,
    &ACCURSED_CENTAUR,
    &ANURID_MURKDIVER,
    &APHETTO_DREDGING,
    &APHETTO_VULTURE,
    &BLACKMAIL,
    &BONEKNITTER,
    &CABAL_ARCHON,
    &CABAL_EXECUTIONER,
    &CABAL_SLAVER,
    &CHAIN_OF_SMOG,
    &COVER_OF_DARKNESS,
    &CROWN_OF_SUSPICION,
    &CRUEL_REVIVAL,
    &DEATH_MATCH,
    &DEATH_PULSE,
    &DIRGE_OF_DREAD,
    &DISCIPLE_OF_MALICE,
    &DOOMED_NECROMANCER,
    &EBONBLADE_REAPER,
    &ENDEMIC_PLAGUE,
    &ENTRAILS_FEASTER,
    &FADE_FROM_MEMORY,
    &FALLEN_CLERIC,
    &FALSE_CURE,
    &FEEDING_FRENZY,
    &FESTERING_GOBLIN,
    &FRIGHTSHROUD_COURIER,
    &GANGRENOUS_GOLIATH,
    &GLUTTONOUS_ZOMBIE,
    &GRAVESPAWN_SOVEREIGN,
    &GRINNING_DEMON,
    &HAUNTED_CADAVER,
    &HEAD_GAMES,
    &HEADHUNTER,
    &INFEST,
    &MISERY_CHARM,
    &NANTUKO_HUSK,
    &OVERSOLD_CEMETERY,
    &PATRIARCH_S_BIDDING,
    &PROFANE_PRAYERS,
    &PROWLING_PANGOLIN,
    &ROTLUNG_REANIMATOR,
    &SCREECHING_BUZZARD,
    &SEVERED_LEGION,
    &SHADE_S_BREATH,
    &SHEPHERD_OF_ROT,
    &SILENT_SPECTER,
    &SMOTHER,
    &SOULLESS_ONE,
    &SPINED_BASHER,
    &STRONGARM_TACTICS,
    &SYPHON_MIND,
    &THRASHING_MUDSPAWN,
    &UNDEAD_GLADIATOR,
    &VISARA_THE_DREADFUL,
    &WALKING_DESECRATION,
    &WITHERING_HEX,
    &WORDS_OF_WASTE,
    &WRETCHED_ANURID,
    &AETHER_CHARGE,
    &AGGRAVATED_ASSAULT,
    &AIRDROP_CONDOR,
    &AVARAX,
    &BATTERING_CRAGHORN,
    &BLISTERING_FIRECAT,
    &BREAK_OPEN,
    &BRIGHTSTONE_RITUAL,
    &BUTCHER_ORGG,
    &CHAIN_OF_PLASMA,
    &CHARGING_SLATEBACK,
    &COMMANDO_RAID,
    &CROWN_OF_FURY,
    &CUSTODY_BATTLE,
    &DRAGON_ROOST,
    &DWARVEN_BLASTMINER,
    &EMBERMAGE_GOBLIN,
    &ERRATIC_EXPLOSION,
    &FEVER_CHARM,
    &FLAMESTICK_COURIER,
    &GOBLIN_MACHINIST,
    &GOBLIN_PILEDRIVER,
    &GOBLIN_PYROMANCER,
    &GOBLIN_SHARPSHOOTER,
    &GOBLIN_SKY_RAIDER,
    &GOBLIN_SLEDDER,
    &GOBLIN_TASKMASTER,
    &GRAND_MELEE,
    &GRATUITOUS_VIOLENCE,
    &INSURRECTION,
    &KABOOM,
    &LAVAMANCER_S_SKILL,
    &LIGHTNING_RIFT,
    &MANA_ECHOES,
    &MENACING_OGRE,
    &NOSY_GOBLIN,
    &PINPOINT_AVALANCHE,
    &RECKLESS_ONE,
    &RISKY_MOVE,
    &RORIX_BLADEWING,
    &SEARING_FLESH,
    &SHALESKIN_BRUISER,
    &SKIRK_COMMANDO,
    &SKIRK_FIRE_MARSHAL,
    &SKIRK_PROSPECTOR,
    &SKITTISH_VALESK,
    &SLICE_AND_DICE,
    &SNAPPING_THRAGG,
    &SOLAR_BLAST,
    &SPARKSMITH,
    &SPITFIRE_HANDLER,
    &SPURRED_WOLVERINE,
    &STARSTORM,
    &TEPHRADERM,
    &THOUGHTBOUND_PRIMOC,
    &THREATEN,
    &THUNDER_OF_HOOVES,
    &WAVE_OF_INDIFFERENCE,
    &WORDS_OF_WAR,
    &ANIMAL_MAGNETISM,
    &BARKHIDE_MAULER,
    &BIORHYTHM,
    &BIRCHLORE_RANGERS,
    &BLOODLINE_SHAMAN,
    &BROODHATCH_NANTUKO,
    &CENTAUR_GLADE,
    &CHAIN_OF_ACID,
    &CROWN_OF_VIGOR,
    &ELVISH_GUIDANCE,
    &ELVISH_PATHCUTTER,
    &ELVISH_PIONEER,
    &ELVISH_SCRAPPER,
    &ELVISH_VANGUARD,
    &ELVISH_WARRIOR,
    &ENCHANTRESS_S_PRESENCE,
    &EVERGLOVE_COURIER,
    &EXPLOSIVE_VEGETATION,
    &GIGAPEDE,
    &HEEDLESS_ONE,
    &HYSTRODON,
    &INVIGORATING_BOON,
    &KAMAHL_FIST_OF_KROSA,
    &KAMAHL_S_SUMMONS,
    &KROSAN_COLOSSUS,
    &KROSAN_GROUNDSHAKER,
    &KROSAN_TUSKER,
    &LEERY_FOGBEAST,
    &MYTHIC_PROPORTIONS,
    &NATURALIZE,
    &OVERWHELMING_INSTINCT,
    &PRIMAL_BOOST,
    &RAVENOUS_BALOTH,
    &RUN_WILD,
    &SERPENTINE_BASILISK,
    &SILKLASH_SPIDER,
    &SILVOS_ROGUE_ELEMENTAL,
    &SNARLING_UNDORAK,
    &SPITTING_GOURNA,
    &STAG_BEETLE,
    &STEELY_RESOLVE,
    &SYMBIOTIC_BEAST,
    &SYMBIOTIC_ELF,
    &SYMBIOTIC_WURM,
    &TEMPTING_WURM,
    &TOWERING_BALOTH,
    &TREESPRING_LORIAN,
    &TRIBAL_UNITY,
    &VENOMSPOUT_BRACKUS,
    &VITALITY_CHARM,
    &VOICE_OF_THE_WOODS,
    &WALL_OF_MULCH,
    &WEIRD_HARVEST,
    &WELLWISHER,
    &WIREWOOD_ELF,
    &WIREWOOD_HERALD,
    &WIREWOOD_PRIDE,
    &WIREWOOD_SAVAGE,
    &WORDS_OF_WILDING,
    &CRYPTIC_GATEWAY,
    &DOOM_CANNON,
    &DREAM_CHISEL,
    &RIPTIDE_REPLICATOR,
    &SLATE_OF_ANCESTRY,
    &TRIBAL_GOLEM,
    &BARREN_MOOR,
    &BLOODSTAINED_MIRE,
    &CONTESTED_CLIFFS,
    &DARU_ENCAMPMENT,
    &FLOODED_STRAND,
    &FORGOTTEN_CAVE,
    &GOBLIN_BURROWS,
    &GRAND_COLISEUM,
    &LONELY_SANDBAR,
    &POLLUTED_DELTA,
    &RIPTIDE_LABORATORY,
    &SEASIDE_HAVEN,
    &SECLUDED_STEPPE,
    &STARLIT_SANCTUM,
    &TRANQUIL_THICKET,
    &UNHOLY_GROTTO,
    &WINDSWEPT_HEATH,
    &WIREWOOD_LODGE,
    &WOODED_FOOTHILLS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    DISCIPLE_OF_GRACE_REPRINT,
    PACIFISM_REPRINT,
    CLONE_REPRINT,
    MEDDLE_REPRINT,
    SWAT_REPRINT,
    SYPHON_SOUL_REPRINT,
    EMBERMAGE_GOBLIN_ALTERNATE_1,
    LAY_WASTE_REPRINT,
    SHOCK_REPRINT,
    ELVEN_RIDERS_REPRINT,
    TAUNTING_ELF_REPRINT,
    PLAINS_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    PLAINS_ALTERNATE_3,
    ISLAND_REPRINT,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    ISLAND_ALTERNATE_3,
    SWAMP_REPRINT,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    SWAMP_ALTERNATE_3,
    MOUNTAIN_REPRINT,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_3,
    FOREST_REPRINT,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
    FOREST_ALTERNATE_3,
];
