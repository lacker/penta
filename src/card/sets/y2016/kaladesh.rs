//! Kaladesh cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AggregateOperationDef;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::AttackEventMatcherDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CounterKind;
use crate::card::CounterKindDef;
use crate::card::CounterOperationDef;
use crate::card::DestroyFollowUpDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectValueAggregateDef;
use crate::card::ObjectValueDef;
use crate::card::PayOrDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SpellCastQueryDef;
use crate::card::TargetChooserDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::ids::ParentBinding;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "KLD",
    slug: "kaladesh",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// KLD 5 — Authority of the Consuls
pub(in crate::card::sets) static AUTHORITY_OF_THE_CONSULS: CardRecord = CardRecord::new(
    "Authority of the Consuls",
    "324b2f55-1e09-490e-8f7e-bfde85a91ac4",
    "Lake Hurwitz",
    CardRules::new_enchantment(mana_cost!("{W}")).with_abilities(&[
        AbilityDef::replacement_for(
            "Creatures your opponents control enter tapped.",
            ReplacementEventDef::ObjectEntersBattlefield {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::Opponent,
                cast: None,
            },
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
        ),
        AbilityDef::triggered(
            "Whenever a creature an opponent controls enters, you gain 1 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// KLD 15 — Fumigate
// Audit: unsupported — Needs destruction-result tracking that includes a successful destruction redirected to exile; the existing destruction continuation only records permanents moved to the graveyard.
pub(in crate::card::sets) static FUMIGATE: CardRecord = CardRecord::new(
    "Fumigate",
    "f00f27a7-9e92-4fbf-baa8-f47a5eee48a6",
    "Svetlin Velinov",
    CardRules::unsupported(),
);

// KLD 25 — Refurbish
pub(in crate::card::sets) static REFURBISH_25: CardRecord = CardRecord::new(
    "Refurbish",
    "f60e2ac4-f21f-4232-abc8-db078472408b",
    "Johann Bodin",
    CardRules::new_sorcery(mana_cost!("{3}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return target artifact card from your graveyard to the battlefield.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Artifact),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )],
        EffectDef::move_to_zone(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ZoneKind::Battlefield,
            ZonePlacement::Top,
        ),
    )]),
);

// KLD 44 — Dramatic Reversal
pub(in crate::card::sets) static DRAMATIC_REVERSAL_44: CardRecord = CardRecord::new(
    "Dramatic Reversal",
    "dcb59045-2743-48ae-8063-727e551b1c41",
    "Eric Deschamps",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell(
        "Untap all nonland permanents you control.",
        EffectDef::Untap {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
        },
    )]),
);

// KLD 48 — Gearseeker Serpent
pub(in crate::card::sets) static GEARSEEKER_SERPENT: CardRecord = CardRecord::new(
    "Gearseeker Serpent",
    "d32d8327-6ec2-4d43-b254-b04407612715",
    "Filip Burburan",
    CardRules::new_creature(mana_cost!("{5}{U}{U}"), &["Serpent"], 5, 6).with_abilities(&[
        AbilityDef::static_ability(
            "Affinity for artifacts (This spell costs {1} less to cast for \
             each artifact you control.)",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                &ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
            )),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        AbilityDef::activated(
            "{5}{U}: This creature can't be blocked this turn.",
            &[CostDef::Mana(mana_cost!("{5}{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// KLD 59 — Padeem, Consul of Innovation
pub(in crate::card::sets) static PADEEM_CONSUL_OF_INNOVATION_59: CardRecord = CardRecord::new(
    "Padeem, Consul of Innovation",
    "e31b30a7-13e8-408e-a758-60e6e9290808",
    "Matt Stewart",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Vedalken", "Artificer"], 1, 4).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::static_ability("Artifacts you control have hexproof. (They can't be the targets of spells or abilities your opponents control.)", EffectDef::StaticApply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Artifact), &[ZoneKind::Battlefield], PlayerRelation::You), effect: AppliedEffectDef::add_ability(&abilities::hexproof()) }),
AbilityDef::triggered_if("At the beginning of your upkeep, if you control the artifact with the greatest mana value or tied for the greatest mana value, draw a card.", TriggerEventDef::StepBegins { step: TurnStepDef::Upkeep, player: PlayerRelation::You }, &TriggerConditionDef::ObjectCount { query: ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::ManaValueEqualTo(ValueDef::AggregateObjectValues(&ObjectValueAggregateDef { objects: ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Artifact), &[ZoneKind::Battlefield], PlayerRelation::Any)), select: ObjectValueDef::ManaValue, operation: AggregateOperationDef::Maximum }))]), &[ZoneKind::Battlefield], PlayerRelation::You), comparison: ComparisonDef::GreaterOrEqual, amount: 1 }, abilities::draw_cards(ValueDef::Constant(1)))
]),
);

