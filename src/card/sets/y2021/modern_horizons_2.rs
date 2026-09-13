//! Modern Horizons 2 cards cataloged as cross-format rules-engine test cases.

use super::CardRecord;
use super::PrintingRecord;
use crate::AdditionalCostIndex;
use crate::ParentBinding;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityKindDef;
use crate::card::AbilityPredicateDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BasicLandType;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BindObjectsDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::CharacteristicOperationDef;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::DamageEventMatcherDef;
use crate::card::DamageKindDef;
use crate::card::DamageRecipientMatcherDef;
use crate::card::DamageSourceMatcherDef;
use crate::card::DiscardFollowUpDef;
use crate::card::DiscardSelectionDef;
use crate::card::DividedTotal;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ExilePlayDurationDef;
use crate::card::FreePlayDef;
use crate::card::FreePlayDurationDef;
use crate::card::GraveyardPlayPermissionDef;
use crate::card::GraveyardTypeConditionDef;
use crate::card::ManaColor;
use crate::card::MillLoopDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::OngoingEffectDef;
use crate::card::PayOrDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::PowerToughnessOperationDef;
use crate::card::PrintedManaCost;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SacrificedAmountDef;
use crate::card::SetOperationDef;
use crate::card::SpellCastQueryDef;
use crate::card::SubtypeDef;
use crate::card::SuspendAbilityDef;
use crate::card::TargetChooserDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::tokens;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "MH2",
    slug: "modern-horizons-2",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const CLUE_TOKEN: TokenCharacteristics = tokens::clue().with_art(CardArt::new(
    "cfcf6efe-9f94-4a55-9994-701e596691ad",
    "John Avon",
));

const TREASURE_TOKEN: TokenCharacteristics = tokens::treasure().with_art(CardArt::new(
    "630c0d1c-9ddb-4e76-a82a-9cdd8a5b487b",
    "Alayna Danner",
));

// MH2 25 — Prismatic Ending
pub(in crate::card::sets) static PRISMATIC_ENDING: CardRecord = CardRecord::new(
    "Prismatic Ending",
    "825969b9-3c70-4fca-8cab-696e9ca7cdb2",
    "John Stanko",
// X buys nothing by itself: it is a sink for the extra colours, and how
    // many different ones went in is the only thing the spell reads.
    CardRules::new_sorcery(mana_cost!("{X}{W}")).with_ability(AbilityDef::spell_with_targets(
            "Converge — Exile target nonland permanent if its mana value is less than or equal to the number of colors of mana spent to cast this spell.",
            // A nonland permanent of any size may be targeted; whether it is actually
            // exiled is settled on resolution, against what paid for the spell.
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            )],
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::ManaValueAtMostValue(ValueDef::ColorsOfManaSpent),
                },
                then: &EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            },
        )),
);

// MH2 27 — Sanctifier en-Vec
// Audit: unsupported — The nonbattlefield graveyard-move replacement matcher reads only owner and
// token status, not color. Its replacement would miss black and red spells and cards moving from
// hand or library, even if battlefield deaths worked.
pub(in crate::card::sets) static SANCTIFIER_EN_VEC_27: CardRecord = CardRecord::new(
    "Sanctifier en-Vec",
    "f8c3cca4-23c0-4c14-ab56-51ba011f5974",
    "Michael C. Hayes",
    CardRules::unsupported(),
);

// MH2 32 — Solitude
pub(in crate::card::sets) static SOLITUDE: CardRecord = CardRecord::new(
    "Solitude",
    "47a6234f-309f-4e03-9263-66da48b57153",
    "Evan Shipard",
// Two white cards for a free Swords to Plowshares at instant speed, and
    // a lifelinking 3/2 on the turns five mana is available instead.
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Elemental", "Incarnation"], 3, 2)
        .with_abilities(&crate::ability_list![
            [
                abilities::flash(),
                abilities::lifelink(),
                abilities::enters_trigger_with_targets(
                    "When this creature enters, exile up to one other target creature. That creature's \
                     controller gains life equal to its power.",
                    // "Up to one other": declining is a legal choice, and Solitude herself is
                    // never one of the options.
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
                        1,
                    )],
                    // Swords to Plowshares' pair, in the same order: the power the life is
                    // read from is the one the creature had as it left the battlefield.
                    EffectDef::Sequence(&[
                        EffectDef::move_to_zone(
                            EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            ZoneKind::Exile,
                            ZonePlacement::Top,
                        ),
                        EffectDef::GainLife {
                            recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                            amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
                        },
                    ]),
                ),
            ],
            abilities::evoke(
                &[CostDef::exile(
                    ObjectPredicateDef::Color(ManaColor::White),
                    ZoneKind::Hand,
                    CostQuantityDef::Fixed(1),
                )],
            ),
        ]),
);

// MH2 36 — Unbounded Potential
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNBOUNDED_POTENTIAL: CardRecord = CardRecord::new(
    "Unbounded Potential",
    "9955a344-dcd8-404d-9757-f62ed158ba22",
    "Iain McCaig",
    crate::card::CardRules::unsupported(),
);

// MH2 39 — Dress Down
pub(in crate::card::sets) static DRESS_DOWN_39: CardRecord = CardRecord::new(
    "Dress Down",
    "04f9f061-67b8-4427-9fcb-b3ccfee8fc5d",
    "Iain McCaig",
    CardRules::new_enchantment(mana_cost!("{1}{U}")).with_abilities(&[
        abilities::flash(),
        abilities::enters_trigger(
            "When this enchantment enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::static_ability(
            "Creatures lose all abilities.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::remove_abilities(crate::card::AbilityPredicateDef::Any),
            },
        ),
        AbilityDef::triggered(
            "At the beginning of the end step, sacrifice this enchantment.",
            TriggerEventDef::StepBegins {
                step: crate::card::TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            EffectDef::sacrifice(EffectRecipientDef::Source),
        ),
    ]),
);

// MH2 46 — Hard Evidence
pub(in crate::card::sets) static HARD_EVIDENCE: CardRecord = CardRecord::new(
    "Hard Evidence",
    "501599d6-1072-4124-b05d-01f96de153f3",
    "Yeong-Hao Han",
    // One mana buys a blocker and a card, which is why the Crab's zero power
    // costs the deck nothing.
    CardRules::new_sorcery(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Create a 0/3 blue Crab creature token. Investigate. (Create a Clue token. It's an \
         artifact with \"{2}, Sacrifice this token: Draw a card.\")",
        EffectDef::Sequence(&[
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Crab"], &[ManaColor::Blue], 0, 3),
            ))),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))),
        ]),
    )),
);

// MH2 49 — Lose Focus
pub(in crate::card::sets) static LOSE_FOCUS: CardRecord = CardRecord::new(
    "Lose Focus",
    "985bdb0c-ce6c-4506-8163-76f3b2fdf5fb",
    "Martina Fačková",
    // A soft counter that stops being soft once there is spare mana: each
    // replicate is another {2} the other player has to find.
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        abilities::replicate(&[CostDef::Mana(mana_cost!("{U}"))]),
        AbilityDef::spell_with_targets(
            "Counter target spell unless its controller pays {2}.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Spell,
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            abilities::counter_target_unless_paid(&[CostDef::GenericMana(ValueDef::Constant(2))]),
        ),
        AbilityDef::triggered(
            "Replicate {U} (When you cast this spell, copy it for each time you paid its \
             replicate cost. You may choose new targets for the copies.)",
            TriggerEventDef::spell_cast(ObjectPredicateDef::Source),
            // The copies are a cast trigger rather than part of the spell's own clause,
            // exactly as storm is: what differs is only where the count comes from, and
            // replicate counts what was paid rather than what was cast.
            EffectDef::CopyStackObject(&crate::card::CopyStackObjectDef {
                object: EffectRecipientDef::Source,
                controller: PlayerRefDef::EffectController,
                count: ValueDef::AdditionalCostPayments(AdditionalCostIndex::PRIMARY),
                retarget: true,
                colors: None,
            }),
        ),
    ]),
);

// MH2 52 — Murktide Regent
pub(in crate::card::sets) static MURKTIDE_REGENT: CardRecord = CardRecord::new(
    "Murktide Regent",
    "20c4aae1-7665-4df7-bd51-a1d95bf8a17d",
    "Lucas Graciano",
CardRules::new_creature(mana_cost!("{5}{U}{U}"), &["Dragon"], 3, 3).with_abilities(&[
        abilities::delve(),
        abilities::flying(),
        AbilityDef::as_enters(
            "This creature enters with a +1/+1 counter on it for each instant and sorcery card exiled with it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCountersValue {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::CountObjects(&ObjectSetDef::Matching {
                        objects: &ObjectSetDef::LinkedExiles,
                        object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ])),
                    }),
                },
            ),
        ),
        AbilityDef::triggered(
            "Whenever an instant or sorcery card leaves your graveyard, put a +1/+1 counter on this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::OwnedBy(PlayerRelation::You),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                ]),
                Some(ZoneKind::Graveyard),
                None,
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// MH2 61 — Scuttletide
pub(in crate::card::sets) static SCUTTLETIDE_61: CardRecord = CardRecord::new(
    "Scuttletide",
    "38e4ce27-aba2-4a3f-8de7-d442323d8be2",
    "Yeong-Hao Han",
    CardRules::new_enchantment(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::activated(
            "{1}, Discard a card: Create a 0/3 blue Crab creature token.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::discard(ObjectPredicateDef::Any)],
            EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::TokenCharacteristics::creature(&["Crab"], &[ManaColor::Blue], 0, 3)))),
        ),
        AbilityDef::static_ability(
            "Delirium — Crabs you control get +1/+1 as long as there are four or more card types among cards in your graveyard.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(4),
                }),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Crab")),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
                },
            },
        ),
    ]),
);

