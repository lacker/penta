//! Which objects a checkpoint can name, and which it cannot.
//!
//! A checkpoint is written for one seat and read back beside a hypothesis
//! about everything that seat cannot see. Anything in a library, in somebody
//! else's hand, or outside the game is minted fresh from that hypothesis, so
//! an id recorded here would name nothing on the way back in. These decide
//! which references survive that, and collect the ones that have to be
//! rebound by position instead.
//!
//! Split out of the parent module for the source-size budget; the paths and
//! imports are the parent module's.

use super::{
    DecisionCardOriginSnapshot, EffectResolutionContext, Game, GameObjectId, PlayerId,
    RetiredObject, StackAbilityPayload, StackObject, StackSnapshot, Target, TargetSelection,
};

pub(in crate::game::state_checkpoint) fn stack_object_requires_retired(
    game: &Game,
    object: &StackObject,
) -> bool {
    referenced_object_ids(object).any(|id| game.retired_objects.contains_key(&id))
}

pub(in crate::game::state_checkpoint) fn stack_object_has_unrebindable_hidden_reference(
    game: &Game,
    viewer: PlayerId,
    object: &StackObject,
) -> bool {
    object
        .source
        .is_some_and(|source| stack_source_requires_hidden_rebinding(game, viewer, source))
        || object.ability.as_ref().is_some_and(|payload| {
            stack_payload_has_unrebindable_hidden_reference_except(game, viewer, payload, &[])
        })
}

pub(in crate::game::state_checkpoint) fn referenced_object_ids(
    object: &StackObject,
) -> impl Iterator<Item = GameObjectId> {
    let mut ids = Vec::new();
    ids.extend(object.source);
    ids.extend(object.chosen_permanents.iter().copied());
    ids.extend(
        object
            .applied_effects
            .iter()
            .filter_map(|effect| effect.source.map(|source| source.object)),
    );
    if let Some(payload) = &object.ability {
        ids.extend(resolution_context_referenced_object_ids(&payload.context));
        ids.extend(lexical_target_referenced_object_ids(payload));
    }
    ids.into_iter()
}

/// Target selections with a declared slot are ordinary targets: once their
/// object changes zones, the id is deliberately left dangling so legality
/// makes the spell or ability fizzle. Extra selections without a declared
/// slot are captured lexical state (for example a delayed follow-up referring
/// to an earlier target), so they still require hidden rebinding and LKI.
pub(in crate::game::state_checkpoint) fn lexical_target_referenced_object_ids(
    payload: &StackAbilityPayload,
) -> Vec<GameObjectId> {
    payload
        .targets
        .iter()
        .filter(|selection| payload.target_defs.get(selection.slot().index()).is_none())
        .flat_map(TargetSelection::targets)
        .filter_map(|target| match target {
            Target::Player(_) => None,
            Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => Some(*id),
        })
        .collect()
}

pub(in crate::game::state_checkpoint) fn stack_payload_has_unrebindable_hidden_reference_except(
    game: &Game,
    viewer: PlayerId,
    payload: &StackAbilityPayload,
    visible_rebindings: &[GameObjectId],
) -> bool {
    lexical_target_referenced_object_ids(payload)
        .into_iter()
        .chain(resolution_context_referenced_object_ids(&payload.context))
        .any(|object| {
            object_reference_requires_hidden_rebinding(game, viewer, object)
                && !visible_rebindings.contains(&object)
        })
}

pub(in crate::game::state_checkpoint) fn target_selections_referenced_object_ids(
    selections: &[TargetSelection],
) -> Vec<GameObjectId> {
    selections
        .iter()
        .flat_map(TargetSelection::targets)
        .filter_map(|target| match target {
            Target::Player(_) => None,
            Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => Some(*id),
        })
        .collect()
}

