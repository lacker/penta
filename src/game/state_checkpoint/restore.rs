// Rebuilding a game from a seat checkpoint.
//
// Split out of `state_checkpoint.rs` to keep that file inside the source-size
// limit, along the seam the two directions already had: the file next door
// writes a checkpoint out, and this reads one back in. Included textually, so
// the imports here are that module's.

impl Game {
    /// Rebuilds a decision-boundary state from its seat checkpoint and
    /// separately supplied hidden-zone hypothesis.
    #[allow(clippy::too_many_lines)]
    pub(crate) fn from_observation_checkpoint(
        catalog: CardCatalog,
        format: Format,
        observation: &Value,
        hidden: &Value,
        rollout_seed: u64,
    ) -> Result<Self, String> {
        let checkpoint_value = field(observation, "checkpoint")?;
        let checkpoint = parse_compatible_game_snapshot(checkpoint_value)?;
        let viewer = seat_value(field(observation, "seat")?)?;
        if checkpoint.viewer != viewer.index() {
            return Err("checkpoint viewer does not match observation seat".into());
        }

        let mut next_object_id = max_public_object_id(observation)
            .unwrap_or(0)
            .saturating_add(1);
        let own_hand = parse_cards(field(observation, "hand")?, viewer, &catalog)?;
        let opponent = viewer.opponent();
        let opponent_hand_defs = hidden_definitions(hidden, "hands", opponent)?;
        if opponent_hand_defs.len() != usize_field(observation, "opponentHandSize")? {
            return Err("hidden opponent hand does not match opponentHandSize".into());
        }
        let opponent_hand =
            mint_cards(&opponent_hand_defs, opponent, &catalog, &mut next_object_id)?;
        let libraries = [PlayerId::One, PlayerId::Two].map(|player| {
            hidden_definitions(hidden, "libraries", player).and_then(|definitions| {
                let expected = array(field(observation, "librarySizes")?)?
                    .get(player.index())
                    .and_then(Value::as_u64)
                    .and_then(|value| usize::try_from(value).ok())
                    .ok_or_else(|| "librarySizes must contain two counts".to_owned())?;
                if definitions.len() != expected {
                    return Err(format!(
                        "hidden {} library has {} cards, expected {expected}",
                        seat_label(player),
                        definitions.len()
                    ));
                }
                mint_cards(&definitions, player, &catalog, &mut next_object_id)
            })
        });
        let [library_one, library_two] = libraries;
        let library_one = library_one?;
        let library_two = library_two?;
        let outside_game = [PlayerId::One, PlayerId::Two].map(|player| {
            hidden_definitions(hidden, "outsideGame", player).and_then(|definitions| {
                mint_cards(&definitions, player, &catalog, &mut next_object_id)
            })
        });
        let [outside_one, outside_two] = outside_game;
        let mut outside_game = [outside_one?, outside_two?];

        let mut graveyards = parse_two_public_zones(field(observation, "graveyards")?, &catalog)?;
        let mut exiles = parse_two_public_zones(field(observation, "exiles")?, &catalog)?;
        let life = i16_pair(field(observation, "life")?)?;
        let player_counters = player_counters(observation)?;
        let mut checkpoint_hands = if viewer == PlayerId::One {
            [own_hand, opponent_hand]
        } else {
            [opponent_hand, own_hand]
        };
        restore_visible_card_counters(
            observation,
            &mut checkpoint_hands,
            &mut graveyards,
            &mut exiles,
        )?;
        let mut libraries = [library_one, library_two];
        // Before the decision's own rebinding: a stack source names a
        // position in the hypothesis, and the decision pass may reorder the
        // very zone it names.
        rebind_stack_source_cards(
            &stack_source_origins(&checkpoint.stack),
            &mut checkpoint_hands,
            &mut libraries,
            &mut outside_game,
        )?;
        rebind_visible_decision_cards(
            observation,
            checkpoint.decision_state.as_ref(),
            viewer,
            &mut checkpoint_hands,
            &mut libraries,
            &mut outside_game,
        )?;
        let lands_played = checkpoint.lands_played_this_turn;
        let tried_empty = checkpoint.tried_to_draw_from_empty_library;
        let mana_values = array(field(observation, "manaPools")?)?;
        if mana_values.len() != 2 {
            return Err("manaPools must contain p1 and p2 values".into());
        }
        let mana_pools = [
            parse_mana_pool(&mana_values[0])?,
            parse_mana_pool(&mana_values[1])?,
        ];
        let mana = [
            parse_mana(&checkpoint.mana[0], &catalog)?,
            parse_mana(&checkpoint.mana[1], &catalog)?,
        ];
        for player in [PlayerId::One, PlayerId::Two] {
            if mana_pool_from_units(&mana[player.index()]) != mana_pools[player.index()] {
                return Err(format!(
                    "checkpoint mana units do not match {} aggregate mana pool",
                    seat_label(player),
                ));
            }
        }
        let players = [PlayerId::One, PlayerId::Two].map(|player| PlayerState {
            life: life[player.index()],
            library: libraries[player.index()].clone(),
            tried_to_draw_from_empty_library: tried_empty[player.index()],
            hand: checkpoint_hands[player.index()].clone(),
            graveyard: graveyards[player.index()].clone(),
            exile: exiles[player.index()].clone(),
            outside_game: outside_game[player.index()].clone(),
            companions: checkpoint.companions[player.index()]
                .iter()
                .map(|definition| crate::CardDefinitionId::new(*definition))
                .collect(),
            mana_pool: mana_pools[player.index()],
            mana: mana[player.index()].clone(),
            lands_played_this_turn: lands_played[player.index()],
            counters: player_counters[player.index()].clone(),
        });

        let turns_started = checkpoint.turns_started;
        let nonbattlefield_ability_grants = checkpoint
            .nonbattlefield_ability_grants
            .iter()
            .map(|grant| {
                Ok(NonbattlefieldAbilityGrant {
                    object: GameObjectId(grant.object),
                    ability: catalog_ability(&catalog, &grant.ability)
                        .ok_or("nonbattlefield ability grant locator is absent from this catalog")?,
                    expiration: parse_expiration(&grant.expiration)?,
                    source: grant.source.map(ability_origin_from_snapshot),
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let damage_preventions = checkpoint
            .damage_preventions
            .iter()
            .map(|prevention| prevention::parse_damage_prevention(&catalog, prevention))
            .collect::<Result<Vec<_>, _>>()?;
        let damage_redirects = checkpoint
            .damage_redirects
            .iter()
            .map(prevention::parse_damage_redirect)
            .collect::<Result<Vec<_>, _>>()?;
        let resolved_play_restrictions = checkpoint
            .resolved_play_restrictions
            .iter()
            .map(|restriction| {
                play_restriction::parse_resolved_play_restriction(&catalog, restriction)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let resolved_attack_restrictions = checkpoint
            .resolved_attack_restrictions
            .iter()
            .map(|restriction| {
                play_restriction::parse_resolved_attack_restriction(&catalog, restriction)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let resolved_play_permissions = checkpoint
            .resolved_play_permissions
            .iter()
            .map(|permission| {
                play_restriction::parse_resolved_play_permission(&catalog, permission)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let resolved_player_protections = checkpoint
            .resolved_player_protections
            .iter()
            .map(|protection| {
                play_restriction::parse_resolved_player_protection(&catalog, protection)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let resolved_player_rules = checkpoint
            .resolved_player_rules
            .iter()
            .map(|rule| play_restriction::parse_resolved_player_rule(&catalog, rule))
            .collect::<Result<Vec<_>, _>>()?;
        let prepared_engine = crate::prepared_engine::PreparedEngine::compile(&catalog);
        let mut game = Self {
            format,
            arrived: None,
            enumerated: EnumeratedActions::default(),
            prospective_x: super::prospective_x::ProspectiveX::default(),
            successors: std::collections::HashMap::new(),
            damage_taken_this_turn: checkpoint.damage_taken_this_turn,
            attacked_subtypes_this_turn: [
                restore_attacked_subtypes(&checkpoint.attacked_subtypes_this_turn[0]),
                restore_attacked_subtypes(&checkpoint.attacked_subtypes_this_turn[1]),
            ],
            damage_taken_by_group_this_turn: {
                let mut groups = [[0; DamageSourceGroupDef::COUNT]; 2];
                for (seat, stored) in checkpoint
                    .damage_taken_by_group_this_turn
                    .iter()
                    .enumerate()
                {
                    // A shorter historical vector is tolerated: groups are
                    // only ever appended.
                    if let Some(row) = groups.get_mut(seat) {
                        for (slot, value) in row.iter_mut().zip(stored) {
                            *slot = *value;
                        }
                    }
                }
                groups
            },
            seed: rollout_seed,
            rng: ReplayRng::new(rollout_seed),
            catalog,
            prepared_engine,
            physical_cards: Vec::new(),
            players,
            battlefield: Vec::new(),
            phased_out: Vec::new(),
            stack: GameStack::default(),
            retired_objects: BTreeMap::new(),
            nonbattlefield_ability_grants,
            ongoing_effects: Vec::new(),
            next_object_id,
            next_continuous_effect_timestamp: checkpoint.next_continuous_effect_timestamp,
            turn: u32_field(observation, "turn")?,
            turns_started,
            active_player: seat_value(field(observation, "activeSeat")?)?,
            priority: seat_value(field(observation, "prioritySeat")?)?,
            consecutive_passes: checkpoint.consecutive_passes,
            step: parse_step(str_field(observation, "step")?)?,
            attackers_declared: checkpoint.attackers_declared,
            creature_died_this_turn: checkpoint.creature_died_this_turn,
            creatures_died_this_turn: checkpoint.creatures_died_this_turn,
            damage_cannot_be_prevented_this_turn: checkpoint.damage_cannot_be_prevented_this_turn,
            // Never live across a checkpoint: it is read and consumed
            // inside one activation, which cannot be interrupted.
            ninjutsu_returned_defender: None,
            exile_play_permissions: checkpoint
                .exile_play_permissions
                .iter()
                .map(exile_play::parse_permission)
                .collect::<Result<Vec<_>, String>>()?,
            monarch: checkpoint.monarch.map(player_from_index).transpose()?,
            linked_exiles: checkpoint
                .linked_exiles
                .iter()
                .map(|pair| (GameObjectId(pair[0]), GameObjectId(pair[1])))
                .collect(),
            graveyard_permission_uses: checkpoint
                .graveyard_permission_uses
                .iter()
                .map(|pair| {
                    (
                        GameObjectId(pair[0]),
                        u16::try_from(pair[1]).unwrap_or(u16::MAX),
                    )
                })
                .collect(),
            cannot_gain_life: checkpoint.cannot_gain_life,
            // Only ever holds anything while one combat damage step is being
            // dealt, which is not a moment a checkpoint is taken.
            combat_damage_to_players: Vec::new(),
            turn_phase_queue: checkpoint
                .turn_phase_queue
                .iter()
                .copied()
                .map(parse_turn_phase)
                .collect(),
            turn_phase_resume: checkpoint.turn_phase_resume.map(parse_turn_phase_resume),
            resolved_play_restrictions,
            resolved_attack_restrictions,
            resolved_play_permissions,
            resolved_player_protections,
            resolved_player_rules,
            emblems: Vec::new(),
            spells_cast_this_turn: checkpoint.spells_cast_this_turn,
            total_spells_cast: checkpoint.spells_cast_this_game,
            spells_cast_last_turn: checkpoint.spells_cast_last_turn,
            spell_cast_history_this_turn: ids(&checkpoint.spell_cast_history_this_turn),
            cards_drawn_this_turn: checkpoint.cards_drawn_this_turn,
            citys_blessing: checkpoint.citys_blessing,
            permanent_left_battlefield_this_turn: checkpoint.permanent_left_battlefield_this_turn,
            card_left_graveyard_this_turn: checkpoint.card_left_graveyard_this_turn,
            life_gained_this_turn: checkpoint.life_gained_this_turn,
            lost_life_this_turn: checkpoint.lost_life_this_turn,
            draw_step_draw_taken: checkpoint.draw_step_draw_taken,
            drawn_this_turn: parse_drawn_this_turn(&checkpoint, hidden, viewer, &checkpoint_hands)?,
            defer_empty_library_loss: checkpoint.defer_empty_library_loss,
            draw_replacements: std::array::from_fn(|_| VecDeque::new()),
            installed_triggers: Vec::new(),
            next_installed_trigger_id: checkpoint.next_installed_trigger_id,
            blockers_declared: checkpoint.blockers_declared,
            untap_pending: checkpoint.untap_pending,
            pregame: parse_pregame(checkpoint.pregame)?,
            mulligans: checkpoint.mulligans,
            cleanup_pending: checkpoint.cleanup_pending,
            pending_decisions: Vec::new(),
            next_decision_id: checkpoint.next_decision_id,
            pending_events: VecDeque::new(),
            // A batch is a thing in flight, and a checkpoint is taken
            // between them rather than inside one.
            entry_event_batch: None,
            pending_procedures: VecDeque::new(),
            pending_triggers: Vec::new(),
            next_trigger_id: checkpoint.next_trigger_id,
            last_seen_hands: [None, None],
            pending_combat_assignments: ids(&checkpoint.pending_combat_attackers),
            combat_damage_stage: parse_combat_stage(&checkpoint.combat_damage_stage),
            combat_blocked_attackers: ids(&checkpoint.combat_blocked_attackers),
            extra_turns: checkpoint
                .extra_turns
                .iter()
                .copied()
                .map(player_from_index)
                .collect::<Result<Vec<_>, _>>()?,
            next_regular_player: player_from_index(checkpoint.next_regular_player)?,
            damage_preventions,
            damage_redirects,
            result: None,
            events: vec![GameEvent::GameStarted { seed: rollout_seed }],
        };
        let (battlefield, phased_out) =
            parse_battlefield(observation, &checkpoint.battlefield, &game.catalog)?;
        game.battlefield = battlefield;
        game.phased_out = phased_out;
        game.emblems = parse_emblems(observation, &checkpoint.emblems, &game)?;
        game.retired_objects = parse_retired_objects(&checkpoint.retired_objects, &game)?;
        game.successors = checkpoint
            .successors
            .iter()
            .map(|entry| (GameObjectId(entry.retired), GameObjectId(entry.became)))
            .collect();

        game.stack = parse_stack(observation, &checkpoint.stack, &game)?;
        game.ongoing_effects = checkpoint
            .ongoing_effects
            .iter()
            .map(|ongoing| parse_ongoing_effect(ongoing, &game))
            .collect::<Result<Vec<_>, _>>()?;
        game.pending_events = parse_pending_events(&checkpoint.pending_events, &game.catalog)?;
        game.installed_triggers = checkpoint
            .installed_triggers
            .iter()
            .map(|trigger| parse_installed_trigger(trigger, &game))
            .collect::<Result<Vec<_>, _>>()?;
        game.pending_triggers = checkpoint
            .pending_triggers
            .iter()
            .map(|trigger| parse_pending_trigger(trigger, &game))
            .collect::<Result<Vec<_>, _>>()?;
        let draw_replacements = [PlayerId::One, PlayerId::Two].map(|player| {
            checkpoint.draw_replacements[player.index()]
                .iter()
                .map(|replacement| parse_draw_replacement(replacement, &game))
                .collect::<Result<VecDeque<_>, _>>()
        });
        let [replacements_one, replacements_two] = draw_replacements;
        game.draw_replacements = [replacements_one?, replacements_two?];
        game.pending_procedures = checkpoint
            .pending_procedures
            .iter()
            .map(|procedure| parse_pending_procedure(procedure, &game))
            .collect::<Result<VecDeque<_>, _>>()?;
        game.pending_decisions = parse_pending_decision(
            observation,
            checkpoint.decision_state.as_ref(),
            hidden,
            &game,
        )?
        .into_iter()
        .collect();
        game.last_seen_hands[viewer.index()] =
            parse_last_seen_hand(observation.get("lastSeenHand"))?;
        if game.pending_decisions.iter().any(|decision| {
            decision.observation.id >= game.next_decision_id && game.next_decision_id != u32::MAX
        }) {
            return Err("checkpoint next decision id does not follow its pending decision".into());
        }
        if game
            .pending_triggers
            .iter()
            .any(|trigger| trigger.id >= game.next_trigger_id && game.next_trigger_id != u32::MAX)
        {
            return Err("checkpoint next trigger id does not follow its pending triggers".into());
        }
        if game.installed_triggers.iter().any(|trigger| {
            trigger.id >= game.next_installed_trigger_id
                && game.next_installed_trigger_id != u32::MAX
        }) {
            return Err(
                "checkpoint next installed trigger id does not follow its installed triggers"
                    .into(),
            );
        }
        Ok(game)
    }
}
