//! Interactions that distinguish Time Spiral compositions from their individual primitives.

use super::*;

fn cast_targeted(game: &mut Game, definition: CardDefinitionId, targets: Vec<Target>, x: u16) {
    let held = game
        .build_zone(PlayerId::One, &[definition])
        .unwrap()
        .remove(0);
    let id = held.id;
    game.players[0].hand.push(held);
    game.apply(PlayerId::One, cast_action(id, targets, Vec::new(), x))
        .expect("the fully announced spell is legal");
}

#[test]
fn draining_whelk_reads_countered_spells_chosen_x_after_it_leaves_the_stack() {
    let mut game = ready_game();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 4);
    cast_targeted(
        &mut game,
        cards::FIREBALL,
        vec![Target::Player(PlayerId::Two)],
        3,
    );
    let whelk = game
        .put_onto_battlefield(PlayerId::Two, cards::DRAINING_WHELK)
        .unwrap();
    drain_pending(&mut game);
    let permanent = game
        .battlefield
        .iter()
        .find(|p| p.card.id == whelk)
        .unwrap();
    assert_eq!(permanent.counters(CounterKind::PlusOnePlusOne), 4);
    assert_eq!(game.players[1].life, 20, "the Fireball was countered");
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|c| c.definition == cards::FIREBALL)
    );
}

#[test]
fn psionic_sliver_grants_damage_to_the_opponents_sliver_as_its_own_source() {
    let mut game = ready_game();
    let granter = creature(98_600, cards::PSIONIC_SLIVER, PlayerId::One);
    let granter_id = granter.card.id;
    let mut other = creature(98_601, cards::VENSER_S_SLIVER, PlayerId::Two);
    other.entered_controller_turn = 0;
    let other_id = other.card.id;
    game.battlefield.extend([granter, other]);
    game.turns_started[1] = 5;
    game.priority = PlayerId::Two;
    let action = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, targets, .. }
            if *source == other_id && targets.iter().any(|selection|
                selection.targets().contains(&Target::Player(PlayerId::One))))
        })
        .expect("the opponent can activate the granted ability");
    game.apply(PlayerId::Two, action).unwrap();
    drain_pending(&mut game);
    assert_eq!(game.players[0].life, 18);
    assert!(game.battlefield.iter().any(|p| p.card.id == granter_id));
    assert!(!game.battlefield.iter().any(|p| p.card.id == other_id));
    assert!(
        game.players[1]
            .graveyard
            .iter()
            .any(|c| c.definition == cards::VENSER_S_SLIVER)
    );
}

#[test]
fn truth_or_tale_conserves_the_revealed_cards_across_both_choices() {
    let mut game = ready_game();
    let definitions = [
        cards::FOREST,
        cards::ISLAND,
        cards::MOUNTAIN,
        cards::PLAINS,
        cards::SWAMP,
    ];
    game.players[0].library = game.build_zone(PlayerId::One, &definitions).unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);
    cast_targeted(&mut game, cards::TRUTH_OR_TALE, Vec::new(), 0);
    drain_pending(&mut game);
    drain_pending(&mut game);
    assert!(game.pending_decisions.is_empty());
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.players[0].library.len(), 4);
    for definition in definitions {
        assert_eq!(
            game.players[0]
                .hand
                .iter()
                .chain(&game.players[0].library)
                .filter(|c| c.definition == definition)
                .count(),
            1
        );
    }
}

#[test]
fn scion_copies_the_dragon_card_it_put_into_the_graveyard() {
    let mut game = ready_game();
    let scion = creature(98_610, cards::SCION_OF_THE_UR_DRAGON, PlayerId::One);
    let source = scion.card.id;
    game.battlefield.push(scion);
    game.players[0].library = game
        .build_zone(PlayerId::One, &[cards::FOREST, cards::SHIVAN_DRAGON])
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source: id, .. } if *id == source),
        )
        .expect("Scion offers its search ability");
    game.apply(PlayerId::One, action).unwrap();
    drain_pending(&mut game);
    let permanent = game
        .battlefield
        .iter()
        .find(|p| p.card.id == source)
        .unwrap();
    assert_eq!(game.power(permanent), Some(5));
    assert_eq!(game.toughness(permanent), Some(5));
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|c| c.definition == cards::SHIVAN_DRAGON)
    );
    assert_eq!(game.players[0].library.len(), 1);
}

#[test]
fn phthisis_reads_power_toughness_and_controller_after_destroying_its_target() {
    let mut game = ready_game();
    let target = creature(98_620, cards::AIR_ELEMENTAL, PlayerId::Two);
    let id = target.card.id;
    game.battlefield.push(target);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 8);
    cast_targeted(&mut game, cards::PHTHISIS, vec![Target::Permanent(id)], 0);
    drain_pending(&mut game);
    assert!(!game.battlefield.iter().any(|p| p.card.id == id));
    assert_eq!(game.players[1].life, 12);
}

#[test]
fn lim_dul_returns_the_dead_opposing_creature_under_your_control_as_a_zombie() {
    let mut game = ready_game();
    game.battlefield.push(creature(
        98_630,
        cards::LIM_D_L_THE_NECROMANCER,
        PlayerId::One,
    ));
    let target = creature(98_631, cards::GRIZZLY_BEARS, PlayerId::Two);
    let id = target.card.id;
    game.battlefield.push(target);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 4);
    cast_targeted(&mut game, cards::TERROR, vec![Target::Permanent(id)], 0);
    for _ in 0..16 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority).unwrap();
    }
    choose_decision_by_label(&mut game, PlayerId::One, "Pay the cost");
    drain_pending(&mut game);
    drain_pending(&mut game);
    let returned = game
        .battlefield
        .iter()
        .find(|p| p.card.definition == ObjectKind::Card(cards::GRIZZLY_BEARS))
        .expect("the dead card returned");
    assert_eq!(returned.controller, PlayerId::One);
    assert_eq!(returned.card.owner, PlayerId::Two);
    assert!(game.effective_subtypes(returned).contains(&"Zombie"));
    assert!(game.effective_subtypes(returned).contains(&"Bear"));
}

#[test]
fn stuffy_doll_survives_lethal_damage_and_reflects_it_to_the_chosen_player() {
    let mut game = ready_game();
    let doll = game
        .put_onto_battlefield(PlayerId::One, cards::STUFFY_DOLL)
        .unwrap();
    choose_decision_by_label(&mut game, PlayerId::One, "Opponent");
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    cast_targeted(
        &mut game,
        cards::LIGHTNING_BOLT,
        vec![Target::Permanent(doll)],
        0,
    );
    drain_pending(&mut game);
    assert!(game.battlefield.iter().any(|p| p.card.id == doll));
    assert_eq!(game.players[1].life, 17);
    assert_eq!(game.players[0].life, 20);
}
