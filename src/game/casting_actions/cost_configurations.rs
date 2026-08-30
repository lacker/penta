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
    configured_base_mana_cost,
};
use crate::card::SpellAdditionalCostDef;
use crate::game::ManaPaymentPurpose;

/// The chosen quantities a cost can be counted from: the X the spell is cast
/// for, and how many modes it was cast with.
#[derive(Clone, Copy)]
pub(in crate::game) struct CastScale {
    pub(in crate::game) x: u16,
    pub(in crate::game) modes: usize,
    pub(in crate::game) offer: Option<CastOfferCost>,
}

#[derive(Clone, Copy)]
struct PrintedAlternativeRequest<'a> {
    definition: &'a CardDefinition,
    card: GameObjectId,
    player: PlayerId,
    option: &'a PlayOptionDef,
    context: CastCostContext,
}

impl Game {
    pub(in crate::game) fn selected_object_additional_costs(
        &self,
        definition: &CardDefinition,
        option: &PlayOptionDef,
        costs: &CostConfiguration,
        card: &CardInstance,
        offer: Option<CastOfferCost>,
    ) -> Vec<SpellAdditionalCostDef> {
        let selected_alternative = costs
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
        let mut required = Vec::new();
        if let Some(cost) = selected_alternative {
            required.push(cost);
        }
        // An alternative replaces only the spell's mana cost. Every mandatory
        // additional cost printed by the spell still applies (CR 118.9d).
        if let Some(cost) = definition
            .rules
            .ability_clauses()
            .iter()
            .find_map(|ability| match ability.definition {
                DeclarativeAbilityDef::Spell(spell) if ability.is_executable() => {
                    spell.additional_cost()
                }
                _ => None,
            })
        {
            required.push(cost);
        }
        for selected in costs.additional() {
            if let Some((_, ability, _)) =
                Self::optional_additional_cost_clause(definition, option, *selected)
                && let DeclarativeAbilityDef::OptionalAdditionalCost(optional) = ability.definition
                && let Some(cost) = optional.additional_cost
            {
                required.push(cost);
            }
        }
        required
    }

    /// Every way to pay a spell's declarative additional cost. A spell with
    /// none has exactly one way to pay it: spend nothing. A spell with one it
    /// cannot afford has none at all, which is what stops it being offered.
    pub(in crate::game) fn additional_cost_choices(
        &self,
        definition: &CardDefinition,
        option: &PlayOptionDef,
        costs: &CostConfiguration,
        card: &CardInstance,
        player: PlayerId,
        scale: CastScale,
    ) -> Vec<Vec<GameObjectId>> {
        let required =
            self.selected_object_additional_costs(definition, option, costs, card, scale.offer);
        if required.is_empty() {
            return vec![Vec::new()];
        }

        let mut combined = vec![Vec::new()];
        for cost in required {
            // "Sacrifice a creature or discard a card" is one cost with two
            // ways to pay it, so the ways for this one cost are a union.
            let mut ways = Vec::new();
            for alternative in cost.alternatives() {
                for payment in self.additional_cost_payments(alternative, card, player, scale) {
                    if !ways.contains(&payment) {
                        ways.push(payment);
                    }
                }
            }
            // "... or pay N life" spends no object, so the way it is paid is
            // the empty one. Offered only when the life is there: paying
            // down to exactly zero is legal (CR 118.4), below it is not.
            if cost
                .life_alternatives()
                .into_iter()
                .any(|life| i64::from(life) <= i64::from(self.players[player.index()].life))
                && !ways.contains(&Vec::new())
            {
                ways.push(Vec::new());
            }
            // Separate additional costs all have to be paid. Form their
            // Cartesian product without allowing one object to pay twice.
            let mut next = Vec::new();
            for paid in &combined {
                for way in &ways {
                    if way.iter().any(|object| paid.contains(object)) {
                        continue;
                    }
                    let mut payment = paid.clone();
                    payment.extend(way);
                    if !next.contains(&payment) {
                        next.push(payment);
                    }
                }
            }
            combined = next;
        }
        combined
    }

