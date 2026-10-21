use super::{
    ProposalError,
    values::{self, Input, invalid},
};
use crate::{
    Action, AlternativeAbilityCost, AttackDefender, CardPartId, CombatDamageAssignment,
    GameObjectId, PlayOptionId,
};
use serde_json::{Map, Value};

#[allow(clippy::too_many_lines)] // One exhaustive wire-tag dispatch, parallel to action_json.
pub(super) fn parse(value: &Value, path: &str) -> Result<Action, ProposalError> {
    let kind = value["type"]
        .as_str()
        .ok_or_else(|| invalid(path, "An action needs a type."))?;
    let keys: &[&str] = match kind {
        "KeepHand"
        | "TakeMulligan"
        | "PassPriority"
        | "BeginPayment"
        | "Concede"
        | "FinishDeclaringAttackers"
        | "FinishDeclaringBlockers" => &["type"],
        "BottomCards" | "DiscardCards" => &["type", "cards"],
        "ChooseDecision" => &["type", "decision", "options"],
        "CancelDecision" => &["type", "decision"],
        "ChooseUntap" => &["type", "permanents"],
        "PlayLand" => &["type", "card", "playOptionId"],
        "CastSpell" => &["type", "card", "choices", "costObjects"],
        "ActivateManaAbility" => &[
            "type",
            "source",
            "ability",
            "color",
            "countersRemoved",
            "costObject",
            "combination",
            "triggeredManaChoices",
        ],
        "ActivateAbility" => &[
            "type",
            "source",
            "ability",
            "targetSelections",
            "costObjects",
            "x",
            "modeIds",
            "manaPayment",
            "alternativeCost",
        ],
        "TurnFaceUp" => &["type", "permanent"],
        "Foretell" | "Plot" | "TakeCompanion" => &["type", "card"],
        "Suspend" => &["type", "card", "ability", "x"],
        "UnlockDoor" => &["type", "room", "door"],
        "DeclareAttacker" => &["type", "attacker", "defender"],
        "ExertAttacker" => &["type", "attacker"],
        "BandAttackers" => &["type", "first", "second"],
        "DeclareBlocker" => &["type", "blocker", "attacker"],
        "AssignCombatDamage" => &["type", "attacker", "assignments"],
        _ => return Err(invalid(path, "Unknown action type.")),
    };
    let i = Input::new(value, path, keys)?;
    let object = |key| i.number(key).map(GameObjectId);
    let objects = |key| {
        i.numbers(key)
            .map(|ids: Vec<u32>| ids.into_iter().map(GameObjectId).collect())
    };
    Ok(match kind {
        "KeepHand" => Action::KeepHand,
        "TakeMulligan" => Action::TakeMulligan,
        "PassPriority" => Action::PassPriority,
        "BeginPayment" => Action::BeginPayment,
        "Concede" => Action::Concede,
        "FinishDeclaringAttackers" => Action::FinishDeclaringAttackers,
        "FinishDeclaringBlockers" => Action::FinishDeclaringBlockers,
        "BottomCards" => Action::BottomCards {
            cards: objects("cards")?,
        },
        "DiscardCards" => Action::DiscardCards {
            cards: objects("cards")?,
        },
        "ChooseUntap" => Action::ChooseUntap {
            permanents: objects("permanents")?,
        },
        "ChooseDecision" => Action::ChooseDecision {
            decision: i.number("decision")?,
            options: i.numbers("options")?,
        },
        "CancelDecision" => Action::CancelDecision {
            decision: i.number("decision")?,
        },
        "PlayLand" => Action::PlayLand {
            card: object("card")?,
            option: PlayOptionId(i.default_number("playOptionId")?),
        },
        "CastSpell" => Action::CastSpell {
            card: object("card")?,
            choices: cast(i.field("choices"), &format!("{path}.choices"))?,
            sacrifices: objects("costObjects")?,
        },
        "ActivateManaAbility" => Action::ActivateManaAbility {
            source: object("source")?,
            ability: values::origin(i.field("ability"), &format!("{path}.ability"))?,
            color: mana_color(&i, path)?,
            counters_removed: i
                .optional("countersRemoved")
                .map(|_| i.number("countersRemoved"))
                .transpose()?,
            cost_object: i
                .optional("costObject")
                .map(|_| object("costObject"))
                .transpose()?,
            combination: i
                .optional("combination")
                .map(|v| values::split(v, &format!("{path}.combination")))
                .transpose()?,
            triggered_mana: i
                .optional("triggeredManaChoices")
                .map(|_| i.list("triggeredManaChoices", values::split))
                .transpose()?,
        },
        "ActivateAbility" => activation(&i, path)?,
        "TurnFaceUp" => Action::TurnFaceUp {
            permanent: object("permanent")?,
        },
        "Foretell" => Action::Foretell {
            card: object("card")?,
        },
        "Plot" => Action::Plot {
            card: object("card")?,
        },
        "TakeCompanion" => Action::TakeCompanion {
            card: object("card")?,
        },
        "Suspend" => Action::Suspend {
            card: object("card")?,
            ability: values::origin(i.field("ability"), &format!("{path}.ability"))?,
            x: i.default_number("x")?,
        },
        "UnlockDoor" => Action::UnlockDoor {
            room: object("room")?,
            door: CardPartId(i.number("door")?),
        },
        "DeclareAttacker" => Action::DeclareAttacker {
            attacker: object("attacker")?,
            defender: defender(i.field("defender"), &format!("{path}.defender"))?,
        },
        "ExertAttacker" => Action::ExertAttacker {
            attacker: object("attacker")?,
        },
        "BandAttackers" => Action::BandAttackers {
            first: object("first")?,
            second: object("second")?,
        },
        "DeclareBlocker" => Action::DeclareBlocker {
            blocker: object("blocker")?,
            attacker: object("attacker")?,
        },
        "AssignCombatDamage" => Action::AssignCombatDamage {
            attacker: object("attacker")?,
            assignments: i.list("assignments", |v, path| {
                let a = Input::new(v, path, &["recipient", "amount"])?;
                Ok(CombatDamageAssignment {
                    recipient: values::target(a.field("recipient"), &format!("{path}.recipient"))?,
                    amount: a.number("amount")?,
                })
            })?,
        },
        _ => unreachable!(),
    })
}

