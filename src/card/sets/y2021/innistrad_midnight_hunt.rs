//! Innistrad: Midnight Hunt cards cataloged for the Vintage Cube pool.

use crate::ParentBinding;
use crate::card::AbilityKindDef;
use crate::card::AbilityPredicateDef;
use crate::card::AggregateOperationDef;
use crate::card::BindObjectsDef;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectValueAggregateDef;
use crate::card::ObjectValueDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerSetDef;
use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AppliedEffectDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::PlayerRelation;
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
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "MID",
    slug: "innistrad-midnight-hunt",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// MID 1 — Adeline, Resplendent Cathar
pub(in crate::card::sets) static ADELINE_RESPLENDENT_CATHAR: CardRecord =
    CardRecord::new(
    "Adeline, Resplendent Cathar",
    "18092f68-b96e-4084-9eba-b240d2195d81",
    "Bryan Sola",
// Three mana that attacks for four the turn after it lands and for more
        // every turn after that, because each token it makes makes it bigger.
        CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Knight"], 0, 4)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                abilities::vigilance(),
                AbilityDef::static_ability(
                    "Adeline's power is equal to the number of creatures you control.",
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        // Adeline is a creature you control, so she counts herself, and
                        // every token she makes adds one more before damage. The count
                        // defines her power rather than adding to it, which is why it
                        // also answers in a hand or a graveyard.
                        effect: AppliedEffectDef::define_power(ValueDef::CountMatchingObjects(
                            &ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                        )),
                    },
                ),
                // The token was never declared as an attacker, so nothing watching a
                // declaration sees it -- and with two players the one opponent is the
                // only thing it could be attacking.
                AbilityDef::triggered(
                    "Whenever you attack, for each opponent, create a 1/1 white Human creature token that's \
                     tapped and attacking that player or a planeswalker they control.",
                    // "Whenever you attack" is one or more creatures you control attacking,
                    // counted once for the declaration rather than once per attacker.
                    TriggerEventDef::attack_declared(
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        1,
                        None,
                    ),
                    EffectDef::CreateToken(
                        CreateTokenDef::new(TokenDef::Literal(
                            TokenCharacteristics::creature(&["Human"], &[ManaColor::White], 1, 1).with_art(
                                CardArt::new("7d13a93a-a43d-4cf5-8300-8341f3b7f1b1", "Miguel Mercado"),
                            ),
                        ))
                        .entering_tapped()
                        .entering_attacking(),
                    ),
                ),
            ]),
);

// MID 7 — Brutal Cathar // Moonrage Brute
// Audit: unsupported — The transforming daybound/nightbound double-faced card procedure is not declaratively represented.
pub(in crate::card::sets) static BRUTAL_CATHAR_MOONRAGE_BRUTE_7: CardRecord = CardRecord::new(
    "Brutal Cathar // Moonrage Brute",
    "0dbac7ce-a6fa-466e-b6ba-173cf2dec98e",
    "Karl Kopinski",
    crate::card::CardRules::unsupported(),
);

// MID 10 — Cathar Commando
pub(in crate::card::sets) static CATHAR_COMMANDO: CardRecord = CardRecord::new(
    "Cathar Commando",
    "98cbc1c2-b76e-4da3-aa43-00e10b2ce532",
    "Evyn Fong",
    // Flash is what makes the two halves one card: it can be held up as
    // removal and cashed in as a 3/1 when nothing needs killing.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 3, 1).with_abilities(&[
        abilities::flash(),
        AbilityDef::activated_with_targets(
            "{1}, Sacrifice this creature: Destroy target artifact or enchantment.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY),
        ),
    ]),
);

// MID 24 — Homestead Courage
pub(in crate::card::sets) static HOMESTEAD_COURAGE: CardRecord = CardRecord::new(
    "Homestead Courage",
    "73a9c49f-fcd3-4572-bac7-6eb06fdc0815",
    "Colin Boyer",
    // A counter is permanent where the vigilance is not, so the second cast
    // out of the graveyard is what the card is really priced on.
    CardRules::new_sorcery(mana_cost!("{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Put a +1/+1 counter on target creature you control. It gains vigilance until end of \
             turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::vigilance()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{W}"))]),
    ]),
);

// MID 32 — Search Party Captain
// Audit: unsupported — Needs a count of the creatures that attacked this turn. Only their subtypes are recorded, and counting attackers still on the battlefield would undercount every trade, which is the line the card is cast in.
pub(in crate::card::sets) static SEARCH_PARTY_CAPTAIN: CardRecord = CardRecord::new(
    "Search Party Captain",
    "cb9006c1-2e6f-4bca-a1c4-3cf2a8b6e964",
    "Mike Bierek",
    crate::card::CardRules::unsupported(),
);

