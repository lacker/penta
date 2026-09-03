use crate::action::AbilityOrigin;
use crate::card::{
    AbilityDef, AbilityPredicateDef, AppliedEffectDef, AppliedRuleDef, BasicLandType,
    CardSupertypeSet, CardTypeSet, ColorSet, CreatureTypeSetDef, PlayRestrictionDef,
    SetOperationDef, ZoneKind,
};
use crate::ids::{GameObjectId, GrantId, PlayerId};

use super::{
    AbilitySourceRef, EffectResolutionContext, ObjectCharacteristics, Permanent, TriggerEventObject,
};

/// Timestamp shared by the continuous-effect slices currently modeled. Static
/// effects use their source permanent's battlefield timestamp; resolving
/// effects receive a fresh timestamp as they are created. Keeping this
/// independent from object identity lets a later layer evaluator preserve the
/// same ordering contract.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct ContinuousEffectTimestamp(pub(super) u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ContinuousEffectExpiration {
    EndOfTurn,
    /// Ends as the end-of-combat step finishes, which is earlier than the
    /// cleanup that ends [`Self::EndOfTurn`].
    EndOfCombat,
    UpkeepOf(PlayerId),
    TurnOf {
        player: PlayerId,
        turn: u32,
    },
    WhileSourceTapped,
    /// For as long as the source is still on the battlefield. Read live, so
    /// nothing has to notice the moment it leaves.
    WhileSourceRemains,
    /// Ends when the affected player next casts a spell matching the applied
    /// permission. Cast legality queries do not satisfy this condition.
    NextMatchingCast,
    /// Ends when any contained atomic condition does. The authored duration
    /// may be nested, but resolution flattens it into this allocation-free
    /// set so resolved effects remain `Copy`.
    AnyOf(ContinuousEffectExpirationSet),
    Never,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct ContinuousEffectExpirationSet {
    flags: u8,
    upkeep_mask: u8,
    turn_of: [Option<u32>; 2],
}

impl ContinuousEffectExpirationSet {
    const END_OF_TURN: u8 = 1 << 0;
    const END_OF_COMBAT: u8 = 1 << 1;
    const WHILE_SOURCE_TAPPED: u8 = 1 << 2;
    const WHILE_SOURCE_REMAINS: u8 = 1 << 3;
    const NEXT_MATCHING_CAST: u8 = 1 << 4;

    const fn has(self, flag: u8) -> bool {
        self.flags & flag != 0
    }

    fn add(&mut self, flag: u8) {
        self.flags |= flag;
    }

    fn insert(&mut self, expiration: ContinuousEffectExpiration) {
        match expiration {
            ContinuousEffectExpiration::EndOfTurn => self.add(Self::END_OF_TURN),
            ContinuousEffectExpiration::EndOfCombat => self.add(Self::END_OF_COMBAT),
            ContinuousEffectExpiration::UpkeepOf(player) => {
                self.upkeep_mask |= 1 << player.index();
            }
            ContinuousEffectExpiration::TurnOf { player, turn } => {
                self.turn_of[player.index()] = Some(turn);
            }
            ContinuousEffectExpiration::WhileSourceTapped => self.add(Self::WHILE_SOURCE_TAPPED),
            ContinuousEffectExpiration::WhileSourceRemains => {
                self.add(Self::WHILE_SOURCE_REMAINS);
            }
            ContinuousEffectExpiration::NextMatchingCast => self.add(Self::NEXT_MATCHING_CAST),
            ContinuousEffectExpiration::AnyOf(other) => self.merge(other),
            ContinuousEffectExpiration::Never => {}
        }
    }

    fn merge(&mut self, other: Self) {
        self.flags |= other.flags;
        self.upkeep_mask |= other.upkeep_mask;
        for (turn, other_turn) in self.turn_of.iter_mut().zip(other.turn_of) {
            *turn = match (*turn, other_turn) {
                (Some(current), Some(other)) => Some(current.min(other)),
                (current, other) => current.or(other),
            };
        }
    }

    fn len(self) -> usize {
        self.flags.count_ones() as usize
            + self.upkeep_mask.count_ones() as usize
            + self.turn_of.into_iter().flatten().count()
    }

