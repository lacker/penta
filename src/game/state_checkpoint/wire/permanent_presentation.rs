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
    chosen_color: Option<ManaColor>,
    chosen_card_name: Option<String>,
    chosen_card_name_binding: Option<String>,
}
