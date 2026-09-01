use std::borrow::Cow;

use crate::ids::{AbilityId, AdditionalCostId, AlternativeCostId, ModeId};

use super::{
    AbilityDef, AdditionalCostDef, AlternativeCostDef, CardSupertype, CardType, CardTypeSet,
    ColorSet, DeclarativeAbilityDef, FlexibleManaSymbol, ImplementationStatus, KeywordAbility,
    ManaColor, ManaCost, ModeSetDef, ObjectPredicateDef, PlayRestriction, PrintedManaCost,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CreatureStats {
    pub power: i16,
    pub toughness: i16,
}

/// Const-friendly storage for the ordered rules clauses of one card part.
///
/// A card with one clause stores it inline; cards with several clauses use a
/// promoted static slice, preserving source order without heap allocation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum CardAbilityList {
    None,
    One(AbilityDef),
    Many(&'static [AbilityDef]),
}

/// One reusable ability definition attached to a card part at a stable
/// position. The attachment supplies identity; [`AbilityDef`] supplies only
/// rules text and semantics.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AttachedAbilityDef {
    pub id: AbilityId,
    pub definition: AbilityDef,
}

impl AttachedAbilityDef {
    /// The stable cost-choice identity of a printed alternative-casting
    /// clause. Printed alternative costs use their owning ability's positional
    /// identity rather than a separately maintained identifier.
    #[must_use]
    pub const fn alternative_cost_id(self) -> Option<AlternativeCostId> {
        if matches!(
            self.definition.definition,
            DeclarativeAbilityDef::AlternativeCast(_)
        ) {
            Some(AlternativeCostId(self.id.0))
        } else {
            None
        }
    }

    /// The stable identity of a printed optional additional-cost clause.
    #[must_use]
    pub const fn additional_cost_id(self) -> Option<AdditionalCostId> {
        if matches!(
            self.definition.definition,
            DeclarativeAbilityDef::OptionalAdditionalCost(_)
        ) {
            Some(AdditionalCostId(self.id.0))
        } else {
            None
        }
    }

    /// Materializes the play-option view of a printed alternative cost.
    #[must_use]
    pub fn alternative_cost(self, card_mana_cost: Option<ManaCost>) -> Option<AlternativeCostDef> {
        let DeclarativeAbilityDef::AlternativeCast(definition) = self.definition.definition else {
            return None;
        };
        definition.alternative_cost(self.id, card_mana_cost)
    }

    /// Materializes the play-option view of a printed optional additional
    /// cost.
    #[must_use]
    pub fn additional_cost(self) -> Option<AdditionalCostDef> {
        let DeclarativeAbilityDef::OptionalAdditionalCost(definition) = self.definition.definition
        else {
            return None;
        };
        Some(definition.additional_cost(self.id))
    }
}

impl CardAbilityList {
    #[must_use]
    pub fn as_slice(&self) -> &[AbilityDef] {
        match self {
            Self::None => &[],
            Self::One(ability) => std::slice::from_ref(ability),
            Self::Many(abilities) => abilities,
        }
    }
}

/// Declarative rules metadata for one card or token face.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct CardRules {
    card_types: CardTypeSet,
    supertypes: [bool; CardSupertype::COUNT],
    subtypes: &'static [&'static str],
    pub(super) printed_mana_cost: PrintedManaCost,
    pub(super) starting_loyalty: Option<u16>,
    pub(super) creature_stats: Option<CreatureStats>,
    /// Whether the ordinary creature spell/permanent rules represented by
    /// `creature_stats` are part of this definition's executable coverage.
    creature_body_is_executable: bool,
    /// Ordered printed rules clauses. Abilities supplied by the rules, such as
    /// those intrinsic to basic land types, are derived by the game engine.
    abilities: CardAbilityList,
    colors: ColorSet,
    /// A timing or zone restriction the card prints on its own casting, which
    /// the derived play option carries into the runtime.
    play_restriction: PlayRestriction,
    /// "Spend only black mana on X." The X portion of the cost stops being
    /// generic and has to come from this colour.
    x_spend_restriction: Option<ManaColor>,
    /// The printed "enchant ..." restriction, for an Aura that does not
    /// announce what it will enchant as it is cast. Every ordinary Aura
    /// targets its host from the stack, so the restriction is read off that
    /// target slot; one that attaches later has no such slot and says it
    /// here instead.
    enchant: Option<ObjectPredicateDef>,
    /// The printed morph cost, which is what turning this permanent face up
    /// costs. It is read off the physical card rather than off the
    /// permanent's presented rules: a face-down permanent has no abilities,
    /// and turning it face up is a special action rather than one of them
    /// (CR 702.37b).
    pub(super) morph: Option<ManaCost>,
}

