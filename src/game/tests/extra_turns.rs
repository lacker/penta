use super::*;

const TIME_VAULT_TURN_REPLACEMENT_TEXT: &str = "If you would begin your turn while this artifact is tapped, you may skip that turn instead. If you do, untap this artifact.";
static TEST_EXTRA_TURN_REPLACEMENT_ABILITIES: [AbilityDef; 1] = [AbilityDef::replacement_for(
    "If a player would begin an extra turn, that player skips that turn instead.",
    ReplacementEventDef::WouldBeginTurn {
        player: PlayerRelation::Any,
        kind: TurnKindDef::Extra,
    },
    ReplacementEffectDef::ReplaceEventWithNothing,
)];

fn install_extra_turn_replacement(game: &mut Game, id: u32) -> GameObjectId {
    let definition_id = CardDefinitionId::new(10_064);
    let mut definition = CardDefinition::new(
        definition_id,
        "Extra Turn Suppressor",
        CardSet::Magic2014,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = CardRules::new_artifact(ManaCost::new(0, 0))
        .with_abilities(&TEST_EXTRA_TURN_REPLACEMENT_ABILITIES);
    synchronize_single_part_definition(&mut definition);
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let source = creature(id, definition_id, PlayerId::One);
    let source_id = source.card.id;
    game.battlefield.push(source);
    source_id
}

fn tapped_time_vault(id: u32, controller: PlayerId) -> Permanent {
    let mut vault = creature(id, cards::TIME_VAULT, controller);
    vault.tapped = true;
    vault
}

fn assert_time_vault_begin_turn_decision(
    game: &Game,
    player: PlayerId,
    vaults: &[GameObjectId],
) -> DecisionObservation {
    let decision = game
        .observe(player)
        .decision
        .expect("the prospective player chooses whether to skip the turn");
    assert_eq!(decision.player, player);
    assert_eq!(decision.kind, DecisionKind::Choice);
    assert_eq!(decision.order_semantics, None);
    assert_eq!(decision.prompt, "A turn would begin");
    assert_eq!(decision.visibility, DecisionVisibility::Public);
    assert_eq!(decision.preference, DecisionPreference::PreferOption(0));
    assert_eq!((decision.minimum, decision.maximum), (1, 1));
    assert!(!decision.cancellable);
    assert_eq!(decision.options.len(), vaults.len() + 1);

    let begin = &decision.options[0];
    assert_eq!(begin.id, 0);
    assert_eq!(begin.label, "Begin the turn");
    assert_eq!(begin.card, None);
    assert!(begin.members.is_empty());
    assert_eq!(begin.ability_text, None);
    assert_eq!(begin.zone, DecisionZone::None);

    for (index, (option, vault)) in decision.options[1..].iter().zip(vaults).enumerate() {
        assert_eq!(option.id, u32::try_from(index + 1).unwrap());
        assert_eq!(option.label, "Apply Time Vault's replacement effect");
        assert_eq!(
            option.card,
            Some((
                *vault,
                ObjectCharacteristics::card(cards::TIME_VAULT, CardPartId::PRIMARY),
            )),
        );
        assert!(option.members.is_empty());
        assert_eq!(
            option.ability_text.as_deref(),
            Some(TIME_VAULT_TURN_REPLACEMENT_TEXT)
        );
        assert_eq!(option.zone, DecisionZone::Battlefield);
    }

    assert_eq!(
        game.observe(player.opponent()).decision,
        Some(decision.clone()),
        "the prospective-turn choice is public"
    );
    decision
}

fn choose_begin_turn_option(game: &mut Game, player: PlayerId, option: u32) {
    let decision = game
        .observe(player)
        .decision
        .expect("a begin-turn decision is pending");
    game.apply(
        player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the begin-turn choice is legal");
}

fn choose_begin_turn_source(game: &mut Game, player: PlayerId, source: GameObjectId) {
    let decision = game
        .observe(player)
        .decision
        .expect("a begin-turn decision is pending");
    let option = decision
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(object, _)| object == source))
        .expect("the replacement source is offered")
        .id;
    choose_begin_turn_option(game, player, option);
}

