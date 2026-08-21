use super::{Action, BOT_ACTION_LIMIT, DecisionKind, PlayerObservation, Step, WebGame};
use crate::presentation::object_presentation;

impl WebGame {
    pub(super) fn automatic_human_action_for(
        &self,
        observation: &PlayerObservation,
    ) -> Option<Action> {
        if self.autopass_enabled
            && let Some(decision) = observation.decision.as_ref()
            && decision.kind == DecisionKind::Choice
            && decision.minimum == 1
            && decision.maximum == 1
            && decision.options.len() == 1
            && !decision.prompt.starts_with("Erhnam Djinn")
        {
            return Some(Action::ChooseDecision {
                decision: decision.id,
                options: vec![decision.options[0].id],
            });
        }
        automatic_human_action_for_context(
            AutoPassContext {
                step: observation.step,
                regular_combat_damage_pending: observation.regular_combat_damage_pending,
                human_is_active: observation.active_player == self.human,
                stack_is_empty: observation.stack.is_empty(),
                has_attacker: observation
                    .battlefield
                    .iter()
                    .any(|permanent| permanent.attacking),
                has_blocker: observation
                    .battlefield
                    .iter()
                    .any(|permanent| !permanent.blocking.is_empty()),
                stop_here: self.should_stop(observation.step),
                autopass_enabled: self.autopass_enabled,
                only_human_objects_on_stack: !observation.stack.is_empty()
                    && observation
                        .stack
                        .iter()
                        .all(|object| object.controller == self.human),
                human_has_floating_mana: observation.mana_pools[self.human.index()].total() > 0,
            },
            &observation.legal_actions,
        )
    }

    /// Where a pass lands if the opponent takes no action of their own.
    ///
    /// A conditional promise about the rules, not a prediction about the
    /// opponent. The walk declines every optional action for them and takes
    /// only the null ones -- pass the window, finish a declaration having
    /// declared nothing -- so the destination is what the turn structure
    /// produces when nobody responds. Anything they really do stops the pass
    /// early, and the replay beats show it.
    ///
    /// The promise ends where their discretion could change which step the
    /// human next acts in. That is their declare-attackers step and nothing
    /// else: whether they block does not move the human's next window, but
    /// whether they attack decides whether there is a block step at all.
    pub(super) fn pass_preview_label(&self) -> Option<String> {
        if self.session.decision_seat() != Some(self.human) {
            return None;
        }
        let observation = self.session.observe(self.human);
        if !observation
            .legal_actions
            .iter()
            .any(|action| matches!(action, Action::PassPriority))
        {
            return None;
        }
        if let Some(top) = observation.stack.last() {
            return Some(format!(
                "Resolve {}",
                object_presentation(&self.catalog, top.characteristics).name
            ));
        }
        let start_turn = observation.turn;
        let start_active_is_human = observation.active_player == self.human;
        let mut sim = self.session.fork_for_preview();
        sim.apply(self.human, Action::PassPriority).ok()?;
        // Combat damage is the loudest thing a pass can cause, so it names the
        // button even when the yield carries on past it. Watch both attackers
        // in a pre-damage step and the explicit inter-wave discriminator, then
        // keep that fact as the simulation advances through the damage event.
        let mut combat_damage_pending = Self::combat_damage_awaiting(&observation);
        let mut deals_combat_damage = false;
        // Every exit below labels the step the simulation reached, so a pass
        // that runs the game out (lethal damage) or that the preview cannot
        // carry further still names a destination instead of falling back to
        // the bare "Pass priority".
        for _ in 0..BOT_ACTION_LIMIT {
            let Some(player) = sim.decision_player() else {
                break;
            };
            let sim_observation = sim.observe(player);
            // Only this turn's combat is the pass's doing; an attack a whole
            // turn away is not what the button is about to cause.
            combat_damage_pending |= sim_observation.turn == start_turn
                && Self::combat_damage_awaiting(&sim_observation);
            if combat_damage_pending
                && (sim_observation.turn != start_turn
                    || matches!(
                        sim_observation.step,
                        Step::CombatDamage
                            | Step::EndOfCombat
                            | Step::PostcombatMain
                            | Step::End
                            | Step::Cleanup
                    ))
            {
                deals_combat_damage = true;
            }
            let action = if player == self.human {
                match self.automatic_human_action_for(&sim_observation) {
                    Some(action) => action,
                    None => break,
                }
            } else if Self::opponent_holds_the_attack_decision(&sim_observation) {
                break;
            } else if let Some(action) = opponent_declines_action(&sim_observation) {
                action
            } else {
                // A real decision of theirs is where the human ends up waiting.
                break;
            };
            if sim.apply(player, action).is_err() {
                break;
            }
        }
        let ending = sim.observe(self.human);
        // Taking an attack unblocked is a commitment, not a destination, so on
        // defense the button names the decision the same way "No attacks" does.
        if !start_active_is_human && Self::declines_all_blocks(&observation, &ending, start_turn) {
            return Some("No blocks".into());
        }
        if deals_combat_damage {
            return Some("Go to damage".into());
        }
        Some(Self::pass_destination_label(
            &ending,
            start_turn,
            start_active_is_human,
        ))
    }

