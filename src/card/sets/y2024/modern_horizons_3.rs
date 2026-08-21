//! Modern Horizons 3 cards cataloged as attachment edge cases.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AlternativeCastKindDef,
    AppliedEffectDef, AppliedRuleDef, BasicLandType, CardArt, CardComposition, CardEffectStatus,
    CardPart, CardRules, CardSet, CardStructure, CardSupertype, CardType, ChoiceVisibilityDef,
    ChooseDef, ComparisonDef, CounterKind, DoubleFacedKind, EffectDef, EffectPaymentCostDef,
    EffectPaymentDef, EffectRecipientDef, InstalledTriggerDef, ManaColor, ObjectChoiceBindingDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, PayOrDef, PlayOptionDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef, SpellAdditionalCostDef,
    SpellForm, SpendModeDef, TokenCharacteristics, TriggerConditionDef, TriggerEventDef,
    TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities, cards,
};
use crate::ids::{CardPartId, ObjectBindingIndex, ObjectSetBindingIndex, PlayOptionId};
use crate::{TargetIndex, mana_cost};

/// "Until this enchantment leaves the battlefield" is one printed ability,
/// so the return rides on the same resolution as a delayed trigger rather
/// than appearing as a second clause the card does not print.
/// "For each token you control that entered this turn." The Cat the clause
/// just made is one of them, which is what makes the doubling compound.
static YOUR_NEW_TOKENS: ObjectQueryDef = ObjectQueryDef::controlled_by(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::Token,
        ObjectPredicateDef::CameUnderControlThisTurn,
    ]),
    &[ZoneKind::Battlefield],
    PlayerSetDef::Related(PlayerRelation::You),
);

static OCELOT_DOUBLES_THEM: EffectDef = EffectDef::CreateTokenCopyOf {
    object: EffectRecipientDef::objects(ObjectSetDef::Query(YOUR_NEW_TOKENS)),
};

static OCELOT_END_STEP: [EffectDef; 2] = [
    EffectDef::create_creature_token(&["Cat"], &[ManaColor::White], 1, 1).with_art(CardArt::new(
        "74bacab2-a4c6-4ba5-a208-6bd09ae4cf9f",
        "Maxime Minard",
    )),
    // The blessing half is checked as this resolves rather than as it
    // triggers, so ascending in response still doubles.
    EffectDef::IfCondition {
        condition: &TriggerConditionDef::ControllerHasCitysBlessing,
        then: &OCELOT_DOUBLES_THEM,
    },
];

static OCELOT_PRIDE_ABILITIES: [AbilityDef; 4] = [
    abilities::first_strike(),
    abilities::lifelink(),
    AbilityDef::static_ability(
        "Ascend (If you control ten or more permanents, you get the city's blessing for the rest \
         of the game.)",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::Ascend),
        },
    ),
    AbilityDef::triggered_if(
        "At the beginning of your end step, if you gained life this turn, create a 1/1 white Cat \
         creature token. Then if you have the city's blessing, for each token you control that \
         entered this turn, create a token that's a copy of it.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::You,
        },
        &TriggerConditionDef::ControllerGainedLifeThisTurn,
        EffectDef::Sequence(&OCELOT_END_STEP),
    ),
];

// MH3 38 — Ocelot Pride
pub(in crate::card::sets) static OCELOT_PRIDE: CardRecord = CardRecord::new(
    cards::OCELOT_PRIDE,
    "Ocelot Pride",
    CardArt::new("89cf6f57-230f-497e-a14e-ad1e8737fd42", "Chris Seaman"),
    CardSet::ModernHorizons3,
    // Its own lifelink turns the trigger on, and once the board is wide
    // enough to ascend every Cat it ever made comes back doubled.
    CardRules::new_creature(mana_cost!("{W}"), &["Cat"], 1, 1)
        .with_abilities(&OCELOT_PRIDE_ABILITIES),
);

static PRISON_RETURNS_IT: AbilityDef = AbilityDef::triggered(
    "When this enchantment leaves the battlefield, return the exiled card to the battlefield under its owner's control.",
    TriggerEventDef::zone_changed(
        ObjectPredicateDef::Source,
        Some(ZoneKind::Battlefield),
        None,
    ),
    EffectDef::ReturnLinkedExiles {
        object: ObjectPredicateDef::Any,
        counters: None,
        arrival_effect: None,
        zone: ZoneKind::Battlefield,
        grant: None,
        controller: None,
        transformed: false,
    },
);

