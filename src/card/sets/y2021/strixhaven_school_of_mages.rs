//! Strixhaven: School of Mages cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AlternativeCastKindDef, CardArt,
    CardRules, CardSet, CardType, ChoiceVisibilityDef, ChooseDef, EffectDef, EffectRecipientDef,
    ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef, TriggerConditionDef, TriggerEventDef, ValueDef,
    ZoneKind, ZonePlacement, abilities,
};
use crate::ids::ObjectBindingIndex;
use crate::{TargetIndex, mana_cost};

/// Not linked to the Spellbinder: killing it does not give the card back,
/// and the tax outlives it. What the owner keeps is the card itself, one
/// turn later and two mana worse.
static SPELLBINDER_EXILE: EffectDef = EffectDef::ExileGrantingOwnerPlay {
    object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    surcharge: mana_cost!("{2}"),
};

/// "You may exile" -- a minimum of none, so a hand of nothing worth taking
/// is looked at and left alone.
static SPELLBINDER_TAKES_A_CARD: [EffectDef; 2] = [
    EffectDef::LookAtHand {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::Choose(ChooseDef {
        binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
        unchosen: None,
        chooser: PlayerRefDef::EffectController,
        candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            &[ZoneKind::Hand],
            PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
        )),
        exclude: None,
        minimum: 0,
        maximum: 1,
        // The card lands in exile face up, so which one was taken stops
        // being private the moment it is taken.
        visibility: ChoiceVisibilityDef::Public,
        then: &SPELLBINDER_EXILE,
    }),
];

static AN_OPPONENT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
)];

static ELITE_SPELLBINDER_ABILITIES: [AbilityDef; 2] = [
    abilities::flying(),
    AbilityDef::triggered_with_targets(
        "When this creature enters, look at target opponent's hand. You may exile a nonland card \
         from it. For as long as that card remains exiled, its owner may play it. A spell cast \
         this way costs {2} more to cast.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        &AN_OPPONENT,
        EffectDef::Sequence(&SPELLBINDER_TAKES_A_CARD),
    ),
];

// STX 17 — Elite Spellbinder
pub(in crate::card::sets) static ELITE_SPELLBINDER: CardRecord = CardRecord::new_with_legacy_id(
    2274,
    "Elite Spellbinder",
    CardArt::new("9d3a7998-ccac-45ad-a4e9-3a2cb057f63b", "Ryan Pancoast"),
    CardSet::StrixhavenSchoolOfMages,
    // A three-mana 3/1 flier that also buys a turn: the card comes back, but
    // a turn later and two mana worse, which is often the whole game.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Cleric"], 3, 1)
        .with_abilities(&ELITE_SPELLBINDER_ABILITIES),
);

static MASTERY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::HasType(CardType::Planeswalker),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

/// The discount is the whole cost of the card: two mana instead of four,
/// and the opponent gets the card back. Which cast was used is read off the
/// spell itself, so the rider is part of one resolution rather than a
/// second clause.
static MASTERY_WAS_DISCOUNTED: TriggerConditionDef =
    TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::AlternativeCost);

static MASTERY_OPPONENT_DRAWS: EffectDef = EffectDef::DrawCards {
    recipient: EffectRecipientDef::Opponent,
    amount: ValueDef::Constant(1),
};

static MASTERY_EXILE: EffectDef = EffectDef::MoveToZone {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    zone: ZoneKind::Exile,
    placement: ZonePlacement::Top,
    controller: None,
    arrival_effect: None,
    attachment: None,
};

/// Printed order: the draw is named before the exile, and it happens first.
static MASTERY_RESOLUTION: [EffectDef; 2] = [
    EffectDef::IfCondition {
        condition: &MASTERY_WAS_DISCOUNTED,
        then: &MASTERY_OPPONENT_DRAWS,
    },
    MASTERY_EXILE,
];

// STX 64 — Baleful Mastery
pub(in crate::card::sets) static BALEFUL_MASTERY: CardRecord = CardRecord::new_with_legacy_id(
    2201,
    "Baleful Mastery",
    CardArt::new("35f1a6ba-e46f-44fb-93f4-fb883d677b36", "Chris Cold"),
    CardSet::StrixhavenSchoolOfMages,
    // Exile at instant speed answers anything, and the choice of price is
    // the card: four mana clean, or two and a card for them.
    CardRules::new_instant(mana_cost!("{3}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "If the {1}{B} cost was paid, an opponent draws a card.\nExile target creature or planeswalker.",
            &MASTERY_TARGET,
            EffectDef::Sequence(&MASTERY_RESOLUTION),
        ),
        AbilityDef::alternative_cast(
            mana_cost!("{1}{B}"),
            AlternativeCastKindDef::AlternativeCost,
            Some("You may pay {1}{B} rather than pay this spell's mana cost."),
            EffectDef::None,
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&ELITE_SPELLBINDER, &BALEFUL_MASTERY];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
