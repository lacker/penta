//! The payment interpretation of ordinary effect programs. Selection is
//! strict here: an effect may do as much as possible, but a cost must be whole.

use super::CostPaymentWindow;
use crate::card::{EffectDef, EffectRecipientDef, ObjectSetDef, PlayerRefDef};
use crate::game::{Game, GameObjectId, Target};

pub(crate) fn supported_action_program(effect: EffectDef) -> bool {
    let EffectDef::ChooseExact(choice) = effect else {
        return false;
    };
    choice.chooser == PlayerRefDef::EffectController
        && matches!(*choice.then, EffectDef::GainControl { object, controller: PlayerRefDef::EffectController, duration: crate::card::ControlDurationDef::Indefinitely }
            if object == EffectRecipientDef::objects(ObjectSetDef::Binding(choice.binding)))
}

impl Game {
    pub(in crate::game) fn action_program_candidates(
        &self,
        window: &CostPaymentWindow,
        effect: EffectDef,
    ) -> Vec<GameObjectId> {
        if !supported_action_program(effect) {
            return Vec::new();
        }
        let EffectDef::ChooseExact(choice) = effect else {
            unreachable!()
        };
        let excluded = choice.exclude.and_then(|reference| {
            self.object_reference_id(
                reference,
                &window.object,
                &window.context,
                window.definition,
            )
        });
        self.effect_objects(
            choice.candidates,
            &window.object,
            &window.context,
            window.definition,
        )
        .into_iter()
        .filter_map(|target| {
            let Target::Permanent(id) = target else {
                return None;
            };
            (excluded != Some(id)
                && self.battlefield.iter().any(|permanent| {
                    permanent.card.id == id
                        && permanent.controller != window.player
                        && !self.cannot_change_controller(permanent)
                }))
            .then_some(id)
        })
        .collect()
    }

    pub(in crate::game) fn commit_action_program(
        &mut self,
        window: &CostPaymentWindow,
        effect: EffectDef,
        objects: &[GameObjectId],
    ) {
        let EffectDef::ChooseExact(choice) = effect else {
            unreachable!("validated action program")
        };
        let mut context = window.context.clone();
        context.bind_object_group(
            choice.binding,
            objects.iter().copied().map(Target::Permanent).collect(),
        );
        self.resolve_effect_def(
            window.definition.with_effect(*choice.then),
            &window.object,
            context,
        );
    }
}
