use super::action_view::{
    source_ability_has_multiple_x_values, source_has_multiple_activated_abilities,
};
use super::presentation::{mana_cost_label, object_presentation};
use super::{
    AbilityOrigin, Action, BattlefieldExit, CardDefinitionId, CardInstanceId, GameEvent,
    GameResult, ModeId, ObjectCharacteristics, PlayOptionId, PlayerId, PlayerObservation, Step,
    Target, WebGame, readable_debug,
};
use penta::card::AbilityDef;
use std::fmt::Write as _;

impl WebGame {
    pub(super) fn card_name(&self, definition: CardDefinitionId) -> String {
        self.catalog
            .get(definition)
            .map_or_else(|| "Unknown card".into(), |card| card.name.clone())
    }

    fn instance_characteristics(
        observation: &PlayerObservation,
        id: CardInstanceId,
    ) -> Option<ObjectCharacteristics> {
        observation
            .hand
            .iter()
            .find_map(|(candidate, definition)| {
                (*candidate == id).then_some(ObjectCharacteristics::card(
                    *definition,
                    penta::CardPartId::PRIMARY,
                ))
            })
            .or_else(|| {
                observation
                    .battlefield
                    .iter()
                    .find_map(|permanent| (permanent.id == id).then_some(permanent.characteristics))
            })
            .or_else(|| {
                observation
                    .graveyards
                    .iter()
                    .flatten()
                    .find_map(|(candidate, definition)| {
                        (*candidate == id).then_some(ObjectCharacteristics::card(
                            *definition,
                            penta::CardPartId::PRIMARY,
                        ))
                    })
            })
            .or_else(|| {
                // A spell the opponent just cast is still on the stack, and it
                // is public there even though it never passed through a zone
                // this observation can see.
                observation
                    .stack
                    .iter()
                    .find_map(|object| (object.id == id).then_some(object.characteristics))
            })
            .or_else(|| {
                // Exiled cards stay public, and the log keeps referring to them
                // long after Swords to Plowshares removed them from the board.
                observation
                    .exiles
                    .iter()
                    .flatten()
                    .find_map(|(candidate, definition)| {
                        (*candidate == id).then_some(ObjectCharacteristics::card(
                            *definition,
                            penta::CardPartId::PRIMARY,
                        ))
                    })
            })
    }

    fn instance_definition(
        observation: &PlayerObservation,
        id: CardInstanceId,
    ) -> Option<CardDefinitionId> {
        Self::instance_characteristics(observation, id)
            .and_then(ObjectCharacteristics::card_definition)
    }

    pub(super) fn instance_name(
        &self,
        observation: &PlayerObservation,
        id: CardInstanceId,
    ) -> String {
        Self::instance_characteristics(observation, id).map_or_else(
            // A card that has since moved somewhere this observation cannot
            // read — shuffled back into a library, say — is still described
            // in words rather than as a raw instance id.
            || "a card".into(),
            |characteristics| object_presentation(&self.catalog, characteristics).name,
        )
    }

    fn play_option_label(
        &self,
        observation: &PlayerObservation,
        card: CardInstanceId,
        option: PlayOptionId,
    ) -> Option<String> {
        Self::instance_definition(observation, card)
            .and_then(|definition| self.catalog.get(definition))
            .and_then(|definition| definition.play_option(option))
            .map(|option| option.label.clone())
    }

    fn mode_labels(
        &self,
        observation: &PlayerObservation,
        card: CardInstanceId,
        option: PlayOptionId,
        modes: &[ModeId],
    ) -> Vec<String> {
        let mode_definitions = Self::instance_definition(observation, card)
            .and_then(|definition| self.catalog.get(definition))
            .and_then(|definition| definition.play_option(option))
            .and_then(|option| option.modes.as_ref());
        modes
            .iter()
            .map(|id| {
                mode_definitions
                    .and_then(|definitions| definitions.modes.iter().find(|mode| mode.id == *id))
                    .map_or_else(|| format!("Mode {}", id.0), |mode| mode.label.clone())
            })
            .collect()
    }

