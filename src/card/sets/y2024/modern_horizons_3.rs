//! Modern Horizons 3 cards cataloged as attachment edge cases.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AddManaEffectDef, AlternativeCastKindDef, AppliedEffectDef, AppliedRuleDef,
    AttackEventMatcherDef, BasicLandType, BattlefieldEntryModificationDef, CardArt,
    CardChoiceSourceDef, CardRules, CardSet, CardSupertype, CardType, CharacteristicOperationDef,
    ChoiceVisibilityDef, ChooseDef, ComparisonDef, ControlDurationDef, CopyExceptionsDef,
    CounterKind, CreatureTypeSetDef, DrawEventMatcherDef, EffectDef, EffectPaymentCostDef,
    EffectPaymentDef, EffectRecipientDef, EmblemCharacteristics, ExiledCastPermissionDef,
    HalvedValueDef, InstalledTriggerDef, InstalledTriggerLifetimeDef, ManaColor, ManaCost,
    ManaSpendEffectDef, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef,
    ObjectSetDef, PayOrDef, PileExileDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    ReplacementEffectDef, ResolvedEffectDurationDef, RoundingDef, SetOperationDef,
    SimultaneousChooseDef, SpellAdditionalCostCountDef, SpellAdditionalCostDef, SpendModeDef,
    SumValueDef, TargetConditionDef, TokenCountersDef, TopCardSelectionDef, TriggerConditionDef,
    TriggerEventDef, TurnStepDef, ValueComparisonDef, ValueDef, ZoneKind, ZonePickDef,
    ZonePlacement, abilities, tokens,
};
use crate::ids::{ObjectBindingIndex, ObjectSetBindingIndex};
use crate::{TargetIndex, mana_cost};

static DEVOURER_OPENING_LOOK: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(4),
    object: None,
    minimum: 0,
    maximum: 1,
    select_all_matching: false,
    select_one_of_each_type: false,
    reveal_inspected: false,
    reveal_selected: false,
    counted: None,
    selected_zone: ZoneKind::Library,
    selected_placement: ZonePlacement::Top,
    selected_hidden: false,
    selected_linked_to_source: false,
    selected_face_down: None,
    rest_zone: ZoneKind::Exile,
    rest_placement: ZonePlacement::Top,
    rest_random_order: false,
    rest_counters: None,
    selected_order_follows_choice: false,
    then: None,
};

static DEVOURER_OPENING_TRIGGER: AbilityDef = AbilityDef::triggered(
    "At the beginning of your first upkeep, look at the top four cards of your library. You may put one of those cards back on top of your library. Exile the rest.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::LookAtTopAndSelect {
        player: EffectRecipientDef::Controller,
        looker: EffectRecipientDef::Controller,
        selection: &DEVOURER_OPENING_LOOK,
    },
);

// MH3 2 — Devourer of Destiny
// Audit: partial — The opening-hand action is declarative; its cast trigger needs a predicate for permanents with one or more colors.
pub(in crate::card::sets) static DEVOURER_OF_DESTINY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("560debcd-feb4-4534-991e-a7aa1cca2409"),
    "Devourer of Destiny",
    CardArt::new("560debcd-feb4-4534-991e-a7aa1cca2409", "Raph Lomotan"),
    CardSet::ModernHorizons3,
    CardRules::new_creature(mana_cost!("{5}{C}{C}"), &["Eldrazi"], 6, 6).with_abilities(&[
        AbilityDef::opening_hand_reveal(
            "You may reveal this card from your opening hand. If you do, at the beginning of your first upkeep, look at the top four cards of your library. You may put one of those cards back on top of your library. Exile the rest.",
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&DEVOURER_OPENING_TRIGGER)),
        ),
        AbilityDef::not_implemented("When you cast this spell, exile target permanent that's one or more colors.", "Needs a target predicate for a permanent whose color set is nonempty."),
    ]),
);

static LANDSCAPE_FETCH_COST: [AbilityCostDef; 2] =
    [AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource];

/// The Landscape cycle: a land that taps for nothing useful, sacrifices
/// itself for one of three tapped basics, and is a cycling card when the
/// board does not need a land at all. Each member differs only in which
/// three basics it names and what its cycling costs.
const fn landscape_abilities(
    fetch_text: &'static str,
    basics: ObjectPredicateDef,
    cycling_text: &'static str,
    cycling_cost: ManaCost,
) -> [AbilityDef; 3] {
    [
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            fetch_text,
            &LANDSCAPE_FETCH_COST,
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: basics,
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: true,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
        abilities::cycling(cycling_text, cycling_cost),
    ]
}

// MH3 18 — Aerie Auxiliary
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AERIE_AUXILIARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5e4c134b-a416-467e-a158-def84c92c6af"),
    "Aerie Auxiliary",
    crate::card::CardArt::new("5e4c134b-a416-467e-a158-def84c92c6af", "Donato Giancola"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 22 — Dog Umbra
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DOG_UMBRA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8d4ba710-eddb-40ca-b2fe-0e4e778aab9c"),
    "Dog Umbra",
    crate::card::CardArt::new("8d4ba710-eddb-40ca-b2fe-0e4e778aab9c", "Brian Valeza"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 34 — Mandibular Kite
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MANDIBULAR_KITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6b922f71-18e6-4a74-b792-d477d4a1deca"),
    "Mandibular Kite",
    crate::card::CardArt::new("6b922f71-18e6-4a74-b792-d477d4a1deca", "Bruno Biazotto"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 38 — Ocelot Pride
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

static OCELOT_DOUBLES_THEM: EffectDef =
    EffectDef::create_token_from_copy(&crate::card::TokenCopyDef {
        object: &EffectRecipientDef::objects(ObjectSetDef::Query(YOUR_NEW_TOKENS)),
        exceptions: CopyExceptionsDef::NONE,
    });

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

pub(in crate::card::sets) static OCELOT_PRIDE: CardRecord = CardRecord::new_with_legacy_id(
    2225,
    "Ocelot Pride",
    CardArt::new("89cf6f57-230f-497e-a14e-ad1e8737fd42", "Chris Seaman"),
    CardSet::ModernHorizons3,
    // Its own lifelink turns the trigger on, and once the board is wide
    // enough to ascend every Cat it ever made comes back doubled.
    CardRules::new_creature(mana_cost!("{W}"), &["Cat"], 1, 1)
        .with_abilities(&OCELOT_PRIDE_ABILITIES),
);

// MH3 40 — Phelia, Exuberant Shepherd
/// "If it entered under your control": what Phelia gives back goes to its
/// owner, so who owned it is the whole of the question. Asked before the
/// return rather than after, because by then there is no exile left to ask
/// about.
static PHELIA_TOOK_YOUR_OWN: TriggerConditionDef = TriggerConditionDef::LinkedExilesMatch {
    object: ObjectPredicateDef::OwnedBy(PlayerRelation::You),
};

static PHELIA_GROWS: EffectDef = EffectDef::AddCounters {
    object: EffectRecipientDef::Source,
    kind: CounterKind::PlusOnePlusOne,
    amount: ValueDef::Constant(1),
};

static PHELIA_RETURNS_IT: [EffectDef; 2] = [
    EffectDef::IfCondition {
        condition: &PHELIA_TOOK_YOUR_OWN,
        then: &PHELIA_GROWS,
    },
    EffectDef::ReturnLinkedExiles {
        object: ObjectPredicateDef::Any,
        zone: ZoneKind::Battlefield,
        grant: None,
        counters: None,
        arrival_effect: None,
        transformed: false,
        controller: None,
    },
];

static PHELIA_END_STEP: AbilityDef = AbilityDef::triggered(
    "At the beginning of the next end step, return that card to the battlefield under its \
     owner's control. If it entered under your control, put a +1/+1 counter on this creature.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::End,
        player: PlayerRelation::Any,
    },
    EffectDef::Sequence(&PHELIA_RETURNS_IT),
);

/// "Up to one other target nonland permanent", which is what makes her a
/// blink as happily as a removal spell: the thing she takes may be yours.
static ANOTHER_NONLAND_PERMANENT: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
    1,
)];

static PHELIA_EXILE: [EffectDef; 2] = [
    EffectDef::ExileLinkedToSource {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        face_down: false,
        then: None,
    },
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&PHELIA_END_STEP)),
];

pub(in crate::card::sets) static PHELIA_EXUBERANT_SHEPHERD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("55707746-da6e-46e5-a5ca-7ac843fdc38e"),
    "Phelia, Exuberant Shepherd",
    CardArt::new("55707746-da6e-46e5-a5ca-7ac843fdc38e", "Rudy Siswanto"),
    CardSet::ModernHorizons3,
    // Two mana that answers something for a turn or blinks something of
    // yours forever, and grows every time it does the second.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Dog"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flash(),
            AbilityDef::triggered_with_targets(
                "Whenever this creature attacks, exile up to one other target nonland permanent. \
                 At the beginning of the next end step, return that card to the battlefield under \
                 its owner's control. If it entered under your control, put a +1/+1 counter on \
                 this creature.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                &ANOTHER_NONLAND_PERMANENT,
                EffectDef::Sequence(&PHELIA_EXILE),
            ),
        ]),
);

// MH3 44 — Static Prison
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
        face_down: false,
        then: None,
    },
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&PRISON_RETURNS_IT)),
    // The energy arrives with the exile rather than paying for it: the first
    // upkeep tax is already covered, and the second is not.
    EffectDef::AddPlayerCounters {
        recipient: EffectRecipientDef::Controller,
        kind: CounterKind::Energy,
        amount: ValueDef::Constant(2),
    },
];

static PRISON_SACRIFICE: EffectDef = EffectDef::Sacrifice {
    object: EffectRecipientDef::Source,
};

pub(in crate::card::sets) static STATIC_PRISON: CardRecord = CardRecord::new_with_legacy_id(
    2194,
    "Static Prison",
    CardArt::new("dd16222e-349c-4a2b-a7c8-8eb35a8ab332", "Jason A. Engle"),
    CardSet::ModernHorizons3,
    // One white answers anything, and the two energy it comes with buy two
    // more turns of holding it. After that the prison opens.
    CardRules::new_enchantment(mana_cost!("{W}")).with_abilities(&[
        abilities::enters_trigger_with_targets("When this enchantment enters, exile target nonland permanent an opponent controls until this enchantment leaves the battlefield. You get {E}{E} (two energy counters).", &PRISON_TARGET, EffectDef::Sequence(&PRISON_ENTERS)),
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

// MH3 45 — Thraben Charm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THRABEN_CHARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dd28a646-f38f-4cdf-948c-969cd979e5e6"),
    "Thraben Charm",
    crate::card::CardArt::new(
        "dd28a646-f38f-4cdf-948c-969cd979e5e6",
        "Carlos Palma Cruchaga",
    ),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 53 — Brainsurge
static BRAINSURGE_HAND: [CardChoiceSourceDef; 1] = [CardChoiceSourceDef::Zone(ZoneKind::Hand)];

/// Brainstorm's two steps for one more card. The arrangement is the order
/// the two are named in: each is placed on top of the last, so the card
/// named second is the one drawn first.
static BRAINSURGE_STEPS: [EffectDef; 2] = [
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(4),
    },
    EffectDef::ChooseCards {
        player: EffectRecipientDef::Controller,
        sources: &BRAINSURGE_HAND,
        object: ObjectPredicateDef::Any,
        minimum: 2,
        maximum: 2,
        reveal: false,
        destination: ZoneKind::Library,
        placement: ZonePlacement::Top,
        arrival_effect: None,
    },
];

pub(in crate::card::sets) static BRAINSURGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ed48f805-b57c-4d7f-a3c2-d16ae71bce2d"),
    "Brainsurge",
    CardArt::new("ed48f805-b57c-4d7f-a3c2-d16ae71bce2d", "Liiga Smilshkalne"),
    CardSet::ModernHorizons3,
    // Two more cards than Brainstorm for two more mana, and the same catch:
    // what it really does is fix a hand, and without a shuffle the two that
    // go back are two draws you have already spent.
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell(
        "Draw four cards, then put two cards from your hand on top of your library in any order.",
        EffectDef::Sequence(&BRAINSURGE_STEPS),
    )),
);

