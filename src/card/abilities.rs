//! Reusable constructors for common ability clauses.
//!
//! The functions here return identity-free [`AbilityDef`] values. A card part,
//! intrinsic rule, or grant site assigns identity when it attaches the clause.

use super::model::{
    AbilityCostDef, AbilityCostList, AbilityCoverageDef, AbilityDef, AbilityTargetDef,
    AbilityTargetPredicate, ActivationTimingDef, AddManaEffectDef, AlternativeCastKindDef,
    AnimationDef, AppliedEffectDef, BasicLandType, BattlefieldEntryModificationDef, CardType,
    CardTypeSet, ConditionDef, CostDef, CounterKind, DeclarativeAbilityDef, EffectDef,
    EffectDurationDef, EffectRecipientDef, KeywordAbility, ManaColor, ManaCost, ObjectPredicateDef,
    ObjectQueryDef, PaymentDef, PlayerRelation, ReplacementAbilityDef, ReplacementEffectDef,
    ReplacementEventDef, ScaledValueDef, ShieldCoverageDef, TriggerEventDef, ValueDef, ZoneKind,
};
use crate::ids::{ChoiceIndex, TargetIndex};

/// The target an "Enchant creature" Aura spell chooses.
pub static ENCHANT_CREATURE_TARGET: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::HasType(CardType::Creature),
    )];

/// The target an "Enchant land" Aura spell chooses.
pub static ENCHANT_LAND_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Land),
)];

static NOT_SOURCE: ObjectPredicateDef = ObjectPredicateDef::Not(&ObjectPredicateDef::Source);
static CREATURE_YOU_CONTROL: [ObjectPredicateDef; 2] = [
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
];
static LAND_YOU_CONTROL: [ObjectPredicateDef; 2] = [
    ObjectPredicateDef::HasType(CardType::Land),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
];
static OTHER_CREATURE_YOU_CONTROL: [ObjectPredicateDef; 3] = [
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
    NOT_SOURCE,
];

pub static ATTACH_CREATURE_YOU_CONTROL_TARGET: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::All(&CREATURE_YOU_CONTROL),
    )];

static RECONFIGURE_CREATURE_YOU_CONTROL_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&OTHER_CREATURE_YOU_CONTROL),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
    1,
)];

pub static ATTACH_LAND_YOU_CONTROL_TARGET: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::All(&LAND_YOU_CONTROL),
    )];

/// An Aura's own spell clause: it targets what it will enchant, and attaching
/// is what the spell does when it resolves. Every Aura prints one, so it
/// belongs here rather than once per set module.
#[must_use]
pub const fn aura_spell(text: &'static str, targets: &'static [AbilityTargetDef]) -> AbilityDef {
    AbilityDef::aura_spell(
        text,
        targets,
        EffectDef::Attach {
            object: EffectRecipientDef::Target(crate::ids::TargetIndex::PRIMARY),
        },
    )
}

/// Bestow changes the spell into an Aura only for this alternative cast.
#[must_use]
pub const fn bestow(mana_cost: ManaCost) -> AbilityDef {
    AbilityDef::alternative_cast_with_targets(
        mana_cost,
        AlternativeCastKindDef::Bestow,
        &ENCHANT_CREATURE_TARGET,
        EffectDef::Attach {
            object: EffectRecipientDef::Target(crate::ids::TargetIndex::PRIMARY),
        },
    )
}

/// The shared equip activation. Attachment legality is rechecked when the
/// ability resolves, independently of its target legality.
#[must_use]
pub const fn equip(mana_cost: ManaCost, text: &'static str) -> AbilityDef {
    AbilityDef::activated_with_cost_list_and_targets(
        text,
        AbilityCostList::one(AbilityCostDef::Mana(mana_cost)),
        &ATTACH_CREATURE_YOU_CONTROL_TARGET,
        EffectDef::Attach {
            object: EffectRecipientDef::Target(crate::ids::TargetIndex::PRIMARY),
        },
    )
    .with_activation_timing(ActivationTimingDef::SorcerySpeed)
}

#[must_use]
pub const fn fortify(mana_cost: ManaCost, text: &'static str) -> AbilityDef {
    AbilityDef::activated_with_cost_list_and_targets(
        text,
        AbilityCostList::one(AbilityCostDef::Mana(mana_cost)),
        &ATTACH_LAND_YOU_CONTROL_TARGET,
        EffectDef::Attach {
            object: EffectRecipientDef::Target(crate::ids::TargetIndex::PRIMARY),
        },
    )
    .with_activation_timing(ActivationTimingDef::SorcerySpeed)
}

