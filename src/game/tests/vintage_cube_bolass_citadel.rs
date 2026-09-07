//! Bolas's Citadel: a library played like a hand, paid for in life, and a
//! finish that eats the board.

use super::*;

/// Player One with a Citadel out and `library` stacked so the last entry is
/// on top.
fn staged(library: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    for definition in library {
        let card = game
            .build_zone(PlayerId::One, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].library.push(card);
    }
    let citadel = game
        .put_onto_battlefield(PlayerId::One, cards::BOLASS_CITADEL)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [1, 1];
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, citadel)
}

fn top(game: &Game) -> GameObjectId {
    game.players[0].library.last().expect("a library").id
}

fn resolve(game: &mut Game) {
    for _ in 0..24 {
        if game.observe(PlayerId::One).decision.is_some()
            || game.observe(PlayerId::Two).decision.is_some()
        {
            return;
        }
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

/// The one action, if any, that casts `card`.
fn cast_of(game: &Game, card: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card: id, .. } if *id == card))
}

fn land_play_of(game: &Game, card: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card: id, .. } if *id == card))
}

/// The look clause is what makes the rest usable: without it a client would
/// be offered a card it cannot see.
#[test]
fn the_top_card_is_revealed_to_its_owner_alone() {
    let (game, citadel) = staged(&[cards::LIGHTNING_BOLT]);
    let card = top(&game);

    assert_eq!(
        game.observe(PlayerId::One).revealed_library_top,
        Some((card, cards::LIGHTNING_BOLT)),
        "the Citadel's controller sees it",
    );
    assert_eq!(
        game.observe(PlayerId::Two).revealed_library_top,
        None,
        "the opponent does not",
    );

    let mut game = game;
    game.battlefield
        .retain(|permanent| permanent.card.id != citadel);
    assert_eq!(
        game.observe(PlayerId::One).revealed_library_top,
        None,
        "and neither does anyone without a Citadel",
    );
}

/// A spell off the top costs life equal to its mana value and no mana at
/// all: the Bolt goes off with an empty pool.
#[test]
fn a_spell_from_the_top_costs_life_instead_of_mana() {
    let (mut game, _citadel) = staged(&[cards::LIGHTNING_BOLT]);
    let bolt = top(&game);
    let before = game.players[0].life;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == bolt
                    && choices.targets().iter().any(|selection| {
                        selection.targets().contains(&Target::Player(PlayerId::Two))
                    })
            }
            _ => false,
        })
        .expect("the Bolt on top is castable at a player");
    game.apply(PlayerId::One, action).expect("it casts");
    resolve(&mut game);

    assert_eq!(
        game.players[0].life,
        before - 1,
        "one life for a one-mana spell",
    );
    assert_eq!(game.players[1].life, 17, "and the Bolt resolved");
    assert!(
        game.players[0].mana_pool.total() == 0,
        "with no mana spent, because none was raised",
    );
}

#[test]
fn library_life_and_buyback_mana_source_life_share_one_cast_budget() {
    let (mut game, _citadel) = staged(&[cards::SPROUT_SWARM]);
    let sprout = top(&game);
    let sources = (0..3)
        .map(|_| {
            game.put_onto_battlefield(PlayerId::One, cards::MANA_CONFLUENCE)
                .expect("cataloged")
        })
        .collect::<Vec<_>>();

    game.players[PlayerId::One.index()].life = 5;
    let bought_back = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == sprout && !choices.costs().additional().is_empty())
        })
        .expect("two library life plus three mana-source life is affordable at five");

    game.players[PlayerId::One.index()].life = 4;
    assert!(
        game.legal_actions(PlayerId::One).iter().all(|action| {
            !matches!(action, Action::CastSpell { card, choices, .. }
                if *card == sprout && !choices.costs().additional().is_empty())
        }),
        "Buyback cannot spend three life on mana after the library cast reserves two",
    );
    assert!(!game.is_legal_action(PlayerId::One, &bought_back));
    assert!(game.apply(PlayerId::One, bought_back.clone()).is_err());
    assert_eq!(game.players[PlayerId::One.index()].life, 4);
    assert_eq!(
        top(&game),
        sprout,
        "rejection leaves the spell on the library"
    );
    assert!(sources.iter().all(|source| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == *source)
            .is_some_and(|permanent| !permanent.tapped)
    }));

    game.players[PlayerId::One.index()].life = 5;
    game.apply(PlayerId::One, bought_back)
        .expect("the same aggregate payment is legal with five life");
    assert_eq!(game.players[PlayerId::One.index()].life, 0);
}

