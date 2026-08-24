use crate::{
    AbilityOrigin, Action, AttackDefender, CardDefinitionId, CardPartId, CardTypeSet,
    CastSignature, DoubleFacedKind, EmblemCharacteristics, FaceDownCharacteristics, GameObjectId,
    PlayerId, Target, TokenCharacteristics,
};
use serde_json::Value;

use super::{DecisionObservation, GameResult, ManaPool, StackObjectKind, Step};

pub(super) type PublicCard = (GameObjectId, CardDefinitionId);
pub(super) type LastSeenHand = Option<(PlayerId, Vec<PublicCard>)>;

/// The authored copiable characteristics an object presents.
///
/// Printed cards join through the global card catalog. Tokens instead carry
/// the complete characteristics supplied by the effect that created them, so
/// duplicate token names never become duplicate card identities. Whether the
/// object itself is a token is recorded separately: a token can copy a card,
/// and a nontoken card can copy a token.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ObjectCharacteristics {
    Card {
        definition: CardDefinitionId,
        part: CardPartId,
    },
    Token {
        token: TokenCharacteristics,
        part: CardPartId,
    },
    Emblem {
        emblem: EmblemCharacteristics,
    },
    /// Rules-owned copiable values for a face-down spell or permanent. The
    /// underlying object's physical card identity is retained separately.
    FaceDown {
        face_down: FaceDownCharacteristics,
    },
}

impl ObjectCharacteristics {
    #[must_use]
    pub const fn card(definition: CardDefinitionId, part: CardPartId) -> Self {
        Self::Card { definition, part }
    }

    #[must_use]
    pub const fn token(token: TokenCharacteristics, part: CardPartId) -> Self {
        Self::Token { token, part }
    }

    #[must_use]
    pub const fn emblem(emblem: EmblemCharacteristics) -> Self {
        Self::Emblem { emblem }
    }

    #[must_use]
    pub const fn face_down(face_down: FaceDownCharacteristics) -> Self {
        Self::FaceDown { face_down }
    }

    #[must_use]
    pub const fn part(self) -> CardPartId {
        match self {
            Self::Card { part, .. } | Self::Token { part, .. } => part,
            Self::Emblem { .. } | Self::FaceDown { .. } => CardPartId::PRIMARY,
        }
    }

    #[must_use]
    pub const fn with_part(self, part: CardPartId) -> Self {
        match self {
            Self::Card { definition, .. } => Self::Card { definition, part },
            Self::Token { token, .. } => Self::Token { token, part },
            Self::Emblem { .. } | Self::FaceDown { .. } => self,
        }
    }

    #[must_use]
    pub const fn card_definition(self) -> Option<CardDefinitionId> {
        match self {
            Self::Card { definition, .. } => Some(definition),
            Self::Token { .. } | Self::Emblem { .. } | Self::FaceDown { .. } => None,
        }
    }

    #[must_use]
    pub const fn token_characteristics(self) -> Option<TokenCharacteristics> {
        match self {
            Self::Card { .. } | Self::Emblem { .. } | Self::FaceDown { .. } => None,
            Self::Token { token, .. } => Some(token),
        }
    }

    #[must_use]
    pub const fn emblem_characteristics(self) -> Option<EmblemCharacteristics> {
        match self {
            Self::Emblem { emblem } => Some(emblem),
            Self::Card { .. } | Self::Token { .. } | Self::FaceDown { .. } => None,
        }
    }

    #[must_use]
    pub const fn face_down_characteristics(self) -> Option<FaceDownCharacteristics> {
        match self {
            Self::FaceDown { face_down } => Some(face_down),
            Self::Card { .. } | Self::Token { .. } | Self::Emblem { .. } => None,
        }
    }
}

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

/// Which physical side of a double-faced permanent is currently up.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PhysicalFaceSide {
    Front,
    Back,
}

/// Public physical topology kept separate from effective copied values.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PhysicalFaceObservation {
    pub kind: DoubleFacedKind,
    pub side: PhysicalFaceSide,
}

/// One public counter kind and how many of it an object or player carries.
/// Names are the stable rules vocabulary; the vector containing these is
/// sparse and omits zero counts.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CounterObservation {
    pub name: String,
    pub count: u16,
}