    /// Whether the opponent is at the one window whose outcome decides which
    /// step the human next acts in. Guessing here is what the preview must
    /// not do: predicting the attack promises a block step, and predicting no
    /// attack promises their end step, and both are the button answering a
    /// question that is theirs.
    fn opponent_holds_the_attack_decision(observation: &PlayerObservation) -> bool {
        observation.step == Step::DeclareAttackers
            && observation
                .legal_actions
                .iter()
                .any(|action| matches!(action, Action::DeclareAttacker { .. }))
    }

    /// Whether this pass carries the defender through the block step without
    /// blocking anything: attackers are in, nothing of yours is blocking, and
    /// the simulation runs out the other side of blockers still that way.
    fn declines_all_blocks(
        before: &PlayerObservation,
        after: &PlayerObservation,
        start_turn: u32,
    ) -> bool {
        let blocking = |observation: &PlayerObservation| {
            observation
                .battlefield
                .iter()
                .any(|permanent| !permanent.blocking.is_empty())
        };
        matches!(
            before.step,
            Step::BeginningOfCombat | Step::DeclareAttackers | Step::DeclareBlockers
        ) && before
            .battlefield
            .iter()
            .any(|permanent| permanent.attacking)
            && !blocking(before)
            && !blocking(after)
            && (after.turn != start_turn
                || matches!(
                    after.step,
                    Step::CombatDamage | Step::EndOfCombat | Step::PostcombatMain | Step::End
                ))
    }

    /// Attackers are declared and the damage step has not happened yet.
    pub(super) fn attack_awaiting_damage(observation: &PlayerObservation) -> bool {
        matches!(
            observation.step,
            Step::DeclareAttackers | Step::DeclareBlockers
        ) && observation
            .battlefield
            .iter()
            .any(|permanent| permanent.attacking)
    }

    /// Combat is awaiting either its first damage wave or the regular wave
    /// after first-strike damage. The latter cannot be inferred from `step`
    /// because both waves are exposed as `CombatDamage`.
    ///
    /// Either way something has to be left to deal the damage: first strike
    /// can kill every attacker, and a wave with nothing in it is not a
    /// destination worth naming the button after.
    pub(super) fn combat_damage_awaiting(observation: &PlayerObservation) -> bool {
        (observation.regular_combat_damage_pending
            && observation
                .battlefield
                .iter()
                .any(|permanent| permanent.attacking))
            || Self::attack_awaiting_damage(observation)
    }

    fn pass_destination_label(
        observation: &PlayerObservation,
        start_turn: u32,
        start_active_is_human: bool,
    ) -> String {
        // Name what happens next rather than the step the rules call it. The
        // simulation above already settled where the pass lands, so these
        // read as a promise the click has to keep.
        if observation.turn != start_turn {
            return if start_active_is_human {
                "End turn".into()
            } else {
                "Your turn".into()
            };
        }
        // The same step means different things depending on whose turn it is:
        // "Go to attacks" is a promise to attack, not a warning to block.
        if start_active_is_human {
            match observation.step {
                Step::Upkeep => "Go to upkeep",
                Step::Draw => "Draw a card",
                Step::PrecombatMain => "Go to main phase",
                Step::BeginningOfCombat | Step::DeclareAttackers => "Go to attacks",
                Step::DeclareBlockers => "Go to blocks",
                Step::CombatDamage => "Go to damage",
                Step::EndOfCombat => "Go to end of combat",
                Step::PostcombatMain => "Go to second main",
                Step::End => "Go to end step",
                // The only reason to hold priority in cleanup is a full hand.
                Step::Cleanup => "Discard down to seven",
            }
        } else {
            match observation.step {
                Step::Upkeep => "Go to their upkeep",
                Step::Draw => "Go to their draw",
                Step::PrecombatMain => "Go to their main phase",
                Step::BeginningOfCombat | Step::DeclareAttackers => "Go to their attack",
                Step::DeclareBlockers => "Go to blocks",
                Step::CombatDamage => "Go to damage",
                Step::EndOfCombat => "Go to end of combat",
                Step::PostcombatMain => "Go to their second main",
                Step::End => "Go to their end step",
                Step::Cleanup => "Go to cleanup",
            }
        }
        .into()
    }

