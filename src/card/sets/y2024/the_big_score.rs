//! The Big Score card inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityOperationDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BindObjectsDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::CharacteristicOperationDef;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseCardsFromCollectionDef;
use crate::card::ChooseDef;
use crate::card::ChooseForEachPlayerDef;
use crate::card::CollectionInspectionDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatedTokensDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DamageEventMatcherDef;
use crate::card::DamageKindDef;
use crate::card::DamageSourceMatcherDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::ManaColor;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::PayOrDef;
use crate::card::PerPlayerSelectionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::RandomizeObjectOrderDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenCopyDef;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

static ANY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

use crate::card::sets::y2011::magic_2012 as catalog_m12;
use crate::card::sets::y2011::new_phyrexia as catalog_nph;
use crate::card::sets::y2012::return_to_ravnica as catalog_rtr;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "BIG",
    slug: "the-big-score",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const CLUE_TOKEN: TokenCharacteristics = crate::card::tokens::clue().with_art(CardArt::new(
    "764a906c-8b27-4ffa-bdc3-7825c6919d3e",
    "Clint Lockwood",
));
const FOOD_TOKEN: TokenCharacteristics = crate::card::tokens::food().with_art(CardArt::new(
    "280e3af6-7904-4214-8141-e145c48e2687",
    "Patrik Hell",
));

// BIG 1 — Collector's Cage
// Audit: unsupported — Needs counting distinct current powers among controlled creatures to gate the hideaway cast; sum/minimum/maximum scalar aggregates do not implement distinct-value cardinality.
pub(in crate::card::sets) static COLLECTOR_S_CAGE: CardRecord = CardRecord::new(
    "Collector's Cage",
    "a33703bb-51c0-4d57-9d06-1148507ddc4f",
    "Bartek Fedyczak",
    CardRules::unsupported(),
);

// BIG 2 — Grand Abolisher (reprint)
const GRAND_ABOLISHER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m12::GRAND_ABOLISHER,
    "ee793ed2-7d59-4640-8868-ad486600df2c",
    "Aurore Folny",
);

// BIG 3 — Oltec Matterweaver
pub(in crate::card::sets) static OLTEC_MATTERWEAVER: CardRecord = CardRecord::new(
    "Oltec Matterweaver",
    "f4f2a818-9fd1-41db-967e-7d2c9b4e4c2f",
    "Villarrte",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Artificer"], 2, 4).with_abilities(&[
        AbilityDef::modal_triggered(
            "Whenever you cast a creature spell, choose one —",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            &[
                AbilityDef::spell(
                    "Create a 1/1 colorless Gnome artifact creature token.",
                    EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::artifact_creature(&["Gnome"], &[], 1, 1),
                    ))),
                ),
                AbilityDef::spell_with_targets(
                    "Create a token that's a copy of target artifact token you \
                     control.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::Token,
                            ]),
                            zones: &[ZoneKind::Battlefield],
                            controller: Some(PlayerRelation::You),
                            owner: None,
                        },
                    )],
                    EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Copy(&TokenCopyDef {
                        object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        exceptions: CopyExceptionsDef::NONE,
                    }))),
                ),
            ],
        ),
    ]),
);

// BIG 4 — Rest in Peace (reprint)
const REST_IN_PEACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rtr::REST_IN_PEACE,
    "d108c2b1-236e-4b8d-8445-d9749ccc4fea",
    "Grady Frederick",
);

// BIG 5 — Esoteric Duplicator
pub(in crate::card::sets) static ESOTERIC_DUPLICATOR: CardRecord = CardRecord::new(
    "Esoteric Duplicator",
    "3dbb2755-97d9-492e-8697-5548160678c8",
    "Anton Solovianchyk",
    CardRules::new_artifact(mana_cost!("{2}{U}"))
        .with_subtypes(&["Clue"])
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever you sacrifice this artifact or another artifact, you \
                 may pay {2}. If you do, at the beginning of the next end \
                 step, create a token that's a copy of that artifact.",
                TriggerEventDef::Sacrificed {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    player: PlayerRelation::You,
                },
                EffectDef::BindObjects(BindObjectsDef {
                    source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::One(
                        ObjectRefDef::TriggeringObject,
                    )),
                    binding: crate::Binding!("artifact"),
                    then: &EffectDef::PayOr(PayOrDef::optional(
                        &[CostDef::Mana(mana_cost!("{2}"))],
                        &EffectDef::InstallTrigger(InstalledTriggerDef::once(
                            &AbilityDef::triggered(
                                "At the beginning of the next end step, create a token that is \
                                 a copy of that artifact.",
                                TriggerEventDef::StepBegins {
                                    step: TurnStepDef::End,
                                    player: PlayerRelation::Any,
                                },
                                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Copy(
                                    &TokenCopyDef {
                                        object: &EffectRecipientDef::objects(
                                            ObjectSetDef::Binding(crate::Binding!("artifact")),
                                        ),
                                        exceptions: CopyExceptionsDef::NONE,
                                    },
                                ))),
                            ),
                        )),
                    )),
                }),
            ),
            AbilityDef::activated(
                "{2}, Sacrifice this artifact: Draw a card.",
                &[CostDef::Mana(mana_cost!("{2}")), CostDef::SacrificeSource],
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
        ]),
);

// BIG 6 — Simulacrum Synthesizer
pub(in crate::card::sets) static SIMULACRUM_SYNTHESIZER: CardRecord = CardRecord::new(
    "Simulacrum Synthesizer",
    "aaa05ad1-5cda-4edd-b6bf-562ae3e5011a",
    "Anton Solovianchyk",
    CardRules::new_artifact(mana_cost!("{2}{U}")).with_abilities(&[
        abilities::enters_trigger(
            "When this artifact enters, scry 2.",
            abilities::scry(ValueDef::Constant(2)),
        ),
        AbilityDef::triggered(
            "Whenever another artifact you control with mana value 3 or \
             greater enters, create a 0/0 colorless Construct artifact \
             creature token with \"This token gets +1/+1 for each artifact \
             you control.\"",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(2)),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::artifact_creature(&["Construct"], &[], 0, 0).with_abilities(
                    &[AbilityDef::static_ability(
                        "This token gets +1/+1 for each artifact you control.",
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::modify_power_toughness(
                                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                )),
                                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                )),
                            ),
                        },
                    )],
                ),
            ))),
        ),
    ]),
);

