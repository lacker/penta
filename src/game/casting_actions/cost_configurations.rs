//! Which combinations of costs a cast can be paid with.
//!
//! A cast is enumerated once per way of paying for it, so this is where the
//! additional costs a spell names, the alternative costs it offers, and the
//! mana each configuration ends up owing are turned into concrete
//! configurations for the caller to walk.

use super::super::{
    AbilityDef, AbilityOrigin, AdditionalCostId, AlternativeCastAbilityDef, AlternativeCastKindDef,
    AlternativeCostId, CardDefinition, CardInstance, CastCostContext, CastOfferCost,
    CastSourceZone, ControlFlow, CostConfiguration, DeclarativeAbilityDef, ExilePlayCost, Game,
    GameObjectId, ManaCost, PlayOptionDef, PlayerId, TriggerContext, ZoneKind, add_mana_cost,
    configured_mana_cost,
};
use crate::card::SpellAdditionalCostDef;

/// The chosen quantities a cost can be counted from: the X the spell is cast
/// for, and how many modes it was cast with.
#[derive(Clone, Copy)]
pub(in crate::game) struct CastScale {
    pub(in crate::game) x: u16,
    pub(in crate::game) modes: usize,
}

impl Game {
    /// Every way to pay a spell's declarative additional cost. A spell with
    /// none has exactly one way to pay it: spend nothing. A spell with one it
    /// cannot afford has none at all, which is what stops it being offered.
    #[allow(clippy::too_many_arguments)]
    pub(in crate::game) fn additional_cost_choices(
        &self,
        definition: &CardDefinition,
        option: &PlayOptionDef,
        costs: &CostConfiguration,
        card: &CardInstance,
        player: PlayerId,
        scale: CastScale,
        offer: Option<CastOfferCost>,
    ) -> Vec<Vec<GameObjectId>> {
        // A cost paid instead of the mana cost replaces the spell's own
        // additional cost rather than stacking with it: "rather than pay this
        // spell's mana cost" is the whole payment.
        let selected = costs
            .alternative()
            .and_then(|selected| Self::alternative_cast_ability(definition, option, selected))
            .and_then(|(_, ability, _)| match ability.definition {
                DeclarativeAbilityDef::AlternativeCast(alternative) => alternative.additional_cost,
                _ => None,
            })
            // A granted alternative has no printed clause to read, so the
            // cost it adds comes off the grant itself.
            .or_else(|| {
                (costs.alternative() == Self::temporary_alternative_cost_id(option))
                    .then(|| self.granted_alternative_for_offer(card.id, option, offer))
                    .flatten()
                    .and_then(|(_, alternative, _)| alternative.additional_cost)
            });
        let cost = selected.or_else(|| {
            definition
                .rules
                .ability_clauses()
                .iter()
                .find_map(|ability| match ability.definition {
                    DeclarativeAbilityDef::Spell(spell) if ability.is_executable() => {
                        spell.additional_cost()
                    }
                    _ => None,
                })
        });
        let Some(cost) = cost else {
            return vec![Vec::new()];
        };
        // "Sacrifice a creature or discard a card" is one cost with two ways
        // to pay it, so the ways of paying are the union: each half is
        // enumerated over its own zone, and a half nothing can pay simply
        // contributes nothing.
        let mut payments = Vec::new();
        for alternative in cost.alternatives() {
            for payment in self.additional_cost_payments(alternative, card, player, scale) {
                if !payments.contains(&payment) {
                    payments.push(payment);
                }
            }
        }
        payments
    }

