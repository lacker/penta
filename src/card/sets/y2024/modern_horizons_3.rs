//! Modern Horizons 3 cards cataloged as attachment edge cases.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef,
    AddManaEffectDef, AggregateOperationDef, AlternativeCastKindDef, AlternativeCastManaCostDef,
    AppliedEffectDef, AppliedRuleDef, AttackEventMatcherDef, BasicLandType,
    BattlefieldEntryModificationDef, CardArt, CardChoiceSourceDef, CardRules, CardSet,
    CardSupertype, CardType, CharacteristicOperationDef, ChoiceVisibilityDef, ChooseDef,
    ChooseForEachPlayerDef, ClassifyObjectsDef, ComparisonDef, ControlDurationDef,
    CopyExceptionsDef, CostQuantityDef, CounterKind, CreatureTypeSetDef, DrawEventMatcherDef,
    EffectDef, EffectPaymentCostDef, EffectPaymentDef, EffectRecipientDef, EmblemCharacteristics,
    ExiledCastPermissionDef, HalvedValueDef, InstalledTriggerDef, InstalledTriggerLifetimeDef,
    ManaColor, ManaCost, ManaSpendEffectDef, MoveObjectsDef, ObjectChoiceBindingDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, ObjectSetFilterDef,
    ObjectSetValueAtLeastDef, ObjectSetValueDef, ObjectValueDef, PayOrDef, PerPlayerSelectionDef,
    PileExileDef, PlayerRefDef, PlayerRelation, PlayerSetDef, ReplacementEffectDef,
    ResolvedEffectDurationDef, RevealObjectsDef, RoundingDef, SacrificedAmountDef, SetOperationDef,
    SpellAdditionalCostDef, SumValueDef, TargetConditionDef, TokenCountersDef, TriggerConditionDef,
    TriggerEventDef, TurnStepDef, ValueComparisonDef, ValueDef, ZoneKind, ZonePickDef,
    ZonePlacement, abilities, tokens,
};
use crate::ids::{Binding, ParentBinding};
use crate::{TargetIndex, mana_cost};

use super::super::y2020::theros_beyond_death::escape;

const DEVOURER_TOP: Binding = Binding!("devourer_top");
const DEVOURER_EXILED: Binding = Binding!("devourer_exiled");
static DEVOURER_EXILE_REST: EffectDef = EffectDef::MoveObjects(MoveObjectsDef {
    input: ObjectSetDef::Binding(DEVOURER_EXILED),
    from: Some(ZoneKind::Library),
    zone: ZoneKind::Exile,
    placement: ZonePlacement::Top,
    moved: None,
    then: &EffectDef::None,
});
static DEVOURER_PUT_TOP: EffectDef = EffectDef::Sequence(&[
    EffectDef::MoveObjects(MoveObjectsDef {
        input: ObjectSetDef::Binding(DEVOURER_TOP),
        from: Some(ZoneKind::Library),
        zone: ZoneKind::Library,
        placement: ZonePlacement::Top,
        moved: None,
        then: &EffectDef::None,
    }),
    DEVOURER_EXILE_REST,
]);
static DEVOURER_CHOOSE: EffectDef = EffectDef::Choose(ChooseDef {
    binding: ObjectChoiceBindingDef::Objects(DEVOURER_TOP),
    unchosen: Some(DEVOURER_EXILED),
    chooser: PlayerRefDef::EffectController,
    candidates: ObjectSetDef::Binding(ParentBinding),
    exclude: None,
    minimum: 0,
    maximum: 1,
    visibility: ChoiceVisibilityDef::Private,
    then: &DEVOURER_PUT_TOP,
});
static DEVOURER_OPENING_LOOK: EffectDef = abilities::bind_top_cards_then(
    PlayerRefDef::EffectController,
    ValueDef::Constant(4),
    &DEVOURER_CHOOSE,
);

static DEVOURER_OPENING_TRIGGER: AbilityDef = AbilityDef::triggered(
    "At the beginning of your first upkeep, look at the top four cards of your library. You may put one of those cards back on top of your library. Exile the rest.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    DEVOURER_OPENING_LOOK,
);

// MH3 2 — Devourer of Destiny
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
        AbilityDef::triggered_with_targets(
            "When you cast this spell, exile target permanent that's one or more colors.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Not(
                &ObjectPredicateDef::ColorCount(0),
            ))],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
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
pub(in crate::card::sets) static AERIE_AUXILIARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5e4c134b-a416-467e-a158-def84c92c6af"),
    "Aerie Auxiliary",
    CardArt::new("5e4c134b-a416-467e-a158-def84c92c6af", "Donato Giancola"),
    CardSet::ModernHorizons3,
    // Four mana for five power across the board, in the air, which is the
    // rate a limited deck is happy with.
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Bird", "Soldier"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, support 2. (Put a +1/+1 counter on each of up to two \
             other target creatures.)",
            // One slot holding up to two targets rather than two slots: the
            // "each of" is what makes them one group, and "other" is what
            // keeps this creature out of its own support.
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                2,
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// MH3 22 — Dog Umbra
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOG_UMBRA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8d4ba710-eddb-40ca-b2fe-0e4e778aab9c"),
    "Dog Umbra",
    crate::card::CardArt::new("8d4ba710-eddb-40ca-b2fe-0e4e778aab9c", "Brian Valeza"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 34 — Mandibular Kite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANDIBULAR_KITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6b922f71-18e6-4a74-b792-d477d4a1deca"),
    "Mandibular Kite",
    crate::card::CardArt::new("6b922f71-18e6-4a74-b792-d477d4a1deca", "Bruno Biazotto"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 38 — Ocelot Pride
pub(in crate::card::sets) static OCELOT_PRIDE: CardRecord = CardRecord::new_with_legacy_id(
    2225,
    "Ocelot Pride",
    CardArt::new("89cf6f57-230f-497e-a14e-ad1e8737fd42", "Chris Seaman"),
    CardSet::ModernHorizons3,
    // Its own lifelink turns the trigger on, and once the board is wide
    // enough to ascend every Cat it ever made comes back doubled.
    CardRules::new_creature(mana_cost!("{W}"), &["Cat"], 1, 1)
        .with_abilities(&[
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
                EffectDef::Sequence(&[
                    EffectDef::create_creature_token(&["Cat"], &[ManaColor::White], 1, 1).with_art(CardArt::new(
                        "74bacab2-a4c6-4ba5-a208-6bd09ae4cf9f",
                        "Maxime Minard",
                    )),
                    // The blessing half is checked as this resolves rather than as it
                    // triggers, so ascending in response still doubles.
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ControllerHasCitysBlessing,
                        then: &EffectDef::create_token_from_copy(&crate::card::TokenCopyDef {
                                // "Until this enchantment leaves the battlefield" is one printed ability,
                                // so the return rides on the same resolution as a delayed trigger rather
                                // than appearing as a second clause the card does not print.
                                // "For each token you control that entered this turn." The Cat the clause
                                // just made is one of them, which is what makes the doubling compound.
                                object: &EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::controlled_by(
                                    ObjectPredicateDef::All(&[
                                        ObjectPredicateDef::Token,
                                        ObjectPredicateDef::CameUnderControlThisTurn,
                                    ]),
                                    &[ZoneKind::Battlefield],
                                    PlayerSetDef::Related(PlayerRelation::You),
                                ))),
                                exceptions: CopyExceptionsDef::NONE,
                            }),
                    },
                ]),
            ),
        ]),
);

// MH3 40 — Phelia, Exuberant Shepherd
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
                // "Up to one other target nonland permanent", which is what makes her a
                // blink as happily as a removal spell: the thing she takes may be yours.
                &[AbilityTargetDef::up_to(
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
                )],
                EffectDef::Sequence(&[
                    EffectDef::ExileLinkedToSource {
                        until_source_leaves: false,
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        face_down: false,
                        then: None,
                    },
                    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                        "At the beginning of the next end step, return that card to the battlefield under its \
                         owner's control. If it entered under your control, put a +1/+1 counter on this creature.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::End,
                            player: PlayerRelation::Any,
                        },
                        EffectDef::Sequence(&[
                            EffectDef::IfCondition {
                                // "If it entered under your control": what Phelia gives back goes to its
                                // owner, so who owned it is the whole of the question. Asked before the
                                // return rather than after, because by then there is no exile left to ask
                                // about.
                                condition: &TriggerConditionDef::ObjectSetCount(
                                    &crate::card::ObjectSetCountConditionDef {
                                        objects: &ObjectSetDef::LinkedExiles,
                                        predicate: crate::card::ObjectSetPredicateDef {
                                            filter: Some(ObjectSetFilterDef::Predicate(
                                                &ObjectPredicateDef::OwnedBy(PlayerRelation::You),
                                            )),
                                            comparison: ComparisonDef::GreaterOrEqual,
                                            amount: 1,
                                        },
                                    },
                                ),
                                then: &EffectDef::AddCounters {
                                    object: EffectRecipientDef::Source,
                                    kind: CounterKind::PlusOnePlusOne,
                                    amount: ValueDef::Constant(1),
                                },
                            },
                            EffectDef::ReturnLinkedExiles {
                                object: ObjectPredicateDef::Any,
                                zone: ZoneKind::Battlefield,
                                grant: None,
                                counters: None,
                                transformed: false,
                                controller: None,
                            },
                        ]),
                    ))),
                ]),
            ),
        ]),
);

// MH3 44 — Static Prison
pub(in crate::card::sets) static STATIC_PRISON: CardRecord = CardRecord::new_with_legacy_id(
    2194,
    "Static Prison",
    CardArt::new("dd16222e-349c-4a2b-a7c8-8eb35a8ab332", "Jason A. Engle"),
    CardSet::ModernHorizons3,
    // One white answers anything, and the two energy it comes with buy two
    // more turns of holding it. After that the prison opens.
    CardRules::new_enchantment(mana_cost!("{W}")).with_abilities(&[
        abilities::enters_trigger_with_targets("When this enchantment enters, exile target nonland permanent an opponent controls until this enchantment leaves the battlefield. You get {E}{E} (two energy counters).", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            },
        )], EffectDef::Sequence(&[
            // "Until this enchantment leaves the battlefield": a Prison answered
            // before its own trigger resolves exiles nobody (CR 610.3b).
            EffectDef::ExileLinkedToSource {
                until_source_leaves: true,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                face_down: false,
                then: None,
            },
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                "When this enchantment leaves the battlefield, return the exiled card to the battlefield under its owner's control.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    None,
                ),
                EffectDef::ReturnLinkedExiles {
                    object: ObjectPredicateDef::Any,
                    counters: None,
                    zone: ZoneKind::Battlefield,
                    grant: None,
                    controller: None,
                    transformed: false,
                },
            ))),
            // The energy arrives with the exile rather than paying for it: the first
            // upkeep tax is already covered, and the second is not.
            EffectDef::AddPlayerCounters {
                recipient: EffectRecipientDef::Controller,
                kind: CounterKind::named("energy"),
                amount: ValueDef::Constant(2),
            },
        ])),
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
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
    ]),
);

