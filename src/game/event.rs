use crate::{CardDefinitionId, GameObjectId, PlayerId, Target};

use super::ObjectCharacteristics;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Step {
    Upkeep,
    Draw,
    PrecombatMain,
    BeginningOfCombat,
    DeclareAttackers,
    DeclareBlockers,
    CombatDamage,
    EndOfCombat,
    PostcombatMain,
    End,
    Cleanup,
}

impl Step {
    pub(super) const fn is_main(self) -> bool {
        matches!(self, Self::PrecombatMain | Self::PostcombatMain)
    }

    /// Whether this step is part of the combat phase. Every step from the
    /// beginning of combat through the end of combat, on either player's
    /// turn -- which is what an unqualified "during combat" asks for.
    pub(super) const fn is_combat(self) -> bool {
        matches!(
            self,
            Self::BeginningOfCombat
                | Self::DeclareAttackers
                | Self::DeclareBlockers
                | Self::CombatDamage
                | Self::EndOfCombat
        )
    }

    pub(super) const fn ends_phase(self) -> bool {
        matches!(
            self,
            Self::Draw
                | Self::PrecombatMain
                | Self::EndOfCombat
                | Self::PostcombatMain
                | Self::Cleanup
        )
    }
}

/// Where ordinary turn progression resumes after an inserted phase sequence.
/// The next-turn sentinel keeps a sequence created during the ending phase in
/// the same turn until every inserted phase has happened.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum TurnPhaseResume {
    Step(Step),
    NextTurn,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameResult {
    Winner { winner: PlayerId, reason: WinReason },
    Draw,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WinReason {
    OpponentConceded,
    OpponentLostAllLife,
    OpponentTriedToDrawFromEmptyLibrary,
    /// An effect said the opponent loses the game, with no life total or
    /// empty library involved.
    OpponentLostToAnEffect,
    /// An effect said you win the game outright. The mirror of
    /// [`Self::OpponentLostToAnEffect`]: nothing happened to the loser, the
    /// winner's card simply said so.
    WonByAnEffect,
    /// The opponent did not act inside the time their host allowed. Distinct
    /// from conceding: nobody chose this, it was imposed by a clock.
    OpponentRanOutOfTime,
    /// The opponent had ten or more poison counters. This is a state-based
    /// action rather than an effect, so it is not
    /// [`Self::OpponentLostToAnEffect`].
    OpponentPoisoned,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StackObjectKind {
    Spell,
    ActivatedAbility,
    TriggeredAbility,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GameEvent {
    GameStarted {
        seed: u64,
    },
    CardDrawn {
        player: PlayerId,
        card: GameObjectId,
    },
    CardRevealed {
        player: PlayerId,
        card: GameObjectId,
        definition: CardDefinitionId,
    },
    CardsDiscarded {
        player: PlayerId,
        cards: Vec<(GameObjectId, CardDefinitionId)>,
    },
    LandPlayed {
        player: PlayerId,
        card: GameObjectId,
        /// Immutable identity for logs after the permanent leaves play.
        definition: CardDefinitionId,
    },
    ManaAdded {
        player: PlayerId,
        source: GameObjectId,
    },
    SpellCast {
        player: PlayerId,
        /// The spell's game object on the stack.
        card: GameObjectId,
        /// Immutable identity for logs after the spell leaves the stack.
        definition: CardDefinitionId,
        targets: Vec<Target>,
    },
    SpellResolved {
        /// The spell's former game object on the stack.
        card: GameObjectId,
        definition: CardDefinitionId,
    },
    /// A targeted spell resolved with every target gone, so it did nothing.
    SpellFizzled {
        /// The spell's former game object on the stack.
        card: GameObjectId,
        definition: CardDefinitionId,
    },
    AbilityActivated {
        player: PlayerId,
        /// The activated ability's own game object on the stack.
        object: GameObjectId,
        /// The permanent object that created the ability.
        source: GameObjectId,
        /// Frozen source presentation for logs after the source leaves play.
        presentation: ObjectCharacteristics,
        chosen_permanents: Vec<GameObjectId>,
    },
    AbilityResolved {
        /// The activated ability's former game object on the stack.
        object: GameObjectId,
        /// The permanent object that created the ability.
        source: GameObjectId,
        presentation: ObjectCharacteristics,
    },
    /// An activated ability resolved with every target illegal, so it did
    /// nothing even though its activation costs remain paid.
    AbilityFizzled {
        object: GameObjectId,
        source: GameObjectId,
        presentation: ObjectCharacteristics,
    },
    AbilityTriggered {
        player: PlayerId,
        trigger: u32,
        source: GameObjectId,
        presentation: ObjectCharacteristics,
    },
    TriggeredAbilityPutOnStack {
        player: PlayerId,
        trigger: u32,
        object: GameObjectId,
        source: GameObjectId,
        presentation: ObjectCharacteristics,
    },
    TriggeredAbilityResolved {
        object: GameObjectId,
        source: GameObjectId,
        presentation: ObjectCharacteristics,
    },
    /// A triggered ability resolved with every target illegal.
    TriggeredAbilityFizzled {
        object: GameObjectId,
        source: GameObjectId,
        presentation: ObjectCharacteristics,
    },
    AttackDeclared {
        player: PlayerId,
        attackers: Vec<GameObjectId>,
    },
    BlockDeclared {
        player: PlayerId,
        assignments: Vec<(GameObjectId, GameObjectId)>,
    },
    DamageDealt {
        player: PlayerId,
        amount: u16,
    },
    /// Life paid or drained rather than dealt. Kept apart from damage because
    /// nothing that triggers on damage may see it, and prevention never
    /// applies.
    LifeLost {
        player: PlayerId,
        amount: u16,
    },
    ManaBurn {
        player: PlayerId,
        amount: u16,
    },
    StepChanged {
        turn: u32,
        active_player: PlayerId,
        step: Step,
    },
    /// A permanent left the battlefield. Emitted from the three functions that
    /// can remove one, so nothing leaves play without the log seeing it. The
    /// characteristics travel with the event because a token is by then gone
    /// and a card may be in a zone the observing player cannot read.
    PermanentLeftBattlefield {
        controller: PlayerId,
        card: GameObjectId,
        characteristics: ObjectCharacteristics,
        destination: BattlefieldExit,
    },
    GameEnded {
        result: GameResult,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BattlefieldExit {
    Graveyard,
    Exile,
    Hand,
    /// On top of its owner's library.
    LibraryTop,
    /// On the bottom of its owner's library.
    LibraryBottom,
}
