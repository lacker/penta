//! The composite shapes an effect carries: a bounded choice among objects,
//! a partition into piles, a triggered ability installed by a resolution,
//! what a token clause puts on the tokens it makes, and how a discard picks
//! and counts. Each is a small vocabulary of its own that several effects
//! reach for, rather than a variant of any one of them.

use super::super::{
    AbilityDef, ArrivalAttachmentDef, BattlefieldEntryModificationDef, CardTypeSet,
    ChoiceVisibilityDef, ColorSet, CounterKind, CreatureTypeSetDef, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, ObjectRefDef, ObjectSetDef, ObjectSetPredicateDef, PlayerRefDef,
    PlayerRelation, ResolvedEffectDurationDef, TargetPredicate, ValueDef, ZoneKind,
};
use crate::ids::Binding;

/// The context slot populated by an object choice.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ObjectChoiceBindingDef {
    Object(Binding),
    Objects(Binding),
    /// Preserve the chooser's submitted order for a continuation that acts on
    /// the chosen objects one at a time.
    OrderedObjects(Binding),
}

/// Choose a bounded number of non-targeted objects, save them in the resolving
/// context, then continue the effect.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ChooseDef {
    pub binding: ObjectChoiceBindingDef,
    /// Where the candidates that were *not* chosen are saved, when the
    /// printed clause goes on to say what happens to them. "Put that card
    /// into your hand and the rest into your graveyard" names both halves of
    /// one partition, so both have to be nameable.
    pub unchosen: Option<Binding>,
    pub chooser: PlayerRefDef,
    pub candidates: ObjectSetDef,
    pub exclude: Option<ObjectRefDef>,
    pub minimum: usize,
    pub maximum: usize,
    pub visibility: ChoiceVisibilityDef,
    pub then: &'static EffectDef,
}

/// Choose exactly a computed number of non-targeted objects, save them as a
/// group, then continue the effect.
///
/// The runtime lowers this to the ordinary bounded object-choice procedure
/// after evaluating `amount` once. A short candidate set contributes every
/// available object, matching Magic's instruction to do as much as possible.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ChooseExactDef {
    pub binding: Binding,
    pub chooser: PlayerRefDef,
    pub candidates: ObjectSetDef,
    pub exclude: Option<ObjectRefDef>,
    pub amount: ValueDef,
    pub visibility: ChoiceVisibilityDef,
    pub then: &'static EffectDef,
}

/// How each affected player partitions their matching objects while every
/// player's answer remains pending.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PerPlayerSelectionDef {
    /// Choose one distinct matching permanent for every predicate when one
    /// exists. A permanent already chosen for an earlier predicate is not
    /// offered again.
    OneOfEach(&'static [ObjectPredicateDef]),
    /// Choose exactly the computed number of matching objects. A short
    /// candidate set contributes every available object.
    Count(ValueDef),
}

/// Each affected player partitions matching objects they control on the
/// battlefield, or own in the named private zone. Choices are locked in APNAP
/// order before the chosen and unchosen unions are bound and the nested effect
/// continues.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ChooseForEachPlayerDef {
    pub player: EffectRecipientDef,
    /// The complete universe divided into `chosen` and `unchosen`.
    pub candidates: ObjectPredicateDef,
    pub zone: ZoneKind,
    pub selection: PerPlayerSelectionDef,
    pub visibility: ChoiceVisibilityDef,
    pub chosen: Binding,
    pub unchosen: Binding,
    pub then: &'static EffectDef,
}

/// What a named colour is used for once it has been chosen.
///
/// An operation rather than a general effect: the colour has to reach a
/// characteristic leaf, and only the leaves that take one are meaningful
/// here.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ColorChoiceOperationDef {
    /// Gain protection from the chosen colour.
    ProtectionFromChosenColor,
    /// The same, with colourless among the choices. "Protection from
    /// colorless or from the color of your choice" is one choice of six
    /// rather than a colour choice with a rider: colourless is a quality a
    /// source has by having no colour, and the player picks it or a colour.
    ProtectionFromChosenColorOrColorless,
    /// Become the chosen colour, replacing whatever colours it had.
    BecomesChosenColor,
}