// BIG 7 — Worldwalker Helm
// Audit: unsupported — Needs a prospective token-creation replacement that appends one token of another kind to the original creation batch; existing token replacements multiply counts or change token characteristics.
pub(in crate::card::sets) static WORLDWALKER_HELM: CardRecord = CardRecord::new(
    "Worldwalker Helm",
    "b74ad496-05bc-4c5a-9027-b14df9c387ab",
    "Camille Alquier",
    CardRules::unsupported(),
);

// BIG 8 — Greed's Gambit
pub(in crate::card::sets) static GREED_S_GAMBIT: CardRecord = CardRecord::new(
    "Greed's Gambit",
    "5b60a1a6-b2ca-4dc2-a6d9-eff97092079a",
    "Inkognit",
    CardRules::new_enchantment(mana_cost!("{3}{B}")).with_abilities(&[
        abilities::enters_trigger(
            "When this enchantment enters, you draw three cards, gain 6 \
             life, and create three 2/1 black Bat creature tokens with \
             flying.",
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(3)),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(6),
                },
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::creature(&["Bat"], &[ManaColor::Black], 2, 1)
                            .with_abilities(&[abilities::flying()]),
                    ))
                    .with_count(ValueDef::Constant(3)),
                ),
            ]),
        ),
        AbilityDef::triggered(
            "At the beginning of your end step, you discard a card, lose 2 \
             life, and sacrifice a creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            EffectDef::Sequence(&[
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                    player: EffectRecipientDef::Controller,
                    zone: ZoneKind::Battlefield,
                    candidates: ObjectPredicateDef::HasType(CardType::Creature),
                    selection: PerPlayerSelectionDef::Count(ValueDef::Constant(1)),
                    chosen: crate::Binding!("sacrifices"),
                    unchosen: crate::Binding!("unchosen_sacrifices"),
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::sacrifice(EffectRecipientDef::objects(
                        ObjectSetDef::Binding(crate::Binding!("sacrifices")),
                    )),
                }),
            ]),
        ),
        AbilityDef::triggered(
            "When this enchantment leaves the battlefield, you discard \
             three cards, lose 6 life, and sacrifice three creatures.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                None,
            ),
            EffectDef::Sequence(&[
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(6),
                },
                EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                    player: EffectRecipientDef::Controller,
                    zone: ZoneKind::Battlefield,
                    candidates: ObjectPredicateDef::HasType(CardType::Creature),
                    selection: PerPlayerSelectionDef::Count(ValueDef::Constant(3)),
                    chosen: crate::Binding!("sacrifices"),
                    unchosen: crate::Binding!("unchosen_sacrifices"),
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::sacrifice(EffectRecipientDef::objects(
                        ObjectSetDef::Binding(crate::Binding!("sacrifices")),
                    )),
                }),
            ]),
        ),
    ]),
);

// BIG 9 — Harvester of Misery
pub(in crate::card::sets) static HARVESTER_OF_MISERY: CardRecord = CardRecord::new(
    "Harvester of Misery",
    "a3012af9-621d-4fae-b00d-079a89ae35fe",
    "Jorge Jacinto",
    // Five mana for a board sweep on a hard-to-block body, or two mana from
    // the hand for one creature when the board does not need sweeping.
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Spirit"], 5, 4).with_abilities(&[
        abilities::menace(),
        abilities::enters_trigger(
            "When this creature enters, other creatures get -2/-2 until end of turn.",
            // "Other creatures": everyone's, and not the Spirit itself, which is what
            // lets a 5/4 sweep a board of two-toughness creatures and survive it.
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-2),
                    ValueDef::Constant(-2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_with_targets(
            "{1}{B}, Discard this card: Target creature gets -2/-2 until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{B}")), CostDef::DiscardSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            // The same shrink, aimed at one creature. The card is discarded to pay for
            // it, so this is what the Spirit does on the turns five mana is too much.
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-2),
                    ValueDef::Constant(-2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .with_source_zones(&[ZoneKind::Hand]),
    ]),
);

// BIG 10 — Hostile Investigator
pub(in crate::card::sets) static HOSTILE_INVESTIGATOR: CardRecord = CardRecord::new(
    "Hostile Investigator",
    "158c000e-7960-4518-b034-a529622b7bf1",
    "Andrew Mar",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Ogre", "Rogue", "Detective"], 4, 3)
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When this creature enters, target opponent discards a card.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ),
            AbilityDef::triggered(
                "Whenever one or more players discard one or more cards, \
                 investigate. This ability triggers only once each turn. \
                 (Create a Clue token. It's an artifact with \"{2}, Sacrifice \
                 this token: Draw a card.\")",
                TriggerEventDef::DiscardedCards(PlayerRelation::Any),
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            )
            .triggering_at_most(1),
        ]),
);

// BIG 11 — Generous Plunderer (alternate printing)
const GENEROUS_PLUNDERER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GENEROUS_PLUNDERER,
    1,
    "4c6cf93a-d073-48ac-88db-c46bf3e10beb",
    "Alexander Mokhov",
);

// BIG 12 — Legion Extruder
pub(in crate::card::sets) static LEGION_EXTRUDER: CardRecord = CardRecord::new(
    "Legion Extruder",
    "5a077de0-1893-40d0-a499-ee2e6e2258f1",
    "Anton Solovianchyk",
    // Two mana that answers a creature on the way in and then turns every
    // spent artifact -- a cracked Lotus Petal, an emptied Bauble -- into a
    // 3/3, which is what the cube's artifact decks have lying around.
    CardRules::new_artifact(mana_cost!("{1}{R}")).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this artifact enters, it deals 2 damage to any target.",
            &ANY_TARGET,
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
        ),
        AbilityDef::activated(
            "{2}, {T}, Sacrifice another artifact: Create a 3/3 colorless Golem artifact creature \
             token.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::TapSource,
                CostDef::SacrificePermanent {
                    // Another one: the Extruder is an artifact itself and may not eat itself,
                    // which is what stops a two-mana artifact from being a Golem on its own.
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::artifact_creature(&["Golem"], &[], 3, 3).with_art(
                    CardArt::new("406e2960-f560-48bb-b4a6-4bd35889a8f8", "Brian Valeza"),
                ),
            ))),
        ),
    ]),
);

