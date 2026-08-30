use super::{
    BasicLandTypeChangeSnapshot, Game, PlayerId, StackObject, StackObjectKind, StackSnapshot,
    applied_stack_effect_snapshots, basic_land_type_snapshot, face_down_characteristics_snapshot,
    object_kind_snapshot, stack_ability_snapshot, stack_object_requires_retired,
};

pub(in crate::game::state_checkpoint) fn current_stack_snapshot(
    game: &Game,
    viewer: PlayerId,
    object: &StackObject,
) -> StackSnapshot {
    let ability_payload = (object.kind != StackObjectKind::Spell)
        .then(|| stack_ability_snapshot(game, viewer, object))
        .flatten();
    let has_unlocated_ability_payload = object.kind != StackObjectKind::Spell
        && object.ability.is_some()
        && ability_payload.is_none();
    let (applied_effects, has_unlocated_applied_effect) =
        applied_stack_effect_snapshots(game, object);
    let face_down = object
        .face_down
        .and_then(face_down_characteristics_snapshot);
    let has_unlocated_face_down = object.face_down.is_some() && face_down.is_none();
    StackSnapshot {
        object_id: object.id.0,
        kind: super::kind_snapshot(object.kind),
        owner: object.card.owner.index(),
        object_kind: object_kind_snapshot(object.card.definition),
        ability_payload,
        requires_retired_object: stack_object_requires_retired(game, object),
        has_runtime_overrides: has_unlocated_ability_payload
            || has_unlocated_applied_effect
            || has_unlocated_face_down,
        applied_effects,
        text_changes: object
            .text_changes
            .iter()
            .map(|change| BasicLandTypeChangeSnapshot {
                from: basic_land_type_snapshot(change.from),
                to: basic_land_type_snapshot(change.to),
            })
            .collect(),
        colors: object.colors.map(crate::card::ColorSet::to_flags),
        colors_of_mana_spent: object
            .cast
            .as_ref()
            .map_or([false; 5], |cast| cast.colors_of_mana_spent.to_flags()),
        phyrexian_symbols_paid_with_life: object
            .cast
            .as_ref()
            .map_or(0, |cast| cast.phyrexian_symbols_paid_with_life),
        cast_via_flashback: object.cast.as_ref().is_some_and(|cast| cast.via_flashback),
        cast_via_suspend: object.cast.as_ref().is_some_and(|cast| cast.via_suspend),
        cast_at_instant_speed: object
            .cast
            .as_ref()
            .is_some_and(|cast| cast.at_instant_speed),
        cast_from_zone: object
            .cast
            .as_ref()
            .and_then(|cast| cast.source_zone)
            .map(|zone| zone.label().to_owned()),
        cast_tags: Vec::new(),
        cast_alternative: object
            .cast
            .as_ref()
            .and_then(|cast| cast.alternative)
            .map(|kind| kind.label().to_owned()),
        cast_x: object.cast.as_ref().map_or(0, |cast| cast.x),
        cast_repeatable_additional_costs: object
            .cast
            .as_ref()
            .map_or(0, |cast| cast.repeatable_additional_costs),
        cast_additional_costs: object
            .cast
            .as_ref()
            .map_or_else(Vec::new, |cast| cast.additional_costs.clone()),
        cast_exiled_payment_cards: object.cast.as_ref().map_or_else(Vec::new, |cast| {
            cast.exiled_payment_cards.iter().map(|id| id.0).collect()
        }),
        face_down,
        is_copy: object.is_copy,
    }
}