    /// Target- and X-independent label for one ordinary activation. A source
    /// with a single currently legal ability keeps the compact menu label;
    /// when several distinct exact origins are legal, their current rules
    /// text keeps actions and opponent beats distinguishable without authored
    /// UI copy.
    fn activation_label(
        &self,
        observation: &PlayerObservation,
        source: CardInstanceId,
        ability: AbilityOrigin,
        modes: &[ModeId],
    ) -> String {
        let source_name = self.instance_name(observation, source);
        let mut label = if source_has_multiple_activated_abilities(observation, source)
            && let Some(text) = self.ability_rules_text(source, ability)
        {
            format!("{source_name} — {text}")
        } else {
            format!("Activate {source_name}")
        };
        // The modes are chosen as the ability is activated, so one printed
        // ability offers one action per selection. Without the chosen mode
        // named, they would all read alike.
        let chosen = self.activated_mode_labels(source, ability, modes);
        if !chosen.is_empty() {
            let _ = write!(label, " — {}", chosen.join(" + "));
        }
        label
    }

    /// Names each mode an activation chose, reading the printed clause off
    /// the ability itself rather than the catalog: an ability's modes are
    /// not a play option's.
    fn activated_mode_labels(
        &self,
        source: CardInstanceId,
        ability: AbilityOrigin,
        modes: &[ModeId],
    ) -> Vec<String> {
        let Some(modal) = self
            .session
            .ability_for_origin(source, ability)
            .and_then(AbilityDef::modal)
        else {
            return Vec::new();
        };
        modes
            .iter()
            .map(|id| {
                modal.modes.get(id.index()).map_or_else(
                    || format!("Mode {}", id.0),
                    |mode| mode.rules_text().into_owned(),
                )
            })
            .collect()
    }

    pub(super) fn action_ability_label(
        &self,
        observation: &PlayerObservation,
        action: &Action,
    ) -> Option<String> {
        match action {
            Action::ActivateAbility {
                source,
                ability,
                modes,
                ..
            } => Some(self.activation_label(observation, *source, *ability, modes)),
            Action::KeepHand
            | Action::TakeMulligan
            | Action::BottomCards { .. }
            | Action::DiscardCards { .. }
            | Action::ChooseDecision { .. }
            | Action::CancelDecision { .. }
            | Action::ChooseUntap { .. }
            | Action::TurnFaceUp { .. }
            | Action::Foretell { .. }
            | Action::UnlockDoor { .. }
            | Action::PassPriority
            | Action::PlayLand { .. }
            | Action::ActivateManaAbility { .. }
            | Action::PayLifeForMana
            | Action::CastSpell { .. }
            | Action::DeclareAttacker { .. }
            | Action::BandAttackers { .. }
            | Action::FinishDeclaringAttackers
            | Action::DeclareBlocker { .. }
            | Action::FinishDeclaringBlockers
            | Action::AssignCombatDamage { .. }
            | Action::Concede => None,
        }
    }

    fn ability_rules_text(&self, source: CardInstanceId, origin: AbilityOrigin) -> Option<String> {
        self.session
            .ability_for_origin(source, origin)
            .map(|ability| ability.rules_text().into_owned())
    }

    fn target_name(&self, observation: &PlayerObservation, target: Target) -> String {
        match target {
            Target::Player(player) if player == self.human => "you".into(),
            Target::Player(_) => "opponent".into(),
            Target::Card(id) | Target::Permanent(id) => self.instance_name(observation, id),
            // A countered or resolved spell leaves no trace the observation can
            // name, and stack object ids are not card ids, so the log says what
            // it honestly knows rather than printing a raw id.
            Target::Spell(id) => observation
                .stack
                .iter()
                .find(|object| object.id == id)
                .map_or_else(
                    || "a spell".into(),
                    |object| object_presentation(&self.catalog, object.characteristics).name,
                ),
        }
    }

