//! Theros Beyond Death cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::CostQuantityDef;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AlternativeCastKindDef,
    AppliedEffectDef, AppliedRuleDef, CardArt, CardChoiceSourceDef, CardRules, CardSet,
    CardSupertype, CardType, ChoiceVisibilityDef, ChooseDef, ComparisonDef, EffectDef,
    EffectRecipientDef, ManaColor, MoveObjectsDef, ObjectChoiceBindingDef, ObjectPredicateDef,
    ObjectSetDef, PlayerRefDef, PlayerRelation, PlayerSetDef, RandomizeObjectOrderDef,
    SpellAdditionalCostDef, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueComparisonDef,
    ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::{ObjectSetBindingIndex, TargetIndex};
use crate::mana_cost;

// THB 20 — Heliod's Pilgrim
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HELIOD_S_PILGRIM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ea54b97-9182-4d46-9d70-3cc7f9b18ada"),
    "Heliod's Pilgrim",
    crate::card::CardArt::new("cafce2f5-f4f4-465b-96dc-bcdd29d4e4bb", "Micah Epstein"),
    crate::card::CardSet::TherosBeyondDeath,
    crate::card::CardRules::unsupported(),
);

// THB 73 — Thassa's Oracle
const ORACLE_INSPECTED: ObjectSetBindingIndex = ObjectSetBindingIndex::new(0);
const ORACLE_TOP: ObjectSetBindingIndex = ObjectSetBindingIndex::new(1);
const ORACLE_REST: ObjectSetBindingIndex = ObjectSetBindingIndex::new(2);
const ORACLE_RANDOMIZED: ObjectSetBindingIndex = ObjectSetBindingIndex::new(3);
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
                ORACLE_INSPECTED,
                &const {
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(ORACLE_TOP),
                        unchosen: Some(ORACLE_REST),
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Binding(ORACLE_INSPECTED),
                        exclude: None,
                        minimum: 0,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Private,
                        then: &const {
                            EffectDef::MoveObjects(MoveObjectsDef {
                                input: ObjectSetDef::Binding(ORACLE_TOP),
                                from: Some(ZoneKind::Library),
                                zone: ZoneKind::Library,
                                placement: ZonePlacement::Top,
                                moved: None,
                                then: &const {
                                    EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef {
                                        input: ObjectSetDef::Binding(ORACLE_REST),
                                        randomized: ORACLE_RANDOMIZED,
                                        then: &const {
                                            EffectDef::MoveObjects(MoveObjectsDef {
                                                input: ObjectSetDef::Binding(ORACLE_RANDOMIZED),
                                                from: Some(ZoneKind::Library),
                                                zone: ZoneKind::Library,
                                                placement: ZonePlacement::Bottom,
                                                moved: None,
                                                then: &EffectDef::IfCondition {
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
                                            })
                                        },
                                    })
                                },
                            })
                        },
                    })
                },
            ),
        ),
    ),
);

// THB 99 — Gray Merchant of Asphodel
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GRAY_MERCHANT_OF_ASPHODEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b06078ce-f534-4e16-9a70-d51620a33eb2"),
    "Gray Merchant of Asphodel",
    crate::card::CardArt::new("7c1a7dd8-8034-4f59-a351-33666b26ff5a", "Scott Murphy"),
    crate::card::CardSet::TherosBeyondDeath,
    crate::card::CardRules::unsupported(),
);

// THB 105 — Mire Triton
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MIRE_TRITON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3f8427d3-4d9e-48c9-838b-239fd1357d95"),
    "Mire Triton",
    crate::card::CardArt::new("3f8427d3-4d9e-48c9-838b-239fd1357d95", "Seb McKinnon"),
    crate::card::CardSet::TherosBeyondDeath,
    crate::card::CardRules::unsupported(),
);

// THB 120 — Underworld Charger
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static UNDERWORLD_CHARGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f2dd847f-0db2-4f6a-bdfb-5c88ce7802f9"),
    "Underworld Charger",
    crate::card::CardArt::new("f2dd847f-0db2-4f6a-bdfb-5c88ce7802f9", "Johann Bodin"),
    crate::card::CardSet::TherosBeyondDeath,
    crate::card::CardRules::unsupported(),
);

// THB 128 — Blood Aspirant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLOOD_ASPIRANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8d4f3fa3-ba1f-48dc-a56b-738936f1bf86"),
    "Blood Aspirant",
    crate::card::CardArt::new("8d4f3fa3-ba1f-48dc-a56b-738936f1bf86", "Tyler Walpole"),
    crate::card::CardSet::TherosBeyondDeath,
    crate::card::CardRules::unsupported(),
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
                    // The escape the Breach hands out. Its mana cost is the card's own, which
                    // is what "equal to the card's mana cost" means, and the three cards are
                    // what the grant adds on top.
                    ability: &AbilityDef::alternative_cast_for_card_mana_cost(
                        AlternativeCastKindDef::Escape,
                        Some("Escape\u{2014}the card's mana cost, Exile three other cards from your graveyard."),
                        EffectDef::None,
                    )
                    // Three cards out of your own graveyard, exiled to pay. The card being cast
                    // is on the stack by the time costs are paid, so "other" takes care of
                    // itself: it is not there to be chosen.
                    .with_alternative_additional_cost(&SpellAdditionalCostDef::exile(
                        ObjectPredicateDef::Any,
                        ZoneKind::Graveyard,
                        CostQuantityDef::Fixed(3),
                    )),
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
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static UNDERWORLD_RAGE_HOUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a04eef82-fd53-41f4-9c7e-28b9ac039032"),
    "Underworld Rage-Hound",
    crate::card::CardArt::new("a04eef82-fd53-41f4-9c7e-28b9ac039032", "Tyler Walpole"),
    crate::card::CardSet::TherosBeyondDeath,
    crate::card::CardRules::unsupported(),
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
                    &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Escape),
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
            AbilityDef::alternative_cast(
                mana_cost!("{G}{G}{U}{U}"),
                AlternativeCastKindDef::Escape,
                Some(
                    "Escape—{G}{G}{U}{U}, Exile five other cards from your graveyard. (You may cast this \
                     card from your graveyard for its escape cost.)",
                ),
                EffectDef::None,
            )
            // Five cards out of your own graveyard, exiled to pay. The card being cast
            // is on the stack by the time costs are paid, so "other" takes care of
            // itself: it is not there to be chosen.
            .with_alternative_additional_cost(&SpellAdditionalCostDef::exile(
                ObjectPredicateDef::Any,
                ZoneKind::Graveyard,
                CostQuantityDef::Fixed(5),
            )),
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
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
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
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
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
