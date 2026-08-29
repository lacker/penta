// A permanent on the battlefield: its own state and the questions it can
// answer about itself, kept apart from the game that holds the battlefield.
// Included textually into `mod.rs`, so the imports here are that module's.

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
struct Permanent {
    card: ObjectInstance,
    /// Authored characteristics for an ordinary created token. Token copies
    /// instead freeze their source in `copy_effect` or
    /// `double_faced_token_copy`; either way token status is explicit in the
    /// object's kind, never inferred from a catalog set.
    token_characteristics: Option<TokenCharacteristics>,
    /// Both intrinsic faces of a token created as a copy of a double-faced
    /// permanent. A later copy effect masks these values but does not erase
    /// the physical double-faced representation (CR 707.8a, 712.9).
    double_faced_token_copy: Option<DoubleFacedCopiableCharacteristics>,
    timestamp: ContinuousEffectTimestamp,
    /// The logical part currently supplying this permanent's baseline
    /// characteristics. Transforming changes this without changing object ID.
    presented: CardPartId,
    controller: PlayerId,
    tapped: bool,
    entered_controller_turn: u32,
    /// The game turn this permanent entered the battlefield, which is not
    /// the same question as the one above: that one is measured against its
    /// controller's own turn count, so it stays true through the opponent's
    /// following turn, which is exactly what summoning sickness wants and
    /// exactly what "as long as this entered this turn" does not.
    entered_turn: u32,
    damage: u16,
    /// The X chosen for the spell that put this permanent here, zero when
    /// nothing chose one. Its own enters trigger is a separate object and so
    /// cannot read the spell's X any other way.
    cast_x: u16,
    /// How many times a repeatable optional additional cost was paid for the
    /// spell that put this permanent here, zero when none was. Read the same
    /// way and for the same reason as the X above: "for each time it was
    /// kicked" is asked after the spell is gone.
    pub(super) cast_kicks: u16,
    /// How many colours of mana paid for that spell, which is what sunburst
    /// counts. Read here rather than off the spell for the same reason as
    /// the two above: the counters go on as the permanent enters, and by the
    /// time anything else asks, the spell is gone.
    pub(super) cast_colors: u16,
    /// How this permanent's spell was cast, when it was cast at all. Evoke's
    /// sacrifice and every other clause that asks "if it was cast this way"
    /// reads it here, because the spell object is gone by the time the
    /// permanent's own triggers resolve.
    cast_alternative: Option<AlternativeCastKindDef>,
    /// Whether the spell this permanent came from was cast at a time a
    /// sorcery could not have been. Necromancy's own drawback asks, and the
    /// spell object is gone by the time the permanent's triggers resolve.
    cast_at_instant_speed: bool,
    /// Whether the spell this permanent came from was cast from its
    /// controller's hand. Amped Raptor's second clause asks, and a permanent
    /// that was never a spell at all answers no.
    cast_from_zone: Option<CastSourceZone>,
    /// Whether this permanent has become an Aura. Necromancy is not one as
    /// it enters -- its own trigger makes it one, in the same resolution
    /// that attaches it -- so the window between entering and reanimating
    /// must not be read as an Aura attached to nothing.
    became_aura: bool,
    attacking: bool,
    attack_defender: Option<crate::AttackDefender>,
    /// Which attacking band this creature belongs to, as an index shared by
    /// every member. A lone attacker has none: a band of one is just an
    /// attacker, and giving it an index would make the two indistinguishable.
    attacking_band: Option<u8>,
    emblem_source: Option<AbilityOrigin>,
    /// Whether a loyalty ability has already been activated this turn. CR
    /// 606.3 allows one per planeswalker per turn.
    activated_loyalty_this_turn: bool,
    /// Detained until this player's next turn begins, recorded with how many
    /// turns they had taken when it landed so "next" means the one after.
    detained_until_turn_of: Option<(PlayerId, u32)>,
    /// How many of this permanent's controller's untap steps it still has to
    /// sit out. Counted rather than flagged because Telekinesis names two.
    skipped_untap_steps: u8,
    /// Who controls this permanent again once the turn ends, set while a
    /// control-changing effect holds it. Cleanup restores it.
    control_reverts_to: Option<PlayerId>,
    /// The permanent whose continued presence is holding this one's control
    /// change. When it leaves the battlefield or changes hands, control goes
    /// back to `control_reverts_to`.
    control_source: Option<GameObjectId>,
    /// Whether that holder also has to stay tapped to keep the change.
    control_requires_source_tapped: bool,
    /// Whether the holder has to remain attached to this permanent. Static
    /// Aura control effects use this instead of merely requiring their source
    /// to remain on the battlefield.
    control_requires_source_attached: bool,
    /// Whether this attacker was blocked. A blocked creature stays blocked
    /// even if every blocker leaves, so this cannot be recomputed from the
    /// blockers still on the battlefield.
    blocked: bool,
    /// Every attacker this creature is blocking, in declaration order.
    ///
    /// A list rather than one attacker because a creature may be allowed to
    /// block several, and because a band is blocked as a group: one
    /// declaration against a band puts every member in here.
    ///
    /// This is the live relationship, and only the live relationship: it is
    /// what combat damage is exchanged along. Whether the creature is a
    /// blocking creature at all is `blocking_this_combat`.
    blocking: Vec<GameObjectId>,
    /// Whether this creature was declared as a blocker this combat. CR 506.4
    /// lists every way a permanent leaves combat, and an attacker leaving is
    /// not one of them, so a blocker stays a blocking creature even once
    /// everything it blocked is gone. That cannot be recomputed from
    /// `blocking`, which those departures empty.
    blocking_this_combat: bool,
    chosen_player: Option<PlayerId>,
    chosen_creature_type: Option<String>,
    /// The basic land type this permanent was told to be as it entered.
    pub(super) chosen_basic_land_type: Option<crate::card::BasicLandType>,
    /// The color this permanent's controller chose as it entered.
    pub(super) chosen_color: Option<crate::card::ManaColor>,
    /// The card name a permanent named as it entered, for Pithing Needle.
    chosen_card_name: Option<String>,
    /// The copiable values supplied by the rule, ability, or effect that made
    /// this permanent face down (CR 708.2). `None` means face up. The physical
    /// card is unchanged: `card.definition` still names it, which is what lets
    /// it be turned face up and what it becomes in another zone.
    face_down: Option<FaceDownCharacteristics>,
    /// Whether this face-down mechanism permits a creature card to turn face
    /// up for its own mana cost. Manifest and Cloak do; Morph and Disguise use
    /// their printed special-action cost instead.
    turn_up_for_mana_cost: bool,
    destroy_at_end: bool,
    temporary_keywords: Vec<KeywordAbility>,
    /// Suspend grants haste until this permanent's controller loses control
    /// of it, rather than merely until cleanup.
    suspend_haste: bool,
    /// Resolved noncopiable characteristic changes and rules modifications,
    /// in creation order.
    resolved_continuous_effects: Vec<ResolvedContinuousEffect>,
    /// How many times each of this permanent's activated abilities has been
    /// activated this turn, for the cards that count their own activations.
    /// Cleared when the next turn begins, after any inserted phases.
    activations_this_turn: Vec<(AbilityOrigin, u8)>,
    /// The exhaust abilities this permanent has already spent. Unlike the
    /// counts above this list never clears: an exhausted ability is
    /// exhausted for as long as the permanent is there.
    pub(super) exhausted: Vec<AbilityOrigin>,
    /// How many times each of this permanent's triggered abilities has
    /// triggered this turn, for the ones that print "this ability triggers
    /// only once each turn". Cleared alongside the activations above.
    triggers_this_turn: Vec<(AbilityOrigin, u8)>,
    /// How many times each of this permanent's abilities has *resolved* this
    /// turn. Distinct from the triggers above: a countered trigger triggered
    /// and never resolved, and Omnath counts resolutions. Counted as the
    /// resolution begins, so an ability asking whether this is the first
    /// time counts itself. Cleared alongside the two above.
    resolutions_this_turn: Vec<(AbilityOrigin, u8)>,
    /// Every counter this permanent carries. The sparse collection preserves
    /// counter identity without coupling object layout to a catalog-wide enum.
    counters: Counters,
    /// Which permanent this Aura, Equipment, or Fortification is attached to.
    /// Player-enchanting Auras use `attached_player` instead.
    attached_to: Option<GameObjectId>,
    /// Which player this Aura enchants. Kept separate from `chosen_player`:
    /// targeting a player is an attachment relation, not an enters choice,
    /// and effects may move that attachment later.
    attached_player: Option<PlayerId>,
    /// The timestamped layer-4 operation created by reconfigure. It lasts
    /// only for this attachment incarnation and therefore clears whenever
    /// the Equipment becomes unattached.
    reconfigured_timestamp: Option<ContinuousEffectTimestamp>,
    /// Legacy checkpoint compatibility for the retired card-local
    /// exile-instead-of-dying path. New effects store the shared applied rule;
    /// this flag preserves already-written checkpoints until cleanup.
    exile_instead_of_dying: bool,
    combat_damage_assignment: Vec<CombatDamageAssignment>,
    /// Values established by the most recent copy effect. This is a frozen
    /// snapshot rather than a live pointer to the target, so later changes to
    /// that object cannot leak through and copy chains preserve exceptions.
    copy_effect: Option<CopiableCharacteristics>,
    /// When the copy above ends. A printed copy effect almost never states a
    /// duration and lasts indefinitely; Saheeli's does, so the copy has to
    /// know how long it is one.
    copy_expiration: Option<ContinuousEffectExpiration>,
    /// Whether this permanent entered as a copy. Transforming double-faced
    /// cards use this to distinguish their own back face from a copied one
    /// when determining mana value.
    copied_from: Option<ObjectCharacteristics>,
    /// Indefinite text changes applied to this object in timestamp order.
    text_changes: Vec<BasicLandTypeChange>,
    regeneration_shields: u8,
    attacked_this_turn: bool,
    /// Exerted as it was declared as an attacker this turn (CR 701.38a).
    /// Recorded rather than derived: what exerting costs is an untap step,
    /// and plenty of other things skip one of those.
    pub(super) exerted: bool,
    /// Saddled (CR 702.166a): a Mount whose saddle ability has been paid
    /// this turn. It is a fact about the permanent rather than a counter,
    /// and it ends with the turn.
    pub(super) saddled: bool,
    /// How many times this creature has been declared as an attacker this
    /// turn. `attacked_this_turn` is already set by the time the attack
    /// triggers are captured, so a "first time each turn" trigger needs the
    /// count rather than the flag.
    attacks_this_turn: u8,
    /// Who controlled this permanent the last time it was declared as an
    /// attacker, and how many turns that player had started by then.
    ///
    /// History rather than turn state: it survives the cleanup that clears
    /// `attacked_this_turn`, which is the whole point of "attacked during
    /// your last turn". The controller is recorded alongside because the
    /// turn count is that player's own.
    last_attacked_turn: Option<(PlayerId, u32)>,
    /// Keywords granted until a named player's next upkeep, which outlive
    /// the cleanup that clears `temporary_keywords`. Erhnam Djinn's
    /// forestwalk is one.
    keywords_until_upkeep_of: Vec<(PlayerId, KeywordAbility)>,
    /// Sources that dealt damage to this permanent during the current turn.
    /// IDs deliberately refer to the damaging object incarnation so a later
    /// death trigger can use the live source or its retired LKI snapshot.
    damage_sources: Vec<GameObjectId>,
    /// Whether any damage landed on this permanent this turn. Distinct from
    /// `damage`, which regeneration and cleanup both wipe, and from
    /// `damage_sources`, which records nothing for a sourceless event.
    was_dealt_damage_this_turn: bool,
    /// The creature this one is soulbonded to. Symmetric: both permanents
    /// name each other, and the pair breaks the moment one of them stops
    /// being a creature one player controls.
    paired_with: Option<GameObjectId>,
    /// The mirror: whether this permanent dealt damage to anything this turn.
    /// Broader than `dealt_damage_to_opponent_this_turn`, which ignores
    /// damage to creatures and to its own controller.
    dealt_damage_this_turn: bool,
    /// Whether this permanent has dealt damage to an opponent of its
    /// controller this turn, by any means. Cleared when the next turn begins,
    /// after any inserted phases.
    dealt_damage_to_opponent_this_turn: bool,
    /// Whether any damage still marked on this permanent came from a source
    /// with deathtouch. The source may leave before state-based actions are
    /// checked, so this is damage-event state rather than a live lookup.
    deathtouch_damage: bool,
    /// The permanent whose ability created this token, for the cards that
    /// later refer to "tokens created with this creature". A token that
    /// outlives its creator keeps pointing at an object ID nothing matches,
    /// which is what makes those tokens permanently orphaned.
    created_by: Option<GameObjectId>,
}

