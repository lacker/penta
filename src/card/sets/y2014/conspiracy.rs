//! Conspiracy cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardRules, CardSet,
    CardSupertype, CardType, ControlDurationDef, DiscardSelectionDef, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, ObjectRefDef, ObjectSetDef, PlayerRefDef,
    PlayerRelation, TriggerEventDef, ValueDef,
};
use crate::{TargetIndex, mana_cost};

// CNS 16 — Council's Judgment
pub(in crate::card::sets) static COUNCILS_JUDGMENT: CardRecord = CardRecord::new(
    CardSet::Conspiracy,
    "Council's Judgment",
    "17f28b16-da65-41a8-ba4f-f1c5e104aad6",
    "Kev Walker",
    // Exiling without targeting is what it is played for: shroud, hexproof,
    // and protection are all no answer at all. Two players usually means two
    // permanents, since a disagreement ties.
    CardRules::new_sorcery(mana_cost!("{1}{W}{W}")).with_ability(AbilityDef::spell(
        "Will of the council — Starting with you, each player votes for a nonland permanent you don't control. Exile each permanent with the most votes or tied for most votes.",
        EffectDef::VoteForPermanentToExile {
            // "A nonland permanent you don't control" is read against the spell's
            // controller for every voter, so both players choose from the same ballot.
            // The vote machinery supplies the "you don't control" half.
            object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        },
    )),
);

// CNS 18 — Custodi Squire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CUSTODI_SQUIRE: CardRecord = CardRecord::new(
    crate::card::CardSet::Conspiracy,
    "Custodi Squire",
    "a9151422-8df1-409c-a686-0cd89247eb43",
    "Alex Horley-Orlandelli",
    crate::card::CardRules::unsupported(),
);

// CNS 42 — Dack Fayden
pub(in crate::card::sets) static DACK_FAYDEN: CardRecord = CardRecord::new(
    CardSet::Conspiracy,
    "Dack Fayden",
    "3fcb7810-1054-4001-855c-6e17939b3d3f",
    "Eric Deschamps",
    // The greatest thief in the multiverse, and in a cube full of Moxen the
    // minus is what he is actually here for.
    CardRules::new_planeswalker(mana_cost!("{1}{U}{R}"), &["Dack"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "+1: Target player draws two then discards two cards.",
                &[AbilityCostDef::Loyalty(1)],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                // Two for two is a wash against most decks and a windmill against a graveyard
                // one, which is the whole reason to point it at yourself.
                EffectDef::Sequence(&[
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(2),
                    },
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(2),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                ]),
            ),
            // Nothing is holding the theft and no cleanup ends it: a control change
            // with no stated duration lasts indefinitely (CR 611.2b).
            AbilityDef::activated_with_targets(
                "−2: Gain control of target artifact.",
                &[AbilityCostDef::Loyalty(-2)],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                )],
                EffectDef::GainControl {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    controller: PlayerRefDef::EffectController,
                    duration: ControlDurationDef::Indefinitely,
                },
            ),
            AbilityDef::activated(
                "−6: You get an emblem with \"Whenever you cast a spell that targets one or more \
                 permanents, gain control of those permanents.\"",
                &[AbilityCostDef::Loyalty(-6)],
                EffectDef::create_emblem("Dack Fayden emblem", &[AbilityDef::triggered(
                    "Whenever you cast a spell that targets one or more permanents, gain control of those \
                         permanents.",
                    TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ObjectPredicateDef::TargetsObjectMatching(&ObjectPredicateDef::Any),
                    ])),
                    EffectDef::GainControl {
                        object: EffectRecipientDef::objects(ObjectSetDef::PermanentsTargetedBy(
                            ObjectRefDef::TriggeringObject,
                        )),
                        controller: PlayerRefDef::EffectController,
                        duration: ControlDurationDef::Indefinitely,
                    },
                )]),
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&COUNCILS_JUDGMENT, &CUSTODI_SQUIRE, &DACK_FAYDEN];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