    fn should_stop(&self, step: Step) -> bool {
        let phase = match step {
            Step::Upkeep | Step::Draw => "Beginning",
            Step::PrecombatMain => "Main 1",
            Step::BeginningOfCombat
            | Step::DeclareAttackers
            | Step::DeclareBlockers
            | Step::CombatDamage
            | Step::EndOfCombat => "Combat",
            Step::PostcombatMain => "Main 2",
            Step::End | Step::Cleanup => "Ending",
        };
        self.phase_stops.iter().any(|candidate| candidate == phase)
    }
}

#[derive(Clone, Copy)]
// These flags are independent policy inputs, not a state machine; keeping
// them named makes the Arena-style priority rules auditable at the call site.
#[allow(clippy::struct_excessive_bools)]
pub(super) struct AutoPassContext {
    pub(super) step: Step,
    pub(super) regular_combat_damage_pending: bool,
    pub(super) human_is_active: bool,
    pub(super) stack_is_empty: bool,
    pub(super) has_attacker: bool,
    pub(super) has_blocker: bool,
    pub(super) stop_here: bool,
    pub(super) autopass_enabled: bool,
    pub(super) only_human_objects_on_stack: bool,
    pub(super) human_has_floating_mana: bool,
}

/// The pass-preview stand-in for the opponent: let the pass through wherever
/// possible, and resolve forced decisions (such as a cleanup discard) with an
/// arbitrary minimal selection so the preview can keep moving. Returns `None`
/// when the opponent holds a choice the preview cannot neutrally guess at.
/// The opponent doing nothing of their own: pass the window, finish a
/// declaration having declared nothing, take a forced discard.
///
/// `None` where they hold a real decision, which is where the promise runs
/// out. Nothing here chooses on their behalf. Answering a decision for them,
/// or declaring their attackers, would be a guess that the button then
/// reports back to the human as though it were known.
fn opponent_declines_action(observation: &PlayerObservation) -> Option<Action> {
    if observation.decision.is_some() {
        return None;
    }
    observation
        .legal_actions
        .iter()
        .find(|action| {
            matches!(
                action,
                Action::PassPriority
                    | Action::FinishDeclaringAttackers
                    | Action::FinishDeclaringBlockers
            )
        })
        .or_else(|| {
            // A forced cleanup discard: any card works as a stand-in, since
            // the preview only reports where the human ends up waiting.
            observation
                .legal_actions
                .iter()
                .find(|action| matches!(action, Action::DiscardCards { .. }))
        })
        .cloned()
}

/// Whether this priority window is one the browser hands back without asking.
fn is_routine_window(context: &AutoPassContext, actions: &[Action]) -> bool {
    // Arena normally hides routine beginning-phase priority windows on either
    // turn. Stops restore them; floating mana in the end step is a
    // smart-priority case.
    let routine_beginning_step = matches!(context.step, Step::Upkeep | Step::Draw);
    // A legal cast, land play, or non-mana activated ability is worth holding
    // second main open for. Mana abilities alone stay routine, or every
    // untapped land would force an otherwise empty stop.
    let has_second_main_action = actions.iter().any(|action| {
        matches!(
            action,
            Action::CastSpell { .. } | Action::PlayLand { .. } | Action::ActivateAbility { .. }
        )
    });
    // Damage ordinarily lands on the way into the damage step, so by the time
    // anyone holds priority there it is history. First strike is the exception:
    // after its damage, players receive priority before the regular damage
    // step, and spells or abilities can still change that second wave.
    let combat_is_settled = context.step == Step::EndOfCombat
        || (context.step == Step::CombatDamage && !context.regular_combat_damage_pending);
    let routine_own_turn_step = context.human_is_active
        && (context.step == Step::BeginningOfCombat
            || (context.step == Step::PostcombatMain && !has_second_main_action)
            || (context.step == Step::End && !context.human_has_floating_mana));
    // On the opponent's turn the interesting window is their end step, where
    // instants are cheapest. Combat they never committed to, and their second
    // main, are both worth skipping to get there.
    let routine_opponent_turn_step = !context.human_is_active
        && (context.step == Step::BeginningOfCombat
            || context.step == Step::PostcombatMain
            || (!context.has_attacker
                && matches!(context.step, Step::DeclareAttackers | Step::DeclareBlockers)));
    let smooth_unblocked_attack = context.human_is_active
        && context.has_attacker
        && !context.has_blocker
        && matches!(context.step, Step::DeclareAttackers | Step::DeclareBlockers);
    // Nothing is blocking, so the rest of their combat is just watching the
    // damage land. The interesting window is their end step, so head there
    // instead of stopping once per remaining combat step. A block still on
    // offer means the decision has not been made yet and this is not it.
    let block_still_available = actions
        .iter()
        .any(|action| matches!(action, Action::DeclareBlocker { .. }));
    let smooth_unblocked_defense = !context.human_is_active
        && context.has_attacker
        && !context.has_blocker
        && !block_still_available
        && context.step == Step::DeclareBlockers;
    routine_beginning_step
        || combat_is_settled
        || routine_own_turn_step
        || routine_opponent_turn_step
        || smooth_unblocked_attack
        || smooth_unblocked_defense
        || (!context.has_attacker
            && matches!(context.step, Step::DeclareAttackers | Step::DeclareBlockers))
}