// KLD 60 — Paradoxical Outcome
pub(in crate::card::sets) static PARADOXICAL_OUTCOME: CardRecord = CardRecord::new(
    "Paradoxical Outcome",
    "17e50157-bf49-4c5f-9b8a-bf73484e63a5",
    "Nils Hamm",
    // Four mana and a fistful of Moxen back, which is a bad rate for a deck
    // that has to pay for them again and a broken one for a deck that does
    // not.
    CardRules::new_instant(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return any number of target nonland, nontoken permanents you control to their owners' \
         hands. Draw a card for each card returned to your hand this way.",
        &[AbilityTargetDef {
            predicate: AbilityTargetPredicate::Object {
                // A permanent that is neither a land nor a token. The slot names the
                // controller, so the predicate only has to say what kind of thing it is.
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
            minimum: 0,
            maximum: AbilityTargetDef::UNLIMITED,
            exact_count: None,
            divided_total: None,
            another: false,
            excludes_source: false,
            chooser: TargetChooserDef::Controller,
        }],
        // Only the targets still legal as this resolves are returned, which is what
        // "each card returned this way" counts.
        abilities::bind_objects_then(
            crate::card::ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::LegalTargets(
                TargetIndex::PRIMARY,
            )),
            &EffectDef::BindObjects(crate::card::BindObjectsDef {
                source: crate::card::ObjectCollectionSourceDef::ObjectSet(
                    ObjectSetDef::MatchingBinding {
                        binding: ParentBinding,
                        object: ObjectPredicateDef::OwnedBy(PlayerRelation::You),
                    },
                ),
                binding: Binding!("outcome_owned_by_you"),
                // The draw counts what reached your hand, which is not always what left the
                // battlefield: a permanent you control but do not own goes back to somebody
                // else's hand and pays you nothing. The count is taken before the move,
                // because afterwards the cards have new identities.
                then: &EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::BoundObjectCount(Binding!("outcome_owned_by_you")),
                    },
                ]),
            }),
        ),
    )),
);

// KLD 67 — Torrential Gearhulk
pub(in crate::card::sets) static TORRENTIAL_GEARHULK_67: CardRecord = CardRecord::new(
    "Torrential Gearhulk",
    "d52868cb-087e-4f91-91bc-455f2e2e7cd7",
    "Svetlin Velinov",
    CardRules::new_artifact_creature(mana_cost!("{4}{U}{U}"), &["Construct"], 5, 6).with_abilities(&[
abilities::flash(),
abilities::enters_trigger_with_targets("When this creature enters, you may cast target instant card from your graveyard without paying its mana cost. If that spell would be put into your graveyard, exile it instead.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::HasType(CardType::Instant), zones: &[ZoneKind::Graveyard], controller: None, owner: Some(PlayerRelation::You) })], EffectDef::MayCastTargetWithoutPaying { object: EffectRecipientDef::Target(TargetIndex::PRIMARY), ability: &AbilityDef::alternative_cast(crate::NO_COSTS, AlternativeCastKindDef::Granted, Some("Cast without paying its mana cost."), EffectDef::None).with_exile_if_put_into_graveyard() })
]),
);

// KLD 96 — Noxious Gearhulk
pub(in crate::card::sets) static NOXIOUS_GEARHULK_96: CardRecord = CardRecord::new(
    "Noxious Gearhulk",
    "9f86e5fe-8723-4494-b4cc-b7ac3a047bd1",
    "Lius Lasahido",
    CardRules::new_artifact_creature(mana_cost!("{4}{B}{B}"), &["Construct"], 5, 4).with_abilities(&[
abilities::menace(),
abilities::enters_trigger_with_targets("When this creature enters, you may destroy another target creature. If a creature is destroyed this way, you gain life equal to its toughness.", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]))], EffectDef::May { player: EffectRecipientDef::Controller, effect: &EffectDef::Destroy { object: EffectRecipientDef::Target(TargetIndex::PRIMARY), then: Some(DestroyFollowUpDef { binding: Binding!("gearhulk_destroyed"), effect: &EffectDef::GainLife { recipient: EffectRecipientDef::Controller, amount: ValueDef::AggregateObjectValues(&ObjectValueAggregateDef { objects: ObjectSetDef::Binding(Binding!("gearhulk_destroyed")), select: ObjectValueDef::Toughness, operation: AggregateOperationDef::Sum }) } }) } })
]),
);

