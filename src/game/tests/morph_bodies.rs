//! Morph end to end: a nine-drop cast face down for three as a 2/2, and
//! turned face up later for its morph cost. Nothing about the printed card
//! is visible until it flips, so the only proof the morph cost is wired up
//! is that the body changes size when it is paid.

use super::*;

/// Player one holding `held` with `mana` of everything.
fn holding(held: CardDefinitionId, mana: u16) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let card = card(78_000, held, PlayerId::One);
    let card_id = card.id;
    game.players[0].hand.push(card);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, mana);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, mana);
    (game, card_id)
}

/// Every way of casting `held` that is offered.
fn casts(game: &Game, held: CardInstanceId) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == held))
        .collect()
}

fn only_permanent(game: &Game) -> (i16, i16) {
    let permanent = game
        .battlefield
        .first()
        .expect("the creature resolved onto the battlefield");
    (
        game.power(permanent).expect("power"),
        game.toughness(permanent).expect("toughness"),
    )
}

fn resolve(game: &mut Game) {
    for _ in 0..8 {
        drain_pending(game);
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }
}

#[test]
fn three_mana_is_enough_to_cast_it_face_down() {
    let (game, colossus) = holding(cards::KROSAN_COLOSSUS, 3);
    assert_eq!(
        casts(&game, colossus).len(),
        1,
        "three mana pays for the face-down cast and nothing else"
    );
}

#[test]
fn the_face_down_body_is_a_two_by_two() {
    let (mut game, colossus) = holding(cards::KROSAN_COLOSSUS, 3);
    let cast = casts(&game, colossus)
        .into_iter()
        .next()
        .expect("the face-down cast is offered");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    resolve(&mut game);

    assert_eq!(
        only_permanent(&game),
        (2, 2),
        "a face-down creature is a 2/2 whatever is printed on it"
    );
}

#[test]
fn paying_the_morph_cost_turns_it_into_a_colossus() {
    let (mut game, colossus) = holding(cards::KROSAN_COLOSSUS, 3);
    let cast = casts(&game, colossus)
        .into_iter()
        .next()
        .expect("the face-down cast is offered");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    resolve(&mut game);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 6);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);

    let flip = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::TurnFaceUp { .. }))
        .expect("the morph cost is payable");
    game.apply(PlayerId::One, flip).expect("it turns face up");

    assert_eq!(
        only_permanent(&game),
        (9, 9),
        "the printed body once the morph cost is paid"
    );
}

#[test]
fn without_the_morph_mana_it_stays_face_down() {
    let (mut game, colossus) = holding(cards::KROSAN_COLOSSUS, 3);
    let cast = casts(&game, colossus)
        .into_iter()
        .next()
        .expect("the face-down cast is offered");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    resolve(&mut game);

    assert!(
        !game
            .legal_actions(PlayerId::One)
            .into_iter()
            .any(|action| matches!(action, Action::TurnFaceUp { .. })),
        "nothing left to pay {{6}}{{G}}{{G}} with"
    );
}

/// Every catalog card that prints a morph cost must also be castable face
/// down: the cost and the permission are two separate clauses, and one
/// without the other is a card whose whole mode is unreachable.
#[test]
fn every_morph_cost_comes_with_a_face_down_cast() {
    let catalog = crate::card::catalog().expect("the built-in catalog is valid");
    let mut missing = Vec::new();
    for definition in catalog.definitions() {
        for part in &definition.parts {
            if part.rules.morph_cost().is_none() {
                continue;
            }
            let castable = part.rules.ability_clauses().iter().any(|ability| {
                matches!(
                    ability.definition,
                    DeclarativeAbilityDef::AlternativeCast(alternative)
                        if matches!(
                            alternative.kind,
                            AlternativeCastKindDef::FaceDown { .. }
                        )
                )
            });
            if !castable {
                missing.push(definition.name.clone());
            }
        }
    }
    assert!(
        missing.is_empty(),
        "a printed morph cost with no face-down cast: {missing:?}"
    );
}