impl Permanent {
    /// Whether this creature is blocking that attacker.
    fn is_blocking(&self, attacker: GameObjectId) -> bool {
        self.blocking.contains(&attacker)
    }

    /// Whether it is blocking anything at all right now. Ask this for combat
    /// damage, which flows along the live relationship.
    fn is_blocking_anything(&self) -> bool {
        !self.blocking.is_empty()
    }

    /// Whether it is a blocking creature. Ask this for everything that reads
    /// the status rather than the relationship -- "attacking or blocking
    /// creature", first strike waves -- because it outlives the attackers.
    ///
    /// Either half answers on its own: a live block is a block, and the flag
    /// carries the ones whose attackers have since left.
    fn is_blocking_this_combat(&self) -> bool {
        self.blocking_this_combat || self.is_blocking_anything()
    }

    /// A permanent as it arrives on the battlefield, before any card-specific
    /// adjustments. Three call sites used to spell out every field, which made
    /// adding one a three-place edit and gave a new entry path nothing to
    /// build on.
    fn entering(
        card: impl Into<ObjectInstance>,
        presented: CardPartId,
        controller: PlayerId,
        entered_controller_turn: u32,
        entered_turn: u32,
    ) -> Self {
        let card = card.into();
        Self {
            timestamp: ContinuousEffectTimestamp(u64::from(card.id.0)),
            card,
            token_characteristics: None,
            double_faced_token_copy: None,
            presented,
            controller,
            tapped: false,
            entered_controller_turn,
            entered_turn,
            damage: 0,
            cast_x: 0,
            cast_kicks: 0,
            cast_colors: 0,
            cast_alternative: None,
            cast_from_zone: None,
            cast_at_instant_speed: false,
            became_aura: false,
            attacking: false,
            attack_defender: None,
            attacking_band: None,
            emblem_source: None,
            activated_loyalty_this_turn: false,
            detained_until_turn_of: None,
            skipped_untap_steps: 0,
            control_reverts_to: None,
            control_source: None,
            control_requires_source_tapped: false,
            control_requires_source_attached: false,
            blocked: false,
            blocking: Vec::new(),
            blocking_this_combat: false,
            chosen_player: None,
            chosen_creature_type: None,
            chosen_basic_land_type: None,
            chosen_color: None,
            chosen_card_name: None,
            face_down: None,
            turn_up_for_mana_cost: false,
            destroy_at_end: false,
            temporary_keywords: Vec::new(),
            suspend_haste: false,
            resolved_continuous_effects: Vec::new(),
            activations_this_turn: Vec::new(),
            exhausted: Vec::new(),
            triggers_this_turn: Vec::new(),
            resolutions_this_turn: Vec::new(),
            counters: crate::game::counters::Counters::new(),
            attached_to: None,
            attached_player: None,
            reconfigured_timestamp: None,
            exile_instead_of_dying: false,
            combat_damage_assignment: Vec::new(),
            copy_effect: None,
            copy_expiration: None,
            copied_from: None,
            text_changes: Vec::new(),
            regeneration_shields: 0,
            attacked_this_turn: false,
            exerted: false,
            saddled: false,
            attacks_this_turn: 0,
            last_attacked_turn: None,
            keywords_until_upkeep_of: Vec::new(),
            damage_sources: Vec::new(),
            paired_with: None,
            was_dealt_damage_this_turn: false,
            dealt_damage_this_turn: false,
            dealt_damage_to_opponent_this_turn: false,
            deathtouch_damage: false,
            created_by: None,
        }
    }