    fn only(self) -> Option<ContinuousEffectExpiration> {
        (self.len() == 1).then(|| {
            if self.has(Self::END_OF_TURN) {
                ContinuousEffectExpiration::EndOfTurn
            } else if self.has(Self::END_OF_COMBAT) {
                ContinuousEffectExpiration::EndOfCombat
            } else if self.upkeep_mask & 1 != 0 {
                ContinuousEffectExpiration::UpkeepOf(PlayerId::One)
            } else if self.upkeep_mask & 2 != 0 {
                ContinuousEffectExpiration::UpkeepOf(PlayerId::Two)
            } else if let Some(turn) = self.turn_of[0] {
                ContinuousEffectExpiration::TurnOf {
                    player: PlayerId::One,
                    turn,
                }
            } else if let Some(turn) = self.turn_of[1] {
                ContinuousEffectExpiration::TurnOf {
                    player: PlayerId::Two,
                    turn,
                }
            } else if self.has(Self::WHILE_SOURCE_TAPPED) {
                ContinuousEffectExpiration::WhileSourceTapped
            } else if self.has(Self::WHILE_SOURCE_REMAINS) {
                ContinuousEffectExpiration::WhileSourceRemains
            } else {
                ContinuousEffectExpiration::NextMatchingCast
            }
        })
    }
}

impl ContinuousEffectExpiration {
    pub(super) fn any_of(expirations: impl IntoIterator<Item = Self>) -> Self {
        let mut set = ContinuousEffectExpirationSet::default();
        for expiration in expirations {
            set.insert(expiration);
        }
        set.only().unwrap_or_else(|| {
            if set.len() == 0 {
                Self::Never
            } else {
                Self::AnyOf(set)
            }
        })
    }

    pub(super) fn atomic_members(self) -> Vec<Self> {
        let Self::AnyOf(set) = self else {
            return vec![self];
        };
        let mut members = Vec::with_capacity(set.len());
        if set.has(ContinuousEffectExpirationSet::END_OF_TURN) {
            members.push(Self::EndOfTurn);
        }
        if set.has(ContinuousEffectExpirationSet::END_OF_COMBAT) {
            members.push(Self::EndOfCombat);
        }
        if set.upkeep_mask & 1 != 0 {
            members.push(Self::UpkeepOf(PlayerId::One));
        }
        if set.upkeep_mask & 2 != 0 {
            members.push(Self::UpkeepOf(PlayerId::Two));
        }
        if let Some(turn) = set.turn_of[0] {
            members.push(Self::TurnOf {
                player: PlayerId::One,
                turn,
            });
        }
        if let Some(turn) = set.turn_of[1] {
            members.push(Self::TurnOf {
                player: PlayerId::Two,
                turn,
            });
        }
        if set.has(ContinuousEffectExpirationSet::WHILE_SOURCE_TAPPED) {
            members.push(Self::WhileSourceTapped);
        }
        if set.has(ContinuousEffectExpirationSet::WHILE_SOURCE_REMAINS) {
            members.push(Self::WhileSourceRemains);
        }
        if set.has(ContinuousEffectExpirationSet::NEXT_MATCHING_CAST) {
            members.push(Self::NextMatchingCast);
        }
        members
    }

    pub(super) const fn expires_on_next_matching_cast(self) -> bool {
        match self {
            Self::NextMatchingCast => true,
            Self::AnyOf(set) => set.has(ContinuousEffectExpirationSet::NEXT_MATCHING_CAST),
            _ => false,
        }
    }

    pub(super) const fn requires_source_tapped(self) -> bool {
        match self {
            Self::WhileSourceTapped => true,
            Self::AnyOf(set) => set.has(ContinuousEffectExpirationSet::WHILE_SOURCE_TAPPED),
            _ => false,
        }
    }

    pub(super) const fn requires_source_to_remain(self) -> bool {
        match self {
            Self::WhileSourceRemains => true,
            Self::AnyOf(set) => set.has(ContinuousEffectExpirationSet::WHILE_SOURCE_REMAINS),
            _ => false,
        }
    }

    /// Whether this effect is still live as a turn begins, which is before
    /// the untap step. An until-your-next-upkeep effect is: the untap step
    /// comes first, so anything that has to be read there -- an untap
    /// prohibition above all -- is still in force. It goes at the upkeep
    /// itself, through [`Self::survives_untap_step`].
    pub(super) fn survives_turn_start(
        self,
        _active_player: PlayerId,
        turns_started: [u32; 2],
    ) -> bool {
        match self {
            Self::TurnOf { player, turn } => turns_started[player.index()] < turn,
            Self::AnyOf(set) => set
                .turn_of
                .into_iter()
                .enumerate()
                .all(|(player, turn)| turn.is_none_or(|turn| turns_started[player] < turn)),
            Self::UpkeepOf(_)
            | Self::EndOfTurn
            | Self::EndOfCombat
            | Self::WhileSourceTapped
            | Self::WhileSourceRemains
            | Self::NextMatchingCast
            | Self::Never => true,
        }
    }

