// The block-keyword mechanics: the ones printed as a single word that
// expand into a whole trigger, and the state a couple of them carry.
//
// Grouped together because they share a shape rather than a subject --
// each is one keyword the card prints and one clause the engine runs.

/// What a keyword counter hands the permanent it sits on (CR 122.1b). No
/// card wrote the clause down, so the counter says which keyword and this
/// says how that keyword is written.
#[must_use]
pub const fn keyword_counter_ability(kind: CounterKind) -> Option<AbilityDef> {
    match kind.granted_keyword() {
        Some(KeywordAbility::Deathtouch) => Some(deathtouch()),
        Some(KeywordAbility::DoubleStrike) => Some(double_strike()),
        Some(KeywordAbility::FirstStrike) => Some(first_strike()),
        Some(KeywordAbility::Flying) => Some(flying()),
        Some(KeywordAbility::Haste) => Some(haste()),
        Some(KeywordAbility::Hexproof) => Some(hexproof()),
        Some(KeywordAbility::Indestructible) => Some(indestructible()),
        Some(KeywordAbility::Lifelink) => Some(lifelink()),
        Some(KeywordAbility::Menace) => Some(menace()),
        Some(KeywordAbility::Reach) => Some(reach()),
        Some(KeywordAbility::Trample) => Some(trample()),
        Some(KeywordAbility::Vigilance) => Some(vigilance()),
        _ => None,
    }
}

/// Exalted. It is written as a keyword but defined as a triggered ability, so
/// each printed instance is its own clause and several on one board each
/// trigger -- which is why this returns an ordinary trigger rather than a
/// keyword. The permanent carrying it need not be a creature.
#[must_use]
pub const fn exalted() -> AbilityDef {
    AbilityDef::triggered(
        "Exalted (Whenever a creature you control attacks alone, that creature gets +1/+1 until \
         end of turn.)",
        TriggerEventDef::attacks_in_declaration(
            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            1,
            Some(1),
        ),
        EffectDef::Apply {
            recipient: EffectRecipientDef::TriggeringObject,
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )
}

/// Bushido N. Each instance is its own trigger, and the same event shape
/// covers both blocking and becoming blocked while naming the other creature
/// as the triggering object. Printed numeric values own their text; dynamic
/// values use "Bushido" as a header for callers to replace with
/// [`AbilityDef::override_text`].
///
/// # Panics
///
/// Panics for a constant other than a printed bushido value (1, 2, or 5).
#[must_use]
pub const fn bushido(amount: ValueDef) -> AbilityDef {
    let text = match amount {
        ValueDef::Constant(1) => "Bushido 1",
        ValueDef::Constant(2) => "Bushido 2",
        ValueDef::Constant(5) => "Bushido 5",
        ValueDef::Constant(_) => panic!("unsupported printed bushido value"),
        _ => "Bushido",
    };
    AbilityDef::triggered(
        text,
        TriggerEventDef::BlocksOrBecomesBlockedBy {
            creature: ObjectPredicateDef::Source,
            other: ObjectPredicateDef::Any,
        },
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::modify_power_toughness(amount, amount),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )
}

/// Flanking. The keyword stays visible to predicates and grants; trigger
/// capture expands every effective instance through [`flanking_trigger`].
#[must_use]
pub const fn flanking() -> AbilityDef {
    keyword(
        "Flanking (Whenever this creature becomes blocked by a creature without flanking, the \
         blocking creature gets -1/-1 until end of turn.)",
        KeywordAbility::Flanking,
    )
}

static WITHOUT_FLANKING: ObjectPredicateDef = ObjectPredicateDef::Not(
    &ObjectPredicateDef::HasKeyword(KeywordAbility::Flanking),
);

/// The executable trigger abbreviated by one effective flanking instance.
/// Kept out of card rules so granting the keyword cannot accidentally leave
/// its behavior behind.
#[must_use]
pub(crate) const fn flanking_trigger() -> AbilityDef {
    AbilityDef::triggered(
        "Flanking",
        TriggerEventDef::BecomesBlockedBy {
            blocker: WITHOUT_FLANKING,
        },
        EffectDef::Apply {
            recipient: EffectRecipientDef::TriggeringObject,
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-1),
                ValueDef::Constant(-1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )
}

/// Bloodthirst N. This is an entry replacement rather than an enters trigger,
/// so the counters are already present when the permanent reaches the
/// battlefield and can affect any event that observes it entering.
///
/// # Panics
///
/// Panics when `amount` is not a printed bloodthirst value (1, 2, 3, or 6).
#[must_use]
pub const fn bloodthirst(amount: u16) -> AbilityDef {
    let text = match amount {
        1 => "Bloodthirst 1",
        2 => "Bloodthirst 2",
        3 => "Bloodthirst 3",
        6 => "Bloodthirst 6",
        _ => panic!("unsupported printed bloodthirst value"),
    };
    AbilityDef::as_enters_if(
        text,
        ReplacementConditionDef::OpponentWasDealtDamageThisTurn,
        ReplacementEffectDef::ModifyBattlefieldEntry(
            BattlefieldEntryModificationDef::AddCounters {
                kind: CounterKind::PlusOnePlusOne,
                amount,
            },
        ),
    )
}

/// One opponent means one life, so "that much" is the same constant on both
/// halves.
static EXTORT_DRAIN: EffectDef = EffectDef::Sequence(&[
    EffectDef::LoseLife {
        recipient: EffectRecipientDef::Opponent,
        amount: ValueDef::Constant(1),
    },
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
]);

/// Extort. Like exalted it is a keyword defined as a triggered ability, so
/// several instances on one permanent each offer their own payment -- which
/// is what makes a card that grants it worth more than one drain.
#[must_use]
pub const fn extort() -> AbilityDef {
    AbilityDef::triggered(
        "Extort (Whenever you cast a spell, you may pay {W/B}. If you do, each opponent loses 1 \
         life and you gain that much life.)",
        TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)),
        EffectDef::PayOr(PayOrDef::optional(
            EffectPaymentDef::mana(
                PlayerSetDef::Related(PlayerRelation::You),
                crate::mana_cost!("{W/B}"),
            ),
            &EXTORT_DRAIN,
        )),
    )
}

