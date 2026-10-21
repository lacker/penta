//! Compact action authority and identity information for proposal clients.
use super::{Game, PlayerId, Pregame, Step};
use serde_json::{Value, json};

impl Game {
    /// Describe the current choice without constructing action alternatives.
    #[must_use]
    pub fn action_context(&self, player: PlayerId) -> Value {
        let mut context = json!({ "kind": "waiting", "ordinaryPriority": false });
        if self.result.is_some() && !self.between_games() {
            context["kind"] = json!("complete");
            return context;
        }
        if self.decision_player() != Some(player) {
            return context;
        }
        let kind = if self.match_decision().is_some() || !self.pending_decisions.is_empty() {
            "decision"
        } else if let Some(attacker) = self.pending_combat_assignments.first() {
            context["attacker"] = json!(attacker.0);
            "combatDamage"
        } else if let Some(pregame) = self.pregame {
            match pregame {
                Pregame::Mulligan(_) => "mulligan",
                Pregame::Bottom(_) => {
                    let count = usize::from(self.mulligans[player.index()])
                        .min(self.players[player.index()].hand.len());
                    context["minimum"] = json!(count);
                    context["maximum"] = json!(count);
                    "bottomCards"
                }
                Pregame::Companion(_) | Pregame::OpeningHand(_) => "decision",
            }
        } else if self.cleanup_pending {
            let count = self.players[player.index()]
                .hand
                .len()
                .saturating_sub(self.maximum_hand_size(player).unwrap_or(usize::MAX));
            context["minimum"] = json!(count);
            context["maximum"] = json!(count);
            "discardCards"
        } else if self.untap_pending {
            "untap"
        } else if self.step == Step::DeclareAttackers && !self.attackers_declared {
            "declareAttackers"
        } else if self.step == Step::DeclareBlockers && !self.blockers_declared {
            "declareBlockers"
        } else {
            context["ordinaryPriority"] = json!(true);
            "priority"
        };
        context["kind"] = json!(kind);
        context["castOffers"] = json!(self.pending_decisions.iter().filter_map(|pending| {
            let offer = pending.continuation.cast_offer()?;
            (offer.player == player).then(|| json!({
                "card": offer.card.0,
                "sourceZone": match offer.source_zone {
                    super::CastSourceZone::Hand => "hand",
                    super::CastSourceZone::Command => "command",
                    super::CastSourceZone::Graveyard => "graveyard",
                    super::CastSourceZone::Exile => "exile",
                    super::CastSourceZone::Library => "libraryTop",
                },
                "cost": match offer.cost {
                    super::CastOfferCost::Any => json!({ "kind": "any" }),
                    super::CastOfferCost::PrintedAlternative(_) => json!({ "kind": "printedAlternative" }),
                    super::CastOfferCost::GrantedAlternative(index) => json!({ "kind": "grantedAlternative", "index": index }),
                },
                "mandatory": pending.continuation.cast_offer_is_mandatory(),
                "decision": pending.observation.id,
            }))
        }).collect::<Vec<_>>());
        context
    }

    /// Origin for a printed alternative required by a currently visible cast offer.
    #[must_use]
    pub fn cast_offer_ability_origin(
        &self,
        player: PlayerId,
        card: crate::GameObjectId,
    ) -> Option<crate::AbilityOrigin> {
        self.pending_decisions
            .iter()
            .filter_map(|pending| pending.continuation.cast_offer())
            .find_map(|offer| match offer.cost {
                super::CastOfferCost::PrintedAlternative(origin)
                    if offer.player == player && offer.card == card =>
                {
                    Some(origin)
                }
                _ => None,
            })
    }