// MID 44 — Consider
pub(in crate::card::sets) static CONSIDER: CardRecord = CardRecord::new(
    "Consider",
    "a211d505-4d40-4914-a9da-220770d6ddbc",
    "Zezhou Chen",
    // One mana to see two cards deep and choose which of them the deck is
    // better off having in the graveyard.
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Surveil 1. (Look at the top card of your library. You may put it into your graveyard.)\n\
         Draw a card.",
        EffectDef::Sequence(&[
            abilities::surveil(ValueDef::Constant(1)),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// MID 90 — Bloodtithe Collector
pub(in crate::card::sets) static BLOODTITHE_COLLECTOR: CardRecord = CardRecord::new(
    "Bloodtithe Collector",
    "57d5e536-7774-4949-8127-727ae4d8fc80",
    "Maria Zolotukhina",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Vampire", "Noble"], 3, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_if(
            "When this creature enters, if an opponent lost life this \
             turn, each opponent discards a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::OpponentLostLifeThisTurn,
            EffectDef::Discard {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ]),
);

// MID 96 — Diregraf Horde
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIREGRAF_HORDE: CardRecord = CardRecord::new(
    "Diregraf Horde",
    "153be768-ddad-44f2-bcdd-c40353c807d7",
    "Alex Negrea",
    crate::card::CardRules::unsupported(),
);

// MID 99 — Eaten Alive
pub(in crate::card::sets) static EATEN_ALIVE: CardRecord = CardRecord::new(
    "Eaten Alive",
    "e7975a3c-570a-4bff-a60d-d274f758b93f",
    "Nicholas Gregory",
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "As an additional cost to cast this spell, sacrifice a \
         creature or pay {3}{B}.\nExile target creature or \
         planeswalker.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Planeswalker),
            ]),
        )],
        EffectDef::move_to_zone(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ZoneKind::Exile,
            ZonePlacement::Top,
        ),
    )
    .with_spell_additional_cost(&CostDef::Choice(&[
        CostDef::Sacrifice {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            quantity: CostQuantityDef::Fixed(1),
        },
        CostDef::Mana(mana_cost!("{3}{B}")),
    ]))]),
);

// MID 100 — Ecstatic Awakener // Awoken Demon
pub(in crate::card::sets) static ECSTATIC_AWAKENER: CardRecord = CardRecord::new_dfc(
    "Ecstatic Awakener // Awoken Demon",
    "bbdad18e-e262-41f9-b252-1cbdcdd1b5f9",
    "Tuan Duong Chu",
    // A one-drop that turns a spare body into a card and a 4/4, which is
    // what a sacrifice deck wants from its cheapest slot.
    &[
        (
            "Ecstatic Awakener",
            CardRules::new_creature(mana_cost!("{B}"), &["Human", "Wizard"], 1, 1).with_ability(
                AbilityDef::activated(
                    "{2}{B}, Sacrifice another creature: Draw a card, then transform this \
                     creature. Activate only once each turn.",
                    &[
                        CostDef::Mana(mana_cost!("{2}{B}")),
                        // "Another creature": this one is transforming rather
                        // than dying, so it cannot pay for its own ability.
                        CostDef::SacrificePermanent {
                            object: ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            controller: PlayerRelation::You,
                        },
                    ],
                    EffectDef::Sequence(&[
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(1),
                        },
                        EffectDef::Transform {
                            object: EffectRecipientDef::Source,
                        },
                    ]),
                )
                .once_each_turn(),
            ),
        ),
        (
            "Awoken Demon",
            CardRules::new_creature_without_mana_cost(&["Demon"], 4, 4)
                .printed_colors(&[ManaColor::Black]),
        ),
    ],
);

// MID 107 — Infernal Grasp
pub(in crate::card::sets) static INFERNAL_GRASP: CardRecord = CardRecord::new(
    "Infernal Grasp",
    "17824929-f131-4b8d-addb-66c25323155e",
    "Naomi Baker",
    // Two mana, no restriction on what it answers, and the two life is the
    // whole of the price.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature. You lose 2 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        // The life is part of the resolution rather than a cost, so a target that
        // survives being destroyed still costs it -- and a Grasp that never
        // resolves at all costs nothing.
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )),
);

