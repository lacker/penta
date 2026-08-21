// Which printed clause a chosen cast configuration actually resolves.
//
// Split from action enumeration because it answers a different question:
// not what a player may do, but which of a card's clauses the choice they
// already made selects -- the base spell, an overloaded one, or a kicked
// one -- and what that clause puts on the stack.

impl Game {
    pub(super) fn alternative_cast_clause(
        definition: &CardDefinition,
        option: &PlayOptionDef,
        alternative: AlternativeCostId,
    ) -> Option<(AbilityOrigin, AbilityDef, AlternativeCastKindDef)> {
        let parts: &[CardPartId] = match &option.form {
            crate::card::SpellForm::Part(part) => std::slice::from_ref(part),
            crate::card::SpellForm::Combined(parts) => parts,
        };
        parts.iter().find_map(|part_id| {
            definition
                .part(*part_id)?
                .rules
                .indexed_abilities()
                .find_map(|attached| {
                    let DeclarativeAbilityDef::AlternativeCast(alternative_cast) =
                        attached.definition.definition
                    else {
                        return None;
                    };
                    (attached.alternative_cost_id() == Some(alternative)).then_some((
                        AbilityOrigin::Printed {
                            definition: definition.id,
                            part: *part_id,
                            ability: attached.id,
                        },
                        attached.definition,
                        alternative_cast.kind,
                    ))
                })
        })
    }

    pub(super) fn alternative_cast_ability(
        definition: &CardDefinition,
        option: &PlayOptionDef,
        alternative: AlternativeCostId,
    ) -> Option<(AbilityOrigin, AbilityDef, AlternativeCastKindDef)> {
        Self::alternative_cast_clause(definition, option, alternative)
            .filter(|(_, ability, _)| ability.is_executable())
    }

    pub(super) fn optional_additional_cost_clause(
        definition: &CardDefinition,
        option: &PlayOptionDef,
        additional: AdditionalCostId,
    ) -> Option<(AbilityOrigin, AbilityDef, OptionalAdditionalCostKindDef)> {
        let parts: &[CardPartId] = match &option.form {
            crate::card::SpellForm::Part(part) => std::slice::from_ref(part),
            crate::card::SpellForm::Combined(parts) => parts,
        };
        parts.iter().find_map(|part_id| {
            definition
                .part(*part_id)?
                .rules
                .indexed_abilities()
                .find_map(|attached| {
                    let DeclarativeAbilityDef::OptionalAdditionalCost(cost) =
                        attached.definition.definition
                    else {
                        return None;
                    };
                    (attached.additional_cost_id() == Some(additional)
                        && attached.definition.is_executable())
                    .then_some((
                        AbilityOrigin::Printed {
                            definition: definition.id,
                            part: *part_id,
                            ability: attached.id,
                        },
                        attached.definition,
                        cost.kind,
                    ))
                })
        })
    }

    pub(super) fn selected_alternative_kind(
        &self,
        definition: &CardDefinition,
        option: &PlayOptionDef,
        card: GameObjectId,
        costs: &CostConfiguration,
    ) -> Option<AlternativeCastKindDef> {
        self.selected_alternative_kind_for_offer(definition, option, card, costs, None)
    }

    pub(super) fn selected_alternative_kind_for_offer(
        &self,
        definition: &CardDefinition,
        option: &PlayOptionDef,
        card: GameObjectId,
        costs: &CostConfiguration,
        offer: Option<CastOfferCost>,
    ) -> Option<AlternativeCastKindDef> {
        let selected = costs.alternative()?;
        if Some(selected) == Self::temporary_alternative_cost_id(option)
            && let Some((_, alternative, _)) = self.granted_alternative_cast(
                card,
                option,
                match offer {
                    Some(CastOfferCost::GrantedAlternative(grant)) => Some(grant),
                    None | Some(CastOfferCost::Any) => None,
                    Some(CastOfferCost::PrintedAlternative(_)) => return None,
                },
            )
        {
            return Some(alternative.kind);
        }
        Self::alternative_cast_ability(definition, option, selected).map(|(_, _, kind)| kind)
    }

    pub(super) fn temporary_alternative_cost_id(
        option: &PlayOptionDef,
    ) -> Option<AlternativeCostId> {
        (u8::MIN..=u8::MAX)
            .rev()
            .map(AlternativeCostId)
            .find(|candidate| {
                option
                    .alternative_costs
                    .iter()
                    .all(|cost| cost.id != *candidate)
            })
    }

    /// The one alternative way to cast this card that something other than
    /// the card itself supplies: a temporary grant hung on the object, or a
    /// static ability on the battlefield speaking about a whole graveyard.
    /// A card in a graveyard has no layer walk, so both are found by asking
    /// elsewhere rather than by asking the card.
    pub(super) fn granted_alternative_cast(
        &self,
        card: GameObjectId,
        option: &PlayOptionDef,
        required: Option<usize>,
    ) -> Option<(AbilityDef, AlternativeCastAbilityDef, ManaCost)> {
        let resolve = |grant: &TemporaryAbilityGrant| {
            (grant.object == card).then_some(())?;
            if !grant.ability.is_executable() {
                return None;
            }
            let DeclarativeAbilityDef::AlternativeCast(alternative) = grant.ability.definition
            else {
                return None;
            };
            matches!(
                alternative.kind,
                AlternativeCastKindDef::Flashback | AlternativeCastKindDef::WithoutPayingManaCost
            )
            .then_some((grant.ability, alternative))
        };
        let temporary = match required {
            Some(grant) => self.temporary_ability_grants.get(grant).and_then(resolve),
            None => self.temporary_ability_grants.iter().find_map(resolve),
        };
        let (ability, alternative) = temporary.or_else(|| {
            if required.is_some() {
                return None;
            }
            let ability = self.granted_graveyard_alternative(card)?;
            let DeclarativeAbilityDef::AlternativeCast(alternative) = ability.definition else {
                return None;
            };
            Some((ability, alternative))
        })?;
        alternative
            .mana_cost
            .resolve(option.mana_cost)
            .map(|mana_cost| (ability, alternative, mana_cost))
    }