// BIG 13 — Memory Vessel
// Audit: unsupported — Needs per-player exile-play permissions and hand-play prohibitions that expire as the activating player's next turn begins; the exile permission duration vocabulary does not express that boundary.
pub(in crate::card::sets) static MEMORY_VESSEL: CardRecord = CardRecord::new(
    "Memory Vessel",
    "2e37a5cd-887d-4b41-97f7-ae0bba85436b",
    "Diego Gisbert",
    CardRules::unsupported(),
);

// BIG 14 — Molten Duplication
pub(in crate::card::sets) static MOLTEN_DUPLICATION: CardRecord = CardRecord::new(
    "Molten Duplication",
    "fdbe1ac1-461f-4746-a8d8-6c8dea2c97c6",
    "Justyna Dura",
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Create a token that's a copy of target artifact or creature \
         you control, except it's an artifact in addition to its other \
         types. It gains haste until end of turn. Sacrifice it at the \
         beginning of the next end step.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )],
        EffectDef::CreateToken(
            CreateTokenDef::new(TokenDef::Copy(&TokenCopyDef {
                object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                exceptions: CopyExceptionsDef::NONE
                    .with_added_types(CardTypeSet::single(CardType::Artifact)),
            }))
            .with_created_tokens(CreatedTokensDef {
                binding: crate::Binding!("copy"),
                then: &EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                            crate::Binding!("copy"),
                        )),
                        effect: AppliedEffectDef::add_ability(&abilities::haste()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                        "At the beginning of the next end step, sacrifice this permanent.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::End,
                            player: PlayerRelation::Any,
                        },
                        EffectDef::sacrifice(EffectRecipientDef::objects(ObjectSetDef::Binding(
                            crate::Binding!("copy"),
                        ))),
                    ))),
                ]),
            }),
        ),
    )]),
);

// BIG 15 — Territory Forge
pub(in crate::card::sets) static TERRITORY_FORGE: CardRecord = CardRecord::new(
    "Territory Forge",
    "71059bc8-f63a-4d9c-9d08-2e995e74cc59",
    "Mirko Failoni",
    CardRules::new_artifact(mana_cost!("{4}{R}")).with_abilities(&[
        AbilityDef::triggered_if_with_targets(
            "When this artifact enters, if you cast it, exile target \
             artifact or land.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::SourceWasCast,
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
            )],
            EffectDef::ExileLinkedToSource {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                face_down: false,
                until_source_leaves: false,
                then: None,
            },
        ),
        AbilityDef::static_ability(
            "This artifact has all activated abilities of the exiled card.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                    AbilityOperationDef::AddActivatedAbilitiesOfLinkedExiles(
                        ObjectPredicateDef::Any,
                    ),
                )),
            },
        ),
    ]),
);

// BIG 16 — Ancient Cornucopia
// Audit: unsupported — Needs a once-per-turn use limit consumed only when the optional life gain is accepted; trigger limits are consumed when an ability triggers, including triggers whose optional effect is declined.
pub(in crate::card::sets) static ANCIENT_CORNUCOPIA: CardRecord = CardRecord::new(
    "Ancient Cornucopia",
    "f977975d-0439-4731-b129-270cc4cdbb23",
    "Bartek Fedyczak",
    CardRules::unsupported(),
);

// BIG 17 — Bristlebud Farmer
pub(in crate::card::sets) static BRISTLEBUD_FARMER: CardRecord = CardRecord::new(
    "Bristlebud Farmer",
    "d498c4de-5e80-4baa-9fcb-70f164880c84",
    "Adrián Rodríguez Pérez",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Plant", "Druid"], 5, 5).with_abilities(&[
        abilities::trample(),
        abilities::enters_trigger(
            "When this creature enters, create two Food tokens. (They're \
             artifacts with \"{2}, {T}, Sacrifice this token: You gain 3 \
             life.\")",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                    .with_count(ValueDef::Constant(2)),
            ),
        ),
        AbilityDef::triggered(
            "Whenever this creature attacks, you may sacrifice a Food. If \
             you do, mill three cards. You may put a permanent card from \
             among them into your hand.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::sacrifice_permanent(ObjectPredicateDef::Subtype(
                    SubtypeDef::Literal("Food"),
                ))],
                &EffectDef::Sequence(&[
                    EffectDef::BindOutput {
                        binding: crate::Binding!("milled"),
                        effect: &EffectDef::Mill {
                            player: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(3),
                        },
                    },
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Matching {
                            objects: &ObjectSetDef::Binding(crate::Binding!("milled")),
                            object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ])),
                        },
                        exclude: None,
                        minimum: 0,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "chosen"
                            ))),
                            ZoneKind::Hand,
                            ZonePlacement::Top,
                        ),
                    }),
                ]),
            )),
        ),
    ]),
);

// BIG 18 — Omenpath Journey
// Audit: unsupported — Needs a library search constrained to distinct selected names and a random selection from this source's linked exiles; current bounded searches and random zone selections do not compose those group constraints.
pub(in crate::card::sets) static OMENPATH_JOURNEY: CardRecord = CardRecord::new(
    "Omenpath Journey",
    "c49c9b72-61c0-4e3a-a3a6-994b149398a9",
    "Nereida",
    CardRules::unsupported(),
);

// BIG 19 — Sandstorm Salvager
pub(in crate::card::sets) static SANDSTORM_SALVAGER: CardRecord = CardRecord::new(
    "Sandstorm Salvager",
    "13b0f27c-a359-4702-833a-82fec161eeec",
    "Francis Tneh",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Artificer"], 1, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a 3/3 colorless Golem \
             artifact creature token.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::artifact_creature(&["Golem"], &[], 3, 3),
            ))),
        ),
        AbilityDef::activated(
            "{2}, {T}: Put a +1/+1 counter on each creature token you \
             control. They gain trample until end of turn.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Token,
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Token,
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// BIG 20 — Vaultborn Tyrant (alternate printing)
const VAULTBORN_TYRANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VAULTBORN_TYRANT,
    1,
    "62b3f560-262b-4bc3-9aef-535fd7082c28",
    "Loïc Canavaggia",
);

