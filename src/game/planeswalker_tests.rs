use super::tests::{creature, ready_game};
use super::*;
use crate::card::cards;
use crate::{CardInstanceId, HandcraftedPolicy, Policy};

fn card(id: u32, definition: CardDefinitionId, owner: PlayerId) -> CardInstance {
    CardInstance {
        id: CardInstanceId(id),
        definition,
        owner,
        backing: ObjectBacking::Cards(vec![PhysicalCardId(id)]),
        characteristics: CharacteristicSource::Card(definition),
    }
}

fn planeswalker_game() -> Game {
    let mut game = ready_game();
    for player in &mut game.players {
        player.library.clear();
    }
    game
}

fn planeswalker(
    id: u32,
    definition: CardDefinitionId,
    controller: PlayerId,
    loyalty: u16,
) -> Permanent {
    let mut permanent = creature(id, definition, controller);
    permanent.set_counters(CounterKind::Loyalty, loyalty);
    permanent
}

const fn loyalty_origin(definition: CardDefinitionId, ability: u8) -> AbilityOrigin {
    AbilityOrigin::Printed {
        definition,
        part: CardPartId::PRIMARY,
        ability: AbilityId(ability),
    }
}

fn loyalty_action(
    source: GameObjectId,
    definition: CardDefinitionId,
    ability: u8,
    targets: Vec<TargetSelection>,
) -> Action {
    Action::ActivateAbility {
        source,
        ability: loyalty_origin(definition, ability),
        targets,
        cost_object: None,
        x: 0,
    }
}

fn single_target(slot: u8, target: Target) -> TargetSelection {
    TargetSelection::single(TargetSlotId(slot), target)
}

fn activate_loyalty(
    game: &mut Game,
    source: GameObjectId,
    definition: CardDefinitionId,
    ability: u8,
    targets: Vec<TargetSelection>,
) {
    let action = loyalty_action(source, definition, ability, targets);
    assert!(
        game.legal_actions(PlayerId::One).contains(&action),
        "expected legal loyalty action: {action:?}"
    );
    game.apply(PlayerId::One, action).unwrap();
}

fn has_loyalty_action(game: &Game, source: GameObjectId, ability: u8) -> bool {
    game.legal_actions(PlayerId::One).iter().any(|action| {
        matches!(
            action,
            Action::ActivateAbility {
                source: actual_source,
                ability: AbilityOrigin::Printed {
                    ability: AbilityId(actual_ability),
                    ..
                },
                ..
            } if *actual_source == source && *actual_ability == ability
        )
    })
}

fn pass_priority_pair(game: &mut Game) {
    let first = game.priority;
    game.apply(first, Action::PassPriority).unwrap();
    game.apply(first.opponent(), Action::PassPriority).unwrap();
}

fn choose_cards(game: &mut Game, player: PlayerId, cards: &[GameObjectId]) {
    let pending = game
        .pending_decisions
        .first()
        .expect("a card decision is pending");
    assert_eq!(pending.observation.player, player);
    let options = cards
        .iter()
        .map(|card| {
            pending
                .observation
                .options
                .iter()
                .find(|option| option.card.is_some_and(|(candidate, _)| candidate == *card))
                .unwrap_or_else(|| panic!("{card:?} is not a decision option"))
                .id
        })
        .collect::<Vec<_>>();
    let decision = pending.observation.id;
    game.apply(player, Action::ChooseDecision { decision, options })
        .unwrap();
}

fn choose_options(game: &mut Game, player: PlayerId, options: Vec<u32>) {
    let pending = game
        .pending_decisions
        .first()
        .expect("a decision is pending");
    assert_eq!(pending.observation.player, player);
    let decision = pending.observation.id;
    game.apply(player, Action::ChooseDecision { decision, options })
        .unwrap();
}

fn fill_library(
    game: &mut Game,
    player: PlayerId,
    definition: CardDefinitionId,
    count: u32,
    first_id: u32,
) {
    game.players[player.index()]
        .library
        .extend((0..count).map(|offset| card(first_id + offset, definition, player)));
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .unwrap_or_else(|| panic!("permanent {id:?} is not on the battlefield"))
}