/// Reconfigure is deliberately not equip. Its one printed clause offers the
/// targeted attach move and the targetless unattach move from the same action
/// origin, and changes the source's characteristics while attached.
#[must_use]
pub const fn reconfigure(mana_cost: ManaCost, text: &'static str) -> AbilityDef {
    AbilityDef::activated_with_cost_list_and_targets(
        text,
        AbilityCostList::one(AbilityCostDef::Mana(mana_cost)),
        &RECONFIGURE_CREATURE_YOU_CONTROL_TARGET,
        EffectDef::Reconfigure {
            object: EffectRecipientDef::Target(crate::ids::TargetIndex::PRIMARY),
        },
    )
    .with_activation_timing(ActivationTimingDef::SorcerySpeed)
}

#[must_use]
pub const fn living_weapon(token: crate::CardDefinitionId) -> AbilityDef {
    AbilityDef::triggered(
        "Living weapon (When this Equipment enters, create a 0/0 black Phyrexian Germ creature token, then attach this to it.)",
        TriggerEventDef::ZoneChanged {
            object: ObjectPredicateDef::Source,
            from: None,
            to: Some(ZoneKind::Battlefield),
        },
        EffectDef::CreateAttachedToken { token },
    )
}

/// Mishra's Factory's 2/2 Assembly-Worker artifact creature. The card's
/// animation still resolves through its legacy immediate path, which reads
/// this definition rather than restating the creature it becomes.
pub static MISHRAS_FACTORY_ANIMATION: AnimationDef = AnimationDef::new(2, 2)
    .with_types(CardTypeSet::single(CardType::Creature).with(CardType::Artifact))
    .with_subtypes(&["Assembly-Worker"]);

/// "Attacks each combat if able." Cards state this in their own words rather
/// than as a printed keyword, so the text is supplied by the caller.
#[must_use]
pub const fn attacks_each_combat_if_able(text: &'static str) -> AbilityDef {
    keyword(text, KeywordAbility::AttacksEachCombatIfAble)
}
const ENTER_TAPPED: [ReplacementEffectDef; 1] = [ReplacementEffectDef::ModifyBattlefieldEntry(
    BattlefieldEntryModificationDef::Tapped,
)];
const PAY_TWO_LIFE: [CostDef; 1] = [CostDef::PayLife(2)];

const fn keyword(text: &'static str, keyword: KeywordAbility) -> AbilityDef {
    AbilityDef::keyword(text, keyword)
}

#[must_use]
pub const fn flying() -> AbilityDef {
    keyword("Flying", KeywordAbility::Flying)
}

#[must_use]
pub const fn trample() -> AbilityDef {
    keyword("Trample", KeywordAbility::Trample)
}

#[must_use]
pub const fn haste() -> AbilityDef {
    keyword("Haste", KeywordAbility::Haste)
}

#[must_use]
pub const fn first_strike() -> AbilityDef {
    keyword("First strike", KeywordAbility::FirstStrike)
}

#[must_use]
pub const fn defender() -> AbilityDef {
    keyword("Defender", KeywordAbility::Defender)
}

#[must_use]
pub const fn double_strike() -> AbilityDef {
    keyword("Double strike", KeywordAbility::DoubleStrike)
}

#[must_use]
pub const fn banding() -> AbilityDef {
    keyword("Banding", KeywordAbility::Banding).with_coverage(AbilityCoverageDef::partial(
        "Blocking with banding moves the attacker's combat damage assignment to the \
         defending player, which is implemented. Attacking in a band is not: bands are \
         neither declared nor blocked as a group.",
    ))
}

#[must_use]
pub const fn vigilance() -> AbilityDef {
    keyword("Vigilance", KeywordAbility::Vigilance)
}

#[must_use]
pub const fn deathtouch() -> AbilityDef {
    keyword("Deathtouch", KeywordAbility::Deathtouch)
}

#[must_use]
pub const fn lifelink() -> AbilityDef {
    keyword("Lifelink", KeywordAbility::Lifelink)
}

#[must_use]
pub const fn reach() -> AbilityDef {
    keyword("Reach", KeywordAbility::Reach)
}

#[must_use]
pub const fn flash() -> AbilityDef {
    keyword("Flash", KeywordAbility::Flash)
}