    /// Every way to pay one half of a spell's additional cost.
    fn additional_cost_payments(
        &self,
        cost: SpellAdditionalCostDef,
        card: &CardInstance,
        player: PlayerId,
        scale: CastScale,
    ) -> Vec<Vec<GameObjectId>> {
        let candidates: Vec<GameObjectId> = match cost.zone {
            ZoneKind::Battlefield => self
                .battlefield
                .iter()
                .filter(|permanent| {
                    permanent.controller == player
                        && self.trigger_object_matches(
                            cost.object,
                            &self.trigger_event_object(permanent),
                            permanent.card.id,
                            false,
                        )
                })
                .map(|permanent| permanent.card.id)
                .collect(),
            // The same exclusion as hand below, for the same reason: escape
            // and flashback are cast from the graveyard, so by the time the
            // cost is paid the card is on the stack and not there to spend.
            // This is what "exile five other cards" means.
            ZoneKind::Graveyard => self.players[player.index()]
                .graveyard
                .iter()
                .filter(|held| {
                    held.id != card.id
                        && self.card_object_matches(cost.object, held, ZoneKind::Graveyard, held.id)
                })
                .map(|held| held.id)
                .collect(),
            // The card paying the cost cannot be the spell itself: it has
            // already left hand by the time the cost is paid.
            ZoneKind::Hand => self.players[player.index()]
                .hand
                .iter()
                .filter(|held| {
                    held.id != card.id
                        && self.card_object_matches(cost.object, held, ZoneKind::Hand, held.id)
                })
                .map(|held| held.id)
                .collect(),
            _ => Vec::new(),
        };
        // One configuration per way of paying, so a cost naming more than one
        // object enumerates combinations rather than candidates. Order does
        // not matter -- exiling A then B is the same payment as B then A --
        // so each combination appears once, in candidate order.
        let required = match cost.counted {
            crate::card::SpellAdditionalCostCountDef::Printed => usize::from(cost.count),
            crate::card::SpellAdditionalCostCountDef::ChosenX => usize::from(scale.x),
            // Escalate: a spell with one mode pays nothing extra, and every
            // mode past the first costs another one of these.
            crate::card::SpellAdditionalCostCountDef::ModesBeyondFirst => {
                usize::from(cost.count).saturating_mul(scale.modes.saturating_sub(1))
            }
        };
        Self::object_combinations(&candidates, required)
    }

    /// Every `size`-element combination of `candidates`, in candidate order.
    /// An empty requirement has exactly one payment: the empty one.
    pub(in crate::game) fn object_combinations(
        candidates: &[GameObjectId],
        size: usize,
    ) -> Vec<Vec<GameObjectId>> {
        if size == 0 {
            return vec![Vec::new()];
        }
        if candidates.len() < size {
            return Vec::new();
        }
        let mut combinations = Vec::new();
        for (index, candidate) in candidates.iter().enumerate() {
            for mut rest in Self::object_combinations(&candidates[index + 1..], size - 1) {
                let mut combination = vec![*candidate];
                combination.append(&mut rest);
                combinations.push(combination);
            }
        }
        combinations
    }

    /// Whether the card's own printed cost is one of the ways to cast it
    /// from this zone.
    ///
    /// Two zones answer with something other than yes. A foretold card was
    /// put into exile by a permission to cast it for its foretell cost and
    /// for nothing else. A card in a graveyard is castable for what it
    /// prints only while something says so -- flashback and escape are
    /// permissions to cast it for *their* cost, and they leave the printed
    /// one where it was.
    fn printed_cost_is_available_from(
        &self,
        source_zone: CastSourceZone,
        card: GameObjectId,
        player: PlayerId,
        option: &PlayOptionDef,
    ) -> bool {
        match source_zone {
            CastSourceZone::Exile => self
                .exile_play_permission(card, player)
                .is_none_or(|permission| permission.cost != ExilePlayCost::Foretell),
            CastSourceZone::Graveyard => self.players[player.index()]
                .graveyard
                .iter()
                .find(|candidate| candidate.id == card)
                .is_some_and(|instance| self.graveyard_play_is_permitted(instance, player, option)),
            CastSourceZone::Hand | CastSourceZone::LibraryTop => true,
        }
    }

