//! Two cards whose audit lines said populate was unavailable.
//!
//! It was not: populate has its own procedure and its own tests already.
//! What is worth pinning here is what each card wraps around it -- a
//! self-counting body on one, and an ordering on the other, since the
//! destruction happens before the copy is chosen.

use super::*;
use crate::ImplementationStatus;

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

fn tokens(game: &Game, token: TokenCharacteristics) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| is_token_with(permanent, token))
        .count()
}

/// Alone, the Temple counts only itself.
#[test]
fn the_temple_is_a_one_one_on_an_empty_board() {
    let mut game = ready();
    let temple = creature(10_000, cards::WAYFARING_TEMPLE, PlayerId::One);
    let temple_id = temple.card.id;
    game.battlefield.push(temple);

    assert_eq!(stats(&game, temple_id), (Some(1), Some(1)));
}

/// The count is live and covers only creatures its controller has.
#[test]
fn the_temple_grows_with_your_own_creatures() {
    let mut game = ready();
    let temple = creature(10_000, cards::WAYFARING_TEMPLE, PlayerId::One);
    let temple_id = temple.card.id;
    game.battlefield.push(temple);
    for index in 0..2 {
        game.battlefield.push(creature(
            10_100 + index,
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }
    game.battlefield
        .push(creature(10_200, cards::GRIZZLY_BEARS, PlayerId::Two));

    assert_eq!(
        stats(&game, temple_id),
        (Some(3), Some(3)),
        "itself and two others; theirs does not count",
    );

    game.battlefield
        .retain(|permanent| permanent.card.id != CardInstanceId(10_100));
    assert_eq!(stats(&game, temple_id), (Some(2), Some(2)));
}

/// Sundering Growth destroys and then populates, in that order.
#[test]
fn sundering_growth_destroys_then_copies_a_token() {
    let mut game = ready();
    let mox = creature(10_000, cards::MOX_JET, PlayerId::Two);
    let mox_id = mox.card.id;
    game.battlefield.push(mox);
    game.battlefield.push(token_permanent(
        10_100,
        tokens::creature(&["Zombie"], &[ManaColor::Black], 2, 2),
        PlayerId::One,
    ));

    let spell = card(20_000, cards::SUNDERING_GROWTH, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.green = 2;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("two green pays a hybrid cost");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == mox_id),
        "the artifact went",
    );
    assert_eq!(
        tokens(
            &game,
            tokens::creature(&["Zombie"], &[ManaColor::Black], 2, 2)
        ),
        2,
        "and the token was copied",
    );
}

/// A board with no creature token is not a failure: the destruction still
/// happens and populate simply does nothing.
#[test]
fn sundering_growth_still_destroys_with_no_token_to_copy() {
    let mut game = ready();
    let mox = creature(10_000, cards::MOX_JET, PlayerId::Two);
    let mox_id = mox.card.id;
    game.battlefield.push(mox);

    let spell = card(20_000, cards::SUNDERING_GROWTH, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.white = 2;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("two white pays it too");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == mox_id),
    );
    assert_eq!(
        tokens(
            &game,
            tokens::creature(&["Zombie"], &[ManaColor::Black], 2, 2)
        ),
        0
    );
}

/// Trostani reads the entering creature's toughness, not her own and not a
/// constant.
#[test]
fn trostani_gains_the_entering_creatures_toughness() {
    let mut game = ready();
    game.battlefield.push(creature(
        10_000,
        cards::TROSTANI_SELESNYAS_VOICE,
        PlayerId::One,
    ));
    let before = game.players[PlayerId::One.index()].life;

    // Air Elemental is a 4/4, so a toughness read gains four.
    let spell = card(20_000, cards::AIR_ELEMENTAL, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.blue = 2;
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("six mana covers it");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(&mut game);

    assert_eq!(game.players[PlayerId::One.index()].life, before + 4);
}

/// "Another" creature: Trostani arriving does not feed herself.
#[test]
fn trostani_does_not_trigger_on_herself() {
    let mut game = ready();
    let before = game.players[PlayerId::One.index()].life;
    let spell = card(20_000, cards::TROSTANI_SELESNYAS_VOICE, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.green = 2;
    game.players[PlayerId::One.index()].mana_pool.white = 2;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("four mana covers her");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(&mut game);

    assert_eq!(game.players[PlayerId::One.index()].life, before);
}

/// The Guildmage's two abilities are separate, each with its own price.
#[test]
fn the_guildmage_offers_both_abilities_at_their_own_costs() {
    let mut game = ready();
    let mage = creature(10_000, cards::VITU_GHAZI_GUILDMAGE, PlayerId::One);
    let mage_id = mage.card.id;
    game.battlefield.push(mage);
    game.battlefield.push(token_permanent(
        10_100,
        tokens::creature(&["Zombie"], &[ManaColor::Black], 2, 2),
        PlayerId::One,
    ));
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    game.players[PlayerId::One.index()].mana_pool.white = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    // Four mana reaches the populate ability but not the Centaur.
    let offered = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == mage_id),
        )
        .count();
    assert_eq!(offered, 1, "only the cheaper of the two is affordable");

    game.players[PlayerId::One.index()].mana_pool.colorless = 4;
    let offered = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == mage_id),
        )
        .count();
    assert_eq!(offered, 2, "six mana reaches both");
}

/// Populate on its own, with nothing else in the spell.
#[test]
fn wake_the_reflections_copies_a_token() {
    let mut game = ready();
    game.battlefield.push(token_permanent(
        10_000,
        tokens::creature(&["Zombie"], &[ManaColor::Black], 2, 2),
        PlayerId::One,
    ));

    let spell = card(20_000, cards::WAKE_THE_REFLECTIONS, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.white = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("one white covers it");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(&mut game);

    assert_eq!(
        tokens(
            &game,
            tokens::creature(&["Zombie"], &[ManaColor::Black], 2, 2)
        ),
        2
    );
}

/// Druid's Deliverance shields only its own controller, not the whole
/// combat: their creature still takes what mine deals it.
#[test]
fn druids_deliverance_shields_only_its_controller() {
    let mut game = ready();
    game.active_player = PlayerId::Two;

    let mut attacker = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let mut blocker = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One);
    blocker.blocking = vec![attacker_id];
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);

    let spell = card(20_000, cards::DRUIDS_DELIVERANCE, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.blockers_declared = true;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("two mana covers it");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(&mut game);

    game.step = Step::CombatDamage;
    game.deal_combat_damage();
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        i16::from(rules::STARTING_LIFE),
        "no combat damage reached its controller",
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == attacker_id),
        "but the shield is player-only, so their creature still traded",
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == blocker_id),
        "and so did mine",
    );
}

#[test]
fn both_cards_report_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [
        cards::WAYFARING_TEMPLE,
        cards::SUNDERING_GROWTH,
        cards::TROSTANI_SELESNYAS_VOICE,
        cards::VITU_GHAZI_GUILDMAGE,
        cards::WAKE_THE_REFLECTIONS,
        cards::DRUIDS_DELIVERANCE,
    ] {
        let card = catalog.get(definition).expect("the card is cataloged");
        assert_eq!(
            card.rules.implementation_status(),
            ImplementationStatus::Complete,
            "{} should be fully executable",
            card.name,
        );
    }
}
