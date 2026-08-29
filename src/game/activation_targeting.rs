//! Target choices made by somebody other than an ability's controller.
//!
//! Ordinary activations carry every target in their legal action. When a
//! printed clause hands one slot to another player, declaration pauses before
//! costs are paid, asks that player, then resumes the ordinary activation.

use super::ability_targeting::TargetingActors;
use super::{
    AbilityOrigin, AbilityTargetDef, ActivationChoices, DecisionContinuation, DecisionOption,
    DecisionPreference, DecisionVisibility, DecisionZone, Game, GameObjectId, ModeId, PlayerId,
    Target, TargetSelection, TargetSlotId, TriggerContext,
};
use crate::{ManaPaymentChoice, TargetChooserDef};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PendingActivationTargeting {
    pub(super) controller: PlayerId,
    pub(super) source: GameObjectId,
    pub(super) ability: AbilityOrigin,
    pub(super) target_defs: Vec<AbilityTargetDef>,
    pub(super) targets: Vec<TargetSelection>,
    pub(super) cost_objects: Vec<GameObjectId>,
    pub(super) x: u16,
    pub(super) modes: Vec<ModeId>,
    pub(super) mana_payment: Option<Box<ManaPaymentChoice>>,
}

impl Game {
    /// Ordinary activations retain their complete target action. A mixed-
    /// chooser activation enumerates only the controller-chosen prefix, then
    /// proves the first deferred slot has a legal answer before offering it.
    pub(super) fn legal_activation_target_selections(
        &self,
        slots: &[AbilityTargetDef],
        controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
        x: u16,
    ) -> Vec<Vec<TargetSelection>> {
        let Some(deferred) = slots
            .iter()
            .position(|slot| slot.chooser != TargetChooserDef::Controller)
        else {
            return self.legal_ability_target_selections(slots, controller, source, context, x);
        };
        let prefixes = self.legal_ability_target_selections(
            &slots[..deferred],
            controller,
            source,
            context,
            x,
        );
        let slot = slots[deferred];
        let chooser = Self::activation_target_chooser(controller, slot.chooser);
        prefixes
            .into_iter()
            .filter(|prefix| {
                let candidates = self.activation_target_candidates(
                    slot,
                    prefix,
                    TargetingActors::for_chooser(chooser, controller),
                    source,
                    context,
                    x,
                );
                let (minimum, _) = slot.count_bounds(x);
                candidates.len() >= usize::from(minimum)
            })
            .collect()
    }

    pub(super) fn begin_deferred_activation_targeting(
        &mut self,
        pending: PendingActivationTargeting,
    ) {
        if pending.targets.len() >= pending.target_defs.len() {
            self.resume_targeted_activation(pending);
            return;
        }
        let target_index = pending.targets.len();
        let slot = pending.target_defs[target_index];
        debug_assert_eq!(target_index + 1, pending.target_defs.len());
        debug_assert_eq!(slot.chooser, TargetChooserDef::Opponent);
        debug_assert_eq!(slot.count_bounds(pending.x), (1, 1));
        debug_assert!(slot.divided_total.is_none());
        let chooser = Self::activation_target_chooser(pending.controller, slot.chooser);
        let candidates = self.activation_target_candidates(
            slot,
            &pending.targets,
            TargetingActors::for_chooser(chooser, pending.controller),
            pending.source,
            TriggerContext::empty(),
            pending.x,
        );
        let (minimum, maximum) = slot.count_bounds(pending.x);
        if candidates.len() < usize::from(minimum) {
            // No cost has been paid and no stack object exists yet. The legal
            // action preflight prevents this for the first deferred slot; a
            // later dependent slot can still make the declaration impossible.
            return;
        }
        let options = candidates
            .iter()
            .enumerate()
            .map(|(index, target)| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: self.target_label(chooser, *target),
                card: self.target_card(*target),
                members: Vec::new(),
                ability_text: None,
                zone: match target {
                    Target::Player(_) => DecisionZone::None,
                    Target::Permanent(_) => DecisionZone::Battlefield,
                    Target::Spell(_) => DecisionZone::Stack,
                    Target::Card(id) => self.card_in_nonbattlefield_zone(*id).map_or(
                        DecisionZone::None,
                        |(zone, _)| match zone {
                            super::ZoneKind::Library => DecisionZone::Library,
                            super::ZoneKind::Hand => DecisionZone::Hand,
                            super::ZoneKind::Graveyard => DecisionZone::Graveyard,
                            super::ZoneKind::Exile => DecisionZone::Exile,
                            super::ZoneKind::Command => DecisionZone::Command,
                            super::ZoneKind::Battlefield | super::ZoneKind::Stack => {
                                DecisionZone::None
                            }
                        },
                    ),
                },
            })
            .collect::<Vec<_>>();
        self.queue_decision(
            chooser,
            "Choose a target for the activation",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            usize::from(minimum)..=usize::from(maximum).min(candidates.len()),
            false,
            options,
            DecisionContinuation::ActivationTargeting {
                pending: Box::new(pending),
                candidates,
            },
        );
        if let Some(decision) = self.pending_decisions.last_mut()
            && let DecisionContinuation::ActivationTargeting { pending, .. } =
                &decision.continuation
        {
            decision.observation.source = Some(pending.source);
        }
    }

    pub(super) fn continue_deferred_activation_targeting(
        &mut self,
        mut pending: PendingActivationTargeting,
        candidates: &[Target],
        options: &[u32],
    ) {
        let selected = options
            .iter()
            .filter_map(|option| {
                usize::try_from(*option)
                    .ok()
                    .and_then(|index| candidates.get(index))
                    .copied()
            })
            .collect::<Vec<_>>();
        let target_index = pending.targets.len();
        let slot_id = TargetSlotId::from_index(target_index)
            .expect("validated activation targets fit the runtime slot space");
        pending
            .targets
            .push(TargetSelection::new(slot_id, selected));
        self.begin_deferred_activation_targeting(pending);
    }

    fn activation_target_candidates(
        &self,
        slot: AbilityTargetDef,
        prefix: &[TargetSelection],
        actors: TargetingActors,
        source: GameObjectId,
        context: TriggerContext,
        x: u16,
    ) -> Vec<Target> {
        let mut candidates = Self::without_excluded_source(
            &slot,
            source,
            self.activated_targets_matching_for_chooser(
                slot.predicate,
                prefix,
                actors,
                source,
                context,
                x,
            ),
        );
        if slot.another {
            candidates.retain(|candidate| {
                !prefix
                    .iter()
                    .flat_map(TargetSelection::targets)
                    .any(|earlier| earlier == candidate)
            });
        }
        candidates
    }

    const fn activation_target_chooser(
        controller: PlayerId,
        chooser: TargetChooserDef,
    ) -> PlayerId {
        match chooser {
            TargetChooserDef::Controller | TargetChooserDef::EventPlayer => controller,
            TargetChooserDef::Opponent => controller.opponent(),
        }
    }

    fn resume_targeted_activation(&mut self, pending: PendingActivationTargeting) {
        let PendingActivationTargeting {
            controller,
            source,
            ability,
            targets,
            cost_objects,
            x,
            modes,
            mana_payment,
            ..
        } = pending;
        self.activate_ability(
            controller,
            source,
            ability,
            ActivationChoices {
                targets,
                cost_objects: &cost_objects,
                x,
                modes: &modes,
                mana_payment: mana_payment.as_deref(),
            },
        );
    }
}
