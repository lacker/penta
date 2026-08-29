//! Creatures whose power and toughness are a count rather than a number.
//!
//! Each is a characteristic-defining ability: the count is what the creature
//! is, in layer 7a and in every zone (CR 604.3), rather than a bonus applied
//! to a printed body on the battlefield. The last two tests here pin both
//! halves of that -- the count outside the battlefield, and the coverage the
//! cards claim -- rather than leaving either as prose.

use super::*;

fn body(game: &Game, id: GameObjectId) -> (i16, i16) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the creature is on the battlefield");
    (
        game.power(permanent).expect("it is a creature"),
        game.toughness(permanent).expect("it is a creature"),
    )
}

#[test]
fn keldon_warlord_counts_your_non_wall_creatures_and_recounts_as_they_change() {
    let mut game = ready_game();
    let warlord = creature(10_000, cards::KELDON_WARLORD, PlayerId::One);
    let warlord_id = warlord.card.id;
    game.battlefield.push(warlord);
    assert_eq!(body(&game, warlord_id), (1, 1), "it counts itself");

    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One));
    assert_eq!(body(&game, warlord_id), (2, 2));

    // A Wall is a creature and still does not count.
    game.battlefield
        .push(creature(10_002, cards::WALL_OF_WOOD, PlayerId::One));
    assert_eq!(body(&game, warlord_id), (2, 2), "Walls are excluded");

    // Neither does an opposing creature.
    game.battlefield
        .push(creature(10_003, cards::SAVANNAH_LIONS, PlayerId::Two));
    assert_eq!(body(&game, warlord_id), (2, 2), "and so is the opponent's");
}

/// Plague Rats counts every Plague Rats anywhere on the battlefield, which is
/// the one query in this group that is not controller-scoped.
#[test]
fn plague_rats_count_each_other_across_both_sides() {
    let mut game = ready_game();
    let rats = creature(10_000, cards::PLAGUE_RATS, PlayerId::One);
    let rats_id = rats.card.id;
    game.battlefield.push(rats);
    assert_eq!(body(&game, rats_id), (1, 1));

    game.battlefield
        .push(creature(10_001, cards::PLAGUE_RATS, PlayerId::Two));
    assert_eq!(
        body(&game, rats_id),
        (2, 2),
        "an opposing Rat counts for yours too"
    );

    game.battlefield
        .push(creature(10_002, cards::SAVANNAH_LIONS, PlayerId::One));
    assert_eq!(body(&game, rats_id), (2, 2), "and nothing else does");
}

/// Gaea's Avenger prints "1 plus", which is carried by the body rather than
/// the bonus, so it is a 1/1 with no artifacts opposing it.
#[test]
fn gaeas_avenger_starts_at_one_and_grows_with_opposing_artifacts() {
    let mut game = ready_game();
    let avenger = creature(10_000, cards::GAEAS_AVENGER, PlayerId::One);
    let avenger_id = avenger.card.id;
    game.battlefield.push(avenger);
    assert_eq!(body(&game, avenger_id), (1, 1));

    game.battlefield
        .push(creature(10_001, cards::SOL_RING, PlayerId::Two));
    assert_eq!(body(&game, avenger_id), (2, 2));

    // Your own artifacts are not what it answers.
    game.battlefield
        .push(creature(10_002, cards::SOL_RING, PlayerId::One));
    assert_eq!(body(&game, avenger_id), (2, 2));
}

/// A creature whose count is zero is a 0/0 and dies to state-based actions,
/// which is the printed behaviour rather than an accident of the encoding.
#[test]
fn a_counted_body_with_nothing_to_count_dies() {
    let mut game = ready_game();
    let dakkon = creature(10_000, cards::DAKKON_BLACKBLADE, PlayerId::One);
    let dakkon_id = dakkon.card.id;
    game.battlefield.push(dakkon);

    game.check_state_based_actions();

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == dakkon_id),
        "no lands means a 0/0"
    );
}

