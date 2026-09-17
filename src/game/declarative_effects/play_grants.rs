//! Resolving grants over exact exile or graveyard objects.
use super::super::{
    EffectResolutionContext, ExilePlayCost, Game, ScopedEffect, StackObject, Target,
};
use crate::card::{ExilePlayDurationDef, ZoneKind, ZonePlayGrantDef};

impl Game {
    pub(super) fn resolve_play_grant(
        &mut self,
        grant: &ZonePlayGrantDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let Some(player) = self.effect_player_reference(grant.player, object, context, scoped)
        else {
            return;
        };
        for target in self.effect_objects(grant.objects, object, context, scoped) {
            let Target::Card(card) = target else { continue };
            let Some((zone, _)) = self.card_in_nonbattlefield_zone(card) else {
                continue;
            };
            if !matches!(zone, ZoneKind::Exile | ZoneKind::Graveyard) {
                continue;
            }
            match grant.duration {
                ExilePlayDurationDef::ThisTurn => self.permit_cast_this_turn(card, player),
                ExilePlayDurationDef::UntilYourNextEndStep => {
                    self.permit_play_until_your_next_end_step(card, player);
                }
                ExilePlayDurationDef::UntilEndOfYourNextTurn => {
                    self.permit_play_until_end_of_your_next_turn(card, player);
                }
                ExilePlayDurationDef::WhileExiled => {
                    self.permit_conditional_cast_while_exiled(card, player);
                }
            }
            let permission = self
                .exile_play_permissions
                .last_mut()
                .expect("permission just granted");
            permission.zone = zone;
            permission.lands_may_be_played = !grant.cast_only;
            if let Some(cost) = grant.mana_cost {
                permission.cost = ExilePlayCost::AlternativeMana(cost);
            }
        }
    }
}