fn step_changed_count(game: &Game) -> usize {
    game.events
        .iter()
        .filter(|event| matches!(event, GameEvent::StepChanged { .. }))
        .count()
}

#[test]
fn time_walk_queues_an_extra_turn() {
    let mut game = ready_game();
    let time_walk = card(10_000, cards::TIME_WALK, PlayerId::One);
    game.players[0].hand.push(time_walk.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;
    game.apply(
        PlayerId::One,
        cast_action(time_walk.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    assert!(
        game.extra_turns.is_empty(),
        "the spell still uses the stack"
    );
    pass_priority_pair(&mut game);
    assert_eq!(game.extra_turns, vec![PlayerId::One]);

    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::One);
    assert_eq!(game.observe(PlayerId::One).active_turn, 2);
    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(game.observe(PlayerId::Two).active_turn, 1);
}

#[test]
fn time_vault_decline_is_offered_before_the_turn_is_committed() {
    let mut game = ready_game();
    game.step = Step::Cleanup;
    let vault = tapped_time_vault(10_000, PlayerId::Two);
    let vault_id = vault.card.id;
    game.battlefield.push(vault);
    let turn_before = game.turn;
    let turns_started_before = game.turns_started;
    let step_changes_before = step_changed_count(&game);

    game.start_next_turn();

    for viewer in [PlayerId::One, PlayerId::Two] {
        let observation = game.observe(viewer);
        assert_eq!(observation.turn, turn_before);
        assert_eq!(observation.active_player, PlayerId::One);
        assert_eq!(observation.step, Step::Cleanup);
    }
    assert_eq!(game.turns_started, turns_started_before);
    assert_eq!(step_changed_count(&game), step_changes_before);
    assert!(game.battlefield[0].tapped);
    assert_time_vault_begin_turn_decision(&game, PlayerId::Two, &[vault_id]);
    let checkpoint = game.checkpoint_json(PlayerId::Two);
    assert!(!checkpoint["decisionState"].is_null());
    assert_eq!(
        checkpoint["hasDeferredState"], false,
        "the typed prospective-turn continuation should reconstruct"
    );

    choose_begin_turn_option(&mut game, PlayerId::Two, 0);

    assert_eq!(game.turn, turn_before + 1);
    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(game.step, Step::Upkeep);
    assert_eq!(
        game.turns_started,
        [turns_started_before[0], turns_started_before[1] + 1]
    );
    assert!(
        game.battlefield[0].tapped,
        "declining does not untap Time Vault, including during the ordinary untap procedure"
    );
    assert_eq!(step_changed_count(&game), step_changes_before + 1);
    assert!(game.pending_decisions.is_empty());
}

#[test]
fn cleanup_discard_suspends_before_the_next_turn_and_resumes_once() {
    let mut game = ready_game();
    game.step = Step::End;
    let vault = tapped_time_vault(10_000, PlayerId::Two);
    let vault_id = vault.card.id;
    game.battlefield.push(vault);
    for id in 10_001..10_009 {
        game.players[0]
            .hand
            .push(card(id, cards::MOUNTAIN, PlayerId::One));
    }

    pass_priority_pair(&mut game);
    assert!(game.cleanup_pending);
    assert_eq!(game.step, Step::Cleanup);
    let cleanup_step_changes = step_changed_count(&game);
    let discard = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::DiscardCards { .. }))
        .expect("cleanup offers the required discard");
    game.apply(PlayerId::One, discard).unwrap();

    assert!(!game.cleanup_pending);
    assert_eq!(game.turn, 1);
    assert_eq!(game.active_player, PlayerId::One);
    assert_eq!(game.step, Step::Cleanup);
    assert_eq!(step_changed_count(&game), cleanup_step_changes);
    assert_time_vault_begin_turn_decision(&game, PlayerId::Two, &[vault_id]);

    choose_begin_turn_option(&mut game, PlayerId::Two, 0);

    assert_eq!(game.turn, 2);
    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(game.step, Step::Upkeep);
    assert_eq!(step_changed_count(&game), cleanup_step_changes + 1);
}

