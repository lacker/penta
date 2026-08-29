//! Otawara, Soaring City: a land that is also the bounce spell, and gets
//! cheaper the more legends are standing beside it.

use super::*;

/// Otawara in hand, `board` on the battlefield under Player One, and no mana
/// at all until a test says otherwise.
fn staged(board: &[CardDefinitionId]) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let mut ids = Vec::new();
    for definition in board {
        let permanent = game
            .put_onto_battlefield(PlayerId::One, *definition)
            .expect("cataloged");
        ids.push(permanent);
    }
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [5, 5];
    drain_pending(&mut game);
    let otawara = game
        .build_zone(PlayerId::One, &[cards::OTAWARA_SOARING_CITY])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = otawara.id;
    game.players[0].hand.push(otawara);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, id, ids)
}

fn settle(game: &mut Game) {
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .take(decision.minimum.max(1))
                .map(|option| option.id)
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

/// Every channel activation Otawara is offering from hand right now.
fn channels(game: &Game, otawara: GameObjectId) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == otawara),
        )
        .collect()
}

fn channel_at(game: &mut Game, otawara: GameObjectId, target: GameObjectId) {
    let action = channels(game, otawara)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility { targets, .. } => targets
                .iter()
                .any(|selection| selection.targets().contains(&Target::Permanent(target))),
            _ => false,
        })
        .expect("the channel is offered at that permanent");
    game.apply(PlayerId::One, action).expect("it activates");
    settle(game);
}

fn on_battlefield(game: &Game, definition: CardDefinitionId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.definition == definition)
}

/// Played as a land, it taps for blue like the Island it mostly is.
#[test]
fn it_taps_for_blue() {
    let (mut game, _otawara, _) = staged(&[]);
    let land = game
        .put_onto_battlefield(PlayerId::One, cards::OTAWARA_SOARING_CITY)
        .expect("cataloged");

    let action = Action::ActivateManaAbility {
        source: land,
        ability: mana_ability_for(&game, land, ManaColor::Blue),
        color: ManaColor::Blue,
        counters_removed: None,
        cost_object: None,
        combination: None,
        triggered_mana: None,
    };
    game.apply(PlayerId::One, action).expect("it taps");
    settle(&mut game);

    assert_eq!(
        game.players[0].mana_pool.amount(ManaColor::Blue),
        1,
        "one blue in the pool",
    );
}

/// Four mana and the card itself returns something across the table.
#[test]
fn channelling_it_returns_a_permanent() {
    let (mut game, otawara, _) = staged(&[]);
    let angel = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 4);

    channel_at(&mut game, otawara, angel);

    assert!(!on_battlefield(&game, cards::SERRA_ANGEL), "it is gone");
    assert!(
        game.players[1]
            .hand
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL),
        "back in its owner's hand",
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::OTAWARA_SOARING_CITY),
        "the land itself was discarded to pay",
    );
}

/// A land is not among the things it can answer, which is what keeps the
/// cycle from answering itself.
#[test]
fn it_cannot_return_a_land() {
    let (mut game, otawara, _) = staged(&[]);
    game.put_onto_battlefield(PlayerId::Two, cards::ISLAND)
        .expect("cataloged");
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 4);

    assert!(
        channels(&game, otawara).is_empty(),
        "nothing on the board is a legal target",
    );
}

/// Each legendary creature takes {1} off the activation: three mana pays for
/// it with one out and does not without.
#[test]
fn a_legend_makes_the_channel_cheaper() {
    let (mut game, otawara, _) = staged(&[]);
    game.put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);

    assert!(
        channels(&game, otawara).is_empty(),
        "three mana does not pay four",
    );

    game.put_onto_battlefield(PlayerId::One, cards::RAGAVAN_NIMBLE_PILFERER)
        .expect("cataloged");
    drain_pending(&mut game);

    assert!(
        !channels(&game, otawara).is_empty(),
        "one legend takes it to three",
    );

    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SERRA_ANGEL)
        .expect("it is here")
        .card
        .id;
    channel_at(&mut game, otawara, angel);

    assert!(!on_battlefield(&game, cards::SERRA_ANGEL), "it is returned");
    assert_eq!(
        game.players[0].mana_pool.total(),
        0,
        "and the discounted cost is what was actually paid",
    );
}

/// The discount counts legends rather than creatures: a Bears beside them
/// changes nothing.
#[test]
fn only_legendary_creatures_count() {
    let (mut game, otawara, _) = staged(&[cards::GRIZZLY_BEARS, cards::GRIZZLY_BEARS]);
    game.put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);

    assert!(
        channels(&game, otawara).is_empty(),
        "two Bears are not two legends",
    );
}

/// The blue cannot be discounted away: a board of legends leaves {U}, and
/// colorless mana alone still cannot pay it.
#[test]
fn the_colored_half_survives_any_board() {
    let (mut game, otawara, _) = staged(&[
        cards::RAGAVAN_NIMBLE_PILFERER,
        cards::MAGDA_BRAZEN_OUTLAW,
        cards::TIFA_LOCKHART,
    ]);
    game.put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    assert!(
        channels(&game, otawara).is_empty(),
        "three legends leave one blue, which colorless mana cannot pay",
    );

    game.empty_mana_pools();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);

    assert!(
        !channels(&game, otawara).is_empty(),
        "one blue is the whole of what is left",
    );
}
