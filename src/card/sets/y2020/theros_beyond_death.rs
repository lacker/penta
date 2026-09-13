//! Theros Beyond Death cards cataloged for the Vintage Cube.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityPredicateDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BasicLandType;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BindObjectsDef;
use crate::card::CardChoiceSourceDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::CharacteristicOperationDef;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ComparisonDef;
use crate::card::ConditionDef;
use crate::card::CostDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::CreatureTypeSetDef;
use crate::card::DrawEventMatcherDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::MoveObjectsDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PayOrDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::RandomizeObjectOrderDef;
use crate::card::ReplacementConditionDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SetOperationDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::ids::Binding;
use crate::ids::ParentBinding;
use crate::ids::TargetIndex;
use crate::mana_cost;

/// The ordinary Escape shape: a resolved mana cost, this many other graveyard
/// cards exiled as an additional cost. The selected alternative cast kind
/// itself is the lasting Escape fact; exceptional costs remain card-local.
pub(in crate::card::sets) const fn escape(costs: &'static [CostDef]) -> AbilityDef {
    AbilityDef::alternative_cast(costs, AlternativeCastKindDef::Escape, None, EffectDef::None)
}

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "THB",
    slug: "theros-beyond-death",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// THB 18 — Heliod, Sun-Crowned
pub(in crate::card::sets) static HELIOD_SUN_CROWNED_18: CardRecord = CardRecord::new(
    "Heliod, Sun-Crowned",
    "01a8576e-cadc-4521-aadd-3a05f0bc4d20",
    "Lius Lasahido",
    CardRules::new_enchantment_creature(mana_cost!("{2}{W}"), &["God"], 5, 5).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::indestructible(),
AbilityDef::static_ability("As long as your devotion to white is less than five, Heliod isn't a creature.", EffectDef::IfCondition { condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::DevotionTo(ManaColor::White), comparison: ComparisonDef::Less, right: ValueDef::Constant(5) }), then: &EffectDef::StaticApply { recipient: EffectRecipientDef::Source, effect: AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(SetOperationDef::Remove(CardTypeSet::single(CardType::Creature)))) } }),
AbilityDef::triggered_with_targets("Whenever you gain life, put a +1/+1 counter on target creature or enchantment you control.", TriggerEventDef::LifeGained(PlayerRelation::You), &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::HasType(CardType::Enchantment)]), zones: &[ZoneKind::Battlefield], controller: Some(PlayerRelation::You), owner: None })], EffectDef::AddCounters { object: EffectRecipientDef::Target(TargetIndex::PRIMARY), kind: CounterKind::PlusOnePlusOne, amount: ValueDef::Constant(1) }),
AbilityDef::activated_with_targets("{1}{W}: Another target creature gains lifelink until end of turn.", &[CostDef::Mana(mana_cost!("{1}{W}"))], &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]))], EffectDef::Apply { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), effect: AppliedEffectDef::add_ability(&abilities::lifelink()), duration: ResolvedEffectDurationDef::UntilEndOfTurn })
]),
);

// THB 20 — Heliod's Pilgrim (reprint)
const HELIOD_S_PILGRIM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2014::magic_2015::HELIOD_S_PILGRIM,
    "cafce2f5-f4f4-465b-96dc-bcdd29d4e4bb",
    "Micah Epstein",
);

// THB 55 — Nadir Kraken
pub(in crate::card::sets) static NADIR_KRAKEN_55: CardRecord = CardRecord::new(
    "Nadir Kraken",
    "7817e039-e509-4b6f-b5a3-deb3769bbdc8",
    "Dan Murayama Scott",
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Kraken"], 2, 3).with_abilities(&[
AbilityDef::triggered("Whenever you draw a card, you may pay {1}. If you do, put a +1/+1 counter on this creature and create a 1/1 blue Tentacle creature token.", TriggerEventDef::DrewCard(DrawEventMatcherDef::any(PlayerRelation::You)), EffectDef::PayOr(PayOrDef::optional(&[CostDef::Mana(mana_cost!("{1}"))], &EffectDef::Sequence(&[EffectDef::AddCounters { object: EffectRecipientDef::Source, kind: CounterKind::PlusOnePlusOne, amount: ValueDef::Constant(1) }, EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::TokenCharacteristics::creature(&["Tentacle"], &[ManaColor::Blue], 1, 1))))]))))
]),
);

