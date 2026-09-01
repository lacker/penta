//! Ravnica: City of Guilds cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::abilities;
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AggregateOperationDef, CardArt,
    CardRules, CardSet, EffectDef, EffectRecipientDef, MoveObjectsDef, ObjectPredicateDef,
    ObjectSetDef, ObjectValueAggregateDef, ObjectValueDef, PlayerRefDef, PlayerRelation,
    RevealObjectsDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement,
};
use crate::{ParentBinding, TargetIndex, mana_cost};

// RAV 81 — Dark Confidant
/// One card off the top, shown to everybody, into your hand. Nothing is
/// chosen and nothing may be declined: the minimum and the maximum are both
/// the one card the trigger names.
pub(in crate::card::sets) static DARK_CONFIDANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("94f7a441-bf2d-46fb-a7b6-9bd6137f86d9"),
    "Dark Confidant",
    CardArt::new("94f7a441-bf2d-46fb-a7b6-9bd6137f86d9", "Ron Spears"),
    CardSet::RavnicaCityOfGuilds,
    // Two mana for an extra card every turn, at whatever the top of your
    // deck happens to cost -- which is why the decks that play him keep
    // their curve low enough to survive him.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Wizard"], 2, 1).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, reveal the top card of your library and put that \
             card into your hand. You lose life equal to its mana value.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            abilities::bind_top_cards_then(
                PlayerRefDef::EffectController,
                ValueDef::Constant(1),
                &const {
                    EffectDef::Sequence(&[
                        EffectDef::RevealObjects(RevealObjectsDef {
                            input: ObjectSetDef::Binding(ParentBinding),
                            then: &EffectDef::None,
                        }),
                        EffectDef::MoveObjects(MoveObjectsDef {
                            input: ObjectSetDef::Binding(ParentBinding),
                            from: Some(ZoneKind::Library),
                            zone: ZoneKind::Hand,
                            placement: ZonePlacement::Top,
                            moved: Some(ParentBinding),
                            // "You lose life equal to its mana value." The card is in your hand by the
                            // time this is asked, so what the reveal hands on is the number rather than
                            // the card.
                            then: &EffectDef::LoseLife {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                                    objects: ObjectSetDef::Binding(ParentBinding),
                                    select: ObjectValueDef::ManaValue,
                                    operation: AggregateOperationDef::Maximum,
                                }),
                            },
                        }),
                    ])
                },
            ),
        ),
    ),
);

// RAV 139 — Reroute
pub(in crate::card::sets) static REROUTE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("42794e10-ddcd-4d2d-ab0c-a6b99b6d4662"),
    "Reroute",
    CardArt::new(
        "42794e10-ddcd-4d2d-ab0c-a6b99b6d4662",
        "Christopher Rush",
    ),
    CardSet::RavnicaCityOfGuilds,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Change the target of target activated ability with a single target. (Mana abilities can't be targeted.)\nDraw a card.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::ActivatedAbility,
                        ObjectPredicateDef::DeclaredTargetCount {
                            minimum: 1,
                            maximum: 1,
                        },
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::ChangeStackTargets(&crate::card::ChangeStackTargetsDef {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    chooser: PlayerRefDef::EffectController,
                    change: crate::card::StackTargetChangeDef::ChooseNew {
                        optional: false,
                        restriction: None,
                    },
                }),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&DARK_CONFIDANT, &REROUTE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
