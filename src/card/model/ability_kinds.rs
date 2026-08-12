use crate::ids::{AbilityId, AlternativeCostId, ModeId, TargetIndex};

use super::{
    AbilityCostDef, AbilityCostList, AbilityDef, AbilityTargetDef, AlternativeCostDef,
    CardBehavior, EffectDef, ImplementationStatus, ManaColor, ManaCost, ObjectPredicateDef,
    ObjectQueryDef, PlayerRelation, ReplacementEventDef, TriggerEventDef, ZoneKind,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SpellAbilityDef {
    Nonmodal {
        targets: &'static [AbilityTargetDef],
    },
    Modal(ModalSpellDef),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ModalSpellDef {
    /// Each mode is an ordinary spell ability. Its positional index supplies
    /// the stable [`ModeId`] used by casting and presentation.
    pub modes: &'static [AbilityDef],
    pub minimum: u8,
    pub maximum: u8,
    /// Some spells explicitly allow the same mode to be chosen more than once.
    pub may_repeat: bool,
}

impl ModalSpellDef {
    #[must_use]
    pub const fn new(
        modes: &'static [AbilityDef],
        minimum: u8,
        maximum: u8,
        may_repeat: bool,
    ) -> Self {
        Self {
            modes,
            minimum,
            maximum,
            may_repeat,
        }
    }

    #[must_use]
    pub const fn choose_one(modes: &'static [AbilityDef]) -> Self {
        Self::new(modes, 1, 1, false)
    }
}

impl SpellAbilityDef {
    #[must_use]
    pub const fn new() -> Self {
        Self::Nonmodal { targets: &[] }
    }

    /// Adds targets to an ordinary, nonmodal spell definition.
    ///
    /// # Panics
    ///
    /// Panics for a modal wrapper because each mode declares its own targets.
    #[must_use]
    pub const fn with_targets(self, targets: &'static [AbilityTargetDef]) -> Self {
        match self {
            Self::Nonmodal { .. } => Self::Nonmodal { targets },
            Self::Modal(_) => panic!("targets belong on modal spell branches"),
        }
    }

    #[must_use]
    pub const fn modal_spell(
        modes: &'static [AbilityDef],
        minimum: u8,
        maximum: u8,
        may_repeat: bool,
    ) -> Self {
        Self::Modal(ModalSpellDef::new(modes, minimum, maximum, may_repeat))
    }

    /// Returns targets declared directly by a nonmodal spell. Modal wrappers
    /// have no direct targets; selected branches supply them instead.
    #[must_use]
    pub const fn targets(self) -> &'static [AbilityTargetDef] {
        match self {
            Self::Nonmodal { targets } => targets,
            Self::Modal(_) => &[],
        }
    }

    #[must_use]
    pub const fn modal(self) -> Option<ModalSpellDef> {
        match self {
            Self::Nonmodal { .. } => None,
            Self::Modal(modal) => Some(modal),
        }
    }

    #[must_use]
    pub fn mode(self, id: ModeId) -> Option<&'static AbilityDef> {
        self.modal()?.modes.get(id.index())
    }
}

impl Default for SpellAbilityDef {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AbilityProcedureDef {
    /// Costs, action generation, trigger capture, and stack handling use the
    /// shared rules procedures for this ability category.
    Shared,
    /// Transitional compatibility path for an ability whose category is
    /// known but whose surrounding rules procedure still lives in legacy
    /// card behavior.
    Legacy,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActivatedAbilityDef {
    pub source_zones: &'static [ZoneKind],
    pub costs: AbilityCostList,
    pub targets: &'static [AbilityTargetDef],
    pub procedure: AbilityProcedureDef,
}

impl ActivatedAbilityDef {
    #[must_use]
    pub const fn new(costs: &'static [AbilityCostDef]) -> Self {
        Self::with_costs(AbilityCostList::borrowed(costs))
    }

