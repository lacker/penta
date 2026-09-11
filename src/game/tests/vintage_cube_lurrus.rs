//! Lurrus of the Dream-Den: one cheap permanent back out of the graveyard
//! every turn, and only on your own.

use super::*;

/// Lurrus out since last turn, with `graveyard` in Player One's graveyard.
fn staged(graveyard: &[CardDefinitionId]) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    for (index, definition) in graveyard.iter().enumerate() {
        game.players[0].graveyard.push(card(
            230_000 + u32::try_from(index).expect("few cards"),
            *definition,
            PlayerId::One,
        ));
    }
    game.put_onto_battlefield(PlayerId::One, cards::LURRUS_OF_THE_DREAM_DEN)
        .expect("cataloged");
    drain_pending(&mut game);
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .map(|option| option.id)
                .take(decision.minimum.max(1))
                .collect();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the offered choice is legal");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

fn castable_from_graveyard(game: &Game, definition: CardDefinitionId) -> Option<Action> {
    let card = game.players[0]
        .graveyard
        .iter()
        .find(|card| card.definition == definition)?
        .id;
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card: id, .. } if *id == card))
}

/// He is lifelinking, which is half of why the deck plays him.
#[test]
fn he_has_lifelink() {
    let game = staged(&[]);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::LURRUS_OF_THE_DREAM_DEN)
        .expect("he is there");

    assert!(game.permanent_has_executable_keyword(permanent, KeywordAbility::Lifelink));
}

/// A cheap permanent in the graveyard is castable; an expensive one and a
/// noncreature spell are not.
#[test]
fn only_cheap_permanents_are_castable() {
    let mut game = staged(&[
        cards::GRIZZLY_BEARS,
        cards::SERRA_ANGEL,
        cards::LIGHTNING_BOLT,
    ]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 6);

    assert!(
        castable_from_graveyard(&game, cards::GRIZZLY_BEARS).is_some(),
        "a two-mana creature is what he is for",
    );
    assert!(
        castable_from_graveyard(&game, cards::SERRA_ANGEL).is_none(),
        "a five-mana one is not",
    );
    assert!(
        castable_from_graveyard(&game, cards::LIGHTNING_BOLT).is_none(),
        "and an instant is not a permanent spell",
    );
}

/// Once each turn: the second cheap permanent has to wait.
#[test]
fn the_permission_is_spent_by_the_first_cast() {
    let mut game = staged(&[cards::GRIZZLY_BEARS, cards::MANIFOLD_KEY]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 6);

    let cast = castable_from_graveyard(&game, cards::GRIZZLY_BEARS).expect("the first is on offer");
    game.apply(PlayerId::One, cast).expect("it is cast");
    settle(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::GRIZZLY_BEARS),
        "the first one resolved",
    );
    assert!(
        castable_from_graveyard(&game, cards::MANIFOLD_KEY).is_none(),
        "and the permission is spent for the turn",
    );

    game.complete_cleanup();
    game.commit_next_turn(PlayerId::Two, Vec::new());
    game.complete_cleanup();
    game.commit_next_turn(PlayerId::One, Vec::new());
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 6);

    assert!(
        castable_from_graveyard(&game, cards::MANIFOLD_KEY).is_some(),
        "a fresh turn opens it again",
    );
}

/// "During each of your turns": not on theirs, even at instant speed.
#[test]
fn it_is_closed_on_their_turn() {
    let mut game = staged(&[cards::MANIFOLD_KEY]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 6);
    game.active_player = PlayerId::Two;
    game.step = Step::End;
    game.priority = PlayerId::One;

    assert!(castable_from_graveyard(&game, cards::MANIFOLD_KEY).is_none());
}

/// When he leaves, the permission goes with him.
#[test]
fn the_permission_belongs_to_him() {
    let mut game = staged(&[cards::GRIZZLY_BEARS]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 6);
    let lurrus = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::LURRUS_OF_THE_DREAM_DEN)
        .expect("he is there")
        .card
        .id;

    game.destroy_permanent(lurrus);
    settle(&mut game);

    assert!(castable_from_graveyard(&game, cards::GRIZZLY_BEARS).is_none());
}

/// "Lurrus doesn't let you play lands from your graveyard." A land is a
/// permanent card of mana value zero and it is still played rather than
/// cast, so the permission never reaches it.
#[test]
fn a_land_in_the_graveyard_stays_there() {
    let game = staged(&[cards::MOUNTAIN, cards::GAEAS_CRADLE]);

    assert!(
        castable_from_graveyard(&game, cards::MOUNTAIN).is_none(),
        "a basic is not a spell to cast",
    );
    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::PlayLand { card, .. }
                if game.players[0]
                    .graveyard
                    .iter()
                    .any(|buried| buried.id == *card))
        }),
        "and neither half of the permission is a land drop",
    );
}

