// The block-keyword mechanics: the ones printed as a single word that
// expand into a whole trigger, and the state a couple of them carry.
//
// Grouped together because they share a shape rather than a subject --
// each is one keyword the card prints and one clause the engine runs.

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
        TriggerEventDef::SpellCast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)),
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
        TriggerEventDef::BecomesTargetOfSpellOrAbility(AN_OPPONENTS_SPELL_OR_ABILITY),
        pay_or_counter(
            PlayerRefDef::ControllerOf(ObjectRefDef::TriggeringObject),
            ValueDef::Constant(amount as i32),
            &COUNTER_TRIGGERING_SPELL,
        ),
    )
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
        TriggerEventDef::SpellCast(ObjectPredicateDef::Source),
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
