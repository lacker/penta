//! Forth Eorlingas!: an army with haste, and the crown for whoever it gets
//! through to.

use super::*;

/// Player One holding the card, with `mana` of every color available.
fn staged(mana: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let spell = game
        .build_zone(PlayerId::One, &[cards::FORTH_EORLINGAS])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let spell_id = spell.id;
    game.players[0].hand.push(spell);
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    for color in ManaColor::COLORS {
        game.add_unrestricted_mana(PlayerId::One, color, mana);
    }
    (game, spell_id)
}

fn settle(game: &mut Game) {
    for _ in 0..40 {
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

/// Casts it for `x`.
fn cast_for(game: &mut Game, spell: GameObjectId, x: u16) {
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == spell && choices.x() == x)
        })
        .unwrap_or_else(|| panic!("it is castable for {x}"));
    game.apply(PlayerId::One, cast).expect("it is cast");
    settle(game);
}

fn riders(game: &Game) -> Vec<GameObjectId> {
    game.battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == ObjectKind::Token)
        .map(|permanent| permanent.card.id)
        .collect()
}

/// Attacks with every creature Player One controls and carries combat
/// through the damage step.
fn attack_with_everything(game: &mut Game) {
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    let attackers: Vec<_> = game
        .battlefield
        .iter()
        .filter(|permanent| permanent.controller == PlayerId::One)
        .map(|permanent| permanent.card.id)
        .collect();
    for attacker in attackers {
        game.declare_attacker(attacker, AttackDefender::Player(PlayerId::Two));
    }
    game.finish_declaring_attackers();
    settle(game);
    game.step = Step::CombatDamage;
    game.deal_combat_damage();
    settle(game);
}

/// X tokens, each a 2/2 with both keywords.
#[test]
fn it_makes_x_riders_with_trample_and_haste() {
    let (mut game, spell) = staged(5);

    cast_for(&mut game, spell, 3);

    let made = riders(&game);
    assert_eq!(made.len(), 3, "one Rider per point of X");
    for rider in made {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == rider)
            .expect("still there");
        assert_eq!(game.power(permanent), Some(2));
        assert_eq!(game.toughness(permanent), Some(2));
        assert!(game.permanent_has_executable_keyword(permanent, KeywordAbility::Trample));
        assert!(game.permanent_has_executable_keyword(permanent, KeywordAbility::Haste));
        assert_eq!(permanent.controller, PlayerId::One);
    }
}

/// Haste is what makes the card: the Riders can attack the turn they arrive,
/// and connecting hands their controller the crown.
#[test]
fn connecting_takes_the_crown() {
    let (mut game, spell) = staged(4);
    cast_for(&mut game, spell, 2);
    assert_eq!(game.monarch(), None, "nobody is the monarch yet");

    attack_with_everything(&mut game);

    assert_eq!(game.players[1].life, 16, "two Riders got through");
    assert_eq!(game.monarch(), Some(PlayerId::One));
}

/// The trigger is one trigger for the whole step, however many creatures
/// dealt the damage.
#[test]
fn the_whole_damage_step_is_one_trigger() {
    let (mut game, spell) = staged(4);
    cast_for(&mut game, spell, 2);

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    for attacker in riders(&game) {
        game.declare_attacker(attacker, AttackDefender::Player(PlayerId::Two));
    }
    game.finish_declaring_attackers();
    settle(&mut game);
    game.step = Step::CombatDamage;
    game.deal_combat_damage();

    assert_eq!(
        game.pending_triggers.len(),
        1,
        "two Riders connecting is one 'one or more' event",
    );
}

/// A creature that was on the battlefield before the spell resolved claims
/// the crown too: the clause watches every creature you control.
#[test]
fn any_creature_of_yours_claims_it() {
    let (mut game, spell) = staged(4);
    game.put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    cast_for(&mut game, spell, 0);

    assert!(riders(&game).is_empty(), "X of zero makes no Riders");

    attack_with_everything(&mut game);

    assert_eq!(game.players[1].life, 18, "the Lions got through");
    assert_eq!(game.monarch(), Some(PlayerId::One));
}