// BIG 21 — Loot, the Key to Everything
pub(in crate::card::sets) static LOOT_THE_KEY_TO_EVERYTHING: CardRecord = CardRecord::new(
    "Loot, the Key to Everything",
    "fb169fa2-c92e-45f7-89a2-0ca0e3910a1c",
    "Rudy Siswanto",
    CardRules::new_creature(mana_cost!("{G}{U}{R}"), &["Beast", "Noble"], 1, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::ward(&[CostDef::Mana(mana_cost!("{1}"))], "Ward {1}"),
            AbilityDef::triggered(
                "At the beginning of your upkeep, exile the top X cards of \
                 your library, where X is the number of card types among other \
                 nonland permanents you control. You may play those cards this \
                 turn.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::BindObjects(BindObjectsDef {
                    source: ObjectCollectionSourceDef::TopCards {
                        player: PlayerRefDef::EffectController,
                        count: ValueDef::CardTypesAmongObjects(&ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                        CardType::Land,
                                    )),
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                        )),
                    },
                    binding: crate::Binding!("top"),
                    then: &EffectDef::ExileGrantingControllerPlayThisTurn {
                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                            crate::Binding!("top"),
                        )),
                    },
                }),
            ),
        ]),
);

// BIG 22 — Pest Control
pub(in crate::card::sets) static PEST_CONTROL: CardRecord = CardRecord::new(
    "Pest Control",
    "a4a01b92-dafb-4ea6-8eff-29f881f6be24",
    "Jonas De Ro",
    CardRules::new_sorcery(mana_cost!("{W}{B}")).with_abilities(&[
        AbilityDef::spell(
            "Destroy all nonland permanents with mana value 1 or less.",
            EffectDef::Destroy {
                object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        ObjectPredicateDef::ManaValueAtMost(1),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ))),
                then: None,
            },
        ),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))]
        ),
    ]),
);

// BIG 23 — Lost Jitte
pub(in crate::card::sets) static LOST_JITTE: CardRecord = CardRecord::new(
    "Lost Jitte",
    "c936504c-4e90-408f-ba98-0fb8c0378471",
    "Yeong-Hao Han",
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever equipped creature deals combat damage, put a charge \
                 counter on Lost Jitte.",
                TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                    source: DamageSourceMatcherDef::Object(ObjectRefDef::AttachedToSource),
                    kind: DamageKindDef::Combat,
                    ..DamageEventMatcherDef::ANY
                }),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("charge"),
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::modal_activated(
                "Remove a charge counter from Lost Jitte: Choose one —",
                &[CostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("charge"),
                    amount: 1,
                }],
                &[
                    AbilityDef::spell_with_targets(
                        "Untap target land.",
                        &[AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Land),
                        )],
                        EffectDef::Untap {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        },
                    ),
                    AbilityDef::spell_with_targets(
                        "Target creature can't block this turn.",
                        &[AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )],
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ),
                    AbilityDef::spell(
                        "Put a +1/+1 counter on equipped creature.",
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::AttachedPermanent,
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                    ),
                ],
                1,
                1,
                false,
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// BIG 24 — Lotus Ring
pub(in crate::card::sets) static LOTUS_RING: CardRecord = CardRecord::new(
    "Lotus Ring",
    "02267717-66e0-41f7-8009-75586a4aa4be",
    "Alayna Danner",
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::indestructible(),
            AbilityDef::static_ability(
                "Equipped creature gets +3/+3 and has vigilance and \"{T}, \
                 Sacrifice this creature: Add three mana of any one color.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(3),
                            ValueDef::Constant(3),
                        ),
                        AppliedEffectDef::add_ability(&abilities::vigilance()),
                        AppliedEffectDef::add_ability(&AbilityDef::activated_mana(
                            "{T}, Sacrifice this creature: Add three mana of any one color.",
                            &[CostDef::TapSource, CostDef::SacrificeSource],
                            EffectDef::AddMana(AddManaEffectDef::any_color().with_amount(3)),
                        )),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{3}"))], "Equip {3}"),
        ]),
);

// BIG 25 — Nexus of Becoming
pub(in crate::card::sets) static NEXUS_OF_BECOMING: CardRecord = CardRecord::new(
    "Nexus of Becoming",
    "b0f61742-522c-4b36-97db-41d0c412a072",
    "Adam Volker",
    CardRules::new_artifact(mana_cost!("{6}")).with_abilities(&[AbilityDef::triggered(
        "At the beginning of combat on your turn, draw a card. Then \
         you may exile an artifact or creature card from your hand. If \
         you do, create a token that's a copy of the exiled card, \
         except it's a 3/3 Golem artifact creature in addition to its \
         other types.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::BeginningOfCombat,
            player: PlayerRelation::You,
        },
        EffectDef::Sequence(&[
            abilities::draw_cards(ValueDef::Constant(1)),
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Private,
                then: &EffectDef::WithZoneMoveResult {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                    binding: crate::Binding!("exiled"),
                    then: &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Copy(
                        &TokenCopyDef {
                            object: &EffectRecipientDef::objects(
                                ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!(
                                    "exiled"
                                )),
                            ),
                            exceptions: CopyExceptionsDef {
                                base_power_toughness: Some((3, 3)),
                                added_types: CardTypeSet::single(CardType::Artifact)
                                    .union(CardTypeSet::single(CardType::Creature)),
                                added_creature_types: CreatureTypeSetDef::named(&["Golem"]),
                                ..CopyExceptionsDef::NONE
                            },
                        },
                    ))),
                },
            }),
        ]),
    )]),
);

// BIG 26 — Sword of Wealth and Power
// Audit: unsupported — Needs a delayed trigger that is consumed by the next matching cast and also expires at end of turn; installed triggers support Once or ThisTurn separately and discard nested per-turn trigger limits, so the latter copies every matching spell.
pub(in crate::card::sets) static SWORD_OF_WEALTH_AND_POWER: CardRecord = CardRecord::new(
    "Sword of Wealth and Power",
    "ed9e5041-3c05-4a8a-9f00-081b01685d0c",
    "Dominik Mayer",
    CardRules::unsupported(),
);

// BIG 27 — Torpor Orb (reprint)
const TORPOR_ORB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_nph::TORPOR_ORB,
    "dbf02a38-d10d-463e-ab99-e7fd848a1bd3",
    "Robin Olausson",
);

// BIG 28 — Transmutation Font
// Audit: unsupported — Needs an activation cost selecting exactly three artifact tokens with pairwise distinct names; existing sacrifice selections constrain count and individual objects but not names across the chosen group.
pub(in crate::card::sets) static TRANSMUTATION_FONT: CardRecord = CardRecord::new(
    "Transmutation Font",
    "e6cfe673-d688-499a-882b-4fe5418739e3",
    "Mark Poole",
    CardRules::unsupported(),
);

