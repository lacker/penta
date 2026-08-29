//! Cards whose printed abilities function while the card is in a graveyard.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    game.players[0].graveyard.clear();
    game.players[1].graveyard.clear();
    game.players[0].exile.clear();
    game.players[1].exile.clear();
    game.turn = 5;
    game.turns_started = [5, 4];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

fn put_in_graveyard(
    game: &mut Game,
    id: u32,
    definition: CardDefinitionId,
    owner: PlayerId,
) -> GameObjectId {
    let dead = card(id, definition, owner);
    let object = dead.id;
    game.players[owner.index()].graveyard.push(dead);
    object
}

fn permanent(game: &Game, definition: CardDefinitionId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == definition)
        .unwrap_or_else(|| panic!("definition {definition:?} is on the battlefield"))
}

fn zombie_count(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Zombie"], &[ManaColor::Black], 2, 2),
            )
        })
        .count()
}

fn drain_accepting_optional_effects(game: &mut Game) {
    for _ in 0..24 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            return;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = if decision
                .options
                .iter()
                .any(|option| option.label == "Decline")
            {
                decision
                    .options
                    .iter()
                    .filter(|option| option.label != "Decline")
                    .take(decision.maximum)
                    .map(|option| option.id)
                    .collect()
            } else {
                decision
                    .options
                    .iter()
                    .take(decision.minimum.max(1).min(decision.maximum))
                    .map(|option| option.id)
                    .collect()
            };
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .unwrap();
            continue;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority).unwrap();
    }
}

#[test]
fn dearly_departed_grants_one_entry_replacement_per_copy_in_the_graveyard() {
    let mut game = ready();
    put_in_graveyard(&mut game, 70_000, cards::DEARLY_DEPARTED, PlayerId::One);
    put_in_graveyard(&mut game, 70_001, cards::DEARLY_DEPARTED, PlayerId::One);
    let human = card(70_002, cards::DOOMED_TRAVELER, PlayerId::One);
    let human_id = human.id;
    game.players[0].hand.push(human);

    game.move_target_to_zone(
        Target::Card(human_id),
        ZoneKind::Battlefield,
        ZoneMoveCause::Effect {
            controller: PlayerId::One,
        },
        Some(BattlefieldArrival::under(PlayerId::One)),
        ZonePlacement::Top,
    );
    drain_pending(&mut game);

    assert_eq!(
        permanent(&game, cards::DOOMED_TRAVELER).counters(CounterKind::PlusOnePlusOne),
        2,
        "each graveyard copy grants its own additional-counter replacement",
    );
}

#[test]
fn riftstone_portal_grants_and_withdraws_the_two_color_mana_ability() {
    let mut game = ready();
    put_in_graveyard(&mut game, 70_010, cards::RIFTSTONE_PORTAL, PlayerId::One);
    let land = creature(70_011, cards::MOUNTAIN, PlayerId::One);
    let land_id = land.card.id;
    game.battlefield.push(land);

    let offered = |game: &Game, color| {
        game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(
                action,
                Action::ActivateManaAbility { source, color: made, .. }
                    if *source == land_id && *made == color
            )
        })
    };
    assert!(offered(&game, ManaColor::Green));
    assert!(offered(&game, ManaColor::White));

    game.players[0].graveyard.clear();
    assert!(!offered(&game, ManaColor::Green));
    assert!(!offered(&game, ManaColor::White));
}

#[test]
fn reassembling_skeleton_activates_from_the_graveyard_and_returns_tapped() {
    let mut game = ready();
    let skeleton = put_in_graveyard(
        &mut game,
        70_020,
        cards::REASSEMBLING_SKELETON,
        PlayerId::One,
    );
    game.players[0].mana_pool.black = 1;
    game.players[0].mana_pool.colorless = 1;

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == skeleton)
        })
        .expect("the graveyard activation is offered");
    game.apply(PlayerId::One, activation).unwrap();
    drain_pending(&mut game);

    assert!(permanent(&game, cards::REASSEMBLING_SKELETON).tapped);
    assert!(game.players[0].graveyard.is_empty());
}