#[must_use]
pub const fn hexproof() -> AbilityDef {
    keyword("Hexproof", KeywordAbility::Hexproof)
}

#[must_use]
pub const fn shroud() -> AbilityDef {
    keyword("Shroud", KeywordAbility::Shroud)
}

#[must_use]
pub const fn intimidate() -> AbilityDef {
    keyword("Intimidate", KeywordAbility::Intimidate)
}

#[must_use]
pub const fn undying() -> AbilityDef {
    keyword("Undying", KeywordAbility::Undying)
}

#[must_use]
pub const fn indestructible() -> AbilityDef {
    keyword("Indestructible", KeywordAbility::Indestructible)
}

/// Rampage N (CR 702.23): whenever this creature becomes blocked, it gets
/// +N/+N until end of turn for each creature blocking it beyond the first.
/// The event carries that count, so the clause only supplies N.
/// The per-blocker bonus for each printed rampage value. `ValueDef::Scaled`
/// holds its operand by reference, and a value built from a parameter cannot
/// be promoted to `'static`, so the printed amounts are named here.
static RAMPAGE_SCALES: [ScaledValueDef; 4] = [
    ScaledValueDef::new(ValueDef::TriggerEventAmount, 0),
    ScaledValueDef::new(ValueDef::TriggerEventAmount, 1),
    ScaledValueDef::new(ValueDef::TriggerEventAmount, 2),
    ScaledValueDef::new(ValueDef::TriggerEventAmount, 3),
];

/// # Panics
///
/// Panics when `amount` is not a printed rampage value (1 through 3).
#[must_use]
pub const fn rampage(amount: usize, text: &'static str) -> AbilityDef {
    assert!(
        amount >= 1 && amount < RAMPAGE_SCALES.len(),
        "rampage is only printed with amounts 1 through 3"
    );
    let scale = &RAMPAGE_SCALES[amount];
    AbilityDef::triggered(
        text,
        TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Scaled(scale),
                toughness: ValueDef::Scaled(scale),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )
}

/// The printed landwalk clause for one basic land type. The rules text is the
/// keyword on its own, exactly as the card prints it.
#[must_use]
pub const fn landwalk(land_type: BasicLandType) -> AbilityDef {
    let text = match land_type {
        BasicLandType::Plains => "Plainswalk",
        BasicLandType::Island => "Islandwalk",
        BasicLandType::Swamp => "Swampwalk",
        BasicLandType::Mountain => "Mountainwalk",
        BasicLandType::Forest => "Forestwalk",
    };
    keyword(text, KeywordAbility::Landwalk(land_type))
}

#[must_use]
pub const fn legendary_landwalk() -> AbilityDef {
    keyword("Legendary landwalk", KeywordAbility::LegendaryLandwalk)
}

#[must_use]
pub const fn mountainwalk() -> AbilityDef {
    landwalk(BasicLandType::Mountain)
}

#[must_use]
pub const fn forestwalk() -> AbilityDef {
    landwalk(BasicLandType::Forest)
}

#[must_use]
pub const fn protection_from(color: ManaColor) -> AbilityDef {
    let text = match color {
        ManaColor::White => "Protection from white",
        ManaColor::Blue => "Protection from blue",
        ManaColor::Black => "Protection from black",
        ManaColor::Red => "Protection from red",
        ManaColor::Green => "Protection from green",
        ManaColor::Colorless => "Protection from colorless",
    };
    keyword(text, KeywordAbility::ProtectionFrom(color))
}

/// The granted protection and the self-retention exception for each printed
/// Ward. `EffectDef::Sequence` holds its clauses by reference, and a sequence
/// built from a parameter cannot be promoted to `'static`, so the five are
/// named here rather than rebuilt per card.
static WARD_PROTECTIONS: [AbilityDef; 5] = [
    protection_from(ManaColor::White),
    protection_from(ManaColor::Blue),
    protection_from(ManaColor::Black),
    protection_from(ManaColor::Red),
    protection_from(ManaColor::Green),
];

static WARD_CLAUSES: [[EffectDef; 2]; 5] = [
    ward_clauses(0),
    ward_clauses(1),
    ward_clauses(2),
    ward_clauses(3),
    ward_clauses(4),
];