/// Counter state for a visible card outside the battlefield.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CardCounterObservation {
    pub object: GameObjectId,
    pub counters: Vec<CounterObservation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct PermanentObservation {
    pub id: GameObjectId,
    /// The logical card or token part currently supplying this permanent's
    /// copiable characteristics. Changing faces does not change `id`.
    pub characteristics: ObjectCharacteristics,
    /// Tokens retain this status even while copying a printed card.
    pub token: bool,
    /// This object carries or is subject to state that a presentation cannot
    /// safely compare from its compact characteristics alone. Battlefield
    /// clients may still render it beside similar objects, but must not
    /// collapse it into a shared visual pile.
    pub has_individual_state: bool,
    pub controller: PlayerId,
    /// Whether this permanent is phased out. It is public information --
    /// both players can see it -- and the rules merely treat it as though it
    /// does not exist, so it is shown and flagged rather than hidden.
    pub phased_out: bool,
    /// Whether this permanent is face down. Everyone sees that much; only
    /// its controller's observation carries the definition underneath.
    pub face_down: bool,
    /// The physical double-faced topology and side up, independently of the
    /// characteristics a copy effect currently supplies. Hidden while face
    /// down and absent for a physical single-faced object.
    pub physical_face: Option<PhysicalFaceObservation>,
    /// The card types this permanent presents right now, including resolved
    /// continuous changes that the printed rules alone cannot say.
    pub types: CardTypeSet,
    /// Public choice associated with permanents such as Cavern of Souls.
    pub chosen_creature_type: Option<String>,
    /// The basic land type this permanent was told to be as it entered.
    pub chosen_basic_land_type: Option<crate::card::BasicLandType>,
    /// Public card name associated with permanents such as Pithing Needle.
    pub chosen_card_name: Option<String>,
    pub tapped: bool,
    pub power: Option<i16>,
    pub toughness: Option<i16>,
    pub damage: u16,
    /// Every counter on this permanent, including ordinary card-defined
    /// names and counters with intrinsic rules meaning.
    pub counters: Vec<CounterObservation>,
    /// Loyalty counters, present only for planeswalkers.
    pub loyalty: Option<u16>,
    pub loyalty_ability_used_this_turn: bool,
    /// The declared player or planeswalker defender while this permanent is
    /// attacking. Declaring an attack does not target that defender.
    pub attack_defender: Option<AttackDefender>,
    pub attacking: bool,
    /// Whether this attacker has been blocked at any point this combat.
    pub blocked_this_combat: bool,
    /// Every attacker this creature is blocking. Several, when it blocks a
    /// band or is allowed more than one block. Emptied as those attackers
    /// leave combat, which does not stop this creature blocking.
    pub blocking: Vec<GameObjectId>,
    /// Whether this creature has blocked at any point this combat. Read this,
    /// not `blocking`, to ask whether it is a blocking creature.
    pub blocking_this_combat: bool,
    /// Which attacking band this creature is in, as an index shared by every
    /// member of that band. None for an attacker in no band at all.
    pub attacking_band: Option<u8>,
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
    /// Frozen presentation of the spell or ability. Token and face-down
    /// sources carry inline characteristics rather than a fake card ID.
    pub characteristics: ObjectCharacteristics,
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
    /// Poison counters each seat has, in seat order. Ten is a loss.
    pub poison_counters: [u16; 2],
    /// Energy counters each seat has, in seat order. Public information,
    /// like any other counter a player holds.
    pub energy_counters: [u16; 2],
    /// Every counter each player carries, in seat order. `poison_counters`
    /// and `energy_counters` remain compatibility projections of this data.
    pub counters: [Vec<CounterObservation>; 2],
    /// Who holds the crown (CR 720), if anyone. Public information: both
    /// players know, and so does anything reading the observation.
    pub monarch: Option<PlayerId>,
    pub mana_pools: [ManaPool; 2],
    pub hand: Vec<(GameObjectId, CardDefinitionId)>,
    pub opponent_hand_size: usize,
    pub last_seen_hand: Option<(PlayerId, Vec<(GameObjectId, CardDefinitionId)>)>,
    pub library_sizes: [usize; 2],
    /// The top card of the viewer's own library, when something lets them
    /// look at it. `None` in the ordinary game, where a library is face
    /// down to everyone including its owner.
    pub revealed_library_top: Option<(GameObjectId, CardDefinitionId)>,
    pub graveyards: [Vec<(GameObjectId, CardDefinitionId)>; 2],
    /// Each player's exile as this viewer sees it. A card lying face down
    /// is absent rather than shown, unless the viewer is its owner.
    pub exiles: [Vec<(GameObjectId, CardDefinitionId)>; 2],
    /// How many cards lie face down in each player's exile. Both players may
    /// count them; only their owner knows what they are.
    pub face_down_exile_sizes: [usize; 2],
    /// Sparse counter state for visible cards in nonbattlefield zones.
    pub card_counters: Vec<CardCounterObservation>,
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

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::ObjectCharacteristics;

    #[test]
    fn object_characteristics_keep_inline_virtual_values_compact() {
        assert!(
            size_of::<ObjectCharacteristics>() <= 128,
            "object characteristics exceeded their 128-byte inline budget",
        );
    }
}
