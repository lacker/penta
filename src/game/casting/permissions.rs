use super::{
    AbilitySourceRef, AlternativeCastKindDef, AppliedEffectDef, AppliedStackEffect, CastSignature,
    CastSourceZone, Game, GameObjectId, Permanent, PlayOptionDef, PlayerId, StackObject,
};

impl Game {
    /// The same bookkeeping for a land, which has no signature to resolve
    /// and whose permanent is still in hand rather than on the battlefield.
    pub(super) fn spend_graveyard_land_permission(
        &mut self,
        permanent: &mut Permanent,
        player: PlayerId,
        option: &PlayOptionDef,
    ) {
        let Some(card) = permanent.card.clone().into_card() else {
            return;
        };
        if let Some((source, effect)) = self.graveyard_play_grant(&card, player, option) {
            self.grant_resolved_ability_to_entering_permanent(permanent, source, *effect);
        }
        self.record_graveyard_permission_use(&card, player, option);
    }

    /// The graveyard-permission bookkeeping for a cast, resolved from the
    /// card and the play option the signature names.
    /// Hands a spell what the permission that allowed it grants, for the
    /// permanent it will become to carry.
    pub(super) fn attach_permission_grant(
        stack_object: &mut StackObject,
        granted: Option<(AbilitySourceRef, &'static AppliedEffectDef)>,
    ) {
        let Some((granting, effect)) = granted else {
            return;
        };
        stack_object.applied_effects.push(AppliedStackEffect {
            source: None,
            granting: Some(granting),
            effect: *effect,
        });
    }

    pub(super) fn record_cast_context(
        stack_object: &mut StackObject,
        via_suspend: bool,
        phyrexian_life_symbols: u16,
        granted: Option<(AbilitySourceRef, &'static AppliedEffectDef)>,
    ) {
        let cast = stack_object
            .cast
            .as_mut()
            .expect("a proposed cast spell has cast context");
        cast.via_suspend = via_suspend;
        cast.phyrexian_symbols_paid_with_life = phyrexian_life_symbols;
        // "If you do, it gains ...": carry the permission's grant through
        // the spell into the permanent it becomes.
        Self::attach_permission_grant(stack_object, granted);
    }

    /// A cast from a graveyard that is not one of the card's own printed ways
    /// of being cast is happening under somebody's permission, and a
    /// permission that allows only so many spends one here.
    pub(super) fn spend_cast_permissions(
        &mut self,
        player: PlayerId,
        card_id: GameObjectId,
        signature: &CastSignature,
        source_zone: CastSourceZone,
        alternative_kind: Option<AlternativeCastKindDef>,
    ) -> (Option<(AbilitySourceRef, &'static AppliedEffectDef)>, bool) {
        let grants_haste = self.exile_cast_grants_haste(card_id, player);
        if signature.costs().permission_source().is_some() {
            let granted = self
                .card_in_nonbattlefield_zone(card_id)
                .and_then(|(_, card)| {
                    let option = self
                        .catalog
                        .get(card.definition)?
                        .play_option(signature.play_option())?;
                    let (source, permission) = self.selected_play_permission(
                        card,
                        player,
                        option,
                        signature.x(),
                        signature.costs(),
                    )?;
                    permission.grants.map(|grant| (source, grant))
                });
            return (granted, grants_haste);
        }
        if source_zone != CastSourceZone::Graveyard || alternative_kind.is_some() {
            return (None, grants_haste);
        }
        (
            self.record_graveyard_permission_use_for_cast(player, card_id, signature),
            grants_haste,
        )
    }

    pub(super) fn record_graveyard_permission_use_for_cast(
        &mut self,
        player: PlayerId,
        card_id: GameObjectId,
        signature: &CastSignature,
    ) -> Option<(AbilitySourceRef, &'static AppliedEffectDef)> {
        let card = self.players[player.index()]
            .graveyard
            .iter()
            .find(|candidate| candidate.id == card_id)
            .cloned()?;
        let option = self
            .catalog
            .get(card.definition)
            .and_then(|definition| definition.play_option(signature.play_option()))
            .cloned()?;
        // "If you do, it gains ...": read here, where the card is still in
        // the graveyard and the permission that names it can still be found.
        // The spell it becomes does not exist yet, so the caller carries it
        // the few lines to the stack object.
        let granted = self.graveyard_play_grant(&card, player, &option);
        self.record_graveyard_permission_use(&card, player, &option);
        granted
    }
}