    /// Current battlefield and emblem ability identities, including grants and
    /// intrinsic abilities. Callers must only request objects visible to them.
    #[must_use]
    pub fn ability_origins(&self, source: crate::GameObjectId) -> Vec<crate::AbilityOrigin> {
        self.battlefield
            .iter()
            .chain(self.emblems.iter())
            .find(|permanent| permanent.card.id == source)
            .map(|permanent| {
                self.effective_abilities(permanent)
                    .into_iter()
                    .map(|ability| ability.origin)
                    .collect()
            })
            .unwrap_or_default()
    }
    /// Effective ability definitions paired with their live identities.
    #[must_use]
    pub fn action_abilities(
        &self,
        source: crate::GameObjectId,
    ) -> Vec<(crate::AbilityOrigin, crate::card::AbilityDef)> {
        if let Some(permanent) = self
            .battlefield
            .iter()
            .chain(self.emblems.iter())
            .find(|permanent| permanent.card.id == source)
        {
            return self
                .effective_abilities(permanent)
                .into_iter()
                .map(|ability| (ability.origin, ability.ability))
                .collect();
        }
        let Some((zone, card)) = self.card_in_nonbattlefield_zone(source) else {
            return Vec::new();
        };
        let context = match zone {
            crate::ZoneKind::Library => super::CharacteristicContext::Library,
            crate::ZoneKind::Hand => super::CharacteristicContext::Hand,
            crate::ZoneKind::Graveyard => super::CharacteristicContext::Graveyard,
            crate::ZoneKind::Exile => super::CharacteristicContext::Exile,
            crate::ZoneKind::Command => super::CharacteristicContext::Command,
            crate::ZoneKind::Battlefield | crate::ZoneKind::Stack => return Vec::new(),
        };
        let mut abilities = Vec::new();
        self.for_each_printed_card_ability(card, &context, |ability| {
            abilities.push((ability.origin, ability.ability));
        });
        abilities
    }

    /// The currently applicable zone permission, without proving timing or
    /// payment. Hosts include this only for objects visible to the requesting seat.
    #[must_use]
    pub fn action_play_permission(
        &self,
        player: PlayerId,
        source: crate::GameObjectId,
    ) -> Option<Value> {
        let (zone, _) = self.card_in_nonbattlefield_zone(source)?;
        let (source_zone, permission) = match zone {
            crate::ZoneKind::Exile => ("exile", self.exile_play_permission(source, player)?),
            crate::ZoneKind::Graveyard => {
                ("graveyard", self.graveyard_cast_permission(source, player)?)
            }
            _ => return None,
        };
        let mana = match permission.cost {
            super::ExilePlayCost::AlternativeMana(mana) => Some(mana.to_string()),
            _ => None,
        };
        Some(json!({
            "sourceZone": source_zone,
            "cost": {"kind": permission.cost.label(), "manaCost": mana},
            "surcharge": permission.surcharge.to_string(),
            "landsMayBePlayed": permission.lands_may_be_played,
            "adventureReturnOnly": permission.adventure_return_only,
            "maximumSpellManaValue": permission.maximum_spell_mana_value,
            "spendAnyColor": permission.spend_any_color,
            "spendAnyType": permission.spend_any_type,
            "grantsHaste": permission.grants_haste,
        }))
    }

    /// Runtime cost identities for a visible card. These describe grants, not
    /// payable action alternatives: no target or funding plans are searched.
    #[must_use]
    pub fn action_cast_costs(&self, player: PlayerId, source: crate::GameObjectId) -> Vec<Value> {
        let Some((zone, card)) = self.card_in_nonbattlefield_zone(source) else {
            return Vec::new();
        };
        let Some(definition) = self.catalog.get(card.definition) else {
            return Vec::new();
        };
        let required = self
            .pending_decisions
            .iter()
            .filter_map(|pending| pending.continuation.cast_offer())
            .find_map(|offer| match offer.cost {
                super::CastOfferCost::GrantedAlternative(index)
                    if offer.player == player && offer.card == source =>
                {
                    Some(index)
                }
                _ => None,
            });
        let mut costs = Vec::new();
        for option in &definition.play_options {
            if matches!(zone, crate::ZoneKind::Graveyard | crate::ZoneKind::Exile)
                && let Some((ability, _, mana)) =
                    self.granted_alternative_cast(source, option, required)
                && let Some(id) = Self::temporary_alternative_cost_id(option)
            {
                costs.push(json!({"playOptionId":option.id.0,"alternativeCostId":id.0,
                    "kind":"granted","text":ability.rules_text(),"manaCost":mana.to_string()}));
            }
            if let Some(spell) = self.proposed_spell_view(player, source, &option.form, None, 0) {
                for (index, cost) in self
                    .battlefield_spell_alternative_costs(spell)
                    .into_iter()
                    .enumerate()
                {
                    if let Some(id) = Self::battlefield_alternative_cost_id(option, index)
                        && let Some(mana) = crate::card::costs::mana_cost(cost, option.mana_cost)
                    {
                        costs.push(json!({"playOptionId":option.id.0,"alternativeCostId":id.0,
                            "kind":"battlefield","text":"Alternative cost granted by battlefield effects",
                            "manaCost":mana.to_string()}));
                    }
                }
            }
        }
        costs
    }
}
