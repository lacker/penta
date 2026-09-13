use crate::Action;
use crate::game::payment::payment_action_object;
use crate::game::payment::{funding::{FundingStep, PaymentDraft}, state::{PaymentDecision, PaymentTarget}, BoundManaPayment};

fn explicit_payment_snapshot(game: &Game, viewer: PlayerId, payment: &PaymentDecision) -> Option<DecisionContinuationSnapshot> {
    match payment {
        PaymentDecision::Funding(draft) => return Some(DecisionContinuationSnapshot::ExplicitFunding { draft: payment_draft_snapshot(game, viewer, draft)? }),
        PaymentDecision::Mana { target: PaymentTarget::Draft(draft), selected, .. } => return Some(DecisionContinuationSnapshot::ExplicitDraftMana {
            draft: payment_draft_snapshot(game, viewer, draft)?, action: None, units: selected.clone(),
        }),
        PaymentDecision::Mana { target: PaymentTarget::Funding { draft, action }, selected, .. } => return Some(DecisionContinuationSnapshot::ExplicitDraftMana {
            draft: payment_draft_snapshot(game, viewer, draft)?,
            action: Some(game.funding_candidates(draft)?.iter().position(|candidate| candidate == action.as_ref())?),
            units: selected.clone(),
        }),
        _ => {}
    }
    Some({

            let units = match payment { PaymentDecision::Mana { selected, .. } => selected.clone(), _ => Vec::new() };
            let (player, action, resume, effect_choice) = match payment {
                PaymentDecision::Funding(_) => unreachable!("funding snapshot handled above"),
                PaymentDecision::Operation { player, resume, .. } => (*player, None, resume.as_deref(), None),
                PaymentDecision::Mana { target, obligation, .. } => match target {
                    PaymentTarget::Draft(_) | PaymentTarget::Funding { .. } => unreachable!("draft snapshot handled above"),
                    PaymentTarget::Action { action, resume } => (obligation.player, Some(action), resume.as_deref(), None),
                    PaymentTarget::Effect { pending, answered, .. } => (obligation.player, None, Some(pending.as_ref()), Some(answered.clone())),
                },
            };
            if viewer != player { return None; }
            let action = match action {
                Some(action) => Some(game.manual_payment_actions_in(player, resume).iter().position(|candidate| candidate == action.as_ref())?),
                None => None,
            };
            let resume = match resume {
                Some(pending) => Some(Box::new(super::model::ExplicitPaymentResumeSnapshot {
                    observation: crate::protocol::decision_json(&game.catalog, &pending.observation),
                    state: Box::new(decision_snapshot(game, viewer, pending)?),
                })),
                None => None,
            };
            DecisionContinuationSnapshot::ExplicitPayment { player: player.index(), x: match payment { PaymentDecision::Operation { x, .. } => *x, _ => 0 }, action, resume, effect_choice, units, allocations: match payment {
                PaymentDecision::Mana { target: PaymentTarget::Effect { allocations, .. }, .. } => allocations.iter().map(|payment| payment.units.clone()).collect(),
                _ => Vec::new(),
            } }
    })
}

fn payment_draft_snapshot(game: &Game, viewer: PlayerId, draft: &PaymentDraft) -> Option<super::model::PaymentDraftSnapshot> {
    let source = payment_action_object(&draft.action)?;
    if viewer != draft.player && !game.battlefield.iter().any(|p| p.card.id == source) { return None; }
    let x = match draft.action.as_ref() { Action::CastSpell { choices, .. } => choices.x(), Action::ActivateAbility { x, .. } => *x, _ => 0 };
    let action = game.manual_payment_actions_at_x(draft.player, draft.resume.as_deref(), x).iter().filter(|action| payment_action_object(action) == Some(source)).position(|a| a == draft.action.as_ref())?;
    let mut prefix = PaymentDraft { player: draft.player, action: draft.action.clone(), funding: Vec::new(), contributions: Vec::new(), announcements: draft.announcements.clone(), resume: draft.resume.clone() };
    let mut funding = Vec::new();
    for step in &draft.funding {
        let step_source = payment_action_object(&step.action)?;
        if viewer != draft.player && !game.battlefield.iter().any(|p| p.card.id == step_source) { return None; }
        let index = game.funding_candidates(&prefix)?.iter().filter(|action| payment_action_object(action) == Some(step_source)).position(|a| a == &step.action)?;
        funding.push(super::model::FundingStepSnapshot { action: index, source: step_source.0, mana: step.mana.as_ref().map(|mana| mana.units.clone()), answers: step.answers.clone() });
        prefix.funding.push(step.clone());
    }
    let mut contributions = Vec::new();
    for contribution in &draft.contributions {
        let index = game.contribution_candidates(&prefix)?.iter().filter(|c| c.source == contribution.source).position(|c| c == contribution)?;
        contributions.push((contribution.source.0, index));
        prefix.contributions.push(*contribution);
    }
    let resume = if let Some(pending) = &draft.resume { Some(Box::new(super::model::ExplicitPaymentResumeSnapshot {
        observation: crate::protocol::decision_json(&game.catalog, &pending.observation),
        state: Box::new(decision_snapshot(game, viewer, pending)?),
    })) } else { None };
    Some(super::model::PaymentDraftSnapshot { player: draft.player.index(), action, source: source.0, x, funding, contributions, announcements: draft.announcements.clone(), resume })
}

