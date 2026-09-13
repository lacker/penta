// Read-only action pricing and the mana-source preview built from it.
// Included into mana_planning.rs so activation and spell payment share helpers.

impl Game {
    /// Returns the mana sources the engine's default payment policy would tap
    /// for an action. This is a read-only preview for clients; applying the
    /// action still performs the authoritative payment and validation.
    #[must_use]
    pub fn mana_sources_for_action(&self, player: PlayerId, action: &Action) -> Vec<GameObjectId> {
        let Some((cost, x, options, purpose)) = self.mana_requirement(player, action) else {
            return Vec::new();
        };
        let reserved = match action {
            Action::CastSpell { sacrifices, .. } => sacrifices.as_slice(),
            _ => &[],
        };
        let life_available = match action {
            Action::CastSpell { .. } => self.mana_ability_life_budget(player, &purpose),
            _ => u16::try_from(self.players[player.index()].life.max(0)).unwrap_or(u16::MAX),
        };
        unique_payment_source_ids(
            self.plan_mana_activations(ManaPlanningRequest {
                player,
                cost,
                x,
                options,
                purpose: &purpose,
                reserved,
                life_available,
            })
            .unwrap_or_default(),
        )
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn mana_requirement(
        &self,
        player: PlayerId,
        action: &Action,
    ) -> Option<(ManaCost, u16, ManaPlanOptions, ManaPaymentPurpose)> {
        match action {
            Action::CastSpell {
                card,
                choices,
                sacrifices,
            } => {
                let (_, held) = self.card_in_nonbattlefield_zone(*card)?;
                let definition = self.catalog.get(held.definition)?;
                let option = definition.play_option(choices.play_option())?;
                let offer = self
                    .pending_decisions
                    .iter()
                    .filter_map(|pending| pending.continuation.cast_offer())
                    .find(|offer| offer.player == player && offer.card == *card)
                    .map(|offer| offer.cost);
                let cost =
                    self.configured_cast_mana_cost(player, *card, option, choices.costs(), offer)?;
                let payment = self.spell_additional_cost_payment_for_objects(
                    super::casting_actions::SpellAdditionalCostRequest {
                        definition,
                        option,
                        costs: choices.costs(),
                        card: held,
                        player,
                        modes: choices.modes(),
                        spliced: choices.spliced(),
                        scale: super::casting_actions::CastScale {
                            x: choices.x(),
                            modes: choices.modes().len(),
                            targets: choices.iter_targets().count(),
                            offer,
                        },
                    },
                    sacrifices,
                )?;
                let (additional_mana, additional_life) = (payment.mana, payment.life);
                let alternative = self.selected_alternative_kind_for_offer(
                    definition,
                    option,
                    *card,
                    choices.costs(),
                    offer,
                );
                let spell = self.proposed_spell_view(
                    player,
                    *card,
                    &option.form,
                    alternative,
                    choices.x(),
                )?;
                let increased = add_mana_cost(
                    add_mana_cost(cost, additional_mana),
                    self.spell_cost_increase(spell, choices.targets()),
                );
                let (locked, phyrexian_life) = Self::locked_mana_payment(
                    increased,
                    choices.mana_payment(),
                    self.card_mana_is_any_color(*card),
                )?;
                let cast_life = self.configured_cast_life_payment(
                    player,
                    definition,
                    option,
                    *card,
                    choices.costs(),
                    offer,
                );
                let library_life = self
                    .players
                    .iter()
                    .flat_map(|state| &state.library)
                    .find(|candidate| candidate.id == *card)
                    .and_then(|held| self.library_top_life_cost(held, player, option))
                    .unwrap_or(0);
                let total_life = cast_life
                    .saturating_add(additional_life)
                    .saturating_add(library_life)
                    .saturating_add(phyrexian_life);
                // Emerge's reduction is settled by what the cast sacrifices,
                // so it is read off the action rather than off the board.
                let emerge = self.emerge_generic_reduction(
                    self.selected_alternative_kind_for_offer(
                        definition,
                        option,
                        *card,
                        choices.costs(),
                        offer,
                    ),
                    sacrifices,
                );
                Some((
                    reduce_generic(
                        Self::apply_spell_cost_reduction(
                            Self::apply_harmonize_reduction(
                                locked,
                                choices.x(),
                                payment.generic_reduction,
                            ),
                            self.spell_cost_reduction(spell, choices.targets()),
                            choices.x(),
                        ),
                        emerge,
                    ),
                    choices.x(),
                    ManaPlanOptions {
                        avoid: None,
                        tap_cost_payer: payment.tap_cost_payer(),
                    },
                    ManaPaymentPurpose::Spell {
                        object: *card,
                        commander_owner: self.commander_owner(*card),
                        definition: definition.id,
                        controller: player,
                        form: option.form.clone(),
                        reserved_life_payment: total_life,
                    },
                ))
            }
            Action::ActivateAbility {
                source,
                ability,
                targets,
                cost_objects,
                x,
                mana_payment,
                ..
            } => self.ability_mana_requirement(AbilityManaRequest {
                player,
                source: *source,
                ability: *ability,
                targets,
                cost_objects,
                x: *x,
                mana_payment: mana_payment.as_deref(),
            }),
            _ => None,
        }
    }
}