static PRISON_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::Opponent),
        owner: None,
    },
)];

static PRISON_ENTERS: [EffectDef; 3] = [
    EffectDef::ExileLinkedToSource {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&PRISON_RETURNS_IT)),
    // The energy arrives with the exile rather than paying for it: the first
    // upkeep tax is already covered, and the second is not.
    EffectDef::AddEnergyCounters {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    },
];

static PRISON_SACRIFICE: EffectDef = EffectDef::Sacrifice {
    object: EffectRecipientDef::Source,
};

// MH3 44 — Static Prison
pub(in crate::card::sets) static STATIC_PRISON: CardRecord = CardRecord::new(
    cards::STATIC_PRISON,
    "Static Prison",
    CardArt::new("dd16222e-349c-4a2b-a7c8-8eb35a8ab332", "Jason A. Engle"),
    CardSet::ModernHorizons3,
    // One white answers anything, and the two energy it comes with buy two
    // more turns of holding it. After that the prison opens.
    CardRules::new_enchantment(mana_cost!("{W}")).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "When this enchantment enters, exile target nonland permanent an opponent controls until this enchantment leaves the battlefield. You get {E}{E} (two energy counters).",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &PRISON_TARGET,
            EffectDef::Sequence(&PRISON_ENTERS),
        ),
        AbilityDef::triggered(
            "At the beginning of your first main phase, sacrifice this enchantment unless you pay {E}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::PrecombatMain,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::unless(
                EffectPaymentDef {
                    payer: PlayerSetDef::One(PlayerRefDef::EffectController),
                    cost: EffectPaymentCostDef::Energy(1),
                },
                &PRISON_SACRIFICE,
            )),
        ),
    ]),
);

/// A land is what the exile walks past; the first thing that is not one is
/// what you get to keep.
static A_NONLAND_CARD: ObjectPredicateDef =
    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land));

static RAPTOR_DIGS: EffectDef = EffectDef::ExileFromTopUntil {
    player: EffectRecipientDef::Controller,
    object: A_NONLAND_CARD,
};

/// "Then if you cast it from your hand" is part of the effect rather than an
/// intervening-if: a Raptor put onto the battlefield gets the energy and
/// nothing else.
static RAPTOR_ENTERS: [EffectDef; 2] = [
    EffectDef::AddEnergyCounters {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    },
    EffectDef::IfCondition {
        condition: &TriggerConditionDef::SourceCastFrom(ZoneKind::Hand),
        then: &RAPTOR_DIGS,
    },
];

static AMPED_RAPTOR_ABILITIES: [AbilityDef; 2] = [
    abilities::first_strike(),
    AbilityDef::triggered(
        "When this creature enters, you get {E}{E} (two energy counters). Then if you cast it \
         from your hand, exile cards from the top of your library until you exile a nonland card. \
         You may cast that card by paying an amount of {E} equal to its mana value rather than \
         paying its mana cost.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::Sequence(&RAPTOR_ENTERS),
    ),
];

/// Anybody's graveyard, and "up to one": an Emperor with nothing worth
/// taking still gets its combat trigger, and simply exiles nothing.
static EMPEROR_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Any,
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: None,
    },
    1,
)];

/// Adapt is a conditional rather than a cost: the ability always resolves,
/// and finding a counter already there is what makes it do nothing.
static EMPEROR_ADAPTS: EffectDef = EffectDef::IfCondition {
    condition: &TriggerConditionDef::SourceCounters {
        kind: CounterKind::PlusOnePlusOne,
        comparison: ComparisonDef::LessOrEqual,
        amount: 0,
    },
    then: &EffectDef::AddCounters {
        object: EffectRecipientDef::Source,
        kind: CounterKind::PlusOnePlusOne,
        amount: ValueDef::Constant(2),
    },
};

static EMPEROR_ADAPT_COST: [AbilityCostDef; 1] = [AbilityCostDef::Mana(mana_cost!("{1}{B}"))];