// THB 73 — Thassa's Oracle
const ORACLE_TOP: Binding = Binding!("oracle_top");
const ORACLE_REST: Binding = Binding!("oracle_rest");
pub(in crate::card::sets) static THASSAS_ORACLE: CardRecord = CardRecord::new(
    "Thassa's Oracle",
    "726e8b29-13e9-4138-b6a9-d2a0d8188d1c",
    "Jesper Ejsing",
// Two blue mana and an empty library is the whole card. The looking is
    // what it does when the library is not empty yet.
    CardRules::new_creature(mana_cost!("{U}{U}"), &["Merfolk", "Wizard"], 1, 3).with_ability(
        abilities::enters_trigger(
            "When this creature enters, look at the top X cards of your library, where X is your devotion to blue. Put up to one of them on top of your library and the rest on the bottom of your library in a random order. If X is greater than or equal to the number of cards in your library, you win the game.",
            abilities::bind_top_cards_then(
                PlayerRefDef::EffectController,
                ValueDef::DevotionTo(ManaColor::Blue),
                &EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(ORACLE_TOP),
                    unchosen: Some(ORACLE_REST),
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Binding(ParentBinding),
                    exclude: None,
                    minimum: 0,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Private,
                    then: &EffectDef::Sequence(&[
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
                                    then: &EffectDef::Sequence(&[
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
                                        ]),
                                })
                        ]),
                }),
            ),
        ),
    ),
);

// THB 87 — Cling to Dust
pub(in crate::card::sets) static CLING_TO_DUST_87: CardRecord = CardRecord::new(
    "Cling to Dust",
    "52c2de5f-e486-4cfe-9fb6-be0078ce5f93",
    "Caio Monteiro",
    CardRules::new_instant(mana_cost!("{B}")).with_abilities(&[
AbilityDef::spell_with_targets("Exile target card from a graveyard. If it was a creature card, you gain 3 life. Otherwise, you draw a card.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::Any, zones: &[ZoneKind::Graveyard], controller: None, owner: None })], EffectDef::BindObjects(BindObjectsDef { source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::LegalTargets(TargetIndex::PRIMARY)), binding: Binding!("dust_target"), then: &EffectDef::ForEachInBinding { objects: Binding!("dust_target"), binding: Binding!("dust_card"), effect: &EffectDef::IfElseCondition { condition: &TriggerConditionDef::BoundObjectMatches { binding: Binding!("dust_card"), object: ObjectPredicateDef::HasType(CardType::Creature) }, then: &EffectDef::Sequence(&[EffectDef::move_to_zone(EffectRecipientDef::object(ObjectRefDef::Binding(Binding!("dust_card"))), ZoneKind::Exile, ZonePlacement::Top), EffectDef::GainLife { recipient: EffectRecipientDef::Controller, amount: ValueDef::Constant(3) }]), otherwise: &EffectDef::Sequence(&[EffectDef::move_to_zone(EffectRecipientDef::object(ObjectRefDef::Binding(Binding!("dust_card"))), ZoneKind::Exile, ZonePlacement::Top), abilities::draw_cards(ValueDef::Constant(1))]) } } })),
escape(&[CostDef::Mana(mana_cost!("{3}{B}")), CostDef::exile(ObjectPredicateDef::Any, ZoneKind::Graveyard, CostQuantityDef::Fixed(5))])
]),
);

// THB 99 — Gray Merchant of Asphodel (reprint)
const GRAY_MERCHANT_OF_ASPHODEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2013::theros::GRAY_MERCHANT_OF_ASPHODEL,
    "7c1a7dd8-8034-4f59-a351-33666b26ff5a",
    "Scott Murphy",
);