// KLD 107 — Brazen Scourge
pub(in crate::card::sets) static BRAZEN_SCOURGE: CardRecord = CardRecord::new(
    "Brazen Scourge",
    "68c6fbdb-7b5c-4ad0-88f5-4779deae16ce",
    "Kev Walker",
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Gremlin"], 3, 3)
        .with_abilities(&[abilities::haste()]),
);

// KLD 110 — Chandra, Torch of Defiance
pub(in crate::card::sets) static CHANDRA_TORCH_OF_DEFIANCE: CardRecord =
    CardRecord::new(
    "Chandra, Torch of Defiance",
    "ff8086cd-b868-4f4e-823e-2635ad7ebc07",
    "Magali Villeneuve",
// Four abilities and no bad one: she draws, she ramps, she kills, and if
        // the game somehow goes long she ends it by herself.
        CardRules::new_planeswalker(mana_cost!("{2}{R}{R}"), &["Chandra"], 4)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                AbilityDef::activated(
                    "+1: Exile the top card of your library. You may cast that card. If you don't, Chandra, \
                     Torch of Defiance deals 2 damage to each opponent.",
                    &[CostDef::Loyalty(1)],
                    EffectDef::ExileTopAndMayCast {
                        player: EffectRecipientDef::Controller,
                        // "If you don't" is the whole of the first ability's tension: the exile
                        // happens either way, and the card is either spent now at its own cost or
                        // traded for two damage.
                        otherwise: Some(&EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(2))),
                    },
                ),
                // A loyalty ability is never a mana ability (CR 605.1a), so this one uses
                // the stack like the rest of her.
                AbilityDef::activated(
                    "+1: Add {R}{R}.",
                    &[CostDef::Loyalty(1)],
                    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red).with_amount(2)),
                ),
                AbilityDef::activated_with_targets(
                    "−3: Chandra, Torch of Defiance deals 4 damage to target creature.",
                    &[CostDef::Loyalty(-3)],
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(4),
                    ),
                ),
                AbilityDef::activated(
                    "−7: You get an emblem with \"Whenever you cast a spell, this emblem deals 5 damage to \
                     any target.\"",
                    &[CostDef::Loyalty(-7)],
                    EffectDef::create_emblem(
                        "Chandra, Torch of Defiance emblem",
                        &[AbilityDef::triggered_with_targets(
                                "Whenever you cast a spell, this emblem deals 5 damage to any target.",
                                TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)),
                                &[AbilityTargetDef::exactly_one(
                                    AbilityTargetPredicate::AnyTarget,
                                )],
                                EffectDef::damage(
                                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                    ValueDef::Constant(5),
                                ),
                            )],
                    ),
                ),
            ]),
);

// KLD 126 — Reckless Fireweaver
pub(in crate::card::sets) static RECKLESS_FIREWEAVER_126: CardRecord = CardRecord::new(
    "Reckless Fireweaver",
    "63ffac51-62c4-4170-85b3-a43d7cfae7d7",
    "Deruchenko Alexander",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Artificer"], 1, 3).with_abilities(&[
AbilityDef::triggered("Whenever an artifact you control enters, this creature deals 1 damage to each opponent.", TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::ControlledBy(PlayerRelation::You)]), None, Some(ZoneKind::Battlefield)), EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)))
]),
);

// KLD 127 — Renegade Tactics
pub(in crate::card::sets) static RENEGADE_TACTICS_127: CardRecord = CardRecord::new(
    "Renegade Tactics",
    "6c06a39c-68bb-4e65-9a6d-9d9bc745201f",
    "Yeong-Hao Han",
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature can't block this turn.\nDraw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            abilities::draw_cards(ValueDef::Constant(1)),
        ]),
    )]),
);

// KLD 131 — Spark of Creativity
// Audit: unsupported — The arbitrary-card exile operation grants a play permission as it exiles the card. It cannot first exile and reveal that card, then grant a paid play permission only if the optional damage is declined.
pub(in crate::card::sets) static SPARK_OF_CREATIVITY_131: CardRecord = CardRecord::new(
    "Spark of Creativity",
    "718bf224-5e1b-439c-a998-ceec5c0a8903",
    "Johann Bodin",
    crate::card::CardRules::unsupported(),
);