    /// Whether an alternative of this kind may be used on a card being cast
    /// from this zone. What separates them is where the permission lets the
    /// card be cast from: flashback and escape are permissions to cast it
    /// where it lies, and everything else is an ordinary cast from hand paid
    /// for differently.
    fn alternative_is_castable_from(
        context: CastCostContext,
        kind: Option<AlternativeCastKindDef>,
        origin: Option<AbilityOrigin>,
    ) -> bool {
        match (context.source_zone, kind) {
            // Miracle is permission supplied by its linked trigger, not an
            // alternative that is generally available from hand.
            (CastSourceZone::Hand, Some(AlternativeCastKindDef::Miracle)) => {
                matches!(
                    (context.offer, origin),
                    (
                        Some(CastOfferCost::PrintedAlternative(required)),
                        Some(candidate)
                    ) if required == candidate
                )
            }
            (
                CastSourceZone::Hand,
                Some(
                    AlternativeCastKindDef::Overload
                        | AlternativeCastKindDef::Kicked
                        | AlternativeCastKindDef::Buyback
                        | AlternativeCastKindDef::AlternativeCost
                        | AlternativeCastKindDef::Impending
                        // Face down is a way of casting the card from hand,
                        // not a permission to cast it elsewhere.
                        | AlternativeCastKindDef::FaceDown { .. }
                )
                | None,
            )
            // Flashback, escape, and a resolution-granted free cast are
            // permissions to cast the card where it lies.
            | (
                CastSourceZone::Graveyard,
                Some(
                    AlternativeCastKindDef::Flashback
                        | AlternativeCastKindDef::Escape
                        | AlternativeCastKindDef::WithoutPayingManaCost
                ),
            )
            // Foretell is the one alternative permission that casts from
            // exile. Adventure and one-shot free permissions use the base
            // configuration instead.
            | (CastSourceZone::Exile, Some(AlternativeCastKindDef::Foretell))
            // A permission to play the top card of a library uses what that
            // play option ordinarily prints.
            | (CastSourceZone::LibraryTop, None) => true,
            _ => false,
        }
    }

    /// The applicable external alternative, narrowed to one temporary grant
    /// when a standing decision names it exactly.
    fn granted_alternative_for_offer(
        &self,
        card: GameObjectId,
        option: &PlayOptionDef,
        offer: Option<CastOfferCost>,
    ) -> Option<(AbilityDef, AlternativeCastAbilityDef, ManaCost)> {
        match offer {
            None | Some(CastOfferCost::Any) => self.granted_alternative_cast(card, option, None),
            Some(CastOfferCost::GrantedAlternative(grant)) => {
                self.granted_alternative_cast(card, option, Some(grant))
            }
            Some(CastOfferCost::PrintedAlternative(_)) => None,
        }
    }

    pub(in crate::game) fn visit_cost_configurations(
        &self,
        definition: &CardDefinition,
        card: GameObjectId,
        player: PlayerId,
        option: &PlayOptionDef,
        context: CastCostContext,
        mut visitor: impl FnMut(CostConfiguration) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let CastCostContext { source_zone, offer } = context;
        let mut selected_additional = Vec::with_capacity(option.additional_costs.len());
        if matches!(offer, None | Some(CastOfferCost::Any))
            && self.printed_cost_is_available_from(source_zone, card, player, option)
            && Self::visit_additional_cost_configurations(
                option,
                None,
                option.additional_costs.len(),
                &mut selected_additional,
                &mut visitor,
            )
            .is_break()
        {
            return ControlFlow::Break(());
        }
        for cost in &option.alternative_costs {
            let (origin, kind) = match Self::alternative_cast_clause(definition, option, cost.id) {
                Some((origin, ability, kind)) if ability.is_executable() => {
                    (Some(origin), Some(kind))
                }
                Some(_) => continue,
                None => (None, None),
            };
            if match offer {
                Some(CastOfferCost::PrintedAlternative(required)) => origin != Some(required),
                Some(CastOfferCost::GrantedAlternative(_)) => true,
                None | Some(CastOfferCost::Any) => false,
            } {
                continue;
            }
            // A free cast gated on the board is not offered while its
            // condition is false, the same way an "activate only if" ability
            // is not offered.
            let gated = match Self::alternative_cast_clause(definition, option, cost.id) {
                Some((origin, ability, _)) => match ability.definition {
                    DeclarativeAbilityDef::AlternativeCast(alternative) => {
                        // CR 118.4: life can only be paid down to zero, so an
                        // alternative that costs more life than the player
                        // has is not on offer at all.
                        i16::try_from(alternative.life).unwrap_or(i16::MAX)
                            > self.players[player.index()].life
                            || alternative.condition.is_some_and(|condition| {
                                !self.trigger_condition_holds(
                                    condition,
                                    card,
                                    player,
                                    TriggerContext::empty(),
                                    Some(origin),
                                    None,
                                )
                            })
                    }
                    _ => false,
                },
                None => false,
            };
            let available = !gated && Self::alternative_is_castable_from(context, kind, origin);
            if available
                && Self::visit_additional_cost_configurations(
                    option,
                    Some(cost.id),
                    option.additional_costs.len(),
                    &mut selected_additional,
                    &mut visitor,
                )
                .is_break()
            {
                return ControlFlow::Break(());
            }
        }
        if source_zone == CastSourceZone::Graveyard
            && let Some((_, _granted_alternative, _)) =
                self.granted_alternative_for_offer(card, option, offer)
            && let Some(granted) = Self::temporary_alternative_cost_id(option)
            && Self::visit_additional_cost_configurations(
                option,
                Some(granted),
                option.additional_costs.len(),
                &mut selected_additional,
                &mut visitor,
            )
            .is_break()
        {
            return ControlFlow::Break(());
        }

        ControlFlow::Continue(())
    }