#[test]
fn planeswalker_spells_resolve_with_starting_loyalty_and_can_activate() {
    for (definition, starting_loyalty) in [
        (cards::JACE_MEMORY_ADEPT, 4),
        (cards::DOMRI_RADE, 3),
        (cards::LILIANA_OF_THE_VEIL, 3),
        (cards::VRASKA_THE_UNSEEN, 5),
    ] {
        let mut game = planeswalker_game();
        let walker_card = card(10_000, definition, PlayerId::One);
        let hand_id = walker_card.id;
        game.players[PlayerId::One.index()].hand.push(walker_card);
        game.players[PlayerId::One.index()].mana_pool = ManaPool {
            white: 10,
            blue: 10,
            black: 10,
            red: 10,
            green: 10,
            colorless: 10,
        };
        let cast = Action::CastSpell {
            card: hand_id,
            choices: CastChoices::default(),
            sacrifices: Vec::new(),
        };

        assert!(game.legal_actions(PlayerId::One).contains(&cast));
        game.apply(PlayerId::One, cast).unwrap();
        pass_priority_pair(&mut game);

        let walker = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == definition)
            .expect("the planeswalker spell resolved");
        assert_eq!(
            walker.counters(CounterKind::Loyalty),
            starting_loyalty,
            "{definition:?}"
        );
        assert!(has_loyalty_action(&game, walker.card.id, 0));
        let observed = game
            .observe(PlayerId::One)
            .battlefield
            .into_iter()
            .find(|permanent| permanent.id == walker.card.id)
            .expect("the planeswalker is observable");
        assert_eq!(observed.loyalty, Some(starting_loyalty));
        assert!(!observed.loyalty_ability_used_this_turn);
    }
}

#[test]
fn loyalty_activations_require_sorcery_timing_sufficient_loyalty_and_once_per_turn() {
    let mut game = planeswalker_game();
    let walker = planeswalker(10_000, cards::JACE_MEMORY_ADEPT, PlayerId::One, 4);
    let walker_id = walker.card.id;
    game.battlefield.push(walker);
    fill_library(&mut game, PlayerId::One, cards::MOUNTAIN, 1, 11_000);

    assert!(has_loyalty_action(&game, walker_id, 0));
    assert!(has_loyalty_action(&game, walker_id, 1));
    assert!(!has_loyalty_action(&game, walker_id, 2));

    game.step = Step::Upkeep;
    assert!(!has_loyalty_action(&game, walker_id, 0));
    game.step = Step::PrecombatMain;
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::One;
    assert!(!has_loyalty_action(&game, walker_id, 0));

    game.active_player = PlayerId::One;
    game.stack.push(StackObject {
        id: CardInstanceId(12_000),
        kind: StackObjectKind::Spell,
        card: card(12_000, cards::MOUNTAIN, PlayerId::Two),
        source: None,
        ability: None,
        controller: PlayerId::Two,
        signature: None,
        colors: None,
        chosen_permanents: Vec::new(),
        applied_effects: Vec::new(),
        is_copy: false,
        text_changes: Vec::new(),
        cast_via_flashback: false,
    });
    assert!(!has_loyalty_action(&game, walker_id, 0));
    game.stack.clear();

    activate_loyalty(
        &mut game,
        walker_id,
        cards::JACE_MEMORY_ADEPT,
        0,
        vec![single_target(0, Target::Player(PlayerId::Two))],
    );
    assert_eq!(
        permanent(&game, walker_id).counters(CounterKind::Loyalty),
        5
    );
    assert!(permanent(&game, walker_id).activated_loyalty_this_turn);
    pass_priority_pair(&mut game);
    assert!(!has_loyalty_action(&game, walker_id, 0));
    assert!(
        game.observe(PlayerId::One)
            .battlefield
            .iter()
            .find(|permanent| permanent.id == walker_id)
            .is_some_and(|permanent| permanent.loyalty_ability_used_this_turn)
    );

    game.extra_turns.push(PlayerId::One);
    game.start_next_turn();
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    assert!(has_loyalty_action(&game, walker_id, 0));
}

