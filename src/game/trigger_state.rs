use std::borrow::Cow;

use crate::action::{AbilityOrigin, Target};
use crate::card::{
    AbilityTargetDef, CardSupertype, CardTypeSet, EffectDef, PlayerRelation, TriggerConditionDef,
    TriggerEventDef, TurnStepDef, ZoneKind,
};
use crate::casting::TargetSelection;
use crate::ids::{CardDefinitionId, ChoiceIndex, GameObjectId, PlayerId};

use super::{ScopedEffect, StackAbilityResolver, StackObject};

/// An effect queued for the next time a step begins. Whatever queued it has
/// usually left by then, so the entry carries its own source and controller.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct DelayedTrigger {
    /// The object that queued this, kept whole so the effect resolves with
    /// the same source and controller it would have had at the time.
    pub(super) object: Box<StackObject>,
    /// Trigger-event information captured when the effect was scheduled.
    pub(super) context: TriggerContext,
    pub(super) step: TurnStepDef,
    pub(super) player: PlayerRelation,
    pub(super) effect: ScopedEffect,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct TriggerContext {
    pub(super) object: Option<GameObjectId>,
    pub(super) object_controller: Option<PlayerId>,
    pub(super) event_player: Option<PlayerId>,
    pub(super) amount: Option<i32>,
    /// Non-targeting object choices made during this resolution, indexed in
    /// the authored effect tree rather than stored on the stack as targets.
    pub(super) chosen_objects: [Option<GameObjectId>; ChoiceIndex::COUNT],
}

impl TriggerContext {
    pub(super) const fn empty() -> Self {
        Self {
            object: None,
            object_controller: None,
            event_player: None,
            amount: None,
            chosen_objects: [None; ChoiceIndex::COUNT],
        }
    }

    pub(super) const fn chosen_object(self, choice: ChoiceIndex) -> Option<GameObjectId> {
        self.chosen_objects[choice.index()]
    }