/// Battalion. Like exalted it is a keyword defined as a triggered ability, so
/// it takes the effect its card prints rather than being one fixed clause.
#[must_use]
pub const fn battalion(text: &'static str, effect: EffectDef) -> AbilityDef {
    AbilityDef::triggered(text, BATTALION_EVENT, effect)
}

/// "This creature and at least two other creatures attack" -- three in all,
/// with this one among them.
pub const BATTALION_EVENT: TriggerEventDef =
    TriggerEventDef::attacks_in_declaration(ObjectPredicateDef::Source, 3, None);

/// Unleash. The engine implements both halves from the keyword: an optional
/// +1/+1 counter offered as the permanent enters, and no blocking for as long
/// as it carries one.
#[must_use]
pub const fn unleash() -> AbilityDef {
    keyword(
        "Unleash (You may have this creature enter with a +1/+1 counter on it. It can't block as \
         long as it has a +1/+1 counter on it.)",
        KeywordAbility::Unleash,
    )
}

/// The optional entry clause unleash offers. It is a separate replacement
/// ability because the keyword itself carries no effect body.
#[must_use]
pub const fn unleash_counter() -> AbilityDef {
    AbilityDef::defined_replacement(
        "You may have this creature enter with a +1/+1 counter on it.",
        ReplacementAbilityDef::new()
            .with_event(ReplacementEventDef::SourceEntersBattlefield)
            .optional(),
        ReplacementEffectDef::ModifyBattlefieldEntry(
            BattlefieldEntryModificationDef::AddCounters {
                kind: CounterKind::PlusOnePlusOne,
                amount: 1,
            },
        ),
    )
}

