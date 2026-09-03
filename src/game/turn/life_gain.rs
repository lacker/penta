//! Life changes and the replacement effects that modify life-gain amounts.

use super::super::{
    AbilitySourceRef, ApplicableReplacement, AppliedRuleDef, CommittedTriggerEvent,
    DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone,
    DeclarativeAbilityDef, Game, PlayerId, ReplacementEffectContext, ReplacementEffectDef,
    ReplacementEventDef, TriggerContext,
};

impl Game {
    /// Whether this player has been told they cannot gain life for the rest
    /// of the game (Screaming Nemesis). A prohibition rather than a
    /// replacement: the life never arrives, so nothing watching for a gain
    /// sees one.
    pub(in crate::game) fn cannot_gain_life(&self, player: PlayerId) -> bool {
        self.cannot_gain_life[player.index()]
            || self.player_rule_applies(player, AppliedRuleDef::CannotGainLife)
    }

    pub(in crate::game) fn life_total_cannot_change(&self, player: PlayerId) -> bool {
        self.player_rule_applies(
            player,
            AppliedRuleDef::PlayerRule(crate::card::PlayerRuleDef::LifeTotalCannotChange),
        )
    }

    /// CR 118.4: a nonzero life payment is impossible while the player's
    /// life total cannot change, even when they have enough life numerically.
    pub(in crate::game) fn can_pay_life(&self, player: PlayerId, amount: u16) -> bool {
        amount == 0
            || (!self.life_total_cannot_change(player)
                && i16::try_from(amount)
                    .is_ok_and(|amount| self.players[player.index()].life >= amount))
    }

    pub(in crate::game) fn gain_life(&mut self, player: PlayerId, amount: u16) {
        if amount == 0 || self.cannot_gain_life(player) || self.life_total_cannot_change(player) {
            return;
        }
        self.continue_life_gain(player, amount, Vec::new());
    }

    fn commit_life_gain(&mut self, player: PlayerId, amount: u16) {
        self.players[player.index()].life = self.players[player.index()]
            .life
            .saturating_add(i16::try_from(amount).unwrap_or(i16::MAX));
        let gained = &mut self.life_gained_this_turn[player.index()];
        *gained = gained.saturating_add(amount);
        self.capture_battlefield_triggers(&CommittedTriggerEvent::LifeGained { player, amount });
    }

    fn continue_life_gain(
        &mut self,
        player: PlayerId,
        mut amount: u16,
        mut applied: Vec<AbilitySourceRef>,
    ) {
        let replacements = self.applicable_life_gain_replacements(player, &applied);
        if replacements.is_empty() {
            self.commit_life_gain(player, amount);
            return;
        }

        let all_add = replacements.iter().all(|replacement| {
            matches!(
                replacement.effect,
                ReplacementEffectDef::AddToEventAmount(_)
            )
        });
        let all_multiply = replacements.iter().all(|replacement| {
            matches!(
                replacement.effect,
                ReplacementEffectDef::MultiplyEventAmount(_)
            )
        });
        if replacements.len() == 1 || all_add || all_multiply {
            for replacement in replacements {
                applied.push(replacement.context.source);
                amount = Self::apply_life_gain_amount_replacement(amount, replacement.effect);
            }
            self.continue_life_gain(player, amount, applied);
            return;
        }

        let options = replacements
            .iter()
            .enumerate()
            .filter_map(|(index, replacement)| {
                Some(DecisionOption {
                    id: u32::try_from(index).ok()?,
                    label: replacement.text.to_owned(),
                    card: Some((
                        replacement.context.source.object,
                        replacement.presentation.clone(),
                    )),
                    members: Vec::new(),
                    ability_text: Some(replacement.text.to_owned()),
                    zone: DecisionZone::Battlefield,
                })
            })
            .collect();
        self.queue_decision(
            player,
            "Choose a replacement effect for the life gain",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::LifeGainReplacement {
                player,
                amount,
                applied,
                replacements,
            },
        );
    }

    fn applicable_life_gain_replacements(
        &self,
        player: PlayerId,
        applied: &[AbilitySourceRef],
    ) -> Vec<ApplicableReplacement> {
        let mut replacements = Vec::new();
        for permanent in &self.battlefield {
            self.for_each_effective_ability(permanent, |effective| {
                let ability = effective.ability;
                let DeclarativeAbilityDef::Replacement(definition) = ability.definition else {
                    return;
                };
                let ReplacementEventDef::WouldGainLife(relation) = definition.event else {
                    return;
                };
                let Some(
                    effect @ (ReplacementEffectDef::MultiplyEventAmount(_)
                    | ReplacementEffectDef::AddToEventAmount(_)),
                ) = ability.declarative_replacement()
                else {
                    return;
                };
                let source = AbilitySourceRef {
                    object: permanent.card.id,
                    ability: effective.origin,
                };
                if !applied.contains(&source)
                    && !definition.optional
                    && definition.condition.is_none()
                    && definition
                        .source_zones
                        .contains(&crate::card::ZoneKind::Battlefield)
                    && self.player_relation_matches(
                        player,
                        relation,
                        permanent.controller,
                        TriggerContext::empty(),
                    )
                {
                    replacements.push(ApplicableReplacement {
                        context: ReplacementEffectContext {
                            source,
                            controller: permanent.controller,
                        },
                        presentation: Self::ability_presentation(
                            effective.origin,
                            Self::effective_rules_source(permanent),
                        ),
                        text: ability.text,
                        optional: false,
                        effect,
                    });
                }
            });
        }
        replacements
    }

    fn apply_life_gain_amount_replacement(amount: u16, effect: ReplacementEffectDef) -> u16 {
        match effect {
            ReplacementEffectDef::MultiplyEventAmount(factor) => {
                amount.saturating_mul(u16::from(factor))
            }
            ReplacementEffectDef::AddToEventAmount(extra) => amount.saturating_add(extra),
            _ => amount,
        }
    }

    pub(in crate::game) fn choose_life_gain_replacement(
        &mut self,
        player: PlayerId,
        amount: u16,
        mut applied: Vec<AbilitySourceRef>,
        replacements: &[ApplicableReplacement],
        option: Option<u32>,
    ) {
        let Some(replacement) = option
            .and_then(|option| usize::try_from(option).ok())
            .and_then(|index| replacements.get(index))
        else {
            return;
        };
        applied.push(replacement.context.source);
        self.continue_life_gain(
            player,
            Self::apply_life_gain_amount_replacement(amount, replacement.effect),
            applied,
        );
    }
}
