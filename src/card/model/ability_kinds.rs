use crate::ids::{ModeId, ObjectBindingIndex, TargetIndex};

use super::{
    AbilityCostDef, AbilityCostList, AbilityDef, AbilityTargetDef, BasicLandType, CardBehavior,
    CardSupertype, CardType, ConditionDef, CounterKind, EffectDef, ImplementationStatus,
    ObjectPredicateDef, ObjectQueryDef, PlayerRelation, ReplacementConditionDef,
    ReplacementEffectDef, ReplacementEventDef, TriggerEventDef, ValueDef, ZoneKind,
};

mod alternative_casts;
mod optional_additional_costs;

pub use alternative_casts::*;
pub use optional_additional_costs::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SpellAbilityDef {
    Nonmodal {
        targets: &'static [AbilityTargetDef],
        /// A nonmana cost paid as the spell is cast, chosen from the objects
        /// it names. Unlike a target this is spent rather than pointed at, so
        /// it is not checked again on resolution.
        additional_cost: Option<SpellAdditionalCostDef>,
        /// A printed "as an additional cost to cast this spell, pay N life".
        /// Life is spent rather than named, so unlike the cost above it
        /// selects nothing and enumerates nothing -- except when the amount
        /// is X, which the caster chooses as the spell is cast.
        life_cost: Option<SpellLifeCostDef>,
        /// Where the card goes after a successful resolution. This is part of
        /// a spell's shared stack procedure rather than an instruction that
        /// can move the resolving object while it is off the stack.
        resolution_destination: SpellResolutionDestinationDef,
    },
    Modal(ModalSpellDef),
}

/// The card's normal post-resolution destination after it has successfully
/// completed its instructions. Countered spells never use this: they follow
/// the countering effect's destination instead. A destination can also carry
/// an instruction that remains meaningful when the spell is a copy, such as
/// shuffling its owner's library.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SpellResolutionDestinationDef {
    Graveyard,
    /// Its owner's hand, which is what buyback buys (CR 702.27a).
    Hand,
    Exile,
    /// Exile the card only when this spell was cast from its owner's hand,
    /// and bury it otherwise. Rebound (CR 702.87a) is the only clause that
    /// says this, and it has to: the cast rebound offers comes from exile,
    /// and that one goes to the graveyard like any other spell.
    ExileIfCastFromHand,
    /// Exile the card "on an adventure" (CR 715.3d): its owner may cast it
    /// later from exile, as the creature it is on the other half. Only the
    /// alternate half of an Adventure card resolves this way.
    ExileOnAdventure,
    /// Exile the card and put these counters on its new object. A zone change
    /// happens before the counters are added, so prior-zone counters cannot
    /// leak into exile.
    ExileWithCounters(&'static [(CounterKind, u16)]),
    /// Move the card to its owner's library, then shuffle it. The shuffle is
    /// still part of the resolution when another effect replaces the move, or
    /// when this resolving spell is a copy with no card to move.
    LibraryShuffled,
}

/// Where an additional cost's count comes from.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum SpellAdditionalCostCountDef {
    /// The printed number, which is what almost every cost names.
    #[default]
    Printed,
    /// The X the spell is cast for. Flash of Insight's flashback exiles as
    /// many blue cards as its X, so the cost cannot be known until that X is
    /// chosen.
    ChosenX,
    /// One for each mode chosen past the first, which is escalate
    /// (CR 702.120a). A spell with one mode pays nothing extra.
    ModesBeyondFirst,
    /// Collect evidence N (CR 701.58a): not a number of cards at all, but a
    /// total mana value the chosen cards have to reach between them. How
    /// many that takes is whatever the graveyard makes it -- one card of
    /// mana value six, or six of one.
    TotalManaValueAtLeast(u8),
}

/// An additional cost that selects objects to spend. The zone decides what
/// spending means: a permanent on the battlefield is sacrificed, a card in a
/// graveyard is exiled, and a card in hand is discarded.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SpellAdditionalCostDef {
    pub object: ObjectPredicateDef,
    pub zone: ZoneKind,
    pub count: u8,
    /// Where the count comes from, when it is not the printed number above.
    pub counted: SpellAdditionalCostCountDef,
    pub spend: SpendModeDef,
    /// A second way to pay the same printed cost. "Sacrifice a creature or
    /// discard a card" is one cost with two ways to pay it, and which one is
    /// paid is settled as the spell is cast rather than asked afterwards --
    /// the chosen objects travel with the action either way.
    ///
    /// Held behind a reference so a cost stays one word wider than the
    /// predicate it carries. Both halves spend what they name the same way;
    /// what differs is the zone they name it in.
    pub or: Option<&'static SpellAdditionalCostDef>,
    /// A way to pay the same printed cost with life instead of objects.
    /// "Discard a card or pay 3 life" is one cost with two ways to pay it,
    /// and only one of them names anything: paying the life spends no
    /// object at all, which is how the payment is told apart afterwards.
    pub or_life: Option<u8>,
}