// BIG 29 — Fomori Vault
pub(in crate::card::sets) static FOMORI_VAULT: CardRecord = CardRecord::new(
    "Fomori Vault",
    "a5433b98-4657-4bbe-9e72-d3c94c6aa8ef",
    "Jonas De Ro",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{3}, {T}, Discard a card: Look at the top X cards of your \
             library, where X is the number of artifacts you control. Put \
             one of those cards into your hand and the rest on the bottom \
             of your library in a random order.",
            &[
                CostDef::Mana(mana_cost!("{3}")),
                CostDef::TapSource,
                CostDef::discard(ObjectPredicateDef::Any),
            ],
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::Any,
                minimum: 1,
                maximum: 1,
                chosen: crate::Binding!("chosen"),
                remainder: crate::Binding!("rest"),
                then: &EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                    EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef {
                        input: ObjectSetDef::Binding(crate::Binding!("rest")),
                        randomized: crate::Binding!("random_bottom"),
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "random_bottom"
                            ))),
                            ZoneKind::Library,
                            ZonePlacement::Bottom,
                        ),
                    }),
                ]),
            }),
        ),
    ]),
);

// BIG 30 — Tarnation Vista
// Audit: unsupported — Needs a single mana activation producing one mana of each color represented by monocolored controlled permanents; the mana planner does not execute a sequence of independently conditional mana outputs.
pub(in crate::card::sets) static TARNATION_VISTA: CardRecord = CardRecord::new(
    "Tarnation Vista",
    "962552a1-ec34-49e2-a23d-85dfb405d5e0",
    "Alayna Danner",
    CardRules::unsupported(),
);

// BIG 31 — Collector's Cage (alternate printing)
const COLLECTOR_S_CAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COLLECTOR_S_CAGE,
    1,
    "8391dbb8-3c8d-42c3-892b-b85e40bb28e0",
    "Ben Hill",
);

// BIG 32 — Grand Abolisher (alternate printing)
const GRAND_ABOLISHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_m12::GRAND_ABOLISHER,
    1,
    "03304c72-fe8e-42af-96ae-1e64ba0105b3",
    "David Astruga",
);

// BIG 33 — Oltec Matterweaver (alternate printing)
const OLTEC_MATTERWEAVER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OLTEC_MATTERWEAVER,
    1,
    "9e068335-92f9-4ebd-bb00-f4ff741e3d7d",
    "Inkognit",
);

// BIG 34 — Rest in Peace (alternate printing)
const REST_IN_PEACE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_rtr::REST_IN_PEACE,
    1,
    "ab6ef698-caad-47cd-ba35-41be64a58c99",
    "Pablo Rivera",
);

// BIG 35 — Esoteric Duplicator (alternate printing)
const ESOTERIC_DUPLICATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ESOTERIC_DUPLICATOR,
    1,
    "94b4b7ac-d45a-4ce9-97b4-35789df8a162",
    "Ben Hill",
);

// BIG 36 — Simulacrum Synthesizer (alternate printing)
const SIMULACRUM_SYNTHESIZER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SIMULACRUM_SYNTHESIZER,
    1,
    "28a11489-11c5-4018-aab8-c9d036638414",
    "Svetlin Velinov",
);

// BIG 37 — Worldwalker Helm (alternate printing)
const WORLDWALKER_HELM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WORLDWALKER_HELM,
    1,
    "8abd5601-c8b4-4ac8-aab0-be5c7b3aaf56",
    "David Astruga",
);

// BIG 38 — Greed's Gambit (alternate printing)
const GREED_S_GAMBIT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GREED_S_GAMBIT,
    1,
    "12cb3add-c39c-4124-a028-87333aab0929",
    "Xabi Gaztelua",
);

// BIG 39 — Harvester of Misery (alternate printing)
const HARVESTER_OF_MISERY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HARVESTER_OF_MISERY,
    1,
    "d1e81c9e-02f4-4c18-9326-127e08c9b47f",
    "Yeong-Hao Han",
);

// BIG 40 — Hostile Investigator (alternate printing)
const HOSTILE_INVESTIGATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HOSTILE_INVESTIGATOR,
    1,
    "b6cd1d1b-dbde-4a2c-94ed-58395867799c",
    "Loïc Canavaggia",
);

// BIG 41 — Generous Plunderer
// Audit: unsupported — Needs a reflexive trigger created by accepting this particular upkeep effect and retained after the Plunderer leaves; OptionalEffectTaken currently finds only battlefield listeners, losing the targeted Treasure gift when the source is gone.
pub(in crate::card::sets) static GENEROUS_PLUNDERER: CardRecord = CardRecord::new(
    "Generous Plunderer",
    "351eea06-f5be-4044-b3b3-cc6bf805abb1",
    "Josiah \"Jo\" Cameron",
    CardRules::unsupported(),
);

// BIG 42 — Legion Extruder (alternate printing)
const LEGION_EXTRUDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LEGION_EXTRUDER,
    1,
    "f1d67fd7-30c8-4cd7-ab1a-97225b9c9437",
    "Kev Fang",
);

// BIG 43 — Memory Vessel (alternate printing)
const MEMORY_VESSEL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MEMORY_VESSEL,
    1,
    "db47f1bc-7f50-493f-bb3f-c1285cbaf244",
    "Nino Vecia",
);

// BIG 44 — Molten Duplication (alternate printing)
const MOLTEN_DUPLICATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOLTEN_DUPLICATION,
    1,
    "ad4ba0c9-ec77-4406-a62a-82a550791e61",
    "Inkognit",
);

// BIG 45 — Territory Forge (alternate printing)
const TERRITORY_FORGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TERRITORY_FORGE,
    1,
    "16cd6bde-29ec-4de2-b369-bb3e1d2714d9",
    "David Astruga",
);

// BIG 46 — Ancient Cornucopia (alternate printing)
const ANCIENT_CORNUCOPIA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ANCIENT_CORNUCOPIA,
    1,
    "2f23dffc-c76b-4f01-98c2-67dae3b3114b",
    "Mark Poole",
);

