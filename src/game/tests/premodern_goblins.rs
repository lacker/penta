//! The Goblins that make up most of a Premodern tribal deck.
//!
//! Each of these reads the others: the Warchief discounts and hastens the
//! rest, the Piledriver is sized by them, the Lackey puts them down for
//! free, and the Commander turns any of them into damage. The tests use a
//! non-Goblin control everywhere the tribe is named, so a clause that
//! quietly dropped the subtype would fail.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game
}

fn castable(game: &Game, id: GameObjectId) -> bool {
    game.legal_actions(PlayerId::One)
        .iter()
        .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
}

fn stats(game: &Game, id: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there");
    (game.power(permanent), game.toughness(permanent))
}

/// Resolves everything, taking the last option of any decision.
fn settle(game: &mut Game) {
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .last()
                .map(|option| vec![option.id])
                .unwrap_or_default();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the decision accepts what it offered");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
}

/// Two other attacking Goblins is +4/+0, and a non-Goblin attacker beside
/// them adds nothing.
#[test]
fn the_piledriver_counts_other_attacking_goblins_only() {
    let mut game = ready();
    let driver = creature(10_000, cards::GOBLIN_PILEDRIVER, PlayerId::One);
    let driver_id = driver.card.id;
    game.battlefield.push(driver);
    game.battlefield
        .push(creature(10_001, cards::MOGG_FANATIC, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::GOBLIN_SHARPSHOOTER, PlayerId::One));
    // Not a Goblin, and a Goblin that is not attacking.
    game.battlefield
        .push(creature(10_003, cards::GRIZZLY_BEARS, PlayerId::One));
    game.battlefield
        .push(creature(10_004, cards::GOBLIN_MATRON, PlayerId::One));
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
        permanent.attacking = permanent.card.id != CardInstanceId(10_004);
    }

    let attacking = game.trigger_event_object(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == driver_id)
            .expect("still there"),
    );
    game.capture_battlefield_triggers(&CommittedTriggerEvent::Attacks {
        object: attacking,
        declaration_size: 4,
        attack_number: 1,
        defending_player: PlayerId::Two,
        attacked_a_planeswalker: false,
    });
    settle(&mut game);

    assert_eq!(
        stats(&game, driver_id),
        (Some(5), Some(2)),
        "two other attacking Goblins, +4/+0 on a 1/2",
    );
}

/// The Warchief discounts Goblin spells and nothing else.
#[test]
fn the_warchief_discounts_only_goblin_spells() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::GOBLIN_WARCHIEF, PlayerId::One));
    let goblin = card(20_000, cards::GOBLIN_MATRON, PlayerId::One);
    let goblin_id = goblin.id;
    game.players[PlayerId::One.index()].hand.push(goblin);
    let bear = card(20_001, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.id;
    game.players[PlayerId::One.index()].hand.push(bear);
    // One red and one generic: enough for a {2}{R} Matron only once the
    // discount applies, and never enough for a green bear.
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    assert!(
        castable(&game, goblin_id),
        "a three-mana Matron paid with two is the discount",
    );
    assert!(!castable(&game, bear_id), "no green for a bear either way");
}

/// The mirror, with a pool that would pay for the bear only if the discount
/// wrongly reached it.
#[test]
fn the_warchief_does_not_discount_a_non_goblin_spell() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::GOBLIN_WARCHIEF, PlayerId::One));
    let bear = card(20_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.id;
    game.players[PlayerId::One.index()].hand.push(bear);
    // A single green: {1}{G} discounted to {G} would be castable, and the
    // printed cost is not.
    game.players[PlayerId::One.index()].mana_pool.green = 1;

    assert!(
        !castable(&game, bear_id),
        "a bear is not a Goblin spell, so it still costs two",
    );
}

/// And hastens Goblins you control, including ones that arrive later.
#[test]
fn the_warchief_hastens_goblins_you_control() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::GOBLIN_WARCHIEF, PlayerId::One));
    let matron = creature(10_001, cards::GOBLIN_MATRON, PlayerId::One);
    let matron_id = matron.card.id;
    game.battlefield.push(matron);
    let bear = creature(10_002, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    let theirs = creature(10_003, cards::MOGG_FANATIC, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);

    let hasty = |game: &Game, id: GameObjectId| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .is_some_and(|permanent| {
                game.permanent_has_executable_keyword(permanent, KeywordAbility::Haste)
            })
    };
    assert!(hasty(&game, matron_id), "your Goblin has haste");
    assert!(!hasty(&game, bear_id), "your non-Goblin does not");
    assert!(!hasty(&game, theirs_id), "and neither does theirs");
}

