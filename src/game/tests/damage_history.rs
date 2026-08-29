//! "Damage dealt to you this turn."
//!
//! The running total is accumulated as the damage lands rather than derived
//! from life totals, which is what lets it survive life gain in between and
//! lets a source group be recorded at the only moment it is answerable --
//! "by artifacts" is read while the artifact is still the source.

use super::*;

fn cast(game: &mut Game, definition: CardDefinitionId, target: Option<GameObjectId>) {
    let spell = card(10_900, definition, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.white = 2;
    pool.black = 1;
    pool.colorless = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == spell_id
                    && target.is_none_or(|target| {
                        choices
                            .iter_targets()
                            .any(|chosen| *chosen == Target::Permanent(target))
                    })
            }
            _ => false,
        })
        .expect("the spell can be cast");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    drain_pending(game);
}

#[test]
fn simulacrum_pays_back_what_the_turn_took() {
    let mut game = ready_game();
    let troll = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);
    let burner = creature(10_001, cards::SOL_RING, PlayerId::Two);
    let burner_id = burner.card.id;
    game.battlefield.push(burner);

    game.damage_target_from(Some(burner_id), Some(Target::Player(PlayerId::One)), 3);
    let life = game.players[PlayerId::One.index()].life;

    cast(&mut game, cards::SIMULACRUM, Some(troll_id));

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        life + 3,
        "three back for the three taken"
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == troll_id),
        "and the same three killed the 2/2 it was pointed at"
    );
}

/// The total is not derived from life, so gaining life in between does not
/// erase what the turn took.
#[test]
fn gaining_life_does_not_reset_the_total() {
    let mut game = ready_game();
    let burner = creature(10_001, cards::SOL_RING, PlayerId::Two);
    let burner_id = burner.card.id;
    game.battlefield.push(burner);
    let troll = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);

    game.damage_target_from(Some(burner_id), Some(Target::Player(PlayerId::One)), 2);
    game.players[PlayerId::One.index()].life += 10;
    let life = game.players[PlayerId::One.index()].life;

    cast(&mut game, cards::SIMULACRUM, Some(troll_id));

    assert_eq!(game.players[PlayerId::One.index()].life, life + 2);
}

/// A new turn starts the count over.
#[test]
fn the_total_resets_with_the_turn() {
    let mut game = ready_game();
    let burner = creature(10_001, cards::SOL_RING, PlayerId::Two);
    let burner_id = burner.card.id;
    game.battlefield.push(burner);
    let troll = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);

    game.damage_target_from(Some(burner_id), Some(Target::Player(PlayerId::One)), 4);
    game.commit_next_turn(PlayerId::Two, Vec::new());
    drain_pending(&mut game);
    // An instant on the other player's turn still needs its caster to hold
    // priority.
    game.priority = PlayerId::One;
    let life = game.players[PlayerId::One.index()].life;

    cast(&mut game, cards::SIMULACRUM, Some(troll_id));

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        life,
        "last turn's damage is not this turn's"
    );
}

/// Reverse Polarity counts only what artifacts dealt, and counts it twice.
#[test]
fn reverse_polarity_doubles_only_the_artifact_damage() {
    let mut game = ready_game();
    let ring = creature(10_001, cards::SOL_RING, PlayerId::Two);
    let ring_id = ring.card.id;
    game.battlefield.push(ring);
    let troll = creature(10_002, cards::SEDGE_TROLL, PlayerId::Two);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);

    game.damage_target_from(Some(ring_id), Some(Target::Player(PlayerId::One)), 3);
    game.damage_target_from(Some(troll_id), Some(Target::Player(PlayerId::One)), 5);
    let life = game.players[PlayerId::One.index()].life;

    cast(&mut game, cards::REVERSE_POLARITY, None);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        life + 6,
        "twice the artifact's three, and nothing for the creature's five"
    );
}