#[test]
fn accepting_time_vault_skips_the_regular_proposal_and_untaps_only_the_source() {
    let mut game = ready_game();
    game.step = Step::Cleanup;
    let vault = tapped_time_vault(10_000, PlayerId::Two);
    let vault_id = vault.card.id;
    game.battlefield.push(vault);
    game.battlefield
        .push(creature(10_001, cards::MANA_VAULT, PlayerId::Two));
    game.spells_cast_this_turn = [2, 3];
    let turn_before = game.turn;
    let turns_started_before = game.turns_started;

    game.start_next_turn();
    assert_time_vault_begin_turn_decision(&game, PlayerId::Two, &[vault_id]);
    choose_begin_turn_option(&mut game, PlayerId::Two, 1);

    assert_eq!(game.turn, turn_before + 1, "a skipped turn never began");
    assert_eq!(
        game.turns_started,
        [turns_started_before[0] + 1, turns_started_before[1]],
        "Player Two's skipped turn is not counted; Player One begins the following turn"
    );
    assert_eq!(game.active_player, PlayerId::One);
    assert_eq!(game.step, Step::Upkeep);
    assert!(!game.battlefield[0].tapped);
    assert_eq!(
        game.spells_cast_last_turn,
        [2, 3],
        "per-turn state rolls over once for the turn that actually begins"
    );
    assert_eq!(game.spells_cast_this_turn, [0, 0]);
    assert!(
        game.pending_triggers.is_empty(),
        "the skipped Player Two upkeep never triggers Mana Vault"
    );
    assert!(game.pending_decisions.is_empty());
}

#[test]
fn time_vault_untap_waits_for_the_next_turn_that_actually_begins() {
    let mut game = ready_game();
    game.step = Step::Cleanup;
    let vault = tapped_time_vault(10_000, PlayerId::Two);
    let vault_id = vault.card.id;
    game.battlefield.push(vault);
    game.extra_turns = vec![PlayerId::Two, PlayerId::Two];
    let turn_before = game.turn;

    game.start_next_turn();
    assert_time_vault_begin_turn_decision(&game, PlayerId::Two, &[vault_id]);
    choose_begin_turn_source(&mut game, PlayerId::Two, vault_id);

    assert_eq!(game.turn, turn_before, "the first extra turn was skipped");
    assert!(
        game.battlefield[0].tapped,
        "CR 614.10b defers the untap while another turn is only prospective"
    );
    assert_time_vault_begin_turn_decision(&game, PlayerId::Two, &[vault_id]);
    choose_begin_turn_source(&mut game, PlayerId::Two, vault_id);

    assert_eq!(game.turn, turn_before, "the second extra turn was skipped");
    assert!(
        game.battlefield[0].tapped,
        "both deferred untaps wait through the regular-turn proposal"
    );
    assert_time_vault_begin_turn_decision(&game, PlayerId::Two, &[vault_id]);
    choose_begin_turn_option(&mut game, PlayerId::Two, 0);

    assert_eq!(game.turn, turn_before + 1);
    assert_eq!(game.active_player, PlayerId::Two);
    assert!(!game.battlefield[0].tapped);
    assert!(game.extra_turns.is_empty());
    assert!(game.pending_decisions.is_empty());
}

#[test]
fn ugins_nexus_skips_each_queued_extra_turn_but_not_the_regular_turn() {
    let mut game = ready_game();
    game.step = Step::Cleanup;
    game.put_onto_battlefield(PlayerId::One, cards::UGINS_NEXUS)
        .expect("Ugin's Nexus is cataloged");
    game.extra_turns = vec![PlayerId::One, PlayerId::Two, PlayerId::One];
    let turn_before = game.turn;
    let turns_started_before = game.turns_started;

    game.start_next_turn();

    assert!(game.extra_turns.is_empty());
    assert!(game.pending_decisions.is_empty());
    assert_eq!(game.turn, turn_before + 1);
    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(
        game.turns_started,
        [turns_started_before[0], turns_started_before[1] + 1],
        "none of the three skipped extra turns began"
    );
    assert_eq!(game.next_regular_player, PlayerId::One);
}

