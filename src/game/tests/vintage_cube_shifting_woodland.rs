//! Shifting Woodland: a Forest that becomes the best thing you have already
//! lost, once the graveyard is deep enough to read.

use super::*;

/// A graveyard with four card types in it, which is delirium.
const FOUR_TYPES: [CardDefinitionId; 4] = [
    cards::MANIFOLD_KEY,
    cards::LIGHTNING_BOLT,
    cards::MOUNTAIN,
    cards::PACIFISM,
];

/// The Woodland on the battlefield with `graveyard` behind it, and enough
/// mana to use it.
fn staged(graveyard: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    for (index, definition) in graveyard.iter().enumerate() {
        game.players[0].graveyard.push(card(
            95_000 + u32::try_from(index).expect("few cards"),
            *definition,
            PlayerId::One,
        ));
    }
    let woodland = game
        .put_onto_battlefield(PlayerId::One, cards::SHIFTING_WOODLAND)
        .expect("cataloged");
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.tapped = false;
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 4);
    (game, woodland)
}

/// The copy activation, if it is on offer at all.
fn copy_action(game: &Game, woodland: GameObjectId, wanted: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, targets, .. }
                    if *source == woodland
                        && targets
                            .iter()
                            .any(|slot| slot.targets().contains(&Target::Card(wanted)))
            )
        })
}

fn graveyard_card(game: &Game, definition: CardDefinitionId) -> GameObjectId {
    game.players[0]
        .graveyard
        .iter()
        .find(|card| card.definition == definition)
        .expect("it is in the graveyard")
        .id
}

fn woodland_on_battlefield(game: &Game) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Card(cards::SHIFTING_WOODLAND))
        .expect("it is still there")
}

/// A Forest on the battlefield means it arrives ready.
#[test]
fn a_forest_lets_it_enter_untapped() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.put_onto_battlefield(PlayerId::One, cards::FOREST)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::SHIFTING_WOODLAND)
        .expect("cataloged");
    drain_pending(&mut game);

    assert!(!woodland_on_battlefield(&game).tapped);
}

/// Without one it enters tapped, the same as any check land.
#[test]
fn without_a_forest_it_enters_tapped() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.put_onto_battlefield(PlayerId::One, cards::MOUNTAIN)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::SHIFTING_WOODLAND)
        .expect("cataloged");
    drain_pending(&mut game);

    assert!(woodland_on_battlefield(&game).tapped);
}

/// Three card types is not delirium, so the ability is not offered at all.
#[test]
fn three_card_types_do_not_turn_it_on() {
    let (game, woodland) = staged(&[cards::LIGHTNING_BOLT, cards::MOUNTAIN, cards::GRIZZLY_BEARS]);
    let target = graveyard_card(&game, cards::GRIZZLY_BEARS);

    assert!(copy_action(&game, woodland, target).is_none());
}

/// With delirium it becomes the creature in the graveyard: that creature's
/// power and toughness, and no longer a land at all.
#[test]
fn with_delirium_it_becomes_the_creature_in_the_graveyard() {
    let mut graveyard = FOUR_TYPES.to_vec();
    graveyard.push(cards::SERRA_ANGEL);
    let (mut game, woodland) = staged(&graveyard);
    let angel = graveyard_card(&game, cards::SERRA_ANGEL);

    let action = copy_action(&game, woodland, angel).expect("delirium offers the copy");
    game.apply(PlayerId::One, action).expect("it activates");
    drain_pending(&mut game);

    let permanent = woodland_on_battlefield(&game);
    assert_eq!(game.power(permanent), Some(4));
    assert_eq!(game.toughness(permanent), Some(4));
    assert!(
        game.permanent_types(permanent)
            .is_some_and(|types| types.contains(CardType::Creature)),
        "it is a creature now",
    );
    assert!(
        game.permanent_types(permanent)
            .is_some_and(|types| !types.contains(CardType::Land)),
        "and not a land any more: the copy replaced every copiable value",
    );
    assert_eq!(
        game.players[0]
            .graveyard
            .iter()
            .filter(|card| card.id == angel)
            .count(),
        1,
        "the card it copied never moved",
    );
}