// BIG 47 — Bristlebud Farmer (alternate printing)
const BRISTLEBUD_FARMER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRISTLEBUD_FARMER,
    1,
    "713be7b8-e73f-4ed1-8a14-cc1c144e844e",
    "Artur Nakhodkin",
);

// BIG 48 — Omenpath Journey (alternate printing)
const OMENPATH_JOURNEY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OMENPATH_JOURNEY,
    1,
    "385e6c23-0eab-4bc9-8b30-524879051ce9",
    "Raymond Swanland",
);

// BIG 49 — Sandstorm Salvager (alternate printing)
const SANDSTORM_SALVAGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SANDSTORM_SALVAGER,
    1,
    "82613173-2892-47d7-8a09-2102f2f0965f",
    "Xabi Gaztelua",
);

// BIG 50 — Vaultborn Tyrant (alternate printing)
const VAULTBORN_TYRANT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &VAULTBORN_TYRANT,
    2,
    "c1f0544c-0124-497c-a8f1-5d5fb959ca39",
    "Simon Dominic",
);

// BIG 51 — Loot, the Key to Everything (alternate printing)
const LOOT_THE_KEY_TO_EVERYTHING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LOOT_THE_KEY_TO_EVERYTHING,
    1,
    "a3db1928-86f2-4606-ad2d-4b7dfd7e5d6b",
    "Gaboleps",
);

// BIG 52 — Pest Control (alternate printing)
const PEST_CONTROL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PEST_CONTROL,
    1,
    "8d67c7fb-043f-4984-a700-5e8aa44c90aa",
    "Nino Vecia",
);

// BIG 53 — Lost Jitte (alternate printing)
const LOST_JITTE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LOST_JITTE,
    1,
    "dc679a0e-bf7b-40b5-b904-9c2d3508fa83",
    "Maxime Minard",
);

// BIG 54 — Lotus Ring (alternate printing)
const LOTUS_RING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LOTUS_RING,
    1,
    "d0fb925a-5845-4d73-8122-6e7a36a9bd0d",
    "Ben Hill",
);

// BIG 55 — Nexus of Becoming (alternate printing)
const NEXUS_OF_BECOMING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NEXUS_OF_BECOMING,
    1,
    "f329b59c-1c1d-4a0a-89b4-c3fc4a8074bc",
    "Mark Poole",
);

// BIG 56 — Sword of Wealth and Power (alternate printing)
const SWORD_OF_WEALTH_AND_POWER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SWORD_OF_WEALTH_AND_POWER,
    1,
    "aaf08dd1-3d35-4842-807d-37f94b9ba578",
    "Artur Nakhodkin",
);

// BIG 57 — Torpor Orb (alternate printing)
const TORPOR_ORB_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_nph::TORPOR_ORB,
    1,
    "84dc2948-f185-4c3f-b80e-2bc8f47825c9",
    "Svetlin Velinov",
);

// BIG 58 — Transmutation Font (alternate printing)
const TRANSMUTATION_FONT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TRANSMUTATION_FONT,
    1,
    "7227906c-043b-4742-8141-694f7811dc06",
    "David Astruga",
);

// BIG 59 — Fomori Vault (alternate printing)
const FOMORI_VAULT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FOMORI_VAULT,
    1,
    "ad282dc0-6310-47ab-99cd-90f34168bd27",
    "Pablo Rivera",
);

// BIG 60 — Tarnation Vista (alternate printing)
const TARNATION_VISTA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TARNATION_VISTA,
    1,
    "ad49fd74-db70-49c0-8ad9-1185e2b0787d",
    "Artur Nakhodkin",
);

// BIG 61 — Vaultborn Tyrant (alternate printing)
const VAULTBORN_TYRANT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &VAULTBORN_TYRANT,
    3,
    "0ba9e2cc-e0a3-4c5a-9ca0-f683ff8b5c94",
    "Simon Dominic",
);

// BIG 62 — Loot, the Key to Everything (alternate printing)
const LOOT_THE_KEY_TO_EVERYTHING_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LOOT_THE_KEY_TO_EVERYTHING,
    2,
    "0d18fa57-fab9-48f6-b80a-d03fb89d2683",
    "Gaboleps",
);

// BIG 63 — Lotus Ring (alternate printing)
const LOTUS_RING_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LOTUS_RING,
    2,
    "aa77cff4-0f0a-430a-b0c8-aab82299c570",
    "Ben Hill",
);

// BIG 64 — Sword of Wealth and Power (alternate printing)
const SWORD_OF_WEALTH_AND_POWER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SWORD_OF_WEALTH_AND_POWER,
    2,
    "59193e1f-7231-4967-82eb-1cb0b5494dc7",
    "Artur Nakhodkin",
);

// BIG 65 — Tarnation Vista (alternate printing)
const TARNATION_VISTA_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TARNATION_VISTA,
    2,
    "ad50cee8-0f7f-4058-8fe1-5d7156c2429b",
    "Artur Nakhodkin",
);

// BIG 66 — Collector's Cage (alternate printing)
const COLLECTOR_S_CAGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &COLLECTOR_S_CAGE,
    2,
    "1f40f6a3-5f38-498e-99fe-e60e93d7a05c",
    "Bartek Fedyczak",
);

// BIG 67 — Grand Abolisher (alternate printing)
const GRAND_ABOLISHER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_m12::GRAND_ABOLISHER,
    2,
    "1439551c-6764-4c35-a5ba-c7adb727cf7a",
    "Aurore Folny",
);

// BIG 68 — Oltec Matterweaver (alternate printing)
const OLTEC_MATTERWEAVER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &OLTEC_MATTERWEAVER,
    2,
    "1617d39d-4989-4ac4-8cd0-ac1b4e7312df",
    "Villarrte",
);

// BIG 69 — Rest in Peace (alternate printing)
const REST_IN_PEACE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_rtr::REST_IN_PEACE,
    2,
    "3a52b1bc-cf29-49bd-a877-03a21fe8a6c5",
    "Grady Frederick",
);

// BIG 70 — Esoteric Duplicator (alternate printing)
const ESOTERIC_DUPLICATOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ESOTERIC_DUPLICATOR,
    2,
    "5ba1736b-9e11-4169-9525-1540a64a3188",
    "Anton Solovianchyk",
);