/// "A creature card exiled with this creature": a pile no query can find,
/// because what puts a card in it is which permanent exiled it.
static A_CREATURE_CARD_THE_EMPEROR_TOOK: ObjectSetDef =
    ObjectSetDef::LinkedExiles(ObjectPredicateDef::HasType(CardType::Creature));

static EMPEROR_SACRIFICES_IT: AbilityDef = AbilityDef::triggered(
    "At the beginning of the next end step, sacrifice that creature.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::End,
        player: PlayerRelation::Any,
    },
    EffectDef::Sacrifice {
        object: EffectRecipientDef::objects(ObjectSetDef::Binding(EMPEROR_ARRIVAL)),
    },
);

const EMPEROR_ARRIVAL: ObjectSetBindingIndex = ObjectSetBindingIndex::PRIMARY;

static EMPEROR_REANIMATES: EffectDef = EffectDef::ReturnWithHasteAndFinality {
    object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    binding: EMPEROR_ARRIVAL,
    then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(&EMPEROR_SACRIFICES_IT)),
};

static EMPEROR_CHOOSES: EffectDef = EffectDef::Choose(ChooseDef {
    binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
    unchosen: None,
    chooser: PlayerRefDef::EffectController,
    candidates: A_CREATURE_CARD_THE_EMPEROR_TOOK,
    exclude: None,
    minimum: 1,
    maximum: 1,
    visibility: ChoiceVisibilityDef::Public,
    then: &EMPEROR_REANIMATES,
});

static EMPEROR_OF_BONES_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::triggered_with_targets(
        "At the beginning of combat on your turn, exile up to one target card from a graveyard.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::BeginningOfCombat,
            player: PlayerRelation::You,
        },
        &EMPEROR_TARGET,
        EffectDef::ExileLinkedToSource {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    ),
    AbilityDef::activated(
        "{1}{B}: Adapt 2. (If this creature has no +1/+1 counters on it, put two +1/+1 counters \
         on it.)",
        &EMPEROR_ADAPT_COST,
        EMPEROR_ADAPTS,
    ),
    AbilityDef::triggered(
        "Whenever one or more +1/+1 counters are put on this creature, put a creature card exiled \
         with this creature onto the battlefield under your control with a finality counter on \
         it. It gains haste. Sacrifice it at the beginning of the next end step.",
        TriggerEventDef::CountersPlaced {
            object: ObjectPredicateDef::Source,
            kind: CounterKind::PlusOnePlusOne,
        },
        EMPEROR_CHOOSES,
    ),
];

// MH3 90 — Emperor of Bones
pub(in crate::card::sets) static EMPEROR_OF_BONES: CardRecord = CardRecord::new(
    cards::EMPEROR_OF_BONES,
    "Emperor of Bones",
    CardArt::new("df9d9075-2d1e-4848-b661-816d539e05eb", "Josh Hass"),
    CardSet::ModernHorizons3,
    // Two mana that eats a graveyard one card a turn and then rents the best
    // of them back for an attack, which is what makes the adapt cost worth
    // paying twice.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Skeleton", "Noble"], 2, 2)
        .with_abilities(&EMPEROR_OF_BONES_ABILITIES),
);

// MH3 114 — Amped Raptor
pub(in crate::card::sets) static AMPED_RAPTOR: CardRecord = CardRecord::new(
    cards::AMPED_RAPTOR,
    "Amped Raptor",
    CardArt::new("1ac0e78b-0fdd-44f9-8b7b-c4f28a32782e", "Alex Konstad"),
    CardSet::ModernHorizons3,
    // Two mana for a 2/1 first striker and a free spell off the top, as long
    // as the top of the deck is cheap enough for two energy to cover.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Dinosaur"], 2, 1)
        .with_abilities(&AMPED_RAPTOR_ABILITIES),
);

