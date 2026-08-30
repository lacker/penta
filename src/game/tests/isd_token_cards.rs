//! Innistrad cards about tokens.
//!
//! A token predicate, a tapped-token creation and a halved count were all
//! built for other cards. The clauses worth pinning are the ones that are
//! easy to widen by accident: Intangible Virtue reaches tokens only, and
//! Endless Ranks rounds down, so one Zombie makes none.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

fn stats(game: &Game, id: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there");
    (game.power(permanent), game.toughness(permanent))
}

fn zombies(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Zombie"], &[ManaColor::Black], 2, 2),
            )
        })
        .count()
}

/// A token gets the anthem; a card creature beside it does not.
#[test]
fn intangible_virtue_reaches_tokens_only() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::INTANGIBLE_VIRTUE, PlayerId::One));
    let token = token_permanent(
        10_100,
        tokens::creature(&["Zombie"], &[ManaColor::Black], 2, 2),
        PlayerId::One,
    );
    let token_id = token.card.id;
    game.battlefield.push(token);
    let bear = creature(10_101, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);

    assert_eq!(stats(&game, token_id), (Some(3), Some(3)), "a 2/2 plus one");
    assert_eq!(stats(&game, bear_id), (Some(2), Some(2)), "not a token");

    let vigilant = |id: GameObjectId| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("still there");
        game.permanent_has_executable_keyword(permanent, KeywordAbility::Vigilance)
    };
    assert!(vigilant(token_id));
    assert!(!vigilant(bear_id));
}

/// Thirteen, and tapped, so none of them can attack the turn they arrive.
#[test]
fn army_of_the_damned_makes_thirteen_tapped_zombies() {
    let mut game = ready();
    let spell = card(20_000, cards::ARMY_OF_THE_DAMNED, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.black = 3;
    game.players[PlayerId::One.index()].mana_pool.colorless = 5;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("eight mana covers it");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(&mut game);

    assert_eq!(zombies(&game), 13);
    assert!(
        game.battlefield
            .iter()
            .filter(|permanent| is_token_with(
                permanent,
                tokens::creature(&["Zombie"], &[ManaColor::Black], 2, 2)
            ))
            .all(|permanent| permanent.tapped),
        "every one of them arrived tapped",
    );
}

/// Rounded down, so the engine does not start from a single Zombie.
#[test]
fn endless_ranks_rounds_the_zombie_count_down() {
    let upkeep_with = |existing: u32| {
        let mut game = ready();
        game.battlefield.push(creature(
            10_000,
            cards::ENDLESS_RANKS_OF_THE_DEAD,
            PlayerId::One,
        ));
        for index in 0..existing {
            game.battlefield.push(token_permanent(
                10_100 + index,
                tokens::creature(&["Zombie"], &[ManaColor::Black], 2, 2),
                PlayerId::One,
            ));
        }
        game.step = Step::Upkeep;
        game.handle_upkeep_triggers();
        drain_pending(&mut game);
        zombies(&game) - existing as usize
    };

    assert_eq!(upkeep_with(1), 0, "half of one, rounded down");
    assert_eq!(upkeep_with(2), 1);
    assert_eq!(upkeep_with(5), 2, "half of five, rounded down");
}

fn attack(game: &mut Game, attacker: GameObjectId) {
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("the creature may attack");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration finishes");
}

fn token_count(game: &Game, token: TokenCharacteristics) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| is_token_with(permanent, token))
        .count()
}

/// Cagebreakers counts on resolution, and only creature cards contribute.
#[test]
fn cagebreakers_make_attacking_wolves_from_the_resolved_graveyard() {
    let mut game = ready();
    let cagebreakers = game
        .put_onto_battlefield(PlayerId::One, cards::KESSIG_CAGEBREAKERS)
        .expect("Kessig Cagebreakers is cataloged");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == cagebreakers)
        .expect("the Cagebreakers are present")
        .entered_controller_turn = 0;
    game.players[0].graveyard = vec![
        card(90_001, cards::GRIZZLY_BEARS, PlayerId::One),
        card(90_002, cards::LIGHTNING_BOLT, PlayerId::One),
    ];

    attack(&mut game, cagebreakers);
    game.players[0]
        .graveyard
        .push(card(90_003, cards::WALKING_CORPSE, PlayerId::One));
    drain_pending(&mut game);

    assert_eq!(
        token_count(
            &game,
            tokens::creature(&["Wolf"], &[ManaColor::Green], 2, 2),
        ),
        2,
    );
    assert!(
        game.battlefield
            .iter()
            .filter(|permanent| {
                is_token_with(
                    permanent,
                    tokens::creature(&["Wolf"], &[ManaColor::Green], 2, 2),
                )
            })
            .all(|wolf| wolf.tapped && wolf.attacking),
        "both Wolves enter tapped and attacking",
    );
}

/// Geist's delayed trigger names exactly the Angel made by its attack.
#[test]
fn geist_makes_an_attacking_angel_and_exiles_it_at_end_of_combat() {
    let mut game = ready();
    let geist = game
        .put_onto_battlefield(PlayerId::One, cards::GEIST_OF_SAINT_TRAFT)
        .expect("Geist of Saint Traft is cataloged");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == geist)
        .expect("Geist is present")
        .entered_controller_turn = 0;

    attack(&mut game, geist);
    drain_pending(&mut game);

    let angel = game
        .battlefield
        .iter()
        .find(|permanent| {
            is_token_with(
                permanent,
                token_with_flying(tokens::creature(&["Angel"], &[ManaColor::White], 4, 4)),
            )
        })
        .expect("Geist created its Angel");
    assert!(angel.tapped && angel.attacking);
    assert!(game.permanent_has_executable_keyword(angel, KeywordAbility::Flying));

    game.step = Step::EndOfCombat;
    game.begin_step_triggers();
    drain_pending(&mut game);

    assert_eq!(
        token_count(
            &game,
            token_with_flying(tokens::creature(&["Angel"], &[ManaColor::White], 4, 4,)),
        ),
        0,
    );
}
