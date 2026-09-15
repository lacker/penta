use super::*;
use crate::ParentBinding;

#[test]
fn effect_resolution_context_round_trips_typed_bindings() {
    let trigger = TriggerContext {
        object: Some(GameObjectId(10)),
        zone_change_result: Some(GameObjectId(12)),
        object_controller: Some(PlayerId::One),
        event_player: Some(PlayerId::Two),
        amount: Some(3),
        damaged_object: None,
        sacrificed_object: None,
        cast_from_zone: Some(crate::card::ZoneKind::Graveyard),
    };
    let mut context = EffectResolutionContext::new(trigger);
    context.bind_single_object(Binding!("object"), Some(Target::Spell(GameObjectId(11))));
    context.bind_object_group(
        Binding!("objects"),
        vec![
            Target::Permanent(GameObjectId(12)),
            Target::Card(GameObjectId(13)),
            Target::Player(PlayerId::Two),
        ],
    );
    context.bind_runtime_card_name(
        &RuntimeBinding::Label("cabal_therapy_name".into()),
        "Lightning Bolt".into(),
    );
    context.bind_single_object(ParentBinding, Some(Target::Permanent(GameObjectId(17))));
    context.bind_object_group(
        ParentBinding,
        vec![
            Target::Card(GameObjectId(18)),
            Target::Card(GameObjectId(19)),
        ],
    );
    context.declare_binding_group_label("optional_card");
    context.bind_binding_group_label("revealed_card", vec![Target::Card(GameObjectId(14))]);
    context.declare_binding_group_label("empty_cards");
    context.bind_binding_group_label(
        "milled_cards",
        vec![
            Target::Card(GameObjectId(15)),
            Target::Card(GameObjectId(16)),
        ],
    );

    let snapshot = effect_resolution_context_snapshot(&context);
    let rebuilt = parse_effect_resolution_context(snapshot).expect("context should parse");

    assert_eq!(rebuilt, context);
    assert_eq!(
        rebuilt.single_object(ParentBinding),
        Some(Target::Permanent(GameObjectId(17)))
    );
    assert_eq!(
        rebuilt.object_group(ParentBinding),
        [
            Target::Card(GameObjectId(18)),
            Target::Card(GameObjectId(19))
        ]
    );
    assert_eq!(
        rebuilt.single_object(Binding!("object")),
        Some(Target::Spell(GameObjectId(11)))
    );
    assert_eq!(
        rebuilt.object_group(Binding!("objects")),
        [
            Target::Permanent(GameObjectId(12)),
            Target::Card(GameObjectId(13)),
            Target::Player(PlayerId::Two),
        ]
    );
    assert!(rebuilt.bindings().contains_key("optional_card"));
    assert!(rebuilt.object_group(Binding!("optional_card")).is_empty());
    assert_eq!(
        rebuilt.object_group(Binding!("revealed_card")),
        [Target::Card(GameObjectId(14))]
    );
    assert!(rebuilt.bindings().contains_key("empty_cards"));
    assert!(rebuilt.object_group(Binding!("empty_cards")).is_empty());
    assert_eq!(
        rebuilt.object_group(Binding!("milled_cards")),
        [
            Target::Card(GameObjectId(15)),
            Target::Card(GameObjectId(16))
        ]
    );
    let mut referenced = resolution_context_referenced_object_ids(&rebuilt);
    referenced.sort_unstable();
    assert_eq!(
        referenced,
        [
            GameObjectId(10),
            GameObjectId(11),
            GameObjectId(12),
            GameObjectId(13),
            GameObjectId(14),
            GameObjectId(15),
            GameObjectId(16),
            GameObjectId(17),
            GameObjectId(18),
            GameObjectId(19),
        ]
    );
}
