use crate::{CardDefinitionId, GameObjectId, PlayerId};

/// The rules procedure represented by a mandatory player decision.
///
/// Ordinary choices are unordered selections. Trigger ordering and trigger
/// placement are separate procedures that happen while no player has
/// priority, even though they use the same indexed-choice protocol command.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DecisionKind {
    Choice,
    TriggerOrder,
    TriggerPlacement,
}

/// How an ordered decision's submitted option list should be interpreted.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DecisionOrderSemantics {
    Resolution,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DecisionVisibility {
    Public,
    Private,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DecisionPreference {
    HigherCardValue,
    LowerCardValue,
    /// Divide all card-backed options into two piles with similar aggregate
    /// value. The decision still exposes the ordinary subset-selection
    /// contract; this only guides automated policies toward a useful split.
    BalancedPartition,
    /// Prefer opposing battlefield cards and cards in the chooser's own
    /// graveyard. Linked-exile effects use this to separate removal and
    /// recovery targets from cards whose eventual return would be harmful.
    LinkedExileTargets,
    /// Prefer a valuable opposing battlefield permanent and avoid the
    /// chooser's own permanents. Used for non-targeting removal choices made
    /// during resolution.
    RemovalChoice,
    /// Prefer one semantically distinguished option before applying the
    /// ordinary minimum-selection rule. This remains an engine-policy hint;
    /// option IDs and labels are still the complete public choice contract.
    PreferOption(u32),
    Neutral,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DecisionZone {
    Hand,
    Graveyard,
    Battlefield,
    Stack,
    Library,
    Exile,
    Command,
    DrawnThisStep,
    None,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecisionOption {
    pub id: u32,
    pub label: String,
    pub card: Option<(GameObjectId, CardDefinitionId)>,
    /// Cards represented collectively by this option, such as one pile in a
    /// choose-a-pile decision. This remains empty for ordinary card options.
    pub members: Vec<(GameObjectId, CardDefinitionId)>,
    /// Frozen creating-ability text when this option represents a pending
    /// trigger. This distinguishes multiple abilities from the same source.
    pub ability_text: Option<String>,
    pub zone: DecisionZone,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecisionObservation {
    pub id: u32,
    pub player: PlayerId,
    pub kind: DecisionKind,
    pub order_semantics: Option<DecisionOrderSemantics>,
    pub prompt: String,
    pub visibility: DecisionVisibility,
    pub preference: DecisionPreference,
    pub minimum: usize,
    pub maximum: usize,
    pub cancellable: bool,
    pub options: Vec<DecisionOption>,
}