/// Captured trigger state has no hidden-zone rebinding table of its own.
/// References to the viewer's hand keep their public observation ids, but
/// libraries, outside-game cards, and the opposing hand are reconstructed
/// from a hypothesis with freshly minted ids. Serializing one of those host
/// ids would both disclose hidden identity and leave a dangling reference in
/// the reconstructed game, so the containing checkpoint must fail closed.
pub(in crate::game::state_checkpoint) fn trigger_capture_has_unrebindable_hidden_reference(
    game: &Game,
    viewer: PlayerId,
    targets: &[TargetSelection],
    context: &EffectResolutionContext,
) -> bool {
    trigger_capture_has_unrebindable_hidden_reference_except(game, viewer, targets, context, &[])
}

pub(in crate::game::state_checkpoint) fn trigger_capture_has_unrebindable_hidden_reference_except(
    game: &Game,
    viewer: PlayerId,
    targets: &[TargetSelection],
    context: &EffectResolutionContext,
    visible_rebindings: &[GameObjectId],
) -> bool {
    target_selections_referenced_object_ids(targets)
        .into_iter()
        .chain(resolution_context_referenced_object_ids(context))
        .any(|object| {
            object_reference_requires_hidden_rebinding(game, viewer, object)
                && !visible_rebindings.contains(&object)
        })
}

/// Whether a stack object's own source is one the checkpoint cannot hand
/// back, so its ability payload has to be left out.
///
/// One case narrower than [`object_reference_requires_hidden_rebinding`]: a
/// source that retired as a card. Every observation publishes a stack
/// ability's `sourceObjectId` whatever became of the card behind it, and the
/// checkpoint carries that card among its retired objects, so naming it here
/// says nothing that was not already said and the importer finds it again by
/// id. That is the ordinary shape of an ability activated from a hand for a
/// cost that discards the card -- bloodrush, and every clause like it.
///
/// A source still sitting in a library, outside the game, or in somebody
/// else's hand is a different matter: nothing in the checkpoint can point at
/// it without saying what it is, which is the case Miracle's linked offer
/// brought in.
pub(in crate::game::state_checkpoint) fn stack_source_requires_hidden_rebinding(
    game: &Game,
    viewer: PlayerId,
    object: GameObjectId,
) -> bool {
    [PlayerId::One, PlayerId::Two].into_iter().any(|player| {
        let state = &game.players[player.index()];
        state.library.iter().any(|card| card.id == object)
            || state.outside_game.iter().any(|card| card.id == object)
            || (player != viewer && state.hand.iter().any(|card| card.id == object))
    })
}

pub(in crate::game::state_checkpoint) fn object_reference_requires_hidden_rebinding(
    game: &Game,
    viewer: PlayerId,
    object: GameObjectId,
) -> bool {
    matches!(
        game.retired_objects.get(&object),
        Some(RetiredObject::Card(_))
    ) || [PlayerId::One, PlayerId::Two].into_iter().any(|player| {
        let state = &game.players[player.index()];
        state.library.iter().any(|card| card.id == object)
            || state.outside_game.iter().any(|card| card.id == object)
            || (player != viewer && state.hand.iter().any(|card| card.id == object))
    })
}

pub(in crate::game::state_checkpoint) fn resolution_context_referenced_object_ids(
    context: &EffectResolutionContext,
) -> Vec<GameObjectId> {
    let mut ids = context.trigger.object.into_iter().collect::<Vec<_>>();
    if let Some(draw) = &context.replaced_draw {
        ids.extend(draw.applied.iter().map(|source| source.object));
    }
    ids.extend(
        context
            .single_objects()
            .iter()
            .flatten()
            .chain(context.object_groups().iter().flatten())
            .chain(context.named_object_groups().values().flatten())
            .copied()
            .filter_map(|target| match target {
                Target::Player(_) => None,
                Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => Some(id),
            }),
    );
    ids
}

/// Every hidden-zone source position the live stack recorded, for the
/// importer to bind its minted cards to before anything reads an id.
pub(in crate::game::state_checkpoint) fn stack_source_origins(
    stack: &[StackSnapshot],
) -> Vec<DecisionCardOriginSnapshot> {
    stack
        .iter()
        .filter_map(|object| object.ability_payload.as_ref())
        .filter_map(|payload| payload.source_origin)
        .collect()
}