// MH3 69 — Serum Visionary
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SERUM_VISIONARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("08a587f5-5910-405e-8982-c889dbbc7f98"),
    "Serum Visionary",
    crate::card::CardArt::new("08a587f5-5910-405e-8982-c889dbbc7f98", "Warren Mahy"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 80 — Accursed Marauder
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ACCURSED_MARAUDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("44a63029-1fb2-4fdc-bca9-0a530c7b42d9"),
    "Accursed Marauder",
    crate::card::CardArt::new("5da14d86-0780-4821-a799-96f64b377df4", "Paolo Parente"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 90 — Emperor of Bones
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

static EMPEROR_HASTE: AbilityDef = abilities::haste();
static EMPEROR_ARRIVAL_EFFECT: AppliedEffectDef = AppliedEffectDef::add_ability(&EMPEROR_HASTE);

static EMPEROR_REANIMATES: EffectDef = EffectDef::PutOntoBattlefieldThen {
    object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    binding: EMPEROR_ARRIVAL,
    counters: Some(TokenCountersDef {
        kind: CounterKind::Finality,
        amount: ValueDef::Constant(1),
    }),
    arrival_effect: Some(&EMPEROR_ARRIVAL_EFFECT),
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
            face_down: false,
            then: None,
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

pub(in crate::card::sets) static EMPEROR_OF_BONES: CardRecord = CardRecord::new_with_legacy_id(
    2269,
    "Emperor of Bones",
    CardArt::new("df9d9075-2d1e-4848-b661-816d539e05eb", "Josh Hass"),
    CardSet::ModernHorizons3,
    // Two mana that eats a graveyard one card a turn and then rents the best
    // of them back for an attack, which is what makes the adapt cost worth
    // paying twice.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Skeleton", "Noble"], 2, 2)
        .with_abilities(&EMPEROR_OF_BONES_ABILITIES),
);

// MH3 103 — Nethergoyf
/// The escape cost counts card types rather than cards: one Artifact
/// Creature Land pays three quarters of it by itself, which is why the deck
/// playing this is the one with a graveyard full of odd things.
static NETHERGOYF_ESCAPE_COST: SpellAdditionalCostDef =
    SpellAdditionalCostDef::new(ObjectPredicateDef::Any, ZoneKind::Graveyard, 0)
        .counted(SpellAdditionalCostCountDef::CardTypesAtLeast(4))
        .spent(SpendModeDef::Exile);

/// "That number plus 1", counted over your own graveyard alone.
static NETHERGOYF_TOUGHNESS: SumValueDef = SumValueDef::new(
    ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
    ValueDef::Constant(1),
);

static NETHERGOYF_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::static_ability(
        "This creature\'s power is equal to the number of card types among cards in your \
         graveyard and its toughness is equal to that number plus 1.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            // Each half is its own amount: the toughness is the count plus
            // one rather than the count applied to a printed body -- the way
            // Barrowgoyf reads it.
            effect: AppliedEffectDef::define_power_toughness(
                ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                ValueDef::Sum(&NETHERGOYF_TOUGHNESS),
            ),
        },
    ),
    AbilityDef::alternative_cast(
        mana_cost!("{2}{B}"),
        AlternativeCastKindDef::Escape,
        Some(
            "Escape—{2}{B}, Exile any number of other cards from your graveyard with four or \
             more card types among them. (You may cast this card from your graveyard for its \
             escape cost.)",
        ),
        EffectDef::None,
    )
    .with_alternative_additional_cost(&NETHERGOYF_ESCAPE_COST),
];

pub(in crate::card::sets) static NETHERGOYF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3ee3945e-5089-4751-b7b3-5961c39d2a33"),
    "Nethergoyf",
    CardArt::new("3ee3945e-5089-4751-b7b3-5961c39d2a33", "Xavier Ribeiro"),
    CardSet::ModernHorizons3,
    // One mana for whatever the graveyard has made of it, and the graveyard
    // pays a second time to buy it back.
    CardRules::new_creature(mana_cost!("{B}"), &["Lhurgoyf"], 0, 1)
        .with_abilities(&NETHERGOYF_ABILITIES),
);

// MH3 106 — Retrofitted Transmogrant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RETROFITTED_TRANSMOGRANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("12c1b83d-710b-4680-855a-02ba1f72abf0"),
    "Retrofitted Transmogrant",
    crate::card::CardArt::new("12c1b83d-710b-4680-855a-02ba1f72abf0", "Kekai Kotaki"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 108 — Scurrilous Sentry
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCURRILOUS_SENTRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29e2805f-59fa-4a6d-97bc-266191b2aa8d"),
    "Scurrilous Sentry",
    crate::card::CardArt::new("29e2805f-59fa-4a6d-97bc-266191b2aa8d", "Leonardo Santanna"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 111 — Wither and Bloom
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WITHER_AND_BLOOM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("95c2390f-71f1-4e42-83da-d603ca86a8d0"),
    "Wither and Bloom",
    crate::card::CardArt::new(
        "95c2390f-71f1-4e42-83da-d603ca86a8d0",
        "Richard Kane Ferguson",
    ),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 114 — Amped Raptor
/// A land is what the exile walks past; the first thing that is not one is
/// what you get to keep.
static A_NONLAND_CARD: ObjectPredicateDef =
    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land));

static RAPTOR_DIGS: EffectDef = EffectDef::ExileFromTopUntil {
    player: EffectRecipientDef::Controller,
    object: A_NONLAND_CARD,
    permission: ExiledCastPermissionDef::EnergyEqualToManaValue,
};

/// "Then if you cast it from your hand" is part of the effect rather than an
/// intervening-if: a Raptor put onto the battlefield gets the energy and
/// nothing else.
static RAPTOR_ENTERS: [EffectDef; 2] = [
    EffectDef::AddPlayerCounters {
        recipient: EffectRecipientDef::Controller,
        kind: CounterKind::Energy,
        amount: ValueDef::Constant(2),
    },
    EffectDef::IfCondition {
        condition: &TriggerConditionDef::SourceCastFrom(ZoneKind::Hand),
        then: &RAPTOR_DIGS,
    },
];

static AMPED_RAPTOR_ABILITIES: [AbilityDef; 2] = [
    abilities::first_strike(),
    abilities::enters_trigger(
        "When this creature enters, you get {E}{E} (two energy counters). Then if you cast it \
         from your hand, exile cards from the top of your library until you exile a nonland card. \
         You may cast that card by paying an amount of {E} equal to its mana value rather than \
         paying its mana cost.",
        EffectDef::Sequence(&RAPTOR_ENTERS),
    ),
];

pub(in crate::card::sets) static AMPED_RAPTOR: CardRecord = CardRecord::new_with_legacy_id(
    2221,
    "Amped Raptor",
    CardArt::new("1ac0e78b-0fdd-44f9-8b7b-c4f28a32782e", "Alex Konstad"),
    CardSet::ModernHorizons3,
    // Two mana for a 2/1 first striker and a free spell off the top, as long
    // as the top of the deck is cheap enough for two energy to cover.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Dinosaur"], 2, 1)
        .with_abilities(&AMPED_RAPTOR_ABILITIES),
);

// MH3 116 — Detective's Phoenix
/// Collect evidence 6 (CR 701.58a): cards out of your own graveyard whose
/// mana values add up to six, however many that takes.
static COLLECT_EVIDENCE_SIX: SpellAdditionalCostDef =
    SpellAdditionalCostDef::new(ObjectPredicateDef::Any, ZoneKind::Graveyard, 0)
        .counted(SpellAdditionalCostCountDef::TotalManaValueAtLeast(6))
        .spent(SpendModeDef::Exile);

static PHOENIX_FLYING: AbilityDef = abilities::flying();

static PHOENIX_HASTE: AbilityDef = abilities::haste();

static PHOENIX_GRANTS: [AppliedEffectDef; 3] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
    AppliedEffectDef::add_ability(&PHOENIX_FLYING),
    AppliedEffectDef::add_ability(&PHOENIX_HASTE),
];

static PHOENIX_ABILITIES: [AbilityDef; 5] = [
    AbilityDef::alternative_cast_with_targets(
        mana_cost!("{R}"),
        AlternativeCastKindDef::Bestow,
        Some(
            "Bestow—{R}, Collect evidence 6. (To pay this bestow cost, pay {R} and exile cards \
             with total mana value 6 or greater from your graveyard.)",
        ),
        &abilities::ENCHANT_CREATURE_TARGET,
        EffectDef::Attach {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )
    .with_alternative_additional_cost(&COLLECT_EVIDENCE_SIX)
    .with_alternative_from_graveyard(),
    abilities::flying(),
    abilities::haste(),
    // Only while it is an Aura: unattached, the recipient names nothing and
    // the clause does nothing, which is exactly CR 702.103d.
    AbilityDef::static_ability(
        "Enchanted creature gets +2/+2 and has flying and haste.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::AttachedPermanent,
            effect: AppliedEffectDef::Composite(&PHOENIX_GRANTS),
        },
    ),
    AbilityDef::static_ability(
        "You may cast this card from your graveyard using its bestow ability.",
        EffectDef::None,
    )
    .with_source_zones(&[ZoneKind::Graveyard])
    .with_coverage(AbilityCoverageDef::explained_complete(
        "The permission is carried by the bestow clause, which this card marks as castable from \
         its owner's graveyard.",
    )),
];

pub(in crate::card::sets) static DETECTIVES_PHOENIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e2a01edd-dbc0-4ed4-b827-9b608290e9a1"),
    "Detective's Phoenix",
    CardArt::new(
        "e2a01edd-dbc0-4ed4-b827-9b608290e9a1",
        "Deruchenko Alexander",
    ),
    CardSet::ModernHorizons3,
    // A three-mana hasty flier that never really dies: once the graveyard is
    // six mana deep it comes back out of it for {R}, as an Aura, and comes
    // back again as a creature when whatever it was wearing is gone.
    CardRules::new_enchantment_creature(mana_cost!("{2}{R}"), &["Phoenix"], 2, 2)
        .with_abilities(&PHOENIX_ABILITIES),
);