// MH3 148 — Colossal Dreadmask
pub(in crate::card::sets) static COLOSSAL_DREADMASK: CardRecord = CardRecord::new(
    cards::COLOSSAL_DREADMASK,
    "Colossal Dreadmask",
    CardArt::new("98164430-64c1-465f-b786-45753c965f44", "Caio Monteiro"),
    CardSet::ModernHorizons3,
    CardRules::new_artifact(mana_cost!("{4}{G}{G}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::living_weapon(
                TokenCharacteristics::creature(&["Phyrexian", "Germ"], &[ManaColor::Black], 0, 0)
                    .with_art(CardArt::new(
                        "5ec719dc-6b07-4b1d-a79c-84ebced33422",
                        "Igor Kieryluk",
                    )),
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +6/+6 and has trample.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(6),
                            ValueDef::Constant(6),
                        ),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{3}{G}{G}"))],
                "Equip {3}{G}{G}",
            ),
        ]),
);

/// The kicked half changes nothing about how the spell resolves: it costs
/// more, and the second cast trigger reads that fact. That is why the
/// alternative carries no instructions of its own.
static MYCOSPAWN_KICKED: TriggerConditionDef =
    TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked);

static MYCOSPAWN_EXILE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Land),
)];

static MYCOSPAWN_ABILITIES: [AbilityDef; 4] = [
    // Devoid is the empty printed colour set below; the keyword is here so
    // the card says what it is.
    abilities::devoid(),
    AbilityDef::alternative_cast(
        mana_cost!("{4}{G}{C}"),
        AlternativeCastKindDef::Kicked,
        Some("Kicker {1}{C} (You may pay an additional {1}{C} as you cast this spell.)"),
        EffectDef::None,
    ),
    AbilityDef::triggered(
        "When you cast this spell, search your library for a land card, put it onto the battlefield, then shuffle.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::Source),
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasType(CardType::Land),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            binding: None,
            then: None,
        },
    ),
    AbilityDef::triggered_if_with_targets(
        "When you cast this spell, if it was kicked, exile target land.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::Source),
        &MYCOSPAWN_KICKED,
        &MYCOSPAWN_EXILE_TARGET,
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Exile,
            controller: None,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
        },
    ),
];

// MH3 170 — Sowing Mycospawn
pub(in crate::card::sets) static SOWING_MYCOSPAWN: CardRecord = CardRecord::new(
    cards::SOWING_MYCOSPAWN,
    "Sowing Mycospawn",
    CardArt::new("cdfadb17-76ad-4d4d-9fa7-33c4b88b4c0a", "Slawomir Maniak"),
    CardSet::ModernHorizons3,
    // Four mana finds a land and six exiles one, and both happen on the cast
    // rather than on arrival -- so countering the creature does not stop
    // either of them.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Eldrazi", "Fungus"], 3, 3)
        .printed_colors(&[])
        .with_abilities(&MYCOSPAWN_ABILITIES),
);

/// The Cats that matter are the other ones: Ajani dying alongside them does
/// not turn him over, and neither does his own death.
static ANOTHER_CAT_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::Subtype("Cat"),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
]);

/// "Exile Ajani, then return him to the battlefield transformed." One
/// resolution: the exile links him to himself and the return brings him
/// straight back on the other face, under his owner's control.
static AJANI_TURNS_OVER: [EffectDef; 2] = [
    EffectDef::ExileLinkedToSource {
        object: EffectRecipientDef::Source,
    },
    EffectDef::ReturnLinkedExiles {
        object: ObjectPredicateDef::Any,
        counters: None,
        arrival_effect: None,
        zone: ZoneKind::Battlefield,
        grant: None,
        controller: None,
        transformed: true,
    },
];

static CATS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::Subtype("Cat"),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

/// "If you control a red permanent other than Ajani." Ajani himself is
/// white, so the clause is about a second permanent rather than about him.
static A_RED_PERMANENT_BESIDES_AJANI: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::Color(ManaColor::Red),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
        ]),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 1,
};

/// The reflexive "when you do" is folded into this resolution: the token is
/// made, and then the damage happens if the condition holds. What that
/// costs is the separate window between the two and the chance to decline
/// the damage; the target is named as the ability is activated instead of
/// after the token appears, and there is always a legal one because a
/// player is a legal target.
static AJANI_MAKES_A_CAT_AND_MAY_BURN: [EffectDef; 2] = [
    EffectDef::create_creature_token(&["Cat", "Warrior"], &[ManaColor::White], 2, 1).with_art(
        CardArt::new("ce5c5bcf-1fdd-4d73-a92b-223292da00ca", "Ben Wootten"),
    ),
    EffectDef::IfCondition {
        condition: &A_RED_PERMANENT_BESIDES_AJANI,
        then: &EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::CountMatchingObjects(&CREATURES_YOU_CONTROL),
        },
    },
];

