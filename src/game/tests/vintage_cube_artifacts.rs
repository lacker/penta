//! Artifacts and Equipment cataloged for the Vintage Cube pool.

use super::*;

/// Resolves whatever is on the stack, answering nothing.
fn resolve(game: &mut Game) {
    for _ in 0..8 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
}

/// The one activation offered for `source`, if there is one.
fn activation(game: &Game, source: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One).into_iter().find(
        |action| matches!(action, Action::ActivateAbility { source: id, .. } if *id == source),
    )
}

/// Equips `source` to `host`, then settles whatever that started.
fn equip_to(game: &mut Game, source: GameObjectId, host: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source: actual,
                targets,
                ..
            } => {
                *actual == source
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(host))
            }
            _ => false,
        })
        .expect("equip is offered for that creature");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(game);
}

#[test]
fn the_orb_eats_a_land_for_two_life_and_nothing_else() {
    let mut game = ready_game();
    game.players[PlayerId::One.index()].life = 10;
    let orb = game
        .put_onto_battlefield(PlayerId::One, cards::ZURAN_ORB)
        .expect("cataloged");
    assert!(
        activation(&game, orb).is_none(),
        "with no land to sacrifice there is nothing to activate",
    );

    game.battlefield
        .push(creature(50_000, cards::FOREST, PlayerId::One));
    let action = activation(&game, orb).expect("a land is available to sacrifice");
    game.apply(PlayerId::One, action).expect("it is activated");
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::FOREST),
        "the land is sacrificed as a cost",
    );
    resolve(&mut game);
    assert_eq!(game.players[PlayerId::One.index()].life, 12);
}

#[test]
fn the_monolith_makes_three_and_stays_tapped_until_it_is_bought_back() {
    let mut game = ready_game();
    let monolith = game
        .put_onto_battlefield(PlayerId::One, cards::GRIM_MONOLITH)
        .expect("cataloged");
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: monolith,
            ability: mana_ability_for(&game, monolith, ManaColor::Colorless),
            color: ManaColor::Colorless,
            counters_removed: None,
            cost_object: None,
            combination: None,
        },
    )
    .expect("it taps for mana");
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.colorless, 3);

    // Four of that mana buys the untap back; three does not.
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;
    assert!(
        activation(&game, monolith).is_none(),
        "three mana does not pay the untap",
    );
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;
    let untap = activation(&game, monolith).expect("four mana pays it");
    game.apply(PlayerId::One, untap).expect("it is activated");
    resolve(&mut game);
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == monolith)
            .expect("still on the battlefield")
            .tapped,
    );
}

#[test]
fn the_mind_stone_trades_itself_for_a_card() {
    let mut game = ready_game();
    let stone = game
        .put_onto_battlefield(PlayerId::One, cards::MIND_STONE)
        .expect("cataloged");
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    let before = game.players[PlayerId::One.index()].library.len();

    let action = activation(&game, stone).expect("the draw ability is offered");
    game.apply(PlayerId::One, action).expect("it is activated");
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != stone),
        "the sacrifice is a cost",
    );
    resolve(&mut game);
    assert_eq!(
        game.players[PlayerId::One.index()].library.len(),
        before - 1
    );
}

/// The Clamp on something that survives it: bigger, frailer, still there.
#[test]
fn the_clamp_gives_plus_one_minus_one_to_what_it_equips() {
    let mut game = ready_game();
    game.battlefield.clear();
    let clamp = creature(51_000, cards::SKULLCLAMP, PlayerId::One);
    let clamp_id = clamp.card.id;
    game.battlefield.push(clamp);
    let bears = creature(51_001, cards::GRIZZLY_BEARS, PlayerId::One);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    let before = game.players[PlayerId::One.index()].library.len();

    equip_to(&mut game, clamp_id, bears_id);

    let bears = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == bears_id)
        .expect("a 2/2 survives losing a toughness");
    assert_eq!(
        (game.power(bears), game.toughness(bears)),
        (Some(3), Some(1))
    );
    assert_eq!(
        game.players[PlayerId::One.index()].library.len(),
        before,
        "nothing is drawn while the creature is alive",
    );
}

