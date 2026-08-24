impl Game {
fn controlled_target_selections(
    &self,
    prefixes: &[Vec<TargetSelection>],
    slot: AbilityTargetDef,
    id: TargetSlotId,
    controller: PlayerId,
    source: GameObjectId,
    x: u16,
) -> Vec<Vec<TargetSelection>> {
    let AbilityTargetPredicate::ControlledByTargetOf {
        object,
        slot: other,
    } = slot.predicate
    else {
        unreachable!("controlled-target enumeration requires its dependent predicate")
    };
    let other = TargetSlotId::from_index(other.index())
        .expect("validated dependent target fits the runtime slot space");
    let mut combined = Vec::new();
    for prefix in prefixes {
        let candidates = prefix
            .iter()
            .find(|selection| selection.slot() == other)
            .and_then(|selection| selection.targets().first().copied())
            .and_then(|target| match target {
                Target::Player(player) => Some(player),
                Target::Permanent(id) | Target::Card(id) | Target::Spell(id) => {
                    self.current_or_last_known_controller(id)
                }
            })
            .map_or_else(Vec::new, |owner| {
                self.battlefield
                    .iter()
                    .filter(|permanent| permanent.controller == owner)
                    .filter(|permanent| {
                        self.trigger_object_matches(
                            object,
                            &self.trigger_event_object(permanent),
                            source,
                            false,
                        ) && self
                            .permanent_can_be_targeted_by(permanent, controller, source, true)
                    })
                    .map(|permanent| Target::Permanent(permanent.card.id))
                    .collect::<Vec<_>>()
            });
        let (minimum, maximum) = slot.count_bounds(x);
        for count in minimum..=maximum {
            for targets in target_combinations(&candidates, usize::from(count)) {
                let mut selected = prefix.clone();
                selected.push(TargetSelection::new(id, targets));
                combined.push(selected);
            }
        }
    }
    combined
}

fn linked_owner_target_selections(
    &self,
    prefixes: &[Vec<TargetSelection>],
    slot: AbilityTargetDef,
    id: TargetSlotId,
    source: GameObjectId,
    x: u16,
) -> Vec<Vec<TargetSelection>> {
    let AbilityTargetPredicate::OwnedByTargetPlayer { slot: other, .. } = slot.predicate else {
        unreachable!("linked-owner enumeration requires its dependent predicate")
    };
    let other = TargetSlotId::from_index(other.index())
        .expect("validated dependent target fits the runtime slot space");
    let mut combined = Vec::new();
    for prefix in prefixes {
        let candidates = prefix
            .iter()
            .find(|selection| selection.slot() == other)
            .and_then(|selection| selection.targets().first())
            .and_then(|target| match target {
                Target::Player(_) => {
                    self.targets_owned_by_target_player(slot.predicate, prefix, source)
                }
                Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
            })
            .unwrap_or_default();
        let (minimum, maximum) = slot.count_bounds(x);
        for count in minimum..=maximum {
            for targets in target_combinations(&candidates, usize::from(count)) {
                let mut selected = prefix.clone();
                selected.push(TargetSelection::new(id, targets));
                combined.push(selected);
            }
        }
    }
    combined
}
}