const fn ward_clauses(color: usize) -> [EffectDef; 2] {
    [
        EffectDef::Apply {
            recipient: EffectRecipientDef::AttachedPermanent,
            effect: AppliedEffectDef::GrantAbility(&WARD_PROTECTIONS[color]),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
        // Without this the Aura would be an illegal attachment the moment it
        // granted protection from its own colour, which is exactly what the
        // printed exception exists to stop.
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::RemainsAttachedThroughProtection,
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    ]
}

/// One Ward's whole granted clause. The card supplies its own rules text,
/// which names the colour and prints the exception in one sentence.
///
/// # Panics
///
/// Panics when `color` is colorless, which no Ward is printed with.
#[must_use]
pub const fn ward(color: ManaColor, text: &'static str) -> AbilityDef {
    let clauses = match color {
        ManaColor::White => &WARD_CLAUSES[0],
        ManaColor::Blue => &WARD_CLAUSES[1],
        ManaColor::Black => &WARD_CLAUSES[2],
        ManaColor::Red => &WARD_CLAUSES[3],
        ManaColor::Green => &WARD_CLAUSES[4],
        ManaColor::Colorless => panic!("no Ward is printed with colorless protection"),
    };
    AbilityDef::static_ability(text, EffectDef::Sequence(clauses))
}

/// A printed flashback clause. Its attached ability identity becomes the
/// spell play option's alternative-cost identity.
#[must_use]
pub const fn flashback(mana_cost: ManaCost) -> AbilityDef {
    AbilityDef::alternative_cast(
        mana_cost,
        AlternativeCastKindDef::Flashback,
        None,
        EffectDef::None,
    )
}

/// Miracle, the permission to cast a card from hand for a different cost in
/// the window opened by drawing it.
#[must_use]
pub const fn miracle(mana_cost: ManaCost) -> AbilityDef {
    AbilityDef::alternative_cast(
        mana_cost,
        AlternativeCastKindDef::Miracle,
        None,
        EffectDef::None,
    )
}

/// A flashback ability whose cost is the mana cost of the card carrying it.
/// This is the form granted by Snapcaster Mage.
#[must_use]
pub const fn flashback_for_card_mana_cost() -> AbilityDef {
    AbilityDef::alternative_cast_for_card_mana_cost(
        AlternativeCastKindDef::Flashback,
        None,
        EffectDef::None,
    )
}

/// A printed overload clause. `effect` is the spell after every instance of
/// "target" has been changed to "each."
#[must_use]
pub const fn overload(
    mana_cost: ManaCost,
    stack_text: &'static str,
    effect: EffectDef,
) -> AbilityDef {
    AbilityDef::alternative_cast(
        mana_cost,
        AlternativeCastKindDef::Overload,
        Some(stack_text),
        effect,
    )
}

/// A Bloodrush ability activated from the card carrying it in hand. The
/// mechanic always discards that card in addition to paying its mana cost;
/// the card supplies its exact rules text, target declaration, and effect.
#[must_use]
pub const fn bloodrush(
    mana_cost: ManaCost,
    text: &'static str,
    targets: &'static [AbilityTargetDef],
    effect: EffectDef,
) -> AbilityDef {
    AbilityDef::activated_with_cost_list_and_targets(
        text,
        AbilityCostList::two(
            AbilityCostDef::Mana(mana_cost),
            AbilityCostDef::DiscardSource,
        ),
        targets,
        effect,
    )
    .with_source_zones(&[ZoneKind::Hand])
}

/// Scavenge, whose printed cost is the card's own exile from its owner's
/// graveyard and whose counter count is the exiled card's power. Reminder
/// text carries the mana cost, so each card supplies its own literal.
#[must_use]
pub const fn scavenge(mana_cost: ManaCost, text: &'static str) -> AbilityDef {
    AbilityDef::activated_with_cost_list_and_targets(
        text,
        AbilityCostList::two(AbilityCostDef::Mana(mana_cost), AbilityCostDef::ExileSource),
        SCAVENGE_TARGET,
        EffectDef::AddCounters {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            kind: CounterKind::PlusOnePlusOne,
            amount: ValueDef::SourcePower,
        },
    )
    .with_source_zones(&[ZoneKind::Graveyard])
    .with_activation_timing(ActivationTimingDef::SorcerySpeed)
}

static SCAVENGE_TARGET: &[AbilityTargetDef] = &[AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

/// The intrinsic stack-zone rule carried by spells that cannot be countered.
#[must_use]
pub const fn cannot_be_countered() -> AbilityDef {
    AbilityDef::static_ability(
        "This spell can't be countered.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::CannotBeCountered,
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    )
    .with_source_zones(&[ZoneKind::Stack])
}

/// A common mana ability that taps its source to add one fixed kind of mana.
#[must_use]
pub const fn tap_for(mana: ManaColor) -> AbilityDef {
    let text = match mana {
        ManaColor::White => "{T}: Add {W}.",
        ManaColor::Blue => "{T}: Add {U}.",
        ManaColor::Black => "{T}: Add {B}.",
        ManaColor::Red => "{T}: Add {R}.",
        ManaColor::Green => "{T}: Add {G}.",
        ManaColor::Colorless => "{T}: Add {C}.",
    };
    AbilityDef::activated_mana(
        text,
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::one(mana)),
    )
}

/// The two mana abilities shared by the allied- and enemy-color painlands.
#[must_use]
pub const fn pain_land(
    colored_text: &'static str,
    colors: &'static [ManaColor],
) -> [AbilityDef; 2] {
    [
        tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            colored_text,
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(colors).with_damage_to_controller(1)),
        ),
    ]
}