static CREATURES_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

static AJANI_BURN_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

/// The four types the ultimate lets each opponent keep one of. Order is the
/// printed order, which is the order the questions are asked in.
static AJANI_SPARED_TYPES: [CardType; 4] = [
    CardType::Artifact,
    CardType::Creature,
    CardType::Enchantment,
    CardType::Planeswalker,
];

static AJANI_TURNS_OVER_SEQUENCE: EffectDef = EffectDef::Sequence(&AJANI_TURNS_OVER);

static AJANI_PARIAH_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::triggered(
        "When Ajani enters, create a 2/1 white Cat Warrior creature token.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::create_creature_token(&["Cat", "Warrior"], &[ManaColor::White], 2, 1).with_art(
            CardArt::new("ce5c5bcf-1fdd-4d73-a92b-223292da00ca", "Ben Wootten"),
        ),
    ),
    // One trigger per Cat rather than one per batch. Several Cats dying at
    // once fire it several times, and every firing after the first finds
    // Ajani already exiled and returned as a new object, so it has nothing
    // left to turn over.
    AbilityDef::triggered(
        "Whenever one or more other Cats you control die, you may exile Ajani, then return him to the battlefield transformed under his owner's control.",
        TriggerEventDef::zone_changed(
            ANOTHER_CAT_YOU_CONTROL,
            Some(ZoneKind::Battlefield),
            Some(ZoneKind::Graveyard),
        ),
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &AJANI_TURNS_OVER_SEQUENCE,
        },
    ),
];

static AJANI_AVENGER_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::activated(
        "+2: Put a +1/+1 counter on each Cat you control.",
        &AJANI_PLUS_TWO_COST,
        EffectDef::AddCounters {
            object: EffectRecipientDef::objects(ObjectSetDef::Query(CATS_YOU_CONTROL)),
            kind: CounterKind::PlusOnePlusOne,
            amount: ValueDef::Constant(1),
        },
    ),
    AbilityDef::activated_with_targets(
        "0: Create a 2/1 white Cat Warrior creature token. When you do, if you control a red permanent other than Ajani, he deals damage equal to the number of creatures you control to any target.",
        &AJANI_ZERO_COST,
        &AJANI_BURN_TARGET,
        EffectDef::Sequence(&AJANI_MAKES_A_CAT_AND_MAY_BURN),
    ),
    AbilityDef::activated(
        "−4: Each opponent chooses an artifact, a creature, an enchantment, and a planeswalker from among the nonland permanents they control, then sacrifices the rest.",
        &AJANI_MINUS_FOUR_COST,
        EffectDef::SacrificeKeepingOnePerType {
            player: EffectRecipientDef::Opponent,
            types: &AJANI_SPARED_TYPES,
        },
    ),
];

static AJANI_PLUS_TWO_COST: [AbilityCostDef; 1] = [AbilityCostDef::Loyalty(2)];
static AJANI_ZERO_COST: [AbilityCostDef; 1] = [AbilityCostDef::Loyalty(0)];
static AJANI_MINUS_FOUR_COST: [AbilityCostDef; 1] = [AbilityCostDef::Loyalty(-4)];

const fn ajani_nacatl_pariah_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Cat", "Warrior"], 1, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&AJANI_PARIAH_ABILITIES)
}

const fn ajani_nacatl_avenger_rules() -> CardRules {
    CardRules::new_planeswalker_without_mana_cost(&["Ajani"])
        .with_supertype(CardSupertype::Legendary)
        .with_starting_loyalty(3)
        .with_abilities(&AJANI_AVENGER_ABILITIES)
}

