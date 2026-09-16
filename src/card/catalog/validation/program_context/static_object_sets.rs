// Static object/player sets and recipient shapes. Included by program_context.rs.

fn static_player_set_supported(players: PlayerSetDef) -> bool {
    match players {
        PlayerSetDef::All
        | PlayerSetDef::One(
            PlayerRefDef::EffectController | PlayerRefDef::Opponent | PlayerRefDef::EnchantedPlayer,
        ) => true,
        PlayerSetDef::Related(relation) => static_player_relation_supported(relation),
        PlayerSetDef::LegalTargets(_)
        | PlayerSetDef::One(
            PlayerRefDef::CastBinding(_)
            | PlayerRefDef::EventPlayer
            | PlayerRefDef::Target(_)
            | PlayerRefDef::ControllerOf(_)
            | PlayerRefDef::OpponentOf(_)
            | PlayerRefDef::OwnerOf(_),
        ) => false,
    }
}

fn static_player_relation_supported(relation: PlayerRelation) -> bool {
    matches!(
        relation,
        PlayerRelation::Any
            | PlayerRelation::You
            | PlayerRelation::NotYou
            | PlayerRelation::Opponent
            | PlayerRelation::ActivePlayer
            | PlayerRelation::NonactivePlayer
            | PlayerRelation::ChosenPlayer
            | PlayerRelation::DefendingPlayer
            | PlayerRelation::EnchantedPlayer
    )
}

fn static_object_set_supported(objects: ObjectSetDef) -> bool {
    match objects {
        ObjectSetDef::Union(sets) => sets.iter().copied().all(static_object_set_supported),
        ObjectSetDef::One(ObjectRefDef::Source | ObjectRefDef::AttachedToSource)
        | ObjectSetDef::LinkedExiles => true,
        ObjectSetDef::Query(query) => {
            query.zones == [ZoneKind::Battlefield] && static_query_supported(query)
        }
        ObjectSetDef::ExceptObject {
            objects,
            object: ObjectRefDef::Source | ObjectRefDef::AttachedToSource,
        } => static_object_set_supported(*objects),
        ObjectSetDef::LegalTargets(_)
        | ObjectSetDef::One(
            ObjectRefDef::ResolvingObject
            | ObjectRefDef::CreatingSource
            | ObjectRefDef::ZoneChangeSuccessor(_)
            | ObjectRefDef::ZoneChangeResultOfTriggeringObject
            | ObjectRefDef::Binding(_)
            | ObjectRefDef::AdditionalCostObject(_)
            | ObjectRefDef::AbilityGrantSource
            | ObjectRefDef::Target(_)
            | ObjectRefDef::SourceOfTargetedStackObject(_)
            | ObjectRefDef::TriggeringObject
            | ObjectRefDef::DamagedObject,
        )
        | ObjectSetDef::Binding(_)
        | ObjectSetDef::ZoneChangeSuccessorsOfBinding(_)
        | ObjectSetDef::MatchingBinding { .. }
        | ObjectSetDef::PermanentsTargetedBy(_)
        | ObjectSetDef::PlayerAttachments(_)
        | ObjectSetDef::LegalAttachmentHosts(_)
        | ObjectSetDef::CardsDrawnThisTurnInHand(_)
        | ObjectSetDef::PermanentsControlledBy(_)
        | ObjectSetDef::TokensCreatedBy(_)
        | ObjectSetDef::AttachmentsOf(_)
        | ObjectSetDef::BottomOfGraveyard(_)
        | ObjectSetDef::TopOfGraveyardMatching { .. }
        | ObjectSetDef::SharingCreatureType { .. }
        | ObjectSetDef::ExceptObject { .. }
        | ObjectSetDef::InZone { .. } => false,
        ObjectSetDef::Matching { objects, object } => {
            static_object_set_supported(*objects)
                && static_object_predicate_supported(object.predicate())
        }
    }
}

/// A condition may inspect objects outside the battlefield even though a
/// static apply can only modify battlefield objects. This preserves the
/// query vocabulary supported by the older `ObjectCount` condition while the
/// count and the applied operation stay separately composed.
fn static_condition_object_set_supported(objects: ObjectSetDef) -> bool {
    match objects {
        ObjectSetDef::Query(query) => static_query_supported(query),
        ObjectSetDef::Matching { objects, object } => {
            static_condition_object_set_supported(*objects)
                && static_object_predicate_supported(object.predicate())
        }
        ObjectSetDef::ExceptObject {
            objects,
            object: ObjectRefDef::Source | ObjectRefDef::AttachedToSource,
        } => static_condition_object_set_supported(*objects),
        ObjectSetDef::ExceptObject { .. } => false,
        _ => static_object_set_supported(objects),
    }
}

pub(super) fn static_query_supported(query: ObjectQueryDef) -> bool {
    !query.zones.is_empty()
        && query.position.is_none_or(|position| {
            matches!(position, crate::card::ZonePositionDef::FromTop(_))
                && query
                    .zones
                    .iter()
                    .all(|zone| matches!(zone, ZoneKind::Library | ZoneKind::Graveyard))
        })
        && [query.related_player, query.controller, query.owner]
            .into_iter()
            .flatten()
            .all(static_player_set_supported)
        && static_query_predicate_supported(query.object, query.zones)
}

fn static_query_predicate_supported(object: ObjectPredicateDef, zones: &[ZoneKind]) -> bool {
    match object {
        ObjectPredicateDef::HasAlternateSpell(_) => zones
            .iter()
            .all(|zone| !matches!(zone, ZoneKind::Battlefield | ZoneKind::Stack)),
        ObjectPredicateDef::All(parts) | ObjectPredicateDef::AnyOf(parts) => parts
            .iter()
            .all(|part| static_query_predicate_supported(*part, zones)),
        ObjectPredicateDef::Not(part) => static_query_predicate_supported(*part, zones),
        _ => static_object_predicate_supported(object),
    }
}

fn static_animation_query_supported(recipient: EffectRecipientDef) -> bool {
    static_direct_characteristic_recipient(recipient)
        || recipient.object_query().is_some_and(|query| {
            query.zones == [ZoneKind::Battlefield]
                && static_query_supported(query)
                && static_animation_predicate_supported(query.object, false)
        })
}

fn static_type_animation_query_supported(recipient: EffectRecipientDef) -> bool {
    static_direct_characteristic_recipient(recipient)
        || recipient.object_query().is_some_and(|query| {
            query.zones == [ZoneKind::Battlefield]
                && static_query_supported(query)
                && static_animation_predicate_supported(query.object, true)
        })
}

fn static_creature_type_query_supported(recipient: EffectRecipientDef) -> bool {
    static_direct_characteristic_recipient(recipient)
        || recipient.object_query().is_some_and(|query| {
            static_query_supported(query)
                && static_animation_predicate_supported(query.object, true)
        })
}

fn static_direct_characteristic_recipient(recipient: EffectRecipientDef) -> bool {
    matches!(
        recipient.object_reference(),
        Some(ObjectRefDef::Source | ObjectRefDef::AttachedToSource)
    )
}
