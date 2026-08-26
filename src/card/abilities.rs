//! Reusable constructors for common ability clauses.
//!
//! The functions here return identity-free [`AbilityDef`] values. A card part,
//! intrinsic rule, or grant site assigns identity when it attaches the clause.

use super::model::{
    AbilityCostDef, AbilityCostList, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    ActivationTimingDef, AddManaEffectDef, AlternativeCastKindDef, AppliedEffectDef,
    AppliedRuleDef, BandingQuality, BasicLandType, BattlefieldEntryModificationDef,
    CardChoiceSourceDef, CardType, CardTypeSet, ChoiceVisibilityDef, ChooseDef, ColorSet,
    ComparisonDef, ConditionDef, CopyExceptionsDef, CopyStackObjectDef, CostAdjustmentDef,
    CostAmountDef, CostModificationDef, CounterKind, DamageEventMatcherDef, DamagePreventionDef,
    DamageRecipientMatcherDef, DiscardFollowUpDef, DiscardSelectionDef, EffectDef,
    EffectPaymentDef, EffectRecipientDef, InstalledTriggerDef, InstalledTriggerLifetimeDef,
    KeywordAbility, ManaColor, ManaCost, ObjectChoiceBindingDef, ObjectCountConditionDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    OptionalAdditionalCostAbilityDef, OptionalAdditionalCostKindDef, PartitionItemsDef, PayOrDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef, ReplacementAbilityDef, ReplacementConditionDef,
    ReplacementEffectDef, ReplacementEventDef, ResolvedEffectDurationDef, SacrificedAmountDef,
    ScaledValueDef, SpellAdditionalCostDef, SpellCostConditionDef, SpellCostModificationDef,
    SpellResolutionDestinationDef, SplitIntoPilesDef, TopCardSelectionDef, TriggerConditionDef,
    TriggerEventDef, TurnStepDef, ValueDef, ZoneChangeEventMatcherDef, ZoneKind, ZonePlacement,
};
use crate::ids::{ObjectBindingIndex, ObjectSetBindingIndex, TargetIndex};

/// "If this card is in your opening hand, you may begin the game with it on
/// the battlefield." The pregame runtime supplies the source card; the move
/// remains an ordinary declarative zone-change effect.
#[must_use]
pub const fn begin_game_on_battlefield(text: &'static str) -> AbilityDef {
    AbilityDef::opening_hand(
        text,
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Source,
            from: Some(ZoneKind::Hand),
            zone: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            controller: None,
            arrival_effect: None,
            attachment: None,
            counters: None,
            tapped: false,
        },
    )
}

/// A source permanent's own enters-the-battlefield trigger.
#[must_use]
pub const fn enters_trigger(text: &'static str, effect: EffectDef) -> AbilityDef {
    enters_trigger_with_targets(text, &[], effect)
}

/// A targeted source permanent's own enters-the-battlefield trigger.
#[must_use]
pub const fn enters_trigger_with_targets(
    text: &'static str,
    targets: &'static [AbilityTargetDef],
    effect: EffectDef,
) -> AbilityDef {
    AbilityDef::triggered_with_targets(
        text,
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        targets,
        effect,
    )
}

/// A source permanent's own battlefield-to-graveyard trigger. The helper is
/// named for the common creature wording, but it also fits noncreature
/// permanents that trigger when put into a graveyard from the battlefield.
#[must_use]
pub const fn dies_trigger(text: &'static str, effect: EffectDef) -> AbilityDef {
    dies_trigger_matching(text, ObjectPredicateDef::Source, effect)
}

/// A targeted source permanent's own battlefield-to-graveyard trigger.
#[must_use]
pub const fn dies_trigger_with_targets(
    text: &'static str,
    targets: &'static [AbilityTargetDef],
    effect: EffectDef,
) -> AbilityDef {
    dies_trigger_matching_with_targets(text, ObjectPredicateDef::Source, targets, effect)
}

/// A battlefield-to-graveyard trigger for any permanent matching `object`.
/// Despite the common creature shorthand in the helper's name, this also
/// covers printed clauses that watch artifacts or other noncreature
/// permanents go to a graveyard from the battlefield.
#[must_use]
pub const fn dies_trigger_matching(
    text: &'static str,
    object: ObjectPredicateDef,
    effect: EffectDef,
) -> AbilityDef {
    dies_trigger_matching_with_targets(text, object, &[], effect)
}