// MH3 45 — Thraben Charm
// Audit: unsupported — Card rules have not been implemented.
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
        // Brainstorm's two steps for one more card. The arrangement is the order
        // the two are named in: each is placed on top of the last, so the card
        // named second is the one drawn first.
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
            EffectDef::ChooseCards {
                player: EffectRecipientDef::Controller,
                sources: &[CardChoiceSourceDef::Zone(ZoneKind::Hand)],
                object: ObjectPredicateDef::Any,
                minimum: 2,
                maximum: 2,
                reveal: false,
                destination: ZoneKind::Library,
                placement: ZonePlacement::Top,
            },
        ]),
    )),
);

// MH3 69 — Serum Visionary
pub(in crate::card::sets) static SERUM_VISIONARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("08a587f5-5910-405e-8982-c889dbbc7f98"),
    "Serum Visionary",
    CardArt::new("08a587f5-5910-405e-8982-c889dbbc7f98", "Warren Mahy"),
    CardSet::ModernHorizons3,
    // Serum Visions on a body: the same draw-then-scry, so the smoothing
    // shapes the two draws after this one rather than this one.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Vedalken", "Wizard"], 2, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, draw a card, then scry 2.",
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                abilities::scry(ValueDef::Constant(2)),
            ]),
        ),
    ),
);

// MH3 80 — Accursed Marauder
pub(in crate::card::sets) static ACCURSED_MARAUDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("44a63029-1fb2-4fdc-bca9-0a530c7b42d9"),
    "Accursed Marauder",
    CardArt::new("5da14d86-0780-4821-a799-96f64b377df4", "Paolo Parente"),
    CardSet::ModernHorizons3,
    // Symmetrical on paper, one-sided in practice: the Marauder itself is a
    // legal answer for its own controller, so a token board pays nothing.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie", "Warrior"], 2, 1).with_ability(
        abilities::enters_trigger(
            "When this creature enters, each player sacrifices a nontoken creature of their choice.",
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::EachPlayer,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                ]),
                count: ValueDef::Constant(1),
                then: None,
                amount: SacrificedAmountDef::Power,
                otherwise: None,
                // "Sacrifices" rather than "may sacrifice": a player holding
                // one gives it up.
                optional: false,
            },
        ),
    ),
);

// MH3 90 — Emperor of Bones

pub(in crate::card::sets) static EMPEROR_OF_BONES: CardRecord = CardRecord::new_with_legacy_id(
    2269,
    "Emperor of Bones",
    CardArt::new("df9d9075-2d1e-4848-b661-816d539e05eb", "Josh Hass"),
    CardSet::ModernHorizons3,
    // Two mana that eats a graveyard one card a turn and then rents the best
    // of them back for an attack, which is what makes the adapt cost worth
    // paying twice.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Skeleton", "Noble"], 2, 2)
        .with_abilities(&[
            AbilityDef::triggered_with_targets(
                "At the beginning of combat on your turn, exile up to one target card from a graveyard.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::BeginningOfCombat,
                    player: PlayerRelation::You,
                },
                // Anybody's graveyard, and "up to one": an Emperor with nothing worth
                // taking still gets its combat trigger, and simply exiles nothing.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Any,
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::ExileLinkedToSource {
                    until_source_leaves: false,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    face_down: false,
                    then: None,
                },
            ),
            AbilityDef::activated(
                "{1}{B}: Adapt 2. (If this creature has no +1/+1 counters on it, put two +1/+1 counters \
                 on it.)",
                &[AbilityCostDef::Mana(mana_cost!("{1}{B}"))],
                // Adapt is a conditional rather than a cost: the ability always resolves,
                // and finding a counter already there is what makes it do nothing.
                EffectDef::IfCondition {
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
                },
            ),
            AbilityDef::triggered(
                "Whenever one or more +1/+1 counters are put on this creature, put a creature card exiled \
                 with this creature onto the battlefield under your control with a finality counter on \
                 it. It gains haste. Sacrifice it at the beginning of the next end step.",
                TriggerEventDef::CountersPlaced {
                    object: ObjectPredicateDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Object(ParentBinding),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    // "A creature card exiled with this creature": a pile no query can find,
                    // because what puts a card in it is which permanent exiled it.
                    candidates: ObjectSetDef::Matching {
                        objects: &ObjectSetDef::LinkedExiles,
                        object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::HasType(
                            CardType::Creature,
                        )),
                    },
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::PutOntoBattlefieldThen {
                        object: EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
                        binding: ParentBinding,
                        counters: Some(TokenCountersDef {
                            kind: CounterKind::Finality,
                            amount: ValueDef::Constant(1),
                        }),
                        then: &EffectDef::Sequence(&const { [
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    ParentBinding,
                                )),
                                effect: AppliedEffectDef::add_ability(&const {
                                    abilities::haste()
                                }),
                                duration: ResolvedEffectDurationDef::Permanent,
                            },
                            EffectDef::InstallTrigger(InstalledTriggerDef::once(
                                &const { AbilityDef::triggered(
                                    "At the beginning of the next end step, sacrifice that creature.",
                                    TriggerEventDef::StepBegins {
                                        step: TurnStepDef::End,
                                        player: PlayerRelation::Any,
                                    },
                                    EffectDef::Sacrifice {
                                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                            ParentBinding,
                                        )),
                                    },
                                ) },
                            )),
                        ] }),
                    },
                }),
            ),
        ]),
);

// MH3 103 — Nethergoyf
pub(in crate::card::sets) static NETHERGOYF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3ee3945e-5089-4751-b7b3-5961c39d2a33"),
    "Nethergoyf",
    CardArt::new("3ee3945e-5089-4751-b7b3-5961c39d2a33", "Xavier Ribeiro"),
    CardSet::ModernHorizons3,
    // One mana for whatever the graveyard has made of it, and the graveyard
    // pays a second time to buy it back.
    CardRules::new_creature(mana_cost!("{B}"), &["Lhurgoyf"], 0, 1)
        .with_abilities(&[
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
                        // "That number plus 1", counted over your own graveyard alone.
                        ValueDef::Sum(&SumValueDef::new(
                            ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                            ValueDef::Constant(1),
                        )),
                    ),
                },
            ),
            AbilityDef::alternative_cast_with_additional_cost(
                AlternativeCastManaCostDef::Fixed(mana_cost!("{2}{B}")),
                AlternativeCastKindDef::Escape,
                Some(
                    "Escape—{2}{B}, Exile any number of other cards from your graveyard with four or \
                     more card types among them. (You may cast this card from your graveyard for its \
                     escape cost.)",
                ),
                // The escape cost counts card types rather than cards: one Artifact
                // Creature Land pays three quarters of it by itself, which is why the deck
                // playing this is the one with a graveyard full of odd things.
                SpellAdditionalCostDef::exile(
                    ObjectPredicateDef::Any,
                    ZoneKind::Graveyard,
                    CostQuantityDef::ObjectSetValueAtLeast(&ObjectSetValueAtLeastDef {
                        value: ObjectSetValueDef::CardTypeCount,
                        minimum: 4,
                    }),
                ),
                EffectDef::None,
            ),
        ]),
);

// MH3 106 — Retrofitted Transmogrant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RETROFITTED_TRANSMOGRANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("12c1b83d-710b-4680-855a-02ba1f72abf0"),
    "Retrofitted Transmogrant",
    crate::card::CardArt::new("12c1b83d-710b-4680-855a-02ba1f72abf0", "Kekai Kotaki"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 108 — Scurrilous Sentry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCURRILOUS_SENTRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29e2805f-59fa-4a6d-97bc-266191b2aa8d"),
    "Scurrilous Sentry",
    crate::card::CardArt::new("29e2805f-59fa-4a6d-97bc-266191b2aa8d", "Leonardo Santanna"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 111 — Wither and Bloom
pub(in crate::card::sets) static WITHER_AND_BLOOM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("95c2390f-71f1-4e42-83da-d603ca86a8d0"),
    "Wither and Bloom",
    CardArt::new(
        "95c2390f-71f1-4e42-83da-d603ca86a8d0",
        "Richard Kane Ferguson",
    ),
    CardSet::ModernHorizons3,
    // Removal now and a counter later out of the same card, which is why it
    // is worth casting the front half even when it trades down.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature gets -3/-3 until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-3),
                    ValueDef::Constant(-3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_with_targets(
            "{1}{B}, Exile this card from your graveyard: Put a +1/+1 counter on target creature you control. Activate only as a sorcery.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{B}")),
                AbilityCostDef::ExileSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )
        .with_source_zones(&[ZoneKind::Graveyard])
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// MH3 114 — Amped Raptor
pub(in crate::card::sets) static AMPED_RAPTOR: CardRecord = CardRecord::new_with_legacy_id(
    2221,
    "Amped Raptor",
    CardArt::new("1ac0e78b-0fdd-44f9-8b7b-c4f28a32782e", "Alex Konstad"),
    CardSet::ModernHorizons3,
    // Two mana for a 2/1 first striker and a free spell off the top, as long
    // as the top of the deck is cheap enough for two energy to cover.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Dinosaur"], 2, 1)
        .with_abilities(&[
            abilities::first_strike(),
            abilities::enters_trigger(
                "When this creature enters, you get {E}{E} (two energy counters). Then if you cast it \
                 from your hand, exile cards from the top of your library until you exile a nonland card. \
                 You may cast that card by paying an amount of {E} equal to its mana value rather than \
                 paying its mana cost.",
                // "Then if you cast it from your hand" is part of the effect rather than an
                // intervening-if: a Raptor put onto the battlefield gets the energy and
                // nothing else.
                EffectDef::Sequence(&[
                    EffectDef::AddPlayerCounters {
                        recipient: EffectRecipientDef::Controller,
                        kind: CounterKind::named("energy"),
                        amount: ValueDef::Constant(2),
                    },
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::SourceCastFrom(ZoneKind::Hand),
                        then: &EffectDef::ExileFromTopUntil {
                            player: EffectRecipientDef::Controller,
                            // A land is what the exile walks past; the first thing that is not one is
                            // what you get to keep.
                            object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                            permission: ExiledCastPermissionDef::EnergyEqualToManaValue,
                        },
                    },
                ]),
            ),
        ]),
);

