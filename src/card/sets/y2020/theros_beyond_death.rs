//! Theros Beyond Death cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::CostQuantityDef;
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AlternativeCastKindDef,
    AlternativeCastManaCostDef, AppliedEffectDef, AppliedRuleDef, BattlefieldEntryModificationDef,
    CardArt, CardChoiceSourceDef, CardRules, CardSet, CardSupertype, CardType, ChoiceVisibilityDef,
    ChooseDef, ComparisonDef, CostDef, CounterKind, EffectDef, EffectRecipientDef, ManaColor,
    MoveObjectsDef, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectSetDef, PlayerRefDef,
    PlayerRelation, PlayerSetDef, RandomizeObjectOrderDef, ReplacementConditionDef,
    ReplacementEffectDef, ResolvedEffectDurationDef, TriggerConditionDef, TriggerEventDef,
    TurnStepDef, ValueComparisonDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::{Binding, ParentBinding, TargetIndex};
use crate::mana_cost;

/// The ordinary Escape shape: a resolved mana cost, this many other graveyard
/// cards exiled as an additional cost. The selected alternative cast kind
/// itself is the lasting Escape fact; exceptional costs remain card-local.
pub(in crate::card::sets) const fn escape(
    mana_cost: AlternativeCastManaCostDef,
    cards: u8,
) -> AbilityDef {
    AbilityDef::alternative_cast_with_additional_cost(
        mana_cost,
        AlternativeCastKindDef::Escape,
        None,
        CostDef::exile(
            ObjectPredicateDef::Any,
            ZoneKind::Graveyard,
            CostQuantityDef::Fixed(cards),
        ),
        EffectDef::None,
    )
}

// THB 20 — Heliod's Pilgrim
pub(in crate::card::sets) static HELIOD_S_PILGRIM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ea54b97-9182-4d46-9d70-3cc7f9b18ada"),
    "Heliod's Pilgrim",
    CardArt::new("cafce2f5-f4f4-465b-96dc-bcdd29d4e4bb", "Micah Epstein"),
    CardSet::TherosBeyondDeath,
    // The body is beside the point: this is a three-mana tutor that an Aura
    // deck plays for whichever Aura the board asks for.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Cleric"], 1, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, you may search your library for an Aura card, reveal it, \
             put it into your hand, then shuffle.",
            // Two ways to decline: the outer may, and a minimum of zero for a
            // search that finds nothing worth taking.
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::Subtype("Aura"),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            },
        ),
    ),
);

