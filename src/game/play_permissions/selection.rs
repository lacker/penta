//! Selecting and spending play permissions independently of their zone query.
use super::{
    AbilitySourceRef, CardInstance, CharacteristicContext, ControlFlow, DeclarativeAbilityDef,
    Game, GameObjectId, PlayActionKind, PlayCostDef, PlayOptionDef, PlayPermission, PlayerId,
};
use crate::{CostConfiguration, card::PlayPermissionDef};

impl Game {
    pub(in crate::game) fn zone_plot_ability(
        &self,
        card: &CardInstance,
        player: PlayerId,
    ) -> Option<crate::game::EffectiveAbility> {
        let mut result = None;
        let _ = self.visit_play_permissions(player, |source, permission| {
            if let PlayPermission::Plot { cards, ability } = permission
                && self.permission_names_card(cards, card.id, player, source.object)
            {
                let Some(permanent) = self.battlefield.iter().find(|p| p.card.id == source.object)
                else {
                    return ControlFlow::Continue(());
                };
                result = Some(crate::game::EffectiveAbility {
                    origin: Self::granted_ability_origin(
                        source.object,
                        source.ability,
                        Self::effective_rules_source(permanent),
                        crate::GrantId::PRIMARY,
                    ),
                    ability: *ability,
                });
                return ControlFlow::Break(());
            }
            ControlFlow::Continue(())
        });
        result
    }

    pub(in crate::game) fn play_land(
        &mut self,
        player: PlayerId,
        card: GameObjectId,
        option: crate::PlayOptionId,
    ) {
        let (sources, options) = self.land_permission_options(player, card, option);
        if sources.len() > 1 {
            self.queue_decision(
                player,
                "Choose a permission to play this land",
                crate::game::DecisionVisibility::Private,
                crate::game::DecisionPreference::Neutral,
                1..=1,
                false,
                options,
                crate::game::DecisionContinuation::PlayLandPermission {
                    player,
                    card,
                    option,
                    sources,
                },
            );
        } else {
            self.play_land_using_permission(player, card, option, sources.first().copied());
        }
    }

    pub(in crate::game) fn land_permission_options(
        &self,
        player: PlayerId,
        card: GameObjectId,
        option: crate::PlayOptionId,
    ) -> (Vec<GameObjectId>, Vec<crate::game::DecisionOption>) {
        let mut sources = Vec::new();
        if let Some((zone, top)) = self.card_in_nonbattlefield_zone(card)
            && zone != crate::card::ZoneKind::Hand
            && let Some(definition) = self
                .catalog
                .get(top.definition)
                .and_then(|card| card.play_option(option))
        {
            sources = self
                .play_permission_sources(card, player)
                .into_iter()
                .filter(|source| {
                    self.selected_play_permission(
                        top,
                        player,
                        definition,
                        0,
                        &CostConfiguration::default().with_permission_source(Some(*source)),
                    )
                    .is_some()
                })
                .collect();
        }
        let options = sources
            .iter()
            .enumerate()
            .map(|(index, source)| crate::game::DecisionOption {
                id: u32::try_from(index).expect("permission index"),
                label: self
                    .permanent_card_name(*source)
                    .map_or_else(|| "Use permission".to_owned(), |name| format!("Use {name}")),
                card: self.card_in_nonbattlefield_zone(card).map(|(_, top)| {
                    (
                        top.id,
                        crate::ObjectCharacteristics::card(
                            top.definition,
                            crate::CardPartId::PRIMARY,
                        ),
                    )
                }),
                members: Vec::new(),
                ability_text: None,
                zone: crate::game::DecisionZone::None,
            })
            .collect();
        (sources, options)
    }

    pub(in crate::game) fn play_permission_sources(
        &self,
        card: GameObjectId,
        player: PlayerId,
    ) -> Vec<GameObjectId> {
        let mut sources = Vec::new();
        let _ = self.visit_play_permissions(player, |source, permission| {
            if matches!(permission, PlayPermission::Cards(definition) if self.permission_names_card(definition.cards, card, player, source.object))
                && permission.is_open_now(self, player, source.object)
                && !sources.contains(&source.object)
            {
                sources.push(source.object);
            }
            ControlFlow::Continue(())
        });
        sources
    }

