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
use crate::ModeId;
use crate::card::CostDef;
use crate::game::ManaPaymentPurpose;

include!("cost_configurations/object_combinations.rs");
include!("cost_configurations/additional_cost_payments.rs");

/// The chosen quantities a cost can be counted from: the X the spell is cast
/// for, how many modes it was cast with, and how many targets it names.
#[derive(Clone, Copy)]
pub(in crate::game) struct CastScale {
    pub(in crate::game) x: u16,
    pub(in crate::game) modes: usize,
    pub(in crate::game) targets: usize,
    pub(in crate::game) offer: Option<CastOfferCost>,
}

impl CastScale {
    fn quantity(self, quantity: crate::card::CostQuantityDef) -> Option<u16> {
        match quantity {
            crate::card::CostQuantityDef::Fixed(amount) => Some(u16::from(amount)),
            crate::card::CostQuantityDef::ChosenX => Some(self.x),
            crate::card::CostQuantityDef::ModeCount => {
                Some(u16::try_from(self.modes).unwrap_or(u16::MAX))
            }
            crate::card::CostQuantityDef::TargetCount => {
                Some(u16::try_from(self.targets).unwrap_or(u16::MAX))
            }
            crate::card::CostQuantityDef::Subtract(left, right) => {
                Some(self.quantity(*left)?.saturating_sub(self.quantity(*right)?))
            }
            crate::card::CostQuantityDef::ObjectSetValueAtLeast(_) => None,
        }
    }
}

#[derive(Clone, Copy)]
pub(in crate::game) struct SpellAdditionalCostRequest<'a> {
    pub(in crate::game) definition: &'a CardDefinition,
    pub(in crate::game) option: &'a PlayOptionDef,
    pub(in crate::game) costs: &'a CostConfiguration,
    pub(in crate::game) card: &'a CardInstance,
    pub(in crate::game) player: PlayerId,
    pub(in crate::game) modes: &'a [ModeId],
    pub(in crate::game) scale: CastScale,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(in crate::game) struct SpellAdditionalCostPayment {
    pub(in crate::game) objects: Vec<(GameObjectId, CostDef)>,
    pub(in crate::game) mana: ManaCost,
    pub(in crate::game) life: u16,
}

impl SpellAdditionalCostPayment {
    fn free() -> Self {
        Self {
            objects: Vec::new(),
            mana: ManaCost::default(),
            life: 0,
        }
    }

    fn combine(&self, other: &Self) -> Option<Self> {
        if other
            .objects
            .iter()
            .any(|(object, _)| self.objects.iter().any(|(paid, _)| paid == object))
        {
            return None;
        }
        let mut objects = self.objects.clone();
        objects.extend(other.objects.iter().copied());
        Some(Self {
            objects,
            mana: add_mana_cost(self.mana, other.mana),
            life: self.life.saturating_add(other.life),
        })
    }

    pub(in crate::game) fn object_ids(&self) -> Vec<GameObjectId> {
        self.objects.iter().map(|(object, _)| *object).collect()
    }
}

#[derive(Clone, Copy)]
struct PrintedAlternativeRequest<'a> {
    definition: &'a CardDefinition,
    card: GameObjectId,
    player: PlayerId,
    option: &'a PlayOptionDef,
    context: CastCostContext,
}

#[derive(Clone, Copy)]
struct SelectedSpellAdditionalCost {
    cost: CostDef,
    repetitions: u16,
}

impl SelectedSpellAdditionalCost {
    const fn once(cost: CostDef) -> Self {
        Self {
            cost,
            repetitions: 1,
        }
    }
}

