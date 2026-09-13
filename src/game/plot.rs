//! Plot (CR 702.170a).
//!
//! The hand action and effects both mark an ordinary exile object as plotted.
//! That designation, independent of its printed abilities, supplies CR 702.170d's
//! cast permission. Suspend instead derives its status from exile, time counters,
//! and the suspend ability; rebound retains an object-linked delayed trigger.

use crate::ids::GameObjectId;

use super::{
    Action, AlternativeCastKindDef, CardInstance, CharacteristicContext, DeclarativeAbilityDef,
    EffectResolutionContext, EffectiveAbility, Game, ObjectCharacteristics, PlayerId, ScopedEffect,
    StackAbilityPayload, StackAbilityResolver, StackObject, StackObjectKind, TriggerContext,
};

impl Game {
    /// The hand clause supplies both the special-action cost and its program.
    pub(in crate::game) fn card_plot_ability(
        &self,
        card: &CardInstance,
    ) -> Option<EffectiveAbility> {
        self.find_printed_card_ability(card, &CharacteristicContext::Hand, |effective| {
            matches!(effective.ability.definition,
                DeclarativeAbilityDef::AlternativeCast(alternative)
                    if alternative.kind == AlternativeCastKindDef::Plot)
        })
    }

    /// "Plot only as a sorcery": your own main phase with the stack empty,
    /// whatever the plotted card's own type would allow.
    pub(super) fn add_plot_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        if player != self.active_player || !self.step.is_main() || !self.stack.is_empty() {
            return;
        }
        actions.extend(
            self.players[player.index()]
                .hand
                .iter()
                .filter(|card| {
                    self.can_pay_special_action(
                        player,
                        card.id,
                        super::special_action_payments::PaidSpecialAction::Plot,
                    )
                })
                .map(|card| Action::Plot { card: card.id }),
        );
    }

    pub(super) fn plot(&mut self, player: PlayerId, card: GameObjectId) {
        self.begin_special_action_payment(
            player,
            card,
            super::special_action_payments::PaidSpecialAction::Plot,
        );
    }

    pub(in crate::game) fn finish_plot(&mut self, player: PlayerId, card: GameObjectId) {
        let Some(source_card) = self.players[player.index()]
            .hand
            .iter()
            .find(|candidate| candidate.id == card)
            .cloned()
        else {
            return;
        };
        let Some(effective) = self.card_plot_ability(&source_card) else {
            return;
        };
        let Some(effect) = effective.ability.declarative_effect() else {
            return;
        };
        let scoped = ScopedEffect::primary(effect);
        let context = EffectResolutionContext::new(TriggerContext::empty());
        let presentation = Self::ability_presentation(
            effective.origin,
            ObjectCharacteristics::card(source_card.definition, crate::CardPartId::PRIMARY),
        );
        let resolution = self.unbacked_ability_object(presentation, player);
        // This is an interpreter frame, never an object placed on the stack.
        // Retain the authored clause and origin for any suspended continuation.
        let object = StackObject {
            id: resolution.id,
            kind: StackObjectKind::TriggeredAbility,
            card: resolution,
            source: Some(card),
            ability: Some(StackAbilityPayload {
                origin: effective.origin,
                definition: Some(Box::new(effective.ability)),
                presentation,
                text: Some(effective.ability.text),
                target_defs: Vec::new(),
                targets: Vec::new(),
                context: context.clone(),
                resolver: StackAbilityResolver::Declarative(scoped),
                condition: None,
                mode_effects: Vec::new(),
                resolution_destination: None,
                x: 0,
                sacrificed_mana_value: 0,
            }),
            controller: player,
            signature: None,
            chosen_permanents: Vec::new(),
            applied_effects: Vec::new(),
            text_changes: Vec::new(),
            colors: None,
            cast: None,
            face_down: None,
            is_copy: false,
        };
        self.resolve_effect_def(scoped, &object, context);
    }
}

impl Game {
    /// Becoming plotted is not performing the plot special action (CR 702.170e).
    pub(super) fn make_plotted(&mut self, card: GameObjectId) {
        if self
            .players
            .iter()
            .any(|state| state.exile.iter().any(|exiled| exiled.id == card))
        {
            self.plotted_cards.insert(
                card,
                (
                    self.active_player,
                    self.turns_started[self.active_player.index()],
                ),
            );
        }
    }

    pub(super) fn plotted_cast_permission(
        &self,
        card: GameObjectId,
        player: PlayerId,
    ) -> Option<super::ExilePlayPermission> {
        let &(active, turn) = self.plotted_cards.get(&card)?;
        if !self.sorcery_speed_window(player)
            || (self.active_player == active && self.turns_started[active.index()] == turn)
            || !self.players[player.index()]
                .exile
                .iter()
                .any(|exiled| exiled.id == card && exiled.owner == player)
        {
            return None;
        }
        Some(super::ExilePlayPermission {
            card,
            player,
            cost: super::ExilePlayCost::Free,
            until_end_of_turn: None,
            adventure_return_only: false,
            surcharge: crate::card::ManaCost::default(),
            not_before_turn: None,
            face_down: false,
            lands_may_be_played: false,
            hidden_from_owner: false,
            spend_any_color: false,
            condition: None,
            hidden_only: false,
            until_holder_end_step: None,
            zone: crate::card::ZoneKind::Exile,
            group: None,
            grants_haste: false,
        })
    }
}
