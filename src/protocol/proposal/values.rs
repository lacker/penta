use super::ProposalError;
use crate::{
    AbilityId, AbilityOrigin, CardPartId, GameObjectId, GrantId, ManaColor, ManaSplit, Target,
    TargetSelection, TargetSlotId,
};
use serde_json::{Map, Value};

pub(super) struct Input<'a> {
    value: &'a Map<String, Value>,
    path: &'a str,
}

impl<'a> Input<'a> {
    pub(super) fn new(
        value: &'a Value,
        path: &'a str,
        keys: &[&str],
    ) -> Result<Self, ProposalError> {
        let value = value
            .as_object()
            .ok_or_else(|| invalid(path, "Expected an object."))?;
        if let Some(key) = value.keys().find(|key| !keys.contains(&key.as_str())) {
            return Err(invalid(&format!("{path}.{key}"), "Unknown field."));
        }
        Ok(Self { value, path })
    }
    pub(super) fn field(&self, key: &str) -> &'a Value {
        self.value.get(key).unwrap_or(&Value::Null)
    }
    pub(super) fn optional(&self, key: &str) -> Option<&'a Value> {
        self.value.get(key).filter(|v| !v.is_null())
    }
    pub(super) fn string(&self, key: &str) -> Result<&'a str, ProposalError> {
        self.field(key)
            .as_str()
            .ok_or_else(|| invalid(&format!("{}.{key}", self.path), "Expected a string."))
    }
    pub(super) fn number<T: TryFrom<u64>>(&self, key: &str) -> Result<T, ProposalError> {
        number(self.field(key), &format!("{}.{key}", self.path))
    }
    pub(super) fn default_number<T: TryFrom<u64> + Default>(
        &self,
        key: &str,
    ) -> Result<T, ProposalError> {
        if self.optional(key).is_some() {
            self.number(key)
        } else {
            Ok(T::default())
        }
    }
    pub(super) fn list<T>(
        &self,
        key: &str,
        parse: impl Fn(&Value, &str) -> Result<T, ProposalError>,
    ) -> Result<Vec<T>, ProposalError> {
        self.optional(key).map_or(Ok(Vec::new()), |value| {
            list(value, &format!("{}.{key}", self.path), parse)
        })
    }
    pub(super) fn numbers<T: TryFrom<u64>>(&self, key: &str) -> Result<Vec<T>, ProposalError> {
        self.list(key, number)
    }
}

pub(super) fn invalid(path: &str, message: &str) -> ProposalError {
    ProposalError::new("invalidField", path, message)
}
pub(super) fn number<T: TryFrom<u64>>(value: &Value, path: &str) -> Result<T, ProposalError> {
    value
        .as_u64()
        .and_then(|n| T::try_from(n).ok())
        .ok_or_else(|| {
            invalid(
                path,
                "Expected an unsigned integer within the field's range.",
            )
        })
}
pub(super) fn list<T>(
    value: &Value,
    path: &str,
    parse: impl Fn(&Value, &str) -> Result<T, ProposalError>,
) -> Result<Vec<T>, ProposalError> {
    let values = value
        .as_array()
        .filter(|items| items.len() <= 4096)
        .ok_or_else(|| invalid(path, "Expected an array with at most 4096 entries."))?;
    values
        .iter()
        .enumerate()
        .map(|(i, v)| parse(v, &format!("{path}[{i}]")))
        .collect()
}
pub(super) fn numbers<T: TryFrom<u64>>(value: &Value, path: &str) -> Result<Vec<T>, ProposalError> {
    list(value, path, number)
}

pub(super) fn seat(value: &str, path: &str) -> Result<crate::PlayerId, ProposalError> {
    super::super::json_common::seat_by_name(value)
        .ok_or_else(|| invalid(path, "Expected p1 or p2."))
}
pub(super) fn color(value: &str, path: &str) -> Result<ManaColor, ProposalError> {
    match value {
        "white" => Ok(ManaColor::White),
        "blue" => Ok(ManaColor::Blue),
        "black" => Ok(ManaColor::Black),
        "red" => Ok(ManaColor::Red),
        "green" => Ok(ManaColor::Green),
        "colorless" => Ok(ManaColor::Colorless),
        _ => Err(invalid(path, "Unknown mana color.")),
    }
}
pub(super) fn split(value: &Value, path: &str) -> Result<ManaSplit, ProposalError> {
    let input = Input::new(
        value,
        path,
        &["white", "blue", "black", "red", "green", "colorless"],
    )?;
    let mut split = ManaSplit::empty();
    for (name, value) in input.value {
        split.add(color(name, path)?, number(value, path)?);
    }
    Ok(split)
}
pub(super) fn target(value: &Value, path: &str) -> Result<Target, ProposalError> {
    let kind = value["type"].as_str().unwrap_or("");
    let input = Input::new(
        value,
        path,
        if kind == "player" {
            &["type", "seat"]
        } else {
            &["type", "objectId"]
        },
    )?;
    Ok(match kind {
        "player" => Target::Player(seat(input.string("seat")?, path)?),
        "permanent" => Target::Permanent(GameObjectId(input.number("objectId")?)),
        "card" => Target::Card(GameObjectId(input.number("objectId")?)),
        "spell" => Target::Spell(GameObjectId(input.number("objectId")?)),
        _ => return Err(invalid(path, "Unknown target type.")),
    })
}
pub(super) fn selection(value: &Value, path: &str) -> Result<TargetSelection, ProposalError> {
    let input = Input::new(value, path, &["slotId", "targets", "amounts"])?;
    let slot = TargetSlotId(input.number("slotId")?);
    let targets = input.list("targets", target)?;
    let amounts = input.numbers::<u16>("amounts")?;
    if amounts.is_empty() {
        return Ok(TargetSelection::new(slot, targets));
    }
    if amounts.len() != targets.len() {
        return Err(invalid(path, "Each divided target needs one amount."));
    }
    Ok(TargetSelection::divided(slot, targets, amounts))
}