// MH2 66 — Step Through
pub(in crate::card::sets) static STEP_THROUGH_66: CardRecord = CardRecord::new(
    "Step Through",
    "716534cb-aa89-4de7-9aa5-8d8aa4422a6a",
    "Randy Gallegos",
    CardRules::new_sorcery(mana_cost!("{3}{U}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return two target creatures to their owners' hands.",
            &[
                AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature)),
                AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature)),
            ],
            EffectDef::Sequence(&[
                EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Hand, ZonePlacement::Top),
                EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex(1)), ZoneKind::Hand, ZonePlacement::Top),
            ]),
        ),
        abilities::typecycling!(
            "Wizardcycling {2} ({2}, Discard this card: Search your library for a Wizard card, reveal it, put it into your hand, then shuffle.)",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Wizard")),
        ),
    ]),
);

// MH2 67 — Subtlety
pub(in crate::card::sets) static SUBTLETY: CardRecord = CardRecord::new(
    "Subtlety",
    "701256d5-1389-48b7-9581-d6037209bd06",
    "Anastasia Ovchinnikova",
// Free interaction that leaves a body when you have the mana, and a
    // blue card off the top of your hand when you do not.
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Elemental", "Incarnation"], 3, 3)
        .with_abilities(&crate::ability_list![
            [
                abilities::flash(),
                abilities::flying(),
                abilities::enters_trigger_with_targets(
                    "When this creature enters, choose up to one target creature spell or planeswalker \
                     spell. Its owner puts it on their choice of the top or bottom of their library.",
                    // A creature or planeswalker spell on the stack, anybody's. "Up to one"
                    // means a Subtlety with nothing worth answering still enters and still
                    // leaves a 3/3 behind.
                    &[AbilityTargetDef::up_to(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Spell,
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                                ]),
                            ]),
                            zones: &[ZoneKind::Stack],
                            controller: None,
                            owner: None,
                        },
                        1,
                    )],
                    EffectDef::PutSpellIntoOwnersLibrary {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                ),
            ],
            abilities::evoke(
                &[CostDef::exile(
                    ObjectPredicateDef::Color(ManaColor::Blue),
                    ZoneKind::Hand,
                    CostQuantityDef::Fixed(1),
                )],
            ),
        ]),
);

// MH2 68 — Suspend
pub(in crate::card::sets) static SUSPEND_68: CardRecord = CardRecord::new(
    "Suspend",
    "40af215c-d3f7-42f0-85cd-77a3fcd919ba",
    "Lake Hurwitz",
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[
AbilityDef::spell_with_targets("Exile target creature and put two time counters on it. If it doesn't have suspend, it gains suspend. (At the beginning of its owner's upkeep, they remove a time counter. When the last is removed, they may play it without paying its mana cost. If it's a creature, it has haste.)", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))], EffectDef::BindObjects(BindObjectsDef { source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::LegalTargets(TargetIndex::PRIMARY)), binding: Binding!("suspend_target"), then: &EffectDef::Sequence(&[EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("suspend_target"))), ZoneKind::Exile, ZonePlacement::Top), EffectDef::BindObjects(BindObjectsDef { source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::ZoneChangeSuccessorsOfBinding(Binding!("suspend_target"))), binding: Binding!("suspend_exiled"), then: &EffectDef::ForEachInBinding { objects: Binding!("suspend_exiled"), binding: Binding!("suspend_card"), effect: &EffectDef::Sequence(&[EffectDef::AddCounters { object: EffectRecipientDef::object(ObjectRefDef::Binding(Binding!("suspend_card"))), kind: CounterKind::named("time"), amount: ValueDef::Constant(2) }, EffectDef::IfCondition { condition: &TriggerConditionDef::Not(&TriggerConditionDef::BoundObjectMatches { binding: Binding!("suspend_card"), object: ObjectPredicateDef::HasAbility(AbilityPredicateDef::Is(AbilityKindDef::Suspend)) }), then: &EffectDef::Apply { recipient: EffectRecipientDef::object(ObjectRefDef::Binding(Binding!("suspend_card"))), effect: AppliedEffectDef::add_ability(&abilities::suspend("Suspend", &SuspendAbilityDef::granted())), duration: ResolvedEffectDurationDef::Permanent } }]) } })]) }))
]),
);

// MH2 75 — Archon of Cruelty
static A_CREATURE_OR_PLANESWALKER: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::HasType(CardType::Planeswalker),
]);

pub(in crate::card::sets) static ARCHON_OF_CRUELTY: CardRecord = CardRecord::new(
    "Archon of Cruelty",
    "1be9d9a4-d7ee-4854-abc2-85cabf993ec9",
    "Andrew Mar",
    // Eight mana nobody pays: he is a reanimation target, and the trigger is
    // why -- a six-point swing and two cards the turn he lands, and again
    // every turn he attacks.
    CardRules::new_creature(mana_cost!("{6}{B}{B}"), &["Archon"], 6, 6).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature enters or attacks, target opponent sacrifices a creature or \
             planeswalker of their choice, discards a card, and loses 3 life. You draw a card and \
             gain 3 life.",
            // One printed ability with two ways in: he arrives, or he attacks. Two
            // abilities would make him trigger twice on a turn he does both, which the
            // card does not say -- and would count as two triggered abilities where the
            // card has one.
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
            ]),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            // Four things in one sentence, in the order they are printed: what the
            // opponent gives up, then what you get. The sacrifice is theirs to choose,
            // which is why it is a procedure rather than a targeted destruction.
            EffectDef::Sequence(&[
                EffectDef::SacrificeOfChoice {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    object: A_CREATURE_OR_PLANESWALKER,
                    count: ValueDef::Constant(1),
                    then: None,
                    amount: SacrificedAmountDef::Power,
                    otherwise: None,
                    optional: false,
                },
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(3),
                },
                EffectDef::Sequence(&[
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                ]),
            ]),
        ),
    ]),
);

// MH2 76 — Bone Shards
pub(in crate::card::sets) static BONE_SHARDS: CardRecord = CardRecord::new(
    "Bone Shards",
    "1ee98955-4c47-4d45-9377-608dfa755337",
    "Tommy Arnold",
// One black kills anything, and the second card is the price. A deck
    // full of things it wants in the graveyard pays it gladly.
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a creature or discard a card.\nDestroy target creature or planeswalker.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
            )],
            // The second half of "sacrifice a creature or discard a card". Which half
            // is paid is settled as the spell is cast: both spend a card the caster
            // already had, and the enumeration offers every one of them.
            CostDef::choice(&[
                CostDef::sacrifice(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    CostQuantityDef::Fixed(1),
                ),
                CostDef::discard(ObjectPredicateDef::Any),
            ]),
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ),
);

// MH2 80 — Damn
pub(in crate::card::sets) static DAMN: CardRecord = CardRecord::new(
    "Damn",
    "efeae088-9ac5-4d2f-a15c-d8675a471ac5",
    "Lucas Graciano",
    // Two black is removal and four with two white is a Wrath, off one card
    // -- and neither half leaves anything to regenerate, which is what puts
    // it ahead of the sorceries it is otherwise a copy of.
    CardRules::new_sorcery(mana_cost!("{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target creature. A creature destroyed this way can't be regenerated.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::WithRule {
                rule: AppliedRuleDef::CannotRegenerate,
                effect: &EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            },
        ),
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{2}{W}{W}"))],
            AlternativeCastKindDef::Overload,
            Some("Destroy each creature. A creature destroyed this way can't be regenerated."),
            EffectDef::WithRule {
                rule: AppliedRuleDef::CannotRegenerate,
                effect: &EffectDef::Destroy {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    then: None,
                },
            },
        ),
    ]),
);

// MH2 87 — Grief
pub(in crate::card::sets) static GRIEF: CardRecord = CardRecord::new(
    "Grief",
    "e6befbc4-1320-4f26-bd9f-b1814fedda10",
    "Nicholas Gregory",
// Two black cards for a Thoughtseize on turn one, and a 3/2 that is
    // hard to block on the turns you have four mana instead.
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Elemental", "Incarnation"], 3, 2)
        // Thoughtseize's clause without the life, and aimed at an opponent rather
        // than any player: revealed rather than looked at, so the choice is one
        // both players can check.
        .with_abilities(&crate::ability_list![
            [
                abilities::menace(),
                abilities::enters_trigger_with_targets(
                    "When this creature enters, target opponent reveals their hand. You choose a nonland \
                     card from it. That player discards that card.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                    )],
                    EffectDef::Sequence(&abilities::reveal_hand_and_discard_chosen_card(
                        PlayerRefDef::Target(TargetIndex::PRIMARY),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    )),
                ),
            ],
            abilities::evoke(
                &[CostDef::exile(
                    ObjectPredicateDef::Color(ManaColor::Black),
                    ZoneKind::Hand,
                    CostQuantityDef::Fixed(1),
                )],
            ),
        ]),
);