// KLD 138 — Thriving Grubs
pub(in crate::card::sets) static THRIVING_GRUBS: CardRecord = CardRecord::new(
    "Thriving Grubs",
    "bbc3184a-eeda-4f22-92de-257c20cff6e2",
    "Steve Prescott",
// The two energy it brings pay for exactly one attack, and everything
    // after that has to come from somewhere else on the board.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Gremlin"], 2, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, you get {E}{E} (two energy counters).",
            EffectDef::AddPlayerCounters {
                recipient: EffectRecipientDef::Controller,
                kind: CounterKind::named("energy"),
                amount: ValueDef::Constant(2),
            },
        ),
        AbilityDef::triggered(
            "Whenever this creature attacks, you may pay {E}{E}. If you do, put a +1/+1 counter on it.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::Energy(2)],
                &EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            )),
        ),
    ]),
);

// KLD 146 — Blossoming Defense
pub(in crate::card::sets) static BLOSSOMING_DEFENSE: CardRecord = CardRecord::new(
    "Blossoming Defense",
    "5c026c39-b09c-408a-844f-fb5eb785862a",
    "Anastasia Ovchinnikova",
    CardRules::new_instant(mana_cost!("{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control gets +2/+2 and gains hexproof \
         until end of turn. (It can't be the target of spells or \
         abilities your opponents control.)",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                AppliedEffectDef::add_ability(&abilities::hexproof()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// KLD 176 — Cloudblazer
pub(in crate::card::sets) static CLOUDBLAZER: CardRecord = CardRecord::new(
    "Cloudblazer",
    "3cb12355-abd8-4bf3-aac1-f710ac162585",
    "Dan Murayama Scott",
    CardRules::new_creature(mana_cost!("{3}{W}{U}"), &["Human", "Scout"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, you gain 2 life and draw two cards.",
            EffectDef::Sequence(&[
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
                abilities::draw_cards(ValueDef::Constant(2)),
            ]),
        ),
    ]),
);

// KLD 192 — Aetherflux Reservoir
pub(in crate::card::sets) static AETHERFLUX_RESERVOIR_192: CardRecord = CardRecord::new(
    "Aetherflux Reservoir",
    "96b6b2e1-c3e6-464c-8a13-b15deb34e862",
    "Cliff Childs",
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you cast a spell, you gain 1 life for each spell you’ve cast this turn.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::CountSpellsCastThisTurn(&SpellCastQueryDef {
                    player: PlayerRelation::You,
                    spell: ObjectPredicateDef::Any,
                }),
            },
        ),
        AbilityDef::activated_with_targets(
            "Pay 50 life: This artifact deals 50 damage to any target.",
            &[CostDef::PayLife(50)],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(50),
            ),
        ),
    ]),
);

// KLD 194 — Animation Module
pub(in crate::card::sets) static ANIMATION_MODULE_194: CardRecord = CardRecord::new(
    "Animation Module",
    "34bdc973-db45-46a6-ac48-ce88fb59920a",
    "Aaron Miller",
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
AbilityDef::triggered("Whenever one or more +1/+1 counters are put on a permanent you control, you may pay {1}. If you do, create a 1/1 colorless Servo artifact creature token.", TriggerEventDef::CountersPlaced { object: ObjectPredicateDef::ControlledBy(PlayerRelation::You), kind: CounterKind::PlusOnePlusOne }, EffectDef::PayOr(PayOrDef::optional(&[CostDef::Mana(mana_cost!("{1}"))], &EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::TokenCharacteristics::artifact_creature(&["Servo"], &[], 1, 1))))))),
AbilityDef::activated_with_targets("{3}, {T}: Choose a counter on target permanent or player. Give that permanent or player another counter of that kind.", &[CostDef::Mana(mana_cost!("{3}")), CostDef::TapSource], &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyOf(&[AbilityTargetPredicate::Player(PlayerRelation::Any), AbilityTargetPredicate::Object { object: ObjectPredicateDef::Any, zones: &[ZoneKind::Battlefield], controller: None, owner: None }]))], EffectDef::ChooseCounterKind { object: EffectRecipientDef::Target(TargetIndex::PRIMARY), then: &EffectDef::ModifyCounters { object: EffectRecipientDef::Target(TargetIndex::PRIMARY), kind: CounterKindDef::Chosen, operation: CounterOperationDef::Add, amount: ValueDef::Constant(1) } })
]),
);