/// How long an effect-created triggered ability listens from outside every
/// zone.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum InstalledTriggerLifetimeDef {
    Once,
    UntilNextTurn(PlayerRefDef),
    /// "This turn": the listener stops when the turn it was installed on
    /// ends, whoever the next turn belongs to.
    ThisTurn,
}

/// A triggered ability installed by a resolving effect.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct InstalledTriggerDef {
    pub ability: &'static AbilityDef,
    pub lifetime: InstalledTriggerLifetimeDef,
}

/// A lexical object-set name for the value produced by one wrapped effect.
///
/// A resolving effect that remains outside every zone and offers one
/// activated ability for a fixed duration.
///
/// When present, the affected recipient is frozen into `binding` as the
/// ongoing effect is created, so the nested ability can read it without
/// targeting it again. An unbound effect instead carries a self-contained
/// ability such as Channel's mana ability. The ongoing effect is a game object
/// for ability-source identity, but it is not a permanent and cannot pay costs
/// that require permanent state. Penta treats it as command-zone-resident for
/// source-zone checks. The rules effect does not technically occupy a zone,
/// but that approximation is gameplay-indistinguishable while the object
/// remains untargetable and separate from emblems.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OngoingEffectDef {
    pub affected: Option<EffectRecipientDef>,
    pub binding: Option<Binding>,
    pub ability: &'static AbilityDef,
    pub duration: ResolvedEffectDurationDef,
}

impl OngoingEffectDef {
    #[must_use]
    pub const fn new(
        affected: EffectRecipientDef,
        binding: Binding,
        ability: &'static AbilityDef,
        duration: ResolvedEffectDurationDef,
    ) -> Self {
        Self {
            affected: Some(affected),
            binding: Some(binding),
            ability,
            duration,
        }
    }

    /// Creates an ongoing effect whose ability does not refer back to an
    /// affected object. Channel is the representative shape.
    #[must_use]
    pub const fn unbound(
        ability: &'static AbilityDef,
        duration: ResolvedEffectDurationDef,
    ) -> Self {
        Self {
            affected: None,
            binding: None,
            ability,
            duration,
        }
    }
}

impl InstalledTriggerDef {
    #[must_use]
    pub const fn once(ability: &'static AbilityDef) -> Self {
        Self {
            ability,
            lifetime: InstalledTriggerLifetimeDef::Once,
        }
    }

    /// "... this turn", which stops listening when the turn it was made on
    /// ends rather than when any player's next one begins.
    #[must_use]
    pub const fn this_turn(ability: &'static AbilityDef) -> Self {
        Self {
            ability,
            lifetime: InstalledTriggerLifetimeDef::ThisTurn,
        }
    }

    #[must_use]
    pub const fn until_next_turn(ability: &'static AbilityDef, player: PlayerRefDef) -> Self {
        Self {
            ability,
            lifetime: InstalledTriggerLifetimeDef::UntilNextTurn(player),
        }
    }
}

/// Counters a token is created with.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TokenCountersDef {
    pub kind: CounterKind,
    pub amount: ValueDef,
}

/// Everything an authored effect adds to a permanent's battlefield entry.
/// The wrapped effect still owns the zone move; this value only describes
/// the prospective permanent that move creates.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BattlefieldArrivalDef {
    /// `None` uses the card's owner.
    pub controller: Option<PlayerRelation>,
    pub modifications: &'static [BattlefieldEntryModificationDef],
    pub attachment: Option<ArrivalAttachmentDef>,
    pub counters: Option<TokenCountersDef>,
}

impl BattlefieldArrivalDef {
    pub const DEFAULT: Self = Self {
        controller: None,
        modifications: &[],
        attachment: None,
        counters: None,
    };
}

/// What happens next to the tokens a clause just created.
///
/// This lexical binding is scoped to its continuation. Producer-to-sibling
/// data flow uses a durable labeled [`EffectDef::BindOutput`] step instead.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CreatedTokensDef {
    pub binding: Binding,
    pub then: &'static EffectDef,
}

