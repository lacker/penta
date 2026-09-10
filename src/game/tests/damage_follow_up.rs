use super::*;

#[test]
fn multiple_damaged_recipients_run_one_follow_up_and_zero_damage_runs_none() {
    for amount in [0, 1] {
        let mut game = ready_game();
        let source = spell(40_000, cards::LIGHTNING_BOLT, PlayerId::One, 0);
        game.resolve_effect_def(
            ScopedEffect::primary(EffectDef::DealDamageWithFollowUp(
                crate::card::DamageFollowUpDef {
                    recipient: EffectRecipientDef::players(PlayerSetDef::All),
                    amount: ValueDef::Constant(i32::from(amount)),
                    then: &EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(5),
                    },
                },
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
        ScopedEffect::primary(EffectDef::Sequence(&[
            EffectDef::DealDamageWithFollowUp(crate::card::DamageFollowUpDef {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
                then: &EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(5),
                    },
                },
            }),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ])),
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
