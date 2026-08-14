//! Reusable constructors for common ability clauses.
//!
//! The functions here return identity-free [`AbilityDef`] values. A card part,
//! intrinsic rule, or grant site assigns identity when it attaches the clause.

use super::model::{
    AbilityCostDef, AbilityCostList, AbilityCoverageDef, AbilityDef, AbilityTargetDef,
    AbilityTargetPredicate, ActivationTimingDef, AddManaEffectDef, AlternativeCastKindDef,
    AppliedEffectDef, AppliedRuleDef, BasicLandType, BattlefieldEntryModificationDef, CardType,
    ChoiceVisibilityDef, ChooseDef, ConditionDef, CounterKind, DamageEventMatcherDef,
    DamagePreventionDef, DamageRecipientMatcherDef, EffectDef, EffectPaymentDef,
    EffectRecipientDef, KeywordAbility, ManaColor, ManaCost, ObjectChoiceBindingDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, PartitionItemsDef, PayOrDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef, ReplacementAbilityDef, ReplacementEffectDef,
    ReplacementEventDef, ResolvedEffectDurationDef, ScaledValueDef, SplitIntoPilesDef,
    TriggerEventDef, TurnStepDef, ValueDef, ZoneKind,
};
use crate::ids::{ObjectBindingIndex, ObjectSetBindingIndex, TargetIndex};

/// The target an "Enchant creature" Aura spell chooses.
pub static ENCHANT_CREATURE_TARGET: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::HasType(CardType::Creature),
    )];

/// The target an "Enchant artifact" Aura spell chooses.
pub static ENCHANT_ARTIFACT_TARGET: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::HasType(CardType::Artifact),
    )];

/// The target an "Enchant enchantment" Aura spell chooses.
pub static ENCHANT_ENCHANTMENT_TARGET: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::HasType(CardType::Enchantment),
    )];

/// An Aura's "at the beginning of the upkeep of enchanted <thing>'s
/// controller" trigger. The host's controller is the one whose upkeep this
/// watches, which is not the Aura's controller once a host changes hands.
#[must_use]
pub const fn enchanted_controller_upkeep(text: &'static str, effect: EffectDef) -> AbilityDef {
    AbilityDef::triggered(
        text,
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::ControllerOfAttachedPermanent,
        },
        effect,
    )
}

/// The target an "Enchant land" Aura spell chooses.
pub static ENCHANT_LAND_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Land),
)];

/// An Aura's own spell clause: it targets what it will enchant, and attaching
/// is what the spell does when it resolves. Every Aura prints one, so it
/// belongs here rather than once per set module.
#[must_use]
pub const fn aura_spell(text: &'static str, targets: &'static [AbilityTargetDef]) -> AbilityDef {
    AbilityDef::spell_with_targets(
        text,
        targets,
        EffectDef::Attach {
            object: EffectRecipientDef::Target(crate::ids::TargetIndex::PRIMARY),
        },
    )
}

/// "Attacks each combat if able." Cards state this in their own words rather
/// than as a printed keyword, so the text is supplied by the caller.
#[must_use]
pub const fn attacks_each_combat_if_able(text: &'static str) -> AbilityDef {
    keyword(text, KeywordAbility::AttacksEachCombatIfAble)
}
const ENTER_TAPPED: [ReplacementEffectDef; 1] = [ReplacementEffectDef::ModifyBattlefieldEntry(
    BattlefieldEntryModificationDef::Tapped,
)];
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
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Scaled(scale),
                ValueDef::Scaled(scale),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
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

/// "Whenever this creature deals damage to a player, that player gets N
/// poison counters." Every printed form of this watches damage of any kind,
/// not only combat damage, and the card supplies its own reminder text.
#[must_use]
pub const fn poisonous_damage(amount: i32, text: &'static str) -> AbilityDef {
    AbilityDef::triggered(
        text,
        TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Any),
        EffectDef::AddPoisonCounters {
            recipient: EffectRecipientDef::EventPlayer,
            amount: ValueDef::Constant(amount),
        },
    )
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
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::AttachedPermanent,
            effect: AppliedEffectDef::add_ability(&WARD_PROTECTIONS[color]),
        },
        // Without this the Aura would be an illegal attachment the moment it
        // granted protection from its own colour, which is exactly what the
        // printed exception exists to stop.
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::RemainsAttachedThroughProtection),
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

/// Populate's copy step, made once its choice has landed.
static POPULATE_COPY: EffectDef = EffectDef::CreateTokenCopyOf {
    object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
};

