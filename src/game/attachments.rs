use super::{
    AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef, AttachmentForm, CardType,
    DeclarativeAbilityDef, EffectDef, EffectRecipientDef, Game, GameObjectId, Permanent, PlayerId,
    Target, TargetIndex, TriggerContext, UntilEndOfTurnControl, WhileSourceControl, ZoneKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum AttachmentKind {
    Aura,
    Equipment,
    Fortification,
}

impl Game {
    /// The rules category governing this permanent's attachment legality.
    pub(super) fn attachment_kind(&self, permanent: &Permanent) -> Option<AttachmentKind> {
        if self.is_aura_permanent(permanent) {
            return Some(AttachmentKind::Aura);
        }
        let subtypes = self.effective_subtypes(permanent);
        if subtypes.contains(&"Equipment") {
            Some(AttachmentKind::Equipment)
        } else if subtypes.contains(&"Fortification") {
            Some(AttachmentKind::Fortification)
        } else {
            None
        }
    }

    /// Whether this permanent currently presents as an Aura. Ordinary Auras
    /// use their printed subtype; bestow, Licids, and Necromancy supply
    /// timestamped layer-4 subtype operations.
    pub(super) fn is_aura_permanent(&self, permanent: &Permanent) -> bool {
        self.permanent_types(permanent)
            .is_some_and(|types| types.contains(CardType::Enchantment))
            && self.effective_subtypes(permanent).contains(&"Aura")
    }

    pub(super) fn has_reconfigure(&self, permanent: &Permanent) -> bool {
        let mut has_reconfigure = false;
        self.for_each_effective_ability(permanent, |effective| {
            has_reconfigure |= effective.ability.is_executable()
                && effective
                    .ability
                    .declarative_effect()
                    .is_some_and(Self::effect_is_reconfigure);
        });
        has_reconfigure
    }

    fn effect_is_reconfigure(effect: EffectDef) -> bool {
        match effect {
            EffectDef::Reconfigure { .. } => true,
            EffectDef::Sequence(effects) => {
                effects.iter().copied().any(Self::effect_is_reconfigure)
            }
            EffectDef::May { effect, .. } => Self::effect_is_reconfigure(*effect),
            _ => false,
        }
    }

    /// The target declaration that supplies this ability's enchant
    /// restriction. An Aura's host is the target named by its `Attach`
    /// instruction, which need not be the spell's first target. Structural
    /// Aura definitions retain primary-target semantics even if a caller
    /// omits the otherwise customary explicit instruction.
    fn enchant_target(ability: &super::AbilityDef) -> Option<&'static AbilityTargetDef> {
        let DeclarativeAbilityDef::Spell(spell) = ability.definition else {
            return None;
        };
        let target = ability
            .declarative_effect()
            .and_then(Self::immediate_attachment_target)
            .or_else(|| spell.is_aura().then_some(TargetIndex::PRIMARY))?;
        spell.targets().get(target.index())
    }

    /// Every enchant ability the Aura currently has must permit its host.
    /// Reading the effective layer-6 set is significant: losing all abilities
    /// makes an Aura illegal, while a later bestow/Licid/reanimation grant can
    /// restore an enchant restriction after an earlier removal.
    fn effective_enchant_restrictions_match(&self, aura: &Permanent, host: GameObjectId) -> bool {
        let mut found = false;
        let mut legal = true;
        self.for_each_effective_ability(aura, |effective| {
            if !effective.ability.is_executable() {
                return;
            }
            let Some(target) = Self::enchant_target(&effective.ability) else {
                return;
            };
            found = true;
            legal &= self.enchant_target_matches(aura, target.predicate, host);
        });
        found && legal
    }

    fn target_for_object(&self, object: GameObjectId) -> Option<Target> {
        if self
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == object)
        {
            Some(Target::Permanent(object))
        } else {
            self.card_in_nonbattlefield_zone(object)
                .map(|_| Target::Card(object))
        }
    }

    fn enchant_target_matches(
        &self,
        aura: &Permanent,
        target: AbilityTargetPredicate,
        host: GameObjectId,
    ) -> bool {
        let AbilityTargetPredicate::Object {
            object,
            zones,
            controller,
            owner,
        } = target
        else {
            return false;
        };
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == host)
        {
            zones.contains(&ZoneKind::Battlefield)
                && controller.is_none_or(|relation| {
                    self.player_relation_matches(
                        permanent.controller,
                        relation,
                        aura.controller,
                        TriggerContext::empty(),
                    )
                })
                && owner.is_none_or(|relation| {
                    self.player_relation_matches(
                        permanent.card.owner,
                        relation,
                        aura.controller,
                        TriggerContext::empty(),
                    )
                })
                && self.trigger_object_matches(
                    object,
                    &self.trigger_event_object(permanent),
                    aura.card.id,
                    false,
                )
        } else if let Some((zone, card)) = self.card_in_nonbattlefield_zone(host) {
            zones.contains(&zone)
                && controller.is_none()
                && owner.is_none_or(|relation| {
                    self.player_relation_matches(
                        card.owner,
                        relation,
                        aura.controller,
                        TriggerContext::empty(),
                    )
                })
                && self.card_object_matches(object, card, zone, aura.card.id)
        } else {
            false
        }
    }

    /// Whether the relation may exist after the attempted move. Targeting
    /// restrictions have already been checked by the spell or ability; this
    /// applies attachment legality itself, which is why shroud and hexproof
    /// are absent while protection is present.
    pub(super) fn is_legal_attachment_host(
        &self,
        attachment: &Permanent,
        host: GameObjectId,
        moving: bool,
    ) -> bool {
        if attachment.card.id == host {
            return false;
        }
        match self.attachment_kind(attachment) {
            Some(AttachmentKind::Aura) => {
                let host_permanent = self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == host);
                if let Some(host_permanent) = host_permanent
                    && (self.cannot_be_enchanted(host_permanent)
                        || moving && self.cannot_become_enchanted(host_permanent)
                        || !self.remains_attached_through_protection(attachment)
                            && self.is_protected_from_colors(
                                host_permanent,
                                self.effective_rules(attachment)
                                    .map_or([false; 5], |rules| {
                                        Self::effective_colors(attachment, rules)
                                    }),
                            ))
                {
                    return false;
                }
                if !self.effective_enchant_restrictions_match(attachment, host) {
                    return false;
                }
                if let Some(linked) = attachment.reanimation_linked {
                    return host == linked
                        && host_permanent.is_some_and(|host| {
                            self.permanent_types(host)
                                .is_some_and(|types| types.contains(CardType::Creature))
                        });
                }
                match attachment.attachment_form {
                    Some(AttachmentForm::Bestowed { .. } | AttachmentForm::Licid) => host_permanent
                        .is_some_and(|host| {
                            self.permanent_types(host)
                                .is_some_and(|types| types.contains(CardType::Creature))
                        }),
                    Some(AttachmentForm::Reconfigured { .. }) => false,
                    None => true,
                }
            }
            Some(AttachmentKind::Equipment) => {
                let source_is_creature = self
                    .permanent_types(attachment)
                    .is_some_and(|types| types.contains(CardType::Creature));
                (!source_is_creature || self.has_reconfigure(attachment))
                    && self
                        .battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == host)
                        .is_some_and(|host| {
                            self.permanent_types(host)
                                .is_some_and(|types| types.contains(CardType::Creature))
                                && !self.is_protected_from_colors(
                                    host,
                                    self.effective_rules(attachment)
                                        .map_or([false; 5], |rules| {
                                            Self::effective_colors(attachment, rules)
                                        }),
                                )
                        })
            }
            Some(AttachmentKind::Fortification) => {
                !self
                    .permanent_types(attachment)
                    .is_some_and(|types| types.contains(CardType::Creature))
                    && self
                        .battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == host)
                        .is_some_and(|host| {
                            self.permanent_types(host)
                                .is_some_and(|types| types.contains(CardType::Land))
                                && !self.is_protected_from_colors(
                                    host,
                                    self.effective_rules(attachment)
                                        .map_or([false; 5], |rules| {
                                            Self::effective_colors(attachment, rules)
                                        }),
                                )
                        })
            }
            None => false,
        }
    }

    /// Moves one attachment through the shared CR 701.3 procedure. An
    /// illegal move and a move to the current host are both strict no-ops.
    pub(super) fn try_attach(&mut self, source: GameObjectId, host: GameObjectId) -> bool {
        let Some(index) = self
            .battlefield
            .iter()
            .position(|permanent| permanent.card.id == source)
        else {
            return false;
        };
        if self.battlefield[index].attached_to == Some(host)
            || !self.is_legal_attachment_host(&self.battlefield[index], host, true)
        {
            return false;
        }
        let timestamp = self.allocate_continuous_effect_timestamp();
        let starts_reconfigured = self.attachment_kind(&self.battlefield[index])
            == Some(AttachmentKind::Equipment)
            && self.has_reconfigure(&self.battlefield[index]);
        self.battlefield[index].attached_to = Some(host);
        if matches!(
            self.battlefield[index].attachment_form,
            Some(AttachmentForm::Bestowed { .. } | AttachmentForm::Reconfigured { .. })
        ) {
            self.battlefield[index].attachment_form = None;
        }
        if starts_reconfigured {
            self.battlefield[index].attachment_form =
                Some(AttachmentForm::Reconfigured { timestamp });
        }
        self.battlefield[index].timestamp = timestamp;
        self.reconcile_all_control_layers();
        true
    }

    /// Ends an attachment relation without moving either object.
    pub(super) fn unattach(&mut self, source: GameObjectId) -> bool {
        let Some(index) = self
            .battlefield
            .iter()
            .position(|permanent| permanent.card.id == source)
        else {
            return false;
        };
        let old_host = self.battlefield[index].attached_to.take();
        let restores_creature = matches!(
            self.battlefield[index].attachment_form,
            Some(AttachmentForm::Bestowed { .. } | AttachmentForm::Reconfigured { .. })
        );
        if restores_creature {
            self.battlefield[index].attachment_form = None;
        }
        self.reconcile_all_control_layers();
        old_host.is_some() || restores_creature
    }

    pub(super) fn end_aura_effect(&mut self, source: GameObjectId) -> bool {
        let Some(index) = self
            .battlefield
            .iter()
            .position(|permanent| permanent.card.id == source)
        else {
            return false;
        };
        let Some(effect) = self.battlefield[index].licid_effects.last().copied() else {
            return false;
        };
        self.end_licid_effect(source, effect.id.0)
    }

    /// Ends one independently resolving Licid effect. The object remains an
    /// Aura, and stays attached, while any other Licid effect is active.
    pub(super) fn end_licid_effect(&mut self, source: GameObjectId, effect_id: u64) -> bool {
        let Some(index) = self
            .battlefield
            .iter()
            .position(|permanent| permanent.card.id == source)
        else {
            return false;
        };
        let Some(effect) = self.battlefield[index]
            .licid_effects
            .iter()
            .position(|effect| effect.id.0 == effect_id)
        else {
            return false;
        };
        self.battlefield[index].licid_effects.remove(effect);
        if self.battlefield[index].licid_effects.is_empty() {
            self.battlefield[index].attached_to = None;
            self.battlefield[index].attachment_form = None;
        }
        self.reconcile_all_control_layers();
        true
    }

    pub(super) fn static_effect_controls_attached(effect: EffectDef) -> bool {
        match effect {
            EffectDef::Apply {
                recipient: EffectRecipientDef::AttachedPermanent,
                effect,
                ..
            } => Self::applied_effect_contains(effect, AppliedEffectDef::ControlBySourceController),
            EffectDef::Sequence(effects) => effects
                .iter()
                .copied()
                .any(Self::static_effect_controls_attached),
            _ => false,
        }
    }

    fn source_controls_attached(&self, source: &Permanent) -> bool {
        let mut controls = false;
        self.for_each_effective_ability(source, |effective| {
            let ability = effective.ability;
            controls |= ability.is_executable()
                && matches!(ability.definition, DeclarativeAbilityDef::Static(_))
                && ability
                    .declarative_effect()
                    .is_some_and(Self::static_effect_controls_attached);
        });
        controls
    }

    /// Adds one fixed-controller layer-2 effect to every eligible target.
    /// Every target of the same resolving instruction receives the same
    /// timestamp, while later instructions naturally supersede it.
    pub(super) fn gain_control_until_end_of_turn(
        &mut self,
        targets: &[GameObjectId],
        controller: PlayerId,
    ) {
        let mut eligible = Vec::new();
        for &target in targets {
            if eligible.contains(&target) {
                continue;
            }
            let Some(permanent) = self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == target)
            else {
                continue;
            };
            // Preserve the established CannotChangeController behavior. A
            // redundant effect controlled by the current controller still
            // gets recorded because its later timestamp can matter if another
            // control effect ends before cleanup.
            if permanent.controller != controller && self.cannot_change_controller(permanent) {
                continue;
            }
            eligible.push(target);
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
                .control_until_end_of_turn
                .push(UntilEndOfTurnControl {
                    timestamp,
                    controller,
                });
        }
        self.reconcile_all_control_layers();
    }

    /// Re-evaluates every layer-2 control effect for one permanent. Fixed
    /// until-end-of-turn effects and live attachment sources share the same
    /// timestamp ordering, so ending either kind reveals the next effect or
    /// the controller beneath the complete layer.
    fn reconcile_control_for(&mut self, host: GameObjectId) -> bool {
        let attachment_controllers = self
            .battlefield
            .iter()
            .filter(|source| source.attached_to == Some(host))
            .filter(|source| self.source_controls_attached(source))
            .map(|source| (source.timestamp, source.controller))
            .collect::<Vec<_>>();
        let Some(index) = self
            .battlefield
            .iter()
            .position(|permanent| permanent.card.id == host)
        else {
            return false;
        };
        let mut controllers = self.battlefield[index]
            .control_until_end_of_turn
            .iter()
            .map(|effect| (effect.timestamp, effect.controller))
            .collect::<Vec<_>>();
        controllers.extend(
            self.battlefield[index]
                .control_while_source_remains
                .iter()
                .filter(|effect| self.while_source_control_is_active(effect))
                .map(|effect| (effect.timestamp, effect.controller)),
        );
        controllers.extend(attachment_controllers);
        if controllers.is_empty() {
            if let Some(base) = self.battlefield[index].control_layer_base.take()
                && self.battlefield[index].controller != base
            {
                self.battlefield[index].controller = base;
                self.battlefield[index].entered_controller_turn = self.turns_started[base.index()];
                return true;
            }
            return false;
        }
        let base = self.battlefield[index].controller;
        self.battlefield[index]
            .control_layer_base
            .get_or_insert(base);
        let controller = controllers
            .into_iter()
            .max_by_key(|(timestamp, _)| *timestamp)
            .map(|(_, controller)| controller)
            .expect("a nonempty control layer has a newest source");
        if self.battlefield[index].controller != controller
            && self.cannot_change_controller(&self.battlefield[index])
        {
            return false;
        }
        if self.battlefield[index].controller != controller {
            self.battlefield[index].controller = controller;
            self.battlefield[index].entered_controller_turn =
                self.turns_started[controller.index()];
            return true;
        }
        false
    }

    /// Compatibility seam for the attachment procedure and focused tests.
    /// The implementation is the shared control layer, not an attachment-only
    /// restoration slot.
    #[cfg(test)]
    pub(super) fn reconcile_attachment_control_for(&mut self, host: Option<GameObjectId>) {
        if let Some(host) = host {
            self.reconcile_control_for(host);
        }
    }

    /// Settles controller dependencies between attachment sources and their
    /// hosts. An acyclic chain can be at most as long as the battlefield, so
    /// that many additional passes are sufficient; ordinary boards converge
    /// on the first pass.
    pub(super) fn reconcile_all_control_layers(&mut self) {
        let mut hosts = self
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.control_layer_base.is_some()
                    || !permanent.control_until_end_of_turn.is_empty()
                    || !permanent.control_while_source_remains.is_empty()
            })
            .map(|permanent| permanent.card.id)
            .chain(
                self.battlefield
                    .iter()
                    .filter(|source| self.source_controls_attached(source))
                    .filter_map(|source| source.attached_to),
            )
            .collect::<Vec<_>>();
        hosts.sort_unstable();
        hosts.dedup();
        for _ in 0..=hosts.len() {
            let mut changed = false;
            for &host in &hosts {
                changed |= self.reconcile_control_for(host);
            }
            if !changed {
                break;
            }
        }
    }

    pub(super) fn reconcile_all_attachment_control(&mut self) {
        self.reconcile_all_control_layers();
    }

    pub(super) fn while_source_control_is_active(&self, effect: &WhileSourceControl) -> bool {
        self.battlefield
            .iter()
            .find(|candidate| candidate.card.id == effect.source)
            .is_some_and(|source| {
                source.controller == effect.controller
                    && (!effect.requires_source_tapped || source.tapped)
            })
    }

    pub(super) fn attachment_target(&self, source: GameObjectId) -> Option<Target> {
        self.attached_host(source)
            .and_then(|host| self.target_for_object(host))
    }
}
