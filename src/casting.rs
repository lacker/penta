//! Runtime choices that determine a spell's locked stack characteristics.
//!
//! Playing a particular card part, choosing rules-text modes, and selecting an
//! alternative cost are independent decisions. [`CastChoices`] carries the
//! proposed decisions; after validation, [`CastSignature`] freezes the choices
//! that copy effects must preserve.

use std::error::Error;
use std::fmt;

use crate::action::Target;
use crate::card::{FlexibleManaSymbol, SpellForm};
use crate::ids::GameObjectId;
use crate::ids::{AdditionalCostId, AlternativeCostId, ModeId, PlayOptionId, TargetSlotId};

/// Semantic cost choices made while casting a spell.
///
/// This deliberately does not contain mana sources, sacrificed objects, or
/// discarded cards. Those are payments and must not be performed again when a
/// spell is copied. The vectors preserve order and multiplicity for cards that
/// allow an additional cost to be selected more than once.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct CostConfiguration {
    alternative: Option<AlternativeCostId>,
    additional: Vec<AdditionalCostId>,
}

/// Copies of one flexible mana symbol paid through its announced alternative.
///
/// For a two-brid symbol such as `{2/B}`, `count` is how many copies are paid
/// with two generic mana. For a Phyrexian symbol, it is how many copies are
/// paid with 2 life. Ordinary hybrid and colorless-hybrid symbols have only
/// mana alternatives and therefore never appear here.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FlexibleManaPayment {
    symbol: FlexibleManaSymbol,
    count: u16,
}

impl FlexibleManaPayment {
    #[must_use]
    pub const fn new(symbol: FlexibleManaSymbol, count: u16) -> Self {
        Self { symbol, count }
    }

    #[must_use]
    pub const fn symbol(self) -> FlexibleManaSymbol {
        self.symbol
    }

    #[must_use]
    pub const fn count(self) -> u16 {
        self.count
    }
}

/// Explicit alternatives selected while paying a spell's flexible symbols.
///
/// Colored-mana allocation for ordinary hybrid stays with the mana planner;
/// this records the branches that change a total cost or spend life. It is a
/// payment fact, not a copiable characteristic, and is deliberately omitted
/// from [`CastSignature`].
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct ManaPaymentChoice {
    alternatives: Vec<FlexibleManaPayment>,
}

static EMPTY_MANA_PAYMENT: ManaPaymentChoice = ManaPaymentChoice {
    alternatives: Vec::new(),
};

impl ManaPaymentChoice {
    #[must_use]
    pub fn new(alternatives: Vec<FlexibleManaPayment>) -> Self {
        Self { alternatives }
    }

    #[must_use]
    pub fn alternatives(&self) -> &[FlexibleManaPayment] {
        &self.alternatives
    }
}

impl CostConfiguration {
    #[must_use]
    pub fn new(alternative: Option<AlternativeCostId>, additional: Vec<AdditionalCostId>) -> Self {
        Self {
            alternative,
            additional,
        }
    }

    #[must_use]
    pub const fn alternative(&self) -> Option<AlternativeCostId> {
        self.alternative
    }

    #[must_use]
    pub fn additional(&self) -> &[AdditionalCostId] {
        &self.additional
    }
}

/// Targets selected for one independently addressable target slot.
///
/// A slot contains a vector because some effects use one grammatical target
/// instruction with variable cardinality. Different slots may contain the
/// same target; for example, fused Turn // Burn may target one creature with
/// both halves.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TargetSelection {
    slot: TargetSlotId,
    targets: Vec<Target>,
    /// How much of a divided total each target takes, parallel to `targets`.
    /// Empty for every ordinary slot, where the effect applies whole to each
    /// target rather than being split among them.
    amounts: Vec<u16>,
}

impl TargetSelection {
    #[must_use]
    pub fn new(slot: TargetSlotId, targets: Vec<Target>) -> Self {
        Self {
            slot,
            targets,
            amounts: Vec::new(),
        }
    }

    /// A slot whose effect is divided as its controller chooses. Each target
    /// takes the amount at the same position.
    #[must_use]
    pub fn divided(slot: TargetSlotId, targets: Vec<Target>, amounts: Vec<u16>) -> Self {
        debug_assert_eq!(targets.len(), amounts.len());
        Self {
            slot,
            targets,
            amounts,
        }
    }

    #[must_use]
    pub fn single(slot: TargetSlotId, target: Target) -> Self {
        Self::new(slot, vec![target])
    }

    #[must_use]
    pub const fn slot(&self) -> TargetSlotId {
        self.slot
    }

    #[must_use]
    pub fn targets(&self) -> &[Target] {
        &self.targets
    }