/// "Until end of turn": by the next turn it is a land again.
#[test]
fn the_copy_wears_off_at_end_of_turn() {
    let mut graveyard = FOUR_TYPES.to_vec();
    graveyard.push(cards::SERRA_ANGEL);
    let (mut game, woodland) = staged(&graveyard);
    let angel = graveyard_card(&game, cards::SERRA_ANGEL);
    let action = copy_action(&game, woodland, angel).expect("delirium offers the copy");
    game.apply(PlayerId::One, action).expect("it activates");
    drain_pending(&mut game);

    for _ in 0..40 {
        if game.turn > 9 && game.step == Step::PrecombatMain {
            break;
        }
        game.advance_step();
    }

    let permanent = woodland_on_battlefield(&game);
    assert!(
        game.permanent_types(permanent)
            .is_some_and(|types| types.contains(CardType::Land)),
        "a land again",
    );
    assert_eq!(game.power(permanent), None, "and no longer a creature");
}

/// It taps for green while it is still a land.
#[test]
fn it_taps_for_green() {
    let (mut game, woodland) = staged(&[]);
    game.players[0].mana_pool = ManaPool::default();

    let add_green = Action::ActivateManaAbility {
        source: woodland,
        ability: mana_ability_for(&game, woodland, ManaColor::Green),
        color: ManaColor::Green,
        counters_removed: None,
        cost_object: None,
        combination: None,
        triggered_mana: None,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&add_green));
    game.apply(PlayerId::One, add_green).expect("it taps");

    assert_eq!(game.players[0].mana_pool.green, 1);
}

/// The graveyard has to be yours: a permanent card in theirs is not a legal
/// target.
#[test]
fn it_cannot_copy_out_of_their_graveyard() {
    let (mut game, woodland) = staged(&FOUR_TYPES);
    game.players[1]
        .graveyard
        .push(card(95_900, cards::SERRA_ANGEL, PlayerId::Two));
    let theirs = game.players[1].graveyard.last().expect("just pushed").id;

    assert!(copy_action(&game, woodland, theirs).is_none());
}

/// "Because it isn't entering the battlefield when it becomes a copy, any
/// 'when this enters' abilities of the copied card won't apply." A Titan
/// copied out of the graveyard is a 6/6 with deathtouch and no Zombies.
#[test]
fn copying_something_does_not_make_it_enter() {
    let mut graveyard = FOUR_TYPES.to_vec();
    graveyard.push(cards::GRAVE_TITAN);
    let (mut game, woodland) = staged(&graveyard);
    let titan = graveyard_card(&game, cards::GRAVE_TITAN);
    let before = game.battlefield.len();

    let action = copy_action(&game, woodland, titan).expect("delirium offers the copy");
    game.apply(PlayerId::One, action).expect("it activates");
    drain_pending(&mut game);

    let permanent = woodland_on_battlefield(&game);
    assert_eq!(game.power(permanent), Some(6), "it is the Titan's body");
    assert!(
        game.permanent_has_executable_keyword(permanent, KeywordAbility::Deathtouch),
        "and carries the printed abilities that come with it",
    );
    assert_eq!(
        game.battlefield.len(),
        before,
        "but nothing entered, so the Titan's own arrival made no Zombies",
    );
}

/// "Any effects that applied before it becomes a copy continue to apply, and
/// the same is true of counters." Becoming something else is not a new
/// object: it keeps what was already on it.
#[test]
fn what_was_already_on_it_stays_on_it() {
    let mut graveyard = FOUR_TYPES.to_vec();
    graveyard.push(cards::SERRA_ANGEL);
    let (mut game, woodland) = staged(&graveyard);
    let angel = graveyard_card(&game, cards::SERRA_ANGEL);
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == woodland)
    {
        permanent.add_counters(CounterKind::PlusOnePlusOne, 1);
        permanent.tapped = true;
    }

    let action = copy_action(&game, woodland, angel).expect("tapping is no bar to activating it");
    game.apply(PlayerId::One, action).expect("it activates");
    drain_pending(&mut game);

    let permanent = woodland_on_battlefield(&game);
    assert_eq!(
        permanent.counters(CounterKind::PlusOnePlusOne),
        1,
        "the counter stayed through the change",
    );
    assert_eq!(
        game.power(permanent),
        Some(5),
        "so what it copied is a 4/4 with a counter on it",
    );
    assert!(permanent.tapped, "and it is still tapped");
}
