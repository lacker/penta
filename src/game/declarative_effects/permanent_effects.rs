use crate::action::Target;
use crate::card::{
    AbilityDef, DeclarativeAbilityDef, EffectDef, EffectRecipientDef, ReanimationAuraDef, ZoneKind,
};
use crate::ids::CardDefinitionId;

use super::super::{
    AbilitySourceRef, AttachmentForm, CopiableAbility, EntryCompletion, Game, LicidEffect,
    ReanimationAttachmentEffect, ScopedEffect, StackObject, TriggerCapture, TriggerContext,
    ZoneMoveCause,
};

impl Game {
    pub(super) fn resolve_permanent_effect_def(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: TriggerContext,
    ) {
        match scoped.effect {
            EffectDef::CreateAttachedToken { token } => {
                self.resolve_create_attached_token_effect(token, object);
            }
            EffectDef::BecomeCopyOf {
                object: recipient,
                retain_source_ability,
            } => {
                self.resolve_become_copy_effect(
                    recipient,
                    retain_source_ability,
                    object,
                    context,
                    scoped,
                );
            }
            EffectDef::MoveToZone {
                object: _,
                zone: _,
                controller: _,
                placement: _,
            } => {
                self.resolve_move_to_zone_effect(scoped, object, context);
            }
            EffectDef::Attach { object: recipient } => {
                self.resolve_attach_effect(recipient, object, context, scoped);
            }
            EffectDef::Unattach { object: recipient } => {
                self.resolve_unattach_effect(recipient, object, context, scoped);
            }
            EffectDef::Reconfigure { object: recipient } => {
                self.resolve_reconfigure_effect(recipient, object, context, scoped);
            }
            EffectDef::BecomeAuraAndAttach {
                object: recipient, ..
            } => {
                self.resolve_become_aura_and_attach_effect(recipient, object, context, scoped);
            }
            EffectDef::EndAuraEffect => {
                if let Some(source) = object.source {
                    self.end_aura_effect(source);
                }
            }
            EffectDef::ReturnToBattlefieldAttached {
                card: recipient,
                aura,
                leave,
            } => {
                self.resolve_return_to_battlefield_attached_effect(
                    recipient, aura, leave, object, context, scoped,
                );
            }
            _ => unreachable!("non-permanent effect passed to the permanent effect resolver"),
        }
    }

    fn resolve_create_attached_token_effect(
        &mut self,
        token: CardDefinitionId,
        object: &StackObject,
    ) {
        self.create_token_from_with_completion(
            object.controller,
            token,
            object.source,
            object.source.map_or(EntryCompletion::None, |source| {
                EntryCompletion::AttachSource {
                    source,
                    reanimation: None,
                    scheduled_trigger: None,
                }
            }),
        );
    }

