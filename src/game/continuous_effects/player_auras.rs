impl Game {
    fn aura_player_relation(rules: &CardRules) -> Option<PlayerRelation> {
        rules.ability_clauses().iter().find_map(|ability| {
            if !ability.is_executable() {
                return None;
            }
            let target = Self::immediate_attachment_target(ability.declarative_effect()?)?;
            let DeclarativeAbilityDef::Spell(spell) = ability.definition else {
                return None;
            };
            match spell.targets().get(target.index())?.predicate {
                AbilityTargetPredicate::Player(relation) => Some(relation),
                AbilityTargetPredicate::AnyTarget
                | AbilityTargetPredicate::PlayerOrPlaneswalker(_)
                | AbilityTargetPredicate::ControlledByTargetOf { .. }
                | AbilityTargetPredicate::OwnedByTargetPlayer { .. }
                | AbilityTargetPredicate::Object { .. }
                | AbilityTargetPredicate::StackObject { .. } => None,
            }
        })
    }

    /// Whether a card outside the battlefield is an Aura that could legally
    /// enchant `player` if an effect put it there under `controller`'s
    /// control. This is the search-time half of an attached arrival: cards
    /// that could not make the requested attachment are not legal choices.
    pub(super) fn card_can_enchant_player(
        &self,
        definition: CardDefinitionId,
        controller: PlayerId,
        player: PlayerId,
    ) -> bool {
        let Some(rules) = self.catalog.get(definition).map(|card| &card.rules) else {
            return false;
        };
        rules.has_subtype("Aura")
            && Self::aura_player_relation(rules).is_some_and(|relation| {
                self.player_relation_matches(
                    player,
                    relation,
                    controller,
                    TriggerContext::empty(),
                )
            })
    }

    /// Whether a player-enchanting Aura may stay attached to `player`.
    /// Targeting protections matter only while the spell is choosing that
    /// player; the enchant restriction itself is the live attachment rule.
    pub(super) fn is_legal_aura_player(&self, aura: &Permanent, player: PlayerId) -> bool {
        let Some(rules) = self.effective_rules(aura) else {
            return false;
        };
        Self::aura_player_relation(&rules).is_some_and(|relation| {
            self.player_relation_matches(
                player,
                relation,
                aura.controller,
                TriggerContext::empty(),
            )
        })
    }

    /// The player an Aura spell targeted, read off the same attachment clause
    /// used for permanent-host Auras.
    pub(super) fn aura_player_for(object: &StackObject) -> Option<PlayerId> {
        let ability = object.ability.as_ref()?;
        let primary = match ability.resolver {
            StackAbilityResolver::Declarative(effect)
            | StackAbilityResolver::DeclarativeIgnoringTargetFizzle(effect)
            | StackAbilityResolver::DeclarativeWithCustomFollowup { effect, .. } => Some(effect),
            StackAbilityResolver::Custom(_)
            | StackAbilityResolver::CardOwned(_)
            | StackAbilityResolver::CastOffer(_) => None,
        };
        primary
            .into_iter()
            .chain(ability.mode_effects.iter().copied())
            .find_map(|scoped| {
                let target = Self::immediate_attachment_target(scoped.effect)?;
                Self::chosen_targets(object, scoped.target_slot(target)).find_map(|target| {
                    match target {
                        Target::Player(player) => Some(player),
                        _ => None,
                    }
                })
            })
    }
}
