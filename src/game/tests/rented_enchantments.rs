//! The halves of these cumulative-upkeep cards that are not the upkeep. The
//! rent is already covered elsewhere; what is new here is a target
//! restriction narrow enough to fail open if the predicate is wrong, and a
//! penalty that has to apply to every creature including its controller's.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

fn resolve(game: &mut Game) {
    for _ in 0..12 {
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

fn power_of(game: &Game, id: GameObjectId) -> i16 {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the creature is on the battlefield");
    game.power(permanent).expect("a creature")
}

/// "Enchant red or green creature" is narrow enough that a predicate which
/// failed open would look identical on a mono-red board, so the colourless
/// creature is the one that proves the restriction.
#[test]
fn mind_harness_only_takes_a_red_or_green_creature() {
    let mut game = ready();
    let held = card(79_000, cards::MIND_HARNESS, PlayerId::One);
    let held_id = held.id;
    game.players[0].hand.push(held);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 3);
    let goblin = creature(79_010, cards::RAGING_GOBLIN, PlayerId::Two);
    let goblin_id = goblin.card.id;
    game.battlefield.push(goblin);
    let bears = creature(79_011, cards::GRIZZLY_BEARS, PlayerId::Two);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    let thopter = creature(79_012, cards::ORNITHOPTER, PlayerId::Two);
    let thopter_id = thopter.card.id;
    game.battlefield.push(thopter);

    let target_of = |action: &Action| match action {
        Action::CastSpell { choices, .. } => choices
            .targets()
            .first()
            .and_then(|selection| selection.targets().first().copied()),
        _ => None,
    };
    let offered: Vec<_> = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == held_id))
        .collect();
    let hosts: Vec<_> = offered.iter().filter_map(target_of).collect();
    assert_eq!(
        hosts,
        vec![Target::Permanent(goblin_id), Target::Permanent(bears_id),],
        "the red and the green one, and not the colourless {thopter_id:?}"
    );

    let cast = offered
        .into_iter()
        .find(|action| target_of(action) == Some(Target::Permanent(goblin_id)))
        .expect("the Goblin is a legal host");
    game.apply(PlayerId::One, cast).expect("it is cast");
    resolve(&mut game);
    let stolen = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == goblin_id)
        .expect("the Goblin is still on the battlefield");
    assert_eq!(
        stolen.controller,
        PlayerId::One,
        "and it changed hands while the Aura holds"
    );
}

/// "All creatures" means all of them, and the activated half stacks on top of
/// the standing one rather than replacing it.
#[test]
fn fyndhorn_pollen_shrinks_both_sides_and_stacks_with_itself() {
    let mut game = ready();
    let mine = creature(79_100, cards::GRIZZLY_BEARS, PlayerId::One);
    let mine_id = mine.card.id;
    game.battlefield.push(mine);
    let theirs = creature(79_101, cards::GRIZZLY_BEARS, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);
    assert_eq!(
        (power_of(&game, mine_id), power_of(&game, theirs_id)),
        (2, 2),
        "printed on both sides"
    );

    game.battlefield
        .push(creature(79_102, cards::FYNDHORN_POLLEN, PlayerId::One));
    assert_eq!(
        (power_of(&game, mine_id), power_of(&game, theirs_id)),
        (1, 1),
        "the standing penalty does not spare its own controller"
    );

    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 4);
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { .. }))
        .expect("the pump-in-reverse is offered");
    game.apply(PlayerId::One, activation).expect("it activates");
    resolve(&mut game);
    assert_eq!(
        (power_of(&game, mine_id), power_of(&game, theirs_id)),
        (0, 0),
        "and the activated half is another -1/-0 on top"
    );
}