// MH3 122 — Galvanic Discharge
static A_CREATURE_OR_PLANESWALKER_DISCHARGE: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::HasType(CardType::Planeswalker),
        ]),
    )];

/// "That much damage": the amount the payment settled, which is what makes
/// the three energy it hands out into three damage the turn it is cast and
/// more than that on a board that has been banking it.
static DISCHARGE_DAMAGE: EffectDef = EffectDef::DealDamage {
    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    amount: ValueDef::PaidAmount,
};

static DISCHARGE_EFFECTS: [EffectDef; 2] = [
    EffectDef::AddPlayerCounters {
        recipient: EffectRecipientDef::Controller,
        kind: CounterKind::Energy,
        amount: ValueDef::Constant(3),
    },
    EffectDef::PayOr(PayOrDef::optional(
        EffectPaymentDef {
            payer: PlayerSetDef::Related(PlayerRelation::You),
            cost: EffectPaymentCostDef::ChosenEnergy,
        },
        &DISCHARGE_DAMAGE,
    )),
];

pub(in crate::card::sets) static GALVANIC_DISCHARGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("32aa6e33-221f-414c-9b51-850d97a7e051"),
    "Galvanic Discharge",
    CardArt::new("32aa6e33-221f-414c-9b51-850d97a7e051", "Zoltan Boros"),
    CardSet::ModernHorizons3,
    // One mana that kills a three-toughness creature and leaves the energy
    // behind when it does not need all of it.
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Choose target creature or planeswalker. You get {E}{E}{E} (three energy counters), then \
         you may pay any amount of {E}. Galvanic Discharge deals that much damage to that \
         permanent.",
        &A_CREATURE_OR_PLANESWALKER_DISCHARGE,
        EffectDef::Sequence(&DISCHARGE_EFFECTS),
    )),
);

// MH3 128 — Molten Gatekeeper
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOLTEN_GATEKEEPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9f5ba065-2806-4e99-a330-168cfe76250f"),
    "Molten Gatekeeper",
    crate::card::CardArt::new("9f5ba065-2806-4e99-a330-168cfe76250f", "Joe Slucher"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 145 — Basking Broodscale
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BASKING_BROODSCALE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5feba5d6-99a6-4e9b-8a7d-90d955868fc3"),
    "Basking Broodscale",
    crate::card::CardArt::new("5feba5d6-99a6-4e9b-8a7d-90d955868fc3", "Caio Monteiro"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 148 — Colossal Dreadmask
pub(in crate::card::sets) static COLOSSAL_DREADMASK: CardRecord = CardRecord::new_with_legacy_id(
    1703,
    "Colossal Dreadmask",
    CardArt::new("98164430-64c1-465f-b786-45753c965f44", "Caio Monteiro"),
    CardSet::ModernHorizons3,
    CardRules::new_artifact(mana_cost!("{4}{G}{G}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::living_weapon(),
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

// MH3 150 — Eldrazi Repurposer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ELDRAZI_REPURPOSER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("37f79ba7-7b65-4387-b498-f770816ce8dd"),
    "Eldrazi Repurposer",
    crate::card::CardArt::new("37f79ba7-7b65-4387-b498-f770816ce8dd", "Daren Bader"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 151 — Evolution Witness
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EVOLUTION_WITNESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4b4ecfa6-5e38-4c0a-91e2-f93cb492f374"),
    "Evolution Witness",
    crate::card::CardArt::new("4d89283e-9783-4006-9294-4ae0473d2ce6", "Nereida"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

/// Ferocious: a creature with power four or greater, which the Fanatic is
/// not, so something else has to be there.
static A_CREATURE_WITH_POWER_FOUR_OR_GREATER: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::PowerAtLeast(4),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

static FEROCIOUS: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: A_CREATURE_WITH_POWER_FOUR_OR_GREATER,
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 1,
};

static FANATIC_TAP: [AbilityCostDef; 1] = [AbilityCostDef::TapSource];

static FANATIC_OF_RHONAS_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::activated_mana(
        "{T}: Add {G}.",
        &FANATIC_TAP,
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
    ),
    AbilityDef::activated_mana_if(
        "Ferocious — {T}: Add {G}{G}{G}{G}. Activate only if you control a creature with power 4 \
         or greater.",
        &FANATIC_TAP,
        &FEROCIOUS,
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green).with_amount(4)),
    ),
    abilities::eternalize(
        "Eternalize {2}{G}{G} ({2}{G}{G}, Exile this card from your graveyard: Create a token \
         that's a copy of it, except it's a 4/4 black Zombie Snake Druid with no mana cost. \
         Eternalize only as a sorcery.)",
        mana_cost!("{2}{G}{G}"),
    ),
];

// MH3 152 — Fanatic of Rhonas
pub(in crate::card::sets) static FANATIC_OF_RHONAS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1f9fb33a-3b39-4aff-93b8-aedafe0ea694"),
    "Fanatic of Rhonas",
    CardArt::new("1f9fb33a-3b39-4aff-93b8-aedafe0ea694", "Scott Murphy"),
    CardSet::ModernHorizons3,
    // Two mana for a 1/4 that taps for one, and for four the moment anything
    // large is beside it -- and a 4/4 out of the graveyard afterwards.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Snake", "Druid"], 1, 4)
        .with_abilities(&FANATIC_OF_RHONAS_ABILITIES),
);

// MH3 157 — Horrific Assault
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HORRIFIC_ASSAULT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cfa6ed13-7bba-40c0-8e0e-4ffd3cea6241"),
    "Horrific Assault",
    crate::card::CardArt::new("cfa6ed13-7bba-40c0-8e0e-4ffd3cea6241", "Justine Jones"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 161 — Malevolent Rumble
static RUMBLE_SPAWN_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated_mana(
    "Sacrifice this token: Add {C}.",
    &RUMBLE_SPAWN_COST,
    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
)];

static RUMBLE_SPAWN_COST: [AbilityCostDef; 1] = [AbilityCostDef::SacrificeSource];

/// Every card type a permanent card can have. A planeswalker is one too,
/// and unlike the older cards written this way this one is new enough to
/// mean it.
static A_PERMANENT_CARD_RUMBLE: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::HasType(CardType::Enchantment),
    ObjectPredicateDef::HasType(CardType::Land),
    ObjectPredicateDef::HasType(CardType::Planeswalker),
]);

/// "A permanent card from among them": taking nothing is a legal answer,
/// and everything not taken is buried whether or not it could have been.
static RUMBLE_DIG: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(4),
    object: Some(A_PERMANENT_CARD_RUMBLE),
    minimum: 0,
    maximum: 1,
    select_all_matching: false,
    select_one_of_each_type: false,
    reveal_inspected: true,
    reveal_selected: true,
    counted: None,
    selected_zone: ZoneKind::Hand,
    selected_placement: ZonePlacement::Top,
    rest_zone: ZoneKind::Graveyard,
    rest_placement: ZonePlacement::Top,
    rest_random_order: false,
    rest_counters: None,
    selected_order_follows_choice: false,
    then: None,
    selected_hidden: false,
    selected_linked_to_source: false,
    selected_face_down: None,
};

static RUMBLE_EFFECTS: [EffectDef; 2] = [
    EffectDef::LookAtTopAndSelect {
        player: EffectRecipientDef::Controller,
        looker: EffectRecipientDef::Controller,
        selection: &RUMBLE_DIG,
    },
    EffectDef::create_creature_token(&["Eldrazi", "Spawn"], &[], 0, 1)
        .with_abilities(&RUMBLE_SPAWN_ABILITIES),
];

pub(in crate::card::sets) static MALEVOLENT_RUMBLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a178cfe8-f9fa-4255-88d0-54a0bed079f5"),
    "Malevolent Rumble",
    CardArt::new(
        "a178cfe8-f9fa-4255-88d0-54a0bed079f5",
        "Néstor Ossandón Leal",
    ),
    CardSet::ModernHorizons3,
    // Two mana that finds a permanent, fills the graveyard with the three
    // it did not want, and leaves behind the mana that makes the next spell
    // a turn early.
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell(
        "Reveal the top four cards of your library. You may put a permanent card from among them \
         into your hand. Put the rest into your graveyard. Create a 0/1 colorless Eldrazi Spawn \
         creature token with \"Sacrifice this token: Add {C}.\"",
        EffectDef::Sequence(&RUMBLE_EFFECTS),
    )),
);

// MH3 164 — Nyxborn Hydra
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NYXBORN_HYDRA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("902a969e-9f22-4e92-93eb-9d4536ca82e5"),
    "Nyxborn Hydra",
    crate::card::CardArt::new(
        "902a969e-9f22-4e92-93eb-9d4536ca82e5",
        "Vincent Christiaens",
    ),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 169 — Six
/// Retrace's own cost: the card's mana cost, plus a land out of your hand.
static SIX_DISCARDS_A_LAND: SpellAdditionalCostDef = SpellAdditionalCostDef::new(
    ObjectPredicateDef::HasType(CardType::Land),
    ZoneKind::Hand,
    1,
);

static SIX_RETRACE: AbilityDef = AbilityDef::alternative_cast_for_card_mana_cost(
    AlternativeCastKindDef::Retrace,
    Some(
        "Retrace (You may cast this card from your graveyard by discarding a land card in \
         addition to paying its other costs.)",
    ),
    EffectDef::None,
)
.with_alternative_additional_cost(&SIX_DISCARDS_A_LAND);

/// "Nonland permanent cards": what the grant reaches is every card that
/// would become a permanent, which is the whole of what a Treefolk deck
/// throws away.
static A_NONLAND_PERMANENT_CARD: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::HasType(CardType::Enchantment),
    ObjectPredicateDef::HasType(CardType::Planeswalker),
]);

static SIX_GRANTS_RETRACE: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
    effect: AppliedEffectDef::Rule(AppliedRuleDef::GrantsAlternativeCastFromGraveyard {
        object: A_NONLAND_PERMANENT_CARD,
        ability: &SIX_RETRACE,
    }),
};

/// "During your turn" is a gate on the permission rather than on what it
/// names: on their turn the cards in your graveyard have nothing.
static DURING_YOUR_TURN: TriggerConditionDef =
    TriggerConditionDef::ActivePlayer(PlayerRelation::You);

/// "From among them" is what the mill just put there, not what the graveyard
/// already held -- and only a land among those.
static A_MILLED_LAND_CARD: ObjectSetDef = ObjectSetDef::MatchingBinding {
    binding: ObjectSetBindingIndex::PRIMARY,
    object: ObjectPredicateDef::HasType(CardType::Land),
};

/// Where the taken land is saved, kept apart from the milled pile so that
/// "them" and "the one you took" stay two different sets.
static SIX_TAKEN_LAND: ObjectSetBindingIndex = ObjectSetBindingIndex::new(1);