#[test]
fn an_extra_turn_still_occurs_if_ugins_nexus_leaves_before_it_begins() {
    let mut game = ready_game();
    game.step = Step::Cleanup;
    let nexus = game
        .put_onto_battlefield(PlayerId::Two, cards::UGINS_NEXUS)
        .expect("Ugin's Nexus is cataloged");
    game.extra_turns.push(PlayerId::One);
    game.exile_permanent(nexus);
    let regular_anchor = game.next_regular_player;

    game.start_next_turn();

    assert_eq!(game.active_player, PlayerId::One);
    assert_eq!(game.next_regular_player, regular_anchor);
    assert!(game.extra_turns.is_empty());
    assert!(game.pending_decisions.is_empty());
}

#[test]
fn surviving_ugins_nexus_skips_the_extra_turn_created_by_the_legend_rule() {
    let mut game = ready_game();
    game.step = Step::Cleanup;
    game.put_onto_battlefield(PlayerId::One, cards::UGINS_NEXUS)
        .expect("Ugin's Nexus is cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::UGINS_NEXUS)
        .expect("a second Ugin's Nexus is cataloged");

    game.check_state_based_actions();

    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == cards::UGINS_NEXUS)
            .count(),
        1
    );
    assert_eq!(game.extra_turns, vec![PlayerId::One]);
    assert_eq!(game.players[0].exile.len(), 1);

    game.start_next_turn();

    assert!(game.extra_turns.is_empty());
    assert_eq!(game.active_player, PlayerId::Two);
    assert!(game.pending_decisions.is_empty());
}

#[test]
fn nexus_can_skip_an_extra_proposal_before_vault_skips_the_regular_one() {
    let mut game = ready_game();
    game.step = Step::Cleanup;
    let nexus = game
        .put_onto_battlefield(PlayerId::One, cards::UGINS_NEXUS)
        .expect("Ugin's Nexus is cataloged");
    let vault = tapped_time_vault(10_000, PlayerId::Two);
    let vault_id = vault.card.id;
    game.battlefield.push(vault);
    game.extra_turns.push(PlayerId::Two);
    let turn_before = game.turn;

    game.start_next_turn();

    let extra_choice = game
        .observe(PlayerId::Two)
        .decision
        .expect("the affected player orders Nexus and Vault");
    assert_eq!(extra_choice.preference, DecisionPreference::Neutral);
    assert!(extra_choice.options.iter().all(|option| option.id != 0));
    assert_eq!(extra_choice.options.len(), 2);
    choose_begin_turn_source(&mut game, PlayerId::Two, nexus);

    assert_eq!(game.turn, turn_before, "Nexus skipped the extra turn");
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == vault_id)
            .expect("Time Vault remains on the battlefield")
            .tapped
    );
    assert_time_vault_begin_turn_decision(&game, PlayerId::Two, &[vault_id]);
    choose_begin_turn_source(&mut game, PlayerId::Two, vault_id);

    assert_eq!(game.turn, turn_before + 1);
    assert_eq!(game.active_player, PlayerId::One);
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == vault_id)
            .expect("Time Vault remains on the battlefield")
            .tapped
    );
    assert!(game.pending_decisions.is_empty());
}

#[test]
fn skipping_an_extra_turn_preserves_the_ordinary_turn_anchor() {
    let mut game = ready_game();

    let vault = creature(10_000, cards::TIME_VAULT, PlayerId::Two);
    let vault_id = vault.card.id;
    game.battlefield.push(vault);
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    let activation = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, .. } if *source == vault_id
            )
        })
        .expect("the untapped Vault can be activated");
    game.apply(PlayerId::Two, activation).unwrap();

    assert!(game.battlefield[0].tapped, "tapping is an activation cost");
    assert!(
        game.extra_turns.is_empty(),
        "the effect still uses the stack"
    );
    pass_priority_pair(&mut game);
    assert_eq!(game.extra_turns, vec![PlayerId::Two]);

    game.step = Step::Cleanup;
    let turn_before = game.turn;
    let turns_started_before = game.turns_started;
    game.start_next_turn();
    assert_time_vault_begin_turn_decision(&game, PlayerId::Two, &[vault_id]);
    choose_begin_turn_option(&mut game, PlayerId::Two, 1);

    assert!(
        game.battlefield[0].tapped,
        "the ordinary turn is still only prospective while Vault may replace it again"
    );
    assert_time_vault_begin_turn_decision(&game, PlayerId::Two, &[vault_id]);
    choose_begin_turn_option(&mut game, PlayerId::Two, 0);

    assert!(!game.battlefield[0].tapped);
    assert_eq!(
        game.turn,
        turn_before + 1,
        "the skipped extra turn never began"
    );
    assert_eq!(
        game.turns_started,
        [turns_started_before[0], turns_started_before[1] + 1]
    );
    assert_eq!(
        game.active_player,
        PlayerId::Two,
        "skipping the extra turn leaves Player Two's ordinary turn next",
    );
    assert_eq!(game.next_regular_player, PlayerId::One);
    assert_eq!(game.step, Step::Upkeep);
    assert!(game.pending_decisions.is_empty());
}