// MH3 116 — Detective's Phoenix
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
        .with_abilities(&[
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
        // Collect evidence 6 (CR 701.58a): cards out of your own graveyard whose
        // mana values add up to six, however many that takes.
        .with_alternative_additional_cost(&SpellAdditionalCostDef::exile(
            ObjectPredicateDef::Any,
            ZoneKind::Graveyard,
            CostQuantityDef::ObjectSetValueAtLeast(&ObjectSetValueAtLeastDef {
                value: ObjectSetValueDef::Aggregate {
                    select: ObjectValueDef::ManaValue,
                    operation: AggregateOperationDef::Sum,
                },
                minimum: 6,
            }),
        ))
        .with_alternative_from_graveyard(),
        abilities::flying(),
        abilities::haste(),
        // Only while it is an Aura: unattached, the recipient names nothing and
        // the clause does nothing, which is exactly CR 702.103d.
        AbilityDef::static_ability(
            "Enchanted creature gets +2/+2 and has flying and haste.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::AttachedPermanent,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                    AppliedEffectDef::add_ability(&abilities::flying()),
                    AppliedEffectDef::add_ability(&abilities::haste()),
                ]),
            },
        ),
        AbilityDef::static_ability(
            "You may cast this card from your graveyard using its bestow ability.",
            EffectDef::None,
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// MH3 122 — Galvanic Discharge
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
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Planeswalker),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::AddPlayerCounters {
                recipient: EffectRecipientDef::Controller,
                kind: CounterKind::named("energy"),
                amount: ValueDef::Constant(3),
            },
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef {
                    payer: PlayerSetDef::Related(PlayerRelation::You),
                    cost: EffectPaymentCostDef::ChosenEnergy,
                },
                // "That much damage": the amount the payment settled, which is what makes
                // the three energy it hands out into three damage the turn it is cast and
                // more than that on a board that has been banking it.
                &EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::PaidAmount,
                },
            )),
        ]),
    )),
);

// MH3 128 — Molten Gatekeeper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOLTEN_GATEKEEPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9f5ba065-2806-4e99-a330-168cfe76250f"),
    "Molten Gatekeeper",
    crate::card::CardArt::new("9f5ba065-2806-4e99-a330-168cfe76250f", "Joe Slucher"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 145 — Basking Broodscale
/// The Spawn token that more than one MH3 card prints in full: a 0/1 body
/// whose only job is to be sacrificed for one colourless mana. A static
/// rather than a const fn, because the ability slice only gets a `'static`
/// lifetime in a static initializer.
static ELDRAZI_SPAWN_TOKEN: EffectDef =
    EffectDef::create_creature_token(&["Eldrazi", "Spawn"], &[], 0, 1).with_abilities(&[
        AbilityDef::activated_mana(
            "Sacrifice this token: Add {C}.",
            &[AbilityCostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
    ]);

pub(in crate::card::sets) static BASKING_BROODSCALE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5feba5d6-99a6-4e9b-8a7d-90d955868fc3"),
    "Basking Broodscale",
    CardArt::new("5feba5d6-99a6-4e9b-8a7d-90d955868fc3", "Caio Monteiro"),
    CardSet::ModernHorizons3,
    // Adapt only ever fires once, so the token engine is what a deck is
    // really buying: anything else that puts counters on this keeps paying.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Eldrazi", "Lizard"], 2, 2).with_abilities(&[
        abilities::devoid(),
        AbilityDef::activated(
            "{1}{G}: Adapt 1. (If this creature has no +1/+1 counters on it, put a +1/+1 counter \
             on it.)",
            &[AbilityCostDef::Mana(mana_cost!("{1}{G}"))],
            // Adapt is a conditional rather than a cost: the ability always
            // resolves, and finding a counter already there is what makes it
            // do nothing.
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    comparison: ComparisonDef::LessOrEqual,
                    amount: 0,
                },
                then: &EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            },
        ),
        AbilityDef::triggered(
            "Whenever one or more +1/+1 counters are put on this creature, you may create a 0/1 \
             colorless Eldrazi Spawn creature token with \"Sacrifice this token: Add {C}.\"",
            // "One or more" is one trigger for the whole placement, not one
            // per counter.
            TriggerEventDef::CountersPlaced {
                object: ObjectPredicateDef::Source,
                kind: CounterKind::PlusOnePlusOne,
            },
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &ELDRAZI_SPAWN_TOKEN,
            },
        ),
    ]),
);

// MH3 147 — Collective Resistance
pub(in crate::card::sets) static COLLECTIVE_RESISTANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f260bd08-68b6-44f4-ace9-e298cb13d82e"),
    "Collective Resistance",
    CardArt::new("f260bd08-68b6-44f4-ace9-e298cb13d82e", "Raoul Vitale"),
    CardSet::ModernHorizons3,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::modal_escalate_spell(
        "Escalate {G} (Pay this cost for each mode chosen beyond the first.)",
        SpellAdditionalCostDef::pay_mana(mana_cost!("{G}")),
        &[
            AbilityDef::destroy_target(
                "Destroy target artifact.",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Artifact,
                )),
                true,
            ),
            AbilityDef::destroy_target(
                "Destroy target enchantment.",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Enchantment,
                )),
                true,
            ),
            AbilityDef::spell_with_targets(
                "Target creature gains hexproof and indestructible until end of turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::hexproof()),
                        AppliedEffectDef::add_ability(&abilities::indestructible()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )),
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
pub(in crate::card::sets) static ELDRAZI_REPURPOSER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("37f79ba7-7b65-4387-b498-f770816ce8dd"),
    "Eldrazi Repurposer",
    CardArt::new("37f79ba7-7b65-4387-b498-f770816ce8dd", "Daren Bader"),
    CardSet::ModernHorizons3,
    // A Spawn on the way in and another on the way out, so trading it away
    // still leaves the mana behind.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Eldrazi", "Drone"], 3, 3).with_abilities(&[
        abilities::devoid(),
        AbilityDef::triggered(
            "When you cast this spell and when this creature dies, create a 0/1 colorless \
             Eldrazi Spawn creature token with \"Sacrifice this token: Add {C}.\"",
            // One printed ability with two ways in, so what it does is
            // written once. The cast half fires while this is still on the
            // stack; the death half is a separate later trigger.
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::spell_cast(ObjectPredicateDef::Source),
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
            ]),
            ELDRAZI_SPAWN_TOKEN,
        ),
    ]),
);

// MH3 151 — Evolution Witness
pub(in crate::card::sets) static EVOLUTION_WITNESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4b4ecfa6-5e38-4c0a-91e2-f93cb492f374"),
    "Evolution Witness",
    CardArt::new("4d89283e-9783-4006-9294-4ae0473d2ce6", "Nereida"),
    CardSet::ModernHorizons3,
    // Adapt only ever fires once, so anything else that puts counters on it
    // is what turns this into a repeatable regrowth.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Shaman", "Mutant"], 2, 1)
        .with_abilities(&[
            AbilityDef::activated(
                "{1}{G}: Adapt 2. (If this creature has no +1/+1 counters on it, put two +1/+1 \
                 counters on it.)",
                &[AbilityCostDef::Mana(mana_cost!("{1}{G}"))],
                // Adapt is a conditional rather than a cost: the ability
                // always resolves, and finding a counter already there is
                // what makes it do nothing.
                EffectDef::IfCondition {
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
                },
            ),
            AbilityDef::triggered_with_targets(
                "Whenever one or more +1/+1 counters are put on this creature, return target \
                 permanent card from your graveyard to your hand.",
                // "One or more" is one trigger for the whole placement, not
                // one per counter, so adapt 2 buys back a single card.
                TriggerEventDef::CountersPlaced {
                    object: ObjectPredicateDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                },
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        // "Permanent card" is spelled out: there is no
                        // permanent card type, only the five that make one.
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
                )],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            ),
        ]),
);

static FANATIC_TAP: [AbilityCostDef; 1] = [AbilityCostDef::TapSource];

// MH3 152 — Fanatic of Rhonas
pub(in crate::card::sets) static FANATIC_OF_RHONAS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1f9fb33a-3b39-4aff-93b8-aedafe0ea694"),
    "Fanatic of Rhonas",
    CardArt::new("1f9fb33a-3b39-4aff-93b8-aedafe0ea694", "Scott Murphy"),
    CardSet::ModernHorizons3,
    // Two mana for a 1/4 that taps for one, and for four the moment anything
    // large is beside it -- and a 4/4 out of the graveyard afterwards.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Snake", "Druid"], 1, 4)
        .with_abilities(&[
            AbilityDef::activated_mana(
                "{T}: Add {G}.",
                &FANATIC_TAP,
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
            ),
            AbilityDef::activated_mana_if(
                "Ferocious — {T}: Add {G}{G}{G}{G}. Activate only if you control a creature with power 4 \
                 or greater.",
                &FANATIC_TAP,
                &TriggerConditionDef::ObjectCount {
                    // Ferocious: a creature with power four or greater, which the Fanatic is
                    // not, so something else has to be there.
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::PowerAtLeast(4),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green).with_amount(4)),
            ),
            abilities::eternalize(
                "Eternalize {2}{G}{G} ({2}{G}{G}, Exile this card from your graveyard: Create a token \
                 that's a copy of it, except it's a 4/4 black Zombie Snake Druid with no mana cost. \
                 Eternalize only as a sorcery.)",
                mana_cost!("{2}{G}{G}"),
            ),
        ]),
);