/// "Rather than pay its mana cost" is the whole cost: a player with no mana
/// at all still casts it, and one with no life does not.
#[test]
fn a_spell_nobody_has_the_life_for_is_not_offered() {
    let (mut game, _citadel) = staged(&[cards::SERRA_ANGEL]);
    let angel = top(&game);
    game.players[0].life = 4;

    assert!(
        cast_of(&game, angel).is_none(),
        "a five-mana Angel is out of reach at four life... ",
    );

    game.players[0].life = 5;
    assert!(
        cast_of(&game, angel).is_some(),
        "...while paying down to exactly zero is allowed",
    );
}

/// Lands come off the top too, and they cost nothing: only spells have a
/// mana cost for the permission to replace.
#[test]
fn a_land_from_the_top_costs_no_life() {
    let (mut game, _citadel) = staged(&[cards::MOUNTAIN]);
    let mountain = top(&game);
    let before = game.players[0].life;

    let action = land_play_of(&game, mountain).expect("the land on top is playable");
    game.apply(PlayerId::One, action).expect("it plays");
    resolve(&mut game);

    assert_eq!(game.players[0].life, before, "a land is free");
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::MOUNTAIN),
        "and it arrived",
    );
    assert!(
        game.players[0].library.is_empty(),
        "off the library rather than out of nowhere",
    );
}

/// Only the topmost card, and only while the Citadel is out.
#[test]
fn the_second_card_down_stays_where_it_is() {
    let (game, citadel) = staged(&[cards::SERRA_ANGEL, cards::LIGHTNING_BOLT]);
    let buried = game.players[0].library[0].id;

    assert!(
        cast_of(&game, buried).is_none(),
        "the card under the top one is not offered",
    );

    let mut game = game;
    let bolt = top(&game);
    game.battlefield
        .retain(|permanent| permanent.card.id != citadel);
    assert!(
        cast_of(&game, bolt).is_none(),
        "and without the Citadel neither is the top one",
    );
}

/// Select the whole payment, then sacrifice ten and drain each opponent.
#[test]
fn the_sacrifice_ability_eats_ten_and_drains_ten() {
    let (mut game, citadel) = staged(&[]);
    for _ in 0..10 {
        game.put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
            .expect("cataloged");
    }
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    drain_pending(&mut game);
    game.priority = PlayerId::One;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == citadel),
        )
        .expect("ten nonland permanents pay for it");
    game.apply(PlayerId::One, action)
        .expect("it opens the payment window");
    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.minimum, 10);
    assert_eq!(decision.maximum, 10);
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == citadel)
            .unwrap()
            .tapped,
        "choosing the payment did not pay the tap cost"
    );
    let options = decision
        .options
        .iter()
        .filter(|option| {
            option.card.is_some_and(|(_, characteristics)| {
                characteristics.card_definition() == Some(cards::SAVANNAH_LIONS)
            })
        })
        .map(|option| option.id)
        .collect();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options,
        },
    )
    .unwrap();
    resolve(&mut game);

    assert_eq!(game.players[1].life, 10, "each opponent lost ten");
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == cards::SAVANNAH_LIONS)
            .count(),
        0,
        "and all ten Lions went",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == citadel),
        "the Citadel itself is a nonland permanent but was not among the ten chosen",
    );
}

/// Nine is not ten: the ability is not offered at all.
#[test]
fn nine_permanents_do_not_pay_for_it() {
    let (mut game, citadel) = staged(&[]);
    for _ in 0..8 {
        game.put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
            .expect("cataloged");
    }
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == citadel)
        ),
        "eight Lions and the Citadel are nine nonland permanents",
    );
}