/// Blocked and dealing no damage to a player is no crown.
#[test]
fn a_blocked_rider_claims_nothing() {
    let (mut game, spell) = staged(3);
    let wall = game
        .put_onto_battlefield(PlayerId::Two, cards::WALL_OF_STONE)
        .expect("cataloged");
    drain_pending(&mut game);
    cast_for(&mut game, spell, 1);
    let rider = riders(&game).into_iter().next().expect("one Rider");

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.declare_attacker(rider, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    settle(&mut game);
    game.declare_blocker(wall, rider);
    game.step = Step::CombatDamage;
    game.deal_combat_damage();
    settle(&mut game);

    assert_eq!(game.players[1].life, 20, "the Wall held");
    assert_eq!(game.monarch(), None);
}

/// The delayed trigger is this turn's: it is gone by the next one.
#[test]
fn the_trigger_lasts_only_for_the_turn() {
    let (mut game, spell) = staged(3);
    cast_for(&mut game, spell, 1);

    assert_eq!(
        game.installed_triggers.len(),
        1,
        "the delayed trigger is waiting",
    );

    game.start_next_turn();

    assert!(
        game.installed_triggers.is_empty(),
        "and the turn it belonged to is over",
    );
}

/// "X 2/2 red Human Knight creature tokens": the half the size and the
/// keywords do not say. Red matters for a Blood Moon deck's own Pyrokinesis
/// and the types for anything that answers Humans.
#[test]
fn the_riders_are_red_human_knights() {
    let (mut game, spell) = staged(3);

    cast_for(&mut game, spell, 1);

    let rider = riders(&game).into_iter().next().expect("one Rider");
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == rider)
        .expect("still there");
    assert_eq!(
        game.effective_colors(permanent, &game.effective_rules(permanent).expect("rules")),
        [false, false, false, true, false],
        "red and nothing else",
    );
    let subtypes = game.effective_subtypes(permanent);
    assert!(
        subtypes.contains(crate::card::Subtype::named("Human")),
        "a Human"
    );
    assert!(
        subtypes.contains(crate::card::Subtype::named("Knight")),
        "and a Knight"
    );
}

/// Trample and the crown together: a Rider held by a 1/1 still sends a point
/// past it, and a point to a player is combat damage to a player.
#[test]
fn a_rider_that_tramples_over_a_blocker_still_claims_the_crown() {
    let (mut game, spell) = staged(3);
    let chump = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    drain_pending(&mut game);
    cast_for(&mut game, spell, 1);
    let rider = riders(&game).into_iter().next().expect("one Rider");

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.declare_attacker(rider, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    settle(&mut game);
    game.declare_blocker(chump, rider);
    game.step = Step::CombatDamage;
    game.deal_combat_damage();
    settle(&mut game);

    assert_eq!(game.players[1].life, 19, "one point trampled over");
    assert_eq!(
        game.monarch(),
        Some(PlayerId::One),
        "and that point is combat damage to a player",
    );
}

/// "Deal combat damage to one or more players." A planeswalker is not a
/// player, so a Rider that spends the combat killing one takes no crown.
#[test]
fn damage_to_a_planeswalker_is_not_damage_to_a_player() {
    let (mut game, spell) = staged(3);
    let walker = game
        .put_onto_battlefield(PlayerId::Two, cards::JACE_THE_MIND_SCULPTOR)
        .expect("cataloged");
    drain_pending(&mut game);
    cast_for(&mut game, spell, 1);
    let rider = riders(&game).into_iter().next().expect("one Rider");

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.declare_attacker(rider, AttackDefender::Planeswalker(walker));
    game.finish_declaring_attackers();
    settle(&mut game);
    game.step = Step::CombatDamage;
    game.deal_combat_damage();
    settle(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == walker)
            .expect("he survived")
            .counters(CounterKind::Loyalty),
        1,
        "the Rider did connect, taking two of his three loyalty",
    );
    assert_eq!(game.players[1].life, 20, "the player took nothing");
    assert_eq!(game.monarch(), None, "and the crown stayed where it was");
}

/// "As a player becomes the monarch, the current monarch ceases being the
/// monarch." The card is usually cast into a crown somebody else is wearing,
/// and connecting takes it off them -- twice over, since a creature dealing
/// combat damage to the monarch does the same thing on its own. The crown is
/// still one crown, and it ends up on one head.
#[test]
fn it_takes_the_crown_off_the_player_wearing_it() {
    let (mut game, spell) = staged(3);
    game.set_monarch(PlayerId::Two);
    drain_pending(&mut game);
    assert_eq!(game.monarch(), Some(PlayerId::Two), "they start wearing it");

    cast_for(&mut game, spell, 1);
    attack_with_everything(&mut game);

    assert_eq!(game.players[1].life, 18, "one Rider got through");
    assert_eq!(
        game.monarch(),
        Some(PlayerId::One),
        "and the crown changed heads",
    );
}
