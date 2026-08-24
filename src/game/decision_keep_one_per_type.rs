//! Deferred "keep one, then affect the rest" procedures.
//!
//! Divine Reckoning locks every player's survivor before one destruction
//! batch. Ajani's ultimate asks each type separately, carrying earlier keeps
//! through the run before sacrificing the rest.

use super::{
    CardType, DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility,
    DecisionZone, Game, GameObjectId, ObjectPredicateDef, PlayerId,
};

impl Game {
    pub(super) fn queue_destroy_all_but_one_per_player(
        &mut self,
        mut players: Vec<PlayerId>,
        predicate: ObjectPredicateDef,
        source: GameObjectId,
        can_regenerate: bool,
    ) {
        players.sort_by_key(|player| (*player != self.active_player, player.index()));
        players.dedup();
        let remaining = players
            .into_iter()
            .map(|player| {
                let candidates = self
                    .battlefield
                    .iter()
                    .filter(|permanent| permanent.controller == player)
                    .filter(|permanent| {
                        self.trigger_object_matches(
                            predicate,
                            &self.trigger_event_object(permanent),
                            source,
                            false,
                        )
                    })
                    .map(|permanent| permanent.card.id)
                    .collect::<Vec<_>>();
                (player, candidates)
            })
            .collect::<Vec<_>>();
        let candidates = remaining
            .iter()
            .flat_map(|(_, candidates)| candidates.iter().copied())
            .collect();
        self.queue_next_destroy_all_but_one(remaining, candidates, Vec::new(), can_regenerate);
    }

    pub(super) fn queue_next_destroy_all_but_one(
        &mut self,
        mut remaining: Vec<(PlayerId, Vec<GameObjectId>)>,
        candidates: Vec<GameObjectId>,
        mut kept: Vec<GameObjectId>,
        can_regenerate: bool,
    ) {
        let Some((player, choices)) = remaining.first().cloned() else {
            let doomed = candidates
                .into_iter()
                .filter(|candidate| !kept.contains(candidate))
                .collect::<Vec<_>>();
            self.destroy_permanents(&doomed, can_regenerate);
            return;
        };
        remaining.remove(0);
        if choices.len() <= 1 {
            kept.extend(choices);
            self.queue_next_destroy_all_but_one(remaining, candidates, kept, can_regenerate);
            return;
        }
        self.queue_decision(
            player,
            "Choose a permanent to keep",
            DecisionVisibility::Public,
            DecisionPreference::HigherCardValue,
            1..=1,
            false,
            self.permanent_decision_options(&choices),
            DecisionContinuation::DestroyAllButOnePerPlayer {
                remaining,
                candidates,
                kept,
                can_regenerate,
            },
        );
    }

    pub(super) fn queue_keep_one_per_type(
        &mut self,
        player: PlayerId,
        controller: PlayerId,
        remaining: Vec<CardType>,
        kept: Vec<GameObjectId>,
    ) {
        let Some(kind) = remaining.first().copied() else {
            self.sacrifice_unkept_nonlands(player, controller, &kept);
            return;
        };
        let rest = remaining[1..].to_vec();
        let candidates = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == player)
            .filter(|permanent| !kept.contains(&permanent.card.id))
            .filter(|permanent| {
                self.permanent_types(permanent)
                    .is_some_and(|types| types.contains(kind))
            })
            .map(|permanent| {
                (
                    permanent.card.id,
                    Self::effective_rules_source(permanent),
                    self.effective_permanent_name(permanent)
                        .map_or_else(|| "Unknown permanent".into(), std::borrow::Cow::into_owned),
                )
            })
            .collect::<Vec<_>>();
        // A player with nothing of this type keeps nothing of it, and is not
        // asked a question with one answer.
        if candidates.is_empty() {
            self.queue_keep_one_per_type(player, controller, rest, kept);
            return;
        }
        let options = candidates
            .iter()
            .enumerate()
            .map(|(index, (id, characteristics, label))| DecisionOption {
                id: u32::try_from(index).expect("a battlefield fits u32"),
                label: label.clone(),
                card: Some((*id, *characteristics)),
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Battlefield,
            })
            .collect();
        self.queue_decision(
            player,
            Self::keep_one_prompt(kind),
            DecisionVisibility::Public,
            DecisionPreference::HigherCardValue,
            1..=1,
            false,
            options,
            DecisionContinuation::KeepOnePerType {
                player,
                controller,
                remaining,
                kept,
            },
        );
    }

    const fn keep_one_prompt(kind: CardType) -> &'static str {
        match kind {
            CardType::Artifact => "Keep an artifact",
            CardType::Creature => "Keep a creature",
            CardType::Enchantment => "Keep an enchantment",
            CardType::Planeswalker => "Keep a planeswalker",
            CardType::Instant | CardType::Sorcery | CardType::Land => "Keep a permanent",
        }
    }

    /// Everything nonland this player controls that survived the choosing.
    fn sacrifice_unkept_nonlands(
        &mut self,
        player: PlayerId,
        controller: PlayerId,
        kept: &[GameObjectId],
    ) {
        let doomed = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == player)
            .filter(|permanent| !kept.contains(&permanent.card.id))
            .filter(|permanent| {
                self.permanent_types(permanent)
                    .is_some_and(|types| !types.contains(CardType::Land))
            })
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();
        let _ = controller;
        self.sacrifice_permanents(&doomed);
    }
}