/// "For spells with {X} in their mana costs, use the value chosen for X to
/// determine the spell's mana value." A Walking Ballista is {X}{X}: at one
/// it costs two and Lurrus reaches it, and there is no larger X on offer.
#[test]
fn an_x_spell_is_reached_only_at_the_x_that_fits() {
    let mut game = staged(&[cards::WALKING_BALLISTA]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 6);

    let ballista = game.players[0]
        .graveyard
        .iter()
        .find(|card| card.definition == cards::WALKING_BALLISTA)
        .expect("it is in the graveyard")
        .id;
    let mut offered = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == ballista => Some(choices.x()),
            _ => None,
        })
        .collect::<Vec<_>>();
    offered.sort_unstable();

    assert_eq!(
        offered,
        vec![0, 1],
        "X of two would be a four-mana permanent, which he does not reach",
    );
}

/// The other side of {X}: "if a card in a player's deck has {X} in its mana
/// cost, X is considered to be 0", so a Ballista in the starting deck is a
/// zero-drop and leaves him a legal companion.
#[test]
fn an_x_permanent_in_the_deck_costs_nothing_for_the_condition() {
    let mut game = companion::staged(
        &[cards::WALKING_BALLISTA, cards::RAGAVAN_NIMBLE_PILFERER],
        &[cards::LURRUS_OF_THE_DREAM_DEN],
    );
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);

    assert_eq!(
        companion::companion_offers(&game).len(),
        1,
        "{{X}}{{X}} in the deck is mana value zero, not four",
    );
}

/// "You must follow the normal timing permissions and restrictions of the
/// spell you cast from your graveyard." The permission is not haste for
/// spells: a creature card still wants an empty stack, and waits for one.
#[test]
fn the_graveyard_cast_keeps_the_spell_own_timing() {
    let mut game = staged(&[cards::GRIZZLY_BEARS]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    assert!(
        castable_from_graveyard(&game, cards::GRIZZLY_BEARS).is_some(),
        "an empty stack in your main phase is when a creature may be cast",
    );

    // Something of your own on the stack closes the window.
    game.players[0]
        .hand
        .push(card(230_900, cards::LIGHTNING_BOLT, PlayerId::One));
    let bolt = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, .. } if *card == CardInstanceId(230_900))
        })
        .expect("one red casts the Bolt");
    game.apply(PlayerId::One, bolt).expect("it is cast");

    assert!(
        castable_from_graveyard(&game, cards::GRIZZLY_BEARS).is_none(),
        "a creature spell waits for the stack to clear, permission or not",
    );

    settle(&mut game);
    game.priority = PlayerId::One;
    assert!(
        castable_from_graveyard(&game, cards::GRIZZLY_BEARS).is_some(),
        "and the permission is still there once it has",
    );
}

/// The other half of the card: the keyword that keeps it out of the deck in
/// the first place, and the {3} that fetches it back.
mod companion {
    use super::*;

    /// A game whose player-one sideboard holds `sideboard` and whose starting
    /// deck holds `deck` beside enough Mountains to shuffle.
    pub(super) fn staged(deck: &[CardDefinitionId], sideboard: &[CardDefinitionId]) -> Game {
        let mut main = deck.to_vec();
        while main.len() < 40 {
            main.push(cards::MOUNTAIN);
        }
        let opponent = crate::Deck {
            commanders: Vec::new(),
            main: vec![cards::MOUNTAIN; 40],
            sideboard: Vec::new(),
        };
        let mine = crate::Deck {
            commanders: Vec::new(),
            main,
            sideboard: sideboard.to_vec(),
        };
        // The cube is the format these cards are legal in, and the format a
        // companion is a real card in.
        let mut game = Game::new_with_format(
            crate::Format::VintageCube,
            poc::catalog().unwrap(),
            [mine, opponent],
            0,
        )
        .expect("a legal game");
        game.pregame = None;
        game.step = Step::PrecombatMain;
        game.active_player = PlayerId::One;
        game.priority = PlayerId::One;
        game.turns_started = [3, 2];
        game
    }

    pub(super) fn companion_offers(game: &Game) -> Vec<Action> {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .filter(|action| matches!(action, Action::TakeCompanion { .. }))
            .collect()
    }

    /// Every permanent in the deck costs two or less, so Lurrus is a legal
    /// companion and the special action is on offer once three mana is up.
    #[test]
    fn a_two_drop_deck_may_take_lurrus() {
        let mut game = staged(
            &[cards::RAGAVAN_NIMBLE_PILFERER, cards::LIGHTNING_BOLT],
            &[cards::LURRUS_OF_THE_DREAM_DEN],
        );
        assert!(
            companion_offers(&game).is_empty(),
            "three mana buys it, and there is none yet",
        );

        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
        let offer = companion_offers(&game)
            .into_iter()
            .next()
            .expect("three mana is the whole cost");
        game.apply(PlayerId::One, offer).expect("it is taken");

        assert_eq!(
            game.players[0]
                .hand
                .iter()
                .filter(|card| card.definition == cards::LURRUS_OF_THE_DREAM_DEN)
                .count(),
            1,
            "it went from outside the game to the hand",
        );
        assert!(
            game.players[0].outside_game.is_empty(),
            "and is no longer out there",
        );
        assert_eq!(game.players[0].mana_pool.total(), 0, "three mana paid");
    }