fn ajani_composition() -> CardComposition {
    CardComposition {
        parts: vec![
            CardPart::new(
                CardPartId::PRIMARY,
                "Ajani, Nacatl Pariah",
                ajani_nacatl_pariah_rules(),
            ),
            CardPart::new(
                CardPartId(1),
                "Ajani, Nacatl Avenger",
                ajani_nacatl_avenger_rules(),
            ),
        ],
        structure: CardStructure::DoubleFaced {
            front: CardPartId::PRIMARY,
            back: CardPartId(1),
            kind: DoubleFacedKind::Transforming,
        },
        play_options: vec![PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Ajani, Nacatl Pariah",
            SpellForm::Part(CardPartId::PRIMARY),
            mana_cost!("{1}{W}"),
            CardEffectStatus::Implemented,
        )],
    }
}

/// Five cards out of your own graveyard, exiled to pay. The card being cast
/// is on the stack by the time costs are paid, so "other" takes care of
/// itself: it is not there to be chosen.
static EXILE_FIVE_OTHER_CARDS: SpellAdditionalCostDef =
    SpellAdditionalCostDef::new(ObjectPredicateDef::Any, ZoneKind::Graveyard, 5)
        .spent(SpendModeDef::Exile);

static PHLAGE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

/// Entering and attacking are two ways for one printed ability to fire, so
/// the damage and the life are written once.
static PHLAGE_EVENTS: [TriggerEventDef; 2] = [
    TriggerEventDef::zone_changed(
        ObjectPredicateDef::Source,
        None,
        Some(ZoneKind::Battlefield),
    ),
    TriggerEventDef::attacks(ObjectPredicateDef::Source),
];

static PHLAGE_BOLTS: [EffectDef; 2] = [
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(3),
    },
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(3),
    },
];

/// "Unless it escaped" reads how the spell was cast, which the permanent
/// remembers: a Phlage cast for its printed cost sacrifices itself and leaves
/// the Lightning Helix behind.
static PHLAGE_DID_NOT_ESCAPE: TriggerConditionDef = TriggerConditionDef::Not(
    &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Escape),
);

static PHLAGE_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::triggered_if(
        "When this creature enters, sacrifice it unless it escaped.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        &PHLAGE_DID_NOT_ESCAPE,
        EffectDef::Sacrifice {
            object: EffectRecipientDef::Source,
        },
    ),
    AbilityDef::triggered_with_targets(
        "Whenever this creature enters or attacks, it deals 3 damage to any target and you gain \
         3 life.",
        TriggerEventDef::AnyOf(&PHLAGE_EVENTS),
        &PHLAGE_TARGET,
        EffectDef::Sequence(&PHLAGE_BOLTS),
    ),
    AbilityDef::alternative_cast(
        mana_cost!("{R}{R}{W}{W}"),
        AlternativeCastKindDef::Escape,
        Some(
            "Escape—{R}{R}{W}{W}, Exile five other cards from your graveyard. (You may cast this \
             card from your graveyard for its escape cost.)",
        ),
        EffectDef::None,
    )
    .with_alternative_additional_cost(&EXILE_FIVE_OTHER_CARDS),
];

// MH3 197 — Phlage, Titan of Fire's Fury
pub(in crate::card::sets) static PHLAGE_TITAN_OF_FIRES_FURY: CardRecord = CardRecord::new(
    cards::PHLAGE_TITAN_OF_FIRES_FURY,
    "Phlage, Titan of Fire's Fury",
    CardArt::new("e419cd0b-2449-4cc5-9ead-b9e45e271700", "Lucas Graciano"),
    CardSet::ModernHorizons3,
    // A three-mana Lightning Helix that stays a Helix until the graveyard is
    // deep enough, and then is a 6/6 that helixes again every attack.
    CardRules::new_creature(mana_cost!("{1}{R}{W}"), &["Elder", "Giant"], 6, 6)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&PHLAGE_ABILITIES),
);

static PSYCHIC_FROG_ABILITIES: [AbilityDef; 3] = [
    // A player or a planeswalker: the Frog is happy to be chumped by neither.
    AbilityDef::triggered(
        "Whenever this creature deals combat damage to a player or planeswalker, draw a card.",
        TriggerEventDef::combat_damage_to_player_or_planeswalker(ObjectPredicateDef::Source),
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    ),
    // No mana in either cost, and no tap: the Frog grows as often as the hand
    // allows and flies as often as the graveyard does.
    AbilityDef::activated(
        "Discard a card: Put a +1/+1 counter on this creature.",
        &[AbilityCostDef::DiscardCardMatching(ObjectPredicateDef::Any)],
        EffectDef::AddCounters {
            object: EffectRecipientDef::Source,
            kind: CounterKind::PlusOnePlusOne,
            amount: ValueDef::Constant(1),
        },
    ),
    AbilityDef::activated(
        "Exile three cards from your graveyard: This creature gains flying until end of turn.",
        &[AbilityCostDef::ExileCardsFromGraveyard {
            object: ObjectPredicateDef::Any,
            count: 3,
        }],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::add_ability(&abilities::flying()),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    ),
];

