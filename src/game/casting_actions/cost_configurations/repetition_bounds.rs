impl Game {
    /// A resource ceiling for enumerating repeated optional payments. Final
    /// payment planning still checks the complete cost and shared resources.
    fn repeatable_additional_cost_bound(
        &self,
        definition: &CardDefinition,
        card: GameObjectId,
        player: PlayerId,
        option: &PlayOptionDef,
    ) -> u16 {
        let Some((_, held)) = self.card_in_nonbattlefield_zone(card) else {
            return 0;
        };
        let purpose = ManaPaymentPurpose::Spell {
            object: card,
            definition: definition.id,
            controller: player,
            form: option.form.clone(),
            reserved_life_payment: 0,
        };
        let mana = self.available_mana_ceiling(player, &purpose);
        option
            .additional_costs
            .iter()
            .filter(|cost| cost.repeatable)
            .filter_map(|cost| Self::optional_additional_cost_clause(definition, option, cost.id))
            .filter_map(|(_, ability, _)| match ability.definition {
                DeclarativeAbilityDef::OptionalAdditionalCost(cost) => Some(cost.costs),
                _ => None,
            })
            .map(|costs| {
                costs
                    .iter()
                    .filter_map(|cost| self.repeated_cost_resource_bound(*cost, held, player, mana))
                    .min()
                    .unwrap_or(0)
            })
            .max()
            .unwrap_or(0)
    }

    fn repeated_cost_resource_bound(
        &self,
        cost: CostDef,
        card: &CardInstance,
        player: PlayerId,
        mana: u16,
    ) -> Option<u16> {
        let cost = Self::canonical_spell_cost(cost);
        let count = |n: usize| u16::try_from(n).unwrap_or(u16::MAX);
        match cost {
            CostDef::Mana(cost) if cost.mana_value() > 0 => Some(mana / cost.mana_value()),
            CostDef::PayLife(amount) if amount > 0 => {
                Some(self.maximum_x_for_life(player) / amount)
            }
            CostDef::DiscardCards(amount) if amount > 0 => Some(
                count(
                    self.players[player.index()]
                        .hand
                        .iter()
                        .filter(|held| held.id != card.id)
                        .count(),
                ) / amount,
            ),
            CostDef::Sacrifice {
                quantity: crate::card::CostQuantityDef::Fixed(amount),
                ..
            }
            | CostDef::Discard {
                quantity: crate::card::CostQuantityDef::Fixed(amount),
                ..
            }
            | CostDef::Exile {
                quantity: crate::card::CostQuantityDef::Fixed(amount),
                ..
            }
            | CostDef::ReturnToHand {
                quantity: crate::card::CostQuantityDef::Fixed(amount),
                ..
            }
            | CostDef::Tap {
                quantity: crate::card::CostQuantityDef::Fixed(amount),
                ..
            } if amount > 0 => Some(
                count(self.additional_cost_candidates(cost, card, player).len())
                    / u16::from(amount),
            ),
            CostDef::All(costs) => costs
                .iter()
                .filter_map(|cost| self.repeated_cost_resource_bound(*cost, card, player, mana))
                .min(),
            CostDef::Choice(costs) => costs
                .iter()
                .filter_map(|cost| self.repeated_cost_resource_bound(*cost, card, player, mana))
                .max(),
            _ => None,
        }
    }
}