    /// One permanent above the curve and the condition fails, so nothing is
    /// offered however much mana is available. An instant above it is fine:
    /// the condition reads permanents.
    #[test]
    fn one_expensive_permanent_costs_the_companion() {
        let mut game = staged(
            &[cards::SNEAK_ATTACK, cards::RAGAVAN_NIMBLE_PILFERER],
            &[cards::LURRUS_OF_THE_DREAM_DEN],
        );
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);

        assert!(
            companion_offers(&game).is_empty(),
            "a four-mana enchantment is a permanent card too",
        );

        let mut game = staged(
            &[cards::CRYPTIC_COMMAND, cards::RAGAVAN_NIMBLE_PILFERER],
            &[cards::LURRUS_OF_THE_DREAM_DEN],
        );
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);

        assert_eq!(
            companion_offers(&game).len(),
            1,
            "a four-mana instant is not a permanent card",
        );
    }

    /// A companion is taken once. Two in the sideboard is still one taken, and
    /// the second stops being offered the moment the first is.
    #[test]
    fn a_game_has_one_companion() {
        let mut game = staged(
            &[cards::RAGAVAN_NIMBLE_PILFERER],
            &[
                cards::LURRUS_OF_THE_DREAM_DEN,
                cards::ZIRDA_THE_DAWNWAKER,
                cards::LUTRI_THE_SPELLCHASER,
            ],
        );
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 6);
        let offers = companion_offers(&game);
        assert!(
            offers.len() > 1,
            "more than one of them found the deck legal",
        );

        game.apply(
            PlayerId::One,
            offers.into_iter().next().expect("one of them"),
        )
        .expect("it is taken");
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);

        assert!(
            companion_offers(&game).is_empty(),
            "the rest stay outside the game for good",
        );
    }

    /// It is a sorcery-speed action: not on the opponent's turn, and not with
    /// something on the stack.
    #[test]
    fn it_is_taken_only_when_a_sorcery_could_be_cast() {
        let mut game = staged(
            &[cards::RAGAVAN_NIMBLE_PILFERER],
            &[cards::LURRUS_OF_THE_DREAM_DEN],
        );
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
        assert_eq!(companion_offers(&game).len(), 1);

        game.active_player = PlayerId::Two;
        assert!(companion_offers(&game).is_empty(), "not on their turn");

        game.active_player = PlayerId::One;
        game.step = Step::DeclareAttackers;
        assert!(
            companion_offers(&game).is_empty(),
            "and not outside a main phase",
        );
    }

    /// A card that prints no companion clause is just a sideboard card.
    #[test]
    fn an_ordinary_sideboard_card_is_not_a_companion() {
        let mut game = staged(&[cards::RAGAVAN_NIMBLE_PILFERER], &[cards::LIGHTNING_BOLT]);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);

        assert!(companion_offers(&game).is_empty());
    }
}

/// "If you cast one permanent spell from your graveyard and then have a new
/// Lurrus come under your control in the same turn, you may cast another
/// permanent spell from your graveyard that turn." The permission belongs to
/// the Lurrus that granted it: a second one brings its own, spent or not.
#[test]
fn a_second_lurrus_brings_a_second_permission() {
    let mut game = staged(&[cards::GRIZZLY_BEARS, cards::MANIFOLD_KEY]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 12);
    let first = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::LURRUS_OF_THE_DREAM_DEN)
        .expect("he is out")
        .card
        .id;

    let cast = castable_from_graveyard(&game, cards::GRIZZLY_BEARS).expect("the first is on offer");
    game.apply(PlayerId::One, cast).expect("it is cast");
    settle(&mut game);
    assert!(
        castable_from_graveyard(&game, cards::MANIFOLD_KEY).is_none(),
        "his permission is spent for the turn",
    );

    // He dies and another takes his place, on the same turn.
    game.move_permanents_to_graveyard(&[first]);
    game.check_state_based_actions();
    drain_pending(&mut game);
    game.put_onto_battlefield(PlayerId::One, cards::LURRUS_OF_THE_DREAM_DEN)
        .expect("cataloged");
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    assert!(
        castable_from_graveyard(&game, cards::MANIFOLD_KEY).is_some(),
        "the new one grants a permission the old one's spending never touched",
    );
}