/// A printed "as an additional cost to cast this spell, pay N life".
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SpellLifeCostDef {
    pub amount: u8,
    /// The amount is X rather than the fixed number above. Toxic Deluge is
    /// cast for as much life as its caster is willing to spend, and every
    /// clause that reads X reads that same choice.
    pub amount_is_x: bool,
}

impl SpellLifeCostDef {
    #[must_use]
    pub const fn new(amount: u8) -> Self {
        Self {
            amount,
            amount_is_x: false,
        }
    }

    /// "Pay X life", chosen as the spell is cast.
    #[must_use]
    pub const fn variable() -> Self {
        Self {
            amount: 0,
            amount_is_x: true,
        }
    }
}

/// What spending a named object actually does to it.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum SpendModeDef {
    /// Whatever the object's zone implies, which is what almost every printed
    /// cost means: a permanent is sacrificed, a card in a graveyard is
    /// exiled, and a card in hand is discarded.
    #[default]
    ByZone,
    /// Exiled rather than put in a graveyard. "Exile a red card from your
    /// hand" spends it without ever making it a graveyard card.
    Exile,
    /// Returned to its owner's hand. The free-spell cycle pays this way: the
    /// lands come back rather than being lost.
    ReturnToHand,
}

impl SpellAdditionalCostDef {
    #[must_use]
    pub const fn new(object: ObjectPredicateDef, zone: ZoneKind, count: u8) -> Self {
        Self {
            or_life: None,
            object,
            zone,
            count,
            counted: SpellAdditionalCostCountDef::Printed,
            spend: SpendModeDef::ByZone,
            or: None,
        }
    }

    /// Where the number of objects comes from, when the printed count is
    /// not it -- an X, an escalate surcharge, or a total mana value to
    /// reach.
    #[must_use]
    pub const fn counted(mut self, counted: SpellAdditionalCostCountDef) -> Self {
        self.counted = counted;
        self
    }

    /// "... or <the other way>." The caster picks one of the two.
    #[must_use]
    pub const fn or(mut self, alternative: &'static SpellAdditionalCostDef) -> Self {
        self.or = Some(alternative);
        self
    }

    /// "... or pay N life." The same cost, paid with life rather than with
    /// anything the clause names.
    #[must_use]
    pub const fn or_pay_life(mut self, life: u8) -> Self {
        self.or_life = Some(life);
        self
    }

    /// The life every alternative way of paying this cost would take,
    /// smallest first. Empty when nothing about it may be paid with life.
    #[must_use]
    pub fn life_alternatives(self) -> Vec<u8> {
        let mut life = self
            .alternatives()
            .into_iter()
            .filter_map(|cost| cost.or_life)
            .collect::<Vec<_>>();
        life.sort_unstable();
        life.dedup();
        life
    }

    /// This cost and every alternative way of paying it, in printed order.
    #[must_use]
    pub fn alternatives(self) -> Vec<Self> {
        let mut costs = vec![self];
        let mut next = self.or;
        while let Some(cost) = next {
            costs.push(*cost);
            next = cost.or;
        }
        costs
    }

    /// The same cost, counted in X rather than in a printed number.
    #[must_use]
    pub const fn counted_in_x(mut self) -> Self {
        self.counted = SpellAdditionalCostCountDef::ChosenX;
        self
    }

    /// Escalate: the same cost, paid once for each mode past the first.
    #[must_use]
    pub const fn counted_per_extra_mode(mut self) -> Self {
        self.counted = SpellAdditionalCostCountDef::ModesBeyondFirst;
        self
    }

    #[must_use]
    pub const fn spent(mut self, spend: SpendModeDef) -> Self {
        self.spend = spend;
        self
    }
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
    /// A cost paid as the whole spell is cast, on top of its mana. Escalate
    /// is the only one printed on a modal spell, and it belongs to the spell
    /// rather than to any mode: what varies is how many modes were chosen.
    pub additional_cost: Option<SpellAdditionalCostDef>,
    /// A printed "if <condition> as you cast this spell, you may choose two
    /// instead". The larger maximum applies when the condition holds where
    /// the spell is offered; it never lowers the printed one, and the
    /// minimum is unaffected because the extra mode is always optional.
    pub conditional_maximum: Option<ConditionalModeMaximumDef>,
}

