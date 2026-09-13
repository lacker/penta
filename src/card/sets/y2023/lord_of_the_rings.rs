//! The Lord of the Rings: Tales of Middle-earth cards cataloged for the
//! Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::ParentBinding;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AggregateOperationDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BlockRestrictionDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CastTimingPermissionDef;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ComparisonDef;
use crate::card::ConditionDef;
use crate::card::CostDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DrawEventMatcherDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ExilePlayDurationDef;
use crate::card::ManaColor;
use crate::card::ManaRestrictionDef;
use crate::card::ManaSpendEffectDef;
use crate::card::MoveToZoneCostDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectValueAggregateDef;
use crate::card::ObjectValueDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SpellCastQueryDef;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnPhaseDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::tokens;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "LTR",
    slug: "lord-of-the-rings",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const FOOD_TOKEN: TokenCharacteristics = tokens::food().with_art(CardArt::new(
    "4a029bdc-92e3-4d85-8af5-e33429a5f017",
    "L J Koh",
));

// LTR 0 — The One Ring (alternate printing)
const THE_ONE_RING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_ONE_RING,
    1,
    "93de9042-cc62-4ade-8d8d-68fdbc84bfae",
    "Veli Nyström",
);

// LTR 7 — Eagles of the North
pub(in crate::card::sets) static EAGLES_OF_THE_NORTH: CardRecord = CardRecord::new(
    "Eagles of the North",
    "c1bd3bc0-77bd-40fe-b4f1-835a04cb6e41",
    "Axel Sauerwald",
// Six mana is not what the card is for: one mana for the Plains is,
    // and the six is what the last copy in the deck is worth on a board
    // that is already wide.
    CardRules::new_creature(mana_cost!("{5}{W}"), &["Bird", "Soldier"], 3, 3)
        .with_abilities(&[
            abilities::flying(),
            abilities::enters_trigger(
                "When this creature enters, creatures you control get +1/+0 and gain first strike until \
                 end of turn.",
                EffectDef::Apply {
                    // Every creature you control as the trigger resolves, the Eagles included:
                    // they are on the battlefield by the time their own arrival is read.
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(0)),
                        AppliedEffectDef::add_ability(&abilities::first_strike()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            abilities::typecycling!(
                "Plainscycling {1} ({1}, Discard this card: Search your library for a Plains card, \
                reveal it, put it into your hand, then shuffle.)",
                &[CostDef::Mana(mana_cost!("{1}"))],
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Plains")),
            ),
        ]),
);

// LTR 15 — Flowering of the White Tree
pub(in crate::card::sets) static FLOWERING_OF_THE_WHITE_TREE_15: CardRecord = CardRecord::new(
    "Flowering of the White Tree",
    "2203b2cd-48e5-471a-85fe-dc81012e5d61",
    "Erikas Perl",
    CardRules::new_enchantment(mana_cost!("{W}{W}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Legendary creatures you control get +2/+1 and have ward {1}.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::ward(
                            &[CostDef::Mana(mana_cost!("{1}"))],
                            "Ward {1}",
                        )),
                    ]),
                },
            ),
            AbilityDef::static_ability(
                "Nonlegendary creatures you control get +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                                CardSupertype::Legendary,
                            )),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
        ]),
);

// LTR 26 — Reprieve
pub(in crate::card::sets) static REPRIEVE: CardRecord = CardRecord::new(
    "Reprieve",
    "1bd3fa8a-6c50-4f7f-9ae3-0810eec5e3db",
    "Justyna Dura",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Return target spell to its owner's hand.\nDraw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        // Returning the spell is not countering it, so a spell that cannot be
        // countered is answered all the same -- and its controller keeps the card,
        // which is the price. Drawing pays for the tempo either way.
        EffectDef::Sequence(&[
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// LTR 28 — Samwise the Stouthearted
// Audit: unsupported — The Ring tempting procedure, persistent Ring-bearer designation, and advancing Ring abilities have no runtime representation.
pub(in crate::card::sets) static SAMWISE_THE_STOUTHEARTED_28: CardRecord = CardRecord::new(
    "Samwise the Stouthearted",
    "214c270e-29ca-4d69-bea6-9252ae7707ad",
    "Irvin Rodriguez",
    crate::card::CardRules::unsupported(),
);

// LTR 56 — Ioreth of the Healing House
pub(in crate::card::sets) static IORETH_OF_THE_HEALING_HOUSE_56: CardRecord = CardRecord::new(
    "Ioreth of the Healing House",
    "03ab74cd-978a-49eb-9d38-bc8b472b3cef",
    "Wei Guan",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Cleric"], 1, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "{T}: Untap another target permanent.",
                &[CostDef::TapSource],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                )],
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::activated_with_targets(
                "{T}: Untap two other target legendary creatures.",
                &[CostDef::TapSource],
                &[AbilityTargetDef::exactly_value(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    ValueDef::Constant(2),
                )],
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
        ]),
);