// MH3 199 — Psychic Frog
pub(in crate::card::sets) static PSYCHIC_FROG: CardRecord = CardRecord::new(
    cards::PSYCHIC_FROG,
    "Psychic Frog",
    CardArt::new("68924203-c3d9-41ce-8ca8-c6dd491eb3ca", "Pete Venters"),
    CardSet::ModernHorizons3,
    // Two mana that turns a full hand into a big evasive body and a full
    // graveyard into the evasion, and draws a card every time it connects.
    CardRules::new_creature(mana_cost!("{U}{B}"), &["Frog"], 1, 2)
        .with_abilities(&PSYCHIC_FROG_ABILITIES),
);

/// A basic one, not merely a card with the type: the tri-fetch cycle names
/// three basics and finds nothing else, which is what separates it from the
/// fetchlands that read "a Mountain or Plains card".
static A_BASIC_TRIOME_LAND: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::Supertype(CardSupertype::Basic),
    ObjectPredicateDef::HasAnyBasicLandType(&[
        BasicLandType::Forest,
        BasicLandType::Island,
        BasicLandType::Mountain,
    ]),
]);

static LANDSCAPE_FETCH_COST: [AbilityCostDef; 2] =
    [AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource];

static BOUNTIFUL_LANDSCAPE_ABILITIES: [AbilityDef; 3] = [
    abilities::tap_for(ManaColor::Colorless),
    AbilityDef::activated(
        "{T}, Sacrifice this land: Search your library for a basic Forest, Island, or Mountain \
         card, put it onto the battlefield tapped, then shuffle.",
        &LANDSCAPE_FETCH_COST,
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: A_BASIC_TRIOME_LAND,
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: true,
            binding: None,
            then: None,
        },
    ),
    abilities::cycling(
        "Cycling {G}{U}{R} ({G}{U}{R}, Discard this card: Draw a card.)",
        mana_cost!("{G}{U}{R}"),
    ),
];

// MH3 217 — Bountiful Landscape
pub(in crate::card::sets) static BOUNTIFUL_LANDSCAPE: CardRecord = CardRecord::new(
    cards::BOUNTIFUL_LANDSCAPE,
    "Bountiful Landscape",
    CardArt::new("b277752b-430a-4f09-8a98-b72f813dd52e", "Mark Poole"),
    CardSet::ModernHorizons3,
    // A land that taps for nothing useful and fetches a tapped basic, which
    // is worth a slot only because it is also a cycling card and because
    // what it finds is a land drop somebody else paid for.
    CardRules::new_land(&[]).with_abilities(&BOUNTIFUL_LANDSCAPE_ABILITIES),
);

// MH3 237 — Ajani, Nacatl Pariah
pub(in crate::card::sets) static AJANI_NACATL_PARIAH: CardRecord = CardRecord::new(
    cards::AJANI_NACATL_PARIAH,
    "Ajani, Nacatl Pariah",
    CardArt::new("0d16e8e0-31b2-4389-afd6-783c501f6fa0", "Chris Rallis"),
    CardSet::ModernHorizons3,
    ajani_nacatl_pariah_rules(),
)
.with_composition(ajani_composition);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &OCELOT_PRIDE,
    &STATIC_PRISON,
    &EMPEROR_OF_BONES,
    &AMPED_RAPTOR,
    &COLOSSAL_DREADMASK,
    &SOWING_MYCOSPAWN,
    &PHLAGE_TITAN_OF_FIRES_FURY,
    &PSYCHIC_FROG,
    &BOUNTIFUL_LANDSCAPE,
    &AJANI_NACATL_PARIAH,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