    /// How many objects one additional cost spends. A collect-evidence cost
    /// has no printed answer: it takes whatever reaches its total, which is
    /// `remaining` -- the objects the payment still has left over after the
    /// counted costs before it. No card prints two of them, so there is
    /// never a second one to divide the remainder with.
    pub(in crate::game) fn additional_cost_object_count(
        cost: SpellAdditionalCostDef,
        scale: CastScale,
        remaining: usize,
    ) -> usize {
        match cost.counted {
            crate::card::SpellAdditionalCostCountDef::Printed => usize::from(cost.count),
            crate::card::SpellAdditionalCostCountDef::ChosenX => usize::from(scale.x),
            crate::card::SpellAdditionalCostCountDef::ModesBeyondFirst => {
                usize::from(cost.count).saturating_mul(scale.modes.saturating_sub(1))
            }
            // Both open-ended measures take whatever the payment had left.
            crate::card::SpellAdditionalCostCountDef::TotalManaValueAtLeast(_)
            | crate::card::SpellAdditionalCostCountDef::CardTypesAtLeast(_) => remaining,
        }
    }

    /// The spend operation paired with each object in one generated action.
    /// `additional_cost_choices` concatenates costs in this same order, so
    /// carrying the parallel list through payment preserves each cost's
    /// provenance without changing the public action shape.
    pub(in crate::game) fn additional_cost_spend_modes(
        &self,
        definition: &CardDefinition,
        option: &PlayOptionDef,
        costs: &CostConfiguration,
        card: &CardInstance,
        scale: CastScale,
        paid_objects: usize,
    ) -> Vec<crate::card::SpendModeDef> {
        let mut modes = Vec::new();
        let mut remaining = paid_objects;
        for cost in
            self.selected_object_additional_costs(definition, option, costs, card, scale.offer)
        {
            let count = Self::additional_cost_object_count(cost, scale, remaining);
            remaining = remaining.saturating_sub(count);
            modes.extend(core::iter::repeat_n(cost.spend, count));
        }
        modes
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
        if let crate::card::SpellAdditionalCostCountDef::TotalManaValueAtLeast(total) = cost.counted
        {
            return self.mana_value_combinations(&candidates, u16::from(total));
        }
        if let crate::card::SpellAdditionalCostCountDef::CardTypesAtLeast(types) = cost.counted {
            return self.card_type_combinations(&candidates, u16::from(types));
        }
        let required = match cost.counted {
            crate::card::SpellAdditionalCostCountDef::Printed => usize::from(cost.count),
            crate::card::SpellAdditionalCostCountDef::ChosenX => usize::from(scale.x),
            // Escalate: a spell with one mode pays nothing extra, and every
            // mode past the first costs another one of these.
            crate::card::SpellAdditionalCostCountDef::ModesBeyondFirst => {
                usize::from(cost.count).saturating_mul(scale.modes.saturating_sub(1))
            }
            crate::card::SpellAdditionalCostCountDef::TotalManaValueAtLeast(_)
            | crate::card::SpellAdditionalCostCountDef::CardTypesAtLeast(_) => 0,
        };
        Self::object_combinations(&candidates, required)
    }

    /// Every way to reach `total` mana value that wastes nothing: a set
    /// counts only if dropping any one of its cards would leave it short.
    /// The rules permit exiling more than that, but every superset is a
    /// strictly worse payment of the same cost, and enumerating them all
    /// would grow the action list exponentially in the size of a graveyard.
    fn mana_value_combinations(
        &self,
        candidates: &[GameObjectId],
        total: u16,
    ) -> Vec<Vec<GameObjectId>> {
        let values = candidates
            .iter()
            .map(|id| {
                self.card_in_nonbattlefield_zone(*id)
                    .and_then(|(_, card)| self.catalog.get(card.definition))
                    .map_or(0, |definition| {
                        definition.rules.printed_mana_cost().mana_value()
                    })
            })
            .collect::<Vec<_>>();
        let mut payments = Vec::new();
        for size in 1..=candidates.len() {
            for combination in Self::object_combinations(candidates, size) {
                let sum = combination
                    .iter()
                    .map(|id| {
                        candidates
                            .iter()
                            .position(|candidate| candidate == id)
                            .map_or(0, |index| values[index])
                    })
                    .fold(0_u16, u16::saturating_add);
                if sum < total {
                    continue;
                }
                // Minimal: with any one card taken out it no longer reaches.
                let minimal = combination.iter().all(|id| {
                    let value = candidates
                        .iter()
                        .position(|candidate| candidate == id)
                        .map_or(0, |index| values[index]);
                    sum.saturating_sub(value) < total
                });
                if minimal {
                    payments.push(combination);
                }
            }
        }
        payments
    }

