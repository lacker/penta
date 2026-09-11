// The permanent checkpoint shape, included into `model.rs` to share its
// snapshot vocabulary and visibility.

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_excessive_bools)]
pub(super) struct PermanentSnapshot {
    pub(super) object_id: u32,
    pub(super) owner: usize,
    pub(super) object_kind: ObjectKindSnapshot,
    /// The authored token characteristics originally minted for this permanent.
    /// A token copy legitimately has none because its single-faced copy effect
    /// or frozen double-faced values supply its copiable characteristics;
    /// `object_kind` still records that it is a token.
    pub(super) token_characteristics: Option<TokenCharacteristicsLocator>,
    /// The size an X/X token came out at. The locator above names the
    /// authored token, which carries the two amounts rather than a size;
    /// these are the numbers they came to, which is a copiable value of the
    /// token and cannot be worked out again from the board. Additive, and
    /// absent for every token whose size is printed on it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) token_stats: Option<[i16; 2]>,
    /// Both intrinsic faces of a token created as a copy of a double-faced
    /// permanent. Additive because older checkpoints could not represent this
    /// state faithfully at all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) double_faced_token_copy: Option<DoubleFacedCopiableCharacteristicsSnapshot>,
    /// Copiable values supplied by the rule or effect that made this
    /// permanent face down. `None` means face up.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) face_down: Option<FaceDownCharacteristicsSnapshot>,
    /// Whether the face-down mechanism grants a mana-cost turn-up permission.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) turn_up_for_mana_cost: bool,
    pub(super) presented_part_id: u8,
    pub(super) timestamp: u64,
    pub(super) entered_controller_turn: u32,
    /// The game turn this permanent entered, for the clauses that ask about
    /// the turn itself rather than about its controller's turn count.
    /// Additive: a checkpoint written before it existed restores a permanent
    /// that entered on turn zero, which is what one that has been there all
    /// along would say anyway.
    #[serde(default, skip_serializing_if = "emptiness::is_zero_turn")]
    pub(super) entered_turn: u32,
    /// Detained until this seat's next turn, with the turn count it landed on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) detained_until_turn_of: Option<(usize, u32)>,
    /// Untap steps this permanent still owes before it untaps normally.
    #[serde(default, skip_serializing_if = "emptiness::is_zero_u8")]
    pub(super) skipped_untap_steps: u8,
    pub(super) control_reverts_to: Option<usize>,
    /// The permanent sustaining a duration-scoped control change, absent for
    /// the turn-scoped form and for everything untouched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) control_source: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) resolving_control_timestamp: Option<u64>,
    /// Whether that holder also has to stay tapped.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) control_requires_source_tapped: bool,
    /// Whether the control source has to remain attached to this permanent.
    /// Additive: older checkpoints contain no static attachment control state.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) control_requires_source_attached: bool,
    pub(super) chosen_player: Option<usize>,
    /// Authored label for the public creature-type choice, retained across reconstruction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) chosen_creature_type_binding: Option<String>,
    /// The X the spell that made this permanent was cast for.
    #[serde(default, skip_serializing_if = "emptiness::is_zero_u16")]
    pub(super) cast_x: u16,
    /// How many times a repeatable optional additional cost was paid for
    /// that spell. Additive, and absent for everything nobody kicked.
    #[serde(default, skip_serializing_if = "emptiness::is_zero_u16")]
    pub(super) cast_kicks: u16,
    /// Per-clause optional additional-cost payment counts. Additive; older
    /// checkpoints retain only the aggregate repeatable-cost count above.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) cast_additional_costs: Vec<u16>,
    /// How many colours paid for it. Additive, and absent for everything
    /// nobody cast.
    #[serde(default, skip_serializing_if = "emptiness::is_zero_u16")]
    pub(super) cast_colors: u16,
    /// Exact colors spent, replacing the legacy count above. The count stays
    /// on the wire so older readers can still restore sunburst correctly.
    #[serde(default, skip_serializing_if = "stack::no_colors_spent")]
    pub(super) cast_colors_of_mana_spent: [bool; 5],
    #[serde(default, skip_serializing_if = "emptiness::is_zero_u16")]
    pub(super) cast_phyrexian_symbols_paid_with_life: u16,
    /// The alternative this permanent's spell was cast with, by its stable
    /// name. Stored as a string so the wire form does not depend on the
    /// order of a catalog enum.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) cast_alternative: Option<String>,
    /// Name of the chosen alternative cost; absent for unlabeled or external costs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) cast_alternative_cost_binding: Option<String>,
    /// Semantic cast facts by stable name. Additive: an absent collection is
    /// an ordinary untagged cast.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) cast_tags: Vec<String>,
    /// Exile-zone identities of cards used to pay this spell's costs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) cast_exiled_payment_cards: Vec<u32>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) cast_via_flashback: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) cast_exile_if_put_into_graveyard: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) cast_via_suspend: bool,
    /// Which zone this spell was cast from, by its stable label. Additive:
    /// a checkpoint written before the zone was recorded restores as
    /// nothing, which is what a permanent nobody cast carries anyway.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) cast_from_zone: Option<String>,
    pub(super) destroy_at_end: bool,
    pub(super) counters: Vec<CounterSnapshot>,
    pub(super) attached_to: Option<u32>,
    /// The player a player-enchanting Aura is attached to. Additive: older
    /// checkpoints restore no such Auras because none were executable then.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) attached_player: Option<usize>,
    pub(super) reconfigured_timestamp: Option<u64>,
    pub(super) exile_instead_of_dying: bool,
    pub(super) combat_damage_assignment: Vec<CombatDamageAssignmentSnapshot>,
    pub(super) regeneration_shields: u8,
    pub(super) attacked_this_turn: bool,
    /// Additive: a checkpoint written before exert existed restores with
    /// nothing exerted, which is what every board without one is.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) exerted: bool,
    /// Additive in the same way: a checkpoint written before saddling
    /// existed restores with nothing saddled.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) saddled: bool,
    /// The exhaust abilities this permanent has already spent. Additive:
    /// a checkpoint written before exhaust existed restores with none
    /// spent, which is what every board without one has.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) exhausted: Vec<AbilityOriginSnapshot>,
    pub(super) attacks_this_turn: u8,
    /// The seat that controlled this permanent the last time it attacked, and
    /// their turn count then. Absent means it has never attacked, which is
    /// what a checkpoint written before this was recorded also means.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) last_attacked_turn: Option<(usize, u32)>,
    pub(super) damage_sources: Vec<u32>,
    #[serde(default)]
    pub(super) was_dealt_damage_this_turn: bool,
    #[serde(default)]
    pub(super) dealt_damage_this_turn: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) paired_with: Option<u32>,
    pub(super) dealt_damage_to_opponent_this_turn: bool,
    pub(super) deathtouch_damage: bool,
    pub(super) created_by: Option<u32>,
    pub(super) temporary_keywords: Vec<KeywordSnapshot>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) suspend_haste: bool,
    pub(super) keywords_until_upkeep_of: Vec<UpkeepKeywordSnapshot>,
    pub(super) resolved_continuous_effects: Vec<ResolvedContinuousEffectSnapshot>,
    pub(super) activations_this_turn: Vec<AbilityActivationSnapshot>,
    /// Additive: a payload written before any ability capped its own
    /// triggering carries none, which is a turn in which none has.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) triggers_this_turn: Vec<AbilityActivationSnapshot>,
    /// Absent from a payload written before any ability counted its own
    /// resolutions, which is why it defaults rather than being required.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) resolutions_this_turn: Vec<AbilityActivationSnapshot>,
    /// Additive: a payload written before either flag existed restores both
    /// as false, which is what an ordinary permanent means anyway.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) cast_at_instant_speed: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) became_aura: bool,
    pub(super) copy_effect: Option<CopiableCharacteristicsSnapshot>,
    pub(super) copy_expiration: Option<ContinuousEffectExpirationSnapshot>,
    pub(super) copied_from: Option<CopiedFromSnapshot>,
    pub(super) text_changes: Vec<BasicLandTypeChangeSnapshot>,
    pub(super) has_dynamic_characteristics: bool,
}