// MH3 157 — Horrific Assault
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HORRIFIC_ASSAULT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cfa6ed13-7bba-40c0-8e0e-4ffd3cea6241"),
    "Horrific Assault",
    crate::card::CardArt::new("cfa6ed13-7bba-40c0-8e0e-4ffd3cea6241", "Justine Jones"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 161 — Malevolent Rumble
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
        EffectDef::Sequence(&[
            abilities::reveal_top_cards_choose_to_hand_rest_graveyard(
                ValueDef::Constant(4),
                // "A permanent card from among them": taking nothing is a legal answer,
                // and everything not taken is buried whether or not it could have been.
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
                0,
                1,
            ),
            ELDRAZI_SPAWN_TOKEN,
        ]),
    )),
);

// MH3 164 — Nyxborn Hydra
// Audit: unsupported — Card rules have not been implemented.
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
/// Where the taken land is saved, kept apart from the milled pile so that
/// "them" and "the one you took" stay two different sets.
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
                EffectDef::Sequence(&[
                    EffectDef::BindOutput {
                        effect: &EffectDef::Mill {
                            player: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(3),
                        },
                        binding: Binding!("milled_cards"),
                    },
                    // A minimum of zero is the "you may": milling three and taking nothing is a
                    // legal answer, and a pile with no land in it never asks.
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        // "From among them" is what the mill just put there, not what the graveyard
                        // already held -- and only a land among those.
                        candidates: ObjectSetDef::Matching {
                            objects: &ObjectSetDef::Binding(Binding!("milled_cards")),
                            object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::HasType(
                                CardType::Land,
                            )),
                        },
                        exclude: None,
                        minimum: 0,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::MoveToZone {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                            zone: ZoneKind::Hand,
                            placement: ZonePlacement::Top,
                        },
                    }),
                ]),
            ),
            AbilityDef::static_ability(
                "During your turn, nonland permanent cards in your graveyard have retrace. (You \
                 may cast permanent cards from your graveyard by discarding a land card in \
                 addition to paying their other costs.)",
                EffectDef::IfCondition {
                    // "During your turn" is a gate on the permission rather than on what it
                    // names: on their turn the cards in your graveyard have nothing.
                    condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::GrantsAlternativeCastFromGraveyard {
                            // "Nonland permanent cards": what the grant reaches is every card that
                            // would become a permanent, which is the whole of what a Treefolk deck
                            // throws away.
                            object: ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ]),
                            ability: &AbilityDef::alternative_cast_for_card_mana_cost(
                                AlternativeCastKindDef::Retrace,
                                Some(
                                    "Retrace (You may cast this card from your graveyard by discarding a land card in \
                                     addition to paying its other costs.)",
                                ),
                                EffectDef::None,
                            )
                            // Retrace's own cost: the card's mana cost, plus a land out of your hand.
                            .with_alternative_additional_cost(&SpellAdditionalCostDef::discard(
                                ObjectPredicateDef::HasType(CardType::Land),
                                CostQuantityDef::Fixed(1),
                            )),
                        }),
                    },
                },
            ),
        ]),
);

// MH3 170 — Sowing Mycospawn
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
        .with_abilities(&[
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
                TriggerEventDef::spell_cast(ObjectPredicateDef::Source),
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
                TriggerEventDef::spell_cast(ObjectPredicateDef::Source),
                // The kicked half changes nothing about how the spell resolves: it costs
                // more, and the second cast trigger reads that fact. That is why the
                // alternative carries no instructions of its own.
                &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Land),
                )],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                },
            ),
        ]),
);

// MH3 171 — Springheart Nantuko
pub(in crate::card::sets) static SPRINGHEART_NANTUKO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("54a3ea87-005e-4985-b2a5-21711d0b71c0"),
    "Springheart Nantuko",
    CardArt::new("54a3ea87-005e-4985-b2a5-21711d0b71c0", "Valera Lutfullina"),
    CardSet::ModernHorizons3,
    // Two mana for a 1/1, or four to bestow it onto something worth copying
    // -- and then every land is another one of that.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Insect", "Monk"], 1, 1)
        .with_type(CardType::Enchantment)
        .with_abilities(&[
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
                TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]), None, Some(ZoneKind::Battlefield)),
                EffectDef::PayOr(
                    PayOrDef::optional_or(
                        EffectPaymentDef::mana(
                            PlayerSetDef::One(PlayerRefDef::EffectController),
                            mana_cost!("{1}{G}"),
                        ),
                        // The whole point of bestowing it: every land is another copy of whatever
                        // it is wearing.
                        &EffectDef::create_token_from_copy(&crate::card::TokenCopyDef {
                                object: &EffectRecipientDef::AttachedPermanent,
                                exceptions: CopyExceptionsDef::NONE,
                            }),
                        // "If you didn't create a token this way": declining, being unable to pay,
                        // and not being attached at all are the same answer, and each leaves an
                        // Insect behind.
                        &EffectDef::create_creature_token(&["Insect"], &[ManaColor::Green], 1, 1),
                    )
                    // "If this permanent is attached to a creature you control": read before
                    // the offer, because a Nantuko that is a creature rather than an Aura has
                    // nothing to copy and should not be asked to pay for one.
                    .only_if(&TriggerConditionDef::AttachedPermanentMatches {
                            object: ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                            ]),
                        }),
                ),
            ),
        ]),
);

// MH3 172 — Temperamental Oozewagg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEMPERAMENTAL_OOZEWAGG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6625df2e-7046-411a-ae86-c46ac0953a0b"),
    "Temperamental Oozewagg",
    crate::card::CardArt::new("6625df2e-7046-411a-ae86-c46ac0953a0b", "Pete Venters"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 179 — Conduit Goblin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONDUIT_GOBLIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5c9ad04d-c4d4-4d06-93bb-a881be733717"),
    "Conduit Goblin",
    crate::card::CardArt::new("5c9ad04d-c4d4-4d06-93bb-a881be733717", "Bruno Biazotto"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 184 — Expanding Ooze
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXPANDING_OOZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bbdb095d-b826-4e3e-8c61-0d408e52d6b8"),
    "Expanding Ooze",
    crate::card::CardArt::new("bbdb095d-b826-4e3e-8c61-0d408e52d6b8", "Randy Gallegos"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 185 — Faithful Watchdog
pub(in crate::card::sets) static FAITHFUL_WATCHDOG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b9afac99-a094-41a8-8323-90dec29691c4"),
    "Faithful Watchdog",
    CardArt::new("b9afac99-a094-41a8-8323-90dec29691c4", "Samuel Perin"),
    CardSet::ModernHorizons3,
    // Printed 0/0, so the counters are the body rather than a bonus: it
    // dies to anything that removes them.
    CardRules::new_creature(mana_cost!("{G}{W}"), &["Dog"], 0, 0).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::as_enters(
            "This creature enters with three +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 3,
                },
            ),
        ),
    ]),
);

// MH3 197 — Phlage, Titan of Fire's Fury
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
            .with_abilities(&[
                AbilityDef::triggered_if(
                    "When this creature enters, sacrifice it unless it escaped.",
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    // "Unless it escaped" reads how the spell was cast, which the permanent
                    // remembers: a Phlage cast for its printed cost sacrifices itself and leaves
                    // the Lightning Helix behind.
                    &TriggerConditionDef::Not(
                        &TriggerConditionDef::All(&[
                            TriggerConditionDef::SourceWasCast,
                            TriggerConditionDef::SourceCastFrom(ZoneKind::Graveyard),
                            TriggerConditionDef::SourceCastWith(
                                AlternativeCastKindDef::Escape,
                            ),
                        ]),
                    ),
                    EffectDef::Sacrifice {
                        object: EffectRecipientDef::Source,
                    },
                ),
                AbilityDef::triggered_with_targets(
                    "Whenever this creature enters or attacks, it deals 3 damage to any target and you gain \
                     3 life.",
                    // Entering and attacking are two ways for one printed ability to fire, so
                    // the damage and the life are written once.
                    TriggerEventDef::AnyOf(&[
                        TriggerEventDef::zone_changed(
                            ObjectPredicateDef::Source,
                            None,
                            Some(ZoneKind::Battlefield),
                        ),
                        TriggerEventDef::attacks(ObjectPredicateDef::Source),
                    ]),
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::AnyTarget,
                    )],
                    EffectDef::Sequence(&[
                        EffectDef::DealDamage {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            amount: ValueDef::Constant(3),
                        },
                        EffectDef::GainLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(3),
                        },
                    ]),
                ),
                escape(
                    AlternativeCastManaCostDef::Fixed(mana_cost!("{R}{R}{W}{W}")),
                    5,
                ),
            ]),
    );

// MH3 199 — Psychic Frog
pub(in crate::card::sets) static PSYCHIC_FROG: CardRecord = CardRecord::new_with_legacy_id(
    2277,
    "Psychic Frog",
    CardArt::new("68924203-c3d9-41ce-8ca8-c6dd491eb3ca", "Pete Venters"),
    CardSet::ModernHorizons3,
    // Two mana that turns a full hand into a big evasive body and a full
    // graveyard into the evasion, and draws a card every time it connects.
    CardRules::new_creature(mana_cost!("{U}{B}"), &["Frog"], 1, 2).with_abilities(&[
        // A player or a planeswalker: the Frog is happy to be chumped by neither.
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player or planeswalker, draw a card.",
            TriggerEventDef::combat_damage_to_player_or_planeswalker(ObjectPredicateDef::Source),
            abilities::draw_cards(ValueDef::Constant(1)),
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
        abilities::gain_ability_until_end_of_turn(
            "Exile three cards from your graveyard: This creature gains flying until end of turn.",
            &[AbilityCostDef::MoveToZone(
                crate::card::MoveToZoneCostDef::new(
                    ObjectPredicateDef::Any,
                    ZoneKind::Graveyard,
                    ZoneKind::Exile,
                    3,
                ),
            )],
            &abilities::flying(),
        ),
    ]),
);

