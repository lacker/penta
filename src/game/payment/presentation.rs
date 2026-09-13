//! Payment choices identify the operation, targets, payers, and individual units.
use super::super::{
    Action, DecisionOption, DecisionZone, Game, GameObjectId, Mana, PlayerId, Target,
};
use super::choices::payment_option;
use std::fmt::Write as _;

impl Game {
    pub(in crate::game) fn payment_contribution_option(
        &self,
        player: PlayerId,
        index: usize,
        contribution: super::contributions::BoundContribution,
    ) -> DecisionOption {
        let name = self.payment_object_label(player, contribution.source);
        let verb = match contribution.kind {
            super::super::ManaContributionKind::Convoke => "Convoke",
            super::super::ManaContributionKind::Delve => "Delve",
            super::super::ManaContributionKind::Improvise => "Improvise with",
        };
        self.payment_card_option(
            index,
            Some(contribution.source),
            format!("{verb} {name} to pay {}", contribution.symbol.cost()),
        )
    }

    fn payment_object_label(&self, player: PlayerId, object: GameObjectId) -> String {
        if self.battlefield.iter().any(|p| p.card.id == object) {
            self.target_label(player, Target::Permanent(object))
        } else if let Some(super::super::RetiredObject::Permanent { permanent, .. }) =
            self.retired_objects.get(&object)
        {
            self.effective_permanent_name(permanent)
                .map_or_else(|| "a former permanent".into(), std::borrow::Cow::into_owned)
        } else {
            self.target_label(player, Target::Card(object))
        }
    }

    fn payment_card_option(
        &self,
        index: usize,
        object: Option<GameObjectId>,
        label: String,
    ) -> DecisionOption {
        let mut option = payment_option(index, label);
        if let Some(object) = object {
            if let Some(permanent) = self.battlefield.iter().find(|p| p.card.id == object) {
                option.card = Some((object, Self::effective_rules_source(permanent)));
                option.zone = DecisionZone::Battlefield;
            } else if let Some((zone, card)) = self.card_in_nonbattlefield_zone(object) {
                option.card = Some((
                    object,
                    super::super::ObjectCharacteristics::card(
                        card.definition,
                        crate::CardPartId::PRIMARY,
                    ),
                ));
                option.zone = match zone {
                    crate::ZoneKind::Hand => DecisionZone::Hand,
                    crate::ZoneKind::Graveyard => DecisionZone::Graveyard,
                    crate::ZoneKind::Exile => DecisionZone::Exile,
                    crate::ZoneKind::Command => DecisionZone::Command,
                    crate::ZoneKind::Library => DecisionZone::Library,
                    _ => DecisionZone::None,
                };
            }
        }
        option
    }

    #[allow(clippy::too_many_lines)]
    pub(in crate::game) fn payment_operation_option(
        &self,
        player: PlayerId,
        index: usize,
        action: &Action,
    ) -> DecisionOption {
        let object = super::payment_action_object(action);
        let name = object.map_or_else(
            || "operation".into(),
            |id| self.payment_object_label(player, id),
        );
        let verb = if matches!(action, Action::CastSpell { .. }) {
            "Cast"
        } else {
            "Activate"
        };
        let cost = self
            .explicit_payment_obligation(player, action)
            .map(|o| o.cost)
            .unwrap_or_default();
        let mut option = self.payment_card_option(index, object, format!("{verb} {name}: {cost}"));
        let (targets, payers) = match action {
            Action::CastSpell {
                card,
                choices,
                sacrifices,
            } => {
                if let Some(cast) = self
                    .card_in_nonbattlefield_zone(*card)
                    .and_then(|(_, held)| self.catalog.get(held.definition))
                    .and_then(|definition| definition.play_option(choices.play_option()))
                {
                    if let Some(alternative) = choices.costs().alternative()
                        && let Some(cost) = cast
                            .alternative_costs
                            .iter()
                            .find(|cost| cost.id == alternative)
                    {
                        let _ = write!(option.label, "; via {}", cost.label);
                    }
                    for selected in choices.costs().additional() {
                        if let Some(cost) = cast
                            .additional_costs
                            .iter()
                            .find(|cost| cost.id == *selected)
                        {
                            let _ = write!(option.label, "; with {}", cost.label);
                        }
                    }
                    if let Some(modes) = &cast.modes {
                        for selected in choices.modes() {
                            if let Some(mode) = modes.modes.iter().find(|mode| mode.id == *selected)
                            {
                                let _ = write!(option.label, "; {}", mode.label);
                            }
                        }
                    }
                }
                if choices.x() > 0 {
                    let _ = write!(option.label, "; X = {}", choices.x());
                }
                let life = Self::mana_payment_life(choices.mana_payment()).unwrap_or(0);
                if life > 0 {
                    let _ = write!(option.label, "; pay {life} life");
                }
                (choices.targets(), sacrifices.as_slice())
            }
            Action::ActivateAbility {
                source,
                ability,
                targets,
                cost_objects,
                modes,
                ..
            } => {
                if let Some(definition) = self.ability_for_origin(*source, *ability) {
                    option.ability_text = Some(definition.text.into());
                    let _ = write!(option.label, "; {}", definition.text);
                    if let Some(modal) = definition.modal() {
                        for mode in modes {
                            if let Some(mode) = modal.modes.get(mode.index()) {
                                let _ = write!(option.label, "; {}", mode.rules_text());
                            }
                        }
                    }
                }
                (targets.as_slice(), cost_objects.as_slice())
            }
            Action::ActivateManaAbility {
                source,
                ability,
                color,
                cost_object,
                combination,
                ..
            } => {
                option.ability_text = self
                    .ability_for_origin(*source, *ability)
                    .map(|a| a.text.into());
                if let Some(text) = &option.ability_text {
                    let _ = write!(option.label, "; {text}");
                }
                let output = combination.map_or_else(
                    || format!("{color:?}"),
                    |split| {
                        split
                            .iter()
                            .map(|(color, amount)| format!("{amount} {color:?}"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    },
                );
                let _ = write!(option.label, "; add {output}");
                (&[][..], cost_object.as_slice())
            }
            _ => (&[][..], &[][..]),
        };
        for selection in targets {
            for target in selection.targets() {
                let _ = write!(
                    option.label,
                    "; target {}",
                    self.target_label(player, *target)
                );
            }
        }
        for payer in payers {
            let _ = write!(
                option.label,
                "; using {}",
                self.payment_object_label(player, *payer)
            );
        }
        option
    }

    pub(in crate::game) fn payment_unit_option(
        &self,
        player: PlayerId,
        index: usize,
        mana: Mana,
    ) -> DecisionOption {
        let source = mana.source.map(|source| source.object);
        let from = source.map_or_else(String::new, |id| {
            format!(" from {}", self.payment_object_label(player, id))
        });
        let restriction = if mana.restrictions.is_empty() {
            ""
        } else {
            " (restricted)"
        };
        let rider = if mana.spend_effects.is_empty() {
            ""
        } else {
            " (spend effect)"
        };
        let mut option = self.payment_card_option(
            index + 1,
            source,
            format!(
                "{:?} mana #{number}{from}{restriction}{rider}",
                mana.color,
                number = index + 1
            ),
        );
        option.ability_text = mana
            .source
            .and_then(|source| self.ability_for_origin(source.object, source.ability))
            .map(|a| a.text.into());
        option
    }
}
