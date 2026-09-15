// Inputs shared by public-observation and checkpoint permanent reconstruction.
#[allow(clippy::struct_excessive_bools)]
struct PermanentPresentation {
    controller: PlayerId,
    tapped: bool,
    damage: u16,
    attacking: bool,
    attack_defender: Option<AttackDefender>,
    blocked: bool,
    blocking: Vec<GameObjectId>,
    blocking_this_combat: bool,
    attacking_band: Option<u8>,
    activated_loyalty_this_turn: bool,
    chosen_creature_type: Option<String>,
    chosen_basic_land_type: Option<BasicLandType>,
    chosen_basic_land_type_substitution: Option<(BasicLandType, BasicLandType)>,
    chosen_color: Option<ManaColor>,
    chosen_colors: std::collections::BTreeMap<String, crate::card::ColorSet>,
    chosen_card_name: Option<String>,
    chosen_card_name_binding: Option<String>,
}

fn parse_observed_chosen_colors(value: Option<&Value>) -> Result<std::collections::BTreeMap<String, crate::card::ColorSet>, String> {
    let mut result = std::collections::BTreeMap::new();
    let Some(value) = value else { return Ok(result); };
    for (binding, colors) in value.as_object().ok_or("chosenColors must be a binding map")? {
        if binding.trim().is_empty() { return Err("chosenColors requires named bindings".into()); }
        let mut chosen = crate::card::ColorSet::empty();
        for value in colors.as_array().ok_or("chosenColors values must be arrays")? {
            let color = value.as_str().and_then(ManaColor::from_label)
                .filter(|color| color.color_index().is_some())
                .ok_or("chosenColors must contain colors")?;
            if chosen.contains(color) { return Err("chosenColors contains a duplicate color".into()); }
            chosen = chosen.with(color);
        }
        if chosen.is_colorless() { return Err("chosenColors bindings cannot be empty".into()); }
        result.insert(binding.clone(), chosen);
    }
    Ok(result)
}