    /// The alternative cast a static ability grants to this card while it
    /// lies in its owner's graveyard.
    fn granted_graveyard_alternative(&self, card: GameObjectId) -> Option<AbilityDef> {
        let (owner, instance) = [PlayerId::One, PlayerId::Two]
            .into_iter()
            .find_map(|player| {
                self.players[player.index()]
                    .graveyard
                    .iter()
                    .find(|candidate| candidate.id == card)
                    .map(|candidate| (player, candidate))
            })?;
        self.granted_graveyard_alternative_cast(instance, owner)
            .copied()
    }
    pub(super) fn spell_custom_followup(
        definition: &CardDefinition,
        option: &PlayOptionDef,
        primary: AbilityId,
    ) -> Option<CardBehavior> {
        let crate::card::SpellForm::Part(part_id) = &option.form else {
            return None;
        };
        definition
            .part(*part_id)?
            .rules
            .indexed_abilities()
            .find_map(|attached| {
                (attached.id != primary)
                    .then(|| attached.definition.custom_behavior())
                    .flatten()
            })
    }

    pub(super) fn frozen_spell_payload(
        &self,
        definition_id: CardDefinitionId,
        signature: &CastSignature,
    ) -> Option<StackAbilityPayload> {
        let definition = self.catalog.get(definition_id)?;
        let option = definition.play_option(signature.play_option())?;
        let (spell_origin, spell_ability) = Self::spell_ability(definition, option)?;
        let DeclarativeAbilityDef::Spell(spell) = spell_ability.definition else {
            unreachable!("spell_ability returns a spell clause")
        };
        let mut resolution_destination = spell.resolution_destination();
        for selected in signature.costs().additional() {
            if let Some((_, selected_ability, _)) =
                Self::optional_additional_cost_clause(definition, option, *selected)
                && let DeclarativeAbilityDef::OptionalAdditionalCost(cost) =
                    selected_ability.definition
            {
                resolution_destination = cost.resolution_destination;
            }
        }
        // Overload and kicker both replace the spell's instructions with the
        // ones printed on their own clause, so both resolve that clause
        // rather than the base spell.
        if let Some(selected) = signature.costs().alternative()
            && let Some((
                origin,
                ability,
                AlternativeCastKindDef::Overload | AlternativeCastKindDef::Kicked,
            )) = Self::alternative_cast_ability(definition, option, selected)
            // A kicker that only costs more resolves the printed spell, so
            // it falls through to the base clause below rather than being
            // resolved in its place.
            && ability.declarative_effect() != Some(crate::card::EffectDef::None)
        {
            let DeclarativeAbilityDef::AlternativeCast(alternative_cast) = ability.definition
            else {
                unreachable!("alternative_cast_ability returns an alternative-cast clause")
            };
            return Some(StackAbilityPayload {
                origin,
                definition: Some(Box::new(ability)),
                presentation: Self::ability_presentation(
                    origin,
                    ObjectCharacteristics::card(definition_id, CardPartId::PRIMARY),
                ),
                text: alternative_cast.stack_text.or(Some(ability.text)),
                // Overload declares none, having changed "target" to "each";
                // a kicked spell declares its own, which need not be the ones
                // the unkicked spell could point at.
                target_defs: alternative_cast.targets.to_vec(),
                targets: signature.targets().to_vec(),
                context: TriggerContext::empty().into(),
                resolver: Self::ability_resolver(origin, &ability),
                condition: None,
                mode_effects: Vec::new(),
                resolution_destination: Some(resolution_destination),
                x: signature.x(),
            });
        }
        let (origin, ability) = (spell_origin, spell_ability);
        let AbilityOrigin::Printed {
            ability: ability_id,
            ..
        } = origin
        else {
            unreachable!("a printed spell clause has a printed origin")
        };
        let DeclarativeAbilityDef::Spell(spell) = ability.definition else {
            unreachable!("spell_ability returns a spell clause")
        };
        let followup = Self::spell_custom_followup(definition, option, ability_id);
        let plan = Self::selected_spell_plan(spell, signature.modes())
            .expect("validated modes select declared spell targets and branches");
        Some(StackAbilityPayload {
            origin,
            definition: Some(Box::new(ability)),
            presentation: Self::ability_presentation(
                origin,
                ObjectCharacteristics::card(definition_id, CardPartId::PRIMARY),
            ),
            text: Some(ability.text),
            target_defs: plan.target_defs,
            targets: signature.targets().to_vec(),
            context: TriggerContext::empty().into(),
            condition: None,
            resolver: match (ability.declarative_effect(), followup) {
                (Some(effect), Some(behavior)) => {
                    StackAbilityResolver::DeclarativeWithCustomFollowup {
                        effect: ScopedEffect::primary(effect),
                        behavior,
                    }
                }
                _ => Self::ability_resolver(origin, &ability),
            },
            mode_effects: plan.mode_effects,
            resolution_destination: Some(resolution_destination),
            x: signature.x(),
        })
    }
}