    pub(in crate::game) fn visit_additional_cost_configurations(
        option: &PlayOptionDef,
        alternative: Option<AlternativeCostId>,
        remaining: usize,
        selected_reversed: &mut Vec<AdditionalCostId>,
        visitor: &mut impl FnMut(CostConfiguration) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let Some(index) = remaining.checked_sub(1) else {
            let additional = selected_reversed.iter().rev().copied().collect();
            return visitor(CostConfiguration::new(alternative, additional));
        };

        if Self::visit_additional_cost_configurations(
            option,
            alternative,
            index,
            selected_reversed,
            visitor,
        )
        .is_break()
        {
            return ControlFlow::Break(());
        }
        selected_reversed.push(option.additional_costs[index].id);
        let result = Self::visit_additional_cost_configurations(
            option,
            alternative,
            index,
            selected_reversed,
            visitor,
        );
        selected_reversed.pop();
        result
    }

    pub(in crate::game) fn configured_cast_mana_cost(
        &self,
        card: GameObjectId,
        option: &PlayOptionDef,
        configuration: &CostConfiguration,
        offer: Option<CastOfferCost>,
    ) -> Option<ManaCost> {
        let granted = Self::temporary_alternative_cost_id(option);
        let granted_alternative = (configuration.alternative().is_some()
            && configuration.alternative() == granted)
            .then(|| self.granted_alternative_for_offer(card, option, offer))
            .flatten();
        let mut cost = granted_alternative.map_or_else(
            || configured_mana_cost(option, configuration),
            |(_, _, mana_cost)| Some(mana_cost),
        )?;
        // `configured_mana_cost` already included additional costs for every
        // printed alternative and the normal cost. Runtime-granted
        // alternatives need them folded in here.
        if granted_alternative.is_some() {
            for selected in configuration.additional() {
                let additional = option
                    .additional_costs
                    .iter()
                    .find(|candidate| candidate.id == *selected)?;
                if let Some(mana) = additional.mana_cost {
                    cost = add_mana_cost(cost, mana);
                }
            }
        }
        // "Without paying its mana cost" and "rather than paying its mana
        // cost" are both permissions held over the card rather than
        // alternatives printed on it, so they are applied here, after
        // everything the card itself asks for. Additional costs still apply
        // (CR 601.2h); only the mana cost is replaced.
        if self.card_mana_cost_is_replaced(card) || self.library_top_cost_is_life(card, option) {
            cost = ManaCost {
                variable_x: cost.variable_x,
                x_multiplier: cost.x_multiplier,
                ..ManaCost::default()
            };
        }
        Some(cost)
    }

    /// Whether this card is the top of somebody's library and the
    /// permission reaching it charges life instead of mana. Only the topmost
    /// card of a player's own library can be, so the library it is sitting
    /// on names the player being asked.
    fn library_top_cost_is_life(&self, card: GameObjectId, option: &PlayOptionDef) -> bool {
        [PlayerId::One, PlayerId::Two].into_iter().any(|player| {
            self.players[player.index()]
                .library
                .last()
                .is_some_and(|top| {
                    top.id == card && self.library_top_life_cost(top, player, option).is_some()
                })
        })
    }

    /// Whether whoever is playing this card pays something other than its
    /// mana cost -- nothing at all, or energy. Read off the exile
    /// permissions, which is the only source today.
    fn card_mana_cost_is_replaced(&self, card: GameObjectId) -> bool {
        self.exile_play_permissions.iter().any(|permission| {
            permission.card == card
                && matches!(
                    permission.cost,
                    ExilePlayCost::Free | ExilePlayCost::EnergyEqualToManaValue
                )
        })
    }
}