// BIG 71 — Simulacrum Synthesizer (alternate printing)
const SIMULACRUM_SYNTHESIZER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SIMULACRUM_SYNTHESIZER,
    2,
    "f0e003f9-2fad-48b4-8e5b-4cda8612ecd2",
    "Anton Solovianchyk",
);

// BIG 72 — Worldwalker Helm (alternate printing)
const WORLDWALKER_HELM_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &WORLDWALKER_HELM,
    2,
    "29db3d9f-d95e-4854-8ea6-7970877221d6",
    "Camille Alquier",
);

// BIG 73 — Greed's Gambit (alternate printing)
const GREED_S_GAMBIT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GREED_S_GAMBIT,
    2,
    "47c57172-6de4-421f-93d8-be1c601c1d0b",
    "Inkognit",
);

// BIG 74 — Harvester of Misery (alternate printing)
const HARVESTER_OF_MISERY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HARVESTER_OF_MISERY,
    2,
    "e2adbac7-e479-4c73-8978-f7fde9f6e14d",
    "Jorge Jacinto",
);

// BIG 75 — Hostile Investigator (alternate printing)
const HOSTILE_INVESTIGATOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HOSTILE_INVESTIGATOR,
    2,
    "0a3c9365-af51-4874-be5b-57d0514825fe",
    "Andrew Mar",
);

// BIG 76 — Generous Plunderer (alternate printing)
const GENEROUS_PLUNDERER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GENEROUS_PLUNDERER,
    2,
    "393bf5fc-0ad2-4116-83b4-346d61ac2911",
    "Alexander Mokhov",
);

// BIG 77 — Legion Extruder (alternate printing)
const LEGION_EXTRUDER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LEGION_EXTRUDER,
    2,
    "0ffddccb-4195-4c8b-b2b3-5c545c656fee",
    "Anton Solovianchyk",
);

// BIG 78 — Memory Vessel (alternate printing)
const MEMORY_VESSEL_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MEMORY_VESSEL,
    2,
    "3e9f1d0c-6f99-4b34-a367-052dd612ecda",
    "Diego Gisbert",
);

// BIG 79 — Molten Duplication (alternate printing)
const MOLTEN_DUPLICATION_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MOLTEN_DUPLICATION,
    2,
    "61fbf320-63e1-461e-9696-561a2d6f554a",
    "Justyna Dura",
);

// BIG 80 — Territory Forge (alternate printing)
const TERRITORY_FORGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TERRITORY_FORGE,
    2,
    "5999e2ac-7111-470a-b597-4c751ffecbfd",
    "Mirko Failoni",
);

// BIG 81 — Ancient Cornucopia (alternate printing)
const ANCIENT_CORNUCOPIA_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ANCIENT_CORNUCOPIA,
    2,
    "b1b9c600-72e2-4648-9276-99b21cec00dd",
    "Bartek Fedyczak",
);

// BIG 82 — Bristlebud Farmer (alternate printing)
const BRISTLEBUD_FARMER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BRISTLEBUD_FARMER,
    2,
    "4fa29302-65bb-4852-94dd-9c5f5477f94f",
    "Adrián Rodríguez Pérez",
);

// BIG 83 — Omenpath Journey (alternate printing)
const OMENPATH_JOURNEY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &OMENPATH_JOURNEY,
    2,
    "a33e6422-801a-48ec-8fb7-ff59e1fed25c",
    "Nereida",
);

// BIG 84 — Sandstorm Salvager (alternate printing)
const SANDSTORM_SALVAGER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SANDSTORM_SALVAGER,
    2,
    "84c96b72-b0e1-4ea3-9984-5a67718d070d",
    "Francis Tneh",
);

// BIG 85 — Vaultborn Tyrant
pub(in crate::card::sets) static VAULTBORN_TYRANT: CardRecord = CardRecord::new(
    "Vaultborn Tyrant",
    "07ca436a-e992-40a9-978a-501a82e443ed",
    "Loïc Canavaggia",
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Dinosaur"], 6, 6).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "Whenever this creature or another creature you control with \
             power 4 or greater enters, you gain 3 life and draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Source,
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::PowerAtLeast(4),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::Sequence(&[
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
        AbilityDef::triggered_if(
            "When this creature dies, if it's not a token, create a token \
             that's a copy of it, except it's an artifact in addition to \
             its other types.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            &TriggerConditionDef::SourceMatches {
                object: ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
            },
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Copy(
                &crate::card::TokenCopyDef {
                    object: &EffectRecipientDef::Source,
                    exceptions: CopyExceptionsDef::NONE
                        .with_added_types(CardTypeSet::single(CardType::Artifact)),
                },
            ))),
        ),
    ]),
);

// BIG 86 — Loot, the Key to Everything (alternate printing)
const LOOT_THE_KEY_TO_EVERYTHING_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &LOOT_THE_KEY_TO_EVERYTHING,
    3,
    "2db35dae-6701-45a1-97c4-3e3049b45555",
    "Rudy Siswanto",
);

// BIG 87 — Pest Control (alternate printing)
const PEST_CONTROL_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &PEST_CONTROL,
    2,
    "b1d62921-f76d-436a-ae8b-c3a52715b47d",
    "Jonas De Ro",
);

// BIG 88 — Lost Jitte (alternate printing)
const LOST_JITTE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LOST_JITTE,
    2,
    "a4a723b3-e93b-499e-a5cf-f79a3634d4da",
    "Yeong-Hao Han",
);

// BIG 89 — Lotus Ring (alternate printing)
const LOTUS_RING_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &LOTUS_RING,
    3,
    "23d0c7a2-9614-4e72-a18f-7187de102dc3",
    "Alayna Danner",
);

// BIG 90 — Nexus of Becoming (alternate printing)
const NEXUS_OF_BECOMING_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &NEXUS_OF_BECOMING,
    2,
    "dbb8d780-ed74-4084-ac3b-245af39d00da",
    "Adam Volker",
);

// BIG 91 — Sword of Wealth and Power (alternate printing)
const SWORD_OF_WEALTH_AND_POWER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SWORD_OF_WEALTH_AND_POWER,
    3,
    "e5fdcd21-55a3-4cad-8b73-3d6f85a69ce1",
    "Dominik Mayer",
);

// BIG 92 — Torpor Orb (alternate printing)
const TORPOR_ORB_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_nph::TORPOR_ORB,
    2,
    "d034bf77-38dc-4b43-9a00-831d89af437a",
    "Robin Olausson",
);