/// Populate: choose a creature token you control, then create a copy of it.
/// The choice is not a target -- nothing about it is checked again -- and a
/// player with no creature tokens simply does nothing.
#[must_use]
pub const fn populate() -> EffectDef {
    EffectDef::Choose(ChooseDef {
        binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
        chooser: PlayerRefDef::EffectController,
        candidates: ObjectSetDef::Query(ObjectQueryDef::controlled_by(
            ObjectPredicateDef::All(&POPULATE_CANDIDATE),
            &[ZoneKind::Battlefield],
            PlayerSetDef::One(PlayerRefDef::EffectController),
        )),
        exclude: None,
        minimum: 1,
        maximum: 1,
        visibility: ChoiceVisibilityDef::Public,
        then: &POPULATE_COPY,
    })
}

static POPULATE_CANDIDATE: [ObjectPredicateDef; 2] = [
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::Token,
];

/// The target an equip ability chooses: a creature its controller controls.
static EQUIP_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

/// The target a fortify ability chooses: a land its controller controls.
static FORTIFY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Land),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

/// The optional target in a reconfigure activation. Choosing none is the
/// unattach branch, which the action generator offers only while attached.
static RECONFIGURE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
    1,
)];

/// Equip. Attaching is what the ability does, and it is sorcery-speed, which
/// is the whole difference between this and an Aura arriving from the stack.
/// Reminder text carries the cost, so each card supplies its own literal.
#[must_use]
pub const fn equip(mana_cost: ManaCost, text: &'static str) -> AbilityDef {
    AbilityDef::activated_with_cost_list_and_targets(
        text,
        AbilityCostList::one(AbilityCostDef::Mana(mana_cost)),
        &EQUIP_TARGET,
        EffectDef::Attach {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )
    .with_activation_timing(ActivationTimingDef::SorcerySpeed)
}

/// Fortify. Like equip, this is a sorcery-speed attachment activation; the
/// shared attachment relation supplies Fortification's distinct land-host
/// legality and state-based unattach behavior.
#[must_use]
pub const fn fortify(mana_cost: ManaCost, text: &'static str) -> AbilityDef {
    AbilityDef::activated_with_cost_list_and_targets(
        text,
        AbilityCostList::one(AbilityCostDef::Mana(mana_cost)),
        &FORTIFY_TARGET,
        EffectDef::Attach {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )
    .with_activation_timing(ActivationTimingDef::SorcerySpeed)
}

/// Living weapon's enter-the-battlefield trigger. The effect's entry
/// continuation attaches the Equipment to the token's exact resulting
/// permanent before state-based actions are checked.
#[must_use]
pub const fn living_weapon(token: crate::CardDefinitionId) -> AbilityDef {
    AbilityDef::triggered(
        "Living weapon (When this Equipment enters, create a 0/0 black Phyrexian Germ creature token, then attach this to it.)",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::CreateAttachedToken { token },
    )
}

/// Reconfigure's paired sorcery-speed attachment procedures.
#[must_use]
pub const fn reconfigure(mana_cost: ManaCost, text: &'static str) -> AbilityDef {
    AbilityDef::activated_with_cost_list_and_targets(
        text,
        AbilityCostList::one(AbilityCostDef::Mana(mana_cost)),
        &RECONFIGURE_TARGET,
        EffectDef::Reconfigure {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )
    .with_activation_timing(ActivationTimingDef::SorcerySpeed)
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
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered),
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
        ReplacementEffectDef::PayOr {
            payment: EffectPaymentDef::life(PlayerSetDef::Related(PlayerRelation::You), 2),
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
        EffectDef::Choose(ChooseDef {
            binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::Query(ObjectQueryDef::new(
                source,
                &[ZoneKind::Battlefield, ZoneKind::Stack],
            )),
            exclude: Some(ObjectRefDef::ResolvingObject),
            minimum: 1,
            maximum: 1,
            visibility: ChoiceVisibilityDef::Public,
            then: &SHIELD_AGAINST_THE_CHOSEN_SOURCE,
        }),
    )
}

static SHIELD_AGAINST_THE_CHOSEN_SOURCE: EffectDef = EffectDef::PreventDamage {
    prevention: DamagePreventionDef::events(
        DamageEventMatcherDef {
            recipient: DamageRecipientMatcherDef::Recipients(EffectRecipientDef::Controller),
            ..DamageEventMatcherDef::from(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY))
        },
        1,
    ),
    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
};

/// The same shape as a Circle of Protection, for the printed cards that
/// choose a source once rather than repeatedly and vary what happens when
/// the shield fires.
#[must_use]
pub const fn shield_against_a_chosen_source(
    source: ObjectPredicateDef,
    then: &'static EffectDef,
) -> EffectDef {
    EffectDef::Choose(ChooseDef {
        binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
        chooser: PlayerRefDef::EffectController,
        candidates: ObjectSetDef::Query(ObjectQueryDef::new(
            source,
            &[ZoneKind::Battlefield, ZoneKind::Stack],
        )),
        exclude: Some(ObjectRefDef::ResolvingObject),
        minimum: 1,
        maximum: 1,
        visibility: ChoiceVisibilityDef::Public,
        then,
    })
}

static COUNTER_PRIMARY_TARGET: EffectDef = EffectDef::Counter {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    zone: ZoneKind::Graveyard,
};
static COUNTER_PRIMARY_TARGET_TO_EXILE: EffectDef = EffectDef::Counter {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    zone: ZoneKind::Exile,
};
static COUNTER_TRIGGERING_SPELL: EffectDef = EffectDef::Counter {
    object: EffectRecipientDef::TriggeringObject,
    zone: ZoneKind::Graveyard,
};

const fn pay_or_counter(
    payer: PlayerRefDef,
    amount: ValueDef,
    otherwise: &'static EffectDef,
) -> EffectDef {
    EffectDef::PayOr(PayOrDef {
        payment: EffectPaymentDef::generic_mana(PlayerSetDef::One(payer), amount),
        if_paid: None,
        otherwise: Some(otherwise),
        visibility: ChoiceVisibilityDef::Public,
    })
}

/// Counter the primary targeted spell unless its controller pays generic mana.
#[must_use]
pub const fn counter_target_unless_paid(amount: ValueDef) -> EffectDef {
    pay_or_counter(
        PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
        amount,
        &COUNTER_PRIMARY_TARGET,
    )
}

/// Counter the primary targeted spell into exile unless its controller pays.
#[must_use]
pub const fn counter_target_to_exile_unless_paid(amount: ValueDef) -> EffectDef {
    pay_or_counter(
        PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
        amount,
        &COUNTER_PRIMARY_TARGET_TO_EXILE,
    )
}

/// Counter the spell that caused a trigger unless its controller pays.
#[must_use]
pub const fn counter_triggering_spell_unless_paid(amount: ValueDef) -> EffectDef {
    pay_or_counter(
        PlayerRefDef::ControllerOf(ObjectRefDef::TriggeringObject),
        amount,
        &COUNTER_TRIGGERING_SPELL,
    )
}

const CHOSEN_PILE_BINDING: ObjectSetBindingIndex = ObjectSetBindingIndex::PRIMARY;
const UNCHOSEN_PILE_BINDING: ObjectSetBindingIndex = ObjectSetBindingIndex::new(1);

/// The pile selected by the chooser in [`split_top_of_library_into_piles`].
pub const CHOSEN_PILE: EffectRecipientDef =
    EffectRecipientDef::objects(ObjectSetDef::Binding(CHOSEN_PILE_BINDING));

/// The pile declined by the chooser in [`split_top_of_library_into_piles`].
pub const UNCHOSEN_PILE: EffectRecipientDef =
    EffectRecipientDef::objects(ObjectSetDef::Binding(UNCHOSEN_PILE_BINDING));

/// Reveal cards from the effect controller's library, let an opponent divide
/// them, and let the controller choose a pile before continuing.
#[must_use]
pub const fn split_top_of_library_into_piles(
    count: ValueDef,
    then: &'static EffectDef,
) -> EffectDef {
    EffectDef::SplitIntoPiles(SplitIntoPilesDef {
        items: PartitionItemsDef::TopOfLibrary {
            player: PlayerRefDef::EffectController,
            count,
        },
        divider: PlayerSetDef::Related(PlayerRelation::Opponent),
        chooser: PlayerSetDef::One(PlayerRefDef::EffectController),
        chosen: CHOSEN_PILE_BINDING,
        unchosen: UNCHOSEN_PILE_BINDING,
        then,
    })
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
        TriggerEventDef::attacks_in_declaration(
            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            1,
            Some(1),
        ),
        EffectDef::Apply {
            recipient: EffectRecipientDef::TriggeringObject,
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
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
pub const BATTALION_EVENT: TriggerEventDef =
    TriggerEventDef::attacks_in_declaration(ObjectPredicateDef::Source, 3, None);

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
    AbilityDef::defined_replacement(
        "You may have this creature enter with a +1/+1 counter on it.",
        ReplacementAbilityDef::new()
            .with_event(ReplacementEventDef::SourceEntersBattlefield)
            .optional(),
        ReplacementEffectDef::ModifyBattlefieldEntry(
            BattlefieldEntryModificationDef::AddCounters {
                kind: CounterKind::PlusOnePlusOne,
                amount: 1,
            },
        ),
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
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::All(&EVOLVE_SUBJECT),
            None,
            Some(ZoneKind::Battlefield),
        ),
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
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlocked),
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
            condition: ConditionDef::Exists(ObjectQueryDef::matching(
                object,
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            )),
            if_true: &[],
            if_false: &ENTER_TAPPED,
        },
    )
}

include!("abilities/tests.rs");
