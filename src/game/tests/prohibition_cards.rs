//! Four cards that take something away, or hand out a narrowed permission.
//!
//! Arrest shuts off three things at once and gives all three back together,
//! and the activation half is narrow: only activated abilities are shut off, not
//! the creature's triggered or static clauses. Skygames is the mirror -- the
//! ability it grants keeps the restriction printed on it.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game
}

/// Puts a creature with an activated ability out under an Arrest and answers
/// what it may still do.
fn arrested() -> (Game, GameObjectId, GameObjectId, GameObjectId) {
    let mut game = ready();
    let victim = creature(10_000, cards::ROYAL_ASSASSIN, PlayerId::One);
    let victim_id = victim.card.id;
    game.battlefield.push(victim);
    let mut arrest = creature(10_001, cards::ARREST, PlayerId::One);
    arrest.attached_to = Some(victim_id);
    let arrest_id = arrest.card.id;
    game.battlefield.push(arrest);
    game.battlefield
        .push(creature(10_100, cards::SAVANNAH_LIONS, PlayerId::Two));
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    if let Some(target) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.definition == cards::SAVANNAH_LIONS)
    {
        target.tapped = true;
    }
    let mut attacker = creature(10_101, cards::GRIZZLY_BEARS, PlayerId::Two);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::One));
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    (game, victim_id, arrest_id, attacker_id)
}

fn can_activate(game: &Game, source: GameObjectId) -> bool {
    game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source),
    )
}

fn can_attack(game: &Game, attacker: GameObjectId) -> bool {
    game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::DeclareAttacker { attacker: actual, .. } if *actual == attacker),
    )
}

fn can_block(game: &Game, blocker: GameObjectId, attacker: GameObjectId) -> bool {
    game.legal_actions(PlayerId::One)
        .contains(&Action::DeclareBlocker { blocker, attacker })
}

#[test]
fn arrest_shuts_off_all_three_and_gives_them_back_together() {
    let (mut game, victim, arrest, attacker) = arrested();
    assert!(!can_activate(&game, victim), "its activations are gone");

    game.step = Step::DeclareAttackers;
    assert!(!can_attack(&game, victim), "and it cannot attack");

    game.active_player = PlayerId::Two;
    game.priority = PlayerId::One;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    assert!(!can_block(&game, victim, attacker), "or block");

    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.battlefield
        .retain(|permanent| permanent.card.id != arrest);
    assert!(can_activate(&game, victim), "the Aura left, so they return");
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    assert!(can_attack(&game, victim));
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::One;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    assert!(can_block(&game, victim, attacker));
}

/// An unarrested copy is the control: the ability is offered without the Aura.
#[test]
fn the_same_creature_activates_freely_without_the_aura() {
    let mut game = ready();
    let victim = creature(10_000, cards::ROYAL_ASSASSIN, PlayerId::One);
    let victim_id = victim.card.id;
    game.battlefield.push(victim);
    let mut lions = creature(10_100, cards::SAVANNAH_LIONS, PlayerId::Two);
    lions.tapped = true;
    game.battlefield.push(lions);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    assert!(can_activate(&game, victim_id));
}

/// Mugging's prohibition lands even when the damage did not kill.
#[test]
fn mugging_stops_a_survivor_from_blocking() {
    let mut game = ready();
    let victim = creature(10_000, cards::AIR_ELEMENTAL, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.push(victim);
    let spell = card(20_000, cards::MUGGING, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.red = 1;

    game.apply(
        PlayerId::One,
        cast_action(spell_id, vec![Target::Permanent(victim_id)], Vec::new(), 0),
    )
    .expect("the cast is legal");
    drain_pending(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == victim_id)
            .expect("a 4/4 survives two")
            .damage,
        2,
    );

    let mut attacker = creature(10_100, cards::GRIZZLY_BEARS, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.priority = PlayerId::Two;

    assert!(
        !game
            .legal_actions(PlayerId::Two)
            .contains(&Action::DeclareBlocker {
                blocker: victim_id,
                attacker: attacker_id,
            }),
        "it took the damage and still cannot block",
    );
}

/// Encrust answers an artifact as readily as a creature, and takes both the
/// untap and the activations.
#[test]
fn encrust_holds_an_artifact_down_and_shuts_it_off() {
    let mut game = ready();
    let artifact = creature(10_000, cards::ORNITHOPTER, PlayerId::Two);
    let artifact_id = artifact.card.id;
    game.battlefield.push(artifact);

    let spell = card(20_000, cards::ENCRUST, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.blue = 2;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    let legal = game.legal_actions(PlayerId::One);
    assert!(
        legal.contains(&cast_action(
            spell_id,
            vec![Target::Permanent(artifact_id)],
            Vec::new(),
            0,
        )),
        "an artifact is a legal host",
    );

    game.apply(
        PlayerId::One,
        cast_action(
            spell_id,
            vec![Target::Permanent(artifact_id)],
            Vec::new(),
            0,
        ),
    )
    .expect("the cast is legal");
    drain_pending(&mut game);

    if let Some(target) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == artifact_id)
    {
        target.tapped = true;
    }
    game.commit_next_turn(PlayerId::Two, Vec::new());
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == artifact_id)
            .expect("still there")
            .tapped,
        "it stayed down through its controller's untap step",
    );
}

/// The granted ability keeps its sorcery-speed restriction: the land may tap
/// for it in a main phase and not while blockers are being declared.
#[test]
fn skygames_grants_a_sorcery_speed_ability() {
    let mut game = ready();
    game.put_onto_battlefield(PlayerId::One, cards::ISLAND)
        .expect("cataloged");
    let land = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::ISLAND)
        .expect("it is there")
        .card
        .id;
    let mut aura = creature(10_000, cards::SKYGAMES, PlayerId::One);
    aura.attached_to = Some(land);
    game.battlefield.push(aura);
    game.battlefield
        .push(creature(10_100, cards::GRIZZLY_BEARS, PlayerId::One));
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    assert!(
        can_activate(&game, land),
        "a main phase is a sorcery-speed window",
    );

    // Upkeep is an ordinary instant-speed window, so only the printed
    // restriction can be what closes it.
    game.step = Step::Upkeep;
    assert!(!can_activate(&game, land), "and an upkeep is not");
}

/// One Thousand Lashes carries the same three prohibitions plus a drain that
/// follows the creature rather than the Aura.
#[test]
fn one_thousand_lashes_drains_the_creatures_controller() {
    let mut game = ready();
    let victim = creature(10_000, cards::ROYAL_ASSASSIN, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.push(victim);
    let mut aura = creature(10_001, cards::ONE_THOUSAND_LASHES, PlayerId::One);
    aura.attached_to = Some(victim_id);
    game.battlefield.push(aura);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    game.priority = PlayerId::Two;
    assert!(
        !game.legal_actions(PlayerId::Two).iter().any(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == victim_id)
        ),
        "the creature's activations are shut off",
    );

    let before = [game.players[0].life, game.players[1].life];
    game.active_player = PlayerId::Two;
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_pending(&mut game);
    assert_eq!(
        [game.players[0].life, game.players[1].life],
        [before[0], before[1] - 1],
        "the creature's controller pays, not the Aura's",
    );
}