// MH2 88 — Hell Mongrel
// Audit: unsupported — Madness needs a discard-to-exile replacement followed by a linked triggered cast-or-graveyard choice. No existing alternative-cast procedure implements that timing and zone-change sequence.
pub(in crate::card::sets) static HELL_MONGREL_88: CardRecord = CardRecord::new(
    "Hell Mongrel",
    "f7da32a3-8e33-4603-abd2-8db144062f6a",
    "Robbie Trevino",
    crate::card::CardRules::unsupported(),
);

// MH2 91 — Loathsome Curator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOATHSOME_CURATOR: CardRecord = CardRecord::new(
    "Loathsome Curator",
    "11a59a6f-6ef0-4acc-8358-a4e2cebdb7d5",
    "Mila Pesic",
    crate::card::CardRules::unsupported(),
);

// MH2 95 — Nested Shambler
pub(in crate::card::sets) static NESTED_SHAMBLER: CardRecord = CardRecord::new(
    "Nested Shambler",
    "9851f290-f502-49f8-9b48-67f7966d4e34",
    "Nicholas Gregory",
// Its own power is the payout, so pumping it before it dies is the
    // whole deck the card asks for.
    CardRules::new_creature(mana_cost!("{B}"), &["Zombie"], 1, 1).with_ability(
        abilities::dies_trigger(
            "When this creature dies, create X tapped 1/1 green Squirrel creature tokens, where X is this creature's power.",
            // Last-known power: it is already in the graveyard by the time
            // the trigger resolves, so a pump that resolved first still
            // counts.
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                    &["Squirrel"],
                    &[ManaColor::Green],
                    1,
                    1,
                )))
                .with_count(ValueDef::SourcePower)
                .entering_tapped(),
            ),
        ),
    ),
);

// MH2 96 — Persist
pub(in crate::card::sets) static PERSIST_96: CardRecord = CardRecord::new(
    "Persist",
    "90f390c3-af1c-424f-9721-e26e9321e5a3",
    "Milivoj Ćeran",
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Return target nonlegendary creature card from your graveyard to the battlefield with a -1/-1 counter on it.",
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(CardSupertype::Legendary)),
            ]),
            zones: &[ZoneKind::Graveyard], controller: None, owner: Some(PlayerRelation::You),
        })],
        EffectDef::WithBattlefieldArrival {
            effect: &EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Battlefield, ZonePlacement::Top),
            arrival: crate::card::BattlefieldArrivalDef {
                counters: Some(crate::card::TokenCountersDef {
                    kind: CounterKind::MinusOneMinusOne,
                    amount: ValueDef::Constant(1),
                }),
                ..crate::card::BattlefieldArrivalDef::DEFAULT
            },
        },
    )),
);

// MH2 102 — Tourach, Dread Cantor
pub(in crate::card::sets) static TOURACH_DREAD_CANTOR_102: CardRecord = CardRecord::new(
"Tourach, Dread Cantor",
"f3526751-0101-4d91-a496-c53cd92326e0",
"Greg Staples",
CardRules::new_creature(mana_cost!("{1}{B}"), &["Human",
"Cleric"], 2, 1).with_supertype(CardSupertype::Legendary).with_abilities(&[abilities::kicker(&[CostDef::Mana(mana_cost!("{B}{B}"))]),
abilities::protection_from_color(ManaColor::White),
AbilityDef::triggered("Whenever an opponent discards a card, put a +1/+1 counter on Tourach.", TriggerEventDef::Discarded(PlayerRelation::Opponent), EffectDef::AddCounters { object: EffectRecipientDef::Source, kind: CounterKind::PlusOnePlusOne, amount: ValueDef::Constant(1) }),
AbilityDef::triggered_if_with_targets("When Tourach enters, if it was kicked, target opponent discards two cards at random.", TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)), &TriggerConditionDef::SourcePaidAdditionalCost(AdditionalCostIndex::PRIMARY), &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Opponent))], EffectDef::Discard { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), amount: ValueDef::Constant(2), selection: DiscardSelectionDef::Random, then: None })]));

// MH2 106 — Unmarked Grave
pub(in crate::card::sets) static UNMARKED_GRAVE_106: CardRecord = CardRecord::new(
    "Unmarked Grave",
    "492b368b-de32-45c1-8459-238aae54f9fc",
    "James Paick",
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell(
        "Search your library for a nonlegendary card, put that card into your graveyard, then shuffle.",
        EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(CardSupertype::Legendary)), minimum: 0, maximum: ValueDef::Constant(1), reveal: false, destination: ZoneKind::Graveyard, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None },
    )),
);

// MH2 107 — Vermin Gorger
pub(in crate::card::sets) static VERMIN_GORGER: CardRecord = CardRecord::new(
    "Vermin Gorger",
    "d3166b10-5bc3-4db6-bb5b-81045d98e446",
    "Tobias Kwan",
    // A four-point swing per creature fed to it, and the Nested Shambler
    // two entries up is exactly the sort of thing it eats.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire"], 2, 2).with_ability(
        AbilityDef::activated(
            "{T}, Sacrifice another creature: Each opponent loses 2 life and you gain 2 life.",
            &[
                CostDef::TapSource,
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(
                        PlayerRelation::Opponent,
                    )),
                    amount: ValueDef::Constant(2),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
    ),
);

// MH2 108 — Vile Entomber
pub(in crate::card::sets) static VILE_ENTOMBER: CardRecord = CardRecord::new(
    "Vile Entomber",
    "d890ae71-da2b-44fa-8cfa-9c3016c9f696",
    "Chris Cold",
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Zombie", "Warlock"], 2, 2).with_abilities(
        &[
            abilities::deathtouch(),
            abilities::enters_trigger(
                "When this creature enters, search your library for a card, \
                 put that card into your graveyard, then shuffle.",
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::Any,
                    minimum: 1,
                    maximum: ValueDef::Constant(1),
                    reveal: false,
                    destination: ZoneKind::Graveyard,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
        ],
    ),
);

// MH2 115 — Blazing Rootwalla
// Audit: unsupported — Madness needs a discard-to-exile replacement followed by a linked triggered cast-or-graveyard choice. No existing alternative-cast procedure implements that timing and zone-change sequence.
pub(in crate::card::sets) static BLAZING_ROOTWALLA_115: CardRecord = CardRecord::new(
    "Blazing Rootwalla",
    "4404fc9c-ef02-479c-9638-0cc163f0b48f",
    "Jokubas Uogintas",
    crate::card::CardRules::unsupported(),
);

// MH2 121 — Dragon's Rage Channeler
pub(in crate::card::sets) static DRAGON_S_RAGE_CHANNELER: CardRecord = CardRecord::new(
    "Dragon's Rage Channeler",
    "4ced112a-e775-4f97-97b3-74877e9dce12",
    "Martina Fačková",
// One mana for a 1/1 that fills its own graveyard and turns into a 3/3
    // flier for doing what the deck was going to do anyway. The compulsion
    // to attack is the price, and it is rarely one.
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Shaman"], 1, 1)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever you cast a noncreature spell, surveil 1. (Look at the top card of your library. \
                 You may put that card into your graveyard.)",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                abilities::surveil(ValueDef::Constant(1)),
            ),
            // "As long as", so it is asked live rather than once: the 3/3 flier is
            // a 1/1 again the moment the fourth card type leaves the graveyard.
            AbilityDef::static_ability(
                "Delirium — As long as there are four or more card types among cards in your graveyard, \
                 this creature gets +2/+2, has flying, and attacks each combat if able.",
                EffectDef::IfCondition {
                    // Four card types among the cards in your own graveyard, which the surveil
                    // above is what fills: the look is the cost of nothing and the delirium is
                    // what it buys.
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                        comparison: ComparisonDef::GreaterOrEqual,
                        right: ValueDef::Constant(4),
                    }),
                    // Three grants under one condition, so they arrive and leave together: a
                    // graveyard that falls back under four takes the flying and the compulsion
                    // with it.
                    then: &EffectDef::Sequence(&[
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(2),
                                ValueDef::Constant(2),
                            ),
                        },
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::add_ability(&abilities::flying()),
                        },
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::add_ability(&abilities::attacks_each_combat_if_able()),
                        },
                    ]),
                },
            ),
        ]),
);

