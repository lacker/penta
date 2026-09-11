//! SOI card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::PlayerRelation;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::card::tokens;
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "SOI",
    slug: "shadows-over-innistrad",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const CLUE_TOKEN: TokenCharacteristics = tokens::clue().with_art(CardArt::new(
    "f2c859e1-181e-44d1-afbd-bbd6e52cf42a",
    "John Avon",
));

// SOI 31 — Odric, Lunarch Marshal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ODRIC_LUNARCH_MARSHAL_31: CardRecord = CardRecord::new(
    "Odric, Lunarch Marshal",
    "5c77c30f-d813-46e6-9cdd-938b4a6359ad",
    "Chase Stone",
    crate::card::CardRules::unsupported(),
);

// SOI 44 — Thraben Inspector
pub(in crate::card::sets) static THRABEN_INSPECTOR: CardRecord = CardRecord::new(
    "Thraben Inspector",
    "d140c3b7-ca78-483d-baeb-307b624fea8b",
    "Matt Stewart",
    // One mana for a body and a card: the Clue is the reason the 1/2 is
    // worth playing, and the body is the reason the card is cheap.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, investigate. (Create a Clue token. It's an artifact with \
             \"{2}, Sacrifice this token: Draw a card.\")",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))),
        ),
    ),
);

// SOI 61 — Essence Flux
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ESSENCE_FLUX_61: CardRecord = CardRecord::new(
    "Essence Flux",
    "639bdbb5-8c2d-439d-bcea-dc54da9686ea",
    "Seb McKinnon",
    crate::card::CardRules::unsupported(),
);

// SOI 98 — Alms of the Vein
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALMS_OF_THE_VEIN_98: CardRecord = CardRecord::new(
    "Alms of the Vein",
    "79b80948-a3cd-4962-8fce-d58f2db7e68e",
    "David Gaillet",
    crate::card::CardRules::unsupported(),
);

// SOI 102 — Biting Rain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BITING_RAIN_102: CardRecord = CardRecord::new(
    "Biting Rain",
    "5ac62d2f-6834-4d98-b69d-bd7b5831d981",
    "John Stanko",
    crate::card::CardRules::unsupported(),
);

// SOI 105 — Crow of Dark Tidings
pub(in crate::card::sets) static CROW_OF_DARK_TIDINGS: CardRecord = CardRecord::new(
    "Crow of Dark Tidings",
    "14e4d1b5-72de-4062-9fdd-e9bfd655ee79",
    "Tianhua X",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie", "Bird"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "When this creature enters or dies, mill two cards. (Put the \
             top two cards of your library into your graveyard.)",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
            ]),
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// SOI 111 — From Under the Floorboards
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FROM_UNDER_THE_FLOORBOARDS_111: CardRecord = CardRecord::new(
    "From Under the Floorboards",
    "3e644706-223d-4e56-9614-b224e281be2f",
    "Steven Belledin",
    crate::card::CardRules::unsupported(),
);

// SOI 114 — Gisa's Bidding
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GISA_S_BIDDING_114: CardRecord = CardRecord::new(
    "Gisa's Bidding",
    "e01e904c-7d8e-447b-90cb-1f4ae3fb304d",
    "Jason Felix",
    crate::card::CardRules::unsupported(),
);

// SOI 126 — Murderous Compulsion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MURDEROUS_COMPULSION_126: CardRecord = CardRecord::new(
    "Murderous Compulsion",
    "33b94db1-ac8c-4667-81d5-408df0f30879",
    "David Palumbo",
    crate::card::CardRules::unsupported(),
);

// SOI 145 — Avacyn's Judgment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVACYN_S_JUDGMENT_145: CardRecord = CardRecord::new(
    "Avacyn's Judgment",
    "0c5f44ce-1464-4282-9afa-20e9ea44c613",
    "Victor Adame Minguez",
    crate::card::CardRules::unsupported(),
);

// SOI 173 — Malevolent Whispers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MALEVOLENT_WHISPERS_173: CardRecord = CardRecord::new(
    "Malevolent Whispers",
    "674d33d6-dfd4-4972-aaa3-6de0236a8c45",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// SOI 189 — Vessel of Volatility
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VESSEL_OF_VOLATILITY_189: CardRecord = CardRecord::new(
    "Vessel of Volatility",
    "81647b86-2c84-4a14-8d5a-919f7a5b8bc7",
    "Kieran Yanner",
    crate::card::CardRules::unsupported(),
);

