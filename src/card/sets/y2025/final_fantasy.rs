//! Final Fantasy cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, ActivationTimingDef, AddManaEffectDef, AppliedEffectDef, CardArt,
    CardRules, CardSet, CardSupertype, CardType, CounterKind, DamageEventMatcherDef, DamageKindDef,
    DamageRecipientMatcherDef, DamageSourceMatcherDef, EffectDef, EffectRecipientDef, ManaColor,
    ObjectPredicateDef, ObjectRefDef, PlayerRelation, ResolvedEffectDurationDef,
    TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, abilities,
};
use crate::mana_cost;

// FIN 91 — Cecil, Dark Knight // Cecil, Redeemed Paladin
/// The front half's payoff, and the reason the card is played: hitting hard
/// enough to halve your own life is what turns Cecil over. Untapping is part
/// of the same clause, so a Cecil that traded its attack for the transform
/// comes back ready to block.
static CECIL_TURNS_OVER: [EffectDef; 2] = [
    EffectDef::Untap {
        object: EffectRecipientDef::Source,
    },
    EffectDef::Transform {
        object: EffectRecipientDef::Source,
    },
];

static CECIL_TRANSFORM_CHECK: TriggerConditionDef =
    TriggerConditionDef::ControllerLifeAtMostHalfStartingLife;

static CECIL_TRANSFORMS: EffectDef = EffectDef::Sequence(&CECIL_TURNS_OVER);

/// "You lose that much life. Then if ..." is one clause resolving in order:
/// the life is lost first, so the very damage that cost it can be what brings
/// the total low enough to turn the card over.
static CECIL_DARKNESS: [EffectDef; 2] = [
    EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::TriggerEventAmount,
    },
    EffectDef::IfCondition {
        condition: &CECIL_TRANSFORM_CHECK,
        then: &CECIL_TRANSFORMS,
    },
];

static CECIL_DARK_KNIGHT_ABILITIES: [AbilityDef; 2] = [
    abilities::deathtouch(),
    AbilityDef::triggered(
        "Darkness — Whenever Cecil deals damage, you lose that much life. Then if your life total is less than or equal to half your starting life total, untap Cecil and transform it.",
        TriggerEventDef::DamageDealt(DamageEventMatcherDef {
            kind: DamageKindDef::Any,
            source: DamageSourceMatcherDef::Object(ObjectRefDef::Source),
            recipient: DamageRecipientMatcherDef::Any,
        }),
        EffectDef::Sequence(&CECIL_DARKNESS),
    ),
];

const fn cecil_dark_knight_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{B}"), &["Human", "Knight"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&CECIL_DARK_KNIGHT_ABILITIES)
}

/// "Other attacking creatures" excludes Cecil and takes in the opponent's
/// too, on the rare turn both sides are attacking at once.
static OTHER_ATTACKING_CREATURES: EffectRecipientDef = EffectRecipientDef::matching_objects(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Attacking,
        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::Any,
);

static INDESTRUCTIBLE: AbilityDef = abilities::indestructible();

static CECIL_REDEEMED_PALADIN_ABILITIES: [AbilityDef; 2] = [
    abilities::lifelink(),
    AbilityDef::triggered(
        "Protect — Whenever Cecil attacks, other attacking creatures gain indestructible until end of turn.",
        TriggerEventDef::attacks(ObjectPredicateDef::Source),
        EffectDef::Apply {
            recipient: OTHER_ATTACKING_CREATURES,
            effect: AppliedEffectDef::add_ability(&INDESTRUCTIBLE),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    ),
];

/// The back face has no printed mana cost and is white, where the front is
/// black: transforming changes the colour it defends in.
const fn cecil_redeemed_paladin_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Human", "Knight"], 4, 4)
        .printed_colors(&[ManaColor::White])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&CECIL_REDEEMED_PALADIN_ABILITIES)
}

pub(in crate::card::sets) static CECIL_DARK_KNIGHT: CardRecord = CardRecord::new_dfc_with_legacy_id(
    2129,
    "Cecil, Dark Knight // Cecil, Redeemed Paladin",
    CardArt::new("026e7167-d665-43d0-a51e-8df2d68cdb5e", "Josu Hernaiz"),
    CardSet::FinalFantasy,
    &[
        ("Cecil, Dark Knight", cecil_dark_knight_rules()),
        ("Cecil, Redeemed Paladin", cecil_redeemed_paladin_rules()),
    ],
);

// FIN 114 — Resentful Revelation
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RESENTFUL_REVELATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("945006ea-c6a1-4ee5-abb2-387c2b6d3123"),
    "Resentful Revelation",
    crate::card::CardArt::new("945006ea-c6a1-4ee5-abb2-387c2b6d3123", "Justyna Dura"),
    crate::card::CardSet::FinalFantasy,
    crate::card::CardRules::unsupported(),
);

// FIN 164 — Suplex
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SUPLEX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f61693a2-7042-44e0-85ba-9bf12ab94e7e"),
    "Suplex",
    crate::card::CardArt::new("f61693a2-7042-44e0-85ba-9bf12ab94e7e", "Fang Xinyu"),
    crate::card::CardSet::FinalFantasy,
    crate::card::CardRules::unsupported(),
);

