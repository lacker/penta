fn validate_object_collection_references(
    collection: crate::card::ObjectCollectionSourceDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match collection {
        crate::card::ObjectCollectionSourceDef::ObjectSet(input) => {
            validate_recipient_target_references(
                EffectRecipientDef::objects(input),
                target_count,
                scope,
            )
        }
        crate::card::ObjectCollectionSourceDef::TopCards { player, count } => {
            validate_player_reference(player, target_count, scope)?;
            validate_value_target_references(count, target_count, scope)
        }
        crate::card::ObjectCollectionSourceDef::TopCardsThroughFirstMatching { player, object } => {
            validate_player_reference(player, target_count, scope)?;
            validate_object_predicate_references(object, target_count, scope)
        }
    }
}

fn validate_object_set_predicate_references(
    predicate: crate::card::ObjectSetPredicateDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    predicate.filter.map_or(Ok(()), |filter| {
        validate_object_predicate_references(filter.predicate(), target_count, scope)
    })
}
