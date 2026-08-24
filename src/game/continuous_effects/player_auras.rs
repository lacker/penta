impl Game {
    /// Whether a player-enchanting Aura may stay attached to `player`.
    /// Targeting protections matter only while the spell is choosing that
    /// player; the enchant restriction itself is the live attachment rule.
    pub(super) fn is_legal_aura_player(&self, aura: &Permanent, player: PlayerId) -> bool {
        let Some(rules) = self.effective_rules(aura) else {
            return false;
        };
        let Some(target) = rules.ability_clauses().iter().find_map(|ability| {
            let target = Self::immediate_attachment_target(ability.declarative_effect()?)?;
            match ability.definition {
                DeclarativeAbilityDef::Spell(spell) => spell.targets().get(target.index()),
                _ => None,
            }
        }) else {
            return false;
        };
        match target.predicate {
            AbilityTargetPredicate::Player(relation) => self.player_relation_matches(
                player,
                relation,
                aura.controller,
                TriggerContext::empty(),
            ),
            AbilityTargetPredicate::AnyTarget
            | AbilityTargetPredicate::PlayerOrPlaneswalker(_)
            | AbilityTargetPredicate::ControlledByTargetOf { .. }
            | AbilityTargetPredicate::OwnedByTargetPlayer { .. }
            | AbilityTargetPredicate::Object { .. }
            | AbilityTargetPredicate::StackObject { .. } => false,
        }
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
