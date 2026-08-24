//! Ludevic's Test Subject: a repeatable mana sink whose fifth hatchling
//! counter removes the whole clutch and turns the Egg into a 13/13.

use super::*;

const HATCHLING_COUNTER: CounterKind = CounterKind::named("hatchling");

fn staged(activations: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let subject = game
        .put_onto_battlefield(PlayerId::One, cards::LUDEVIC_S_TEST_SUBJECT)
        .expect("cataloged");
    drain_pending(&mut game);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, activations);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, activations);
    game.step = Step::PrecombatMain;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    (game, subject)
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("it is on the battlefield")
}

fn hatch(game: &Game, subject: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One).into_iter().find(
        |action| matches!(action, Action::ActivateAbility { source, .. } if *source == subject),
    )
}

fn activate_and_resolve(game: &mut Game, subject: GameObjectId) {
    let action = hatch(game, subject).expect("one generic and one blue pay for it");
    game.apply(PlayerId::One, action).expect("it activates");
    drain_pending(game);
}

#[test]
fn the_front_face_is_a_defending_egg() {
    let (game, subject) = staged(0);
    let front = permanent(&game, subject);

    assert_eq!(
        (game.power(front), game.toughness(front)),
        (Some(0), Some(3))
    );
    assert!(game.permanent_has_executable_keyword(front, KeywordAbility::Defender));
    assert!(game.effective_subtypes(front).contains(&"Egg"));
}

#[test]
fn the_first_four_activations_store_hatchling_counters() {
    let (mut game, subject) = staged(4);

    for expected in 1..=4 {
        activate_and_resolve(&mut game, subject);
        let subject = permanent(&game, subject);
        assert_eq!(subject.counters(HATCHLING_COUNTER), expected);
        let observed = game
            .observe(PlayerId::One)
            .battlefield
            .into_iter()
            .find(|permanent| permanent.id == subject.card.id)
            .expect("the subject is public");
        assert_eq!(
            observed.counters,
            vec![CounterObservation {
                name: "hatchling".to_owned(),
                count: expected,
            }]
        );
        assert!(
            observed.has_individual_state,
            "a counter-bearing permanent must not collapse with a lookalike"
        );
        assert_eq!(
            game.effective_permanent_name(subject).as_deref(),
            Some("Ludevic's Test Subject"),
        );
    }
}

#[test]
fn the_fifth_counter_is_removed_before_the_subject_transforms() {
    let (mut game, subject) = staged(5);
    for _ in 0..5 {
        activate_and_resolve(&mut game, subject);
    }

    let abomination = permanent(&game, subject);
    assert_eq!(abomination.counters(HATCHLING_COUNTER), 0);
    assert_eq!(
        game.effective_permanent_name(abomination).as_deref(),
        Some("Ludevic's Abomination"),
    );
    assert_eq!(
        (game.power(abomination), game.toughness(abomination)),
        (Some(13), Some(13)),
    );
    assert!(game.permanent_has_executable_keyword(abomination, KeywordAbility::Trample));
    assert!(
        hatch(&game, subject).is_none(),
        "the back face has no hatch ability"
    );
}

#[test]
fn five_counters_already_present_still_means_remove_all_after_adding_one() {
    let (mut game, subject) = staged(1);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == subject)
        .expect("it is on the battlefield")
        .set_counters(HATCHLING_COUNTER, 5);

    activate_and_resolve(&mut game, subject);

    let abomination = permanent(&game, subject);
    assert_eq!(abomination.counters(HATCHLING_COUNTER), 0);
    assert_eq!(
        game.effective_permanent_name(abomination).as_deref(),
        Some("Ludevic's Abomination"),
    );
}

#[test]
fn the_activation_needs_both_mana_symbols() {
    let (mut game, subject) = staged(0);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    assert!(
        hatch(&game, subject).is_none(),
        "blue alone cannot pay {{1}}{{U}}"
    );

    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    assert!(hatch(&game, subject).is_some());
}
