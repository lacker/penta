//! The Lord of the Rings: Tales of Middle-earth cards cataloged for the
//! Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, BattlefieldEntryModificationDef, CardArt, CardRules, CardSet,
    CardSupertype, CardType, ChoiceVisibilityDef, ChooseDef, ComparisonDef, ConditionDef,
    CounterKind, CreatureTypeSetDef, DrawEventMatcherDef, EffectDef, EffectRecipientDef, ManaColor,
    ManaRestrictionDef, ManaSpendEffectDef, ObjectChoiceBindingDef, ObjectPredicateDef,
    ObjectQueryDef, ObjectSetDef, PlayerRefDef, PlayerRelation, PlayerSetDef, ReplacementEffectDef,
    ResolvedEffectDurationDef, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef,
    ZoneKind, ZonePlacement, abilities, tokens,
};
use crate::mana_cost;
use crate::{ParentBinding, TargetIndex};

// LTR 0 — The One Ring
pub(in crate::card::sets) static THE_ONE_RING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("93de9042-cc62-4ade-8d8d-68fdbc84bfae"),
    "The One Ring",
    crate::card::CardArt::new("93de9042-cc62-4ade-8d8d-68fdbc84bfae", "Veli Nyström"),
    crate::card::CardSet::LordOfTheRings,
    // A turn of complete safety, then a card every turn for a life total
    // that runs out faster than it looks like it will.
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
                &[AbilityCostDef::TapSource],
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

// LTR 7 — Eagles of the North
pub(in crate::card::sets) static EAGLES_OF_THE_NORTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c1bd3bc0-77bd-40fe-b4f1-835a04cb6e41"),
    "Eagles of the North",
    CardArt::new("c1bd3bc0-77bd-40fe-b4f1-835a04cb6e41", "Axel Sauerwald"),
    CardSet::LordOfTheRings,
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
            abilities::typecycling(
                "Plainscycling {1} ({1}, Discard this card: Search your library for a Plains card, \
                 reveal it, put it into your hand, then shuffle.)",
                mana_cost!("{1}"),
                ObjectPredicateDef::Subtype("Plains"),
            ),
        ]),
);

// LTR 26 — Reprieve
pub(in crate::card::sets) static REPRIEVE: CardRecord = CardRecord::new_with_legacy_id(
    2168,
    "Reprieve",
    CardArt::new("1bd3fa8a-6c50-4f7f-9ae3-0810eec5e3db", "Justyna Dura"),
    CardSet::LordOfTheRings,
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
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// LTR 60 — Lórien Revealed
pub(in crate::card::sets) static LORIEN_REVEALED: CardRecord = CardRecord::new_with_legacy_id(
    2209,
    "Lórien Revealed",
    CardArt::new("0ce44270-a684-4489-9077-521456e6dfaa", "Randy Gallegos"),
    CardSet::LordOfTheRings,
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
        abilities::typecycling(
            "Islandcycling {1} ({1}, Discard this card: Search your library for an Island card, reveal it, put it into your hand, then shuffle.)",
            mana_cost!("{1}"),
            ObjectPredicateDef::Subtype("Island"),
        ),
    ]),
);

// LTR 71 — Stern Scolding
pub(in crate::card::sets) static STERN_SCOLDING: CardRecord = CardRecord::new_with_legacy_id(
    2125,
    "Stern Scolding",
    CardArt::new("3ca1e1de-b916-445f-b3b2-0f4d0cc7ceeb", "Valera Lutfullina"),
    CardSet::LordOfTheRings,
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
    ObjectPredicateDef::Subtype("Army"),
    &[ZoneKind::Battlefield],
    PlayerSetDef::Related(PlayerRelation::You),
);

pub(in crate::card::sets) static ORCISH_BOWMASTERS: CardRecord = CardRecord::new_with_legacy_id(
    2215,
    "Orcish Bowmasters",
    CardArt::new("adfd33cb-086c-48f4-b9fa-91b5e8f6f3d7", "Anna Podedworna"),
    CardSet::LordOfTheRings,
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
                    EffectDef::DealDamage {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(1),
                    },
                    // The token is made first so that the choice below always has something
                    // to find; with an Army already out, nothing new arrives.
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ObjectCount {
                            query: AN_ARMY_YOU_CONTROL,
                            comparison: ComparisonDef::Equal,
                            amount: 0,
                        },
                        then: &EffectDef::create_creature_token(&["Orc", "Army"], &[ManaColor::Black], 0, 0).with_art(
                                CardArt::new("6943f966-fd21-427c-a13f-44727edcaa4b", "Veli Nyström"),
                            ),
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
    PrintingAnchor::scryfall("a6539e26-b63b-4725-9407-caaf451de084"),
    "Troll of Khazad-dûm",
    CardArt::new("a6539e26-b63b-4725-9407-caaf451de084", "Simon Dominic"),
    CardSet::LordOfTheRings,
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
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlockedExceptByAtLeast(3)),
                },
            ),
            abilities::typecycling(
                "Swampcycling {1} ({1}, Discard this card: Search your library for a Swamp card, reveal \
                 it, put it into your hand, then shuffle.)",
                mana_cost!("{1}"),
                ObjectPredicateDef::Subtype("Swamp"),
            ),
        ]),
);

