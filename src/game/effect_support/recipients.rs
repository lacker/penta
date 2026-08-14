use super::super::{EffectRecipientDef, Game, StackObject, Target, TriggerContext};

impl Game {
    pub(super) fn direct_effect_recipients(
        &self,
        recipient: EffectRecipientDef,
        object: &StackObject,
        context: TriggerContext,
    ) -> Vec<Target> {
        match recipient {
            EffectRecipientDef::Source => object.source.map(Target::Permanent),
            EffectRecipientDef::ChosenPermanent(_) => {
                unreachable!("chosen permanent returned above")
            }
            EffectRecipientDef::AttachedPermanent => object
                .source
                .and_then(|source| self.attachment_target(source))
                .or_else(|| {
                    context
                        .source_attachment
                        .and_then(|id| self.live_object_target(id))
                }),
            EffectRecipientDef::LinkedPermanent => object
                .source
                .and_then(|source| {
                    self.battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == source)
                        .and_then(|permanent| permanent.reanimation_linked)
                })
                .or(context.source_linked)
                .map(Target::Permanent),
            EffectRecipientDef::Controller => Some(Target::Player(object.controller)),
            EffectRecipientDef::Opponent => Some(Target::Player(object.controller.opponent())),
            EffectRecipientDef::EachPlayer => unreachable!("each player returned above"),
            EffectRecipientDef::TriggeringObject => context
                .object
                .and_then(|object| self.live_object_target(object)),
            EffectRecipientDef::ControllerOfTriggeringObject => context
                .object
                .and_then(|object| self.current_or_last_known_controller(object))
                .or(context.object_controller)
                .map(Target::Player),
            EffectRecipientDef::EventPlayer => context.event_player.map(Target::Player),
            EffectRecipientDef::Target(_)
            | EffectRecipientDef::ControllerOfTarget(_)
            | EffectRecipientDef::ObjectsControlledByTarget { .. }
            | EffectRecipientDef::ObjectsOwnedByTarget { .. }
            | EffectRecipientDef::CardsOwnedByTarget { .. }
            | EffectRecipientDef::MatchingObjects { .. }
            | EffectRecipientDef::ObjectsSharingNameWithTarget(_) => {
                unreachable!("target, matching, and shared-name recipients returned above")
            }
        }
        .into_iter()
        .collect()
    }
}