fn mana_color(input: &Input<'_>, path: &str) -> Result<crate::ManaColor, ProposalError> {
    let color = input
        .optional("color")
        .map(|_| values::color(input.string("color")?, &format!("{path}.color")))
        .transpose()?;
    if let Some(combination) = input.optional("combination") {
        // The internal action keeps a canonical representative color alongside
        // the split. A caller chooses the split, not that redundant identity.
        return values::split(combination, &format!("{path}.combination"))?
            .iter()
            .next()
            .map(|(color, _)| color)
            .ok_or_else(|| {
                invalid(
                    &format!("{path}.combination"),
                    "A combination must produce some mana.",
                )
            });
    }
    color.ok_or_else(|| {
        invalid(
            &format!("{path}.color"),
            "Provide color or a mana combination.",
        )
    })
}

fn defender(value: &Value, path: &str) -> Result<AttackDefender, ProposalError> {
    match value["type"].as_str() {
        Some("player") => {
            let i = Input::new(value, path, &["type", "seat"])?;
            Ok(AttackDefender::Player(values::seat(
                i.string("seat")?,
                path,
            )?))
        }
        Some("planeswalker") => {
            let i = Input::new(value, path, &["type", "objectId"])?;
            Ok(AttackDefender::Planeswalker(GameObjectId(
                i.number("objectId")?,
            )))
        }
        _ => Err(invalid(path, "Expected a player or planeswalker defender.")),
    }
}

