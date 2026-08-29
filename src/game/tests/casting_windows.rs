//! Two spells that exist only inside one step.
//!
//! Festival is cast in an opponent's upkeep -- their turn, before they have
//! done anything with it -- and Teleport in the declare-attackers step, which
//! is what makes it usable once the attack is on the table. The window is
//! most of each card, so what these check is when each one is offered.

use super::*;

/// `spell` in player one's hand with enough mana, and a creature apiece.
fn holding(spell: CardDefinitionId) -> (Game, GameObjectId, CardInstanceId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    let theirs = creature(10_000, cards::SEDGE_TROLL, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);

    let card_in_hand = card(20_000, spell, PlayerId::One);
    let spell_id = card_in_hand.id;
    game.players[PlayerId::One.index()].hand.push(card_in_hand);
    game.players[PlayerId::One.index()].mana_pool.white = 1;
    game.players[PlayerId::One.index()].mana_pool.blue = 3;
    game.priority = PlayerId::One;
    (game, theirs_id, spell_id)
}

fn castable(game: &Game, spell: CardInstanceId) -> bool {
    game.legal_actions(PlayerId::One)
        .iter()
        .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
}

fn cast_it(game: &mut Game, spell: CardInstanceId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("the window is open");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(game);
}

#[test]
fn the_festival_waits_for_an_opponents_upkeep() {
    let (mut game, _theirs, spell) = holding(cards::FESTIVAL);

    game.active_player = PlayerId::One;
    game.step = Step::Upkeep;
    assert!(!castable(&game, spell), "your own upkeep is not theirs");

    game.active_player = PlayerId::Two;
    game.step = Step::PrecombatMain;
    assert!(!castable(&game, spell), "their turn, but past the upkeep");

    game.step = Step::Upkeep;
    assert!(castable(&game, spell), "their turn and their upkeep");
}

#[test]
fn the_festival_stops_the_attack() {
    let (mut game, theirs, spell) = holding(cards::FESTIVAL);
    game.active_player = PlayerId::Two;
    game.step = Step::Upkeep;
    cast_it(&mut game, spell);

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    assert!(
        !game.legal_actions(PlayerId::Two).iter().any(|action| {
            matches!(action, Action::DeclareAttacker { attacker, .. } if *attacker == theirs)
        }),
        "nothing can be declared as an attacker this turn",
    );
}

#[test]
fn the_teleport_waits_for_the_declare_attackers_step() {
    let (mut game, _theirs, spell) = holding(cards::TELEPORT);
    game.active_player = PlayerId::One;

    game.step = Step::PrecombatMain;
    assert!(!castable(&game, spell));

    game.step = Step::BeginningOfCombat;
    assert!(!castable(&game, spell), "combat, but not the step it names");

    // Past the declaration, where priority is actually offered: the step is
    // still declare-attackers, which is what the window names.
    game.step = Step::DeclareAttackers;
    game.attackers_declared = true;
    assert!(castable(&game, spell));
}

#[test]
fn the_teleport_makes_its_target_unblockable() {
    let (mut game, theirs, spell) = holding(cards::TELEPORT);
    game.active_player = PlayerId::Two;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = true;
    cast_it(&mut game, spell);

    // Read through the blocker offer: an unblockable attacker is one no
    // blocker is ever offered against.
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == theirs)
        .expect("still there")
        .attacking = true;
    game.battlefield
        .push(creature(10_500, cards::SERRA_ANGEL, PlayerId::One));
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::DeclareBlocker { attacker, .. } if *attacker == theirs)
        }),
        "nothing may be declared as a blocker against it",
    );
}

#[test]
fn the_reset_waits_for_an_opponents_turn_past_their_upkeep() {
    let (mut game, _theirs, spell) = holding(cards::RESET);

    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    assert!(!castable(&game, spell), "your own turn is not theirs");

    game.active_player = PlayerId::Two;
    game.step = Step::Upkeep;
    assert!(!castable(&game, spell), "their turn, but still the upkeep");

    game.step = Step::PrecombatMain;
    assert!(castable(&game, spell), "their turn and past the upkeep");
}

#[test]
fn the_reset_untaps_only_your_own_lands() {
    let (mut game, _theirs, spell) = holding(cards::RESET);
    let mut mine = creature(10_100, cards::ISLAND, PlayerId::One);
    mine.tapped = true;
    let mine_id = mine.card.id;
    game.battlefield.push(mine);
    let mut theirs = creature(10_101, cards::ISLAND, PlayerId::Two);
    theirs.tapped = true;
    let their_land = theirs.card.id;
    game.battlefield.push(theirs);

    game.active_player = PlayerId::Two;
    game.step = Step::PrecombatMain;
    cast_it(&mut game, spell);

    let tapped = |id: GameObjectId| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("still there")
            .tapped
    };
    assert!(!tapped(mine_id), "yours came back");
    assert!(tapped(their_land), "and theirs did not");
}