#[test]
fn an_exact_source_reference_does_not_follow_a_card_out_of_the_graveyard() {
    let mut game = ready();
    let skeleton = put_in_graveyard(
        &mut game,
        70_025,
        cards::REASSEMBLING_SKELETON,
        PlayerId::One,
    );
    game.players[0].mana_pool.black = 1;
    game.players[0].mana_pool.colorless = 1;
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == skeleton)
        })
        .expect("the graveyard activation is offered");
    game.apply(PlayerId::One, activation).unwrap();

    game.move_target_to_zone(
        Target::Card(skeleton),
        ZoneKind::Exile,
        ZoneMoveCause::Effect {
            controller: PlayerId::Two,
        },
        None,
        ZonePlacement::Top,
    );
    drain_pending(&mut game);

    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::REASSEMBLING_SKELETON),
        "the resolving move cannot follow the card into exile",
    );
    assert!(game.battlefield.is_empty());
}

#[test]
fn nether_shadow_counts_only_creature_cards_above_it() {
    let mut game = ready();
    put_in_graveyard(&mut game, 70_030, cards::NETHER_SHADOW, PlayerId::One);
    put_in_graveyard(&mut game, 70_031, cards::SAVANNAH_LIONS, PlayerId::One);
    put_in_graveyard(&mut game, 70_032, cards::LIGHTNING_BOLT, PlayerId::One);
    put_in_graveyard(&mut game, 70_033, cards::GRIZZLY_BEARS, PlayerId::One);
    game.step = Step::Upkeep;

    game.handle_upkeep_triggers();
    assert!(
        game.pending_triggers.is_empty(),
        "two creatures are not enough"
    );

    put_in_graveyard(&mut game, 70_034, cards::SEDGE_TROLL, PlayerId::One);
    game.handle_upkeep_triggers();
    drain_accepting_optional_effects(&mut game);
    assert_eq!(
        permanent(&game, cards::NETHER_SHADOW).controller,
        PlayerId::One
    );
}

#[test]
fn ichorid_exiles_another_black_creature_then_sacrifices_itself_at_end_step() {
    let mut game = ready();
    put_in_graveyard(&mut game, 70_040, cards::ICHORID, PlayerId::One);
    put_in_graveyard(&mut game, 70_041, cards::BLACK_KNIGHT, PlayerId::One);
    put_in_graveyard(&mut game, 70_042, cards::SAVANNAH_LIONS, PlayerId::One);
    game.step = Step::Upkeep;

    game.handle_upkeep_triggers();
    drain_accepting_optional_effects(&mut game);
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::BLACK_KNIGHT),
        "the chosen black creature paid the return",
    );
    assert_eq!(permanent(&game, cards::ICHORID).controller, PlayerId::One);

    game.step = Step::End;
    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::End,
        player: PlayerId::One,
    });
    drain_pending(&mut game);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::ICHORID),
        "the ordinary battlefield trigger sacrifices the returned Ichorid",
    );
}

#[test]
fn bridge_from_below_listens_only_when_it_was_already_in_the_graveyard() {
    let mut game = ready();
    put_in_graveyard(&mut game, 70_050, cards::BRIDGE_FROM_BELOW, PlayerId::One);
    let own = creature(70_051, cards::GRIZZLY_BEARS, PlayerId::One);
    let own_id = own.card.id;
    game.battlefield.push(own);
    game.destroy_permanent(own_id);
    drain_pending(&mut game);
    assert_eq!(zombie_count(&game), 1);

    let opposing = creature(70_052, cards::SEDGE_TROLL, PlayerId::Two);
    let opposing_id = opposing.card.id;
    game.battlefield.push(opposing);
    game.destroy_permanent(opposing_id);
    drain_pending(&mut game);
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::BRIDGE_FROM_BELOW),
        "an opposing creature death exiles the Bridge",
    );

    let mut simultaneous = ready();
    let bridge = creature(70_053, cards::BRIDGE_FROM_BELOW, PlayerId::One);
    let bridge_id = bridge.card.id;
    let victim = creature(70_054, cards::GRIZZLY_BEARS, PlayerId::One);
    let victim_id = victim.card.id;
    simultaneous.battlefield.extend([bridge, victim]);
    simultaneous.move_permanents_to_graveyard(&[bridge_id, victim_id]);
    drain_pending(&mut simultaneous);
    assert_eq!(
        zombie_count(&simultaneous),
        0,
        "arriving in the same death batch is too late to listen",
    );
}

