use std::borrow::Cow;

use crate::action::{AbilityOrigin, Target};
use crate::card::{
    AbilityTargetDef, CardSupertype, CardTypeSet, EffectDef, ModalSpellDef, TriggerConditionDef,
    TriggerEventDef, TurnStepDef, ZoneKind,
};
use crate::casting::TargetSelection;
use crate::ids::{GameObjectId, ObjectBindingIndex, ObjectSetBindingIndex, PlayerId};

use super::{ObjectCharacteristics, StackAbilityResolver};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct TriggerContext {
    pub(super) object: Option<GameObjectId>,
    /// The exact object created by the zone change that caused this trigger.
    /// This is event-local rather than inferred from the global successor
    /// graph, and becomes stale if that object moves again before resolution.
    pub(super) zone_change_result: Option<GameObjectId>,
    pub(super) object_controller: Option<PlayerId>,
    pub(super) event_player: Option<PlayerId>,
    pub(super) amount: Option<i32>,
    /// What the event's damage was dealt to, when it was dealt to an object.
    /// Kept apart from `object`, which for a damage event is the source that
    /// dealt it: "whenever this creature deals combat damage to a creature,
    /// exile that creature" names both, and they are never the same one.
    pub(super) damaged_object: Option<GameObjectId>,
}

impl TriggerContext {
    pub(super) const fn empty() -> Self {
        Self {
            object: None,
            zone_change_result: None,
            object_controller: None,
            event_player: None,
            amount: None,
            damaged_object: None,
        }
    }
}

/// State local to one declarative effect resolution. Trigger information is
/// kept separate and copyable because it is also captured by abilities before
/// they ever resolve; bindings belong only to a particular continuation of an
/// effect program.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct EffectResolutionContext {
    pub(super) trigger: TriggerContext,
    /// What a payment made during this resolution actually cost, for the
    /// branch that reads it back. "You may pay {X}" settles X here rather
    /// than at the cast, which is where an ordinary X lives.
    pub(super) paid_amount: Option<u16>,
    /// How many objects the previous step matched, for a follow-up that
    /// counts them: the land cards a discard actually took.
    pub(super) matched_count: Option<u16>,
    /// Distinct card types among those same matched objects.
    pub(super) matched_card_types: Option<u16>,
    /// What those same matched objects add up to in mana value, for a
    /// follow-up measured by what the step before it turned up.
    pub(super) matched_mana_value: Option<u16>,
    /// A card name chosen while this effect resolves, which the rest of the
    /// same resolution reads back. Cabal Therapy names one and then discards
    /// every copy of it.
    pub(super) chosen_name: Option<String>,
    single_objects: [Option<Target>; ObjectBindingIndex::COUNT],
    object_groups: [Vec<Target>; ObjectSetBindingIndex::COUNT],
}

impl EffectResolutionContext {
    pub(super) fn new(trigger: TriggerContext) -> Self {
        Self {
            trigger,
            paid_amount: None,
            matched_count: None,
            matched_card_types: None,
            matched_mana_value: None,
            chosen_name: None,
            single_objects: [None; ObjectBindingIndex::COUNT],
            object_groups: std::array::from_fn(|_| Vec::new()),
        }
    }

    #[cfg(test)]
    pub(super) fn empty() -> Self {
        Self::new(TriggerContext::empty())
    }

    pub(super) const fn single_object(&self, binding: ObjectBindingIndex) -> Option<Target> {
        self.single_objects[binding.index()]
    }

    pub(super) fn bind_single_object(
        &mut self,
        binding: ObjectBindingIndex,
        object: Option<Target>,
    ) {
        self.single_objects[binding.index()] = object;
    }

    pub(super) fn object_group(&self, binding: ObjectSetBindingIndex) -> &[Target] {
        &self.object_groups[binding.index()]
    }

    pub(super) fn bind_object_group(
        &mut self,
        binding: ObjectSetBindingIndex,
        objects: Vec<Target>,
    ) {
        self.object_groups[binding.index()] = objects;
    }

    pub(super) fn single_objects(&self) -> &[Option<Target>; ObjectBindingIndex::COUNT] {
        &self.single_objects
    }

    pub(super) fn object_groups(&self) -> &[Vec<Target>; ObjectSetBindingIndex::COUNT] {
        &self.object_groups
    }