/// Annihilator N: when this creature attacks, the defending player sacrifices
/// N permanents. The attack event carries that player as its event player, so
/// the sacrifice resolves correctly even when a planeswalker was attacked.
///
/// # Panics
///
/// Panics when `count` has no authored static rules-text spelling.
#[must_use]
pub const fn annihilator(count: i32) -> AbilityDef {
    let text = match count {
        1 => "Annihilator 1",
        2 => "Annihilator 2",
        3 => "Annihilator 3",
        4 => "Annihilator 4",
        5 => "Annihilator 5",
        6 => "Annihilator 6",
        _ => panic!("annihilator count has no authored rules text"),
    };
    AbilityDef::triggered(
        text,
        TriggerEventDef::attacks(ObjectPredicateDef::Source),
        EffectDef::SacrificeOfChoice {
            player: EffectRecipientDef::EventPlayer,
            object: ObjectPredicateDef::Any,
            count: ValueDef::Constant(count),
            then: None,
            amount: SacrificedAmountDef::Power,
            otherwise: None,
            optional: false,
        },
    )
}

#[must_use]
pub const fn protection_from_color(color: ManaColor) -> AbilityDef {
    match color {
        ManaColor::White => keyword(
            "Protection from white",
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Color(ManaColor::White)),
        ),
        ManaColor::Blue => keyword(
            "Protection from blue",
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Color(ManaColor::Blue)),
        ),
        ManaColor::Black => keyword(
            "Protection from black",
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Color(ManaColor::Black)),
        ),
        ManaColor::Red => keyword(
            "Protection from red",
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Color(ManaColor::Red)),
        ),
        ManaColor::Green => keyword(
            "Protection from green",
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Color(ManaColor::Green)),
        ),
        ManaColor::Colorless => keyword(
            "Protection from colorless",
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::ColorCount(0)),
        ),
    }
}

/// "Protection from multicolored", read off the source's color count rather
/// than any one color.
#[must_use]
pub const fn protection_from_multicolored() -> AbilityDef {
    keyword(
        "Protection from multicolored",
        KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::ColorCount(0),
            ObjectPredicateDef::ColorCount(1),
        ]))),
    )
}

/// "Protection from monocolored", the mirror of the clause above: exactly one
/// colour, so a colourless source and a gold one both get through.
#[must_use]
pub const fn protection_from_monocolored() -> AbilityDef {
    keyword(
        "Protection from monocolored",
        KeywordAbility::ProtectionFrom(&ObjectPredicateDef::ColorCount(1)),
    )
}

/// The reminder text every detain clause prints, so the cards agree on it.
pub const DETAIN_REMINDER: &str = "(Until your next turn, that permanent can't attack or block \
                                   and its activated abilities can't be activated.)";

/// Evolve. The comparison is against the source's own power and toughness at
/// the moment the creature enters, which is what makes a growing creature
/// stop evolving once it has outgrown what arrives.
#[must_use]
pub const fn evolve() -> AbilityDef {
    AbilityDef::triggered(
        "Evolve (Whenever a creature you control enters, if that creature has greater power or \
         toughness than this creature, put a +1/+1 counter on this creature.)",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::All(&EVOLVE_SUBJECT),
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::AddCounters {
            object: EffectRecipientDef::Source,
            kind: CounterKind::PlusOnePlusOne,
            amount: ValueDef::Constant(1),
        },
    )
}

static EVOLVE_SUBJECT: [ObjectPredicateDef; 4] = [
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
    // A creature does not evolve itself as it arrives.
    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
    ObjectPredicateDef::AnyOf(&EVOLVE_BIGGER),
];

static EVOLVE_BIGGER: [ObjectPredicateDef; 2] = [
    ObjectPredicateDef::PowerGreaterThan(ValueDef::SourcePower),
    ObjectPredicateDef::ToughnessGreaterThan(ValueDef::SourceToughness),
];

/// A spell or ability an opponent controls, which is the half of the stack
/// ward answers.
static AN_OPPONENTS_SPELL_OR_ABILITY: ObjectPredicateDef =
    ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent);

