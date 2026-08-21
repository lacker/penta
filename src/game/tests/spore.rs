//! The Fallen Empires spore cycle.
//!
//! Every Thallid stores spore counters on an upkeep trigger and spends three
//! of them on something. The something differs per card and each one is a
//! capability the engine gained separately, so this cycle is mostly a test
//! that those pieces compose: a counter kind, a counter-removal cost, and
//! whatever the card does with it.

use super::*;

fn thallid_game(definition: CardDefinitionId) -> (Game, GameObjectId) {
    let mut game = ready_game();
    let thallid = creature(10_000, definition, PlayerId::One);
    let id = thallid.card.id;
    game.battlefield.push(thallid);
    (game, id)
}

fn spores(game: &Game, id: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .map_or(0, |permanent| permanent.counters(CounterKind::Spore))
}

fn take_upkeep(game: &mut Game) {
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    game.finish_rules_procedure();
    for _ in 0..8 {
        if game.stack.is_empty() {
            break;
        }
        let priority = game.priority;
        game.apply(priority, Action::PassPriority)
            .expect("priority passes while the spore trigger resolves");
    }
}

fn spore_ability(game: &Game, source: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One).into_iter().find(|action| {
        matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source)
    })
}

#[test]
fn a_thallid_stores_one_spore_counter_each_upkeep() {
    let (mut game, thallid) = thallid_game(cards::THALLID);
    assert_eq!(spores(&game, thallid), 0);
    for expected in 1..=3 {
        take_upkeep(&mut game);
        assert_eq!(spores(&game, thallid), expected);
    }
}

/// Three counters is the cost, so the ability is unavailable below that.
#[test]
fn the_spore_ability_is_not_offered_until_three_counters_are_stored() {
    let (mut game, thallid) = thallid_game(cards::THALLID);
    game.step = Step::PrecombatMain;
    assert!(
        spore_ability(&game, thallid).is_none(),
        "no counters, no ability"
    );

    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == thallid)
    {
        permanent.set_counters(CounterKind::Spore, 2);
    }
    assert!(
        spore_ability(&game, thallid).is_none(),
        "two counters is still short"
    );

    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == thallid)
    {
        permanent.set_counters(CounterKind::Spore, 3);
    }
    assert!(spore_ability(&game, thallid).is_some(), "three pays for it");
}

#[test]
fn spending_three_counters_makes_a_saproling() {
    let (mut game, thallid) = thallid_game(cards::THALLID);
    game.step = Step::PrecombatMain;
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == thallid)
    {
        permanent.set_counters(CounterKind::Spore, 3);
    }

    let action = spore_ability(&game, thallid).expect("the ability is available");
    game.apply(PlayerId::One, action).expect("it activates");
    pass_priority_pair(&mut game);

    assert_eq!(spores(&game, thallid), 0, "the counters were the cost");
    assert!(
        game.battlefield.iter().any(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Saproling"], &[ManaColor::Green], 1, 1),
            )
        }),
        "a Saproling token arrived"
    );
}

/// Each card in the cycle spends its counters on a different capability, so
/// this is also a check that those compose rather than merely coexist.
#[test]
fn every_spore_card_reports_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [
        cards::THALLID,
        cards::THORN_THALLID,
        cards::FERAL_THALLID,
        cards::SPORE_FLOWER,
        cards::FUNGAL_BLOOM,
    ] {
        let card = catalog.get(definition).expect("the card is cataloged");
        assert_eq!(
            card.rules.implementation_status(),
            crate::ImplementationStatus::Complete,
            "{} should be fully executable",
            card.name,
        );
    }
}

/// Two identities the first spore pass left behind. Their spore clauses were
/// already built; what they add is a second use for the token those clauses
/// make, spending a Saproling as an activation cost. That is the clause the
/// original cycle never had, so it is what these drive.
mod saproling_sacrifice {
    use super::*;

    fn farm(card: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
        let mut game = ready_game();
        let farmer = creature(10_000, card, PlayerId::One);
        let farmer_id = farmer.card.id;
        game.battlefield.push(farmer);
        let saproling = token_permanent(
            10_001,
            tokens::creature(&["Saproling"], &[ManaColor::Green], 1, 1),
            PlayerId::One,
        );
        let saproling_id = saproling.card.id;
        game.battlefield.push(saproling);
        (game, farmer_id, saproling_id)
    }

    fn sacrifice_clause(game: &Game, source: GameObjectId, fodder: GameObjectId) -> Action {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(
                    action,
                    Action::ActivateAbility { source: actual, cost_objects, .. }
                        if *actual == source && cost_objects.as_slice() == [fodder]
                )
            })
            .expect("sacrificing the Saproling is offered")
    }

    #[test]
    fn elvish_farmer_eats_a_saproling_for_two_life() {
        let (mut game, farmer_id, saproling_id) = farm(cards::ELVISH_FARMER);
        let action = sacrifice_clause(&game, farmer_id, saproling_id);
        game.apply(PlayerId::One, action)
            .expect("the ability activates");
        pass_priority_pair(&mut game);

        assert_eq!(
            game.players[PlayerId::One.index()].life,
            i16::from(rules::STARTING_LIFE) + 2,
        );
        assert!(
            !game
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == saproling_id),
            "the Saproling paid the cost"
        );
    }

    #[test]
    fn thallid_devourer_eats_a_saproling_to_grow() {
        let (mut game, devourer_id, saproling_id) = farm(cards::THALLID_DEVOURER);
        let action = sacrifice_clause(&game, devourer_id, saproling_id);
        game.apply(PlayerId::One, action)
            .expect("the ability activates");
        pass_priority_pair(&mut game);

        let devourer = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == devourer_id)
            .expect("the Devourer is still there");
        assert_eq!(game.power(devourer), Some(3));
        assert_eq!(game.toughness(devourer), Some(4));
    }

    /// A Saproling is not the only creature it could eat, but it is the only
    /// one the cost accepts.
    #[test]
    fn the_cost_refuses_a_creature_that_is_not_a_saproling() {
        let (mut game, farmer_id, _) = farm(cards::ELVISH_FARMER);
        let bear = creature(10_002, cards::SEDGE_TROLL, PlayerId::One);
        let bear_id = bear.card.id;
        game.battlefield.push(bear);

        assert!(
            !game.legal_actions(PlayerId::One).iter().any(|action| {
                matches!(
                    action,
                    Action::ActivateAbility { source, cost_objects, .. }
                        if *source == farmer_id && cost_objects.as_slice() == [bear_id]
                )
            }),
            "only a Saproling pays this cost"
        );
    }

    #[test]
    fn both_identities_report_complete_coverage() {
        let catalog = poc::catalog().expect("catalog builds");
        for definition in [cards::ELVISH_FARMER, cards::THALLID_DEVOURER] {
            let card = catalog.get(definition).expect("the card is cataloged");
            assert_eq!(
                card.rules.implementation_status(),
                crate::ImplementationStatus::Complete,
                "{} should be fully executable",
                card.name,
            );
        }
    }
}
