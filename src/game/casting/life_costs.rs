// Life costs committed while casting a spell.
//
// Included textually into `casting.rs`, so the imports here are the parent
// module's.

impl Game {
    /// The spell's own "as an additional cost to cast this spell, pay N
    /// life", if it prints one. An alternative cost replaces the mana cost
    /// rather than the additional one, so this is read whichever way the
    /// spell is being cast.
    pub(super) fn spell_life_cost(
        definition: &CardDefinition,
        option: &PlayOptionDef,
    ) -> Option<SpellLifeCostDef> {
        let (_, ability) = Self::spell_ability(definition, option)?;
        let DeclarativeAbilityDef::Spell(spell) = ability.definition else {
            return None;
        };
        spell.life_cost()
    }

    /// How much life a cast of this spell for `x` actually pays.
    pub(super) fn spell_life_payment(
        definition: &CardDefinition,
        option: &PlayOptionDef,
        x: u16,
    ) -> u16 {
        Self::spell_life_cost(definition, option).map_or(0, |cost| {
            if cost.amount_is_x {
                x
            } else {
                u16::from(cost.amount)
            }
        })
    }

    /// The largest X a "pay X life" cost can be paid at. A player may pay
    /// life only down to zero (CR 118.4), so their life total is the bound;
    /// paying none is always available.
    pub(super) fn maximum_x_for_life(&self, player: PlayerId) -> u16 {
        u16::try_from(self.players[player.index()].life.max(0)).unwrap_or(u16::MAX)
    }

    /// The life a cast owes before any mana ability is activated: what its
    /// selected alternative names, plus the spell's own additional life cost.
    pub(super) fn configured_cast_life_payment(
        &self,
        definition: &CardDefinition,
        option: &PlayOptionDef,
        card: GameObjectId,
        costs: &CostConfiguration,
        x: u16,
        offer: Option<CastOfferCost>,
    ) -> u16 {
        let alternative_life = costs
            .alternative()
            .and_then(|selected| {
                if Some(selected) == Self::temporary_alternative_cost_id(option) {
                    let required = match offer {
                        Some(CastOfferCost::GrantedAlternative(grant)) => Some(grant),
                        None | Some(CastOfferCost::Any) => None,
                        Some(CastOfferCost::PrintedAlternative(_)) => return None,
                    };
                    return self
                        .granted_alternative_cast(card, option, required)
                        .map(|(_, alternative, _)| alternative.life);
                }
                Self::alternative_cast_clause(definition, option, selected).and_then(
                    |(_, ability, _)| match ability.definition {
                        DeclarativeAbilityDef::AlternativeCast(alternative) => {
                            Some(alternative.life)
                        }
                        _ => None,
                    },
                )
            })
            .unwrap_or(0);
        alternative_life.saturating_add(Self::spell_life_payment(definition, option, x))
    }

    /// The life still available to mana abilities after an already chosen
    /// cast payment. `None` means the cast itself asks for more life than the
    /// player has, before mana planning even begins.
    pub(super) fn life_available_after_payment(
        &self,
        player: PlayerId,
        payment: u16,
    ) -> Option<u16> {
        u16::try_from(self.players[player.index()].life.max(0))
            .unwrap_or(u16::MAX)
            .checked_sub(payment)
    }

    pub(super) fn life_available_for_cast_action(
        &self,
        player: PlayerId,
        card_id: GameObjectId,
        choices: &CastChoices,
    ) -> Option<u16> {
        let state = &self.players[player.index()];
        let (card, source_zone) = state
            .hand
            .iter()
            .find(|card| card.id == card_id)
            .map(|card| (card, CastSourceZone::Hand))
            .or_else(|| {
                state
                    .graveyard
                    .iter()
                    .find(|card| card.id == card_id)
                    .map(|card| (card, CastSourceZone::Graveyard))
            })
            .or_else(|| {
                self.players
                    .iter()
                    .flat_map(|state| &state.exile)
                    .find(|card| card.id == card_id)
                    .map(|card| (card, CastSourceZone::Exile))
            })
            .or_else(|| {
                state
                    .library
                    .last()
                    .filter(|card| card.id == card_id)
                    .map(|card| (card, CastSourceZone::LibraryTop))
            })?;
        let definition = self.catalog.get(card.definition)?;
        let option = definition.play_option(choices.play_option())?;
        let cast_life = self.configured_cast_life_payment(
            definition,
            option,
            card_id,
            choices.costs(),
            choices.x(),
            self.current_cast_offer(player, card_id, source_zone)
                .map(|offer| offer.cost),
        );
        let library_life = (source_zone == CastSourceZone::LibraryTop)
            .then(|| self.library_top_life_cost(card, player, option))
            .flatten()
            .unwrap_or(0);
        self.life_available_after_payment(player, cast_life.saturating_add(library_life))
    }

    /// The life a spell cast off the top of a library owes, read while it
    /// is still up there.
    pub(super) fn library_top_life_for_cast(
        &self,
        player: PlayerId,
        card_id: GameObjectId,
        choices: &CastChoices,
    ) -> u16 {
        self.players[player.index()]
            .library
            .last()
            .filter(|top| top.id == card_id)
            .and_then(|top| {
                let definition = self.catalog.get(top.definition)?;
                let option = definition.play_option(choices.play_option())?;
                self.library_top_life_cost(top, player, option)
            })
            .unwrap_or(0)
    }
}
