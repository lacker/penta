//! Suspend (CR 702.62).
//!
//! Paying the suspend cost is a special action from hand. The two abilities
//! that work in exile are surfaced by the exile trigger-listener walk, so a
//! suspended card remains an ordinary targetable card object in that zone.

use crate::card::{
    CardEffectStatus, CharacteristicContext, DeclarativeAbilityDef, KeywordAbility, PlayActionKind,
    SuspendAbilityDef, SuspendTimeDef, ZoneKind,
};
use crate::{AbilityOrigin, Action, GameObjectId, PlayerId};

use super::{Game, ManaPaymentPurpose};

impl Game {
    pub(in crate::game) fn suspend_abilities_in_hand(
        &self,
        card: &super::CardInstance,
    ) -> Vec<(AbilityOrigin, SuspendAbilityDef)> {
        let mut abilities = Vec::new();
        self.for_each_printed_card_ability(card, &CharacteristicContext::Hand, |effective| {
            if let DeclarativeAbilityDef::Keyword(KeywordAbility::Suspend(
                definition @ SuspendAbilityDef::Hand { .. },
            )) = effective.ability.definition
            {
                abilities.push((effective.origin, *definition));
            }
        });
        abilities
    }

    /// Suspend's hand action is available only when the player could begin
    /// casting that card from hand. This deliberately does not ask whether
    /// its mana cost can be paid: the rule asks whether casting could begin,
    /// not whether it could finish (CR 702.62a).
    fn suspend_timing_allows(&self, card: &super::CardInstance, player: PlayerId) -> bool {
        let Some(definition) = self.catalog.get(card.definition) else {
            return false;
        };
        definition
            .play_options
            .iter()
            .filter(|option| option.action == PlayActionKind::CastSpell)
            .any(|option| {
                if self.play_is_prohibited(card, player, option)
                    || !self.play_timing_allows(player, option.restriction)
                {
                    return false;
                }
                let Some(types) = Self::play_option_types(definition, option) else {
                    return false;
                };
                if option.effect_status == CardEffectStatus::Unsupported {
                    return false;
                }
                self.spell_form_timing_allows(definition, card, player, option, types)
            })
    }

    pub(super) fn add_suspend_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        for card in &self.players[player.index()].hand {
            if !self.suspend_timing_allows(card, player) {
                continue;
            }
            for (index, (ability, suspend)) in
                self.suspend_abilities_in_hand(card).into_iter().enumerate()
            {
                let SuspendAbilityDef::Hand { time, costs } = suspend else {
                    continue;
                };
                match time {
                    SuspendTimeDef::Fixed(_) => {
                        if self.can_pay_special_action(
                            player,
                            card.id,
                            super::special_action_payments::PaidSpecialAction::Suspend {
                                ability: index,
                                x: 0,
                            },
                        ) {
                            actions.push(Action::Suspend {
                                card: card.id,
                                ability,
                                x: 0,
                            });
                        }
                    }
                    SuspendTimeDef::ChosenX { minimum } => {
                        let maximum = self.maximum_x_for(
                            player,
                            crate::card::costs::mana_cost(
                                costs,
                                self.catalog
                                    .get(card.definition)
                                    .and_then(|d| d.rules.mana_cost()),
                            )
                            .unwrap_or_default(),
                            &ManaPaymentPurpose::Other,
                        );
                        actions.extend((minimum..=maximum).filter_map(|x| {
                            self.can_pay_special_action(
                                player,
                                card.id,
                                super::special_action_payments::PaidSpecialAction::Suspend {
                                    ability: index,
                                    x,
                                },
                            )
                            .then_some(Action::Suspend {
                                card: card.id,
                                ability,
                                x,
                            })
                        }));
                    }
                }
            }
        }
    }

    pub(super) fn suspend(
        &mut self,
        player: PlayerId,
        card: GameObjectId,
        ability: AbilityOrigin,
        x: u16,
    ) {
        let Some(card_ref) = self.players[player.index()]
            .hand
            .iter()
            .find(|held| held.id == card)
        else {
            return;
        };
        let Some(index) = self
            .suspend_abilities_in_hand(card_ref)
            .iter()
            .position(|(origin, _)| *origin == ability)
        else {
            return;
        };
        self.begin_special_action_payment(
            player,
            card,
            super::special_action_payments::PaidSpecialAction::Suspend { ability: index, x },
        );
    }

    pub(in crate::game) fn finish_suspend(
        &mut self,
        player: PlayerId,
        card: GameObjectId,
        ability: usize,
        x: u16,
    ) {
        let Some(index) = self.players[player.index()]
            .hand
            .iter()
            .position(|held| held.id == card)
        else {
            return;
        };
        let Some((_, SuspendAbilityDef::Hand { time, .. })) = self
            .suspend_abilities_in_hand(&self.players[player.index()].hand[index])
            .get(ability)
            .copied()
        else {
            return;
        };
        let counters = match time {
            SuspendTimeDef::Fixed(counters) => counters,
            SuspendTimeDef::ChosenX { .. } => x,
        };
        let moved = self.players[player.index()].hand.remove(index);
        let owner = moved.owner;
        let (mut moved, _zone_change) = self.zone_change_card(moved);
        moved
            .counters
            .add(crate::CounterKind::named("time"), counters);
        self.players[owner.index()].exile.push(moved.clone());
        self.capture_cards_exiled(std::slice::from_ref(&moved), ZoneKind::Hand);
        self.consecutive_passes = 0;
    }

    pub(super) fn card_has_suspend(&self, card: &super::CardInstance) -> bool {
        self.object_has_ability(
            card.id,
            crate::card::AbilityPredicateDef::Is(crate::card::AbilityKindDef::Suspend),
        )
    }

    pub(super) fn is_suspended(&self, object: GameObjectId) -> bool {
        self.players.iter().any(|player| {
            player.exile.iter().any(|card| {
                card.id == object
                    && card.counters.count(crate::CounterKind::named("time")) > 0
                    && self.card_has_suspend(card)
            })
        })
    }
}