// THB 105 — Mire Triton
pub(in crate::card::sets) static MIRE_TRITON: CardRecord = CardRecord::new(
    "Mire Triton",
    "3f8427d3-4d9e-48c9-838b-239fd1357d95",
    "Seb McKinnon",
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
    "Underworld Charger",
    "f2dd847f-0db2-4f6a-bdfb-5c88ce7802f9",
    "Johann Bodin",
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
        escape(&[
            CostDef::Mana(mana_cost!("{4}{B}")),
            CostDef::exile(
                ObjectPredicateDef::Any,
                ZoneKind::Graveyard,
                CostQuantityDef::Fixed(3),
            ),
        ]),
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
    "Blood Aspirant",
    "8d4f3fa3-ba1f-48dc-a56b-738936f1bf86",
    "Tyler Walpole",
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
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(1),
                ),
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
pub(in crate::card::sets) static UNDERWORLD_BREACH: CardRecord = CardRecord::new(
    "Underworld Breach",
    "0e51d796-7279-4c06-87f0-37adbdaa41df",
    "Lie Setiawan",
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
                    ability: &escape(&[CostDef::ManaCostOf(crate::ObjectRefDef::Source), CostDef::exile(ObjectPredicateDef::Any, ZoneKind::Graveyard, CostQuantityDef::Fixed(3))]),
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
            EffectDef::sacrifice(EffectRecipientDef::Source),
        ),
    ]),
);

// THB 163 — Underworld Rage-Hound
pub(in crate::card::sets) static UNDERWORLD_RAGE_HOUND: CardRecord = CardRecord::new(
    "Underworld Rage-Hound",
    "a04eef82-fd53-41f4-9c7e-28b9ac039032",
    "Tyler Walpole",
    // It has to attack, so escaping it back is a commitment rather than a
    // free extra body -- and the counter is what makes the second one hit
    // harder than the first.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Elemental", "Dog"], 3, 1).with_abilities(&[
        abilities::attacks_each_combat_if_able(),
        escape(&[
            CostDef::Mana(mana_cost!("{3}{R}")),
            CostDef::exile(
                ObjectPredicateDef::Any,
                ZoneKind::Graveyard,
                CostQuantityDef::Fixed(3),
            ),
        ]),
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

// THB 168 — Destiny Spinner
pub(in crate::card::sets) static DESTINY_SPINNER_168: CardRecord = CardRecord::new(
    "Destiny Spinner",
    "ba264166-948b-47d4-b302-64476acc1a55",
    "Livia Prima",
    CardRules::new_enchantment_creature(mana_cost!("{1}{G}"), &["Human"], 2, 3).with_abilities(&[
AbilityDef::static_ability("Creature and enchantment spells you control can't be countered.", EffectDef::StaticApply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::HasType(CardType::Enchantment)]), &[ZoneKind::Stack], PlayerRelation::You), effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered) }),
AbilityDef::activated_with_targets("{3}{G}: Target land you control becomes an X/X Elemental creature with trample and haste until end of turn, where X is the number of enchantments you control. It's still a land.", &[CostDef::Mana(mana_cost!("{3}{G}"))], &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::HasType(CardType::Land), zones: &[ZoneKind::Battlefield], controller: Some(PlayerRelation::You), owner: None })], EffectDef::Apply { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), effect: AppliedEffectDef::Composite(&[AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(SetOperationDef::Add(CardTypeSet::single(CardType::Creature)))), AppliedEffectDef::set_base_power_toughness(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Enchantment), &[ZoneKind::Battlefield], PlayerRelation::You)), ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Enchantment), &[ZoneKind::Battlefield], PlayerRelation::You))), AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Elemental"])), AppliedEffectDef::add_ability(&abilities::trample()), AppliedEffectDef::add_ability(&abilities::haste())]), duration: ResolvedEffectDurationDef::UntilEndOfTurn })
]),
);