static SIX_TAKES_IT: EffectDef = EffectDef::MoveToZone {
    counters: None,
    object: EffectRecipientDef::objects(ObjectSetDef::Binding(SIX_TAKEN_LAND)),
    from: None,
    zone: ZoneKind::Hand,
    placement: ZonePlacement::Top,
    controller: None,
    arrival_effect: None,
    attachment: None,
    tapped: false,
};

/// A minimum of zero is the "you may": milling three and taking nothing is a
/// legal answer, and a pile with no land in it never asks.
static SIX_CHOOSES: EffectDef = EffectDef::Choose(ChooseDef {
    binding: ObjectChoiceBindingDef::Objects(SIX_TAKEN_LAND),
    unchosen: None,
    chooser: PlayerRefDef::EffectController,
    candidates: A_MILLED_LAND_CARD,
    exclude: None,
    minimum: 0,
    maximum: 1,
    visibility: ChoiceVisibilityDef::Public,
    then: &SIX_TAKES_IT,
});

static SIX_MILLS: EffectDef = EffectDef::Mill {
    player: EffectRecipientDef::Controller,
    amount: ValueDef::Constant(3),
    binding: Some(ObjectSetBindingIndex::PRIMARY),
    then: Some(&SIX_CHOOSES),
};

pub(in crate::card::sets) static SIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f9246b68-580f-4f53-883d-7900880e4b0d"),
    "Six",
    CardArt::new("f9246b68-580f-4f53-883d-7900880e4b0d", "Andrew Mar"),
    CardSet::ModernHorizons3,
    // A blocker that fills the graveyard and then plays out of it: every
    // land the attack finds is another permanent cast back from the pile.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Treefolk"], 2, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::reach(),
            AbilityDef::triggered(
                "Whenever Six attacks, mill three cards. You may put a land card from among them \
                 into your hand.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                SIX_MILLS,
            ),
            AbilityDef::static_ability(
                "During your turn, nonland permanent cards in your graveyard have retrace. (You \
                 may cast permanent cards from your graveyard by discarding a land card in \
                 addition to paying their other costs.)",
                EffectDef::IfCondition {
                    condition: &DURING_YOUR_TURN,
                    then: &SIX_GRANTS_RETRACE,
                },
            ),
        ]),
);

// MH3 170 — Sowing Mycospawn
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
            attachment: None,
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
            counters: None,
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            from: None,
            zone: ZoneKind::Exile,
            controller: None,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
            tapped: false,
        },
    ),
];

pub(in crate::card::sets) static SOWING_MYCOSPAWN: CardRecord = CardRecord::new_with_legacy_id(
    2176,
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

// MH3 171 — Springheart Nantuko
static A_LAND_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Land),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

/// "If this permanent is attached to a creature you control": read before
/// the offer, because a Nantuko that is a creature rather than an Aura has
/// nothing to copy and should not be asked to pay for one.
static NANTUKO_IS_WEARING_A_CREATURE: TriggerConditionDef =
    TriggerConditionDef::AttachedPermanentMatches {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
        ]),
    };

/// The whole point of bestowing it: every land is another copy of whatever
/// it is wearing.
static NANTUKO_COPIES_ITS_HOST: EffectDef =
    EffectDef::create_token_from_copy(&crate::card::TokenCopyDef {
        object: &EffectRecipientDef::AttachedPermanent,
        exceptions: CopyExceptionsDef::NONE,
    });

/// "If you didn't create a token this way": declining, being unable to pay,
/// and not being attached at all are the same answer, and each leaves an
/// Insect behind.
static NANTUKO_MAKES_AN_INSECT: EffectDef =
    EffectDef::create_creature_token(&["Insect"], &[ManaColor::Green], 1, 1);

static NANTUKO_LANDFALL: EffectDef = EffectDef::PayOr(
    PayOrDef::optional_or(
        EffectPaymentDef::mana(
            PlayerSetDef::One(PlayerRefDef::EffectController),
            mana_cost!("{1}{G}"),
        ),
        &NANTUKO_COPIES_ITS_HOST,
        &NANTUKO_MAKES_AN_INSECT,
    )
    .only_if(&NANTUKO_IS_WEARING_A_CREATURE),
);

static NANTUKO_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::alternative_cast_with_targets(
        mana_cost!("{1}{G}"),
        AlternativeCastKindDef::Bestow,
        Some(
            "Bestow {1}{G} (If you cast this card for its bestow cost, it's an Aura spell with \
             enchant creature. It becomes a creature again if it's not attached to a creature.)",
        ),
        &abilities::ENCHANT_CREATURE_TARGET,
        EffectDef::Attach {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    ),
    // Only while it is an Aura (CR 702.103d): as a creature it enchants
    // nothing and the clause names nothing.
    AbilityDef::static_ability(
        "Enchanted creature gets +1/+1.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::AttachedPermanent,
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            ),
        },
    ),
    AbilityDef::triggered(
        "Landfall — Whenever a land you control enters, you may pay {1}{G} if this permanent is \
         attached to a creature you control. If you do, create a token that's a copy of that \
         creature. If you didn't create a token this way, create a 1/1 green Insect creature \
         token.",
        TriggerEventDef::zone_changed(A_LAND_YOU_CONTROL, None, Some(ZoneKind::Battlefield)),
        NANTUKO_LANDFALL,
    ),
];

pub(in crate::card::sets) static SPRINGHEART_NANTUKO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("54a3ea87-005e-4985-b2a5-21711d0b71c0"),
    "Springheart Nantuko",
    CardArt::new("54a3ea87-005e-4985-b2a5-21711d0b71c0", "Valera Lutfullina"),
    CardSet::ModernHorizons3,
    // Two mana for a 1/1, or four to bestow it onto something worth copying
    // -- and then every land is another one of that.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Insect", "Monk"], 1, 1)
        .with_type(CardType::Enchantment)
        .with_abilities(&NANTUKO_ABILITIES),
);

// MH3 172 — Temperamental Oozewagg
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TEMPERAMENTAL_OOZEWAGG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6625df2e-7046-411a-ae86-c46ac0953a0b"),
    "Temperamental Oozewagg",
    crate::card::CardArt::new("6625df2e-7046-411a-ae86-c46ac0953a0b", "Pete Venters"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 179 — Conduit Goblin
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CONDUIT_GOBLIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5c9ad04d-c4d4-4d06-93bb-a881be733717"),
    "Conduit Goblin",
    crate::card::CardArt::new("5c9ad04d-c4d4-4d06-93bb-a881be733717", "Bruno Biazotto"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 184 — Expanding Ooze
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EXPANDING_OOZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bbdb095d-b826-4e3e-8c61-0d408e52d6b8"),
    "Expanding Ooze",
    crate::card::CardArt::new("bbdb095d-b826-4e3e-8c61-0d408e52d6b8", "Randy Gallegos"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 185 — Faithful Watchdog
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FAITHFUL_WATCHDOG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b9afac99-a094-41a8-8323-90dec29691c4"),
    "Faithful Watchdog",
    crate::card::CardArt::new("b9afac99-a094-41a8-8323-90dec29691c4", "Samuel Perin"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 197 — Phlage, Titan of Fire's Fury
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

pub(in crate::card::sets) static PHLAGE_TITAN_OF_FIRES_FURY: CardRecord =
    CardRecord::new_with_legacy_id(
        2227,
        "Phlage, Titan of Fire's Fury",
        CardArt::new("e419cd0b-2449-4cc5-9ead-b9e45e271700", "Lucas Graciano"),
        CardSet::ModernHorizons3,
        // A three-mana Lightning Helix that stays a Helix until the graveyard is
        // deep enough, and then is a 6/6 that helixes again every attack.
        CardRules::new_creature(mana_cost!("{1}{R}{W}"), &["Elder", "Giant"], 6, 6)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&PHLAGE_ABILITIES),
    );

// MH3 199 — Psychic Frog
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
        &[AbilityCostDef::MoveToZone(
            crate::card::MoveToZoneCostDef::new(
                ObjectPredicateDef::Any,
                ZoneKind::Graveyard,
                ZoneKind::Exile,
                3,
            ),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::add_ability(&abilities::flying()),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    ),
];

pub(in crate::card::sets) static PSYCHIC_FROG: CardRecord = CardRecord::new_with_legacy_id(
    2277,
    "Psychic Frog",
    CardArt::new("68924203-c3d9-41ce-8ca8-c6dd491eb3ca", "Pete Venters"),
    CardSet::ModernHorizons3,
    // Two mana that turns a full hand into a big evasive body and a full
    // graveyard into the evasion, and draws a card every time it connects.
    CardRules::new_creature(mana_cost!("{U}{B}"), &["Frog"], 1, 2)
        .with_abilities(&PSYCHIC_FROG_ABILITIES),
);

// MH3 204 — Snapping Voidcraw
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SNAPPING_VOIDCRAW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ab3a5a5-9cb1-4ee5-b7b2-d870c9a56097"),
    "Snapping Voidcraw",
    crate::card::CardArt::new("9185371c-2dde-48ad-ab27-08be04b3c522", "Camille Alquier"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 208 — Writhing Chrysalis
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WRITHING_CHRYSALIS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f54dbeb1-51f8-40e2-912a-ec25457de5a2"),
    "Writhing Chrysalis",
    crate::card::CardArt::new("f54dbeb1-51f8-40e2-912a-ec25457de5a2", "Domenico Cava"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 209 — Disruptor Flute
pub(in crate::card::sets) static DISRUPTOR_FLUTE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5cad8671-4761-4014-a8a3-af45627e6e79"),
    "Disruptor Flute",
    crate::card::CardArt::new("5cad8671-4761-4014-a8a3-af45627e6e79", "Xavier Ribeiro"),
    crate::card::CardSet::ModernHorizons3,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        abilities::flash(),
        abilities::choose_card_name_as_enters(
            "As this artifact enters, choose a card name.",
            crate::card::BattlefieldEntryScalarChoiceDef::CARD_NAME,
        ),
        abilities::chosen_name_spell_cost_increase(
            "Spells with the chosen name cost {3} more to cast.",
            PlayerRelation::Any,
            mana_cost!("{3}"),
        ),
        abilities::cannot_activate_nonmana_abilities_with_chosen_name(
            "Activated abilities of sources with the chosen name can't be activated unless they're mana abilities.",
        ),
    ]),
);

// MH3 217 — Bountiful Landscape
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

static BOUNTIFUL_LANDSCAPE_ABILITIES: [AbilityDef; 3] = landscape_abilities(
    "{T}, Sacrifice this land: Search your library for a basic Forest, Island, or Mountain card, \
     put it onto the battlefield tapped, then shuffle.",
    A_BASIC_TRIOME_LAND,
    "Cycling {G}{U}{R} ({G}{U}{R}, Discard this card: Draw a card.)",
    mana_cost!("{G}{U}{R}"),
);

pub(in crate::card::sets) static BOUNTIFUL_LANDSCAPE: CardRecord = CardRecord::new_with_legacy_id(
    2265,
    "Bountiful Landscape",
    CardArt::new("b277752b-430a-4f09-8a98-b72f813dd52e", "Mark Poole"),
    CardSet::ModernHorizons3,
    // A land that taps for nothing useful and fetches a tapped basic, which
    // is worth a slot only because it is also a cycling card and because
    // what it finds is a land drop somebody else paid for.
    CardRules::new_land(&[]).with_abilities(&BOUNTIFUL_LANDSCAPE_ABILITIES),
);

// MH3 218 — Contaminated Landscape
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CONTAMINATED_LANDSCAPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e2312c49-1627-47ad-8113-78a999a97d8d"),
    "Contaminated Landscape",
    crate::card::CardArt::new("e2312c49-1627-47ad-8113-78a999a97d8d", "Donato Giancola"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 219 — Deceptive Landscape
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DECEPTIVE_LANDSCAPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2ae6828e-ff19-45db-8b59-61616353491f"),
    "Deceptive Landscape",
    crate::card::CardArt::new("2ae6828e-ff19-45db-8b59-61616353491f", "Erikas Perl"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 221 — Foreboding Landscape
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FOREBODING_LANDSCAPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57fb0fa7-0c5c-4a75-9461-c51403c30282"),
    "Foreboding Landscape",
    crate::card::CardArt::new("57fb0fa7-0c5c-4a75-9461-c51403c30282", "Erikas Perl"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 223 — Perilous Landscape
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PERILOUS_LANDSCAPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4b0bd07e-cf80-4d64-af29-f4cec6632b3e"),
    "Perilous Landscape",
    crate::card::CardArt::new("4b0bd07e-cf80-4d64-af29-f4cec6632b3e", "Alayna Danner"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 225 — Seething Landscape
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SEETHING_LANDSCAPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("661fc907-7003-45c6-820c-9616e9a71c30"),
    "Seething Landscape",
    crate::card::CardArt::new("661fc907-7003-45c6-820c-9616e9a71c30", "Piotr Dura"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 226 — Shattered Landscape
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHATTERED_LANDSCAPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b3da28c7-6e92-439d-a163-91682d4f11dc"),
    "Shattered Landscape",
    crate::card::CardArt::new("b3da28c7-6e92-439d-a163-91682d4f11dc", "Erikas Perl"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 227 — Sheltering Landscape
/// The Naya half of the same cycle, and the same shape: three basics, a
/// tapped land, and a cycling cost nobody pays for the mana.
static A_BASIC_NAYA_LAND: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::Supertype(CardSupertype::Basic),
    ObjectPredicateDef::HasAnyBasicLandType(&[
        BasicLandType::Mountain,
        BasicLandType::Forest,
        BasicLandType::Plains,
    ]),
]);

static SHELTERING_LANDSCAPE_ABILITIES: [AbilityDef; 3] = [
    abilities::tap_for(ManaColor::Colorless),
    AbilityDef::activated(
        "{T}, Sacrifice this land: Search your library for a basic Mountain, Forest, or Plains \
         card, put it onto the battlefield tapped, then shuffle.",
        &LANDSCAPE_FETCH_COST,
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: A_BASIC_NAYA_LAND,
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: true,
            attachment: None,
            binding: None,
            then: None,
        },
    ),
    abilities::cycling(
        "Cycling {R}{G}{W} ({R}{G}{W}, Discard this card: Draw a card.)",
        mana_cost!("{R}{G}{W}"),
    ),
];

pub(in crate::card::sets) static SHELTERING_LANDSCAPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0fe070f4-8877-4280-b8fd-869f3ac34ab6"),
    "Sheltering Landscape",
    CardArt::new("0fe070f4-8877-4280-b8fd-869f3ac34ab6", "Erikas Perl"),
    CardSet::ModernHorizons3,
    // The same bargain as its Temur cousin: a colourless tap nobody wants, a
    // tapped basic when you have the land drop, and a card when you do not.
    CardRules::new_land(&[]).with_abilities(&SHELTERING_LANDSCAPE_ABILITIES),
);

// MH3 228 — Shifting Woodland
/// Delirium, as an activation restriction rather than a trigger condition:
/// the ability is not offered at all while the graveyard is short of four
/// card types.
static WOODLAND_DELIRIUM: ValueComparisonDef = ValueComparisonDef {
    left: ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
    comparison: ComparisonDef::GreaterOrEqual,
    right: ValueDef::Constant(4),
};

static WOODLAND_HAS_DELIRIUM: TriggerConditionDef =
    TriggerConditionDef::ValueComparison(&WOODLAND_DELIRIUM);

/// "Target permanent card in your graveyard": the five permanent types, in
/// your own graveyard rather than either.
static A_PERMANENT_CARD_IN_YOUR_GRAVEYARD: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::HasType(CardType::Enchantment),
            ObjectPredicateDef::HasType(CardType::Land),
            ObjectPredicateDef::HasType(CardType::Planeswalker),
        ]),
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: Some(PlayerRelation::You),
    },
)];

