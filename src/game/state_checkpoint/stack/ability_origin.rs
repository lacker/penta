pub(super) fn parse_ability_origin(value: &Value) -> Result<AbilityOrigin, String> {
    match str_field(value, "kind")? {
        "printed" => Ok(AbilityOrigin::Printed {
            definition: card_definition_id_field(value, "definition")?,
            part: CardPartId(u8_field(value, "partId")?),
            ability: AbilityId(u8_field(value, "abilityId")?),
        }),
        "token" => Ok(AbilityOrigin::Token {
            part: CardPartId(u8_field(value, "partId")?),
            ability: AbilityId(u8_field(value, "abilityId")?),
        }),
        "emblem" => Ok(AbilityOrigin::Emblem {
            ability: AbilityId(u8_field(value, "abilityId")?),
        }),
        "intrinsicBasicLand" => Ok(AbilityOrigin::IntrinsicBasicLand(
            match str_field(value, "landType")? {
                "plains" => BasicLandType::Plains,
                "island" => BasicLandType::Island,
                "swamp" => BasicLandType::Swamp,
                "mountain" => BasicLandType::Mountain,
                "forest" => BasicLandType::Forest,
                other => return Err(format!("unknown intrinsic basic land type {other}")),
            },
        )),
        "intrinsicCounter" => {
            let counter = str_field(value, "counter")?;
            let kind = super::CounterKind::from_name(counter)
                .ok_or_else(|| format!("unknown intrinsic counter kind {counter}"))?;
            Ok(AbilityOrigin::IntrinsicCounter(kind))
        }
        "granted" => Ok(AbilityOrigin::Granted {
            source: GameObjectId(u32_field(value, "source")?),
            source_definition: card_definition_id_field(value, "sourceDefinition")?,
            source_part: CardPartId(u8_field(value, "sourcePartId")?),
            source_ability: AbilityId(u8_field(value, "sourceAbilityId")?),
            grant: GrantId(u8_field(value, "grantId")?),
        }),
        "tokenGranted" => Ok(AbilityOrigin::TokenGranted {
            source: GameObjectId(u32_field(value, "source")?),
            source_part: CardPartId(u8_field(value, "sourcePartId")?),
            source_ability: AbilityId(u8_field(value, "sourceAbilityId")?),
            grant: GrantId(u8_field(value, "grantId")?),
        }),
        "emblemGranted" => Ok(AbilityOrigin::EmblemGranted {
            source: GameObjectId(u32_field(value, "source")?),
            source_ability: AbilityId(u8_field(value, "sourceAbilityId")?),
            grant: GrantId(u8_field(value, "grantId")?),
        }),
        other => Err(format!("unknown ability origin kind {other}")),
    }
}
