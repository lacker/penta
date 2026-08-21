//! Sacrificing creatures until their power adds up to a printed total.
//!
//! Named one at a time rather than all at once, because the decision model
//! bounds a selection by how many things are chosen and this cost is bounded
//! by what they add up to. Stopping is offered as soon as the total is
//! reached, so a payer who wants more creature cards in their graveyard may
//! keep going and one who does not may stop at the minimum.

use super::{
    DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone,
    EffectResolutionContext, Game, GameObjectId, PlayerId, ScopedEffect, StackObject,
};

impl Game {
    /// What the payer's creatures could add up to. Negative power counts as
    /// nothing rather than subtracting: a -1/-1 creature makes the total no
    /// harder to reach.
    pub(super) fn total_creature_power_controlled(&self, player: PlayerId) -> i32 {
        self.battlefield
            .iter()
            .filter(|permanent| permanent.controller == player)
            .filter_map(|permanent| self.creature_stats(permanent))
            .map(|stats| i32::from(stats.power).max(0))
            .sum()
    }

    /// Offers the next creature to sacrifice, and finishes once the total is
    /// reached and the payer stops.
    pub(super) fn queue_total_power_sacrifice(
        &mut self,
        player: PlayerId,
        remaining: i32,
        object: &StackObject,
        context: EffectResolutionContext,
        if_paid: Option<ScopedEffect>,
    ) {
        let candidates: Vec<(GameObjectId, i32)> = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == player)
            .filter_map(|permanent| {
                self.creature_stats(permanent)
                    .map(|stats| (permanent.card.id, i32::from(stats.power).max(0)))
            })
            .collect();
        // Nothing left to give. A board that could not reach the total is
        // never asked in the first place, so this is the payer who ran out
        // after starting.
        if candidates.is_empty() {
            if let Some(effect) = if_paid {
                self.resolve_nested_effect_before_later(effect, object, context);
            }
            return;
        }
        let mut options = Vec::new();
        // "Any number ... with total power 12 or greater" is a floor, not a
        // quota: once it is met the payer may stop, and a deck that wants
        // creature cards in its graveyard may keep going instead.
        if remaining <= 0 {
            options.push(DecisionOption {
                id: 0,
                label: "Stop".to_string(),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::None,
            });
        }
        for (index, (permanent, _)) in candidates.iter().enumerate() {
            let name = self
                .permanent_card_name(*permanent)
                .map_or_else(|| "a creature".to_string(), std::borrow::Cow::into_owned);
            options.push(DecisionOption {
                id: u32::try_from(index + 1).unwrap_or(u32::MAX),
                label: format!("Sacrifice {name}"),
                card: self
                    .battlefield
                    .iter()
                    .find(|candidate| candidate.card.id == *permanent)
                    .map(|candidate| (*permanent, Self::effective_rules_source(candidate))),
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Battlefield,
            });
        }
        self.queue_decision(
            player,
            "Sacrifice creatures to pay",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::SacrificeToTotalPower {
                player,
                remaining,
                object: Box::new(object.clone()),
                context,
                if_paid,
            },
        );
    }

    /// Applies one answer and asks again while anything is still owed.
    pub(super) fn continue_total_power_sacrifice(
        &mut self,
        player: PlayerId,
        remaining: i32,
        chosen: Option<GameObjectId>,
        object: &StackObject,
        context: EffectResolutionContext,
        if_paid: Option<ScopedEffect>,
    ) {
        let Some(chosen) = chosen else {
            if let Some(effect) = if_paid {
                self.resolve_nested_effect_before_later(effect, object, context);
            }
            return;
        };
        let power = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == chosen)
            .and_then(|permanent| self.creature_stats(permanent))
            .map_or(0, |stats| i32::from(stats.power).max(0));
        self.move_permanents_to_graveyard(&[chosen]);
        self.queue_total_power_sacrifice(player, remaining - power, object, context, if_paid);
    }
}
