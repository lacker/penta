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
    chosen_colors: crate::card::ColorSet,
    chosen_card_name: Option<String>,
    chosen_card_name_binding: Option<String>,
}

fn parse_observed_chosen_colors(value: Option<&Value>) -> Result<crate::card::ColorSet, String> {
    let Some(value) = value else {
        return Ok(crate::card::ColorSet::empty());
    };
    let colors = value.as_array().ok_or("chosenColors must be an array")?;
    let mut result = crate::card::ColorSet::empty();
    for value in colors {
        let color = value
            .as_str()
            .and_then(ManaColor::from_label)
            .filter(|color| color.color_index().is_some())
            .ok_or("chosenColors must contain colors")?;
        if result.contains(color) {
            return Err("chosenColors contains a duplicate color".into());
        }
        result = result.with(color);
    }
    Ok(result)
}