/// The shared replacement clause printed on shock lands.
#[must_use]
pub const fn shock_land_enters() -> AbilityDef {
    AbilityDef::as_enters(
        "As this land enters, you may pay 2 life. If you don't, it enters tapped.",
        ReplacementEffectDef::OptionalPayment {
            payment: PaymentDef::new(PlayerRelation::You, &PAY_TWO_LIFE),
            if_paid: &[],
            if_declined: &ENTER_TAPPED,
        },
    )
}

/// An unconditional battlefield-entry replacement shared by permanents that
/// enter tapped.
#[must_use]
pub const fn enters_tapped(text: &'static str) -> AbilityDef {
    AbilityDef::as_enters(text, ENTER_TAPPED[0])
}

/// "{cost}: Regenerate this creature." -- by far the most common printed
/// regeneration clause, and the one every self-regenerating creature shares.
/// Cards that regenerate something else, or pay something other than mana,
/// build the clause themselves around [`EffectDef::Regenerate`].
#[must_use]
pub const fn regenerate_self(text: &'static str, costs: &'static [AbilityCostDef]) -> AbilityDef {
    AbilityDef::activated(
        text,
        costs,
        EffectDef::Regenerate {
            object: EffectRecipientDef::Source,
        },
    )
}

/// A Circle of Protection: "the next time a <kind> source of your choice
/// would deal damage to you this turn, prevent that damage". The source is
/// chosen as the ability resolves rather than targeted, so it may be a spell
/// still on the stack, and the shield it leaves answers that source alone.
#[must_use]
pub const fn circle_of_protection(
    text: &'static str,
    costs: &'static [AbilityCostDef],
    source: ObjectPredicateDef,
) -> AbilityDef {
    AbilityDef::activated(
        text,
        costs,
        EffectDef::ChooseDamageSource {
            choice: ChoiceIndex::PRIMARY,
            chooser: EffectRecipientDef::Controller,
            object: source,
            then: &SHIELD_AGAINST_THE_CHOSEN_SOURCE,
        },
    )
}

static SHIELD_AGAINST_THE_CHOSEN_SOURCE: EffectDef = EffectDef::PreventNextDamageFromSource {
    object: EffectRecipientDef::Controller,
    source: EffectRecipientDef::ChosenPermanent(ChoiceIndex::PRIMARY),
    coverage: ShieldCoverageDef::All,
    gain_life: false,
};

/// The same shape as a Circle of Protection, for the printed cards that
/// choose a source once rather than repeatedly and vary what happens when
/// the shield fires.
#[must_use]
pub const fn shield_against_a_chosen_source(
    source: ObjectPredicateDef,
    then: &'static EffectDef,
) -> EffectDef {
    EffectDef::ChooseDamageSource {
        choice: ChoiceIndex::PRIMARY,
        chooser: EffectRecipientDef::Controller,
        object: source,
        then,
    }
}

