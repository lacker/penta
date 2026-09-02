//! Printed and rule-created abilities that trigger from exile.

use super::{
    AbilityProcedureDef, AbilitySourceRef, BattlefieldTriggerListener, CardPartId,
    CharacteristicContext, DeclarativeAbilityDef, EffectDef, Game, ObjectCharacteristics, PlayerId,
    TriggerCapture, TriggerContext, ZoneKind,
};
use crate::card::{AbilityDef, CounterKind, abilities};
use crate::{AbilityOrigin, KeywordAbility};

const TIME: CounterKind = CounterKind::named("time");

impl Game {
    fn push_suspend_listener(
        listeners: &mut Vec<BattlefieldTriggerListener>,
        card: &crate::game::CardInstance,
        origin: AbilityOrigin,
        presentation: ObjectCharacteristics,
        ability: &'static AbilityDef,
    ) {
        let DeclarativeAbilityDef::Triggered(definition) = ability.definition else {
            unreachable!("Suspend's generated exile ability is triggered")
        };
        let effect = ability.declarative_effect().unwrap_or(EffectDef::None);
        listeners.push(BattlefieldTriggerListener {
            event: definition.event,
            uses_stack: true,
            trigger_limit: definition.trigger_limit,
            installed: None,
            capture: TriggerCapture {
                source: AbilitySourceRef {
                    object: card.id,
                    ability: origin,
                },
                presentation,
                owner: card.owner,
                controller: card.owner,
                text: ability.text,
                target_defs: definition.targets.to_vec(),
                targets: Vec::new(),
                effect,
                resolver: Self::ability_resolver(origin, ability),
                context: TriggerContext::empty().into(),
                condition: definition.condition,
                modes: definition.modes,
                x: 0,
            },
        });
    }

    pub(super) fn extend_with_exile_trigger_listeners(
        &self,
        listeners: &mut Vec<BattlefieldTriggerListener>,
    ) {
        for player in [PlayerId::One, PlayerId::Two] {
            for card in &self.players[player.index()].exile {
                self.for_each_printed_card_ability(
                    card,
                    &CharacteristicContext::Exile,
                    |effective| {
                        let ability = effective.ability;
                        let DeclarativeAbilityDef::Triggered(definition) = ability.definition
                        else {
                            return;
                        };
                        if definition.procedure != AbilityProcedureDef::Shared
                            || !definition.source_zones.contains(&ZoneKind::Exile)
                        {
                            return;
                        }
                        listeners.push(BattlefieldTriggerListener {
                            event: definition.event,
                            uses_stack: true,
                            trigger_limit: definition.trigger_limit,
                            installed: None,
                            capture: TriggerCapture {
                                source: AbilitySourceRef {
                                    object: card.id,
                                    ability: effective.origin,
                                },
                                presentation: Self::ability_presentation(
                                    effective.origin,
                                    ObjectCharacteristics::card(
                                        card.definition,
                                        CardPartId::PRIMARY,
                                    ),
                                ),
                                owner: card.owner,
                                controller: card.owner,
                                text: ability.text,
                                target_defs: definition.targets.to_vec(),
                                targets: Vec::new(),
                                effect: ability.declarative_effect().unwrap_or(EffectDef::None),
                                resolver: Self::ability_resolver(effective.origin, &ability),
                                context: TriggerContext::empty().into(),
                                condition: definition.condition,
                                modes: definition.modes,
                                x: 0,
                            },
                        });
                    },
                );

                if !self.card_has_suspend(card) {
                    continue;
                }
                let origin = self
                    .find_printed_card_ability(card, &CharacteristicContext::Exile, |effective| {
                        matches!(
                            effective.ability.definition,
                            DeclarativeAbilityDef::Keyword(KeywordAbility::Suspend(_))
                        )
                    })
                    .map(|effective| effective.origin)
                    .or_else(|| {
                        self.nonbattlefield_ability_grants
                            .iter()
                            .find(|grant| {
                                grant.object == card.id
                                    && crate::card::AbilityPredicateDef::Suspend
                                        .matches(&grant.ability)
                            })
                            .and_then(|grant| grant.source)
                    })
                    .unwrap_or(AbilityOrigin::IntrinsicCounter(TIME));
                let presentation = Self::ability_presentation(
                    origin,
                    ObjectCharacteristics::card(card.definition, CardPartId::PRIMARY),
                );
                if self.is_suspended(card.id) {
                    Self::push_suspend_listener(
                        listeners,
                        card,
                        origin,
                        presentation,
                        &abilities::SUSPEND_UPKEEP_ABILITY,
                    );
                }
                Self::push_suspend_listener(
                    listeners,
                    card,
                    origin,
                    presentation,
                    &abilities::SUSPEND_LAST_COUNTER_ABILITY,
                );
            }
        }
    }
}
