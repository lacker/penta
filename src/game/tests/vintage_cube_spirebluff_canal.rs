//! Spirebluff Canal under a Blood Moon.
//!
//! The fastland cycle is checked as a family where the cycle lives; what
//! this file adds is what happens to one of them when the type line is
//! replaced wholesale: the clause that taps it is printed text, and printed
//! text is exactly what a Blood Moon takes away.

use super::*;

fn canal(game: &Game) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SPIREBLUFF_CANAL)
        .expect("the Canal is on the battlefield")
}

/// Five lands is well past the boundary, so the Canal arrives tapped and
/// makes either of its two colours.
#[test]
fn a_late_canal_arrives_tapped_and_makes_both_colors() {
    let mut game = ready_game();
    game.battlefield.clear();
    for index in 0..5 {
        game.battlefield
            .push(creature(96_000 + index, cards::FOREST, PlayerId::One));
    }

    game.put_onto_battlefield(PlayerId::One, cards::SPIREBLUFF_CANAL)
        .expect("cataloged");
    drain_pending(&mut game);

    assert!(canal(&game).tapped, "five other lands is more than two");
    let id = canal(&game).card.id;
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == id)
        .expect("it is there")
        .tapped = false;
    assert_eq!(
        game.legal_actions(PlayerId::One)
            .into_iter()
            .filter_map(|action| match action {
                Action::ActivateManaAbility { source, color, .. } if source == id => Some(color),
                _ => None,
            })
            .collect::<Vec<_>>(),
        vec![ManaColor::Blue, ManaColor::Red],
        "blue or red, as printed",
    );
}

/// A Blood Moon takes the printed text with the type line: the Canal is a
/// Mountain, so the clause that would have tapped it is gone and it makes
/// red alone -- which is the one board state where a late fastland arrives
/// untapped.
#[test]
fn a_blood_moon_leaves_it_an_untapped_mountain() {
    let mut game = ready_game();
    game.battlefield.clear();
    for index in 0..5 {
        game.battlefield
            .push(creature(96_100 + index, cards::FOREST, PlayerId::One));
    }
    game.put_onto_battlefield(PlayerId::One, cards::BLOOD_MOON)
        .expect("cataloged");
    drain_pending(&mut game);

    game.put_onto_battlefield(PlayerId::One, cards::SPIREBLUFF_CANAL)
        .expect("cataloged");
    drain_pending(&mut game);

    assert!(
        !canal(&game).tapped,
        "the clause that taps it went with the rest of the text",
    );
    let id = canal(&game).card.id;
    assert!(
        game.effective_subtypes(canal(&game))
            .contains(crate::card::Subtype::named("Mountain")),
        "it is a Mountain now",
    );
    assert_eq!(
        game.legal_actions(PlayerId::One)
            .into_iter()
            .filter_map(|action| match action {
                Action::ActivateManaAbility { source, color, .. } if source == id => Some(color),
                _ => None,
            })
            .collect::<Vec<_>>(),
        vec![ManaColor::Red],
        "and a Mountain makes red",
    );
}