/// The Clamp on a one-toughness creature, which is the whole point of it: the
/// minus kills what it equips through state-based actions, and the trigger
/// still finds the creature it was attached to a moment ago.
#[test]
fn the_clamp_kills_a_one_toughness_creature_and_draws_two() {
    let mut game = ready_game();
    game.battlefield.clear();
    let clamp = creature(51_100, cards::SKULLCLAMP, PlayerId::One);
    let clamp_id = clamp.card.id;
    game.battlefield.push(clamp);
    let lions = creature(51_101, cards::SAVANNAH_LIONS, PlayerId::One);
    let lions_id = lions.card.id;
    game.battlefield.push(lions);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    let before = game.players[PlayerId::One.index()].library.len();

    equip_to(&mut game, clamp_id, lions_id);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != lions_id),
        "a 2/1 loses its last toughness and dies",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SAVANNAH_LIONS),
    );
    assert_eq!(
        game.players[PlayerId::One.index()].library.len(),
        before - 2,
        "the Clamp draws two off a creature that was gone before it triggered",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == clamp_id),
        "the Clamp stays behind, ready for the next one",
    );
}

/// Living weapon on a count that includes the Equipment itself. A Nettlecyst
/// arriving alone makes a 0/0 and immediately makes it a 1/1, so the Germ
/// survives the state-based actions that would otherwise bury it.
#[test]
fn nettlecyst_arrives_with_a_germ_that_it_alone_keeps_alive() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.put_onto_battlefield(PlayerId::One, cards::NETTLECYST)
        .expect("cataloged");
    drain_pending(&mut game);
    game.check_state_based_actions();

    let germ = game
        .battlefield
        .iter()
        .find(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Phyrexian", "Germ"], &[ManaColor::Black], 0, 0),
            )
        })
        .expect("living weapon made a Germ and the Germ survived");
    assert_eq!(
        (game.power(germ), game.toughness(germ)),
        (Some(1), Some(1)),
        "the Equipment counts itself and nothing else counts the Germ",
    );

    // Each further artifact or enchantment is another point in both
    // directions.
    game.battlefield
        .push(creature(56_000, cards::BLACK_LOTUS, PlayerId::One));
    game.battlefield
        .push(creature(56_001, cards::PHYREXIAN_ARENA, PlayerId::One));
    let germ = game
        .battlefield
        .iter()
        .find(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Phyrexian", "Germ"], &[ManaColor::Black], 0, 0),
            )
        })
        .expect("still there");
    assert_eq!((game.power(germ), game.toughness(germ)), (Some(3), Some(3)));

    // An opponent's artifact is not one you control.
    game.battlefield
        .push(creature(56_002, cards::MOX_JET, PlayerId::Two));
    let germ = game
        .battlefield
        .iter()
        .find(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Phyrexian", "Germ"], &[ManaColor::Black], 0, 0),
            )
        })
        .expect("still there");
    assert_eq!((game.power(germ), game.toughness(germ)), (Some(3), Some(3)));
}

/// Moved onto a real creature, the same count applies there instead, and the
/// Germ it leaves behind is a 0/0 again.
#[test]
fn moving_nettlecyst_takes_the_bonus_with_it() {
    let mut game = ready_game();
    game.battlefield.clear();
    let nettlecyst = game
        .put_onto_battlefield(PlayerId::One, cards::NETTLECYST)
        .expect("cataloged");
    drain_pending(&mut game);
    game.check_state_based_actions();
    let bears = creature(56_100, cards::GRIZZLY_BEARS, PlayerId::One);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    equip_to(&mut game, nettlecyst, bears_id);
    game.check_state_based_actions();

    let bears = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == bears_id)
        .expect("the creature is still there");
    assert_eq!(
        (game.power(bears), game.toughness(bears)),
        (Some(3), Some(3))
    );
    assert!(
        game.battlefield.iter().all(|permanent| !is_token_with(
            permanent,
            tokens::creature(&["Phyrexian", "Germ"], &[ManaColor::Black], 0, 0)
        )),
        "the Germ is a 0/0 once the Equipment leaves it",
    );
}

