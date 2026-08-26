//! The two ordered choices that make up scry.

use super::{
    CardInstance, DecisionContinuation, DecisionOrderSemantics, DecisionPreference,
    DecisionVisibility, DecisionZone, Game, PlayerId,
};

impl Game {
    pub(super) fn queue_scry(&mut self, player: PlayerId, count: usize) {
        let count = count.min(self.players[player.index()].library.len());
        let mut revealed = Vec::with_capacity(count);
        for _ in 0..count {
            if let Some(card) = self.players[player.index()].library.pop() {
                revealed.push(card);
            }
        }
        if revealed.is_empty() {
            return;
        }
        let options = self.card_decision_options(&revealed, DecisionZone::Library);
        self.queue_decision(
            player,
            "Scry: choose cards for the bottom, naming the bottom card first",
            DecisionVisibility::Private,
            DecisionPreference::LowerCardValue,
            0..=revealed.len(),
            false,
            options,
            DecisionContinuation::ScryBottom { player, revealed },
        );
        if let Some(decision) = self.pending_decisions.last_mut() {
            decision.observation.order_semantics = Some(DecisionOrderSemantics::Resolution);
        }
    }

    pub(super) fn resolve_scry_decision(
        &mut self,
        continuation: DecisionContinuation,
        offered: &[super::DecisionOption],
        options: &[u32],
    ) {
        match continuation {
            DecisionContinuation::ScryBottom { player, revealed } => {
                let bottom_ids = options
                    .iter()
                    .filter_map(|chosen| offered.iter().find(|option| option.id == *chosen))
                    .filter_map(|option| option.card.map(|(card, _)| card))
                    .collect::<Vec<_>>();
                let mut bottom = ordered_cards(&revealed, &bottom_ids);
                let top = revealed
                    .into_iter()
                    .filter(|card| !bottom_ids.contains(&card.id))
                    .collect::<Vec<_>>();
                if top.len() <= 1 {
                    self.finish_scry(player, top, bottom);
                    return;
                }
                let top_len = top.len();
                let options = self.card_decision_options(&top, DecisionZone::Library);
                self.queue_decision(
                    player,
                    "Scry: order the cards staying on top, naming the top card first",
                    DecisionVisibility::Private,
                    DecisionPreference::HigherCardValue,
                    top_len..=top_len,
                    false,
                    options,
                    DecisionContinuation::ScryTop {
                        player,
                        top,
                        bottom: std::mem::take(&mut bottom),
                    },
                );
                if let Some(decision) = self.pending_decisions.last_mut() {
                    decision.observation.order_semantics = Some(DecisionOrderSemantics::Resolution);
                }
            }
            DecisionContinuation::ScryTop {
                player,
                top,
                bottom,
            } => {
                let top_ids = options
                    .iter()
                    .filter_map(|chosen| offered.iter().find(|option| option.id == *chosen))
                    .filter_map(|option| option.card.map(|(card, _)| card))
                    .collect::<Vec<_>>();
                self.finish_scry(player, ordered_cards(&top, &top_ids), bottom);
            }
            _ => unreachable!("only scry decisions reach the scry resolver"),
        }
    }

    fn finish_scry(
        &mut self,
        player: PlayerId,
        top_first: Vec<CardInstance>,
        bottom_first: Vec<CardInstance>,
    ) {
        self.players[player.index()]
            .library
            .splice(0..0, bottom_first);
        self.players[player.index()]
            .library
            .extend(top_first.into_iter().rev());
    }
}

fn ordered_cards(cards: &[CardInstance], ids: &[crate::GameObjectId]) -> Vec<CardInstance> {
    ids.iter()
        .filter_map(|id| cards.iter().find(|card| card.id == *id).cloned())
        .collect()
}
