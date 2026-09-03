impl Game {
    pub(super) fn entry_scalar_choices(
        &self,
        player: PlayerId,
        choice: BattlefieldEntryScalarChoiceDef,
    ) -> (&'static str, Vec<String>) {
        let (prompt, mut choices, fallback) = match choice.list {
            ScalarChoiceListDef::Players => (
                "Choose a player",
                vec!["You".to_owned(), "Opponent".to_owned()],
                "You",
            ),
            ScalarChoiceListDef::BasicLandTypes => (
                "Choose a basic land type",
                crate::card::BasicLandType::ALL
                    .into_iter()
                    .map(|land_type| land_type.subtype().to_owned())
                    .collect::<Vec<_>>(),
                crate::card::BasicLandType::Plains.subtype(),
            ),
            ScalarChoiceListDef::Colors => (
                "Choose a color",
                crate::card::ManaColor::COLORS
                    .into_iter()
                    .map(|color| color.label().to_owned())
                    .collect::<Vec<_>>(),
                crate::card::ManaColor::White.label(),
            ),
            ScalarChoiceListDef::CardNames
            | ScalarChoiceListDef::NonlandCardNames
            | ScalarChoiceListDef::LandCardNames
            | ScalarChoiceListDef::NonbasicLandCardNames
            | ScalarChoiceListDef::CardNamesOtherThanBasicLands => {
                let mut names = self
                    .catalog
                    .definitions()
                    .into_iter()
                    .filter(|definition| definition.debut_set != CardSet::Token)
                    .flat_map(|definition| definition.parts.iter())
                    // A split card is nameable half by half, so the land test
                    // belongs to the part rather than to the whole card.
                    .filter(|part| match choice.list {
                        ScalarChoiceListDef::CardNames => true,
                        ScalarChoiceListDef::NonlandCardNames => {
                            !part.rules.has_type(CardType::Land)
                        }
                        ScalarChoiceListDef::LandCardNames => part.rules.has_type(CardType::Land),
                        ScalarChoiceListDef::NonbasicLandCardNames => {
                            part.rules.has_type(CardType::Land)
                                && !part.rules.has_supertype(crate::card::CardSupertype::Basic)
                        }
                        ScalarChoiceListDef::CardNamesOtherThanBasicLands => {
                            !part.rules.has_type(CardType::Land)
                                || !part.rules.has_supertype(crate::card::CardSupertype::Basic)
                        }
                        ScalarChoiceListDef::Players
                        | ScalarChoiceListDef::BasicLandTypes
                        | ScalarChoiceListDef::Colors
                        | ScalarChoiceListDef::CreatureTypes => false,
                    })
                    .map(|part| part.name.clone())
                    .collect::<Vec<_>>();
                names.sort();
                names.dedup();
                (
                    match choice.list {
                        ScalarChoiceListDef::NonlandCardNames => "Choose a nonland card name",
                        ScalarChoiceListDef::LandCardNames => "Choose a land card name",
                        ScalarChoiceListDef::NonbasicLandCardNames => {
                            "Choose a nonbasic land card name"
                        }
                        ScalarChoiceListDef::CardNamesOtherThanBasicLands => {
                            "Choose a card name other than a basic land card name"
                        }
                        ScalarChoiceListDef::CardNames => "Choose a card name",
                        ScalarChoiceListDef::Players
                        | ScalarChoiceListDef::BasicLandTypes
                        | ScalarChoiceListDef::Colors
                        | ScalarChoiceListDef::CreatureTypes => unreachable!(
                            "non-card-name scalar lists are handled by earlier match arms"
                        ),
                    },
                    names,
                    "Black Lotus",
                )
            }
            ScalarChoiceListDef::CreatureTypes => (
                "Choose a creature type",
                self.creature_type_choices(player),
                "Human",
            ),
        };
        // A deliberately tiny catalog must not strand an entry procedure.
        if choices.is_empty() {
            choices.push(fallback.into());
        }
        (prompt, choices)
    }
}
