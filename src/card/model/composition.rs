use crate::ids::{
    AdditionalCostId, AlternativeCostId, CardDefinitionId, CardPartId, PlayOptionId, TargetSlotId,
};

use super::{
    CardBehavior, CardEffectStatus, CardPart, CardPrinting, CardRules, CardSet, CardStructure,
    CardSupertype, CardType, DeclarativeAbilityDef, DoubleFacedKind, ImplementationStatus,
    ManaCost, ModeSetDef, PlayActionKind, PlayRestriction, PrintedManaCost, SpellForm,
    TargetSlotDef,
};

/// A named alternative to the cost supplied by a play option.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlternativeCostDef {
    pub id: AlternativeCostId,
    pub label: String,
    pub mana_cost: ManaCost,
}

/// A named additional cost. Some additional costs are nonmana costs, so the
/// mana component is optional and the authoritative rules remain in `label`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdditionalCostDef {
    pub id: AdditionalCostId,
    pub label: String,
    pub mana_cost: Option<ManaCost>,
    /// Whether one cast may pay this cost more than once, which only
    /// replicate does. A repeated payment appears once per payment in the
    /// cast's cost configuration, so the number of times it was paid is the
    /// number of times its id is there.
    pub repeatable: bool,
}

/// One legal way to play a card. This is distinct from rules-text modes and
/// from alternative/additional cost choices.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayOptionDef {
    pub id: PlayOptionId,
    pub label: String,
    pub action: PlayActionKind,
    pub form: SpellForm,
    pub mana_cost: Option<ManaCost>,
    pub restriction: PlayRestriction,
    pub modes: Option<ModeSetDef>,
    pub targets: Vec<TargetSlotDef>,
    pub alternative_costs: Vec<AlternativeCostDef>,
    pub additional_costs: Vec<AdditionalCostDef>,
    pub effect_status: CardEffectStatus,
}

impl PlayOptionDef {
    #[must_use]
    pub fn cast(
        id: PlayOptionId,
        label: impl Into<String>,
        form: SpellForm,
        mana_cost: ManaCost,
        effect_status: CardEffectStatus,
    ) -> Self {
        Self::cast_with_printed_mana_cost(
            id,
            label,
            form,
            PrintedManaCost::Cost(mana_cost),
            effect_status,
        )
    }

    /// Defines a cast action without collapsing a nonexistent printed cost
    /// into `{0}`. A spell with `PrintedManaCost::None` ordinarily needs a
    /// separate casting permission or alternative cost before it is legal.
    #[must_use]
    pub fn cast_with_printed_mana_cost(
        id: PlayOptionId,
        label: impl Into<String>,
        form: SpellForm,
        printed_mana_cost: PrintedManaCost,
        effect_status: CardEffectStatus,
    ) -> Self {
        Self {
            id,
            label: label.into(),
            action: PlayActionKind::CastSpell,
            form,
            mana_cost: printed_mana_cost.as_option(),
            restriction: PlayRestriction::Normal,
            modes: None,
            targets: Vec::new(),
            alternative_costs: Vec::new(),
            additional_costs: Vec::new(),
            effect_status,
        }
    }

    #[must_use]
    pub fn play_land(
        id: PlayOptionId,
        label: impl Into<String>,
        part: CardPartId,
        effect_status: CardEffectStatus,
    ) -> Self {
        Self {
            id,
            label: label.into(),
            action: PlayActionKind::PlayLand,
            form: SpellForm::Part(part),
            mana_cost: None,
            restriction: PlayRestriction::Normal,
            modes: None,
            targets: Vec::new(),
            alternative_costs: Vec::new(),
            additional_costs: Vec::new(),
            effect_status,
        }
    }

    #[must_use]
    pub fn with_targets(mut self, targets: Vec<TargetSlotDef>) -> Self {
        self.targets = targets;
        self
    }

    #[must_use]
    pub fn with_modes(mut self, modes: ModeSetDef) -> Self {
        self.modes = Some(modes);
        self
    }

