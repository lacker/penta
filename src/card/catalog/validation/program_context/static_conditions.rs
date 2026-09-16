fn static_trigger_condition_supported(condition: TriggerConditionDef) -> bool {
    match condition {
        TriggerConditionDef::All(conditions) | TriggerConditionDef::AnyOf(conditions) => conditions
            .iter()
            .copied()
            .all(static_trigger_condition_supported),
        TriggerConditionDef::Not(condition) => static_trigger_condition_supported(*condition),
        TriggerConditionDef::ObjectCount { query, .. } => static_query_supported(query),
        TriggerConditionDef::ObjectSetCount(condition) => {
            static_object_set_supported(*condition.objects)
                && condition
                    .predicate
                    .filter
                    .is_none_or(|filter| static_object_predicate_supported(filter.predicate()))
        }
        TriggerConditionDef::ActivePlayer(relation)
        // Life totals are plain state, so reading them cannot re-enter the
        // static walk the way a power comparison would.
        | TriggerConditionDef::PlayerHasMostLife(relation)
        | TriggerConditionDef::SpellsCastThisTurn {
            player: relation, ..
        }
        | TriggerConditionDef::SpellsCastLastTurn {
            player: relation, ..
        } => static_player_relation_supported(relation),
        TriggerConditionDef::SourceActivationsThisTurn { .. }
        // Both count something about a resolution, which a static walk is
        // not one of.
        | TriggerConditionDef::SourceAbilityUsedThisTurn
        | TriggerConditionDef::SourceResolutionsThisTurn { .. }
        | TriggerConditionDef::TargetMatches { .. }
        // And this reads a binding, which only a resolution has.
        | TriggerConditionDef::BoundObjectMatches { .. }
        // And this reads what a resolution sacrificed, which a static walk
        // has no event to ask.
        | TriggerConditionDef::SacrificedObjectMatches(_)
        | TriggerConditionDef::ControlsGreatestPowerCreature => false,
        TriggerConditionDef::SourceMatches { object }
        | TriggerConditionDef::AttachedPermanentMatches { object } => {
            static_object_predicate_supported(object)
        }
        TriggerConditionDef::ControlsCreaturesWithDifferentPowers(_)
        | TriggerConditionDef::ControllerHadPermanentLeaveThisTurn
        | TriggerConditionDef::ControllerHadCardLeaveGraveyardThisTurn
        | TriggerConditionDef::ControllerHasEnduringStory
        | TriggerConditionDef::ControllerHasCitysBlessing
        | TriggerConditionDef::ControllerGainedLifeThisTurn
        | TriggerConditionDef::OpponentLostLifeThisTurn
        | TriggerConditionDef::CreatureDiedThisTurn
        | TriggerConditionDef::SourceArrivedSinceControllersLastUpkeep
        | TriggerConditionDef::SourceOnBattlefield
        | TriggerConditionDef::SourceHasDesignation(_)
        | TriggerConditionDef::SourceInZone(_)
        | TriggerConditionDef::SourceUntapped
        | TriggerConditionDef::SourceIsPaired
        | TriggerConditionDef::SourceClassLevel { .. }
        | TriggerConditionDef::SourceCounters { .. }
        | TriggerConditionDef::SourceCastWith(_)
        | TriggerConditionDef::SourcePaidAlternativeCost(_)
        | TriggerConditionDef::SourceHasCastPlayerBinding(_)
        | TriggerConditionDef::SourcePaidAdditionalCost(_)
        | TriggerConditionDef::SourceCastFrom(_)
        | TriggerConditionDef::SourceWasCast
        | TriggerConditionDef::SourceCastAtInstantSpeed
        | TriggerConditionDef::ValueComparison(_)
        | TriggerConditionDef::SourceLoyalty { .. }
        | TriggerConditionDef::SourceDealtDamageToOpponentThisTurn
        | TriggerConditionDef::OpponentWasDealtDamageThisTurn
        | TriggerConditionDef::SourceIsTapped
        | TriggerConditionDef::SourceIsUntapped
        | TriggerConditionDef::ControllerLifeAtMost(_)
        | TriggerConditionDef::ControllerLifeAtMostHalfStartingLife => true,
    }
}

/// Presence is read while discovering the ability set, so it must not ask
/// for characteristics derived from that same set or a stack-event binding.
pub(super) fn ability_presence_condition_supported(condition: TriggerConditionDef) -> bool {
    match condition {
        TriggerConditionDef::All(conditions) | TriggerConditionDef::AnyOf(conditions) => conditions
            .iter()
            .copied()
            .all(ability_presence_condition_supported),
        TriggerConditionDef::Not(condition) => ability_presence_condition_supported(*condition),
        TriggerConditionDef::ObjectCount { query, .. } => {
            query.zones.iter().all(|zone| {
                matches!(
                    zone,
                    ZoneKind::Hand
                        | ZoneKind::Library
                        | ZoneKind::Graveyard
                        | ZoneKind::Exile
                        | ZoneKind::Command
                )
            }) && query.object == ObjectPredicateDef::Any
                && static_query_supported(query)
        }
        TriggerConditionDef::SourceClassLevel { .. }
        | TriggerConditionDef::SourceCounters { .. }
        | TriggerConditionDef::SourceIsTapped
        | TriggerConditionDef::SourceIsUntapped
        | TriggerConditionDef::SourceUntapped
        | TriggerConditionDef::ControllerLifeAtMost(_)
        | TriggerConditionDef::ControllerLifeAtMostHalfStartingLife => true,
        _ => false,
    }
}
