//! Target legality reads the real numbers, not the recursion-safe ones.
//!
//! Trigger capture and static resolution share a characteristics view that
//! deliberately leaves continuous static effects out: it is used *while* those
//! effects are being resolved, so asking for a value that depends on them
//! would re-enter the computation. Target legality is asked from outside that
//! resolution and so gets the real values, which is what these pin.

use super::*;
use crate::ImplementationStatus;

fn legal_targets(game: &Game, source: GameObjectId) -> Vec<GameObjectId> {
    let mut found = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility {
                source: actual,
                targets,
                ..
            } if actual == source => targets
                .iter()
                .flat_map(crate::casting::TargetSelection::targets)
                .find_map(|target| match target {
                    Target::Permanent(id) => Some(*id),
                    _ => None,
                }),
            _ => None,
        })
        .collect::<Vec<_>>();
    found.sort_unstable();
    found
}

/// Pendelhaven targets a 1/1. A Crusade makes every white creature 2/2, so a
/// white 1/1 stops being a legal target while the Crusade is out -- which is
/// the whole point of the clause it prints.
#[test]
fn a_statically_pumped_creature_is_no_longer_a_one_one() {
    let mut game = ready_game();
    let pendelhaven = creature(10_000, cards::PENDELHAVEN, PlayerId::One);
    let pendelhaven_id = pendelhaven.card.id;
    game.battlefield.push(pendelhaven);
    // A white 1/1, so the Crusade below reaches it.
    let javelineers = creature(10_001, cards::ICATIAN_JAVELINEERS, PlayerId::One);
    let javelineers_id = javelineers.card.id;
    game.battlefield.push(javelineers);
    game.turns_started[PlayerId::One.index()] = 1;

    assert_eq!(
        legal_targets(&game, pendelhaven_id),
        vec![javelineers_id],
        "a printed 1/1 is a legal target"
    );

    game.battlefield
        .push(creature(10_002, cards::CRUSADE, PlayerId::One));

    assert!(
        legal_targets(&game, pendelhaven_id).is_empty(),
        "the Crusade made it a 2/2, so it is not a 1/1 any more"
    );
}

/// The same seam in the other direction. "Power 2 or less" is a negation, so
/// reading the smaller number made the clause too permissive: a creature a
/// Crusade had already pushed past the ceiling still qualified.
#[test]
fn a_statically_pumped_creature_leaves_a_power_ceiling() {
    let mut game = ready_game();
    let warriors = creature(10_000, cards::DWARVEN_WARRIORS, PlayerId::One);
    let warriors_id = warriors.card.id;
    game.battlefield.push(warriors);
    game.turns_started[PlayerId::One.index()] = 1;
    let bear = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);

    assert!(
        legal_targets(&game, warriors_id).contains(&bear_id),
        "a 2/1 is within the ceiling"
    );

    // Two Crusades put it at 4/3, well past "power 2 or less".
    game.battlefield
        .push(creature(10_002, cards::CRUSADE, PlayerId::One));
    game.battlefield
        .push(creature(10_003, cards::CRUSADE, PlayerId::One));

    assert!(
        !legal_targets(&game, warriors_id).contains(&bear_id),
        "the statics pushed it past the ceiling"
    );
}

#[test]
fn a_static_toughness_bonus_moves_a_creature_out_of_range() {
    let mut game = ready_game();
    let source = creature(10_000, cards::FLESHPULPER_GIANT, PlayerId::One);
    let source_id = source.card.id;
    game.battlefield.push(source);
    let lions = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    let lions_id = lions.card.id;
    game.battlefield.push(lions);

    let predicate = AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::ToughnessLessThan(ValueDef::Constant(3)),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::Opponent),
        owner: None,
    };
    assert!(
        game.ability_targets_matching(
            predicate,
            PlayerId::One,
            source_id,
            TriggerContext::empty(),
        )
        .contains(&Target::Permanent(lions_id)),
        "one toughness is below three",
    );

    game.battlefield
        .push(creature(10_002, cards::CRUSADE, PlayerId::Two));
    game.battlefield
        .push(creature(10_003, cards::CRUSADE, PlayerId::Two));

    assert!(
        !game
            .ability_targets_matching(predicate, PlayerId::One, source_id, TriggerContext::empty(),)
            .contains(&Target::Permanent(lions_id)),
        "two static bonuses make its toughness three",
    );
}

#[test]
fn a_static_source_power_bonus_widens_a_relative_target_predicate() {
    let mut game = ready_game();
    let source = creature(10_000, cards::ICATIAN_JAVELINEERS, PlayerId::One);
    let source_id = source.card.id;
    game.battlefield.push(source);
    let bears = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    let predicate = AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::ToughnessLessThan(ValueDef::SourcePower),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    };

    assert!(
        !game
            .ability_targets_matching(predicate, PlayerId::One, source_id, TriggerContext::empty(),)
            .contains(&Target::Permanent(bears_id)),
        "two toughness is not below one power",
    );

    game.battlefield
        .push(creature(10_002, cards::CRUSADE, PlayerId::One));
    game.battlefield
        .push(creature(10_003, cards::CRUSADE, PlayerId::One));

    assert!(
        game.ability_targets_matching(
            predicate,
            PlayerId::One,
            source_id,
            TriggerContext::empty(),
        )
        .contains(&Target::Permanent(bears_id)),
        "the statics raise the source to three power",
    );
}

#[test]
fn every_live_stat_target_card_reports_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [
        cards::DWARVEN_WARRIORS,
        cards::STONE_GIANT,
        cards::TAWNOSS_WAND,
        cards::PENDELHAVEN,
        cards::SMITE_THE_MONSTROUS,
        cards::SELESNYA_CHARM,
        cards::SKYMARK_ROC,
        cards::FLESHPULPER_GIANT,
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