/// Ward (CR 702.21): "Whenever this permanent becomes the target of a spell
/// or ability an opponent controls, counter it unless that player pays
/// `amount`."
///
/// Written out as the triggered ability it abbreviates, for the same reason
/// prowess is: nothing in the rules reads "has ward" the way combat reads
/// flying, so the clause is the whole of it. The text is the caller's
/// because a card that grants ward prints the reminder in its own voice.
#[must_use]
pub const fn ward(amount: u16, text: &'static str) -> AbilityDef {
    AbilityDef::triggered(
        text,
        TriggerEventDef::becomes_targeted(AN_OPPONENTS_SPELL_OR_ABILITY),
        pay_or_counter(
            PlayerRefDef::ControllerOf(ObjectRefDef::TriggeringObject),
            ValueDef::Constant(amount as i32),
            &COUNTER_TRIGGERING_SPELL,
        ),
    )
}

/// "Ward--Pay N life", the same keyword with a life cost. Ward's cost is any
/// cost the card cares to print (CR 702.21a), so the shape above is the mana
/// case rather than the whole of it.
#[must_use]
pub const fn ward_life(amount: u16, text: &'static str) -> AbilityDef {
    AbilityDef::triggered(
        text,
        TriggerEventDef::becomes_targeted(AN_OPPONENTS_SPELL_OR_ABILITY),
        EffectDef::PayOr(PayOrDef {
            payment: EffectPaymentDef::life(
                PlayerSetDef::One(PlayerRefDef::ControllerOf(ObjectRefDef::TriggeringObject)),
                amount,
            ),
            if_paid: None,
            otherwise: Some(&COUNTER_TRIGGERING_SPELL),
            visibility: ChoiceVisibilityDef::Public,
            condition: None,
        }),
    )
}

/// The creature a backup trigger points at. Any creature, its own included:
/// backing yourself up is a legal and sometimes correct choice, and it is
/// the case the second half of the keyword has to ask about.
static BACKUP_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

/// "If that's another creature": counters put on the backer itself lend it
/// nothing, because what would be lent is already printed on it.
static BACKUP_TARGET_IS_ANOTHER: TriggerConditionDef = TriggerConditionDef::TargetMatches {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
};

/// Backup N (CR 702.165). The enters trigger it abbreviates; the abilities
/// listed after the word are printed on the creature as well, so a card with
/// backup writes them out beside this.
#[must_use]
pub const fn backup(text: &'static str, steps: &'static [EffectDef]) -> AbilityDef {
    enters_trigger_with_targets(text, &BACKUP_TARGET, EffectDef::Sequence(steps))
}

/// The two steps every backup takes: the counters, and the loan when the
/// counters went somewhere else. What is lent belongs to the card, which is
/// why it is passed in rather than built here.
#[must_use]
pub const fn backup_steps(count: i32, lends: &'static EffectDef) -> [EffectDef; 2] {
    [
        EffectDef::AddCounters {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            kind: CounterKind::PlusOnePlusOne,
            amount: ValueDef::Constant(count),
        },
        EffectDef::IfCondition {
            condition: &BACKUP_TARGET_IS_ANOTHER,
            then: lends,
        },
    ]
}

/// Cascade (CR 702.85). A triggered ability that fires on the cast, like
/// storm, and whose whole procedure is one effect: the bound it digs to is
/// the cascading spell's own mana value, so nothing about it is written down
/// on the card beyond the word.
#[must_use]
pub const fn cascade() -> AbilityDef {
    AbilityDef::triggered(
        "Cascade (When you cast this spell, exile cards from the top of your library until you \
         exile a nonland card that costs less. You may cast it without paying its mana cost. Put \
         the exiled cards on the bottom of your library in a random order.)",
        TriggerEventDef::spell_cast(ObjectPredicateDef::Source),
        EffectDef::Cascade,
    )
}

/// Saddle N (CR 702.166a). The cost is paid by tapping other untapped
/// creatures whose power adds up to at least N, and what it buys is a fact
/// about the Mount that lasts until end of turn -- the Mount's own printed
/// clauses are the only things that read it.
#[must_use]
pub const fn saddle(minimum: u8, text: &'static str) -> AbilityDef {
    AbilityDef::activated_with_cost_list_and_targets(
        text,
        AbilityCostList::one(AbilityCostDef::TapCreaturesWithTotalPower { minimum }),
        &[],
        EffectDef::Saddle {
            object: EffectRecipientDef::Source,
        },
    )
    .with_activation_timing(ActivationTimingDef::SorcerySpeed)
}

