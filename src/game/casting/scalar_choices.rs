use crate::card::CardNameSetDef;

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
            ScalarChoiceListDef::CardNames(name_set) => {
                let names = self
                    .catalog_card_names(name_set)
                    .unwrap_or_default()
                    .into_iter()
                    .collect();
                (
                    match name_set {
                        CardNameSetDef::NonlandCardNames => "Choose a nonland card name",
                        CardNameSetDef::LandCardNames => "Choose a land card name",
                        CardNameSetDef::NonbasicLandCardNames => {
                            "Choose a nonbasic land card name"
                        }
                        CardNameSetDef::CardNamesOtherThanBasicLands => {
                            "Choose a card name other than a basic land card name"
                        }
                        CardNameSetDef::BasicLandNames => "Choose a basic land card name",
                        CardNameSetDef::AllCardNames
                        | CardNameSetDef::NamesOf(_)
                        | CardNameSetDef::Union(_)
                        | CardNameSetDef::NamesAppearingAtLeast { .. } => "Choose a card name",
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
