// The choices a resolving effect stops to ask about a colour or a land type.
//
// Split out of `entry_replacements.rs` for the source-size budget, and split
// here because they are not replacements at all: each one queues a question
// and carries on where the answer arrives. Included textually, so the
// imports here are that module's.

impl Game {
    pub(super) fn queue_text_change(
        &mut self,
        player: PlayerId,
        target: Target,
        kind: TextChangeKindDef,
        duration: ResolvedEffectDurationDef,
    ) {
        let mut options = Vec::new();
        if matches!(
            kind,
            TextChangeKindDef::BasicLandType | TextChangeKindDef::BasicLandTypeOrColorWord
        ) {
            options.extend(BasicLandType::ALL.into_iter().flat_map(|from| {
                BasicLandType::ALL
                    .into_iter()
                    .filter(move |to| from != *to)
                    .map(move |to| DecisionOption {
                        id: u32::try_from(from.index() * BasicLandType::ALL.len() + to.index())
                            .expect("the basic-land-type choice id fits u32"),
                        label: format!("{} → {}", from.subtype(), to.subtype()),
                        card: None,
                        members: Vec::new(),
                        ability_text: None,
                        zone: DecisionZone::None,
                    })
            }));
        }
        if matches!(
            kind,
            TextChangeKindDef::ColorWord | TextChangeKindDef::BasicLandTypeOrColorWord
        ) {
            let offset = u32::try_from(BasicLandType::ALL.len().pow(2))
                .expect("the text-change choice offset fits u32");
            options.extend(Self::CHOOSABLE_COLORS.into_iter().flat_map(|from| {
                Self::CHOOSABLE_COLORS
                    .into_iter()
                    .filter(move |to| from != *to)
                    .map(move |to| DecisionOption {
                        id: offset
                            + u32::try_from(
                                Self::color_index(from) * Self::CHOOSABLE_COLORS.len()
                                    + Self::color_index(to),
                            )
                            .expect("the color-word choice id fits u32"),
                        label: format!("{} → {}", from.label(), to.label()),
                        card: None,
                        members: Vec::new(),
                        ability_text: None,
                        zone: DecisionZone::None,
                    })
            }));
        }
        let expiration = Self::continuous_effect_expiration(
            duration,
            player,
            self.turns_started[player.index()],
        );
        self.queue_decision(
            player,
            "Replace one word with another",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::TextChange {
                target,
                kind,
                expiration,
            },
        );
    }

    fn color_index(color: ManaColor) -> usize {
        Self::CHOOSABLE_COLORS
            .iter()
            .position(|candidate| *candidate == color)
            .expect("a text-change color is one of the five colors")
    }

    /// The five colours a card can name. Colourless is not among them: "the
    /// color of your choice" names a colour, and colourless is the absence
    /// of one.
    pub(super) const CHOOSABLE_COLORS: [ManaColor; 5] = [
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
    ];

    /// The qualities "protection from colorless or from the color of your
    /// choice" offers, in the order the decision numbers them. Colourless
    /// goes last so the five-colour indices keep their meaning.
    pub(super) const CHOOSABLE_COLORS_WITH_COLORLESS: [ManaColor; 6] = [
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
        ManaColor::Colorless,
    ];

    /// What one colour-choice operation may name.
    pub(super) fn choosable_qualities(operation: ColorChoiceOperationDef) -> &'static [ManaColor] {
        match operation {
            ColorChoiceOperationDef::ProtectionFromChosenColor
            | ColorChoiceOperationDef::BecomesChosenColor => &Self::CHOOSABLE_COLORS,
            ColorChoiceOperationDef::ProtectionFromChosenColorOrColorless => {
                &Self::CHOOSABLE_COLORS_WITH_COLORLESS
            }
        }
    }

    /// Offers one colour of a run of "add one mana of any color for each ...".
    /// Each mana is named separately, so the run is answered one at a time
    /// and this re-queues itself until it is spent.
    pub(super) fn queue_chosen_color_mana(
        &mut self,
        controller: PlayerId,
        prototype: Mana,
        remaining: u16,
        choosable: ColorSet,
    ) {
        let colors = Self::chosen_mana_colors(choosable);
        if remaining == 0 || colors.is_empty() {
            return;
        }
        let options = colors
            .iter()
            .enumerate()
            .map(|(index, color)| DecisionOption {
                id: u32::try_from(index).expect("a colour list fits u32"),
                label: Self::color_label(*color).to_owned(),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::None,
            })
            .collect();
        self.queue_decision(
            controller,
            "Choose a color to add",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::ChosenColorMana {
                controller,
                prototype,
                remaining,
                choosable,
            },
        );
    }

    /// The colours a set admits, in the order the options are numbered.
    pub(super) fn chosen_mana_colors(choosable: ColorSet) -> Vec<ManaColor> {
        Self::CHOOSABLE_COLORS
            .into_iter()
            .filter(|color| choosable.contains(*color))
            .collect()
    }

    pub(super) const fn color_label(color: ManaColor) -> &'static str {
        match color {
            ManaColor::White => "White",
            ManaColor::Blue => "Blue",
            ManaColor::Black => "Black",
            ManaColor::Red => "Red",
            ManaColor::Green => "Green",
            ManaColor::Colorless => "Colorless",
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn queue_color_choice(
        &mut self,
        player: PlayerId,
        object: Box<StackObject>,
        context: EffectResolutionContext,
        scoped: ScopedEffect,
        targets: Vec<Target>,
        operation: ColorChoiceOperationDef,
        duration: ResolvedEffectDurationDef,
    ) {
        let options = Self::choosable_qualities(operation)
            .iter()
            .enumerate()
            .map(|(index, color)| DecisionOption {
                id: u32::try_from(index).expect("six qualities fit u32"),
                label: Self::color_label(*color).to_owned(),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::None,
            })
            .collect();
        self.queue_decision(
            player,
            "Choose a color",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::ChooseColor {
                object,
                context,
                scoped,
                targets,
                operation,
                duration,
            },
        );
    }
}
