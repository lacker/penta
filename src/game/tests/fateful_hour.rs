//! Fateful hour: a clause that only applies at five life or less.
//!
//! The two cards read the threshold differently, and the difference matters.
//! Break of Day checks once as it resolves, so what it grants survives a
//! later life gain; Gavony Ironwright's "as long as" is continuous, so the
//! anthem switches off the moment life climbs back above five.

use super::*;
use crate::ImplementationStatus;

fn ready(life: i16) -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.players[PlayerId::One.index()].life = life;
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

fn indestructible(game: &Game, id: GameObjectId) -> bool {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there");
    game.permanent_has_executable_keyword(permanent, KeywordAbility::Indestructible)
}

/// Casts Break of Day and returns the creature it affected.
fn break_of_day(game: &mut Game) -> GameObjectId {
    let bear = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);

    let spell = card(20_000, cards::BREAK_OF_DAY, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.white = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("two mana covers it");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(game);
    bear_id
}

#[test]
fn break_of_day_grants_indestructible_at_five_life() {
    let mut game = ready(5);
    let bear = break_of_day(&mut game);

    assert_eq!(stats(&game, bear), (Some(3), Some(3)), "the pump landed");
    assert!(indestructible(&game, bear), "and so did the fateful hour");
}

/// The control: above the threshold only the pump happens.
#[test]
fn break_of_day_only_pumps_above_five_life() {
    let mut game = ready(6);
    let bear = break_of_day(&mut game);

    assert_eq!(stats(&game, bear), (Some(3), Some(3)));
    assert!(!indestructible(&game, bear), "six life is not fateful");
}

/// The Ironwright's anthem is continuous, so life moving turns it on and off.
#[test]
fn the_ironwrights_anthem_follows_the_life_total() {
    let mut game = ready(5);
    game.battlefield
        .push(creature(10_000, cards::GAVONY_IRONWRIGHT, PlayerId::One));
    let bear = creature(10_100, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);

    assert_eq!(
        stats(&game, bear_id),
        (Some(3), Some(6)),
        "a 2/2 with +1/+4",
    );

    game.players[PlayerId::One.index()].life = 6;
    assert_eq!(
        stats(&game, bear_id),
        (Some(2), Some(2)),
        "one life back and the anthem is gone",
    );

    game.players[PlayerId::One.index()].life = 1;
    assert_eq!(stats(&game, bear_id), (Some(3), Some(6)), "and back again");
}

/// "Other creatures", so the Ironwright does not pump itself.
#[test]
fn the_ironwright_does_not_pump_itself() {
    let mut game = ready(3);
    let iron = creature(10_000, cards::GAVONY_IRONWRIGHT, PlayerId::One);
    let iron_id = iron.card.id;
    game.battlefield.push(iron);

    assert_eq!(stats(&game, iron_id), (Some(1), Some(4)), "printed size");
}

/// "Instead", so the count is chosen rather than the second creation being
/// skipped: two above the threshold, five at or below it.
#[test]
fn gather_the_townsfolk_makes_two_or_five() {
    let make = |life: i16| {
        let mut game = ready(life);
        let spell = card(20_000, cards::GATHER_THE_TOWNSFOLK, PlayerId::One);
        let spell_id = spell.id;
        game.players[PlayerId::One.index()].hand.push(spell);
        game.players[PlayerId::One.index()].mana_pool.white = 1;
        game.players[PlayerId::One.index()].mana_pool.colorless = 1;

        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
            .expect("two mana covers it");
        game.apply(PlayerId::One, action)
            .expect("the cast is legal");
        drain_pending(&mut game);
        game.battlefield
            .iter()
            .filter(|permanent| {
                is_token_with(
                    permanent,
                    tokens::creature(&["Human"], &[ManaColor::White], 1, 1),
                )
            })
            .count()
    };

    assert_eq!(make(6), 2, "above the threshold");
    assert_eq!(make(5), 5, "at it");
}