static WOODLAND_COPY_COST: [AbilityCostDef; 1] = [AbilityCostDef::Mana(mana_cost!("{2}{G}{G}"))];

static SHIFTING_WOODLAND_ABILITIES: [AbilityDef; 3] = [
    abilities::enters_tapped_unless_you_control(
        "This land enters tapped unless you control a Forest.",
        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
    ),
    AbilityDef::activated_mana(
        "{T}: Add {G}.",
        &WOODLAND_MANA_COST,
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
    ),
    // No "except it has this ability" clause, unlike Thespian's Stage: the
    // copy replaces every copiable value, so while it is a creature it is
    // not a land, taps for nothing, and cannot do this again.
    AbilityDef::activated_with_targets(
        "Delirium — {2}{G}{G}: This land becomes a copy of target permanent card in your \
         graveyard until end of turn. Activate only if there are four or more card types among \
         cards in your graveyard.",
        &WOODLAND_COPY_COST,
        &A_PERMANENT_CARD_IN_YOUR_GRAVEYARD,
        EffectDef::BecomeCopyOf {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            copier: None,
            exceptions: CopyExceptionsDef::NONE,
            duration: Some(ResolvedEffectDurationDef::UntilEndOfTurn),
        },
    )
    .with_activation_condition(&WOODLAND_HAS_DELIRIUM),
];

static WOODLAND_MANA_COST: [AbilityCostDef; 1] = [AbilityCostDef::TapSource];

pub(in crate::card::sets) static SHIFTING_WOODLAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("059164e1-894d-4586-9800-e60d6fbd6eb6"),
    "Shifting Woodland",
    CardArt::new("059164e1-894d-4586-9800-e60d6fbd6eb6", "Josu Hernaiz"),
    CardSet::ModernHorizons3,
    // A Forest that turns into the best thing you have already lost, once
    // the graveyard is deep enough to be worth reading.
    CardRules::new_land(&[]).with_abilities(&SHIFTING_WOODLAND_ABILITIES),
);

// MH3 231 — Tranquil Landscape
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TRANQUIL_LANDSCAPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("113f48b9-a972-4e2c-af95-05ab078e01f2"),
    "Tranquil Landscape",
    crate::card::CardArt::new("113f48b9-a972-4e2c-af95-05ab078e01f2", "Randy Gallegos"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 232 — Twisted Landscape
/// The Jund member: the same land with the other three basics on it.
static A_BASIC_JUND_LAND: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::Supertype(CardSupertype::Basic),
    ObjectPredicateDef::HasAnyBasicLandType(&[
        BasicLandType::Swamp,
        BasicLandType::Mountain,
        BasicLandType::Forest,
    ]),
]);

static TWISTED_LANDSCAPE_ABILITIES: [AbilityDef; 3] = landscape_abilities(
    "{T}, Sacrifice this land: Search your library for a basic Swamp, Mountain, or Forest card, \
     put it onto the battlefield tapped, then shuffle.",
    A_BASIC_JUND_LAND,
    "Cycling {B}{R}{G} ({B}{R}{G}, Discard this card: Draw a card.)",
    mana_cost!("{B}{R}{G}"),
);

pub(in crate::card::sets) static TWISTED_LANDSCAPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0d647d67-f963-43b4-ade8-6c90e91f65ac"),
    "Twisted Landscape",
    CardArt::new("d0e3e7b3-7ba9-47a2-b46c-a40bffb445e2", "Piotr Dura"),
    CardSet::ModernHorizons3,
    // The land drop it finds is the point; cycling is what it does on the
    // turns the deck already has enough of them.
    CardRules::new_land(&[]).with_abilities(&TWISTED_LANDSCAPE_ABILITIES),
);

// MH3 237 — Ajani, Nacatl Pariah // Ajani, Nacatl Avenger
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
        face_down: false,
        then: None,
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

/// The four roles the ultimate lets each opponent fill. Order is printed
/// order, which is also APNAP choice order within one player's selection.
static AJANI_SPARED_KINDS: [ObjectPredicateDef; 4] = [
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::HasType(CardType::Enchantment),
    ObjectPredicateDef::HasType(CardType::Planeswalker),
];

static AJANI_SACRIFICE_REST: EffectDef = EffectDef::Sacrifice {
    object: EffectRecipientDef::objects(ObjectSetDef::Binding(ObjectSetBindingIndex::new(1))),
};

static AJANI_CHOOSE_SURVIVORS: SimultaneousChooseDef = SimultaneousChooseDef {
    player: EffectRecipientDef::Opponent,
    candidates: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
    one_of_each: &AJANI_SPARED_KINDS,
    chosen: ObjectSetBindingIndex::PRIMARY,
    unchosen: ObjectSetBindingIndex::new(1),
    then: &AJANI_SACRIFICE_REST,
};

static AJANI_TURNS_OVER_SEQUENCE: EffectDef = EffectDef::Sequence(&AJANI_TURNS_OVER);

static AJANI_PARIAH_ABILITIES: [AbilityDef; 2] = [
    abilities::enters_trigger(
        "When Ajani enters, create a 2/1 white Cat Warrior creature token.",
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
        EffectDef::SimultaneousChoose(AJANI_CHOOSE_SURVIVORS),
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

pub(in crate::card::sets) static AJANI_NACATL_PARIAH: CardRecord =
    CardRecord::new_dfc_with_legacy_id(
        2199,
        "Ajani, Nacatl Pariah // Ajani, Nacatl Avenger",
        CardArt::new("0d16e8e0-31b2-4389-afd6-783c501f6fa0", "Chris Rallis"),
        CardSet::ModernHorizons3,
        &[
            ("Ajani, Nacatl Pariah", ajani_nacatl_pariah_rules()),
            ("Ajani, Nacatl Avenger", ajani_nacatl_avenger_rules()),
        ],
    );

// MH3 239 — Witch Enchanter // Witch-Blessed Meadow
/// "Target artifact or enchantment an opponent controls": two types and a
/// controller, which together are the whole restriction.
static ENCHANTER_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Enchantment),
        ]),
        ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
    ]),
)];

