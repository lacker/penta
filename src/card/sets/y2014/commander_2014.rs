//! Commander 2014 cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt, CardRules, CardSet,
    CardSupertype, CardType, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    PlayerRelation, ReplacementEffectDef, ReplacementEventDef, TriggerEventDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// C14 5 — Containment Priest
/// A nontoken creature that was not cast. Tokens are exempt because the card
/// says so; everything else that arrives without going through the stack --
/// reanimation, Show and Tell, a fetched Natural Order target -- is not.
static AN_UNCAST_CREATURE: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
]);

pub(in crate::card::sets) static CONTAINMENT_PRIEST: CardRecord = CardRecord::new_with_legacy_id(
    2156,
    "Containment Priest",
    CardArt::new("c2c794b9-09da-49be-b258-b0e21f1663e3", "John Stanko"),
    CardSet::Commander2014,
    // Flash is half the card: it is held up like a counterspell and answers
    // the reanimation on the stack rather than the creature on the board.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 2, 2).with_abilities(&[
        abilities::flash(),
        AbilityDef::replacement_for(
            "If a nontoken creature would enter and it wasn't cast, exile it instead.",
            ReplacementEventDef::ObjectEntersBattlefield {
                object: AN_UNCAST_CREATURE,
                controller: PlayerRelation::Any,
                cast: Some(false),
            },
            ReplacementEffectDef::MoveToZone(ZoneKind::Exile),
        ),
    ]),
);

// C14 50 — Titania, Protector of Argoth
static A_LAND_CARD_IN_YOUR_GRAVEYARD: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Land),
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: Some(PlayerRelation::You),
    },
)];

/// "A land you control", read as it leaves: the trigger is captured from
/// the battlefield as it was a moment before, which is the only place a
/// land that is now in a graveyard was ever controlled by anyone.
static A_LAND_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Land),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

static TITANIA_ABILITIES: [AbilityDef; 2] = [
    abilities::enters_trigger_with_targets(
        "When Titania enters, return target land card from your graveyard to the battlefield.",
        &A_LAND_CARD_IN_YOUR_GRAVEYARD,
        EffectDef::MoveToZone {
            counters: None,
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            from: None,
            zone: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            controller: None,
            arrival_effect: None,
            attachment: None,
            tapped: false,
        },
    ),
    AbilityDef::triggered(
        "Whenever a land you control is put into a graveyard from the battlefield, create a 5/3 \
         green Elemental creature token.",
        TriggerEventDef::zone_changed(
            A_LAND_YOU_CONTROL,
            Some(ZoneKind::Battlefield),
            Some(ZoneKind::Graveyard),
        ),
        EffectDef::create_creature_token(&["Elemental"], &[ManaColor::Green], 5, 3).with_art(
            CardArt::new("27440269-3b09-4010-8401-f159dc49a4cd", "Nils Hamm"),
        ),
    ),
];

pub(in crate::card::sets) static TITANIA_PROTECTOR_OF_ARGOTH: CardRecord =
    CardRecord::new_with_legacy_id(
        2296,
        "Titania, Protector of Argoth",
        CardArt::new("224d904a-5972-4152-878a-9a922e7a55b6", "Magali Villeneuve"),
        CardSet::Commander2014,
        // Five mana that gives a land back on the way in and then turns every
        // fetchland the deck was already playing into five power.
        CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Elemental"], 5, 3)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&TITANIA_ABILITIES),
    );

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&CONTAINMENT_PRIEST, &TITANIA_PROTECTOR_OF_ARGOTH];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
