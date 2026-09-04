//! Aetherdrift cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef, AppliedEffectDef,
    BasicLandType, BattlefieldEntryScalarChoiceDef, CardArt, CardRules, CardSet, CardType,
    ComparisonDef, CostDef, EffectDef, EffectRecipientDef, ManaColor, ManaTypeDef,
    ObjectPredicateDef, ObjectQueryDef, PlayerRelation, ReplacementChoiceDef, ReplacementEffectDef,
    ResolvedEffectDurationDef, TriggerConditionDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// DFT 67 — Stock Up
pub(in crate::card::sets) static STOCK_UP: CardRecord = CardRecord::new_with_legacy_id(
    2179,
    "Stock Up",
    CardArt::new("0a786855-6eb4-42c0-a528-4842db46809d", "Izzy"),
    CardSet::Aetherdrift,
    // Two cards for three mana at sorcery speed is unremarkable; seeing five
    // to find them is what puts it in a deck built around one or two cards.
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell(
        "Look at the top five cards of your library. Put two of them into your hand and the rest on the bottom of your library in any order.",
        abilities::look_at_top_cards_choose_to_hand_rest_bottom(
            ValueDef::Constant(5),
            ObjectPredicateDef::Any,
            2,
            2,
        ),
    )),
);

// DFT 79 — Chitin Gravestalker
pub(in crate::card::sets) static CHITIN_GRAVESTALKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("903b4141-04a3-44c4-9d3e-aa2a773d9883"),
    "Chitin Gravestalker",
    CardArt::new("903b4141-04a3-44c4-9d3e-aa2a773d9883", "Slawomir Maniak"),
    CardSet::Aetherdrift,
    // Cycling is what makes the discount reachable: the card fills the
    // graveyard it later reads, including with copies of itself.
    CardRules::new_creature(mana_cost!("{5}{B}"), &["Insect", "Warrior"], 5, 4).with_abilities(&[
        AbilityDef::static_ability(
            "This spell costs {1} less to cast for each artifact and/or creature card in your graveyard.",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                &ObjectQueryDef::matching(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ),
            )),
        )
        // Read from hand, where the cost is paid.
        .with_source_zones(&[ZoneKind::Hand]),
        abilities::cycling("Cycling {2} ({2}, Discard this card: Draw a card.)", mana_cost!("{2}")),
    ]),
);

// DFT 88 — Grim Bauble
pub(in crate::card::sets) static GRIM_BAUBLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9bfdf60a-6f67-4872-8961-d63776b192c3"),
    "Grim Bauble",
    CardArt::new("9bfdf60a-6f67-4872-8961-d63776b192c3", "Wero Gallo"),
    CardSet::Aetherdrift,
    // One mana kills an early creature and the artifact stays behind, which
    // is what makes the four-mana surveil a bonus rather than the plan.
    CardRules::new_artifact(mana_cost!("{B}")).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this artifact enters, target creature an opponent controls gets -2/-2 until \
             end of turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-2),
                    ValueDef::Constant(-2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{2}{B}, {T}, Sacrifice this artifact: Surveil 2. (Look at the top two cards of your \
             library, then put any number of them into your graveyard and the rest on top of \
             your library in any order.)",
            &[
                CostDef::Mana(mana_cost!("{2}{B}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            abilities::surveil(ValueDef::Constant(2)),
        ),
    ]),
);

// DFT 191 — Brightglass Gearhulk
pub(in crate::card::sets) static BRIGHTGLASS_GEARHULK: CardRecord = CardRecord::new_with_legacy_id(
    2301,
    "Brightglass Gearhulk",
    CardArt::new("3dea5b45-925c-4732-8e9d-fa8232792736", "José Parodi"),
    CardSet::Aetherdrift,
    // A 4/4 first striker with trample that also finds the two one-drops the
    // deck is built around, which is what four coloured pips buy.
    CardRules::new_artifact_creature(mana_cost!("{G}{G}{W}{W}"), &["Construct"], 4, 4)
        .with_abilities(&[
            abilities::first_strike(),
            abilities::trample(),
            // "You may" on top of a search that already allows none: declining and
            // finding nothing look the same from the outside, and the card offers
            // both because a library nobody wants to shuffle is a real answer.
            abilities::enters_trigger(
                "When this creature enters, you may search your library for up to two artifact, creature, \
                 and/or enchantment cards with mana value 1 or less, reveal them, put them into your \
                 hand, then shuffle.",
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    // "Up to two" and revealed: a minimum of none, and everything taken is
                    // shown, which is what stops the search being private information.
                    effect: &EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        // "Artifact, creature, and/or enchantment cards with mana value 1 or less."
                        // The three types are alternatives and the mana value applies to all of
                        // them, so the bound is outside the choice rather than inside it.
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                            ]),
                            ObjectPredicateDef::ManaValueAtMost(1),
                        ]),
                        minimum: 0,
                        maximum: ValueDef::Constant(2),
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
        ]),
);