    #[must_use]
    pub(crate) const fn with_costs(costs: AbilityCostList) -> Self {
        Self {
            source_zones: &[ZoneKind::Battlefield],
            costs,
            targets: &[],
            procedure: AbilityProcedureDef::Shared,
        }
    }

    #[must_use]
    pub const fn with_source_zones(mut self, source_zones: &'static [ZoneKind]) -> Self {
        self.source_zones = source_zones;
        self
    }

    #[must_use]
    pub const fn with_targets(mut self, targets: &'static [AbilityTargetDef]) -> Self {
        self.targets = targets;
        self
    }

    #[must_use]
    pub const fn with_procedure(mut self, procedure: AbilityProcedureDef) -> Self {
        self.procedure = procedure;
        self
    }
}

/// Whether a condition has to hold for every matching player or just one.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum QuantifierDef {
    Every,
    Any,
}

/// How a counted amount is compared against a printed number.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ComparisonDef {
    Less,
    LessOrEqual,
    Equal,
    GreaterOrEqual,
    Greater,
}

/// An intervening-if condition, the "if ..." clause a trigger reads before it
/// does anything. Rule 603.4 checks such a condition twice: once when the
/// ability would go on the stack, and again as it resolves.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TriggerConditionDef {
    /// How many objects the query matches, against a printed number.
    ObjectCount {
        query: ObjectQueryDef,
        comparison: ComparisonDef,
        amount: u8,
    },
    /// Whose turn it is, relative to the ability's controller.
    ActivePlayer(PlayerRelation),
    /// How many spells a matching player cast during the turn before this
    /// one. "No spells were cast last turn" is every player at zero, and "a
    /// player cast two or more" is any player at two.
    SpellsCastLastTurn {
        /// Whether every matching player has to satisfy the comparison or
        /// only one. "No spells were cast last turn" is every player at zero;
        /// "a player cast two or more" is one player at two.
        quantifier: QuantifierDef,
        player: PlayerRelation,
        comparison: ComparisonDef,
        amount: u8,
    },
    /// How much loyalty the ability's own source has left.
    SourceLoyalty {
        comparison: ComparisonDef,
        amount: u8,
    },
    /// How many times this ability has been activated from its source this
    /// turn, counting the activation now resolving.
    SourceActivationsThisTurn {
        comparison: ComparisonDef,
        amount: u8,
    },
    /// Whether this ability's own source has dealt damage to an opponent of
    /// its controller at any point this turn, by any means.
    SourceDealtDamageToOpponentThisTurn,
    /// Whether the ability's own source is tapped, using last-known
    /// information if it has left the battlefield.
    SourceIsTapped,
    /// Whether what a target slot points at still matches. Read when the
    /// condition is checked, so a delayed effect can ask about the target as
    /// it is then rather than as it was.
    TargetMatches {
        slot: TargetIndex,
        object: ObjectPredicateDef,
    },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TriggeredAbilityDef {
    pub source_zones: &'static [ZoneKind],
    pub event: TriggerEventDef,
    pub targets: &'static [AbilityTargetDef],
    pub procedure: AbilityProcedureDef,
    /// Held by reference so that this definition stays small enough to pass
    /// around by value alongside a captured trigger.
    pub condition: Option<&'static TriggerConditionDef>,
}

impl TriggeredAbilityDef {
    #[must_use]
    pub const fn new(event: TriggerEventDef) -> Self {
        Self {
            source_zones: &[ZoneKind::Battlefield],
            event,
            targets: &[],
            procedure: AbilityProcedureDef::Shared,
            condition: None,
        }
    }

    #[must_use]
    pub const fn with_condition(mut self, condition: &'static TriggerConditionDef) -> Self {
        self.condition = Some(condition);
        self
    }

    #[must_use]
    pub const fn with_source_zones(mut self, source_zones: &'static [ZoneKind]) -> Self {
        self.source_zones = source_zones;
        self
    }

    #[must_use]
    pub const fn with_targets(mut self, targets: &'static [AbilityTargetDef]) -> Self {
        self.targets = targets;
        self
    }