// LTR 137 — Improvised Club
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IMPROVISED_CLUB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b8397d13-eeaf-4b4e-b3cd-9a9ac231873a"),
    "Improvised Club",
    crate::card::CardArt::new("b8397d13-eeaf-4b4e-b3cd-9a9ac231873a", "Pablo Mendoza"),
    crate::card::CardSet::LordOfTheRings,
    crate::card::CardRules::unsupported(),
);

// LTR 139 — Oliphaunt
pub(in crate::card::sets) static OLIPHAUNT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6989018c-37b1-4282-a4af-9cc97f160b4d"),
    "Oliphaunt",
    CardArt::new("6989018c-37b1-4282-a4af-9cc97f160b4d", "John Di Giovanni"),
    CardSet::LordOfTheRings,
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
        abilities::typecycling(
            "Mountaincycling {1} ({1}, Discard this card: Search your library for a Mountain card, reveal it, put it into your hand, then shuffle.)",
            mana_cost!("{1}"),
            ObjectPredicateDef::Subtype("Mountain"),
        ),
    ]),
);

// LTR 142 — Rally at the Hornburg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RALLY_AT_THE_HORNBURG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee7292f7-1c7e-449c-9c52-7584d6a14c2c"),
    "Rally at the Hornburg",
    crate::card::CardArt::new("ee7292f7-1c7e-449c-9c52-7584d6a14c2c", "Ekaterina Burmak"),
    crate::card::CardSet::LordOfTheRings,
    crate::card::CardRules::unsupported(),
);

// LTR 158 — Delighted Halfling
pub(in crate::card::sets) static DELIGHTED_HALFLING: CardRecord = CardRecord::new_with_legacy_id(
    2153,
    "Delighted Halfling",
    CardArt::new("71384418-173a-4f77-adab-56e52fa23692", "Inka Schulz"),
    CardSet::LordOfTheRings,
    CardRules::new_creature(mana_cost!("{G}"), &["Halfling", "Citizen"], 1, 2).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color. Spend this mana only to cast a legendary spell, and that spell can't be countered.",
            &[AbilityCostDef::TapSource],
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
pub(in crate::card::sets) static GENEROUS_ENT: CardRecord = CardRecord::new_with_legacy_id(
    2122,
    "Generous Ent",
    CardArt::new("85d22d5d-3875-42ff-b51e-c6e21db201f5", "Simon Dominic"),
    CardSet::LordOfTheRings,
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Treefolk"], 5, 7).with_abilities(&[
        abilities::reach(),
        abilities::enters_trigger("When this creature enters, create a Food token.", EffectDef::create_token(tokens::food()).with_art(CardArt::new(
                "4a029bdc-92e3-4d85-8af5-e33429a5f017",
                "L J Koh",
            ))),
        // Six mana is not what this card is for. Forestcycling is: one mana
        // from hand, and the Ent becomes the land the draw did not give you.
        abilities::typecycling(
            "Forestcycling {1} ({1}, Discard this card: Search your library for a Forest card, reveal it, put it into your hand, then shuffle.)",
            mana_cost!("{1}"),
            ObjectPredicateDef::Subtype("Forest"),
        ),
    ]),
);

// LTR 193 — Arwen, Mortal Queen
pub(in crate::card::sets) static ARWEN_MORTAL_QUEEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("547f92d4-cd1d-4ca7-a6e2-6473b4d3c832"),
    "Arwen, Mortal Queen",
    CardArt::new("547f92d4-cd1d-4ca7-a6e2-6473b4d3c832", "Miranda Meeks"),
    CardSet::LordOfTheRings,
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
                    AbilityCostDef::Mana(mana_cost!("{1}")),
                    AbilityCostDef::RemoveCountersFromSource {
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
pub(in crate::card::sets) static FLAME_OF_ANOR: CardRecord = CardRecord::new_with_legacy_id(
    2163,
    "Flame of Anor",
    CardArt::new("04779a7e-b453-48b9-b392-6d6fd0b8d283", "Yigit Koroglu"),
    CardSet::LordOfTheRings,
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
                    &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Artifact)),
                    true,
                ),
                AbilityDef::spell_with_targets(
                    "This spell deals 5 damage to target creature.",
                    &[AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )],
                    EffectDef::DealDamage {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(5),
                    },
                ),
            ],
        )
        // The condition is read as the spell is cast, not as it resolves, so a
        // Wizard that dies in response has already done its work.
        .with_conditional_mode_maximum(ConditionDef::Exists(ObjectQueryDef::controlled_by(
            ObjectPredicateDef::Subtype("Wizard"),
            &[ZoneKind::Battlefield],
            PlayerSetDef::Related(PlayerRelation::You),
        )), 2),
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &THE_ONE_RING,
    &EAGLES_OF_THE_NORTH,
    &REPRIEVE,
    &LORIEN_REVEALED,
    &STERN_SCOLDING,
    &ORCISH_BOWMASTERS,
    &TROLL_OF_KHAZAD_DUM,
    &IMPROVISED_CLUB,
    &OLIPHAUNT,
    &RALLY_AT_THE_HORNBURG,
    &DELIGHTED_HALFLING,
    &GENEROUS_ENT,
    &ARWEN_MORTAL_QUEEN,
    &FLAME_OF_ANOR,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
