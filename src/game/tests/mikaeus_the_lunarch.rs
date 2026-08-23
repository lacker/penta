//! Mikaeus, the Lunarch: X arrives as his body, and either tap ability can
//! turn those counters into more growth.

use super::*;

fn staged(x: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.catalog = crate::card::catalog().expect("the full catalog is valid");
    let mikaeus = game
        .build_zone(PlayerId::One, &[cards::MIKAEUS_THE_LUNARCH])
        .expect("Mikaeus is cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = mikaeus.id;
    game.players[0].hand.push(mikaeus);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, x);
    (game, id)
}

fn resolve(game: &mut Game) {
    for _ in 0..16 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority)
            .expect("passing priority is legal");
    }
    game.check_state_based_actions();
}

fn cast(game: &mut Game, card: GameObjectId, x: u16) -> GameObjectId {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card: candidate, choices, .. }
                    if *candidate == card && choices.x() == x
            )
        })
        .unwrap_or_else(|| panic!("Mikaeus is castable for X={x}"));
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    resolve(game);
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::MIKAEUS_THE_LUNARCH)
        .expect("Mikaeus resolved")
        .card
        .id
}

fn activate(game: &mut Game, source: GameObjectId, index: usize) {
    let ability = activated_ability_for(game, source, index);
    let action = plain_activation(source, ability);
    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action)
        .expect("the activation is legal");
}

fn counters(game: &Game, object: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == object)
        .map_or(0, |permanent| {
            permanent.counters(CounterKind::PlusOnePlusOne)
        })
}

#[test]
fn chosen_x_is_put_on_mikaeus_as_he_enters() {
    let (mut game, card) = staged(3);
    let mikaeus = cast(&mut game, card, 3);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == mikaeus)
        .expect("Mikaeus is on the battlefield");

    assert_eq!(permanent.counters(CounterKind::PlusOnePlusOne), 3);
    assert_eq!(game.power(permanent), Some(3));
    assert_eq!(game.toughness(permanent), Some(3));
}

#[test]
fn tapping_mikaeus_puts_a_counter_on_him() {
    let (mut game, card) = staged(2);
    let mikaeus = cast(&mut game, card, 2);
    game.turns_started[PlayerId::One.index()] += 1;

    activate(&mut game, mikaeus, 0);
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == mikaeus)
            .expect("Mikaeus remains on the battlefield")
            .tapped,
        "tapping is paid before the ability resolves",
    );
    assert_eq!(counters(&game, mikaeus), 2);

    resolve(&mut game);
    assert_eq!(counters(&game, mikaeus), 3);
}

#[test]
fn removing_a_counter_buffs_only_your_other_creatures() {
    let (mut game, card) = staged(2);
    let mikaeus = cast(&mut game, card, 2);
    game.turns_started[PlayerId::One.index()] += 1;

    let ally = creature(30_000, cards::SAVANNAH_LIONS, PlayerId::One);
    let ally_id = ally.card.id;
    let opponent = creature(30_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    let opponent_id = opponent.card.id;
    game.battlefield.extend([ally, opponent]);

    activate(&mut game, mikaeus, 1);
    assert_eq!(
        counters(&game, mikaeus),
        1,
        "the counter is removed as a cost",
    );
    assert_eq!(counters(&game, ally_id), 0, "the effect is still stacked");

    resolve(&mut game);

    assert_eq!(counters(&game, mikaeus), 1, "Mikaeus excludes himself");
    assert_eq!(counters(&game, ally_id), 1);
    assert_eq!(
        counters(&game, opponent_id),
        0,
        "an opponent's creature is not affected",
    );
}