#[test]
fn jace_plus_one_draws_for_its_controller_and_mills_its_target() {
    let mut game = planeswalker_game();
    let walker = planeswalker(10_000, cards::JACE_MEMORY_ADEPT, PlayerId::One, 4);
    let walker_id = walker.card.id;
    game.battlefield.push(walker);
    fill_library(&mut game, PlayerId::One, cards::SAVANNAH_LIONS, 1, 11_000);
    fill_library(&mut game, PlayerId::Two, cards::MOUNTAIN, 1, 12_000);

    activate_loyalty(
        &mut game,
        walker_id,
        cards::JACE_MEMORY_ADEPT,
        0,
        vec![single_target(0, Target::Player(PlayerId::Two))],
    );
    pass_priority_pair(&mut game);

    assert_eq!(game.players[PlayerId::One.index()].hand.len(), 1);
    assert_eq!(
        game.players[PlayerId::One.index()].hand[0].definition,
        cards::SAVANNAH_LIONS
    );
    assert!(game.players[PlayerId::Two.index()].library.is_empty());
    assert_eq!(game.players[PlayerId::Two.index()].graveyard.len(), 1);
    assert_eq!(
        permanent(&game, walker_id).counters(CounterKind::Loyalty),
        5
    );
}

#[test]
fn jace_zero_mills_ten_and_the_ultimate_resolves_after_exact_zero_loyalty() {
    let mut game = planeswalker_game();
    let walker = planeswalker(10_000, cards::JACE_MEMORY_ADEPT, PlayerId::One, 4);
    let walker_id = walker.card.id;
    game.battlefield.push(walker);
    fill_library(&mut game, PlayerId::Two, cards::MOUNTAIN, 12, 11_000);

    activate_loyalty(
        &mut game,
        walker_id,
        cards::JACE_MEMORY_ADEPT,
        1,
        vec![single_target(0, Target::Player(PlayerId::Two))],
    );
    assert_eq!(
        permanent(&game, walker_id).counters(CounterKind::Loyalty),
        4
    );
    pass_priority_pair(&mut game);
    assert_eq!(game.players[PlayerId::Two.index()].library.len(), 2);
    assert_eq!(game.players[PlayerId::Two.index()].graveyard.len(), 10);

    let mut game = planeswalker_game();
    let walker = planeswalker(20_000, cards::JACE_MEMORY_ADEPT, PlayerId::One, 7);
    let walker_id = walker.card.id;
    game.battlefield.push(walker);
    fill_library(&mut game, PlayerId::One, cards::ISLAND, 20, 21_000);
    fill_library(&mut game, PlayerId::Two, cards::MOUNTAIN, 20, 22_000);

    activate_loyalty(
        &mut game,
        walker_id,
        cards::JACE_MEMORY_ADEPT,
        2,
        vec![TargetSelection::new(
            TargetSlotId(0),
            vec![Target::Player(PlayerId::One), Target::Player(PlayerId::Two)],
        )],
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != walker_id),
        "the zero-loyalty source leaves as a state-based action"
    );
    assert_eq!(
        game.stack.len(),
        1,
        "its activated ability remains on the stack"
    );

    pass_priority_pair(&mut game);
    assert_eq!(game.players[PlayerId::One.index()].hand.len(), 20);
    assert_eq!(game.players[PlayerId::Two.index()].hand.len(), 20);
}

#[test]
fn domri_plus_one_reveals_a_top_creature_but_not_a_noncreature() {
    let mut game = planeswalker_game();
    let walker = planeswalker(10_000, cards::DOMRI_RADE, PlayerId::One, 3);
    let walker_id = walker.card.id;
    game.battlefield.push(walker);
    let top = card(11_000, cards::LOXODON_SMITER, PlayerId::One);
    let top_id = top.id;
    game.players[PlayerId::One.index()].library.push(top);

    activate_loyalty(&mut game, walker_id, cards::DOMRI_RADE, 0, Vec::new());
    pass_priority_pair(&mut game);
    assert_eq!(game.decision_player(), Some(PlayerId::One));
    choose_cards(&mut game, PlayerId::One, &[top_id]);
    assert_eq!(game.players[PlayerId::One.index()].hand.len(), 1);
    assert_eq!(
        game.players[PlayerId::One.index()].hand[0].definition,
        cards::LOXODON_SMITER
    );

    let mut game = planeswalker_game();
    let walker = planeswalker(20_000, cards::DOMRI_RADE, PlayerId::One, 3);
    let walker_id = walker.card.id;
    game.battlefield.push(walker);
    game.players[PlayerId::One.index()]
        .library
        .push(card(21_000, cards::MOUNTAIN, PlayerId::One));

    activate_loyalty(&mut game, walker_id, cards::DOMRI_RADE, 0, Vec::new());
    pass_priority_pair(&mut game);
    assert!(game.pending_decisions.is_empty());
    assert!(game.players[PlayerId::One.index()].hand.is_empty());
    assert_eq!(game.players[PlayerId::One.index()].library.len(), 1);
}

