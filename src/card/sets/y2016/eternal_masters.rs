//! Eternal Masters cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AlternativeCastKindDef, AppliedEffectDef, CardArt, CardRules, CardSet, ComparisonDef,
    ConditionalStaticEffectDef, CostQuantityDef, EffectDef, EffectRecipientDef, LikelihoodDef,
    ManaColor, ObjectPredicateDef, ObjectQueryDef, ObjectSetCountConditionDef, ObjectSetDef,
    ObjectSetPredicateDef, PlayerRelation, SpellAdditionalCostDef, StaticApplyDef, TriggerEventDef,
    TurnStepDef, ValueDef, ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

// EMA 6 — Coalition Honor Guard
// Audit: unsupported — Needs a rule that constrains how an opponent chooses targets. AppliedRuleDef can forbid actions and require blocks, but nothing requires a target to be chosen, which is the whole of what a Flagbearer does.
pub(in crate::card::sets) static COALITION_HONOR_GUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c5b7be3e-b4af-46d4-bcc6-b44c651f2012"),
    "Coalition Honor Guard",
    crate::card::CardArt::new("2c7c2b5c-634a-4d83-81bc-c6128e3ac339", "Eric Peterson"),
    crate::card::CardSet::EternalMasters,
    crate::card::CardRules::unsupported(),
);

// EMA 45 — Deep Analysis
pub(in crate::card::sets) static DEEP_ANALYSIS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("01e3c2e9-d8df-4a7a-be86-7be8c6254fa2"),
    "Deep Analysis",
    CardArt::new("821cc8b6-eb2e-4441-8d88-c54cb44ab024", "Jesper Ejsing"),
    CardSet::EternalMasters,
    // Four cards out of one card, paid for in life and tempo rather than in
    // mana: the flashback is the half that actually gets cast.
    CardRules::new_sorcery(mana_cost!("{3}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target player draws two cards.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
        AbilityDef::alternative_cast(
            mana_cost!("{1}{U}"),
            AlternativeCastKindDef::Flashback,
            Some("Flashback—{1}{U}, Pay 3 life."),
            EffectDef::None,
        )
        .with_alternative_additional_cost(&SpellAdditionalCostDef::pay_life(
            CostQuantityDef::Fixed(3),
        )),
    ]),
);

// EMA 119 — Beetleback Chief
pub(in crate::card::sets) static BEETLEBACK_CHIEF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1e3ccf3d-583c-46b4-b51e-ae1b0628d506"),
    "Beetleback Chief",
    CardArt::new("779d4745-ff14-4c79-b2c8-8e273faf7375", "Wayne England"),
    CardSet::EternalMasters,
    // Four power across three bodies for four mana: the Chief is a sacrifice
    // outlet's worth of goblins rather than one creature.
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Goblin", "Warrior"], 2, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, create two 1/1 red Goblin creature tokens.",
            EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1).with_amount(2),
        ),
    ),
);

// EMA 139 — Mogg War Marshal
pub(in crate::card::sets) static MOGG_WAR_MARSHAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8b9e0bdb-b615-447a-b80d-d7244c25c56e"),
    "Mogg War Marshal",
    CardArt::new("deed0a5a-6662-460c-bd78-e3d95e8bc83e", "Jesper Ejsing"),
    CardSet::EternalMasters,
    // Letting the echo go unpaid is the normal line: three bodies for two
    // mana, and the last one arrives because the Marshal died.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Warrior"], 1, 1).with_abilities(&[
        abilities::echo(
            "Echo {1}{R} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
            mana_cost!("{1}{R}"),
        ),
        AbilityDef::triggered(
            "When this creature enters or dies, create a 1/1 red Goblin creature token.",
            // One printed sentence with two ways in, so it is one ability
            // watching both zone changes rather than two abilities.
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
            ]),
            EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1),
        ),
    ]),
);

// EMA 191 — Werebear
pub(in crate::card::sets) static WEREBEAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("964cf7e3-932d-432f-8ad4-9bd651aada96"),
    "Werebear",
    CardArt::new("224ea635-b95b-4803-8716-edd4cb655923", "Filip Burburan"),
    CardSet::EternalMasters,
    // A mana elf that turns into a 4/4 once the graveyard fills, and the
    // mana ability is what fills it in the decks that want the body.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Bear", "Druid"], 1, 1)
        .with_abilities(&[
            AbilityDef::activated_mana(
                "{T}: Add {G}.",
                &[AbilityCostDef::TapSource],
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
            ),
            AbilityDef::static_ability(
                "Threshold — This creature gets +3/+3 as long as there are seven or more cards in your graveyard.",
                // Read continuously rather than latched, so a graveyard that
                // shrinks back under seven takes the bonus away again.
                EffectDef::ConditionalStatic(ConditionalStaticEffectDef {
                    condition: ObjectSetCountConditionDef {
                        objects: &ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        )),
                        predicate: ObjectSetPredicateDef {
                            filter: None,
                            comparison: ComparisonDef::GreaterOrEqual,
                            amount: 7,
                        },
                    },
                    then: StaticApplyDef {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(3),
                            ValueDef::Constant(3),
                        ),
                    },
                }),
            ),
        ]),
);

// EMA 225 — Mana Crypt
pub(in crate::card::sets) static MANA_CRYPT: CardRecord = CardRecord::new_with_legacy_id(
    2142,
    "Mana Crypt",
    CardArt::new("0cb33b46-4d1b-4f97-bfdc-d815aee111da", "Matt Stewart"),
    CardSet::EternalMasters,
    CardRules::new_artifact(mana_cost!("{0}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, flip a coin. If you lose the flip, this artifact deals 3 damage to you.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::Randomized {
                likelihood: LikelihoodDef::new(0.5),
                on_success: &EffectDef::None,
                // Losing the flip is the whole cost of the card, and it is paid to the
                // artifact itself: three damage from a source its controller chose to keep
                // around.
                on_failure: &EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {C}{C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(2)),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &COALITION_HONOR_GUARD,
    &DEEP_ANALYSIS,
    &BEETLEBACK_CHIEF,
    &MOGG_WAR_MARSHAL,
    &WEREBEAR,
    &MANA_CRYPT,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
