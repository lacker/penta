//! Auras that prevent all damage in one direction. The two point opposite
//! ways and are one field apart in the matcher, so each is checked against
//! the other's direction: the Muzzle's creature still takes damage, and the
//! Inviolable one still deals it.

use super::*;

/// A Grizzly Bears of player one's carrying `aura`, opposite a Serra Angel
/// of player two's.
fn enchanted(aura: Option<CardDefinitionId>) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut bear = creature(79_000, cards::GRIZZLY_BEARS, PlayerId::One);
    bear.entered_controller_turn = 0;
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    let mut angel = creature(79_001, cards::SERRA_ANGEL, PlayerId::Two);
    angel.entered_controller_turn = 0;
    let angel_id = angel.card.id;
    game.battlefield.push(angel);

    if let Some(definition) = aura {
        let spell = card(79_010, definition, PlayerId::One);
        let spell_id = spell.id;
        game.players[0].hand.push(spell);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 2);
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
    }
    (game, bear_id, angel_id)
}

/// Runs one combat where `attacker` attacks and `blocker` blocks it.
fn fight(mut game: Game, attacker: GameObjectId, blocker: GameObjectId, active: PlayerId) -> Game {
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == attacker)
        .expect("the attacker is on the battlefield")
        .attacking = true;
    let defender = active.opponent();
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == attacker)
        .expect("the attacker is on the battlefield")
        .attack_defender = Some(AttackDefender::Player(defender));
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == blocker)
        .expect("the blocker is on the battlefield")
        .blocking = vec![attacker];
    game.active_player = active;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.finish_declaring_blockers();
    drain_pending(&mut game);
    game.step = Step::CombatDamage;
    game.begin_combat_damage_assignment();
    take_default_combat_assignment(&mut game);
    for _ in 0..8 {
        drain_pending(&mut game);
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }
    game
}

fn alive(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == id)
}

#[test]
fn an_unenchanted_bears_dies_to_the_angel() {
    let (game, bear, angel) = enchanted(None);
    let game = fight(game, angel, bear, PlayerId::Two);
    assert!(!alive(&game, bear), "four damage kills a 2/2");
    assert!(alive(&game, angel), "and two does not kill a 4/4");
}

#[test]
fn inviolability_stops_the_damage_coming_in() {
    let (game, bear, angel) = enchanted(Some(cards::INVIOLABILITY));
    let game = fight(game, angel, bear, PlayerId::Two);
    assert!(alive(&game, bear), "nothing may damage it");
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == angel)
            .expect("the Angel is on the battlefield")
            .damage,
        2,
        "and it still deals its own damage out"
    );
}

#[test]
fn muzzle_stops_the_damage_going_out() {
    let (game, bear, angel) = enchanted(Some(cards::MUZZLE));
    let game = fight(game, bear, angel, PlayerId::One);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == angel)
            .expect("the Angel is on the battlefield")
            .damage,
        0,
        "the muzzled creature deals nothing"
    );
    assert!(
        !alive(&game, bear),
        "and still takes the four coming back at it"
    );
}