/// A targeted battlefield-to-graveyard trigger for any matching permanent.
#[must_use]
pub const fn dies_trigger_matching_with_targets(
    text: &'static str,
    object: ObjectPredicateDef,
    targets: &'static [AbilityTargetDef],
    effect: EffectDef,
) -> AbilityDef {
    AbilityDef::triggered_with_targets(
        text,
        TriggerEventDef::zone_changed(
            object,
            Some(ZoneKind::Battlefield),
            Some(ZoneKind::Graveyard),
        ),
        targets,
        effect,
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

/// Myriad's reusable attack trigger. In the current two-player engine there
/// is no opponent other than the defending player, so its explicit procedure
/// resolves without creating a token.
#[must_use]
pub const fn myriad() -> AbilityDef {
    AbilityDef::triggered(
        "Myriad (Whenever this creature attacks, for each opponent other than defending player, you may create a token copy that's tapped and attacking that player or a planeswalker they control. Exile the tokens at end of combat.)",
        TriggerEventDef::attacks(ObjectPredicateDef::Source),
        EffectDef::CreateMyriadTokens,
    )
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
    keyword("Banding", KeywordAbility::Banding)
}

/// "Bands with other X", the banding variant CR 702.21j narrows to a quality.
/// The text names the quality, because it is what the printed clause says.
#[must_use]
pub const fn bands_with_other(quality: BandingQuality) -> AbilityDef {
    let text = match quality {
        BandingQuality::LegendaryCreatures => "Bands with other legendary creatures",
        BandingQuality::WolvesOfTheHunt => "Bands with other creatures named Wolves of the Hunt",
    };
    keyword(text, KeywordAbility::BandsWithOther(quality))
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

/// Devoid (CR 702.114): "This card has no color." A characteristic-defining
/// ability rather than a behaviour, so the colourlessness comes from the
/// card's empty printed colour set and this clause is what names it.
#[must_use]
pub const fn devoid() -> AbilityDef {
    keyword("Devoid (This card has no color.)", KeywordAbility::Devoid)
}

/// Split second (CR 702.19): while this spell is on the stack, nobody may
/// cast a spell or activate anything that is not a mana ability.
#[must_use]
pub const fn split_second() -> AbilityDef {
    keyword(
        "Split second (As long as this spell is on the stack, players can't cast spells or \
         activate abilities that aren't mana abilities.)",
        KeywordAbility::SplitSecond,
    )
}

/// Compleated (CR 702.150): this permanent enters with two fewer loyalty
/// counters if life was paid for a Phyrexian mana symbol while casting it.
#[must_use]
pub const fn compleated(text: &'static str) -> AbilityDef {
    keyword(text, KeywordAbility::Compleated)
}

/// Infect (CR 702.90): "This creature deals damage to creatures in the form
/// of -1/-1 counters and to players in the form of poison counters."
#[must_use]
pub const fn infect() -> AbilityDef {
    keyword(
        "Infect (This creature deals damage to creatures in the form of -1/-1 counters and to players in the form of poison counters.)",
        KeywordAbility::Infect,
    )
}

#[must_use]
pub const fn shroud() -> AbilityDef {
    keyword("Shroud", KeywordAbility::Shroud)
}

#[must_use]
pub const fn intimidate() -> AbilityDef {
    keyword("Intimidate", KeywordAbility::Intimidate)
}

/// "Shadow (This creature can block or be blocked by only creatures with
/// shadow.)"
#[must_use]
pub const fn shadow() -> AbilityDef {
    keyword(
        "Shadow (This creature can block or be blocked by only creatures with shadow.)",
        KeywordAbility::Shadow,
    )
}

/// "Menace (This creature can't be blocked except by two or more
/// creatures.)"
#[must_use]
pub const fn menace() -> AbilityDef {
    keyword(
        "Menace (This creature can't be blocked except by two or more creatures.)",
        KeywordAbility::Menace,
    )
}

#[must_use]
pub const fn undying() -> AbilityDef {
    keyword("Undying", KeywordAbility::Undying)
}

/// "Persist (When this creature dies, if it had no -1/-1 counters on it,
/// return it to the battlefield under its owner's control with a -1/-1
/// counter on it.)"
#[must_use]
pub const fn persist() -> AbilityDef {
    keyword(
        "Persist (When this creature dies, if it had no -1/-1 counters on it, return it to the \
         battlefield under its owner's control with a -1/-1 counter on it.)",
        KeywordAbility::Persist,
    )
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

/// A noncreature spell its controller cast, which is what prowess watches.
static A_NONCREATURE_SPELL_YOU_CAST: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::NoncreatureSpell,
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

/// "Prowess (Whenever you cast a noncreature spell, this creature gets +1/+1
/// until end of turn.)"
///
/// Written out as the triggered ability it abbreviates rather than as a
/// keyword: nothing in the rules reads "has prowess" the way combat reads
/// flying, so the clause is the whole of it.
#[must_use]
pub const fn prowess() -> AbilityDef {
    AbilityDef::triggered(
        "Prowess (Whenever you cast a noncreature spell, this creature gets +1/+1 until end of \
         turn.)",
        TriggerEventDef::SpellCast(A_NONCREATURE_SPELL_YOU_CAST),
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
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

/// "Whenever this creature deals damage to a player, that player gets N
/// poison counters." Every printed form of this watches damage of any kind,
/// not only combat damage, and the card supplies its own reminder text.
#[must_use]
pub const fn poisonous_damage(amount: i32, text: &'static str) -> AbilityDef {
    AbilityDef::triggered(
        text,
        TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Any),
        EffectDef::AddPlayerCounters {
            recipient: EffectRecipientDef::EventPlayer,
            kind: CounterKind::Poison,
            amount: ValueDef::Constant(amount),
        },
    )
}

/// The granted protection and the self-retention exception for each printed
/// Ward. `EffectDef::Sequence` holds its clauses by reference, and a sequence
/// built from a parameter cannot be promoted to `'static`, so the five are
/// named here rather than rebuilt per card.
static WARD_PROTECTIONS: [AbilityDef; 5] = [
    protection_from_color(ManaColor::White),
    protection_from_color(ManaColor::Blue),
    protection_from_color(ManaColor::Black),
    protection_from_color(ManaColor::Red),
    protection_from_color(ManaColor::Green),
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

/// One Ward Aura's whole granted clause -- the Alpha cycle of Auras named
/// "<colour> Ward", which have nothing to do with the ward keyword. The card
/// supplies its own rules text, which names the colour and prints the
/// exception in one sentence.
///
/// # Panics
///
/// Panics when `color` is colorless, which no Ward Aura is printed with.
#[must_use]
pub const fn ward_aura_protection(color: ManaColor, text: &'static str) -> AbilityDef {
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
/// Foretell: an alternative cast taken from exile, where the special action
/// that costs {2} put the card. The reminder text is generated from the cost
/// the card prints, the same way flashback's is.
#[must_use]
pub const fn foretell(mana_cost: ManaCost) -> AbilityDef {
    AbilityDef::alternative_cast(
        mana_cost,
        AlternativeCastKindDef::Foretell,
        None,
        EffectDef::None,
    )
}

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

/// Evoke: an alternative cost that comes due once the creature has arrived.
/// The permission is the alternative cast; the sacrifice is a separate
/// triggered ability, because it happens after the spell has already
/// resolved and the creature's own enters triggers have gone on the stack
/// alongside it.
#[must_use]
pub const fn evoke_sacrifice(text: &'static str) -> AbilityDef {
    AbilityDef::triggered_if(
        text,
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        &EVOKED,
        EffectDef::Sacrifice {
            object: EffectRecipientDef::Source,
        },
    )
}

static EVOKED: TriggerConditionDef =
    TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::AlternativeCost);

/// A kicker: "Kicker {cost} (You may pay an additional {cost} as you cast
/// this spell.)", together with what the spell does when it was paid.
///
/// `mana_cost` is the whole kicked total, not the surcharge -- a kicked spell
/// is a spell cast for more mana with different instructions, which is what
/// an alternative cast already models. The caller supplies the printed
/// reminder text, because that text names the surcharge instead.
#[must_use]
pub const fn kicker(
    mana_cost: ManaCost,
    stack_text: &'static str,
    targets: &'static [AbilityTargetDef],
    effect: EffectDef,
) -> AbilityDef {
    AbilityDef::alternative_cast_with_targets(
        mana_cost,
        AlternativeCastKindDef::Kicked,
        Some(stack_text),
        targets,
        effect,
    )
}

/// Echo (CR 702.29): "At the beginning of your upkeep, if this came under
/// your control since the beginning of your last upkeep, sacrifice it unless
/// you pay its echo cost."
///
/// The intervening-if is what makes the cost come due exactly once. The
/// caller supplies the printed text, because the reminder repeats the cost.
#[must_use]
pub const fn echo(text: &'static str, cost: ManaCost) -> AbilityDef {
    AbilityDef::triggered_if(
        text,
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        &TriggerConditionDef::SourceArrivedSinceControllersLastUpkeep,
        EffectDef::PayOr(PayOrDef::unless_mana(cost, &SACRIFICE_SOURCE)),
    )
}

static SACRIFICE_SOURCE: EffectDef = EffectDef::Sacrifice {
    object: EffectRecipientDef::Source,
};

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

/// Connive, on the permanent doing it: draw a card, then discard a card, and
/// take a +1/+1 counter for the discard if what went was not a land.
///
/// The counter count is how many of the discarded cards were nonland rather
/// than a yes-or-no, which for a connive-one is the same 0 or 1 the printed
/// clause means and which is already what connive N says for larger numbers.
#[must_use]
pub const fn connive() -> EffectDef {
    EffectDef::Sequence(&CONNIVE_STEPS)
}

static CONNIVE_STEPS: [EffectDef; 2] = [
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
    EffectDef::Discard {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
        selection: DiscardSelectionDef::RecipientChooses,
        then: Some(CONNIVE_COUNTERS),
    },
];

static CONNIVE_COUNTERS: DiscardFollowUpDef = DiscardFollowUpDef {
    counted: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
    bound: None,
    effect: &EffectDef::AddCounters {
        object: EffectRecipientDef::Source,
        kind: CounterKind::PlusOnePlusOne,
        amount: ValueDef::MatchedCount,
    },
};

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

/// An unconditional battlefield-entry replacement shared by permanents that
/// enter tapped.
#[must_use]
pub const fn enters_tapped(text: &'static str) -> AbilityDef {
    AbilityDef::as_enters(text, ENTER_TAPPED[0])
}

/// "Cycling {cost} ({cost}, Discard this card: Draw a card.)"
///
/// Cycling is an activated ability that exists only while the card is in
/// hand, which is what keeps it off the battlefield version of the same
/// permanent. Nothing else about it is special: the discard is a cost, so it
/// happens on activation rather than on resolution, and the draw is what goes
/// on the stack. The caller supplies the printed text because the reminder
/// repeats the cost.
#[must_use]
pub const fn cycling(text: &'static str, cost: ManaCost) -> AbilityDef {
    AbilityDef::activated_with_cost_list_and_targets(
        text,
        AbilityCostList::two(AbilityCostDef::Mana(cost), AbilityCostDef::DiscardSource),
        &[],
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )
    .with_source_zones(&[ZoneKind::Hand])
}

/// "<Type>cycling {cost}" -- the same ability as [`cycling`], except that
/// what it buys is a search rather than a draw. Failing to find is allowed,
/// so the minimum is zero: the discard has already been paid either way.
#[must_use]
pub const fn typecycling(
    text: &'static str,
    cost: ManaCost,
    object: ObjectPredicateDef,
) -> AbilityDef {
    AbilityDef::activated_with_cost_list_and_targets(
        text,
        AbilityCostList::two(AbilityCostDef::Mana(cost), AbilityCostDef::DiscardSource),
        &[],
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object,
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: true,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )
    .with_source_zones(&[ZoneKind::Hand])
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
            unchosen: None,
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
        unchosen: None,
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
    placement: ZonePlacement::Top,
};
static COUNTER_PRIMARY_TARGET_TO_EXILE: EffectDef = EffectDef::Counter {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    zone: ZoneKind::Exile,
    placement: ZonePlacement::Top,
};
static COUNTER_TRIGGERING_SPELL: EffectDef = EffectDef::Counter {
    object: EffectRecipientDef::TriggeringObject,
    zone: ZoneKind::Graveyard,
    placement: ZonePlacement::Top,
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
        condition: None,
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

/// The printed static "this creature can't be blocked".
#[must_use]
pub const fn cannot_be_blocked(text: &'static str) -> AbilityDef {
    AbilityDef::static_ability(
        text,
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
        },
    )
}

include!("abilities/cost_modifications.rs");
include!("abilities/keyword_actions.rs");
include!("abilities/sagas.rs");
include!("abilities/lands.rs");
include!("abilities/death_triggers.rs");
include!("abilities/repeated_clauses.rs");
include!("abilities/keyword_mechanics.rs");
include!("abilities/convoke_buyback.rs");
include!("abilities/attachment.rs");
include!("abilities/named_cards.rs");
include!("abilities/tests.rs");