    /// Whether this effect outlives the untap step it was carried into.
    pub(super) fn survives_untap_step(self, active_player: PlayerId) -> bool {
        match self {
            Self::UpkeepOf(player) => player != active_player,
            Self::AnyOf(set) => set.upkeep_mask & (1 << active_player.index()) == 0,
            Self::TurnOf { .. }
            | Self::EndOfTurn
            | Self::EndOfCombat
            | Self::WhileSourceTapped
            | Self::WhileSourceRemains
            | Self::NextMatchingCast
            | Self::Never => true,
        }
    }

    pub(super) const fn survives_cleanup(self) -> bool {
        // An end-of-combat effect is already gone by cleanup; keeping it in
        // this list would be harmless but says the wrong thing.
        match self {
            Self::EndOfTurn | Self::EndOfCombat => false,
            Self::AnyOf(set) => {
                !set.has(ContinuousEffectExpirationSet::END_OF_TURN)
                    && !set.has(ContinuousEffectExpirationSet::END_OF_COMBAT)
            }
            _ => true,
        }
    }

    pub(super) const fn survives_end_of_combat(self) -> bool {
        match self {
            Self::EndOfCombat => false,
            Self::AnyOf(set) => !set.has(ContinuousEffectExpirationSet::END_OF_COMBAT),
            _ => true,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub(super) enum ResolvedAbilityOperation {
    Add { ability: AbilityDef, grant: GrantId },
    Remove(AbilityPredicateDef),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ResolvedPowerToughnessOperation {
    SetBase {
        power: i16,
        toughness: i16,
    },
    SetBasePower {
        power: i16,
    },
    SetBaseToughness {
        toughness: i16,
    },
    Modify {
        power: i16,
        toughness: i16,
    },
    /// Layer 7e, applied after all of the above. Carries nothing: two in
    /// effect at once cancel each other out.
    Switch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub(super) enum ResolvedContinuousEffectKind {
    Abilities(ResolvedAbilityOperation),
    BasicLandTypes(SetOperationDef<&'static [BasicLandType]>),
    CardTypes(SetOperationDef<CardTypeSet>),
    Supertypes(SetOperationDef<CardSupertypeSet>),
    Colors(SetOperationDef<ColorSet>),
    CreatureTypes(SetOperationDef<CreatureTypeSetDef>),
    Subtypes(SetOperationDef<&'static [&'static str]>),
    PowerToughness(ResolvedPowerToughnessOperation),
    Rule(AppliedRuleDef),
}

/// One resolved, noncopiable continuous-effect component attached to a
/// battlefield object. Compound authored effects share a timestamp and keep
/// their depth-first component order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ResolvedContinuousEffect {
    pub(super) definition: AppliedEffectDef,
    pub(super) source: super::AbilitySourceRef,
    pub(super) timestamp: ContinuousEffectTimestamp,
    pub(super) component_order: u16,
    pub(super) expiration: ContinuousEffectExpiration,
    pub(super) kind: ResolvedContinuousEffectKind,
}

/// One resolving play prohibition after its player recipient has been frozen.
/// Static play prohibitions stay source-derived and are not stored here.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ResolvedPlayRestriction {
    pub(super) definition: AppliedEffectDef,
    pub(super) source: AbilitySourceRef,
    pub(super) affected_player: PlayerId,
    pub(super) timestamp: ContinuousEffectTimestamp,
    pub(super) component_order: u16,
    pub(super) expiration: ContinuousEffectExpiration,
    pub(super) restriction: PlayRestrictionDef,
}

/// A player's protection after it has resolved. Stored the way the play
/// prohibitions beside it are: the player it protects and how long it lasts
/// are frozen here, so the source can leave without taking it away.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ResolvedPlayerProtection {
    pub(super) definition: AppliedEffectDef,
    pub(super) source: AbilitySourceRef,
    pub(super) affected_player: PlayerId,
    pub(super) expiration: ContinuousEffectExpiration,
    pub(super) quality: crate::card::ObjectPredicateDef,
}

/// A resolving player-facing rule whose authored definition is all the
/// runtime needs beyond its affected player and expiration. Static versions
/// of the same rules remain source-derived from the battlefield.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ResolvedPlayerRule {
    pub(super) definition: AppliedEffectDef,
    pub(super) source: AbilitySourceRef,
    pub(super) affected_player: PlayerId,
    pub(super) expiration: ContinuousEffectExpiration,
    pub(super) rule: crate::card::PlayerRuleDef,
}

/// A resolving attack prohibition after its protected player has been
/// frozen. Static creature-scoped attack restrictions remain source-derived;
/// this is for effects such as Island Sanctuary that protect a player for a
/// duration even after their source leaves the battlefield.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ResolvedAttackRestriction {
    pub(super) definition: AppliedEffectDef,
    pub(super) source: AbilitySourceRef,
    pub(super) affected_player: PlayerId,
    pub(super) expiration: ContinuousEffectExpiration,
    pub(super) restriction: crate::card::AttackRestrictionDef,
}

