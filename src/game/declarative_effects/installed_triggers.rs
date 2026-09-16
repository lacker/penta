use super::{
    AbilitySourceRef, DeclarativeAbilityDef, EffectResolutionContext, Game, ScopedEffect,
    StackAbilityResolver, StackObject,
};
use crate::card::{InstalledTriggerDef, InstalledTriggerLifetimeDef};
use crate::game::{
    AbilityOrigin, AbilityProcedureDef, InstalledTrigger, InstalledTriggerLifetime, TriggerCapture,
};

impl Game {
    pub(super) fn create_reflexive_trigger(
        &mut self,
        ability: &'static crate::card::AbilityDef,
        object: &StackObject,
        mut context: EffectResolutionContext,
    ) {
        let DeclarativeAbilityDef::Triggered(definition) = ability.definition else {
            return;
        };
        let (Some(effect), Some(frozen)) = (ability.declarative_effect(), object.ability.as_ref())
        else {
            return;
        };
        if let Some(condition) = definition.condition
            && !self.trigger_condition_holds(
                condition,
                object.source.unwrap_or(object.id),
                object.controller,
                context.trigger,
                Some(frozen.origin),
                Some((object, &ScopedEffect::primary(effect), &context)),
            )
        {
            return;
        }
        context.source_transform_count = None;
        self.capture_trigger(&TriggerCapture {
            source: AbilitySourceRef {
                object: object.source.unwrap_or(object.id),
                ability: frozen.origin,
            },
            presentation: frozen.presentation,
            owner: object.card.owner,
            controller: object.controller,
            text: ability.text,
            target_defs: definition.targets.to_vec(),
            targets: Vec::new(),
            effect,
            resolver: Self::ability_resolver(frozen.origin, ability),
            context,
            condition: definition.condition,
            modes: None,
            x: frozen.x,
        });
    }

    pub(in crate::game) fn install_trigger_from(
        &mut self,
        installed: InstalledTriggerDef,
        scoped: ScopedEffect,
        object: &StackObject,
        context: &EffectResolutionContext,
        source_ability: AbilityOrigin,
    ) {
        let DeclarativeAbilityDef::Triggered(definition) = installed.ability.definition else {
            return;
        };
        // Installed triggers use the ordinary pending-trigger and stack paths.
        // Declaring fresh targets would require a second target namespace;
        // until that exists they may only retain the installing object's
        // already-chosen target slots.
        if definition.procedure != AbilityProcedureDef::Shared || !definition.targets.is_empty() {
            return;
        }
        let Some(effect) = installed.ability.declarative_effect() else {
            return;
        };
        let Some(frozen) = object.ability.as_ref() else {
            return;
        };
        let lifetime = match installed.lifetime {
            InstalledTriggerLifetimeDef::Once => InstalledTriggerLifetime::Once,
            InstalledTriggerLifetimeDef::ThisTurn => {
                InstalledTriggerLifetime::ThisTurn { turn: self.turn }
            }
            InstalledTriggerLifetimeDef::UntilNextTurn(player) => {
                let Some(player) = self.effect_player_reference(player, object, context, scoped)
                else {
                    return;
                };
                InstalledTriggerLifetime::UntilTurn {
                    player,
                    turn: self.turns_started[player.index()].saturating_add(1),
                }
            }
        };
        let id = self.next_installed_trigger_id;
        self.next_installed_trigger_id = self.next_installed_trigger_id.saturating_add(1);
        let mut context = context.clone();
        context.source_transform_count = self
            .battlefield
            .iter()
            .find(|permanent| Some(permanent.card.id) == object.source)
            .map(|permanent| permanent.transform_count);
        self.installed_triggers.push(InstalledTrigger {
            id,
            event: definition.event,
            capture: TriggerCapture {
                source: AbilitySourceRef {
                    object: object.source.unwrap_or(object.id),
                    ability: source_ability,
                },
                presentation: frozen.presentation,
                owner: object.card.owner,
                controller: object.controller,
                text: installed.ability.text,
                // The selections belong to the installing ability's lexical
                // target namespace. They remain readable by the nested effect,
                // but the installed ability does not target them again when it
                // triggers.
                target_defs: Vec::new(),
                targets: frozen.targets.clone(),
                effect,
                resolver: StackAbilityResolver::Declarative(scoped.with_effect(effect)),
                // Resolution branches share mutable bindings. A delayed trigger must
                // retain the values at installation, before later instructions rebind them.
                context,
                condition: definition.condition,
                // An installed trigger carries the effect it was installed
                // with; nothing about it is modal.
                modes: None,
                x: frozen.x,
            },
            lifetime,
        });
    }
}
