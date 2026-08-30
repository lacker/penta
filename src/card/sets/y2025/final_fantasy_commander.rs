//! Final Fantasy Commander card records.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, CardSupertype, CounterKind, DiscardSelectionDef,
    EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef, PlayerRelation,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, abilities,
};
use crate::mana_cost;

// FIC 52 — Transpose
pub(in crate::card::sets) static TRANSPOSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("66392b0e-8691-42a4-bc84-03b017174a73"),
    "Transpose",
    CardArt::new("66392b0e-8691-42a4-bc84-03b017174a73", "Toni Infante"),
    CardSet::FinalFantasyCommander,
    CardRules::new_instant(mana_cost!("{2}{B}")).with_abilities(&[
        AbilityDef::spell(
            "Draw a card, then discard a card. You lose 1 life. If this spell was cast from your hand, create a 0/1 black Wizard creature token with \"Whenever you cast a noncreature spell, this token deals 1 damage to each opponent.\"",
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceCastFrom(ZoneKind::Hand),
                    then: &EffectDef::create_creature_token(
                        &["Wizard"],
                        &[ManaColor::Black],
                        0,
                        1,
                    )
                    .with_abilities(&[AbilityDef::triggered(
                        "Whenever you cast a noncreature spell, this token deals 1 damage to each opponent.",
                        TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                            ObjectPredicateDef::NoncreatureSpell,
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ])),
                        EffectDef::DealDamage {
                            recipient: EffectRecipientDef::Opponent,
                            amount: ValueDef::Constant(1),
                        },
                    )]),
                },
            ]),
        ),
        abilities::rebound(),
    ]),
);

// FIC 55 — Gau, Feral Youth
pub(in crate::card::sets) static GAU_FERAL_YOUTH: CardRecord = CardRecord::new_with_legacy_id(
    2304,
    "Gau, Feral Youth",
    CardArt::new("89175ce1-0746-4ba1-970e-617d134b0527", "Eglė Mosakaitė"),
    CardSet::FinalFantasyCommander,
    // Two mana that grows every attack and, in a deck that is already using
    // its graveyard, throws that growth at the opponent every end step.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Berserker"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            // "Rage" is an ability word: flavour on the front of an ordinary attack
            // trigger, and nothing the rules read.
            AbilityDef::triggered(
                "Rage — Whenever Gau attacks, put a +1/+1 counter on it.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            // Each end step, not just yours: a graveyard emptied on their turn pays
            // out on their turn too.
            AbilityDef::triggered_if(
                "At the beginning of each end step, if a card left your graveyard this turn, Gau deals \
                 damage equal to its power to each opponent.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::Any,
                },
                // An intervening-if, so it is checked twice: once when the end step begins
                // and again as the ability resolves. A graveyard that gave a card up and
                // then got it back is still a graveyard a card left.
                &TriggerConditionDef::ControllerHadCardLeaveGraveyardThisTurn,
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::SourcePower,
                },
            ),
        ]),
);

// FIC 119 — Transpose (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&TRANSPOSE, &GAU_FERAL_YOUTH];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&TRANSPOSE, 1), // FIC 119
];
