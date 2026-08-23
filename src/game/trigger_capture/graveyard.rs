//! Triggers that listen from a graveyard rather than from the battlefield.
//!
//! A permanent's abilities are read through an effective-rules walk, because
//! something on the battlefield may be changing them. Nothing modifies a card
//! lying in a graveyard, so its own printed clause is the whole of what
//! listens, and it is read directly.

use super::{
    AbilityProcedureDef, AbilitySourceRef, BattlefieldTriggerListener, CardPartId,
    CharacteristicContext, DeclarativeAbilityDef, EffectDef, Game, ObjectCharacteristics, PlayerId,
    TriggerCapture, TriggerContext, ZoneKind,
};
use crate::game::CardInstance;

impl Game {
    pub(in crate::game) fn extend_with_card_graveyard_trigger_listeners(
        &self,
        listeners: &mut Vec<BattlefieldTriggerListener>,
        card: &CardInstance,
    ) {
        self.for_each_printed_card_ability(card, &CharacteristicContext::Graveyard, |effective| {
            let ability = effective.ability;
            let DeclarativeAbilityDef::Triggered(definition) = ability.definition else {
                return;
            };
            if !ability.is_executable()
                || definition.procedure != AbilityProcedureDef::Shared
                || !definition.source_zones.contains(&ZoneKind::Graveyard)
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
                        ObjectCharacteristics::card(card.definition, CardPartId::PRIMARY),
                    ),
                    owner: card.owner,
                    // A card outside the battlefield has no controller. Its
                    // owner controls its triggered ability (CR 113.8).
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
        });
    }

    pub(super) fn extend_with_graveyard_trigger_listeners(
        &self,
        listeners: &mut Vec<BattlefieldTriggerListener>,
    ) {
        // A card in a graveyard can carry a trigger too. It is read from the
        // printed card rather than from an effective-rules walk: nothing on
        // the battlefield is modifying a card lying in a graveyard, and its
        // own printed clause is the whole of what listens.
        for player in [PlayerId::One, PlayerId::Two] {
            for card in &self.players[player.index()].graveyard {
                self.extend_with_card_graveyard_trigger_listeners(listeners, card);
            }
        }
    }
}