/// The two halves of "you may choose two instead": what has to be true, and
/// how many modes that allows.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ConditionalModeMaximumDef {
    pub condition: ConditionDef,
    pub maximum: u8,
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
            additional_cost: None,
            conditional_maximum: None,
        }
    }

    /// "If <condition> as you cast this spell, you may choose two instead."
    #[must_use]
    pub const fn with_conditional_maximum(mut self, condition: ConditionDef, maximum: u8) -> Self {
        self.conditional_maximum = Some(ConditionalModeMaximumDef { condition, maximum });
        self
    }

    #[must_use]
    pub const fn choose_one(modes: &'static [AbilityDef]) -> Self {
        Self::new(modes, 1, 1, false)
    }
}

impl SpellAbilityDef {
    #[must_use]
    pub const fn new() -> Self {
        Self::Nonmodal {
            targets: &[],
            additional_cost: None,
            life_cost: None,
            resolution_destination: SpellResolutionDestinationDef::Graveyard,
        }
    }

    /// Adds targets to an ordinary, nonmodal spell definition.
    ///
    /// # Panics
    ///
    /// Panics for a modal wrapper because each mode declares its own targets.
    #[must_use]
    pub const fn with_targets(self, targets: &'static [AbilityTargetDef]) -> Self {
        match self {
            Self::Nonmodal {
                additional_cost,
                life_cost,
                ..
            } => Self::Nonmodal {
                targets,
                additional_cost,
                life_cost,
                resolution_destination: self.resolution_destination(),
            },
            Self::Modal(_) => panic!("targets belong on modal spell branches"),
        }
    }

    /// # Panics
    ///
    /// Panics for a modal wrapper, which has no single cost to attach.
    #[must_use]
    pub const fn with_additional_cost(self, cost: SpellAdditionalCostDef) -> Self {
        match self {
            Self::Nonmodal {
                targets,
                life_cost,
                resolution_destination,
                ..
            } => Self::Nonmodal {
                targets,
                additional_cost: Some(cost),
                life_cost,
                resolution_destination,
            },
            // Escalate is printed on a modal spell and belongs to the whole
            // of it; what varies is how many modes it was cast with.
            Self::Modal(modal) => Self::Modal(ModalSpellDef {
                additional_cost: Some(cost),
                ..modal
            }),
        }
    }

    /// "As an additional cost to cast this spell, pay N life."
    ///
    /// # Panics
    ///
    /// Panics for a modal wrapper, which has no single cost to attach.
    #[must_use]
    pub const fn with_life_cost(self, cost: SpellLifeCostDef) -> Self {
        match self {
            Self::Nonmodal {
                targets,
                additional_cost,
                resolution_destination,
                ..
            } => Self::Nonmodal {
                targets,
                additional_cost,
                life_cost: Some(cost),
                resolution_destination,
            },
            Self::Modal(_) => panic!("a life cost belongs to a whole spell"),
        }
    }

    #[must_use]
    pub const fn life_cost(self) -> Option<SpellLifeCostDef> {
        match self {
            Self::Nonmodal { life_cost, .. } => life_cost,
            Self::Modal(_) => None,
        }
    }

    #[must_use]
    pub const fn additional_cost(self) -> Option<SpellAdditionalCostDef> {
        match self {
            Self::Nonmodal {
                additional_cost, ..
            } => additional_cost,
            Self::Modal(modal) => modal.additional_cost,
        }
    }

    /// Changes the ordinary destination used after this spell resolves. Modal
    /// wrappers share one spell object and therefore one destination.
    #[must_use]
    pub const fn with_resolution_destination(
        self,
        destination: SpellResolutionDestinationDef,
    ) -> Self {
        match self {
            Self::Nonmodal {
                targets,
                additional_cost,
                life_cost,
                ..
            } => Self::Nonmodal {
                targets,
                additional_cost,
                life_cost,
                resolution_destination: destination,
            },
            Self::Modal(modal) => Self::Modal(modal),
        }
    }

