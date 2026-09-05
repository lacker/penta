// The choices a resolving effect stops to ask about a colour or a land type.
//
// Split out of `entry_replacements.rs` for the source-size budget, and split
// here because they are not replacements at all: each one queues a question
// and carries on where the answer arrives. Included textually, so the
// imports here are that module's.

impl Game {
    fn queue_optional_entry_replacement(
        &mut self,
        player: PlayerId,
        name: &str,
        context: ReplacementEffectContext,
        effect: ReplacementEffectDef,
    ) {
        self.queue_decision(
            player,
            format!("Apply the optional replacement for {name}?"),
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            Self::optional_entry_replacement_options(),
            DecisionContinuation::BattlefieldEntryOptional { context, effect },
        );
    }

    pub(super) fn optional_entry_replacement_options() -> Vec<DecisionOption> {
        [(0, "Decline"), (1, "Accept")]
            .into_iter()
            .map(|(id, label)| DecisionOption {
                id,
                label: label.into(),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::None,
            })
            .collect()
    }

    /// Resumes the exact typed replacement operation that was offered. The
    /// ability is already in the prospective event's applied set, so either
    /// answer continues without asking twice.
    pub(super) fn resume_optional_entry_replacement(
        &mut self,
        context: ReplacementEffectContext,
        effect: ReplacementEffectDef,
        options: &[u32],
    ) {
        let accepted = options.first().is_some_and(|option| *option == 1);
        if let Some(mut pending) = self.pending_events.pop_front() {
            if accepted {
                pending
                    .effects
                    .push(PendingReplacementEffect { context, effect });
            }
            self.pending_events.push_front(pending);
        }
        self.continue_pending_events();
    }

    pub(super) fn queue_basic_land_type_text_change(&mut self, player: PlayerId, target: Target) {
        let options = BasicLandType::ALL
            .into_iter()
            .flat_map(|from| {
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
            })
            .collect();
        self.queue_decision(
            player,
            "Replace one basic land type with another",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::BasicLandTypeTextChange { target },
        );
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
