// Which printed clause a chosen cast configuration actually resolves.
//
// Split from action enumeration because it answers a different question:
// not what a player may do, but which of a card's clauses the choice they
// already made selects -- the base spell, an overloaded one, or a kicked
// one -- and what that clause puts on the stack.

impl Game {
    /// The rebound clause carried by the spell form being cast. A combined
    /// split spell has the keyword if either of its component parts does.
    pub(super) fn rebound_ability_origin(
        definition: &CardDefinition,
        option: &PlayOptionDef,
    ) -> Option<AbilityOrigin> {
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
                    (matches!(
                            attached.definition.definition,
                            DeclarativeAbilityDef::Keyword(crate::card::KeywordAbility::Rebound)
                        ))
                    .then_some(AbilityOrigin::Printed {
                        definition: definition.id,
                        part: *part_id,
                        ability: attached.id,
                    })
                })
        })
    }

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
                    (attached.additional_cost_id() == Some(additional))
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

    /// Emerge's reduction (CR 702.119a): the emerge cost is reduced by the
    /// mana value of the permanent sacrificed to pay it. The reduction is
    /// generic only, so the caller applies it with `reduce_generic` and the
    /// coloured pips are still owed in their own colours.
    ///
    /// Read off the objects the cast spends rather than off any choice of
    /// its own: the sacrifice named in `additional_cost` is what settles
    /// this, which is the one thing emerge adds to an ordinary alternative.
    pub(in crate::game) fn emerge_generic_reduction(
        &self,
        kind: Option<AlternativeCastKindDef>,
        sacrifices: &[GameObjectId],
    ) -> u16 {
        if kind != Some(AlternativeCastKindDef::Emerge) {
            return 0;
        }
        sacrifices
            .iter()
            .filter_map(|sacrifice| {
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == *sacrifice)
            })
            .filter_map(|permanent| permanent.card.definition.card_definition())
            .filter_map(|definition| self.catalog.get(definition))
            .map(|definition| definition.rules.printed_mana_cost().mana_value())
            .fold(0_u16, u16::saturating_add)
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
        if Some(selected) == Self::temporary_alternative_cost_id(option) {
            return self
                .granted_alternative_cast(
                    card,
                    option,
                    match offer {
                        Some(CastOfferCost::GrantedAlternative(grant)) => Some(grant),
                        None | Some(CastOfferCost::Any) => None,
                        Some(CastOfferCost::PrintedAlternative(_)) => return None,
                    },
                )
                .map(|(_, alternative, _)| alternative.kind);
        }
        Self::alternative_cast_ability(definition, option, selected)
            .map(|(_, _, kind)| kind)
            // A runtime alternative has no printed clause to recover. Its ID
            // was validated while the cast was announced, and the signature
            // must keep saying it was cast for an alternative cost even after
            // its battlefield source is gone -- but somebody else's cost is
            // not the card's own, which is what a rider on the printed
            // alternative asks about.
            .or(Some(AlternativeCastKindDef::Granted))
    }

    /// The smallest X the selected alternative may be cast for. "Kicker
    /// {X}. X can't be 0" is the only thing that says so, and it says it
    /// about the kicker rather than about the spell.
    pub(super) fn configured_alternative_minimum_x(
        definition: &CardDefinition,
        option: &PlayOptionDef,
        costs: &CostConfiguration,
    ) -> u16 {
        let Some(selected) = costs.alternative() else {
            return 0;
        };
        Self::alternative_cast_ability(definition, option, selected)
            .and_then(|(_, ability, _)| match ability.definition {
                DeclarativeAbilityDef::AlternativeCast(alternative) => Some(alternative.minimum_x),
                _ => None,
            })
            .unwrap_or(0)
    }

    pub(super) fn temporary_alternative_cost_id(
        option: &PlayOptionDef,
    ) -> Option<AlternativeCostId> {
        Self::external_alternative_cost_id(option, 0)
    }

    /// A stable ID for an alternative supplied outside the printed card.
    /// Slot zero remains the one-shot/graveyard grant; battlefield alternatives
    /// use later slots so their reconstruction never collides with it.
    fn external_alternative_cost_id(
        option: &PlayOptionDef,
        slot: usize,
    ) -> Option<AlternativeCostId> {
        (u8::MIN..=u8::MAX)
            .rev()
            .map(AlternativeCostId)
            .filter(|candidate| {
                option
                    .alternative_costs
                    .iter()
                    .all(|cost| cost.id != *candidate)
            })
            .nth(slot)
    }

    pub(super) fn battlefield_alternative_cost_id(
        option: &PlayOptionDef,
        index: usize,
    ) -> Option<AlternativeCostId> {
        Self::external_alternative_cost_id(option, index.saturating_add(1))
    }

    pub(super) fn battlefield_spell_alternative_cost_for_id(
        &self,
        player: PlayerId,
        card: GameObjectId,
        option: &PlayOptionDef,
        selected: AlternativeCostId,
    ) -> Option<ManaCost> {
        self.battlefield_spell_alternative_costs(player, card)
            .into_iter()
            .enumerate()
            .find_map(|(index, cost)| {
                (Self::battlefield_alternative_cost_id(option, index) == Some(selected))
                    .then_some(cost)
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
        let resolve = |grant: &NonbattlefieldAbilityGrant| {
            (grant.object == card).then_some(())?;
            let DeclarativeAbilityDef::AlternativeCast(alternative) = grant.ability.definition
            else {
                return None;
            };
            matches!(
                alternative.kind,
                AlternativeCastKindDef::Flashback
                    | AlternativeCastKindDef::WithoutPayingManaCost
                    // Rebound's own card, lent back to its caster out of the
                    // exile it put itself in.
                    | AlternativeCastKindDef::Rebound
            )
            .then_some((grant.ability, alternative))
        };
        let temporary = match required {
            Some(grant) => self.nonbattlefield_ability_grants.get(grant).and_then(resolve),
            None => self.nonbattlefield_ability_grants.iter().find_map(resolve),
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
    pub(super) fn frozen_spell_payload(
        &self,
        definition_id: CardDefinitionId,
        signature: &CastSignature,
    ) -> Option<StackAbilityPayload> {
        let definition = self.catalog.get(definition_id)?;
        let option = definition.play_option(signature.play_option())?;
        // A creature card has no spell clause at all. Ordinarily that means
        // there is nothing to put on the stack beyond the card itself -- but
        // a bestowed one resolves an Aura clause that is not the card's, so
        // the base clause is looked up without being required.
        let spell_clause = Self::spell_ability(definition, option);
        let mut resolution_destination = spell_clause.map_or(
            crate::card::SpellResolutionDestinationDef::Graveyard,
            |(_, ability)| match ability.definition {
                DeclarativeAbilityDef::Spell(spell) => spell.resolution_destination(),
                _ => crate::card::SpellResolutionDestinationDef::Graveyard,
            },
        );
        if resolution_destination == crate::card::SpellResolutionDestinationDef::Graveyard
            && Self::rebound_ability_origin(definition, option).is_some()
        {
            resolution_destination = crate::card::SpellResolutionDestinationDef::Rebound;
        }
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
                AlternativeCastKindDef::Overload
                | AlternativeCastKindDef::Kicked
                // Bestow is not a cheaper way to cast the same spell: it is
                // an Aura spell with a target of its own, so it resolves the
                // clause that says so.
                | AlternativeCastKindDef::Bestow,
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
                sacrificed_mana_value: 0,
            });
        }
        let (origin, ability) = spell_clause?;
        let DeclarativeAbilityDef::Spell(spell) = ability.definition else {
            unreachable!("spell_ability returns a spell clause")
        };
        let spliced = self
            .spliced_clauses_of(signature.spliced())
            .expect("validated splices name cards that print a clause to add");
        let plan = Self::selected_spell_plan(spell, signature.modes(), &spliced)
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
            resolver: Self::ability_resolver(origin, &ability),
            mode_effects: plan.mode_effects,
            resolution_destination: Some(resolution_destination),
            x: signature.x(),
            sacrificed_mana_value: 0,
        })
    }
}
