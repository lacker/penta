//! Dominaria United Commander card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet, CardSupertype,
    CardType, CopyExceptionsDef, CreatedTokensDef, EffectDef, EffectPaymentDef, EffectRecipientDef,
    InstalledTriggerDef, ManaColor, ObjectPredicateDef, ObjectSetDef, PlayerRelation,
    PlayerRuleDef, PlayerSetDef, ResolvedEffectDurationDef, TriggerEventDef, TurnStepDef, ValueDef,
    ZoneKind, abilities,
};
use crate::ids::ParentBinding;
use crate::mana_cost;

// DMC 10 — Cadric, Soul Kindler
pub(in crate::card::sets) static CADRIC_SOUL_KINDLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f82f8cab-5039-4e3a-a2ba-cbf829db80ed"),
    "Cadric, Soul Kindler",
    CardArt::new(
        "f82f8cab-5039-4e3a-a2ba-cbf829db80ed",
        "Joseph Weston",
    ),
    CardSet::DominariaUnitedCommander,
    CardRules::new_creature(mana_cost!("{2}{R}{W}"), &["Dwarf", "Wizard"], 4, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "The \"legend rule\" doesn't apply to tokens you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(
                        PlayerRelation::You,
                    )),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                        PlayerRuleDef::LegendRuleDoesNotApplyTo(&ObjectPredicateDef::Token),
                    )),
                },
            ),
            AbilityDef::triggered(
                "Whenever another nontoken legendary permanent you control enters, you may pay {1}. If you do, create a token that's a copy of it. That token gains haste. Sacrifice it at the beginning of the next end step.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::PayOr(crate::card::PayOrDef::optional(
                    EffectPaymentDef::mana(
                        PlayerSetDef::Related(PlayerRelation::You),
                        mana_cost!("{1}"),
                    ),
                    &EffectDef::create_token_from_copy(&crate::card::TokenCopyDef {
                        object: &EffectRecipientDef::TriggeringObject,
                        exceptions: CopyExceptionsDef::NONE,
                    })
                    .with_created_tokens(CreatedTokensDef {
                        binding: ParentBinding,
                        then: &EffectDef::Sequence(&[
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    ParentBinding,
                                )),
                                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                                duration: ResolvedEffectDurationDef::Permanent,
                            },
                            EffectDef::InstallTrigger(InstalledTriggerDef::once(
                                &AbilityDef::triggered(
                                    "Sacrifice it at the beginning of the next end step.",
                                    TriggerEventDef::StepBegins {
                                        step: TurnStepDef::End,
                                        player: PlayerRelation::Any,
                                    },
                                    EffectDef::Sacrifice {
                                        object: EffectRecipientDef::objects(
                                            ObjectSetDef::Binding(ParentBinding),
                                        ),
                                    },
                                ),
                            )),
                        ]),
                    }),
                )),
            ),
        ]),
);

// DMC 47 — Torsten, Founder of Benalia
pub(in crate::card::sets) static TORSTEN_FOUNDER_OF_BENALIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0783b426-a527-42c1-9271-be28b229e1c6"),
    "Torsten, Founder of Benalia",
    CardArt::new("0783b426-a527-42c1-9271-be28b229e1c6", "Volkan Baǵa"),
    CardSet::DominariaUnitedCommander,
    // Seven mana, and the two halves answer the two ways it goes wrong: it
    // refills your hand the turn it lands, and leaves seven bodies behind if
    // somebody kills it.
    CardRules::new_creature(mana_cost!("{5}{G}{W}"), &["Human", "Soldier"], 7, 7)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Torsten enters, reveal the top seven cards of your library. Put any number of \
                 creature and/or land cards from among them into your hand and the rest on the bottom of \
                 your library in a random order.",
                // "Any number", so the choice is real: a land you would rather not draw
                // later can be left to the bottom, which is the only reason the clause is
                // bounded rather than mandatory. All seven are revealed, and what remains
                // is randomized rather than ordered as a plan for later.
                abilities::reveal_top_cards_choose_to_hand_rest_random_bottom(
                    ValueDef::Constant(7),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Land),
                    ]),
                    0,
                    7,
                ),
            ),
            abilities::dies_trigger(
                "When Torsten dies, create seven 1/1 white Soldier creature tokens.",
                EffectDef::create_creature_token(&["Soldier"], &[ManaColor::White], 1, 1)
                    .with_count(ValueDef::Constant(7))
                    .with_art(CardArt::new(
                        "8c4b0257-2ca5-4015-9d63-d7cf6e87ab9d",
                        "Justine Cruz",
                    )),
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&CADRIC_SOUL_KINDLER, &TORSTEN_FOUNDER_OF_BENALIA];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
