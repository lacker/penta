//! "This creature can't attack unless defending player controls an Island."
//!
//! A static restriction read while attackers are declared. The query carries
//! its own controller relation, so the clause is an ordinary opponent-relative
//! battlefield query rather than a card-specific rule.

use super::*;

fn island_attacker(defender_land: Option<CardDefinitionId>) -> (Game, GameObjectId) {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::SEA_SERPENT, PlayerId::One);
    // Not summoning sick, so only the restriction is in question.
    attacker.entered_controller_turn = 0;
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    // The controller keeps an Island so the sacrifice trigger stays quiet.
    game.battlefield
        .push(creature(10_001, cards::ISLAND, PlayerId::One));
    if let Some(land) = defender_land {
        game.battlefield.push(creature(10_002, land, PlayerId::Two));
    }
    game.turns_started[PlayerId::One.index()] = 1;
    (game, attacker_id)
}

fn can_attack(game: &Game, attacker: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == attacker)
        .is_some_and(|permanent| game.can_attack(permanent))
}

#[test]
fn the_creature_cannot_attack_without_the_named_land() {
    let (game, attacker) = island_attacker(None);
    assert!(!can_attack(&game, attacker), "no land, no attack");

    let (game, attacker) = island_attacker(Some(cards::MOUNTAIN));
    assert!(
        !can_attack(&game, attacker),
        "a different land does not satisfy the clause"
    );
}

#[test]
fn the_creature_attacks_once_the_defender_controls_the_land() {
    let (game, attacker) = island_attacker(Some(cards::ISLAND));
    assert!(can_attack(&game, attacker));
}

/// The clause reads effective land types, so a dual counts.
#[test]
fn the_restriction_reads_effective_land_types() {
    let (game, attacker) = island_attacker(Some(cards::TROPICAL_ISLAND));
    assert!(
        can_attack(&game, attacker),
        "Tropical Island is an Island for this clause"
    );
}

/// It is the defending player's board that matters, not the attacker's.
#[test]
fn the_attackers_own_island_does_not_satisfy_the_clause() {
    let (game, attacker) = island_attacker(None);
    assert!(
        !can_attack(&game, attacker),
        "the controller's own Island is not the defending player's"
    );
}

#[test]
fn losing_the_restriction_ability_removes_the_attack_prohibition() {
    let (mut game, attacker) = island_attacker(None);
    assert!(
        !can_attack(&game, attacker),
        "the printed restriction applies"
    );

    attach_constant_resolved_characteristics(
        &mut game,
        attacker,
        &[AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any)],
        ContinuousEffectExpiration::Never,
    );

    assert!(
        can_attack(&game, attacker),
        "a resolved loses-all-abilities effect removes the static restriction"
    );
}

/// The tapped predicate is read at target-legality time, so an untapped
/// creature is not merely a bad choice: it is not offered at all.
#[test]
fn a_tapped_only_target_is_offered_only_for_tapped_creatures() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::ROYAL_ASSASSIN, PlayerId::One));
    let mut victim = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    victim.tapped = false;
    let victim_id = victim.card.id;
    game.battlefield.push(victim);
    game.turns_started[PlayerId::One.index()] = 1;

    let targets_victim = |game: &Game| {
        game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::ActivateAbility { targets, .. }
                if targets.iter().flat_map(crate::TargetSelection::targets)
                    .any(|target| *target == Target::Permanent(victim_id)))
        })
    };
    assert!(
        !targets_victim(&game),
        "an untapped creature is not a target"
    );

    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == victim_id)
    {
        permanent.tapped = true;
    }
    assert!(targets_victim(&game), "a tapped creature is");
}
