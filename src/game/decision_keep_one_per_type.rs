//! "Each opponent chooses an artifact, a creature, an enchantment, and a
//! planeswalker from among the nonland permanents they control, then
//! sacrifices the rest."
//!
//! Each type is asked separately. One multi-select could not say "at most
//! one of each", and four independent questions would let a player keep the
//! same permanent twice -- so what has been kept travels with the run, and
//! the sacrifice happens once the last type has been answered.

use super::{
    CardType, DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility,
    DecisionZone, Game, GameObjectId, PlayerId,
};

impl Game {
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
        self.move_permanents_to_graveyard(&doomed);
    }
}