    #[must_use]
    pub const fn with_procedure(mut self, procedure: AbilityProcedureDef) -> Self {
        self.procedure = procedure;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StaticAbilityDef {
    pub source_zones: &'static [ZoneKind],
}

/// The rules procedure and mana cost supplied by a printed
/// alternative-casting keyword.
///
/// A play option exposes a derived [`AlternativeCostDef`] whose identity is
/// the positional [`AbilityId`] of this clause. An overload clause uses its
/// [`AbilityDef::effect`] as the targetless text-replacement result; flashback
/// uses `EffectDef::None` and changes where the card may be cast and where it
/// goes after the stack.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AlternativeCastAbilityDef {
    pub mana_cost: AlternativeCastManaCostDef,
    pub kind: AlternativeCastKindDef,
    /// Rules text for the spell as modified by this alternative, when the
    /// procedure changes its visible instructions (as overload does).
    pub stack_text: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AlternativeCastKindDef {
    Flashback,
    Overload,
    /// Cast from hand only in the window opened by drawing the card, as the
    /// first card drawn that turn.
    Miracle,
}

/// How an alternative-casting ability determines the cost it supplies.
///
/// Printed abilities normally carry a fixed cost. A granted ability such as
/// Snapcaster Mage's flashback instead reads the mana cost of the card that
/// gained it, after a concrete play option has selected the spell form.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AlternativeCastManaCostDef {
    Fixed(ManaCost),
    ThisCardManaCost,
}

impl AlternativeCastManaCostDef {
    #[must_use]
    pub const fn resolve(self, card_mana_cost: Option<ManaCost>) -> Option<ManaCost> {
        match self {
            Self::Fixed(mana_cost) => Some(mana_cost),
            Self::ThisCardManaCost => card_mana_cost,
        }
    }
}

impl AlternativeCastKindDef {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Flashback => "Flashback",
            Self::Overload => "Overload",
            Self::Miracle => "Miracle",
        }
    }
}

impl AlternativeCastAbilityDef {
    #[must_use]
    pub fn rules_text(self) -> String {
        match (self.kind, self.mana_cost) {
            (AlternativeCastKindDef::Flashback, AlternativeCastManaCostDef::Fixed(mana_cost)) => {
                format!(
                    "Flashback {mana_cost} (You may cast this card from your graveyard for its flashback cost. Then exile it.)",
                )
            }
            (
                AlternativeCastKindDef::Flashback,
                AlternativeCastManaCostDef::ThisCardManaCost,
            ) => "Flashback—the flashback cost is equal to this card's mana cost. (You may cast this card from your graveyard for its flashback cost. Then exile it.)".into(),
            (AlternativeCastKindDef::Overload, AlternativeCastManaCostDef::Fixed(mana_cost)) => {
                format!(
                    "Overload {mana_cost} (You may cast this spell for its overload cost. If you do, change \"target\" in its text to \"each.\")",
                )
            }
            (
                AlternativeCastKindDef::Overload,
                AlternativeCastManaCostDef::ThisCardManaCost,
            ) => "Overload—the overload cost is equal to this card's mana cost. (You may cast this spell for its overload cost. If you do, change \"target\" in its text to \"each.\")".into(),
            (AlternativeCastKindDef::Miracle, AlternativeCastManaCostDef::Fixed(mana_cost)) => {
                format!(
                    "Miracle {mana_cost} (You may cast this card for its miracle cost when you draw it if it's the first card you drew this turn.)",
                )
            }
            (
                AlternativeCastKindDef::Miracle,
                AlternativeCastManaCostDef::ThisCardManaCost,
            ) => "Miracle—the miracle cost is equal to this card's mana cost. (You may cast this card for its miracle cost when you draw it if it's the first card you drew this turn.)".into(),
        }
    }

