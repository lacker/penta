use std::error::Error;
use std::fmt;

use crate::card::{BasicLandType, CounterKind, ManaSplit};
use crate::casting::{CastChoices, TargetSelection};
use crate::{
    AbilityId, CardDefinitionId, CardPartId, GameObjectId, GrantId, ModeId, PlayOptionId, PlayerId,
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
    /// An ability printed by inline token characteristics. Pairing this with
    /// the token's game-object ID gives it stable in-game identity without
    /// inventing a globally cataloged card definition for the token.
    Token {
        part: CardPartId,
        ability: AbilityId,
    },
    /// An ability supplied by creator-owned emblem characteristics. Emblems
    /// have no card definition or card part; pair this with the emblem's
    /// game-object ID for stable in-game identity.
    Emblem {
        ability: AbilityId,
    },
    /// An ability supplied by the rule-owned characteristics of a face-down
    /// permanent. Pair this positional ID with the permanent's object ID.
    FaceDown {
        ability: AbilityId,
    },
    IntrinsicBasicLand(BasicLandType),
    /// A keyword a counter on the permanent grants (CR 122.1e). Like the
    /// land one above it is nobody's printed ability: the permanent has it
    /// because of what is sitting on it.
    IntrinsicCounter(CounterKind),
    Granted {
        source: GameObjectId,
        source_definition: CardDefinitionId,
        source_part: CardPartId,
        source_ability: AbilityId,
        grant: GrantId,
    },
    /// An ability granted by an inline token ability. The granting object's
    /// ID and positional clause identity are sufficient in live state; its
    /// frozen token characteristics travel with the affected object and stack
    /// presentation instead of being assigned a catalog identity.
    TokenGranted {
        source: GameObjectId,
        source_part: CardPartId,
        source_ability: AbilityId,
        grant: GrantId,
    },
    /// An ability granted by an emblem ability. The emblem object identifies
    /// the source, while the ability and grant IDs locate the authored clause.
    EmblemGranted {
        source: GameObjectId,
        source_ability: AbilityId,
        grant: GrantId,
    },
    /// An ability granted by a rule-owned face-down ability.
    FaceDownGranted {
        source: GameObjectId,
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
        /// How many counters an open-ended removal cost takes, when the
        /// ability has one. Source, ability, and colour do not distinguish
        /// "remove one storage counter" from "remove three", so the size is
        /// part of the action rather than something chosen afterwards.
        /// `None` for every ability whose cost has only one size.
        counters_removed: Option<u16>,
        /// The permanent a "Sacrifice a <thing>" cost consumes. Source,
        /// ability, and colour name one Goblin-sacrificing ability once per
        /// Goblin, so which one is part of the action: a mana ability
        /// resolves without ever holding priority, so there is no window in
        /// which to ask afterwards. `None` for every ability that sacrifices
        /// nothing but itself.
        cost_object: Option<GameObjectId>,
        /// How the amount is divided, for an ability that adds mana "in any
        /// combination of" more than one type. Source, ability, and colour
        /// name one such ability once per division, so the division is part
        /// of the action: like the two choices above, a mana ability resolves
        /// without ever holding priority. `None` for every ability that
        /// produces one type at a time.
        combination: Option<ManaSplit>,
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
        /// The objects chosen to pay a nonmana cost: the permanent a
        /// sacrifice cost takes, or the cards an exile cost lifts from a
        /// graveyard. Most costs name one or none; a cost that spends several
        /// names them all, because an activation has no window in which to
        /// ask afterwards. Empty when the cost spends nothing chosen.
        cost_objects: Vec<GameObjectId>,
        /// The value chosen for X in the activation cost, zero when the cost
        /// has no X.
        x: u16,
        /// The modes chosen for an ability that prints "choose one --",
        /// in ascending order. Modes are chosen as the ability is activated
        /// (CR 601.2b), so they travel with the action. Empty for every
        /// ability that prints no modes, which is nearly all of them.
        modes: Vec<ModeId>,
    },
    /// Turn a face-down permanent face up by paying its morph cost. A
    /// special action rather than an ability: it uses no stack, nothing can
    /// respond to it, and the permanent it names has no abilities to
    /// activate while it is face down (CR 702.37b).
    TurnFaceUp {
        permanent: GameObjectId,
    },
    /// Foretell a card in hand: pay {2} and exile it face down, to be cast
    /// on a later turn for its foretell cost (CR 702.143a). A special action
    /// like the one above -- no stack, nothing to respond to, and only
    /// during your own turn.
    Foretell {
        card: GameObjectId,
    },
    /// Unlock a locked door of a Room you control by paying that door's mana
    /// cost (CR 714.4a). A special action like the two above: no stack,
    /// nothing to respond to, and only in your own main phase.
    UnlockDoor {
        room: GameObjectId,
        door: CardPartId,
    },
    DeclareAttacker {
        attacker: GameObjectId,
        defender: AttackDefender,
    },
    /// Puts two declared attackers, and everything already banded with
    /// either of them, into one attacking band. Bands are built a pair at a
    /// time rather than named all at once so that the legal ones can be
    /// enumerated the way every other declaration is.
    BandAttackers {
        first: GameObjectId,
        second: GameObjectId,
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