// LTR 60 — Lórien Revealed
pub(in crate::card::sets) static LORIEN_REVEALED: CardRecord = CardRecord::new(
    "Lórien Revealed",
    "0ce44270-a684-4489-9077-521456e6dfaa",
    "Randy Gallegos",
// Five mana is not what this card is for either. One mana from hand for
    // an Island is, and the three cards are what makes the last copy in the
    // deck worth drawing.
    CardRules::new_sorcery(mana_cost!("{3}{U}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Draw three cards.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
        abilities::typecycling!(
            "Islandcycling {1} ({1}, Discard this card: Search your library for an Island card, reveal it, put it into your hand, then shuffle.)",
            &[CostDef::Mana(mana_cost!("{1}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Island")),
        ),
    ]),
);

// LTR 71 — Stern Scolding
pub(in crate::card::sets) static STERN_SCOLDING: CardRecord = CardRecord::new(
    "Stern Scolding",
    "3ca1e1de-b916-445f-b3b2-0f4d0cc7ceeb",
    "Valera Lutfullina",
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::counter_target(
        "Counter target creature spell with power or toughness 2 or less.",
        // "Power or toughness 2 or less" is a disjunction, not a pair of bounds: a
        // 5/1 is small enough and a 1/5 is too. Written as "less than 3" because
        // that is the comparison the predicate offers.
        &AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::Spell,
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                        ObjectPredicateDef::ToughnessLessThan(ValueDef::Constant(3)),
                    ]),
                ]),
            ]),
            zones: &[ZoneKind::Stack],
            controller: None,
            owner: None,
        }),
    )),
);

// LTR 103 — Orcish Bowmasters
static AN_ARMY_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::controlled_by(
    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
    &[ZoneKind::Battlefield],
    PlayerSetDef::Related(PlayerRelation::You),
);

pub(in crate::card::sets) static ORCISH_BOWMASTERS: CardRecord = CardRecord::new(
    "Orcish Bowmasters",
    "7c024bae-5631-4e20-ac69-df392ac9e109",
    "Maxim Kostin",
// Flash makes the entry itself an ambush, and every extra card an
    // opponent draws afterwards is another arrow and another counter.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Orc", "Archer"], 1, 1)
        .with_abilities(&[
            abilities::flash(),
            AbilityDef::triggered_with_targets(
                "When this creature enters and whenever an opponent draws a card except the first one they \
                 draw in each of their draw steps, this creature deals 1 damage to any target. Then amass \
                 Orcs 1.",
                // The enters clause and the draws clause are one printed ability with two
                // ways to fire, not two abilities, so the damage and the amass are written
                // once and both events reach them.
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::DrewCard(DrawEventMatcherDef::except_first_in_draw_step(
                        PlayerRelation::Opponent,
                    )),
                ]),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                EffectDef::Sequence(&[
                    EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(1),
                    ),
                    // The token is made first so that the choice below always has something
                    // to find; with an Army already out, nothing new arrives.
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ObjectCount {
                            query: AN_ARMY_YOU_CONTROL,
                            comparison: ComparisonDef::Equal,
                            amount: 0,
                        },
                        then: &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                            TokenCharacteristics::creature(&["Orc", "Army"], &[ManaColor::Black], 0, 0).with_art(
                                CardArt::new("6943f966-fd21-427c-a13f-44727edcaa4b", "Veli Nyström"),
                            ),
                        ))),
                    },
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Query(AN_ARMY_YOU_CONTROL),
                        exclude: None,
                        minimum: 1,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        // Amass chooses among the Armies you control, so the counter and the type
                        // both land on the same one -- and the type is added rather than set, which
                        // is what keeps an Army that was already something else both things.
                        then: &EffectDef::Sequence(&[
                            EffectDef::AddCounters {
                                object: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                                kind: CounterKind::PlusOnePlusOne,
                                amount: ValueDef::Constant(1),
                            },
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                                effect: AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Orc"])),
                                duration: ResolvedEffectDurationDef::Permanent,
                            },
                        ]),
                    }),
                ]),
            ),
        ]),
);

