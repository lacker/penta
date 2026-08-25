//! Focused coverage for Mirrodin Besieged declarations that exercise shared
//! rules-engine abstractions rather than card-owned procedures.

use super::*;

fn resolve(game: &mut Game) {
    for _ in 0..16 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

#[test]
fn leonin_relic_warder_returns_what_it_exiled_when_it_leaves() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let artifact = game
        .put_onto_battlefield(PlayerId::Two, cards::SOL_RING)
        .expect("Sol Ring is cataloged");
    let warder = game
        .build_zone(PlayerId::One, &[cards::LEONIN_RELIC_WARDER])
        .expect("Leonin Relic-Warder is cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let warder_card = warder.id;
    game.players[0].hand.push(warder);
    game.turns_started = [1, 1];
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 2);
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == warder_card))
        .expect("two white mana casts Leonin Relic-Warder");
    game.apply(PlayerId::One, cast).expect("the Warder casts");
    resolve(&mut game);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the enters trigger offers an artifact or enchantment");
    let target = decision
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(id, _)| id == artifact))
        .expect("Sol Ring is a legal target")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![target],
        },
    )
    .expect("the target is legal");
    resolve(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Do it");
    resolve(&mut game);

    assert!(
        game.players[1]
            .exile
            .iter()
            .any(|card| card.definition == cards::SOL_RING),
        "the Warder exiles the artifact",
    );
    let warder_permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::LEONIN_RELIC_WARDER)
        .expect("the Warder remains on the battlefield")
        .card
        .id;
    game.move_permanents_to_graveyard(&[warder_permanent]);
    resolve(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SOL_RING),
        "the linked artifact returns when the Warder leaves",
    );
}

#[test]
fn myr_welder_gains_activated_abilities_from_a_linked_noncreature_artifact() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[1].graveyard.clear();
    let sol_ring = game
        .build_zone(PlayerId::Two, &[cards::SOL_RING])
        .expect("Sol Ring is cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let sol_ring_id = sol_ring.id;
    game.players[1].graveyard.push(sol_ring);
    let welder = game
        .put_onto_battlefield(PlayerId::One, cards::MYR_WELDER)
        .expect("Myr Welder is cataloged");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == welder)
        .expect("Myr Welder is on the battlefield")
        .entered_controller_turn = 0;
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == welder
                    && targets
                        .iter()
                        .any(|selection| selection.targets().contains(&Target::Card(sol_ring_id)))
            }
            _ => false,
        })
        .expect("Myr Welder can exile the artifact card");
    game.apply(PlayerId::One, action)
        .expect("the imprint ability activates");
    resolve(&mut game);

    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == welder)
        .expect("Myr Welder remains on the battlefield");
    let mut texts = Vec::new();
    let _ = game.visit_effective_abilities(permanent, |effective| {
        if matches!(
            effective.ability.definition,
            crate::card::DeclarativeAbilityDef::Activated(_)
                | crate::card::DeclarativeAbilityDef::ActivatedMana(_)
        ) {
            texts.push(effective.ability.text);
        }
        std::ops::ControlFlow::Continue(())
    });

    assert!(
        texts.contains(&"{T}: Add {C}{C}."),
        "the linked noncreature artifact grants its activated ability: {texts:?}",
    );
}