// THB 73 — Thassa's Oracle
const ORACLE_TOP: Binding = Binding!("oracle_top");
const ORACLE_REST: Binding = Binding!("oracle_rest");
pub(in crate::card::sets) static THASSAS_ORACLE: CardRecord = CardRecord::new_with_legacy_id(
    2212,
    "Thassa's Oracle",
    CardArt::new("13d7e352-4d01-4947-a76f-f8a01dd876cc", "Jesper Ejsing"),
    CardSet::TherosBeyondDeath,
    // Two blue mana and an empty library is the whole card. The looking is
    // what it does when the library is not empty yet.
    CardRules::new_creature(mana_cost!("{U}{U}"), &["Merfolk", "Wizard"], 1, 3).with_ability(
        abilities::enters_trigger(
            "When this creature enters, look at the top X cards of your library, where X is your devotion to blue. Put up to one of them on top of your library and the rest on the bottom of your library in a random order. If X is greater than or equal to the number of cards in your library, you win the game.",
            abilities::bind_top_cards_then(
                PlayerRefDef::EffectController,
                ValueDef::DevotionTo(ManaColor::Blue),
                &const {
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(ORACLE_TOP),
                        unchosen: Some(ORACLE_REST),
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Binding(ParentBinding),
                        exclude: None,
                        minimum: 0,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Private,
                        then: &const {
                            EffectDef::Sequence(&[
                                EffectDef::MoveObjects(MoveObjectsDef {
                                    input: ObjectSetDef::Binding(ORACLE_TOP),
                                    from: Some(ZoneKind::Library),
                                    zone: ZoneKind::Library,
                                    placement: ZonePlacement::Top,
                                    moved: None,
                                    then: &EffectDef::None,
                                }),
                                    EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef {
                                        input: ObjectSetDef::Binding(ORACLE_REST),
                                        randomized: ParentBinding,
                                        then: &const {
                                            EffectDef::Sequence(&[
                                                EffectDef::MoveObjects(MoveObjectsDef {
                                                    input: ObjectSetDef::Binding(ParentBinding),
                                                    from: Some(ZoneKind::Library),
                                                    zone: ZoneKind::Library,
                                                    placement: ZonePlacement::Bottom,
                                                    moved: None,
                                                    then: &EffectDef::None,
                                                }),
                                                EffectDef::IfCondition {
                                                    // Both sides are read as the trigger resolves,
                                                    // which is what makes an empty library and a
                                                    // single blue permanent enough.
                                                    condition: &TriggerConditionDef::ValueComparison(
                                                        &ValueComparisonDef {
                                                            left: ValueDef::DevotionTo(
                                                                ManaColor::Blue,
                                                            ),
                                                            comparison: ComparisonDef::GreaterOrEqual,
                                                            right: ValueDef::LibrarySize(
                                                                PlayerRelation::You,
                                                            ),
                                                        },
                                                    ),
                                                    then: &EffectDef::WinTheGame {
                                                        player: EffectRecipientDef::Controller,
                                                    },
                                                },
                                            ])
                                        },
                                    })
                            ])
                        },
                    })
                },
            ),
        ),
    ),
);

// THB 99 — Gray Merchant of Asphodel
pub(in crate::card::sets) static GRAY_MERCHANT_OF_ASPHODEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b06078ce-f534-4e16-9a70-d51620a33eb2"),
    "Gray Merchant of Asphodel",
    CardArt::new("7c1a7dd8-8034-4f59-a351-33666b26ff5a", "Scott Murphy"),
    CardSet::TherosBeyondDeath,
    // Its own two black pips count, so the Merchant is never worth less than
    // two even on an otherwise empty board.
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Zombie"], 2, 4).with_ability(
        abilities::enters_trigger(
            "When this creature enters, each opponent loses X life, where X is your devotion to black. You gain life equal to the life lost this way.",
            // Devotion is counted once for the whole resolution, so both
            // halves read the same number and the gain always matches the
            // loss.
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(
                        PlayerRelation::Opponent,
                    )),
                    amount: ValueDef::DevotionTo(ManaColor::Black),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::DevotionTo(ManaColor::Black),
                },
            ]),
        ),
    ),
);

// THB 105 — Mire Triton
pub(in crate::card::sets) static MIRE_TRITON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3f8427d3-4d9e-48c9-838b-239fd1357d95"),
    "Mire Triton",
    CardArt::new("3f8427d3-4d9e-48c9-838b-239fd1357d95", "Seb McKinnon"),
    CardSet::TherosBeyondDeath,
    // A deathtouch blocker that fills the graveyard and pays for the two
    // cards with life, which is what makes the self-mill upside instead of
    // a cost.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie", "Merfolk"], 2, 1).with_abilities(&[
        abilities::deathtouch(),
        abilities::enters_trigger(
            "When this creature enters, mill two cards and you gain 2 life. (To mill a card, put \
             the top card of your library into your graveyard.)",
            EffectDef::Sequence(&[
                EffectDef::Mill {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
    ]),
);

// THB 120 — Underworld Charger
pub(in crate::card::sets) static UNDERWORLD_CHARGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f2dd847f-0db2-4f6a-bdfb-5c88ce7802f9"),
    "Underworld Charger",
    CardArt::new("f2dd847f-0db2-4f6a-bdfb-5c88ce7802f9", "Johann Bodin"),
    CardSet::TherosBeyondDeath,
    // A body that only ever attacks, sold twice: the escape copy is a 5/5,
    // which is what pays for five mana and three cards of graveyard.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Nightmare", "Horse"], 3, 3).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            },
        ),
        escape(AlternativeCastManaCostDef::Fixed(mana_cost!("{4}{B}")), 3),
        AbilityDef::as_enters_if(
            "This creature escapes with two +1/+1 counters on it.",
            ReplacementConditionDef::SourceCastWith(AlternativeCastKindDef::Escape),
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 2,
                },
            ),
        ),
    ]),
);

