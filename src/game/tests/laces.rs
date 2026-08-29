//! The Lace cycle.
//!
//! "Becomes" replaces a colour rather than adding one, the change has no
//! duration, and the target can be a spell still on the stack. What these
//! check is that the new colour is the one every other rule then reads.

use super::*;

/// Casts a Lace at `target`, with the mana it needs already in the pool.
fn cast_lace(game: &mut Game, lace: CardDefinitionId, color: ManaColor, target: Target) {
    let card = card(11_000, lace, PlayerId::One);
    let card_id = card.id;
    game.players[PlayerId::One.index()].hand.push(card);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    match color {
        ManaColor::White => pool.white += 1,
        ManaColor::Blue => pool.blue += 1,
        ManaColor::Black => pool.black += 1,
        ManaColor::Red => pool.red += 1,
        ManaColor::Green => pool.green += 1,
        ManaColor::Colorless => pool.colorless += 1,
    }

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == card_id && choices.iter_targets().any(|chosen| *chosen == target)
            }
            _ => false,
        })
        .expect("the Lace can target that");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    // Resolves the Lace alone, so a target still on the stack under it is
    // still there to be looked at.
    pass_priority_pair(game);
}

fn colors_of(game: &Game, id: GameObjectId) -> [bool; 5] {
    game.object_colors(id)
}

/// White, blue, black, red, green.
const BLUE: [bool; 5] = [false, true, false, false, false];
const BLACK: [bool; 5] = [false, false, true, false, false];
const RED: [bool; 5] = [false, false, false, true, false];

#[test]
fn a_lace_replaces_a_permanents_color_rather_than_adding_to_it() {
    let mut game = ready_game();
    let troll = creature(10_000, cards::SEDGE_TROLL, PlayerId::Two);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);
    assert_eq!(
        colors_of(&game, troll_id),
        RED,
        "Sedge Troll is printed red"
    );

    cast_lace(
        &mut game,
        cards::THOUGHTLACE,
        ManaColor::Blue,
        Target::Permanent(troll_id),
    );

    assert_eq!(
        colors_of(&game, troll_id),
        BLUE,
        "it is blue and no longer red, because becoming a colour replaces it"
    );
}

/// The colour is what protection reads, so repainting an Aura can make it
/// something its host is protected from. Black Ward is the control: it grants
/// the protection and prints the exception, so painting *it* black changes
/// nothing.
#[test]
fn the_new_color_is_the_one_protection_reads() {
    let mut game = ready_game();
    let host = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    let host_id = host.card.id;
    game.battlefield.push(host);
    let mut ward = creature(10_001, cards::BLACK_WARD, PlayerId::One);
    ward.attached_to = Some(host_id);
    let ward_id = ward.card.id;
    game.battlefield.push(ward);
    let mut blessing = creature(10_002, cards::BLESSING, PlayerId::One);
    blessing.attached_to = Some(host_id);
    let blessing_id = blessing.card.id;
    game.battlefield.push(blessing);
    game.check_state_based_actions();
    assert!(
        [ward_id, blessing_id]
            .into_iter()
            .all(|id| game.battlefield.iter().any(|p| p.card.id == id)),
        "two white Auras sit happily on a creature protected from black"
    );

    cast_lace(
        &mut game,
        cards::DEATHLACE,
        ManaColor::Black,
        Target::Permanent(blessing_id),
    );
    game.check_state_based_actions();

    assert!(
        !game.battlefield.iter().any(|p| p.card.id == blessing_id),
        "a black Aura cannot stay on a creature protected from black"
    );
    assert!(
        game.battlefield.iter().any(|p| p.card.id == ward_id),
        "and the Ward is still there, because its exception is its own"
    );
}

/// The other half of "spell or permanent": a Lace can be cast at something
/// still on the stack.
#[test]
fn a_lace_can_repaint_a_spell_on_the_stack() {
    let mut game = ready_game();
    let bolt = card(10_000, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[PlayerId::One.index()].hand.push(bolt);
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == bolt_id))
        .expect("the Bolt is castable");
    game.apply(PlayerId::One, cast).expect("the spell is cast");
    let spell_id = game
        .stack
        .iter()
        .next()
        .expect("the Bolt is on the stack")
        .id;

    cast_lace(
        &mut game,
        cards::DEATHLACE,
        ManaColor::Black,
        Target::Spell(spell_id),
    );

    assert_eq!(
        colors_of(&game, spell_id),
        BLACK,
        "a red spell that is now a black one"
    );
}
