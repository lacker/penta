//! Born of the Gods card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, AppliedRuleDef, CardRules, CardType, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, PlayActionMatcherDef, PlayRestrictionDef,
    PlayerRelation, PlayerSetDef, TopOfLibraryCostDef, TriggerEventDef, ValueDef, ZoneKind,
};
use crate::mana_cost;

// BNG 119 — Courser of Kruphix
pub(in crate::card::sets) static COURSER_OF_KRUPHIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("da5a807f-58e8-4d92-a61c-47bb9b28977f"),
    "Courser of Kruphix",
    crate::card::CardArt::new("da5a807f-58e8-4d92-a61c-47bb9b28977f", "Eric Deschamps"),
    crate::card::CardSet::BornOfTheGods,
    // Two toughness past what red reaches, which is most of why the body is
    // worth three mana at all.
    CardRules::new_enchantment_creature(mana_cost!("{1}{G}{G}"), &["Centaur"], 2, 4)
        .with_abilities(&[
            // The reveal is what makes the permission worth having: a top card you
            // cannot see is a land drop you cannot plan. It is public rather than
            // private, so the other player plans around it too.
            AbilityDef::static_ability(
                "Play with the top card of your library revealed.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(
                        PlayerRelation::You,
                    )),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::PlaysWithTopOfLibraryRevealed),
                },
            ),
            AbilityDef::static_ability(
                "You may play lands from the top of your library.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(
                        PlayerRelation::You,
                    )),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromTopOfLibrary {
                        // Lands only, and at their printed cost -- which for a land is no cost at
                        // all beyond the land drop it still has to spend.
                        restriction: PlayRestrictionDef::new(
                            PlayActionMatcherDef::PlayLand,
                            ObjectPredicateDef::HasType(CardType::Land),
                        ),
                        cost: TopOfLibraryCostDef::Printed,
                    }),
                },
            ),
            AbilityDef::triggered(
                "Landfall — Whenever a land you control enters, you gain 1 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&COURSER_OF_KRUPHIX];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