/// A characteristic-defining ability functions in every zone, so the corner
/// and the creature disagree on purpose: the card prints 0/0 and is not one
/// anywhere.
#[test]
fn the_counted_body_answers_outside_the_battlefield() {
    let mut game = ready_game();
    game.battlefield.clear();
    let warlord = poc::catalog()
        .expect("catalog builds")
        .get(cards::KELDON_WARLORD)
        .expect("the card is cataloged")
        .rules
        .creature_stats()
        .expect("Keldon Warlord is a creature");
    assert_eq!(
        (warlord.power, warlord.toughness),
        (0, 0),
        "the corner says nothing, which is why the ability has to"
    );

    let card = game
        .build_zone(PlayerId::One, &[cards::KELDON_WARLORD])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let card_id = card.id;
    game.players[0].graveyard.push(card);
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::GRIZZLY_BEARS, PlayerId::One));

    assert_eq!(
        game.current_or_last_known_power(card_id),
        Some(2),
        "two non-Wall creatures out, counted from a graveyard",
    );
    assert_eq!(game.current_or_last_known_toughness(card_id), Some(2));
}

/// The other half of "attacking or blocking", which neither single-sided
/// predicate could express. Tetsuo Umezawa is the card that needs it, and the
/// distinction it draws is the whole point: an attacker is not a blocker.
mod blocking_predicate {
    use super::*;

    fn tetsuo_game() -> (Game, GameObjectId) {
        let mut game = ready_game();
        game.turns_started[PlayerId::One.index()] = 1;
        let tetsuo = creature(10_000, cards::TETSUO_UMEZAWA, PlayerId::One);
        let tetsuo_id = tetsuo.card.id;
        game.battlefield.push(tetsuo);
        game.players[PlayerId::One.index()].mana_pool.blue = 1;
        game.players[PlayerId::One.index()].mana_pool.black = 2;
        game.players[PlayerId::One.index()].mana_pool.red = 1;
        (game, tetsuo_id)
    }

    fn targets(game: &Game, source: GameObjectId) -> Vec<GameObjectId> {
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

    #[test]
    fn tetsuo_reaches_a_blocker_but_not_an_untapped_attacker() {
        let (mut game, tetsuo_id) = tetsuo_game();

        // Attacking, and untapped because it has vigilance-like state here.
        let mut attacker = creature(10_001, cards::SEDGE_TROLL, PlayerId::Two);
        attacker.attacking = true;
        let attacker_id = attacker.card.id;
        game.battlefield.push(attacker);

        // Blocking that attacker, and untapped: blockers do not tap.
        let mut blocker = creature(10_002, cards::SAVANNAH_LIONS, PlayerId::One);
        blocker.blocking = vec![attacker_id];
        let blocker_id = blocker.card.id;
        game.battlefield.push(blocker);

        assert_eq!(
            targets(&game, tetsuo_id),
            vec![blocker_id],
            "a blocker qualifies and an untapped attacker does not"
        );
    }

    #[test]
    fn tetsuo_also_reaches_anything_tapped() {
        let (mut game, tetsuo_id) = tetsuo_game();
        let mut tapped = creature(10_001, cards::SEDGE_TROLL, PlayerId::Two);
        tapped.tapped = true;
        let tapped_id = tapped.card.id;
        game.battlefield.push(tapped);
        game.battlefield
            .push(creature(10_002, cards::SAVANNAH_LIONS, PlayerId::Two));

        assert_eq!(targets(&game, tetsuo_id), vec![tapped_id]);
    }

    #[test]
    fn people_of_the_woods_counts_only_its_toughness() {
        let mut game = ready_game();
        let people = creature(10_000, cards::PEOPLE_OF_THE_WOODS, PlayerId::One);
        let people_id = people.card.id;
        game.battlefield.push(people);
        game.battlefield
            .push(creature(10_001, cards::FOREST, PlayerId::One));
        game.battlefield
            .push(creature(10_002, cards::FOREST, PlayerId::One));

        assert_eq!(
            body(&game, people_id),
            (1, 2),
            "the printed power stays put while the toughness counts Forests"
        );
    }
}