    fn player_name(&self, player: PlayerId) -> &'static str {
        if player == self.human {
            "You"
        } else {
            "Opponent"
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn event_label(
        &self,
        observation: &PlayerObservation,
        event: &GameEvent,
    ) -> Option<String> {
        match event {
            GameEvent::CardDrawn { player, card } if *player == self.human => Some(format!(
                "You drew {}",
                self.instance_name(observation, *card)
            )),
            GameEvent::CardDrawn { .. } => Some("Opponent drew a card".into()),
            GameEvent::CardRevealed {
                player, definition, ..
            } => Some(format!(
                "{} revealed {}",
                self.player_name(*player),
                self.card_name(*definition)
            )),
            GameEvent::CardsDiscarded { player, cards } => Some(format!(
                "{} discarded {}",
                self.player_name(*player),
                cards
                    .iter()
                    .map(|(_, definition)| self.card_name(*definition))
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
            GameEvent::LandPlayed {
                player, definition, ..
            } => Some(format!(
                "{} played {}",
                self.player_name(*player),
                self.card_name(*definition)
            )),
            GameEvent::SpellCast {
                player,
                definition,
                targets,
                ..
            } => {
                let mut label = format!(
                    "{} cast {}",
                    self.player_name(*player),
                    self.card_name(*definition)
                );
                if !targets.is_empty() {
                    let target_names = targets
                        .iter()
                        .map(|target| self.target_name(observation, *target))
                        .collect::<Vec<_>>()
                        .join(", ");
                    let _ = write!(label, " → {target_names}");
                }
                Some(label)
            }
            GameEvent::AbilityActivated {
                player,
                presentation,
                ..
            } => Some(format!(
                "{} activated {}",
                self.player_name(*player),
                object_presentation(&self.catalog, *presentation).name
            )),
            GameEvent::AbilityTriggered {
                player,
                presentation,
                ..
            } => Some(format!(
                "{} {} triggered",
                if *player == self.human {
                    "Your"
                } else {
                    "Opponent’s"
                },
                object_presentation(&self.catalog, *presentation).name
            )),
            GameEvent::AttackDeclared { player, attackers } => Some(format!(
                "{} attacked with {}",
                self.player_name(*player),
                attackers
                    .iter()
                    .map(|attacker| self.instance_name(observation, *attacker))
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
            GameEvent::BlockDeclared {
                player,
                assignments,
            } => Some(format!(
                "{} blocked {}",
                self.player_name(*player),
                assignments
                    .iter()
                    .map(|(blocker, attacker)| {
                        format!(
                            "{} with {}",
                            self.instance_name(observation, *attacker),
                            self.instance_name(observation, *blocker)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
            GameEvent::DamageDealt { player, amount } => Some(format!(
                "{} took {amount} damage",
                self.player_name(*player)
            )),
            GameEvent::LifeLost { player, amount } => {
                Some(format!("{} lost {amount} life", self.player_name(*player)))
            }
            GameEvent::ManaBurn { player, amount } => Some(format!(
                "{} took {amount} mana burn",
                self.player_name(*player)
            )),
            GameEvent::StepChanged {
                turn,
                active_player,
                step: Step::PrecombatMain,
            } => Some(format!(
                "Turn {} · {} turn",
                turn.div_ceil(2),
                if *active_player == self.human {
                    "your"
                } else {
                    "opponent’s"
                }
            )),
            GameEvent::SpellFizzled { definition, .. } => Some(format!(
                "{} fizzled — its target was gone",
                self.card_name(*definition)
            )),
            GameEvent::AbilityFizzled { presentation, .. }
            | GameEvent::TriggeredAbilityFizzled { presentation, .. } => Some(format!(
                "{} fizzled — its target was gone",
                object_presentation(&self.catalog, *presentation).name
            )),
            GameEvent::PermanentLeftBattlefield {
                controller,
                characteristics,
                destination,
                ..
            } => Some(format!(
                "{} {} {}",
                if *controller == self.human {
                    "Your"
                } else {
                    "Opponent’s"
                },
                object_presentation(&self.catalog, *characteristics).name,
                match destination {
                    BattlefieldExit::Graveyard => "was destroyed",
                    BattlefieldExit::Exile => "was exiled",
                    BattlefieldExit::Hand => "returned to hand",
                    BattlefieldExit::LibraryTop => "was put on top of its owner's library",
                    BattlefieldExit::LibraryBottom =>
                        "was put on the bottom of its owner's library",
                }
            )),
            GameEvent::GameEnded { result } => Some(match result {
                GameResult::Winner { winner, .. } if *winner == self.human => "You won".into(),
                GameResult::Winner { .. } => "Opponent won".into(),
                GameResult::Draw => "Game ended in a draw".into(),
            }),
            // GameStarted never reaches here: `events_for` withholds it, and
            // the seed line is added from the engine where the log is built.
            GameEvent::GameStarted { .. }
            | GameEvent::ManaAdded { .. }
            | GameEvent::SpellResolved { .. }
            | GameEvent::AbilityResolved { .. }
            | GameEvent::TriggeredAbilityPutOnStack { .. }
            | GameEvent::TriggeredAbilityResolved { .. }
            | GameEvent::StepChanged { .. } => None,
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn action_label(&self, observation: &PlayerObservation, action: &Action) -> String {
        let targets = |values: &[Target]| {
            values
                .iter()
                .map(|target| self.target_name(observation, *target))
                .collect::<Vec<_>>()
                .join(", ")
        };
        match action {
            Action::KeepHand => "Keep this hand".into(),
            Action::TakeMulligan => "Take a mulligan".into(),
            Action::BottomCards { cards } => format!(
                "Bottom {}",
                cards
                    .iter()
                    .map(|id| self.instance_name(observation, *id))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Action::DiscardCards { cards } => format!(
                "Discard {}",
                cards
                    .iter()
                    .map(|id| self.instance_name(observation, *id))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Action::ChooseDecision { options, .. } => {
                let labels = observation
                    .decision
                    .as_ref()
                    .map_or_else(Vec::new, |decision| {
                        decision
                            .options
                            .iter()
                            .filter(|option| options.contains(&option.id))
                            .map(|option| option.label.clone())
                            .collect::<Vec<_>>()
                    });
                if labels.is_empty() {
                    // The engine also enumerates a bare schema placeholder for
                    // the pending decision; never hand the browser a blank
                    // label it could render as an unlabelled control.
                    "Choose an option".into()
                } else {
                    labels.join(", ")
                }
            }
            Action::CancelDecision { .. } => "Cancel".into(),
            Action::TurnFaceUp { permanent } => format!(
                "Turn {} face up",
                self.instance_name(observation, *permanent)
            ),
            Action::Foretell { card } => {
                format!("Foretell {}", self.instance_name(observation, *card))
            }
            // Named for the door rather than for the Room: "Unlock Forgotten
            // Cellar" says which half is being bought, and the Room's own
            // name is the pair of them.
            Action::UnlockDoor { room, door } => format!(
                "Unlock {}",
                Self::instance_definition(observation, *room)
                    .and_then(|definition| self.catalog.get(definition))
                    .and_then(|card| card.part(*door))
                    .map_or_else(
                        || self.instance_name(observation, *room),
                        |part| part.name.clone()
                    )
            ),
            Action::ChooseUntap { permanents } => format!(
                "Untap {}",
                permanents
                    .iter()
                    .map(|id| self.instance_name(observation, *id))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Action::PassPriority => "Pass priority".into(),
            Action::PlayLand { card, option } => {
                let option = self
                    .play_option_label(observation, *card, *option)
                    .unwrap_or_else(|| self.instance_name(observation, *card));
                format!("Play {option}")
            }
            Action::ActivateManaAbility {
                source,
                color,
                cost_object,
                combination,
                ..
            } => {
                // "Add three in any combination" is offered once per
                // division, and `color` names only the first type each one
                // produces -- so the division is what the label has to name,
                // or the choices read as duplicates.
                if let Some(division) = combination {
                    return format!(
                        "Activate {} for {} mana",
                        self.instance_name(observation, *source),
                        division
                            .iter()
                            .map(|(color, amount)| format!("{amount} {}", readable_debug(color)))
                            .collect::<Vec<_>>()
                            .join(" and ")
                    );
                }
                // An ability that sacrifices some other permanent is offered
                // once per candidate, and the source and colour are the same
                // every time -- so the sacrifice is what the label has to
                // name, or the choices read as duplicates.
                match cost_object {
                    Some(sacrificed) => format!(
                        "Sacrifice {} for {} mana",
                        self.instance_name(observation, *sacrificed),
                        readable_debug(*color)
                    ),
                    None => format!(
                        "Tap {} for {} mana",
                        self.instance_name(observation, *source),
                        readable_debug(*color)
                    ),
                }
            }
            Action::PayLifeForMana => "Pay 1 life for 1 colorless mana".into(),
            Action::CastSpell {
                card,
                choices,
                sacrifices,
            } => {
                let option = self
                    .play_option_label(observation, *card, choices.play_option())
                    .unwrap_or_else(|| self.instance_name(observation, *card));
                let mut label = format!("Cast {option}");
                let cast_option = Self::instance_definition(observation, *card)
                    .and_then(|definition| self.catalog.get(definition))
                    .and_then(|definition| definition.play_option(choices.play_option()));
                if let Some(alternative) = choices.costs().alternative() {
                    let printed = cast_option.and_then(|option| {
                        option
                            .alternative_costs
                            .iter()
                            .find(|cost| cost.id == alternative)
                    });
                    let alternative_label = printed.map_or("Flashback", |cost| cost.label.as_str());
                    let _ = write!(label, " via {alternative_label}");
                    if let Some(cost) = printed
                        .map(|cost| cost.mana_cost)
                        .or_else(|| cast_option.and_then(|option| option.mana_cost))
                    {
                        let _ = write!(label, " {}", mana_cost_label(cost));
                    }
                }
                for selected in choices.costs().additional() {
                    if let Some(additional) = cast_option.and_then(|option| {
                        option
                            .additional_costs
                            .iter()
                            .find(|cost| cost.id == *selected)
                    }) {
                        let _ = write!(label, " with {}", additional.label);
                        if let Some(cost) = additional.mana_cost {
                            let _ = write!(label, " {}", mana_cost_label(cost));
                        }
                    }
                }
                let modes =
                    self.mode_labels(observation, *card, choices.play_option(), choices.modes());
                if !modes.is_empty() {
                    let _ = write!(label, " — {}", modes.join(" + "));
                }
                if choices.x() > 0 {
                    let _ = write!(label, " (X={})", choices.x());
                }
                if !sacrifices.is_empty() {
                    let _ = write!(
                        label,
                        " (sacrifice {})",
                        sacrifices
                            .iter()
                            .map(|id| self.instance_name(observation, *id))
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }
                let values = choices.iter_targets().copied().collect::<Vec<_>>();
                if !values.is_empty() {
                    let _ = write!(label, " → {}", targets(&values));
                }
                label
            }
            Action::ActivateAbility {
                source,
                ability,
                targets: target_selections,
                cost_objects,
                x,
                modes,
            } => {
                let mut label = self.activation_label(observation, *source, *ability, modes);
                if source_ability_has_multiple_x_values(observation, *source, *ability) {
                    let _ = write!(label, " (X={x})");
                }
                // Two activations of one ability can differ only in which
                // objects the cost spends -- the Lavamancer's pairs of
                // graveyard cards -- so the label names all of them.
                let spent: Vec<String> = cost_objects
                    .iter()
                    .filter(|spent| *spent != source)
                    .map(|spent| self.instance_name(observation, *spent))
                    .collect();
                if !spent.is_empty() {
                    let _ = write!(label, " (sacrifice {})", spent.join(", "));
                }
                let values = target_selections
                    .iter()
                    .flat_map(penta::TargetSelection::targets)
                    .copied()
                    .collect::<Vec<_>>();
                if !values.is_empty() {
                    let _ = write!(label, " → {}", targets(&values));
                }
                label
            }
            Action::DeclareAttacker { attacker, .. } => {
                format!("Attack with {}", self.instance_name(observation, *attacker))
            }
            Action::BandAttackers { first, second } => format!(
                "Band {} with {}",
                self.instance_name(observation, *first),
                self.instance_name(observation, *second),
            ),
            // Naming the commitment reads better than naming the step: the
            // button is the last chance to see how big the attack is.
            Action::FinishDeclaringAttackers => {
                let declared = observation
                    .battlefield
                    .iter()
                    .filter(|permanent| {
                        permanent.controller == observation.viewer && permanent.attacking
                    })
                    .count();
                match declared {
                    0 => "No attacks".into(),
                    1 => "Attack with 1 creature".into(),
                    count => format!("Attack with {count} creatures"),
                }
            }
            Action::DeclareBlocker { blocker, attacker } => format!(
                "Block {} with {}",
                self.instance_name(observation, *attacker),
                self.instance_name(observation, *blocker)
            ),
            Action::FinishDeclaringBlockers => "Finish blocking".into(),
            // The attacker is already named in the prompt above these buttons,
            // so each option only has to say where the damage lands. Recipients
            // taking nothing are noise and stay out of the label.
            Action::AssignCombatDamage { assignments, .. } => {
                let landed = assignments
                    .iter()
                    .filter(|assignment| assignment.amount > 0)
                    .map(|assignment| {
                        format!(
                            "{} to {}",
                            assignment.amount,
                            self.target_name(observation, assignment.recipient)
                        )
                    })
                    .collect::<Vec<_>>();
                if landed.is_empty() {
                    "Deal no damage".into()
                } else {
                    landed.join(", ")
                }
            }
            Action::Concede => "Concede game".into(),
        }
    }

    pub(super) fn opponent_action_label(
        &self,
        observation: &PlayerObservation,
        action: &Action,
    ) -> String {
        match action {
            Action::BottomCards { cards } => format!(
                "Bottom {} {}",
                cards.len(),
                if cards.len() == 1 { "card" } else { "cards" }
            ),
            // Only the human's own pending decision has option labels this
            // observation can read. Anything else the opponent chose stays
            // private, including when the human is mid-decision themselves.
            Action::ChooseDecision { decision, .. }
                if observation
                    .decision
                    .as_ref()
                    .is_none_or(|visible| visible.id != *decision) =>
            {
                "Opponent made a private choice".into()
            }
            _ => self.action_label(observation, action),
        }
    }
}
