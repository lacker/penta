//! The Lord of the Rings: Tales of Middle-earth cards cataloged for the
//! Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::ParentBinding;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BlockRestrictionDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
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
use crate::card::ManaColor;
use crate::card::ManaRestrictionDef;
use crate::card::ManaSpendEffectDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAMWISE_THE_STOUTHEARTED_28: CardRecord = CardRecord::new(
    "Samwise the Stouthearted",
    "214c270e-29ca-4d69-bea6-9252ae7707ad",
    "Irvin Rodriguez",
    crate::card::CardRules::unsupported(),
);

// LTR 56 — Ioreth of the Healing House
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IORETH_OF_THE_HEALING_HOUSE_56: CardRecord = CardRecord::new(
    "Ioreth of the Healing House",
    "03ab74cd-978a-49eb-9d38-bc8b472b3cef",
    "Wei Guan",
    crate::card::CardRules::unsupported(),
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EOMER_MARSHAL_OF_ROHAN_120: CardRecord = CardRecord::new(
    "Éomer, Marshal of Rohan",
    "0bd31ce9-9551-4efe-8bd2-b97d8efbf75e",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
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

// LTR 245 — Mithril Coat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MITHRIL_COAT_245: CardRecord = CardRecord::new(
    "Mithril Coat",
    "0fd1fc09-a09d-45e6-8a07-3a8a83b4e6ec",
    "Igor Krstic",
    crate::card::CardRules::unsupported(),
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STING_THE_GLINTING_DAGGER_250: CardRecord = CardRecord::new(
    "Sting, the Glinting Dagger",
    "afbec7e7-f5b9-407e-bf96-2e088710e791",
    "Nino Is",
    crate::card::CardRules::unsupported(),
);

// LTR 254 — Great Hall of the Citadel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GREAT_HALL_OF_THE_CITADEL_254: CardRecord = CardRecord::new(
    "Great Hall of the Citadel",
    "219c7b57-b62b-42d1-85d9-4b57624a3f54",
    "Campbell White",
    crate::card::CardRules::unsupported(),
);

// LTR 257 — Mines of Moria
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MINES_OF_MORIA_257: CardRecord = CardRecord::new(
    "Mines of Moria",
    "0be723d6-4ada-4c3f-b87b-8ab83a4bbb8f",
    "Arthur Yuan",
    crate::card::CardRules::unsupported(),
);

// LTR 258 — Mount Doom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOUNT_DOOM_258: CardRecord = CardRecord::new(
    "Mount Doom",
    "b5bc71a1-2344-4bc6-aa60-658cec19d0d6",
    "Jonas De Ro",
    crate::card::CardRules::unsupported(),
);

// LTR 305 — Gandalf the White
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GANDALF_THE_WHITE_305: CardRecord = CardRecord::new(
    "Gandalf the White",
    "2c9dc67a-5c26-4044-82b6-d5b6e195ae64",
    "Dominik Mayer",
    crate::card::CardRules::unsupported(),
);

// LTR 344 — Rivendell
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIVENDELL_344: CardRecord = CardRecord::new(
    "Rivendell",
    "650fa2f4-2916-427c-a0f9-37e2dbe8e1fc",
    "Josu Solano",
    crate::card::CardRules::unsupported(),
);

// LTR 350 — Borne Upon a Wind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BORNE_UPON_A_WIND_350: CardRecord = CardRecord::new(
    "Borne Upon a Wind",
    "60ebd6a8-2e93-40ee-951a-4fcf12c85e3d",
    "Alexander Mokhov",
    crate::card::CardRules::unsupported(),
);

// LTR 360 — Glóin, Dwarf Emissary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLOIN_DWARF_EMISSARY_360: CardRecord = CardRecord::new(
    "Glóin, Dwarf Emissary",
    "6d74d1af-5cc6-422e-949c-de9e39b76154",
    "Tomas Duchek",
    crate::card::CardRules::unsupported(),
);

// LTR 362 — Moria Marauder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MORIA_MARAUDER_362: CardRecord = CardRecord::new(
    "Moria Marauder",
    "b9e36249-02f5-4c11-9a2b-6be81eb6b490",
    "Andrea Piparo",
    crate::card::CardRules::unsupported(),
);

// LTR 370 — Lotho, Corrupt Shirriff
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOTHO_CORRUPT_SHIRRIFF_370: CardRecord = CardRecord::new(
    "Lotho, Corrupt Shirriff",
    "69d97af0-8af0-4124-b56f-2633d34e5574",
    "Ilker Yildiz",
    crate::card::CardRules::unsupported(),
);

// LTR 407 — Boromir, Warden of the Tower
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOROMIR_WARDEN_OF_THE_TOWER_407: CardRecord = CardRecord::new(
    "Boromir, Warden of the Tower",
    "97ec04f9-0563-4490-b252-714df2ddbf58",
    "Colin Boyer",
    crate::card::CardRules::unsupported(),
);

// LTR 418 — Last March of the Ents
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAST_MARCH_OF_THE_ENTS_418: CardRecord = CardRecord::new(
    "Last March of the Ents",
    "66763118-6a1e-465a-bfe0-6fe18c419875",
    "David Rapoza",
    crate::card::CardRules::unsupported(),
);

// LTR 437 — Merry, Esquire of Rohan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERRY_ESQUIRE_OF_ROHAN_437: CardRecord = CardRecord::new(
    "Merry, Esquire of Rohan",
    "259ff889-fc9a-42f7-998d-0ab23c94ad8a",
    "Tyler Jacobson",
    crate::card::CardRules::unsupported(),
);

// LTR 443 — The Grey Havens
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_GREY_HAVENS_443: CardRecord = CardRecord::new(
    "The Grey Havens",
    "9714aa30-1db2-4670-9a0b-72acfc3f703c",
    "Kieran Yanner",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &EAGLES_OF_THE_NORTH,
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
    &MITHRIL_COAT_245,
    &THE_ONE_RING,
    &STING_THE_GLINTING_DAGGER_250,
    &GREAT_HALL_OF_THE_CITADEL_254,
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
