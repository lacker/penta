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
        | TriggerConditionDef::SourceResolutionsThisTurn { .. }
        | TriggerConditionDef::TargetMatches { .. }
        // And this reads a binding, which only a resolution has.
        | TriggerConditionDef::BoundObjectMatches { .. }
        | TriggerConditionDef::ControlsGreatestPowerCreature => false,
        TriggerConditionDef::SourceMatches { object }
        | TriggerConditionDef::AttachedPermanentMatches { object } => {
            static_object_predicate_supported(object)
        }
        TriggerConditionDef::ControllerHadPermanentLeaveThisTurn
        | TriggerConditionDef::ControllerHadCardLeaveGraveyardThisTurn
        | TriggerConditionDef::ControllerHasCitysBlessing
        | TriggerConditionDef::ControllerGainedLifeThisTurn
        | TriggerConditionDef::CreatureDiedThisTurn
        | TriggerConditionDef::BoundObjectsShareName { .. }
        | TriggerConditionDef::SourceArrivedSinceControllersLastUpkeep
        | TriggerConditionDef::SourceOnBattlefield
        | TriggerConditionDef::SourceInZone(_)
        | TriggerConditionDef::SourceUntapped
        | TriggerConditionDef::SourceIsPaired
        | TriggerConditionDef::SourceCounters { .. }
        | TriggerConditionDef::SourceCastWith(_)
        | TriggerConditionDef::SourcePaidAdditionalCost(_)
        | TriggerConditionDef::SourceCastFrom(_)
        | TriggerConditionDef::SourceWasCast
        | TriggerConditionDef::SourceCastAtInstantSpeed
        | TriggerConditionDef::ValueComparison(_)
        | TriggerConditionDef::SourceLoyalty { .. }
        | TriggerConditionDef::SourceDealtDamageToOpponentThisTurn
        | TriggerConditionDef::SourceIsTapped
        | TriggerConditionDef::SourceIsUntapped
        | TriggerConditionDef::ControllerLifeAtMost(_)
        | TriggerConditionDef::ControllerLifeAtMostHalfStartingLife => true,
    }
}