/// The Commander brings three friends.
#[test]
fn the_commander_arrives_with_three_goblins() {
    let mut game = ready();
    game.put_onto_battlefield(PlayerId::One, cards::SIEGE_GANG_COMMANDER)
        .expect("cataloged");
    settle(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| is_token_with(
                permanent,
                tokens::creature(&["Goblin"], &[ManaColor::Red], 1, 1)
            ))
            .count(),
        3,
        "three tokens",
    );
}

/// Connecting puts a Goblin down for free, and only a Goblin: the offer
/// names the tribe rather than the hand.
#[test]
fn the_lackey_puts_a_goblin_from_hand_onto_the_battlefield() {
    let mut game = ready();
    let lackey = creature(10_000, cards::GOBLIN_LACKEY, PlayerId::One);
    let lackey_id = lackey.card.id;
    game.battlefield.push(lackey);
    game.players[PlayerId::One.index()].hand.push(card(
        20_000,
        cards::SIEGE_GANG_COMMANDER,
        PlayerId::One,
    ));
    game.players[PlayerId::One.index()].hand.push(card(
        20_001,
        cards::GRIZZLY_BEARS,
        PlayerId::One,
    ));

    game.damage_target_from(Some(lackey_id), Some(Target::Player(PlayerId::Two)), 1);
    settle(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SIEGE_GANG_COMMANDER),
        "a five-drop for free, which is the whole point of the card",
    );
    assert!(
        game.stack.is_empty(),
        "it was put onto the battlefield rather than cast",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::GRIZZLY_BEARS),
        "and the bear stayed in hand, never having been on offer",
    );
}

/// The Tinkerer takes the destroyed artifact's mana value, read after it has
/// already gone: a one-mana artifact leaves the 1/2 alive, a two-mana one
/// kills it, which is the trade the card offers.
fn tinker_at(artifact: CardDefinitionId) -> (Game, GameObjectId) {
    let mut game = ready();
    let tinkerer = creature(10_000, cards::GOBLIN_TINKERER, PlayerId::One);
    let tinkerer_id = tinkerer.card.id;
    game.battlefield.push(tinkerer);
    let target = creature(10_001, artifact, PlayerId::Two);
    let target_id = target.card.id;
    game.battlefield.push(target);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.players[PlayerId::One.index()].mana_pool.red = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == tinkerer_id
                    && targets.iter().any(|selection| {
                        selection.targets().contains(&Target::Permanent(target_id))
                    })
            }
            _ => false,
        })
        .expect("the artifact can be named");
    game.apply(PlayerId::One, action).expect("it is activated");
    settle(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == target_id),
        "the artifact was destroyed",
    );
    (game, tinkerer_id)
}

#[test]
fn the_tinkerer_takes_one_damage_from_a_one_mana_artifact() {
    let (game, tinkerer) = tinker_at(cards::BLACK_VISE);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == tinkerer)
        .expect("a 1/2 survives one damage");
    assert_eq!(permanent.damage, 1, "a Vise costs one, so one damage");
}

#[test]
fn a_two_mana_artifact_kills_the_tinkerer() {
    let (game, tinkerer) = tinker_at(cards::ANKH_OF_MISHRA);
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == tinkerer),
        "two damage is lethal to a 1/2",
    );
}

/// The Ringleader takes every Goblin in the top four and leaves the rest,
/// asking nothing: the clause is mandatory, so a bounded choice would be the
/// wrong shape for it.
#[test]
fn the_ringleader_takes_every_goblin_from_the_top_four() {
    let mut game = ready();
    game.players[PlayerId::One.index()].library.clear();
    // Bottom to top: the last four pushed are the ones revealed. Two Goblins
    // and two not, plus a fifth Goblin sitting below the dig so that "all
    // Goblins" cannot quietly mean "all Goblins in the library".
    game.players[PlayerId::One.index()].library.push(card(
        30_000,
        cards::GOBLIN_MATRON,
        PlayerId::One,
    ));
    game.players[PlayerId::One.index()].library.push(card(
        30_001,
        cards::GRIZZLY_BEARS,
        PlayerId::One,
    ));
    game.players[PlayerId::One.index()].library.push(card(
        30_002,
        cards::MOGG_FANATIC,
        PlayerId::One,
    ));
    game.players[PlayerId::One.index()].library.push(card(
        30_003,
        cards::SAVANNAH_LIONS,
        PlayerId::One,
    ));
    game.players[PlayerId::One.index()].library.push(card(
        30_004,
        cards::GOBLIN_SHARPSHOOTER,
        PlayerId::One,
    ));

    game.put_onto_battlefield(PlayerId::One, cards::GOBLIN_RINGLEADER)
        .expect("cataloged");
    settle(&mut game);

    // The order cards arrive in hand is not something the card specifies.
    let mut hand: Vec<_> = game.players[PlayerId::One.index()]
        .hand
        .iter()
        .map(|card| card.definition)
        .collect();
    hand.sort_unstable();
    let mut expected = vec![cards::MOGG_FANATIC, cards::GOBLIN_SHARPSHOOTER];
    expected.sort_unstable();
    assert_eq!(hand, expected, "both revealed Goblins, and nothing else");
    assert_eq!(
        game.players[PlayerId::One.index()].library.len(),
        3,
        "the two non-Goblins went to the bottom, and the fifth card never moved",
    );
    assert!(
        game.pending_decisions.is_empty(),
        "nothing was asked -- the clause does not offer a choice",
    );
}