// SOI 200 — Cryptolith Rite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRYPTOLITH_RITE_200: CardRecord = CardRecord::new(
    "Cryptolith Rite",
    "2910adcd-882a-46af-8236-ca1a9e2c19ab",
    "Zack Stella",
    crate::card::CardRules::unsupported(),
);

// SOI 203 — Duskwatch Recruiter // Krallenhorde Howler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUSKWATCH_RECRUITER_KRALLENHORDE_HOWLER_203: CardRecord =
    CardRecord::new(
        "Duskwatch Recruiter // Krallenhorde Howler",
        "e1915d93-c7dd-4bb7-bd5c-63a359a02b97",
        "Craig J Spearing",
        crate::card::CardRules::unsupported(),
    );

// SOI 216 — Loam Dryad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOAM_DRYAD_216: CardRecord = CardRecord::new(
    "Loam Dryad",
    "61c9441d-18d9-4ec6-859e-e9a7893b54e3",
    "Jose Cabrera",
    crate::card::CardRules::unsupported(),
);

// SOI 223 — Rabid Bite
pub(in crate::card::sets) static RABID_BITE: CardRecord = CardRecord::new(
    "Rabid Bite",
    "2f573622-877b-4d21-adfc-40a32b7c2e6d",
    "Karl Kopinski",
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control deals damage equal to its power \
         to target creature you don't control.",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            }),
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            }),
        ],
        EffectDef::damage_from(
            ObjectRefDef::Target(TargetIndex::PRIMARY),
            EffectRecipientDef::Target(TargetIndex(1)),
            ValueDef::TargetPower(TargetIndex::PRIMARY),
        ),
    )]),
);

// SOI 233 — Tireless Tracker
pub(in crate::card::sets) static TIRELESS_TRACKER: CardRecord = CardRecord::new(
    "Tireless Tracker",
    "ee8e9928-d9b2-4570-adb8-44b34115decd",
    "Eric Deschamps",
    // Three mana for a 3/2 that turns every land after it into a card, and
    // grows every time one of those cards is cashed in.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Scout"], 3, 2).with_abilities(&[
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, investigate. (Create a Clue token. \
             It's an artifact with \"{2}, Sacrifice this token: Draw a card.\")",
            // A land you control arriving, which is what landfall is: the Tracker's
            // own arrival is not one, and neither is a land somebody else plays.
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))),
        ),
        AbilityDef::triggered(
            "Whenever you sacrifice a Clue, put a +1/+1 counter on this creature.",
            TriggerEventDef::Sacrificed {
                object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Clue")),
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// SOI 258 — Magnifying Glass
pub(in crate::card::sets) static MAGNIFYING_GLASS: CardRecord = CardRecord::new(
    "Magnifying Glass",
    "f7a708d5-f757-4fcf-a167-5b5920c6adeb",
    "Dan Murayama Scott",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{4}, {T}: Investigate. (Create a Clue token. It's an artifact \
             with \"{2}, Sacrifice this token: Draw a card.\")",
            &[CostDef::Mana(mana_cost!("{4}")), CostDef::TapSource],
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
    ]),
);

// SOI 274 — Fortified Village
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORTIFIED_VILLAGE_274: CardRecord = CardRecord::new(
    "Fortified Village",
    "1feb9dc1-671d-43ad-ae22-ed1a9916b140",
    "Cliff Childs",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ODRIC_LUNARCH_MARSHAL_31,
    &THRABEN_INSPECTOR,
    &ESSENCE_FLUX_61,
    &ALMS_OF_THE_VEIN_98,
    &BITING_RAIN_102,
    &CROW_OF_DARK_TIDINGS,
    &FROM_UNDER_THE_FLOORBOARDS_111,
    &GISA_S_BIDDING_114,
    &MURDEROUS_COMPULSION_126,
    &AVACYN_S_JUDGMENT_145,
    &MALEVOLENT_WHISPERS_173,
    &VESSEL_OF_VOLATILITY_189,
    &CRYPTOLITH_RITE_200,
    &DUSKWATCH_RECRUITER_KRALLENHORDE_HOWLER_203,
    &LOAM_DRYAD_216,
    &RABID_BITE,
    &TIRELESS_TRACKER,
    &MAGNIFYING_GLASS,
    &FORTIFIED_VILLAGE_274,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
