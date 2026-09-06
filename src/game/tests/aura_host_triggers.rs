//! Auras that watch their own host. The trigger names the attached
//! permanent rather than the Aura, and it fires as that permanent leaves --
//! so what needs covering is that the Aura sees the death at all, and that
//! what it hands back is the card the creature became rather than the
//! creature that is gone.

use super::*;

/// `aura` resolved onto a Grizzly Bears of `host_owner`'s.
fn enchanted(aura: CardDefinitionId, host_owner: PlayerId) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let mut bear = creature(87_000, cards::GRIZZLY_BEARS, host_owner);
    bear.entered_controller_turn = 0;
    let bear_id = bear.card.id;
    game.battlefield.push(bear);

    let spell = card(87_100, aura, PlayerId::One);
    let spell_id = spell.id;
    game.players[0].hand.push(spell);
    for color in [ManaColor::White, ManaColor::Green, ManaColor::Red] {
        game.add_unrestricted_mana(PlayerId::One, color, 2);
    }
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);
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
        .expect("the Aura is castable onto the Bears");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    pass_priority_pair(&mut game);
    (game, bear_id)
}

fn kill(game: &mut Game, id: GameObjectId) {
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == id)
        .expect("the creature is on the battlefield")
        .damage = 99;
    game.check_state_based_actions();
    drain_pending(game);
    game.check_state_based_actions();
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
fn the_embrace_hands_the_creature_card_back() {
    let (mut game, bear) = enchanted(cards::SQUEE_S_EMBRACE, PlayerId::One);
    kill(&mut game, bear);

    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == ObjectKind::Card(cards::GRIZZLY_BEARS)),
        "the Bears came back to its owner's hand"
    );
    assert!(
        !game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == ObjectKind::Card(cards::GRIZZLY_BEARS)),
        "and did not stay in the graveyard as well"
    );
}

#[test]
fn the_guide_leaves_an_elephant_behind() {
    let (mut game, bear) = enchanted(cards::ELEPHANT_GUIDE, PlayerId::One);
    assert_eq!(
        game.battlefield.len(),
        2,
        "the Bears and the Aura, before anything dies"
    );
    kill(&mut game, bear);

    let elephant = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition.is_token())
        .expect("an Elephant token was created");
    assert_eq!(
        (
            game.power(elephant).expect("power"),
            game.toughness(elephant).expect("toughness")
        ),
        (3, 3),
        "a 3/3 in place of the creature that died"
    );
}

#[test]
fn guilty_conscience_kills_what_it_enchants() {
    let (mut game, bear) = enchanted(cards::GUILTY_CONSCIENCE, PlayerId::Two);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == bear)
        .expect("the Bears is on the battlefield")
        .attacking = true;
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == bear)
        .expect("the Bears is on the battlefield")
        .attack_defender = Some(AttackDefender::Player(PlayerId::One));
    game.active_player = PlayerId::Two;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.finish_declaring_blockers();
    drain_pending(&mut game);
    game.step = Step::CombatDamage;
    game.begin_combat_damage_assignment();
    take_default_combat_assignment(&mut game);
    for _ in 0..12 {
        drain_pending(&mut game);
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();

    assert_eq!(game.players[0].life, 18, "the Bears connected for two");
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == bear),
        "and took two back from the Aura, which is lethal to a 2/2"
    );
}
