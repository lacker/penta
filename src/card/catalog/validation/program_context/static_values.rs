// Which amounts a static ability may read.
//
// Separated from the effect shapes next door because it answers a narrower
// question: those say which operations a static clause may perform, and
// these say which numbers those operations may be given. Included textually
// into `program_context.rs`, so the imports here are that module's.

fn static_source_value_supported(value: ValueDef) -> bool {
    matches!(
        value,
        ValueDef::Constant(_)
            | ValueDef::SourcePower
            | ValueDef::CountersOnSource(_)
            | ValueDef::CardsInHandAbove { .. }
    )
}

fn static_object_value_aggregate_supported(aggregate: ObjectValueAggregateDef) -> bool {
    matches!(
        aggregate.select,
        ObjectValueDef::ManaValue | ObjectValueDef::Power | ObjectValueDef::Toughness
    ) && static_object_set_supported(aggregate.objects)
}

fn static_power_toughness_value_supported(value: ValueDef) -> bool {
    match value {
        // The first two are read live from the static effect's own
        // controller: a battlefield count, or the size of that player's
        // hand. The third is read from the affected object instead, which
        // the static power-and-toughness layer has in hand.
        ValueDef::Constant(_)
        | ValueDef::CardsInHandAbove { .. }
        | ValueDef::AffectedManaValue
        | ValueDef::AffectedColorCount
        // Read live from every graveyard, which the static layer can see the
        // same way it sees a battlefield count.
        | ValueDef::CardTypesAmongGraveyards(_)
        // A per-turn tally the game keeps and clears with the turn, read the
        // same way and just as live.
        | ValueDef::CardsDrawnThisTurn(_)
        | ValueDef::LandsPlayedThisTurn(_)
        | ValueDef::LifeGainedThisTurn(_)
        | ValueDef::CountSpellsCastThisTurn(_)
        // Counters on the effect's own source: plain state on a permanent the
        // layer already has, so reading it cannot re-enter the walk.
        | ValueDef::CountersOnSource(_)
        | ValueDef::DevotionTo(_)
        | ValueDef::BasicLandTypesControlled(_)
        | ValueDef::LibrarySize(_) => true,
        ValueDef::CountersOnObject(counted) => {
            matches!(counted.object, ObjectRefDef::Source | ObjectRefDef::CreatingSource)
        }
        ValueDef::AggregateObjectValues(aggregate) => {
            static_object_value_aggregate_supported(*aggregate)
        }
        ValueDef::CountObjects(objects) | ValueDef::CardTypesAmongObjects(objects) => {
            static_object_set_supported(*objects)
        }
        ValueDef::CountMatchingObjects(query)
        | ValueDef::AnyMatchingObject(query)
        | ValueDef::DistinctNamesAmong(query) => static_query_supported(*query),
        ValueDef::Scaled(scaled) => static_power_toughness_value_supported(scaled.value),
        ValueDef::Halved(halved) => static_power_toughness_value_supported(halved.value),
        ValueDef::IfSourceMatches(branches) => {
            static_object_predicate_supported(branches.object)
                && static_power_toughness_value_supported(branches.then)
                && static_power_toughness_value_supported(branches.otherwise)
        }
        ValueDef::Sum(sum) => {
            static_power_toughness_value_supported(sum.left)
                && static_power_toughness_value_supported(sum.right)
        }
        // A conditional amount belongs to a resolving effect, not to the
        // static power-and-toughness layer.
        ValueDef::IfCardTypesAmongGraveyards(_)
        | ValueDef::IfAdditionalCostPaid(_)
        | ValueDef::CountMatchingPlayerAttachments(_)
        | ValueDef::CreaturesDiedThisTurn
        | ValueDef::OpponentsWhoLostLifeThisTurn
        | ValueDef::ChosenX
        | ValueDef::SourceCastX
        | ValueDef::SourcePower
        | ValueDef::SourceToughness
        | ValueDef::TriggeringObjectPower
        | ValueDef::TriggeringObjectToughness
        | ValueDef::TriggerEventAmount
        | ValueDef::DamageTakenThisTurn { .. }
        | ValueDef::Negate(_)
        | ValueDef::IfCreatureDiedThisTurn(_)
        | ValueDef::IfControllerLifeAtMost(_)
        | ValueDef::IfCondition(_)
        | ValueDef::IfTargetMatches(_)
        | ValueDef::IfMatchingObjectCount(_)
        | ValueDef::ColorsOfManaSpent
        | ValueDef::PaidAmount
        | ValueDef::MatchedCount
        | ValueDef::MatchedCardTypes
        | ValueDef::MatchedManaValue
        | ValueDef::BoundObjectCount(_)
        | ValueDef::SpellsCastBeforeThisTurn
        | ValueDef::PlayerCounters { .. }
        | ValueDef::SacrificedManaValue
        | ValueDef::AdditionalCostPayments(_)
        | ValueDef::SpellsCastThisGame(_)
        | ValueDef::TargetPower(_)
        | ValueDef::TargetToughness(_)
        | ValueDef::TargetLibrarySize(_)
        | ValueDef::LifeTotal(_)
        | ValueDef::StartingLifeTotal(_)
        | ValueDef::TargetManaValue(_)
        | ValueDef::ObjectPower(_)
        | ValueDef::ObjectManaValue(_)
        | ValueDef::DistinctTargets
        | ValueDef::DividedAmongTargets => false,
    }
}

