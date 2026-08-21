//! Goblin Rabblemaster: a Goblin every turn, a board that has to attack,
//! and a body that grows with the crowd it sends.

use super::*;

/// Player One with a Rabblemaster out since last turn and `others` beside
/// it, on Player One's turn just before combat.
fn staged(
    others: &[CardDefinitionId],
    goblin_tokens: usize,
) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let rabblemaster = game
        .put_onto_battlefield(PlayerId::One, cards::GOBLIN_RABBLEMASTER)
        .expect("cataloged");
    let mut friends = Vec::new();
    for definition in others {
        friends.push(
            game.put_onto_battlefield(PlayerId::One, *definition)
                .expect("cataloged"),
        );
    }
    for index in 0..goblin_tokens {
        let permanent = token_permanent(
            90_000 + u32::try_from(index).expect("the fixture has few tokens"),
            tokens::creature(&["Goblin"], &[ManaColor::Red], 1, 1),
            PlayerId::One,
        );
        friends.push(permanent.card.id);
        game.battlefield.push(permanent);
    }
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [1, 1];
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, rabblemaster, friends)
}

/// Passes until the game stops, answering nothing: this card asks no
/// questions of its own.
fn settle(game: &mut Game) {
    for _ in 0..24 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

/// Walks to the declare-attackers step, letting the beginning-of-combat
/// trigger resolve on the way.
fn reach_declare_attackers(game: &mut Game) {
    for _ in 0..24 {
        if game.step == Step::DeclareAttackers && !game.attackers_declared {
            return;
        }
        settle(game);
        if game.step == Step::DeclareAttackers && !game.attackers_declared {
            return;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

fn goblin_tokens(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(
                permanent,
                token_with_haste(tokens::creature(&["Goblin"], &[ManaColor::Red], 1, 1)),
            )
        })
        .count()
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("it is on the battlefield")
}

/// One Goblin at the beginning of each of your combats, and it can attack
/// the turn it arrives.
#[test]
fn it_makes_a_hasty_goblin_at_the_beginning_of_combat() {
    let (mut game, _rabblemaster, _friends) = staged(&[], 0);
    assert_eq!(goblin_tokens(&game), 0, "nothing yet");

    reach_declare_attackers(&mut game);

    assert_eq!(goblin_tokens(&game), 1, "one Goblin token");
    let token = game
        .battlefield
        .iter()
        .find(|permanent| {
            is_token_with(
                permanent,
                token_with_haste(tokens::creature(&["Goblin"], &[ManaColor::Red], 1, 1)),
            )
        })
        .expect("a token was made");
    assert!(
        game.permanent_has_executable_keyword(token, KeywordAbility::Haste),
        "with haste, which is the only reason it is worth making now",
    );
}

/// "Attack each combat if able" is granted, not printed: the game will not
/// let the attack step finish while an untapped Goblin is sitting home.
#[test]
fn other_goblins_are_made_to_attack() {
    let (mut game, _rabblemaster, friends) = staged(&[], 1);
    let goblin = friends[0];
    reach_declare_attackers(&mut game);

    assert!(
        game.permanent_has_executable_keyword(
            permanent(&game, goblin),
            KeywordAbility::AttacksEachCombatIfAble
        ),
        "the other Goblin was handed the requirement",
    );
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .contains(&Action::FinishDeclaringAttackers),
        "and the step cannot be finished while it stays home",
    );
}

/// The Rabblemaster is a Goblin, but "other" excludes it: it is never made
/// to attack by its own clause.
#[test]
fn the_rabblemaster_is_not_made_to_attack_by_itself() {
    let (mut game, rabblemaster, _friends) = staged(&[], 0);
    reach_declare_attackers(&mut game);

    assert!(
        !game.permanent_has_executable_keyword(
            permanent(&game, rabblemaster),
            KeywordAbility::AttacksEachCombatIfAble
        ),
        "\"other\" leaves the Rabblemaster out",
    );
}

/// A creature that is not a Goblin is left alone, however friendly.
#[test]
fn a_nongoblin_is_left_alone() {
    let (mut game, _rabblemaster, friends) = staged(&[cards::SAVANNAH_LIONS], 0);
    let lions = friends[0];
    reach_declare_attackers(&mut game);

    assert!(
        !game.permanent_has_executable_keyword(
            permanent(&game, lions),
            KeywordAbility::AttacksEachCombatIfAble
        ),
        "a Savannah Lions is nobody's Goblin",
    );
}

/// The attack trigger counts the crowd: two other attacking Goblins make a
/// 2/2 into a 4/2.
#[test]
fn it_grows_by_one_for_each_other_attacking_goblin() {
    let (mut game, rabblemaster, friends) = staged(&[], 2);
    reach_declare_attackers(&mut game);

    let _ = friends;
    // Everything goes, which is the point of the card: the two Goblin
    // tokens staged here, the one its own combat trigger just made, and the
    // Rabblemaster itself.
    while let Some(action) = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::DeclareAttacker { .. }))
    {
        game.apply(PlayerId::One, action).expect("it attacks");
    }
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("every Goblin that had to attack did");
    settle(&mut game);

    assert_eq!(
        game.power(permanent(&game, rabblemaster)),
        Some(5),
        "a 2/2 plus one for each of the three other attacking Goblins",
    );
    assert_eq!(
        game.toughness(permanent(&game, rabblemaster)),
        Some(2),
        "and +1/+0 leaves the toughness alone",
    );
}

/// Attacking alone is worth nothing: the count is of *other* Goblins.
#[test]
fn attacking_alone_grows_it_by_nothing() {
    let (mut game, rabblemaster, _friends) = staged(&[], 0);
    reach_declare_attackers(&mut game);
    // The token its combat trigger just made would have to attack too, so
    // it is taken off the board: what is being measured is a Rabblemaster
    // with no company.
    game.battlefield.retain(|permanent| {
        !is_token_with(
            permanent,
            token_with_haste(tokens::creature(&["Goblin"], &[ManaColor::Red], 1, 1)),
        )
    });
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::DeclareAttacker { attacker, .. } if *attacker == rabblemaster)
        })
        .expect("it can attack");
    game.apply(PlayerId::One, action).expect("it attacks");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("nothing else had to attack");
    settle(&mut game);

    assert_eq!(
        game.power(permanent(&game, rabblemaster)),
        Some(2),
        "still a 2/2",
    );
}
