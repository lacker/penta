//! An Aura that fires as it leaves the battlefield, and one that watches
//! damage aimed at its host. An Aura goes to the graveyard when its host
//! does, so the first only ever triggers as a consequence of something
//! happening to somebody else -- which is exactly the path nothing else
//! exercises.

use super::*;

/// `aura` on a Grizzly Bears of player one's, with a Serra Angel of player
/// two's standing by as a target.
fn enchanted(aura: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let mut bear = creature(92_000, cards::GRIZZLY_BEARS, PlayerId::One);
    bear.entered_controller_turn = 0;
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    let mut angel = creature(92_001, cards::SERRA_ANGEL, PlayerId::Two);
    angel.entered_controller_turn = 0;
    let angel_id = angel.card.id;
    game.battlefield.push(angel);

    let spell = card(92_010, aura, PlayerId::One);
    let spell_id = spell.id;
    game.players[0].hand.push(spell);
    for color in [ManaColor::Black, ManaColor::Green] {
        game.add_unrestricted_mana(PlayerId::One, color, 2);
    }
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == spell_id
                    && choices
                        .targets()
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(bear_id))
            }
            _ => false,
        })
        .expect("the Aura is castable onto my Bears");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    pass_priority_pair(&mut game);
    (game, bear_id, angel_id)
}

fn settle(game: &mut Game) {
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
    game.check_state_based_actions();
}

fn alive(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == id)
}

#[test]
fn the_chime_fires_when_its_host_takes_it_down() {
    let (mut game, bear, angel) = enchanted(cards::CHIME_OF_NIGHT);
    assert!(alive(&game, angel), "the Angel is there to be aimed at");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == bear)
        .expect("the Bears is on the battlefield")
        .damage = 99;
    game.check_state_based_actions();
    settle(&mut game);

    assert!(!alive(&game, bear), "the Bears died");
    assert!(
        !alive(&game, angel),
        "and the Aura followed it to the graveyard, killing the Angel"
    );
}

#[test]
fn the_wound_kills_its_host_on_any_damage() {
    let (mut game, bear, _) = enchanted(cards::MORTAL_WOUND);
    game.damage_target(Some(Target::Permanent(bear)), 1);
    settle(&mut game);

    assert!(
        !alive(&game, bear),
        "one point of damage is enough once the Wound is on it"
    );
}