// THB 128 — Blood Aspirant
pub(in crate::card::sets) static BLOOD_ASPIRANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8d4f3fa3-ba1f-48dc-a56b-738936f1bf86"),
    "Blood Aspirant",
    CardArt::new("8d4f3fa3-ba1f-48dc-a56b-738936f1bf86", "Tyler Walpole"),
    CardSet::TherosBeyondDeath,
    // Its own activation feeds its own trigger, so each use both shrinks the
    // opposing board and grows this one.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Satyr", "Berserker"], 1, 1).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you sacrifice a permanent, put a +1/+1 counter on this creature.",
            // Any permanent, not only creatures, so a sacrificed enchantment
            // counts twice with the ability below.
            TriggerEventDef::Sacrificed {
                object: ObjectPredicateDef::Any,
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_with_targets(
            "{1}{R}, {T}, Sacrifice a creature or enchantment: This creature deals 1 damage to target creature. That creature can't block this turn.",
            &[
                CostDef::Mana(mana_cost!("{1}{R}")),
                CostDef::TapSource,
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
                // "That creature" is the same target, so a creature that
                // survived the point still cannot block.
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// THB 161 — Underworld Breach
pub(in crate::card::sets) static UNDERWORLD_BREACH: CardRecord = CardRecord::new_with_legacy_id(
    2271,
    "Underworld Breach",
    CardArt::new("0e51d796-7279-4c06-87f0-37adbdaa41df", "Lie Setiawan"),
    CardSet::TherosBeyondDeath,
    // Two mana that turns a graveyard into a hand for one turn, which is as
    // long as anything playing it needs.
    CardRules::new_enchantment(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::static_ability(
            "Each nonland card in your graveyard has escape. The escape cost is equal to the card's \
             mana cost plus exile three other cards from your graveyard. (You may cast cards from \
             your graveyard for their escape cost.)",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::GrantsAlternativeCastFromGraveyard {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    // The card being cast is on the stack by the time the three other cards
                    // are chosen, so it is already absent from its graveyard candidates.
                    ability: &escape(AlternativeCastManaCostDef::ThisCardManaCost, 3),
                }),
            },
        ),
        // Each end step, not just yours: the Breach is one turn's worth of
        // graveyard however many turns you take.
        AbilityDef::triggered(
            "At the beginning of the end step, sacrifice this enchantment.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// THB 163 — Underworld Rage-Hound
pub(in crate::card::sets) static UNDERWORLD_RAGE_HOUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a04eef82-fd53-41f4-9c7e-28b9ac039032"),
    "Underworld Rage-Hound",
    CardArt::new("a04eef82-fd53-41f4-9c7e-28b9ac039032", "Tyler Walpole"),
    CardSet::TherosBeyondDeath,
    // It has to attack, so escaping it back is a commitment rather than a
    // free extra body -- and the counter is what makes the second one hit
    // harder than the first.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Elemental", "Dog"], 3, 1).with_abilities(&[
        abilities::attacks_each_combat_if_able(),
        escape(AlternativeCastManaCostDef::Fixed(mana_cost!("{3}{R}")), 3),
        AbilityDef::as_enters_if(
            "This creature escapes with a +1/+1 counter on it.",
            ReplacementConditionDef::SourceCastWith(AlternativeCastKindDef::Escape),
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ),
        ),
    ]),
);