/// Splice onto Arcane (CR 702.47a): a cost paid as somebody else's Arcane
/// spell is cast, which adds this card's clause to that spell. The card
/// stays in hand, so like plot this is not a way to cast it and nothing
/// offers it as one -- the clause exists to carry the cost.
#[must_use]
pub const fn splice_onto_arcane(mana_cost: ManaCost) -> AbilityDef {
    AbilityDef::alternative_cast(
        mana_cost,
        AlternativeCastKindDef::Splice,
        None,
        EffectDef::None,
    )
}

/// Plot (CR 702.170a): a cost paid to a special action rather than to a
/// cast. What it buys is a card sitting in exile that its owner may cast for
/// nothing on a later turn, which is why the clause carries the cost and
/// nothing offers it as a way to cast the card now.
#[must_use]
pub const fn plot(mana_cost: ManaCost) -> AbilityDef {
    AbilityDef::alternative_cast(
        mana_cost,
        AlternativeCastKindDef::Plot,
        None,
        EffectDef::None,
    )
}

const REBOUND_TEXT: &str = "Rebound (If you cast this spell from your hand, exile it as it resolves. At the beginning of your next upkeep, you may cast this card from exile without paying its mana cost.)";

/// What the card is lent while rebound's offer stands. The cast is free and
/// comes from exile; unlike the free cast a resolution lends, nothing exiles
/// the card afterwards, because it was already there.
static REBOUND_FREE_CAST: AbilityDef = AbilityDef::alternative_cast(
    crate::mana_cost!("{0}"),
    AlternativeCastKindDef::Rebound,
    Some("Cast this card from exile without paying its mana cost."),
    EffectDef::None,
);

/// The delayed half: at the caster's next upkeep, the card sitting in exile
/// is offered back. `Source` is the spell that installed this, and following
/// it lands on the card that spell became when it was exiled.
pub(crate) static REBOUND_OFFER: AbilityDef = AbilityDef::triggered(
    "At the beginning of your next upkeep, you may cast this card from exile without paying its \
     mana cost.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::MayCastTargetWithoutPaying {
        object: EffectRecipientDef::SourceZoneChangeSuccessor,
        ability: &REBOUND_FREE_CAST,
    },
);

pub(crate) static REBOUND_DELAYED_TRIGGER: InstalledTriggerDef = InstalledTriggerDef {
    ability: &REBOUND_OFFER,
    lifetime: InstalledTriggerLifetimeDef::Once,
};

/// Rebound (CR 702.87a). The keyword is one structural clause: resolving a
/// spell cast from hand exiles it and installs the next-upkeep offer that may
/// cast the exiled card for free. The stack-resolution path owns both halves,
/// so a card declaration cannot accidentally request only one of them.
#[must_use]
pub const fn rebound() -> AbilityDef {
    keyword(REBOUND_TEXT, KeywordAbility::Rebound)
}

/// "Eternalize {cost}" (CR 702.129a).
///
/// An activated ability of the card in its owner's graveyard: it exiles
/// itself as a cost and makes a token copy of what it just exiled, except
/// for the four things the keyword fixes -- a 4/4 body, black, a Zombie on
/// top of the types it already had, and no mana cost. Sorcery timing,
/// because the reminder says so.
///
/// The caller supplies the printed text, which repeats both the cost and the
/// card's own creature types.
#[must_use]
pub const fn eternalize(text: &'static str, cost: ManaCost) -> AbilityDef {
    AbilityDef::activated_with_cost_list_and_targets(
        text,
        AbilityCostList::two(AbilityCostDef::Mana(cost), AbilityCostDef::ExileSource),
        &[],
        EffectDef::create_token_from_copy(&ETERNALIZE_COPY),
    )
    .with_source_zones(&[ZoneKind::Graveyard])
    .with_activation_timing(ActivationTimingDef::SorcerySpeed)
}

