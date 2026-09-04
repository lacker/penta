use crate::ids::{Binding, ModeId, TargetIndex};

use super::{
    AbilityCostDef, AbilityCostList, AbilityDef, AbilityTargetDef, BasicLandType, CardSupertype,
    CardType, ConditionDef, CostQuantityDef, CounterKind, EffectDef, ManaCost, ObjectPredicateDef,
    ObjectQueryDef, ObjectSetCountConditionDef, PlayerRelation, ReplacementConditionDef,
    ReplacementEffectDef, ReplacementEventDef, TriggerEventDef, ValueDef, ZoneKind,
};

mod alternative_casts;
mod optional_additional_costs;
mod pregame;

pub use alternative_casts::*;
pub use optional_additional_costs::*;
pub use pregame::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SpellAbilityDef {
    Nonmodal {
        targets: &'static [AbilityTargetDef],
        /// A semantic cost paid as the spell is cast, in addition to its
        /// ordinary mana cost. Any objects it names are payment choices, not
        /// targets, and are spent before the spell can resolve.
        additional_cost: Option<SpellAdditionalCostDef>,
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
    /// Rebound (CR 702.87a): exile the card only when this spell was cast from
    /// hand and install its next-upkeep free-cast offer. A rebounded cast
    /// comes from exile, so it goes to the graveyard like any other spell.
    Rebound,
    /// Exile the card "on an adventure" (CR 715.3d): its owner may cast it
    /// later from exile, as the creature it is on the other half. Only the
    /// alternate half of an Adventure card resolves this way.
    ExileOnAdventure,
    /// Exile the resolving spell and put these counters on the successor card.
    /// This remains a destination-level sequencing primitive: ordinary effect
    /// composition cannot name the new object after the stack-to-exile move,
    /// and adding the counters to the old spell would lose them at zone change.
    ExileWithCounters(&'static [(CounterKind, u16)]),
    /// Move the card to its owner's library, then shuffle it. The shuffle is
    /// still part of the resolution when another effect replaces the move, or
    /// when this resolving spell is a copy with no card to move.
    LibraryShuffled,
}

/// A semantic action or composition of actions paid while casting a spell.
///
/// Named game actions remain explicit here even when they share lower-level
/// zone-change machinery. Sacrificing and discarding are not interchangeable
/// with an arbitrary move to a graveyard: rules and triggers can care which
/// action paid the cost.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SpellAdditionalCostDef {
    /// Pay this mana cost the computed number of times.
    PayMana {
        cost: ManaCost,
        quantity: CostQuantityDef,
    },
    PayLife(CostQuantityDef),
    Sacrifice {
        object: ObjectPredicateDef,
        quantity: CostQuantityDef,
    },
    Discard {
        object: ObjectPredicateDef,
        quantity: CostQuantityDef,
    },
    Exile {
        object: ObjectPredicateDef,
        from: ZoneKind,
        quantity: CostQuantityDef,
    },
    ReturnToHand {
        object: ObjectPredicateDef,
        quantity: CostQuantityDef,
    },
    /// Tap untapped permanents you control. Unlike an activated ability's
    /// tap-symbol cost, this does not care how long a creature has been under
    /// your control.
    Tap {
        object: ObjectPredicateDef,
        quantity: CostQuantityDef,
    },
    /// Forage (CR 701.59): exile three cards from your graveyard or sacrifice
    /// a Food. Card definitions name the keyword while payment expands it
    /// into its semantic exile and sacrifice actions.
    Forage,
    /// Pay every child cost.
    All(&'static [SpellAdditionalCostDef]),
    /// Choose exactly one child cost to pay.
    Choice(&'static [SpellAdditionalCostDef]),
}

impl SpellAdditionalCostDef {
    #[must_use]
    pub const fn pay_mana(cost: ManaCost) -> Self {
        Self::pay_mana_times(cost, CostQuantityDef::Fixed(1))
    }

    #[must_use]
    pub const fn pay_mana_times(cost: ManaCost, quantity: CostQuantityDef) -> Self {
        Self::PayMana { cost, quantity }
    }

    #[must_use]
    pub const fn pay_life(quantity: CostQuantityDef) -> Self {
        Self::PayLife(quantity)
    }

    #[must_use]
    pub const fn sacrifice(object: ObjectPredicateDef, quantity: CostQuantityDef) -> Self {
        Self::Sacrifice { object, quantity }
    }

    #[must_use]
    pub const fn discard(object: ObjectPredicateDef, quantity: CostQuantityDef) -> Self {
        Self::Discard { object, quantity }
    }

    #[must_use]
    pub const fn exile(
        object: ObjectPredicateDef,
        from: ZoneKind,
        quantity: CostQuantityDef,
    ) -> Self {
        Self::Exile {
            object,
            from,
            quantity,
        }
    }

    #[must_use]
    pub const fn return_to_hand(object: ObjectPredicateDef, quantity: CostQuantityDef) -> Self {
        Self::ReturnToHand { object, quantity }
    }

    #[must_use]
    pub const fn tap(object: ObjectPredicateDef, quantity: CostQuantityDef) -> Self {
        Self::Tap { object, quantity }
    }

    #[must_use]
    pub const fn forage() -> Self {
        Self::Forage
    }

    #[must_use]
    pub const fn all(costs: &'static [Self]) -> Self {
        Self::All(costs)
    }

    #[must_use]
    pub const fn choice(costs: &'static [Self]) -> Self {
        Self::Choice(costs)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ModalSpellDef {
    /// Each mode is an ordinary spell ability. Its positional index supplies
    /// the stable [`ModeId`] used by casting and presentation.
    pub modes: ModalModeListDef,
    pub minimum: u8,
    pub maximum: u8,
    /// Some spells explicitly allow the same mode to be chosen more than once.
    pub may_repeat: bool,
    /// The single additional cost Escalate charges for each mode chosen
    /// beyond the first. The cast planner derives the number of payments from
    /// the selected modes, just as Spree derives costs from its modal shape.
    pub escalate_cost: Option<SpellAdditionalCostDef>,
    /// A printed "if <condition> as you cast this spell, you may choose two
    /// instead". The larger maximum applies when the condition holds where
    /// the spell is offered; it never lowers the printed one, and the
    /// minimum is unaffected because the extra mode is always optional.
    pub conditional_maximum: Option<ConditionalModeMaximumDef>,
}

/// The ordered branches of a modal ability.
///
/// Ordinary modes are selected instructions. A mode with a listed cost also
/// owns the mana paid for choosing that instruction; keeping the two together
/// prevents a cast signature from selecting one while paying for another.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ModalModeListDef {
    Ordinary(&'static [AbilityDef]),
    WithAdditionalManaCosts(&'static [(ManaCost, AbilityDef)]),
}

pub enum ModalModeIter {
    Ordinary(std::slice::Iter<'static, AbilityDef>),
    WithAdditionalManaCosts(std::slice::Iter<'static, (ManaCost, AbilityDef)>),
}

impl Iterator for ModalModeIter {
    type Item = &'static AbilityDef;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Ordinary(modes) => modes.next(),
            Self::WithAdditionalManaCosts(modes) => modes.next().map(|(_, mode)| mode),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::Ordinary(modes) => modes.size_hint(),
            Self::WithAdditionalManaCosts(modes) => modes.size_hint(),
        }
    }
}

impl ExactSizeIterator for ModalModeIter {}

impl ModalModeListDef {
    #[must_use]
    pub const fn len(self) -> usize {
        match self {
            Self::Ordinary(modes) => modes.len(),
            Self::WithAdditionalManaCosts(modes) => modes.len(),
        }
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn iter(self) -> ModalModeIter {
        match self {
            Self::Ordinary(modes) => ModalModeIter::Ordinary(modes.iter()),
            Self::WithAdditionalManaCosts(modes) => {
                ModalModeIter::WithAdditionalManaCosts(modes.iter())
            }
        }
    }

    #[must_use]
    pub fn get(self, index: usize) -> Option<&'static AbilityDef> {
        match self {
            Self::Ordinary(modes) => modes.get(index),
            Self::WithAdditionalManaCosts(modes) => match modes.get(index) {
                Some((_, mode)) => Some(mode),
                None => None,
            },
        }
    }

    #[must_use]
    pub fn additional_mana_cost(self, mode: ModeId) -> Option<ManaCost> {
        let Self::WithAdditionalManaCosts(modes) = self else {
            return None;
        };
        modes.get(mode.index()).map(|(cost, _)| *cost)
    }

    #[must_use]
    pub const fn has_additional_mana_costs(self) -> bool {
        matches!(self, Self::WithAdditionalManaCosts(_))
    }
}

impl IntoIterator for ModalModeListDef {
    type Item = &'static AbilityDef;
    type IntoIter = ModalModeIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
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
            modes: ModalModeListDef::Ordinary(modes),
            minimum,
            maximum,
            may_repeat,
            escalate_cost: None,
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

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    /// # Panics
    ///
    /// Panics when more modes are supplied than the runtime mode-count field
    /// can represent.
    pub const fn spree(modes: &'static [(ManaCost, AbilityDef)]) -> Self {
        assert!(modes.len() <= u8::MAX as usize);
        Self {
            modes: ModalModeListDef::WithAdditionalManaCosts(modes),
            minimum: 1,
            maximum: modes.len() as u8,
            may_repeat: false,
            escalate_cost: None,
            conditional_maximum: None,
        }
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    /// # Panics
    ///
    /// Panics when no modes are supplied or the mode count does not fit the
    /// runtime mode-count field.
    pub const fn escalate(cost: SpellAdditionalCostDef, modes: &'static [AbilityDef]) -> Self {
        assert!(!modes.is_empty());
        assert!(modes.len() <= u8::MAX as usize);
        Self {
            modes: ModalModeListDef::Ordinary(modes),
            minimum: 1,
            maximum: modes.len() as u8,
            may_repeat: false,
            escalate_cost: Some(cost),
            conditional_maximum: None,
        }
    }

    #[must_use]
    pub fn mode_additional_mana_cost(self, mode: ModeId) -> Option<ManaCost> {
        self.modes.additional_mana_cost(mode)
    }
}

impl SpellAbilityDef {
    #[must_use]
    pub const fn new() -> Self {
        Self::Nonmodal {
            targets: &[],
            additional_cost: None,
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
                additional_cost, ..
            } => Self::Nonmodal {
                targets,
                additional_cost,
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
                resolution_destination,
                ..
            } => Self::Nonmodal {
                targets,
                additional_cost: Some(cost),
                resolution_destination,
            },
            Self::Modal(_) => panic!("ordinary additional costs do not belong on modal spells"),
        }
    }

    #[must_use]
    pub const fn additional_cost(self) -> Option<SpellAdditionalCostDef> {
        match self {
            Self::Nonmodal {
                additional_cost, ..
            } => additional_cost,
            Self::Modal(_) => None,
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
                ..
            } => Self::Nonmodal {
                targets,
                additional_cost,
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
    /// The ninjutsu window, which opens once blockers have been declared
    /// and runs to the end of combat: before blockers are declared an
    /// attacker is neither blocked nor unblocked, so there is nothing the
    /// cost could name. The variant is named for the declaration it used to
    /// key off; what it means is "after blockers".
    AfterAttackersDeclared,
    /// "Activate only before the combat damage step." The window is open all
    /// turn until damage is about to be dealt, on either player's turn, which
    /// is what makes the ability something the attacker can be surprised by.
    BeforeCombatDamage,
}

/// Which players may activate an activated ability on a permanent.
///
/// This is separate from control of the ability after activation: whoever is
/// allowed to activate it becomes the stack object's controller, while the
/// permanent remains the source.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ActivationPermissionDef {
    /// The ordinary rule: only the permanent's controller may activate it.
    #[default]
    Controller,
    /// "Any player may activate this ability."
    AnyPlayer,
    /// "Only your opponents may activate this ability."
    Opponents,
}

// Each flag is a separate printed sentence about the same ability, so they
// stay separate fields rather than being folded into one shape.
#[allow(clippy::struct_excessive_bools)]
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
    /// "Activate only as an instant." A mana ability may ordinarily be
    /// activated whenever mana is called for, including while a spell is
    /// being paid for; this one may be activated only at a time its
    /// controller could cast an instant, which is to say while they hold
    /// priority. Lion's Eye Diamond is the whole point of the clause: it
    /// cannot be cracked to pay for the spell being cast, because the hand
    /// it discards would be the hand that spell came out of.
    pub only_as_instant: bool,
    /// Exhaust (CR 702.184a): "Activate each exhaust ability only once."
    /// A cap on the permanent's whole lifetime rather than on its turn,
    /// which is why it is counted apart from the limit above -- that one
    /// clears when the turn does, and this one never clears.
    pub exhaust: bool,
    /// Who may activate it. The permanent stays the ability's source whoever
    /// pays, so the damage it deals is still the permanent's damage.
    pub activation_permission: ActivationPermissionDef,
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
    /// "This ability costs {1} less to activate for each legendary creature
    /// you control." A discount the ability prints about itself rather than
    /// one a permanent hands out, which is why it is not a
    /// [`CostModificationDef`]: those are read off the battlefield, and a
    /// channel ability is activated from hand, where its own card has no
    /// static abilities at all.
    pub cost_reduction: Option<AbilityCostReductionDef>,
    /// Whether this ability is cycling (CR 702.29), typecycling included.
    /// Cycling raises its own event as it is activated, and a clause
    /// watching for one has to be able to tell it from every other ability
    /// that pays by discarding its own card from hand -- channel prints
    /// exactly that cost and is not cycling.
    pub cycling: bool,
}

/// What an ability's own printed discount takes off its activation cost.
///
/// Like every cost reduction it touches generic mana only (CR 601.2f), so
/// the coloured half of a channel cost survives any board.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AbilityCostReductionDef {
    pub amount: ValueDef,
    /// The least mana the cost may be left with, for the abilities that
    /// print "this effect can't reduce the mana in that cost to less than
    /// one mana". Nothing is stopping a reduction without one from taking
    /// the generic half away entirely.
    pub minimum: u16,
}

impl ActivatedAbilityDef {
    /// The same ability, marked as the cycling it is. Only the cycling
    /// constructors call this: it is what the discard raises its event on.
    #[must_use]
    pub const fn cycling(mut self) -> Self {
        self.cycling = true;
        self
    }

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
            only_as_instant: false,
            exhaust: false,
            activation_permission: ActivationPermissionDef::Controller,
            condition: None,
            modes: None,
            cost_reduction: None,
            cycling: false,
        }
    }

    /// "Activate only as an instant."
    #[must_use]
    pub const fn only_as_instant(mut self) -> Self {
        self.only_as_instant = true;
        self
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
        self.activation_permission = ActivationPermissionDef::AnyPlayer;
        self
    }

    /// "Only your opponents may activate this ability."
    #[must_use]
    pub const fn only_opponents_may_activate(mut self) -> Self {
        self.activation_permission = ActivationPermissionDef::Opponents;
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

    /// "This ability costs {N} less to activate for each ..."
    #[must_use]
    pub const fn with_cost_reduction(mut self, amount: ValueDef, minimum: u16) -> Self {
        self.cost_reduction = Some(AbilityCostReductionDef { amount, minimum });
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
    /// Whether this ability expires after it replaces one event. This is the
    /// declarative lifetime of "the next time" replacements granted by a
    /// resolving effect, rather than state stored by the card that granted it.
    pub once: bool,
}

impl ReplacementAbilityDef {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            source_zones: &[ZoneKind::Battlefield],
            event: ReplacementEventDef::SourceEntersBattlefield,
            condition: None,
            optional: false,
            once: false,
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
    pub const fn once(mut self) -> Self {
        self.once = true;
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

/// The rules category and structural procedure of an ability. Printed text
/// lives on [`AbilityDef`] so every clause has one canonical text string.
/// Identity is supplied only when a definition is attached.
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
    Pregame(PregameAbilityDef),
    Keyword(KeywordAbility),
    /// A permission the card grants the deck it is built into. It is read
    /// while a deck is assembled and is silent during play.
    DeckConstruction(DeckConstructionDef),
}

/// The structured program of an ability.
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
}

impl AbilityEffectDef {
    #[must_use]
    pub const fn declarative(definition: EffectDef) -> Self {
        Self {
            definition: AbilityProgramDef::Effects(definition),
        }
    }

    #[must_use]
    pub const fn replacement_program(definition: ReplacementEffectDef) -> Self {
        Self {
            definition: AbilityProgramDef::Replacement(definition),
        }
    }

    #[must_use]
    pub const fn declarative_definition(self) -> Option<EffectDef> {
        match self.definition {
            AbilityProgramDef::Effects(definition) => Some(definition),
            AbilityProgramDef::Replacement(_) => None,
        }
    }

    #[must_use]
    pub const fn declarative_replacement(self) -> Option<ReplacementEffectDef> {
        match self.definition {
            AbilityProgramDef::Replacement(definition) => Some(definition),
            AbilityProgramDef::Effects(_) => None,
        }
    }
}

include!("ability_kinds/deck_construction.rs");
include!("ability_kinds/conditions.rs");
include!("ability_kinds/keywords.rs");
include!("ability_kinds/triggered.rs");