#[test]
fn domri_minus_two_records_its_distinct_target_limitation() {
    let mut game = planeswalker_game();
    let walker = planeswalker(10_000, cards::DOMRI_RADE, PlayerId::One, 3);
    let walker_id = walker.card.id;
    let own = creature(10_001, cards::LOXODON_SMITER, PlayerId::One);
    let own_id = own.card.id;
    let opposing = creature(10_002, cards::SAVANNAH_LIONS, PlayerId::Two);
    let opposing_id = opposing.card.id;
    game.battlefield.extend([walker, own, opposing]);

    let same_creature = loyalty_action(
        walker_id,
        cards::DOMRI_RADE,
        1,
        vec![
            single_target(0, Target::Permanent(own_id)),
            single_target(1, Target::Permanent(own_id)),
        ],
    );
    assert!(
        game.legal_actions(PlayerId::One).contains(&same_creature),
        "the partial target model cannot yet require another creature"
    );

    activate_loyalty(
        &mut game,
        walker_id,
        cards::DOMRI_RADE,
        1,
        vec![
            single_target(0, Target::Permanent(own_id)),
            single_target(1, Target::Permanent(opposing_id)),
        ],
    );
    pass_priority_pair(&mut game);

    assert_eq!(permanent(&game, own_id).damage, 2);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != opposing_id)
    );
    assert_eq!(
        permanent(&game, walker_id).counters(CounterKind::Loyalty),
        1
    );
}

#[test]
fn domri_ultimate_creates_an_emblem_that_grants_all_four_keywords() {
    let mut game = planeswalker_game();
    let walker = planeswalker(10_000, cards::DOMRI_RADE, PlayerId::One, 7);
    let walker_id = walker.card.id;
    let creature = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One);
    let creature_id = creature.card.id;
    game.battlefield.extend([walker, creature]);

    activate_loyalty(&mut game, walker_id, cards::DOMRI_RADE, 2, Vec::new());
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != walker_id)
    );
    pass_priority_pair(&mut game);

    assert_eq!(game.emblems.len(), 1);
    let creature = permanent(&game, creature_id);
    assert!(game.permanent_has_executable_keyword(creature, KeywordAbility::DoubleStrike));
    assert!(game.has_trample(creature));
    assert!(game.has_hexproof(creature));
    assert!(game.permanent_has_executable_keyword(creature, KeywordAbility::Haste));
    let observed = game.observe(PlayerId::One);
    assert_eq!(observed.emblems.len(), 1);
    assert_eq!(observed.emblems[0].name, "Domri Rade emblem");
    assert_eq!(
        observed.emblems[0].source_ability,
        loyalty_origin(cards::DOMRI_RADE, 2)
    );
}

#[test]
fn liliana_plus_one_collects_both_discard_choices_before_moving_cards() {
    let mut game = planeswalker_game();
    let walker = planeswalker(10_000, cards::LILIANA_OF_THE_VEIL, PlayerId::One, 3);
    let walker_id = walker.card.id;
    game.battlefield.push(walker);
    let first = card(11_000, cards::ISLAND, PlayerId::One);
    let first_id = first.id;
    let first_other = card(11_001, cards::PLAINS, PlayerId::One);
    let second = card(12_000, cards::MOUNTAIN, PlayerId::Two);
    let second_id = second.id;
    let second_other = card(12_001, cards::SWAMP, PlayerId::Two);
    game.players[PlayerId::One.index()]
        .hand
        .extend([first, first_other]);
    game.players[PlayerId::Two.index()]
        .hand
        .extend([second, second_other]);

    activate_loyalty(
        &mut game,
        walker_id,
        cards::LILIANA_OF_THE_VEIL,
        0,
        Vec::new(),
    );
    pass_priority_pair(&mut game);
    assert_eq!(game.decision_player(), Some(PlayerId::One));
    choose_cards(&mut game, PlayerId::One, &[first_id]);
    assert_eq!(game.decision_player(), Some(PlayerId::Two));
    assert_eq!(game.players[PlayerId::One.index()].hand.len(), 2);
    assert_eq!(game.players[PlayerId::Two.index()].hand.len(), 2);
    choose_cards(&mut game, PlayerId::Two, &[second_id]);

    assert_eq!(game.players[PlayerId::One.index()].hand.len(), 1);
    assert_eq!(game.players[PlayerId::Two.index()].hand.len(), 1);
    assert_eq!(game.players[PlayerId::One.index()].graveyard.len(), 1);
    assert_eq!(game.players[PlayerId::Two.index()].graveyard.len(), 1);
}