    fn entering_token(
        card: ObjectInstance,
        token: TokenCharacteristics,
        controller: PlayerId,
        entered_controller_turn: u32,
        entered_turn: u32,
    ) -> Self {
        debug_assert!(card.definition.is_token());
        let mut permanent = Self::entering(
            card,
            token.primary_part_id(),
            controller,
            entered_controller_turn,
            entered_turn,
        );
        permanent.token_characteristics = Some(token);
        permanent
    }

    /// Copiable values currently supplying this permanent's characteristics.
    /// A live copy effect masks every physical face. Without one, a
    /// double-faced copy-token selects the intrinsic values of its face up.
    fn active_copy_values(&self) -> Option<&CopiableCharacteristics> {
        self.copy_effect.as_ref().or_else(|| {
            self.double_faced_token_copy
                .as_ref()
                .and_then(|faces| faces.face(self.presented))
        })
    }

    /// Untaps, unless a stun counter replaces it (CR 122.1d): a permanent
    /// carrying one removes it and stays tapped instead. A permanent that is
    /// already untapped is not becoming untapped, so nothing is replaced and
    /// no counter comes off.
    fn untap(&mut self) {
        if !self.tapped {
            return;
        }
        if self.counters(CounterKind::Stun) > 0 {
            self.remove_counters(CounterKind::Stun, 1);
        } else {
            self.tapped = false;
        }
    }

    fn counters(&self, kind: CounterKind) -> u16 {
        self.counters.count(kind)
    }

    fn add_counters(&mut self, kind: CounterKind, amount: u16) {
        self.counters.add(kind, amount);
    }

    fn remove_counters(&mut self, kind: CounterKind, amount: u16) {
        self.counters.remove(kind, amount);
    }

    fn set_counters(&mut self, kind: CounterKind, amount: u16) {
        self.counters.set(kind, amount);
    }
}
