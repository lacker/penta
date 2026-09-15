use super::{
    DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone,
    Game, ManaColor, PlayerId, ReplacementEffectContext,
};

impl Game {
    pub(in crate::game) fn entry_color_options() -> Vec<DecisionOption> {
        ManaColor::COLORS
            .into_iter()
            .enumerate()
            .map(|(index, color)| DecisionOption {
                id: u32::try_from(index).expect("five color indices fit u32"),
                label: color.label().into(),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::None,
            })
            .collect()
    }

    pub(super) fn queue_entry_colors_choice(
        &mut self,
        player: PlayerId,
        context: ReplacementEffectContext,
        count: u8,
    ) {
        let amount = usize::from(count);
        self.queue_decision(
            player,
            format!("Choose {count} different colors"),
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            amount..=amount,
            false,
            Self::entry_color_options(),
            DecisionContinuation::BattlefieldEntryColorsChoice { context, count },
        );
    }
}