// MH2 126 — Fury
pub(in crate::card::sets) static FURY: CardRecord = CardRecord::new(
    "Fury",
    "bd281158-8180-40b9-a5b7-03cfc712d81a",
    "Raoul Vitale",
CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Elemental", "Incarnation"], 3, 3)
        .with_abilities(&crate::ability_list![
            [
                abilities::double_strike(),
                abilities::enters_trigger_with_targets(
                    "When this creature enters, it deals 4 damage divided as you choose among any number of target creatures and/or planeswalkers.",
                    // Four damage split however the caster likes, over creatures and
                    // planeswalkers alike. Every target must be assigned at least one, so four
                    // is the most it can ever cover.
                    &[AbilityTargetDef {
                        predicate: AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ]),
                            zones: &[ZoneKind::Battlefield],
                            controller: None,
                            owner: None,
                        },
                        minimum: 1,
                        maximum: AbilityTargetDef::UNLIMITED,
                        exact_count: None,
                        divided_total: Some(DividedTotal::Fixed(4)),
                        another: false,
                        excludes_source: false,
                        chooser: TargetChooserDef::Controller,
                    }],
                    EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::DividedAmongTargets,
                    ),
                ),
            ],
            abilities::evoke(
                &[CostDef::exile(
                    ObjectPredicateDef::Color(ManaColor::Red),
                    ZoneKind::Hand,
                    CostQuantityDef::Fixed(1),
                )],
            ),
        ]),
);

// MH2 127 — Galvanic Relay
// Audit: unsupported — ExilePlayDurationDef only offers permissions starting immediately. It cannot defer the play window until the next turn and disallow playing the exiled card during the current turn.
pub(in crate::card::sets) static GALVANIC_RELAY_127: CardRecord = CardRecord::new(
    "Galvanic Relay",
    "06373318-e548-4664-b227-17e3b6fd0a88",
    "Lucas Staniec",
    crate::card::CardRules::unsupported(),
);

// MH2 132 — Harmonic Prodigy
// Audit: unsupported — AdditionalTriggerDef only doubles entry-caused triggers. It cannot double arbitrary triggered abilities of Shamans and other Wizards, including prowess itself.
pub(in crate::card::sets) static HARMONIC_PRODIGY_132: CardRecord = CardRecord::new(
    "Harmonic Prodigy",
    "22579ac0-ad3f-4000-a65a-46a17a7f1aa5",
    "Paul Scott Canavan",
    crate::card::CardRules::unsupported(),
);

// MH2 135 — Mine Collapse
pub(in crate::card::sets) static MINE_COLLAPSE: CardRecord = CardRecord::new(
    "Mine Collapse",
    "56e2e8b5-660d-4469-a4fe-2367dfadb709",
    "Bud Cook",
    // Nobody pays four mana for this. What it is worth is a land off an
    // already-flooded board on your own turn, which is why the free half is
    // the half that reads "if it's your turn".
    CardRules::new_instant(mana_cost!("{3}{R}")).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::sacrifice(
                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
                CostQuantityDef::Fixed(1),
            )],
            AlternativeCastKindDef::AlternativeCost,
            Some(
                "If it's your turn, you may sacrifice a Mountain rather than pay this spell's \
                 mana cost.",
            ),
            EffectDef::None,
        )
        // A Mountain, not a red source: what the cost names is the land type, so a
        // Sacred Foundry pays it and a Mountain that has stopped being one does not.
        // "If it's your turn" gates only the free cast. The printed cost is always
        // available, which is why this is a condition on the alternative rather
        // than a restriction on the card.
        .with_alternative_condition(&TriggerConditionDef::ActivePlayer(PlayerRelation::You)),
        AbilityDef::spell_with_targets(
            "Mine Collapse deals 5 damage to target creature or planeswalker.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(5),
            ),
        ),
    ]),
);

// MH2 138 — Ragavan, Nimble Pilferer
pub(in crate::card::sets) static RAGAVAN_NIMBLE_PILFERER: CardRecord = CardRecord::new(
    "Ragavan, Nimble Pilferer",
    "a9738cda-adb1-47fb-9f4c-ecd930228c4d",
    "Simon Dominic",
// One mana for a 2/1 that pays for itself the first time it connects,
    // and a dash cost for the turns when leaving it out would only get it
    // killed.
    CardRules::new_creature(mana_cost!("{R}"), &["Monkey", "Pirate"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever this creature deals combat damage to a player, create a Treasure token and \
                 exile the top card of that player's library. Until end of turn, you may cast that card.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                EffectDef::Sequence(&[
                    EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        TREASURE_TOKEN,
                    ))),
                    // "That player's library", and the permission is yours: what the Monkey
                    // steals is theirs to lose and yours to cast.
                    EffectDef::ExileTopOfLibraryToPlay {
                        player: EffectRecipientDef::EventPlayer,
                        amount: ValueDef::Constant(1),
                        free: false,
                        face_down: false,
                        duration: ExilePlayDurationDef::ThisTurn,
                        spend_any_color: false,
                        play_condition: None,
                        cast_only: true,
                    },
                ]),
            ),
            abilities::dash(
                &[CostDef::Mana(mana_cost!("{1}{R}"))],
                "Dash {1}{R} (You may cast this spell for its dash cost. If you do, it gains haste, and \
                it's returned from the battlefield to its owner's hand at the beginning of the next end \
                step.)",
            ),
            abilities::dashed_haste(),
            abilities::dashed_return(),
        ]),
);

// MH2 139 — Revolutionist
// Audit: unsupported — Madness needs a discard-to-exile replacement followed by a linked triggered cast-or-graveyard choice. No existing alternative-cast procedure implements that timing and zone-change sequence.
pub(in crate::card::sets) static REVOLUTIONIST_139: CardRecord = CardRecord::new(
    "Revolutionist",
    "bb8f3008-a3ba-4f73-afa6-ad81074b3196",
    "Scott Murphy",
    crate::card::CardRules::unsupported(),
);

// MH2 143 — Strike It Rich
pub(in crate::card::sets) static STRIKE_IT_RICH_143: CardRecord = CardRecord::new(
    "Strike It Rich",
    "1c7c2814-a617-4123-acdf-1b01b2768210",
    "Volkan Baǵa",
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell(
            "Create a Treasure token.",
            EffectDef::CreateToken(crate::card::CreateTokenDef::new(
                crate::card::TokenDef::Literal(tokens::treasure()),
            )),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{2}{R}"))]),
    ]),
);

// MH2 144 — Tavern Scoundrel
pub(in crate::card::sets) static TAVERN_SCOUNDREL_144: CardRecord = CardRecord::new(
    "Tavern Scoundrel",
    "55082c8a-d792-4cd8-94b1-d80c65804463",
    "Cynthia Sheppard",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Rogue"], 1, 3).with_abilities(&[
AbilityDef::triggered("Whenever you win a coin flip, create two Treasure tokens. (They're artifacts with \"{T}, Sacrifice this token: Add one mana of any color.\")", TriggerEventDef::CoinFlipWon(PlayerRelation::You), EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::tokens::treasure())).with_count(ValueDef::Constant(2)))),
AbilityDef::activated("{1}, {T}, Sacrifice another permanent: Flip a coin.", &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource, CostDef::sacrifice_permanent(ObjectPredicateDef::Not(&ObjectPredicateDef::Source))], EffectDef::FlipCoin { on_win: &EffectDef::None, on_loss: &EffectDef::None })
]),
);

// MH2 145 — Unholy Heat
pub(in crate::card::sets) static UNHOLY_HEAT: CardRecord = CardRecord::new(
    "Unholy Heat",
    "2b73d294-6ab1-4051-9b0f-d8e335d37674",
    "Kari Christensen",
CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Unholy Heat deals 2 damage to target creature or planeswalker.\nDelirium — Unholy Heat deals 6 damage instead if there are four or more card types among cards in your graveyard.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY), // Delirium changes the amount, not the effect, so it is a conditional value
            // rather than a second clause: four card types in your own graveyard, and
            // the same spell deals six.
            ValueDef::IfCardTypesAmongGraveyards(&GraveyardTypeConditionDef {
                player: PlayerRelation::You,
                minimum: 4,
                then: ValueDef::Constant(6),
                otherwise: ValueDef::Constant(2),
            }),
        ),
    )),
);

// MH2 147 — Abundant Harvest
// Audit: unsupported — Needs a reveal-until whose unmatched cards go somewhere other than the graveyard. MillUntilDef mills everything it passes over, where this puts the rest on the bottom of the library in a random order.
pub(in crate::card::sets) static ABUNDANT_HARVEST: CardRecord = CardRecord::new(
    "Abundant Harvest",
    "5ad86b17-3fed-418a-938c-c49adb409531",
    "Iris Compiet",
    crate::card::CardRules::unsupported(),
);

// MH2 148 — Aeve, Progenitor Ooze
pub(in crate::card::sets) static AEVE_PROGENITOR_OOZE: CardRecord = CardRecord::new(
    "Aeve, Progenitor Ooze",
    "dfe9b1b8-dffe-427d-be1e-2c6b8395bd54",
    "Andrew Mar",
CardRules::new_creature(mana_cost!("{2}{G}{G}{G}"), &["Ooze"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::storm().override_text(
                "Storm (When you cast this spell, copy it for each spell cast before it this turn. Copies become tokens.)",
            ),
            AbilityDef::static_ability(
                "Aeve isn't legendary if it's a token.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceMatches {
                        object: ObjectPredicateDef::Token,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::remove_supertype(CardSupertype::Legendary),
                    },
                },
            ),
            AbilityDef::as_enters(
                "Aeve enters with a +1/+1 counter on it for each other Ooze you control.",
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCountersValue {
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Ooze")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                    },
                ),
            ),
        ]),
);