    #[must_use]
    pub fn amounts(&self) -> &[u16] {
        &self.amounts
    }

    /// Replaces the targets while preserving the amount assigned to each
    /// target position. Target-changing effects cannot redistribute a spell
    /// whose damage or counters were divided as it was cast.
    #[must_use]
    pub fn with_replaced_targets(&self, targets: Vec<Target>) -> Option<Self> {
        (self.targets.len() == targets.len()).then(|| Self {
            slot: self.slot,
            targets,
            amounts: self.amounts.clone(),
        })
    }

    /// How much this target takes of a divided total, or nothing when the
    /// slot is not divided.
    #[must_use]
    pub fn amount_for(&self, target: Target) -> Option<u16> {
        self.targets
            .iter()
            .position(|candidate| *candidate == target)
            .and_then(|index| self.amounts.get(index).copied())
    }
}

/// Player-supplied choices for a proposed cast action.
///
/// The game engine must validate these against the selected card definition
/// before constructing a [`CastSignature`]. In particular, it derives the
/// authoritative [`SpellForm`] from `play_option` rather than accepting a form
/// from the player.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CastChoices {
    play_option: PlayOptionId,
    modes: Box<[ModeId]>,
    costs: CostConfiguration,
    mana_payment: Option<Box<ManaPaymentChoice>>,
    x: u16,
    targets: Vec<TargetSelection>,
    /// Cards revealed from the caster's hand and spliced onto this spell
    /// (CR 702.47a). Each one adds its own clause and its own targets to
    /// what is being cast, and its splice cost to what is being paid; the
    /// cards themselves stay in hand.
    spliced: Box<[GameObjectId]>,
}

impl Default for CastChoices {
    fn default() -> Self {
        Self::new(PlayOptionId::DEFAULT)
    }
}

impl CastChoices {
    #[must_use]
    pub fn new(play_option: PlayOptionId) -> Self {
        Self {
            play_option,
            modes: Box::default(),
            costs: CostConfiguration::default(),
            mana_payment: None,
            x: 0,
            targets: Vec::new(),
            spliced: Box::default(),
        }
    }

    #[must_use]
    pub fn with_modes(mut self, modes: Vec<ModeId>) -> Self {
        self.modes = modes.into_boxed_slice();
        self
    }

    #[must_use]
    pub fn with_costs(mut self, costs: CostConfiguration) -> Self {
        self.costs = costs;
        self
    }

    #[must_use]
    pub fn with_mana_payment(mut self, payment: ManaPaymentChoice) -> Self {
        self.mana_payment = if payment.alternatives().is_empty() {
            None
        } else {
            Some(Box::new(payment))
        };
        self
    }

    #[must_use]
    pub const fn with_x(mut self, x: u16) -> Self {
        self.x = x;
        self
    }

    #[must_use]
    pub fn with_targets(mut self, targets: Vec<TargetSelection>) -> Self {
        self.targets = targets;
        self
    }

    #[must_use]
    pub fn with_spliced(mut self, spliced: Vec<GameObjectId>) -> Self {
        self.spliced = spliced.into_boxed_slice();
        self
    }

    #[must_use]
    pub fn spliced(&self) -> &[GameObjectId] {
        &self.spliced
    }

    #[must_use]
    pub const fn play_option(&self) -> PlayOptionId {
        self.play_option
    }

    #[must_use]
    pub fn modes(&self) -> &[ModeId] {
        &self.modes
    }

    #[must_use]
    pub const fn costs(&self) -> &CostConfiguration {
        &self.costs
    }

    #[must_use]
    pub fn mana_payment(&self) -> &ManaPaymentChoice {
        self.mana_payment.as_deref().unwrap_or(&EMPTY_MANA_PAYMENT)
    }

    #[must_use]
    pub const fn x(&self) -> u16 {
        self.x
    }

    #[must_use]
    pub fn targets(&self) -> &[TargetSelection] {
        &self.targets
    }

    /// Returns the targets in printed slot order for compatibility with rules
    /// code that does not yet need to distinguish the slots.
    pub fn iter_targets(&self) -> impl Iterator<Item = &Target> {
        self.targets
            .iter()
            .flat_map(|selection| selection.targets.iter())
    }
}

/// The immutable casting choices carried by a spell on the stack.
///
/// Modes are stored in canonical printed order, with repeated IDs preserving
/// multiplicity. Copying a spell clones every copiable casting choice; actual
/// payments such as Phyrexian life are intentionally absent. A copy effect
/// such as Fork may then replace only target values through
/// [`Self::copy_with_targets`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CastSignature {
    play_option: PlayOptionId,
    form: SpellForm,
    modes: Box<[ModeId]>,
    costs: CostConfiguration,
    x: u16,
    targets: Vec<TargetSelection>,
    /// The cards spliced onto this spell, frozen with the rest of what was
    /// announced: their clauses are part of what it resolves.
    spliced: Box<[GameObjectId]>,
}