fn activation(i: &Input<'_>, path: &str) -> Result<Action, ProposalError> {
    let source = GameObjectId(i.number("source")?);
    let ability = values::origin(i.field("ability"), &format!("{path}.ability"))?;
    let targets = i.list("targetSelections", values::selection)?;
    let cost_objects = i
        .numbers::<u32>("costObjects")?
        .into_iter()
        .map(GameObjectId)
        .collect();
    let x = i.default_number("x")?;
    let modes = i
        .numbers::<u8>("modeIds")?
        .into_iter()
        .map(crate::ModeId)
        .collect();
    let mana_payment = i
        .optional("manaPayment")
        .map(|v| mana_payment(v, &format!("{path}.manaPayment")).map(Box::new))
        .transpose()?;
    if let Some(value) = i.optional("alternativeCost") {
        let path = format!("{path}.alternativeCost");
        let cost = Input::new(value, &path, &["source", "ability"])?;
        Ok(Action::ActivateAbilityWithAlternativeCost {
            cost: AlternativeAbilityCost {
                source: GameObjectId(cost.number("source")?),
                ability: values::origin(cost.field("ability"), &path)?,
            },
            source,
            ability,
            targets,
            cost_objects,
            x,
            modes,
            mana_payment,
        })
    } else {
        Ok(Action::ActivateAbility {
            source,
            ability,
            targets,
            cost_objects,
            x,
            modes,
            mana_payment,
        })
    }
}

fn cast(value: &Value, path: &str) -> Result<crate::CastChoices, ProposalError> {
    let empty = Value::Object(Map::default());
    let value = if value.is_null() { &empty } else { value };
    let i = Input::new(
        value,
        path,
        &[
            "playOptionId",
            "modeIds",
            "targetSelections",
            "x",
            "alternativeCostId",
            "additionalCostIds",
            "permissionSource",
            "chosenCreatureType",
            "splicedCards",
            "manaPayment",
        ],
    )?;
    let alternative = i
        .optional("alternativeCostId")
        .map(|_| i.number("alternativeCostId").map(crate::AlternativeCostId))
        .transpose()?;
    let additional = i
        .numbers::<u8>("additionalCostIds")?
        .into_iter()
        .map(crate::AdditionalCostId)
        .collect();
    let permission = i
        .optional("permissionSource")
        .map(|_| i.number("permissionSource").map(GameObjectId))
        .transpose()?;
    let creature_type = i
        .optional("chosenCreatureType")
        .map(|_| {
            crate::card::Subtype::from_name(i.string("chosenCreatureType")?)
                .ok_or_else(|| invalid(path, "Unknown creature type."))
        })
        .transpose()?;
    let mut choices = crate::CastChoices::new(PlayOptionId(i.default_number("playOptionId")?))
        .with_modes(
            i.numbers::<u8>("modeIds")?
                .into_iter()
                .map(crate::ModeId)
                .collect(),
        )
        .with_costs(
            crate::CostConfiguration::new(alternative, additional)
                .with_permission_source(permission)
                .with_chosen_creature_type(creature_type),
        )
        .with_targets(i.list("targetSelections", values::selection)?)
        .with_x(i.default_number("x")?)
        .with_spliced(
            i.numbers::<u32>("splicedCards")?
                .into_iter()
                .map(GameObjectId)
                .collect(),
        );
    if let Some(value) = i.optional("manaPayment") {
        choices = choices.with_mana_payment(mana_payment(value, &format!("{path}.manaPayment"))?);
    }
    Ok(choices)
}

pub(super) fn flexible(
    value: &str,
    path: &str,
) -> Result<crate::FlexibleManaSymbol, ProposalError> {
    crate::FlexibleManaSymbol::ALL
        .into_iter()
        .find(|symbol| symbol.symbol() == value)
        .ok_or_else(|| invalid(path, "Unknown flexible mana symbol."))
}

fn mana_payment(value: &Value, path: &str) -> Result<crate::ManaPaymentChoice, ProposalError> {
    Ok(crate::ManaPaymentChoice::new(values::list(
        value,
        path,
        |v, path| {
            let i = Input::new(v, path, &["symbol", "count"])?;
            Ok(crate::FlexibleManaPayment::new(
                flexible(i.string("symbol")?, path)?,
                i.number("count")?,
            ))
        },
    )?))
}
