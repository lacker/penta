//! Ninjutsu (CR 702.49).
//!
//! An activated ability that works from its owner's hand, in the one window
//! where a creature can be both unblocked and still worth swapping out:
//! after attackers are declared and before blockers are. It returns an
//! unblocked attacker and puts the ninja onto the battlefield in its place,
//! tapped and attacking the same defender.

use super::{
    AttackDefender, BattlefieldArrival, Game, GameObjectId, PlayerId, StackObject, Target,
    ZoneKind, ZoneMoveCause, ZonePlacement,
};

impl Game {
    /// The attackers `player` controls that nothing has been declared
    /// against. Written against the blocked set rather than against the
    /// window, because "unblocked" is what the cost actually asks.
    pub(super) fn unblocked_attackers_controlled_by(&self, player: PlayerId) -> Vec<GameObjectId> {
        self.battlefield
            .iter()
            .filter(|permanent| permanent.controller == player && permanent.attacking)
            .filter(|permanent| !self.combat_blocked_attackers.contains(&permanent.card.id))
            .map(|permanent| permanent.card.id)
            .collect()
    }

    /// Who a permanent is attacking, read before it is returned to hand.
    pub(super) fn attack_defender_of(&self, attacker: GameObjectId) -> Option<AttackDefender> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == attacker)
            .and_then(|permanent| permanent.attack_defender)
    }

    /// The player defending against one attacking permanent. Attacks on a
    /// planeswalker still have that permanent's controller as their
    /// defending player, including after the planeswalker has left combat.
    pub(super) fn defending_player_of(&self, attacker: GameObjectId) -> Option<PlayerId> {
        match self.attack_defender_of(attacker)? {
            AttackDefender::Player(player) => Some(player),
            AttackDefender::Planeswalker(planeswalker) => {
                self.current_or_last_known_controller(planeswalker)
            }
        }
    }

    /// Puts the ninja onto the battlefield from its owner's hand, tapped and
    /// attacking whoever the returned creature was attacking (CR 702.49b).
    /// It was never declared as an attacker, so nothing watching for a
    /// declaration fires.
    pub(super) fn put_ninja_onto_the_battlefield(&mut self, object: &StackObject) {
        let Some(source) = object.source else {
            return;
        };
        let controller = object.controller;
        let defender = self.ninjutsu_returned_defender.take();
        let Some(arrived) = self.move_target_to_zone(
            Target::Card(source),
            ZoneKind::Battlefield,
            ZoneMoveCause::Effect { controller },
            Some(BattlefieldArrival::tapped_under(controller)),
            ZonePlacement::Top,
        ) else {
            return;
        };
        if let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == arrived)
        {
            permanent.attacking = true;
            permanent.attack_defender = defender;
            permanent.attacked_this_turn = true;
            permanent.attacks_this_turn = permanent.attacks_this_turn.saturating_add(1);
        }
    }
}