impl Game {
    fn selected_spell_additional_costs(
        &self,
        definition: &CardDefinition,
        option: &PlayOptionDef,
        costs: &CostConfiguration,
        card: &CardInstance,
        selected_modes: &[ModeId],
        offer: Option<CastOfferCost>,
    ) -> Vec<SelectedSpellAdditionalCost> {
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
            required.push(SelectedSpellAdditionalCost::once(cost));
        }
        // An alternative replaces only the spell's mana cost. Every mandatory
        // additional cost printed by the spell still applies (CR 118.9d).
        if let Some(cost) = definition
            .rules
            .ability_clauses()
            .iter()
            .find_map(|ability| match ability.definition {
                DeclarativeAbilityDef::Spell(spell) => match spell {
                    crate::card::SpellAbilityDef::Nonmodal {
                        additional_cost: Some(cost),
                        ..
                    } => Some(SelectedSpellAdditionalCost::once(cost)),
                    crate::card::SpellAbilityDef::Modal(modal) => {
                        modal.escalate_cost.map(|cost| SelectedSpellAdditionalCost {
                            cost,
                            repetitions: u16::try_from(selected_modes.len().saturating_sub(1))
                                .unwrap_or(u16::MAX),
                        })
                    }
                    crate::card::SpellAbilityDef::Nonmodal {
                        additional_cost: None,
                        ..
                    } => None,
                },
                _ => None,
            })
        {
            required.push(cost);
        }
        if let Some((_, ability)) = Self::spell_ability(definition, option)
            && let DeclarativeAbilityDef::Spell(spell) = ability.definition
            && let Some(modal) = spell.modal()
        {
            required.extend(selected_modes.iter().filter_map(|mode| {
                modal
                    .mode_additional_mana_cost(*mode)
                    .map(CostDef::pay_mana)
                    .map(SelectedSpellAdditionalCost::once)
            }));
        }
        for selected in costs.additional() {
            if let Some((_, ability, _)) =
                Self::optional_additional_cost_clause(definition, option, *selected)
                && let DeclarativeAbilityDef::OptionalAdditionalCost(optional) = ability.definition
                && let Some(cost) = optional.additional_cost
            {
                required.push(SelectedSpellAdditionalCost::once(cost));
            }
        }
        required
    }

    /// The largest X that the selected semantic additional costs can pay.
    pub(in crate::game) fn maximum_x_for_spell_additional_costs(
        &self,
        request: SpellAdditionalCostRequest<'_>,
    ) -> Option<u16> {
        self.selected_spell_additional_costs(
            request.definition,
            request.option,
            request.costs,
            request.card,
            request.modes,
            request.scale.offer,
        )
        .into_iter()
        .filter_map(|selected| {
            self.maximum_x_for_spell_additional_cost(selected.cost, request.card, request.player)
        })
        .min()
    }

    fn maximum_x_for_spell_additional_cost(
        &self,
        cost: CostDef,
        card: &CardInstance,
        player: PlayerId,
    ) -> Option<u16> {
        match cost {
            CostDef::PayLifeTimes(crate::card::CostQuantityDef::ChosenX) => {
                Some(self.maximum_x_for_life(player))
            }
            CostDef::Sacrifice {
                quantity: crate::card::CostQuantityDef::ChosenX,
                ..
            }
            | CostDef::Discard {
                quantity: crate::card::CostQuantityDef::ChosenX,
                ..
            }
            | CostDef::Exile {
                quantity: crate::card::CostQuantityDef::ChosenX,
                ..
            }
            | CostDef::ReturnToHand {
                quantity: crate::card::CostQuantityDef::ChosenX,
                ..
            }
            | CostDef::Tap {
                quantity: crate::card::CostQuantityDef::ChosenX,
                ..
            } => Some(
                u16::try_from(self.additional_cost_candidates(cost, card, player).len())
                    .unwrap_or(u16::MAX),
            ),
            CostDef::All(costs) => costs
                .iter()
                .filter_map(|cost| self.maximum_x_for_spell_additional_cost(*cost, card, player))
                .min(),
            CostDef::Choice(costs) => costs
                .iter()
                .filter_map(|cost| self.maximum_x_for_spell_additional_cost(*cost, card, player))
                .max(),
            _ => None,
        }
    }

    /// Every semantic way to pay the selected spell costs. Named object
    /// actions stay attached to their objects through execution; scalar mana
    /// and life payments travel beside them.
    pub(in crate::game) fn spell_additional_cost_payments(
        &self,
        request: SpellAdditionalCostRequest<'_>,
    ) -> Vec<SpellAdditionalCostPayment> {
        let required = self.selected_spell_additional_costs(
            request.definition,
            request.option,
            request.costs,
            request.card,
            request.modes,
            request.scale.offer,
        );
        if required.is_empty() {
            return vec![SpellAdditionalCostPayment::free()];
        }

        let mut combined = vec![SpellAdditionalCostPayment::free()];
        for selected in required {
            let ways = self.repeated_spell_additional_cost_payment_options(
                selected.cost,
                selected.repetitions,
                request.card,
                request.player,
                request.scale,
            );
            let mut next = Vec::new();
            // Deduplicated through a set rather than a scan of `next`. The
            // combinations multiply out, and each `contains` compared whole
            // payments -- object predicates and all -- against every one kept
            // so far, which on a control board was most of a legal-action
            // sweep. The vector still decides the order the actions are
            // offered in, which is part of the wire contract.
            let mut seen = std::collections::HashSet::new();
            for paid in &combined {
                for way in &ways {
                    if let Some(payment) = paid.combine(way)
                        && i64::from(payment.life)
                            <= i64::from(self.players[request.player.index()].life)
                        && seen.insert(payment.clone())
                    {
                        next.push(payment);
                    }
                }
            }
            combined = next;
        }
        combined
    }

    pub(in crate::game) fn spell_additional_cost_payment_for_objects(
        &self,
        request: SpellAdditionalCostRequest<'_>,
        objects: &[GameObjectId],
    ) -> Option<SpellAdditionalCostPayment> {
        self.spell_additional_cost_payments(request)
            .into_iter()
            .find(|payment| payment.object_ids() == objects)
    }

    fn spell_object_additional_cost_payments(
        &self,
        cost: CostDef,
        quantity: crate::card::CostQuantityDef,
        card: &CardInstance,
        player: PlayerId,
        scale: CastScale,
    ) -> Vec<SpellAdditionalCostPayment> {
        let candidates = self.additional_cost_candidates(cost, card, player);
        // One configuration per way of paying, so a cost naming more than one
        // object enumerates combinations rather than candidates. Order does
        // not matter -- exiling A then B is the same payment as B then A --
        // so each combination appears once, in candidate order.
        if let crate::card::CostQuantityDef::ObjectSetValueAtLeast(requirement) = quantity {
            return self
                .object_set_value_combinations(&candidates, *requirement)
                .into_iter()
                .map(|objects| SpellAdditionalCostPayment {
                    objects: objects.into_iter().map(|object| (object, cost)).collect(),
                    mana: ManaCost::default(),
                    life: 0,
                })
                .collect();
        }
        let required = usize::from(
            scale
                .quantity(quantity)
                .expect("object thresholds are handled before scalar quantities"),
        );
        self.spell_object_additional_cost_payments_for_count(cost, required, card, player)
    }

    fn spell_object_additional_cost_payments_for_count(
        &self,
        cost: CostDef,
        required: usize,
        card: &CardInstance,
        player: PlayerId,
    ) -> Vec<SpellAdditionalCostPayment> {
        let candidates = self.additional_cost_candidates(cost, card, player);
        Self::object_combinations(&candidates, required)
            .into_iter()
            .map(|objects| SpellAdditionalCostPayment {
                objects: objects.into_iter().map(|object| (object, cost)).collect(),
                mana: ManaCost::default(),
                life: 0,
            })
            .collect()
    }

    fn additional_cost_candidates(
        &self,
        cost: CostDef,
        card: &CardInstance,
        player: PlayerId,
    ) -> Vec<GameObjectId> {
        let (object, from) = match cost {
            CostDef::Sacrifice { object, .. } | CostDef::ReturnToHand { object, .. } => {
                (object, ZoneKind::Battlefield)
            }
            CostDef::Tap { object, .. } => (object, ZoneKind::Battlefield),
            CostDef::Discard { object, .. } => (object, ZoneKind::Hand),
            CostDef::Exile { object, from, .. } => (object, from),
            _ => return Vec::new(),
        };
        match from {
            ZoneKind::Battlefield => self
                .battlefield
                .iter()
                .filter(|permanent| {
                    permanent.controller == player
                        && (!matches!(cost, CostDef::Tap { .. }) || !permanent.tapped)
                        && self.trigger_object_matches(
                            object,
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
                        && self.card_object_matches(object, held, ZoneKind::Graveyard, held.id)
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
                        && self.card_object_matches(object, held, ZoneKind::Hand, held.id)
                })
                .map(|held| held.id)
                .collect(),
            _ => Vec::new(),
        }
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
                Some((origin, _ability, kind)) => (Some(origin), Some(kind)),
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