impl CastSignature {
    pub(crate) fn target_shape_replacement(
        original: &[TargetSelection],
        targets: Vec<TargetSelection>,
    ) -> Result<Vec<TargetSelection>, TargetReplacementError> {
        validate_target_replacement(original, &targets)?;
        Ok(targets)
    }

    /// Freezes choices after the engine has validated `choices` and derived
    /// the resulting spell form from its cataloged play option.
    #[must_use]
    pub fn from_validated_choices(form: SpellForm, choices: CastChoices) -> Self {
        Self {
            play_option: choices.play_option,
            form,
            modes: choices.modes,
            costs: choices.costs,
            x: choices.x,
            targets: choices.targets,
            spliced: choices.spliced,
        }
    }

    #[must_use]
    pub const fn play_option(&self) -> PlayOptionId {
        self.play_option
    }

    #[must_use]
    pub fn spliced(&self) -> &[GameObjectId] {
        &self.spliced
    }

    #[must_use]
    pub const fn form(&self) -> &SpellForm {
        &self.form
    }

    #[must_use]
    pub fn modes(&self) -> &[ModeId] {
        &self.modes
    }

    #[must_use]
    pub const fn costs(&self) -> &CostConfiguration {
        &self.costs
    }

    #[must_use]
    pub const fn x(&self) -> u16 {
        self.x
    }

    #[must_use]
    pub fn targets(&self) -> &[TargetSelection] {
        &self.targets
    }

    /// Returns the targets in printed slot order.
    pub fn iter_targets(&self) -> impl Iterator<Item = &Target> {
        self.targets
            .iter()
            .flat_map(|selection| selection.targets.iter())
    }

    /// Produces a signature with replacement target values.
    ///
    /// The replacement must retain the same ordered slots and target counts.
    /// Target legality is deliberately checked by the game engine: retaining
    /// an original target that has since become illegal is permitted when a
    /// spell is copied, and it will be rechecked when the copy resolves.
    ///
    /// # Errors
    ///
    /// Returns [`TargetReplacementError`] when the replacement changes the
    /// number, order, identity, or cardinality of the original target slots.
    pub fn with_replaced_targets(
        &self,
        targets: Vec<TargetSelection>,
    ) -> Result<Self, TargetReplacementError> {
        validate_target_replacement(&self.targets, &targets)?;

        let mut copied = self.clone();
        copied.targets = targets;
        Ok(copied)
    }

    /// Compatibility spelling for callers performing the copy procedure.
    ///
    /// # Errors
    ///
    /// Returns [`TargetReplacementError`] when the replacement changes the
    /// ordered shape of the original target selections.
    pub fn copy_with_targets(
        &self,
        targets: Vec<TargetSelection>,
    ) -> Result<Self, TargetReplacementError> {
        self.with_replaced_targets(targets)
    }
}

fn validate_target_replacement(
    original: &[TargetSelection],
    replacement: &[TargetSelection],
) -> Result<(), TargetReplacementError> {
    if original.len() != replacement.len() {
        return Err(TargetReplacementError::SelectionCount {
            expected: original.len(),
            actual: replacement.len(),
        });
    }
    for (index, (original, replacement)) in original.iter().zip(replacement).enumerate() {
        if original.slot != replacement.slot {
            return Err(TargetReplacementError::Slot {
                index,
                expected: original.slot,
                actual: replacement.slot,
            });
        }
        if original.targets.len() != replacement.targets.len() {
            return Err(TargetReplacementError::TargetCount {
                slot: original.slot,
                expected: original.targets.len(),
                actual: replacement.targets.len(),
            });
        }
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TargetReplacementError {
    SelectionCount {
        expected: usize,
        actual: usize,
    },
    Slot {
        index: usize,
        expected: TargetSlotId,
        actual: TargetSlotId,
    },
    TargetCount {
        slot: TargetSlotId,
        expected: usize,
        actual: usize,
    },
}

impl fmt::Display for TargetReplacementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SelectionCount { expected, actual } => write!(
                formatter,
                "a target change must keep {expected} target selections, but received {actual}"
            ),
            Self::Slot {
                index,
                expected,
                actual,
            } => write!(
                formatter,
                "target selection {index} must keep slot {expected:?}, but used {actual:?}"
            ),
            Self::TargetCount {
                slot,
                expected,
                actual,
            } => write!(
                formatter,
                "target slot {slot:?} must keep {expected} targets, but received {actual}"
            ),
        }
    }
}