    /// Every way to reach `types` distinct card types between the chosen
    /// cards, minimal in the same sense the mana-value search is: a set
    /// counts only if dropping any one of its cards would leave it short.
    /// One Artifact Creature Land pays for three of them at once, which is
    /// what makes the cost cheap in the deck that wants it.
    fn card_type_combinations(
        &self,
        candidates: &[GameObjectId],
        types: u16,
    ) -> Vec<Vec<GameObjectId>> {
        let sets = candidates
            .iter()
            .map(|id| {
                self.card_in_nonbattlefield_zone(*id)
                    .and_then(|(_, card)| self.catalog.get(card.definition))
                    .map_or_else(crate::card::CardTypeSet::empty, |definition| {
                        definition.rules.types()
                    })
            })
            .collect::<Vec<_>>();
        let union = |combination: &[GameObjectId]| {
            combination
                .iter()
                .filter_map(|id| candidates.iter().position(|candidate| candidate == id))
                .fold(crate::card::CardTypeSet::empty(), |seen, index| {
                    seen.union(sets[index])
                })
        };
        let mut payments = Vec::new();
        for size in 1..=candidates.len() {
            for combination in Self::object_combinations(candidates, size) {
                if union(&combination).count() < types {
                    continue;
                }
                // Minimal: with any one card taken out it no longer reaches.
                let minimal = combination.iter().all(|dropped| {
                    let without = combination
                        .iter()
                        .copied()
                        .filter(|id| id != dropped)
                        .collect::<Vec<_>>();
                    union(&without).count() < types
                });
                if minimal {
                    payments.push(combination);
                }
            }
        }
        payments
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
        from_graveyard: bool,
    ) -> bool {
        // A card that prints its own permission to use this alternative
        // where it lies. Nothing else about the cast changes, so it is a
        // second zone the same clause is offered from rather than a
        // different clause.
        if from_graveyard && context.source_zone == CastSourceZone::Graveyard {
            return true;
        }
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
                        | AlternativeCastKindDef::AlternativeCost
                        | AlternativeCastKindDef::Impending
                        // Dash is an ordinary cast from hand for a different
                        // price, exactly as impending is.
                        | AlternativeCastKindDef::Dash
                        // Offspring is the kicker shape: more mana as the
                        // creature is cast, and a trigger that asks whether
                        // it was paid.
                        | AlternativeCastKindDef::Offspring
                        // Bestow is an ordinary cast from hand for a
                        // different price and a different spell.
                        | AlternativeCastKindDef::Bestow
                        | AlternativeCastKindDef::Warp
                        // Emerge is an ordinary cast from hand whose price
                        // the sacrifice settles.
                        | AlternativeCastKindDef::Emerge
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
                        | AlternativeCastKindDef::Retrace
                        | AlternativeCastKindDef::WithoutPayingManaCost
                ),
            )
            // Foretell is the one alternative permission that casts from
            // exile. Adventure and one-shot free permissions use the base
            // configuration instead.
            | (
                CastSourceZone::Exile,
                Some(AlternativeCastKindDef::Foretell | AlternativeCastKindDef::Rebound),
            )
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

    /// How many times over a repeatable optional additional cost could be
    /// paid on this cast: what the player could pay for at all, divided by
    /// what one payment costs. A ceiling for the enumeration rather than an
    /// answer -- a configuration nobody can actually pay for is dropped
    /// where every unpayable cast is.
    fn repeatable_additional_cost_bound(
        &self,
        definition: &CardDefinition,
        card: GameObjectId,
        player: PlayerId,
        option: &PlayOptionDef,
    ) -> u16 {
        let Some(each) = option
            .additional_costs
            .iter()
            .filter(|cost| cost.repeatable)
            .map(|cost| cost.mana_cost.map_or(1, |mana| mana.mana_value().max(1)))
            .min()
        else {
            return 0;
        };
        let purpose = ManaPaymentPurpose::Spell {
            object: card,
            definition: definition.id,
            controller: player,
            form: option.form.clone(),
            reserved_life_payment: 0,
        };
        self.available_mana_ceiling(player, &purpose) / each
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
        let repeats = self.repeatable_additional_cost_bound(definition, card, player, option);
        let printed_cost_available =
            self.printed_cost_is_available_from(source_zone, card, player, option);
        if matches!(offer, None | Some(CastOfferCost::Any))
            && printed_cost_available
            && Self::visit_additional_cost_configurations(
                option,
                None,
                option.additional_costs.len(),
                repeats,
                &mut selected_additional,
                &mut visitor,
            )
            .is_break()
        {
            return ControlFlow::Break(());
        }
        if self
            .visit_printed_alternative_configurations(
                PrintedAlternativeRequest {
                    definition,
                    card,
                    player,
                    option,
                    context,
                },
                repeats,
                &mut selected_additional,
                &mut visitor,
            )
            .is_break()
        {
            return ControlFlow::Break(());
        }
        if matches!(offer, None | Some(CastOfferCost::Any))
            && printed_cost_available
            && option.mana_cost.is_some()
            && self
                .visit_battlefield_alternative_configurations(
                    player,
                    card,
                    option,
                    repeats,
                    &mut selected_additional,
                    &mut visitor,
                )
                .is_break()
        {
            return ControlFlow::Break(());
        }
        // A lent cast comes from wherever the clause that lent it put the
        // card: a graveyard for the ones that buy a spell back, exile for
        // rebound's own.
        if matches!(
            source_zone,
            CastSourceZone::Graveyard | CastSourceZone::Exile
        ) && let Some((_, _granted_alternative, _)) =
            self.granted_alternative_for_offer(card, option, offer)
            && let Some(granted) = Self::temporary_alternative_cost_id(option)
            && Self::visit_additional_cost_configurations(
                option,
                Some(granted),
                option.additional_costs.len(),
                repeats,
                &mut selected_additional,
                &mut visitor,
            )
            .is_break()
        {
            return ControlFlow::Break(());
        }

        ControlFlow::Continue(())
    }

    fn visit_printed_alternative_configurations(
        &self,
        request: PrintedAlternativeRequest<'_>,
        repeats: u16,
        selected_additional: &mut Vec<AdditionalCostId>,
        visitor: &mut impl FnMut(CostConfiguration) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let PrintedAlternativeRequest {
            definition,
            card,
            player,
            option,
            context,
        } = request;
        for cost in &option.alternative_costs {
            let (origin, kind) = match Self::alternative_cast_clause(definition, option, cost.id) {
                Some((origin, ability, kind)) if ability.is_executable() => {
                    (Some(origin), Some(kind))
                }
                Some(_) => continue,
                None => (None, None),
            };
            if match context.offer {
                Some(CastOfferCost::PrintedAlternative(required)) => origin != Some(required),
                Some(CastOfferCost::GrantedAlternative(_)) => true,
                None | Some(CastOfferCost::Any) => false,
            } {
                continue;
            }
            // A free cast gated on the board is not offered while its
            // condition is false. CR 118.4 also means life cannot be paid
            // down to zero, so an unaffordable life alternative is absent.
            let gated = match Self::alternative_cast_clause(definition, option, cost.id) {
                Some((origin, ability, _)) => match ability.definition {
                    DeclarativeAbilityDef::AlternativeCast(alternative) => {
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
            let from_graveyard = match Self::alternative_cast_clause(definition, option, cost.id) {
                Some((_, ability, _)) => match ability.definition {
                    DeclarativeAbilityDef::AlternativeCast(alternative) => {
                        alternative.from_graveyard
                    }
                    _ => false,
                },
                None => false,
            };
            if !gated
                && Self::alternative_is_castable_from(context, kind, origin, from_graveyard)
                && Self::visit_additional_cost_configurations(
                    option,
                    Some(cost.id),
                    option.additional_costs.len(),
                    repeats,
                    selected_additional,
                    visitor,
                )
                .is_break()
            {
                return ControlFlow::Break(());
            }
        }
        ControlFlow::Continue(())
    }

    fn visit_battlefield_alternative_configurations(
        &self,
        player: PlayerId,
        card: GameObjectId,
        option: &PlayOptionDef,
        repeats: u16,
        selected_additional: &mut Vec<AdditionalCostId>,
        visitor: &mut impl FnMut(CostConfiguration) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        for (index, _) in self
            .battlefield_spell_alternative_costs(player, card)
            .into_iter()
            .enumerate()
        {
            let Some(alternative) = Self::battlefield_alternative_cost_id(option, index) else {
                break;
            };
            if Self::visit_additional_cost_configurations(
                option,
                Some(alternative),
                option.additional_costs.len(),
                repeats,
                selected_additional,
                visitor,
            )
            .is_break()
            {
                return ControlFlow::Break(());
            }
        }
        ControlFlow::Continue(())
    }

    /// Every combination of optional additional costs, as a walk over the
    /// list: each is taken or left. A repeatable cost is taken any number of
    /// times up to `repeats`, and appears in the configuration once per
    /// payment -- which is how many times it was paid.
    pub(in crate::game) fn visit_additional_cost_configurations(
        option: &PlayOptionDef,
        alternative: Option<AlternativeCostId>,
        remaining: usize,
        repeats: u16,
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
            repeats,
            selected_reversed,
            visitor,
        )
        .is_break()
        {
            return ControlFlow::Break(());
        }
        let cost = &option.additional_costs[index];
        let most = if cost.repeatable { repeats.max(1) } else { 1 };
        for payments in 1..=most {
            selected_reversed.push(cost.id);
            let result = Self::visit_additional_cost_configurations(
                option,
                alternative,
                index,
                repeats,
                selected_reversed,
                visitor,
            );
            if result.is_break() {
                selected_reversed.truncate(selected_reversed.len() - usize::from(payments));
                return ControlFlow::Break(());
            }
        }
        selected_reversed.truncate(selected_reversed.len() - usize::from(most));
        ControlFlow::Continue(())
    }

    pub(in crate::game) fn configured_cast_mana_cost(
        &self,
        player: PlayerId,
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
        let battlefield_alternative = configuration.alternative().and_then(|selected| {
            self.battlefield_spell_alternative_cost_for_id(player, card, option, selected)
        });
        let mut cost = battlefield_alternative
            .or_else(|| granted_alternative.map(|(_, _, mana_cost)| mana_cost))
            .or_else(|| configured_base_mana_cost(option, configuration))?;
        // "Without paying its mana cost" and "rather than paying its mana
        // cost" replace the base or alternative cost, not optional
        // additional costs (CR 118.9d).
        if self.card_mana_cost_is_replaced(card) || self.library_top_cost_is_life(card, option) {
            cost = ManaCost {
                variable_x: cost.variable_x,
                x_multiplier: cost.x_multiplier,
                ..ManaCost::default()
            };
        }
        // "You may spend mana as though it were mana of any color to cast
        // that spell": what the payer owes stops being a colour and becomes
        // an amount.
        if self.card_mana_is_any_color(card) {
            cost = ManaCost {
                generic: cost
                    .generic
                    .saturating_add(cost.white + cost.blue + cost.black + cost.red + cost.green),
                white: 0,
                blue: 0,
                black: 0,
                red: 0,
                green: 0,
                ..cost
            };
        }
        for selected in configuration.additional() {
            let additional = option
                .additional_costs
                .iter()
                .find(|candidate| candidate.id == *selected)?;
            if let Some(mana) = additional.mana_cost {
                cost = add_mana_cost(cost, mana);
            }
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

    /// Whether a permission over this card lets its mana be spent as any
    /// colour.
    pub(in crate::game) fn card_mana_is_any_color(&self, card: GameObjectId) -> bool {
        self.exile_play_permissions
            .iter()
            .any(|permission| permission.card == card && permission.spend_any_color)
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