/// One live static or resolved attack restriction after its player recipient
/// has been identified. Attack restrictions do not layer; prohibitions
/// compose and declaration costs add.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AppliedAttackRestriction {
    pub(super) source: GameObjectId,
    pub(super) affected_player: PlayerId,
    pub(super) restriction: crate::card::AttackRestrictionDef,
}

/// A permission to play from an unusual zone that resolved rather than being
/// printed on a permanent.
///
/// The mirror of [`ResolvedPlayRestriction`], and stored the same way for the
/// same reason: its subject is a player, so there is no object to hang it on
/// and no layer walk that would find it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ResolvedPlayPermission {
    pub(super) definition: AppliedEffectDef,
    pub(super) source: AbilitySourceRef,
    pub(super) affected_player: PlayerId,
    pub(super) expiration: ContinuousEffectExpiration,
    pub(super) rule: AppliedRuleDef,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
// Ability definitions are immutable catalog values with static references.
// Keeping this operation Copy avoids allocation in the hot ability-layer walk.
#[allow(clippy::large_enum_variant)]
pub(super) enum AbilityLayerOperationKind {
    Add {
        origin: AbilityOrigin,
        ability: AbilityDef,
    },
    Remove(AbilityPredicateDef),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AbilityLayerOperation {
    pub(super) timestamp: ContinuousEffectTimestamp,
    pub(super) order: u16,
    pub(super) kind: AbilityLayerOperationKind,
}

/// An ability granted to one non-battlefield object. The object identity
/// naturally makes the grant end if that card changes zones; the expiration
/// handles any shorter printed duration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct NonbattlefieldAbilityGrant {
    pub(super) object: GameObjectId,
    pub(super) ability: AbilityDef,
    pub(super) expiration: ContinuousEffectExpiration,
    /// The authored ability that created this grant, when it resolved from a
    /// cataloged source. Generated abilities use this provenance too.
    pub(super) source: Option<AbilityOrigin>,
}

/// A resolved duration-scoped effect that can be activated from outside every
/// ordinary zone. It deliberately is not a [`Permanent`]: the command-zone
/// source classification is only the closest gameplay-equivalent home for
/// activation checks, not a claim that the rules effect technically has a
/// zone.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ResolvedOngoingEffect {
    pub(super) source: AbilitySourceRef,
    pub(super) owner: PlayerId,
    pub(super) controller: PlayerId,
    pub(super) presentation: ObjectCharacteristics,
    pub(super) ability: AbilityDef,
    pub(super) context: EffectResolutionContext,
    pub(super) expiration: ContinuousEffectExpiration,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct StaticAppliedEffect {
    pub(super) source: GameObjectId,
    pub(super) timestamp: ContinuousEffectTimestamp,
    pub(super) source_presentation: ObjectCharacteristics,
    pub(super) source_origin: AbilityOrigin,
    pub(super) grant: Option<GrantId>,
    pub(super) component_order: u16,
    pub(super) effect: AppliedEffectDef,
}

/// One rule leaf after static and resolved continuous effects have converged.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AppliedRuleEffect {
    pub(super) source: GameObjectId,
    pub(super) timestamp: ContinuousEffectTimestamp,
    pub(super) component_order: u16,
    pub(super) rule: AppliedRuleDef,
}

/// One static or resolved play prohibition after source, recipient, and
/// ordering metadata have converged.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AppliedPlayRestriction {
    pub(super) source: GameObjectId,
    pub(super) timestamp: ContinuousEffectTimestamp,
    pub(super) component_order: u16,
    pub(super) restriction: PlayRestrictionDef,
}

#[derive(Clone, Copy)]
pub(super) enum StaticAffectedObject<'a> {
    Permanent {
        affected: &'a Permanent,
        prospective: Option<&'a Permanent>,
    },
    Object {
        characteristics: &'a TriggerEventObject,
        controller: Option<PlayerId>,
        owner: PlayerId,
        zone: ZoneKind,
        is_spell: bool,
    },
}

pub(super) struct StaticEffectTraversal<'a> {
    pub(super) source: &'a Permanent,
    pub(super) source_timestamp: ContinuousEffectTimestamp,
    pub(super) source_presentation: ObjectCharacteristics,
    pub(super) source_origin: AbilityOrigin,
    pub(super) affected: StaticAffectedObject<'a>,
    pub(super) next_grant: usize,
    pub(super) next_component_order: u16,
}
