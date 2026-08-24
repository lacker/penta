impl Game {
    fn draw_trigger_matches(
        &self,
        matcher: crate::card::DrawEventMatcherDef,
        event: &CommittedTriggerEvent,
        source: GameObjectId,
        controller: Option<PlayerId>,
    ) -> bool {
        let CommittedTriggerEvent::DrewCard {
            player,
            card,
            first_in_draw_step,
            nth_this_turn,
        } = event
        else {
            return false;
        };
        let controller = controller.unwrap_or(*player);
        let player_matches = match matcher.player {
            PlayerRelation::ChosenPlayer => self.chosen_player_of(source) == Some(*player),
            PlayerRelation::ControllerOfAttachedPermanent => {
                self.attached_host_controller_of(source) == Some(*player)
            }
            relation => {
                self.player_relation_matches(*player, relation, controller, event.context())
            }
        };
        !(matcher.except_first_in_draw_step && *first_in_draw_step)
            && matcher
                .nth_this_turn
                .is_none_or(|wanted| wanted == *nth_this_turn)
            && player_matches
            && self.trigger_object_matches_for_controller(
                matcher.card,
                card,
                source,
                false,
                Some(controller),
            )
    }
}