    #[must_use]
    pub fn alternative_cost(
        self,
        ability: AbilityId,
        card_mana_cost: Option<ManaCost>,
    ) -> Option<AlternativeCostDef> {
        Some(AlternativeCostDef {
            id: AlternativeCostId(ability.0),
            label: self.kind.label().into(),
            mana_cost: self.mana_cost.resolve(card_mana_cost)?,
        })
    }
}

/// A replacement ability changes how an event happens and never uses the
/// stack. It is modeled separately from a triggered ability even when both
/// watch the same event.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ReplacementAbilityDef {
    pub source_zones: &'static [ZoneKind],
    pub event: ReplacementEventDef,
}

impl ReplacementAbilityDef {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            source_zones: &[ZoneKind::Battlefield],
            event: ReplacementEventDef::EntersBattlefield,
        }
    }

    #[must_use]
    pub const fn with_event(mut self, event: ReplacementEventDef) -> Self {
        self.event = event;
        self
    }

    #[must_use]
    pub const fn with_source_zones(mut self, source_zones: &'static [ZoneKind]) -> Self {
        self.source_zones = source_zones;
        self
    }
}

impl Default for ReplacementAbilityDef {
    fn default() -> Self {
        Self::new()
    }
}

/// A rules-defined action a player may take without using the stack, such as
/// turning a face-down permanent face up. This is deliberately distinct from
/// both activated abilities and mana abilities; its timing category is never
/// inferred from its cost or effect.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SpecialActionDef {
    pub source_zones: &'static [ZoneKind],
    pub costs: &'static [AbilityCostDef],
}

impl SpecialActionDef {
    #[must_use]
    pub const fn new(source_zones: &'static [ZoneKind], costs: &'static [AbilityCostDef]) -> Self {
        Self {
            source_zones,
            costs,
        }
    }
}

impl StaticAbilityDef {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            source_zones: &[ZoneKind::Battlefield],
        }
    }

    #[must_use]
    pub const fn with_source_zones(mut self, source_zones: &'static [ZoneKind]) -> Self {
        self.source_zones = source_zones;
        self
    }
}

impl Default for StaticAbilityDef {
    fn default() -> Self {
        Self::new()
    }
}

/// A keyword ability carried as an ordinary, ordered rules clause.
///
/// The clause's [`AbilityCoverageDef`] says whether the engine currently
/// executes the keyword. This keeps unimplemented keywords such as banding
/// visible and accurately reflected in aggregate coverage without hiding them
/// in card-level booleans.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum KeywordAbility {
    Flying,
    Trample,
    Haste,
    FirstStrike,
    DoubleStrike,
    Banding,
    Vigilance,
    Defender,
    Deathtouch,
    Lifelink,
    Reach,
    Flash,
    Hexproof,
    Intimidate,
    Undying,
    Indestructible,
    /// "Attacks each combat if able." Not a printed keyword, but it behaves
    /// like one: a static requirement with no parameters that several cards
    /// state in the same words.
    AttacksEachCombatIfAble,
    Mountainwalk,
    Forestwalk,
    ProtectionFrom(ManaColor),
}

impl KeywordAbility {
    /// A dense index for the keywords that carry no parameter, so a set of
    /// them fits in a bitmask. Protection is excluded: it is really one
    /// keyword per color.
    #[must_use]
    pub const fn simple_index(self) -> Option<u32> {
        Some(match self {
            Self::Flying => 0,
            Self::Trample => 1,
            Self::Haste => 2,
            Self::FirstStrike => 3,
            Self::DoubleStrike => 4,
            Self::Banding => 5,
            Self::Vigilance => 6,
            Self::Defender => 7,
            Self::Deathtouch => 8,
            Self::Lifelink => 9,
            Self::Reach => 10,
            Self::Flash => 11,
            Self::Hexproof => 12,
            Self::Intimidate => 13,
            Self::Undying => 14,
            Self::Mountainwalk => 15,
            Self::AttacksEachCombatIfAble => 16,
            Self::Forestwalk => 17,
            Self::Indestructible => 18,
            Self::ProtectionFrom(_) => return None,
        })
    }
}