// MID 123 — Stromkirk Bloodthief
pub(in crate::card::sets) static STROMKIRK_BLOODTHIEF: CardRecord = CardRecord::new(
    "Stromkirk Bloodthief",
    "fa819123-bf13-44ea-9a6e-06c8ab023e44",
    "Caroline Gariba",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Vampire", "Rogue"], 2, 2).with_abilities(&[
        AbilityDef::triggered_if_with_targets(
            "At the beginning of your end step, if an opponent lost life \
             this turn, put a +1/+1 counter on target Vampire you control.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::OpponentLostLifeThisTurn,
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vampire")),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// MID 128 — Ardent Elementalist
pub(in crate::card::sets) static ARDENT_ELEMENTALIST: CardRecord = CardRecord::new(
    "Ardent Elementalist",
    "f58592f7-1df5-428d-9dde-e6acd9a5d1d5",
    "Miguel Mercado",
// Archaeomancer's trigger in red, on a body that trades rather than
    // blocks: the card it buys back is the whole reason to cast it.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Shaman"], 2, 1).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, return target instant or sorcery card from your graveyard to your hand.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ),
);

// MID 133 — Cathartic Pyre
pub(in crate::card::sets) static CATHARTIC_PYRE_133: CardRecord = CardRecord::new(
    "Cathartic Pyre",
    "b045c28a-39f9-4cd9-8f3a-a626b697f409",
    "Ryan Yee",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Cathartic Pyre deals 3 damage to target creature or planeswalker.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(3),
                ),
            ),
            AbilityDef::spell(
                "Discard up to two cards, then draw that many cards.",
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Hand],
                        PlayerSetDef::One(PlayerRefDef::EffectController),
                    )),
                    exclude: None,
                    minimum: 0,
                    maximum: 2,
                    visibility: ChoiceVisibilityDef::Private,
                    then: &EffectDef::Sequence(&[
                        EffectDef::discard_cards(EffectRecipientDef::objects(
                            ObjectSetDef::Binding(ParentBinding),
                        )),
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::BoundObjectCount(ParentBinding),
                        },
                    ]),
                }),
            ),
        ],
    )),
);

// MID 245 — Teferi, Who Slows the Sunset
// Audit: unsupported — The emblem requires additional untapping and drawing as turn-based actions during opponents' untap and draw steps. Installed step triggers resolve later and cannot implement those continuous turn-based-action modifications.
pub(in crate::card::sets) static TEFERI_WHO_SLOWS_THE_SUNSET_245: CardRecord = CardRecord::new(
    "Teferi, Who Slows the Sunset",
    "ad2e18d4-986c-4a44-8f26-1b8689339cfb",
    "Heonhwa",
    crate::card::CardRules::unsupported(),
);

// MID 254 — Jack-o'-Lantern
// Audit: unsupported — Nonpermanent mana activations support hand exile and ongoing command-zone rules objects only. The graveyard mana ability cannot be offered or paid; making it an ordinary activated ability would incorrectly use the stack.
pub(in crate::card::sets) static JACK_O_LANTERN_254: CardRecord = CardRecord::new(
    "Jack-o'-Lantern",
    "21b589ab-45a0-480a-a891-581c34f8a9bf",
    "Josu Hernaiz",
    crate::card::CardRules::unsupported(),
);

// MID 255 — Moonsilver Key
pub(in crate::card::sets) static MOONSILVER_KEY_255: CardRecord = CardRecord::new(
    "Moonsilver Key",
    "87778e37-af92-402e-b037-5fbd6112b682",
    "Joseph Meehan",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
AbilityDef::activated("{1}, {T}, Sacrifice this artifact: Search your library for an artifact card with a mana ability or a basic land card, reveal it, put it into your hand, then shuffle.", &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource, CostDef::SacrificeSource], EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::HasAbility(AbilityPredicateDef::Is(AbilityKindDef::ActivatedMana))]), ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Land), ObjectPredicateDef::Supertype(CardSupertype::Basic)])]), minimum: 0, maximum: ValueDef::Constant(1), reveal: true, destination: ZoneKind::Hand, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None })
]),
);

// MID 265 — Overgrown Farmland
pub(in crate::card::sets) static OVERGROWN_FARMLAND_265: CardRecord = CardRecord::new(
    "Overgrown Farmland",
    "84a76e0f-49fc-4087-8859-98f4a4deacdf",
    "Jonas De Ro",
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped unless you control two or more other lands.",
            crate::card::ReplacementEffectDef::Conditional {
                condition: crate::card::ConditionDef::ObjectCount(&TWO_OR_MORE_OTHER_LANDS),
                if_true: &[],
                if_false: &ENTER_TAPPED,
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {G} or {W}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(crate::card::AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::White,
            ])),
        ),
    ]),
);

// MID 282 — Haunted Ridge
pub(in crate::card::sets) static HAUNTED_RIDGE_282: CardRecord = CardRecord::new(
    "Haunted Ridge",
    "91f67a64-b97d-473a-be9d-c8044ff86605",
    "Piotr Dura",
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped unless you control two or more other lands.",
            crate::card::ReplacementEffectDef::Conditional {
                condition: crate::card::ConditionDef::ObjectCount(&TWO_OR_MORE_OTHER_LANDS),
                if_true: &[],
                if_false: &ENTER_TAPPED,
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(crate::card::AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Red,
            ])),
        ),
    ]),
);