// LTR 111 — Troll of Khazad-dûm
pub(in crate::card::sets) static TROLL_OF_KHAZAD_DUM: CardRecord = CardRecord::new(
    "Troll of Khazad-dûm",
    "a6539e26-b63b-4725-9407-caaf451de084",
    "Simon Dominic",
// Six mana for a body nobody blocks, or one mana for the Swamp the deck
    // was missing. It is in the cube for the second half.
    CardRules::new_creature(mana_cost!("{5}{B}"), &["Troll"], 6, 5)
        .with_abilities(&[
            // Menace with a bigger number, which is why it is written out rather
            // than printed as the keyword.
            AbilityDef::static_ability(
                "This creature can't be blocked except by three or more creatures.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                        BlockRestrictionDef::MinimumBlockers(3),
                    )),
                },
            ),
            abilities::typecycling!(
                "Swampcycling {1} ({1}, Discard this card: Search your library for a Swamp card, reveal \
                it, put it into your hand, then shuffle.)",
                &[CostDef::Mana(mana_cost!("{1}"))],
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Swamp")),
            ),
        ]),
);

// LTR 120 — Éomer, Marshal of Rohan
pub(in crate::card::sets) static EOMER_MARSHAL_OF_ROHAN_120: CardRecord = CardRecord::new(
    "Éomer, Marshal of Rohan",
    "0bd31ce9-9551-4efe-8bd2-b97d8efbf75e",
    "Jesper Ejsing",
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Human", "Knight"], 4, 4).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::haste(),
AbilityDef::triggered("Whenever one or more other attacking legendary creatures you control die, untap all creatures you control. After this phase, there is an additional combat phase. This ability triggers only once each turn.", TriggerEventDef::ObjectsDied { object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Supertype(CardSupertype::Legendary), ObjectPredicateDef::Attacking, ObjectPredicateDef::Not(&ObjectPredicateDef::Source), ObjectPredicateDef::ControlledBy(PlayerRelation::You)]) }, EffectDef::Sequence(&[EffectDef::Untap { object: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You) }, EffectDef::ScheduleTurnPhases(&[TurnPhaseDef::Combat])])).triggering_at_most(1)
]),
);

// LTR 137 — Improvised Club
pub(in crate::card::sets) static IMPROVISED_CLUB: CardRecord = CardRecord::new(
    "Improvised Club",
    "b8397d13-eeaf-4b4e-b3cd-9a9ac231873a",
    "Pablo Mendoza",
    // Four damage for two mana, paid for with something the board was going
    // to lose anyway -- a Food or a spent token makes this nearly free.
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice an artifact or creature. \
             Improvised Club deals 4 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            // Either type pays it, so an artifact deck and a creature deck
            // both cast this without giving up a body they wanted.
            CostDef::sacrifice(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(4),
            ),
        ),
    ),
);

// LTR 139 — Oliphaunt
pub(in crate::card::sets) static OLIPHAUNT: CardRecord = CardRecord::new(
    "Oliphaunt",
    "6989018c-37b1-4282-a4af-9cc97f160b4d",
    "John Di Giovanni",
// Six mana is not what the card is for either. Mountaincycling is: one
    // mana from hand, and the Oliphaunt becomes the land the draw did not
    // give you.
    CardRules::new_creature(mana_cost!("{5}{R}"), &["Elephant"], 6, 4).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, another target creature you control gets +2/+0 and \
             gains trample until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            // "Another": the Oliphaunt cannot lend itself the bonus, which is why the
            // trigger does nothing when it attacks alone.
            &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                // What the charge lends: the same trample the Oliphaunt already has, and
                // two more power to push it through with.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(0)),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::typecycling!(
            "Mountaincycling {1} ({1}, Discard this card: Search your library for a Mountain card, reveal it, put it into your hand, then shuffle.)",
            &[CostDef::Mana(mana_cost!("{1}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mountain")),
        ),
    ]),
);

