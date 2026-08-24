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
        | ValueDef::TotalPowerOfLinkedExiles
        | ValueDef::TotalToughnessOfLinkedExiles
        // Read live from every graveyard, which the static layer can see the
        // same way it sees a battlefield count.
        | ValueDef::CardTypesAmongGraveyards(_)
        // A per-turn tally the game keeps and clears with the turn, read the
        // same way and just as live.
        | ValueDef::CardsDrawnThisTurn(_)
        // Counters on the effect's own source: plain state on a permanent the
        // layer already has, so reading it cannot re-enter the walk.
        | ValueDef::CountersOnSource(_)
        | ValueDef::DevotionTo(_)
        | ValueDef::BasicLandTypesControlled(_)
        | ValueDef::LibrarySize(_) => true,
        ValueDef::CountMatchingObjects(query)
        | ValueDef::AnyMatchingObject(query)
        | ValueDef::GreatestPowerAmong(query) => static_query_supported(*query),
        ValueDef::Scaled(scaled) => static_power_toughness_value_supported(scaled.value),
        ValueDef::Halved(halved) => static_power_toughness_value_supported(halved.value),
        ValueDef::Sum(sum) => {
            static_power_toughness_value_supported(sum.left)
                && static_power_toughness_value_supported(sum.right)
        }
        // A conditional amount belongs to a resolving effect, not to the
        // static power-and-toughness layer.
        ValueDef::IfCardTypesAmongGraveyards(_)
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
        | ValueDef::IfTargetMatches(_)
        | ValueDef::IfMatchingObjectCount(_)
        | ValueDef::ColorsOfManaSpent
        | ValueDef::PaidAmount
        | ValueDef::MatchedCount
        | ValueDef::MatchedCardTypes
        | ValueDef::MatchedManaValue
        | ValueDef::BoundObjectCount(_)
        | ValueDef::SpellsCastBeforeThisTurn
        | ValueDef::SpellsCastThisGame(_)
        | ValueDef::TargetPower(_)
        | ValueDef::TargetToughness(_)
        | ValueDef::TargetLibrarySize(_)
        | ValueDef::LifeTotal(_)
        | ValueDef::TargetManaValue(_)
        | ValueDef::DividedAmongTargets => false,
    }
}

fn static_cost_reduction_value_supported(value: ValueDef) -> bool {
    match value {
        ValueDef::Constant(_) => true,
        ValueDef::CountMatchingObjects(query) => static_query_supported(*query),
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
        | ValueDef::CardTypesAmongGraveyards(_)
        | ValueDef::IfCardTypesAmongGraveyards(_)
        | ValueDef::GreatestPowerAmong(_)
        | ValueDef::ChosenX
        | ValueDef::SourceCastX
        | ValueDef::SourcePower
        | ValueDef::AffectedManaValue
        | ValueDef::AffectedColorCount
        | ValueDef::TotalPowerOfLinkedExiles
        | ValueDef::TotalToughnessOfLinkedExiles
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
        | ValueDef::IfTargetMatches(_)
        | ValueDef::IfMatchingObjectCount(_)
        | ValueDef::CountersOnSource(_)
        | ValueDef::CardsDrawnThisTurn(_)
        | ValueDef::DevotionTo(_)
        | ValueDef::LibrarySize(_)
        | ValueDef::ColorsOfManaSpent
        | ValueDef::PaidAmount
        | ValueDef::MatchedCount
        | ValueDef::MatchedCardTypes
        | ValueDef::MatchedManaValue
        | ValueDef::BoundObjectCount(_)
        | ValueDef::SpellsCastBeforeThisTurn
        | ValueDef::SpellsCastThisGame(_)
        | ValueDef::TargetPower(_)
        | ValueDef::TargetToughness(_)
        | ValueDef::TargetLibrarySize(_)
        | ValueDef::LifeTotal(_)
        | ValueDef::TargetManaValue(_)
        | ValueDef::DividedAmongTargets => false,
    }
}