/// Copiable values used as the base of a token creation instruction.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TokenCopyDef {
    /// The source is static catalog data like the effect itself; keeping the
    /// recipient behind a reference prevents optional copy initialization
    /// from enlarging every ordinary authored-token instruction.
    pub object: &'static EffectRecipientDef,
    pub exceptions: CopyExceptionsDef,
}

/// One stack-copy operation. Kept behind a static reference in [`EffectDef`]
/// so the rich recipient and copy-process options do not enlarge every effect
/// value in the catalog and runtime.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CopyStackObjectDef {
    pub object: EffectRecipientDef,
    pub controller: PlayerRefDef,
    pub count: ValueDef,
    /// Whether each copy's controller may choose new targets.
    pub retarget: bool,
    /// A copy-process color override. Fork is the canonical case.
    pub colors: Option<ColorSet>,
}

/// One target-changing operation over a spell or ability already on the stack.
///
/// Kept behind a static reference in [`EffectDef`] for the same reason as
/// [`CopyStackObjectDef`]: the ordinary effect stays small while the uncommon
/// stack procedure carries its complete semantics beside the card that uses it.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ChangeStackTargetsDef {
    /// The spell or ability whose locked targets may change.
    pub object: EffectRecipientDef,
    /// Who makes any choice the change requires.
    pub chooser: PlayerRefDef,
    pub change: StackTargetChangeDef,
}

/// How a target-changing effect edits the locked target selections.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StackTargetChangeDef {
    /// Choose a legal replacement configuration for any number of the
    /// object's targets. `optional` is the printed "may": when false, the
    /// unchanged configuration is not offered if any change is possible.
    ChooseNew {
        optional: bool,
        /// An extra restriction on each target that actually changes. The
        /// original stack object's own slot restriction is always enforced
        /// first. Rebound uses this to require the new target to be a player.
        restriction: Option<TargetPredicate>,
    },
    /// Change exactly one target occurrence to the named object if that
    /// object is legal for the original slot. Spellskite is this shape.
    ReplaceOneWith(EffectRecipientDef),
}

/// What follows a destruction, with the permanents actually put into graveyards saved for it.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DestroyFollowUpDef {
    /// The destroyed permanents, under the identities they have after moving.
    pub binding: Binding,
    pub effect: &'static EffectDef,
}

/// What follows a discard, and what it counts among the cards that went.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DiscardFollowUpDef {
    /// Which discarded cards the follow-up counts, read back with
    /// [`ValueDef::MatchedCount`].
    pub counted: ObjectPredicateDef,
    /// Where the counted cards are saved, so the follow-up can name them
    /// rather than only count them. "You may cast the discarded card" needs
    /// the card itself, and by the time the follow-up runs it is one card in
    /// a graveyard among however many were already there.
    pub bound: Option<Binding>,
    pub effect: &'static EffectDef,
}

/// Revealing from the top of a library until a matching card turns up.
///
/// What was passed over goes to the graveyard; the match itself goes where
/// `matched_zone` says, which is the graveyard for a plain dig and the hand
/// for Hermit Druid.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MillUntilDef {
    pub player: EffectRecipientDef,
    /// Stop once the cards revealed so far satisfy this predicate. The final
    /// card is the one sent to `matched_zone` when that zone is not the
    /// graveyard; every earlier card is milled.
    pub until: ObjectSetPredicateDef,
    pub matched_zone: ZoneKind,
}

/// How cards are selected for a discard effect.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DiscardSelectionDef {
    /// Each affected player chooses cards from their own hand.
    RecipientChooses,
    /// The engine selects cards using the recorded random seed.
    Random,
    /// The same, but only from the cards in hand that match. "Discards a
    /// creature card at random" leaves everything else where it is, and
    /// discards nothing when the hand holds none.
    RandomMatching(&'static ObjectPredicateDef),
}

