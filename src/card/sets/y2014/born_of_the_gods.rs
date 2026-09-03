//! Born of the Gods card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet,
    CardSupertype, CardType, EffectDef, EffectRecipientDef, ObjectPredicateDef, ObjectQueryDef,
    PlayActionMatcherDef, PlayRestrictionDef, PlayerRelation, PlayerSetDef, SumValueDef,
    TopOfLibraryCostDef, TriggerEventDef, ValueDef, ZoneKind, abilities,
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

static OTHER_LEGENDS_YOU_CONTROL: ValueDef = ValueDef::Sum(&SumValueDef::new(
    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Supertype(CardSupertype::Legendary),
        ]),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    )),
    ValueDef::Constant(-1),
));

// BNG 159 — Heroes' Podium
pub(in crate::card::sets) static HEROES_PODIUM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a3cb14f9-343c-4672-b4ee-db7f1d1a98ff"),
    "Heroes' Podium",
    CardArt::new("a3cb14f9-343c-4672-b4ee-db7f1d1a98ff", "Willian Murai"),
    CardSet::BornOfTheGods,
    CardRules::new_artifact(mana_cost!("{5}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Each legendary creature you control gets +1/+1 for each other legendary creature you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        OTHER_LEGENDS_YOU_CONTROL,
                        OTHER_LEGENDS_YOU_CONTROL,
                    ),
                },
            ),
            AbilityDef::activated(
                "{X}, {T}: Look at the top X cards of your library. You may reveal a legendary creature card from among them and put it into your hand. Put the rest on the bottom of your library in a random order.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{X}")),
                    AbilityCostDef::TapSource,
                ],
                abilities::look_at_top_cards_reveal_choice_to_hand_rest_random_bottom(
                    ValueDef::ChosenX,
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                    ]),
                    0,
                    1,
                ),
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&COURSER_OF_KRUPHIX, &HEROES_PODIUM];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
