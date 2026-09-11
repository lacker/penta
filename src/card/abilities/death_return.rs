// The executable clauses behind every effective undying and persist instance.
use super::{
    AbilityDef, ComparisonDef, CounterKind, EffectDef, EffectRecipientDef, ObjectPredicateDef,
    ObjectRefDef, TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
};

pub(crate) static UNDYING_TRIGGER: AbilityDef =
    death_return_trigger("Undying", CounterKind::PlusOnePlusOne);
pub(crate) static PERSIST_TRIGGER: AbilityDef =
    death_return_trigger("Persist", CounterKind::MinusOneMinusOne);

static RETURN_DYING_OBJECT: EffectDef = crate::card::actions::move_to_zone(
    EffectRecipientDef::object(ObjectRefDef::ZoneChangeResultOfTriggeringObject),
    ZoneKind::Battlefield,
    ZonePlacement::Top,
)
.as_effect();

const fn death_return_trigger(text: &'static str, counter: CounterKind) -> AbilityDef {
    AbilityDef::triggered_if(
        text,
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            Some(ZoneKind::Battlefield),
            Some(ZoneKind::Graveyard),
        ),
        match counter {
            CounterKind::PlusOnePlusOne => &TriggerConditionDef::SourceCounters {
                kind: CounterKind::PlusOnePlusOne,
                comparison: ComparisonDef::Equal,
                amount: 0,
            },
            _ => &TriggerConditionDef::SourceCounters {
                kind: CounterKind::MinusOneMinusOne,
                comparison: ComparisonDef::Equal,
                amount: 0,
            },
        },
        EffectDef::WithBattlefieldArrival {
            effect: &RETURN_DYING_OBJECT,
            arrival: crate::card::BattlefieldArrivalDef {
                counters: Some(crate::card::TokenCountersDef {
                    kind: counter,
                    amount: ValueDef::Constant(1),
                }),
                ..crate::card::BattlefieldArrivalDef::DEFAULT
            },
        },
    )
}
