//! Resolution programs that cross several zones or change entry characteristics.
use super::woe_hob_completion::{hand, setup};
use super::*;

#[test]
fn woe_hob_deadly_cover_up_accepts_nonminimal_evidence_and_counts_only_hand_exiles() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        game.players[0].graveyard = game
            .build_zone(PlayerId::One, &[cards::HEARTH_ELEMENTAL, cards::FOREST])
            .unwrap();
        game.players[1].graveyard = game
            .build_zone(PlayerId::Two, &[cards::GRIZZLY_BEARS])
            .unwrap();
        game.players[1].hand = game
            .build_zone(
                PlayerId::Two,
                &[cards::GRIZZLY_BEARS, cards::LIGHTNING_BOLT],
            )
            .unwrap();
        game.players[1].library = game
            .build_zone(
                PlayerId::Two,
                &[cards::FOREST, cards::FOREST, cards::GRIZZLY_BEARS],
            )
            .unwrap();
        game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        game.put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
            .unwrap();
        let spell = hand(&mut game, cards::DEADLY_COVER_UP);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 5);
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action,
            Action::CastSpell { card, sacrifices, .. } if *card == spell && sacrifices.len() == 2)
            })
            .unwrap();
        game.apply(PlayerId::One, action).unwrap();
        assert_eq!(
            game.players[0].exile.len(),
            2,
            "zero-value extra evidence is allowed"
        );
        pass_priority_pair(&mut game);
        assert!(game.battlefield.is_empty());
        choose_decision_by_label(&mut game, PlayerId::One, "Grizzly Bears");
        let decision = game.observe(PlayerId::One).decision.unwrap();
        let selected = decision
            .options
            .iter()
            .filter(|option| option.label.contains("Grizzly Bears"))
            .map(|option| option.id)
            .collect::<Vec<_>>();
        assert_eq!(
            selected.len(),
            3,
            "the remaining graveyard, hand, and library copies are selectable"
        );
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: selected,
            },
        )
        .unwrap();
        drain_pending(&mut game);
        assert_eq!(game.players[1].exile.len(), 4);
        assert_eq!(
            game.players[1].hand.len(),
            2,
            "exactly the one card exiled from hand is replaced"
        );
        assert!(
            game.players[1]
                .hand
                .iter()
                .any(|card| card.definition == cards::FOREST)
        );
        assert!(
            game.players[1]
                .hand
                .iter()
                .any(|card| card.definition == cards::LIGHTNING_BOLT)
        );
        assert_eq!(game.players[1].library.len(), 1);
    }
}

#[test]
fn woe_hob_ghost_vacuum_preserves_ownership_and_installs_arrival_characteristics() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        game.players[1].graveyard = game
            .build_zone(PlayerId::Two, &[cards::GRIZZLY_BEARS])
            .unwrap();
        let graveyard_card = game.players[1].graveyard[0].id;
        let vacuum = game
            .put_onto_battlefield(PlayerId::One, cards::GHOST_VACUUM)
            .unwrap();
        let exile = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action,
            Action::ActivateAbility { source, targets, .. } if *source == vacuum
                && targets.iter().any(|target| target.targets() == [Target::Card(graveyard_card)]))
            })
            .unwrap();
        game.apply(PlayerId::One, exile).unwrap();
        drain_pending(&mut game);
        game.battlefield
            .iter_mut()
            .find(|p| p.card.id == vacuum)
            .unwrap()
            .tapped = false;
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 6);
        let release = game.legal_actions(PlayerId::One).into_iter().find(|action| matches!(action,
            Action::ActivateAbility { source, targets, .. } if *source == vacuum && targets.is_empty())).unwrap();
        game.apply(PlayerId::One, release).unwrap();
        assert!(!game.battlefield.iter().any(|p| p.card.id == vacuum));
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut rebuilt =
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
                .unwrap();
        for state in [&mut game, &mut rebuilt] {
            drain_pending(state);
            let bear = state
                .battlefield
                .iter()
                .find(|p| p.card.definition == cards::GRIZZLY_BEARS)
                .unwrap();
            assert_eq!(bear.card.owner, PlayerId::Two);
            assert_eq!(bear.controller, PlayerId::One);
            assert_eq!(
                (state.power(bear), state.toughness(bear)),
                (Some(1), Some(1))
            );
            assert_eq!(bear.counters(CounterKind::Flying), 1);
            assert!(
                state
                    .effective_subtypes(bear)
                    .contains(crate::card::Subtype::Spirit)
            );
            assert!(
                state
                    .effective_subtypes(bear)
                    .contains(crate::card::Subtype::Bear)
            );
        }
    }
}

#[test]
fn woe_hob_sneak_tokens_choose_their_attack_defenders_independently() {
    let mut game = setup(true);
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let walker = game
        .put_onto_battlefield(PlayerId::Two, cards::JACE_THE_MIND_SCULPTOR)
        .unwrap();
    let creature = game
        .battlefield
        .iter_mut()
        .find(|p| p.card.id == bear)
        .unwrap();
    creature.attacking = true;
    creature.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.blockers_declared = true;
    let spell = hand(&mut game, cards::THE_LAST_RONIN_S_TECHNIQUE);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 2);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action,
        Action::CastSpell { card, .. } if *card == spell)
        })
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(&mut game);
    for defender in [0, 1, 1] {
        let decision = game.observe(PlayerId::One).decision.unwrap();
        let option = decision.options[defender].id;
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![option],
            },
        )
        .unwrap();
    }
    drain_pending(&mut game);
    let tokens = game
        .battlefield
        .iter()
        .filter(|p| p.card.definition.is_token())
        .collect::<Vec<_>>();
    assert_eq!(tokens.len(), 3);
    assert!(tokens.iter().all(|p| p.tapped && p.attacking));
    assert_eq!(
        tokens
            .iter()
            .filter(|p| p.attack_defender == Some(AttackDefender::Planeswalker(walker)))
            .count(),
        2
    );
}

#[test]
fn woe_hob_sneak_enters_tapped_but_does_not_attack_a_departed_planeswalker() {
    let mut game = setup(true);
    let attacker = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let defender = game
        .put_onto_battlefield(PlayerId::Two, cards::JACE_THE_MIND_SCULPTOR)
        .unwrap();
    let permanent = game
        .battlefield
        .iter_mut()
        .find(|p| p.card.id == attacker)
        .unwrap();
    permanent.attacking = true;
    permanent.attack_defender = Some(AttackDefender::Planeswalker(defender));
    game.step = Step::DeclareBlockers;
    game.blockers_declared = true;
    let elektra = hand(&mut game, cards::ELEKTRA_DAUGHTER_OF_THE_HAND);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 3);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == elektra))
        .unwrap();
    game.apply(PlayerId::One, cast).unwrap();
    game.sacrifice_permanent(defender);
    drain_pending(&mut game);
    let ninja = game
        .battlefield
        .iter()
        .find(|p| p.card.definition == cards::ELEKTRA_DAUGHTER_OF_THE_HAND)
        .unwrap();
    assert!(ninja.tapped && !ninja.attacking);
    assert_eq!(ninja.attack_defender, None);
}
