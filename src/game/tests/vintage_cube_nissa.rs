//! Nissa, Who Shakes the World: doubled Forests, an awakened land, and the
//! ultimate that protects what it fetches.

use super::*;

/// Resolves whatever is on the stack, answering nothing.
fn resolve(game: &mut Game) {
    for _ in 0..12 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
}

/// Puts Nissa on the battlefield and returns her object id.
fn nissa_on_the_battlefield(game: &mut Game, id: u32) -> GameObjectId {
    let nissa = creature(id, cards::NISSA_WHO_SHAKES_THE_WORLD, PlayerId::One);
    let nissa_id = nissa.card.id;
    game.battlefield.push(nissa);
    nissa_id
}

/// Tapping a Forest for mana adds a second green, and tapping something else
/// does not.
#[test]
fn nissa_doubles_your_forests_and_nothing_else() {
    for (land, expected) in [(cards::FOREST, 2), (cards::MOUNTAIN, 0)] {
        let mut game = ready_game();
        game.battlefield.clear();
        nissa_on_the_battlefield(&mut game, 85_000);
        let source = creature(85_001, land, PlayerId::One);
        let source_id = source.card.id;
        game.battlefield.push(source);

        let color = if land == cards::FOREST {
            ManaColor::Green
        } else {
            ManaColor::Red
        };
        game.apply(
            PlayerId::One,
            Action::ActivateManaAbility {
                source: source_id,
                ability: mana_ability_for(&game, source_id, color),
                color,
                counters_removed: None,
                cost_object: None,
                combination: None,
                triggered_mana: None,
            },
        )
        .expect("the land taps for mana");
        drain_pending(&mut game);

        assert_eq!(
            game.players[0].mana_pool.green, expected,
            "{land:?} should make {expected} green",
        );
    }
}

/// The +1 grows a land into a 3/3 that is still a land, and that can attack
/// the turn it wakes up.
#[test]
fn the_plus_one_wakes_a_land_that_is_still_a_land() {
    let mut game = ready_game();
    game.battlefield.clear();
    let nissa_id = nissa_on_the_battlefield(&mut game, 85_010);
    let forest = creature(85_011, cards::FOREST, PlayerId::One);
    let forest_id = forest.card.id;
    game.battlefield.push(forest);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == nissa_id
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(forest_id))
            }
            _ => false,
        })
        .expect("the +1 can point at a noncreature land");
    game.apply(PlayerId::One, action).expect("it is activated");
    resolve(&mut game);
    drain_pending(&mut game);

    let land = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == forest_id)
        .expect("the land is still there");
    assert_eq!(game.power(land), Some(3), "three counters on a 0/0 base");
    assert_eq!(game.toughness(land), Some(3));
    assert!(
        game.permanent_has_executable_keyword(land, KeywordAbility::Haste),
        "and it can attack at once",
    );
    assert!(
        game.permanent_has_executable_keyword(land, KeywordAbility::Vigilance),
        "without tapping to do it",
    );
    let types = game
        .permanent_types(land)
        .expect("an animated land has types");
    assert!(
        types.contains(CardType::Land),
        "still a land, which is why Nissa still doubles it",
    );
    assert!(types.is_creature());
}

/// The ultimate fetches every Forest in the library and leaves an emblem
/// that keeps them alive.
#[test]
fn the_ultimate_fetches_the_forests_and_protects_them() {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut nissa = creature(85_020, cards::NISSA_WHO_SHAKES_THE_WORLD, PlayerId::One);
    nissa.add_counters(CounterKind::Loyalty, 8);
    let nissa_id = nissa.card.id;
    game.battlefield.push(nissa);
    game.players[0].library.clear();
    for id in 85_021..85_024 {
        game.players[0]
            .library
            .push(card(id, cards::FOREST, PlayerId::One));
    }
    game.players[0]
        .library
        .push(card(85_030, cards::MOUNTAIN, PlayerId::One));

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == nissa_id)
                && matches!(action, Action::ActivateAbility { ability, .. }
                    if game
                        .ability_for_origin(nissa_id, *ability)
                        .is_some_and(|ability| ability.text.starts_with('\u{2212}')))
        })
        .expect("eight loyalty pays for the ultimate");
    game.apply(PlayerId::One, action).expect("it is activated");
    resolve(&mut game);

    // "Any number" is sized from the library, so the search offers every
    // Forest in it and none of the Mountain.
    let search = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the search asks which cards to take");
    assert_eq!((search.minimum, search.maximum), (0, 3));
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: search.id,
            options: search.options.iter().map(|option| option.id).collect(),
        },
    )
    .expect("taking all three is a legal answer");
    drain_pending(&mut game);

    let forests = game
        .battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == cards::FOREST)
        .count();
    assert_eq!(forests, 3, "every Forest in the library arrives");
    assert!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == cards::FOREST)
            .all(|permanent| permanent.tapped),
        "tapped, as the card says",
    );
    assert_eq!(game.emblems.len(), 1, "and the emblem is made");
}