/// Exalted. It is written as a keyword but defined as a triggered ability, so
/// each printed instance is its own clause and several on one board each
/// trigger -- which is why this returns an ordinary trigger rather than a
/// keyword. The permanent carrying it need not be a creature.
#[must_use]
pub const fn exalted() -> AbilityDef {
    AbilityDef::triggered(
        "Exalted (Whenever a creature you control attacks alone, that creature gets +1/+1 until \
         end of turn.)",
        TriggerEventDef::AttacksInGroup {
            attacker: ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            minimum_total: 1,
            maximum_total: Some(1),
        },
        EffectDef::Apply {
            recipient: EffectRecipientDef::TriggeringObject,
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(1),
                toughness: ValueDef::Constant(1),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )
}

/// Battalion. Like exalted it is a keyword defined as a triggered ability, so
/// it takes the effect its card prints rather than being one fixed clause.
#[must_use]
pub const fn battalion(text: &'static str, effect: EffectDef) -> AbilityDef {
    AbilityDef::triggered(text, BATTALION_EVENT, effect)
}

/// "This creature and at least two other creatures attack" -- three in all,
/// with this one among them.
pub const BATTALION_EVENT: TriggerEventDef = TriggerEventDef::AttacksInGroup {
    attacker: ObjectPredicateDef::Source,
    minimum_total: 3,
    maximum_total: None,
};

/// Unleash. The engine implements both halves from the keyword: an optional
/// +1/+1 counter offered as the permanent enters, and no blocking for as long
/// as it carries one.
#[must_use]
pub const fn unleash() -> AbilityDef {
    keyword(
        "Unleash (You may have this creature enter with a +1/+1 counter on it. It can't block as \
         long as it has a +1/+1 counter on it.)",
        KeywordAbility::Unleash,
    )
}

/// The optional entry clause unleash offers. It is a separate replacement
/// ability because the keyword itself carries no effect body.
#[must_use]
pub const fn unleash_counter() -> AbilityDef {
    AbilityDef::defined(
        "You may have this creature enter with a +1/+1 counter on it.",
        DeclarativeAbilityDef::Replacement(
            ReplacementAbilityDef::new()
                .with_event(ReplacementEventDef::SourceEntersBattlefield)
                .optional(),
        ),
        EffectDef::Replacement(ReplacementEffectDef::ModifyBattlefieldEntry(
            BattlefieldEntryModificationDef::AddCounters {
                kind: CounterKind::PlusOnePlusOne,
                amount: 1,
            },
        )),
    )
}

/// The reminder text every detain clause prints, so the cards agree on it.
pub const DETAIN_REMINDER: &str = "(Until your next turn, that permanent can't attack or block \
                                   and its activated abilities can't be activated.)";

/// Evolve. The comparison is against the source's own power and toughness at
/// the moment the creature enters, which is what makes a growing creature
/// stop evolving once it has outgrown what arrives.
#[must_use]
pub const fn evolve() -> AbilityDef {
    AbilityDef::triggered(
        "Evolve (Whenever a creature you control enters, if that creature has greater power or \
         toughness than this creature, put a +1/+1 counter on this creature.)",
        TriggerEventDef::ZoneChanged {
            object: ObjectPredicateDef::All(&EVOLVE_SUBJECT),
            from: None,
            to: Some(ZoneKind::Battlefield),
        },
        EffectDef::AddCounters {
            object: EffectRecipientDef::Source,
            kind: CounterKind::PlusOnePlusOne,
            amount: ValueDef::Constant(1),
        },
    )
}

static EVOLVE_SUBJECT: [ObjectPredicateDef; 4] = [
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
    // A creature does not evolve itself as it arrives.
    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
    ObjectPredicateDef::AnyOf(&EVOLVE_BIGGER),
];

static EVOLVE_BIGGER: [ObjectPredicateDef; 2] = [
    ObjectPredicateDef::PowerGreaterThan(ValueDef::SourcePower),
    ObjectPredicateDef::ToughnessGreaterThan(ValueDef::SourceToughness),
];

/// The printed static "this creature can't be blocked".
#[must_use]
pub const fn cannot_be_blocked(text: &'static str) -> AbilityDef {
    AbilityDef::static_ability(
        text,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::CannotBeBlocked,
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    )
}

/// A shared checkland-style entry clause backed by the general object-query
/// condition vocabulary.
#[must_use]
pub const fn check_land_enters(
    text: &'static str,
    land_types: &'static [BasicLandType],
) -> AbilityDef {
    enters_tapped_unless_you_control(text, ObjectPredicateDef::HasAnyBasicLandType(land_types))
}

/// An as-enters clause whose untapped branch depends on any controlled
/// battlefield object matching `object`.
#[must_use]
pub const fn enters_tapped_unless_you_control(
    text: &'static str,
    object: ObjectPredicateDef,
) -> AbilityDef {
    AbilityDef::as_enters(
        text,
        ReplacementEffectDef::Conditional {
            condition: ConditionDef::Exists(ObjectQueryDef {
                object,
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::You,
            }),
            if_true: &[],
            if_false: &ENTER_TAPPED,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::{
        banding, bloodrush, check_land_enters, double_strike, first_strike, flashback,
        flashback_for_card_mana_cost, flying, intimidate, overload, pain_land, shock_land_enters,
        tap_for,
    };
    use crate::card::{
        AbilityCostDef, AbilityCostList, AbilityCoverageDef, AbilityDef, AddManaEffectDef,
        AlternativeCastKindDef, AlternativeCastManaCostDef, BasicLandType, CardRules, ConditionDef,
        CostDef, DeclarativeAbilityDef, EffectDef, KeywordAbility, ManaColor, ManaCost,
        ObjectPredicateDef, PlayerRelation, ReplacementEffectDef, ZoneKind,
    };
    use crate::mana_cost;

    #[test]
    fn tap_for_builds_a_complete_executable_mana_ability() {
        let cases = [
            (ManaColor::White, "{T}: Add {W}."),
            (ManaColor::Blue, "{T}: Add {U}."),
            (ManaColor::Black, "{T}: Add {B}."),
            (ManaColor::Red, "{T}: Add {R}."),
            (ManaColor::Green, "{T}: Add {G}."),
            (ManaColor::Colorless, "{T}: Add {C}."),
        ];

        for (mana, text) in cases {
            let ability = tap_for(mana);
            assert_eq!(ability.text, text);
            assert_eq!(ability.coverage, AbilityCoverageDef::complete());
            assert!(ability.is_executable());
            assert!(matches!(
                ability.definition,
                DeclarativeAbilityDef::ActivatedMana(definition)
                    if definition.costs.as_slice() == [AbilityCostDef::TapSource]
            ));
            assert_eq!(
                ability.declarative_effect(),
                Some(EffectDef::AddMana(AddManaEffectDef::one(mana)))
            );
        }
    }

    #[test]
    fn pain_land_keeps_damage_on_only_the_colored_ability() {
        let abilities = pain_land(
            "{T}: Add {W} or {U}. This land deals 1 damage to you.",
            &[ManaColor::White, ManaColor::Blue],
        );

        assert_eq!(
            abilities[0].declarative_effect(),
            Some(EffectDef::AddMana(AddManaEffectDef::one(
                ManaColor::Colorless
            )))
        );
        assert_eq!(
            abilities[1].declarative_effect(),
            Some(EffectDef::AddMana(
                AddManaEffectDef::choice(&[ManaColor::White, ManaColor::Blue])
                    .with_damage_to_controller(1)
            ))
        );
    }

    #[test]
    fn common_land_entry_abilities_use_shared_conditions_and_costs() {
        let shock = shock_land_enters();
        assert!(matches!(
            shock.declarative_effect(),
            Some(EffectDef::Replacement(ReplacementEffectDef::OptionalPayment {
                payment,
                if_declined: [_],
                ..
            })) if payment.payer == PlayerRelation::You
                && payment.costs == [CostDef::PayLife(2)]
        ));

        let check = check_land_enters(
            "This land enters tapped unless you control a Mountain or a Plains.",
            &[BasicLandType::Mountain, BasicLandType::Plains],
        );
        assert!(matches!(
            check.declarative_effect(),
            Some(EffectDef::Replacement(ReplacementEffectDef::Conditional {
                condition: ConditionDef::Exists(query),
                ..
            })) if query.controller == PlayerRelation::You
                && matches!(
                    query.object,
                    ObjectPredicateDef::HasAnyBasicLandType(types)
                        if types == [BasicLandType::Mountain, BasicLandType::Plains]
                )
        ));
    }

    /// A card can print a keyword the engine only records. The distinction is
    /// a property of the coverage model rather than of any particular keyword,
    /// so the metadata-only case is built here instead of borrowing whichever
    /// keyword happens to be unimplemented today.
    #[test]
    fn keyword_presence_is_distinct_from_executable_keyword_support() {
        static RECORDED_ONLY: AbilityDef = AbilityDef::keyword("Shroud", KeywordAbility::Shroud)
            .with_coverage(AbilityCoverageDef::metadata_only(
                "Recorded for this test, not executed.",
            ));
        static KEYWORDS: [AbilityDef; 2] = [flying(), RECORDED_ONLY];
        let rules =
            CardRules::new_creature(ManaCost::default(), &[], 1, 1).with_abilities(&KEYWORDS);

        assert!(rules.has_keyword(KeywordAbility::Flying));
        assert!(rules.has_executable_keyword(KeywordAbility::Flying));
        assert!(rules.has_keyword(KeywordAbility::Shroud));
        assert!(!rules.has_executable_keyword(KeywordAbility::Shroud));
    }

    /// Banding is the one keyword the engine implements in part: blocking
    /// with it works, attacking in a band does not.
    #[test]
    fn banding_is_executable_but_only_partially_covered() {
        assert!(banding().is_executable());
        assert_eq!(
            banding().coverage.status,
            crate::card::ImplementationStatus::Partial
        );
    }

    #[test]
    fn common_combat_keywords_are_complete_definitions() {
        let cases = [
            (first_strike(), KeywordAbility::FirstStrike),
            (double_strike(), KeywordAbility::DoubleStrike),
            (intimidate(), KeywordAbility::Intimidate),
        ];

        for (ability, expected) in cases {
            assert_eq!(ability.coverage, AbilityCoverageDef::complete());
            assert!(ability.is_executable());
            assert_eq!(ability.definition, DeclarativeAbilityDef::Keyword(expected));
        }
        assert_eq!(intimidate().text, "Intimidate");
    }

    #[test]
    fn alternative_cast_helpers_own_costs_and_render_canonical_text() {
        let flashback = flashback(mana_cost!("{2}{U}"));
        let overload = overload(
            mana_cost!("{3}{R}{R}{R}"),
            "Deal 4 damage to each creature you don't control.",
            EffectDef::None,
        );

        assert!(matches!(
            flashback.definition,
            DeclarativeAbilityDef::AlternativeCast(definition)
                if definition.kind == AlternativeCastKindDef::Flashback
                    && definition.mana_cost
                        == AlternativeCastManaCostDef::Fixed(mana_cost!("{2}{U}"))
        ));
        assert_eq!(
            flashback.rules_text(),
            "Flashback {2}{U} (You may cast this card from your graveyard for its flashback cost. Then exile it.)",
        );
        assert!(matches!(
            overload.definition,
            DeclarativeAbilityDef::AlternativeCast(definition)
                if definition.kind == AlternativeCastKindDef::Overload
                    && definition.mana_cost
                        == AlternativeCastManaCostDef::Fixed(mana_cost!("{3}{R}{R}{R}"))
                    && definition.stack_text
                        == Some("Deal 4 damage to each creature you don't control.")
        ));
        assert_eq!(
            overload.rules_text(),
            "Overload {3}{R}{R}{R} (You may cast this spell for its overload cost. If you do, change \"target\" in its text to \"each.\")",
        );

        let granted = flashback_for_card_mana_cost();
        assert!(matches!(
            granted.definition,
            DeclarativeAbilityDef::AlternativeCast(definition)
                if definition.kind == AlternativeCastKindDef::Flashback
                    && definition.mana_cost == AlternativeCastManaCostDef::ThisCardManaCost
                    && definition.mana_cost.resolve(Some(mana_cost!("{1}{U}")))
                        == Some(mana_cost!("{1}{U}"))
        ));
        let DeclarativeAbilityDef::AlternativeCast(definition) = granted.definition else {
            unreachable!("the helper always builds an alternative-cast ability")
        };
        assert_eq!(definition.mana_cost.resolve(None), None);
    }

    #[test]
    fn bloodrush_owns_its_hand_zone_and_discard_procedure() {
        let effect = EffectDef::Special("Test Bloodrush effect");
        let text = "Bloodrush — {R}{G}, Discard this card: Test Bloodrush effect.";
        let ability = bloodrush(mana_cost!("{R}{G}"), text, &[], effect);
        let DeclarativeAbilityDef::Activated(definition) = ability.definition else {
            panic!("Bloodrush should be an activated ability")
        };

        assert_eq!(ability.text, text);
        assert_eq!(definition.source_zones, [ZoneKind::Hand]);
        assert_eq!(
            definition.costs,
            AbilityCostList::borrowed(&[
                AbilityCostDef::Mana(mana_cost!("{R}{G}")),
                AbilityCostDef::DiscardSource,
            ]),
            "inline and borrowed cost storage should compare by their costs",
        );
        assert_eq!(
            definition.costs.as_slice(),
            [
                AbilityCostDef::Mana(mana_cost!("{R}{G}")),
                AbilityCostDef::DiscardSource,
            ],
        );
        assert_eq!(ability.declarative_effect(), Some(effect));
    }
}
