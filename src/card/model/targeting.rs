use crate::ids::TargetIndex;

use super::{
    AbilityPredicateDef, BasicLandType, BattlefieldEntryChoiceDestinationDef, CardSet,
    CardSupertype, CardType, ComparisonDef, CounterKind, KeywordAbility, ManaColor, ObjectRefDef,
    ObjectSetDef, PlayerRelation, TargetPredicate, ValueDef, ZoneKind,
};

/// A composable predicate over a card or game object.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ObjectPredicateDef {
    Any,
    Source,
    /// Whether the object is a token rather than a card represented by a
    /// physical printing. Negate this for the common "nontoken" qualifier.
    Token,
    /// The permanent is currently tapped. Only a battlefield object can be,
    /// so this never matches a card in another zone.
    Tapped,
    /// Damage landed on this permanent at some point this turn. Deliberately
    /// not "has damage marked on it": regeneration and cleanup both wipe the
    /// marks, and the printed clause asks what happened rather than what is
    /// still showing.
    WasDealtDamageThisTurn,
    /// The mirror: this permanent dealt damage to something this turn. The
    /// recipient does not matter, so a creature that burned its own
    /// controller matches as readily as one that connected in combat.
    DealtDamageThisTurn,
    HasType(CardType),
    /// Whether the object's effective name is the canonical name of a basic
    /// land card in the catalog. This is about the name, not the object's
    /// current supertypes: a copied Plains still has a basic land name.
    NameIsBasicLandName,
    /// A land with at least one of the listed effective basic land subtypes.
    ///
    /// This uses the object's prospective/effective type line, so continuous
    /// effects such as Blood Moon and Nylea's Presence participate.
    HasAnyBasicLandType(&'static [BasicLandType]),
    Spell,
    NoncreatureSpell,
    /// An activated or triggered ability currently waiting on the stack.
    Ability,
    /// An activated ability currently waiting on the stack.
    ActivatedAbility,
    /// A triggered ability currently waiting on the stack.
    TriggeredAbility,
    /// A stack object with this inclusive number of declared targets.
    DeclaredTargetCount {
        minimum: u8,
        maximum: u8,
    },
    /// A stack object that currently has at least one declared target of this
    /// broad kind.
    HasDeclaredTarget(TargetPredicate),
    /// A stack object that currently has at least one player target in this
    /// relation to the controller of the ability asking the question.
    HasDeclaredPlayerTarget(PlayerRelation),
    Color(ManaColor),
    /// Has exactly this many colors. Zero matches colorless objects and one
    /// matches monocolored objects.
    ColorCount(u8),
    Subtype(&'static str),
    /// Mana value at most this much, for "with mana value N or less".
    ManaValueAtMost(u8),
    /// Mana value exactly this much, where the number is read off the
    /// ability's own source rather than printed on the card.
    ManaValueEqualTo(ValueDef),
    /// Mana value at most a computed amount, for "with mana value X or less".
    ManaValueAtMostValue(ValueDef),
    /// Printed mana cost made of generic mana alone, totalling at most this
    /// much: the literal reading of "with mana cost {0} or {1}", which is a
    /// smaller set than mana value one or less. Only a zone predicate that
    /// holds the card's own definition can answer it, so the catalog
    /// boundary keeps it out of static and stack contexts.
    GenericManaCostAtMost(u8),
    /// Power at least this much, for "power N or greater". Target legality
    /// reads real current power, so a creature a Crusade has pumped qualifies.
    /// Trigger and static matching still read power without continuous
    /// statics, because that view is what static resolution is handed.
    PowerAtLeast(i16),
    /// Power exactly this much. Like [`Self::PowerAtLeast`] this reads
    /// current power for target legality, so "target 1/1 creature" stops being
    /// one the moment anything pumps it, a Crusade included.
    PowerExactly(i16),
    /// Toughness exactly this much, read the same way.
    ToughnessExactly(i16),
    /// Power and toughness adding up to no more than this, for "total power
    /// and toughness 5 or less". Read live like the two halves are, so a
    /// creature anything has pumped past the sum stops qualifying. A
    /// creature missing either stat -- which no creature on the battlefield
    /// is -- does not qualify.
    TotalPowerAndToughnessAtMost(i16),
    /// Toughness strictly below a value computed from the ability's own
    /// source, for "toughness less than this creature's power".
    ToughnessLessThan(ValueDef),
    /// Power strictly above a computed value, for "greater power than this
    /// creature". The mirror of [`Self::ToughnessLessThan`], read the same
    /// way and against the same source.
    PowerGreaterThan(ValueDef),
    /// Toughness strictly above a computed value.
    ToughnessGreaterThan(ValueDef),
    /// Power strictly below a computed value, for "creatures with power less
    /// than this creature's power". The mirror of [`Self::PowerGreaterThan`].
    PowerLessThan(ValueDef),
    /// The creature's own toughness is greater than its own power. A
    /// comparison of one object with itself rather than with anything the
    /// ability supplies, which is what "each creature you control with
    /// toughness greater than its power" counts.
    ToughnessGreaterThanItsPower,
    /// Carries at least one counter of this kind, for "each creature you
    /// control with a +1/+1 counter on it". Read live, so a creature that
    /// loses its last counter stops matching.
    HasCounter(CounterKind),
    /// Carries one or more counters of any kind. Used when a later
    /// resolution choice names which existing kind to adjust.
    HasAnyCounter,
    /// How many counters of one kind it carries, against a printed number.
    /// The counting form of [`Self::HasCounter`], for the clauses that name
    /// a bound rather than asking whether there is one at all -- a level
    /// band is two of them, an open bottom and an open top.
    CounterCount {
        kind: CounterKind,
        comparison: ComparisonDef,
        amount: u8,
    },
    /// Controlled by a player in this relation to the ability's controller,
    /// for "a creature you control" and "whenever you cast".
    ControlledBy(PlayerRelation),
    /// Who owns the physical card, which is not who controls it. "Returned
    /// to your hand" asks this: a permanent you control but do not own goes
    /// back to somebody else's hand.
    OwnedBy(PlayerRelation),
    /// Carries this supertype. Negate it for "nonbasic".
    Supertype(CardSupertype),
    /// "With a name originally printed in the <set> expansion", which reads
    /// the card's debut set rather than the printing in front of you. Tokens
    /// were printed in no expansion, so they never match.
    DebutSet(CardSet),
    /// Has the same name as the referenced object. The common printed "with
    /// the same name as this" form uses [`ObjectRefDef::Source`].
    HasName(ObjectRefDef),
    /// Shares its effective name with at least one member of the resolved
    /// set. The tested object is not implicitly added to that set.
    SharesNameWithAny(&'static ObjectSetDef),
    /// A spell or ability on the stack whose chosen targets include an object
    /// matching this. "That targets a land you control" reads the targets it
    /// already has rather than what it could have taken.
    TargetsObjectMatching(&'static ObjectPredicateDef),
    /// Bears exactly this name. Printed name matching is rare enough that the
    /// name is written out; "bands with other creatures named X" is the one
    /// place the rules ask for it without a source to compare against.
    Named(&'static str),
    /// Bears the card name chosen earlier in this resolution, by
    /// [`super::EffectDef::ChooseCardName`]. Nothing matches when no name was
    /// chosen, and nothing matches outside a resolution that chose one --
    /// the name lives in the resolution rather than on the board.
    HasChosenName,
    /// Matches the scalar the ability's source chose as it entered: the card
    /// name Meddling Mage locked out, or the creature type Engineered Plague
    /// named. The axis is the same one the entry choice was recorded on, so
    /// the two halves cannot drift apart.
    ///
    /// A source that never made its choice matches nothing rather than
    /// everything, which is the difference between a Plague that shrinks one
    /// tribe and one that shrinks the board.
    HasSourcesChosenScalar(BattlefieldEntryChoiceDestinationDef),
    /// Currently attacking or blocking. Only a battlefield object can be, so
    /// this never matches a card or a spell.
    AttackingOrBlocking,
    /// Has this keyword. Protection is not askable this way, because it is a
    /// keyword per color rather than one keyword.
    ///
    /// This reads keywords a continuous static effect grants or removes, so a
    /// creature wearing a Lord's granted keyword matches. The one place it
    /// does not is a static ability that grants or removes abilities choosing
    /// its own recipients, which is answered from the layer below itself.
    HasKeyword(KeywordAbility),
    /// Has an ability satisfying this selector among those that apply to the
    /// card in its current zone. This asks about the ability itself rather
    /// than whether its costs can be paid now.
    HasAbility(AbilityPredicateDef),
    /// Has at least one ordinary activated ability rather than only mana
    /// abilities. This is the distinction Tsabo's Web asks for.
    HasNonManaActivatedAbility,
    /// A creature currently declared as an attacker in combat.
    Attacking,
    /// A Mount whose saddle ability has been paid this turn (CR 702.166a).
    /// Its own printed clause is what reads it, which is why this is a
    /// characteristic rather than a counter.
    Saddled,
    /// Came under its current controller's control during the turn they are
    /// taking now. This is the same fact summoning sickness reads, asked as a
    /// characteristic: "unless it came under your control this turn".
    CameUnderControlThisTurn,
    /// Entered the battlefield during the turn being taken now. Not the same
    /// question as [`Self::CameUnderControlThisTurn`], which is measured
    /// against its controller's own turn count and so stays true through the
    /// opponent's following turn: a permanent that entered on your turn did
    /// not enter on theirs.
    EnteredThisTurn,
    /// The permanent the ability's source is attached to, for an Aura whose
    /// trigger watches its own host rather than itself.
    AttachedToSource,
    /// A creature currently blocking one. This is the other half of
    /// [`Self::AttackingOrBlocking`], which neither of the single-sided
    /// predicates could express on its own.
    Blocking,
    /// The attacker the ability's source is blocking. Unlike [`Self::Blocking`]
    /// the relationship is read from the source, which is what a Wall printing
    /// "creatures it's blocking" asks about.
    BlockedBySource,
    /// A creature blocking the ability's source. This is the other half of
    /// [`Self::BlockedBySource`]: together in an `AnyOf` they are the printed
    /// "blocking or blocked by this creature".
    BlockingSource,
    /// A creature that is not soulbonded to anything. Soulbond pairs only
    /// unpaired creatures, on both sides.
    Unpaired,
    /// The creature the ability's source is soulbonded to. Like
    /// [`Self::BandedWithSource`] the relation is symmetric, so it reads the
    /// same from either side.
    PairedWithSource,
    /// A creature in the same attacking band as the ability's source, the
    /// source itself excluded. "All creatures banded with it" names this and
    /// nothing else: a lone attacker is in no band, so it matches nothing.
    BandedWithSource,
    /// A permanent with an Aura attached to it. Nothing is said about whose
    /// Aura it is: "enchanted creatures" covers both players' Auras.
    Enchanted,
    /// A permanent attached to something matching. This is the Aura's side of
    /// [`Self::Enchanted`]: it asks what the object is attached *to*, which
    /// is how "target Aura attached to a land" picks one out.
    AttachedTo(&'static ObjectPredicateDef),
    /// A creature that was declared as an attacker at any point this turn,
    /// whether or not it is still attacking or even still in combat.
    AttackedThisTurn,
    /// A creature that was declared as an attacker during its controller's
    /// previous turn. History rather than turn state, and the counterpart of
    /// [`Self::AttackedThisTurn`] one turn back: a creature that has changed
    /// hands since does not answer about its new controller's last turn.
    AttackedDuringControllersLastTurn,
    All(&'static [ObjectPredicateDef]),
    AnyOf(&'static [ObjectPredicateDef]),
    Not(&'static ObjectPredicateDef),
    /// A narrow, named predicate that cannot yet be expressed by the common
    /// vocabulary. The engine owns the meaning of each supported name.
    Special(&'static str),
}

/// The legal subject of one ability target slot.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TargetChooserDef {
    /// The player who controls the spell or ability, which is who chooses
    /// unless something says otherwise.
    Controller,
    /// "That player chooses target ...", where "that player" is whoever the
    /// event named: the player whose upkeep it is, for Oath of Druids. The
    /// ability is still its controller's; only the choice moves.
    EventPlayer,
    /// The other player chooses this target. Penta games are two-player, so
    /// "an opponent's choice" identifies one seat without another selection.
    Opponent,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AbilityTargetPredicate {
    AnyTarget,
    /// Selects the target restriction that applies to this cast according to
    /// whether one optional additional cost was paid. This keeps Kicker and
    /// similar costs compositional while still enforcing modified targeting
    /// instructions during declaration.
    IfAdditionalCostPaid {
        cost: crate::AdditionalCostIndex,
        if_paid: &'static AbilityTargetPredicate,
        otherwise: &'static AbilityTargetPredicate,
    },
    /// Any target matching one complete alternative. Each branch owns its
    /// object predicate, zones, and player relations, so a restriction from
    /// one side of a printed "or" cannot leak into another.
    AnyOf(&'static [AbilityTargetPredicate]),
    /// "Target player who controls more creatures than they do": a player
    /// in this relation to whoever is choosing, who controls more matching
    /// objects than that chooser does. The comparison is per candidate,
    /// which is what makes it a targeting restriction rather than a
    /// condition on the ability.
    PlayerWithMoreObjectsThanChooser {
        relation: PlayerRelation,
        object: ObjectPredicateDef,
        zones: &'static [ZoneKind],
    },
    /// "Target player or planeswalker", which is every damage target except
    /// the creatures, narrowed to players in this relation.
    PlayerOrPlaneswalker(PlayerRelation),
    /// A permanent controlled by whoever controls an earlier slot's target,
    /// for "that player or that planeswalker's controller controls".
    ControlledByTargetOf {
        object: ObjectPredicateDef,
        slot: TargetIndex,
    },
    /// A card in one of these nonbattlefield zones owned by the player named
    /// by an earlier target slot. This is the linked "cards from that player's graveyard"
    /// relation: choosing the player first narrows every later card target to
    /// that player's private zones.
    OwnedByTargetPlayer {
        object: ObjectPredicateDef,
        zones: &'static [ZoneKind],
        slot: TargetIndex,
    },
    Player(PlayerRelation),
    Object {
        object: ObjectPredicateDef,
        zones: &'static [ZoneKind],
        /// Relation of the object's controller, when the zone supplies one.
        controller: Option<PlayerRelation>,
        /// Relation of the physical card's owner. This is the relevant
        /// relation for private zones such as a graveyard.
        owner: Option<PlayerRelation>,
    },
}

/// A const-friendly target declaration kept beside a printed ability.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AbilityTargetDef {
    pub predicate: AbilityTargetPredicate,
    /// Who picks what fills this slot. The ability's controller does it
    /// almost always (CR 601.2c); a few cards hand the choice to somebody
    /// else and say so in their own text.
    pub chooser: TargetChooserDef,
    pub minimum: u8,
    pub maximum: u8,
    /// A cast-time value that fixes both bounds. This is the composable form
    /// for counts such as "one plus the number of times kicker was paid".
    pub exact_count: Option<ValueDef>,
    /// The total this slot divides among its targets, when the card says
    /// "divided as you choose". Every chosen target takes at least one, which
    /// is what makes the number of targets a consequence of the division.
    pub divided_total: Option<DividedTotal>,
    /// "Another": this slot may not name anything an earlier slot already
    /// named. Enforced as targets are chosen rather than checked afterwards,
    /// because a declaration that repeats itself is not legal to begin with.
    pub another: bool,
    /// "Any other target": the ability's own source is not among the choices.
    /// Distinct from [`Self::another`], which is about what a previous slot
    /// took rather than about the source.
    pub excludes_source: bool,
}

impl AbilityTargetDef {
    /// The same slot, filled by whoever the event named rather than by the
    /// ability's controller: "that player chooses target ...".
    #[must_use]
    pub const fn chosen_by_event_player(mut self) -> Self {
        self.chooser = TargetChooserDef::EventPlayer;
        self
    }

    /// The same slot, filled by the ability controller's opponent.
    #[must_use]
    pub const fn chosen_by_opponent(mut self) -> Self {
        self.chooser = TargetChooserDef::Opponent;
        self
    }

    /// The same slot, restricted to something no earlier slot named.
    #[must_use]
    pub const fn another(mut self) -> Self {
        self.another = true;
        self
    }

    /// The same slot, restricted to something other than the ability's own
    /// source. "Any other target" is this over an any-target slot.
    #[must_use]
    pub const fn excluding_source(mut self) -> Self {
        self.excludes_source = true;
        self
    }
}

/// How much a divided slot has to share out.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DividedTotal {
    Fixed(u8),
    /// The X chosen as the spell was cast, so the number of targets is not
    /// known until then either.
    ChosenX,
}

impl AbilityTargetDef {
    /// A maximum standing for "as many as there are". Spelled as a sentinel
    /// rather than an `Option` so that every existing bounded declaration
    /// keeps its plain number.
    pub const UNLIMITED: u8 = u8::MAX;

    /// "Any number of targets," including none.
    #[must_use]
    pub const fn any_number(predicate: AbilityTargetPredicate) -> Self {
        Self {
            predicate,
            chooser: TargetChooserDef::Controller,
            minimum: 0,
            maximum: Self::UNLIMITED,
            exact_count: None,
            divided_total: None,
            another: false,
            excludes_source: false,
        }
    }

    /// A count standing for "the X chosen as this was activated". Like
    /// [`Self::UNLIMITED`] it is a sentinel rather than an `Option`, for the
    /// same reason: no printed card names 254 targets, and every bounded
    /// declaration keeps its plain number.
    pub const CHOSEN_X: u8 = u8::MAX - 1;

    /// "Exactly X target ...", where X is chosen as the ability is
    /// activated. Both bounds are the same, because the count is the X that
    /// was paid rather than a range the controller picks from afterwards.
    #[must_use]
    pub const fn exactly_chosen_x(predicate: AbilityTargetPredicate) -> Self {
        Self {
            predicate,
            chooser: TargetChooserDef::Controller,
            minimum: Self::CHOSEN_X,
            maximum: Self::CHOSEN_X,
            exact_count: None,
            divided_total: None,
            another: false,
            excludes_source: false,
        }
    }

    /// "Up to X target ...", where X is chosen as the ability is activated.
    /// The lower bound is none: an X of three may still name one, and a
    /// board with nothing to name is no obstacle to casting it.
    #[must_use]
    pub const fn up_to_chosen_x(predicate: AbilityTargetPredicate) -> Self {
        Self {
            predicate,
            chooser: TargetChooserDef::Controller,
            minimum: 0,
            maximum: Self::CHOSEN_X,
            exact_count: None,
            divided_total: None,
            another: false,
            excludes_source: false,
        }
    }

    /// This slot's count with any sentinel resolved against the X that
    /// was actually chosen. Enumerating past the candidate list produces no
    /// combinations, so an X larger than the board simply offers nothing --
    /// which is the same as saying the activation is not legal for that X.
    #[must_use]
    pub fn count_bounds(self, x: u16, additional_cost_payments: &[u16]) -> (u8, u8) {
        if let Some(value) = self.exact_count {
            let Some(count) = cast_target_count_value(value, x, additional_cost_payments) else {
                debug_assert!(
                    false,
                    "catalog validation rejects unsupported target counts"
                );
                return (0, 0);
            };
            let count = u8::try_from(count.clamp(0, i32::from(u8::MAX))).unwrap_or_default();
            return (count, count);
        }
        // An X past 255 cannot be a target count on any real board, so it
        // saturates rather than wrapping.
        let chosen = if x > u16::from(u8::MAX) {
            u8::MAX
        } else {
            #[allow(clippy::cast_possible_truncation, reason = "guarded above")]
            {
                x as u8
            }
        };
        let resolve = |bound| {
            if bound == Self::CHOSEN_X {
                chosen
            } else {
                bound
            }
        };
        let minimum = resolve(self.minimum);
        let maximum = resolve(self.maximum);
        (minimum, maximum)
    }

    /// Exactly the number of targets computed from cast-time values.
    #[must_use]
    pub const fn exactly_value(predicate: AbilityTargetPredicate, count: ValueDef) -> Self {
        Self {
            predicate,
            chooser: TargetChooserDef::Controller,
            minimum: 0,
            // The exact runtime value overrides these bounds. Keep the
            // static projection conservative so validators never mistake a
            // dynamic count for a singular target slot.
            maximum: Self::UNLIMITED,
            exact_count: Some(count),
            divided_total: None,
            another: false,
            excludes_source: false,
        }
    }

    #[must_use]
    pub const fn exactly_one(predicate: AbilityTargetPredicate) -> Self {
        Self {
            predicate,
            chooser: TargetChooserDef::Controller,
            minimum: 1,
            maximum: 1,
            exact_count: None,
            divided_total: None,
            another: false,
            excludes_source: false,
        }
    }

    /// One or more targets with no printed limit.
    ///
    /// [`Self::UNLIMITED`] is not a cap of 255: the declaration enumerator
    /// clamps the count to how many legal targets there actually are, so the
    /// ceiling is the board rather than the number.
    #[must_use]
    pub const fn one_or_more(predicate: AbilityTargetPredicate) -> Self {
        Self {
            predicate,
            chooser: TargetChooserDef::Controller,
            minimum: 1,
            maximum: Self::UNLIMITED,
            exact_count: None,
            divided_total: None,
            another: false,
            excludes_source: false,
        }
    }

    /// Any number of targets up to a limit, for "up to three target ...".
    /// Choosing none is a legal choice.
    #[must_use]
    pub const fn up_to(predicate: AbilityTargetPredicate, maximum: u8) -> Self {
        Self {
            predicate,
            chooser: TargetChooserDef::Controller,
            minimum: 0,
            maximum,
            exact_count: None,
            divided_total: None,
            another: false,
            excludes_source: false,
        }
    }

    /// One permanent target, optionally narrowed by color, type, or another
    /// object predicate.
    #[must_use]
    pub const fn exactly_one_permanent(object: ObjectPredicateDef) -> Self {
        Self::exactly_one(AbilityTargetPredicate::Object {
            object,
            zones: &[ZoneKind::Battlefield],
            controller: None,
            owner: None,
        })
    }
}

fn cast_target_count_value(value: ValueDef, x: u16, payments: &[u16]) -> Option<i32> {
    match value {
        ValueDef::Constant(value) => Some(value),
        ValueDef::ChosenX => Some(i32::from(x)),
        ValueDef::AdditionalCostPayments(index) => Some(i32::from(
            payments.get(index.index()).copied().unwrap_or_default(),
        )),
        ValueDef::IfAdditionalCostPaid(conditional) => {
            let paid = conditional.cost.index();
            let paid = payments.get(paid).copied().unwrap_or_default();
            let selected = if paid > 0 {
                conditional.if_paid
            } else {
                conditional.otherwise
            };
            cast_target_count_value(selected, x, payments)
        }
        ValueDef::Negate(value) => cast_target_count_value(*value, x, payments)?.checked_neg(),
        ValueDef::Scaled(scaled) => {
            cast_target_count_value(scaled.value, x, payments)?.checked_mul(scaled.factor)
        }
        ValueDef::Sum(sum) => cast_target_count_value(sum.left, x, payments)?
            .checked_add(cast_target_count_value(sum.right, x, payments)?),
        ValueDef::Halved(halved) => {
            Some(halved.apply(cast_target_count_value(halved.value, x, payments)?))
        }
        ValueDef::Quotient(quotient) => Some(quotient.apply(
            cast_target_count_value(quotient.numerator, x, payments)?,
            cast_target_count_value(quotient.denominator, x, payments)?,
        )),
        _ => None,
    }
}