    fn resolve_become_copy_effect(
        &mut self,
        recipient: EffectRecipientDef,
        retain_source_ability: bool,
        object: &StackObject,
        context: TriggerContext,
        scoped: ScopedEffect,
    ) {
        let Some(Target::Permanent(target)) = self
            .effect_recipients(recipient, object, context, scoped)
            .into_iter()
            .next()
        else {
            return;
        };
        let Some(mut copy) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == target)
            .map(Self::copiable_characteristics)
        else {
            return;
        };
        if retain_source_ability
            && let Some(payload) = &object.ability
            && let Some(definition) = payload.definition.as_deref()
        {
            copy.added_abilities.push(CopiableAbility {
                origin: payload.origin,
                definition: *definition,
            });
        }
        if let Some(source) = object.source
            && let Some(permanent) = self
                .battlefield
                .iter_mut()
                .find(|permanent| permanent.card.id == source)
        {
            permanent.copy_effect = Some(copy);
        }
        self.reconcile_all_control_layers();
    }

    fn resolve_move_to_zone_effect(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: TriggerContext,
    ) {
        let EffectDef::MoveToZone {
            object: recipient,
            zone,
            controller,
            placement,
        } = scoped.effect
        else {
            unreachable!("non-zone-move effect passed to the zone-move resolver");
        };
        let arriving_controller = controller.map(|relation| {
            if self.player_relation_matches(object.controller, relation, object.controller, context)
            {
                object.controller
            } else {
                object.controller.opponent()
            }
        });
        for target in self.effect_recipients(recipient, object, context, scoped) {
            self.move_target_to_zone(
                target,
                zone,
                ZoneMoveCause::Effect {
                    controller: object.controller,
                },
                arriving_controller,
                placement,
            );
        }
    }

    fn resolve_attach_effect(
        &mut self,
        recipient: EffectRecipientDef,
        object: &StackObject,
        context: TriggerContext,
        scoped: ScopedEffect,
    ) {
        let Some(source) = object.source else { return };
        for target in self.effect_recipients(recipient, object, context, scoped) {
            if let Target::Card(host) | Target::Permanent(host) = target {
                self.try_attach(source, host);
            }
        }
    }

    fn resolve_unattach_effect(
        &mut self,
        recipient: EffectRecipientDef,
        object: &StackObject,
        context: TriggerContext,
        scoped: ScopedEffect,
    ) {
        for target in self.effect_recipients(recipient, object, context, scoped) {
            if let Target::Permanent(attachment) = target {
                self.unattach(attachment);
            }
        }
    }

    fn resolve_reconfigure_effect(
        &mut self,
        recipient: EffectRecipientDef,
        object: &StackObject,
        context: TriggerContext,
        scoped: ScopedEffect,
    ) {
        let Some(source) = object.source else { return };
        let host = self
            .effect_recipients(recipient, object, context, scoped)
            .into_iter()
            .find_map(|target| match target {
                Target::Permanent(host) => Some(host),
                Target::Card(_) | Target::Player(_) | Target::Spell(_) => None,
            });
        if let Some(host) = host {
            self.try_attach(source, host);
        } else {
            self.unattach(source);
        }
    }

    fn resolve_become_aura_and_attach_effect(
        &mut self,
        recipient: EffectRecipientDef,
        object: &StackObject,
        context: TriggerContext,
        scoped: ScopedEffect,
    ) {
        let Some(source) = object.source else { return };
        let Some(host) = self
            .effect_recipients(recipient, object, context, scoped)
            .into_iter()
            .find_map(|target| match target {
                Target::Permanent(host) => Some(host),
                Target::Card(_) | Target::Player(_) | Target::Spell(_) => None,
            })
        else {
            return;
        };
        let Some((transform_action, end)) = self.licid_form_actions_for_resolution(object, source)
        else {
            return;
        };
        let effect_id = self.allocate_continuous_effect_timestamp();
        if let Some(attachment) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == source)
        {
            attachment.attachment_form = Some(AttachmentForm::Licid);
            attachment.licid_effects.push(LicidEffect {
                id: effect_id,
                ender: object.controller,
                transform_action,
                end,
            });
        }
        self.try_attach(source, host);
    }

    fn resolve_return_to_battlefield_attached_effect(
        &mut self,
        recipient: EffectRecipientDef,
        aura: ReanimationAuraDef,
        leave: &'static AbilityDef,
        object: &StackObject,
        context: TriggerContext,
        scoped: ScopedEffect,
    ) {
        let Some(source) = object.source.filter(|source| {
            self.battlefield
                .iter()
                .any(|permanent| permanent.card.id == *source)
        }) else {
            return;
        };
        let reanimation = ReanimationAttachmentEffect {
            timestamp: self.allocate_continuous_effect_timestamp(),
            aura,
        };
        let scheduled_trigger = object.ability.as_ref().and_then(|payload| {
            let DeclarativeAbilityDef::Triggered(definition) = leave.definition else {
                return None;
            };
            let capture = TriggerCapture {
                source: AbilitySourceRef {
                    object: source,
                    ability: payload.origin,
                },
                definition: payload.presentation_definition,
                owner: object.card.owner,
                controller: object.controller,
                text: leave.text,
                target_defs: definition.targets,
                effect: leave.effect.definition,
                resolver: Self::ability_resolver(payload.origin, leave),
                context: TriggerContext::empty(),
                condition: definition.condition,
            };
            Some(self.schedule_one_shot_event_trigger(definition.event, &capture))
        });
        for target in self.effect_recipients(recipient, object, context, scoped) {
            let Target::Card(card) = target else { continue };
            self.move_card_from_nonbattlefield_zone_with_completion(
                card,
                ZoneKind::Graveyard,
                ZoneKind::Battlefield,
                ZoneMoveCause::Effect {
                    controller: object.controller,
                },
                Some(object.controller),
                EntryCompletion::AttachSource {
                    source,
                    reanimation: Some(reanimation),
                    scheduled_trigger,
                },
            );
        }
    }
}