#[test]
fn liliana_minus_two_has_the_target_player_choose_the_sacrificed_creature() {
    let mut game = planeswalker_game();
    let walker = planeswalker(10_000, cards::LILIANA_OF_THE_VEIL, PlayerId::One, 3);
    let walker_id = walker.card.id;
    let first = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    let first_id = first.card.id;
    let second = creature(10_002, cards::ATOG, PlayerId::Two);
    let second_id = second.card.id;
    game.battlefield.extend([walker, first, second]);

    activate_loyalty(
        &mut game,
        walker_id,
        cards::LILIANA_OF_THE_VEIL,
        1,
        vec![single_target(0, Target::Player(PlayerId::Two))],
    );
    pass_priority_pair(&mut game);
    assert_eq!(game.decision_player(), Some(PlayerId::Two));
    choose_cards(&mut game, PlayerId::Two, &[second_id]);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == first_id)
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != second_id)
    );
    assert_eq!(game.players[PlayerId::Two.index()].graveyard.len(), 1);
}

#[test]
fn liliana_ultimate_separates_permanents_then_the_target_chooses_a_pile() {
    let mut game = planeswalker_game();
    let walker = planeswalker(10_000, cards::LILIANA_OF_THE_VEIL, PlayerId::One, 6);
    let walker_id = walker.card.id;
    let first = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    let first_id = first.card.id;
    let second = creature(10_002, cards::ATOG, PlayerId::Two);
    let second_id = second.card.id;
    let third = creature(10_003, cards::MOUNTAIN, PlayerId::Two);
    let third_id = third.card.id;
    game.battlefield.extend([walker, first, second, third]);

    activate_loyalty(
        &mut game,
        walker_id,
        cards::LILIANA_OF_THE_VEIL,
        2,
        vec![single_target(0, Target::Player(PlayerId::Two))],
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != walker_id)
    );
    pass_priority_pair(&mut game);
    assert_eq!(game.decision_player(), Some(PlayerId::One));

    let partition = game
        .observe(PlayerId::One)
        .decision
        .expect("Liliana's controller separates the target's permanents");
    assert_eq!(partition.preference, DecisionPreference::BalancedPartition);
    assert!(
        partition
            .options
            .iter()
            .all(|option| option.members.is_empty())
    );

    choose_cards(&mut game, PlayerId::One, &[first_id, second_id]);
    assert_eq!(game.decision_player(), Some(PlayerId::Two));

    let pile_choice = game
        .observe(PlayerId::Two)
        .decision
        .expect("the target player chooses which pile to sacrifice");
    assert_eq!(pile_choice.preference, DecisionPreference::LowerCardValue);
    assert_eq!(pile_choice.options.len(), 2);
    assert!(
        pile_choice
            .options
            .iter()
            .all(|option| option.card.is_none())
    );
    assert_eq!(
        pile_choice.options[0]
            .members
            .iter()
            .map(|(object, _)| *object)
            .collect::<Vec<_>>(),
        vec![first_id, second_id],
    );
    assert_eq!(
        pile_choice.options[1]
            .members
            .iter()
            .map(|(object, _)| *object)
            .collect::<Vec<_>>(),
        vec![third_id],
    );

    choose_options(&mut game, PlayerId::Two, vec![0]);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != first_id)
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != second_id)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == third_id)
    );
    assert_eq!(game.players[PlayerId::Two.index()].graveyard.len(), 2);
}