/// One offer, not one per way to pay it. The whole reason the cost is a
/// decision rather than an enumeration.
#[test]
fn a_wide_board_still_offers_the_ability_once() {
    let (mut game, citadel) = staged(&[]);
    for _ in 0..19 {
        game.put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
            .expect("cataloged");
    }
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    assert_eq!(
        game.legal_actions(PlayerId::One)
            .iter()
            .filter(|action| matches!(
                action,
                Action::ActivateAbility { source, .. } if *source == citadel
            ))
            .count(),
        1,
        "twenty nonland permanents would be 184,756 ways to choose ten",
    );
}

/// "Bolas's Citadel may be one of the permanents you sacrifice to activate
/// its last ability." Nine Lions and the Citadel itself is ten.
#[test]
fn the_citadel_may_eat_itself() {
    let (mut game, citadel) = staged(&[]);
    for _ in 0..9 {
        game.put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
            .expect("cataloged");
    }
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == citadel),
        )
        .expect("nine Lions and the Citadel are ten nonland permanents");
    game.apply(PlayerId::One, action).expect("it activates");

    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.minimum, 10);
    assert_eq!(decision.maximum, 10);
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == citadel)
            .unwrap()
            .tapped,
        "choosing the payment did not pay the tap cost"
    );
    let options = decision.options.iter().map(|option| option.id).collect();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options,
        },
    )
    .unwrap();
    resolve(&mut game);

    assert_eq!(game.players[1].life, 10, "the drain happened all the same");
    assert!(
        game.battlefield.is_empty(),
        "and the board it ate included the Citadel itself",
    );
}

/// "If a spell has {X} in its mana cost, you must choose 0 as the value of X
/// when casting it without paying its mana cost." A Ballista off the top is
/// an {X}{X} spell at nought, so the life it asks for is nothing and what
/// arrives is a 0/0 that nothing can save.
#[test]
fn an_x_spell_from_the_top_is_cast_for_nought() {
    let (mut game, _citadel) = staged(&[cards::WALKING_BALLISTA]);
    let ballista = top(&game);
    let life = game.players[0].life;

    let cast = cast_of(&game, ballista).expect("the top card is castable");
    game.apply(PlayerId::One, cast).expect("it is cast");
    resolve(&mut game);
    game.check_state_based_actions();

    assert_eq!(
        game.players[0].life, life,
        "X of nought is a mana value of nought, and no life to pay",
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::WALKING_BALLISTA),
        "and X was nought, so what arrived was a 0/0",
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::WALKING_BALLISTA),
        "buried by state-based actions",
    );
}

/// "You can, however, pay additional costs. If the card has any mandatory
/// additional costs, those must be paid to cast the card." A Bitter Triumph
/// off the top is two life for the mana value and three more for the cost it
/// prints -- five life for a removal spell that cost two mana yesterday, and
/// no mana at all.
#[test]
fn a_mandatory_additional_cost_is_paid_on_top_of_the_life() {
    let (mut game, _citadel) = staged(&[cards::MOUNTAIN, cards::BITTER_TRIUMPH]);
    let triumph = top(&game);
    let bears = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.priority = PlayerId::One;
    let life = game.players[0].life;

    // With an empty hand the discard half is unavailable, so the only way to
    // pay the additional cost is the three life beside the two.
    assert!(game.players[0].hand.is_empty(), "nothing to discard");
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == triumph
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Permanent(bears))
            }
            _ => false,
        })
        .expect("the Triumph is castable off the top");
    game.apply(PlayerId::One, cast).expect("it is cast");
    resolve(&mut game);

    assert_eq!(
        game.players[0].mana_pool.total(),
        0,
        "no mana was there and none was spent",
    );
    assert_eq!(
        game.players[0].life,
        life - 5,
        "two for the mana value and three for the cost it prints",
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == bears),
        "and the bear is destroyed, which is what the five bought",
    );
}
