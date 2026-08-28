/// Suspend N--cost (CR 702.62): a hand special action and the linked upkeep
/// and last-counter triggers that function while the card is in exile.
#[must_use]
pub const fn suspend(text: &'static str, time: u16, cost: &'static ManaCost) -> AbilityDef {
    keyword(
        text,
        KeywordAbility::Suspend(SuspendAbilityDef::fixed(time, cost)),
    )
}

/// Suspend X--cost, with the printed lower bound on X.
#[must_use]
pub const fn suspend_x(text: &'static str, cost: &'static ManaCost, minimum: u16) -> AbilityDef {
    keyword(
        text,
        KeywordAbility::Suspend(SuspendAbilityDef::chosen_x(cost, minimum)),
    )
}

/// Suspend granted to a card already in exile. Moving it and adding its time
/// counters remain ordinary effects, so this ability supplies only the rules
/// permission and exile triggers represented by the keyword.
pub(crate) static GRANTED_SUSPEND: AbilityDef = keyword(
    "Suspend",
    KeywordAbility::Suspend(SuspendAbilityDef::granted()),
);

const SUSPEND_TIME: CounterKind = CounterKind::named("time");
const SUSPENDED_CARD_PARTS: [ObjectPredicateDef; 2] = [
    ObjectPredicateDef::HasCounter(SUSPEND_TIME),
    ObjectPredicateDef::HasAbility(crate::card::AbilityPredicateDef::Suspend),
];
pub(crate) const SUSPENDED_CARD: ObjectPredicateDef =
    ObjectPredicateDef::All(&SUSPENDED_CARD_PARTS);
const SOURCE_HAS_SUSPEND: ObjectPredicateDef =
    ObjectPredicateDef::HasAbility(crate::card::AbilityPredicateDef::Suspend);
const SUSPEND_SOURCE_IS_SUSPENDED_PARTS: [TriggerConditionDef; 3] = [
    TriggerConditionDef::SourceInZone(ZoneKind::Exile),
    TriggerConditionDef::SourceCounters {
        kind: SUSPEND_TIME,
        comparison: ComparisonDef::Greater,
        amount: 0,
    },
    TriggerConditionDef::SourceMatches {
        object: SOURCE_HAS_SUSPEND,
    },
];
pub(crate) const SUSPEND_SOURCE_IS_SUSPENDED: TriggerConditionDef =
    TriggerConditionDef::All(&SUSPEND_SOURCE_IS_SUSPENDED_PARTS);
const SUSPEND_SOURCE_IS_EXILED: TriggerConditionDef =
    TriggerConditionDef::SourceInZone(ZoneKind::Exile);

pub(crate) const SUSPEND_REMOVE_TIME: EffectDef = EffectDef::RemoveCounters {
    object: EffectRecipientDef::Source,
    kind: SUSPEND_TIME,
    amount: ValueDef::Constant(1),
};

pub(crate) const SUSPEND_PLAY: EffectDef = EffectDef::MayPlayWithoutPaying(FreePlayDef {
    objects: ObjectSetDef::One(ObjectRefDef::Source),
    duration: FreePlayDurationDef::WhileResolving,
    mandatory: true,
    grants_haste: true,
});

pub(crate) static SUSPEND_UPKEEP_ABILITY: AbilityDef = AbilityDef::triggered_if(
    "At the beginning of your upkeep, if this card is suspended, remove a time counter from it.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    &SUSPEND_SOURCE_IS_SUSPENDED,
    SUSPEND_REMOVE_TIME,
)
.with_source_zones(&[ZoneKind::Exile]);

pub(crate) static SUSPEND_LAST_COUNTER_ABILITY: AbilityDef = AbilityDef::triggered_if(
    "When the last time counter is removed from this card, if it's exiled, play it without paying its mana cost if able.",
    TriggerEventDef::LastCounterRemoved {
        object: ObjectPredicateDef::Source,
        kind: SUSPEND_TIME,
    },
    &SUSPEND_SOURCE_IS_EXILED,
    SUSPEND_PLAY,
)
.with_source_zones(&[ZoneKind::Exile]);