/// Whether any flexible symbol in this cost contains one colour.
const fn hybrid_includes(cost: ManaCost, color: ManaColor) -> bool {
    let mut index = 0;
    while index < FlexibleManaSymbol::COUNT {
        let symbol = FlexibleManaSymbol::ALL[index];
        if cost.flexible_count(symbol) > 0 && symbol.contains_color(color) {
            return true;
        }
        index += 1;
    }
    false
}

/// The one subtype every Vehicle prints.
static VEHICLE_SUBTYPES: &[&str] = &["Vehicle"];

const VEHICLE: &[u8] = b"Vehicle";

impl CardRules {
    /// The characteristic-level constructor used when a typed convenience
    /// constructor cannot represent a card face exactly.
    pub(in crate::card) const fn base(
        card_types: CardTypeSet,
        printed_mana_cost: PrintedManaCost,
    ) -> Self {
        let mana_cost = match printed_mana_cost {
            PrintedManaCost::None => ManaCost::new(0, 0),
            PrintedManaCost::Cost(cost) => cost,
        };
        let mut colors = ColorSet::empty();
        if mana_cost.white > 0 || hybrid_includes(mana_cost, ManaColor::White) {
            colors = colors.with(ManaColor::White);
        }
        if mana_cost.blue > 0 || hybrid_includes(mana_cost, ManaColor::Blue) {
            colors = colors.with(ManaColor::Blue);
        }
        if mana_cost.black > 0 || hybrid_includes(mana_cost, ManaColor::Black) {
            colors = colors.with(ManaColor::Black);
        }
        if mana_cost.red > 0 || hybrid_includes(mana_cost, ManaColor::Red) {
            colors = colors.with(ManaColor::Red);
        }
        if mana_cost.green > 0 || hybrid_includes(mana_cost, ManaColor::Green) {
            colors = colors.with(ManaColor::Green);
        }
        Self {
            card_types,
            supertypes: [false; CardSupertype::COUNT],
            subtypes: &[],
            printed_mana_cost,
            starting_loyalty: None,
            creature_stats: None,
            creature_body_is_executable: true,
            abilities: CardAbilityList::None,
            colors,
            play_restriction: PlayRestriction::Normal,
            x_spend_restriction: None,
            enchant: None,
            morph: None,
        }
    }

