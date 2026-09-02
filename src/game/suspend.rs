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
    fn suspend_abilities_in_hand(
        &self,
        card: &super::CardInstance,
    ) -> Vec<(AbilityOrigin, SuspendAbilityDef)> {
        let mut abilities = Vec::new();
        self.for_each_printed_card_ability(card, &CharacteristicContext::Hand, |effective| {
            if let DeclarativeAbilityDef::Keyword(KeywordAbility::Suspend(
                definition @ SuspendAbilityDef::Hand { .. },
            )) = effective.ability.definition
            {
                abilities.push((effective.origin, definition));
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
            for (ability, suspend) in self.suspend_abilities_in_hand(card) {
                let SuspendAbilityDef::Hand { time, cost } = suspend else {
                    continue;
                };
                match time {
                    SuspendTimeDef::Fixed(_) => {
                        if self.can_pay_cost_for(player, *cost, 0, &ManaPaymentPurpose::Other) {
                            actions.push(Action::Suspend {
                                card: card.id,
                                ability,
                                x: 0,
                            });
                        }
                    }
                    SuspendTimeDef::ChosenX { minimum } => {
                        let maximum = self.maximum_x_for(player, *cost, &ManaPaymentPurpose::Other);
                        actions.extend((minimum..=maximum).filter_map(|x| {
                            self.can_pay_cost_for(player, *cost, x, &ManaPaymentPurpose::Other)
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
        let Some(index) = self.players[player.index()]
            .hand
            .iter()
            .position(|candidate| candidate.id == card)
        else {
            return;
        };
        let moved = self.players[player.index()].hand[index].clone();
        let Some((_, suspend)) = self
            .suspend_abilities_in_hand(&moved)
            .into_iter()
            .find(|(origin, _)| *origin == ability)
        else {
            return;
        };
        let SuspendAbilityDef::Hand { time, cost } = suspend else {
            return;
        };
        let counters = match time {
            SuspendTimeDef::Fixed(counters) if x == 0 => counters,
            SuspendTimeDef::ChosenX { minimum } if x >= minimum => x,
            _ => return,
        };
        self.activate_mana_for_cost(player, *cost, x);
        let _spent = self.pay_player_cost(player, *cost, x);
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
        self.object_has_ability(card.id, crate::card::AbilityPredicateDef::Suspend)
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
