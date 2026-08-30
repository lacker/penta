// Life costs committed while casting a spell.
//
// Included textually into `casting.rs`, so the imports here are the parent
// module's.

impl Game {
    /// The largest X a "pay X life" cost can be paid at. A player may pay
    /// life only down to zero (CR 118.4), so their life total is the bound;
    /// paying none is always available.
    pub(super) fn maximum_x_for_life(&self, player: PlayerId) -> u16 {
        if self.life_total_cannot_change(player) {
            return 0;
        }
        u16::try_from(self.players[player.index()].life.max(0)).unwrap_or(u16::MAX)
    }

    /// Every non-mana life or energy cost a cast owes, paid together and
    /// before the spell is finished on the stack.
    ///
    /// Life named by the chosen alternative, by the spell's own additional
    /// cost, or by the permission that let it be cast off a library is the
    /// caster's. "Have an opponent gain 3 life" is the same kind of cost
    /// pointed the other way: it is paid here rather than on resolution, and
    /// paid even if the spell is answered afterwards.
    pub(super) fn pay_cast_life_and_energy(
        &mut self,
        player: PlayerId,
        life: u16,
        opponent_life_gain: u16,
        energy: u16,
    ) {
        if life > 0 {
            self.lose_life(player, life);
        }
        if opponent_life_gain > 0 {
            self.gain_life(player.opponent(), opponent_life_gain);
        }
        if energy > 0 {
            self.spend_energy(player, energy);
        }
    }

    /// The same, read off the card being cast rather than off a definition
    /// the caller already has in hand.
    pub(super) fn cast_opponent_life_gain(
        &self,
        card: GameObjectId,
        signature: &CastSignature,
    ) -> u16 {
        self.object_definition(card)
            .and_then(|definition| self.catalog.get(definition))
            .and_then(|definition| {
                let option = definition.play_option(signature.play_option())?;
                Some(Self::configured_cast_opponent_life_gain(
                    definition,
                    option,
                    signature.costs(),
                ))
            })
            .unwrap_or(0)
    }

    /// The life an opponent gains as a cost of this cast, which is what
    /// Invigorate charges instead of mana. Read the same way the caster's own
    /// life payment is, off whichever alternative was selected.
    pub(super) fn configured_cast_opponent_life_gain(
        definition: &CardDefinition,
        option: &PlayOptionDef,
        costs: &CostConfiguration,
    ) -> u16 {
        costs
            .alternative()
            .and_then(|selected| {
                Self::alternative_cast_clause(definition, option, selected).and_then(
                    |(_, ability, _)| match ability.definition {
                        DeclarativeAbilityDef::AlternativeCast(alternative) => {
                            Some(alternative.opponent_life_gain)
                        }
                        _ => None,
                    },
                )
            })
            .unwrap_or(0)
    }

    /// The life a selected alternative cast cost owes before any mana ability
    /// is activated. A spell's semantic additional cost is selected
    /// separately and added by the caller.
    pub(super) fn configured_cast_life_payment(
        &self,
        definition: &CardDefinition,
        option: &PlayOptionDef,
        card: GameObjectId,
        costs: &CostConfiguration,
        _x: u16,
        offer: Option<CastOfferCost>,
    ) -> u16 {
        costs
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
            .unwrap_or(0)
    }

    /// The life still available to mana abilities after an already chosen
    /// cast payment. `None` means the cast itself asks for more life than the
    /// player has, before mana planning even begins.
    pub(super) fn life_available_after_payment(
        &self,
        player: PlayerId,
        payment: u16,
    ) -> Option<u16> {
        if self.life_total_cannot_change(player) {
            return (payment == 0).then_some(0);
        }
        u16::try_from(self.players[player.index()].life.max(0))
            .unwrap_or(u16::MAX)
            .checked_sub(payment)
    }

    #[allow(dead_code)]
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

    fn cast_object_payments_and_life(
        &self,
        player: PlayerId,
        card_id: GameObjectId,
        signature: &CastSignature,
        behavior: CardBehavior,
        context: super::CastCostContext,
        sacrifices: &[GameObjectId],
    ) -> (Vec<(GameObjectId, SpellAdditionalCostDef)>, u16) {
        let super::CastCostContext { source_zone, offer } = context;
        let held = match source_zone {
            CastSourceZone::Hand => self.players[player.index()]
                .hand
                .iter()
                .find(|card| card.id == card_id),
            CastSourceZone::Graveyard => self.players[player.index()]
                .graveyard
                .iter()
                .find(|card| card.id == card_id),
            CastSourceZone::Exile => self
                .players
                .iter()
                .flat_map(|state| &state.exile)
                .find(|card| card.id == card_id),
            CastSourceZone::LibraryTop => self.players[player.index()]
                .library
                .last()
                .filter(|card| card.id == card_id),
        }
        .expect("the validated cast card remains in its source zone");
        let definition = self
            .catalog
            .get(held.definition)
            .expect("a validated cast definition remains in the catalog");
        let option = definition
            .play_option(signature.play_option())
            .expect("a validated cast option remains in the catalog");
        let payment = if behavior == CardBehavior::GoblinGrenade {
            super::casting_actions::SpellAdditionalCostPayment {
                objects: sacrifices
                    .iter()
                    .copied()
                    .map(|object| {
                        (
                            object,
                            SpellAdditionalCostDef::sacrifice(
                                crate::card::ObjectPredicateDef::Any,
                                1,
                            ),
                        )
                    })
                    .collect(),
                mana: ManaCost::default(),
                life: 0,
            }
        } else {
            self.spell_additional_cost_payment_for_objects(
                super::casting_actions::SpellAdditionalCostRequest {
                    definition,
                    option,
                    costs: signature.costs(),
                    card: held,
                    player,
                    scale: super::casting_actions::CastScale {
                        x: signature.x(),
                        modes: signature.modes().len(),
                        offer,
                    },
                },
                sacrifices,
            )
            .expect("a validated object payment remains a legal semantic payment")
        };
        assert_eq!(
            sacrifices,
            payment.object_ids(),
            "a validated object payment retains its semantic action",
        );
        let life = self
            .configured_cast_life_payment(
                definition,
                option,
                card_id,
                signature.costs(),
                signature.x(),
                offer,
            )
            .saturating_add(payment.life);
        (payment.objects, life)
    }
}