fn static_cost_reduction_value_supported(value: ValueDef) -> bool {
    match value {
        ValueDef::Constant(_) => true,
        ValueDef::CountMatchingObjects(query) => static_query_supported(*query),
        ValueDef::IfMatchingObjectCount(condition) => {
            static_query_supported(condition.query)
                && static_cost_reduction_value_supported(condition.then)
                && static_cost_reduction_value_supported(condition.otherwise)
        }
        // Domain counts basic land types rather than permanents, which no
        // query can say. The planner reads it off the board the same way.
        ValueDef::BasicLandTypesControlled(relation) => static_player_relation_supported(relation),
        ValueDef::Sum(sum) => {
            static_cost_reduction_value_supported(sum.left)
                && static_cost_reduction_value_supported(sum.right)
        }
        ValueDef::CreaturesDiedThisTurn
        | ValueDef::CountMatchingPlayerAttachments(_)
        | ValueDef::OpponentsWhoLostLifeThisTurn
        | ValueDef::DistinctNamesAmong(_)
        | ValueDef::CardTypesAmongGraveyards(_)
        | ValueDef::IfCardTypesAmongGraveyards(_)
        | ValueDef::IfAdditionalCostPaid(_)
        | ValueDef::AggregateObjectValues(_)
        | ValueDef::ChosenX
        | ValueDef::SourceCastX
        | ValueDef::SourcePower
        | ValueDef::AffectedManaValue
        | ValueDef::AffectedColorCount
        | ValueDef::CardTypesAmongObjects(_)
        | ValueDef::SourceToughness
        | ValueDef::TriggeringObjectPower
        | ValueDef::TriggeringObjectToughness
        | ValueDef::TriggerEventAmount
        | ValueDef::CardsInHandAbove { .. }
        | ValueDef::DamageTakenThisTurn { .. }
        | ValueDef::AnyMatchingObject(_)
        | ValueDef::Negate(_)
        | ValueDef::Scaled(_)
        | ValueDef::Halved(_)
        | ValueDef::IfCreatureDiedThisTurn(_)
        | ValueDef::IfControllerLifeAtMost(_)
        | ValueDef::IfCondition(_)
        | ValueDef::IfSourceMatches(_)
        | ValueDef::IfTargetMatches(_)
        | ValueDef::CountersOnSource(_)
        | ValueDef::CountersOnObject(_)
        | ValueDef::CardsDrawnThisTurn(_)
        | ValueDef::LandsPlayedThisTurn(_)
        | ValueDef::LifeGainedThisTurn(_)
        | ValueDef::DevotionTo(_)
        | ValueDef::LibrarySize(_)
        | ValueDef::ColorsOfManaSpent
        | ValueDef::PaidAmount
        | ValueDef::MatchedCount
        | ValueDef::MatchedCardTypes
        | ValueDef::MatchedManaValue
        | ValueDef::CountObjects(_)
        | ValueDef::BoundObjectCount(_)
        | ValueDef::SpellsCastBeforeThisTurn
        | ValueDef::PlayerCounters { .. }
        | ValueDef::SacrificedManaValue
        | ValueDef::AdditionalCostPayments(_)
        | ValueDef::SpellsCastThisGame(_)
        | ValueDef::CountSpellsCastThisTurn(_)
        | ValueDef::TargetPower(_)
        | ValueDef::TargetToughness(_)
        | ValueDef::TargetLibrarySize(_)
        | ValueDef::LifeTotal(_)
        | ValueDef::StartingLifeTotal(_)
        | ValueDef::TargetManaValue(_)
        | ValueDef::ObjectPower(_)
        | ValueDef::ObjectManaValue(_)
        | ValueDef::DistinctTargets
        | ValueDef::DividedAmongTargets => false,
    }
}

fn static_spell_cost_value_supported(value: ValueDef) -> bool {
    match value {
        ValueDef::DistinctTargets => true,
        ValueDef::CountSpellsCastThisTurn(query) => {
            static_player_relation_supported(query.player)
                && static_object_predicate_supported(query.spell)
        }
        _ => static_cost_reduction_value_supported(value),
    }
}

fn static_spell_cost_modification_supported(
    modification: CostModificationDef,
    source_zones: &[ZoneKind],
) -> bool {
    match modification {
        CostModificationDef::Spell(modification) => {
            let source_supported = match source_zones {
                [ZoneKind::Battlefield] => true,
                [ZoneKind::Stack] => {
                    modification.condition == SpellCostConditionDef::TargetsSource
                        && matches!(modification.adjustment, CostAdjustmentDef::Add(_))
                }
                _ => false,
            };
            let amount_supported = match modification.adjustment {
                CostAdjustmentDef::Add(CostAmountDef::Mana(_)) => true,
                CostAdjustmentDef::Add(CostAmountDef::Generic(value))
                | CostAdjustmentDef::Subtract(CostAmountDef::Generic(value)) => {
                    static_spell_cost_value_supported(value)
                }
                CostAdjustmentDef::Subtract(CostAmountDef::Mana(amount)) => {
                    amount.hybrid.iter().all(|count| *count == 0)
                        && amount.additional_flexible.iter().all(|count| *count == 0)
                        && !amount.variable_x
                        && amount.x_multiplier == 0
                }
            };
            source_supported
                && static_object_predicate_supported(modification.spell)
                && static_player_relation_supported(modification.caster)
                && amount_supported
        }
        CostModificationDef::SpellAlternative {
            spell,
            caster,
            zones,
            ..
        } => {
            !zones.is_empty()
                && zones.iter().all(|zone| {
                    matches!(
                        zone,
                        ZoneKind::Library | ZoneKind::Hand | ZoneKind::Graveyard | ZoneKind::Exile
                    )
                })
                && static_object_predicate_supported(spell)
                && static_player_relation_supported(caster)
        }
        CostModificationDef::AbilityIncrease { .. }
        | CostModificationDef::SourceAbilityIncrease { .. }
        | CostModificationDef::AbilityReduction { .. } => false,
    }
}
