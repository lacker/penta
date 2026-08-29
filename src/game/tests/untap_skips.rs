//! Skipped untap steps.
//!
//! "Doesn't untap during its controller's next untap step" is spent as those
//! steps arrive, unlike the continuous prohibition an Aura supplies. What
//! these check is that it is the *controller's* step that spends it, that two
//! of them really means two, and that the creature untaps normally after.

use super::*;

fn tapped_creature(game: &mut Game, id: u32, controller: PlayerId) -> GameObjectId {
    let mut permanent = creature(id, cards::SEDGE_TROLL, controller);
    permanent.tapped = true;
    let permanent_id = permanent.card.id;
    game.battlefield.push(permanent);
    permanent_id
}

fn is_tapped(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still on the battlefield")
        .tapped
}

/// Runs one whole turn for `player`, which is what spends a skip.
fn take_turn(game: &mut Game, player: PlayerId) {
    game.commit_next_turn(player, Vec::new());
    drain_pending(game);
}

#[test]
fn barls_cage_makes_a_creature_sit_out_one_untap_step() {
    let mut game = ready_game();
    let cage = creature(10_000, cards::BARLS_CAGE, PlayerId::One);
    let cage_id = cage.card.id;
    game.battlefield.push(cage);
    let victim = tapped_creature(&mut game, 10_001, PlayerId::Two);
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == cage_id
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(victim))
            }
            _ => false,
        })
        .expect("the Cage can name that creature");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    take_turn(&mut game, PlayerId::Two);
    assert!(
        is_tapped(&game, victim),
        "its controller's untap step came and went"
    );

    take_turn(&mut game, PlayerId::Two);
    assert!(
        !is_tapped(&game, victim),
        "and the one after that untaps it normally"
    );
}

/// The count is spent by the controller's own untap step, so an intervening
/// turn taken by anyone else does not use it up.
#[test]
fn an_opponents_untap_step_does_not_spend_the_skip() {
    let mut game = ready_game();
    let victim = tapped_creature(&mut game, 10_000, PlayerId::Two);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == victim)
        .expect("just pushed")
        .skipped_untap_steps = 1;

    take_turn(&mut game, PlayerId::One);
    assert!(
        is_tapped(&game, victim),
        "player one untapping does nothing for a creature player two controls"
    );

    take_turn(&mut game, PlayerId::Two);
    assert!(
        is_tapped(&game, victim),
        "player two's step spends the skip"
    );

    take_turn(&mut game, PlayerId::Two);
    assert!(!is_tapped(&game, victim), "and the next one untaps it");
}

/// Telekinesis names two steps, which is the reason this is a count rather
/// than a flag.
#[test]
fn telekinesis_costs_two_untap_steps() {
    let mut game = ready_game();
    let mut victim = creature(10_000, cards::SEDGE_TROLL, PlayerId::Two);
    victim.tapped = false;
    let victim_id = victim.card.id;
    game.battlefield.push(victim);

    let spell = card(10_001, cards::TELEKINESIS, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.blue = 2;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == spell_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Permanent(victim_id))
            }
            _ => false,
        })
        .expect("Telekinesis can name that creature");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    drain_pending(&mut game);

    assert!(
        is_tapped(&game, victim_id),
        "the spell taps it as it resolves"
    );

    for step in 1..=2 {
        take_turn(&mut game, PlayerId::Two);
        assert!(
            is_tapped(&game, victim_id),
            "untap step {step} of the two it owes"
        );
    }

    take_turn(&mut game, PlayerId::Two);
    assert!(!is_tapped(&game, victim_id), "the third one untaps it");
}

/// Elvish Hunter spends its own tap to take one untap step away, which is
/// the same skip Barl's Cage supplies from an artifact.
#[test]
fn elvish_hunter_takes_one_untap_step() {
    let mut game = ready_game();
    let hunter = creature(10_000, cards::ELVISH_HUNTER, PlayerId::One);
    let hunter_id = hunter.card.id;
    game.battlefield.push(hunter);
    let victim = tapped_creature(&mut game, 10_001, PlayerId::Two);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.green = 1;
    pool.colorless = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == hunter_id
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(victim))
            }
            _ => false,
        })
        .expect("the Hunter can name that creature");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    take_turn(&mut game, PlayerId::Two);
    assert!(is_tapped(&game, victim), "the skip was spent on this step");

    take_turn(&mut game, PlayerId::One);
    take_turn(&mut game, PlayerId::Two);
    assert!(!is_tapped(&game, victim), "and only on that one");
}

/// Giant Tortoise is the other half of the same idea, read continuously:
/// the bonus is on the creature only while it is untapped.
#[test]
fn giant_tortoise_shrinks_when_it_taps() {
    let mut game = ready_game();
    let tortoise = creature(10_000, cards::GIANT_TORTOISE, PlayerId::One);
    let tortoise_id = tortoise.card.id;
    game.battlefield.push(tortoise);

    let stats = |game: &Game| {
        let tortoise = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == tortoise_id)
            .expect("still there");
        (game.power(tortoise), game.toughness(tortoise))
    };
    assert_eq!(stats(&game), (Some(1), Some(4)), "untapped");

    let _ = game.tap_permanent(tortoise_id);
    assert_eq!(stats(&game), (Some(1), Some(1)), "tapped");
}
