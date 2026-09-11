//! RNA card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AlternativeCastKindDef;
use crate::card::CardChoiceSourceDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::CostQuantityDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::ObjectPredicateDef;
use crate::card::PayOrDef;
use crate::card::PlayerRelation;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

static SPHINX_OPENING_TRIGGER: AbilityDef = AbilityDef::triggered(
    "At the beginning of your first upkeep, scry 3.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    abilities::scry(ValueDef::Constant(3)),
);

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "RNA",
    slug: "ravnica-allegiance",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// RNA 55 — Sphinx of Foresight
pub(in crate::card::sets) static SPHINX_OF_FORESIGHT: CardRecord = CardRecord::new(
    "Sphinx of Foresight",
    "cf2386fd-edc0-4731-8f4e-7a7c45548bf3",
    "Titus Lunter",
CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Sphinx"], 4, 4).with_abilities(&[
        AbilityDef::opening_hand_reveal(
            "You may reveal this card from your opening hand. If you do, scry 3 at the beginning of your first upkeep.",
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&SPHINX_OPENING_TRIGGER)),
        ),
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, scry 1.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            abilities::scry(ValueDef::Constant(1)),
        ),
    ]),
);

// RNA 115 — Skewer the Critics
pub(in crate::card::sets) static SKEWER_THE_CRITICS: CardRecord = CardRecord::new(
    "Skewer the Critics",
    "97295660-6bea-46ae-9a3b-0fc6abba407f",
    "Heonhwa",
    // A one-mana Lava Spike in the deck that was already attacking, and a
    // dead card in the deck that was not. Nothing about the spell changes
    // when spectacle pays for it; only the price does.
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Skewer the Critics deals 3 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
        ),
        // Spectacle (CR 702.137a) is an alternative cost gated on a board
        // condition, which is the same shape Mogg Salvage's free cast has.
        // "Lost life", not "was dealt damage": a Thoughtseize or a painland
        // turns it on just as well as an attack.
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{R}"))],
            AlternativeCastKindDef::AlternativeCost,
            Some(
                "Spectacle {R} (You may cast this spell for its spectacle cost rather than its \
                 mana cost if an opponent lost life this turn.)",
            ),
            EffectDef::None,
        )
        .with_alternative_condition(&TriggerConditionDef::OpponentLostLifeThisTurn),
    ]),
);

// RNA 171 — Final Payment
pub(in crate::card::sets) static FINAL_PAYMENT: CardRecord = CardRecord::new(
    "Final Payment",
    "49a21a8f-9c7b-4ae8-8635-f2ee2151c8de",
    "Victor Adame Minguez",
    CardRules::new_instant(mana_cost!("{W}{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, pay 5 life or sacrifice a creature or \
             enchantment.\nDestroy target creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            CostDef::choice(&[
                CostDef::pay_life(CostQuantityDef::Fixed(5)),
                CostDef::sacrifice(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                    CostQuantityDef::Fixed(1),
                ),
            ]),
            EffectDef::destroy_target(crate::TargetIndex::PRIMARY),
        ),
    ),
);

// RNA 172 — Fireblade Artist
pub(in crate::card::sets) static FIREBLADE_ARTIST: CardRecord = CardRecord::new(
    "Fireblade Artist",
    "21e1161f-bd2c-45a7-a86b-3b2e5210f148",
    "Steve Argyle",
    // Two damage every upkeep for a spare creature, and haste means the
    // Artist itself can be the first thing fed to it after it attacks.
    CardRules::new_creature(mana_cost!("{B}{R}"), &["Human", "Shaman"], 2, 2).with_abilities(&[
        abilities::haste(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, you may sacrifice a creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                ))],
                &EffectDef::None,
            )),
        ),
        AbilityDef::triggered_with_targets(
            "When you do, this creature deals 2 damage to target opponent or planeswalker.",
            // The reflexive half: it goes on the stack by itself once the
            // sacrifice is taken, and names its target only then.
            TriggerEventDef::OptionalEffectTaken(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Opponent),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
        ),
    ]),
);

// RNA 178 — Growth Spiral
pub(in crate::card::sets) static GROWTH_SPIRAL: CardRecord = CardRecord::new(
    "Growth Spiral",
    "7c77a6b1-ef06-4da5-8e86-a5204216cb77",
    "Seb McKinnon",
    // Ramping at instant speed is the point: the land drop it hands out is
    // extra, so this is a cantrip on a turn where the land would rot in hand.
    CardRules::new_instant(mana_cost!("{G}{U}")).with_ability(AbilityDef::spell(
        "Draw a card. You may put a land card from your hand onto the battlefield.",
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
            // "You may put": the card drawn can itself be the land, and
            // declining matters when the only land in hand is one you would
            // rather keep for a real land drop.
            EffectDef::ChooseCards {
                player: EffectRecipientDef::Controller,
                sources: &[CardChoiceSourceDef::Zone(ZoneKind::Hand)],
                object: ObjectPredicateDef::HasType(CardType::Land),
                minimum: 0,
                maximum: 1,
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
            },
        ]),
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SPHINX_OF_FORESIGHT,
    &SKEWER_THE_CRITICS,
    &FINAL_PAYMENT,
    &FIREBLADE_ARTIST,
    &GROWTH_SPIRAL,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