// MH3 204 — Snapping Voidcraw
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNAPPING_VOIDCRAW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ab3a5a5-9cb1-4ee5-b7b2-d870c9a56097"),
    "Snapping Voidcraw",
    crate::card::CardArt::new("9185371c-2dde-48ad-ab27-08be04b3c522", "Camille Alquier"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 208 — Writhing Chrysalis
pub(in crate::card::sets) static WRITHING_CHRYSALIS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f54dbeb1-51f8-40e2-912a-ec25457de5a2"),
    "Writhing Chrysalis",
    CardArt::new("f54dbeb1-51f8-40e2-912a-ec25457de5a2", "Domenico Cava"),
    CardSet::ModernHorizons3,
    // Four mana for three bodies and a sacrifice engine to feed on them,
    // which is what makes the small stats beside the point.
    CardRules::new_creature(mana_cost!("{2}{R}{G}"), &["Eldrazi", "Drone"], 2, 3).with_abilities(
        &[
            abilities::devoid(),
            AbilityDef::triggered(
                "When you cast this spell, create two 0/1 colorless Eldrazi Spawn creature tokens \
             with \"Sacrifice this token: Add {C}.\"",
                // A cast trigger, so the Spawn arrive while this is still on the
                // stack and can help pay for whatever follows it.
                TriggerEventDef::spell_cast(ObjectPredicateDef::Source),
                ELDRAZI_SPAWN_TOKEN.with_amount(2),
            ),
            abilities::reach(),
            AbilityDef::triggered(
                "Whenever you sacrifice another Eldrazi, put a +1/+1 counter on this creature.",
                TriggerEventDef::Sacrificed {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype("Eldrazi"),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    player: PlayerRelation::You,
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ],
    ),
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
pub(in crate::card::sets) static BOUNTIFUL_LANDSCAPE: CardRecord = CardRecord::new_with_legacy_id(
    2265,
    "Bountiful Landscape",
    CardArt::new("b277752b-430a-4f09-8a98-b72f813dd52e", "Mark Poole"),
    CardSet::ModernHorizons3,
    // A land that taps for nothing useful and fetches a tapped basic, which
    // is worth a slot only because it is also a cycling card and because
    // what it finds is a land drop somebody else paid for.
    CardRules::new_land(&[]).with_abilities(&landscape_abilities(
        "{T}, Sacrifice this land: Search your library for a basic Forest, Island, or Mountain card, \
         put it onto the battlefield tapped, then shuffle.",
        // A basic one, not merely a card with the type: the tri-fetch cycle names
        // three basics and finds nothing else, which is what separates it from the
        // fetchlands that read "a Mountain or Plains card".
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::Supertype(CardSupertype::Basic),
            ObjectPredicateDef::HasAnyBasicLandType(&[
                BasicLandType::Forest,
                BasicLandType::Island,
                BasicLandType::Mountain,
            ]),
        ]),
        "Cycling {G}{U}{R} ({G}{U}{R}, Discard this card: Draw a card.)",
        mana_cost!("{G}{U}{R}"),
    )),
);

// MH3 218 — Contaminated Landscape
pub(in crate::card::sets) static CONTAMINATED_LANDSCAPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e2312c49-1627-47ad-8113-78a999a97d8d"),
    "Contaminated Landscape",
    CardArt::new("e2312c49-1627-47ad-8113-78a999a97d8d", "Donato Giancola"),
    CardSet::ModernHorizons3,
    // Colourless mana now, a tapped basic later, or a card when the deck
    // has enough colours to pay for the cycling.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        abilities::landscape_fetch(
            "{T}, Sacrifice this land: Search your library for a basic Plains, Island, or Swamp card, put it onto the battlefield tapped, then shuffle.",
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ObjectPredicateDef::HasAnyBasicLandType(&[
                    BasicLandType::Plains,
                    BasicLandType::Island,
                    BasicLandType::Swamp,
                ]),
            ]),
        ),
        abilities::cycling(
            "Cycling {W}{U}{B} ({W}{U}{B}, Discard this card: Draw a card.)",
            mana_cost!("{W}{U}{B}"),
        ),
    ]),
);

// MH3 219 — Deceptive Landscape
pub(in crate::card::sets) static DECEPTIVE_LANDSCAPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2ae6828e-ff19-45db-8b59-61616353491f"),
    "Deceptive Landscape",
    CardArt::new("2ae6828e-ff19-45db-8b59-61616353491f", "Erikas Perl"),
    CardSet::ModernHorizons3,
    // The white-black-green Landscape; only the three types and the cycling
    // cost below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        abilities::landscape_fetch(
            "{T}, Sacrifice this land: Search your library for a basic Plains, Swamp, or Forest card, put it onto the battlefield tapped, then shuffle.",
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ObjectPredicateDef::HasAnyBasicLandType(&[
                    BasicLandType::Plains,
                    BasicLandType::Swamp,
                    BasicLandType::Forest,
                ]),
            ]),
        ),
        abilities::cycling(
            "Cycling {W}{B}{G} ({W}{B}{G}, Discard this card: Draw a card.)",
            mana_cost!("{W}{B}{G}"),
        ),
    ]),
);

// MH3 221 — Foreboding Landscape
pub(in crate::card::sets) static FOREBODING_LANDSCAPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57fb0fa7-0c5c-4a75-9461-c51403c30282"),
    "Foreboding Landscape",
    CardArt::new("57fb0fa7-0c5c-4a75-9461-c51403c30282", "Erikas Perl"),
    CardSet::ModernHorizons3,
    // The black-green-blue Landscape; only the three types and the cycling
    // cost below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        abilities::landscape_fetch(
            "{T}, Sacrifice this land: Search your library for a basic Swamp, Forest, or Island card, put it onto the battlefield tapped, then shuffle.",
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ObjectPredicateDef::HasAnyBasicLandType(&[
                    BasicLandType::Swamp,
                    BasicLandType::Forest,
                    BasicLandType::Island,
                ]),
            ]),
        ),
        abilities::cycling(
            "Cycling {B}{G}{U} ({B}{G}{U}, Discard this card: Draw a card.)",
            mana_cost!("{B}{G}{U}"),
        ),
    ]),
);

// MH3 223 — Perilous Landscape
pub(in crate::card::sets) static PERILOUS_LANDSCAPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4b0bd07e-cf80-4d64-af29-f4cec6632b3e"),
    "Perilous Landscape",
    CardArt::new("4b0bd07e-cf80-4d64-af29-f4cec6632b3e", "Alayna Danner"),
    CardSet::ModernHorizons3,
    // The blue-red-white Landscape; only the three types and the cycling
    // cost below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        abilities::landscape_fetch(
            "{T}, Sacrifice this land: Search your library for a basic Island, Mountain, or Plains card, put it onto the battlefield tapped, then shuffle.",
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ObjectPredicateDef::HasAnyBasicLandType(&[
                    BasicLandType::Island,
                    BasicLandType::Mountain,
                    BasicLandType::Plains,
                ]),
            ]),
        ),
        abilities::cycling(
            "Cycling {U}{R}{W} ({U}{R}{W}, Discard this card: Draw a card.)",
            mana_cost!("{U}{R}{W}"),
        ),
    ]),
);

// MH3 225 — Seething Landscape
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEETHING_LANDSCAPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("661fc907-7003-45c6-820c-9616e9a71c30"),
    "Seething Landscape",
    crate::card::CardArt::new("661fc907-7003-45c6-820c-9616e9a71c30", "Piotr Dura"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 226 — Shattered Landscape
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHATTERED_LANDSCAPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b3da28c7-6e92-439d-a163-91682d4f11dc"),
    "Shattered Landscape",
    crate::card::CardArt::new("b3da28c7-6e92-439d-a163-91682d4f11dc", "Erikas Perl"),
    crate::card::CardSet::ModernHorizons3,
    crate::card::CardRules::unsupported(),
);

// MH3 227 — Sheltering Landscape
pub(in crate::card::sets) static SHELTERING_LANDSCAPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0fe070f4-8877-4280-b8fd-869f3ac34ab6"),
    "Sheltering Landscape",
    CardArt::new("0fe070f4-8877-4280-b8fd-869f3ac34ab6", "Erikas Perl"),
    CardSet::ModernHorizons3,
    // The same bargain as its Temur cousin: a colourless tap nobody wants, a
    // tapped basic when you have the land drop, and a card when you do not.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{T}, Sacrifice this land: Search your library for a basic Mountain, Forest, or Plains \
             card, put it onto the battlefield tapped, then shuffle.",
            &LANDSCAPE_FETCH_COST,
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                // The Naya half of the same cycle, and the same shape: three basics, a
                // tapped land, and a cycling cost nobody pays for the mana.
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ObjectPredicateDef::HasAnyBasicLandType(&[
                        BasicLandType::Mountain,
                        BasicLandType::Forest,
                        BasicLandType::Plains,
                    ]),
                ]),
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
    ]),
);

// MH3 228 — Shifting Woodland
pub(in crate::card::sets) static SHIFTING_WOODLAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("059164e1-894d-4586-9800-e60d6fbd6eb6"),
    "Shifting Woodland",
    CardArt::new("059164e1-894d-4586-9800-e60d6fbd6eb6", "Josu Hernaiz"),
    CardSet::ModernHorizons3,
    // A Forest that turns into the best thing you have already lost, once
    // the graveyard is deep enough to be worth reading.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped_unless_you_control(
            "This land enters tapped unless you control a Forest.",
            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ),
        // No "except it has this ability" clause, unlike Thespian's Stage: the
        // copy replaces every copiable value, so while it is a creature it is
        // not a land, taps for nothing, and cannot do this again.
        AbilityDef::activated_with_targets(
            "Delirium — {2}{G}{G}: This land becomes a copy of target permanent card in your \
             graveyard until end of turn. Activate only if there are four or more card types among \
             cards in your graveyard.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{G}{G}"))],
            // "Target permanent card in your graveyard": the five permanent types, in
            // your own graveyard rather than either.
            &[AbilityTargetDef::exactly_one(
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
            )],
            EffectDef::BecomeCopyOf {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                copier: None,
                exceptions: CopyExceptionsDef::NONE,
                duration: Some(ResolvedEffectDurationDef::UntilEndOfTurn),
            },
        )
        .with_activation_condition(
            &// Delirium, as an activation restriction rather than a trigger condition:
            // the ability is not offered at all while the graveyard is short of four
            // card types.
            TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                comparison: ComparisonDef::GreaterOrEqual,
                right: ValueDef::Constant(4),
            }),
        ),
    ]),
);

