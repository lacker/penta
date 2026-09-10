// Whether a selected cost includes a mana payment is independent of its
// amount. In particular, an empty alternative and an explicit {0} differ.
impl Game {
    pub(in crate::game) fn configured_cast_includes_mana_payment(
        &self,
        player: PlayerId,
        card: GameObjectId,
        definition: &CardDefinition,
        option: &PlayOptionDef,
        configuration: &CostConfiguration,
        offer: Option<CastOfferCost>,
    ) -> bool {
        let base = if self.card_mana_cost_is_replaced(card)
            || self.library_top_cost_is_life(card, option)
        {
            false
        } else if let Some(selected) = configuration.alternative() {
            if let Some(costs) =
                self.battlefield_spell_alternative_cost_for_id(player, card, option, selected)
            {
                crate::card::costs::includes_fixed_mana_payment(costs)
            } else if Some(selected) == Self::temporary_alternative_cost_id(option) {
                self.granted_alternative_for_offer(card, option, offer)
                    .is_some_and(|(_, alternative, _)| {
                        crate::card::costs::includes_fixed_mana_payment(alternative.costs)
                    })
            } else {
                Self::alternative_cast_ability(definition, option, selected).is_some_and(
                    |(_, ability, _)| match ability.definition {
                        DeclarativeAbilityDef::AlternativeCast(alternative) => {
                            crate::card::costs::includes_fixed_mana_payment(alternative.costs)
                        }
                        _ => false,
                    },
                )
            }
        } else {
            option.mana_cost.is_some()
        };
        base || configuration.additional().iter().any(|selected| {
            Self::optional_additional_cost_clause(definition, option, *selected).is_some_and(
                |(_, ability, _)| match ability.definition {
                    DeclarativeAbilityDef::OptionalAdditionalCost(optional) => {
                        crate::card::costs::includes_fixed_mana_payment(optional.costs)
                    }
                    _ => false,
                },
            )
        })
    }
}
