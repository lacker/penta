//! The final shared activation-cost stage after every deferred choice is paid.

use super::{
    BattlefieldExitCompletion, FrozenActivatedAbility, Game, GameObjectId, ObjectInstance,
    PlayerId, TargetSelection,
};

impl Game {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn continue_activated_ability_costs(
        &mut self,
        source: GameObjectId,
        source_card: ObjectInstance,
        controller: PlayerId,
        frozen: FrozenActivatedAbility,
        targets: Vec<TargetSelection>,
        chosen_permanents: Vec<GameObjectId>,
        mut remaining_sacrifices: Vec<GameObjectId>,
    ) {
        if !remaining_sacrifices.is_empty() {
            let sacrificed = remaining_sacrifices.remove(0);
            self.capture_sacrifices(&[sacrificed]);
            self.move_permanents_to_graveyard_then(
                &[sacrificed],
                Some(BattlefieldExitCompletion::CompleteActivatedAbility {
                    source,
                    source_card,
                    controller,
                    frozen,
                    targets,
                    chosen_permanents,
                    remaining_sacrifices,
                }),
            );
            return;
        }

        self.push_activated_ability(
            source,
            &source_card,
            controller,
            frozen,
            targets,
            chosen_permanents,
        );
        self.consecutive_passes = 0;
        self.check_state_based_actions();
    }
}
