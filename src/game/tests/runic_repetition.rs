//! Runic Repetition: recognizing flashback as a card ability in exile.

use super::*;

fn runic_game() -> Game {
    let mut game = ready_game();
    game.catalog = crate::card::catalog().unwrap();
    game
}

fn add_runic_repetition(game: &mut Game, id: u32) -> CardInstance {
    let runic = card(id, cards::RUNIC_REPETITION, PlayerId::One);
    game.players[PlayerId::One.index()].hand.push(runic.clone());
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;
    runic
}

#[test]
fn returns_an_owned_card_that_naturally_has_flashback() {
    let mut game = runic_game();
    let runic = add_runic_repetition(&mut game, 20_000);
    let think_twice = card(20_001, cards::THINK_TWICE, PlayerId::One);
    let target = think_twice.id;
    game.players[PlayerId::One.index()].exile.push(think_twice);

    let cast = cast_action(runic.id, vec![Target::Card(target)], Vec::new(), 0);
    assert!(
        game.legal_actions(PlayerId::One).contains(&cast),
        "a naturally flashbacked card is a legal target however it was exiled"
    );
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    assert!(game.players[PlayerId::One.index()].exile.is_empty());
    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::THINK_TWICE),
        "the chosen card returned to its owner's hand"
    );
}

#[test]
fn rejects_cards_without_a_current_natural_flashback_ability() {
    let mut game = runic_game();
    let runic = add_runic_repetition(&mut game, 20_010);

    let ordinary = card(20_011, cards::LIGHTNING_BOLT, PlayerId::One);
    let ordinary_target = ordinary.id;
    game.players[PlayerId::One.index()].exile.push(ordinary);

    let theirs = card(20_012, cards::THINK_TWICE, PlayerId::Two);
    let their_target = theirs.id;
    game.players[PlayerId::Two.index()].exile.push(theirs);

    let face_down = card(20_013, cards::THINK_TWICE, PlayerId::One);
    let face_down_target = face_down.id;
    game.players[PlayerId::One.index()].exile.push(face_down);
    game.permit_look_while_exiled(face_down_target, PlayerId::One);

    for target in [ordinary_target, their_target, face_down_target] {
        assert!(
            !game.legal_actions(PlayerId::One).contains(&cast_action(
                runic.id,
                vec![Target::Card(target)],
                Vec::new(),
                0,
            )),
            "only an owned, face-up card with flashback is legal"
        );
    }
}

#[test]
fn graveyard_only_granted_flashback_does_not_follow_the_card_into_exile() {
    let mut game = runic_game();
    let runic = add_runic_repetition(&mut game, 20_020);
    let bolt = card(20_021, cards::LIGHTNING_BOLT, PlayerId::One);
    let graveyard_id = bolt.id;
    game.players[PlayerId::One.index()].graveyard.push(bolt);
    game.temporary_ability_grants.push(TemporaryAbilityGrant {
        object: graveyard_id,
        ability: CARD_COST_FLASHBACK,
    });

    let (exiled, destination) = game
        .move_card_from_nonbattlefield_zone(
            graveyard_id,
            ZoneKind::Graveyard,
            ZoneKind::Exile,
            ZoneMoveCause::Rules,
            None,
        )
        .expect("the granted card moves to exile");
    assert_eq!(destination, ZoneKind::Exile);
    assert_ne!(
        exiled.id, graveyard_id,
        "the zone change minted a new object"
    );

    assert!(
        !game.legal_actions(PlayerId::One).contains(&cast_action(
            runic.id,
            vec![Target::Card(exiled.id)],
            Vec::new(),
            0,
        )),
        "a grant tied to the graveyard object no longer applies in exile"
    );
}