/// One pile of cards exiled out of several zones at once.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PileExileDef {
    pub player: EffectRecipientDef,
    pub zones: &'static [ZonePickDef],
    /// What the effect's controller may do with the pile: cast one spell
    /// from among its cards on the terms named here. It is one permission
    /// over the whole pile rather than one per card, so casting any of them
    /// spends it. `None` is a pure exile.
    pub permission: Option<ExiledCastPermissionDef>,
}

/// One repeating mill: what runs before each mill, what the milled card has
/// to be for the process to go on, what runs when it is, and the hard stop.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MillLoopDef {
    pub player: EffectRecipientDef,
    pub body: &'static EffectDef,
    pub object: ObjectPredicateDef,
    pub on_match: &'static EffectDef,
    pub limit: u16,
}

/// How one card is taken from a zone by an effect that exiles one card
/// from each of several zones. A library has an order to read from; a hand
/// and a graveyard do not, so a card is picked from them at random.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ZonePickModeDef {
    Top,
    AtRandom,
}

/// One zone an exile draws from, and how it draws.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ZonePickDef {
    pub zone: ZoneKind,
    pub pick: ZonePickModeDef,
}

impl ZonePickDef {
    #[must_use]
    pub const fn top(zone: ZoneKind) -> Self {
        Self {
            zone,
            pick: ZonePickModeDef::Top,
        }
    }

    #[must_use]
    pub const fn at_random(zone: ZoneKind) -> Self {
        Self {
            zone,
            pick: ZonePickModeDef::AtRandom,
        }
    }
}

/// What a permission to play an exiled card asks for where it is used.
///
/// A closed vocabulary rather than a general condition: the permission
/// outlives the resolution that granted it, so what it asks has to be
/// something a checkpoint can write down and read back.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ExilePlayConditionDef {
    /// "During any turn you attacked with a Rogue." Read off the creatures
    /// that attacked this turn, so a Rogue that attacked and then died is
    /// not among them.
    AttackedWithSubtypeThisTurn(&'static str),
}

/// "You may play those cards without paying their mana costs."
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FreePlayDef {
    /// Which cards the permission covers. A set rather than a zone, because
    /// what a clause hands over is a pile it already knows about: the cards
    /// a search bound, or the ones exiled with a source.
    pub objects: ObjectSetDef,
    pub duration: FreePlayDurationDef,
    /// Whether the card must be played if doing so is legal. Most authored
    /// permissions say "may"; Suspend's last-counter instruction does not.
    pub mandatory: bool,
    /// A creature cast through this permission gains haste until its
    /// controller loses control of it, as suspend requires.
    pub grants_haste: bool,
}

/// How long a "without paying its mana cost" permission lasts.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FreePlayDurationDef {
    /// "You may cast it": the offer stands only while the effect that made
    /// it is resolving, and a card left unplayed is not playable afterwards.
    /// This is what a bare "you may play" means.
    WhileResolving,
    /// "Until end of turn, you may cast those cards without paying their
    /// mana costs": a permission that outlives the resolution that granted
    /// it, and says so.
    UntilEndOfTurn,
}

/// How long a permission to play an exiled card lasts.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ExilePlayDurationDef {
    /// "This turn", which is the turn the card was exiled on and no longer.
    ThisTurn,
    /// "Until your next end step", which reaches into the holder's own turn
    /// when the card was exiled on somebody else's.
    UntilYourNextEndStep,
    /// For as long as the card is in exile. What bounds it is not a turn but
    /// whatever the clause granting it says: Robber of the Rich hands one
    /// out that only works on the turns you attacked with a Rogue.
    WhileExiled,
}