    pub(in crate::game) fn play_permission_cost(
        &self,
        player: PlayerId,
        selected: Option<GameObjectId>,
    ) -> Option<PlayCostDef> {
        let mut result = None;
        let _ = self.visit_play_permissions(player, |source, permission| {
            if let PlayPermission::Cards(definition) = permission
                && selected.is_none_or(|selected| selected == source.object)
                && permission.is_open_now(self, player, source.object)
            {
                result = Some(definition.cost);
                return ControlFlow::Break(());
            }
            ControlFlow::Continue(())
        });
        result
    }

    pub(in crate::game) fn selected_play_permission(
        &self,
        card: &CardInstance,
        player: PlayerId,
        option: &PlayOptionDef,
        x: u16,
        configuration: &CostConfiguration,
    ) -> Option<(AbilitySourceRef, PlayPermissionDef)> {
        let object = match option.action {
            PlayActionKind::CastSpell => {
                let alternative = configuration.alternative().and_then(|cost| {
                    Self::alternative_cast_clause(self.catalog.get(card.definition)?, option, cost)
                        .map(|(_, _, kind)| kind)
                });
                let spell =
                    self.proposed_spell_view(player, card.id, &option.form, alternative, x)?;
                self.spell_view_characteristics(spell)?
            }
            PlayActionKind::PlayLand => {
                let crate::SpellForm::Part(presented) = option.form else {
                    return None;
                };
                self.printed_trigger_event_object(
                    card.id,
                    card.definition,
                    player,
                    &CharacteristicContext::Battlefield { presented },
                )?
            }
        };
        let selected = configuration.permission_source();
        let mut result = None;
        let _ = self.visit_play_permissions(player, |source, permission| {
            if let PlayPermission::Cards(definition) = permission
                && self.permission_names_card(definition.cards, card.id, player, source.object)
                && selected.is_none_or(|selected| selected == source.object)
                && permission.is_open_now(self, player, source.object)
                && definition.restriction.action.matches(option.action)
                && self.trigger_object_matches(
                    definition.restriction.object,
                    &object,
                    source.object,
                    option.action == PlayActionKind::CastSpell,
                )
            {
                result = Some((source, definition));
                return ControlFlow::Break(());
            }
            ControlFlow::Continue(())
        });
        result
    }

    pub(in crate::game) fn play_life_for_configuration(
        &self,
        card: &CardInstance,
        player: PlayerId,
        option: &PlayOptionDef,
        configuration: &CostConfiguration,
    ) -> Option<u16> {
        configuration.permission_source()?;
        if self.play_permission_cost(player, configuration.permission_source())?
            != PlayCostDef::LifeEqualToManaValue
        {
            return None;
        }
        self.printed_trigger_event_object(
            card.id,
            card.definition,
            player,
            &CharacteristicContext::Stack {
                form: option.form.clone(),
            },
        )
        .map(|object| object.mana_value)
    }

    pub(in crate::game) fn spend_play_permission(
        &mut self,
        source: AbilitySourceRef,
        permission: PlayPermissionDef,
    ) {
        if permission.per_turn.is_some() {
            self.record_play_permission_use(source.object);
        }
        let Some(ability) = permission.benefit.and_then(|benefit| benefit.on_play) else {
            return;
        };
        let Some(permanent) = self.battlefield.iter().find(|p| p.card.id == source.object) else {
            return;
        };
        let DeclarativeAbilityDef::Triggered(definition) = ability.definition else {
            return;
        };
        self.capture_trigger(&crate::game::TriggerCapture {
            source,
            presentation: Self::ability_presentation(
                source.ability,
                Self::effective_rules_source(permanent),
            ),
            owner: permanent.card.owner,
            controller: permanent.controller,
            text: ability.text,
            target_defs: definition.targets.to_vec(),
            targets: Vec::new(),
            effect: ability
                .declarative_effect()
                .expect("permission trigger has a program"),
            resolver: Self::ability_resolver(source.ability, ability),
            context: crate::game::TriggerContext::empty().into(),
            condition: None,
            modes: None,
            x: 0,
        });
    }
}
