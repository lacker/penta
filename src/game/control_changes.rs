//! Control changes sustained by a permanent.
//!
//! Turn-scoped theft lives in the shared layer-2 attachment machinery too.
//! This module records the source-sustained duration used by Aladdin, Thrull
//! Champion, Rubinia Soulsinger, and Willow Satyr.

use super::{
    EffectRecipientDef, Game, GameObjectId, ScopedEffect, StackObject, Target, TriggerContext,
    WhileSourceControl,
};

impl Game {
    /// Records a timestamped control effect for as long as `holder` remains
    /// under the resolving ability's controller, and optionally stays tapped.
    pub(super) fn take_control_of(
        &mut self,
        recipient: EffectRecipientDef,
        object: &StackObject,
        context: TriggerContext,
        scoped: ScopedEffect,
        holder: Option<(GameObjectId, bool)>,
    ) {
        let Some((source, requires_source_tapped)) = holder else {
            let targets = self
                .effect_recipients(recipient, object, context, scoped)
                .into_iter()
                .filter_map(|target| match target {
                    Target::Permanent(id) => Some(id),
                    Target::Card(_) | Target::Player(_) | Target::Spell(_) => None,
                })
                .collect::<Vec<_>>();
            self.gain_control_until_end_of_turn(&targets, object.controller);
            return;
        };
        let controller = object.controller;
        let mut eligible = Vec::new();
        for target in self.effect_recipients(recipient, object, context, scoped) {
            let Target::Permanent(id) = target else {
                continue;
            };
            let Some(index) = self
                .battlefield
                .iter()
                .position(|permanent| permanent.card.id == id)
            else {
                continue;
            };
            // A redundant control effect still gets its own timestamp. It can
            // become visible after a newer effect ends, so discard it only
            // when it would actually change control and that change is
            // prohibited.
            if self.battlefield[index].controller != controller
                && self.cannot_change_controller(&self.battlefield[index])
            {
                continue;
            }
            if !eligible.contains(&id) {
                eligible.push(id);
            }
        }
        if eligible.is_empty() {
            return;
        }
        let timestamp = self.allocate_continuous_effect_timestamp();
        for target in eligible {
            self.battlefield
                .iter_mut()
                .find(|permanent| permanent.card.id == target)
                .expect("an eligible control target remains on the battlefield")
                .control_while_source_remains
                .push(WhileSourceControl {
                    timestamp,
                    controller,
                    source,
                    requires_source_tapped,
                });
        }
        self.reconcile_all_control_layers();
    }
}
