//! Eldritch Moon cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef, CardArt, CardRules,
    CardSet, CardType, ChoiceVisibilityDef, ChooseDef, EffectDef, EffectRecipientDef,
    ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef, SpellAdditionalCostDef,
    ValueDef, ZoneKind,
};
use crate::ids::{ObjectBindingIndex, TargetIndex};
use crate::mana_cost;

/// Escalate: one discard for every mode past the first, so one mode is free
/// and all three cost two cards.
static ESCALATE_DISCARD: SpellAdditionalCostDef =
    SpellAdditionalCostDef::new(ObjectPredicateDef::Any, ZoneKind::Hand, 1)
        .counted_per_extra_mode();

static AN_OPPONENT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
)];

static A_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

static BRUTALITY_TAKE: EffectDef = EffectDef::DiscardCards {
    object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
};

static AN_INSTANT_OR_SORCERY: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Instant),
    ObjectPredicateDef::HasType(CardType::Sorcery),
]);

static BRUTALITY_STRIP: [EffectDef; 2] = [
    EffectDef::RevealHand {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::Choose(ChooseDef {
        binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
        unchosen: None,
        chooser: PlayerRefDef::EffectController,
        candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
            AN_INSTANT_OR_SORCERY,
            &[ZoneKind::Hand],
            PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
        )),
        exclude: None,
        minimum: 1,
        maximum: 1,
        visibility: ChoiceVisibilityDef::Public,
        then: &BRUTALITY_TAKE,
    }),
];

static BRUTALITY_DRAIN: [EffectDef; 2] = [
    EffectDef::LoseLife {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(2),
    },
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    },
];

/// Each mode declares its own target slot, so a Brutality that takes two
/// modes points at two things.
static BRUTALITY_MODES: [AbilityDef; 3] = [
    AbilityDef::spell_with_targets(
        "Target opponent reveals their hand. You choose an instant or sorcery card from it. That \
         player discards that card.",
        &AN_OPPONENT,
        EffectDef::Sequence(&BRUTALITY_STRIP),
    ),
    AbilityDef::spell_with_targets(
        "Target creature gets -2/-2 until end of turn.",
        &A_CREATURE,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-2),
                ValueDef::Constant(-2),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    ),
    AbilityDef::spell_with_targets(
        "Target opponent loses 2 life and you gain 2 life.",
        &AN_OPPONENT,
        EffectDef::Sequence(&BRUTALITY_DRAIN),
    ),
];

// EMN 85 — Collective Brutality
pub(in crate::card::sets) static COLLECTIVE_BRUTALITY: CardRecord = CardRecord::new_with_legacy_id(
    2244,
    "Collective Brutality",
    CardArt::new("cb94a02f-4660-45b6-8a39-941b710cf8f3", "Johann Bodin"),
    CardSet::EldritchMoon,
    // Two mana that answers three different decks, and the escalate cost is
    // paid in the cards those decks least want you to have anyway.
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_ability(
        AbilityDef::modal_spell(
            "Escalate—Discard a card. (Pay this cost for each mode chosen beyond the \
             first.)\nChoose one or more —\n• Target opponent reveals their hand. You choose an \
             instant or sorcery card from it. That player discards that card.\n• Target creature \
             gets -2/-2 until end of turn.\n• Target opponent loses 2 life and you gain 2 life.",
            &BRUTALITY_MODES,
            1,
            3,
            false,
        )
        .with_spell_additional_cost(&ESCALATE_DISCARD),
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&COLLECTIVE_BRUTALITY];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