// THB 173 — Hyrax Tower Scout
pub(in crate::card::sets) static HYRAX_TOWER_SCOUT_173: CardRecord = CardRecord::new(
    "Hyrax Tower Scout",
    "bb7f2638-d757-4df6-90b0-b616534dd3a0",
    "Micah Epstein",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Scout"], 3, 3).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, untap target creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// THB 174 — Ilysian Caryatid
pub(in crate::card::sets) static ILYSIAN_CARYATID_174: CardRecord = CardRecord::new(
    "Ilysian Caryatid",
    "7cdf8ab8-f221-4f7b-9af9-3849cad1f596",
    "Winona Nelson",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Plant"], 1, 1).with_abilities(&[
AbilityDef::activated_mana("{T}: Add one mana of any color. If you control a creature with power 4 or greater, add two mana of any one color instead.", &[CostDef::TapSource], EffectDef::AddMana(AddManaEffectDef::any_color().with_amount_override(&ConditionDef::Exists(ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::PowerAtLeast(4)]), &[ZoneKind::Battlefield], PlayerRelation::You)), 2)))
]),
);

// THB 182 — Nessian Hornbeetle
pub(in crate::card::sets) static NESSIAN_HORNBEETLE: CardRecord = CardRecord::new(
    "Nessian Hornbeetle",
    "8200fcda-e30c-460f-9964-47e657b7c758",
    "Jason Felix",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Insect"], 2, 2).with_abilities(&[
        AbilityDef::triggered_if(
            "At the beginning of combat on your turn, if you control \
             another creature with power 4 or greater, put a +1/+1 counter \
             on this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ObjectPredicateDef::PowerAtLeast(4),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// THB 190 — Nyxbloom Ancient
// Audit: unsupported — Needs a mana-production replacement that preserves every produced unit's type and triples its amount.
pub(in crate::card::sets) static NYXBLOOM_ANCIENT_190: CardRecord = CardRecord::new(
    "Nyxbloom Ancient",
    "a391da36-0b40-46ea-b771-50d2b920207e",
    "Filip Burburan",
    crate::card::CardRules::unsupported(),
);

// THB 229 — Uro, Titan of Nature's Wrath
pub(in crate::card::sets) static URO_TITAN_OF_NATURE_S_WRATH: CardRecord = CardRecord::new(
    "Uro, Titan of Nature's Wrath",
    "a0b6a71e-56cb-4d25-8f2b-7a4f1b60900d",
    "Vincent Proce",
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
                EffectDef::sacrifice(EffectRecipientDef::Source),
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
            escape(&[CostDef::Mana(mana_cost!("{G}{G}{U}{U}")), CostDef::exile(ObjectPredicateDef::Any, ZoneKind::Graveyard, CostQuantityDef::Fixed(5))]),
        ]),
);

// THB 236 — Shadowspear
pub(in crate::card::sets) static SHADOWSPEAR_236: CardRecord = CardRecord::new(
    "Shadowspear",
    "939c6e19-4b27-4023-bb9c-ae440f91e21c",
    "Yeong-Hao Han",
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_supertype(CardSupertype::Legendary)
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 and has trample and lifelink.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                        AppliedEffectDef::add_ability(&abilities::lifelink()),
                    ]),
                },
            ),
            AbilityDef::activated(
                "{1}: Permanents your opponents control lose hexproof and indestructible \
                 until end of turn.",
                &[CostDef::Mana(mana_cost!("{1}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    ),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                            KeywordAbility::Hexproof,
                        )),
                        AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                            KeywordAbility::Indestructible,
                        )),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// THB 237 — Soul-Guide Lantern
