use penta::PlayerId;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::seat_name;

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub(super) enum Condition {
    NextStep {
        step: Step,
        #[serde(rename = "activeSeat", default)]
        active_seat: ActiveSeat,
    },
    OpponentActs {},
    OpponentCastsSpell {},
    NonManaAction {},
}

#[derive(Clone, Copy, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum ActiveSeat {
    #[default]
    Either,
    #[serde(rename = "self")]
    Own,
    Opponent,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub(super) enum Step {
    Upkeep,
    Draw,
    PrecombatMain,
    BeginningOfCombat,
    DeclareAttackers,
    DeclareBlockers,
    CombatDamage,
    EndOfCombat,
    PostcombatMain,
    End,
    Cleanup,
}

impl Condition {
    pub(super) const fn needs_discovery(&self) -> bool {
        matches!(self, Self::NonManaAction {})
    }
    pub(super) fn matches_event(&self, seat: PlayerId, event: &Value) -> bool {
        match self {
            Self::NextStep { step, active_seat } => {
                event["type"] == "StepChanged"
                    && event["step"] == serde_json::to_value(step).expect("step serializes")
                    && match active_seat {
                        ActiveSeat::Either => true,
                        ActiveSeat::Own => event["activePlayer"] == seat_name(seat),
                        ActiveSeat::Opponent => event["activePlayer"] == seat_name(seat.opponent()),
                    }
            }
            Self::OpponentCastsSpell {} => {
                event["seat"] == seat_name(seat.opponent()) && event["type"] == "SpellCast"
            }
            Self::OpponentActs {} => {
                event["seat"] == seat_name(seat.opponent())
                    && matches!(
                        event["type"].as_str(),
                        Some(
                            "SpellCast"
                                | "AbilityActivated"
                                | "LandPlayed"
                                | "ManaAdded"
                                | "AttackDeclared"
                                | "BlockDeclared"
                        )
                    )
            }
            Self::NonManaAction {} => false,
        }
    }

    pub(super) fn matches_menu(&self, observation: &Value) -> bool {
        matches!(self, Self::NonManaAction {})
            && (observation["nonManaActionAvailable"] == true
                || observation["legalActions"]
                    .as_array()
                    .is_some_and(|actions| {
                        actions.iter().any(|action| {
                            !matches!(
                                action["type"].as_str(),
                                Some("PassPriority" | "ActivateManaAbility")
                            )
                        })
                    }))
    }
}