// MID 284 — Rockfall Vale
pub(in crate::card::sets) static ROCKFALL_VALE_284: CardRecord = CardRecord::new(
    "Rockfall Vale",
    "3bfcc5d4-babd-4b66-95fa-c5ec6c49e93a",
    "Piotr Dura",
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped unless you control two or more other lands.",
            crate::card::ReplacementEffectDef::Conditional {
                condition: crate::card::ConditionDef::ObjectCount(&TWO_OR_MORE_OTHER_LANDS),
                if_true: &[],
                if_false: &ENTER_TAPPED,
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(crate::card::AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// MID 285 — Shipwreck Marsh
pub(in crate::card::sets) static SHIPWRECK_MARSH_285: CardRecord = CardRecord::new(
    "Shipwreck Marsh",
    "07ad2562-fc26-40a1-9e6c-21f4f88dc2d8",
    "Steven Belledin",
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped unless you control two or more other lands.",
            crate::card::ReplacementEffectDef::Conditional {
                condition: crate::card::ConditionDef::ObjectCount(&TWO_OR_MORE_OTHER_LANDS),
                if_true: &[],
                if_false: &ENTER_TAPPED,
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {B}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(crate::card::AddManaEffectDef::choice(&[
                ManaColor::Blue,
                ManaColor::Black,
            ])),
        ),
    ]),
);

// MID 336 — Malevolent Hermit // Benevolent Geist
// Audit: unsupported — Its disturb double-faced transformation is not declaratively represented.
pub(in crate::card::sets) static MALEVOLENT_HERMIT_BENEVOLENT_GEIST_336: CardRecord =
    CardRecord::new(
        "Malevolent Hermit // Benevolent Geist",
        "7d0d1d48-559f-48f9-b486-50fc81533443",
        "Daarken",
        crate::card::CardRules::unsupported(),
    );

// MID 365 — Unnatural Growth
pub(in crate::card::sets) static UNNATURAL_GROWTH_365: CardRecord = CardRecord::new(
    "Unnatural Growth",
    "61baa102-9bc0-4f97-89e1-cca4dbd823bd",
    "Svetlin Velinov",
    CardRules::new_enchantment(mana_cost!("{1}{G}{G}{G}{G}")).with_abilities(&[
AbilityDef::triggered("At the beginning of each combat, double the power and toughness of each creature you control until end of turn.", TriggerEventDef::StepBegins { step: TurnStepDef::BeginningOfCombat, player: PlayerRelation::Any }, EffectDef::BindObjects(BindObjectsDef { source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You))), binding: Binding!("growth_creatures"), then: &EffectDef::ForEachInBinding { objects: Binding!("growth_creatures"), binding: Binding!("growth_creature"), effect: &EffectDef::Apply { recipient: EffectRecipientDef::object(ObjectRefDef::Binding(Binding!("growth_creature"))), effect: AppliedEffectDef::modify_power_toughness(ValueDef::ObjectPower(ObjectRefDef::Binding(Binding!("growth_creature"))), ValueDef::AggregateObjectValues(&ObjectValueAggregateDef { objects: ObjectSetDef::One(ObjectRefDef::Binding(Binding!("growth_creature"))), select: ObjectValueDef::Toughness, operation: AggregateOperationDef::Sum })), duration: ResolvedEffectDurationDef::UntilEndOfTurn } } }))
]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ADELINE_RESPLENDENT_CATHAR,
    &BRUTAL_CATHAR_MOONRAGE_BRUTE_7,
    &CATHAR_COMMANDO,
    &HOMESTEAD_COURAGE,
    &SEARCH_PARTY_CAPTAIN,
    &CONSIDER,
    &BLOODTITHE_COLLECTOR,
    &DIREGRAF_HORDE,
    &EATEN_ALIVE,
    &ECSTATIC_AWAKENER,
    &INFERNAL_GRASP,
    &STROMKIRK_BLOODTHIEF,
    &ARDENT_ELEMENTALIST,
    &CATHARTIC_PYRE_133,
    &TEFERI_WHO_SLOWS_THE_SUNSET_245,
    &JACK_O_LANTERN_254,
    &MOONSILVER_KEY_255,
    &OVERGROWN_FARMLAND_265,
    &HAUNTED_RIDGE_282,
    &ROCKFALL_VALE_284,
    &SHIPWRECK_MARSH_285,
    &MALEVOLENT_HERMIT_BENEVOLENT_GEIST_336,
    &UNNATURAL_GROWTH_365,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
