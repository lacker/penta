// The committed trigger events a resolution publishes, and the context each
// one hands the abilities that watch it.
//
// Separated from the surrounding resolution state because this answers a
// different question: that says what a resolution is carrying, while this
// says what it announced to the rest of the board. Included textually into
// `trigger_state.rs`, so the paths and imports here are the parent module's.

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum CommittedStackObjectEvent {
    Cast {
        from: CastSourceZone,
    },
    Copied,
    /// One distinct recipient newly selected in an atomic target-selection
    /// batch. Declarative aggregation decides whether matching recipients
    /// trigger independently or collapse to one occurrence.
    TargetSelection {
        target: Target,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum CommittedTriggerEvent {
    CumulativeUpkeepPaid {
        object: TriggerEventObject,
        player: PlayerId,
        age_counters: u16,
        mana_spent: Vec<crate::ManaColor>,
    },
    CumulativeUpkeepNotPaid {
        object: TriggerEventObject,
        player: PlayerId,
        age_counters: u16,
    },
    CoinFlipped {
        player: PlayerId,
        won: bool,
    },
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
    /// A committed occurrence involving an object on the stack. `kind` is
    /// frozen separately because object characteristics do not distinguish a
    /// spell from an activated or triggered ability.
    StackObject {
        object: TriggerEventObject,
        kind: StackObjectKind,
        event: CommittedStackObjectEvent,
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
    /// A land was played, which is a special action rather than a zone
    /// change of its own (CR 305.1). Kept apart from the entry it causes
    /// because the two are not the same event: a land put onto the
    /// battlefield by an effect enters without ever being played, and
    /// "when you play another land" is about the playing.
    LandPlayed {
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
    CountersRemoved {
        object: TriggerEventObject,
        kind: crate::card::CounterKind,
        amount: u16,
        remaining: u16,
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
    /// The controller of a "you may ..." clause accepted it, which is what
    /// the reflexive "when you do" half watches.
    OptionalEffectTaken {
        object: TriggerEventObject,
    },
    /// A permanent was sacrificed, captured before it left so what it was
    /// is still readable.
    Sacrificed {
        object: TriggerEventObject,
        player: PlayerId,
    },
    /// A resolution sacrificed a permanent as part of its own clause, which
    /// is what the compulsory "when you do" watches. The object is the
    /// source of that clause, the way a damage event names the dealer;
    /// `sacrificed` is what it gave up, captured before it left.
    SacrificePerformed {
        object: TriggerEventObject,
        sacrificed: TriggerEventObject,
        /// The sacrificed permanent's power, frozen here so the reflexive
        /// ability reads "that creature's power" off the event rather than
        /// off a permanent that has already moved.
        power: i32,
    },
}

impl CommittedTriggerEvent {
    #[allow(clippy::too_many_lines)]
    pub(super) fn context(&self) -> TriggerContext {
        match self {
            Self::CumulativeUpkeepPaid {
                object,
                player,
                age_counters,
                ..
            }
            | Self::CumulativeUpkeepNotPaid {
                object,
                player,
                age_counters,
                ..
            } => TriggerContext {
                object: Some(object.id),
                zone_change_result: None,
                object_controller: Some(object.controller),
                event_player: Some(*player),
                amount: Some(i32::from(*age_counters)),
                damaged_object: None,
                sacrificed_object: None,
                cast_from_zone: None,
            },
            Self::ZoneChanged { before, after, .. } => {
                let object = before.as_ref().or(after.as_ref());
                TriggerContext {
                    object: object.map(|object| object.id),
                    zone_change_result: after.as_ref().map(|object| object.id),
                    object_controller: object.map(|object| object.controller),
                    event_player: None,
                    amount: None,
                    damaged_object: None,
                    sacrificed_object: None,
                    cast_from_zone: None,
                }
            }
            Self::SacrificePerformed {
                object,
                sacrificed,
                power,
            } => TriggerContext {
                object: Some(object.id),
                zone_change_result: None,
                object_controller: Some(object.controller),
                event_player: None,
                amount: Some(*power),
                damaged_object: None,
                sacrificed_object: Some(sacrificed.id),
                cast_from_zone: None,
            },
            Self::Transformed { object }
            | Self::Cycled { object }
            | Self::Exerted { object }
            | Self::OptionalEffectTaken { object }
            | Self::AttacksAndIsNotBlocked { object } => TriggerContext {
                object: Some(object.id),
                zone_change_result: None,
                object_controller: Some(object.controller),
                event_player: None,
                amount: None,
                damaged_object: None,
                sacrificed_object: None,
                cast_from_zone: None,
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
                    sacrificed_object: None,
                    cast_from_zone: None,
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
                sacrificed_object: None,
                cast_from_zone: None,
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
                sacrificed_object: None,
                cast_from_zone: None,
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
                sacrificed_object: None,
                cast_from_zone: None,
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
                sacrificed_object: None,
                cast_from_zone: None,
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
                sacrificed_object: None,
                cast_from_zone: None,
            },
            Self::AttackersDeclared { attackers } => TriggerContext {
                object: None,
                zone_change_result: None,
                object_controller: attackers.first().map(|attacker| attacker.controller),
                event_player: attackers.first().map(|attacker| attacker.controller),
                amount: Some(i32::try_from(attackers.len()).unwrap_or(i32::MAX)),
                damaged_object: None,
                sacrificed_object: None,
                cast_from_zone: None,
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
                sacrificed_object: None,
                cast_from_zone: None,
            },
            Self::Tapped { object, for_mana } => TriggerContext {
                object: Some(object.id),
                zone_change_result: None,
                object_controller: Some(object.controller),
                event_player: for_mana.then_some(object.controller),
                amount: None,
                damaged_object: None,
                sacrificed_object: None,
                cast_from_zone: None,
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
                sacrificed_object: None,
                cast_from_zone: None,
            },
            Self::BlocksOrBecomesBlocked { other, .. } => TriggerContext {
                object: Some(other.id),
                zone_change_result: None,
                object_controller: Some(other.controller),
                event_player: None,
                amount: None,
                damaged_object: None,
                sacrificed_object: None,
                cast_from_zone: None,
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
                sacrificed_object: None,
                cast_from_zone: None,
            },
            Self::LifeGained { player, amount } => TriggerContext {
                object: None,
                zone_change_result: None,
                object_controller: None,
                event_player: Some(*player),
                amount: Some(i32::from(*amount)),
                damaged_object: None,
                sacrificed_object: None,
                cast_from_zone: None,
            },
            Self::CountersPlaced { object, amount, .. }
            | Self::CountersRemoved { object, amount, .. } => TriggerContext {
                object: Some(object.id),
                zone_change_result: None,
                object_controller: Some(object.controller),
                event_player: None,
                amount: Some(i32::from(*amount)),
                damaged_object: None,
                sacrificed_object: None,
                cast_from_zone: None,
            },
            Self::StackObject {
                object,
                event: CommittedStackObjectEvent::Cast { from },
                ..
            } => TriggerContext {
                object: Some(object.id),
                zone_change_result: None,
                object_controller: Some(object.controller),
                event_player: Some(object.controller),
                amount: None,
                damaged_object: None,
                sacrificed_object: None,
                cast_from_zone: Some(from.zone()),
            },
            Self::StackObject { object, .. } => TriggerContext {
                object: Some(object.id),
                zone_change_result: None,
                object_controller: Some(object.controller),
                event_player: Some(object.controller),
                amount: None,
                damaged_object: None,
                sacrificed_object: None,
                cast_from_zone: None,
            },
            Self::BecameLevel { object, .. } => TriggerContext {
                object: Some(*object),
                zone_change_result: None,
                object_controller: None,
                event_player: None,
                amount: None,
                damaged_object: None,
                sacrificed_object: None,
                cast_from_zone: None,
            },
            Self::Discarded { player, card } => TriggerContext {
                object: card.as_ref().map(|card| card.id),
                zone_change_result: None,
                object_controller: card.as_ref().map(|card| card.controller),
                event_player: Some(*player),
                amount: None,
                damaged_object: None,
                sacrificed_object: None,
                cast_from_zone: None,
            },
            // A drawn card snapshot belongs to trigger matching only. The
            // draw does not reveal it, so these player-only events carry no
            // hidden-zone identity into resolution or a public checkpoint.
            Self::CoinFlipped { player, .. }
            | Self::StepBegins { player, .. }
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
                sacrificed_object: None,
                cast_from_zone: None,
            },
        }
    }
}
