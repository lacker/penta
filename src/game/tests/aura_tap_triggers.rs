//! Auras that watch their own host being tapped.
//!
//! The trigger is the Aura's, but the event belongs to the land: the
//! relationship is read from the Aura outwards, and the player it answers is
//! the host's controller rather than the Aura's. Tapping a land for mana is
//! the ordinary way this comes up, so that is what these drive.

use super::*;

/// A Forest under player two, enchanted by `aura` cast by player one.
fn enchanted_forest(aura: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    let forest = creature(10_000, cards::FOREST, PlayerId::Two);
    let forest_id = forest.card.id;
    game.battlefield.push(forest);

    let mut enchantment = creature(10_001, aura, PlayerId::One);
    enchantment.attached_to = Some(forest_id);
    let aura_id = enchantment.card.id;
    game.battlefield.push(enchantment);
    game.check_state_based_actions();
    (game, forest_id, aura_id)
}

/// Taps the land the ordinary way -- for mana.
fn activate_for_mana(game: &mut Game, land: GameObjectId) {
    game.priority = PlayerId::Two;
    let action = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateManaAbility { source, .. } if *source == land))
        .expect("the land can be tapped for mana");
    game.apply(PlayerId::Two, action)
        .expect("the mana ability activates");
    game.priority = PlayerId::One;
}

/// Taps the land the ordinary way -- for mana -- and settles the triggers.
fn tap_for_mana(game: &mut Game, land: GameObjectId) {
    activate_for_mana(game, land);
    drain_pending(game);
}

#[test]
fn psychic_venom_burns_the_lands_controller() {
    let (mut game, forest, _) = enchanted_forest(cards::PSYCHIC_VENOM);
    let before = [
        game.players[PlayerId::One.index()].life,
        game.players[PlayerId::Two.index()].life,
    ];

    tap_for_mana(&mut game, forest);

    assert_eq!(
        game.players[PlayerId::Two.index()].life,
        before[1] - 2,
        "the land's controller took it, not the Aura's"
    );
    assert_eq!(game.players[PlayerId::One.index()].life, before[0]);
}

/// Another land tapping is not this Aura's business.
#[test]
fn an_unenchanted_land_taps_freely() {
    let (mut game, _, _) = enchanted_forest(cards::PSYCHIC_VENOM);
    let other = creature(10_002, cards::FOREST, PlayerId::Two);
    let other_id = other.card.id;
    game.battlefield.push(other);
    let before = game.players[PlayerId::Two.index()].life;

    tap_for_mana(&mut game, other_id);

    assert_eq!(game.players[PlayerId::Two.index()].life, before);
}

#[test]
fn blight_destroys_the_land_it_enchants() {
    let (mut game, forest, aura) = enchanted_forest(cards::BLIGHT);

    tap_for_mana(&mut game, forest);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == forest),
        "the land it enchanted is gone"
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == aura),
        "and the Aura fell off with it"
    );
}

#[test]
fn kudzu_hands_the_optional_reattachment_to_the_destroyed_lands_controller() {
    let (mut game, forest, aura) = enchanted_forest(cards::KUDZU);
    let destination = creature(10_002, cards::ISLAND, PlayerId::One);
    let destination_id = destination.card.id;
    let forbidden = creature(10_003, cards::GRIZZLY_BEARS, PlayerId::Two);
    let forbidden_id = forbidden.card.id;
    game.battlefield.extend([destination, forbidden]);

    activate_for_mana(&mut game, forest);
    let decision = advance_to_prompt(&mut game, PlayerId::Two, "Choose objects");

    assert_eq!((decision.minimum, decision.maximum), (0, 1));
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == forest),
        "the tapped land was destroyed before the choice"
    );
    let offered = decision
        .options
        .iter()
        .filter_map(|option| option.card.map(|(card, _)| card))
        .collect::<Vec<_>>();
    assert!(offered.contains(&destination_id));
    assert!(
        !offered.contains(&forbidden_id),
        "a nonland is not a legal host for Kudzu"
    );
    let destination_option = decision
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(card, _)| card == destination_id))
        .expect("the legal destination is offered")
        .id;
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![destination_option],
        },
    )
    .expect("the land's controller may move Kudzu");

    let kudzu = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == aura)
        .expect("Kudzu remains on the battlefield after reattaching");
    assert_eq!(kudzu.attached_to, Some(destination_id));
}

#[test]
fn kudzu_dies_when_the_lands_controller_declines_reattachment() {
    let (mut game, forest, aura) = enchanted_forest(cards::KUDZU);
    let destination = creature(10_002, cards::ISLAND, PlayerId::One);
    game.battlefield.push(destination);

    activate_for_mana(&mut game, forest);
    let decision = advance_to_prompt(&mut game, PlayerId::Two, "Choose objects");
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: Vec::new(),
        },
    )
    .expect("reattachment is optional");

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == aura),
        "an unattached Aura is put into its owner's graveyard"
    );
}

/// Spirit Shackle's counter is toughness only, and it stacks: two taps take
/// four toughness off rather than one.
#[test]
fn spirit_shackle_stacks_its_counters_on_the_creature() {
    let mut game = ready_game();
    let troll = creature(10_000, cards::SHIVAN_DRAGON, PlayerId::Two);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);
    let mut shackle = creature(10_001, cards::SPIRIT_SHACKLE, PlayerId::One);
    shackle.attached_to = Some(troll_id);
    game.battlefield.push(shackle);
    game.check_state_based_actions();

    for _ in 0..2 {
        let troll = game
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == troll_id)
            .expect("still there");
        troll.tapped = false;
        let _ = game.tap_permanent(troll_id);
        game.priority = PlayerId::One;
        drain_pending(&mut game);
    }

    let troll = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == troll_id)
        .expect("a 5/5 survives four toughness coming off");
    assert_eq!(troll.counters(CounterKind::MinusZeroMinusTwo), 2);
    assert_eq!(
        (game.power(troll), game.toughness(troll)),
        (Some(5), Some(1)),
        "toughness only"
    );
}