// MH2 149 — Bannerhide Krushok
pub(in crate::card::sets) static BANNERHIDE_KRUSHOK: CardRecord = CardRecord::new(
    "Bannerhide Krushok",
    "1271251b-7d79-4cb4-80bb-98574aa63249",
    "Joe Slucher",
    // Three ways to spend the same card: a 4/4 trampler, two counters out of
    // the hand for two mana, or the whole four back out of the graveyard once
    // the game has gone long.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Beast"], 4, 4).with_abilities(&[
        abilities::trample(),
        AbilityDef::activated_with_targets(
            "Reinforce 2—{1}{G} ({1}{G}, Discard this card: Put two +1/+1 counters on target \
             creature.)",
            &[CostDef::Mana(mana_cost!("{1}{G}")), CostDef::DiscardSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(2),
            },
        )
        .with_source_zones(&[ZoneKind::Hand]),
        AbilityDef::activated_with_targets(
            "Scavenge {5}{G}{G} ({5}{G}{G}, Exile this card from your graveyard: Put a number of \
             +1/+1 counters equal to this card's power on target creature. Scavenge only as a \
             sorcery.)",
            &[CostDef::Mana(mana_cost!("{5}{G}{G}")), CostDef::ExileSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            // "This card's power" is the Krushok's own, read after it has
            // already been exiled to pay: the printed 4, since nothing on a
            // card in a graveyard was modifying it.
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::SourcePower,
            },
        )
        .with_source_zones(&[ZoneKind::Graveyard])
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// MH2 151 — Chatterfang, Squirrel General
// Audit: unsupported — Token replacement only multiplies the prospective token batch. It cannot append one independently declared Squirrel token for each token in the replaced batch.
pub(in crate::card::sets) static CHATTERFANG_SQUIRREL_GENERAL_151: CardRecord = CardRecord::new(
    "Chatterfang, Squirrel General",
    "1785cf85-1ac0-4246-9b89-1a8221a8e1b2",
    "Jason A. Engle",
    crate::card::CardRules::unsupported(),
);

// MH2 157 — Endurance
pub(in crate::card::sets) static ENDURANCE: CardRecord = CardRecord::new(
    "Endurance",
    "eb0e0404-4846-4891-acfa-bd0951ecf9c6",
    "Anastasia Ovchinnikova",
// A free answer to a graveyard that leaves a 3/4 blocker behind, or a
    // green card off the top of your hand when the graveyard is the whole
    // reason you are casting it.
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Elemental", "Incarnation"], 3, 4)
        .with_abilities(&crate::ability_list![
            [
                abilities::flash(),
                abilities::reach(),
                abilities::enters_trigger_with_targets(
                    "When this creature enters, up to one target player puts all the cards from their \
                     graveyard on the bottom of their library in a random order.",
                    // "Up to one target player" includes yourself, which is the mode nobody
                    // prints on the card: an Endurance can put your own graveyard back when
                    // something else is trying to eat it.
                    &[AbilityTargetDef::up_to(
                        AbilityTargetPredicate::Player(PlayerRelation::Any),
                        1,
                    )],
                    EffectDef::RandomizeObjectOrder(crate::card::RandomizeObjectOrderDef {
                        input: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Graveyard],
                            PlayerSetDef::LegalTargets(TargetIndex::PRIMARY),
                        )),
                        randomized: ParentBinding,
                        then: &EffectDef::MoveObjects(crate::card::MoveObjectsDef {
                            input: ObjectSetDef::Binding(ParentBinding),
                            from: Some(ZoneKind::Graveyard),
                            zone: ZoneKind::Library,
                            placement: ZonePlacement::Bottom,
                            moved: None,
                            then: &EffectDef::None,
                        }),
                    }),
                ),
            ],
            abilities::evoke(
                &[CostDef::exile(
                    ObjectPredicateDef::Color(ManaColor::Green),
                    ZoneKind::Hand,
                    CostQuantityDef::Fixed(1),
                )],
            ),
        ]),
);

// MH2 162 — Gaea's Will
pub(in crate::card::sets) static GAEA_S_WILL: CardRecord = CardRecord::new(
    "Gaea's Will",
    "488996b2-06c8-4866-bf0d-4664640c2be1",
    "Lucas Graciano",
    CardRules::base(
        CardTypeSet::single(CardType::Sorcery),
        PrintedManaCost::None,
    )
    .printed_colors(&[ManaColor::Green])
    .with_abilities(&[
        abilities::suspend(
            "Suspend 4—{G}",
            &SuspendAbilityDef::fixed(4, &[CostDef::Mana(mana_cost!("{G}"))]),
        ),
        AbilityDef::spell(
            "Until end of turn, you may play lands and cast spells from your graveyard.\nIf a card \
             would be put into your graveyard from anywhere this turn, exile that card instead.",
            // The permission belongs to the player. The replacement belongs to nothing
            // at all: the card making it is in the graveyard -- or in exile, by its own
            // clause -- before it applies, so it is created as an effect object that
            // lasts the turn rather than granted to a source that will not be there.
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromGraveyard(
                        // Everything, played every way: the permission names no card type and no
                        // one play action, which is the whole of "play lands and cast spells".
                        GraveyardPlayPermissionDef::unlimited(PlayRestrictionDef::new(
                            PlayActionMatcherDef::Any,
                            ObjectPredicateDef::Any,
                        )),
                    )),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::CreateOngoingEffect(OngoingEffectDef::unbound(
                    // "A card", not "a card or token": a token put into a graveyard goes there
                    // and ceases to exist as it always would.
                    &AbilityDef::replacement_for(
                        "If a card would be put into your graveyard from anywhere this turn, \
                         exile that card instead.",
                        ReplacementEventDef::AnyObjectWouldMove {
                            object: ObjectPredicateDef::All(&[
                                ObjectPredicateDef::OwnedBy(PlayerRelation::You),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                            ]),
                            to: ZoneKind::Graveyard,
                        },
                        ReplacementEffectDef::MoveToZone(ZoneKind::Exile),
                    ),
                    ResolvedEffectDurationDef::UntilEndOfTurn,
                )),
            ]),
        ),
    ]),
);

// MH2 181 — Urban Daggertooth
pub(in crate::card::sets) static URBAN_DAGGERTOOTH: CardRecord = CardRecord::new(
    "Urban Daggertooth",
    "4ab83a39-d90d-403e-b74d-fe99c8b2aacd",
    "Randy Vargas",
    // Vigilance is what makes the enrage reliable: it attacks and is still
    // back to be blocked, so the deck chooses when to be dealt damage.
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Dinosaur"], 4, 3).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::triggered(
            "Enrage — Whenever this creature is dealt damage, proliferate. (Choose any number of \
             permanents and/or players, then give each another counter of each kind already \
             there.)",
            // Any damage at all, not just combat damage, and once per damage
            // event rather than once per point.
            TriggerEventDef::damage_to_source(),
            EffectDef::Proliferate,
        ),
    ]),
);