// KLD 203 — Cultivator's Caravan
pub(in crate::card::sets) static CULTIVATOR_S_CARAVAN: CardRecord = CardRecord::new(
    "Cultivator's Caravan",
    "b46b3726-4bc8-4e3a-bc6d-402c81663712",
    "Mark Zug",
    CardRules::new_vehicle(mana_cost!("{3}"), 5, 5).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
        abilities::crew("Crew 3", 3),
    ]),
);

// KLD 212 — Filigree Familiar
pub(in crate::card::sets) static FILIGREE_FAMILIAR: CardRecord = CardRecord::new(
    "Filigree Familiar",
    "9cc9ecfd-6cf0-4488-a14a-afec1bc0d253",
    "Izzy",
    // Colourless, so any deck plays it, and it is never a blank: the life
    // comes in and the card comes out whatever else happens to it.
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Fox"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, you gain 2 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
        abilities::dies_trigger(
            "When this creature dies, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// KLD 215 — Foundry Inspector
pub(in crate::card::sets) static FOUNDRY_INSPECTOR_215: CardRecord = CardRecord::new(
    "Foundry Inspector",
    "93f827e8-1cc4-4a15-a4be-2e74323963b9",
    "Jason A. Engle",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Construct"], 3, 2).with_abilities(&[
        AbilityDef::static_ability(
            "Artifact spells you cast cost {1} less to cast.",
            EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                ObjectPredicateDef::HasType(CardType::Artifact),
                PlayerRelation::You,
                ValueDef::Constant(1),
            )),
        ),
    ]),
);

// KLD 230 — Renegade Freighter
pub(in crate::card::sets) static RENEGADE_FREIGHTER: CardRecord = CardRecord::new(
    "Renegade Freighter",
    "7a10e2c3-0132-4eb2-94f0-5915caca2a17",
    "Izzy",
    // A 5/4 trampler for three that any two power can turn on, which is what
    // made it the limited card of its format.
    CardRules::new_vehicle(mana_cost!("{3}"), 4, 3).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this Vehicle attacks, it gets +1/+1 and gains trample until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::crew(
            "Crew 2 (Tap any number of creatures you control with total power 2 or more: This \
             Vehicle becomes an artifact creature until end of turn.)",
            2,
        ),
    ]),
);

// KLD 235 — Smuggler's Copter
pub(in crate::card::sets) static SMUGGLER_S_COPTER: CardRecord = CardRecord::new(
    "Smuggler's Copter",
    "7832abb5-5107-4603-904e-491b221bd3e3",
    "Florian de Gesincourt",
// Two mana for a 3/3 flier that any one creature can turn on, and that
    // fixes every draw it connects with. Banned in Standard for exactly
    // that.
    CardRules::new_vehicle(mana_cost!("{2}"), 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this Vehicle attacks or blocks, you may draw a card. If you do, discard a card.",
            // "Attacks or blocks" is one printed clause with two ways in, so it is one
            // ability rather than two: a Copter that does both in a turn still loots
            // once for each.
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::Attacks(AttackEventMatcherDef::any(ObjectPredicateDef::Source)),
                TriggerEventDef::Blocks {
                    blocked: ObjectPredicateDef::Any,
                },
            ]),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                // "If you do" rather than a second clause: declining the draw declines the
                // discard with it.
                effect: &EffectDef::Sequence(&[
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                ]),
            },
        ),
        abilities::crew(
            "Crew 1 (Tap any number of creatures you control with total power 1 or more: This \
             Vehicle becomes an artifact creature until end of turn.)",
            1,
        ),
    ]),
);

// KLD 243 — Blooming Marsh
pub(in crate::card::sets) static BLOOMING_MARSH: CardRecord = CardRecord::new(
    "Blooming Marsh",
    "90da33d4-fe9c-42fe-b326-2fe337dc3ecd",
    "Adam Paquette",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::fast_land_enters(),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// KLD 244 — Botanical Sanctum
pub(in crate::card::sets) static BOTANICAL_SANCTUM: CardRecord = CardRecord::new(
    "Botanical Sanctum",
    "8744471b-a528-47d9-84d0-4526273f55e9",
    "Christine Choi",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::fast_land_enters(),
        AbilityDef::activated_mana(
            "{T}: Add {G} or {U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// KLD 245 — Concealed Courtyard
pub(in crate::card::sets) static CONCEALED_COURTYARD: CardRecord = CardRecord::new(
    "Concealed Courtyard",
    "c8769e97-aee8-4466-a9d7-0f4245ae4a97",
    "Jung Park",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::fast_land_enters(),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {B}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Black,
            ])),
        ),
    ]),
);

