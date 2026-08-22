//! Zendikar Rising cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AddManaEffectDef, CardArt, CardRules, CardSet, CardSupertype,
    CardType, ComparisonDef, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    ObjectQueryDef, ObjectSetDef, PlayerRelation, TriggerConditionDef, TriggerEventDef, ValueDef,
    ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

static CREATURE_OR_PLANESWALKER: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::HasType(CardType::Planeswalker),
]);

/// The mana-value bound is part of what may be targeted rather than something
/// checked on resolution, so an unkicked Thirst never points at anything
/// bigger in the first place.
static THIRST_SMALL_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        CREATURE_OR_PLANESWALKER,
        ObjectPredicateDef::ManaValueAtMost(2),
    ]),
)];

static THIRST_ANY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    CREATURE_OR_PLANESWALKER,
)];

static THIRST_DESTROY: EffectDef = EffectDef::Destroy {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    can_regenerate: true,
};

// ZNR 85 — Thieving Skydiver
// Audit: blocked — Kicker here is a spell cast for more mana with different instructions, and the kicked clause has to carry those instructions. This card's kicker changes nothing about how the spell resolves; it changes whether a triggered ability fires afterwards and what that ability may target, which the kicked alternative has no way to say. It also needs a minimum on X, since casts are enumerated from zero and "X can't be 0" would otherwise let an unkicked-sized cast steal a nothing-cost artifact.

// ZNR 94 — Bloodchief's Thirst
pub(in crate::card::sets) static BLOODCHIEFS_THIRST: CardRecord = CardRecord::new_with_legacy_id(
    2165,
    "Bloodchief's Thirst",
    CardArt::new("059e8447-6b1c-4651-a734-a8fea2cbf7b2", "Jason Rainville"),
    CardSet::ZendikarRising,
    // One black kills most of what an aggressive deck leads with; four kills
    // whatever is left, which is why the card is played over a cheaper
    // removal spell that can only do the first job.
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Kicker {2}{B} (You may pay an additional {2}{B} as you cast this spell.)\nDestroy target creature or planeswalker with mana value 2 or less.",
            &THIRST_SMALL_TARGET,
            THIRST_DESTROY,
        ),
        abilities::kicker(
            mana_cost!("{3}{B}"),
            "Destroy target creature or planeswalker.",
            &THIRST_ANY_TARGET,
            THIRST_DESTROY,
        ),
    ]),
);

/// A land arriving under its controller. Landfall watches the battlefield
/// rather than the land drop, so a land put onto the battlefield by a fetch
/// or a search counts the same way one played from hand does.
static A_LAND_YOU_CONTROL_ENTERING: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Land),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

static PLANESWALKERS_YOU_DO_NOT_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Planeswalker),
    &[ZoneKind::Battlefield],
    PlayerRelation::NotYou,
);

const fn omnath_resolution(amount: u8) -> TriggerConditionDef {
    TriggerConditionDef::SourceResolutionsThisTurn {
        comparison: ComparisonDef::Equal,
        amount,
    }
}

/// The count includes the resolution asking, so the first time reads one.
static OMNATH_FIRST_TIME: TriggerConditionDef = omnath_resolution(1);
static OMNATH_SECOND_TIME: TriggerConditionDef = omnath_resolution(2);
static OMNATH_THIRD_TIME: TriggerConditionDef = omnath_resolution(3);

static OMNATH_GAINS_FOUR: EffectDef = EffectDef::GainLife {
    recipient: EffectRecipientDef::Controller,
    amount: ValueDef::Constant(4),
};

/// Four mana of four colours is four separate additions: what the pool ends
/// up holding is the same either way, and one `AddMana` names a run of like
/// units plus at most one other.
static OMNATH_ADDS_FOUR_COLORS: [EffectDef; 4] = [
    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
];

static OMNATH_ADDS_MANA: EffectDef = EffectDef::Sequence(&OMNATH_ADDS_FOUR_COLORS);

static OMNATH_BURNS: [EffectDef; 2] = [
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Opponent,
        amount: ValueDef::Constant(4),
    },
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
            PLANESWALKERS_YOU_DO_NOT_CONTROL,
        )),
        amount: ValueDef::Constant(4),
    },
];

static OMNATH_BURNS_EVERYTHING: EffectDef = EffectDef::Sequence(&OMNATH_BURNS);

/// Three exclusive branches on one count, so a fourth land does nothing at
/// all rather than repeating the third.
static OMNATH_LANDFALL: [EffectDef; 3] = [
    EffectDef::IfCondition {
        condition: &OMNATH_FIRST_TIME,
        then: &OMNATH_GAINS_FOUR,
    },
    EffectDef::IfCondition {
        condition: &OMNATH_SECOND_TIME,
        then: &OMNATH_ADDS_MANA,
    },
    EffectDef::IfCondition {
        condition: &OMNATH_THIRD_TIME,
        then: &OMNATH_BURNS_EVERYTHING,
    },
];

static OMNATH_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::triggered(
        "When Omnath enters, draw a card.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    ),
    AbilityDef::triggered(
        "Landfall — Whenever a land you control enters, you gain 4 life if this is the first time \
         this ability has resolved this turn. If it's the second time, add {R}{G}{W}{U}. If it's \
         the third time, Omnath deals 4 damage to each opponent and each planeswalker you don't \
         control.",
        TriggerEventDef::zone_changed(
            A_LAND_YOU_CONTROL_ENTERING,
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::Sequence(&OMNATH_LANDFALL),
    ),
];

// ZNR 232 — Omnath, Locus of Creation
pub(in crate::card::sets) static OMNATH_LOCUS_OF_CREATION: CardRecord =
    CardRecord::new_with_legacy_id(
        2264,
        "Omnath, Locus of Creation",
        CardArt::new("4e4fb50c-a81f-44d3-93c5-fa9a0b37f617", "Chris Rahn"),
        CardSet::ZendikarRising,
        // Four colours for a 4/4 that replaces itself, and a deck full of
        // fetchlands turns the third land of a turn into eight damage.
        CardRules::new_creature(mana_cost!("{R}{G}{W}{U}"), &["Elemental"], 4, 4)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&OMNATH_ABILITIES),
    );

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&BLOODCHIEFS_THIRST, &OMNATH_LOCUS_OF_CREATION];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