// MH2 188 — Captured by Lagacs
pub(in crate::card::sets) static CAPTURED_BY_LAGACS: CardRecord = CardRecord::new(
    "Captured by Lagacs",
    "7ce1c2a8-688b-4f63-8d58-e325efc6052a",
    "Andrew Mar",
    // Three mana for a Pacifism is a poor rate on its own; the two counters
    // are what pay the difference, and they stay after the Aura is gone.
    CardRules::new_enchantment(mana_cost!("{1}{G}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            abilities::enchanted_creature_pacified(),
            abilities::enters_trigger_with_targets(
                "When this Aura enters, support 2. (Put a +1/+1 counter on each of up to two \
                 target creatures.)",
                // One slot holding up to two targets: "each of" is what makes
                // them one group. No "other" here -- the Aura is not a
                // creature, so nothing has to be excluded.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
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

// MH2 202 — Grist, the Hunger Tide
pub(in crate::card::sets) static GRIST_THE_HUNGER_TIDE: CardRecord = CardRecord::new(
    "Grist, the Hunger Tide",
    "69af2825-18c2-4463-b6ba-42eaa070ccc1",
    "Yongjae Choi",
// Three mana that makes a body every turn and answers one on the turn
    // it lands, which is why it is played over the planeswalkers that only
    // do one of those.
    CardRules::new_planeswalker(mana_cost!("{1}{B}{G}"), &["Grist"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "As long as Grist isn't on the battlefield, it's a 1/1 Insect creature in addition to its \
                 other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: // "A 1/1 Insect creature in addition to its other types": a creature card
                        // with an Insect subtype and a body, added to what the card already is
                        // rather than replacing it.
                        AppliedEffectDef::Composite(&[
                            AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(SetOperationDef::Add(
                                CardTypeSet::single(CardType::Creature),
                            ))),
                            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Subtypes(SetOperationDef::Add(
                                &["Insect"],
                            ))),
                            AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                                PowerToughnessOperationDef::SetBase {
                                    power: ValueDef::Constant(1),
                                    toughness: ValueDef::Constant(1),
                                },
                            )),
                        ]),
                },
            )
            // "As long as Grist isn't on the battlefield": every zone but that one,
            // which is a list of source zones rather than a condition to recheck.
            // The stack is one of them, so the spell on its way in is a creature
            // spell -- Essence Scatter counters it and Negate does not.
            .with_source_zones(&[
                ZoneKind::Library,
                ZoneKind::Hand,
                ZoneKind::Graveyard,
                ZoneKind::Stack,
                ZoneKind::Exile,
                ZoneKind::Command,
            ]),
            AbilityDef::activated(
                "+1: Create a 1/1 black and green Insect creature token, then mill a card. If an Insect \
                 card was milled this way, put a loyalty counter on Grist and repeat this process.",
                &[CostDef::Loyalty(1)],
                // The library is what bounds this in practice; the limit is only there so
                // a process with nothing to stop it still stops.
                EffectDef::MillWhileMatching(&MillLoopDef {
                    player: EffectRecipientDef::Controller,
                    body: &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::creature(&["Insect"], &[ManaColor::Black, ManaColor::Green], 1, 1),
                    ))),
                    // An Insect card in the library keeps the process going -- and a Grist on
                    // top is one, which is what his own first clause is for.
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Insect")),
                    on_match: &EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::Loyalty,
                        amount: ValueDef::Constant(1),
                    },
                    limit: 512,
                }),
            ),
            AbilityDef::activated(
                "\u{2212}2: You may sacrifice a creature.",
                &[CostDef::Loyalty(-2)],
                EffectDef::PayOr(PayOrDef::optional(
                    &[CostDef::sacrifice_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    &EffectDef::None,
                )),
            ),
            AbilityDef::triggered_with_targets(
                "When you do, destroy target creature or planeswalker.",
                TriggerEventDef::OptionalEffectTaken(ObjectPredicateDef::Source),
                &[AbilityTargetDef::exactly_one_permanent(
                    A_CREATURE_OR_PLANESWALKER,
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
            AbilityDef::activated(
                "\u{2212}5: Each opponent loses life equal to the number of creature cards in your \
                 graveyard.",
                &[CostDef::Loyalty(-5)],
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                },
            ),
        ]),
);

// MH2 215 — Terminal Agony
// Audit: unsupported — Madness needs a discard-to-exile replacement followed by a linked triggered cast-or-graveyard choice. No existing alternative-cast procedure implements that timing and zone-change sequence.
pub(in crate::card::sets) static TERMINAL_AGONY_215: CardRecord = CardRecord::new(
    "Terminal Agony",
    "314e94ad-0e12-48bb-aae1-2c842943114a",
    "Lucas Graciano",
    crate::card::CardRules::unsupported(),
);

// MH2 216 — Territorial Kavu
pub(in crate::card::sets) static TERRITORIAL_KAVU: CardRecord = CardRecord::new(
    "Territorial Kavu",
    "2605df98-0b02-4aab-bc36-01e93c693743",
    "E. M. Gist",
    // Two mana for as big a body as your mana base is greedy, and an attack
    // trigger that either loots or eats a graveyard.
    CardRules::new_creature(mana_cost!("{R}{G}"), &["Kavu"], 0, 0).with_abilities(&[
        AbilityDef::static_ability(
            "Domain — This creature's power and toughness are each equal to the number of basic \
             land types among lands you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                // Domain: how many of the five basic land types are among your lands. A
                // Kavu on a two-colour board is a 2/2, and one behind a full spread of
                // fetched duals is a 5/5.
                effect: AppliedEffectDef::define_power_toughness(
                    ValueDef::BasicLandTypesControlled(PlayerRelation::You),
                    ValueDef::BasicLandTypesControlled(PlayerRelation::You),
                ),
            },
        ),
        AbilityDef::modal_triggered(
            "Whenever this creature attacks, choose one —\n• Discard a card. If you do, draw a \
             card.\n• Exile up to one target card from a graveyard.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[
                AbilityDef::spell(
                    "Discard a card. If you do, draw a card.",
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: Some(DiscardFollowUpDef {
                            counted: ObjectPredicateDef::Any,
                            bound: None,
                            // "If you do": the draw is sized by what the discard actually took, so an
                            // empty hand discards nothing and draws nothing.
                            effect: &EffectDef::DrawCards {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::MatchedCount,
                            },
                        }),
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Exile up to one target card from a graveyard.",
                    &[AbilityTargetDef::up_to(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::Any,
                            zones: &[ZoneKind::Graveyard],
                            controller: None,
                            owner: None,
                        },
                        1,
                    )],
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                ),
            ],
        ),
    ]),
);

// MH2 227 — Kaldra Compleat
pub(in crate::card::sets) static KALDRA_COMPLEAT: CardRecord = CardRecord::new(
    "Kaldra Compleat",
    "87cc2855-6b14-44dd-a398-7dc2bbae081f",
    "Vincent Proce",
// Seven mana that arrives as a 5/5 first-striking, trampling,
    // indestructible, hasty creature which exiles whatever blocks it. The
    // Germ is the point: it never needs a creature to equip.
    CardRules::new_artifact(mana_cost!("{7}"))
        .with_supertype(CardSupertype::Legendary)
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::living_weapon(),
            abilities::indestructible(),
            AbilityDef::static_ability(
                "Equipped creature gets +5/+5 and has first strike, trample, indestructible, haste, and \
                 \"Whenever this creature deals combat damage to a creature, exile that creature.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(ValueDef::Constant(5), ValueDef::Constant(5)),
                        AppliedEffectDef::add_ability(&abilities::first_strike()),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                        AppliedEffectDef::add_ability(&abilities::indestructible()),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                        // The clause the equipped creature gains, not one Kaldra has itself: "that
                        // creature" is the one that took the damage, which is a different object
                        // from the one that dealt it.
                        AppliedEffectDef::add_ability(&AbilityDef::triggered(
                            "Whenever this creature deals combat damage to a creature, exile that creature.",
                            TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                                kind: DamageKindDef::Combat,
                                source: DamageSourceMatcherDef::Object(ObjectRefDef::Source),
                                recipient: DamageRecipientMatcherDef::MatchingObject(ObjectPredicateDef::HasType(
                                    CardType::Creature,
                                )),
                            }),
                            EffectDef::move_to_zone(
                                EffectRecipientDef::DamagedObject,
                                ZoneKind::Exile,
                                ZonePlacement::Top,
                            ),
                        )),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{7}"))], "Equip {7}"),
        ]),
);

// MH2 228 — Liquimetal Torque
pub(in crate::card::sets) static LIQUIMETAL_TORQUE_228: CardRecord = CardRecord::new(
    "Liquimetal Torque",
    "13c6101a-da40-4785-8ccb-4e779bbbdb55",
    "Brian Snõddy",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets("{T}: Target nonland permanent becomes an artifact in addition to its other types until end of turn.", &[CostDef::TapSource], &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)))], EffectDef::Apply { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), effect: AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Artifact)), duration: ResolvedEffectDurationDef::UntilEndOfTurn }),
    ]),
);

// MH2 231 — Nettlecyst
/// "Artifact and/or enchantment" is one query rather than two sums: a
/// permanent that is both is counted once, and Nettlecyst counts itself.
static ARTIFACTS_AND_ENCHANTMENTS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Artifact),
        ObjectPredicateDef::HasType(CardType::Enchantment),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static NETTLECYST: CardRecord = CardRecord::new(
    "Nettlecyst",
    "4a0bb5dc-75a6-4bd6-81f8-611197fb0fba",
    "Vincent Proce",
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::living_weapon(),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 for each artifact and/or enchantment you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountMatchingObjects(&ARTIFACTS_AND_ENCHANTMENTS_YOU_CONTROL),
                        ValueDef::CountMatchingObjects(&ARTIFACTS_AND_ENCHANTMENTS_YOU_CONTROL),
                    ),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// MH2 232 — Ornithopter of Paradise
