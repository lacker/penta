//! Artifacts whose value depends on something the catalog cannot check: a
//! mana ability the planner has to reach on its own, a cost reduction that
//! is written for every player rather than its controller, and an
//! activation cost paid by tapping other permanents.

use super::*;

fn can_cast(game: &Game, player: PlayerId, spell: CardInstanceId) -> bool {
    game.legal_actions(player)
        .into_iter()
        .any(|action| matches!(action, Action::CastSpell { card, .. } if card == spell))
}

/// `artifacts` and `creatures` under player one, with `held` in their hand
/// and no mana anywhere.
fn staged(
    artifacts: &[CardDefinitionId],
    creatures: &[CardDefinitionId],
    held: CardDefinitionId,
) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].mana_pool = ManaPool::default();
    game.turns_started[PlayerId::One.index()] = 5;
    for (index, definition) in artifacts.iter().chain(creatures).enumerate() {
        let mut permanent = creature(
            68_000 + u32::try_from(index).expect("a small fixture"),
            *definition,
            PlayerId::One,
        );
        permanent.entered_controller_turn = 0;
        game.battlefield.push(permanent);
    }
    let spell = card(68_100, held, PlayerId::One);
    let spell_id = spell.id;
    game.players[0].hand.push(spell);
    (game, spell_id)
}

#[test]
fn the_altar_is_reached_by_the_planner() {
    let (game, spell) = staged(&[], &[cards::GRIZZLY_BEARS], cards::UNSUMMON);
    assert!(
        !can_cast(&game, PlayerId::One, spell),
        "a Bears alone makes no mana"
    );

    let (game, spell) = staged(
        &[cards::PHYREXIAN_ALTAR],
        &[cards::GRIZZLY_BEARS],
        cards::UNSUMMON,
    );
    assert!(
        can_cast(&game, PlayerId::One, spell),
        "the Altar turns the Bears into the blue mana for Unsummon"
    );
}

#[test]
fn the_altar_actually_eats_the_creature() {
    let (mut game, spell) = staged(
        &[cards::PHYREXIAN_ALTAR],
        &[cards::GRIZZLY_BEARS],
        cards::UNSUMMON,
    );
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("the Altar covers {U}");
    game.apply(PlayerId::One, cast).expect("the cast is legal");

    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == ObjectKind::Card(cards::GRIZZLY_BEARS)),
        "the Bears paid for it"
    );
}

/// Player two holding Words of Wisdom with one blue mana and priority.
fn opposing_instant(artifacts: &[CardDefinitionId]) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    game.players[0].mana_pool = ManaPool::default();
    game.players[1].mana_pool = ManaPool::default();
    for (index, definition) in artifacts.iter().enumerate() {
        let mut permanent = creature(
            68_200 + u32::try_from(index).expect("a small fixture"),
            *definition,
            PlayerId::One,
        );
        permanent.entered_controller_turn = 0;
        game.battlefield.push(permanent);
    }
    let spell = card(68_300, cards::WORDS_OF_WISDOM, PlayerId::Two);
    let spell_id = spell.id;
    game.players[1].hand.push(spell);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 1);
    game.priority = PlayerId::Two;
    (game, spell_id)
}

#[test]
fn the_helm_makes_the_opponents_spells_cheaper_too() {
    let (game, spell) = opposing_instant(&[]);
    assert!(
        !can_cast(&game, PlayerId::Two, spell),
        "one blue does not pay for {{1}}{{U}}"
    );

    let (game, spell) = opposing_instant(&[cards::HELM_OF_AWAKENING]);
    assert!(
        can_cast(&game, PlayerId::Two, spell),
        "but the Helm is not written for its controller alone"
    );
}

/// The Clock plus `others` more artifacts, and one tapped artifact to aim at.
fn clockwork(others: usize) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut clock = creature(68_400, cards::CLOCK_OF_OMENS, PlayerId::One);
    clock.entered_controller_turn = 0;
    game.battlefield.push(clock);
    for index in 0..others {
        let mut spare = creature(
            68_410 + u32::try_from(index).expect("a small fixture"),
            cards::SQUEE_S_TOY,
            PlayerId::One,
        );
        spare.entered_controller_turn = 0;
        game.battlefield.push(spare);
    }
    let mut target = creature(68_500, cards::EMMESSI_TOME, PlayerId::One);
    target.entered_controller_turn = 0;
    target.tapped = true;
    let target_id = target.card.id;
    game.battlefield.push(target);
    (game, target_id)
}

fn clock_activation(game: &Game, target: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == GameObjectId(68_400)
                    && targets
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|chosen| *chosen == Target::Permanent(target))
            }
            _ => false,
        })
}

#[test]
fn the_clock_needs_a_second_untapped_artifact() {
    let (game, target) = clockwork(0);
    assert!(
        clock_activation(&game, target).is_none(),
        "the Clock alone is one untapped artifact, not two"
    );
}

#[test]
fn the_clock_taps_two_to_untap_one() {
    let (mut game, target) = clockwork(1);
    let activation =
        clock_activation(&game, target).expect("two untapped artifacts pay for the untap");
    game.apply(PlayerId::One, activation)
        .expect("the cost is payable");
    for _ in 0..12 {
        drain_pending(&mut game);
        if game.stack.is_empty() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }

    let tapped = game
        .battlefield
        .iter()
        .filter(|permanent| permanent.tapped)
        .count();
    assert_eq!(tapped, 2, "two artifacts paid, and the Tome came untapped");
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == target && !permanent.tapped),
        "the Tome is the one that untapped"
    );
}
