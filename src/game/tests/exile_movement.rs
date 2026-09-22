use super::*;

const VICTIMS: EffectRecipientDef = EffectRecipientDef::matching_objects(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::Opponent,
);

static CANCEL: AbilityDef = AbilityDef::replacement_for(
    "If this creature would be exiled, instead it remains on the battlefield.",
    ReplacementEventDef::WouldMove {
        from: Some(ZoneKind::Battlefield),
        to: ZoneKind::Exile,
        cause: ZoneMoveCauseDef::Any,
    },
    ReplacementEffectDef::ReplaceEventWithNothing,
);

static REDIRECT: AbilityDef = AbilityDef::replacement_for(
    "If this creature would be exiled, put it into its owner's hand instead.",
    ReplacementEventDef::WouldMove {
        from: Some(ZoneKind::Battlefield),
        to: ZoneKind::Exile,
        cause: ZoneMoveCauseDef::Any,
    },
    ReplacementEffectDef::MoveToZone(ZoneKind::Hand),
);

fn grant(game: &mut Game, source: GameObjectId, ability: &'static AbilityDef) {
    let mut object = spell(410_000, cards::LIGHTNING_BOLT, PlayerId::One, 0);
    object.source = Some(source);
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::add_ability(ability),
            duration: ResolvedEffectDurationDef::Permanent,
        }),
        &object,
        TriggerContext::empty(),
    );
}

fn linked(object: EffectRecipientDef, face_down: bool) -> EffectDef {
    EffectDef::ExileLinkedToSource {
        object,
        face_down,
        until_source_leaves: false,
        then: Some(&EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
        }),
    }
}

#[test]
fn exile_variants_wait_for_replacements_and_only_remember_actual_exiles() {
    for prepared in [false, true] {
        for variant in 0..5 {
            for cancel in [false, true] {
                let mut game = ready_game();
                game.set_prepared_engine_enabled(prepared);
                game.battlefield.clear();
                game.players[1].hand.clear();
                for id in 410_001..410_004 {
                    game.battlefield
                        .push(creature(id, cards::GRIZZLY_BEARS, PlayerId::Two));
                }
                grant(&mut game, GameObjectId(410_001), &CANCEL);
                grant(&mut game, GameObjectId(410_002), &CANCEL);
                grant(&mut game, GameObjectId(410_002), &REDIRECT);
                let object = spell(410_010, cards::LIGHTNING_BOLT, PlayerId::One, 0);
                let effect = match variant {
                    0 => EffectDef::move_to_zone(VICTIMS, ZoneKind::Exile, ZonePlacement::Top),
                    1 | 4 => linked(VICTIMS, variant == 4),
                    2 => EffectDef::ExileGrantingOwnerPlay {
                        object: VICTIMS,
                        surcharge: mana_cost!("{2}"),
                        later_turn: false,
                        cast_only: true,
                    },
                    _ => EffectDef::ExileGrantingControllerPlayThisTurn { object: VICTIMS },
                };
                let life = game.players[0].life;
                game.resolve_effect_defs(
                    vec![
                        ScopedEffect::primary(effect),
                        ScopedEffect::primary(EffectDef::GainLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(1),
                        }),
                    ],
                    &object,
                    &TriggerContext::empty().into(),
                );
                assert_eq!(game.battlefield.len(), 3, "the entire instruction waits");
                assert_eq!(game.players[0].life, life, "neither continuation ran early");
                assert!(game.linked_exiles.is_empty());
                assert!(game.exile_play_permissions.is_empty());
                let decision = game.observe(PlayerId::Two).decision.unwrap();
                let option = decision
                    .options
                    .iter()
                    .find(|option| {
                        option
                            .label
                            .contains(if cancel { "remains" } else { "hand" })
                    })
                    .unwrap()
                    .id;
                game.apply(
                    PlayerId::Two,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options: vec![option],
                    },
                )
                .unwrap();
                assert_eq!(game.battlefield.len(), if cancel { 2 } else { 1 });
                assert_eq!(game.players[1].hand.len(), usize::from(!cancel));
                assert_eq!(game.players[1].exile.len(), 1);
                let exiled = game.players[1].exile[0].id;
                assert_ne!(exiled, GameObjectId(410_003));
                assert_eq!(
                    game.players[0].life,
                    life + if matches!(variant, 1 | 4) { 3 } else { 1 }
                );
                if matches!(variant, 1 | 4) {
                    assert_eq!(game.linked_exile_ids(object.id), vec![exiled]);
                } else {
                    assert!(game.linked_exiles.is_empty());
                }
                if matches!(variant, 2 | 3) {
                    assert_eq!(game.exile_play_permissions.len(), 1);
                    let permission = &game.exile_play_permissions[0];
                    assert_eq!(permission.card, exiled);
                    assert_eq!(
                        permission.player,
                        if variant == 2 {
                            PlayerId::Two
                        } else {
                            PlayerId::One
                        }
                    );
                } else if variant == 4 {
                    assert!(game.exiled_card_is_face_down(exiled));
                    assert!(
                        game.exile_play_permissions
                            .iter()
                            .all(|permission| permission.hidden_only)
                    );
                } else {
                    assert!(game.exile_play_permissions.is_empty());
                }
            }
        }
    }
}

