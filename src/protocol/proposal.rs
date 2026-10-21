//! Typed proposals are parsed independently of legal-action discovery.
//! Parsing errors describe only caller-supplied fields, never hidden state.

mod actions;
mod values;

use serde::Serialize;
use serde_json::Value;

use crate::action::{
    ContributionKind, ContributionSymbol, ExplicitContribution, ExplicitFundingStep,
    ExplicitPayment,
};
use crate::{Action, Game, PlayerId};
use values::Input;

/// A stable, seat-safe refusal with a JSON field path.
#[derive(Clone, Debug, Serialize)]
pub struct ProposalError {
    pub code: String,
    pub path: String,
    pub message: String,
}

impl ProposalError {
    pub(crate) fn new(code: &'static str, path: &str, message: &str) -> Self {
        Self {
            code: code.into(),
            path: path.into(),
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ProposalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.path, self.message)
    }
}

impl std::error::Error for ProposalError {}

/// A chosen action with optional explicit funding. No alternative actions are built.
#[derive(Clone, Debug)]
pub struct ActionProposal {
    pub action: Action,
    pub payment: Option<ExplicitPayment>,
}

impl ActionProposal {
    /// Parses the `proposal-v1` input contract, including unknown-field rejection.
    ///
    /// # Errors
    /// Returns the malformed field, without consulting game state.
    pub fn parse(value: &Value) -> Result<Self, ProposalError> {
        let input = Input::new(value, "proposal", &["action", "payment"])?;
        Ok(Self {
            action: actions::parse(input.field("action"), "proposal.action")?,
            payment: input
                .optional("payment")
                .map(|value| payment(value, "proposal.payment"))
                .transpose()?,
        })
    }

    /// Validates and commits one proposal; rejected proposals leave the game unchanged.
    ///
    /// # Errors
    /// Rejects illegal actions or funding with seat-safe explanations.
    pub fn apply(self, game: &mut Game, player: PlayerId) -> Result<(), ProposalError> {
        if game.decision_player() != Some(player) && self.action != Action::Concede {
            return Err(ProposalError::new(
                "notYourDecision",
                "proposal.action",
                "This seat does not hold the current decision.",
            ));
        }
        let result = if let Some(payment) = &self.payment {
            game.apply_with_payment(player, self.action.clone(), payment)
        } else {
            game.apply(player, self.action.clone())
        };
        result.map_err(|_| {
            let reason =
                game.explain_proposal_rejection(player, &self.action, self.payment.as_ref());
            ProposalError {
                code: reason["code"].as_str().unwrap_or("illegalProposal").into(),
                path: format!("proposal.{}", reason["path"].as_str().unwrap_or("action")),
                message: reason["message"]
                    .as_str()
                    .unwrap_or("The proposal is not legal; no resources were spent.")
                    .into(),
            }
        })
    }
}

fn payment(value: &Value, path: &str) -> Result<ExplicitPayment, ProposalError> {
    let input = Input::new(
        value,
        path,
        &["funding", "announcements", "manaUnits", "contributions"],
    )?;
    let funding = input.list("funding", |value, path| {
        let step = Input::new(value, path, &["action", "manaUnits", "answers"])?;
        let action = actions::parse(step.field("action"), &format!("{path}.action"))?;
        if !matches!(action, Action::ActivateManaAbility { .. }) {
            return Err(ProposalError::new(
                "invalidField",
                &format!("{path}.action"),
                "Funding requires ActivateManaAbility.",
            ));
        }
        Ok(ExplicitFundingStep {
            action,
            mana_units: step
                .optional("manaUnits")
                .map(|_| step.numbers("manaUnits"))
                .transpose()?,
            answers: step.list("answers", values::numbers)?,
        })
    })?;
    Ok(ExplicitPayment {
        funding,
        announcements: input.list("announcements", values::numbers)?,
        mana_units: input
            .optional("manaUnits")
            .map(|_| input.numbers("manaUnits"))
            .transpose()?,
        contributions: input.list("contributions", |value, path| {
            let i = Input::new(value, path, &["source", "kind", "symbol"])?;
            let kind = match i.string("kind")? {
                "convoke" => ContributionKind::Convoke,
                "delve" => ContributionKind::Delve,
                "improvise" => ContributionKind::Improvise,
                "waterbend" => ContributionKind::Waterbend,
                _ => return Err(values::invalid(path, "Unknown contribution kind.")),
            };
            let symbol = match i.string("symbol")? {
                "generic" => ContributionSymbol::Generic,
                value => values::color(value, path)
                    .map(ContributionSymbol::Color)
                    .or_else(|_| {
                        actions::flexible(value, path).map(ContributionSymbol::Flexible)
                    })?,
            };
            Ok(ExplicitContribution {
                source: crate::GameObjectId(i.number("source")?),
                kind,
                symbol,
            })
        })?,
    })
}