    #[must_use]
    pub const fn resolution_destination(self) -> SpellResolutionDestinationDef {
        match self {
            Self::Nonmodal {
                resolution_destination,
                ..
            } => resolution_destination,
            Self::Modal(_) => SpellResolutionDestinationDef::Graveyard,
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
            Self::Nonmodal { targets, .. } => targets,
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

/// When a printed "Activate only ..." clause allows an ability to be
/// activated. This restricts the window; it does not change priority, so an
/// ability that is also sorcery-speed still needs an empty stack.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ActivationTimingDef {
    /// Any time its controller has priority, which is the printed default.
    #[default]
    Any,
    /// Only during a turn its controller is taking.
    YourTurn,
    /// Only during the upkeep step of a turn its controller is taking.
    YourUpkeep,
    /// Only during an upkeep step, whoever is taking the turn. Tolaria opens
    /// on both, which is what makes it an answer to an attack.
    AnyUpkeep,
    /// Only during the combat phase, whoever is taking the turn. Every step
    /// from the beginning of combat through the end of combat, which is what
    /// lets an animated artifact block as well as attack.
    DuringCombat,
    /// Only during the end-of-combat step. Combat is over and the damage is
    /// dealt, so a land shooting an attacker here is finishing off something
    /// that survived rather than stopping it.
    EndOfCombat,
    /// Only when its controller could cast a sorcery: their own main phase,
    /// with the stack empty. Unlike the windows above, this one does depend
    /// on the stack, because that is what "as a sorcery" means.
    SorcerySpeed,
    /// The ninjutsu window: the priority round the attack declaration
    /// opens. It is the only moment an attacker can be both unblocked and
    /// still able to be swapped out, which is the whole of the mechanic.
    AfterAttackersDeclared,
    /// "Activate only before the combat damage step." The window is open all
    /// turn until damage is about to be dealt, on either player's turn, which
    /// is what makes the ability something the attacker can be surprised by.
    BeforeCombatDamage,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActivatedAbilityDef {
    pub source_zones: &'static [ZoneKind],
    pub costs: AbilityCostList,
    pub targets: &'static [AbilityTargetDef],
    pub procedure: AbilityProcedureDef,
    pub timing: ActivationTimingDef,
    /// How many times a printed "only once each turn" or "no more than twice
    /// each turn" clause allows this ability to be activated per turn from
    /// one object. `None` is the ordinary unlimited case.
    pub activation_limit: Option<u8>,
    /// Exhaust (CR 702.184a): "Activate each exhaust ability only once."
    /// A cap on the permanent's whole lifetime rather than on its turn,
    /// which is why it is counted apart from the limit above -- that one
    /// clears when the turn does, and this one never clears.
    pub exhaust: bool,
    /// Whether anyone may activate it, not just the permanent's controller.
    /// The permanent stays the ability's source whoever pays, so the damage
    /// it deals is still the permanent's damage.
    pub any_player_may_activate: bool,
    /// A printed "activate only if ..." restriction, checked where the
    /// activation is offered rather than where it resolves -- an ability
    /// whose condition is false is not a legal action at all, which is what
    /// threshold means.
    pub condition: Option<&'static TriggerConditionDef>,
    /// "Choose one --", for an activated ability that prints modes. The
    /// choice is made as the ability is activated rather than as it
    /// resolves (CR 601.2b), so it travels with the action; each mode is a
    /// clause of its own with its own targets, exactly as a modal spell's
    /// modes are.
    pub modes: Option<ModalSpellDef>,
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
            timing: ActivationTimingDef::Any,
            activation_limit: None,
            exhaust: false,
            any_player_may_activate: false,
            condition: None,
            modes: None,
        }
    }

    /// "Choose one --" on an activated ability.
    #[must_use]
    pub const fn with_modes(mut self, modes: ModalSpellDef) -> Self {
        self.modes = Some(modes);
        self
    }

    /// "Any player may activate this ability." The permanent stays the
    /// source, so what it does is still the permanent's doing.
    #[must_use]
    pub const fn open_to_any_player(mut self) -> Self {
        self.any_player_may_activate = true;
        self
    }

    /// "Activate only if ...", the restriction threshold and its relatives
    /// print. It gates the offer, not the resolution.
    #[must_use]
    pub const fn only_if(mut self, condition: &'static TriggerConditionDef) -> Self {
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

    #[must_use]
    pub const fn with_timing(mut self, timing: ActivationTimingDef) -> Self {
        self.timing = timing;
        self
    }

    /// Exhaust: once per object, for as long as that object is there.
    #[must_use]
    pub const fn exhausting(mut self) -> Self {
        self.exhaust = true;
        self
    }

    #[must_use]
    pub const fn with_activation_limit(mut self, limit: u8) -> Self {
        self.activation_limit = Some(limit);
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TriggeredAbilityDef {
    pub source_zones: &'static [ZoneKind],
    pub event: TriggerEventDef,
    /// "This ability triggers only once each turn." A cap on how often one
    /// object's copy of this ability may trigger per turn; `None` is the
    /// ordinary unlimited case. Checked where the trigger is captured, so a
    /// capped ability past its count simply does not trigger.
    pub trigger_limit: Option<u8>,
    pub targets: &'static [AbilityTargetDef],
    /// An Oracle exception to rule 608.2b: the ability continues resolving
    /// even when every target has become illegal. Illegal targets remain
    /// unaffected; only the ordinary all-targets-illegal early exit is
    /// suppressed.
    pub resolves_with_illegal_targets: bool,
    pub procedure: AbilityProcedureDef,
    /// Held by reference so that this definition stays small enough to pass
    /// around by value alongside a captured trigger.
    pub condition: Option<&'static TriggerConditionDef>,
    /// "Choose one --" on a trigger. The mode is chosen as the ability is
    /// put onto the stack (CR 603.3c), which is why it is settled during
    /// placement beside target selection rather than as the ability
    /// resolves. Exactly one mode: a trigger carries one effect, so a
    /// clause choosing two would have nowhere to put the second.
    pub modes: Option<ModalSpellDef>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StaticAbilityDef {
    pub source_zones: &'static [ZoneKind],
}

/// A replacement ability changes how an event happens and never uses the
/// stack. It is modeled separately from a triggered ability even when both
/// watch the same event.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ReplacementAbilityDef {
    pub source_zones: &'static [ZoneKind],
    pub event: ReplacementEventDef,
    pub condition: Option<ReplacementConditionDef>,
    /// Whether the affected player may decline to apply this replacement.
    pub optional: bool,
}

impl ReplacementAbilityDef {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            source_zones: &[ZoneKind::Battlefield],
            event: ReplacementEventDef::SourceEntersBattlefield,
            condition: None,
            optional: false,
        }
    }

    #[must_use]
    pub const fn with_event(mut self, event: ReplacementEventDef) -> Self {
        self.event = event;
        self
    }

    #[must_use]
    pub const fn with_condition(mut self, condition: ReplacementConditionDef) -> Self {
        self.condition = Some(condition);
        self
    }

    #[must_use]
    pub const fn optional(mut self) -> Self {
        self.optional = true;
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
    OptionalAdditionalCost(OptionalAdditionalCostAbilityDef),
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
pub enum AbilityProgramDef {
    Effects(EffectDef),
    Replacement(ReplacementEffectDef),
}

/// The structured program and the resolver responsible for executing it.
///
/// Replacement programs are typed separately because they mutate a
/// prospective event and preserve that event across any decisions they make;
/// they are not resolving stack effects.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AbilityEffectDef {
    pub definition: AbilityProgramDef,
    pub execution: EffectExecutionDef,
}

impl AbilityEffectDef {
    #[must_use]
    pub const fn declarative(definition: EffectDef) -> Self {
        Self {
            definition: AbilityProgramDef::Effects(definition),
            execution: EffectExecutionDef::Declarative,
        }
    }

    #[must_use]
    pub const fn replacement_program(definition: ReplacementEffectDef) -> Self {
        Self {
            definition: AbilityProgramDef::Replacement(definition),
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
        match (self.execution, self.definition) {
            (EffectExecutionDef::Declarative, AbilityProgramDef::Effects(definition)) => {
                Some(definition)
            }
            (EffectExecutionDef::Declarative, AbilityProgramDef::Replacement(_))
            | (EffectExecutionDef::Custom(_) | EffectExecutionDef::CardOwned, _) => None,
        }
    }

    #[must_use]
    pub const fn declarative_replacement(self) -> Option<ReplacementEffectDef> {
        match (self.execution, self.definition) {
            (EffectExecutionDef::Declarative, AbilityProgramDef::Replacement(definition)) => {
                Some(definition)
            }
            (EffectExecutionDef::Declarative, AbilityProgramDef::Effects(_))
            | (EffectExecutionDef::Custom(_) | EffectExecutionDef::CardOwned, _) => None,
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

include!("ability_kinds/conditions.rs");
include!("ability_kinds/keywords.rs");
include!("ability_kinds/triggered.rs");