static ENCHANTER_ABILITIES: [AbilityDef; 1] = [abilities::enters_trigger_with_targets(
    "When this creature enters, destroy target artifact or enchantment an opponent controls.",
    &ENCHANTER_TARGET,
    EffectDef::destroy_target(TargetIndex::PRIMARY, true),
)];

/// Declining is what taps it, so the paid branch does nothing and the
/// declined branch is the whole of the cost.
static MEADOW_ENTERS_TAPPED: [ReplacementEffectDef; 1] =
    [ReplacementEffectDef::ModifyBattlefieldEntry(
        BattlefieldEntryModificationDef::Tapped,
    )];

static MEADOW_PAID: [ReplacementEffectDef; 0] = [];

static MEADOW_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::replacement(
        "As this land enters, you may pay 3 life. If you don't, it enters tapped.",
        ReplacementEffectDef::PayOr {
            payment: EffectPaymentDef::life(PlayerSetDef::Related(PlayerRelation::You), 3),
            if_paid: &MEADOW_PAID,
            if_declined: &MEADOW_ENTERS_TAPPED,
        },
    ),
    AbilityDef::activated_mana(
        "{T}: Add {W}.",
        &MEADOW_MANA_COST,
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
    ),
];

static MEADOW_MANA_COST: [AbilityCostDef; 1] = [AbilityCostDef::TapSource];

const fn witch_enchanter_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Warlock"], 2, 2)
        .with_abilities(&ENCHANTER_ABILITIES)
}

const fn witch_blessed_meadow_rules() -> CardRules {
    CardRules::new_land(&[]).with_abilities(&MEADOW_ABILITIES)
}

pub(in crate::card::sets) static WITCH_ENCHANTER: CardRecord = CardRecord::new_mdfc(
    PrintingAnchor::scryfall("62061e7c-cf19-4f03-b8fa-2bdba62d6b0b"),
    "Witch Enchanter // Witch-Blessed Meadow",
    CardArt::new("62061e7c-cf19-4f03-b8fa-2bdba62d6b0b", "Tyler Walpole"),
    CardSet::ModernHorizons3,
    &[
        ("Witch Enchanter", witch_enchanter_rules()),
        ("Witch-Blessed Meadow", witch_blessed_meadow_rules()),
    ],
);

// MH3 241 — Sink into Stupor // Soporific Springs
/// One slot over two zones: a land is never a spell, so "nonland" is the
/// whole of the restriction on either side of the "or".
static A_SPELL_OR_NONLAND_PERMANENT_OF_THEIRS: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            zones: &[ZoneKind::Stack, ZoneKind::Battlefield],
            controller: Some(PlayerRelation::Opponent),
            owner: None,
        },
    )];

/// Returning a spell is not countering it: one that cannot be countered is
/// answered all the same, and its controller keeps the card.
static STUPOR_RETURNS_IT: EffectDef = EffectDef::MoveToZone {
    counters: None,
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    from: None,
    zone: ZoneKind::Hand,
    placement: ZonePlacement::Top,
    arrival_effect: None,
    attachment: None,
    controller: None,
    tapped: false,
};

static SPRINGS_PAID: [ReplacementEffectDef; 0] = [];

static SPRINGS_ENTERS_TAPPED: [ReplacementEffectDef; 1] =
    [ReplacementEffectDef::ModifyBattlefieldEntry(
        BattlefieldEntryModificationDef::Tapped,
    )];

static SPRINGS_MANA_COST: [AbilityCostDef; 1] = [AbilityCostDef::TapSource];

static SPRINGS_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::replacement(
        "As this land enters, you may pay 3 life. If you don't, it enters tapped.",
        ReplacementEffectDef::PayOr {
            payment: EffectPaymentDef::life(PlayerSetDef::Related(PlayerRelation::You), 3),
            if_paid: &SPRINGS_PAID,
            if_declined: &SPRINGS_ENTERS_TAPPED,
        },
    ),
    AbilityDef::activated_mana(
        "{T}: Add {U}.",
        &SPRINGS_MANA_COST,
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
    ),
];

const fn sink_into_stupor_rules() -> CardRules {
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target spell or nonland permanent an opponent controls to its owner's hand.",
        &A_SPELL_OR_NONLAND_PERMANENT_OF_THEIRS,
        STUPOR_RETURNS_IT,
    ))
}

const fn soporific_springs_rules() -> CardRules {
    CardRules::new_land(&[]).with_abilities(&SPRINGS_ABILITIES)
}

pub(in crate::card::sets) static SINK_INTO_STUPOR: CardRecord = CardRecord::new_mdfc(
    PrintingAnchor::scryfall("5358b87a-1a29-426d-b165-40c97da2c14d"),
    "Sink into Stupor // Soporific Springs",
    CardArt::new("5358b87a-1a29-426d-b165-40c97da2c14d", "Peter Polach"),
    CardSet::ModernHorizons3,
    &[
        ("Sink into Stupor", sink_into_stupor_rules()),
        ("Soporific Springs", soporific_springs_rules()),
    ],
);

// MH3 284 — Annoyed Altisaur
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ANNOYED_ALTISAUR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7536d618-0c98-45bb-913b-b8117b4acf87"),
    "Annoyed Altisaur",
    crate::card::CardArt::new("4aa9354d-3496-47f4-81c9-aead15efb8bb", "Lars Grant-West"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 286 — Priest of Titania
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PRIEST_OF_TITANIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("965c33c3-0c68-4516-b8b0-5a0552ed44b6"),
    "Priest of Titania",
    crate::card::CardArt::new("eb11921b-1b28-483f-a707-4de21a6daa31", "Rebecca Guay"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 351 — Arena of Glory
static ARENA_HASTE: AbilityDef = abilities::haste();

/// The rider asks what the mana paid for rather than restricting what it may
/// pay for: this mana casts anything, and only a creature gets anything out
/// of it.
static ARENA_MANA_GRANTS_HASTE: [ManaSpendEffectDef; 1] =
    [ManaSpendEffectDef::ApplyToPaidSpellMatching {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        effect: AppliedEffectDef::add_ability(&ARENA_HASTE),
    }];

/// {R} in, {R}{R} out, and one untap step owed: the land pays for the haste
/// out of next turn rather than out of this one.
static ARENA_EXERT_COST: [AbilityCostDef; 3] = [
    AbilityCostDef::Mana(mana_cost!("{R}")),
    AbilityCostDef::TapSource,
    AbilityCostDef::ExertSource,
];

static ARENA_OF_GLORY_ABILITIES: [AbilityDef; 3] = [
    abilities::check_land_enters(
        "This land enters tapped unless you control a Mountain.",
        &[BasicLandType::Mountain],
    ),
    AbilityDef::activated_mana(
        "{T}: Add {R}.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
    ),
    AbilityDef::activated_mana(
        "{R}, {T}, Exert this land: Add {R}{R}. If that mana is spent on a creature spell, it \
         gains haste until end of turn.",
        &ARENA_EXERT_COST,
        EffectDef::AddMana(
            AddManaEffectDef::one(ManaColor::Red)
                .with_amount(2)
                .with_spend_effects(&ARENA_MANA_GRANTS_HASTE),
        ),
    ),
];

pub(in crate::card::sets) static ARENA_OF_GLORY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3d7d07bb-b875-4a6d-8b87-4187e823af75"),
    "Arena of Glory",
    crate::card::CardArt::new("3d7d07bb-b875-4a6d-8b87-4187e823af75", "Piotr Dura"),
    crate::card::CardSet::ModernHorizons3,
    // A red source that costs nothing to play and turns one creature a game
    // into a surprise, which is what a haste land is for.
    CardRules::new_land(&[]).with_abilities(&ARENA_OF_GLORY_ABILITIES),
);

// MH3 377 — Nadu, Winged Wisdom
/// One card off the top, sorted by whether it is a land: the land goes to
/// the battlefield and anything else goes to the hand, so nothing is left
/// for the player to decide.
static NADU_REVEAL: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(1),
    object: Some(ObjectPredicateDef::HasType(CardType::Land)),
    minimum: 0,
    maximum: 1,
    select_all_matching: true,
    select_one_of_each_type: false,
    reveal_inspected: true,
    reveal_selected: true,
    counted: None,
    selected_zone: ZoneKind::Battlefield,
    selected_placement: ZonePlacement::Top,
    rest_zone: ZoneKind::Hand,
    rest_placement: ZonePlacement::Top,
    rest_random_order: false,
    rest_counters: None,
    selected_order_follows_choice: false,
    then: None,
    selected_hidden: false,
    selected_linked_to_source: false,
    selected_face_down: None,
};

/// The granted ability, carried by each creature rather than by Nadu: the
/// cap is on one creature's copy of it, so every creature you control has
/// two of these a turn.
static NADU_GRANTED: AbilityDef = AbilityDef::triggered(
    "Whenever this creature becomes the target of a spell or ability, reveal the top card of \
     your library. If it's a land card, put it onto the battlefield. Otherwise, put it into your \
     hand. This ability triggers only twice each turn.",
    TriggerEventDef::BecomesTargetOfSpellOrAbility(ObjectPredicateDef::Any),
    EffectDef::LookAtTopAndSelect {
        player: EffectRecipientDef::Controller,
        looker: EffectRecipientDef::Controller,
        selection: &NADU_REVEAL,
    },
)
.triggering_at_most(2);

static CREATURES_YOU_CONTROL_NADU: EffectRecipientDef = EffectRecipientDef::matching_objects(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

static NADU_ABILITIES: [AbilityDef; 2] = [
    abilities::flying(),
    AbilityDef::static_ability(
        "Creatures you control have \"Whenever this creature becomes the target of a spell or \
         ability, reveal the top card of your library. If it's a land card, put it onto the \
         battlefield. Otherwise, put it into your hand. This ability triggers only twice each \
         turn.\"",
        EffectDef::StaticApply {
            recipient: CREATURES_YOU_CONTROL_NADU,
            effect: AppliedEffectDef::add_ability(&NADU_GRANTED),
        },
    ),
];

pub(in crate::card::sets) static NADU_WINGED_WISDOM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8281df8a-2fde-454a-813c-d9f86bb35d36"),
    "Nadu, Winged Wisdom",
    CardArt::new("8281df8a-2fde-454a-813c-d9f86bb35d36", "Gossip Goblin"),
    CardSet::ModernHorizons3,
    // Three mana for a 3/4 flier that turns every targeting spell you own
    // into a card, twice per creature per turn -- and Nadu is a creature you
    // control, so pointing something at him counts too.
    CardRules::new_creature(mana_cost!("{1}{G}{U}"), &["Bird", "Wizard"], 3, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&NADU_ABILITIES),
);