pub(super) fn automatic_human_action_for_context(
    context: AutoPassContext,
    actions: &[Action],
) -> Option<Action> {
    if !context.autopass_enabled {
        return None;
    }
    let has_attack_option = actions
        .iter()
        .any(|action| matches!(action, Action::DeclareAttacker { .. }));
    let empty_combat_after_attackers = !context.has_attacker
        && (matches!(
            context.step,
            Step::DeclareBlockers | Step::CombatDamage | Step::EndOfCombat
        ) || (context.step == Step::DeclareAttackers && !has_attack_option));
    if context.stop_here && !empty_combat_after_attackers {
        return None;
    }
    if context.only_human_objects_on_stack
        && let Some(pass) = actions
            .iter()
            .find(|action| matches!(action, Action::PassPriority))
    {
        return Some(pass.clone());
    }
    // Never fast-forward through the human's entire turn. Even with no card
    // actions available, Main 1 is the stable point where the player can see
    // the draw and deliberately advance into combat.
    if context.human_is_active && context.step == Step::PrecombatMain && context.stack_is_empty {
        return None;
    }
    let has_meaningful_choice = actions.iter().any(|action| {
        !matches!(
            action,
            Action::Concede
                | Action::PassPriority
                | Action::ActivateManaAbility { .. }
                | Action::FinishDeclaringAttackers
                | Action::FinishDeclaringBlockers
        )
    });
    let has_combat_ability = actions
        .iter()
        .any(|action| matches!(action, Action::ActivateAbility { .. }));
    let auto_yield_step = is_routine_window(&context, actions);
    // A combat ability is worth pausing for while it can still change the
    // outcome. Once damage is dealt, pumping a creature decides nothing.
    let combat_step = matches!(context.step, Step::DeclareAttackers | Step::DeclareBlockers);
    // Defending an attack nobody blocked is the exception: no creature of
    // yours is in the combat, so no ability of yours can change it.
    let ability_changes_combat = context.human_is_active || context.has_blocker;
    if auto_yield_step
        && (!combat_step || !context.has_attacker || !has_combat_ability || !ability_changes_combat)
        && context.stack_is_empty
        && let Some(pass) = actions
            .iter()
            .find(|action| matches!(action, Action::PassPriority))
    {
        return Some(pass.clone());
    }
    if has_meaningful_choice {
        return None;
    }
    // Committing an attack is the player's call. Running out of creatures to
    // declare is not the same as being finished, so once anything is attacking
    // the browser waits for them to confirm.
    if context.has_attacker && context.step == Step::DeclareAttackers {
        return None;
    }
    actions
        .iter()
        .find(|action| {
            matches!(
                action,
                Action::PassPriority
                    | Action::FinishDeclaringAttackers
                    | Action::FinishDeclaringBlockers
            )
        })
        .cloned()
}

#[cfg(test)]
#[allow(clippy::fn_params_excessive_bools, clippy::too_many_arguments)]
pub(super) fn automatic_human_action(
    step: Step,
    human_is_active: bool,
    stack_is_empty: bool,
    has_attacker: bool,
    stop_here: bool,
    autopass_enabled: bool,
    only_human_objects_on_stack: bool,
    human_has_floating_mana: bool,
    actions: &[Action],
) -> Option<Action> {
    automatic_human_action_for_context(
        AutoPassContext {
            step,
            regular_combat_damage_pending: false,
            human_is_active,
            stack_is_empty,
            has_attacker,
            has_blocker: false,
            stop_here,
            autopass_enabled,
            only_human_objects_on_stack,
            human_has_floating_mana,
        },
        actions,
    )
}

#[cfg(test)]
#[allow(clippy::fn_params_excessive_bools, clippy::too_many_arguments)]
pub(super) fn automatic_human_action_with_blockers(
    step: Step,
    human_is_active: bool,
    stack_is_empty: bool,
    has_attacker: bool,
    has_blocker: bool,
    stop_here: bool,
    autopass_enabled: bool,
    only_human_objects_on_stack: bool,
    human_has_floating_mana: bool,
    actions: &[Action],
) -> Option<Action> {
    automatic_human_action_for_context(
        AutoPassContext {
            step,
            regular_combat_damage_pending: false,
            human_is_active,
            stack_is_empty,
            has_attacker,
            has_blocker,
            stop_here,
            autopass_enabled,
            only_human_objects_on_stack,
            human_has_floating_mana,
        },
        actions,
    )
}