// THB 229 — Uro, Titan of Nature's Wrath
pub(in crate::card::sets) static URO_TITAN_OF_NATURE_S_WRATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a0b6a71e-56cb-4d25-8f2b-7a4f1b60900d"),
    "Uro, Titan of Nature's Wrath",
    CardArt::new("a0b6a71e-56cb-4d25-8f2b-7a4f1b60900d", "Vincent Proce"),
    CardSet::TherosBeyondDeath,
    // Three mana for a ramp spell that gains three and draws, and the same
    // card again later as a 6/6 that does it every attack.
    CardRules::new_creature(mana_cost!("{1}{G}{U}"), &["Elder", "Giant"], 6, 6)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered_if(
                "When Uro enters, sacrifice it unless it escaped.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                // "Unless it escaped" reads how the spell was cast, which the permanent
                // remembers: an Uro cast for its printed cost sacrifices itself and leaves
                // the growth spell behind.
                &TriggerConditionDef::Not(
                    &TriggerConditionDef::All(&[
                        TriggerConditionDef::SourceWasCast,
                        TriggerConditionDef::SourceCastFrom(ZoneKind::Graveyard),
                        TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Escape),
                    ]),
                ),
                EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            ),
            AbilityDef::triggered(
                "Whenever Uro enters or attacks, you gain 3 life and draw a card, then you may put a \
                 land card from your hand onto the battlefield.",
                // Entering and attacking are two ways for one printed ability to fire, so
                // what it does is written once.
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                ]),
                EffectDef::Sequence(&[
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                    // "You may put a land card": the land drop this hands out is free of the
                    // one a turn, and declining is a real answer -- a hand with a land you
                    // would rather keep is not made to play it.
                    EffectDef::ChooseCards {
                        player: EffectRecipientDef::Controller,
                        sources: &[CardChoiceSourceDef::Zone(ZoneKind::Hand)],
                        object: ObjectPredicateDef::HasType(CardType::Land),
                        minimum: 0,
                        maximum: 1,
                        reveal: false,
                        destination: ZoneKind::Battlefield,
                        placement: ZonePlacement::Top,
                    },
                ]),
            ),
            escape(
                AlternativeCastManaCostDef::Fixed(mana_cost!("{G}{G}{U}{U}")),
                5,
            ),
        ]),
);

// THB 237 — Soul-Guide Lantern
pub(in crate::card::sets) static SOUL_GUIDE_LANTERN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7c850b94-75c9-4457-8b5e-1193352d6fcb"),
    "Soul-Guide Lantern",
    crate::card::CardArt::new("7c850b94-75c9-4457-8b5e-1193352d6fcb", "Cliff Childs"),
    crate::card::CardSet::TherosBeyondDeath,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this artifact enters, exile target card from a graveyard.",
            // One card out of one graveyard, chosen when the Lantern arrives. Any
            // graveyard: the Lantern is as happy to eat your own flashback card as
            // theirs.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
        // Untargeted, so it does not care whether those graveyards hold
        // anything: unlike Tormod's Crypt this one can be cashed in against an
        // empty board purely to stop what has not happened yet.
        AbilityDef::activated(
            "{T}, Sacrifice this artifact: Exile each opponent's graveyard.",
            // The two sacrifice abilities differ only in what they buy, so the shared
            // half of the cost is written once.
            &[CostDef::TapSource, CostDef::SacrificeSource],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Graveyard],
                    PlayerRelation::Opponent,
                ),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
        AbilityDef::activated(
            "{1}, {T}, Sacrifice this artifact: Draw a card.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &HELIOD_S_PILGRIM,
    &THASSAS_ORACLE,
    &GRAY_MERCHANT_OF_ASPHODEL,
    &MIRE_TRITON,
    &UNDERWORLD_CHARGER,
    &BLOOD_ASPIRANT,
    &UNDERWORLD_BREACH,
    &UNDERWORLD_RAGE_HOUND,
    &URO_TITAN_OF_NATURE_S_WRATH,
    &SOUL_GUIDE_LANTERN,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
