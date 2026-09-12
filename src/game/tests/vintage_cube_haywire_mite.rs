//! Haywire Mite: a one-mana body that answers whichever artifact or
//! enchantment the format is afraid of, and pays two life on the way out.

use super::*;

/// The Mite on the battlefield with a green source, and a board holding one
/// of everything its ability might name.
fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let mite = game
        .put_onto_battlefield(PlayerId::One, cards::HAYWIRE_MITE)
        .expect("cataloged");
    drain_pending(&mut game);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.step = Step::PrecombatMain;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    (game, mite)
}

fn activation(game: &Game, mite: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == mite))
}

fn offered_targets(game: &Game, mite: GameObjectId) -> Vec<GameObjectId> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } if source == mite => Some(targets),
            _ => None,
        })
        .flatten()
        .flat_map(|selection| selection.targets().to_vec())
        .filter_map(|target| match target {
            Target::Permanent(id) | Target::Card(id) => Some(id),
            Target::Player(_) | Target::Spell(_) => None,
        })
        .collect()
}

/// Dying gains two life, however it happened.
#[test]
fn dying_gains_two_life() {
    let (mut game, mite) = staged();
    game.players[0].life = 20;

    game.move_permanents_to_graveyard(&[mite]);
    drain_pending(&mut game);

    assert_eq!(game.players[0].life, 22);
}

/// The ability exiles a noncreature artifact.
#[test]
fn it_exiles_a_noncreature_artifact() {
    let (mut game, mite) = staged();
    game.put_onto_battlefield(PlayerId::Two, cards::BLACK_LOTUS)
        .expect("cataloged");
    drain_pending(&mut game);

    let action = activation(&game, mite).expect("one green and a sacrifice");
    game.apply(PlayerId::One, action).expect("it activates");
    drain_pending(&mut game);

    assert!(
        game.players[1]
            .exile
            .iter()
            .any(|card| card.definition == cards::BLACK_LOTUS),
        "the Lotus is exiled, not destroyed",
    );
}

/// Sacrificing itself is a cost, so the Mite is gone and its own dies
/// trigger still pays out.
#[test]
fn activating_it_sacrifices_it_and_gains_the_life() {
    let (mut game, mite) = staged();
    game.put_onto_battlefield(PlayerId::Two, cards::BLACK_LOTUS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.players[0].life = 20;

    let action = activation(&game, mite).expect("one green and a sacrifice");
    game.apply(PlayerId::One, action).expect("it activates");
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != mite),
        "it sacrificed itself as a cost",
    );
    assert_eq!(game.players[0].life, 22, "and died on the way, as it says");
}

/// "Noncreature": an artifact creature is not on the menu, and neither is
/// an ordinary creature.
#[test]
fn it_leaves_creatures_alone() {
    let (mut game, mite) = staged();
    let other_mite = game
        .put_onto_battlefield(PlayerId::Two, cards::HAYWIRE_MITE)
        .expect("cataloged");
    let bears = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    let lotus = game
        .put_onto_battlefield(PlayerId::Two, cards::BLACK_LOTUS)
        .expect("cataloged");
    drain_pending(&mut game);

    let offered = offered_targets(&game, mite);

    assert!(
        offered.contains(&lotus),
        "the Lotus is a noncreature artifact"
    );
    assert!(
        !offered.contains(&other_mite),
        "an artifact creature is still a creature: {offered:?}",
    );
    assert!(!offered.contains(&bears));
}

/// Without green mana there is nothing to activate it with.
#[test]
fn it_needs_the_green_mana() {
    let (mut game, mite) = staged();
    game.players[0].mana_pool = ManaPool::default();
    game.put_onto_battlefield(PlayerId::Two, cards::BLACK_LOTUS)
        .expect("cataloged");
    drain_pending(&mut game);

    assert!(
        activation(&game, mite).is_none(),
        "every deck can cast it; not every deck can use it",
    );
}

/// The other half of the ability, and the same exclusion on that side: a
/// plain enchantment goes, and an enchantment creature stays.
#[test]
fn it_exiles_a_noncreature_enchantment() {
    let (mut game, mite) = staged();
    let oath = game
        .put_onto_battlefield(PlayerId::Two, cards::OATH_OF_DRUIDS)
        .expect("cataloged");
    let innocence = game
        .put_onto_battlefield(PlayerId::Two, cards::FEAR_OF_SURVEILLANCE)
        .expect("cataloged");
    drain_pending(&mut game);

    let offered = offered_targets(&game, mite);
    assert!(offered.contains(&oath), "the Oath is a noncreature one");
    assert!(
        !offered.contains(&innocence),
        "an enchantment creature is still a creature: {offered:?}",
    );

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == mite
                    && targets
                        .iter()
                        .any(|selection| selection.targets().contains(&Target::Permanent(oath)))
            }
            _ => false,
        })
        .expect("naming the Oath is legal");
    game.apply(PlayerId::One, action).expect("it activates");
    drain_pending(&mut game);

    assert!(
        game.players[1]
            .exile
            .iter()
            .any(|card| card.definition == cards::OATH_OF_DRUIDS),
        "exiled, which is how an Oath stays answered",
    );
}

/// Nothing in the cost is a tap, so the Mite eats something the turn it
/// lands: one mana, one artifact, and a body that was never going to attack.
#[test]
fn a_fresh_mite_still_eats() {
    let (mut game, mite) = staged();
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == mite)
    {
        permanent.entered_controller_turn = game.turns_started[0];
    }
    game.put_onto_battlefield(PlayerId::Two, cards::BLACK_LOTUS)
        .expect("cataloged");
    drain_pending(&mut game);

    let action = activation(&game, mite).expect("summoning sickness stops taps, not sacrifices");
    game.apply(PlayerId::One, action).expect("it activates");
    drain_pending(&mut game);

    assert!(
        game.players[1]
            .exile
            .iter()
            .any(|card| card.definition == cards::BLACK_LOTUS),
        "the Lotus is gone on the turn the Mite arrived",
    );
}

