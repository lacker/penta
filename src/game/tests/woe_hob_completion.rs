//! Deck-corpus coverage across shared payment and zone-permission boundaries.
use super::*;

pub(super) fn setup(prepared: bool) -> Game {
    let mut game = ready_game();
    game.set_prepared_engine_enabled(prepared);
    game.turns_started = [3, 3];
    game
}

fn cast_for(game: &Game, player: PlayerId, id: GameObjectId) -> Option<Action> {
    game.legal_actions(player)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
}

pub(super) fn hand(game: &mut Game, definition: CardDefinitionId) -> GameObjectId {
    let card = game
        .build_zone(PlayerId::One, &[definition])
        .unwrap()
        .remove(0);
    let id = card.id;
    game.players[0].hand.push(card);
    id
}

mod payments;
mod permanent_rules;
mod triggers;
mod zone_permissions;

fn activation_named(game: &Game, source: GameObjectId, prefix: &str) -> Action {
    let permanent = game
        .battlefield
        .iter()
        .find(|p| p.card.id == source)
        .unwrap();
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            let Action::ActivateAbility {
                source: selected,
                ability,
                ..
            } = action
            else {
                return false;
            };
            if *selected != source {
                return false;
            }
            let mut matches = false;
            game.for_each_effective_ability(permanent, |effective| {
                matches |=
                    effective.origin == *ability && effective.ability.text.starts_with(prefix);
            });
            matches
        })
        .unwrap_or_else(|| panic!("missing activation {prefix}"))
}