    pub(super) fn bind_choice(&mut self, choice: ChoiceIndex, object: Option<GameObjectId>) {
        self.chosen_objects[choice.index()] = object;
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub(super) struct TriggerEventObject {
    pub(super) id: GameObjectId,
    pub(super) token: bool,
    pub(super) types: CardTypeSet,
    pub(super) controller: PlayerId,
    pub(super) colors: [bool; 5],
    pub(super) subtypes: Cow<'static, [&'static str]>,
    pub(super) mana_value: u16,
    /// Current power where one exists: a battlefield creature reports what it
    /// is now, not what it was printed as.
    pub(super) power: Option<i16>,
    /// Current toughness, read the same way and with the same caveat.
    pub(super) toughness: Option<i16>,
    pub(super) supertypes: [bool; CardSupertype::COUNT],
    /// Whether this object is in combat. Cheap to carry and it cannot feed
    /// back into a characteristic, unlike a keyword or a static bonus.
    pub(super) attacking_or_blocking: bool,
    /// Printed and temporary keywords, as a bitmask over
    /// [`crate::card::KeywordAbility::simple_index`].
    ///
    /// A keyword granted by a static effect is deliberately missing, for the
    /// same reason power is: resolving static effects matches their sources
    /// against these characteristics, so reading a granted keyword back here
    /// would not terminate.
    pub(super) keywords: u32,
    /// Whether this creature is attacking, excluding a creature that is only
    /// blocking. Bloodrush and similar predicates need the narrower state.
    pub(super) attacking: bool,
    /// Whether this creature attacked at any point this turn, which outlives
    /// combat and so is not the same question as `attacking`.
    pub(super) attacked_this_turn: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum CommittedTriggerEvent {
    ZoneChanged {
        object: TriggerEventObject,
        from: ZoneKind,
        to: ZoneKind,
    },
    BecomesTapped {
        object: TriggerEventObject,
    },
    LifeGained {
        player: PlayerId,
        amount: u16,
    },
    Attacks {
        object: TriggerEventObject,
    },
    TappedForMana {
        object: TriggerEventObject,
    },
    DamageDealt {
        source: TriggerEventObject,
        recipient: Target,
        amount: u16,
        combat: bool,
    },
    CombatDamageDealtToPlayer {
        object: TriggerEventObject,
        player: PlayerId,
        amount: u16,
    },
    DamageDealtToPlayer {
        object: TriggerEventObject,
        player: PlayerId,
        amount: u16,
    },
    SpellCast {
        object: TriggerEventObject,
    },
    Transformed {
        object: TriggerEventObject,
    },
    StepBegins {
        step: TurnStepDef,
        player: PlayerId,
    },
    DamagedCreatureDied {
        object: TriggerEventObject,
        source: GameObjectId,
    },
}

impl CommittedTriggerEvent {
    pub(super) fn context(&self) -> TriggerContext {
        match self {
            Self::ZoneChanged { object, .. }
            | Self::BecomesTapped { object }
            | Self::Attacks { object }
            | Self::Transformed { object }
            | Self::DamagedCreatureDied { object, .. } => TriggerContext {
                object: Some(object.id),
                object_controller: Some(object.controller),
                event_player: None,
                amount: None,
                chosen_objects: [None; ChoiceIndex::COUNT],
            },
            Self::DamageDealt {
                source,
                recipient,
                amount,
                ..
            } => TriggerContext {
                object: Some(source.id),
                object_controller: Some(source.controller),
                event_player: match recipient {
                    Target::Player(player) => Some(*player),
                    Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                },
                amount: Some(i32::from(*amount)),
                chosen_objects: [None; ChoiceIndex::COUNT],
            },
            Self::CombatDamageDealtToPlayer {
                object,
                player,
                amount,
            }
            | Self::DamageDealtToPlayer {
                object,
                player,
                amount,
            } => TriggerContext {
                object: Some(object.id),
                object_controller: Some(object.controller),
                event_player: Some(*player),
                amount: Some(i32::from(*amount)),
                chosen_objects: [None; ChoiceIndex::COUNT],
            },
            Self::LifeGained { player, amount } => TriggerContext {
                object: None,
                object_controller: None,
                event_player: Some(*player),
                amount: Some(i32::from(*amount)),
                chosen_objects: [None; ChoiceIndex::COUNT],
            },
            // The player who tapped a permanent for mana is its controller,
            // which is the same shape a cast spell has.
            Self::TappedForMana { object } | Self::SpellCast { object } => TriggerContext {
                object: Some(object.id),
                object_controller: Some(object.controller),
                event_player: Some(object.controller),
                amount: None,
                chosen_objects: [None; ChoiceIndex::COUNT],
            },
            Self::StepBegins { player, .. } => TriggerContext {
                object: None,
                object_controller: None,
                event_player: Some(*player),
                amount: None,
                chosen_objects: [None; ChoiceIndex::COUNT],
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AbilitySourceRef {
    pub(super) object: GameObjectId,
    pub(super) ability: AbilityOrigin,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PendingTrigger {
    pub(super) id: u32,
    pub(super) source: AbilitySourceRef,
    pub(super) definition: CardDefinitionId,
    pub(super) owner: PlayerId,
    pub(super) controller: PlayerId,
    pub(super) text: &'static str,
    pub(super) target_defs: &'static [AbilityTargetDef],
    pub(super) targets: Vec<TargetSelection>,
    pub(super) effect: EffectDef,
    pub(super) resolver: StackAbilityResolver,
    pub(super) context: TriggerContext,
    pub(super) condition: Option<&'static TriggerConditionDef>,
}

/// The immutable declaration captured when one event matches one source
/// ability. The game assigns the ephemeral trigger ID when it accepts this
/// record into the pending-trigger queue.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct TriggerCapture {
    pub(super) source: AbilitySourceRef,
    pub(super) definition: CardDefinitionId,
    pub(super) owner: PlayerId,
    pub(super) controller: PlayerId,
    pub(super) text: &'static str,
    pub(super) target_defs: &'static [AbilityTargetDef],
    pub(super) effect: EffectDef,
    pub(super) resolver: StackAbilityResolver,
    pub(super) context: TriggerContext,
    /// The intervening-if condition this trigger reads, checked both when the
    /// ability would go on the stack and again when it resolves.
    pub(super) condition: Option<&'static TriggerConditionDef>,
}

/// A triggered ability with no object behind it, installed by an effect and
/// listening until its controller's next turn begins. Everything the trigger
/// needs is frozen here, because the ability that created it has finished
/// resolving and its source may be long gone.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct FloatingTrigger {
    pub(super) event: TriggerEventDef,
    pub(super) capture: TriggerCapture,
    pub(super) until_turn_of: PlayerId,
    /// How many turns that player had already started, so the turn the
    /// ability resolved during does not count as their next one.
    pub(super) created_after_turns: u32,
}

/// One battlefield trigger listener frozen at the start of an atomic event.
/// A simultaneous zone change can remove the source before another object in
/// the same event is published, so listener discovery cannot consult the
/// incrementally-mutated battlefield.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct BattlefieldTriggerListener {
    pub(super) event: TriggerEventDef,
    pub(super) uses_stack: bool,
    pub(super) capture: TriggerCapture,
}

#[derive(Clone, Debug)]
pub(super) struct TriggerPlacementBatch {
    pub(super) controller: PlayerId,
    pub(super) triggers: Vec<PendingTrigger>,
}