impl Error for TargetReplacementError {}

#[cfg(test)]
mod tests {
    use super::{
        CastChoices, CastSignature, CostConfiguration, TargetReplacementError, TargetSelection,
    };
    use crate::action::Target;
    use crate::card::SpellForm;
    use crate::ids::{
        AdditionalCostId, AlternativeCostId, CardPartId, GameObjectId, ModeId, PlayOptionId,
        TargetSlotId,
    };

    fn fused_signature(targets: Vec<TargetSelection>) -> CastSignature {
        CastSignature::from_validated_choices(
            SpellForm::Combined(vec![CardPartId(0), CardPartId(1)]),
            CastChoices::new(PlayOptionId(2))
                .with_modes(vec![ModeId(2), ModeId(0)])
                .with_costs(CostConfiguration::new(
                    Some(AlternativeCostId(3)),
                    vec![AdditionalCostId(4), AdditionalCostId(4)],
                ))
                .with_x(7)
                .with_targets(targets),
        )
    }

    #[test]
    fn fork_copy_changes_only_targets() {
        let original = fused_signature(vec![
            TargetSelection::single(TargetSlotId(0), Target::Permanent(GameObjectId(10))),
            TargetSelection::single(TargetSlotId(1), Target::Player(crate::PlayerId::One)),
        ]);
        let copied = original
            .copy_with_targets(vec![
                TargetSelection::single(TargetSlotId(0), Target::Permanent(GameObjectId(20))),
                TargetSelection::single(TargetSlotId(1), Target::Player(crate::PlayerId::Two)),
            ])
            .unwrap();

        assert_eq!(copied.play_option(), original.play_option());
        assert_eq!(copied.form(), original.form());
        assert_eq!(copied.modes(), original.modes());
        assert_eq!(copied.costs(), original.costs());
        assert_eq!(copied.x(), original.x());
        assert_ne!(copied.targets(), original.targets());
        assert_eq!(
            original.targets()[0].targets(),
            &[Target::Permanent(GameObjectId(10))]
        );
    }

    #[test]
    fn two_slots_may_select_the_same_target() {
        let same_creature = Target::Permanent(GameObjectId(10));
        let original = fused_signature(vec![
            TargetSelection::single(TargetSlotId(0), same_creature),
            TargetSelection::single(TargetSlotId(1), same_creature),
        ]);
        let copied = original
            .copy_with_targets(original.targets().to_vec())
            .unwrap();

        assert_eq!(copied.targets()[0].targets(), &[same_creature]);
        assert_eq!(copied.targets()[1].targets(), &[same_creature]);
    }

    #[test]
    fn spell_copy_cannot_change_target_slots_or_cardinality() {
        let original = fused_signature(vec![
            TargetSelection::single(TargetSlotId(0), Target::Permanent(GameObjectId(10))),
            TargetSelection::single(TargetSlotId(1), Target::Player(crate::PlayerId::One)),
        ]);

        assert!(matches!(
            original.copy_with_targets(vec![TargetSelection::single(
                TargetSlotId(0),
                Target::Permanent(GameObjectId(20)),
            )]),
            Err(TargetReplacementError::SelectionCount { .. })
        ));
        assert!(matches!(
            original.copy_with_targets(vec![
                TargetSelection::single(TargetSlotId(9), Target::Permanent(GameObjectId(20)),),
                TargetSelection::single(TargetSlotId(1), Target::Player(crate::PlayerId::Two)),
            ]),
            Err(TargetReplacementError::Slot { .. })
        ));
        assert!(matches!(
            original.copy_with_targets(vec![
                TargetSelection::new(
                    TargetSlotId(0),
                    vec![
                        Target::Permanent(GameObjectId(20)),
                        Target::Permanent(GameObjectId(21)),
                    ],
                ),
                TargetSelection::single(TargetSlotId(1), Target::Player(crate::PlayerId::Two)),
            ]),
            Err(TargetReplacementError::TargetCount { .. })
        ));
    }

    #[test]
    fn target_changes_preserve_divided_amounts_by_position() {
        let original = TargetSelection::divided(
            TargetSlotId(0),
            vec![
                Target::Permanent(GameObjectId(10)),
                Target::Permanent(GameObjectId(11)),
            ],
            vec![1, 4],
        );
        let changed = original
            .with_replaced_targets(vec![
                Target::Permanent(GameObjectId(20)),
                Target::Permanent(GameObjectId(21)),
            ])
            .expect("a target change keeps the target count");

        assert_eq!(changed.amounts(), &[1, 4]);
    }
}
