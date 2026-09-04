//! Commander 2017 card records required by supported formats.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AppliedEffectDef, AppliedRuleDef, CardRules, CardSet, CardType,
    CopyExceptionsDef, EffectDef, EffectRecipientDef, ObjectPredicateDef, ObjectRefDef,
    PlayerRefDef, PlayerRelation, PlayerRuleDef, ResolvedEffectDurationDef,
    SpellResolutionDestinationDef, ZoneKind, ZonePlacement,
};
use crate::{TargetIndex, mana_cost};

// C17 8 — Teferi's Protection
pub(in crate::card::sets) static TEFERIS_PROTECTION: CardRecord = CardRecord::new(
    CardSet::Commander2017,
    "Teferi's Protection",
    "77f130c7-0138-4a1a-9f67-62d2c302dc48",
    "Chase Stone",
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(
        AbilityDef::spell(
            "Until your next turn, your life total can't change and you gain protection from everything. All permanents you control phase out. (While they're phased out, they're treated as though they don't exist. They phase in before you untap during your untap step.)\nExile Teferi's Protection.",
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                            PlayerRuleDef::LifeTotalCannotChange,
                        )),
                        AppliedEffectDef::Rule(AppliedRuleDef::PlayerProtectionFrom(
                            ObjectPredicateDef::Any,
                        )),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilYourNextTurn,
                },
                EffectDef::PhaseOut {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                },
            ]),
        )
        .with_resolution_destination(SpellResolutionDestinationDef::Exile),
    ),
);

// C17 37 — Fractured Identity
pub(in crate::card::sets) static FRACTURED_IDENTITY: CardRecord = CardRecord::new(
    CardSet::Commander2017,
    "Fractured Identity",
    "b2f73f5d-1aad-48c2-9e74-5f7bdd87900f",
    "Yongjae Choi",
    // Five mana that answers anything and keeps it: what leaves their board
    // arrives on yours, which is why the card is played over the cheaper
    // exile effects beside it.
    CardRules::new_sorcery(mana_cost!("{3}{W}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target nonland permanent. Each player other than its controller creates a token \
         that's a copy of it.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        )],
        // The copy is made before the exile rather than after it, which is the one
        // place this differs from the printed order. A target that has already left
        // the battlefield is no longer a legal target, so the copy has to be taken
        // while the permanent is still there; what it copies -- the permanent's
        // copiable values -- is the same either way, and no player receives
        // priority in between.
        //
        // "Each player other than its controller" is one player here, and it is
        // read off the target rather than off the spell: a Fractured Identity
        // pointed at your own permanent hands the copy to your opponent.
        EffectDef::Sequence(&[
            EffectDef::create_token_from_copy(&crate::card::TokenCopyDef {
                object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                exceptions: CopyExceptionsDef::NONE,
            })
            .with_controller(PlayerRefDef::OpponentOf(ObjectRefDef::Target(
                TargetIndex::PRIMARY,
            ))),
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ]),
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&TEFERIS_PROTECTION, &FRACTURED_IDENTITY];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
