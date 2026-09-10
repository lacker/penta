use super::*;

#[test]
fn multiple_damaged_recipients_run_one_follow_up_and_zero_damage_runs_none() {
    for amount in [0, 1] {
        let mut game = ready_game();
        let source = spell(40_000, cards::LIGHTNING_BOLT, PlayerId::One, 0);
        game.resolve_effect_def(
            ScopedEffect::primary(EffectDef::DealDamage(
                crate::card::DamageDef::new(
                    EffectRecipientDef::players(PlayerSetDef::All),
                    ValueDef::Constant(i32::from(amount)),
                )
                .with_follow_up(crate::card::DamageFollowUpDef::IfDealtToIntended(
                    &EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(5),
                    },
                )),
            )),
            &source,
            TriggerContext::empty(),
        );

        assert_eq!(game.players[0].life, if amount == 0 { 20 } else { 24 });
        assert_eq!(game.players[1].life, 20 - amount);
    }
}

#[test]
fn a_decision_in_the_follow_up_preserves_the_outer_sequence_tail() {
    let mut game = ready_game();
    let source = spell(40_000, cards::LIGHTNING_BOLT, PlayerId::One, 0);
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::Sequence(
            &const {
                [
                    EffectDef::DealDamage(
                        crate::card::DamageDef::new(
                            EffectRecipientDef::Opponent,
                            ValueDef::Constant(1),
                        )
                        .with_follow_up(
                            crate::card::DamageFollowUpDef::IfDealtToIntended(&EffectDef::May {
                                player: EffectRecipientDef::Controller,
                                effect: &EffectDef::GainLife {
                                    recipient: EffectRecipientDef::Controller,
                                    amount: ValueDef::Constant(5),
                                },
                            }),
                        ),
                    ),
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                    },
                ]
            },
        )),
        &source,
        TriggerContext::empty(),
    );

    assert_eq!(game.players[0].life, 20, "the tail waits for the decision");
    assert_eq!(game.players[1].life, 19);
    let decision = game.pending_decisions[0].observation.clone();
    let decline = decision.options[0].id;
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decline],
        },
    )
    .expect("decline the optional life gain");

    assert_eq!(game.players[0].life, 22, "the outer tail still resolves");
    assert!(game.pending_decisions.is_empty());
}

fn redirected_batch() -> (Game, StackObject, GameObjectId) {
    let mut game = ready_game();
    let mut artifact = creature(40_001, cards::MISHRA_S_WAR_MACHINE, PlayerId::One);
    artifact.counters.set(CounterKind::Lifelink, 1);
    let artifact_id = artifact.card.id;
    let martyrs = creature(40_002, cards::MARTYRS_OF_KORLIS, PlayerId::Two);
    let martyrs_id = martyrs.card.id;
    game.battlefield.extend([artifact, martyrs]);
    let resolving = spell_with_targets(
        40_000,
        cards::LIGHTNING_BOLT,
        PlayerId::One,
        vec![Target::Permanent(artifact_id)],
        0,
    );
    (game, resolving, martyrs_id)
}

#[test]
fn explicit_sources_and_batch_riders_use_actual_recipients_and_damage_attribution() {
    let (mut game, resolving, martyrs) = redirected_batch();
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::DealDamage(
            crate::card::DamageDef::simultaneous(
                &const {
                    [
                        crate::card::DamageAssignmentDef::from(
                            ObjectRefDef::Target(TargetIndex::PRIMARY),
                            EffectRecipientDef::Opponent,
                            ValueDef::Constant(3),
                        ),
                        crate::card::DamageAssignmentDef::from_effect(
                            EffectRecipientDef::Controller,
                            ValueDef::Constant(1),
                        ),
                    ]
                },
            )
            .with_follow_up(crate::card::DamageFollowUpDef::ApplyToDamaged {
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotGainLife),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            }),
        )),
        &resolving,
        TriggerContext::empty(),
    );

    assert_eq!(
        game.players[0].life, 22,
        "only the artifact's three damage has lifelink"
    );
    assert_eq!(
        game.players[1].life, 20,
        "Martyrs took the artifact's damage"
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == martyrs)
            .unwrap()
            .damage,
        3
    );
    game.gain_life(PlayerId::One, 1);
    game.gain_life(PlayerId::Two, 1);
    assert_eq!(
        game.players[0].life, 22,
        "the damaged player gets the rider"
    );
    assert_eq!(
        game.players[1].life, 21,
        "a fully redirected recipient gets no rider"
    );
}

#[test]
fn a_batch_conditional_rider_ignores_damage_redirected_to_other_recipients() {
    let (mut game, resolving, _) = redirected_batch();
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::DealDamage(
            crate::card::DamageDef::simultaneous(
                &const {
                    [crate::card::DamageAssignmentDef::from(
                        ObjectRefDef::Target(TargetIndex::PRIMARY),
                        EffectRecipientDef::Opponent,
                        ValueDef::Constant(3),
                    )]
                },
            )
            .with_follow_up(crate::card::DamageFollowUpDef::IfDealtToIntended(
                &EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(5),
                },
            )),
        )),
        &resolving,
        TriggerContext::empty(),
    );

    assert_eq!(
        game.players[0].life, 23,
        "lifelink gains three, but the conditional rider does not run"
    );
    assert_eq!(game.players[1].life, 20);
}

#[test]
fn single_and_batch_assignments_evaluate_recipient_counts_the_same_way() {
    for assignments in [
        crate::card::DamageAssignmentsDef::One(crate::card::DamageAssignmentDef::from_effect(
            EffectRecipientDef::players(PlayerSetDef::All),
            ValueDef::ResolvedRecipientCount,
        )),
        crate::card::DamageAssignmentsDef::Many(
            &const {
                [crate::card::DamageAssignmentDef::from_effect(
                    EffectRecipientDef::players(PlayerSetDef::All),
                    ValueDef::ResolvedRecipientCount,
                )]
            },
        ),
    ] {
        let mut game = ready_game();
        let resolving = spell(40_000, cards::LIGHTNING_BOLT, PlayerId::One, 0);
        game.resolve_effect_def(
            ScopedEffect::primary(EffectDef::DealDamage(crate::card::DamageDef {
                assignments,
                follow_up: None,
            })),
            &resolving,
            TriggerContext::empty(),
        );
        assert_eq!(game.players[0].life, 18);
        assert_eq!(game.players[1].life, 18);
    }
}