fn parse_payment_draft(game: &Game, snapshot: &super::model::PaymentDraftSnapshot, hidden: &Value) -> Result<PaymentDraft, String> {
    let player = player(snapshot.player)?;
    let resume = snapshot.resume.as_ref().map(|snapshot| {
        parse_pending_decision(&serde_json::json!({ "decision": snapshot.observation }), Some(&snapshot.state), hidden, game)?
            .map(Box::new).ok_or_else(|| "payment draft has no enclosing offer".to_owned())
    }).transpose()?;
    if resume.as_ref().is_some_and(|pending| pending.continuation.cast_offer().is_none()) { return Err("payment draft requires a cast offer".into()); }
    let action = game.manual_payment_actions_at_x(player, resume.as_deref(), snapshot.x).into_iter().filter(|a| payment_action_object(a) == Some(GameObjectId(snapshot.source))).nth(snapshot.action).ok_or("payment draft operation is unavailable")?;
    let mut draft = PaymentDraft { player, action: Box::new(action), funding: Vec::new(), contributions: Vec::new(), announcements: snapshot.announcements.clone(), resume };
    for step in &snapshot.funding {
        let action = game.funding_candidates(&draft).and_then(|actions| actions.into_iter().filter(|a| payment_action_object(a) == Some(GameObjectId(step.source))).nth(step.action)).ok_or("payment draft funding step is unavailable")?;
        draft.funding.push(FundingStep { action, mana: step.mana.as_ref().map(|units| BoundManaPayment { units: units.clone() }), answers: step.answers.clone() });
        game.preview_funding(&draft).ok_or("payment draft funding step is invalid")?;
    }
    for (source, index) in &snapshot.contributions {
        let contribution = game.contribution_candidates(&draft).and_then(|choices| choices.into_iter().filter(|c| c.source.0 == *source).nth(*index)).ok_or("payment contribution is unavailable")?;
        draft.contributions.push(contribution);
    }
    game.prepare_payment_draft(&draft).ok_or("payment draft cannot be prepared")?;
    Ok(draft)
}

/// Rebuild the live offer from its continuation; snapshot labels, bounds, and
/// option IDs cannot grant choices that the payment executor did not offer.
fn validate_explicit_payment_observation(game: &Game, observation: &DecisionObservation,
    payment: &PaymentDecision) -> Result<(), String>
{
    let mut view = game.clone();
    view.pending_decisions.clear();
    match payment.clone() {
        PaymentDecision::Operation { player, x, resume } => {
            if let Some(pending) = resume { view.pending_decisions.push(*pending); }
            view.begin_explicit_payment_at_x(player, x);
        }
        PaymentDecision::Funding(draft) => view.queue_funding(draft),
        PaymentDecision::Mana { target, obligation, selected } => {
            if !view.exact_payment_view(&target).is_some_and(|pool| selected.is_empty() || pool.mana_selection_can_complete(&obligation, &selected)) {
                return Err("explicit payment selection cannot be completed".into());
            }
            view.queue_exact_mana(target, obligation, selected);
        }
    }
    let mut expected = view.pending_decisions.remove(0).observation;
    expected.id = observation.id;
    if expected != *observation { return Err("explicit payment observation disagrees with the legal choices".into()); }
    Ok(())
}
