//! A committed cost retains only authored-node locators, frozen repetition
//! counts, resource assignments, and the continuation's ordinary context.

use super::*;
use crate::game::cost_payment::{
    CostPaymentWindow, NamedPayment, PaymentAnswer, PaymentPart, PaymentPlan, PaymentQuestion,
};
use crate::game::state_checkpoint::model::PaymentAnswerSnapshot;
use crate::game::state_checkpoint::model_procedure::{
    CommittedPaymentSnapshot, NamedPaymentSnapshot, PaymentPartSnapshot,
};

pub(super) fn snapshot(
    game: &Game,
    viewer: PlayerId,
    window: &CostPaymentWindow,
    visible_rebindings: &[GameObjectId],
) -> Option<CommittedPaymentSnapshot> {
    if window.visibility() == crate::game::DecisionVisibility::Private && viewer != window.player {
        return None;
    }
    let crate::card::EffectDef::PayOr(definition) = window.definition.effect else {
        return None;
    };
    let costs = definition.payment.cost.subcosts();
    let plan = window.committing.as_ref()?;
    validate_remainder(game, window, plan).ok()?;
    Some(CommittedPaymentSnapshot {
        player: window.player.index(),
        continuation: effect_continuation_snapshot(game, viewer, &window.object, &window.context, window.definition, visible_rebindings)?,
        answers: window.answers.iter().map(|answer| match answer {
            PaymentAnswer::Choice(index) => PaymentAnswerSnapshot::Choice(*index),
            PaymentAnswer::Objects(ids) => PaymentAnswerSnapshot::Objects(ids.iter().map(|id| id.0).collect()),
        }).collect(),
        remaining: plan.parts.iter().map(|part| Some(PaymentPartSnapshot {
            cost: costs.iter().position(|cost| *cost == part.cost)?, times: part.times,
            objects: part.objects.iter().map(|id| id.0).collect(),
        })).collect::<Option<Vec<_>>>()?,
        named: plan.named.iter().map(|named| Some(NamedPaymentSnapshot {
            cost: costs.iter().position(|cost| matches!(cost, crate::card::CostDef::Named { mechanic, .. } if *mechanic == named.mechanic))?,
            repetitions: named.repetitions,
        })).collect::<Option<Vec<_>>>()?,
        mana_spent: plan.mana_spent.iter().map(|mana| super::super::mana_snapshot(&game.catalog, *mana)).collect(),
    })
}

pub(super) fn restore(
    game: &Game,
    snapshot: &CommittedPaymentSnapshot,
) -> Result<CostPaymentWindow, String> {
    use crate::card::{CostDef, EffectDef};
    let continuation = parse_effect_continuation(&snapshot.continuation, game)?;
    let EffectDef::PayOr(definition) = continuation.effect.effect else {
        return Err("committed payment locator is not a payment".into());
    };
    if !crate::game::cost_payment::uses_cost_payment_window(definition.payment.cost) {
        return Err("committed payment locator does not use a payment program".into());
    }
    let player = player_from_index(snapshot.player)?;
    if game.effect_players(
        definition.payment.payer,
        &continuation.object,
        &continuation.context,
        continuation.effect,
    ) != [player]
    {
        return Err("committed payment payer disagrees with its authored program".into());
    }
    let costs = definition.payment.cost.subcosts();
    let mut window = CostPaymentWindow {
        player,
        definition: continuation.effect,
        object: continuation.object,
        context: continuation.context,
        answers: snapshot
            .answers
            .iter()
            .map(|answer| match answer {
                PaymentAnswerSnapshot::Choice(index) => PaymentAnswer::Choice(*index),
                PaymentAnswerSnapshot::Objects(ids) => {
                    PaymentAnswer::Objects(ids.iter().copied().map(GameObjectId).collect())
                }
            })
            .collect(),
        chosen: Vec::new(),
        committing: None,
    };
    let mut plan = PaymentPlan::default();
    for part in &snapshot.remaining {
        let cost = *costs
            .get(part.cost)
            .ok_or("committed payment cost node is absent")?;
        if matches!(
            cost,
            CostDef::Named { .. }
                | CostDef::Repeat { .. }
                | CostDef::All(_)
                | CostDef::Choice(_)
                | CostDef::Mana(_)
                | CostDef::SnowMana(_)
        ) {
            return Err("committed payment node is not a remaining nonmana action".into());
        }
        let part = PaymentPart {
            cost,
            times: part.times,
            objects: part.objects.iter().copied().map(GameObjectId).collect(),
            mechanics: Vec::new(),
        };
        if let Some((_, quantity)) = game.payment_part_quantity(&window, &part) {
            if quantity
                .fixed_value()
                .is_some_and(|count| usize::from(count) != part.objects.len())
                || part
                    .objects
                    .iter()
                    .enumerate()
                    .any(|(index, id)| part.objects[..index].contains(id))
            {
                return Err(
                    "committed payment selection does not match its frozen quantity".into(),
                );
            }
        } else if !part.objects.is_empty() || Game::payment_scalar(&window, &part).is_none() {
            return Err("committed payment scalar node is invalid".into());
        }
        plan.parts.push_back(part);
    }
    for named in &snapshot.named {
        let Some(CostDef::Named { mechanic, .. }) = costs.get(named.cost) else {
            return Err("committed payment identity is not an authored named cost".into());
        };
        plan.named.push(NamedPayment {
            mechanic: *mechanic,
            repetitions: named.repetitions,
        });
    }
    plan.mana_spent = super::super::wire::parse_mana(&snapshot.mana_spent, &game.catalog)?;
    validate_remainder(game, &window, &plan)?;
    window.committing = Some(plan);
    Ok(window)
}

fn validate_remainder(
    game: &Game,
    window: &CostPaymentWindow,
    plan: &PaymentPlan,
) -> Result<(), String> {
    let Some(mut original) = game.evaluate_payment_program(window, false) else {
        return Err("committed payment answers do not describe its authored program".into());
    };
    if !matches!(original.question, PaymentQuestion::Confirm) || original.plan.named != plan.named {
        return Err("committed payment completion identities disagree with its program".into());
    }
    game.order_payment_actions(window, &mut original.plan);
    original.plan.parts.retain(|part| part.times != 0);
    let same = |left: &PaymentPart, right: &PaymentPart| {
        left.cost == right.cost && left.times == right.times && left.objects == right.objects
    };
    let suffix = plan.parts.is_empty()
        || original
            .plan
            .parts
            .iter()
            .enumerate()
            .any(|(index, first)| {
                if original.plan.parts.len() - index != plan.parts.len() {
                    return false;
                }
                let remaining = &plan.parts[0];
                if first.cost != remaining.cost
                    || remaining.times == 0
                    || remaining.times > first.times
                {
                    return false;
                }
                let consumed = usize::from(first.times - remaining.times)
                    * (first.objects.len() / usize::from(first.times));
                first.objects[consumed..] == remaining.objects
                    && original
                        .plan
                        .parts
                        .iter()
                        .skip(index + 1)
                        .zip(plan.parts.iter().skip(1))
                        .all(|(left, right)| same(left, right))
            });
    if !suffix {
        return Err(
            "committed payment remainder is not a suffix of its authored action plan".into(),
        );
    }
    Ok(())
}