#[test]
fn a_mandatory_extra_turn_replacement_skips_only_the_extra_proposal() {
    let mut game = ready_game();
    install_extra_turn_replacement(&mut game, 10_000);
    game.extra_turns.push(PlayerId::Two);
    game.step = Step::Cleanup;
    let turn_before = game.turn;
    let turns_started_before = game.turns_started;

    game.start_next_turn();

    assert!(game.extra_turns.is_empty());
    assert!(game.pending_decisions.is_empty());
    assert_eq!(game.turn, turn_before + 1);
    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(
        game.turns_started,
        [turns_started_before[0], turns_started_before[1] + 1]
    );
    assert_eq!(
        game.next_regular_player,
        PlayerId::One,
        "skipping an extra turn preserves and then consumes the ordinary turn anchor",
    );
}

#[test]
fn affected_player_may_apply_time_vault_before_a_mandatory_extra_turn_replacement() {
    let mut game = ready_game();
    let mandatory = install_extra_turn_replacement(&mut game, 10_000);
    let vault = tapped_time_vault(10_001, PlayerId::Two);
    let vault_id = vault.card.id;
    game.battlefield.push(vault);
    game.extra_turns.push(PlayerId::Two);
    game.step = Step::Cleanup;

    game.start_next_turn();

    let decision = game
        .observe(PlayerId::Two)
        .decision
        .expect("the affected player orders the applicable replacements");
    assert_eq!(decision.preference, DecisionPreference::Neutral);
    assert_eq!(decision.options.len(), 2);
    assert!(decision.options.iter().all(|option| option.id != 0));
    assert_eq!(
        decision.options[0].card,
        Some((
            mandatory,
            ObjectCharacteristics::card(CardDefinitionId::new(10_064), CardPartId::PRIMARY),
        ))
    );
    assert_eq!(
        decision.options[1].card,
        Some((
            vault_id,
            ObjectCharacteristics::card(cards::TIME_VAULT, CardPartId::PRIMARY),
        ))
    );

    choose_begin_turn_option(&mut game, PlayerId::Two, 2);

    assert!(
        game.battlefield[1].tapped,
        "Vault remains applicable to the following regular proposal"
    );
    assert_time_vault_begin_turn_decision(&game, PlayerId::Two, &[vault_id]);
    choose_begin_turn_option(&mut game, PlayerId::Two, 0);

    assert!(!game.battlefield[1].tapped);
    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(game.step, Step::Upkeep);
    assert!(game.extra_turns.is_empty());
    assert!(game.pending_decisions.is_empty());
}

#[test]
fn multiple_time_vaults_share_one_choice_and_only_one_replaces_each_turn() {
    let mut game = ready_game();
    game.step = Step::Cleanup;
    let first = tapped_time_vault(10_000, PlayerId::Two);
    let first_id = first.card.id;
    let second = tapped_time_vault(10_001, PlayerId::Two);
    let second_id = second.card.id;
    game.battlefield.extend([first, second]);

    game.start_next_turn();
    assert_time_vault_begin_turn_decision(&game, PlayerId::Two, &[first_id, second_id]);
    choose_begin_turn_option(&mut game, PlayerId::Two, 1);

    assert!(!game.battlefield[0].tapped);
    assert!(game.battlefield[1].tapped);
    assert_eq!(game.active_player, PlayerId::One);
    assert!(
        game.pending_decisions.is_empty(),
        "the other Vault cannot also replace the turn that was already skipped"
    );

    game.step = Step::Cleanup;
    game.start_next_turn();
    let decision = assert_time_vault_begin_turn_decision(&game, PlayerId::Two, &[second_id]);
    assert_eq!(
        decision.options[1].card,
        Some((
            second_id,
            ObjectCharacteristics::card(cards::TIME_VAULT, CardPartId::PRIMARY),
        )),
        "the still-tapped Vault is offered on a later prospective turn"
    );
    choose_begin_turn_option(&mut game, PlayerId::Two, 0);
}

