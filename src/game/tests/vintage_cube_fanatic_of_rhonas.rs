//! Fanatic of Rhonas: a 1/4 that taps for one, for four beside anything
//! large, and comes back as a 4/4 Zombie afterwards.

use super::*;

/// Her on the battlefield, with `others` beside her.
fn staged(others: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    let fanatic = game
        .put_onto_battlefield(PlayerId::One, cards::FANATIC_OF_RHONAS)
        .expect("cataloged");
    for definition in others {
        game.put_onto_battlefield(PlayerId::One, *definition)
            .expect("cataloged");
    }
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
        permanent.tapped = false;
    }
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, fanatic)
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

/// The green mana abilities she is offering, by how much each makes.
fn mana_amounts(game: &Game, fanatic: GameObjectId) -> Vec<u16> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateManaAbility {
                source,
                color: ManaColor::Green,
                ..
            } if source == fanatic => Some(()),
            _ => None,
        })
        .map(|()| 0)
        .collect()
}

/// Alone she taps for one and nothing else.
#[test]
fn without_ferocious_she_taps_for_one() {
    let (mut game, fanatic) = staged(&[]);
    assert_eq!(mana_amounts(&game, fanatic).len(), 1, "one ability offered");

    let green = Action::ActivateManaAbility {
        source: fanatic,
        ability: mana_ability_for(&game, fanatic, ManaColor::Green),
        color: ManaColor::Green,
        counters_removed: None,
        cost_object: None,
        combination: None,
        triggered_mana: None,
    };
    game.apply(PlayerId::One, green).expect("it taps");

    assert_eq!(game.players[0].mana_pool.green, 1);
}

/// With something big beside her both abilities are on offer, and the
/// ferocious one makes four.
#[test]
fn ferocious_offers_four() {
    let (mut game, fanatic) = staged(&[cards::SERRA_ANGEL]);
    assert_eq!(
        mana_amounts(&game, fanatic).len(),
        2,
        "both abilities are offered",
    );

    let four = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateManaAbility {
                source,
                ability: AbilityOrigin::Printed { ability, .. },
                color: ManaColor::Green,
                ..
            } => *source == fanatic && *ability == AbilityId(1),
            _ => false,
        })
        .expect("the ferocious ability is activatable");
    game.apply(PlayerId::One, four).expect("it taps");

    assert_eq!(game.players[0].mana_pool.green, 4);
}

/// A 2/2 is not power four, so only the small ability is offered.
#[test]
fn a_small_creature_is_not_ferocious() {
    let (game, fanatic) = staged(&[cards::GRIZZLY_BEARS]);

    assert_eq!(mana_amounts(&game, fanatic).len(), 1);
}

/// Puts her in the graveyard with the mana to eternalize her.
fn ready_to_eternalize() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    let card = game
        .build_zone(PlayerId::One, &[cards::FANATIC_OF_RHONAS])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = card.id;
    game.players[0].graveyard.push(card);
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 4);
    (game, id)
}

fn eternalize(game: &mut Game, card: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == card))
        .expect("eternalize is activatable from the graveyard");
    game.apply(PlayerId::One, action).expect("it activates");
    settle(game);
}

/// The token it makes is a 4/4.
#[test]
fn the_eternalized_token_is_a_four_four() {
    let (mut game, card) = ready_to_eternalize();

    eternalize(&mut game, card);

    let token = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Token)
        .expect("the token was made");
    assert_eq!(game.power(token), Some(4));
    assert_eq!(game.toughness(token), Some(4));
}

/// Black, whatever the card it copied was.
#[test]
fn the_eternalized_token_is_black() {
    let (mut game, card) = ready_to_eternalize();

    eternalize(&mut game, card);

    let token = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Token)
        .expect("the token was made");
    let colors = game.permanent_colors(token);
    assert!(colors[ManaColor::Black.color_index().expect("a color")]);
    assert!(!colors[ManaColor::Green.color_index().expect("a color")]);
}

/// A Zombie on top of the types it already had.
#[test]
fn the_eternalized_token_is_a_zombie_snake_druid() {
    let (mut game, card) = ready_to_eternalize();

    eternalize(&mut game, card);

    let token = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Token)
        .expect("the token was made");
    let subtypes = game.effective_subtypes(token);
    assert!(subtypes.contains(&"Zombie"), "{subtypes:?}");
    assert!(subtypes.contains(&"Snake"), "{subtypes:?}");
    assert!(subtypes.contains(&"Druid"), "{subtypes:?}");
}

/// With no mana cost, which is what makes its mana value zero.
#[test]
fn the_eternalized_token_has_no_mana_cost() {
    let (mut game, card) = ready_to_eternalize();

    eternalize(&mut game, card);

    let token = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Token)
        .expect("the token was made");
    assert_eq!(game.permanent_mana_value(token), 0);
}

/// It keeps what it copied: the token taps for green like the card did.
#[test]
fn the_eternalized_token_keeps_its_mana_ability() {
    let (mut game, card) = ready_to_eternalize();
    eternalize(&mut game, card);
    let token = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Token)
        .expect("the token was made")
        .card
        .id;
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == token)
    {
        permanent.entered_controller_turn = 0;
    }
    game.players[0].mana_pool = ManaPool::default();

    assert!(
        game.legal_actions(PlayerId::One).into_iter().any(|action| {
            matches!(
                action,
                Action::ActivateManaAbility { source, color: ManaColor::Green, .. }
                    if source == token
            )
        }),
        "the copy makes green the same way",
    );
}

/// The card is exiled to pay for it, so it cannot be eternalized twice.
#[test]
fn the_card_is_exiled_paying_for_it() {
    let (mut game, card) = ready_to_eternalize();

    eternalize(&mut game, card);

    assert!(
        game.players[0].graveyard.is_empty(),
        "it left the graveyard",
    );
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|exiled| exiled.definition == cards::FANATIC_OF_RHONAS),
        "and is in exile",
    );
}
