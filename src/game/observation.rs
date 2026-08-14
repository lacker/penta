use crate::{
    AbilityOrigin, Action, AttackDefender, CardDefinitionId, CardPartId, CardTypeSet,
    CastSignature, GameObjectId, PlayerId, Target,
};
use serde_json::Value;

use super::{DecisionObservation, GameResult, ManaPool, StackObjectKind, Step};

pub(super) type PublicCard = (GameObjectId, CardDefinitionId);
pub(super) type LastSeenHand = Option<(PlayerId, Vec<PublicCard>)>;

/// One card in a hand or library, as a simulation rearranging hidden state
/// sees it. Unlike an observation this is never redacted, because a `Game` in
/// your own process has nobody to hide it from — see [`crate::Game::hand`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ZoneCard {
    pub object: GameObjectId,
    pub definition: CardDefinitionId,
}

/// Why a hand or library could not be set.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ZoneError {
    /// The definition is not in the catalog this game was built with.
    UnknownCard(CardDefinitionId),
    /// The game ran out of object identifiers.
    TooManyCards,
}

impl std::fmt::Display for ZoneError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownCard(definition) => write!(
                formatter,
                "{definition:?} is not in this game's card catalog"
            ),
            Self::TooManyCards => formatter.write_str("the game ran out of object identifiers"),
        }
    }
}

impl std::error::Error for ZoneError {}

/// A command-zone emblem and the printed ability that created it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmblemObservation {
    pub id: GameObjectId,
    pub controller: PlayerId,
    pub name: String,
    pub source_ability: AbilityOrigin,
    pub ability_texts: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct PermanentObservation {
    pub id: GameObjectId,
    pub definition: CardDefinitionId,
    /// The logical card part currently supplying this permanent's printed
    /// characteristics. Changing faces does not change `id`.
    pub presented: CardPartId,
    pub controller: PlayerId,
    /// The object this Aura, Equipment, or Fortification is attached to. The
    /// field can name a public nonbattlefield card, as Animate Dead does while
    /// attached to a creature card in a graveyard.
    pub attached_to: Option<GameObjectId>,
    /// The card types this permanent presents right now. An animated land
    /// reports the creature types its animation added, which is what the
    /// printed rules alone cannot say.
    pub types: CardTypeSet,
    /// Public choice associated with permanents such as Cavern of Souls.
    pub chosen_creature_type: Option<String>,
    /// Public card name associated with permanents such as Pithing Needle.
    pub chosen_card_name: Option<String>,
    pub tapped: bool,
    pub power: Option<i16>,
    pub toughness: Option<i16>,
    pub damage: u16,
    /// Loyalty counters, present only for planeswalkers.
    pub loyalty: Option<u16>,
    pub loyalty_ability_used_this_turn: bool,
    /// The declared player or planeswalker defender while this permanent is
    /// attacking. Declaring an attack does not target that defender.
    pub attack_defender: Option<AttackDefender>,
    pub attacking: bool,
    /// Whether this attacker has been blocked at any point this combat.
    pub blocked_this_combat: bool,
    pub blocking: Option<GameObjectId>,
    pub flying: bool,
    pub can_attack: bool,
    pub entered_this_turn: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StackObservation {
    pub id: GameObjectId,
    pub kind: StackObjectKind,
    pub source: Option<GameObjectId>,
    /// The exact printed, intrinsic, or granted ability that created this stack
    /// object. Spells omit it.
    pub ability: Option<AbilityOrigin>,
    /// Frozen rules text for the creating ability. This remains inspectable
    /// even when its source changes zones or characteristics.
    pub ability_text: Option<String>,
    pub definition: CardDefinitionId,
    pub controller: PlayerId,
    /// Public resolution constraint. An uncounterable object remains a legal
    /// target for counter spells, but those effects cannot remove it.
    pub counterable: bool,
    /// Locked spell form, modes, costs, X, and target slots. Activated
    /// abilities have no cast signature.
    pub signature: Option<CastSignature>,
    pub targets: Vec<Target>,
    pub chosen_permanents: Vec<GameObjectId>,
    pub x: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayerObservation {
    pub viewer: PlayerId,
    pub turn: u32,
    /// The number of turns the active player has started, including extras.
    pub active_turn: u32,
    pub active_player: PlayerId,
    pub priority: PlayerId,
    pub step: Step,
    /// Whether priority is currently between the first-strike and regular
    /// combat-damage steps. Both steps otherwise share [`Step::CombatDamage`].
    pub regular_combat_damage_pending: bool,
    pub life_totals: [i16; 2],
    pub mana_pools: [ManaPool; 2],
    pub hand: Vec<(GameObjectId, CardDefinitionId)>,
    pub opponent_hand_size: usize,
    pub last_seen_hand: Option<(PlayerId, Vec<(GameObjectId, CardDefinitionId)>)>,
    pub library_sizes: [usize; 2],
    pub graveyards: [Vec<(GameObjectId, CardDefinitionId)>; 2],
    pub exiles: [Vec<(GameObjectId, CardDefinitionId)>; 2],
    pub battlefield: Vec<PermanentObservation>,
    pub emblems: Vec<EmblemObservation>,
    pub stack: Vec<StackObservation>,
    pub decision: Option<DecisionObservation>,
    pub result: Option<GameResult>,
    pub legal_actions: Vec<Action>,
    /// Hidden-safe rules bookkeeping needed to treat this observation as a
    /// current-state checkpoint for local determinization.
    pub checkpoint: Value,
}