#[test]
fn removing_time_vaults_abilities_removes_the_begin_turn_replacement() {
    let mut game = ready_game();
    game.step = Step::Cleanup;
    let vault = tapped_time_vault(10_000, PlayerId::Two);
    game.battlefield.push(vault);
    attach_constant_resolved_characteristics(
        &mut game,
        GameObjectId(10_000),
        &[AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any)],
        ContinuousEffectExpiration::Never,
    );
    assert!(game.effective_abilities(&game.battlefield[0]).is_empty());

    game.start_next_turn();

    assert!(game.pending_decisions.is_empty());
    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(game.step, Step::Upkeep);
    assert!(
        !game.battlefield[0].tapped,
        "removing all abilities also removes the static untap restriction"
    );
}

#[test]
fn extra_turn_effect_schedules_multiple_recipients_in_apnap_order() {
    let mut game = ready_game();
    let effect = EffectDef::TakeExtraTurn {
        player: EffectRecipientDef::EachPlayer,
    };
    let object = spell(10_000, cards::TIME_WALK, PlayerId::Two, 0);

    game.resolve_effect_def(
        ScopedEffect::primary(effect),
        &object,
        TriggerContext::empty(),
    );

    assert_eq!(
        game.extra_turns,
        vec![PlayerId::One, PlayerId::Two],
        "turns created together are added APNAP and consumed newest-first",
    );
    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::Two);
    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::One);
}

/// "If multiple extra-turn effects resolve in the same turn, take them in the
/// reverse of the order that the effects resolved. In other words, the most
/// recently created extra turn is taken first." Two Time Warps, and the
/// second one cast is the first one taken.
#[test]
fn the_most_recently_created_extra_turn_is_taken_first() {
    let mut game = ready_game();
    game.players[0].hand.clear();
    let mut warps = Vec::new();
    for index in 0..2 {
        let warp = card(10_400 + index, cards::TIME_WARP, PlayerId::One);
        warps.push(warp.id);
        game.players[0].hand.push(warp);
    }
    game.players[0].mana_pool.blue = 4;
    game.players[0].mana_pool.colorless = 6;

    // Their extra turn is created first, yours second.
    for (warp, player) in warps.into_iter().zip([PlayerId::Two, PlayerId::One]) {
        game.apply(
            PlayerId::One,
            cast_action(warp, vec![Target::Player(player)], Vec::new(), 0),
        )
        .expect("five mana casts it");
        pass_priority_pair(&mut game);
    }
    assert_eq!(
        game.extra_turns,
        vec![PlayerId::Two, PlayerId::One],
        "queued in the order they resolved",
    );

    game.start_next_turn();
    assert_eq!(
        game.active_player,
        PlayerId::One,
        "and taken from the other end: the newest extra turn goes first",
    );
    game.start_next_turn();
    assert_eq!(
        game.active_player,
        PlayerId::Two,
        "then the one made before it",
    );
}

/// "Take an extra turn" is a turn like any other: it untaps what you have
/// and draws you a card, which is most of why the card is worth its two
/// mana.
#[test]
fn the_extra_turn_untaps_and_draws_like_any_other() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turn = 9;
    game.turns_started = [5, 5];
    let mut land = creature(10_500, cards::ISLAND, PlayerId::One);
    land.tapped = true;
    let land_id = land.card.id;
    game.battlefield.push(land);
    let time_walk = card(10_501, cards::TIME_WALK, PlayerId::One);
    let walk_id = time_walk.id;
    game.players[0].hand.push(time_walk);
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    game.apply(
        PlayerId::One,
        cast_action(walk_id, Vec::new(), Vec::new(), 0),
    )
    .expect("two mana casts it");
    pass_priority_pair(&mut game);
    let library = game.players[0].library.len();

    game.start_next_turn();
    drain_pending(&mut game);
    assert_eq!(game.active_player, PlayerId::One, "the turn is yours again");
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == land_id)
            .expect("the land is there")
            .tapped,
        "and it untapped for it",
    );

    game.step = Step::Upkeep;
    game.advance_step();
    drain_pending(&mut game);
    assert_eq!(
        game.players[0].library.len(),
        library - 1,
        "an extra turn has a draw step like any other",
    );
}