pub(in crate::card::sets) static ORNITHOPTER_OF_PARADISE_232: CardRecord = CardRecord::new(
    "Ornithopter of Paradise",
    "025b0f0f-daf6-4071-82e7-39c015447ce4",
    "Raoul Vitale",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Thopter"], 0, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// MH2 247 — Goldmire Bridge
pub(in crate::card::sets) static GOLDMIRE_BRIDGE_247: CardRecord = CardRecord::new(
    "Goldmire Bridge",
    "dbe2a1fa-196f-497f-a15f-0b3b04da9cbb",
    "Aaron Miller",
    CardRules::new_land(&[])
        .with_type(CardType::Artifact)
        .with_abilities(&[
            abilities::enters_tapped(CardType::Land),
            abilities::indestructible(),
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

// MH2 249 — Mistvault Bridge
pub(in crate::card::sets) static MISTVAULT_BRIDGE_249: CardRecord = CardRecord::new(
    "Mistvault Bridge",
    "9f36a6e2-3e51-4a30-a225-10cfe6650b9d",
    "Mathias Kollros",
    CardRules::new_land(&[])
        .with_type(CardType::Artifact)
        .with_abilities(&[
            abilities::enters_tapped(CardType::Land),
            abilities::indestructible(),
            AbilityDef::activated_mana(
                "{T}: Add {U} or {B}.",
                &[CostDef::TapSource],
                EffectDef::AddMana(AddManaEffectDef::choice(&[
                    ManaColor::Blue,
                    ManaColor::Black,
                ])),
            ),
        ]),
);

// MH2 252 — Razortide Bridge
pub(in crate::card::sets) static RAZORTIDE_BRIDGE_252: CardRecord = CardRecord::new(
    "Razortide Bridge",
    "e7ea7395-430e-4036-92c9-17a850ec2371",
    "Rob Alexander",
    CardRules::new_land(&[])
        .with_type(CardType::Artifact)
        .with_abilities(&[
            abilities::enters_tapped(CardType::Land),
            abilities::indestructible(),
            AbilityDef::activated_mana(
                "{T}: Add {W} or {U}.",
                &[CostDef::TapSource],
                EffectDef::AddMana(AddManaEffectDef::choice(&[
                    ManaColor::White,
                    ManaColor::Blue,
                ])),
            ),
        ]),
);

// MH2 261 — Yavimaya, Cradle of Growth
pub(in crate::card::sets) static YAVIMAYA_CRADLE_OF_GROWTH: CardRecord = CardRecord::new(
    "Yavimaya, Cradle of Growth",
    "4e4b6e22-93b2-4896-bba5-0ceaa5d8ea3c",
    "Sarah Finnigan",
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::static_ability(
            "Each land is a Forest in addition to its other land types.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_basic_land_types(&[BasicLandType::Forest]),
            },
        )),
);

// MH2 328 — Esper Sentinel
pub(in crate::card::sets) static ESPER_SENTINEL_328: CardRecord = CardRecord::new(
    "Esper Sentinel",
    "676758ee-dac8-4c97-8a62-fff25bcbb6df",
    "Eric Deschamps",
    CardRules::new_artifact_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 1).with_abilities(&[
AbilityDef::triggered("Whenever an opponent casts their first noncreature spell each turn, draw a card unless that player pays {X}, where X is this creature's power.", TriggerEventDef::While { event: &TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[ObjectPredicateDef::NoncreatureSpell, ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent)])), condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::CountSpellsCastThisTurn(&SpellCastQueryDef { spell: ObjectPredicateDef::NoncreatureSpell, player: PlayerRelation::EventPlayer }), comparison: ComparisonDef::Equal, right: ValueDef::Constant(1) }) }, EffectDef::PayOr(PayOrDef::unless(&[CostDef::GenericMana(ValueDef::SourcePower)], &abilities::draw_cards(ValueDef::Constant(1))).with_payer(PlayerSetDef::One(PlayerRefDef::EventPlayer))))
]),
);

// MH2 333 — Serra's Emissary
// Audit: unsupported — Protection predicates cannot consume a stored chosen card type, so a static chosen-card-type protection grant to the player and creatures cannot be declared.
pub(in crate::card::sets) static SERRA_S_EMISSARY_333: CardRecord = CardRecord::new(
    "Serra's Emissary",
    "8de657fb-e68e-4400-9a12-60aaaa075fc4",
    "Nils Hamm",
    crate::card::CardRules::unsupported(),
);

// MH2 341 — Thought Monitor
pub(in crate::card::sets) static THOUGHT_MONITOR_341: CardRecord = CardRecord::new(
    "Thought Monitor",
    "55c98ef7-be05-4bcf-be4b-62a437297330",
    "Martina Pilcerova",
    CardRules::new_artifact_creature(mana_cost!("{6}{U}"), &["Construct"], 2, 2).with_abilities(&[
        AbilityDef::static_ability("Affinity for artifacts (This spell costs {1} less to cast for each artifact you control.)", EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Artifact), &[ZoneKind::Battlefield], PlayerRelation::You)))).with_source_zones(&[ZoneKind::Hand]),
        abilities::flying(),
        abilities::enters_trigger("When this creature enters, draw two cards.", EffectDef::DrawCards { recipient: EffectRecipientDef::Controller, amount: ValueDef::Constant(2) }),
    ]),
);

// MH2 343 — Kitchen Imp
// Audit: unsupported — Madness needs its discard-to-exile replacement and linked cast-or-graveyard procedure.
pub(in crate::card::sets) static KITCHEN_IMP_343: CardRecord = CardRecord::new(
    "Kitchen Imp",
    "20c7b777-c002-45c7-b2bb-d21dab591445",
    "Evyn Fong",
    crate::card::CardRules::unsupported(),
);

// MH2 355 — Ignoble Hierarch
pub(in crate::card::sets) static IGNOBLE_HIERARCH: CardRecord = CardRecord::new(
    "Ignoble Hierarch",
    "3139cce8-3467-4c50-add2-5b78fb33b90a",
    "Mark Zug",
    // Noble Hierarch in the other three colours: the same one-mana
    // accelerant, and the same 0/1 that exalted turns into a real
    // dividend on a turn when only one creature attacks.
    CardRules::new_creature(mana_cost!("{G}"), &["Goblin", "Shaman"], 0, 1).with_abilities(&[
        abilities::exalted(),
        AbilityDef::activated_mana(
            "{T}: Add {B}, {R}, or {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Red,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// MH2 358 — Timeless Witness
pub(in crate::card::sets) static TIMELESS_WITNESS_358: CardRecord = CardRecord::new(
    "Timeless Witness",
    "8a0f47b0-2254-4df2-b3dc-74c1d3811d2f",
    "Deruchenko Alexander",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Human", "Shaman"], 2, 1).with_abilities(&[
        abilities::enters_trigger_with_targets("When this creature enters, return target card from your graveyard to your hand.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::Any, zones: &[ZoneKind::Graveyard], controller: Some(PlayerRelation::You), owner: None })], EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Hand, ZonePlacement::Top)),
        abilities::eternalize!("Eternalize {5}{G}{G} ({5}{G}{G}, Exile this card from your graveyard: Create a token that's a copy of it, except it's a 4/4 black Zombie Human Shaman with no mana cost. Eternalize only as a sorcery.)", &[CostDef::Mana(mana_cost!("{5}{G}{G}"))]),
    ]),
);

// MH2 380 — Urza's Saga
static ARTIFACTS_YOU_CONTROL_SAGA: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Artifact),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static URZA_S_SAGA: CardRecord = CardRecord::new(
    "Urza's Saga",
    "2138dfbb-a4e3-49db-b908-95d0b2b7e82f",
    "Titus Lunter",
// A land that costs nothing, taps for one turn's mana, spends the next
    // two turns making Constructs, and fetches the artifact that makes them
    // bigger on its way out.
    // Two subtypes rather than one name: the land type "Urza's" that the
    // Urzatron cares about, and the enchantment type "Saga" that the lore
    // counters read.
    CardRules::new_land(&["Urza's", "Saga"])
        .with_type(CardType::Enchantment)
        .with_abilities(&[
            abilities::saga_chapter(
                1,
                "I — This Saga gains \"{T}: Add {C}.\"",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    // What chapter I hands the land. It is granted for good rather than for the
                    // turn: the Saga taps for mana from the moment the first chapter resolves
                    // until it sacrifices itself after the third.
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated_mana(
                        "{T}: Add {C}.",
                        &[CostDef::TapSource],
                        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
                    )),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            ),
            abilities::saga_chapter(
                2,
                "II — This Saga gains \"{2}, {T}: Create a 0/0 colorless Construct artifact creature \
                 token with 'This token gets +1/+1 for each artifact you control.'\"",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated(
                        "{2}, {T}: Create a 0/0 colorless Construct artifact creature token with \"This token gets \
                         +1/+1 for each artifact you control.\"",
                        &[
                            CostDef::Mana(mana_cost!("{2}")),
                            CostDef::TapSource,
                        ],
                        EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                            TokenCharacteristics::artifact_creature(&["Construct"], &[], 0, 0)
                                // The token's own clause, printed on the token rather than on the Saga:
                                // it counts itself, so the first one is a 1/1 on an otherwise empty board.
                                .with_abilities(&[AbilityDef::static_ability(
                                    "This token gets +1/+1 for each artifact you control.",
                                    EffectDef::StaticApply {
                                        recipient: EffectRecipientDef::Source,
                                        effect: AppliedEffectDef::modify_power_toughness(
                                            ValueDef::CountMatchingObjects(&ARTIFACTS_YOU_CONTROL_SAGA),
                                            ValueDef::CountMatchingObjects(&ARTIFACTS_YOU_CONTROL_SAGA),
                                        ),
                                    },
                                )]),
                        ))),
                    )),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            ),
            abilities::saga_chapter(
                3,
                "III — Search your library for an artifact card with mana cost {0} or {1}, put it onto \
                 the battlefield, then shuffle.",
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    // "Mana cost {0} or {1}" is the printed cost, not the mana
                    // value: Portable Hole costs {W} and Walking Ballista costs
                    // {X}{X}, and neither is findable even though both have a
                    // mana value of at most one.
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::GenericManaCostAtMost(1),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
        ]),
);