// MH3 231 — Tranquil Landscape
pub(in crate::card::sets) static TRANQUIL_LANDSCAPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("113f48b9-a972-4e2c-af95-05ab078e01f2"),
    "Tranquil Landscape",
    CardArt::new("113f48b9-a972-4e2c-af95-05ab078e01f2", "Randy Gallegos"),
    CardSet::ModernHorizons3,
    // The green-white-blue Landscape; only the three types and the cycling
    // cost below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        abilities::landscape_fetch(
            "{T}, Sacrifice this land: Search your library for a basic Forest, Plains, or Island card, put it onto the battlefield tapped, then shuffle.",
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ObjectPredicateDef::HasAnyBasicLandType(&[
                    BasicLandType::Forest,
                    BasicLandType::Plains,
                    BasicLandType::Island,
                ]),
            ]),
        ),
        abilities::cycling(
            "Cycling {G}{W}{U} ({G}{W}{U}, Discard this card: Draw a card.)",
            mana_cost!("{G}{W}{U}"),
        ),
    ]),
);

// MH3 232 — Twisted Landscape
pub(in crate::card::sets) static TWISTED_LANDSCAPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0d647d67-f963-43b4-ade8-6c90e91f65ac"),
    "Twisted Landscape",
    CardArt::new("d0e3e7b3-7ba9-47a2-b46c-a40bffb445e2", "Piotr Dura"),
    CardSet::ModernHorizons3,
    // The land drop it finds is the point; cycling is what it does on the
    // turns the deck already has enough of them.
    CardRules::new_land(&[]).with_abilities(&landscape_abilities(
        "{T}, Sacrifice this land: Search your library for a basic Swamp, Mountain, or Forest card, \
         put it onto the battlefield tapped, then shuffle.",
        // The Jund member: the same land with the other three basics on it.
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::Supertype(CardSupertype::Basic),
            ObjectPredicateDef::HasAnyBasicLandType(&[
                BasicLandType::Swamp,
                BasicLandType::Mountain,
                BasicLandType::Forest,
            ]),
        ]),
        "Cycling {B}{R}{G} ({B}{R}{G}, Discard this card: Draw a card.)",
        mana_cost!("{B}{R}{G}"),
    )),
);

// MH3 237 — Ajani, Nacatl Pariah // Ajani, Nacatl Avenger
pub(in crate::card::sets) static AJANI_NACATL_PARIAH: CardRecord =
    CardRecord::new_dfc_with_legacy_id(
        2199,
        "Ajani, Nacatl Pariah // Ajani, Nacatl Avenger",
        CardArt::new("0d16e8e0-31b2-4389-afd6-783c501f6fa0", "Chris Rallis"),
        CardSet::ModernHorizons3,
        &[
            (
                "Ajani, Nacatl Pariah",
                const {
                    CardRules::new_creature(mana_cost!("{1}{W}"), &const { ["Cat", "Warrior"] }, 1, 2)
                    .with_supertype(CardSupertype::Legendary)
                    .with_abilities(&const { [
                        abilities::enters_trigger(
                            "When Ajani enters, create a 2/1 white Cat Warrior creature token.",
                            EffectDef::create_creature_token(&const { ["Cat", "Warrior"] }, &const { [ManaColor::White] }, 2, 1).with_art(
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
                                // The Cats that matter are the other ones: Ajani dying alongside them does
                                // not turn him over, and neither does his own death.
                                ObjectPredicateDef::All(&const { [
                                    ObjectPredicateDef::Subtype("Cat"),
                                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                ] }),
                                Some(ZoneKind::Battlefield),
                                Some(ZoneKind::Graveyard),
                            ),
                            EffectDef::May {
                                player: EffectRecipientDef::Controller,
                                // "Exile Ajani, then return him to the battlefield transformed." One
                                // resolution: the exile links him to himself and the return brings him
                                // straight back on the other face, under his owner's control.
                                effect: &EffectDef::Sequence(&const { [
                                    EffectDef::ExileLinkedToSource {
                                        until_source_leaves: false,
                                        object: EffectRecipientDef::Source,
                                        face_down: false,
                                        then: None,
                                    },
                                    EffectDef::ReturnLinkedExiles {
                                        object: ObjectPredicateDef::Any,
                                        counters: None,
                                        zone: ZoneKind::Battlefield,
                                        grant: None,
                                        controller: None,
                                        transformed: true,
                                    },
                                ] }),
                            },
                        ),
                    ] })
                },
            ),
            (
                "Ajani, Nacatl Avenger",
                const {
                    CardRules::new_planeswalker_without_mana_cost(&const { ["Ajani"] })
                    .with_supertype(CardSupertype::Legendary)
                    .with_starting_loyalty(3)
                    .with_abilities(&const { [
                        AbilityDef::activated(
                            "+2: Put a +1/+1 counter on each Cat you control.",
                            &const { [AbilityCostDef::Loyalty(2)] },
                            EffectDef::AddCounters {
                                object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                                    ObjectPredicateDef::Subtype("Cat"),
                                    &const { [ZoneKind::Battlefield] },
                                    PlayerRelation::You,
                                ))),
                                kind: CounterKind::PlusOnePlusOne,
                                amount: ValueDef::Constant(1),
                            },
                        ),
                        AbilityDef::activated_with_targets(
                            "0: Create a 2/1 white Cat Warrior creature token. When you do, if you control a red permanent other than Ajani, he deals damage equal to the number of creatures you control to any target.",
                            &const { [AbilityCostDef::Loyalty(0)] },
                            &const { [AbilityTargetDef::exactly_one(
                                AbilityTargetPredicate::AnyTarget,
                            )] },
                            // The reflexive "when you do" is folded into this resolution: the token is
                            // made, and then the damage happens if the condition holds. What that
                            // costs is the separate window between the two and the chance to decline
                            // the damage; the target is named as the ability is activated instead of
                            // after the token appears, and there is always a legal one because a
                            // player is a legal target.
                            EffectDef::Sequence(&const { [
                                EffectDef::create_creature_token(&const { ["Cat", "Warrior"] }, &const { [ManaColor::White] }, 2, 1).with_art(
                                    CardArt::new("ce5c5bcf-1fdd-4d73-a92b-223292da00ca", "Ben Wootten"),
                                ),
                                EffectDef::IfCondition {
                                    // "If you control a red permanent other than Ajani." Ajani himself is
                                    // white, so the clause is about a second permanent rather than about him.
                                    condition: &TriggerConditionDef::ObjectCount {
                                        query: ObjectQueryDef::matching(
                                            ObjectPredicateDef::All(&const { [
                                                ObjectPredicateDef::Color(ManaColor::Red),
                                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                            ] }),
                                            &const { [ZoneKind::Battlefield] },
                                            PlayerRelation::You,
                                        ),
                                        comparison: ComparisonDef::GreaterOrEqual,
                                        amount: 1,
                                    },
                                    then: &EffectDef::DealDamage {
                                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                        amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                            ObjectPredicateDef::HasType(CardType::Creature),
                                            &const { [ZoneKind::Battlefield] },
                                            PlayerRelation::You,
                                        )),
                                    },
                                },
                            ] }),
                        ),
                        AbilityDef::activated(
                            "−4: Each opponent chooses an artifact, a creature, an enchantment, and a planeswalker from among the nonland permanents they control, then sacrifices the rest.",
                            &const { [AbilityCostDef::Loyalty(-4)] },
                            EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                                player: EffectRecipientDef::Opponent,
                                candidates: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                                zone: ZoneKind::Battlefield,
                                // The four roles the ultimate lets each opponent fill. Order is printed
                                // order, which is also APNAP choice order within one player's selection.
                                selection: PerPlayerSelectionDef::OneOfEach(&const { [
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::HasType(CardType::Enchantment),
                                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                                ] }),
                                visibility: ChoiceVisibilityDef::Public,
                                chosen: Binding!("ugin_spared_permanents"),
                                unchosen: Binding!("ugin_sacrificed_permanents"),
                                then: &const { EffectDef::Sacrifice {
                                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("ugin_sacrificed_permanents"))),
                                } },
                            }),
                        ),
                    ] })
                },
            ),
        ],
    );

// MH3 239 — Witch Enchanter // Witch-Blessed Meadow
pub(in crate::card::sets) static WITCH_ENCHANTER: CardRecord = CardRecord::new_mdfc(
    PrintingAnchor::scryfall("62061e7c-cf19-4f03-b8fa-2bdba62d6b0b"),
    "Witch Enchanter // Witch-Blessed Meadow",
    CardArt::new("62061e7c-cf19-4f03-b8fa-2bdba62d6b0b", "Tyler Walpole"),
    CardSet::ModernHorizons3,
    &[
        (
            "Witch Enchanter",
            const {
                CardRules::new_creature(mana_cost!("{3}{W}"), &const { ["Human", "Warlock"] }, 2, 2)
                .with_abilities(&const { [abilities::enters_trigger_with_targets(
                    "When this creature enters, destroy target artifact or enchantment an opponent controls.",
                    // "Target artifact or enchantment an opponent controls": two types and a
                    // controller, which together are the whole restriction.
                    &const { [AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::All(&const { [
                            ObjectPredicateDef::AnyOf(&const { [
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                            ] }),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                        ] }),
                    )] },
                    EffectDef::destroy_target(TargetIndex::PRIMARY, true),
                )] })
            },
        ),
        (
            "Witch-Blessed Meadow",
            const {
                CardRules::new_land(&const { [] }).with_abilities(&const { [
                AbilityDef::replacement(
                    "As this land enters, you may pay 3 life. If you don't, it enters tapped.",
                    ReplacementEffectDef::PayOr {
                        payment: EffectPaymentDef::life(PlayerSetDef::Related(PlayerRelation::You), 3),
                        if_paid: &const { [] },
                        // Declining is what taps it, so the paid branch does nothing and the
                        // declined branch is the whole of the cost.
                        if_declined: &const { [ReplacementEffectDef::ModifyBattlefieldEntry(
                                BattlefieldEntryModificationDef::Tapped,
                            )] },
                    },
                ),
                AbilityDef::activated_mana(
                    "{T}: Add {W}.",
                    &const { [AbilityCostDef::TapSource] },
                    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
                ),
            ] })
            },
        ),
    ],
);