/// Two Walks in a turn are two turns, both yours: the ruling about ordering
/// says nothing different when both extra turns belong to the same player.
#[test]
fn two_time_walks_are_two_turns_in_a_row() {
    let mut game = ready_game();
    game.players[0].hand.clear();
    let mut walks = Vec::new();
    for index in 0..2 {
        let walk = card(10_600 + index, cards::TIME_WALK, PlayerId::One);
        walks.push(walk.id);
        game.players[0].hand.push(walk);
    }
    game.players[0].mana_pool.blue = 2;
    game.players[0].mana_pool.colorless = 2;

    for walk in walks {
        game.apply(PlayerId::One, cast_action(walk, Vec::new(), Vec::new(), 0))
            .expect("two mana casts it");
        pass_priority_pair(&mut game);
    }
    assert_eq!(
        game.extra_turns,
        vec![PlayerId::One, PlayerId::One],
        "two extra turns are queued",
    );

    for expected in [PlayerId::One, PlayerId::One, PlayerId::Two] {
        game.start_next_turn();
        assert_eq!(
            game.active_player, expected,
            "both extra turns are taken before theirs",
        );
    }
}

/// The land drop comes with the turn: a Time Walk cast after the land for
/// the turn was played hands you another one, which is the other half of
/// what two mana buys.
#[test]
fn the_extra_turn_brings_a_fresh_land_drop() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turn = 9;
    game.turns_started = [5, 5];
    let island = card(10_700, cards::ISLAND, PlayerId::One);
    let island_id = island.id;
    game.players[0].hand.push(island);
    let time_walk = card(10_701, cards::TIME_WALK, PlayerId::One);
    let walk_id = time_walk.id;
    game.players[0].hand.push(time_walk);
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.players[0].lands_played_this_turn = 1;

    let land_drop = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .any(|action| matches!(action, Action::PlayLand { card, .. } if card == island_id))
    };
    assert!(!land_drop(&game), "this turn's land is already played");

    game.apply(
        PlayerId::One,
        cast_action(walk_id, Vec::new(), Vec::new(), 0),
    )
    .expect("two mana casts it");
    pass_priority_pair(&mut game);

    game.start_next_turn();
    drain_pending(&mut game);
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    assert_eq!(game.active_player, PlayerId::One, "the turn is yours again");
    assert_eq!(
        game.players[0].lands_played_this_turn, 0,
        "and the count that stopped you started over",
    );
    assert!(land_drop(&game), "so the Island is playable after all");
}

/// "After this one": the turn you are in finishes first, and the extra one
/// is a turn of its own rather than a continuation.
#[test]
fn the_extra_turn_comes_after_the_one_it_was_cast_in() {
    let mut game = ready_game();
    game.players[0].hand.clear();
    game.turn = 9;
    game.turns_started = [5, 5];
    let time_walk = card(10_800, cards::TIME_WALK, PlayerId::One);
    let walk_id = time_walk.id;
    game.players[0].hand.push(time_walk);
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let turn = game.turn;

    game.apply(
        PlayerId::One,
        cast_action(walk_id, Vec::new(), Vec::new(), 0),
    )
    .expect("two mana casts it");
    pass_priority_pair(&mut game);

    assert_eq!(
        game.turn, turn,
        "the extra turn has not started: this one is still going",
    );
    assert_eq!(
        game.extra_turns,
        vec![PlayerId::One],
        "it is queued behind the turn it was cast in",
    );

    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::One, "and then it is yours");
    assert!(game.turn > turn, "a new turn, not the old one continued");
}
