//! Tarkir: Dragonstorm cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet, CreatedTokensDef,
    EffectDef, EffectRecipientDef, InstalledTriggerDef, ManaColor, ObjectPredicateDef,
    ObjectSetDef, PlayActionMatcherDef, PlayRestrictionDef, PlayerRelation, PlayerSetDef,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, cards,
};
use crate::ids::ObjectSetBindingIndex;
use crate::mana_cost;

/// The tokens go away at the next end step, and it has to be exactly the
/// ones this attack made: by then nothing about the board could tell them
/// apart from the pair the last attack made, or from a Warrior that arrived
/// some other way. So they are bound as they are created and the delayed
/// clause names the binding.
static MOBILIZE_SACRIFICE: EffectDef =
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
        "At the beginning of the next end step, sacrifice those tokens.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::Any,
        },
        EffectDef::Sacrifice {
            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                ObjectSetBindingIndex::PRIMARY,
            )),
        },
    )));

/// Mobilize 2 (CR 702.180a). Written out rather than abbreviated: the
/// keyword is a shorthand for a triggered ability, and this is that ability.
static MOBILIZE_TWO: AbilityDef = AbilityDef::triggered(
    "Mobilize 2 (Whenever this creature attacks, create two tapped and attacking 1/1 red Warrior \
     creature tokens. Sacrifice them at the beginning of the next end step.)",
    TriggerEventDef::attacks(ObjectPredicateDef::Source),
    EffectDef::create_creature_token(&["Warrior"], &[ManaColor::Red], 1, 1)
        .with_art(CardArt::new(
            "7edc0515-a130-45a7-aa09-0e23bba41587",
            "Forrest Imel",
        ))
        .with_amount(2)
        .entering_tapped()
        .entering_attacking()
        .with_created_tokens(CreatedTokensDef {
            binding: ObjectSetBindingIndex::PRIMARY,
            then: &MOBILIZE_SACRIFICE,
        }),
);

static NO_SPELLS: PlayRestrictionDef =
    PlayRestrictionDef::new(PlayActionMatcherDef::CastSpell, ObjectPredicateDef::Any);

/// "During your turn" is the whole of the clause's timing, and it gates the
/// restriction rather than narrowing who it names: on their own turn the
/// same opponents may cast whatever they like.
static SILENCE_ON_YOUR_TURN: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::Opponent)),
    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(NO_SPELLS)),
};

static VOICE_OF_VICTORY_ABILITIES: [AbilityDef; 2] = [
    MOBILIZE_TWO,
    AbilityDef::static_ability(
        "Your opponents can't cast spells during your turn.",
        EffectDef::IfCondition {
            condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
            then: &SILENCE_ON_YOUR_TURN,
        },
    ),
];

// TDM 33 — Voice of Victory
pub(in crate::card::sets) static VOICE_OF_VICTORY: CardRecord = CardRecord::new(
    cards::VOICE_OF_VICTORY,
    "Voice of Victory",
    CardArt::new("ec3de5f4-bb55-4ab9-995f-f3e0dc22c1bb", "Joshua Cairos"),
    CardSet::TarkirDragonstorm,
    // Two mana that adds two power to every attack and turns off every
    // instant your opponent was holding for the turn you attack.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Bard"], 1, 3)
        .with_abilities(&VOICE_OF_VICTORY_ABILITIES),
);

// TDM 127 — Tersa Lightshatter
// Audit: blocked — Two of her three abilities need capabilities that are already blocking other cards. "Discard up to two cards, then draw that many" needs a discard whose size the player chooses, where a discard here takes a fixed number; the same gap blocks Mind Bomb in The Dark. And "you may play that card this turn" needs a permission to play one exiled card for a duration, which nothing here can grant and which also blocks Robber of the Rich. Haste alone is not the card.

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&VOICE_OF_VICTORY];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