#[test]
fn liliana_ultimate_allows_an_empty_pile() {
    let mut game = planeswalker_game();
    let walker = planeswalker(10_000, cards::LILIANA_OF_THE_VEIL, PlayerId::One, 6);
    let walker_id = walker.card.id;
    let first = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    let first_id = first.card.id;
    let second = creature(10_002, cards::ATOG, PlayerId::Two);
    let second_id = second.card.id;
    game.battlefield.extend([walker, first, second]);

    activate_loyalty(
        &mut game,
        walker_id,
        cards::LILIANA_OF_THE_VEIL,
        2,
        vec![single_target(0, Target::Player(PlayerId::Two))],
    );
    pass_priority_pair(&mut game);

    choose_cards(&mut game, PlayerId::One, &[]);
    let pile_choice = game
        .observe(PlayerId::Two)
        .decision
        .expect("the target player chooses between the empty and full piles");
    assert!(pile_choice.options[0].members.is_empty());
    assert_eq!(pile_choice.options[1].members.len(), 2);
    assert!(pile_choice.options[0].label.contains("Empty pile"));

    choose_options(&mut game, PlayerId::Two, vec![0]);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == first_id)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == second_id)
    );
    assert!(game.players[PlayerId::Two.index()].graveyard.is_empty());
}

#[test]
fn liliana_ultimate_respects_sigardas_forced_sacrifice_protection() {
    let mut game = planeswalker_game();
    let walker = planeswalker(10_000, cards::LILIANA_OF_THE_VEIL, PlayerId::One, 6);
    let walker_id = walker.card.id;
    let protected = creature(10_001, cards::ATOG, PlayerId::Two);
    let protected_id = protected.card.id;
    let sigarda = creature(10_002, cards::SIGARDA_HOST_OF_HERONS, PlayerId::Two);
    let sigarda_id = sigarda.card.id;
    game.battlefield.extend([walker, protected, sigarda]);

    activate_loyalty(
        &mut game,
        walker_id,
        cards::LILIANA_OF_THE_VEIL,
        2,
        vec![single_target(0, Target::Player(PlayerId::Two))],
    );
    pass_priority_pair(&mut game);

    choose_cards(&mut game, PlayerId::One, &[protected_id]);
    choose_options(&mut game, PlayerId::Two, vec![0]);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == protected_id)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == sigarda_id)
    );
    assert!(game.players[PlayerId::Two.index()].graveyard.is_empty());
}

#[test]
fn liliana_ultimate_skips_pile_decisions_when_the_target_controls_nothing() {
    let mut game = planeswalker_game();
    let walker = planeswalker(10_000, cards::LILIANA_OF_THE_VEIL, PlayerId::One, 6);
    let walker_id = walker.card.id;
    game.battlefield.push(walker);

    activate_loyalty(
        &mut game,
        walker_id,
        cards::LILIANA_OF_THE_VEIL,
        2,
        vec![single_target(0, Target::Player(PlayerId::Two))],
    );
    pass_priority_pair(&mut game);

    assert!(game.pending_decisions.is_empty());
    assert!(game.stack.is_empty());
    assert!(game.players[PlayerId::Two.index()].graveyard.is_empty());
}

#[test]
fn liliana_ultimate_policy_prefers_the_opponent_and_the_valuable_ultimate() {
    let mut game = planeswalker_game();
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    let walker = planeswalker(10_000, cards::LILIANA_OF_THE_VEIL, PlayerId::Two, 6);
    let walker_id = walker.card.id;
    game.battlefield.push(walker);
    game.battlefield
        .extend((0_u32..6).map(|offset| creature(10_001 + offset, cards::ATOG, PlayerId::One)));

    let observation = game.observe(PlayerId::Two);
    let ultimate_actions = observation
        .legal_actions
        .iter()
        .filter(|action| {
            matches!(
                action,
                Action::ActivateAbility {
                    source,
                    ability: AbilityOrigin::Printed {
                        definition: cards::LILIANA_OF_THE_VEIL,
                        ability: AbilityId(2),
                        ..
                    },
                    ..
                } if *source == walker_id
            )
        })
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(ultimate_actions.len(), 2);

    let expected = loyalty_action(
        walker_id,
        cards::LILIANA_OF_THE_VEIL,
        2,
        vec![single_target(0, Target::Player(PlayerId::One))],
    );
    let mut policy = HandcraftedPolicy::new(game.catalog.clone());
    let mut ultimate_only = observation.clone();
    ultimate_only.legal_actions = ultimate_actions;
    assert_eq!(policy.choose_action(&ultimate_only), Some(expected.clone()));
    assert_eq!(policy.choose_action(&observation), Some(expected));
}

