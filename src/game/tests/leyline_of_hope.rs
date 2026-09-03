//! Leyline of Hope: one extra life from each gain and a live anthem once its
//! controller is seven above the life total supplied by the format.

use super::*;

fn stats(game: &Game, id: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still on the battlefield");
    (game.power(permanent), game.toughness(permanent))
}

fn add_leyline(game: &mut Game, controller: PlayerId) {
    game.put_onto_battlefield(controller, cards::LEYLINE_OF_HOPE)
        .expect("Leyline of Hope is cataloged");
    drain_pending(game);
}

#[test]
fn leyline_adds_one_to_its_controllers_life_gain() {
    let mut game = ready_game();
    add_leyline(&mut game, PlayerId::One);

    game.gain_life(PlayerId::One, 3);
    game.gain_life(PlayerId::Two, 3);

    assert_eq!(game.players[PlayerId::One.index()].life, 24);
    assert_eq!(
        game.players[PlayerId::Two.index()].life,
        23,
        "the opponent is outside the replacement's relation",
    );
}

#[test]
fn each_leyline_replaces_the_same_life_gain_once() {
    let mut game = ready_game();
    add_leyline(&mut game, PlayerId::One);
    add_leyline(&mut game, PlayerId::One);

    game.gain_life(PlayerId::One, 3);

    assert_eq!(game.players[PlayerId::One.index()].life, 25);
}

#[test]
fn noncommuting_life_gain_replacements_are_ordered_by_the_affected_player() {
    let life_after_ordering = |first_text: &str| {
        let mut game = ready_game();
        add_leyline(&mut game, PlayerId::One);
        game.battlefield
            .push(creature(10_100, cards::RHOX_FAITHMENDER, PlayerId::One));

        game.gain_life(PlayerId::One, 3);
        assert_eq!(
            game.players[PlayerId::One.index()].life,
            20,
            "the prospective gain waits for its replacement order",
        );
        let decision = game
            .observe(PlayerId::One)
            .decision
            .expect("the affected player chooses the first replacement");
        let option = decision
            .options
            .iter()
            .find(|option| {
                option
                    .ability_text
                    .as_deref()
                    .is_some_and(|text| text.contains(first_text))
            })
            .expect("the named replacement is offered")
            .id;
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![option],
            },
        )
        .expect("the replacement order is legal");
        game.players[PlayerId::One.index()].life
    };

    assert_eq!(life_after_ordering("plus 1"), 28, "(3 + 1) × 2");
    assert_eq!(life_after_ordering("twice"), 27, "(3 × 2) + 1");
}

#[test]
fn the_anthem_tracks_seven_above_the_formats_starting_life() {
    let mut game = ready_game();
    add_leyline(&mut game, PlayerId::One);
    let bear = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);

    let threshold = i16::from(game.starting_life_total()) + 7;
    game.players[PlayerId::One.index()].life = threshold - 1;
    assert_eq!(stats(&game, bear_id), (Some(2), Some(2)));

    game.players[PlayerId::One.index()].life = threshold;
    assert_eq!(stats(&game, bear_id), (Some(4), Some(4)));

    game.players[PlayerId::One.index()].life = threshold - 1;
    assert_eq!(
        stats(&game, bear_id),
        (Some(2), Some(2)),
        "the as-long-as effect turns off again",
    );
}

#[test]
fn the_anthem_only_affects_creatures_its_source_controls() {
    let mut game = ready_game();
    add_leyline(&mut game, PlayerId::One);
    let ours = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let ours_id = ours.card.id;
    game.battlefield.push(ours);
    let theirs = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);
    game.players[PlayerId::One.index()].life = i16::from(game.starting_life_total()) + 7;

    assert_eq!(stats(&game, ours_id), (Some(4), Some(4)));
    assert_eq!(stats(&game, theirs_id), (Some(2), Some(2)));
}
