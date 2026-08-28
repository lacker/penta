//! Paying and stacking activated abilities whose source is an exiled card.

use super::{
    AbilityOrigin, AbilityProcedureDef, CharacteristicContext, DeclarativeAbilityDef,
    FrozenActivatedAbility, Game, GameObjectId, ObjectCharacteristics, PlayerId, Target,
    TargetSelection, ZoneKind, remove_card,
};
use crate::ModeId;
use crate::card::MoveToZoneCostDef;

impl Game {
    pub(super) fn pay_nonbattlefield_move_cost(
        &mut self,
        player: PlayerId,
        movement: MoveToZoneCostDef,
        cost_objects: &[GameObjectId],
    ) {
        debug_assert_eq!(movement.to, ZoneKind::Exile);
        let mut moved = Vec::new();
        for chosen in cost_objects {
            let card = match movement.from {
                ZoneKind::Hand => remove_card(&mut self.players[player.index()].hand, *chosen),
                ZoneKind::Graveyard => {
                    remove_card(&mut self.players[player.index()].graveyard, *chosen)
                }
                _ => None,
            };
            if let Some(card) = card {
                let (card, _zone_change) = self.zone_change_card(card);
                self.players[player.index()].exile.push(card.clone());
                moved.push(card);
            }
        }
        if moved.is_empty() {
            return;
        }
        self.capture_cards_exiled(&moved, movement.from);
        if movement.from == ZoneKind::Graveyard {
            self.note_card_left_graveyard(player);
        }
    }

    /// Activates one supported printed ability from exile. Returns whether
    /// the source was found there, so the ordinary battlefield and graveyard
    /// activation paths know whether to continue looking.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn activate_exile_ability(
        &mut self,
        player: PlayerId,
        source: GameObjectId,
        ability: AbilityOrigin,
        targets: Vec<TargetSelection>,
        cost_objects: &[GameObjectId],
        x: u16,
        modes: &[ModeId],
    ) -> bool {
        let Some(source_card) = self.players[player.index()]
            .exile
            .iter()
            .find(|card| card.id == source)
            .cloned()
        else {
            return false;
        };
        let Some(effective) = self.find_printed_card_ability(
            &source_card,
            &CharacteristicContext::Exile,
            |effective| effective.origin == ability,
        ) else {
            return true;
        };
        let DeclarativeAbilityDef::Activated(definition) = effective.ability.definition else {
            return true;
        };
        if !effective.ability.is_executable()
            || definition.procedure != AbilityProcedureDef::Shared
            || !definition.source_zones.contains(&ZoneKind::Exile)
        {
            return true;
        }
        let Some(plan) = Self::selected_activated_plan(&definition, modes) else {
            return true;
        };
        let frozen = FrozenActivatedAbility {
            origin: effective.origin,
            definition: Some(Box::new(effective.ability)),
            presentation: Self::ability_presentation(
                effective.origin,
                ObjectCharacteristics::card(source_card.definition, crate::CardPartId::PRIMARY),
            ),
            text: Some(effective.ability.text),
            target_defs: plan.target_defs,
            resolver: Self::ability_resolver(effective.origin, &effective.ability),
            mode_effects: plan.mode_effects,
            x,
            sacrificed_mana_value: 0,
        };
        self.sacrifice_permanents(cost_objects);
        let chosen_permanents = targets
            .iter()
            .flat_map(TargetSelection::targets)
            .filter_map(|target| match target {
                Target::Permanent(id) => Some(*id),
                Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
            })
            .chain(cost_objects.iter().copied())
            .collect();
        self.push_activated_ability(
            source,
            &source_card.into(),
            player,
            frozen,
            targets,
            chosen_permanents,
        );
        self.consecutive_passes = 0;
        self.check_state_based_actions();
        true
    }
}