/// What a copy is created or becomes "except" for.
///
/// Thespian's Stage adds an ability, Quicksilver Gargantuan replaces power
/// and toughness, and embalm and eternalize name several exceptions at once.
/// They are the same kind of thing: copy-process exceptions, which CR 707.9a
/// makes copiable values in their own right. A later copy copies them along
/// with everything else.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CopyAbilityDef {
    /// The resolving activated or triggered ability whose effect is currently
    /// making the copy. This is a reference rather than a recursive
    /// definition, so "except it has this ability" does not build an infinite
    /// declarative tree. Entry replacements use [`Self::Ability`] because
    /// they do not resolve on the stack.
    This,
    /// A separately authored ability the copy gains.
    Ability(&'static AbilityDef),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CopyExceptionsDef {
    /// "Except it's a 1/1", or a 4/4.
    pub base_power_toughness: Option<(i16, i16)>,
    /// "Except it's black": the colours it has instead of the ones it
    /// copied, rather than in addition to them.
    pub colors: Option<ColorSet>,
    /// "Except it's a Zombie <its own types>": creature types on top of the
    /// ones it copied.
    pub added_creature_types: CreatureTypeSetDef,
    /// "Except it's an artifact in addition to its other types": card types
    /// the copy has on top of the ones it copied, rather than instead of
    /// them. Like every other exception it is a copiable value, so a copy of
    /// the copy is an artifact too.
    pub added_types: CardTypeSet,
    /// "With no mana cost", which is what makes an eternalized card's mana
    /// value zero.
    pub no_mana_cost: bool,
    /// Abilities the copy has in addition to the copied ones. References let
    /// an ability add itself -- the exact shape of Thespian's Stage's "except
    /// it has this ability" -- as well as name arbitrary nested abilities.
    pub added_abilities: &'static [CopyAbilityDef],
}

impl CopyExceptionsDef {
    /// A plain copy, with nothing said after "except".
    pub const NONE: Self = Self {
        base_power_toughness: None,
        colors: None,
        added_creature_types: CreatureTypeSetDef::named(&[]),
        added_types: CardTypeSet::empty(),
        no_mana_cost: false,
        added_abilities: &[],
    };

    /// "Except it's an artifact in addition to its other types."
    #[must_use]
    pub const fn with_added_types(mut self, added_types: CardTypeSet) -> Self {
        self.added_types = added_types;
        self
    }

    /// "Except it's blue", and its relatives.
    #[must_use]
    pub const fn with_colors(mut self, colors: ColorSet) -> Self {
        self.colors = Some(colors);
        self
    }

    /// "Except it's a Spirit in addition to its other types."
    #[must_use]
    pub const fn with_added_creature_types(
        mut self,
        added_creature_types: &'static [&'static str],
    ) -> Self {
        self.added_creature_types = CreatureTypeSetDef::named(added_creature_types);
        self
    }

    /// "Except it has haste", "except it has this ability", and their
    /// multi-ability relatives.
    #[must_use]
    pub const fn with_abilities(mut self, added_abilities: &'static [CopyAbilityDef]) -> Self {
        self.added_abilities = added_abilities;
        self
    }

    #[must_use]
    pub const fn power_toughness(power: i16, toughness: i16) -> Self {
        Self {
            base_power_toughness: Some((power, toughness)),
            ..Self::NONE
        }
    }

    /// The list embalm and eternalize print: a body, a colour, a type on
    /// top of the ones it had, and no mana cost.
    #[must_use]
    pub const fn undead(
        power: i16,
        toughness: i16,
        colors: ColorSet,
        added_creature_types: &'static [&'static str],
    ) -> Self {
        Self {
            base_power_toughness: Some((power, toughness)),
            colors: Some(colors),
            added_creature_types: CreatureTypeSetDef::named(added_creature_types),
            added_types: CardTypeSet::empty(),
            no_mana_cost: true,
            added_abilities: &[],
        }
    }
}

/// What a card exiled off the top of a library may be cast for.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ExiledCastPermissionDef {
    /// "You may cast that card by paying an amount of {E} equal to its mana
    /// value rather than paying its mana cost." No duration is printed,
    /// which means the casting happens as the ability resolves: Amped
    /// Raptor's ruling says you cannot wait to cast it later in the turn.
    EnergyEqualToManaValue,
    /// "You may cast any number of spells from among the nonland cards
    /// exiled this way without paying their mana costs." No duration is
    /// printed, which means the casting happens as the ability resolves:
    /// what is not cast then stays in exile uncastable, so the permission
    /// is offered rather than left standing for the turn.
    FreeWhileResolving,
}
