use super::action_view::{
    ability_origin_value, action_ability_origin, action_attack_defender, action_card, action_kind,
    action_sacrifices, action_target_card, action_target_cards, action_target_player,
    action_target_players, action_target_selections, action_target_stack, action_target_stacks,
    action_targets, attack_defender_value, cast_signature_value,
};
use super::presentation::{
    card_art_value, hand_mana_cost_value, implementation_status_name, object_presentation,
    stack_card_presentation, win_reason_text,
};
use super::{
    AbilityOrigin, Action, CardDefinitionId, DecisionKind, DecisionOrderSemantics, GameResult,
    PlayerId, Target, Value, WebGame, json,
};

const fn step_name(step: penta::Step) -> &'static str {
    match step {
        penta::Step::Upkeep => "Upkeep",
        penta::Step::Draw => "Draw",
        penta::Step::PrecombatMain => "Precombat Main",
        penta::Step::BeginningOfCombat => "Beginning Of Combat",
        penta::Step::DeclareAttackers => "Declare Attackers",
        penta::Step::DeclareBlockers => "Declare Blockers",
        penta::Step::CombatDamage => "Combat Damage",
        penta::Step::EndOfCombat => "End Of Combat",
        penta::Step::PostcombatMain => "Postcombat Main",
        penta::Step::End => "End",
        penta::Step::Cleanup => "Cleanup",
    }
}

const fn decision_visibility_name(visibility: penta::DecisionVisibility) -> &'static str {
    match visibility {
        penta::DecisionVisibility::Public => "Public",
        penta::DecisionVisibility::Private => "Private",
    }
}

const fn decision_zone_name(zone: penta::DecisionZone) -> &'static str {
    match zone {
        penta::DecisionZone::Hand => "Hand",
        penta::DecisionZone::Graveyard => "Graveyard",
        penta::DecisionZone::Battlefield => "Battlefield",
        penta::DecisionZone::Stack => "Stack",
        penta::DecisionZone::Library => "Library",
        penta::DecisionZone::Exile => "Exile",
        penta::DecisionZone::Command => "Command",
        penta::DecisionZone::OutsideGame => "Outside Game",
        penta::DecisionZone::DrawnThisStep => "Drawn This Step",
        penta::DecisionZone::None => "None",
    }
}

fn physical_face_value(face: penta::PhysicalFaceObservation) -> Value {
    json!({
        "kind": match face.kind {
            penta::DoubleFacedKind::Transforming => "transforming",
            penta::DoubleFacedKind::Modal => "modal",
        },
        "side": match face.side {
            penta::PhysicalFaceSide::Front => "front",
            penta::PhysicalFaceSide::Back => "back",
        },
    })
}

impl WebGame {
    #[allow(clippy::too_many_lines)]
    pub(super) fn snapshot(&self) -> Value {
        self.snapshot_value(true)
    }

