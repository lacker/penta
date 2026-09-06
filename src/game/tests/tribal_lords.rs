//! Lords that say "all", not "you control". The older Slivers hand their
//! keyword to the opponent's Slivers as readily as to their controller's,
//! which is the drawback the cycle was priced on -- and the easiest thing to
//! lose by writing `PlayerRelation::You` out of habit. The tribe itself is the
//! other half: a creature that is not a Sliver gets nothing.

use super::*;

/// The lord under player one, with a Sliver and a non-Sliver under player
/// two.
fn staged(lord: CardDefinitionId) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut source = creature(75_000, lord, PlayerId::One);
    source.entered_controller_turn = 0;
    game.battlefield.push(source);
    let mut theirs = creature(75_001, cards::METALLIC_SLIVER, PlayerId::Two);
    theirs.entered_controller_turn = 0;
    game.battlefield.push(theirs);
    let mut bystander = creature(75_002, cards::GRIZZLY_BEARS, PlayerId::Two);
    bystander.entered_controller_turn = 0;
    game.battlefield.push(bystander);
    game
}

fn permanent(game: &Game, id: u32) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(id))
        .expect("the permanent is on the battlefield")
}

#[test]
fn a_sliver_lord_reaches_the_opponents_slivers() {
    let game = staged(cards::WINGED_SLIVER);
    assert!(
        game.has_flying(permanent(&game, 75_001)),
        "the opponent's Sliver gets the keyword too"
    );
    assert!(
        game.has_flying(permanent(&game, 75_000)),
        "and so does the lord itself"
    );
}

#[test]
fn it_reaches_nothing_outside_the_tribe() {
    let game = staged(cards::WINGED_SLIVER);
    assert!(
        !game.has_flying(permanent(&game, 75_002)),
        "a creature that is not a Sliver is untouched"
    );
}

#[test]
fn a_tribeless_lord_reaches_every_creature() {
    let game = staged(cards::LUMBERING_SATYR);
    for id in [75_000, 75_001, 75_002] {
        assert!(
            game.permanent_has_executable_keyword(
                permanent(&game, id),
                KeywordAbility::Landwalk(BasicLandType::Forest)
            ),
            "\"all creatures\" leaves nothing out, including {id}"
        );
    }
}