// MH3 443 — Tamiyo, Inquisitive Student // Tamiyo, Seasoned Scholar
/// Exile and return, which is how a permanent turns over into a new object
/// rather than merely flipping: the Tamiyo that comes back has no counters,
/// no summoning history, and a fresh set of loyalty.
static TAMIYO_TURNS_OVER: [EffectDef; 2] = [
    EffectDef::ExileLinkedToSource {
        object: EffectRecipientDef::Source,
        face_down: false,
        then: None,
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

static TAMIYO_STUDENT_ABILITIES: [AbilityDef; 3] = [
    abilities::flying(),
    AbilityDef::triggered(
        "Whenever Tamiyo attacks, investigate. (Create a Clue token. It's an artifact with \"{2}, \
         Sacrifice this token: Draw a card.\")",
        TriggerEventDef::Attacks(AttackEventMatcherDef::any(ObjectPredicateDef::Source)),
        EffectDef::create_token(tokens::clue()),
    ),
    // The third card of the turn, counted over the whole turn rather than
    // any one step: her own attack Clue and the draw step are usually two
    // of the three.
    AbilityDef::triggered(
        "When you draw your third card in a turn, exile Tamiyo, then return her to the \
         battlefield transformed under her owner's control.",
        TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(PlayerRelation::You, 3)),
        EffectDef::Sequence(&TAMIYO_TURNS_OVER),
    ),
];

/// The attackers her plus ability shrinks. It is installed on resolution and
/// watches until her controller's next turn, so it catches the attack it was
/// played to blunt.
static TAMIYO_SHRINKS_ATTACKERS: AbilityDef = AbilityDef::triggered(
    "Whenever a creature attacks you or a planeswalker you control, it gets -1/-0 until end of \
     turn.",
    TriggerEventDef::Attacks(AttackEventMatcherDef::attacking(
        ObjectPredicateDef::HasType(CardType::Creature),
        PlayerRelation::You,
    )),
    EffectDef::Apply {
        recipient: EffectRecipientDef::TriggeringObject,
        effect: AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(-1),
            ValueDef::Constant(0),
        ),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
);

static AN_INSTANT_OR_SORCERY_IN_YOUR_GRAVEYARD: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::HasType(CardType::Sorcery),
            ]),
            zones: &[ZoneKind::Graveyard],
            controller: None,
            owner: Some(PlayerRelation::You),
        },
    )];

/// "If it's a green card, add one mana of any color." One mana when the card
/// returned was green and none otherwise, which is the whole rider: an
/// amount rather than a branch, read off the target the clause already has.
static TAMIYO_GREEN_REBATE: TargetConditionDef = TargetConditionDef {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::Color(ManaColor::Green),
    then: ValueDef::Constant(1),
    otherwise: ValueDef::Constant(0),
};

static TAMIYO_RETURNS_AND_REBATES: [EffectDef; 2] = [
    EffectDef::MoveToZone {
        counters: None,
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        from: None,
        zone: ZoneKind::Hand,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        controller: None,
        tapped: false,
    },
    EffectDef::AddMana(
        AddManaEffectDef::any_color()
            .with_variable_amount(ValueDef::IfTargetMatches(&TAMIYO_GREEN_REBATE)),
    ),
];

static TAMIYO_EMBLEM_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
    "You have no maximum hand size.",
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::Controller,
        effect: AppliedEffectDef::Rule(AppliedRuleDef::NoMaximumHandSize),
    },
)];

static TAMIYO_EMBLEM: EmblemCharacteristics =
    EmblemCharacteristics::new("Tamiyo, Seasoned Scholar emblem", &TAMIYO_EMBLEM_ABILITIES);

static TAMIYO_HALF_HER_LIBRARY: HalvedValueDef =
    HalvedValueDef::new(ValueDef::LibrarySize(PlayerRelation::You), RoundingDef::Up);

static TAMIYO_ULTIMATE: [EffectDef; 2] = [
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Halved(&TAMIYO_HALF_HER_LIBRARY),
    },
    EffectDef::CreateEmblem {
        emblem: TAMIYO_EMBLEM,
    },
];

static TAMIYO_PLUS_TWO_COST: [AbilityCostDef; 1] = [AbilityCostDef::Loyalty(2)];

static TAMIYO_MINUS_THREE_COST: [AbilityCostDef; 1] = [AbilityCostDef::Loyalty(-3)];

static TAMIYO_MINUS_SEVEN_COST: [AbilityCostDef; 1] = [AbilityCostDef::Loyalty(-7)];

static TAMIYO_SCHOLAR_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::activated(
        "+2: Until your next turn, whenever a creature attacks you or a planeswalker you \
         control, it gets -1/-0 until end of turn.",
        &TAMIYO_PLUS_TWO_COST,
        EffectDef::InstallTrigger(InstalledTriggerDef {
            ability: &TAMIYO_SHRINKS_ATTACKERS,
            lifetime: InstalledTriggerLifetimeDef::UntilNextTurn(PlayerRefDef::EffectController),
        }),
    ),
    AbilityDef::activated_with_targets(
        "−3: Return target instant or sorcery card from your graveyard to your hand. If it's a \
         green card, add one mana of any color.",
        &TAMIYO_MINUS_THREE_COST,
        &AN_INSTANT_OR_SORCERY_IN_YOUR_GRAVEYARD,
        EffectDef::Sequence(&TAMIYO_RETURNS_AND_REBATES),
    ),
    AbilityDef::activated(
        "−7: Draw cards equal to half the number of cards in your library, rounded up. You get \
         an emblem with \"You have no maximum hand size.\"",
        &TAMIYO_MINUS_SEVEN_COST,
        EffectDef::Sequence(&TAMIYO_ULTIMATE),
    ),
];

const fn tamiyo_inquisitive_student_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{U}"), &["Moonfolk", "Wizard"], 0, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&TAMIYO_STUDENT_ABILITIES)
}

const fn tamiyo_seasoned_scholar_rules() -> CardRules {
    CardRules::new_planeswalker_without_mana_cost(&["Tamiyo"])
        .with_supertype(CardSupertype::Legendary)
        .with_starting_loyalty(2)
        .with_abilities(&TAMIYO_SCHOLAR_ABILITIES)
}

pub(in crate::card::sets) static TAMIYO_INQUISITIVE_STUDENT: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("1b234fee-a2b6-4661-9f98-4da6fc26aebc"),
    "Tamiyo, Inquisitive Student // Tamiyo, Seasoned Scholar",
    CardArt::new("1b234fee-a2b6-4661-9f98-4da6fc26aebc", "Evyn Fong"),
    CardSet::ModernHorizons3,
    &[
        (
            "Tamiyo, Inquisitive Student",
            tamiyo_inquisitive_student_rules(),
        ),
        ("Tamiyo, Seasoned Scholar", tamiyo_seasoned_scholar_rules()),
    ],
);

// MH3 444 — Sorin of House Markov // Sorin, Ravenous Neonate
/// Three life in a turn, counted as a running total: gaining it and losing
/// it again still turns him over, because what the clause reads is the
/// gaining rather than where the life total ended up.
static SORIN_GAINED_THREE: ValueComparisonDef = ValueComparisonDef {
    left: ValueDef::LifeGainedThisTurn(PlayerRelation::You),
    comparison: ComparisonDef::GreaterOrEqual,
    right: ValueDef::Constant(3),
};

static SORIN_HAS_FED: TriggerConditionDef =
    TriggerConditionDef::ValueComparison(&SORIN_GAINED_THREE);

/// The same exile-and-return Ajani uses: one resolution, so he is gone and
/// back before anything else happens, and he comes back a new object with
/// his printed loyalty.
static SORIN_TURNS_OVER: [EffectDef; 2] = [
    EffectDef::ExileLinkedToSource {
        object: EffectRecipientDef::Source,
        face_down: false,
        then: None,
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

static SORIN_MARKOV_ABILITIES: [AbilityDef; 3] = [
    abilities::lifelink(),
    abilities::extort(),
    AbilityDef::triggered_if(
        "At the beginning of each of your postcombat main phases, if you gained 3 or more life \
         this turn, exile Sorin, then return him to the battlefield transformed under his \
         owner's control.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::PostcombatMain,
            player: PlayerRelation::You,
        },
        &SORIN_HAS_FED,
        EffectDef::Sequence(&SORIN_TURNS_OVER),
    ),
];

/// "It becomes a Vampire in addition to its other types": added rather than
/// set, and with no duration, so what it was stays and the Vampire sticks.
static AS_A_VAMPIRE: AppliedEffectDef =
    AppliedEffectDef::Characteristic(CharacteristicOperationDef::CreatureTypes(
        SetOperationDef::Add(CreatureTypeSetDef::named(&["Vampire"])),
    ));

/// "A white permanent other than that creature or Sorin." The source is
/// Sorin; the creature is the one this ability just took, which is why the
/// query has to leave the target out rather than merely counting what you
/// control.
static A_WHITE_PERMANENT_BESIDES_THEM: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::Color(ManaColor::White),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
        ]),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    )
    .excluding_target(TargetIndex::PRIMARY),
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 1,
};

static SORIN_LIFELINK_COUNTER: EffectDef = EffectDef::AddCounters {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    kind: CounterKind::Lifelink,
    amount: ValueDef::Constant(1),
};

static SORIN_TAKES_IT: [EffectDef; 3] = [
    EffectDef::GainControl {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        controller: PlayerRefDef::EffectController,
        duration: ControlDurationDef::Indefinitely,
    },
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AS_A_VAMPIRE,
        duration: ResolvedEffectDurationDef::Permanent,
    },
    EffectDef::IfCondition {
        condition: &A_WHITE_PERMANENT_BESIDES_THEM,
        then: &SORIN_LIFELINK_COUNTER,
    },
];

static SORIN_ANY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

static SORIN_STEAL_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

static SORIN_PLUS_TWO: [AbilityCostDef; 1] = [AbilityCostDef::Loyalty(2)];
static SORIN_MINUS_ONE: [AbilityCostDef; 1] = [AbilityCostDef::Loyalty(-1)];
static SORIN_MINUS_SIX: [AbilityCostDef; 1] = [AbilityCostDef::Loyalty(-6)];

static SORIN_NEONATE_ABILITIES: [AbilityDef; 4] = [
    abilities::extort(),
    AbilityDef::activated(
        "+2: Create a Food token.",
        &SORIN_PLUS_TWO,
        EffectDef::create_token(tokens::food()),
    ),
    // The same tally the front face reads to turn over, spent here as
    // damage: the lifelink body he arrived as is what loads this.
    AbilityDef::activated_with_targets(
        "\u{2212}1: Sorin deals damage equal to the amount of life you gained this turn to any \
         target.",
        &SORIN_MINUS_ONE,
        &SORIN_ANY_TARGET,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::LifeGainedThisTurn(PlayerRelation::You),
        },
    ),
    AbilityDef::activated_with_targets(
        "\u{2212}6: Gain control of target creature. It becomes a Vampire in addition to its \
         other types. Put a lifelink counter on it if you control a white permanent other than \
         that creature or Sorin.",
        &SORIN_MINUS_SIX,
        &SORIN_STEAL_TARGET,
        EffectDef::Sequence(&SORIN_TAKES_IT),
    ),
];

const fn sorin_of_house_markov_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Noble"], 1, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&SORIN_MARKOV_ABILITIES)
}

