//! Partial coverage left behind after its shared rules primitive arrived.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

fn settle_choosing_last(game: &mut Game) {
    for _ in 0..24 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .last()
                .map(|option| vec![option.id])
                .unwrap_or_default();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the last offered choice is legal");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            return;
        }
        let priority = game.priority;
        game.apply(priority, Action::PassPriority)
            .expect("priority advances");
    }
    panic!("the game did not settle");
}

#[test]
fn cyclopean_mummy_exiles_the_death_events_graveyard_object() {
    let mut game = ready();
    let mummy = creature(10_000, cards::CYCLOPEAN_MUMMY, PlayerId::One);
    let mummy_id = mummy.card.id;
    game.battlefield.push(mummy);

    game.destroy_permanent(mummy_id);
    drain_pending(&mut game);

    assert!(
        game.players[0]
            .graveyard
            .iter()
            .all(|card| card.definition != cards::CYCLOPEAN_MUMMY),
        "the dies trigger used the destination object recorded by its event",
    );
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::CYCLOPEAN_MUMMY),
    );
}

#[test]
fn restoration_angel_keeps_a_stolen_creature() {
    let mut game = ready();
    let mut stolen = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    stolen.controller = PlayerId::One;
    game.battlefield.push(stolen);

    game.enqueue_battlefield_entry(PendingBattlefieldEntry {
        permanent: creature(10_001, cards::RESTORATION_ANGEL, PlayerId::One),
        from: ZoneKind::Hand,
        completion: EntryCompletion::None,
        redirected_to: None,
    });
    settle_choosing_last(&mut game);

    let returned = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::GRIZZLY_BEARS)
        .expect("the bear returned");
    assert_eq!(returned.controller, PlayerId::One);
    assert_eq!(returned.card.owner, PlayerId::Two);
}

#[test]
fn treasured_find_returns_its_target_and_exiles_itself() {
    let mut game = ready();
    let target = card(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let target_id = target.id;
    game.players[0].graveyard.push(target);
    let find = card(10_001, cards::TREASURED_FIND, PlayerId::One);
    let find_id = find.id;
    game.players[0].hand.push(find);
    game.players[0].mana_pool.black = 1;
    game.players[0].mana_pool.green = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == find_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Card(target_id))
            }
            _ => false,
        })
        .expect("the graveyard card is a legal target");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    drain_pending(&mut game);

    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::GRIZZLY_BEARS),
    );
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::TREASURED_FIND),
    );
}

#[test]
fn increasing_ambition_searches_for_two_via_flashback() {
    let mut game = ready();
    let ambition = card(10_000, cards::INCREASING_AMBITION, PlayerId::One);
    let ambition_id = ambition.id;
    game.players[0].graveyard.push(ambition);
    game.players[0].library.extend([
        card(10_001, cards::FOREST, PlayerId::One),
        card(10_002, cards::ISLAND, PlayerId::One),
        card(10_003, cards::MOUNTAIN, PlayerId::One),
    ]);
    game.players[0].mana_pool.black = 1;
    game.players[0].mana_pool.colorless = 7;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == ambition_id))
        .expect("printed flashback offers the spell");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    pass_priority_pair(&mut game);

    let decision = game.observe(PlayerId::One).decision.expect("search choice");
    assert_eq!((decision.minimum, decision.maximum), (2, 2));
    let choices = decision
        .options
        .iter()
        .take(2)
        .map(|option| option.id)
        .collect();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: choices,
        },
    )
    .expect("two cards may be chosen");

    assert_eq!(game.players[0].hand.len(), 2);
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::INCREASING_AMBITION),
    );
}

#[test]
fn increasing_devotion_uses_cast_origin_for_its_single_token_amount() {
    fn created(from: ZoneKind) -> usize {
        let mut game = ready();
        let devotion = card(10_000, cards::INCREASING_DEVOTION, PlayerId::One);
        let devotion_id = devotion.id;
        match from {
            ZoneKind::Hand => game.players[0].hand.push(devotion),
            ZoneKind::Graveyard => game.players[0].graveyard.push(devotion),
            _ => unreachable!("the test only casts from hand or graveyard"),
        }
        game.players[0].mana_pool.white = 2;
        game.players[0].mana_pool.colorless = if from == ZoneKind::Hand { 3 } else { 7 };

        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == devotion_id))
            .expect("the appropriate cast is offered");
        game.apply(PlayerId::One, action)
            .expect("the spell is cast");
        drain_pending(&mut game);
        game.battlefield.len()
    }

    assert_eq!(created(ZoneKind::Hand), 5);
    assert_eq!(created(ZoneKind::Graveyard), 10);
}

