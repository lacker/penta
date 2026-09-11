//! Answering the decisions that put a triggered ability on the stack: which
//! ability resolves first, what it targets, and how a fixed total is split
//! among those targets.

use super::super::{
    DecisionContinuation, DividedTotal, Game, TargetSelection, TargetSlotId, positive_compositions,
};

impl Game {
    pub(super) fn complete_trigger_continuation(
        &mut self,
        continuation: DecisionContinuation,
        options: &[u32],
    ) {
        match continuation {
            DecisionContinuation::TriggerOrder { batch, remaining } => {
                self.complete_trigger_order(&batch, remaining, options);
            }
            DecisionContinuation::TriggerPlacement {
                mut trigger,
                pending,
                remaining,
                candidates,
            } => {
                let target_index = trigger.targets.len();
                let selected = options
                    .iter()
                    .filter_map(|option| {
                        usize::try_from(*option)
                            .ok()
                            .and_then(|index| candidates.get(index))
                            .copied()
                    })
                    .collect::<Vec<_>>();
                let slot = TargetSlotId::from_index(target_index)
                    .expect("validated trigger targets fit the runtime slot space");
                let total = trigger
                    .target_defs
                    .get(target_index)
                    .and_then(|target| target.divided_total)
                    .map(|total| match total {
                        DividedTotal::Fixed(total) => u16::from(total),
                        DividedTotal::ChosenX => trigger.x,
                    });
                match total {
                    // Every target takes at least one, so with a single
                    // target there is nothing left to ask.
                    Some(total) if selected.len() > 1 => {
                        let divisions = positive_compositions(
                            u8::try_from(total).unwrap_or(u8::MAX),
                            selected.len(),
                        );
                        self.queue_trigger_division_decision(
                            trigger, pending, remaining, selected, divisions,
                        );
                        return;
                    }
                    Some(total) => {
                        let amounts = vec![total; selected.len()];
                        trigger
                            .targets
                            .push(TargetSelection::divided(slot, selected, amounts));
                    }
                    None => trigger.targets.push(TargetSelection::new(slot, selected)),
                }
                let mut continued = vec![trigger];
                continued.extend(pending);
                self.place_trigger_sequence(continued, remaining);
            }
            DecisionContinuation::TriggerMode {
                mut trigger,
                pending,
                remaining,
                modes,
            } => {
                if let Some(chosen) = options
                    .first()
                    .and_then(|option| usize::try_from(*option).ok())
                {
                    Self::apply_trigger_mode(&mut trigger, *modes, chosen);
                }
                let mut continued = vec![trigger];
                continued.extend(pending);
                self.place_trigger_sequence(continued, remaining);
            }
            DecisionContinuation::TriggerDivision {
                mut trigger,
                pending,
                remaining,
                targets,
                divisions,
            } => {
                let amounts = options
                    .first()
                    .and_then(|option| usize::try_from(*option).ok())
                    .and_then(|index| divisions.get(index))
                    .cloned()
                    .unwrap_or_else(|| vec![1; targets.len()]);
                let slot = TargetSlotId::from_index(trigger.targets.len())
                    .expect("validated trigger targets fit the runtime slot space");
                trigger
                    .targets
                    .push(TargetSelection::divided(slot, targets, amounts));
                let mut continued = vec![trigger];
                continued.extend(pending);
                self.place_trigger_sequence(continued, remaining);
            }
            _ => unreachable!("only trigger continuations reach this arm"),
        }
    }
}