const fn sorin_ravenous_neonate_rules() -> CardRules {
    // The back face has no mana cost, so its colours come from the printed
    // indicator. They matter to his own ultimate: he is a white permanent,
    // and the clause has to say "other than Sorin" precisely because of it.
    CardRules::new_planeswalker_without_mana_cost(&["Sorin"])
        .with_supertype(CardSupertype::Legendary)
        .with_starting_loyalty(3)
        .printed_colors(&[ManaColor::White, ManaColor::Black])
        .with_abilities(&SORIN_NEONATE_ABILITIES)
}

pub(in crate::card::sets) static SORIN_OF_HOUSE_MARKOV: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("0347bf13-1ccb-4d4d-a5f2-68181d494b85"),
    "Sorin of House Markov // Sorin, Ravenous Neonate",
    crate::card::CardArt::new("0347bf13-1ccb-4d4d-a5f2-68181d494b85", "Livia Prima"),
    crate::card::CardSet::ModernHorizons3,
    &[
        ("Sorin of House Markov", sorin_of_house_markov_rules()),
        ("Sorin, Ravenous Neonate", sorin_ravenous_neonate_rules()),
    ],
);

// MH3 448 — Guide of Souls
/// One life and one energy per creature, which is what makes the three-
/// energy payment a matter of a turn or two rather than a deck built for it.
static GUIDE_OF_SOULS_PAYOFF: [EffectDef; 2] = [
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
    EffectDef::AddPlayerCounters {
        recipient: EffectRecipientDef::Controller,
        kind: CounterKind::Energy,
        amount: ValueDef::Constant(1),
    },
];

/// All three stick: the counters and the type are permanent, so the
/// creature is still a flying Angel next turn.
static GUIDE_OF_SOULS_ANGEL: [EffectDef; 3] = [
    EffectDef::AddCounters {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        kind: CounterKind::PlusOnePlusOne,
        amount: ValueDef::Constant(2),
    },
    EffectDef::AddCounters {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        kind: CounterKind::Flying,
        amount: ValueDef::Constant(1),
    },
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Angel"])),
        duration: ResolvedEffectDurationDef::Permanent,
    },
];

static GUIDE_OF_SOULS_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Attacking,
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

static GUIDE_OF_SOULS_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::triggered(
        "Whenever another creature you control enters, you gain 1 life and get {E} (an energy \
         counter).",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            ]),
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::Sequence(&GUIDE_OF_SOULS_PAYOFF),
    ),
    // As with Inti, the target is declared when the attack trigger goes on
    // the stack rather than when the energy is paid, which is the one place
    // this differs from the printed reflexive trigger. "Whenever you attack"
    // guarantees an attacking creature, so there is always something to name.
    AbilityDef::triggered_with_targets(
        "Whenever you attack, you may pay {E}{E}{E}. When you do, put two +1/+1 counters and a \
         flying counter on target attacking creature. It becomes an Angel in addition to its \
         other types.",
        TriggerEventDef::attack_declared(ObjectPredicateDef::Any, 1, None),
        &GUIDE_OF_SOULS_TARGET,
        EffectDef::PayOr(PayOrDef::optional(
            EffectPaymentDef {
                payer: PlayerSetDef::Related(PlayerRelation::You),
                cost: EffectPaymentCostDef::Energy(3),
            },
            &EffectDef::Sequence(&GUIDE_OF_SOULS_ANGEL),
        )),
    ),
];

pub(in crate::card::sets) static GUIDE_OF_SOULS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("298de33f-cb39-47c5-9579-54d91eb34414"),
    "Guide of Souls",
    CardArt::new("298de33f-cb39-47c5-9579-54d91eb34414", "Ryan Valle"),
    CardSet::ModernHorizons3,
    // A one-mana body that turns every other creature into a life and an
    // energy, and then spends the energy making one of them an Angel.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Cleric"], 1, 2)
        .with_abilities(&GUIDE_OF_SOULS_ABILITIES),
);

// MH3 452 — Crabomination
static SACRIFICE_AN_ARTIFACT: SpellAdditionalCostDef = SpellAdditionalCostDef::new(
    ObjectPredicateDef::HasType(CardType::Artifact),
    ZoneKind::Battlefield,
    1,
);

/// The three zones, in the order the card names them. A library has a top
/// to take from; a graveyard and a hand do not, so those are drawn at
/// random.
static CRABOMINATION_ZONES: [ZonePickDef; 3] = [
    ZonePickDef::top(ZoneKind::Library),
    ZonePickDef::at_random(ZoneKind::Graveyard),
    ZonePickDef::at_random(ZoneKind::Hand),
];

static AN_OPPONENT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
)];

static CRABOMINATION_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::alternative_cast(
        mana_cost!("{5}{B}{B}"),
        AlternativeCastKindDef::Emerge,
        Some(
            "Emerge from artifact {5}{B}{B} (You may cast this spell by sacrificing an artifact \
             and paying the emerge cost reduced by that artifact's mana value.)",
        ),
        EffectDef::None,
    )
    .with_alternative_additional_cost(&SACRIFICE_AN_ARTIFACT),
    abilities::enters_trigger_with_targets(
        "When this creature enters, target opponent exiles the top card of their library, a card \
         at random from their graveyard, and a card at random from their hand. You may cast a \
         spell from among cards exiled this way without paying its mana cost.",
        &AN_OPPONENT,
        EffectDef::ExileOneFromEachZone(&PileExileDef {
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zones: &CRABOMINATION_ZONES,
            permission: Some(ExiledCastPermissionDef::FreeThisTurn),
        }),
    ),
];

pub(in crate::card::sets) static CRABOMINATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b6ac511f-6c28-45f9-968b-9ac72872641b"),
    "Crabomination",
    CardArt::new("b6ac511f-6c28-45f9-968b-9ac72872641b", "Nicholas Gregory"),
    CardSet::ModernHorizons3,
    // Six mana for a 5/5 is not the price anybody pays: an artifact that
    // has already done its work pays most of it.
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Crab", "Demon"], 5, 5)
        .with_abilities(&CRABOMINATION_ABILITIES),
);

// MH3 457 — Detective's Phoenix (alternate printing)

// MH3 460 — Wight of the Reliquary
/// Your own graveyard, which is what makes the sacrifice cost pay twice: the
/// creature it eats is a land and a point of power both.
static RELIQUARY_CREATURE_CARDS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Graveyard],
    PlayerRelation::You,
);

static ANOTHER_CREATURE_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
]);

static RELIQUARY_FETCH_COST: [AbilityCostDef; 2] = [
    AbilityCostDef::TapSource,
    AbilityCostDef::SacrificePermanent {
        object: ANOTHER_CREATURE_YOU_CONTROL,
        controller: PlayerRelation::You,
    },
];

static RELIQUARY_ABILITIES: [AbilityDef; 3] = [
    abilities::vigilance(),
    AbilityDef::static_ability(
        "This creature gets +1/+1 for each creature card in your graveyard.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::CountMatchingObjects(&RELIQUARY_CREATURE_CARDS),
                ValueDef::CountMatchingObjects(&RELIQUARY_CREATURE_CARDS),
            ),
        },
    ),
    AbilityDef::activated(
        "{T}, Sacrifice another creature: Search your library for a land card, put it onto the \
         battlefield tapped, then shuffle.",
        &RELIQUARY_FETCH_COST,
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
            enters_tapped: true,
            attachment: None,
            binding: None,
            then: None,
        },
    ),
];

pub(in crate::card::sets) static WIGHT_OF_THE_RELIQUARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("915715f7-5487-47aa-ada5-de1bce282164"),
    "Wight of the Reliquary",
    CardArt::new("915715f7-5487-47aa-ada5-de1bce282164", "Scott Murphy"),
    CardSet::ModernHorizons3,
    // Two mana for a body that grows with the graveyard it is filling, and
    // turns every spare creature into whatever land the deck needs.
    CardRules::new_creature(mana_cost!("{B}{G}"), &["Zombie", "Knight"], 2, 2)
        .with_abilities(&RELIQUARY_ABILITIES),
);

// MH3 484 — Six (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &DEVOURER_OF_DESTINY,
    &AERIE_AUXILIARY,
    &DOG_UMBRA,
    &MANDIBULAR_KITE,
    &OCELOT_PRIDE,
    &PHELIA_EXUBERANT_SHEPHERD,
    &STATIC_PRISON,
    &THRABEN_CHARM,
    &BRAINSURGE,
    &SERUM_VISIONARY,
    &ACCURSED_MARAUDER,
    &EMPEROR_OF_BONES,
    &NETHERGOYF,
    &RETROFITTED_TRANSMOGRANT,
    &SCURRILOUS_SENTRY,
    &WITHER_AND_BLOOM,
    &AMPED_RAPTOR,
    &DETECTIVES_PHOENIX,
    &GALVANIC_DISCHARGE,
    &MOLTEN_GATEKEEPER,
    &BASKING_BROODSCALE,
    &COLOSSAL_DREADMASK,
    &ELDRAZI_REPURPOSER,
    &EVOLUTION_WITNESS,
    &FANATIC_OF_RHONAS,
    &HORRIFIC_ASSAULT,
    &MALEVOLENT_RUMBLE,
    &NYXBORN_HYDRA,
    &SIX,
    &SOWING_MYCOSPAWN,
    &SPRINGHEART_NANTUKO,
    &TEMPERAMENTAL_OOZEWAGG,
    &CONDUIT_GOBLIN,
    &EXPANDING_OOZE,
    &FAITHFUL_WATCHDOG,
    &PHLAGE_TITAN_OF_FIRES_FURY,
    &PSYCHIC_FROG,
    &SNAPPING_VOIDCRAW,
    &WRITHING_CHRYSALIS,
    &DISRUPTOR_FLUTE,
    &BOUNTIFUL_LANDSCAPE,
    &CONTAMINATED_LANDSCAPE,
    &DECEPTIVE_LANDSCAPE,
    &FOREBODING_LANDSCAPE,
    &PERILOUS_LANDSCAPE,
    &SEETHING_LANDSCAPE,
    &SHATTERED_LANDSCAPE,
    &SHELTERING_LANDSCAPE,
    &SHIFTING_WOODLAND,
    &TRANQUIL_LANDSCAPE,
    &TWISTED_LANDSCAPE,
    &AJANI_NACATL_PARIAH,
    &WITCH_ENCHANTER,
    &SINK_INTO_STUPOR,
    &ANNOYED_ALTISAUR,
    &PRIEST_OF_TITANIA,
    &ARENA_OF_GLORY,
    &NADU_WINGED_WISDOM,
    &TAMIYO_INQUISITIVE_STUDENT,
    &SORIN_OF_HOUSE_MARKOV,
    &GUIDE_OF_SOULS,
    &CRABOMINATION,
    &WIGHT_OF_THE_RELIQUARY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&DETECTIVES_PHOENIX, 1), // MH3 457
    PrintingRecord::alternate(&SIX, 1),                // MH3 484
];
