//! Enters-the-battlefield triggers that answer something already on the
//! board or the stack. Two things need covering that the catalog cannot
//! check: that a flash creature's own trigger can still catch a spell it
//! was cast in response to, and that a "nonblack creature" filter really
//! excludes black creatures -- a negated predicate that fails to evaluate
//! admits everything instead.

use super::*;

/// Player one holding `held`, with mana for anything in this file, and the
/// creatures in `board` under player two.
fn staged(held: CardDefinitionId, board: &[CardDefinitionId]) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let spell = card(65_000, held, PlayerId::One);
    let spell_id = spell.id;
    game.players[0].hand.push(spell);
    for (index, definition) in board.iter().enumerate() {
        let mut permanent = creature(
            65_100 + u32::try_from(index).expect("a small fixture"),
            *definition,
            PlayerId::Two,
        );
        permanent.entered_controller_turn = 0;
        game.battlefield.push(permanent);
    }
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);
    (game, spell_id)
}

fn cast_of(game: &Game, spell: CardInstanceId) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .collect()
}

#[test]
fn the_snake_may_be_cast_with_a_spell_on_the_stack() {
    let (mut game, snake) = staged(cards::MYSTIC_SNAKE, &[]);
    assert!(
        !cast_of(&game, snake).is_empty(),
        "flash lets it be cast at all"
    );
    game.stack
        .push(spell(65_500, cards::LIGHTNING_BOLT, PlayerId::Two, 0));
    game.priority = PlayerId::One;

    let responses = cast_of(&game, snake);
    assert!(
        !responses.is_empty(),
        "and it is still castable in response to the Bolt"
    );
}

#[test]
fn the_snake_counters_what_it_was_cast_against() {
    let (mut game, snake) = staged(cards::MYSTIC_SNAKE, &[]);
    game.stack
        .push(spell(65_500, cards::LIGHTNING_BOLT, PlayerId::Two, 0));
    game.priority = PlayerId::One;
    let cast = cast_of(&game, snake)
        .into_iter()
        .next()
        .expect("the Snake is castable in response");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    for _ in 0..12 {
        if game.stack.is_empty() {
            break;
        }
        if game.pending_decisions.is_empty() {
            let holder = game.priority;
            game.apply(holder, Action::PassPriority).unwrap();
        } else {
            let chooser = game.decision_player().expect("a chooser");
            choose_all_offered(&mut game, chooser);
        }
    }

    assert!(
        game.players[1]
            .graveyard
            .iter()
            .any(|card| card.definition == ObjectKind::Card(cards::LIGHTNING_BOLT)),
        "the Bolt was countered into its owner's graveyard"
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == ObjectKind::Card(cards::MYSTIC_SNAKE)),
        "and the Snake stayed behind"
    );
}

/// The labels the Lancer's enters trigger offers as targets.
fn lancer_target_labels(board: &[CardDefinitionId]) -> Vec<String> {
    let (mut game, lancer) = staged(cards::DAKMOR_LANCER, board);
    let cast = cast_of(&game, lancer)
        .into_iter()
        .next()
        .expect("the Lancer is castable");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    // The trigger's target is chosen as it goes on the stack, which is
    // after the creature itself has resolved.
    for _ in 0..8 {
        if game.observe(PlayerId::One).decision.is_some() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }
    game.observe(PlayerId::One)
        .decision
        .map(|decision| {
            decision
                .options
                .iter()
                .map(|option| option.label.clone())
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn the_lancer_passes_over_a_black_creature() {
    let green = lancer_target_labels(&[cards::GRIZZLY_BEARS]);
    assert!(
        green.iter().any(|label| label.contains("Grizzly Bears")),
        "a green Bears is a legal target, but the offer was {green:?}"
    );

    let black = lancer_target_labels(&[cards::ZOMBIE_CANNIBAL]);
    assert!(
        !black.iter().any(|label| label.contains("Zombie Cannibal")),
        "a black Zombie is not, and the negation does not fail open: {black:?}"
    );
}