// LTR 142 — Rally at the Hornburg
pub(in crate::card::sets) static RALLY_AT_THE_HORNBURG: CardRecord = CardRecord::new(
    "Rally at the Hornburg",
    "ee7292f7-1c7e-449c-9c52-7584d6a14c2c",
    "Ekaterina Burmak",
    // The tokens it makes are Humans, so the haste half applies to them:
    // two mana at sorcery speed for two immediate attackers.
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell(
        "Create two 1/1 white Human Soldier creature tokens. Humans you control gain haste until \
         end of turn.",
        EffectDef::Sequence(&[
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                    &["Human", "Soldier"],
                    &[ManaColor::White],
                    1,
                    1,
                )))
                .with_amount(2),
            ),
            // Read after the tokens arrive, which is what lets them attack
            // the turn this resolves.
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Human")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// LTR 158 — Delighted Halfling
pub(in crate::card::sets) static DELIGHTED_HALFLING: CardRecord = CardRecord::new(
    "Delighted Halfling",
    "71384418-173a-4f77-adab-56e52fa23692",
    "Inka Schulz",
CardRules::new_creature(mana_cost!("{G}"), &["Halfling", "Citizen"], 1, 2).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color. Spend this mana only to cast a legendary spell, and that spell can't be countered.",
            &[CostDef::TapSource],
            EffectDef::AddMana(
                AddManaEffectDef::any_color()
                    .with_restrictions(&[ManaRestrictionDef::CastSpell(
                        ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                    )])
                    // The rider is the reason the card is played: uncounterable is not a
                    // property of the Halfling but of whatever its mana paid for.
                    .with_spend_effects(&[ManaSpendEffectDef::ApplyToPaidSpell(
                            AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered),
                        )]),
            ),
        ),
    ]),
);

// LTR 169 — Generous Ent
pub(in crate::card::sets) static GENEROUS_ENT: CardRecord = CardRecord::new(
    "Generous Ent",
    "85d22d5d-3875-42ff-b51e-c6e21db201f5",
    "Simon Dominic",
CardRules::new_creature(mana_cost!("{5}{G}"), &["Treefolk"], 5, 7).with_abilities(&[
        abilities::reach(),
        abilities::enters_trigger("When this creature enters, create a Food token.", EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
            FOOD_TOKEN,
        )))),
        // Six mana is not what this card is for. Forestcycling is: one mana
        // from hand, and the Ent becomes the land the draw did not give you.
        abilities::typecycling!(
            "Forestcycling {1} ({1}, Discard this card: Search your library for a Forest card, reveal it, put it into your hand, then shuffle.)",
            &[CostDef::Mana(mana_cost!("{1}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Forest")),
        ),
    ]),
);

// LTR 193 — Arwen, Mortal Queen
pub(in crate::card::sets) static ARWEN_MORTAL_QUEEN: CardRecord = CardRecord::new(
    "Arwen, Mortal Queen",
    "547f92d4-cd1d-4ca7-a6e2-6473b4d3c832",
    "Miranda Meeks",
// Three mana for a 2/2 that is hard to kill until the turn she decides
    // to spend that on somebody else, and leaves both of them bigger for
    // good when she does.
    CardRules::new_creature(mana_cost!("{1}{G}{W}"), &["Elf", "Noble"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::as_enters(
                "Arwen enters with an indestructible counter on her.",
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCounters {
                        kind: CounterKind::Indestructible,
                        amount: 1,
                    },
                ),
            ),
            AbilityDef::activated_with_targets(
                "{1}, Remove an indestructible counter from Arwen: Another target creature gains \
                 indestructible until end of turn. Put a +1/+1 counter and a lifelink counter on that \
                 creature and a +1/+1 counter and a lifelink counter on Arwen.",
                &[
                    CostDef::Mana(mana_cost!("{1}")),
                    CostDef::RemoveCountersFromSource {
                        kind: CounterKind::Indestructible,
                        amount: 1,
                    },
                ],
                // "Another target creature": Arwen is not among the choices, which is what
                // keeps her from handing herself the counters twice.
                &[
                        AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))
                            .excluding_source(),
                    ],
                // The counter she spends buys the other creature a turn of
                // indestructibility outright, and both of them keep the pair of counters
                // afterwards -- so the ability is a trade of her own safety for two
                // permanently bigger creatures.
                EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::Lifelink,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::Lifelink,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
        ]),
);

// LTR 203 — Flame of Anor
pub(in crate::card::sets) static FLAME_OF_ANOR: CardRecord = CardRecord::new(
    "Flame of Anor",
    "04779a7e-b453-48b9-b392-6d6fd0b8d283",
    "Yigit Koroglu",
CardRules::new_instant(mana_cost!("{1}{U}{R}")).with_ability(
        AbilityDef::modal_spell(
            "Choose one. If you control a Wizard as you cast this spell, you may choose two instead.",
            &[
                AbilityDef::spell_with_targets(
                    "Target player draws two cards.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Player(PlayerRelation::Any),
                    )],
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(2),
                    },
                ),
                AbilityDef::destroy_target(
                    "Destroy target artifact.",
                    &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Artifact))
),
                AbilityDef::spell_with_targets(
                    "This spell deals 5 damage to target creature.",
                    &[AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )],
                    EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(5),
                    ),
                ),
            ],
        )
        // The condition is read as the spell is cast, not as it resolves, so a
        // Wizard that dies in response has already done its work.
        .with_conditional_mode_maximum(ConditionDef::Exists(ObjectQueryDef::controlled_by(
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Wizard")),
            &[ZoneKind::Battlefield],
            PlayerSetDef::Related(PlayerRelation::You),
        )), 2),
    ),
);