// BIG 93 — Transmutation Font (alternate printing)
const TRANSMUTATION_FONT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TRANSMUTATION_FONT,
    2,
    "47a2058a-24a7-4b10-a1d5-0b69e94c8688",
    "Mark Poole",
);

// BIG 94 — Fomori Vault (alternate printing)
const FOMORI_VAULT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &FOMORI_VAULT,
    2,
    "56419022-bed8-425c-99a9-fc288446efd1",
    "Jonas De Ro",
);

// BIG 95 — Tarnation Vista (alternate printing)
const TARNATION_VISTA_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &TARNATION_VISTA,
    3,
    "8b85d6ea-e312-48b9-9c22-398454d9a45c",
    "Alayna Danner",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &COLLECTOR_S_CAGE,
    &OLTEC_MATTERWEAVER,
    &ESOTERIC_DUPLICATOR,
    &SIMULACRUM_SYNTHESIZER,
    &WORLDWALKER_HELM,
    &GREED_S_GAMBIT,
    &HARVESTER_OF_MISERY,
    &HOSTILE_INVESTIGATOR,
    &LEGION_EXTRUDER,
    &MEMORY_VESSEL,
    &MOLTEN_DUPLICATION,
    &TERRITORY_FORGE,
    &ANCIENT_CORNUCOPIA,
    &BRISTLEBUD_FARMER,
    &OMENPATH_JOURNEY,
    &SANDSTORM_SALVAGER,
    &LOOT_THE_KEY_TO_EVERYTHING,
    &PEST_CONTROL,
    &LOST_JITTE,
    &LOTUS_RING,
    &NEXUS_OF_BECOMING,
    &SWORD_OF_WEALTH_AND_POWER,
    &TRANSMUTATION_FONT,
    &FOMORI_VAULT,
    &TARNATION_VISTA,
    &GENEROUS_PLUNDERER,
    &VAULTBORN_TYRANT,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    GRAND_ABOLISHER_REPRINT,
    REST_IN_PEACE_REPRINT,
    GENEROUS_PLUNDERER_ALTERNATE_1,
    VAULTBORN_TYRANT_ALTERNATE_1,
    TORPOR_ORB_REPRINT,
    COLLECTOR_S_CAGE_ALTERNATE_1,
    GRAND_ABOLISHER_ALTERNATE_1,
    OLTEC_MATTERWEAVER_ALTERNATE_1,
    REST_IN_PEACE_ALTERNATE_1,
    ESOTERIC_DUPLICATOR_ALTERNATE_1,
    SIMULACRUM_SYNTHESIZER_ALTERNATE_1,
    WORLDWALKER_HELM_ALTERNATE_1,
    GREED_S_GAMBIT_ALTERNATE_1,
    HARVESTER_OF_MISERY_ALTERNATE_1,
    HOSTILE_INVESTIGATOR_ALTERNATE_1,
    LEGION_EXTRUDER_ALTERNATE_1,
    MEMORY_VESSEL_ALTERNATE_1,
    MOLTEN_DUPLICATION_ALTERNATE_1,
    TERRITORY_FORGE_ALTERNATE_1,
    ANCIENT_CORNUCOPIA_ALTERNATE_1,
    BRISTLEBUD_FARMER_ALTERNATE_1,
    OMENPATH_JOURNEY_ALTERNATE_1,
    SANDSTORM_SALVAGER_ALTERNATE_1,
    VAULTBORN_TYRANT_ALTERNATE_2,
    LOOT_THE_KEY_TO_EVERYTHING_ALTERNATE_1,
    PEST_CONTROL_ALTERNATE_1,
    LOST_JITTE_ALTERNATE_1,
    LOTUS_RING_ALTERNATE_1,
    NEXUS_OF_BECOMING_ALTERNATE_1,
    SWORD_OF_WEALTH_AND_POWER_ALTERNATE_1,
    TORPOR_ORB_ALTERNATE_1,
    TRANSMUTATION_FONT_ALTERNATE_1,
    FOMORI_VAULT_ALTERNATE_1,
    TARNATION_VISTA_ALTERNATE_1,
    VAULTBORN_TYRANT_ALTERNATE_3,
    LOOT_THE_KEY_TO_EVERYTHING_ALTERNATE_2,
    LOTUS_RING_ALTERNATE_2,
    SWORD_OF_WEALTH_AND_POWER_ALTERNATE_2,
    TARNATION_VISTA_ALTERNATE_2,
    COLLECTOR_S_CAGE_ALTERNATE_2,
    GRAND_ABOLISHER_ALTERNATE_2,
    OLTEC_MATTERWEAVER_ALTERNATE_2,
    REST_IN_PEACE_ALTERNATE_2,
    ESOTERIC_DUPLICATOR_ALTERNATE_2,
    SIMULACRUM_SYNTHESIZER_ALTERNATE_2,
    WORLDWALKER_HELM_ALTERNATE_2,
    GREED_S_GAMBIT_ALTERNATE_2,
    HARVESTER_OF_MISERY_ALTERNATE_2,
    HOSTILE_INVESTIGATOR_ALTERNATE_2,
    GENEROUS_PLUNDERER_ALTERNATE_2,
    LEGION_EXTRUDER_ALTERNATE_2,
    MEMORY_VESSEL_ALTERNATE_2,
    MOLTEN_DUPLICATION_ALTERNATE_2,
    TERRITORY_FORGE_ALTERNATE_2,
    ANCIENT_CORNUCOPIA_ALTERNATE_2,
    BRISTLEBUD_FARMER_ALTERNATE_2,
    OMENPATH_JOURNEY_ALTERNATE_2,
    SANDSTORM_SALVAGER_ALTERNATE_2,
    LOOT_THE_KEY_TO_EVERYTHING_ALTERNATE_3,
    PEST_CONTROL_ALTERNATE_2,
    LOST_JITTE_ALTERNATE_2,
    LOTUS_RING_ALTERNATE_3,
    NEXUS_OF_BECOMING_ALTERNATE_2,
    SWORD_OF_WEALTH_AND_POWER_ALTERNATE_3,
    TORPOR_ORB_ALTERNATE_2,
    TRANSMUTATION_FONT_ALTERNATE_2,
    FOMORI_VAULT_ALTERNATE_2,
    TARNATION_VISTA_ALTERNATE_3,
];