/// Ninjutsu (CR 702.49): "`cost`, Return an unblocked attacker you control
/// to hand: Put this card onto the battlefield from your hand tapped and
/// attacking."
///
/// The return is a cost rather than an effect, so a Ninja whose activation
/// is answered has already swapped the attacker away. Activation waits for
/// attackers to be declared, since until then there is no unblocked attacker
/// to give back. The caller supplies the printed text, which repeats the
/// cost inside its own reminder.
#[must_use]
pub const fn ninjutsu(text: &'static str, cost: ManaCost) -> AbilityDef {
    AbilityDef::activated_with_cost_list_and_targets(
        text,
        AbilityCostList::two(
            AbilityCostDef::Mana(cost),
            AbilityCostDef::ReturnUnblockedAttackerToHand,
        ),
        &[],
        EffectDef::PutSourceOntoBattlefieldAttacking,
    )
    .with_source_zones(&[ZoneKind::Hand])
    .with_activation_timing(ActivationTimingDef::AfterAttackersDeclared)
}

/// The one type every eternalized token gains, whatever it was before.
static ETERNALIZE_ADDED_TYPES: [&str; 1] = ["Zombie"];

static ETERNALIZE_COPY: crate::card::TokenCopyDef = crate::card::TokenCopyDef {
    object: &EffectRecipientDef::SourceZoneChangeSuccessor,
    exceptions: CopyExceptionsDef::undead(
        4,
        4,
        ColorSet::from_colors(&[ManaColor::Black]),
        &ETERNALIZE_ADDED_TYPES,
    ),
};

/// The one type crewing adds. A Vehicle is already an artifact, so what
/// crewing gives it is the creature half of "artifact creature".
static CREW_ADDS_CREATURE: AppliedEffectDef =
    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature));

/// "Crew N" (CR 702.122a).
///
/// An activated ability whose cost is tapping creatures rather than mana:
/// any number of untapped creatures you control whose power adds up to N or
/// more. What it buys lasts until end of turn, so a Vehicle crewed to block
/// is an artifact again by the next turn.
///
/// The caller supplies the printed text, which repeats the number.
#[must_use]
pub const fn crew(text: &'static str, minimum: u8) -> AbilityDef {
    AbilityDef::activated(
        text,
        crew_cost(minimum),
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: CREW_ADDS_CREATURE,
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )
}

/// Crew's cost, kept beside it because a const slice cannot be built from a
/// parameter inline: the table is indexed by the printed number.
const fn crew_cost(minimum: u8) -> &'static [AbilityCostDef] {
    let index = minimum as usize;
    assert!(
        index < CREW_COSTS.len(),
        "no printed Vehicle asks for that much crew"
    );
    &CREW_COSTS[index]
}

/// One cost per crew number, from zero up. Zero is unreachable from any
/// printed card and sits here only so the table is indexed by the number
/// itself rather than by an offset.
static CREW_COSTS: [[AbilityCostDef; 1]; 10] = [
    crew_tap(0),
    crew_tap(1),
    crew_tap(2),
    crew_tap(3),
    crew_tap(4),
    crew_tap(5),
    crew_tap(6),
    crew_tap(7),
    crew_tap(8),
    crew_tap(9),
];

const fn crew_tap(minimum: u8) -> [AbilityCostDef; 1] {
    [AbilityCostDef::TapCreaturesWithTotalPower { minimum }]
}

/// "Battle cry (Whenever this creature attacks, each other attacking creature
/// gets +1/+0 until end of turn.)"
///
/// Written out as the triggered ability it abbreviates. Each printed instance
/// triggers independently and boosts only the creatures attacking alongside
/// its own source.
#[must_use]
pub const fn battle_cry() -> AbilityDef {
    AbilityDef::triggered(
        "Battle cry (Whenever this creature attacks, each other attacking creature gets +1/+0 \
         until end of turn.)",
        TriggerEventDef::attacks(ObjectPredicateDef::Source),
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(0),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )
}