// LTR 225 — Sauron's Ransom
// Audit: unsupported — Needs the Ring-bearer designation and progressive Ring temptation rules in
// addition to the opponent-created public and hidden piles.
pub(in crate::card::sets) static SAURON_S_RANSOM_225: CardRecord = CardRecord::new(
    "Sauron's Ransom",
    "6b98850c-ad69-42da-b91a-8dc5e226c444",
    "Alex Brock",
    CardRules::unsupported(),
);

// LTR 245 — Mithril Coat
pub(in crate::card::sets) static MITHRIL_COAT_245: CardRecord = CardRecord::new(
    "Mithril Coat",
    "0fd1fc09-a09d-45e6-8a07-3a8a83b4e6ec",
    "Igor Krstic",
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flash(),
            abilities::indestructible(),
            abilities::enters_trigger_with_targets(
                "When Mithril Coat enters, attach it to target legendary creature you control.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Equipped creature has indestructible.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{3}"))], "Equip {3}"),
        ]),
);

// LTR 246 — The One Ring
// A turn of complete safety, then a card every turn for a life total
// that runs out faster than it looks like it will.
pub(in crate::card::sets) static THE_ONE_RING: CardRecord = CardRecord::new(
    "The One Ring",
    "d5806e68-1054-458e-866d-1f2470f682b2",
    "Veli Nyström",
    CardRules::new_artifact(mana_cost!("{4}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::indestructible(),
            // An intervening-if rather than part of the effect: a One Ring reanimated
            // or put onto the battlefield never puts the trigger on the stack at all.
            AbilityDef::triggered_if(
                "When this artifact enters, if you cast it, you gain protection from everything until \
                 your next turn.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                &TriggerConditionDef::SourceWasCast,
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Controller,
                    // Protection checks source qualities rather than controller, so this also
                    // shuts off the controller's own targeted spells for the turn it lasts.
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerProtectionFrom(
                        ObjectPredicateDef::Any,
                    )),
                    duration: ResolvedEffectDurationDef::UntilYourNextTurn,
                },
            ),
            AbilityDef::triggered(
                "At the beginning of your upkeep, you lose 1 life for each burden counter on this \
                 artifact.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::CountersOnSource(CounterKind::named("burden")),
                },
            ),
            AbilityDef::activated(
                "{T}: Put a burden counter on this artifact, then draw a card for each burden counter \
                 on it.",
                &[CostDef::TapSource],
                // The counter goes on first, then the draw counts every burden counter.
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::named("burden"),
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::CountersOnSource(CounterKind::named("burden")),
                    },
                ]),
            ),
        ]),
);

// LTR 250 — Sting, the Glinting Dagger
// Audit: unsupported — The equipment needs a live conditional first-strike ability evaluated in the equipped creature's source scope. Granted executable static abilities are rejected, and the existing blocker predicates are relative to the Equipment rather than its host.
pub(in crate::card::sets) static STING_THE_GLINTING_DAGGER_250: CardRecord = CardRecord::new(
    "Sting, the Glinting Dagger",
    "afbec7e7-f5b9-407e-bf96-2e088710e791",
    "Nino Is",
    crate::card::CardRules::unsupported(),
);

// LTR 254 — Great Hall of the Citadel
pub(in crate::card::sets) static GREAT_HALL_OF_THE_CITADEL_254: CardRecord = CardRecord::new(
    "Great Hall of the Citadel",
    "219c7b57-b62b-42d1-85d9-4b57624a3f54",
    "Campbell White",
    CardRules::new_land(&[]).with_abilities(&[
abilities::tap_for(ManaColor::Colorless),
AbilityDef::activated_mana("{1}, {T}: Add two mana in any combination of colors. Spend this mana only to cast legendary spells.", &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource], EffectDef::AddMana(AddManaEffectDef::combination(&[ManaColor::White, ManaColor::Blue, ManaColor::Black, ManaColor::Red, ManaColor::Green], 2).with_restrictions(&[ManaRestrictionDef::CastSpell(ObjectPredicateDef::Supertype(CardSupertype::Legendary))])))
]),
);

