/// Answers every pending decision until the stack is quiet, taking the named
/// amount or number of options from any prompt containing `prompt` and the
/// smallest legal answer everywhere else. Tetravus puts two triggers on the
/// stack at once, so the test cannot assume which one is asked about first.
fn answer_upkeep(game: &mut Game, prompt: &str, take: usize) -> Vec<usize> {
    let mut offered = Vec::new();
    for _ in 0..16 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            break;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let matches_prompt = decision
                .prompt
                .to_ascii_lowercase()
                .contains(&prompt.to_ascii_lowercase());
            let wanted = if matches_prompt {
                offered.push(decision.options.len());
                take
            } else {
                decision.minimum
            };
            let options = if matches_prompt
                && decision
                    .options
                    .first()
                    .is_some_and(|option| option.label == "Decline")
            {
                vec![u32::try_from(wanted).expect("the test amount fits")]
            } else {
                decision
                    .options
                    .iter()
                    .map(|option| option.id)
                    .take(wanted.max(decision.minimum))
                    .collect::<Vec<_>>()
            };
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .unwrap();
            continue;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    offered
}