    /// Adds the printed alternative costs owned by alternative-casting
    /// clauses on `rules`. Existing manually authored generic alternatives
    /// remain intact.
    #[must_use]
    pub fn with_alternative_cast_costs(mut self, rules: &CardRules) -> Self {
        let card_mana_cost = self.mana_cost;
        self.alternative_costs.extend(
            rules
                .indexed_abilities()
                .filter_map(|ability| ability.alternative_cost(card_mana_cost)),
        );
        self
    }

    /// Adds optional additional costs owned by semantic casting clauses on
    /// `rules`. These remain independent of every alternative cost.
    #[must_use]
    pub fn with_optional_additional_costs(mut self, rules: &CardRules) -> Self {
        self.additional_costs
            .extend(rules.indexed_abilities().filter_map(|ability| {
                ability
                    .definition
                    .is_executable()
                    .then(|| ability.additional_cost())
                    .flatten()
            }));
        self
    }

    #[must_use]
    pub const fn restricted_to_hand(mut self) -> Self {
        self.restriction = PlayRestriction::FromHandOnly;
        self
    }
}

/// The structured portion of a card definition supplied by a set record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardComposition {
    pub parts: Vec<CardPart>,
    pub structure: CardStructure,
    pub play_options: Vec<PlayOptionDef>,
}

impl CardComposition {
    fn effect_status(rules: &CardRules) -> CardEffectStatus {
        match rules.implementation_status() {
            ImplementationStatus::MetadataOnly => CardEffectStatus::MetadataOnly,
            ImplementationStatus::Complete | ImplementationStatus::Partial => {
                CardEffectStatus::Implemented
            }
        }
    }

    fn face_play_option(
        id: PlayOptionId,
        part: CardPartId,
        name: &'static str,
        rules: &CardRules,
    ) -> PlayOptionDef {
        let mut option = if rules.has_type(CardType::Land) {
            PlayOptionDef::play_land(id, name, part, Self::effect_status(rules))
        } else {
            PlayOptionDef::cast_with_printed_mana_cost(
                id,
                name,
                SpellForm::Part(part),
                rules.printed_mana_cost,
                Self::effect_status(rules),
            )
            .with_alternative_cast_costs(rules)
            .with_optional_additional_costs(rules)
        };
        if rules.play_restriction() != PlayRestriction::Normal {
            option.restriction = rules.play_restriction();
        }
        if let Some(modes) = rules.presentation_spell_modes() {
            option = option.with_modes(modes);
        }
        option
    }

    #[must_use]
    pub fn single(name: impl Into<String>, rules: CardRules) -> Self {
        let printed_mana_cost = rules.printed_mana_cost;
        let name = name.into();
        let is_land = rules.has_type(CardType::Land);
        let effect_status = Self::effect_status(&rules);
        let part = CardPart::new(CardPartId::PRIMARY, name.clone(), rules);
        let mut option = if is_land {
            PlayOptionDef::play_land(
                PlayOptionId::DEFAULT,
                name,
                CardPartId::PRIMARY,
                effect_status,
            )
        } else {
            PlayOptionDef::cast_with_printed_mana_cost(
                PlayOptionId::DEFAULT,
                name,
                SpellForm::Part(CardPartId::PRIMARY),
                printed_mana_cost,
                effect_status,
            )
            .with_alternative_cast_costs(&rules)
            .with_optional_additional_costs(&rules)
        };
        if rules.play_restriction() != PlayRestriction::Normal {
            option.restriction = rules.play_restriction();
        }
        if let Some(modes) = rules.presentation_spell_modes() {
            option = option.with_modes(modes);
        }
        Self {
            parts: vec![part],
            structure: CardStructure::Single {
                main: CardPartId::PRIMARY,
            },
            play_options: vec![option],
        }
        .with_derived_spell_targets()
    }

