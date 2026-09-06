fn trigger_event_object_zone(event: TriggerEventDef) -> Option<ZoneKind> {
    match event {
        // Every alternative has to agree, or the ability's targets would read
        // an object from one zone on one path and another zone on the next.
        TriggerEventDef::AnyOf(events) => {
            let mut zones = events.iter().map(|event| trigger_event_object_zone(*event));
            let first = zones.next()?;
            zones.all(|zone| zone == first).then_some(first)?
        }
        // The condition narrows when the event counts, not what it names,
        // so the object comes from the event it wraps.
        TriggerEventDef::While { event, .. } => trigger_event_object_zone(*event),
        TriggerEventDef::ZoneChanged(matcher) => matcher.to,
        TriggerEventDef::Tapped(_)
        | TriggerEventDef::CumulativeUpkeepPaid { .. }
        | TriggerEventDef::CumulativeUpkeepNotPaid
        | TriggerEventDef::Attacks(_)
        | TriggerEventDef::Exerted(_)
        | TriggerEventDef::OptionalEffectTaken(_)
        // The predicate names the source of the clause, which is still where
        // it was when it sacrificed something.
        | TriggerEventDef::SacrificePerformed(_)
        | TriggerEventDef::Sacrificed { .. }
        | TriggerEventDef::AttackDeclared { .. }
        | TriggerEventDef::CardsExiled { .. }
        | TriggerEventDef::AttacksAndIsNotBlocked { .. }
        | TriggerEventDef::UnblockedAttackersDeclared { .. }
        // The event is the step rather than any creature in it, so nothing
        // here names an object in a zone.
        | TriggerEventDef::CombatDamageDealtToPlayers { .. }
        // The dead are read as they last stood on the battlefield.
        | TriggerEventDef::ObjectsDied { .. }
        // A token is created as it enters, so the batch is read there.
        | TriggerEventDef::TokensCreated { .. }
        | TriggerEventDef::BecomesBlocked(_)
        | TriggerEventDef::BlocksOrBecomesBlockedBy { .. }
        | TriggerEventDef::Blocks { .. }
        | TriggerEventDef::BecomesBlockedBy { .. }
        | TriggerEventDef::CountersPlaced { .. }
        | TriggerEventDef::Transforms(_) => Some(ZoneKind::Battlefield),
        // The named object is the spell or ability rather than what it
        // points at.
        TriggerEventDef::StackObject(_) => Some(ZoneKind::Stack),
        // The cycled card is in the graveyard by the time the trigger goes
        // on the stack, but nothing reads it as an object, so it names no
        // zone at all.
        TriggerEventDef::CommittedCrime(_)
        | TriggerEventDef::CoinFlipWon(_)
        | TriggerEventDef::CoinFlipLost(_)
        | TriggerEventDef::BecomesLevel(_)
        | TriggerEventDef::Cycled
        | TriggerEventDef::DoorUnlocked
        | TriggerEventDef::StepBegins { .. }
        | TriggerEventDef::LandPlayed { .. }
        | TriggerEventDef::DamageDealt(_)
        | TriggerEventDef::CountersRemoved { .. }
        | TriggerEventDef::LastCounterRemoved { .. }
        | TriggerEventDef::StateCondition
        | TriggerEventDef::LifeGained(_)
        | TriggerEventDef::BecomesMonarch(_)
        | TriggerEventDef::DrewCard(_)
        // The card is already in a graveyard and nothing reads it, so the
        // event names no object at all.
        | TriggerEventDef::Discarded(_)
        | TriggerEventDef::DiscardedCards(_) => None,
    }
}