#[test]
fn bloodghast_returns_for_landfall_and_reads_an_opponents_life_for_haste() {
    let mut game = ready();
    put_in_graveyard(&mut game, 70_060, cards::BLOODGHAST, PlayerId::One);
    let land = card(70_061, cards::MOUNTAIN, PlayerId::One);
    let land_id = land.id;
    game.players[0].hand.push(land);
    let play = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == land_id))
        .expect("the land can be played");
    game.apply(PlayerId::One, play).unwrap();
    drain_accepting_optional_effects(&mut game);

    let bloodghast = permanent(&game, cards::BLOODGHAST);
    assert!(!game.permanent_has_executable_keyword(bloodghast, KeywordAbility::Haste));
    game.players[1].life = 10;
    let bloodghast = permanent(&game, cards::BLOODGHAST);
    assert!(game.permanent_has_executable_keyword(bloodghast, KeywordAbility::Haste));

    let bloodghast_id = bloodghast.card.id;
    let mut attacker = creature(70_062, cards::SEDGE_TROLL, PlayerId::Two);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::One));
    game.battlefield.push(attacker);
    game.active_player = PlayerId::Two;
    game.step = Step::DeclareBlockers;
    game.priority = PlayerId::One;
    assert!(game.legal_actions(PlayerId::One).iter().all(|action| {
        !matches!(action, Action::DeclareBlocker { blocker, .. } if *blocker == bloodghast_id)
    }));
}

fn cast_think_twice(game: &mut Game, id: u32) {
    let spell = card(id, cards::THINK_TWICE, PlayerId::One);
    let spell_id = spell.id;
    game.players[0].hand.push(spell);
    game.players[0].mana_pool.blue += 1;
    game.players[0].mana_pool.colorless += 1;
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("Think Twice can be cast");
    game.apply(PlayerId::One, cast).unwrap();
    drain_pending(game);
}

fn cast_savannah_lions(game: &mut Game, id: u32) {
    let spell = card(id, cards::SAVANNAH_LIONS, PlayerId::One);
    let spell_id = spell.id;
    game.players[0].hand.push(spell);
    game.players[0].mana_pool.white += 1;
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("Savannah Lions can be cast");
    game.apply(PlayerId::One, cast).unwrap();
    drain_pending(game);
}

#[test]
fn arclight_phoenix_counts_actual_instant_and_sorcery_casts() {
    let mut game = ready();
    put_in_graveyard(&mut game, 70_070, cards::ARCLIGHT_PHOENIX, PlayerId::One);
    cast_think_twice(&mut game, 70_071);
    cast_think_twice(&mut game, 70_072);
    cast_savannah_lions(&mut game, 70_074);
    assert_eq!(game.spell_cast_history_this_turn.len(), 3);

    game.advance_step();
    drain_pending(&mut game);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::ARCLIGHT_PHOENIX),
        "a creature spell does not contribute to the filtered cast count",
    );

    game.step = Step::PrecombatMain;
    cast_think_twice(&mut game, 70_073);
    game.advance_step();
    drain_pending(&mut game);
    assert_eq!(
        permanent(&game, cards::ARCLIGHT_PHOENIX).controller,
        PlayerId::One
    );
}