/// The Prospector's sacrifice is a choice of which Goblin, and a mana
/// ability has no window in which to ask -- so each candidate is its own
/// offered activation, and a non-Goblin is never among them.
#[test]
fn the_prospector_offers_one_activation_per_goblin() {
    let mut game = ready();
    let prospector = creature(10_000, cards::SKIRK_PROSPECTOR, PlayerId::One);
    let prospector_id = prospector.card.id;
    game.battlefield.push(prospector);
    let matron = creature(10_001, cards::GOBLIN_MATRON, PlayerId::One);
    let matron_id = matron.card.id;
    game.battlefield.push(matron);
    // Not a Goblin, and a Goblin the other player controls: neither is
    // yours to sacrifice.
    game.battlefield
        .push(creature(10_002, cards::GRIZZLY_BEARS, PlayerId::One));
    game.battlefield
        .push(creature(10_003, cards::MOGG_FANATIC, PlayerId::Two));

    let mut offered: Vec<_> = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateManaAbility {
                source,
                cost_object,
                ..
            } if source == prospector_id => cost_object,
            _ => None,
        })
        .collect();
    offered.sort_unstable();
    let mut expected = vec![prospector_id, matron_id];
    expected.sort_unstable();
    assert_eq!(
        offered, expected,
        "your two Goblins, and the Prospector counts as one of them",
    );
}

/// Sacrificing the Prospector to its own ability is legal, and it produces
/// the mana on its way out rather than being unable to pay for itself.
#[test]
fn the_prospector_can_eat_itself_for_mana() {
    let mut game = ready();
    let prospector = creature(10_000, cards::SKIRK_PROSPECTOR, PlayerId::One);
    let prospector_id = prospector.card.id;
    game.battlefield.push(prospector);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateManaAbility { cost_object: Some(id), .. } if *id == prospector_id
            )
        })
        .expect("a lone Prospector can still sacrifice itself");
    game.apply(PlayerId::One, action).expect("it is activated");
    settle(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.red,
        1,
        "one red mana",
    );
    assert!(game.battlefield.is_empty(), "and the Prospector is gone");
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SKIRK_PROSPECTOR),
        "into the graveyard, not exile",
    );
}

/// Eating a different Goblin leaves the Prospector standing, ready to do it
/// again.
#[test]
fn the_prospector_survives_eating_another_goblin() {
    let mut game = ready();
    let prospector = creature(10_000, cards::SKIRK_PROSPECTOR, PlayerId::One);
    let prospector_id = prospector.card.id;
    game.battlefield.push(prospector);
    let matron = creature(10_001, cards::GOBLIN_MATRON, PlayerId::One);
    let matron_id = matron.card.id;
    game.battlefield.push(matron);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateManaAbility { cost_object: Some(id), .. } if *id == matron_id
            )
        })
        .expect("the Matron is on offer");
    game.apply(PlayerId::One, action).expect("it is activated");
    settle(&mut game);

    assert_eq!(game.players[PlayerId::One.index()].mana_pool.red, 1);
    let standing: Vec<_> = game
        .battlefield
        .iter()
        .map(|permanent| permanent.card.id)
        .collect();
    assert_eq!(
        standing,
        vec![prospector_id],
        "the Matron went, the Prospector stayed",
    );
}

/// With no Goblin to eat there is no ability at all -- the cost is not
/// optional.
#[test]
fn a_prospector_with_nothing_to_eat_offers_nothing() {
    let mut game = ready();
    // The Prospector itself is the only Goblin it could ever eat, so the
    // negative case has to be a board where it is not there. A Matron alone
    // has no sacrifice ability of its own.
    game.battlefield
        .push(creature(10_000, cards::GOBLIN_MATRON, PlayerId::One));

    let offers_mana = game
        .legal_actions(PlayerId::One)
        .iter()
        .any(|action| matches!(action, Action::ActivateManaAbility { .. }));
    assert!(!offers_mana, "no Prospector, no red mana from a Goblin");
}