/// The Doomsayer's own tokens are among the creatures its anthem pumps.
#[test]
fn the_doomsayer_pumps_the_tokens_it_makes() {
    let mut game = ready(5);
    let sayer = creature(10_000, cards::THRABEN_DOOMSAYER, PlayerId::One);
    let sayer_id = sayer.card.id;
    game.battlefield.push(sayer);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == sayer_id))
        .expect("tapping is the whole cost");
    game.apply(PlayerId::One, action).expect("legal");
    drain_pending(&mut game);

    let token = game
        .battlefield
        .iter()
        .find(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Human"], &[ManaColor::White], 1, 1),
            )
        })
        .expect("a token arrived")
        .card
        .id;
    assert_eq!(stats(&game, token), (Some(3), Some(3)), "a 1/1 with +2/+2");
    assert_eq!(
        stats(&game, sayer_id),
        (Some(2), Some(2)),
        "\"other\", so not itself",
    );

    game.players[PlayerId::One.index()].life = 20;
    assert_eq!(
        stats(&game, token),
        (Some(1), Some(1)),
        "and the anthem lapses with the life total",
    );
}

/// Village Survivors has vigilance outright; the fateful-hour clause hands it
/// to everything else, and takes it back when life climbs.
#[test]
fn the_survivors_share_vigilance_only_below_the_threshold() {
    let mut game = ready(5);
    let survivors = creature(10_000, cards::VILLAGE_SURVIVORS, PlayerId::One);
    let survivors_id = survivors.card.id;
    game.battlefield.push(survivors);
    let bear = creature(10_100, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);

    let vigilant = |game: &Game, id: GameObjectId| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("still there");
        game.permanent_has_executable_keyword(permanent, KeywordAbility::Vigilance)
    };

    assert!(vigilant(&game, bear_id), "shared at five life");
    assert!(vigilant(&game, survivors_id), "and printed on itself");

    game.players[PlayerId::One.index()].life = 20;
    assert!(!vigilant(&game, bear_id), "taken back above the threshold");
    assert!(
        vigilant(&game, survivors_id),
        "but the printed keyword is not what the branch reads",
    );
}

/// Clinging Mists fogs either way and only holds the attackers down at five
/// life or less.
#[test]
fn clinging_mists_holds_attackers_only_below_the_threshold() {
    let cast_at = |life: i16| {
        let mut game = ready(life);
        game.active_player = PlayerId::Two;
        let mut attacker = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::Two);
        attacker.attacking = true;
        let attacker_id = attacker.card.id;
        game.battlefield.push(attacker);

        let spell = card(20_000, cards::CLINGING_MISTS, PlayerId::One);
        let spell_id = spell.id;
        game.players[PlayerId::One.index()].hand.push(spell);
        game.players[PlayerId::One.index()].mana_pool.green = 1;
        game.players[PlayerId::One.index()].mana_pool.colorless = 2;
        game.step = Step::DeclareBlockers;
        game.attackers_declared = true;
        game.blockers_declared = true;
        game.priority = PlayerId::One;

        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
            .expect("three mana covers it");
        game.apply(PlayerId::One, action)
            .expect("the cast is legal");
        drain_pending(&mut game);
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == attacker_id)
            .expect("still there")
            .tapped
    };

    assert!(!cast_at(6), "above the threshold it is only a Fog");
    assert!(cast_at(5), "at it the attackers go down too");
}

/// And the attackers it taps miss their controller's next untap step, which
/// is the half of the clause the tap alone does not show.
#[test]
fn clinging_mists_holds_them_through_the_next_untap_step() {
    let mut game = ready(5);
    game.active_player = PlayerId::Two;
    let mut attacker = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);

    let spell = card(20_000, cards::CLINGING_MISTS, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.blockers_declared = true;
    game.priority = PlayerId::One;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("three mana covers it");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(&mut game);

    let tapped = |game: &Game| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == attacker_id)
            .expect("still there")
            .tapped
    };
    game.commit_next_turn(PlayerId::Two, Vec::new());
    drain_pending(&mut game);
    assert!(tapped(&game), "it skipped their untap step");

    game.commit_next_turn(PlayerId::One, Vec::new());
    game.commit_next_turn(PlayerId::Two, Vec::new());
    drain_pending(&mut game);
    assert!(!tapped(&game), "and came back a cycle later");
}

#[test]
fn both_cards_report_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [
        cards::BREAK_OF_DAY,
        cards::GAVONY_IRONWRIGHT,
        cards::GATHER_THE_TOWNSFOLK,
        cards::THRABEN_DOOMSAYER,
        cards::CLINGING_MISTS,
        cards::VILLAGE_SURVIVORS,
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