// MH2 401 — Profane Tutor
pub(in crate::card::sets) static PROFANE_TUTOR_401: CardRecord = CardRecord::new(
    "Profane Tutor",
    "0c7f7fdb-9c38-43f8-acb9-6ea1797387a6",
    "Richard Kane Ferguson",
    CardRules::base(
        CardTypeSet::single(CardType::Sorcery),
        crate::card::PrintedManaCost::None,
    )
    .with_abilities(&[
        abilities::suspend(
            "Suspend 2—{1}{B}",
            &crate::card::SuspendAbilityDef::fixed(2, &[CostDef::Mana(mana_cost!("{1}{B}"))]),
        ),
        AbilityDef::spell(
            "Search your library for a card, put that card into your hand, then shuffle.",
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::Any,
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: false,
                destination: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
    ]),
);

// MH2 411 — Chatterstorm
pub(in crate::card::sets) static CHATTERSTORM_411: CardRecord = CardRecord::new(
    "Chatterstorm",
    "4c1b91d8-39c4-4ab1-996b-2a5a78243fd4",
    "Milivoj Ćeran",
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[
        AbilityDef::spell(
            "Create a 1/1 green Squirrel creature token.",
            EffectDef::CreateToken(crate::card::CreateTokenDef::new(
                crate::card::TokenDef::Literal(crate::card::TokenCharacteristics::creature(
                    &["Squirrel"],
                    &[ManaColor::Green],
                    1,
                    1,
                )),
            )),
        ),
        abilities::storm(),
    ]),
);

// MH2 421 — Goblin Anarchomancer
pub(in crate::card::sets) static GOBLIN_ANARCHOMANCER: CardRecord = CardRecord::new(
    "Goblin Anarchomancer",
    "f7f07a80-05b5-4108-9e68-f8da05866acc",
    "Joe Slucher",
    // A two-mana body that pays for itself from the third spell on, in a
    // deck whose spells are nearly all one of its two colours.
    CardRules::new_creature(mana_cost!("{R}{G}"), &["Goblin", "Shaman"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "Each spell you cast that's red or green costs {1} less to cast.",
            // Read off the spell's colour rather than its cost, so a
            // colourless artifact you cast gets nothing even in this deck.
            EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Color(ManaColor::Red),
                    ObjectPredicateDef::Color(ManaColor::Green),
                ]),
                PlayerRelation::You,
                ValueDef::Constant(1),
            )),
        ),
    ),
);

// MH2 450 — Dauthi Voidwalker
pub(in crate::card::sets) static DAUTHI_VOIDWALKER: CardRecord = CardRecord::new(
    "Dauthi Voidwalker",
    "29632951-3c3d-478c-8c5a-9a34f30a5c28",
    "Sidharth Chaturvedi",
// Two mana for a body nothing ordinary can block, a graveyard nobody
    // else gets to use, and one card off the top of that pile.
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Dauthi", "Rogue"], 3, 2)
        .with_abilities(&[
            abilities::shadow(),
            AbilityDef::replacement_for(
                "If a card would be put into an opponent\'s graveyard from anywhere, instead exile it \
                 with a void counter on it.",
                // Their cards, not yours, and cards rather than tokens: a token that would
                // die still dies, and ceases to exist as it always would.
                ReplacementEventDef::AnyObjectWouldMove {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::OwnedBy(PlayerRelation::Opponent),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                    ]),
                    to: ZoneKind::Graveyard,
                },
                // The counter is the whole point: it marks the pile this creature is
                // allowed to reach back into, which is what separates it from the
                // graveyard hate that only takes things away.
                ReplacementEffectDef::Sequence(&[
                    ReplacementEffectDef::MoveToZone(ZoneKind::Exile),
                    ReplacementEffectDef::PlaceCountersOnMovedObject {
                        kind: CounterKind::named("void"),
                        amount: 1,
                    },
                ]),
            ),
            AbilityDef::activated(
                "{T}, Sacrifice this creature: Choose an exiled card an opponent owns with a void counter \
                 on it. You may play it this turn without paying its mana cost.",
                &[CostDef::TapSource, CostDef::SacrificeSource],
                // One card, chosen as the ability resolves and cast then or not at all.
                // What it costs is nothing at all, which is why the creature has to die to
                // ask.
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::HasCounter(CounterKind::named("void")),
                        &[ZoneKind::Exile],
                        PlayerRelation::Opponent,
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::MayPlayWithoutPaying(FreePlayDef {
                        objects: ObjectSetDef::Binding(ParentBinding),
                        duration: FreePlayDurationDef::WhileResolving,
                        mandatory: false,
                        grants_haste: false,
                    }),
                }),
            ),
        ]),
);

// MH2 451 — Necrogoyf
// Audit: unsupported — Madness needs a discard-to-exile replacement followed by a linked triggered cast-or-graveyard choice. No existing alternative-cast procedure implements that timing and zone-change sequence.
pub(in crate::card::sets) static NECROGOYF_451: CardRecord = CardRecord::new(
    "Necrogoyf",
    "11a2d158-d69e-4854-b84f-9f271e36101c",
    "Nicholas Gregory",
    crate::card::CardRules::unsupported(),
);

// MH2 462 — Sanctum Weaver
pub(in crate::card::sets) static SANCTUM_WEAVER_462: CardRecord = CardRecord::new(
    "Sanctum Weaver",
    "15fd218c-3e14-4b82-9e03-16a0fad1b530",
    "Kimonas Theodossiou",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Dryad"], 0, 2)
        .with_ability(AbilityDef::activated_mana(
            "{T}: Add X mana of any one color, where X is the number of enchantments you control.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color().with_variable_amount(
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            )),
        ))
        .with_type(crate::card::CardType::Enchantment),
);

// MH2 470 — Diamond Lion
pub(in crate::card::sets) static DIAMOND_LION_470: CardRecord = CardRecord::new(
    "Diamond Lion",
    "2116f1a3-f621-4bef-b5bd-fa4a644f76b1",
    "Howard Lyon",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Cat"], 2, 2).with_ability(
        AbilityDef::activated_mana(
            "{T}, Discard your hand, Sacrifice this creature: Add three mana of any one color.",
            &[
                CostDef::TapSource,
                CostDef::DiscardHand,
                CostDef::SacrificeSource,
            ],
            EffectDef::AddMana(AddManaEffectDef::any_color().with_amount(3)),
        ),
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &PRISMATIC_ENDING,
    &SANCTIFIER_EN_VEC_27,
    &SOLITUDE,
    &UNBOUNDED_POTENTIAL,
    &DRESS_DOWN_39,
    &HARD_EVIDENCE,
    &LOSE_FOCUS,
    &MURKTIDE_REGENT,
    &SCUTTLETIDE_61,
    &STEP_THROUGH_66,
    &SUBTLETY,
    &SUSPEND_68,
    &ARCHON_OF_CRUELTY,
    &BONE_SHARDS,
    &DAMN,
    &GRIEF,
    &HELL_MONGREL_88,
    &LOATHSOME_CURATOR,
    &NESTED_SHAMBLER,
    &PERSIST_96,
    &TOURACH_DREAD_CANTOR_102,
    &UNMARKED_GRAVE_106,
    &VERMIN_GORGER,
    &VILE_ENTOMBER,
    &BLAZING_ROOTWALLA_115,
    &DRAGON_S_RAGE_CHANNELER,
    &FURY,
    &GALVANIC_RELAY_127,
    &HARMONIC_PRODIGY_132,
    &MINE_COLLAPSE,
    &RAGAVAN_NIMBLE_PILFERER,
    &REVOLUTIONIST_139,
    &STRIKE_IT_RICH_143,
    &TAVERN_SCOUNDREL_144,
    &UNHOLY_HEAT,
    &ABUNDANT_HARVEST,
    &AEVE_PROGENITOR_OOZE,
    &BANNERHIDE_KRUSHOK,
    &CHATTERFANG_SQUIRREL_GENERAL_151,
    &ENDURANCE,
    &GAEA_S_WILL,
    &URBAN_DAGGERTOOTH,
    &CAPTURED_BY_LAGACS,
    &GRIST_THE_HUNGER_TIDE,
    &TERMINAL_AGONY_215,
    &TERRITORIAL_KAVU,
    &KALDRA_COMPLEAT,
    &LIQUIMETAL_TORQUE_228,
    &NETTLECYST,
    &ORNITHOPTER_OF_PARADISE_232,
    &GOLDMIRE_BRIDGE_247,
    &MISTVAULT_BRIDGE_249,
    &RAZORTIDE_BRIDGE_252,
    &YAVIMAYA_CRADLE_OF_GROWTH,
    &ESPER_SENTINEL_328,
    &SERRA_S_EMISSARY_333,
    &THOUGHT_MONITOR_341,
    &KITCHEN_IMP_343,
    &IGNOBLE_HIERARCH,
    &TIMELESS_WITNESS_358,
    &URZA_S_SAGA,
    &PROFANE_TUTOR_401,
    &CHATTERSTORM_411,
    &GOBLIN_ANARCHOMANCER,
    &DAUTHI_VOIDWALKER,
    &NECROGOYF_451,
    &SANCTUM_WEAVER_462,
    &DIAMOND_LION_470,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