// DFT 250 — Bleachbone Verge
pub(in crate::card::sets) static BLEACHBONE_VERGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("52dcdabd-a186-45fe-9fee-6c0f1afeaf16"),
    "Bleachbone Verge",
    CardArt::new("52dcdabd-a186-45fe-9fee-6c0f1afeaf16", "Mark Tedin"),
    CardSet::Aetherdrift,
    // Untapped and free either way: the black is unconditional, and the
    // white is what the rest of the mana base is for.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {B}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
        AbilityDef::activated_mana_if(
            "{T}: Add {W}. Activate only if you control a Plains or a Swamp.",
            &[CostDef::TapSource],
            &TriggerConditionDef::ObjectCount {
                // The verge condition in this cycle's Orzhov colours. Either type answers
                // it, so a Godless Shrine is both halves at once.
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[
                        BasicLandType::Plains,
                        BasicLandType::Swamp,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
        ),
    ]),
);

// DFT 258 — Night Market
pub(in crate::card::sets) static NIGHT_MARKET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a8c1dce3-6136-4294-9d2b-5ef8527d733b"),
    "Night Market",
    CardArt::new("a8c1dce3-6136-4294-9d2b-5ef8527d733b", "David Álvarez"),
    CardSet::Aetherdrift,
    // A tapped land that fixes one colour and cycles away once the mana is
    // there, so it is never the draw that loses the game.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::as_enters(
            "As this land enters, choose a color.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                BattlefieldEntryScalarChoiceDef::COLOR,
            )),
        ),
        AbilityDef::activated_mana(
            "{T}: Add one mana of the chosen color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_type(ManaTypeDef::ChosenColor)),
        ),
        abilities::cycling(
            "Cycling {3} ({3}, Discard this card: Draw a card.)",
            mana_cost!("{3}"),
        ),
    ]),
);

// DFT 260 — Riverpyre Verge
pub(in crate::card::sets) static RIVERPYRE_VERGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57a93a71-d77c-417f-85d0-cd420f573331"),
    "Riverpyre Verge",
    CardArt::new("57a93a71-d77c-417f-85d0-cd420f573331", "Titus Lunter"),
    CardSet::Aetherdrift,
    // Untapped and free either way: the red is unconditional, and the blue
    // is what the second land in the deck is for.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
        AbilityDef::activated_mana_if(
            "{T}: Add {U}. Activate only if you control an Island or a Mountain.",
            &[CostDef::TapSource],
            &TriggerConditionDef::ObjectCount {
                // The same verge condition in this cycle's other pair of colours: either
                // type answers it, so a Volcanic Island is both halves at once.
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[
                        BasicLandType::Island,
                        BasicLandType::Mountain,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
        ),
    ]),
);

// DFT 264 — Sunbillow Verge
pub(in crate::card::sets) static SUNBILLOW_VERGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("94ed132f-b818-4dbf-9b4a-e5acb067e0a4"),
    "Sunbillow Verge",
    CardArt::new("94ed132f-b818-4dbf-9b4a-e5acb067e0a4", "Pete Venters"),
    CardSet::Aetherdrift,
    // Untapped and free either way: the white is unconditional, and the red
    // is what the rest of the mana base is for.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {W}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
        ),
        AbilityDef::activated_mana_if(
            "{T}: Add {R}. Activate only if you control a Mountain or a Plains.",
            &[CostDef::TapSource],
            &TriggerConditionDef::ObjectCount {
                // The verge condition in this cycle's Boros colours. Either type answers
                // it, so a Plateau is both halves at once.
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[
                        BasicLandType::Mountain,
                        BasicLandType::Plains,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
    ]),
);

// DFT 268 — Wastewood Verge
pub(in crate::card::sets) static WASTEWOOD_VERGE: CardRecord = CardRecord::new_with_legacy_id(
    2196,
    "Wastewood Verge",
    CardArt::new("5ceacc7d-d407-4f82-af58-9bdf8426924e", "Bartek Fedyczak"),
    CardSet::Aetherdrift,
    // Untapped and free either way: the green is unconditional, and the
    // black is what the second land in the deck is for.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ),
        AbilityDef::activated_mana_if(
            "{T}: Add {B}. Activate only if you control a Swamp or a Forest.",
            &[CostDef::TapSource],
            &TriggerConditionDef::ObjectCount {
                // The verge condition: any land you control with either type answers it,
                // so a Bayou is both halves at once and a land whose types were changed
                // counts for what it is now rather than what it was printed as.
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[
                        BasicLandType::Swamp,
                        BasicLandType::Forest,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &STOCK_UP,
    &CHITIN_GRAVESTALKER,
    &GRIM_BAUBLE,
    &BRIGHTGLASS_GEARHULK,
    &BLEACHBONE_VERGE,
    &NIGHT_MARKET,
    &RIVERPYRE_VERGE,
    &SUNBILLOW_VERGE,
    &WASTEWOOD_VERGE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