    pub(super) fn from_bindings(
        trigger: TriggerContext,
        single_objects: [Option<Target>; ObjectBindingIndex::COUNT],
        object_groups: [Vec<Target>; ObjectSetBindingIndex::COUNT],
    ) -> Self {
        Self {
            trigger,
            paid_amount: None,
            matched_count: None,
            matched_card_types: None,
            matched_mana_value: None,
            chosen_name: None,
            single_objects,
            object_groups,
        }
    }
}

impl From<TriggerContext> for EffectResolutionContext {
    fn from(trigger: TriggerContext) -> Self {
        Self::new(trigger)
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
    pub(super) keywords: u64,
    /// Whether this creature is attacking, excluding a creature that is only
    /// blocking. Bloodrush and similar predicates need the narrower state.
    pub(super) attacking: bool,
    /// Whether the object is a tapped permanent. Cheap to carry, and like
    /// `attacking` it cannot feed back into a characteristic.
    pub(super) tapped: bool,
    /// Whether this creature attacked at any point this turn, which outlives
    /// combat and so is not the same question as `attacking`.
    pub(super) attacked_this_turn: bool,
    pub(super) saddled: bool,
    /// Whether it attacked during its controller's previous turn. Answered
    /// where the snapshot is built, because the turn count it is measured
    /// against belongs to the game rather than to the permanent.
    pub(super) attacked_during_controllers_last_turn: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum CommittedTriggerEvent {
    ZoneChanged {
        /// Last-known information for the object before the move. Some entry
        /// paths do not need or retain a readable pre-move representation.
        before: Option<TriggerEventObject>,
        /// The new object created in the destination, absent when a token
        /// leaves the battlefield and ceases to exist.
        after: Option<TriggerEventObject>,
        from: ZoneKind,
        to: ZoneKind,
        /// Damage sources recorded on the departing battlefield object. This
        /// is frozen with the exit event and empty for all other moves.
        damage_sources: Vec<GameObjectId>,
    },
    Tapped {
        object: TriggerEventObject,
        for_mana: bool,
    },
    LifeGained {
        player: PlayerId,
        amount: u16,
    },
    /// One card left a hand for a graveyard. The card is read where it now
    /// lies, which is what "whenever you discard a card, you may exile that
    /// card from your graveyard" names: the discard is over by the time the
    /// trigger fires, so the object it points at is the graveyard card
    /// rather than the one that was in hand. `None` where the card left no
    /// readable object behind.
    Discarded {
        player: PlayerId,
        card: Option<TriggerEventObject>,
    },
    /// One discard, however many cards it moved. Raised beside the per-card
    /// event above rather than instead of it: the two wordings are both
    /// printed, and both have to be answerable.
    CardsDiscarded {
        player: PlayerId,
    },
    Attacks {
        object: TriggerEventObject,
        declaration_size: u8,
        attack_number: u8,
        /// The player being attacked, including the controller of a
        /// planeswalker chosen as this attack's defender.
        defending_player: PlayerId,
        /// Whether the attack was declared against a planeswalker rather
        /// than against that player. Both are attacking them (CR 506.3b);
        /// a clause that says "attacks the player" wants only the one.
        attacked_a_planeswalker: bool,
    },
    /// One whole move of cards into exile, published once however many
    /// cards it took. "Whenever one or more cards are put into exile" reads
    /// this rather than any card in it.
    CardsExiled {
        cards: Vec<TriggerEventObject>,
        from: ZoneKind,
        owner: PlayerId,
    },
    /// Every token one instruction created, published once. A token is
    /// created as it enters (CR 111.11), so what is published here is what
    /// actually arrived rather than what was asked for.
    TokensCreated {
        tokens: Vec<TriggerEventObject>,
        controller: PlayerId,
    },
    /// One whole attack declaration, published once however many creatures
    /// were declared. "Whenever you attack" watches this rather than any of
    /// the attackers in it (CR 508.1).
    AttackersDeclared {
        attackers: Vec<TriggerEventObject>,
    },
    BecomesBlocked {
        object: TriggerEventObject,
        /// Blockers beyond the first, so a clause reading the trigger amount
        /// gets the quantity it is printed against without recounting.
        blockers_beyond_first: u16,
    },
    /// Every object that went from the battlefield to a graveyard in one
    /// batch, published once. The batched counterpart of the per-object zone
    /// change beside it, for the clauses that read "one or more ... die" as
    /// one event.
    ObjectsDied {
        objects: Vec<TriggerEventObject>,
    },
    /// An attacker that no creature blocked, committed once blockers are
    /// declared.
    AttacksAndIsNotBlocked {
        object: TriggerEventObject,
    },
    /// Every unblocked attacker pointed at one player, published once as
    /// blockers are declared. The batched counterpart of
    /// [`Self::AttacksAndIsNotBlocked`], for the clauses that read "one or
    /// more creatures ... attack you and aren't blocked" as one event.
    UnblockedAttackersDeclared {
        attackers: Vec<TriggerEventObject>,
        defending_player: PlayerId,
    },
    /// One combat damage step's worth of combat damage to players, published
    /// once however many creatures dealt it. The batched counterpart of the
    /// per-source [`Self::DamageDealt`], for the clauses that read "one or
    /// more creatures ... deal combat damage to one or more players" as one
    /// event. Damage redirected onto a permanent is not in it: what lands on
    /// a player is what the clause counts.
    CombatDamageDealtToPlayers {
        sources: Vec<TriggerEventObject>,
        players: Vec<PlayerId>,
    },
    /// One side of one blocking relationship. Emitted once per ordered pair,
    /// so a clause on either creature sees the other as the triggering
    /// object without having to know which of them attacked.
    BlocksOrBecomesBlocked {
        creature: TriggerEventObject,
        other: TriggerEventObject,
    },
    DamageDealt {
        source: Option<TriggerEventObject>,
        /// Whether the source snapshot represents a spell on the stack.
        /// Object characteristics alone cannot distinguish a spell from the
        /// same card in another zone, so this fact is frozen separately.
        source_is_spell: bool,
        recipient: Target,
        recipient_object: Option<TriggerEventObject>,
        amount: u16,
        combat: bool,
    },
    SpellCast {
        object: TriggerEventObject,
    },
    /// A copy of a spell was put on the stack. Not a cast: nothing was
    /// announced, nothing was paid, and only the clauses that say "or copy"
    /// watch for it.
    SpellCopied {
        object: TriggerEventObject,
    },
    /// A player became the monarch.
    BecameMonarch {
        player: PlayerId,
    },
    /// A Class reached a level it had not reached before. One event per
    /// level crossed, so a Class taken from one to three raises two.
    BecameLevel {
        object: GameObjectId,
        level: u8,
    },
    /// A player drew a card. Raised once per card, where the card reaches
    /// the hand -- a draw that was replaced never happened.
    DrewCard {
        player: PlayerId,
        card: TriggerEventObject,
        /// Whether this was the first card the player drew during their own
        /// draw step. Orcish Bowmasters is the reason the event carries it:
        /// nothing about the game state afterwards distinguishes the
        /// turn-based draw from the one a Howling Mine added to it.
        first_in_draw_step: bool,
        /// Which card of this player's turn it was, counting from one. The
        /// clauses that name an ordinary ordinal -- "their second card each
        /// turn" -- read this rather than the running count, which has
        /// already moved on by the time a trigger resolves.
        nth_this_turn: u16,
    },
    /// An object became the target of a spell as that spell was cast. The
    /// object carried is the spell, so "that spell's controller" reads off
    /// the event; `target` is what it pointed at.
    BecameTargetOfSpell {
        target: GameObjectId,
        object: TriggerEventObject,
    },
    /// An object became the target of an activated or triggered ability as
    /// that ability was put onto the stack. Kept apart from
    /// [`Self::BecameTargetOfSpell`] because the printed clauses do: "becomes
    /// the target of a spell" is not answered by an ability, and ward is
    /// answered by both.
    BecameTargetOfAbility {
        target: GameObjectId,
        object: TriggerEventObject,
    },
    /// A land was played, which is a special action rather than a zone
    /// change of its own (CR 305.1). Kept apart from the entry it causes
    /// because the two are not the same event: a land put onto the
    /// battlefield by an effect enters without ever being played, and
    /// "when you play another land" is about the playing.
    LandPlayed {
        player: PlayerId,
        object: TriggerEventObject,
    },
    /// A player became the target of a spell or of an ability. Kept apart
    /// from the two above because a player is not an object: nothing on the
    /// battlefield was pointed at, so a clause watching this reads the side
    /// of the table rather than a permanent.
    PlayerBecameTarget {
        player: PlayerId,
        object: TriggerEventObject,
    },
    Transformed {
        object: TriggerEventObject,
    },
    /// Counters were put on an object. The amount is carried so a clause
    /// that asks how many can read it, though the trigger itself fires once
    /// however many arrived at once.
    CountersPlaced {
        object: TriggerEventObject,
        kind: crate::card::CounterKind,
        amount: u16,
    },
    /// A player committed a crime. Only who did it is carried: the printed
    /// clauses ask whether it was you, never what you pointed at.
    CommittedCrime {
        player: PlayerId,
    },
    StepBegins {
        step: TurnStepDef,
        player: PlayerId,
    },
    /// A card was cycled. The object is the card in the graveyard, which is
    /// where the discard cost has already put it.
    Cycled {
        object: TriggerEventObject,
    },
    /// A creature was exerted as it was declared as an attacker
    /// (CR 701.38a).
    Exerted {
        object: TriggerEventObject,
    },
    /// A permanent was sacrificed, captured before it left so what it was
    /// is still readable.
    Sacrificed {
        object: TriggerEventObject,
        player: PlayerId,
    },
}

impl CommittedTriggerEvent {
    #[allow(clippy::too_many_lines)]
    pub(super) fn context(&self) -> TriggerContext {
        match self {
            Self::ZoneChanged { before, after, .. } => {
                let object = before.as_ref().or(after.as_ref());
                TriggerContext {
                    object: object.map(|object| object.id),
                    zone_change_result: after.as_ref().map(|object| object.id),
                    object_controller: object.map(|object| object.controller),
                    event_player: None,
                    amount: None,
                    damaged_object: None,
                }
            }
            Self::Transformed { object }
            | Self::Cycled { object }
            | Self::Exerted { object }
            | Self::AttacksAndIsNotBlocked { object } => TriggerContext {
                object: Some(object.id),
                zone_change_result: None,
                object_controller: Some(object.controller),
                event_player: None,
                amount: None,
                damaged_object: None,
            },
            // Who did it is the half that "whenever you sacrifice" and "when
            // you play another land" read, and what it was done to is the
            // other.
            Self::Sacrificed { object, player } | Self::LandPlayed { object, player } => {
                TriggerContext {
                    object: Some(object.id),
                    zone_change_result: None,
                    object_controller: Some(object.controller),
                    event_player: Some(*player),
                    amount: None,
                    damaged_object: None,
                }
            }
            // The event is the instruction rather than any token in it, so
            // nothing here names one; how many were made is the amount.
            Self::TokensCreated { tokens, controller } => TriggerContext {
                object: None,
                zone_change_result: None,
                object_controller: Some(*controller),
                event_player: Some(*controller),
                amount: Some(i32::try_from(tokens.len()).unwrap_or(i32::MAX)),
                damaged_object: None,
            },
            // The event is the move rather than any card in it, so nothing
            // here names one; how many there were is the amount.
            Self::CardsExiled { cards, owner, .. } => TriggerContext {
                object: None,
                zone_change_result: None,
                object_controller: Some(*owner),
                event_player: Some(*owner),
                amount: Some(i32::try_from(cards.len()).unwrap_or(i32::MAX)),
                damaged_object: None,
            },
            // The event is the declaration rather than any creature in it,
            // so nothing here names one.
            // The event is the batch rather than any one death in it. The
            // amount is how many died, counted before the clause's own
            // predicate narrows them -- no card reads it, and the two would
            // only differ for a batch that mixed creatures with anything
            // else.
            Self::ObjectsDied { objects } => TriggerContext {
                object: None,
                zone_change_result: None,
                object_controller: objects.first().map(|object| object.controller),
                event_player: objects.first().map(|object| object.controller),
                amount: Some(i32::try_from(objects.len()).unwrap_or(i32::MAX)),
                damaged_object: None,
            },
            // The event is the batch rather than any creature in it. Who it
            // was aimed at is the defending player, and who aimed it is the
            // attackers' own controller, which is the player such a clause
            // hands its consequences to.
            Self::UnblockedAttackersDeclared {
                attackers,
                defending_player,
            } => TriggerContext {
                object: None,
                zone_change_result: None,
                object_controller: attackers.first().map(|attacker| attacker.controller),
                event_player: Some(*defending_player),
                amount: Some(i32::try_from(attackers.len()).unwrap_or(i32::MAX)),
                damaged_object: None,
            },
            // The event is the whole step rather than any creature in it, and
            // the amount is how many players took damage.
            Self::CombatDamageDealtToPlayers { sources, players } => TriggerContext {
                object: None,
                zone_change_result: None,
                object_controller: sources.first().map(|source| source.controller),
                event_player: players.first().copied(),
                amount: Some(i32::try_from(players.len()).unwrap_or(i32::MAX)),
                damaged_object: None,
            },
            Self::AttackersDeclared { attackers } => TriggerContext {
                object: None,
                zone_change_result: None,
                object_controller: attackers.first().map(|attacker| attacker.controller),
                event_player: attackers.first().map(|attacker| attacker.controller),
                amount: Some(i32::try_from(attackers.len()).unwrap_or(i32::MAX)),
                damaged_object: None,
            },
            Self::Attacks {
                object,
                defending_player,
                ..
            } => TriggerContext {
                object: Some(object.id),
                zone_change_result: None,
                object_controller: Some(object.controller),
                event_player: Some(*defending_player),
                amount: None,
                damaged_object: None,
            },
            Self::Tapped { object, for_mana } => TriggerContext {
                object: Some(object.id),
                zone_change_result: None,
                object_controller: Some(object.controller),
                event_player: for_mana.then_some(object.controller),
                amount: None,
                damaged_object: None,
            },
            Self::DamageDealt {
                source,
                recipient,
                recipient_object,
                amount,
                ..
            } => TriggerContext {
                object: source.as_ref().map(|source| source.id),
                zone_change_result: None,
                object_controller: source.as_ref().map(|source| source.controller),
                event_player: match recipient {
                    Target::Player(player) => Some(*player),
                    Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                },
                amount: Some(i32::from(*amount)),
                damaged_object: recipient_object.as_ref().map(|object| object.id),
            },
            Self::BlocksOrBecomesBlocked { other, .. } => TriggerContext {
                object: Some(other.id),
                zone_change_result: None,
                object_controller: Some(other.controller),
                event_player: None,
                amount: None,
                damaged_object: None,
            },
            Self::BecomesBlocked {
                object,
                blockers_beyond_first,
            } => TriggerContext {
                object: Some(object.id),
                zone_change_result: None,
                object_controller: Some(object.controller),
                event_player: None,
                amount: Some(i32::from(*blockers_beyond_first)),
                damaged_object: None,
            },
            Self::LifeGained { player, amount } => TriggerContext {
                object: None,
                zone_change_result: None,
                object_controller: None,
                event_player: Some(*player),
                amount: Some(i32::from(*amount)),
                damaged_object: None,
            },
            Self::CountersPlaced { object, amount, .. } => TriggerContext {
                object: Some(object.id),
                zone_change_result: None,
                object_controller: Some(object.controller),
                event_player: None,
                amount: Some(i32::from(*amount)),
                damaged_object: None,
            },
            Self::BecameTargetOfSpell { object, .. }
            | Self::BecameTargetOfAbility { object, .. }
            | Self::PlayerBecameTarget { object, .. }
            | Self::SpellCopied { object }
            | Self::SpellCast { object } => TriggerContext {
                object: Some(object.id),
                zone_change_result: None,
                object_controller: Some(object.controller),
                event_player: Some(object.controller),
                amount: None,
                damaged_object: None,
            },
            Self::BecameLevel { object, .. } => TriggerContext {
                object: Some(*object),
                zone_change_result: None,
                object_controller: None,
                event_player: None,
                amount: None,
                damaged_object: None,
            },
            Self::Discarded { player, card } => TriggerContext {
                object: card.as_ref().map(|card| card.id),
                zone_change_result: None,
                object_controller: card.as_ref().map(|card| card.controller),
                event_player: Some(*player),
                amount: None,
                damaged_object: None,
            },
            // A drawn card snapshot belongs to trigger matching only. The
            // draw does not reveal it, so these player-only events carry no
            // hidden-zone identity into resolution or a public checkpoint.
            Self::StepBegins { player, .. }
            | Self::CommittedCrime { player }
            | Self::CardsDiscarded { player }
            | Self::BecameMonarch { player }
            | Self::DrewCard { player, .. } => TriggerContext {
                object: None,
                zone_change_result: None,
                object_controller: None,
                event_player: Some(*player),
                amount: None,
                damaged_object: None,
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
    pub(super) presentation: ObjectCharacteristics,
    pub(super) owner: PlayerId,
    pub(super) controller: PlayerId,
    pub(super) text: &'static str,
    pub(super) target_defs: Vec<AbilityTargetDef>,
    pub(super) targets: Vec<TargetSelection>,
    pub(super) effect: EffectDef,
    pub(super) resolver: StackAbilityResolver,
    pub(super) context: EffectResolutionContext,
    pub(super) condition: Option<&'static TriggerConditionDef>,
    /// "Choose one --": the modes still to be chosen as this trigger is put
    /// onto the stack. Cleared once one is, because what the trigger then
    /// carries is that mode's own effect and targets.
    pub(super) modes: Option<ModalSpellDef>,
    pub(super) x: u16,
}

/// The immutable declaration captured when one event matches one source
/// ability. The game assigns the ephemeral trigger ID when it accepts this
/// record into the pending-trigger queue.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct TriggerCapture {
    pub(super) source: AbilitySourceRef,
    pub(super) presentation: ObjectCharacteristics,
    pub(super) owner: PlayerId,
    pub(super) controller: PlayerId,
    pub(super) text: &'static str,
    pub(super) target_defs: Vec<AbilityTargetDef>,
    pub(super) targets: Vec<TargetSelection>,
    pub(super) effect: EffectDef,
    pub(super) resolver: StackAbilityResolver,
    pub(super) context: EffectResolutionContext,
    /// The intervening-if condition this trigger reads, checked both when the
    /// ability would go on the stack and again when it resolves.
    pub(super) condition: Option<&'static TriggerConditionDef>,
    /// "Choose one --": the modes still to be chosen as this trigger is put
    /// onto the stack. Cleared once one is, because what the trigger then
    /// carries is that mode's own effect and targets.
    pub(super) modes: Option<ModalSpellDef>,
    /// The X chosen for the installing ability. Installed triggers retain the
    /// same resolving context as the effect that created them.
    pub(super) x: u16,
}

/// How long a trigger installed outside every zone continues listening.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum InstalledTriggerLifetime {
    /// Consume the listener on the first matching event, before checking an
    /// intervening-if condition or putting its ability on the stack.
    Once,
    /// Stop listening when this player's frozen future turn begins.
    UntilTurn { player: PlayerId, turn: u32 },
    /// Stop listening when the frozen turn it was installed on ends. The
    /// turn number is the game's own count, so an extra turn ends this the
    /// same way an ordinary one does.
    ThisTurn { turn: u32 },
}

/// A triggered ability installed by a resolved effect. Everything needed to
/// construct its stack object is frozen here because its source may be gone
/// by the time an event matches.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct InstalledTrigger {
    pub(super) id: u32,
    pub(super) event: TriggerEventDef,
    pub(super) capture: TriggerCapture,
    pub(super) lifetime: InstalledTriggerLifetime,
}

/// One battlefield trigger listener frozen at the start of an atomic event.
/// A simultaneous zone change can remove the source before another object in
/// the same event is published, so listener discovery cannot consult the
/// incrementally-mutated battlefield.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct BattlefieldTriggerListener {
    pub(super) event: TriggerEventDef,
    pub(super) uses_stack: bool,
    /// "This ability triggers only once each turn", carried from the
    /// printed definition so the capture can count without rediscovering
    /// which ability it came from.
    pub(super) trigger_limit: Option<u8>,
    /// Identifies an effect-installed listener. Battlefield listeners have no
    /// ID because their source's zone presence determines their lifetime.
    pub(super) installed: Option<u32>,
    pub(super) capture: TriggerCapture,
}

#[derive(Clone, Debug)]
pub(super) struct TriggerPlacementBatch {
    pub(super) controller: PlayerId,
    pub(super) triggers: Vec<PendingTrigger>,
}