    /// Materializes the parts, topology, and legal play options of a
    /// double-faced card from its two face definitions.
    #[must_use]
    pub fn double_faced(
        faces: &'static [(&'static str, CardRules); 2],
        kind: DoubleFacedKind,
    ) -> Self {
        let [(front_name, front_rules), (back_name, back_rules)] = *faces;
        let mut play_options = vec![Self::face_play_option(
            PlayOptionId::DEFAULT,
            CardPartId::PRIMARY,
            front_name,
            &front_rules,
        )];
        if kind == DoubleFacedKind::Modal {
            play_options.push(Self::face_play_option(
                PlayOptionId(1),
                CardPartId(1),
                back_name,
                &back_rules,
            ));
        }
        Self {
            parts: vec![
                CardPart::new(CardPartId::PRIMARY, front_name, front_rules),
                CardPart::new(CardPartId(1), back_name, back_rules),
            ],
            structure: CardStructure::DoubleFaced {
                front: CardPartId::PRIMARY,
                back: CardPartId(1),
                kind,
            },
            play_options,
        }
        .with_derived_spell_targets()
    }

    /// Materializes the parts and cast options of a split card. A fused cost
    /// adds the combined hand-only option; without one, either half remains
    /// independently castable.
    #[must_use]
    pub fn split(
        halves: &'static [(&'static str, CardRules); 2],
        fuse_cost: Option<ManaCost>,
    ) -> Self {
        let [(first_name, first_rules), (second_name, second_rules)] = *halves;
        let parts = vec![CardPartId::PRIMARY, CardPartId(1)];
        let mut play_options = vec![
            Self::face_play_option(
                PlayOptionId::DEFAULT,
                CardPartId::PRIMARY,
                first_name,
                &first_rules,
            ),
            Self::face_play_option(PlayOptionId(1), CardPartId(1), second_name, &second_rules),
        ];
        let fused = fuse_cost.map(|cost| {
            let id = PlayOptionId(2);
            let status = if Self::effect_status(&first_rules) == CardEffectStatus::Implemented
                && Self::effect_status(&second_rules) == CardEffectStatus::Implemented
            {
                CardEffectStatus::Implemented
            } else {
                CardEffectStatus::MetadataOnly
            };
            play_options.push(
                PlayOptionDef::cast(
                    id,
                    format!("{first_name} // {second_name}"),
                    SpellForm::Combined(parts.clone()),
                    cost,
                    status,
                )
                .restricted_to_hand(),
            );
            id
        });
        Self {
            parts: vec![
                CardPart::new(CardPartId::PRIMARY, first_name, first_rules),
                CardPart::new(CardPartId(1), second_name, second_rules),
            ],
            structure: CardStructure::Split { parts, fused },
            play_options,
        }
        .with_derived_spell_targets()
    }

    /// A Room (CR 714): two doors, the pair of them, and neither of them.
    ///
    /// `combined` is what the permanent is once both doors are open -- the
    /// two halves' abilities together, for the two halves' costs added up --
    /// and `locked` is what a Room that arrived without anyone choosing a
    /// door is: a Room enchantment with nothing in it. Both are states of
    /// the permanent rather than printed faces, which is why only the doors
    /// are castable and only the doors are what the card is in a library.
    ///
    /// # Panics
    ///
    /// Panics if either door has no printed mana cost. A door is a half you
    /// cast, so there is always something to pay.
    #[must_use]
    #[allow(clippy::large_types_passed_by_value)]
    pub fn room(
        combined_name: impl Into<String>,
        first_name: &str,
        first: CardRules,
        second_name: &str,
        second: CardRules,
        combined: CardRules,
    ) -> Self {
        const COMBINED: CardPartId = CardPartId(2);
        const LOCKED: CardPartId = CardPartId(3);
        let combined_name = combined_name.into();
        let door_option = |id: PlayOptionId, part: CardPartId, name: &str, rules: &CardRules| {
            PlayOptionDef::cast(
                id,
                name,
                SpellForm::Part(part),
                rules
                    .mana_cost()
                    .expect("a Room's door has a printed mana cost"),
                match rules.implementation_status() {
                    ImplementationStatus::MetadataOnly => CardEffectStatus::MetadataOnly,
                    ImplementationStatus::Complete | ImplementationStatus::Partial => {
                        CardEffectStatus::Implemented
                    }
                },
            )
        };
        let options = vec![
            door_option(
                PlayOptionId::DEFAULT,
                CardPartId::PRIMARY,
                first_name,
                &first,
            ),
            door_option(PlayOptionId(1), CardPartId(1), second_name, &second),
        ];
        Self {
            parts: vec![
                CardPart::new(CardPartId::PRIMARY, first_name, first),
                CardPart::new(CardPartId(1), second_name, second),
                CardPart::new(COMBINED, combined_name.clone(), combined),
                CardPart::new(
                    LOCKED,
                    combined_name,
                    CardRules::new_enchantment_without_mana_cost().with_subtypes(&["Room"]),
                ),
            ],
            structure: CardStructure::Room {
                doors: vec![CardPartId::PRIMARY, CardPartId(1)],
                combined: COMBINED,
                locked: LOCKED,
            },
            play_options: options,
        }
        .with_derived_spell_targets()
    }

    /// Derives nonmodal play-option target presentations from the spell
    /// clauses of the option's parts. Combined forms flatten their parts in
    /// printed order, assigning runtime slot IDs only after composition.
    ///
    /// A composition can still supply explicit presentation targets when it
    /// has no semantic spell clause. When the semantic predicate vocabulary
    /// is richer than the legacy presentation vocabulary, the projection is
    /// left empty and runtime target generation uses the semantic definition.
    #[must_use]
    pub(crate) fn with_derived_spell_targets(mut self) -> Self {
        for option in &mut self.play_options {
            if option.action != PlayActionKind::CastSpell
                || option.modes.is_some()
                || !option.targets.is_empty()
            {
                continue;
            }
            let part_ids = match &option.form {
                SpellForm::Part(part) => core::slice::from_ref(part),
                SpellForm::Combined(parts) => parts.as_slice(),
            };
            let derived = part_ids
                .iter()
                .try_fold(Vec::new(), |mut targets, part_id| {
                    let part = self.parts.iter().find(|part| part.id == *part_id)?;
                    let spell = part.rules.ability_clauses().iter().find_map(|ability| {
                        let DeclarativeAbilityDef::Spell(spell) = ability.definition else {
                            return None;
                        };
                        spell.modal().is_none().then_some(spell)
                    })?;
                    for target in spell.targets() {
                        let id = TargetSlotId::from_index(targets.len())?;
                        targets.push(target.presentation(id)?);
                    }
                    Some(targets)
                });
            if let Some(derived) = derived {
                option.targets = derived;
            }
        }
        self
    }
}