#[test]
fn rummaging_goblin_discards_as_a_cost_then_draws() {
    let mut game = ready();
    let goblin = creature(10_000, cards::RUMMAGING_GOBLIN, PlayerId::One);
    let goblin_id = goblin.card.id;
    game.battlefield.push(goblin);
    game.players[0]
        .hand
        .push(card(10_001, cards::MOUNTAIN, PlayerId::One));
    game.players[0]
        .library
        .push(card(10_002, cards::FOREST, PlayerId::One));

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == goblin_id))
        .expect("one card can pay the discard cost");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    assert!(game.players[0].hand.is_empty(), "the discard is paid now");
    drain_pending(&mut game);

    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.players[0].hand[0].definition, cards::FOREST);
    assert!(game.battlefield[0].tapped);
}

#[test]
fn viashino_racketeer_draws_only_after_the_optional_discard() {
    let mut game = ready();
    game.players[0]
        .hand
        .push(card(10_001, cards::MOUNTAIN, PlayerId::One));
    game.players[0]
        .library
        .push(card(10_002, cards::FOREST, PlayerId::One));

    game.enqueue_battlefield_entry(PendingBattlefieldEntry {
        permanent: creature(10_000, cards::VIASHINO_RACKETEER, PlayerId::One),
        from: ZoneKind::Hand,
        completion: EntryCompletion::None,
        redirected_to: None,
    });
    settle_choosing_last(&mut game);

    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.players[0].hand[0].definition, cards::FOREST);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::MOUNTAIN),
    );
}

#[test]
fn quag_sickness_tracks_its_controllers_swamps() {
    let mut game = ready();
    let victim = creature(10_000, cards::SERRA_ANGEL, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.push(victim);
    let swamp_one = creature(10_001, cards::SWAMP, PlayerId::One);
    let swamp_one_id = swamp_one.card.id;
    game.battlefield.push(swamp_one);
    game.battlefield
        .push(creature(10_002, cards::SWAMP, PlayerId::One));
    let mut aura = creature(10_003, cards::QUAG_SICKNESS, PlayerId::One);
    aura.attached_to = Some(victim_id);
    game.battlefield.push(aura);

    let victim = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == victim_id)
        .expect("still there");
    assert_eq!(
        (game.power(victim), game.toughness(victim)),
        (Some(2), Some(2))
    );

    game.destroy_permanent(swamp_one_id);
    let victim = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == victim_id)
        .expect("still there");
    assert_eq!(
        (game.power(victim), game.toughness(victim)),
        (Some(3), Some(3))
    );
}

#[test]
fn awaken_the_ancient_animates_without_erasing_the_land() {
    let mut game = ready();
    let mountain = creature(10_000, cards::MOUNTAIN, PlayerId::One);
    let mountain_id = mountain.card.id;
    game.battlefield.push(mountain);
    let mut aura = creature(10_001, cards::AWAKEN_THE_ANCIENT, PlayerId::One);
    let aura_id = aura.card.id;
    aura.attached_to = Some(mountain_id);
    game.battlefield.push(aura);

    let mountain = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == mountain_id)
        .expect("still there");
    let types = game.permanent_types(mountain).expect("effective types");
    assert!(types.contains(CardType::Land));
    assert!(types.contains(CardType::Creature));
    let subtypes = game.effective_subtypes(mountain);
    assert!(subtypes.contains(&"Mountain"));
    assert!(subtypes.contains(&"Giant"));
    let colors = game.permanent_colors(mountain);
    assert_eq!(colors.iter().filter(|is_color| **is_color).count(), 1);
    assert!(colors[ManaColor::Red.color_index().expect("red is a color")]);
    assert_eq!(
        (game.power(mountain), game.toughness(mountain)),
        (Some(7), Some(7))
    );
    assert!(game.permanent_has_executable_keyword(mountain, KeywordAbility::Haste));

    game.destroy_permanent(aura_id);
    let mountain = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == mountain_id)
        .expect("still there");
    assert_eq!(game.power(mountain), None);
    assert!(
        game.permanent_types(mountain)
            .is_some_and(|types| types.contains(CardType::Land)),
    );
}
