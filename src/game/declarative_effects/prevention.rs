use super::super::{
    EffectDef, Game, PreventionShield, RelationalDamagePrevention, ScopedEffect, ShieldCoverageDef,
    StackObject, Target, TriggerContext,
};

impl Game {
    pub(super) fn resolve_prevention_effect(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: TriggerContext,
    ) {
        match scoped.effect {
            EffectDef::PreventNextDamage {
                object: recipient,
                amount,
            } => {
                let amount = self
                    .effect_value(amount, object, context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    self.prevention_shields.push(PreventionShield {
                        recipient: target,
                        remaining: Some(amount),
                        source: None,
                        coverage: ShieldCoverageDef::All,
                        gain_life: false,
                    });
                }
            }
            EffectDef::ChooseDamageSource { .. } => {
                self.resolve_damage_source_choice(scoped, object, context);
            }
            EffectDef::PreventNextDamageFromSource { .. } => {
                self.install_damage_source_shield(scoped, object, context);
            }
            EffectDef::PreventAllDamageThisTurn { object: recipient } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    self.prevention_shields.push(PreventionShield {
                        recipient: target,
                        remaining: None,
                        source: None,
                        coverage: ShieldCoverageDef::All,
                        gain_life: false,
                    });
                }
            }
            EffectDef::PreventAllCombatDamageThisTurn => {
                self.all_combat_damage_prevented = true;
            }
            EffectDef::PreventCombatDamageThisTurn { object: recipient } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Permanent(id) = target
                        && let Some(permanent) = self
                            .battlefield
                            .iter_mut()
                            .find(|permanent| permanent.card.id == id)
                    {
                        permanent.combat_damage_prevented = true;
                    }
                }
            }
            EffectDef::PreventCombatDamageDealtByThisTurn { object: recipient } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Permanent(id) = target
                        && let Some(permanent) = self
                            .battlefield
                            .iter_mut()
                            .find(|permanent| permanent.card.id == id)
                    {
                        permanent.combat_damage_dealt_by_prevented = true;
                    }
                }
            }
            EffectDef::PreventDamageToPlayerAndControlledCreaturesThisTurn { player } => {
                for target in self.effect_recipients(player, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.relational_damage_preventions.push(
                            RelationalDamagePrevention::ToPlayerAndControlledCreatures(player),
                        );
                    }
                }
            }
            EffectDef::PreventAllCombatDamageExceptSourceThisTurn { source } => {
                let source = self
                    .effect_recipients(source, object, context, scoped)
                    .into_iter()
                    .find_map(|target| match target {
                        Target::Permanent(id) | Target::Spell(id) | Target::Card(id) => Some(id),
                        Target::Player(_) => None,
                    });
                if let Some(source) = source {
                    self.relational_damage_preventions
                        .push(RelationalDamagePrevention::FromAllExcept(source));
                }
            }
            _ => unreachable!("resolve_prevention_effect called for a non-prevention effect"),
        }
    }

    fn resolve_damage_source_choice(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: TriggerContext,
    ) {
        let EffectDef::ChooseDamageSource {
            choice,
            chooser,
            object: predicate,
            then,
        } = scoped.effect
        else {
            unreachable!("damage source choice helper called for a different effect");
        };
        for chooser in self.effect_recipients(chooser, object, context, scoped) {
            if let Target::Player(chooser) = chooser {
                self.queue_damage_source_choice(
                    choice,
                    chooser,
                    predicate,
                    object,
                    context,
                    scoped.with_effect(*then),
                );
            }
        }
    }

    fn install_damage_source_shield(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: TriggerContext,
    ) {
        let EffectDef::PreventNextDamageFromSource {
            object: recipient,
            source,
            coverage,
            gain_life,
        } = scoped.effect
        else {
            unreachable!("damage source shield helper called for a different effect");
        };
        let named = self
            .effect_recipients(source, object, context, scoped)
            .into_iter()
            .find_map(|target| match target {
                Target::Permanent(id) | Target::Spell(id) | Target::Card(id) => Some(id),
                Target::Player(_) => None,
            });
        let Some(named) = named else {
            return;
        };
        for target in self.effect_recipients(recipient, object, context, scoped) {
            self.prevention_shields.push(PreventionShield {
                recipient: target,
                remaining: None,
                source: Some(named),
                coverage,
                gain_life,
            });
        }
    }
}