// LTR 256 — Minas Tirith
// Audit: unsupported — Activation conditions cannot query the number of creatures a player
// attacked with earlier this turn after those creatures leave combat or the battlefield.
pub(in crate::card::sets) static MINAS_TIRITH_256: CardRecord = CardRecord::new(
    "Minas Tirith",
    "b38b6760-616f-4b11-8ce7-ac1223c7fd53",
    "Arthur Yuan",
    CardRules::unsupported(),
);

// LTR 257 — Mines of Moria
pub(in crate::card::sets) static MINES_OF_MORIA_257: CardRecord = CardRecord::new(
    "Mines of Moria",
    "0be723d6-4ada-4c3f-b87b-8ab83a4bbb8f",
    "Arthur Yuan",
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_tapped_unless_you_control(
                "Mines of Moria enters tapped unless you control a legendary creature.",
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                ]),
            ),
            abilities::tap_for(ManaColor::Red),
            AbilityDef::activated(
                "{3}{R}, {T}, Exile three cards from your graveyard: Create two Treasure tokens.",
                &[
                    CostDef::Mana(mana_cost!("{3}{R}")),
                    CostDef::TapSource,
                    CostDef::MoveToZone(MoveToZoneCostDef::new(
                        ObjectPredicateDef::Any,
                        ZoneKind::Graveyard,
                        ZoneKind::Exile,
                        3,
                    )),
                ],
                EffectDef::CreateToken(
                    crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(
                        crate::card::tokens::treasure(),
                    ))
                    .with_count(ValueDef::Constant(2)),
                ),
            ),
        ]),
);

// LTR 258 — Mount Doom
pub(in crate::card::sets) static MOUNT_DOOM_258: CardRecord = CardRecord::new(
    "Mount Doom",
    "b5bc71a1-2344-4bc6-aa60-658cec19d0d6",
    "Jonas De Ro",
    CardRules::new_land(&[]).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::activated_mana("{T}, Pay 1 life: Add {B} or {R}.", &[CostDef::TapSource, CostDef::PayLife(1)], EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Black, ManaColor::Red]))),
AbilityDef::activated("{1}{B}{R}, {T}: Mount Doom deals 1 damage to each opponent.", &[CostDef::Mana(mana_cost!("{1}{B}{R}")), CostDef::TapSource], EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1))),
AbilityDef::activated("{5}{B}{R}, {T}, Sacrifice Mount Doom and a legendary artifact: Choose up to two creatures, then destroy the rest. Activate only as a sorcery.", &[CostDef::Mana(mana_cost!("{5}{B}{R}")), CostDef::TapSource, CostDef::SacrificeSource, CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::Supertype(CardSupertype::Legendary)]))], EffectDef::Choose(ChooseDef { chooser: PlayerRefDef::EffectController, candidates: ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::Any)), exclude: None, minimum: 0, maximum: 2, binding: ObjectChoiceBindingDef::Objects(Binding!("doom_saved")), unchosen: Some(Binding!("doom_destroyed")), visibility: ChoiceVisibilityDef::Private, then: &EffectDef::Destroy { object: EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("doom_destroyed"))), then: None } })).with_activation_timing(ActivationTimingDef::SorcerySpeed)
]),
);

// LTR 305 — Gandalf the White
// Audit: unsupported — AdditionalTriggerDef can double entry-caused triggers but has no leaving-battlefield half, so it cannot double the required death and other departure triggers.
pub(in crate::card::sets) static GANDALF_THE_WHITE_305: CardRecord = CardRecord::new(
    "Gandalf the White",
    "2c9dc67a-5c26-4044-82b6-d5b6e195ae64",
    "Dominik Mayer",
    crate::card::CardRules::unsupported(),
);

// LTR 344 — Rivendell
pub(in crate::card::sets) static RIVENDELL_344: CardRecord = CardRecord::new(
    "Rivendell",
    "650fa2f4-2916-427c-a0f9-37e2dbe8e1fc",
    "Josu Solano",
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_tapped_unless_you_control(
                "Rivendell enters tapped unless you control a legendary creature.",
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                ]),
            ),
            abilities::tap_for(ManaColor::Blue),
            AbilityDef::activated(
                "{1}{U}, {T}: Scry 2. Activate only if you control a legendary creature.",
                &[CostDef::Mana(mana_cost!("{1}{U}")), CostDef::TapSource],
                abilities::scry(ValueDef::Constant(2)),
            )
            .with_activation_condition(&TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            }),
        ]),
);

