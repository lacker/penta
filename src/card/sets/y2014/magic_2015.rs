//! Magic 2015 cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AppliedEffectDef, AppliedRuleDef, BlockRestrictionDef, CardArt,
    CardRules, CardSet, CardType, EffectDef, EffectPaymentCostDef, EffectPaymentDef,
    EffectRecipientDef, ManaColor, ObjectPredicateDef, ObjectQueryDef, ObjectSetDef, PayOrDef,
    PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef, TriggerEventDef, TurnStepDef,
    ValueDef, ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

// M15 40 — Triplicate Spirits
pub(in crate::card::sets) static TRIPLICATE_SPIRITS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3d6498d3-bf1f-4bf1-a602-7c21fb44c106"),
    "Triplicate Spirits",
    CardArt::new("3d6498d3-bf1f-4bf1-a602-7c21fb44c106", "Izzy"),
    CardSet::Magic2015,
    // Six mana printed, but the tokens it already made are what pay for the
    // next copy, so the real cost falls every time a token deck casts it.
    CardRules::new_sorcery(mana_cost!("{4}{W}{W}")).with_abilities(&[
        abilities::convoke(),
        AbilityDef::spell(
            "Create three 1/1 white Spirit creature tokens with flying.",
            EffectDef::create_creature_token(&["Spirit"], &[ManaColor::White], 1, 1)
                .with_abilities(&[abilities::flying()])
                .with_amount(3),
        ),
    ]),
);

// M15 142 — Frenzied Goblin
pub(in crate::card::sets) static FRENZIED_GOBLIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d307d8c7-b9b5-4f8f-933d-f1c64cbbf92f"),
    "Frenzied Goblin",
    CardArt::new("7ddfe382-3a80-45f3-a022-54739c4b69a6", "Carl Critchlow"),
    CardSet::Magic2015,
    // One mana an attack to push whichever blocker matters, which is what
    // keeps a one-drop relevant into the late game.
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, you may pay {R}. If you do, target creature can't block this turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            // The target is chosen as the trigger goes on the stack and the
            // payment is offered as it resolves, so the creature is named
            // before its controller knows whether the mana is there.
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef {
                    payer: PlayerSetDef::Related(PlayerRelation::You),
                    cost: EffectPaymentCostDef::ColoredMana {
                        color: ManaColor::Red,
                        amount: ValueDef::Constant(1),
                    },
                },
                &const {
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                            BlockRestrictionDef::CANNOT_BLOCK,
                        )),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    }
                },
            )),
        ),
    ),
);

// M15 145 — Goblin Rabblemaster
pub(in crate::card::sets) static GOBLIN_RABBLEMASTER: CardRecord = CardRecord::new_with_legacy_id(
    2263,
    "Goblin Rabblemaster",
    CardArt::new("ee9c697e-d2c0-413b-9142-ecf5d7cf5322", "Svetlin Velinov"),
    CardSet::Magic2015,
    // Three mana that makes a Goblin every turn and then sends the whole
    // pile in whether or not that was the plan.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Warrior"], 2, 2)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Other Goblin creatures you control attack each combat if able.",
                EffectDef::StaticApply {
                    // "Other Goblin creatures you control": the Rabblemaster is a Goblin too
                    // and is not made to attack by its own clause.
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Goblin"),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ))),
                    effect: AppliedEffectDef::add_ability(&abilities::attacks_each_combat_if_able()),
                },
            ),
            AbilityDef::triggered(
                "At the beginning of combat on your turn, create a 1/1 red Goblin creature token with \
                 haste.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::BeginningOfCombat,
                    player: PlayerRelation::You,
                },
                EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1)
                    .with_abilities(&[abilities::haste()])
                    .with_art(CardArt::new(
                        "98993a45-4aff-4f9b-a030-7d72fbb4ec6c",
                        "Karl Kopinski",
                    )),
            ),
            AbilityDef::triggered(
                "Whenever this creature attacks, it gets +1/+0 until end of turn for each other attacking \
                 Goblin.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        // Every other Goblin in the attack, whoever controls it. The count is read
                        // as the trigger resolves, so a Goblin that was removed in response is not
                        // among them.
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::new(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Subtype("Goblin"),
                                ObjectPredicateDef::Attacking,
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                        )),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&TRIPLICATE_SPIRITS, &FRENZIED_GOBLIN, &GOBLIN_RABBLEMASTER];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