/// Canonical artwork metadata for a card definition or created-token effect.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CardArt {
    pub scryfall_id: &'static str,
    pub artist: &'static str,
}

impl CardArt {
    #[must_use]
    pub const fn new(scryfall_id: &'static str, artist: &'static str) -> Self {
        Self {
            scryfall_id,
            artist,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardDefinition {
    pub id: CardDefinitionId,
    pub name: String,
    pub art: Option<CardArt>,
    /// The canonical record's debut set within this catalog.
    ///
    /// Rules that care where a card debuted, such as City in a Bottle, use
    /// this field. Format legality instead considers every known `printing`.
    pub debut_set: CardSet,
    pub printings: Vec<CardPrinting>,
    /// Compatibility view of the primary/front part. Contextual rules should
    /// use `parts` once the game engine is part-aware.
    pub rules: CardRules,
    pub parts: Vec<CardPart>,
    pub structure: CardStructure,
    pub play_options: Vec<PlayOptionDef>,
}

impl CardDefinition {
    /// Creates a definition using the built-in metadata for `behavior`.
    #[must_use]
    pub fn new(
        id: CardDefinitionId,
        name: impl Into<String>,
        debut_set: CardSet,
        is_basic_land: bool,
        behavior: CardBehavior,
    ) -> Self {
        let name = name.into();
        let rules = if is_basic_land {
            (*behavior.rules()).with_supertype(CardSupertype::Basic)
        } else {
            *behavior.rules()
        };
        let composition = CardComposition::single(name.clone(), rules);
        Self {
            id,
            name,
            art: None,
            debut_set,
            printings: vec![CardPrinting::new(id, debut_set)],
            rules,
            parts: composition.parts,
            structure: composition.structure,
            play_options: composition.play_options,
        }
    }

    #[must_use]
    pub const fn is_basic_land(&self) -> bool {
        self.rules.has_type(CardType::Land) && self.rules.has_supertype(CardSupertype::Basic)
    }

    #[must_use]
    pub fn part(&self, id: CardPartId) -> Option<&CardPart> {
        self.parts.iter().find(|part| part.id == id)
    }

    #[must_use]
    pub fn play_option(&self, id: PlayOptionId) -> Option<&PlayOptionDef> {
        self.play_options.iter().find(|option| option.id == id)
    }

    /// Whether a metadata-only play option still represents the shared,
    /// executable body of at least one creature part.
    #[must_use]
    pub(crate) fn play_option_has_executable_creature_body(&self, option: &PlayOptionDef) -> bool {
        let part_ids = match &option.form {
            SpellForm::Part(part) => core::slice::from_ref(part),
            SpellForm::Combined(parts) => parts.as_slice(),
        };
        part_ids.iter().any(|part_id| {
            self.part(*part_id)
                .is_some_and(|part| part.rules.has_executable_creature_body())
        })
    }

    /// Derives card-level coverage from every ordered clause on every part.
    /// A mix of complete and unimplemented parts is partial; a card is
    /// metadata-only only when every represented clause is unimplemented.
    #[must_use]
    pub fn implementation_status(&self) -> ImplementationStatus {
        let mut statuses = self
            .parts
            .iter()
            .map(|part| part.rules.implementation_status());
        statuses
            .next()
            .map_or(ImplementationStatus::Complete, |first| {
                statuses.fold(first, ImplementationStatus::combine)
            })
    }

    #[must_use]
    pub fn primary_part_id(&self) -> CardPartId {
        match &self.structure {
            CardStructure::Single { main } | CardStructure::AlternateSpell { main, .. } => *main,
            CardStructure::Split { parts, .. } | CardStructure::Room { doors: parts, .. } => {
                parts.first().copied().unwrap_or(CardPartId::PRIMARY)
            }
            CardStructure::Flip { normal, .. } => *normal,
            CardStructure::DoubleFaced { front, .. } | CardStructure::MeldPart { front, .. } => {
                *front
            }
        }
    }

    /// The part a permanent of this card presents when it arrives from
    /// anywhere but the stack.
    ///
    /// Only a Room has an answer other than its primary part: it enters with
    /// both doors locked, because nothing chose a door for it (CR 714.3d).
    #[must_use]
    pub fn battlefield_entry_part(&self) -> CardPartId {
        match &self.structure {
            CardStructure::Room { locked, .. } => *locked,
            _ => self.primary_part_id(),
        }
    }

    /// The doors of this Room that are locked while it presents `presented`.
    ///
    /// Empty for every card that is not a Room, and for a Room with both
    /// doors already open.
    #[must_use]
    pub fn locked_doors(&self, presented: CardPartId) -> Vec<CardPartId> {
        let CardStructure::Room {
            doors,
            combined,
            locked,
        } = &self.structure
        else {
            return Vec::new();
        };
        if presented == *locked {
            return doors.clone();
        }
        if presented == *combined {
            return Vec::new();
        }
        doors
            .iter()
            .copied()
            .filter(|door| *door != presented)
            .collect()
    }

    /// What this Room presents once `door` is unlocked on top of `presented`.
    ///
    /// `None` when the card is not a Room, when `door` is not one of its
    /// doors, or when that door is already unlocked -- a door that is already
    /// open cannot be opened again (CR 714.4b).
    #[must_use]
    pub fn presentation_after_unlocking(
        &self,
        presented: CardPartId,
        door: CardPartId,
    ) -> Option<CardPartId> {
        let CardStructure::Room {
            doors,
            combined,
            locked,
        } = &self.structure
        else {
            return None;
        };
        if !doors.contains(&door) {
            return None;
        }
        if presented == *locked {
            return Some(door);
        }
        if presented == door || presented == *combined {
            return None;
        }
        Some(*combined)
    }

    /// The face on the other side of a double-faced card, or nothing when the
    /// card has only one side to present.
    #[must_use]
    pub fn other_face(&self, presented: CardPartId) -> Option<CardPartId> {
        let CardStructure::DoubleFaced { front, back, .. } = &self.structure else {
            return None;
        };
        if presented == *front {
            Some(*back)
        } else if presented == *back {
            Some(*front)
        } else {
            None
        }
    }

    #[must_use]
    pub fn primary_part(&self) -> Option<&CardPart> {
        self.part(self.primary_part_id())
    }
}
