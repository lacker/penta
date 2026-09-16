pub(super) fn parse_cast_signature(value: &Value) -> Result<CastSignature, String> {
    let form_value = field(value, "form")?;
    let form = match str_field(form_value, "kind")? {
        "part" => SpellForm::Part(CardPartId(
            u8::try_from(usize_field(form_value, "partId")?).map_err(|_| "part id too large")?,
        )),
        "combined" => SpellForm::Combined(
            array(field(form_value, "partIds")?)?
                .iter()
                .map(|part| read_u8(part).map(CardPartId))
                .collect::<Result<Vec<_>, _>>()?,
        ),
        other => return Err(format!("unknown spell form {other}")),
    };
    let alternative = value
        .get("alternativeCostId")
        .filter(|v| !v.is_null())
        .map(|v| read_u8(v).map(AlternativeCostId))
        .transpose()?;
    let additional = array(field(value, "additionalCostIds")?)?
        .iter()
        .map(|v| read_u8(v).map(AdditionalCostId))
        .collect::<Result<Vec<_>, _>>()?;
    let modes = array(field(value, "modeIds")?)?
        .iter()
        .map(|v| read_u8(v).map(ModeId))
        .collect::<Result<Vec<_>, _>>()?;
    let selections = array(field(value, "targetSelections")?)?
        .iter()
        .map(parse_target_selection)
        .collect::<Result<Vec<_>, _>>()?;
    // Absent from every cast written before splice existed, all of which
    // spliced nothing.
    let spliced = match value.get("splicedCards") {
        Some(value) => array(value)?
            .iter()
            .map(|card| {
                read_u32(card)
                    .map(GameObjectId)
                    .map_err(|_| "spliced card id".to_owned())
            })
            .collect::<Result<Vec<_>, _>>()?,
        None => Vec::new(),
    };
    let choices = CastChoices::new(PlayOptionId(
        u8::try_from(usize_field(value, "playOptionId")?).map_err(|_| "play option too large")?,
    ))
    .with_modes(modes)
    .with_costs(
        CostConfiguration::new(alternative, additional.clone())
            .with_permission_source(
                value
                    .get("permissionSource")
                    .filter(|v| !v.is_null())
                    .map(|v| read_u32(v).map(GameObjectId))
                    .transpose()?,
            )
            .with_chosen_creature_type(parse_chosen_creature_type(
                value.get("chosenCreatureType").and_then(Value::as_str),
            )?),
    )
    .with_x(u16::try_from(usize_field(value, "x")?).map_err(|_| "x too large")?)
    .with_targets(selections)
    .with_spliced(spliced);
    Ok(CastSignature::from_validated_choices(form, choices))
}

pub(super) fn parse_target_selection(value: &Value) -> Result<TargetSelection, String> {
    let slot = TargetSlotId(
        u8::try_from(usize_field(value, "slotId")?).map_err(|_| "target slot too large")?,
    );
    let targets = array(field(value, "targets")?)?
        .iter()
        .map(parse_target)
        .collect::<Result<Vec<_>, _>>()?;
    let amounts = array(field(value, "amounts")?)?
        .iter()
        .map(read_u16)
        .collect::<Result<Vec<_>, _>>()?;
    if amounts.is_empty() {
        Ok(TargetSelection::new(slot, targets))
    } else if amounts.len() == targets.len() {
        Ok(TargetSelection::divided(slot, targets, amounts))
    } else {
        Err("divided target amounts do not match targets".into())
    }
}

pub(super) fn parse_target(value: &Value) -> Result<Target, String> {
    match str_field(value, "type")? {
        "player" => Ok(Target::Player(seat_value(field(value, "seat")?)?)),
        "card" => Ok(Target::Card(PublicGameObjectId(u32_field(
            value, "objectId",
        )?))),
        "permanent" => Ok(Target::Permanent(PublicGameObjectId(u32_field(
            value, "objectId",
        )?))),
        "spell" => Ok(Target::Spell(PublicGameObjectId(u32_field(
            value, "objectId",
        )?))),
        other => Err(format!("unknown target type {other}")),
    }
}

pub(super) fn parse_chosen_creature_type(
    name: Option<&str>,
) -> Result<Option<crate::card::Subtype>, String> {
    name.map(|name| {
        crate::card::Subtype::from_name(name)
            .filter(|subtype| subtype.in_family(crate::card::SubtypeFamily::Creature))
            .ok_or_else(|| "invalid chosen creature type".into())
    })
    .transpose()
}