/// "Exile" rather than "destroy": indestructible is no answer to it.
#[test]
fn it_exiles_an_indestructible_artifact() {
    let (mut game, mite) = staged();
    game.put_onto_battlefield(PlayerId::Two, cards::DARKSTEEL_INGOT)
        .expect("cataloged");
    drain_pending(&mut game);

    let action = activation(&game, mite).expect("one green and a sacrifice");
    game.apply(PlayerId::One, action).expect("it activates");
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::DARKSTEEL_INGOT),
        "what cannot be destroyed can still be exiled",
    );
    assert_eq!(game.players[0].life, 22, "and the Mite paid out on the way");
}

/// "Target noncreature artifact" does not say whose: the Mite is as happy to
/// eat your own Lotus, which is how it answers a Winter Orb you played
/// yourself.
#[test]
fn it_eats_your_own_artifact_as_readily_as_theirs() {
    let (mut game, mite) = staged();
    let own_lotus = game
        .put_onto_battlefield(PlayerId::One, cards::BLACK_LOTUS)
        .expect("cataloged");
    drain_pending(&mut game);

    assert!(
        offered_targets(&game, mite).contains(&own_lotus),
        "your own artifact is a legal target",
    );

    let action = activation(&game, mite).expect("one green and a sacrifice");
    game.apply(PlayerId::One, action).expect("it activates");
    drain_pending(&mut game);

    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::BLACK_LOTUS),
        "and it goes to its owner's exile",
    );
}

/// The sacrifice is a cost, so answering the Mite by removing its target
/// costs you the Mite anyway: the ability fizzles, and the two life is
/// already yours.
#[test]
fn the_cost_is_paid_even_when_the_target_is_gone() {
    let (mut game, mite) = staged();
    let lotus = game
        .put_onto_battlefield(PlayerId::Two, cards::BLACK_LOTUS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.players[0].life = 20;
    let event_start = game.events().len();

    let action = activation(&game, mite).expect("one green and a sacrifice");
    game.apply(PlayerId::One, action).expect("it activates");
    game.move_permanents_to_graveyard(&[lotus]);
    drain_pending(&mut game);

    assert!(
        game.events()[event_start..]
            .iter()
            .any(|event| matches!(event, GameEvent::AbilityFizzled { .. })),
        "with its only target gone the ability does nothing",
    );
    assert!(
        game.players[1].exile.is_empty(),
        "nothing was exiled: the Lotus is in the graveyard",
    );
    assert_eq!(
        game.players[0].life, 22,
        "but the Mite is dead all the same"
    );
}

/// "Noncreature" is read as the ability is offered rather than when the
/// artifact was played: a Jade Statue is on the menu while it stands there
/// and off it the moment it animates itself.
#[test]
fn an_artifact_that_animates_stops_being_a_legal_target() {
    let (mut game, mite) = staged();
    let statue = game
        .put_onto_battlefield(PlayerId::One, cards::JADE_STATUE)
        .expect("cataloged");
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    assert!(
        offered_targets(&game, mite).contains(&statue),
        "an artifact that is no creature is what the Mite eats",
    );

    // Its own ability only opens in combat, which is where it becomes one.
    game.step = Step::BeginningOfCombat;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    let animate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == statue),
        )
        .expect("two mana animates it during combat");
    game.apply(PlayerId::One, animate).expect("it activates");
    drain_pending(&mut game);

    let animated = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == statue)
        .expect("it is still there");
    assert!(
        game.permanent_types(animated)
            .is_some_and(|types| types.contains(CardType::Creature)),
        "it is a creature now",
    );
    assert!(
        !offered_targets(&game, mite).contains(&statue),
        "and the Mite has nothing to say to a creature",
    );
}

/// "When this creature dies" is a graveyard move from the battlefield and
/// nothing else. A Mite exiled or bounced pays nobody the two life, which is
/// what separates its trigger from "leaves the battlefield".
#[test]
fn only_dying_pays_the_two_life() {
    for exiled in [true, false] {
        let (mut game, mite) = staged();
        let before = game.players[0].life;

        if exiled {
            game.exile_permanent(mite);
        } else {
            game.return_permanent_to_hand(mite);
        }
        drain_pending(&mut game);
        game.check_state_based_actions();
        drain_pending(&mut game);

        assert!(
            game.battlefield
                .iter()
                .all(|permanent| permanent.card.id != mite),
            "it left the battlefield, exiled: {exiled}",
        );
        assert_eq!(
            game.players[0].life, before,
            "and left without dying, so nothing was gained, exiled: {exiled}",
        );
    }
}

/// A land is neither an artifact nor an enchantment, and neither is a
/// planeswalker: the two halves of the clause are the whole menu.
#[test]
fn a_land_and_a_planeswalker_are_no_targets() {
    let (mut game, mite) = staged();
    let island = game
        .put_onto_battlefield(PlayerId::Two, cards::ISLAND)
        .expect("cataloged");
    let walker = game
        .put_onto_battlefield(PlayerId::Two, cards::JACE_THE_MIND_SCULPTOR)
        .expect("cataloged");
    let lotus = game
        .put_onto_battlefield(PlayerId::Two, cards::BLACK_LOTUS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    let offered = offered_targets(&game, mite);
    assert!(offered.contains(&lotus), "the artifact is on the menu");
    assert!(!offered.contains(&island), "the land is not");
    assert!(
        !offered.contains(&walker),
        "and neither is the planeswalker"
    );
}
