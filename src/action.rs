use std::error::Error;
use std::fmt;

use crate::card::BasicLandType;
use crate::casting::{CastChoices, TargetSelection};
use crate::{
    AbilityId, CardDefinitionId, CardPartId, GameObjectId, GrantId, PlayOptionId, PlayerId,
};

pub use crate::card::ManaColor;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Target {
    Player(PlayerId),
    /// A card object in a non-battlefield, non-stack zone. The object ID is
    /// the current zone incarnation, so moving the card makes this target
    /// illegal without conflating it with the new object created there.
    Card(GameObjectId),
    Permanent(GameObjectId),
    Spell(GameObjectId),
}

/// The stable origin of an effective ability on a game object.
///
/// Printed IDs are local to one card part, so copied abilities freeze their
/// effective card definition as well as the part and clause ID. Intrinsic land
/// abilities are identified by the subtype that grants them. A granted origin
/// records the granting object, the effective card definition and part that
/// supplied its positional source clause, and the grant site inside that
/// clause; it is provenance, not an executable definition. Stack objects
/// separately freeze the effective text, target declarations, and resolver
/// they received at creation. Pair this with the affected object's
/// [`GameObjectId`] to identify one ability in a game.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AbilityOrigin {
    Printed {
        definition: CardDefinitionId,
        part: CardPartId,
        ability: AbilityId,
    },
    IntrinsicBasicLand(BasicLandType),
    Granted {
        source: GameObjectId,
        source_definition: CardDefinitionId,
        source_part: CardPartId,
        source_ability: AbilityId,
        grant: GrantId,
    },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CombatDamageAssignment {
    pub recipient: Target,
    pub amount: u16,
}

/// The player or planeswalker a creature is attacking.
///
/// Declaring an attack does not target, so this is deliberately distinct from
/// [`Target`]. Keeping the defender on the attacker also prevents a
/// planeswalker that leaves combat from silently redirecting that attack to
/// its controller.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AttackDefender {
    Player(PlayerId),
    Planeswalker(GameObjectId),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Action {
    KeepHand,
    TakeMulligan,
    BottomCards {
        cards: Vec<GameObjectId>,
    },
    DiscardCards {
        cards: Vec<GameObjectId>,
    },
    ChooseDecision {
        decision: u32,
        options: Vec<u32>,
    },
    CancelDecision {
        decision: u32,
    },
    ChooseUntap {
        permanents: Vec<GameObjectId>,
    },
    PassPriority,
    PlayLand {
        card: GameObjectId,
        option: PlayOptionId,
    },
    ActivateManaAbility {
        source: GameObjectId,
        ability: AbilityOrigin,
        color: ManaColor,
    },
    PayLifeForMana,
    CastSpell {
        card: GameObjectId,
        choices: CastChoices,
        sacrifices: Vec<GameObjectId>,
    },
    ActivateAbility {
        source: GameObjectId,
        ability: AbilityOrigin,
        targets: Vec<TargetSelection>,
        /// The object chosen to pay a cost that names one, such as the
        /// permanent a sacrifice cost takes or the card an exile cost lifts
        /// from a graveyard.
        cost_object: Option<GameObjectId>,
        /// The value chosen for X in the activation cost, zero when the cost
        /// has no X.
        x: u16,
    },
    /// Takes a rules-defined special action from a battlefield object.
    ///
    /// Special actions pay their costs and happen immediately; unlike an
    /// activated ability, they never create a stack object.
    TakeSpecialAction {
        source: GameObjectId,
        ability: AbilityOrigin,
        /// Stable identity of the rules effect that granted this action.
        /// Ordinary special actions are not effect-scoped and use `None`.
        effect_id: Option<u64>,
    },
    DeclareAttacker {
        attacker: GameObjectId,
        defender: AttackDefender,
    },
    FinishDeclaringAttackers,
    DeclareBlocker {
        blocker: GameObjectId,
        attacker: GameObjectId,
    },
    FinishDeclaringBlockers,
    AssignCombatDamage {
        attacker: GameObjectId,
        assignments: Vec<CombatDamageAssignment>,
    },
    Concede,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActionError {
    GameAlreadyFinished,
    NotLegal { player: PlayerId, action: Action },
}

impl fmt::Display for ActionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GameAlreadyFinished => formatter.write_str("the game is already finished"),
            Self::NotLegal { player, action } => {
                write!(formatter, "{action:?} is not legal for {player}")
            }
        }
    }
}

impl Error for ActionError {}
