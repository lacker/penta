//! Portal Three Kingdoms card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::SubtypeDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::sets::y1997::visions as catalog_vis;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "PTK",
    slug: "portal-three-kingdoms",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// PTK 17 — Ravages of War
pub(in crate::card::sets) static RAVAGES_OF_WAR_17: CardRecord = CardRecord::new(
    "Ravages of War",
    "11dca9ba-b27f-4af8-9962-3794e743886f",
    "Fang Yue",
    CardRules::new_sorcery(mana_cost!("{3}{W}")).with_abilities(&[AbilityDef::spell(
        "Destroy all lands.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Land),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            then: None,
        },
    )]),
);

// PTK 38 — Capture of Jingzhou
pub(in crate::card::sets) static CAPTURE_OF_JINGZHOU_38: CardRecord = CardRecord::new(
    "Capture of Jingzhou",
    "d2df84f2-08e8-43e4-825f-dccfe096d92b",
    "Jack Wei",
    CardRules::new_sorcery(mana_cost!("{3}{U}{U}")).with_abilities(&[AbilityDef::spell(
        "Take an extra turn after this one.",
        EffectDef::TakeExtraTurn {
            player: EffectRecipientDef::Controller,
        },
    )]),
);

// PTK 71 — Corrupt Court Official
pub(in crate::card::sets) static CORRUPT_COURT_OFFICIAL: CardRecord = CardRecord::new(
    "Corrupt Court Official",
    "9d3ba2e3-e680-47cd-81c5-555deea7d00f",
    "Li Yousong",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Advisor"], 1, 1).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, target opponent discards a card.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ]),
);

// PTK 78 — Imperial Seal
pub(in crate::card::sets) static IMPERIAL_SEAL: CardRecord = CardRecord::new(
    "Imperial Seal",
    "822e30db-40c5-4099-868b-185ad9b7c7dc",
    "Li Tie",
    // Vampiric Tutor's clause at sorcery speed, which is the whole of the
    // difference: the card you want is on top of your library, and you wait
    // a turn to draw it.
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell(
        "Search your library for a card, then shuffle and put that card on top. You lose 2 life.",
        EffectDef::Sequence(&catalog_vis::VAMPIRIC_TUTOR_EFFECT),
    )),
);

// PTK 113 — Imperial Recruiter
pub(in crate::card::sets) static IMPERIAL_RECRUITER_113: CardRecord = CardRecord::new(
    "Imperial Recruiter",
    "1c473253-3992-4cc1-8b46-5d1da308c537",
    "Mitsuaki Sagiri",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Advisor"], 1, 1).with_abilities(&[
AbilityDef::triggered("When this creature enters, search your library for a creature card with power 2 or less, reveal it, put it into your hand, then shuffle.", TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)), EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Not(&ObjectPredicateDef::PowerGreaterThan(ValueDef::Constant(2)))]), minimum: 0, maximum: ValueDef::Constant(1), reveal: true, destination: ZoneKind::Hand, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None })
]),
);

// PTK 124 — Warrior's Oath
// Audit: unsupported — Extra turns are supported, but no delayed-loss trigger can be bound to the specific extra turn this spell creates.
pub(in crate::card::sets) static WARRIOR_S_OATH_124: CardRecord = CardRecord::new(
    "Warrior's Oath",
    "d582861a-ca6e-4b74-adf0-3eb588ea5ed2",
    "Mitsuaki Sagiri",
    crate::card::CardRules::unsupported(),
);

// PTK 153 — Three Visits
pub(in crate::card::sets) static THREE_VISITS_153: CardRecord = CardRecord::new(
    "Three Visits",
    "306d22ae-657e-4b52-8cd9-6fc3df9e8376",
    "Qu Xin",
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell(
        "Search your library for a Forest card, put it onto the battlefield, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Forest")),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &RAVAGES_OF_WAR_17,
    &CAPTURE_OF_JINGZHOU_38,
    &CORRUPT_COURT_OFFICIAL,
    &IMPERIAL_SEAL,
    &IMPERIAL_RECRUITER_113,
    &WARRIOR_S_OATH_124,
    &THREE_VISITS_153,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
