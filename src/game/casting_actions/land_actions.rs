//! Playing a land, which is not casting a spell.
//!
//! A land drop is announced rather than cast: it uses no stack, takes no
//! targets and pays no cost, so the only questions are whose turn it is,
//! whether the drop is spent, and which zone the card may be played from.

use super::super::{
    Action, CardInstance, CardType, Game, GameObjectId, PlayActionKind, PlayerId, ZoneKind,
};

impl Game {
    pub(in crate::game) fn add_land_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        if player != self.active_player
            || !self.step.is_main()
            || !self.stack.is_empty()
            || !self.land_drop_available(player)
        {
            return;
        }
        let state = &self.players[player.index()];
        // A graveyard is walked too, for the permissions that reach into it.
        // Nothing there is playable without one, so the ordinary game pays
        // only the cost of the filter below.
        // The top of the library is walked for the same reason, and named
        // one card at a time.
        for (card, zone) in state
            .hand
            .iter()
            .map(|card| (card, ZoneKind::Hand))
            .chain(
                state
                    .graveyard
                    .iter()
                    .map(|card| (card, ZoneKind::Graveyard)),
            )
            .chain(state.library.last().map(|card| (card, ZoneKind::Library)))
            // Exile is walked for both players, the way the cast offers walk
            // it: a permission to *play* a card reaches a land, and a land
            // somebody else exiled is still played from where it lies. A
            // permission to *cast* one does not -- playing a land is not
            // casting a spell (CR 305.1).
            .chain(
                self.players
                    .iter()
                    .flat_map(|state| state.exile.iter())
                    .filter(|card| self.exile_permission_reaches_lands(card.id, player))
                    .map(|card| (card, ZoneKind::Exile)),
            )
        {
            actions.extend(self.land_actions_for(card, player, zone));
        }
    }

    /// The land plays a card in exile offers `player`, asked without the
    /// timing gate an ordinary land drop passes.
    ///
    /// "You may play the exiled card" is answered while the ability that
    /// said it resolves, which is a time no land could ordinarily be played:
    /// the stack is not empty, and it need not even be this player's turn.
    /// The permission is what makes the play legal then (CR 305.1), and the
    /// land drop is the only limit it does not lift.
    pub(in crate::game) fn offered_land_actions(
        &self,
        player: PlayerId,
        card: GameObjectId,
    ) -> Vec<Action> {
        if !self.land_drop_available(player) || !self.exile_permission_reaches_lands(card, player) {
            return Vec::new();
        }
        self.players
            .iter()
            .flat_map(|state| state.exile.iter())
            .find(|exiled| exiled.id == card)
            .map(|exiled| self.land_actions_for(exiled, player, ZoneKind::Exile))
            .unwrap_or_default()
    }

    fn exile_permission_reaches_lands(&self, card: GameObjectId, player: PlayerId) -> bool {
        self.matching_exile_play_permission(card, player, |permission| {
            permission.lands_may_be_played
        })
        .is_some()
    }

    /// Whether this player has a land drop left to spend.
    fn land_drop_available(&self, player: PlayerId) -> bool {
        self.players[player.index()].lands_played_this_turn <= self.additional_land_plays(player)
            || self
                .player_rule_applies(player, crate::card::AppliedRuleDef::MayPlayAnyNumberOfLands)
    }

    fn land_actions_for(
        &self,
        card: &CardInstance,
        player: PlayerId,
        zone: ZoneKind,
    ) -> Vec<Action> {
        let Some(definition) = self.catalog.get(card.definition) else {
            return Vec::new();
        };
        definition
            .play_options
            .iter()
            .filter(|option| option.action == PlayActionKind::PlayLand)
            .filter(|option| !self.play_is_prohibited(card, player, option))
            .filter(|option| match zone {
                ZoneKind::Graveyard => self.graveyard_play_is_permitted(card, player, option),
                ZoneKind::Library => self.library_top_play_cost(card, player, option).is_some(),
                // The permission was already checked to get here.
                _ => true,
            })
            .filter(|option| match &option.form {
                crate::card::SpellForm::Part(part) => definition
                    .part(*part)
                    .is_some_and(|part| part.rules.has_type(CardType::Land)),
                crate::card::SpellForm::Combined(_) => false,
            })
            .map(|option| Action::PlayLand {
                card: card.id,
                option: option.id,
            })
            .collect()
    }
}