// KLD 246 — Inspiring Vantage
pub(in crate::card::sets) static INSPIRING_VANTAGE: CardRecord = CardRecord::new(
    "Inspiring Vantage",
    "160ac412-005f-48ca-a204-10207307c6c2",
    "Jonas De Ro",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::fast_land_enters(),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {W}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::White,
            ])),
        ),
    ]),
);

// KLD 247 — Inventors' Fair
pub(in crate::card::sets) static INVENTORS_FAIR_247: CardRecord = CardRecord::new(
    "Inventors' Fair",
    "275471e3-ded1-40ac-91ef-369dce5764d9",
    "Jonas De Ro",
    CardRules::new_land(&[]).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::triggered_if("At the beginning of your upkeep, if you control three or more artifacts, you gain 1 life.", TriggerEventDef::StepBegins { step: TurnStepDef::Upkeep, player: PlayerRelation::You }, &TriggerConditionDef::ObjectCount { query: ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Artifact), &[ZoneKind::Battlefield], PlayerRelation::You), comparison: ComparisonDef::GreaterOrEqual, amount: 3 }, EffectDef::GainLife { recipient: EffectRecipientDef::Controller, amount: ValueDef::Constant(1) }),
abilities::tap_for(ManaColor::Colorless),
AbilityDef::activated("{4}, {T}, Sacrifice Inventors' Fair: Search your library for an artifact card, reveal it, put it into your hand, then shuffle. Activate only if you control three or more artifacts.", &[CostDef::Mana(mana_cost!("{4}")), CostDef::TapSource, CostDef::SacrificeSource], EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::HasType(CardType::Artifact), minimum: 0, maximum: ValueDef::Constant(1), reveal: true, destination: ZoneKind::Hand, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None }).with_activation_condition(&TriggerConditionDef::ObjectCount { query: ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Artifact), &[ZoneKind::Battlefield], PlayerRelation::You), comparison: ComparisonDef::GreaterOrEqual, amount: 3 })
]),
);

// KLD 249 — Spirebluff Canal
pub(in crate::card::sets) static SPIREBLUFF_CANAL: CardRecord = CardRecord::new(
    "Spirebluff Canal",
    "4e587ea7-0632-4789-ba75-3c410da2bb96",
    "Adam Paquette",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::fast_land_enters(),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Red])),
        ),
    ]),
);

// KLD 266 — Flame Lash
pub(in crate::card::sets) static FLAME_LASH: CardRecord = CardRecord::new(
    "Flame Lash",
    "ac44e3cb-cc69-4222-87bc-ffa54b7ab34a",
    "Viktor Titov",
    CardRules::new_instant(mana_cost!("{3}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Flame Lash deals 4 damage to any target.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::Constant(4),
        ),
    )]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AUTHORITY_OF_THE_CONSULS,
    &FUMIGATE,
    &REFURBISH_25,
    &DRAMATIC_REVERSAL_44,
    &GEARSEEKER_SERPENT,
    &PADEEM_CONSUL_OF_INNOVATION_59,
    &PARADOXICAL_OUTCOME,
    &TORRENTIAL_GEARHULK_67,
    &NOXIOUS_GEARHULK_96,
    &BRAZEN_SCOURGE,
    &CHANDRA_TORCH_OF_DEFIANCE,
    &RECKLESS_FIREWEAVER_126,
    &RENEGADE_TACTICS_127,
    &SPARK_OF_CREATIVITY_131,
    &THRIVING_GRUBS,
    &BLOSSOMING_DEFENSE,
    &CLOUDBLAZER,
    &AETHERFLUX_RESERVOIR_192,
    &ANIMATION_MODULE_194,
    &CULTIVATOR_S_CARAVAN,
    &FILIGREE_FAMILIAR,
    &FOUNDRY_INSPECTOR_215,
    &RENEGADE_FREIGHTER,
    &SMUGGLER_S_COPTER,
    &BLOOMING_MARSH,
    &BOTANICAL_SANCTUM,
    &CONCEALED_COURTYARD,
    &INSPIRING_VANTAGE,
    &INVENTORS_FAIR_247,
    &SPIREBLUFF_CANAL,
    &FLAME_LASH,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