// LTR 350 — Borne Upon a Wind
pub(in crate::card::sets) static BORNE_UPON_A_WIND_350: CardRecord = CardRecord::new(
    "Borne Upon a Wind",
    "60ebd6a8-2e93-40ee-951a-4fcf12c85e3d",
    "Alexander Mokhov",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell(
        "You may cast spells this turn as though they had flash.\nDraw a card.",
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayCastAsThoughItHadFlash(
                    CastTimingPermissionDef::new(ObjectPredicateDef::Any),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            abilities::draw_cards(ValueDef::Constant(1)),
        ]),
    )]),
);

// LTR 360 — Glóin, Dwarf Emissary
// With one opponent, goad imposes only the attack requirement until your next turn.
pub(in crate::card::sets) static GLOIN_DWARF_EMISSARY_360: CardRecord = CardRecord::new(
    "Glóin, Dwarf Emissary",
    "6d74d1af-5cc6-422e-949c-de9e39b76154",
    "Tomas Duchek",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Dwarf", "Advisor"], 3, 3).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::triggered("Whenever you cast a historic spell, create a Treasure token. This ability triggers only once each turn. (Artifacts, legendaries, and Sagas are historic.)", TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::Supertype(CardSupertype::Legendary), ObjectPredicateDef::Subtype(SubtypeDef::Literal("Saga"))]), ObjectPredicateDef::ControlledBy(PlayerRelation::You)])), EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::tokens::treasure())))).triggering_at_most(1),
AbilityDef::activated_with_targets("{T}, Sacrifice a Treasure: Goad target creature. (Until your next turn, that creature attacks each combat if able and attacks a player other than you if able.)", &[CostDef::TapSource, CostDef::sacrifice_permanent(ObjectPredicateDef::Subtype(SubtypeDef::Literal("Treasure")))], &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))], EffectDef::Apply { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), effect: AppliedEffectDef::add_ability(&abilities::attacks_each_combat_if_able()), duration: ResolvedEffectDurationDef::UntilYourNextTurn })
]),
);

// LTR 362 — Moria Marauder
pub(in crate::card::sets) static MORIA_MARAUDER_362: CardRecord = CardRecord::new(
    "Moria Marauder",
    "b9e36249-02f5-4c11-9a2b-6be81eb6b490",
    "Andrea Piparo",
    CardRules::new_creature(mana_cost!("{R}{R}"), &["Goblin", "Warrior"], 1, 1).with_abilities(&[
abilities::double_strike(),
AbilityDef::triggered("Whenever a Goblin or Orc you control deals combat damage to a player, exile the top card of your library. You may play that card this turn.", TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::All(&[ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")), ObjectPredicateDef::Subtype(SubtypeDef::Literal("Orc"))]), ObjectPredicateDef::ControlledBy(PlayerRelation::You)])), EffectDef::ExileTopOfLibraryToPlay { player: EffectRecipientDef::Controller, amount: ValueDef::Constant(1), free: false, face_down: false, duration: ExilePlayDurationDef::ThisTurn, spend_any_color: false, play_condition: None, cast_only: false })
]),
);

// LTR 370 — Lotho, Corrupt Shirriff
pub(in crate::card::sets) static LOTHO_CORRUPT_SHIRRIFF_370: CardRecord = CardRecord::new(
    "Lotho, Corrupt Shirriff",
    "69d97af0-8af0-4124-b56f-2633d34e5574",
    "Ilker Yildiz",
    CardRules::new_creature(mana_cost!("{W}{B}"), &["Halfling", "Rogue"], 2, 1).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::triggered("Whenever a player casts their second spell each turn, you lose 1 life and create a Treasure token. (It's an artifact with \"{T}, Sacrifice this token: Add one mana of any color.\")", TriggerEventDef::While { event: &TriggerEventDef::spell_cast(ObjectPredicateDef::Any), condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::CountSpellsCastThisTurn(&SpellCastQueryDef { spell: ObjectPredicateDef::Any, player: PlayerRelation::EventPlayer }), comparison: ComparisonDef::Equal, right: ValueDef::Constant(2) }) }, EffectDef::Sequence(&[EffectDef::LoseLife { recipient: EffectRecipientDef::Controller, amount: ValueDef::Constant(1) }, EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::tokens::treasure())))]))
]),
);

// LTR 407 — Boromir, Warden of the Tower
// Audit: unsupported — The Ring tempting procedure, persistent Ring-bearer designation, and advancing Ring abilities have no runtime representation.
pub(in crate::card::sets) static BOROMIR_WARDEN_OF_THE_TOWER_407: CardRecord = CardRecord::new(
    "Boromir, Warden of the Tower",
    "97ec04f9-0563-4490-b252-714df2ddbf58",
    "Colin Boyer",
    crate::card::CardRules::unsupported(),
);

