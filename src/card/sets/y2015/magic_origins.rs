//! ORI card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef,
    CardArt, CardRules, CardSet, CardSupertype, CardType, ComparisonDef, DiscardSelectionDef,
    EffectDef, EffectRecipientDef, ObjectPredicateDef, ObjectQueryDef, PlayerRelation,
    ResolvedEffectDurationDef, TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

// ORI 60 — Jace, Vryn's Prodigy // Jace, Telepath Unbound
pub(in crate::card::sets) static JACE_VRYN_S_PRODIGY: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("e7b5705f-dc56-41af-a781-8a41aaa7c5b8"),
    "Jace, Vryn's Prodigy // Jace, Telepath Unbound",
    CardArt::new("02d6d693-f1f3-4317-bcc0-c21fa8490d38", "Jaime Jones"),
    CardSet::MagicOrigins,
    &[
        (
            "Jace, Vryn's Prodigy",
            const {
                CardRules::new_creature(mana_cost!("{1}{U}"), &const { ["Human", "Wizard"] }, 0, 2)
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&const { [AbilityDef::activated(
                    "{T}: Draw a card, then discard a card. If there are five or more cards in your graveyard, \
                     exile Jace, then return him to the battlefield transformed under his owner's control.",
                    &const { [AbilityCostDef::TapSource] },
                    EffectDef::Sequence(&const { [
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
                        EffectDef::IfCondition {
                            // Counted after the loot, so the card just discarded is one of the five --
                            // which is what makes the turn he arrives and the turn he flips so often
                            // the same turn.
                            condition: &TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::Any,
                                    &const { [ZoneKind::Graveyard] },
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 5,
                            },
                            // The same exile-and-return every flip creature uses: one resolution, so he
                            // is gone and back before anything else happens, and what comes back is a
                            // new object with the loyalty the back face prints.
                            then: &EffectDef::Sequence(&const { [
                                EffectDef::ExileLinkedToSource {
                                    until_source_leaves: false,
                                    object: EffectRecipientDef::Source,
                                    face_down: false,
                                    then: None,
                                },
                                EffectDef::ReturnLinkedExiles {
                                    object: ObjectPredicateDef::Any,
                                    counters: None,
                                    zone: ZoneKind::Battlefield,
                                    grant: None,
                                    controller: None,
                                    transformed: true,
                                },
                            ] }),
                        },
                    ] }),
                )] })
            },
        ),
        (
            "Jace, Telepath Unbound",
            const {
                CardRules::new_planeswalker_without_mana_cost(&const { ["Jace"] })
                .with_supertype(CardSupertype::Legendary)
                .with_starting_loyalty(5)
                .printed_colors(&const { [crate::card::ManaColor::Blue] })
                .with_abilities(&const { [
                    AbilityDef::activated_with_targets(
                        "+1: Up to one target creature gets -2/-0 until your next turn.",
                        &const { [AbilityCostDef::Loyalty(1)] },
                        // "Up to one", so a Jace with nothing worth shrinking still ticks up.
                        &const { [AbilityTargetDef::up_to(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::HasType(CardType::Creature),
                                zones: &const { [ZoneKind::Battlefield] },
                                controller: None,
                                owner: None,
                            },
                            1,
                        )] },
                        // "Until your next turn" rather than until end of turn: the creature is
                        // smaller on their swing back as well, which is what makes the plus a
                        // defensive ability rather than a combat trick.
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(-2), ValueDef::Constant(0)),
                            duration: ResolvedEffectDurationDef::UntilYourNextTurn,
                        },
                    ),
                    // Written as the flashback his clause comes to: the cost is the card's
                    // own, the window is this turn, and the card is exiled rather than left
                    // in the graveyard. What differs from the printed wording is that the
                    // card is lent the keyword, so anything reading "has flashback" would
                    // see it.
                    AbilityDef::activated_with_targets(
                        "\u{2212}3: You may cast target instant or sorcery card from your graveyard this turn. \
                         If that spell would be put into your graveyard, exile it instead.",
                        &const { [AbilityCostDef::Loyalty(-3)] },
                        &const { [AbilityTargetDef::exactly_one(
                                AbilityTargetPredicate::Object {
                                    object: ObjectPredicateDef::AnyOf(&const { [
                                        ObjectPredicateDef::HasType(CardType::Instant),
                                        ObjectPredicateDef::HasType(CardType::Sorcery),
                                    ] }),
                                    zones: &const { [ZoneKind::Graveyard] },
                                    controller: None,
                                    owner: Some(PlayerRelation::You),
                                },
                            )] },
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::add_ability(&const {
                                abilities::flashback_for_card_mana_cost()
                            }),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ),
                    AbilityDef::activated(
                        "\u{2212}9: You get an emblem with \"Whenever you cast a spell, target opponent mills \
                         five cards.\"",
                        &const { [AbilityCostDef::Loyalty(-9)] },
                        EffectDef::create_emblem("Jace, Telepath Unbound emblem", &const { [AbilityDef::triggered_with_targets(
                            "Whenever you cast a spell, target opponent mills five cards.",
                            TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)),
                            &const { [AbilityTargetDef::exactly_one(
                                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                            )] },
                            EffectDef::Mill {
                                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                amount: ValueDef::Constant(5),
                            },
                        )] }),
                    ),
                ] })
            },
        ),
    ],
);

// ORI 62 — Jhessian Thief
pub(in crate::card::sets) static JHESSIAN_THIEF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("33b8553d-d326-4280-bc3a-2fffdd377cd2"),
    "Jhessian Thief",
    CardArt::new("33b8553d-d326-4280-bc3a-2fffdd377cd2", "Miles Johnston"),
    CardSet::MagicOrigins,
    // A 1/3 that gets through on its own rarely, so prowess is what turns a
    // spell-heavy turn into both a bigger body and a card.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Rogue"], 1, 3).with_abilities(&[
        abilities::prowess(),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, draw a card.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ORI 171 — Conclave Naturalists
pub(in crate::card::sets) static CONCLAVE_NATURALISTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3759fc28-9adb-41ed-851c-566a3a424e09"),
    "Conclave Naturalists",
    CardArt::new("3759fc28-9adb-41ed-851c-566a3a424e09", "Howard Lyon"),
    CardSet::MagicOrigins,
    // A 4/4 body that carries its own answer, so the trigger is optional
    // rather than a liability when the opponent has nothing worth breaking.
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Dryad"], 4, 4).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, you may destroy target artifact or enchantment.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
            },
        ),
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&JACE_VRYN_S_PRODIGY, &JHESSIAN_THIEF, &CONCLAVE_NATURALISTS];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
