//! Duskmourn: House of Horror Commander cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, CardArt, CardRules, CardSet, EffectDef, EffectRecipientDef,
    PlayerRelation, ResolvedEffectDurationDef, TriggerEventDef, TurnStepDef, ValueDef, abilities,
};
use crate::mana_cost;

static MONSTROSITY_INDESTRUCTIBLE: AbilityDef = abilities::indestructible();

/// "This creature attacks that player this combat if able." In a two-player
/// game the chosen opponent is the only player there is to attack, so the
/// requirement is the ordinary one; it is granted for the turn rather than
/// printed, and the trigger renews it at the beginning of every combat.
static MONSTROSITY_MUST_ATTACK: AbilityDef = abilities::attacks_each_combat_if_able(
    "This creature attacks that player this combat if able.",
);

static MONSTROSITY_GROWS: [AppliedEffectDef; 3] = [
    AppliedEffectDef::add_ability(&MONSTROSITY_INDESTRUCTIBLE),
    AppliedEffectDef::add_ability(&MONSTROSITY_MUST_ATTACK),
    // Read as the trigger resolves, which is after the mill: the card it
    // just put there counts toward its own bonus.
    AppliedEffectDef::modify_power_toughness(
        ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
        ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
    ),
];

static MONSTROSITY_COMBAT: [EffectDef; 2] = [
    EffectDef::Mill {
        player: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
        binding: None,
        then: None,
    },
    EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::Composite(&MONSTROSITY_GROWS),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
];

// DSC 36 — Ursine Monstrosity
pub(in crate::card::sets) static URSINE_MONSTROSITY: CardRecord = CardRecord::new_with_legacy_id(
    2195,
    "Ursine Monstrosity",
    CardArt::new("73cc6df4-3564-4ace-bf8a-eac3e62d725a", "Carlos Palma Cruchaga"),
    CardSet::DuskmournHouseOfHorrorCommander,
    // The bear feeds itself: every combat mills one more card, and every
    // card type that turns up is another point in both directions.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Bear", "Mutant"], 3, 3).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "At the beginning of combat on your turn, mill a card and choose an opponent at random. This creature attacks that player this combat if able. Until end of turn, this creature gains indestructible and gets +1/+1 for each card type among cards in your graveyard.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            EffectDef::Sequence(&MONSTROSITY_COMBAT),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&URSINE_MONSTROSITY];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
