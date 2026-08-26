// Entry-copy decisions used by the battlefield replacement procedure.
//
// Included textually into `casting.rs`, so the imports here are the parent
// module's.

impl Game {
    /// "You may have this enter as a copy of ...": the copy is picked as the
    /// permanent enters, and entering as itself is always an option.
    pub(super) fn queue_entry_copy_choice(
        &mut self,
        player: PlayerId,
        choices: Vec<GameObjectId>,
        exceptions: crate::card::CopyExceptionsDef,
        added_abilities: Vec<super::CopiableAbility>,
    ) {
        let mut options = vec![DecisionOption {
            id: 0,
            label: "Enter as itself".into(),
            card: None,
            members: Vec::new(),
            ability_text: None,
            zone: DecisionZone::None,
        }];
        options.extend(choices.iter().enumerate().filter_map(|(index, id)| {
            let permanent = self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == *id)?;
            let presentation = Self::effective_rules_source(permanent);
            Some(DecisionOption {
                id: u32::try_from(index + 1).unwrap_or(u32::MAX),
                label: self.presentation_name(presentation).map_or_else(
                    || "Copy an unknown permanent".into(),
                    |name| format!("Enter as a copy of {name}"),
                ),
                card: Some((*id, presentation)),
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Battlefield,
            })
        }));
        self.queue_decision(
            player,
            "Choose what this permanent enters as",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::BattlefieldEntryCopy {
                choices,
                added_types: exceptions.added_types,
                retain_printed_subtypes: false,
                base_power_toughness: exceptions.base_power_toughness,
                colors: exceptions.colors,
                added_creature_types: exceptions.added_creature_types.named.to_vec(),
                no_mana_cost: exceptions.no_mana_cost,
                added_abilities,
            },
        );
    }
}