/// The rules category and structural procedure of an ability. Text and
/// implementation coverage live on [`AbilityDef`] so every printed clause has
/// one canonical text string regardless of how it executes. Identity is
/// supplied only when a definition is attached.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DeclarativeAbilityDef {
    Spell(SpellAbilityDef),
    ActivatedMana(ActivatedAbilityDef),
    TriggeredMana(TriggeredAbilityDef),
    Activated(ActivatedAbilityDef),
    Triggered(TriggeredAbilityDef),
    Static(StaticAbilityDef),
    Replacement(ReplacementAbilityDef),
    AlternativeCast(AlternativeCastAbilityDef),
    SpecialAction(SpecialActionDef),
    Keyword(KeywordAbility),
    /// Transitional structural marker for a clause still dispatched through
    /// the owning card's legacy custom behavior.
    Legacy,
}

/// How an ability's declared effect is executed.
///
/// Coverage is deliberately not represented here: a custom effect can be
/// complete or partial, and a declarative effect can likewise have a gap in
/// its costs, targeting, timing, or another non-effect portion of the clause.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EffectExecutionDef {
    Declarative,
    Custom(CardBehavior),
    /// A resolver the card itself supplies, reached through the set module's
    /// ability bindings rather than a shared dispatch key. The clause says so
    /// here so that a reader, the coverage view, and the shared-runtime
    /// boundary all learn how it executes from the clause itself.
    CardOwned,
}

/// The structured effect and the resolver responsible for executing it.
///
/// Custom execution retains the structured definition as documentation and a
/// migration target, but the shared resolver must not execute that definition
/// until the execution kind becomes [`EffectExecutionDef::Declarative`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AbilityEffectDef {
    pub definition: EffectDef,
    pub execution: EffectExecutionDef,
}

impl AbilityEffectDef {
    #[must_use]
    pub const fn declarative(definition: EffectDef) -> Self {
        Self {
            definition,
            execution: EffectExecutionDef::Declarative,
        }
    }

    #[must_use]
    pub const fn with_execution(mut self, execution: EffectExecutionDef) -> Self {
        self.execution = execution;
        self
    }

    #[must_use]
    pub const fn declarative_definition(self) -> Option<EffectDef> {
        match self.execution {
            EffectExecutionDef::Declarative => Some(self.definition),
            EffectExecutionDef::Custom(_) | EffectExecutionDef::CardOwned => None,
        }
    }

    #[must_use]
    pub const fn custom_behavior(self) -> Option<CardBehavior> {
        match self.execution {
            EffectExecutionDef::Custom(behavior) => Some(behavior),
            EffectExecutionDef::Declarative | EffectExecutionDef::CardOwned => None,
        }
    }
}

/// Clause-level implementation coverage, independent of effect dispatch.
///
/// An explanation is optional only for an ordinary complete declarative
/// clause. Complete custom and compatibility clauses keep a note explaining
/// their implementation; partial and metadata-only clauses explain the gap.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AbilityCoverageDef {
    pub status: ImplementationStatus,
    pub explanation: Option<&'static str>,
}

impl AbilityCoverageDef {
    #[must_use]
    pub const fn complete() -> Self {
        Self {
            status: ImplementationStatus::Complete,
            explanation: None,
        }
    }

    #[must_use]
    pub const fn explained_complete(explanation: &'static str) -> Self {
        Self {
            status: ImplementationStatus::Complete,
            explanation: Some(explanation),
        }
    }

    #[must_use]
    pub const fn partial(explanation: &'static str) -> Self {
        Self {
            status: ImplementationStatus::Partial,
            explanation: Some(explanation),
        }
    }

    #[must_use]
    pub const fn metadata_only(explanation: &'static str) -> Self {
        Self {
            status: ImplementationStatus::MetadataOnly,
            explanation: Some(explanation),
        }
    }

    #[must_use]
    pub const fn is_executable(self) -> bool {
        !matches!(self.status, ImplementationStatus::MetadataOnly)
    }
}
