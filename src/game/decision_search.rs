use super::{
    CardChoiceSourceDef, CardInstance, CardPartId, DecisionContinuation, DecisionOption,
    DecisionPreference, DecisionVisibility, DecisionZone, EffectResolutionContext, Game,
    GameObjectId, ObjectCharacteristics, ObjectPredicateDef, PlayerId, ScopedEffect,
    SearchFollowUp, StackObject, ZoneKind, ZonePlacement,
};

impl Game {
    /// Offers a search over the cards a predicate admits. Hidden-zone choices
    /// stay private; graveyards and exile are already public information.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn queue_zone_search(
        &mut self,
        player: PlayerId,
        source_zone: ZoneKind,
        predicate: ObjectPredicateDef,
        minimum: usize,
        maximum: usize,
        reveal: bool,
        destination: ZoneKind,
        placement: ZonePlacement,
        shuffle: bool,
        binding: Option<crate::ids::ObjectSetBindingIndex>,
        follow_up: Option<(StackObject, EffectResolutionContext, ScopedEffect)>,
        enters_tapped: bool,
        attached_player: Option<PlayerId>,
        source: GameObjectId,
        controller: PlayerId,
    ) {
        if maximum == 0 {
            if shuffle && source_zone == ZoneKind::Library {
                self.rng.shuffle(&mut self.players[player.index()].library);
            }
            return;
        }
        let cards = match source_zone {
            ZoneKind::Library => &self.players[player.index()].library,
            ZoneKind::Hand => &self.players[player.index()].hand,
            ZoneKind::Graveyard => &self.players[player.index()].graveyard,
            ZoneKind::Exile => &self.players[player.index()].exile,
            ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => return,
        };
        let decision_zone = match source_zone {
            ZoneKind::Library => DecisionZone::Library,
            ZoneKind::Hand => DecisionZone::Hand,
            ZoneKind::Graveyard => DecisionZone::Graveyard,
            ZoneKind::Exile => DecisionZone::Exile,
            ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => return,
        };
        let options: Vec<_> = cards
            .iter()
            .filter(|card| self.card_object_matches(predicate, card, source_zone, source))
            .filter(|card| {
                attached_player.is_none_or(|player| {
                    self.card_can_enchant_player(card.definition, controller, player)
                })
            })
            .enumerate()
            .map(|(index, card)| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: self
                    .catalog
                    .get(card.definition)
                    .map_or_else(|| "Unknown card".into(), |card| card.name.clone()),
                card: Some((
                    card.id,
                    ObjectCharacteristics::card(card.definition, CardPartId::PRIMARY),
                )),
                members: Vec::new(),
                ability_text: None,
                zone: decision_zone,
            })
            .collect();
        let minimum = minimum.min(options.len());
        let maximum = maximum.min(options.len()).max(minimum);
        if options.is_empty() {
            if shuffle && source_zone == ZoneKind::Library {
                self.rng.shuffle(&mut self.players[player.index()].library);
            }
            return;
        }
        self.queue_decision(
            player,
            "Choose cards from the searched zone",
            if matches!(source_zone, ZoneKind::Library | ZoneKind::Hand) {
                DecisionVisibility::Private
            } else {
                DecisionVisibility::Public
            },
            DecisionPreference::HigherCardValue,
            minimum..=maximum,
            false,
            options,
            DecisionContinuation::SearchZone {
                controller,
                source: source_zone,
                destination,
                placement,
                reveal,
                shuffle,
                enters_tapped,
                attached_player,
                binding,
                follow_up: follow_up.map(|(object, context, effect)| {
                    Box::new(SearchFollowUp {
                        object,
                        context,
                        effect,
                    })
                }),
            },
        );
    }

    /// Offers a non-search card choice over owned cards in one or more places.
    /// Any hidden source makes the whole decision private, so combining a
    /// public exile with a hidden sideboard never reveals the sideboard.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn queue_owned_card_choice(
        &mut self,
        player: PlayerId,
        sources: &'static [CardChoiceSourceDef],
        predicate: ObjectPredicateDef,
        minimum: usize,
        maximum: usize,
        reveal: bool,
        destination: ZoneKind,
        placement: ZonePlacement,
        arrival: Option<(StackObject, EffectResolutionContext, ScopedEffect)>,
        source: GameObjectId,
        controller: PlayerId,
    ) {
        let mut options = Vec::new();
        for source_def in sources {
            let (cards, zone, predicate_zone): (&[CardInstance], DecisionZone, ZoneKind) =
                match source_def {
                    CardChoiceSourceDef::Zone(ZoneKind::Library) => (
                        &self.players[player.index()].library,
                        DecisionZone::Library,
                        ZoneKind::Library,
                    ),
                    CardChoiceSourceDef::Zone(ZoneKind::Hand) => (
                        &self.players[player.index()].hand,
                        DecisionZone::Hand,
                        ZoneKind::Hand,
                    ),
                    CardChoiceSourceDef::Zone(ZoneKind::Graveyard) => (
                        &self.players[player.index()].graveyard,
                        DecisionZone::Graveyard,
                        ZoneKind::Graveyard,
                    ),
                    CardChoiceSourceDef::Zone(ZoneKind::Exile) => (
                        &self.players[player.index()].exile,
                        DecisionZone::Exile,
                        ZoneKind::Exile,
                    ),
                    CardChoiceSourceDef::OutsideGame => (
                        &self.players[player.index()].outside_game,
                        DecisionZone::OutsideGame,
                        ZoneKind::Hand,
                    ),
                    CardChoiceSourceDef::Zone(
                        ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command,
                    ) => continue,
                };
            for card in cards
                .iter()
                .filter(|card| self.card_object_matches(predicate, card, predicate_zone, source))
            {
                options.push(DecisionOption {
                    id: u32::try_from(options.len()).unwrap_or(u32::MAX),
                    label: self
                        .catalog
                        .get(card.definition)
                        .map_or_else(|| "Unknown card".into(), |card| card.name.clone()),
                    card: Some((
                        card.id,
                        ObjectCharacteristics::card(card.definition, CardPartId::PRIMARY),
                    )),
                    members: Vec::new(),
                    ability_text: None,
                    zone,
                });
            }
        }

        // An impossible instruction does nothing. In particular, a Ring draw
        // is still replaced when neither exile nor the sideboard has a card.
        if options.is_empty() {
            return;
        }
        let minimum = minimum.min(options.len());
        let maximum = maximum.min(options.len()).max(minimum);
        self.queue_decision(
            player,
            "Choose an owned card to put into your hand",
            if sources.iter().any(|source| {
                matches!(
                    source,
                    CardChoiceSourceDef::OutsideGame
                        | CardChoiceSourceDef::Zone(ZoneKind::Library | ZoneKind::Hand)
                )
            }) {
                DecisionVisibility::Private
            } else {
                DecisionVisibility::Public
            },
            DecisionPreference::HigherCardValue,
            minimum..=maximum,
            false,
            options,
            DecisionContinuation::ChooseCards {
                controller,
                destination,
                placement,
                reveal,
                arrival: arrival.map(|(object, context, effect)| {
                    Box::new(SearchFollowUp {
                        object,
                        context,
                        effect,
                    })
                }),
            },
        );
    }
}