/// Three wish counters, one spent per activation, and the ability is only
/// ever offered on its controller's turn.
#[test]
fn wishclaw_talisman_enters_with_three_wishes_and_waits_for_your_turn() {
    let mut game = ready_game();
    game.battlefield.clear();
    let talisman = card(79_000, cards::WISHCLAW_TALISMAN, PlayerId::One);
    let talisman_id = talisman.id;
    game.players[0].hand.push(talisman);
    game.players[0].mana_pool.black = 1;
    game.players[0].mana_pool.colorless = 1;

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == talisman_id))
        .expect("two mana casts it");
    game.apply(PlayerId::One, cast).expect("it is cast");
    resolve(&mut game);

    // Resolving onto the battlefield makes a new object, so the permanent is
    // found by what it is rather than by the id the card had in hand.
    let talisman = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::WISHCLAW_TALISMAN)
        .expect("it is on the battlefield");
    let talisman_id = talisman.card.id;
    assert_eq!(talisman.counters(CounterKind::named("wish")), 3);

    game.players[0].mana_pool.colorless = 1;
    assert!(
        activation(&game, talisman_id).is_some(),
        "its controller's turn is open",
    );
    game.active_player = PlayerId::Two;
    assert!(
        activation(&game, talisman_id).is_none(),
        "and the opponent's turn is not",
    );
}

/// The tutor and the handover are one clause resolving in order: the card is
/// in hand before the artifact changes sides, and the opponent inherits the
/// two counters left in it.
#[test]
fn wishclaw_talisman_tutors_then_hands_itself_to_the_opponent() {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut talisman = creature(79_010, cards::WISHCLAW_TALISMAN, PlayerId::One);
    talisman.add_counters(CounterKind::named("wish"), 3);
    let talisman_id = talisman.card.id;
    game.battlefield.push(talisman);
    game.players[0]
        .library
        .push(card(79_011, cards::BLACK_LOTUS, PlayerId::One));
    game.players[0].mana_pool.colorless = 1;
    let before = game.players[0].hand.len();

    let action = activation(&game, talisman_id).expect("the ability is offered");
    game.apply(PlayerId::One, action).expect("it is activated");
    resolve(&mut game);
    // The search asks which card to take; there is only one, and the engine
    // chooses for a lone eligible card.
    drain_pending(&mut game);

    assert_eq!(
        game.players[0].hand.len(),
        before + 1,
        "the searched card reaches the hand",
    );
    let talisman = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == talisman_id)
        .expect("the artifact stays on the battlefield");
    assert_eq!(
        talisman.controller,
        PlayerId::Two,
        "and the opponent controls it now",
    );
    assert_eq!(
        talisman.counters(CounterKind::named("wish")),
        2,
        "with the two wishes that are left",
    );
}

/// Haste is the half the creature notices: a Giant that arrived this turn
/// attacks the moment the Greaves are on it, and stops being able to when
/// they move on.
#[test]
fn the_greaves_hand_their_haste_to_whatever_wears_them() {
    let mut game = ready_game();
    game.battlefield.clear();
    let greaves = game
        .put_onto_battlefield(PlayerId::One, cards::LIGHTNING_GREAVES)
        .expect("cataloged");
    let mut bears = creature(83_000, cards::GRIZZLY_BEARS, PlayerId::One);
    // Arrived this turn, so it is summoning sick without help.
    bears.entered_controller_turn = game.turns_started[PlayerId::One.index()];
    let bears_id = bears.card.id;
    game.battlefield.push(bears);

    let hasty = |game: &Game| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == bears_id)
            .is_some_and(|permanent| {
                game.permanent_has_executable_keyword(permanent, KeywordAbility::Haste)
            })
    };

    assert!(!hasty(&game), "an unequipped creature has no haste");
    equip_to(&mut game, greaves, bears_id);
    assert!(hasty(&game), "and an equipped one does");
}

/// Shroud is the half the opponent notices, and it does not care whose spell
/// it is: the Greaves protect the creature from its own controller too.
#[test]
fn the_greaves_put_their_creature_out_of_reach_of_everyone() {
    let mut game = ready_game();
    game.battlefield.clear();
    let greaves = game
        .put_onto_battlefield(PlayerId::One, cards::LIGHTNING_GREAVES)
        .expect("cataloged");
    let bears = creature(83_010, cards::GRIZZLY_BEARS, PlayerId::One);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    let bolt = card(83_011, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[0].hand.push(bolt);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    let targetable = |game: &Game| {
        game.legal_actions(PlayerId::One).into_iter().any(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if card == bolt_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Permanent(bears_id)))
        })
    };

    assert!(
        targetable(&game),
        "an unequipped creature can be pointed at"
    );
    equip_to(&mut game, greaves, bears_id);
    assert!(
        !targetable(&game),
        "shroud stops its own controller too, which is the cost of the card",
    );
}