pub(super) fn origin(value: &Value, path: &str) -> Result<AbilityOrigin, ProposalError> {
    let kind = value["kind"].as_str().unwrap_or("");
    let keys: &[&str] = match kind {
        "printed" => &["kind", "definition", "partId", "abilityId"],
        "token" => &["kind", "partId", "abilityId"],
        "emblem" | "faceDown" => &["kind", "abilityId"],
        "intrinsicBasicLand" => &["kind", "landType"],
        "intrinsicCounter" => &["kind", "counter"],
        "granted" => &[
            "kind",
            "source",
            "sourceDefinition",
            "sourcePartId",
            "sourceAbilityId",
            "grantId",
        ],
        "tokenGranted" => &[
            "kind",
            "source",
            "sourcePartId",
            "sourceAbilityId",
            "grantId",
        ],
        "emblemGranted" | "faceDownGranted" => &["kind", "source", "sourceAbilityId", "grantId"],
        _ => return Err(invalid(path, "Unknown ability origin.")),
    };
    let i = Input::new(value, path, keys)?;
    let definition = |key| {
        serde_json::from_value(i.field(key).clone())
            .map_err(|_| invalid(path, "Invalid card definition UUID."))
    };
    Ok(match kind {
        "printed" => AbilityOrigin::Printed {
            definition: definition("definition")?,
            part: CardPartId(i.number("partId")?),
            ability: AbilityId(i.number("abilityId")?),
        },
        "token" => AbilityOrigin::Token {
            part: CardPartId(i.number("partId")?),
            ability: AbilityId(i.number("abilityId")?),
        },
        "emblem" => AbilityOrigin::Emblem {
            ability: AbilityId(i.number("abilityId")?),
        },
        "faceDown" => AbilityOrigin::FaceDown {
            ability: AbilityId(i.number("abilityId")?),
        },
        "intrinsicBasicLand" => AbilityOrigin::IntrinsicBasicLand(match i.string("landType")? {
            "plains" => crate::BasicLandType::Plains,
            "island" => crate::BasicLandType::Island,
            "swamp" => crate::BasicLandType::Swamp,
            "mountain" => crate::BasicLandType::Mountain,
            "forest" => crate::BasicLandType::Forest,
            _ => return Err(invalid(path, "Unknown basic land type.")),
        }),
        "intrinsicCounter" => AbilityOrigin::IntrinsicCounter(
            crate::CounterKind::from_name(i.string("counter")?)
                .ok_or_else(|| invalid(path, "Unknown intrinsic counter."))?,
        ),
        "granted" => AbilityOrigin::Granted {
            source: GameObjectId(i.number("source")?),
            source_definition: definition("sourceDefinition")?,
            source_part: CardPartId(i.number("sourcePartId")?),
            source_ability: AbilityId(i.number("sourceAbilityId")?),
            grant: GrantId(i.number("grantId")?),
        },
        "tokenGranted" => AbilityOrigin::TokenGranted {
            source: GameObjectId(i.number("source")?),
            source_part: CardPartId(i.number("sourcePartId")?),
            source_ability: AbilityId(i.number("sourceAbilityId")?),
            grant: GrantId(i.number("grantId")?),
        },
        "emblemGranted" => AbilityOrigin::EmblemGranted {
            source: GameObjectId(i.number("source")?),
            source_ability: AbilityId(i.number("sourceAbilityId")?),
            grant: GrantId(i.number("grantId")?),
        },
        "faceDownGranted" => AbilityOrigin::FaceDownGranted {
            source: GameObjectId(i.number("source")?),
            source_ability: AbilityId(i.number("sourceAbilityId")?),
            grant: GrantId(i.number("grantId")?),
        },
        _ => unreachable!(),
    })
}