#[test]
fn vraska_plus_one_retaliates_against_combat_damage_until_the_next_turn() {
    let mut game = planeswalker_game();
    let walker = planeswalker(10_000, cards::VRASKA_THE_UNSEEN, PlayerId::One, 5);
    let walker_id = walker.card.id;
    game.battlefield.push(walker);

    activate_loyalty(
        &mut game,
        walker_id,
        cards::VRASKA_THE_UNSEEN,
        0,
        Vec::new(),
    );
    pass_priority_pair(&mut game);
    assert_eq!(
        permanent(&game, walker_id)
            .temporary_granted_abilities
            .len(),
        1
    );

    let mut attacker = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    let attacker_id = attacker.card.id;
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Planeswalker(walker_id));
    game.battlefield.push(attacker);
    game.deal_combat_damage();
    game.finish_rules_procedure();

    assert_eq!(
        permanent(&game, walker_id).counters(CounterKind::Loyalty),
        4
    );
    assert_eq!(game.stack.len(), 1);
    assert_eq!(
        game.stack.last().unwrap().kind,
        StackObjectKind::TriggeredAbility
    );
    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != attacker_id)
    );

    game.start_next_turn();
    assert_eq!(
        permanent(&game, walker_id)
            .temporary_granted_abilities
            .len(),
        1
    );
    game.start_next_turn();
    assert!(
        permanent(&game, walker_id)
            .temporary_granted_abilities
            .is_empty()
    );
}

#[test]
fn vraska_minus_three_destroys_nonlands_but_cannot_target_lands() {
    let mut game = planeswalker_game();
    let walker = planeswalker(10_000, cards::VRASKA_THE_UNSEEN, PlayerId::One, 5);
    let walker_id = walker.card.id;
    let nonland = creature(10_001, cards::SOL_RING, PlayerId::Two);
    let nonland_id = nonland.card.id;
    let land = creature(10_002, cards::MOUNTAIN, PlayerId::Two);
    let land_id = land.card.id;
    game.battlefield.extend([walker, nonland, land]);

    let target_land = loyalty_action(
        walker_id,
        cards::VRASKA_THE_UNSEEN,
        1,
        vec![single_target(0, Target::Permanent(land_id))],
    );
    assert!(!game.legal_actions(PlayerId::One).contains(&target_land));
    activate_loyalty(
        &mut game,
        walker_id,
        cards::VRASKA_THE_UNSEEN,
        1,
        vec![single_target(0, Target::Permanent(nonland_id))],
    );
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != nonland_id)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == land_id)
    );
    assert_eq!(
        permanent(&game, walker_id).counters(CounterKind::Loyalty),
        2
    );
}

#[test]
fn vraska_ultimate_creates_assassins_whose_combat_damage_makes_a_player_lose() {
    let mut game = planeswalker_game();
    let walker = planeswalker(10_000, cards::VRASKA_THE_UNSEEN, PlayerId::One, 7);
    let walker_id = walker.card.id;
    game.battlefield.push(walker);

    activate_loyalty(
        &mut game,
        walker_id,
        cards::VRASKA_THE_UNSEEN,
        2,
        Vec::new(),
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != walker_id)
    );
    assert_eq!(game.stack.len(), 1);
    pass_priority_pair(&mut game);

    let assassin_ids = game
        .battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == cards::ASSASSIN_TOKEN_1_1_BLACK)
        .map(|permanent| permanent.card.id)
        .collect::<Vec<_>>();
    assert_eq!(assassin_ids.len(), 3);
    for assassin in &assassin_ids {
        let assassin = permanent(&game, *assassin);
        assert!(game.is_token(assassin.card.definition));
        let rules = game
            .effective_rules(assassin)
            .expect("the Assassin token has cataloged rules");
        assert_eq!(rules.colors(), [false, false, true, false, false]);
        assert!(rules.has_subtype("Assassin"));
        assert_eq!(game.power(assassin), Some(1));
        assert_eq!(game.toughness(assassin), Some(1));
    }

    let attacker_id = assassin_ids[0];
    let attacker = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == attacker_id)
        .unwrap();
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    game.deal_combat_damage();
    game.finish_rules_procedure();
    assert_eq!(game.players[PlayerId::Two.index()].life, 19);
    assert_eq!(game.stack.len(), 1);

    pass_priority_pair(&mut game);
    assert_eq!(
        game.result(),
        Some(GameResult::Winner {
            winner: PlayerId::One,
            reason: WinReason::OpponentLostToAnEffect,
        })
    );
}
