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

/// A frozen triggered ability listening for one future committed event.
///
/// Unlike [`DelayedTrigger`], this is a real trigger: the matching event only
/// moves its capture into the pending-trigger queue. Ordinary APNAP ordering,
/// target selection, countering, and priority then apply. The listener is
/// removed after its first matching event even if an intervening-if condition
/// keeps the ability from triggering.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ScheduledTrigger {
    pub(super) id: u32,
    pub(super) event: TriggerEventDef,
    pub(super) capture: TriggerCapture,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct TriggerContext {
    pub(super) object: Option<GameObjectId>,
    pub(super) object_controller: Option<PlayerId>,
    pub(super) event_player: Option<PlayerId>,
    pub(super) amount: Option<i32>,
    /// The host and mechanic link captured from the triggered ability's own
    /// source. These are source LKI, not properties of the triggering event.
    pub(super) source_attachment: Option<GameObjectId>,
    pub(super) source_linked: Option<GameObjectId>,
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
            source_attachment: None,
            source_linked: None,
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
    /// The object's keywords, as a bitmask over
    /// [`crate::card::KeywordAbility::simple_index`].
    ///
    /// Unlike power, this includes keywords a static continuous effect grants
    /// or removes: `Game::keyword_mask` stratifies the layer-6 walk rather than
    /// omitting it, so a predicate here and the combat rules give one answer.
    /// The one exception is the walk's own recipient matching, which reads the
    /// layer below itself; `Game::collect_ability_layer_operations` says why.
    pub(super) keywords: u32,
    /// Whether this creature is attacking, excluding a creature that is only
    /// blocking. Bloodrush and similar predicates need the narrower state.
    pub(super) attacking: bool,
    /// Whether the object is a tapped permanent. Cheap to carry, and like
    /// `attacking` it cannot feed back into a characteristic.
    pub(super) tapped: bool,
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
    AttacksInGroup {
        object: TriggerEventObject,
        total: u8,
    },
    Attacks {
        object: TriggerEventObject,
    },
    BecomesBlocked {
        object: TriggerEventObject,
        defending_player: PlayerId,
        /// Blockers beyond the first, so a clause reading the trigger amount
        /// gets the quantity it is printed against without recounting.
        blockers_beyond_first: u16,
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
            | Self::AttacksInGroup { object, .. }
            | Self::Attacks { object }
            | Self::Transformed { object }
            | Self::DamagedCreatureDied { object, .. } => TriggerContext {
                object: Some(object.id),
                object_controller: Some(object.controller),
                event_player: None,
                amount: None,
                ..TriggerContext::empty()
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
                ..TriggerContext::empty()
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
                ..TriggerContext::empty()
            },
            Self::BecomesBlocked {
                object,
                defending_player,
                blockers_beyond_first,
            } => TriggerContext {
                object: Some(object.id),
                object_controller: Some(object.controller),
                event_player: Some(*defending_player),
                amount: Some(i32::from(*blockers_beyond_first)),
                ..TriggerContext::empty()
            },
            Self::LifeGained { player, amount } => TriggerContext {
                object: None,
                object_controller: None,
                event_player: Some(*player),
                amount: Some(i32::from(*amount)),
                ..TriggerContext::empty()
            },
            // The player who tapped a permanent for mana is its controller,
            // which is the same shape a cast spell has.
            Self::TappedForMana { object } | Self::SpellCast { object } => TriggerContext {
                object: Some(object.id),
                object_controller: Some(object.controller),
                event_player: Some(object.controller),
                amount: None,
                ..TriggerContext::empty()
            },
            Self::StepBegins { player, .. } => TriggerContext {
                object: None,
                object_controller: None,
                event_player: Some(*player),
                amount: None,
                ..TriggerContext::empty()
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
