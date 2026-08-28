// Which hosts an Aura may legally be attached to.
//
// Split out of `continuous_effects.rs` for the source-size budget, and split
// here because it answers one question in two halves: a permanent an Aura
// enchants, and -- for the one Aura that says so -- a card in a graveyard it
// enchants instead. Included textually, so the imports here are that
// module's.

impl Game {
    /// Whether this Aura's enchant clause names a card where this one is.
    /// Only an Aura whose printed slot reaches that zone may be attached to
    /// a card there; every other Aura enchants permanents and nothing else.
    fn aura_may_enchant_card(&self, aura: &Permanent, host: GameObjectId) -> bool {
        let Some((zone, card)) = self.card_in_nonbattlefield_zone(host) else {
            return false;
        };
        let Some(rules) = self.effective_rules(aura) else {
            return false;
        };
        let Some(target) = Self::aura_enchant_target(&rules) else {
            return false;
        };
        let AbilityTargetPredicate::Object {
            object,
            zones,
            owner,
            ..
        } = target.predicate
        else {
            return false;
        };
        zones.contains(&zone)
            && owner.is_none_or(|relation| {
                self.player_relation_matches(
                    card.owner,
                    relation,
                    aura.controller,
                    TriggerContext::empty(),
                )
            })
            && self.card_object_matches(object, card, zone, aura.card.id)
    }

    /// The slot an Aura's own spell clause fills with what it will enchant.
    fn aura_enchant_target(
        rules: &crate::card::CardRules,
    ) -> Option<crate::card::AbilityTargetDef> {
        rules.ability_clauses().iter().find_map(|ability| {
            let target = Self::immediate_attachment_target(ability.declarative_effect()?)?;
            match ability.definition {
                DeclarativeAbilityDef::Spell(spell) => spell.targets().get(target.index()).copied(),
                DeclarativeAbilityDef::AlternativeCast(alternative)
                    if alternative.kind == crate::card::AlternativeCastKindDef::Bestow =>
                {
                    alternative.targets.get(target.index()).copied()
                }
                _ => None,
            }
        })
    }

    /// Whether an Aura may stay attached to `host`: the host has to still be
    /// somewhere the Aura's own clause reaches, and still satisfy what that
    /// clause names.
    pub(super) fn is_legal_aura_host(&self, aura: &Permanent, host: GameObjectId) -> bool {
        let Some(host) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == host)
        else {
            // "Enchant creature card in a graveyard": an Aura may be
            // attached to a card outside the battlefield when its own
            // enchant clause says so, which is what keeps Animate Dead from
            // being binned between resolving and returning what it
            // enchants.
            return self.aura_may_enchant_card(aura, host);
        };
        if self.cannot_be_enchanted(host) {
            return false;
        }
        if self.effective_rules(aura).is_none() {
            return false;
        }
        let Some(rules) = self.effective_rules(aura) else {
            return false;
        };
        // An Aura that never announces a host as it is cast declares its
        // restriction outright, because there is no target slot to read one
        // from.
        if let Some(object) = rules.enchant() {
            return self.trigger_object_matches(
                object,
                &self.trigger_event_object(host),
                aura.card.id,
                false,
            );
        }
        // A bestowed permanent's restriction is printed on the bestow
        // clause rather than on a spell clause: bestow is what turned it
        // into an Aura, so bestow is what says what it may enchant.
        let Some(target) = Self::aura_enchant_target(&rules) else {
            return false;
        };
        match target.predicate {
            AbilityTargetPredicate::Object {
                object,
                zones,
                controller,
                owner,
            } => {
                zones.contains(&ZoneKind::Battlefield)
                        && controller.is_none_or(|relation| {
                            self.player_relation_matches(
                                host.controller,
                                relation,
                                aura.controller,
                                TriggerContext::empty(),
                            )
                        })
                        && owner.is_none_or(|relation| {
                            self.player_relation_matches(
                                host.card.owner,
                                relation,
                                aura.controller,
                                TriggerContext::empty(),
                            )
                        })
                        && self.trigger_object_matches(
                            object,
                            &self.trigger_event_object(host),
                            aura.card.id,
                            false,
                        )
                        // Hexproof only constrains targeting. Protection also
                        // makes an existing attachment illegal, unless this
                        // Aura is the one printing the exception -- which is
                        // what an Aura granting protection from its own color
                        // has to do to survive its own effect.
                        && (self.remains_attached_through_protection(aura)
                            || !self.is_protected_from_characteristics(
                                host,
                                &self.trigger_event_object(aura),
                                false,
                            ))
            }
            AbilityTargetPredicate::AnyTarget
            | AbilityTargetPredicate::AnyOf(_)
            | AbilityTargetPredicate::PlayerOrPlaneswalker(_)
            | AbilityTargetPredicate::ControlledByTargetOf { .. }
            | AbilityTargetPredicate::OwnedByTargetPlayer { .. }
            // An Aura attaches to a permanent, so a stack slot never names one.
            | AbilityTargetPredicate::StackObject { .. }
            | AbilityTargetPredicate::PlayerWithMoreObjectsThanChooser { .. }
            | AbilityTargetPredicate::Player(_) => false,
        }
    }
}