    fn automatic_mana_sources(&self, action: &Action) -> Vec<u32> {
        self.session
            .mana_sources_for_action(self.human, action)
            .into_iter()
            .map(|source| source.0)
            .collect()
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn snapshot_value(&self, include_opponent_actions: bool) -> Value {
        let observation = self.session.observe(self.human);
        let opponent = self.human.opponent();
        let actions = observation
            .legal_actions
            .iter()
            .enumerate()
            .map(|(index, action)| {
                json!({
                    "index": index,
                    "label": self.action_label(&observation, action),
                    "kind": action_kind(action),
                    "cardId": action_card(action).map(|id| id.0),
                    "targetSelections": action_target_selections(action, self.human),
                    "attackDefender": action_attack_defender(action)
                        .map(|defender| attack_defender_value(defender, self.human)),
                    "targetCardId": action_target_card(action).map(|id| id.0),
                    "targetPlayer": action_target_player(action, self.human),
                    "targetStackId": action_target_stack(action),
                    "targetCardIds": action_target_cards(action),
                    "targetPlayers": action_target_players(action, self.human),
                    "targetStackIds": action_target_stacks(action),
                    "targetCount": action_targets(action).len(),
                    "ability": action_ability_origin(action),
                    "abilityLabel": self.action_ability_label(&observation, action),
                    "manaAbility": matches!(action, Action::ActivateManaAbility { .. }),
                    "spellAction": matches!(action, Action::CastSpell { .. }),
                    "sacrificeCardIds": action_sacrifices(action),
                    "combatDamageAttacker": match action {
                        Action::AssignCombatDamage { attacker, .. } => Some(attacker.0),
                        _ => None,
                    },
                    "x": match action {
                        Action::CastSpell { choices, .. } => Some(choices.x()),
                        Action::ActivateAbility { x, .. } => Some(*x),
                        _ => None,
                    },
                    "playOptionId": match action {
                        Action::PlayLand { option, .. } => Some(option.0),
                        Action::CastSpell { choices, .. } => Some(choices.play_option().0),
                        _ => None,
                    },
                    "modeIds": match action {
                        Action::CastSpell { choices, .. } => Some(
                            choices.modes().iter().map(|mode| mode.0).collect::<Vec<_>>(),
                        ),
                        Action::ActivateAbility { modes, .. } => {
                            Some(modes.iter().map(|mode| mode.0).collect::<Vec<_>>())
                        }
                        _ => None,
                    },
                    "paymentAction": matches!(action, Action::CastSpell { .. } | Action::ActivateAbility { .. }),
                    "manaSourceIds": self.automatic_mana_sources(action),
                    "decisionId": match action {
                        Action::ChooseDecision { decision, .. }
                        | Action::CancelDecision { decision } => Some(*decision),
                        _ => None,
                    },
                    "decisionOptionIds": match action {
                        Action::ChooseDecision { options, .. } => options.clone(),
                        _ => Vec::new(),
                    },
                    // Mulligan combinations remain part of the stable bot
                    // protocol, but the browser groups them into one picker.
                    // These IDs let it stage individual card choices and
                    // submit the matching atomic engine action on confirmation.
                    "bottomCardIds": match action {
                        Action::BottomCards { cards } =>
                            cards.iter().map(|card| card.0).collect::<Vec<_>>(),
                        _ => Vec::new(),
                    },
                })
            })
            .collect::<Vec<_>>();
        let battlefield = observation
            .battlefield
            .iter()
            .map(|permanent| {
                let presentation = object_presentation(&self.catalog, permanent.characteristics);
                // The engine reports what the permanent is right now, so an
                // animated land renders as the creature it became rather than
                // as the land it is printed as.
                let current_kind = if permanent.types.is_empty() {
                    presentation.kind.clone()
                } else {
                    permanent.types.kind_name().to_ascii_lowercase()
                };
                // The printed line still carries the subtypes, so it is only
                // replaced when the permanent has stopped matching it.
                let current_type_line = if !permanent.types.is_empty()
                    && presentation.types != permanent.types
                {
                    permanent.types.type_name()
                } else {
                    presentation.type_line.clone()
                };
                let mut value = json!({
                    "id": permanent.id.0,
                    "partId": permanent.characteristics.part().0,
                    "name": presentation.name,
                    "art": card_art_value(presentation.art.as_ref()),
                    "kind": current_kind,
                    "typeLine": current_type_line,
                    "implementationStatus": implementation_status_name(
                        presentation.implementation_status,
                    ),
                    "isLand": presentation.is_land,
                    "manaCost": presentation.mana_cost.map(|cost| json!({
                        "generic": cost.generic,
                        "white": cost.white,
                        "blue": cost.blue,
                        "black": cost.black,
                        "red": cost.red,
                        "green": cost.green,
                        "colorless": cost.colorless,
                        "hybrid": penta::HybridPair::ALL
                            .into_iter()
                            .filter(|pair| cost.hybrid[pair.index()] > 0)
                            .map(|pair| {
                                json!({ "symbol": pair.symbol(), "count": cost.hybrid[pair.index()] })
                            })
                            .collect::<Vec<_>>(),
                        "x": cost.variable_x,
                    })),
                    "rulesText": presentation.rules_text,
                    "owner": if permanent.controller == self.human { "human" } else { "opponent" },
                    "chosenCardName": permanent.chosen_card_name.as_deref(),
                    "chosenCreatureType": permanent.chosen_creature_type.as_deref(),
                    "tapped": permanent.tapped,
                    "power": permanent.power,
                    "toughness": permanent.toughness,
                    "damage": permanent.damage,
                    "loyalty": permanent.loyalty,
                    "loyaltyAbilityUsedThisTurn": permanent.loyalty_ability_used_this_turn,
                    "attacking": permanent.attacking,
                    "attackDefender": permanent
                        .attack_defender
                        .map(|defender| attack_defender_value(defender, self.human)),
                    "blockedThisCombat": permanent.blocked_this_combat,
                    "blocking": permanent.blocking.iter().map(|id| id.0).collect::<Vec<_>>(),
                    "blockingThisCombat": permanent.blocking_this_combat,
                    "attackingBand": permanent.attacking_band,
                    "flying": permanent.flying,
                    "canAttack": permanent.can_attack,
                    "enteredThisTurn": permanent.entered_this_turn,
                });
                if let Some(face) = permanent.physical_face {
                    value["physicalFace"] = physical_face_value(face);
                }
                value
            })
            .collect::<Vec<_>>();
        // The hand and the graveyard both draw real cards in the browser, and
        // flashback means a graveyard card can carry actions of its own, so
        // both zones need the same shape rather than a list of names.
        let card_in_zone = |id: penta::GameObjectId, definition: CardDefinitionId| {
            let card = self.catalog.get(definition);
            let art = card.and_then(|card| card.art.as_ref());
            let creature_stats = card.and_then(|card| card.rules.creature_stats());
            json!({
                "id": id.0,
                "name": self.card_name(definition),
                "art": card_art_value(art),
                "kind": card.map_or("unknown".into(), |card| {
                    card.rules.kind_name().to_ascii_lowercase()
                }),
                "typeLine": card.map_or_else(String::new, |card| card.rules.type_line()),
                "implementationStatus": card.map_or("complete", |card| {
                    implementation_status_name(card.implementation_status())
                }),
                "isLand": card.is_some_and(|card| card.rules.has_type(penta::CardType::Land)),
                "manaCost": hand_mana_cost_value(card),
                "rulesText": card.map_or_else(String::new, |card| {
                    card.rules.rules_text().into_owned()
                }),
                "power": creature_stats.map(|stats| stats.power),
                "toughness": creature_stats.map(|stats| stats.toughness),
            })
        };
        let hand = observation
            .hand
            .iter()
            .map(|(id, definition)| card_in_zone(*id, *definition))
            .collect::<Vec<_>>();
        let stack = observation
            .stack
            .iter()
            .rev()
            .map(|object| {
                let ability_id = object.ability.and_then(|origin| match origin {
                    AbilityOrigin::Printed { ability, .. } => Some(ability.0),
                    AbilityOrigin::Token { .. }
                    | AbilityOrigin::Emblem { .. }
                    | AbilityOrigin::FaceDown { .. }
                    | AbilityOrigin::IntrinsicBasicLand(_)
                    | AbilityOrigin::IntrinsicCounter(_)
                    | AbilityOrigin::Granted { .. }
                    | AbilityOrigin::TokenGranted { .. }
                    | AbilityOrigin::EmblemGranted { .. }
                    | AbilityOrigin::FaceDownGranted { .. } => None,
                });
                // Enough card detail for the browser to draw a real card on
                // the stack rather than a name tag.
                let signature = object.signature.as_ref();
                let presentation = stack_card_presentation(
                    &self.catalog,
                    object.characteristics,
                    signature,
                );
                let targets = signature.map_or_else(
                    || object.targets.clone(),
                    |signature| signature.iter_targets().copied().collect(),
                );
                json!({
                    "id": object.id.0,
                    // Kept as a JSON compatibility field for the browser;
                    // this is the spell/ability object, not physical lineage.
                    "cardId": object.id.0,
                    "sourceId": object.source.map(|source| source.0),
                    "ability": object.ability.map(ability_origin_value),
                    // Compatibility projection for clients that only know printed clause IDs.
                    "abilityId": ability_id,
                    "abilityText": object.ability_text,
                    "name": presentation.name,
                    "art": card_art_value(presentation.art.as_ref()),
                    "owner": if object.controller == self.human { "human" } else { "opponent" },
                    "kind": match object.kind {
                        penta::StackObjectKind::Spell => "Spell",
                        penta::StackObjectKind::ActivatedAbility => "ActivatedAbility",
                        penta::StackObjectKind::TriggeredAbility => "TriggeredAbility",
                    },
                    "counterable": object.counterable,
                    "x": signature.map_or(0, penta::CastSignature::x),
                    "playOptionId": signature.map(|signature| signature.play_option().0),
                    "modeIds": signature.map(|signature| {
                        signature.modes().iter().map(|mode| mode.0).collect::<Vec<_>>()
                    }),
                    "signature": signature.map(|signature| {
                        cast_signature_value(signature, self.human)
                    }),
                    "targetCardIds": targets
                        .iter()
                        .filter_map(|target| match target {
                            Target::Card(id) | Target::Permanent(id) => Some(id.0),
                            _ => None,
                        })
                        .collect::<Vec<_>>(),
                    "targetPlayers": targets
                        .iter()
                        .filter_map(|target| match target {
                            Target::Player(player) if *player == self.human => Some("human"),
                            Target::Player(_) => Some("opponent"),
                            _ => None,
                        })
                        .collect::<Vec<_>>(),
                    "targetStackIds": targets
                        .iter()
                        .filter_map(|target| match target {
                            Target::Spell(id) => Some(id.0),
                            _ => None,
                        })
                        .collect::<Vec<_>>(),
                    "cardKind": presentation.kind,
                    "typeLine": presentation.type_line,
                    "implementationStatus": implementation_status_name(
                        presentation.implementation_status,
                    ),
                    "isLand": presentation.is_land,
                    "manaCost": presentation.mana_cost.map(|cost| json!({
                        "generic": cost.generic,
                        "white": cost.white,
                        "blue": cost.blue,
                        "black": cost.black,
                        "red": cost.red,
                        "green": cost.green,
                        "colorless": cost.colorless,
                        "hybrid": penta::HybridPair::ALL
                            .into_iter()
                            .filter(|pair| cost.hybrid[pair.index()] > 0)
                            .map(|pair| {
                                json!({ "symbol": pair.symbol(), "count": cost.hybrid[pair.index()] })
                            })
                            .collect::<Vec<_>>(),
                        "x": cost.variable_x,
                    })),
                    "rulesText": presentation.rules_text,
                    "power": presentation.power,
                    "toughness": presentation.toughness,
                })
            })
            .collect::<Vec<_>>();
        let graveyard = |player: PlayerId| {
            observation.graveyards[player.index()]
                .iter()
                .rev()
                .map(|(id, definition)| card_in_zone(*id, *definition))
                .collect::<Vec<_>>()
        };
        let result = self.session.result().map(|result| match result {
            GameResult::Winner { winner, reason } => json!({
                "outcome": if winner == self.human { "win" } else { "loss" },
                "message": format!(
                    "{} — {}",
                    if winner == self.human { "You win" } else { "You lose" },
                    // WinReason names the loser as "the opponent" from the
                    // winner's seat. The browser only ever has the human's
                    // seat, so say who actually did the losing.
                    win_reason_text(reason, winner != self.human)
                ),
            }),
            GameResult::Draw => json!({"outcome": "draw", "message": "Draw"}),
        });
        // `events_for` withholds the seed. This client owns the engine, so it
        // may still show it; a remote one would not have it to show.
        let seat_events = self.session.events_for(self.human);
        // Against a built-in policy the seed is a courtesy: the human owns
        // the whole game, locally or via their own room. Against an external
        // driver it is the opponent's hand and library order, so it is not
        // printed at all.
        let seed_line = (!self.opponent_is_externally_driven())
            .then(|| format!("Game started · seed {}", self.session.seed()));
        let events = std::iter::once(seed_line)
            .chain(
                seat_events
                    .iter()
                    .map(|event| self.event_label(&observation, event)),
            )
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .flatten()
            .take(16)
            .collect::<Vec<_>>();
        let opponent_actions = if include_opponent_actions {
            self.opponent_actions.clone()
        } else {
            Vec::new()
        };
        // Only worth sending alongside a replay: with nothing to replay the
        // client shows this state directly and your action is already in it.
        let human_action_state = if include_opponent_actions && !self.opponent_actions.is_empty() {
            self.human_action_state.clone()
        } else {
            None
        };
        let decision = observation.decision.as_ref().map(|decision| {
            let mut value = json!({
                "id": decision.id,
                "kind": match decision.kind {
                    DecisionKind::Choice => "Choice",
                    DecisionKind::TriggerOrder => "TriggerOrder",
                    DecisionKind::TriggerPlacement => "TriggerPlacement",
                },
                "prompt": decision.prompt,
                "minimum": decision.minimum,
                "maximum": decision.maximum,
                "cancellable": decision.cancellable,
                "visibility": decision_visibility_name(decision.visibility),
                "options": decision.options.iter().map(|option| json!({
                    "id": option.id,
                    "triggerId": matches!(decision.kind, DecisionKind::TriggerOrder).then_some(option.id),
                    "label": option.label,
                    "cardId": option.card.map(|(card, _)| card.0),
                    "cardName": option.card.map(|(_, characteristics)| {
                        object_presentation(&self.catalog, characteristics).name
                    }),
                    "members": option.members.iter().map(|(card, characteristics)| json!({
                        "id": card.0,
                        "name": object_presentation(&self.catalog, *characteristics).name,
                    })).collect::<Vec<_>>(),
                    "abilityText": option.ability_text,
                    "zone": decision_zone_name(option.zone),
                })).collect::<Vec<_>>(),
            });
            if let Some(order_semantics) = decision.order_semantics {
                value["orderSemantics"] = Value::from(match order_semantics {
                    DecisionOrderSemantics::Resolution => "resolution",
                });
            }
            value
        });

        json!({
            "format": self.session.format().slug(),
            "turn": observation.active_turn,
            "gameTurn": observation.turn,
            "step": step_name(observation.step),
            "regularCombatDamagePending": observation.regular_combat_damage_pending,
            // Turn one has not started yet, so the board should not be
            // claiming an upkeep is happening.
            "pregame": self.session.in_pregame(),
            "active": if observation.active_player == self.human { "You" } else { "Opponent" },
            "priority": if observation.priority == self.human { "You" } else { "Opponent" },
            "human": {
                "life": observation.life_totals[self.human.index()],
                "library": observation.library_sizes[self.human.index()],
                "mana": {
                    "white": observation.mana_pools[self.human.index()].white,
                    "blue": observation.mana_pools[self.human.index()].blue,
                    "black": observation.mana_pools[self.human.index()].black,
                    "red": observation.mana_pools[self.human.index()].red,
                    "green": observation.mana_pools[self.human.index()].green,
                    "colorless": observation.mana_pools[self.human.index()].colorless,
                },
                "hand": hand,
                "graveyard": graveyard(self.human),
            },
            "opponent": {
                "life": observation.life_totals[opponent.index()],
                "library": observation.library_sizes[opponent.index()],
                "handSize": observation.opponent_hand_size,
                "mana": {
                    "white": observation.mana_pools[opponent.index()].white,
                    "blue": observation.mana_pools[opponent.index()].blue,
                    "black": observation.mana_pools[opponent.index()].black,
                    "red": observation.mana_pools[opponent.index()].red,
                    "green": observation.mana_pools[opponent.index()].green,
                    "colorless": observation.mana_pools[opponent.index()].colorless,
                },
                "graveyard": graveyard(opponent),
            },
            "battlefield": battlefield,
            "emblems": observation.emblems.iter().map(|emblem| json!({
                "id": emblem.id.0,
                "owner": if emblem.controller == self.human { "human" } else { "opponent" },
                "name": emblem.name,
                "rulesText": emblem.ability_texts.join(" "),
                "abilityTexts": emblem.ability_texts,
                "sourceAbility": ability_origin_value(emblem.source_ability),
            })).collect::<Vec<_>>(),
            "stack": stack,
            "actions": actions,
            "passLabel": self.pass_preview_label(),
            "decision": decision,
            "canUndoMana": !self.mana_undo_history.is_empty(),
            "canCancelAttackers": self.attack_undo.is_some(),
            "phaseStops": self.phase_stops,
            "autopassEnabled": self.autopass_enabled,
            "opponentActions": opponent_actions,
            "afterYourAction": human_action_state,
            "result": result,
            "events": events,
        })
    }
}