#[test]
fn links_do_not_follow_a_card_out_of_exile_and_back() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.battlefield
        .push(creature(410_020, cards::GRIZZLY_BEARS, PlayerId::Two));
    let object = spell(410_021, cards::LIGHTNING_BOLT, PlayerId::One, 0);
    game.resolve_effect_def(
        ScopedEffect::primary(linked(VICTIMS, false)),
        &object,
        TriggerContext::empty(),
    );
    let exiled = game.linked_exile_ids(object.id)[0];
    game.move_card_target_to_zone(
        exiled,
        ZoneKind::Graveyard,
        ZoneMoveCause::Rules,
        None,
        ZonePlacement::Top,
    );
    let graveyard = game.successors[&exiled];
    game.move_card_target_to_zone(
        graveyard,
        ZoneKind::Exile,
        ZoneMoveCause::Rules,
        None,
        ZonePlacement::Top,
    );
    assert!(game.linked_exile_ids(object.id).is_empty());
    assert_ne!(game.players[1].exile[0].id, exiled);
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::ReturnLinkedExiles {
            object: ObjectPredicateDef::Any,
            zone: ZoneKind::Battlefield,
            grant: None,
            counters: None,
            controller: None,
            transformed: false,
        }),
        &object,
        TriggerContext::empty(),
    );
    assert!(game.battlefield.is_empty());
    assert_eq!(game.players[1].exile.len(), 1);
}

#[test]
#[allow(clippy::too_many_lines)]
fn face_down_exile_masks_arrivals_and_preserves_simultaneous_batches() {
    use crate::card::{SimultaneousTriggerDef, TriggerAggregationDef};
    static WATCHERS: [AbilityDef; 2] = [
        AbilityDef::triggered(
            "Whenever a creature card enters exile.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::HasType(CardType::Creature),
                None,
                Some(ZoneKind::Exile),
            ),
            EffectDef::None,
        ),
        AbilityDef::triggered(
            "Whenever two or more cards enter exile together.",
            TriggerEventDef::Simultaneous(
                SimultaneousTriggerDef::new(
                    &TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Any,
                        None,
                        Some(ZoneKind::Exile),
                    ),
                    TriggerAggregationDef::Once,
                )
                .at_least(2),
            ),
            EffectDef::None,
        ),
    ];
    for prepared in [false, true] {
        for from in [ZoneKind::Battlefield, ZoneKind::Hand, ZoneKind::Graveyard] {
            for face_down in [false, true] {
                let mut game = ready_game();
                game.set_prepared_engine_enabled(prepared);
                game.battlefield
                    .push(creature(410_030, cards::SOL_RING, PlayerId::One));
                for watcher in &WATCHERS {
                    grant(&mut game, GameObjectId(410_030), watcher);
                }
                for id in 410_031..410_033 {
                    match from {
                        ZoneKind::Battlefield => {
                            game.battlefield.push(creature(
                                id,
                                cards::GRIZZLY_BEARS,
                                PlayerId::Two,
                            ));
                        }
                        ZoneKind::Hand => {
                            game.players[1].hand.push(card(
                                id,
                                cards::GRIZZLY_BEARS,
                                PlayerId::Two,
                            ));
                        }
                        _ => game.players[1].graveyard.push(card(
                            id,
                            cards::GRIZZLY_BEARS,
                            PlayerId::Two,
                        )),
                    }
                }
                let mut context = EffectResolutionContext::from(TriggerContext::empty());
                context.bind_object_group(
                    crate::Binding!("exile_test"),
                    (410_031..410_033)
                        .map(|id| {
                            if from == ZoneKind::Battlefield {
                                Target::Permanent(GameObjectId(id))
                            } else {
                                Target::Card(GameObjectId(id))
                            }
                        })
                        .collect(),
                );
                let object = spell(410_034, cards::LIGHTNING_BOLT, PlayerId::One, 0);
                game.resolve_effect_def(
                    ScopedEffect::primary(linked(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "exile_test"
                        ))),
                        face_down,
                    )),
                    &object,
                    context,
                );
                assert_eq!(game.linked_exile_ids(object.id).len(), 2);
                let typed = game
                    .pending_triggers
                    .iter()
                    .filter(|trigger| trigger.text == WATCHERS[0].text)
                    .count();
                let grouped = game
                    .pending_triggers
                    .iter()
                    .filter(|trigger| trigger.text == WATCHERS[1].text)
                    .count();
                assert_eq!(
                    typed,
                    if face_down { 0 } else { 2 },
                    "arrival characteristics, from {from:?}"
                );
                assert_eq!(grouped, 1, "one simultaneous instruction, from {from:?}");
                for viewer in [PlayerId::One, PlayerId::Two] {
                    let observation = game.observe(viewer);
                    assert_eq!(observation.exiles[1].len(), if face_down { 0 } else { 2 });
                    assert_eq!(
                        observation.face_down_exile_sizes[1],
                        if face_down { 2 } else { 0 }
                    );
                }
            }
        }
    }
}