// FIN 206 — Tifa Lockhart
/// A land you control, not any land: the opponent's fetchland does nothing
/// for her.
static A_LAND_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Land),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

/// Doubling is +X/+0 where X is her power as this resolves, so two landfalls
/// in a turn compound: the second reads the size the first left behind.
static TIFA_DOUBLES: [AbilityDef; 2] = [
    abilities::trample(),
    AbilityDef::triggered(
        "Landfall — Whenever a land you control enters, double Tifa Lockhart's power until end of turn.",
        TriggerEventDef::zone_changed(A_LAND_YOU_CONTROL, None, Some(ZoneKind::Battlefield)),
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::SourcePower,
                ValueDef::Constant(0),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    ),
];

pub(in crate::card::sets) static TIFA_LOCKHART: CardRecord = CardRecord::new_with_legacy_id(
    2146,
    "Tifa Lockhart",
    CardArt::new("fb781323-2746-405d-a9b2-e778c037a6e9", "Laurel Austin"),
    CardSet::FinalFantasy,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Monk"], 1, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&TIFA_DOUBLES),
);

// FIN 248 — Vivi Ornitier
/// "Add X mana in any combination of {U} and/or {R}" divides one amount
/// across two types, so the runtime offers the ability once per division.
/// Vivi enters with no power at all, so the first activation worth making
/// comes after a noncreature spell has grown it.
static VIVI_MANA: AddManaEffectDef =
    AddManaEffectDef::combination(&VIVI_COLORS, 0).with_variable_amount(ValueDef::SourcePower);

static VIVI_COLORS: [ManaColor; 2] = [ManaColor::Blue, ManaColor::Red];

static VIVI_COST: [AbilityCostDef; 1] = [AbilityCostDef::Mana(mana_cost!("{0}"))];

/// The counter and the damage are one clause, and the counter comes first --
/// so a Vivi that has just been cast at is already bigger by the time its own
/// mana ability is next offered.
static VIVI_PAYOFF: [EffectDef; 2] = [
    EffectDef::AddCounters {
        object: EffectRecipientDef::Source,
        kind: CounterKind::PlusOnePlusOne,
        amount: ValueDef::Constant(1),
    },
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Opponent,
        amount: ValueDef::Constant(1),
    },
];

static VIVI_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::activated_mana(
        "{0}: Add X mana in any combination of {U} and/or {R}, where X is this creature's power. Activate only during your turn and only once each turn.",
        &VIVI_COST,
        EffectDef::AddMana(VIVI_MANA),
    )
    .with_activation_timing(ActivationTimingDef::YourTurn)
    .activations_each_turn(1),
    AbilityDef::triggered(
        "Whenever you cast a noncreature spell, put a +1/+1 counter on this creature and it deals 1 damage to each opponent.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
            ObjectPredicateDef::NoncreatureSpell,
            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
        ])),
        EffectDef::Sequence(&VIVI_PAYOFF),
    ),
];

pub(in crate::card::sets) static VIVI_ORNITIER: CardRecord = CardRecord::new_with_legacy_id(
    2162,
    "Vivi Ornitier",
    CardArt::new("ecc1027a-8c07-44a0-bdde-fa2844cff694", "Toni Infante"),
    CardSet::FinalFantasy,
    CardRules::new_creature(mana_cost!("{1}{U}{R}"), &["Wizard"], 0, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&VIVI_ABILITIES),
);

// FIN 289 — Starting Town
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STARTING_TOWN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fc7d1912-7e27-49ef-bd98-375d975a42b0"),
    "Starting Town",
    crate::card::CardArt::new("fc7d1912-7e27-49ef-bd98-375d975a42b0", "Hristo D. Chukov"),
    crate::card::CardSet::FinalFantasy,
    crate::card::CardRules::unsupported(),
);

// FIN 551c — Traveling Chocobo
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TRAVELING_CHOCOBO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("156cfd45-1556-4804-becf-039cfff7de3d"),
    "Traveling Chocobo",
    crate::card::CardArt::new("156cfd45-1556-4804-becf-039cfff7de3d", "Toni Infante"),
    crate::card::CardSet::FinalFantasy,
    crate::card::CardRules::unsupported(),
);

// FIN 581 — Astrologian's Planisphere
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ASTROLOGIAN_S_PLANISPHERE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a0f6e2d7-58b5-4a7d-8c42-e25185cd173f"),
    "Astrologian's Planisphere",
    crate::card::CardArt::new("a0f6e2d7-58b5-4a7d-8c42-e25185cd173f", "Josephine Chang"),
    crate::card::CardSet::FinalFantasy,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &CECIL_DARK_KNIGHT,
    &RESENTFUL_REVELATION,
    &SUPLEX,
    &TIFA_LOCKHART,
    &VIVI_ORNITIER,
    &STARTING_TOWN,
    &TRAVELING_CHOCOBO,
    &ASTROLOGIAN_S_PLANISPHERE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