    /// Materializes the ordinary rules view of compact inline
    /// characteristics. Virtual and face-down values keep abilities behind a
    /// slice so a clause that creates another object does not make the
    /// declarative schema recursively sized.
    pub(super) const fn from_inline_characteristics(
        card_types: CardTypeSet,
        supertypes: [bool; CardSupertype::COUNT],
        subtypes: &'static [&'static str],
        colors: ColorSet,
        creature_stats: Option<CreatureStats>,
        abilities: &'static [AbilityDef],
    ) -> Self {
        let mut rules = Self::base(card_types, PrintedManaCost::None);
        rules.supertypes = supertypes;
        rules.subtypes = subtypes;
        rules.colors = colors;
        rules.creature_stats = creature_stats;
        rules.abilities = if abilities.is_empty() {
            CardAbilityList::None
        } else {
            CardAbilityList::Many(abilities)
        };
        rules
    }

    /// Records a printed morph cost. The permission to cast the card face
    /// down is a separate declared clause, so that a card carrying one
    /// without the other fails catalog validation rather than half-working.
    #[must_use]
    pub const fn with_morph(mut self, cost: ManaCost) -> Self {
        self.morph = Some(cost);
        self
    }

    #[must_use]
    pub const fn morph_cost(&self) -> Option<ManaCost> {
        self.morph
    }

    #[must_use]
    pub const fn new_creature(
        mana_cost: ManaCost,
        subtypes: &'static [&'static str],
        power: i16,
        toughness: i16,
    ) -> Self {
        let mut rules = Self::base(
            CardTypeSet::single(CardType::Creature),
            PrintedManaCost::Cost(mana_cost),
        );
        rules.subtypes = subtypes;
        rules.creature_stats = Some(CreatureStats { power, toughness });
        rules
    }

    /// A Vehicle (CR 301.7): an artifact with printed power and toughness
    /// that is not a creature until something crews it.
    ///
    /// The stats are on the card rather than granted by the crewing, which
    /// is why this is its own constructor: `with_creature_stats` is for
    /// cards whose type line already says creature, and a Vehicle's does
    /// not until an effect adds it.
    #[must_use]
    pub const fn new_vehicle(mana_cost: ManaCost, power: i16, toughness: i16) -> Self {
        let mut rules = Self::base(
            CardTypeSet::single(CardType::Artifact),
            PrintedManaCost::Cost(mana_cost),
        );
        rules.subtypes = VEHICLE_SUBTYPES;
        rules.creature_stats = Some(CreatureStats { power, toughness });
        rules
    }

    /// Keeps printed creature characteristics as metadata without exposing
    /// the baseline creature spell or permanent as executable behavior.
    #[must_use]
    pub const fn with_metadata_only_creature_body(mut self) -> Self {
        self.creature_body_is_executable = false;
        self
    }

    /// Whether this is a Vehicle, which is the one noncreature card type
    /// that prints power and toughness.
    #[must_use]
    pub const fn is_vehicle(&self) -> bool {
        let mut index = 0;
        while index < self.subtypes.len() {
            // Compared byte by byte because the coherence check that reads
            // this runs in a const context, where `==` on strings does not.
            let candidate = self.subtypes[index].as_bytes();
            if candidate.len() == VEHICLE.len() {
                let mut byte = 0;
                while byte < candidate.len() && candidate[byte] == VEHICLE[byte] {
                    byte += 1;
                }
                if byte == candidate.len() {
                    return true;
                }
            }
            index += 1;
        }
        false
    }

    #[must_use]
    pub const fn has_executable_creature_body(&self) -> bool {
        self.creature_stats.is_some() && self.creature_body_is_executable
    }

    /// Whether this printed creature exists only as catalog metadata and must
    /// not be exposed as a face-up gameplay object.
    #[must_use]
    pub const fn has_metadata_only_creature_body(&self) -> bool {
        self.creature_stats.is_some() && !self.creature_body_is_executable
    }

    /// Adapts an emblem's ability slice to shared runtime ability machinery
    /// without inventing card characteristics for the emblem itself.
    pub(crate) const fn from_emblem_abilities(abilities: &'static [AbilityDef]) -> Self {
        Self::base(CardTypeSet::empty(), PrintedManaCost::None).with_abilities(abilities)
    }

    #[must_use]
    pub const fn new_creature_without_mana_cost(
        subtypes: &'static [&'static str],
        power: i16,
        toughness: i16,
    ) -> Self {
        let mut rules = Self::base(
            CardTypeSet::single(CardType::Creature),
            PrintedManaCost::None,
        );
        rules.subtypes = subtypes;
        rules.creature_stats = Some(CreatureStats { power, toughness });
        rules
    }

    /// An enchantment creature: one card that is both, as the Glimmers and
    /// the Theros gods print it.
    #[must_use]
    pub const fn new_enchantment_creature(
        mana_cost: ManaCost,
        subtypes: &'static [&'static str],
        power: i16,
        toughness: i16,
    ) -> Self {
        let mut rules = Self::base(
            CardTypeSet::single(CardType::Enchantment).with(CardType::Creature),
            PrintedManaCost::Cost(mana_cost),
        );
        rules.subtypes = subtypes;
        rules.creature_stats = Some(CreatureStats { power, toughness });
        rules
    }

    #[must_use]
    pub const fn new_artifact_creature(
        mana_cost: ManaCost,
        subtypes: &'static [&'static str],
        power: i16,
        toughness: i16,
    ) -> Self {
        let mut rules = Self::base(
            CardTypeSet::single(CardType::Artifact).with(CardType::Creature),
            PrintedManaCost::Cost(mana_cost),
        );
        rules.subtypes = subtypes;
        rules.creature_stats = Some(CreatureStats { power, toughness });
        rules
    }

    #[must_use]
    pub const fn new_artifact_creature_without_mana_cost(
        subtypes: &'static [&'static str],
        power: i16,
        toughness: i16,
    ) -> Self {
        let mut rules = Self::base(
            CardTypeSet::single(CardType::Artifact).with(CardType::Creature),
            PrintedManaCost::None,
        );
        rules.subtypes = subtypes;
        rules.creature_stats = Some(CreatureStats { power, toughness });
        rules
    }

    #[must_use]
    pub const fn new_land(subtypes: &'static [&'static str]) -> Self {
        let mut rules = Self::base(CardTypeSet::single(CardType::Land), PrintedManaCost::None);
        rules.subtypes = subtypes;
        rules
    }

    #[must_use]
    pub const fn new_artifact(mana_cost: ManaCost) -> Self {
        Self::base(
            CardTypeSet::single(CardType::Artifact),
            PrintedManaCost::Cost(mana_cost),
        )
    }

    /// An artifact with no mana cost, which is what a noncreature artifact
    /// token is. Its subtypes carry the artifact type a card can name -- a
    /// Food is sacrificed by things that look for one.
    #[must_use]
    pub const fn new_artifact_without_mana_cost(subtypes: &'static [&'static str]) -> Self {
        let mut rules = Self::base(
            CardTypeSet::single(CardType::Artifact),
            PrintedManaCost::None,
        );
        rules.subtypes = subtypes;
        rules
    }

    /// An enchantment with no mana cost, which is what a Room with both
    /// doors locked is: it was never cast for anything, and until a door
    /// opens there is nothing to pay for.
    #[must_use]
    pub const fn new_enchantment_without_mana_cost() -> Self {
        Self::base(
            CardTypeSet::single(CardType::Enchantment),
            PrintedManaCost::None,
        )
    }

    #[must_use]
    pub const fn new_enchantment(mana_cost: ManaCost) -> Self {
        Self::base(
            CardTypeSet::single(CardType::Enchantment),
            PrintedManaCost::Cost(mana_cost),
        )
    }

    #[must_use]
    pub const fn new_instant(mana_cost: ManaCost) -> Self {
        Self::base(
            CardTypeSet::single(CardType::Instant),
            PrintedManaCost::Cost(mana_cost),
        )
    }

    #[must_use]
    pub const fn new_instant_without_mana_cost() -> Self {
        Self::base(
            CardTypeSet::single(CardType::Instant),
            PrintedManaCost::None,
        )
    }

    #[must_use]
    pub const fn new_sorcery(mana_cost: ManaCost) -> Self {
        Self::base(
            CardTypeSet::single(CardType::Sorcery),
            PrintedManaCost::Cost(mana_cost),
        )
    }

    #[must_use]
    pub const fn new_planeswalker(
        mana_cost: ManaCost,
        subtypes: &'static [&'static str],
        starting_loyalty: u16,
    ) -> Self {
        let mut rules = Self::base(
            CardTypeSet::single(CardType::Planeswalker),
            PrintedManaCost::Cost(mana_cost),
        );
        rules.subtypes = subtypes;
        rules.starting_loyalty = Some(starting_loyalty);
        rules
    }

    /// Creates a planeswalker back face, which has neither a printed mana cost
    /// nor a printed starting-loyalty value.
    #[must_use]
    pub const fn new_planeswalker_without_mana_cost(subtypes: &'static [&'static str]) -> Self {
        let mut rules = Self::base(
            CardTypeSet::single(CardType::Planeswalker),
            PrintedManaCost::None,
        );
        rules.subtypes = subtypes;
        rules
    }

    /// A back face's printed loyalty. It has no mana cost -- nothing casts
    /// it -- but it still prints a number, and a permanent that arrives on
    /// that face enters with it.
    #[must_use]
    pub const fn with_starting_loyalty(mut self, loyalty: u16) -> Self {
        self.starting_loyalty = Some(loyalty);
        self
    }

    #[must_use]
    pub const fn types(&self) -> CardTypeSet {
        self.card_types
    }

    #[must_use]
    pub const fn has_type(&self, card_type: CardType) -> bool {
        self.card_types.contains(card_type)
    }

    /// Compatibility spelling for clients that still expose one `kind`
    /// string instead of a collection of card types.
    #[must_use]
    pub fn kind_name(&self) -> String {
        self.card_types.kind_name()
    }

    #[must_use]
    pub const fn subtypes(&self) -> &'static [&'static str] {
        self.subtypes
    }

    #[must_use]
    pub const fn printed_mana_cost(&self) -> PrintedManaCost {
        self.printed_mana_cost
    }

    #[must_use]
    pub const fn mana_cost(&self) -> Option<ManaCost> {
        self.printed_mana_cost.as_option()
    }

    #[must_use]
    pub const fn starting_loyalty(&self) -> Option<u16> {
        self.starting_loyalty
    }

    #[must_use]
    pub const fn creature_stats(&self) -> Option<CreatureStats> {
        self.creature_stats
    }

    #[must_use]
    pub const fn colors(&self) -> [bool; 5] {
        self.colors.to_flags()
    }

    #[must_use]
    pub const fn color_set(&self) -> ColorSet {
        self.colors
    }

    #[must_use]
    pub const fn has_color(&self, color: ManaColor) -> bool {
        self.colors.contains(color)
    }

    /// Returns a concise explanation when internal or compatibility code has
    /// bypassed the type-specific constructors and produced contradictory
    /// characteristics.
    #[must_use]
    pub(in crate::card) const fn coherence_error(&self) -> Option<&'static str> {
        if self.card_types.is_empty() {
            return Some("a card part must have at least one card type");
        }
        let instant = self.has_type(CardType::Instant);
        let sorcery = self.has_type(CardType::Sorcery);
        if instant && sorcery {
            return Some("one card part cannot be both an instant and a sorcery");
        }
        if (instant || sorcery) && self.card_types.is_permanent() {
            return Some("an instant or sorcery cannot also be a permanent card type");
        }
        if self.has_type(CardType::Land) && !matches!(self.printed_mana_cost, PrintedManaCost::None)
        {
            return Some("a land cannot have a printed mana cost");
        }
        if self.has_type(CardType::Creature) && self.creature_stats.is_none() {
            return Some("a creature must have power and toughness");
        }
        // CR 208.1 was written before Vehicles: they are the one printed
        // exception, carrying power and toughness that mean nothing until
        // something crews them into being a creature.
        if !self.has_type(CardType::Creature) && self.creature_stats.is_some() && !self.is_vehicle()
        {
            return Some("a noncreature cannot have creature power and toughness");
        }
        if !self.has_type(CardType::Planeswalker) && self.starting_loyalty.is_some() {
            return Some("a nonplaneswalker cannot have starting loyalty");
        }
        if self.has_type(CardType::Planeswalker)
            && matches!(self.printed_mana_cost, PrintedManaCost::Cost(_))
            && self.starting_loyalty.is_none()
        {
            return Some("a castable planeswalker face must have starting loyalty");
        }
        None
    }

    #[cfg(test)]
    pub(in crate::card) const fn with_printed_mana_cost_for_test(
        mut self,
        printed_mana_cost: PrintedManaCost,
    ) -> Self {
        self.printed_mana_cost = printed_mana_cost;
        self
    }

    #[must_use]
    pub const fn with_ability(mut self, ability: AbilityDef) -> Self {
        self.abilities = CardAbilityList::One(ability);
        self
    }

    #[must_use]
    pub const fn with_abilities(mut self, abilities: &'static [AbilityDef]) -> Self {
        self.abilities = if abilities.is_empty() {
            CardAbilityList::None
        } else {
            CardAbilityList::Many(abilities)
        };
        self
    }

    #[must_use]
    pub fn ability_clauses(&self) -> &[AbilityDef] {
        self.abilities.as_slice()
    }

    pub(super) fn presentation_spell_modes(&self) -> Option<ModeSetDef> {
        let mut spell_abilities = self.ability_clauses().iter().filter_map(|ability| {
            let DeclarativeAbilityDef::Spell(spell) = ability.definition else {
                return None;
            };
            Some((ability, spell))
        });
        let (ability, spell) = spell_abilities.next()?;
        let modal = spell.modal()?;
        if spell_abilities.next().is_some() {
            return None;
        }
        let modes = modal
            .modes
            .iter()
            .copied()
            .enumerate()
            .map(|(index, mode)| {
                let id = ModeId::from_index(index)?;
                mode.mode_presentation(
                    id,
                    ability.is_executable(),
                    modal.mode_additional_mana_cost(id),
                )
            })
            .collect::<Option<Vec<_>>>()?;
        Some(ModeSetDef {
            minimum: modal.minimum,
            maximum: modal.maximum,
            may_repeat: modal.may_repeat,
            conditional_maximum: modal.conditional_maximum,
            modes,
        })
    }

    /// Iterates the ordered ability definitions with the positional identity
    /// they receive when attached to this card part.
    ///
    /// # Panics
    ///
    /// Panics when a rule set contains more than 256 clauses. Catalog
    /// validation rejects such a definition before it can enter a game.
    #[must_use]
    pub fn indexed_abilities(&self) -> impl ExactSizeIterator<Item = AttachedAbilityDef> + '_ {
        self.ability_clauses()
            .iter()
            .copied()
            .enumerate()
            .map(|(index, definition)| AttachedAbilityDef {
                id: AbilityId::from_index(index)
                    .expect("validated card parts contain at most 256 abilities"),
                definition,
            })
    }

    /// Looks up one attached ability by its positional identity.
    #[must_use]
    pub fn ability(&self, id: AbilityId) -> Option<&AbilityDef> {
        self.ability_clauses().get(id.index())
    }

    /// Renders the ordered card text from the same clauses used by execution
    /// and implementation auditing.
    #[must_use]
    pub fn rules_text(&self) -> Cow<'static, str> {
        match self.abilities {
            CardAbilityList::None => Cow::Borrowed(""),
            CardAbilityList::One(ability) => ability.rules_text(),
            CardAbilityList::Many(abilities) => Cow::Owned(
                abilities
                    .iter()
                    .map(AbilityDef::rules_text)
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
        }
    }

    #[must_use]
    pub fn implementation_status(&self) -> ImplementationStatus {
        // Playing a land and casting/using a modeled creature body are shared,
        // executable rules even when every card-specific clause is deferred.
        let mut has_full = self.has_type(CardType::Land) || self.has_executable_creature_body();
        let mut has_partial = false;
        let mut has_unimplemented = self.has_metadata_only_creature_body();
        for ability in self.ability_clauses() {
            match ability.implementation_status() {
                ImplementationStatus::Complete => has_full = true,
                ImplementationStatus::Partial => has_partial = true,
                ImplementationStatus::MetadataOnly => has_unimplemented = true,
            }
        }
        if has_partial || (has_full && has_unimplemented) {
            ImplementationStatus::Partial
        } else if has_unimplemented {
            ImplementationStatus::MetadataOnly
        } else {
            ImplementationStatus::Complete
        }
    }

    #[must_use]
    pub const fn with_supertype(mut self, supertype: CardSupertype) -> Self {
        self.supertypes[supertype.index()] = true;
        self
    }

    #[must_use]
    pub const fn with_subtypes(mut self, subtypes: &'static [&'static str]) -> Self {
        self.subtypes = subtypes;
        self
    }

    #[must_use]
    pub const fn has_supertype(&self, supertype: CardSupertype) -> bool {
        self.supertypes[supertype.index()]
    }

    #[must_use]
    pub fn has_subtype(&self, subtype: &str) -> bool {
        self.subtypes.contains(&subtype)
    }

    #[must_use]
    pub fn type_line(&self) -> String {
        let mut words = [
            CardSupertype::Basic,
            CardSupertype::Legendary,
            CardSupertype::Snow,
            CardSupertype::World,
        ]
        .into_iter()
        .filter(|supertype| self.has_supertype(*supertype))
        .map(CardSupertype::name)
        .collect::<Vec<_>>();
        let type_name = self.card_types.type_name();
        words.push(&type_name);
        let mut line = words.join(" ");
        if !self.subtypes.is_empty() {
            line.push_str(" — ");
            line.push_str(&self.subtypes.join(" "));
        }
        line
    }

    /// Overrides colors supplied by a color indicator or another printed
    /// characteristic that cannot be derived from the mana cost.
    #[must_use]
    pub const fn printed_colors(mut self, colors: &'static [ManaColor]) -> Self {
        self.colors = ColorSet::from_colors(colors);
        self
    }

    /// "Cast this spell only before the combat damage step."
    #[must_use]
    pub const fn cast_only_before_combat_damage(mut self) -> Self {
        self.play_restriction = PlayRestriction::BeforeCombatDamage;
        self
    }

    #[must_use]
    pub const fn cast_only_before_blockers_declared(mut self) -> Self {
        self.play_restriction = PlayRestriction::BeforeBlockersDeclared;
        self
    }

    #[must_use]
    pub const fn cast_only_during_opponents_upkeep(mut self) -> Self {
        self.play_restriction = PlayRestriction::OpponentsUpkeep;
        self
    }

    #[must_use]
    pub const fn cast_only_during_declare_attackers(mut self) -> Self {
        self.play_restriction = PlayRestriction::DeclareAttackersStep;
        self
    }

    #[must_use]
    pub const fn cast_only_after_an_opponents_upkeep(mut self) -> Self {
        self.play_restriction = PlayRestriction::OpponentsTurnAfterUpkeep;
        self
    }

    #[must_use]
    pub const fn play_restriction(&self) -> PlayRestriction {
        self.play_restriction
    }

    /// "Spend only <colour> mana on X."
    #[must_use]
    pub const fn spend_only_on_x(mut self, color: ManaColor) -> Self {
        self.x_spend_restriction = Some(color);
        self
    }

    #[must_use]
    pub const fn x_spend_restriction(&self) -> Option<ManaColor> {
        self.x_spend_restriction
    }

    /// "Enchant creature", for an Aura that attaches after it has resolved
    /// rather than announcing a host as it is cast.
    #[must_use]
    pub const fn enchanting(mut self, object: ObjectPredicateDef) -> Self {
        self.enchant = Some(object);
        self
    }

    #[must_use]
    pub const fn enchant(&self) -> Option<ObjectPredicateDef> {
        self.enchant
    }

    #[must_use]
    pub const fn with_type(mut self, card_type: CardType) -> Self {
        self.card_types = self.card_types.with(card_type);
        self
    }

    /// Supplies the printed power and toughness after a definition has been
    /// assembled with the creature card type.
    ///
    /// # Panics
    ///
    /// Panics when called on rules without the creature card type.
    #[must_use]
    pub const fn with_creature_stats(mut self, stats: CreatureStats) -> Self {
        assert!(
            self.has_type(CardType::Creature),
            "with_creature_stats() is only valid for creature rules"
        );
        self.creature_stats = Some(stats);
        self
    }

    /// Whether the printed clauses declare this keyword, regardless of its
    /// current implementation coverage.
    #[must_use]
    pub fn has_keyword(&self, expected: KeywordAbility) -> bool {
        self.ability_clauses().iter().any(
            |ability| matches!(ability.definition, DeclarativeAbilityDef::Keyword(actual) if actual == expected),
        )
    }

    /// Whether the card declares this keyword and the engine executes it.
    #[must_use]
    pub fn has_executable_keyword(&self, expected: KeywordAbility) -> bool {
        self.ability_clauses().iter().any(|ability| {
            ability.is_executable()
                && matches!(ability.definition, DeclarativeAbilityDef::Keyword(actual) if actual == expected)
        })
    }

    #[must_use]
    pub const fn unsupported() -> Self {
        Self::base(
            CardTypeSet::single(CardType::Artifact),
            PrintedManaCost::None,
        )
        .with_ability(AbilityDef::not_implemented(
            "Rules text is not implemented.",
            "The card's printed rules have not been cataloged or implemented.",
        ))
    }
}
