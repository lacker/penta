//! Prevention offered as one mode among others.
//!
//! Nothing new in the engine: each of these needed a mode, a target, or a
//! delayed self-return that already worked. What these check is that the
//! shield really lands on either kind of target a "any target" slot admits,
//! and that choosing the other mode does something different.

use super::*;

/// Casts `definition`, choosing the mode whose text contains `mode`, at
/// `target`.
fn cast_mode(game: &mut Game, definition: CardDefinitionId, mode: &str, target: Target, x: u16) {
    let spell = card(10_000, definition, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.white = 2;
    pool.colorless = 4;

    let catalog = poc::catalog().expect("catalog builds");
    let card = catalog.get(definition).expect("cataloged");
    let modes = card
        .rules
        .ability_clauses()
        .iter()
        .find_map(|ability| match ability.definition {
            crate::card::DeclarativeAbilityDef::Spell(spell) => spell.modal(),
            _ => None,
        })
        .expect("the card is modal");
    let index = modes
        .modes
        .iter()
        .position(|candidate| candidate.text.contains(mode))
        .expect("the named mode exists");

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == spell_id
                    && choices.x() == x
                    && choices
                        .modes()
                        .iter()
                        .any(|selected| selected.index() == index)
                    && choices.iter_targets().any(|chosen| *chosen == target)
            }
            _ => false,
        })
        .expect("that mode and target are offered");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    drain_pending(game);
}

#[test]
fn the_prevention_mode_shields_a_creature() {
    let mut game = ready_game();
    let troll = creature(10_001, cards::SEDGE_TROLL, PlayerId::One);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);

    cast_mode(
        &mut game,
        cards::HEALING_SALVE,
        "Prevent",
        Target::Permanent(troll_id),
        0,
    );

    game.damage_target_from(None, Some(Target::Permanent(troll_id)), 3);
    drain_pending(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == troll_id)
            .expect("still alive")
            .damage,
        0,
        "all three points were prevented"
    );
}

/// "Any target" admits a player as readily as a creature.
#[test]
fn the_prevention_mode_shields_a_player() {
    let mut game = ready_game();
    let before = game.players[PlayerId::One.index()].life;

    cast_mode(
        &mut game,
        cards::HEALING_SALVE,
        "Prevent",
        Target::Player(PlayerId::One),
        0,
    );

    game.damage_target_from(None, Some(Target::Player(PlayerId::One)), 5);
    drain_pending(&mut game);
    assert_eq!(
        game.players[PlayerId::One.index()].life,
        before - 2,
        "three of the five points were prevented"
    );
}

/// The other mode is life, not prevention, which is what makes the choice a
/// choice.
#[test]
fn the_life_mode_gains_life_instead() {
    let mut game = ready_game();
    let before = game.players[PlayerId::One.index()].life;

    cast_mode(
        &mut game,
        cards::HEALING_SALVE,
        "gains 3 life",
        Target::Player(PlayerId::One),
        0,
    );

    assert_eq!(game.players[PlayerId::One.index()].life, before + 3);
}

/// Alabaster Potion's amount is the X paid for it, on either mode.
#[test]
fn alabaster_potion_reads_its_chosen_x() {
    let mut game = ready_game();
    let before = game.players[PlayerId::One.index()].life;

    cast_mode(
        &mut game,
        cards::ALABASTER_POTION,
        "gains X life",
        Target::Player(PlayerId::One),
        2,
    );

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        before + 2,
        "X was two, so two life"
    );
}
