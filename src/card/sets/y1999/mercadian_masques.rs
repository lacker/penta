//! Mercadian Masques cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AlternativeCastKindDef,
    AppliedEffectDef, AppliedRuleDef, BasicLandType, CardArt, CardRules, CardSet, CardSupertype,
    CardType, ComparisonDef, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    ObjectQueryDef, PlayerRelation, SpellAdditionalCostDef, SpendModeDef, TriggerConditionDef,
    ValueDef, ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

/// Three prohibitions, applied together for the same duration, so the Aura
/// leaving gives all three back at once.
static ARREST_PROHIBITIONS: [AppliedEffectDef; 3] = [
    AppliedEffectDef::Rule(AppliedRuleDef::CannotAttack),
    AppliedEffectDef::Rule(AppliedRuleDef::CannotBlock),
    AppliedEffectDef::Rule(AppliedRuleDef::CannotActivateAbilities),
];

// MMQ 4 — Arrest
pub(in crate::card::sets) static ARREST: CardRecord = CardRecord::new_with_legacy_id(
    1952,
    "Arrest",
    CardArt::new("3b083fd8-6422-4cd3-a27d-41b6d88598c2", "Dan Frazier"),
    CardSet::MercadianMasques,
    // The creature keeps its triggered and static abilities: only the
    // activations are shut off.
    CardRules::new_enchantment(mana_cost!("{2}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature can't attack or block, and its activated abilities can't be \
                 activated.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&ARREST_PROHIBITIONS),
                },
            ),
        ]),
);

static NONBASIC_LAND: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Land),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(CardSupertype::Basic)),
]);

/// Islands you control, returned rather than sacrificed: what the cycle buys
/// is tempo, not card advantage, and the lands are back in hand to replay.
const fn return_islands(count: u8) -> SpellAdditionalCostDef {
    SpellAdditionalCostDef::new(
        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
        ZoneKind::Battlefield,
        count,
    )
    .spent(SpendModeDef::ReturnToHand)
}

static GUSH_COST: SpellAdditionalCostDef = return_islands(2);
static THWART_COST: SpellAdditionalCostDef = return_islands(3);

static TARGET_SPELL: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Spell,
        zones: &[ZoneKind::Stack],
        controller: None,
        owner: None,
    },
)];

// MMQ 82 — Gush
pub(in crate::card::sets) static GUSH: CardRecord = CardRecord::new_with_legacy_id(
    2045,
    "Gush",
    CardArt::new("e755bbef-bf34-49c0-ae72-d70e3599de52", "Kev Walker"),
    CardSet::MercadianMasques,
    CardRules::new_instant(mana_cost!("{4}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Draw two cards.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some("You may return two Islands you control to their owner's hand rather than pay this spell's mana cost."),
            EffectDef::None,
        )
        .with_alternative_additional_cost(&GUSH_COST),
    ]),
);

// MMQ 108 — Thwart
pub(in crate::card::sets) static THWART: CardRecord = CardRecord::new_with_legacy_id(
    2046,
    "Thwart",
    CardArt::new("c12a0717-e9ea-4be3-a29f-179671ed4489", "Christopher Moeller"),
    CardSet::MercadianMasques,
    CardRules::new_instant(mana_cost!("{2}{U}{U}")).with_abilities(&[
        AbilityDef::counter_target("Counter target spell.", &TARGET_SPELL[0]),
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some("You may return three Islands you control to their owner's hand rather than pay this spell's mana cost."),
            EffectDef::None,
        )
        .with_alternative_additional_cost(&THWART_COST),
    ]),
);

/// A Swamp on the battlefield, which is what the free cast is gated on.
static YOU_CONTROL_A_SWAMP: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 1,
};

static SNUFF_OUT_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
    ]),
)];

// MMQ 162 — Snuff Out
pub(in crate::card::sets) static SNUFF_OUT: CardRecord = CardRecord::new_with_legacy_id(
    2158,
    "Snuff Out",
    CardArt::new("18a3cca1-e50e-49b6-9e1a-f86640e3b177", "Mike Ploog"),
    CardSet::MercadianMasques,
    // Four life and no mana is why it is played: the answer costs nothing on
    // the turn it is needed, which is somebody else's.
    CardRules::new_instant(mana_cost!("{3}{B}")).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some("If you control a Swamp, you may pay 4 life rather than pay this spell's mana cost."),
            EffectDef::None,
        )
        .with_alternative_life(4)
        .with_alternative_condition(&YOU_CONTROL_A_SWAMP),
        AbilityDef::destroy_target(
            "Destroy target nonblack creature. It can't be regenerated.",
            &SNUFF_OUT_TARGET[0],
            false,
        ),
    ]),
);

// MMQ 316 — Dust Bowl
pub(in crate::card::sets) static DUST_BOWL: CardRecord = CardRecord::new_with_legacy_id(
    280,
    "Dust Bowl",
    CardArt::new("75b03c30-c2b8-4207-b675-26c59c40a7e5", "Ben Thompson"),
    CardSet::MercadianMasques,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{3}, {T}, Sacrifice a land: Destroy target nonbasic land.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one_permanent(NONBASIC_LAND)],
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ]),
);

// MMQ 324 — Rishadan Port
pub(in crate::card::sets) static RISHADAN_PORT: CardRecord = CardRecord::new_with_legacy_id(
    281,
    "Rishadan Port",
    CardArt::new("477a1f53-5cdf-4b45-b584-2e36b31a3fdb", "Jerry Tiritilli"),
    CardSet::MercadianMasques,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{1}, {T}: Tap target land.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Land),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARREST,
    &GUSH,
    &THWART,
    &SNUFF_OUT,
    &DUST_BOWL,
    &RISHADAN_PORT,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
