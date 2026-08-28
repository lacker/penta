use crate::action::AbilityOrigin;
use crate::card::{
    AbilityDef, AbilityPredicateDef, AppliedEffectDef, AppliedRuleDef, BasicLandType, CardTypeSet,
    ColorSet, CreatureTypeSetDef, PlayRestrictionDef, SetOperationDef,
};
use crate::ids::{GameObjectId, GrantId, PlayerId};

use super::{AbilitySourceRef, EffectResolutionContext, ObjectCharacteristics, Permanent};

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
    Never,
}

impl ContinuousEffectExpiration {
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
            Self::UpkeepOf(_)
            | Self::EndOfTurn
            | Self::EndOfCombat
            | Self::WhileSourceTapped
            | Self::WhileSourceRemains
            | Self::Never => true,
        }
    }

    /// Whether this effect outlives the untap step it was carried into.
    pub(super) fn survives_untap_step(self, active_player: PlayerId) -> bool {
        match self {
            Self::UpkeepOf(player) => player != active_player,
            Self::TurnOf { .. }
            | Self::EndOfTurn
            | Self::EndOfCombat
            | Self::WhileSourceTapped
            | Self::WhileSourceRemains
            | Self::Never => true,
        }
    }

    pub(super) const fn survives_cleanup(self) -> bool {
        // An end-of-combat effect is already gone by cleanup; keeping it in
        // this list would be harmless but says the wrong thing.
        !matches!(self, Self::EndOfTurn | Self::EndOfCombat)
    }

    pub(super) const fn survives_end_of_combat(self) -> bool {
        !matches!(self, Self::EndOfCombat)
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

pub(super) struct StaticEffectTraversal<'a> {
    pub(super) source: &'a Permanent,
    pub(super) source_timestamp: ContinuousEffectTimestamp,
    pub(super) source_presentation: ObjectCharacteristics,
    pub(super) source_origin: AbilityOrigin,
    pub(super) affected: &'a Permanent,
    pub(super) prospective: Option<&'a Permanent>,
    pub(super) next_grant: usize,
    pub(super) next_component_order: u16,
}