// MH3 241 — Sink into Stupor // Soporific Springs
pub(in crate::card::sets) static SINK_INTO_STUPOR: CardRecord = CardRecord::new_mdfc(
    PrintingAnchor::scryfall("5358b87a-1a29-426d-b165-40c97da2c14d"),
    "Sink into Stupor // Soporific Springs",
    CardArt::new("5358b87a-1a29-426d-b165-40c97da2c14d", "Peter Polach"),
    CardSet::ModernHorizons3,
    &[
        (
            "Sink into Stupor",
            const {
                CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
                "Return target spell or nonland permanent an opponent controls to its owner's hand.",
                &const { [AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::AnyOf(&const { [
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::Spell,
                                zones: &[ZoneKind::Stack],
                                controller: Some(PlayerRelation::Opponent),
                                owner: None,
                            },
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                                zones: &[ZoneKind::Battlefield],
                                controller: Some(PlayerRelation::Opponent),
                                owner: None,
                            },
                        ] }),
                    )] },
                // Returning a spell is not countering it: one that cannot be countered is
                // answered all the same, and its controller keeps the card.
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            ))
            },
        ),
        (
            "Soporific Springs",
            const {
                CardRules::new_land(&const { [] }).with_abilities(&const { [
                AbilityDef::replacement(
                    "As this land enters, you may pay 3 life. If you don't, it enters tapped.",
                    ReplacementEffectDef::PayOr {
                        payment: EffectPaymentDef::life(PlayerSetDef::Related(PlayerRelation::You), 3),
                        if_paid: &const { [] },
                        if_declined: &const { [ReplacementEffectDef::ModifyBattlefieldEntry(
                                BattlefieldEntryModificationDef::Tapped,
                            )] },
                    },
                ),
                AbilityDef::activated_mana(
                    "{T}: Add {U}.",
                    &const { [AbilityCostDef::TapSource] },
                    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
                ),
            ] })
            },
        ),
    ],
);

// MH3 284 — Annoyed Altisaur
pub(in crate::card::sets) static ANNOYED_ALTISAUR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7536d618-0c98-45bb-913b-b8117b4acf87"),
    "Annoyed Altisaur",
    CardArt::new("4aa9354d-3496-47f4-81c9-aead15efb8bb", "Lars Grant-West"),
    CardSet::ModernHorizons3,
    // Seven mana with cascade attached, which is why a limited deck plays it
    // as two cards rather than as an expensive one.
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Dinosaur"], 6, 5).with_abilities(&[
        abilities::reach(),
        abilities::trample(),
        abilities::cascade(),
    ]),
);

// MH3 286 — Priest of Titania
pub(in crate::card::sets) static PRIEST_OF_TITANIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("965c33c3-0c68-4516-b8b0-5a0552ed44b6"),
    "Priest of Titania",
    CardArt::new("eb11921b-1b28-483f-a707-4de21a6daa31", "Rebecca Guay"),
    CardSet::ModernHorizons3,
    // Every Elf on the battlefield, not only yours, and the Priest is an Elf
    // herself -- so she taps for at least one the turn she can.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Druid"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {G} for each Elf on the battlefield.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddManaEqualTo {
                color: ManaColor::Green,
                amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::new(
                    ObjectPredicateDef::Subtype("Elf"),
                    &[ZoneKind::Battlefield],
                )),
            },
        ),
    ),
);

// MH3 351 — Arena of Glory
pub(in crate::card::sets) static ARENA_OF_GLORY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3d7d07bb-b875-4a6d-8b87-4187e823af75"),
    "Arena of Glory",
    crate::card::CardArt::new("3d7d07bb-b875-4a6d-8b87-4187e823af75", "Piotr Dura"),
    crate::card::CardSet::ModernHorizons3,
    // A red source that costs nothing to play and turns one creature a game
    // into a surprise, which is what a haste land is for.
    CardRules::new_land(&[]).with_abilities(&[
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
            // {R} in, {R}{R} out, and one untap step owed: the land pays for the haste
            // out of next turn rather than out of this one.
            &[
                AbilityCostDef::Mana(mana_cost!("{R}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::ExertSource,
            ],
            EffectDef::AddMana(
                AddManaEffectDef::one(ManaColor::Red)
                    .with_amount(2)
                    // The rider asks what the mana paid for rather than restricting what it may
                    // pay for: this mana casts anything, and only a creature gets anything out
                    // of it.
                    .with_spend_effects(&[ManaSpendEffectDef::ApplyToPaidSpellMatching {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        effect: AppliedEffectDef::add_ability(&abilities::haste()),
                    }]),
            ),
        ),
    ]),
);

// MH3 377 — Nadu, Winged Wisdom
/// One card off the top, sorted by whether it is a land: the land goes to
/// the battlefield and anything else goes to the hand, so nothing is left
/// for the player to decide.
const NADU_LAND: Binding = Binding!("nadu_land");
const NADU_NONLAND: Binding = Binding!("nadu_nonland");
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
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::static_ability(
                "Creatures you control have \"Whenever this creature becomes the target of a spell or \
                 ability, reveal the top card of your library. If it's a land card, put it onto the \
                 battlefield. Otherwise, put it into your hand. This ability triggers only twice each \
                 turn.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    // The granted ability, carried by each creature rather than by Nadu: the
                    // cap is on one creature's copy of it, so every creature you control has
                    // two of these a turn.
                    effect: AppliedEffectDef::add_ability(&const {
                        AbilityDef::triggered(
                            "Whenever this creature becomes the target of a spell or ability, reveal the top card of \
                             your library. If it's a land card, put it onto the battlefield. Otherwise, put it into your \
                             hand. This ability triggers only twice each turn.",
                            TriggerEventDef::becomes_targeted(
                                ObjectPredicateDef::Any,
                            ),
                            abilities::bind_top_cards_then(
                                PlayerRefDef::EffectController,
                                ValueDef::Constant(1),
                                &const {
                                    EffectDef::Sequence(&[
                                        EffectDef::RevealObjects(RevealObjectsDef {
                                            input: ObjectSetDef::Binding(ParentBinding),
                                            then: &EffectDef::None,
                                        }),
                                        EffectDef::ClassifyObjects(ClassifyObjectsDef {
                                                input: ObjectSetDef::Binding(ParentBinding),
                                                object: ObjectPredicateDef::HasType(CardType::Land),
                                                matching: NADU_LAND,
                                                remainder: NADU_NONLAND,
                                                then: &const {
                                                    EffectDef::Sequence(&[
                                                        EffectDef::MoveObjects(MoveObjectsDef {
                                                            input: ObjectSetDef::Binding(NADU_LAND),
                                                            from: Some(ZoneKind::Library),
                                                            zone: ZoneKind::Battlefield,
                                                            placement: ZonePlacement::Top,
                                                            moved: None,
                                                            then: &EffectDef::None,
                                                        }),
                                                        EffectDef::MoveObjects(MoveObjectsDef {
                                                                input: ObjectSetDef::Binding(
                                                                    NADU_NONLAND,
                                                                ),
                                                                from: Some(ZoneKind::Library),
                                                                zone: ZoneKind::Hand,
                                                                placement: ZonePlacement::Top,
                                                                moved: None,
                                                                then: &EffectDef::None,
                                                        }),
                                                    ])
                                                },
                                        }),
                                    ])
                                },
                            ),
                        )
                        .triggering_at_most(2)
                    }),
                },
            ),
        ]),
);

// MH3 443 — Tamiyo, Inquisitive Student // Tamiyo, Seasoned Scholar
pub(in crate::card::sets) static TAMIYO_INQUISITIVE_STUDENT: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("1b234fee-a2b6-4661-9f98-4da6fc26aebc"),
    "Tamiyo, Inquisitive Student // Tamiyo, Seasoned Scholar",
    CardArt::new("1b234fee-a2b6-4661-9f98-4da6fc26aebc", "Evyn Fong"),
    CardSet::ModernHorizons3,
    &[
        (
            "Tamiyo, Inquisitive Student",
            const {
                CardRules::new_creature(mana_cost!("{U}"), &const { ["Moonfolk", "Wizard"] }, 0, 3)
                    .with_supertype(CardSupertype::Legendary)
                    .with_abilities(&const { [
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
                            // Exile and return, which is how a permanent turns over into a new object
                            // rather than merely flipping: the Tamiyo that comes back has no counters,
                            // no summoning history, and a fresh set of loyalty.
                            EffectDef::Sequence(&const { [
                                EffectDef::ExileLinkedToSource {
                                    until_source_leaves: false,
                                    object: EffectRecipientDef::Source,
                                    face_down: false,
                                    then: None,
                                },
                                EffectDef::ReturnLinkedExiles {
                                    object: ObjectPredicateDef::Any,
                                    counters: None,
                                    zone: ZoneKind::Battlefield,
                                    grant: None,
                                    controller: None,
                                    transformed: true,
                                },
                            ] }),
                        ),
                    ] })
            },
        ),
        (
            "Tamiyo, Seasoned Scholar",
            const {
                CardRules::new_planeswalker_without_mana_cost(&const { ["Tamiyo"] })
                .with_supertype(CardSupertype::Legendary)
                // The back face has no mana cost to read a colour off, and
                // prints a colour indicator instead: she is blue on both
                // sides.
                .printed_colors(&const { [ManaColor::Blue] })
                .with_starting_loyalty(2)
                .with_abilities(&const { [
                    AbilityDef::activated(
                        "+2: Until your next turn, whenever a creature attacks you or a planeswalker you \
                         control, it gets -1/-0 until end of turn.",
                        &const { [AbilityCostDef::Loyalty(2)] },
                        EffectDef::InstallTrigger(InstalledTriggerDef {
                            // The attackers her plus ability shrinks. It is installed on resolution and
                            // watches until her controller's next turn, so it catches the attack it was
                            // played to blunt.
                            ability: &const { AbilityDef::triggered(
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
                            ) },
                            lifetime: InstalledTriggerLifetimeDef::UntilNextTurn(PlayerRefDef::EffectController),
                        }),
                    ),
                    AbilityDef::activated_with_targets(
                        "−3: Return target instant or sorcery card from your graveyard to your hand. If it's a \
                         green card, add one mana of any color.",
                        &const { [AbilityCostDef::Loyalty(-3)] },
                        &const { [AbilityTargetDef::exactly_one(
                                AbilityTargetPredicate::Object {
                                    object: ObjectPredicateDef::AnyOf(&const { [
                                        ObjectPredicateDef::HasType(CardType::Instant),
                                        ObjectPredicateDef::HasType(CardType::Sorcery),
                                    ] }),
                                    zones: &const { [ZoneKind::Graveyard] },
                                    controller: None,
                                    owner: Some(PlayerRelation::You),
                                },
                            )] },
                        EffectDef::Sequence(&const { [
                            EffectDef::MoveToZone {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                zone: ZoneKind::Hand,
                                placement: ZonePlacement::Top,
                            },
                            EffectDef::AddMana(
                                AddManaEffectDef::any_color()
                                    // "If it's a green card, add one mana of any color." One mana when the card
                                    // returned was green and none otherwise, which is the whole rider: an
                                    // amount rather than a branch, read off the target the clause already has.
                                    .with_variable_amount(ValueDef::IfTargetMatches(&TargetConditionDef {
                                        slot: TargetIndex::PRIMARY,
                                        object: ObjectPredicateDef::Color(ManaColor::Green),
                                        then: ValueDef::Constant(1),
                                        otherwise: ValueDef::Constant(0),
                                    })),
                            ),
                        ] }),
                    ),
                    AbilityDef::activated(
                        "−7: Draw cards equal to half the number of cards in your library, rounded up. You get \
                         an emblem with \"You have no maximum hand size.\"",
                        &const { [AbilityCostDef::Loyalty(-7)] },
                        EffectDef::Sequence(&const { [
                            EffectDef::DrawCards {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::Halved(&HalvedValueDef::new(ValueDef::LibrarySize(PlayerRelation::You), RoundingDef::Up)),
                            },
                            EffectDef::CreateEmblem {
                                emblem: EmblemCharacteristics::new("Tamiyo, Seasoned Scholar emblem", &const { [AbilityDef::static_ability(
                                        "You have no maximum hand size.",
                                        EffectDef::StaticApply {
                                            recipient: EffectRecipientDef::Controller,
                                            effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                                                crate::card::PlayerRuleDef::NoMaximumHandSize,
                                            )),
                                        },
                                    )] }),
                            },
                        ] }),
                    ),
                ] })
            },
        ),
    ],
);