// LTR 418 — Last March of the Ents
pub(in crate::card::sets) static LAST_MARCH_OF_THE_ENTS_418: CardRecord = CardRecord::new(
    "Last March of the Ents",
    "66763118-6a1e-465a-bfe0-6fe18c419875",
    "David Rapoza",
    CardRules::new_sorcery(mana_cost!("{6}{G}{G}")).with_abilities(&[
abilities::cannot_be_countered(),
AbilityDef::spell("Draw cards equal to the greatest toughness among creatures you control, then put any number of creature cards from your hand onto the battlefield.", EffectDef::Sequence(&[abilities::draw_cards(ValueDef::AggregateObjectValues(&ObjectValueAggregateDef { objects: ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You)), select: ObjectValueDef::Toughness, operation: AggregateOperationDef::Maximum })), EffectDef::Choose(ChooseDef { chooser: PlayerRefDef::EffectController, candidates: ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Hand], PlayerRelation::You)), exclude: None, minimum: 0, maximum: 255, binding: ObjectChoiceBindingDef::Objects(Binding!("ents_creatures")), unchosen: None, visibility: ChoiceVisibilityDef::Private, then: &EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("ents_creatures"))), ZoneKind::Battlefield, ZonePlacement::Top) })]))
]),
);

// LTR 437 — Merry, Esquire of Rohan
pub(in crate::card::sets) static MERRY_ESQUIRE_OF_ROHAN_437: CardRecord = CardRecord::new(
    "Merry, Esquire of Rohan",
    "259ff889-fc9a-42f7-998d-0ab23c94ad8a",
    "Tyler Jacobson",
    CardRules::new_creature(mana_cost!("{R}{W}"), &["Halfling", "Knight"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::haste(),
            AbilityDef::static_ability(
                "Merry has first strike as long as it's equipped.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Equipment")),
                                ObjectPredicateDef::AttachedTo(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 1,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                    },
                },
            ),
            AbilityDef::triggered(
                "Whenever you attack with Merry and another legendary creature, draw a card.",
                TriggerEventDef::While {
                    event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                    condition: &TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                ObjectPredicateDef::Attacking,
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 1,
                    },
                },
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
        ]),
);

// LTR 443 — The Grey Havens
// Audit: unsupported — Mana-selection domains cannot take the union of colors of queried legendary creature cards in a graveyard. CouldBeProducedBy inspects mana abilities, not card colors.
pub(in crate::card::sets) static THE_GREY_HAVENS_443: CardRecord = CardRecord::new(
    "The Grey Havens",
    "9714aa30-1db2-4670-9a0b-72acfc3f703c",
    "Kieran Yanner",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &EAGLES_OF_THE_NORTH,
    &FLOWERING_OF_THE_WHITE_TREE_15,
    &REPRIEVE,
    &SAMWISE_THE_STOUTHEARTED_28,
    &IORETH_OF_THE_HEALING_HOUSE_56,
    &LORIEN_REVEALED,
    &STERN_SCOLDING,
    &ORCISH_BOWMASTERS,
    &TROLL_OF_KHAZAD_DUM,
    &EOMER_MARSHAL_OF_ROHAN_120,
    &IMPROVISED_CLUB,
    &OLIPHAUNT,
    &RALLY_AT_THE_HORNBURG,
    &DELIGHTED_HALFLING,
    &GENEROUS_ENT,
    &ARWEN_MORTAL_QUEEN,
    &FLAME_OF_ANOR,
    &SAURON_S_RANSOM_225,
    &MITHRIL_COAT_245,
    &THE_ONE_RING,
    &STING_THE_GLINTING_DAGGER_250,
    &GREAT_HALL_OF_THE_CITADEL_254,
    &MINAS_TIRITH_256,
    &MINES_OF_MORIA_257,
    &MOUNT_DOOM_258,
    &GANDALF_THE_WHITE_305,
    &RIVENDELL_344,
    &BORNE_UPON_A_WIND_350,
    &GLOIN_DWARF_EMISSARY_360,
    &MORIA_MARAUDER_362,
    &LOTHO_CORRUPT_SHIRRIFF_370,
    &BOROMIR_WARDEN_OF_THE_TOWER_407,
    &LAST_MARCH_OF_THE_ENTS_418,
    &MERRY_ESQUIRE_OF_ROHAN_437,
    &THE_GREY_HAVENS_443,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[THE_ONE_RING_ALTERNATE_1];
