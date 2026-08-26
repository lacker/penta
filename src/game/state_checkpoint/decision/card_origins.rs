fn visible_decision_card_origins(
    game: &Game,
    viewer: PlayerId,
    pending: &PendingDecision,
) -> Vec<DecisionCardOriginSnapshot> {
    if pending.observation.visibility != DecisionVisibility::Public
        && pending.observation.player != viewer
    {
        return Vec::new();
    }

    let mut origins = Vec::new();
    let option_objects = pending
        .observation
        .options
        .iter()
        .flat_map(|option| option.card.iter().chain(option.members.iter()))
        .map(|(object, _)| *object);
    let continuation_objects = match &pending.continuation {
        DecisionContinuation::PregameActions { actions, .. } => actions
            .iter()
            .flat_map(|action| {
                std::iter::once(action.source).chain(action.cost_objects.iter().copied())
            })
            .collect::<Vec<_>>(),
        _ => Vec::new(),
    };
    for object in option_objects.chain(continuation_objects) {
        if origins
            .iter()
            .any(|origin: &DecisionCardOriginSnapshot| origin.object_id == object.0)
        {
            continue;
        }
        if let Some((seat, zone, index)) = hidden_card_origin(game, object) {
            origins.push(DecisionCardOriginSnapshot {
                object_id: object.0,
                seat: seat.index(),
                zone,
                index,
            });
        }
    }
    origins
}

pub(super) fn hidden_card_origin(
    game: &Game,
    object: GameObjectId,
) -> Option<(PlayerId, DecisionZoneSnapshot, usize)> {
    for seat in [PlayerId::One, PlayerId::Two] {
        let player = &game.players[seat.index()];
        for (zone, cards) in [
            (DecisionZoneSnapshot::Hand, &player.hand),
            (DecisionZoneSnapshot::Library, &player.library),
            (DecisionZoneSnapshot::OutsideGame, &player.outside_game),
        ] {
            if let Some(index) = cards.iter().position(|card| card.id == object) {
                return Some((seat, zone, index));
            }
        }
    }
    None
}