// MH3 444 — Sorin of House Markov // Sorin, Ravenous Neonate
pub(in crate::card::sets) static SORIN_OF_HOUSE_MARKOV: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("0347bf13-1ccb-4d4d-a5f2-68181d494b85"),
    "Sorin of House Markov // Sorin, Ravenous Neonate",
    crate::card::CardArt::new("0347bf13-1ccb-4d4d-a5f2-68181d494b85", "Livia Prima"),
    crate::card::CardSet::ModernHorizons3,
    &[
        (
            "Sorin of House Markov",
            const {
                CardRules::new_creature(mana_cost!("{1}{B}"), &const { ["Human", "Noble"] }, 1, 4)
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&const { [
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
                        &// Three life in a turn, counted as a running total: gaining it and losing
                            // it again still turns him over, because what the clause reads is the
                            // gaining rather than where the life total ended up.
                            TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                                left: ValueDef::LifeGainedThisTurn(PlayerRelation::You),
                                comparison: ComparisonDef::GreaterOrEqual,
                                right: ValueDef::Constant(3),
                            }),
                        // The same exile-and-return Ajani uses: one resolution, so he is gone and
                        // back before anything else happens, and he comes back a new object with
                        // his printed loyalty.
                        EffectDef::Sequence(&const { [
                            EffectDef::ExileLinkedToSource {
                                until_source_leaves: false,
                                object: EffectRecipientDef::Source,
                                face_down: false,
                                then: None,
                            },
                            EffectDef::ReturnLinkedExiles {
                                object: ObjectPredicateDef::Any,
                                counters: None,
                                zone: ZoneKind::Battlefield,
                                grant: None,
                                controller: None,
                                transformed: true,
                            },
                        ] }),
                    ),
                ] })
            },
        ),
        (
            "Sorin, Ravenous Neonate",
            const {
                // The back face has no mana cost, so its colours come from the printed
                // indicator. They matter to his own ultimate: he is a white permanent,
                // and the clause has to say "other than Sorin" precisely because of it.
                CardRules::new_planeswalker_without_mana_cost(&const { ["Sorin"] })
                .with_supertype(CardSupertype::Legendary)
                .with_starting_loyalty(3)
                .printed_colors(&const { [ManaColor::White, ManaColor::Black] })
                .with_abilities(&const { [
                    abilities::extort(),
                    AbilityDef::activated(
                        "+2: Create a Food token.",
                        &const { [AbilityCostDef::Loyalty(2)] },
                        EffectDef::create_token(tokens::food()),
                    ),
                    // The same tally the front face reads to turn over, spent here as
                    // damage: the lifelink body he arrived as is what loads this.
                    AbilityDef::activated_with_targets(
                        "\u{2212}1: Sorin deals damage equal to the amount of life you gained this turn to any \
                         target.",
                        &const { [AbilityCostDef::Loyalty(-1)] },
                        &const { [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::AnyTarget,
                        )] },
                        EffectDef::DealDamage {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            amount: ValueDef::LifeGainedThisTurn(PlayerRelation::You),
                        },
                    ),
                    AbilityDef::activated_with_targets(
                        "\u{2212}6: Gain control of target creature. It becomes a Vampire in addition to its \
                         other types. Put a lifelink counter on it if you control a white permanent other than \
                         that creature or Sorin.",
                        &const { [AbilityCostDef::Loyalty(-6)] },
                        &const { [AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )] },
                        EffectDef::Sequence(&const { [
                            EffectDef::GainControl {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                controller: PlayerRefDef::EffectController,
                                duration: ControlDurationDef::Indefinitely,
                            },
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                // "It becomes a Vampire in addition to its other types": added rather than
                                // set, and with no duration, so what it was stays and the Vampire sticks.
                                effect: AppliedEffectDef::Characteristic(CharacteristicOperationDef::CreatureTypes(
                                        SetOperationDef::Add(CreatureTypeSetDef::named(&const { ["Vampire"] })),
                                    )),
                                duration: ResolvedEffectDurationDef::Permanent,
                            },
                            EffectDef::IfCondition {
                                // "A white permanent other than that creature or Sorin." The source is
                                // Sorin; the creature is the one this ability just took, which is why the
                                // query has to leave the target out rather than merely counting what you
                                // control.
                                condition: &TriggerConditionDef::ObjectCount {
                                    query: ObjectQueryDef::matching(
                                        ObjectPredicateDef::All(&const { [
                                            ObjectPredicateDef::Color(ManaColor::White),
                                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                        ] }),
                                        &const { [ZoneKind::Battlefield] },
                                        PlayerRelation::You,
                                    )
                                    .excluding_target(TargetIndex::PRIMARY),
                                    comparison: ComparisonDef::GreaterOrEqual,
                                    amount: 1,
                                },
                                then: &EffectDef::AddCounters {
                                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                    kind: CounterKind::Lifelink,
                                    amount: ValueDef::Constant(1),
                                },
                            },
                        ] }),
                    ),
                ] })
            },
        ),
    ],
);

// MH3 448 — Guide of Souls
pub(in crate::card::sets) static GUIDE_OF_SOULS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("298de33f-cb39-47c5-9579-54d91eb34414"),
    "Guide of Souls",
    CardArt::new("298de33f-cb39-47c5-9579-54d91eb34414", "Ryan Valle"),
    CardSet::ModernHorizons3,
    // A one-mana body that turns every other creature into a life and an
    // energy, and then spends the energy making one of them an Angel.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Cleric"], 1, 2)
        .with_abilities(&[
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
                // One life and one energy per creature, which is what makes the three-
                // energy payment a matter of a turn or two rather than a deck built for it.
                EffectDef::Sequence(&[
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::AddPlayerCounters {
                        recipient: EffectRecipientDef::Controller,
                        kind: CounterKind::named("energy"),
                        amount: ValueDef::Constant(1),
                    },
                ]),
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
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Attacking,
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::PayOr(PayOrDef::optional(
                    EffectPaymentDef {
                        payer: PlayerSetDef::Related(PlayerRelation::You),
                        cost: EffectPaymentCostDef::Energy(3),
                    },
                    // All three stick: the counters and the type are permanent, so the
                    // creature is still a flying Angel next turn.
                    &EffectDef::Sequence(&[
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
                    ]),
                )),
            ),
        ]),
);

// MH3 452 — Crabomination
pub(in crate::card::sets) static CRABOMINATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b6ac511f-6c28-45f9-968b-9ac72872641b"),
    "Crabomination",
    CardArt::new("b6ac511f-6c28-45f9-968b-9ac72872641b", "Nicholas Gregory"),
    CardSet::ModernHorizons3,
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Crab", "Demon"], 5, 5).with_abilities(&[
        AbilityDef::alternative_cast_with_additional_cost(
            AlternativeCastManaCostDef::Fixed(mana_cost!("{5}{B}{B}")),
            AlternativeCastKindDef::Emerge,
            None,
            // The reduction the keyword applies is generic only, so a big
            // enough artifact still leaves both black pips owed.
            SpellAdditionalCostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Artifact),
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::None,
        ),
        abilities::enters_trigger_with_targets(
            "When this creature enters, target opponent exiles the top card of their library, a card at random from their graveyard, and a card at random from their hand. You may cast a spell from among cards exiled this way without paying its mana cost.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                PlayerRelation::Opponent,
            ))],
            // One pile out of three zones, and one permission over the whole
            // pile: the free cast is spent on whichever of the three is
            // worth having.
            EffectDef::ExileOneFromEachZone(&PileExileDef {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zones: &[
                    ZonePickDef::top(ZoneKind::Library),
                    // A library has an order to read from; a hand and a
                    // graveyard do not, which is why the card says "at
                    // random" for those two.
                    ZonePickDef::at_random(ZoneKind::Graveyard),
                    ZonePickDef::at_random(ZoneKind::Hand),
                ],
                permission: Some(ExiledCastPermissionDef::FreeWhileResolving),
            }),
        ),
    ]),
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

pub(in crate::card::sets) static WIGHT_OF_THE_RELIQUARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("915715f7-5487-47aa-ada5-de1bce282164"),
    "Wight of the Reliquary",
    CardArt::new("915715f7-5487-47aa-ada5-de1bce282164", "Scott Murphy"),
    CardSet::ModernHorizons3,
    // Two mana for a body that grows with the graveyard it is filling, and
    // turns every spare creature into whatever land the deck needs.
    CardRules::new_creature(mana_cost!("{B}{G}"), &["Zombie", "Knight"], 2, 2).with_abilities(&[
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
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    controller: PlayerRelation::You,
                },
            ],
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
    ]),
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
    &COLLECTIVE_RESISTANCE,
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