pub(in crate::card::sets) static SOUL_GUIDE_LANTERN: CardRecord = CardRecord::new(
    "Soul-Guide Lantern",
    "7c850b94-75c9-4457-8b5e-1193352d6fcb",
    "Cliff Childs",
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
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
        ),
        // Untargeted, so it does not care whether those graveyards hold
        // anything: unlike Tormod's Crypt this one can be cashed in against an
        // empty board purely to stop what has not happened yet.
        AbilityDef::activated(
            "{T}, Sacrifice this artifact: Exile each opponent's graveyard.",
            // The two sacrifice abilities differ only in what they buy, so the shared
            // half of the cost is written once.
            &[CostDef::TapSource, CostDef::SacrificeSource],
            EffectDef::move_to_zone(
                EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Graveyard],
                    PlayerRelation::Opponent,
                ),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
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

// THB 295 — Terror of Mount Velus
pub(in crate::card::sets) static TERROR_OF_MOUNT_VELUS: CardRecord = CardRecord::new(
    "Terror of Mount Velus",
    "332dc6c3-7802-4bde-aa4e-0feab70c216f",
    "Billy Christian",
    CardRules::new_creature(mana_cost!("{5}{R}{R}"), &["Dragon"], 5, 5).with_abilities(&[
        abilities::flying(),
        abilities::double_strike(),
        abilities::enters_trigger(
            "When this creature enters, creatures you control gain double \
             strike until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// THB 325 — Arasta of the Endless Web
pub(in crate::card::sets) static ARASTA_OF_THE_ENDLESS_WEB_325: CardRecord = CardRecord::new(
    "Arasta of the Endless Web",
    "03b9304c-9993-4539-9165-48568eb81db1",
    "Sam Rowan",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Spider"], 3, 5).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::reach(),
AbilityDef::triggered("Whenever an opponent casts an instant or sorcery spell, create a 1/2 green Spider creature token with reach.", TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Instant), ObjectPredicateDef::HasType(CardType::Sorcery)]), ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent)])), EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::TokenCharacteristics::creature(&["Spider"], &[ManaColor::Green], 1, 2).with_abilities(&[abilities::reach()])))))
])
.with_type(crate::card::CardType::Enchantment),
);

// THB 326 — Dryad of the Ilysian Grove
pub(in crate::card::sets) static DRYAD_OF_THE_ILYSIAN_GROVE_326: CardRecord = CardRecord::new(
    "Dryad of the Ilysian Grove",
    "36adefc7-44a8-40d0-8bdf-ad12d010b0bd",
    "Scott Murphy",
    CardRules::new_enchantment_creature(mana_cost!("{2}{G}"), &["Nymph", "Dryad"], 2, 4)
        .with_abilities(&[
            AbilityDef::static_ability(
                "You may play an additional land on each of your turns.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayAdditionalLands(1)),
                },
            ),
            AbilityDef::static_ability(
                "Lands you control are every basic land type in addition to their other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::Characteristic(
                        CharacteristicOperationDef::BasicLandTypes(SetOperationDef::Add(
                            &BasicLandType::ALL,
                        )),
                    ),
                },
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &HELIOD_SUN_CROWNED_18,
    &NADIR_KRAKEN_55,
    &THASSAS_ORACLE,
    &CLING_TO_DUST_87,
    &MIRE_TRITON,
    &UNDERWORLD_CHARGER,
    &BLOOD_ASPIRANT,
    &UNDERWORLD_BREACH,
    &UNDERWORLD_RAGE_HOUND,
    &DESTINY_SPINNER_168,
    &HYRAX_TOWER_SCOUT_173,
    &ILYSIAN_CARYATID_174,
    &NESSIAN_HORNBEETLE,
    &NYXBLOOM_ANCIENT_190,
    &URO_TITAN_OF_NATURE_S_WRATH,
    &SHADOWSPEAR_236,
    &SOUL_GUIDE_LANTERN,
    &TERROR_OF_MOUNT_VELUS,
    &ARASTA_OF_THE_ENDLESS_WEB_325,
    &DRYAD_OF_THE_ILYSIAN_GROVE_326,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[HELIOD_S_PILGRIM_REPRINT, GRAY_MERCHANT_OF_ASPHODEL_REPRINT];
