//! Tarkir: Dragonstorm cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::Binding;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef,
    AddManaEffectDef, AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet, CardSupertype,
    CardType, ChoiceVisibilityDef, ChooseDef, ComparisonDef, CounterKind, CreatedTokensDef,
    EffectDef, EffectPaymentDef, EffectRecipientDef, FreePlayDef, FreePlayDurationDef,
    InstalledTriggerDef, ManaColor, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef,
    ObjectSetDef, ObjectSetFilterDef, PayOrDef, PlayActionMatcherDef, PlayRestrictionDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef, QuantifierDef, ResolvedEffectDurationDef,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueComparisonDef, ValueDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::ids::{ParentBinding, TargetIndex};
use crate::mana_cost;

// TDM 1 — Ugin, Eye of the Storms
/// "Up to one target permanent that's one or more colors": colorless is what
/// Ugin does not touch, which is the whole bargain of the deck built around
/// him -- your own artifacts and Eldrazi are safe from every one of these
/// triggers.
static UP_TO_ONE_COLORED_PERMANENT: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Not(&ObjectPredicateDef::ColorCount(0)),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
    1,
)];

static UGIN_EXILES_IT: EffectDef = EffectDef::MoveToZone {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    zone: ZoneKind::Exile,
    placement: ZonePlacement::Top,
};

pub(in crate::card::sets) static UGIN_EYE_OF_THE_STORMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("64a5d494-efa1-446b-bebe-2ad36e154376"),
    "Ugin, Eye of the Storms",
    CardArt::new("64a5d494-efa1-446b-bebe-2ad36e154376", "Joshua Raphael"),
    CardSet::TarkirDragonstorm,
    // Seven mana that answers something the moment it is cast and again for
    // every colorless spell after it, pays for the next one itself, and
    // eventually empties the library onto the table for free.
    CardRules::new_planeswalker(mana_cost!("{7}"), &["Ugin"], 7)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered_with_targets(
                "When you cast this spell, exile up to one target permanent that's one or more colors.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::Source),
                &UP_TO_ONE_COLORED_PERMANENT,
                UGIN_EXILES_IT,
            ),
            AbilityDef::triggered_with_targets(
                "Whenever you cast a colorless spell, exile up to one target permanent that's one or \
                 more colors.",
                // A colorless spell you cast, which is every spell the deck around him is
                // made of. His own cast is not one of these: he is still on the stack, and
                // this clause is read off the battlefield.
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::ColorCount(0),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                &UP_TO_ONE_COLORED_PERMANENT,
                UGIN_EXILES_IT,
            ),
            AbilityDef::activated(
                "+2: You gain 3 life and draw a card.",
                &[AbilityCostDef::Loyalty(2)],
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
            // A loyalty ability that makes mana is still a mana ability: it never
            // uses the stack, and it is still the one loyalty ability he may use
            // this turn.
            AbilityDef::activated_mana(
                "0: Add {C}{C}{C}.",
                &[AbilityCostDef::Loyalty(0)],
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(3)),
            ),
            AbilityDef::activated(
                "\u{2212}11: Search your library for any number of colorless nonland cards, exile them, \
                 then shuffle. Until end of turn, you may cast those cards without paying their mana \
                 costs.",
                &[AbilityCostDef::Loyalty(-11)],
                // "Any number": the bound is the library, so the search offers everything
                // that matches and takes as many as its controller wants.
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::ColorCount(0),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(i32::MAX),
                    reveal: false,
                    destination: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: Some(ParentBinding),
                    // "Until end of turn, you may cast those cards without paying their mana
                    // costs": the cards the search just exiled, named by what it bound rather
                    // than by anything about exile, since a card that was already there is not
                    // one of them.
                    then: Some(&EffectDef::MayPlayWithoutPaying(FreePlayDef {
                        objects: ObjectSetDef::Binding(ParentBinding),
                        // "Until end of turn" is printed, so this one outlives its resolution.
                        duration: FreePlayDurationDef::UntilEndOfTurn,
                        mandatory: false,
                        grants_haste: false,
                    })),
                },
            ),
        ]),
);

// TDM 8 — Descendant of Storms
pub(in crate::card::sets) static DESCENDANT_OF_STORMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f632be90-9e7f-41f8-a52e-a2952354d730"),
    "Descendant of Storms",
    CardArt::new("f632be90-9e7f-41f8-a52e-a2952354d730", "Lie Setiawan"),
    CardSet::TarkirDragonstorm,
    // A one-mana 2/1 that attacks well early and has somewhere to put mana
    // late. Which half of endure you want changes with the board: the
    // counter makes the attack bigger, the Spirit makes the next one wider.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature attacks, you may pay {1}{W}. If you do, it endures 1.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef::mana(
                    PlayerSetDef::Related(PlayerRelation::You),
                    mana_cost!("{1}{W}"),
                ),
                // "It endures 1": the counter or the Spirit, and the attacking body is
                // what either one is about.
                &EffectDef::Endure {
                    object: EffectRecipientDef::Source,
                    amount: ValueDef::Constant(1),
                },
            )),
        ),
    ),
);

// TDM 12 — Fortress Kin-Guard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORTRESS_KIN_GUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b647a018-1d70-43a1-a265-928bcd863689"),
    "Fortress Kin-Guard",
    crate::card::CardArt::new("b647a018-1d70-43a1-a265-928bcd863689", "Daneen Wilkerson"),
    crate::card::CardSet::TarkirDragonstorm,
    crate::card::CardRules::unsupported(),
);

// TDM 21 — Riling Dawnbreaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RILING_DAWNBREAKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("312f7072-3bf8-449f-bfb7-93727ef26c66"),
    "Riling Dawnbreaker",
    crate::card::CardArt::new("312f7072-3bf8-449f-bfb7-93727ef26c66", "Tuan Duong Chu"),
    crate::card::CardSet::TarkirDragonstorm,
    crate::card::CardRules::unsupported(),
);

// TDM 23 — Salt Road Packbeast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SALT_ROAD_PACKBEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("98d548c9-42bc-4155-8211-0aea801c3724"),
    "Salt Road Packbeast",
    crate::card::CardArt::new("98d548c9-42bc-4155-8211-0aea801c3724", "Ben Wootten"),
    crate::card::CardSet::TarkirDragonstorm,
    crate::card::CardRules::unsupported(),
);

// TDM 33 — Voice of Victory
pub(in crate::card::sets) static VOICE_OF_VICTORY: CardRecord = CardRecord::new_with_legacy_id(
    2282,
    "Voice of Victory",
    CardArt::new("ec3de5f4-bb55-4ab9-995f-f3e0dc22c1bb", "Joshua Cairos"),
    CardSet::TarkirDragonstorm,
    // Two mana that adds two power to every attack and turns off every
    // instant your opponent was holding for the turn you attack.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Bard"], 1, 3)
        .with_abilities(&[
            // Mobilize 2 (CR 702.180a). Written out rather than abbreviated: the
            // keyword is a shorthand for a triggered ability, and this is that ability.
            AbilityDef::triggered(
                "Mobilize 2 (Whenever this creature attacks, create two tapped and attacking 1/1 red Warrior \
                 creature tokens. Sacrifice them at the beginning of the next end step.)",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::create_creature_token(&["Warrior"], &[ManaColor::Red], 1, 1)
                    .with_art(CardArt::new(
                        "7edc0515-a130-45a7-aa09-0e23bba41587",
                        "Forrest Imel",
                    ))
                    .with_amount(2)
                    .entering_tapped()
                    .entering_attacking()
                    .with_created_tokens(CreatedTokensDef {
                        binding: ParentBinding,
                        // The tokens go away at the next end step, and it has to be exactly the
                        // ones this attack made: by then nothing about the board could tell them
                        // apart from the pair the last attack made, or from a Warrior that arrived
                        // some other way. So they are bound as they are created and the delayed
                        // clause names the binding.
                        then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                                "At the beginning of the next end step, sacrifice those tokens.",
                                TriggerEventDef::StepBegins {
                                    step: TurnStepDef::End,
                                    player: PlayerRelation::Any,
                                },
                                EffectDef::Sacrifice {
                                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        ParentBinding,
                                    )),
                                },
                            ))),
                    }),
            ),
            AbilityDef::static_ability(
                "Your opponents can't cast spells during your turn.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                    // "During your turn" is the whole of the clause's timing, and it gates the
                    // restriction rather than narrowing who it names: on their own turn the
                    // same opponents may cast whatever they like.
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::Opponent)),
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(PlayActionMatcherDef::CastSpell, ObjectPredicateDef::Any))),
                    },
                },
            ),
        ]),
);

// TDM 119 — Seize Opportunity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEIZE_OPPORTUNITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f7818d28-b9a5-4341-9adc-666070b8878d"),
    "Seize Opportunity",
    crate::card::CardArt::new(
        "f7818d28-b9a5-4341-9adc-666070b8878d",
        "Josiah \"Jo\" Cameron",
    ),
    crate::card::CardSet::TarkirDragonstorm,
    crate::card::CardRules::unsupported(),
);

// TDM 120 — Shock Brigade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHOCK_BRIGADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("66940466-8e9d-4a85-bfb0-e92189b7a121"),
    "Shock Brigade",
    crate::card::CardArt::new("66940466-8e9d-4a85-bfb0-e92189b7a121", "Fajareka Setiawan"),
    crate::card::CardSet::TarkirDragonstorm,
    crate::card::CardRules::unsupported(),
);

// TDM 127 — Tersa Lightshatter
pub(in crate::card::sets) static TERSA_LIGHTSHATTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("39f07b5b-d764-4c88-920b-36b0ba1c62b0"),
    "Tersa Lightshatter",
    CardArt::new("39f07b5b-d764-4c88-920b-36b0ba1c62b0", "Olivier Bernard"),
    CardSet::TarkirDragonstorm,
    // Three mana for a 3/3 that attacks immediately and turns a spent hand
    // into a card a turn. What she asks for is the graveyard the deck was
    // filling anyway.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Orc", "Wizard"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::haste(),
            abilities::enters_trigger(
                "When Tersa Lightshatter enters, discard up to two cards, then draw that many cards.",
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
                    // "Discard up to two cards, then draw that many." The size is the player's
                    // to choose, so the discard is a choice with a floor of none rather than a
                    // fixed number, and what is drawn is however many that turned out to be.
                    then: &EffectDef::Sequence(&[
                        EffectDef::DiscardCards {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                        },
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::BoundObjectCount(ParentBinding),
                        },
                    ]),
                }),
            ),
            AbilityDef::triggered_if(
                "Whenever Tersa Lightshatter attacks, if there are seven or more cards in your graveyard, \
                 exile a card at random from your graveyard. You may play that card this turn.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                // Seven cards is a real threshold rather than a formality: the attack that
                // turns it on is the one that has already spent a hand.
                &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 7,
                },
                EffectDef::Sequence(&[
                    EffectDef::BindOutput {
                        effect: &EffectDef::SelectAtRandomFromZone {
                            player: EffectRecipientDef::Controller,
                            source: ZoneKind::Graveyard,
                            object: ObjectPredicateDef::Any,
                            amount: ValueDef::Constant(1),
                        },
                        binding: Binding!("random_graveyard_card"),
                    },
                    EffectDef::ExileGrantingControllerPlayThisTurn {
                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                            Binding!("random_graveyard_card"),
                        )),
                    },
                ]),
            ),
        ]),
);

// TDM 134 — Ainok Wayfarer
pub(in crate::card::sets) static AINOK_WAYFARER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57695a9b-8f72-4ccc-a946-5d5037b09b8f"),
    "Ainok Wayfarer",
    CardArt::new("57695a9b-8f72-4ccc-a946-5d5037b09b8f", "Filipe Pagliuso"),
    CardSet::TarkirDragonstorm,
    // Never a blank: it finds a land when the draw is short and grows when
    // it is not, which is what two mana is buying.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Dog", "Scout"], 1, 1).with_ability(
        abilities::enters_trigger(
            "When this creature enters, mill three cards. You may put a land card from among \
             them into your hand. If you don't, put a +1/+1 counter on this creature. (To mill \
             three cards, put the top three cards of your library into your graveyard.)",
            EffectDef::Sequence(&[
                EffectDef::BindOutput {
                    effect: &EffectDef::Mill {
                        player: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                    binding: Binding!("milled_cards"),
                },
                // A minimum of zero is the "you may", and a pile with no land
                // in it never asks.
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    // "From among them" is what this mill just put there,
                    // not what the graveyard already held.
                    candidates: ObjectSetDef::Matching {
                        objects: &ObjectSetDef::Binding(Binding!("milled_cards")),
                        object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::HasType(
                            CardType::Land,
                        )),
                    },
                    exclude: None,
                    minimum: 0,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::Sequence(&[
                        EffectDef::MoveToZone {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                ParentBinding,
                            )),
                            zone: ZoneKind::Hand,
                            placement: ZonePlacement::Top,
                        },
                        // "If you don't" is read off what was taken rather
                        // than off what was offered: declining and having
                        // nothing to take both leave the counter.
                        EffectDef::IfCondition {
                            condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                                left: ValueDef::BoundObjectCount(ParentBinding),
                                comparison: ComparisonDef::LessOrEqual,
                                right: ValueDef::Constant(0),
                            }),
                            then: &EffectDef::AddCounters {
                                object: EffectRecipientDef::Source,
                                kind: CounterKind::PlusOnePlusOne,
                                amount: ValueDef::Constant(1),
                            },
                        },
                    ]),
                }),
            ]),
        ),
    ),
);

// TDM 137 — Champion of Dusan
pub(in crate::card::sets) static CHAMPION_OF_DUSAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c51dcdab-38ee-4804-8859-09adc353c182"),
    "Champion of Dusan",
    CardArt::new("c51dcdab-38ee-4804-8859-09adc353c182", "Bastien L. Deharme"),
    CardSet::TarkirDragonstorm,
    // A 4/2 trades early and then hands its trample to something better
    // from the graveyard, which is the whole arc of the card.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Warrior"], 4, 2).with_abilities(&[
        abilities::trample(),
        AbilityDef::activated_with_targets(
            "Renew — {1}{G}, Exile this card from your graveyard: Put a +1/+1 counter and a trample counter on target creature. Activate only as a sorcery.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{G}")),
                AbilityCostDef::ExileSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            // A trample counter rather than a granted keyword: it stays on
            // the creature and survives anything that ends a duration.
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::Trample,
                    amount: ValueDef::Constant(1),
                },
            ]),
        )
        .with_source_zones(&[ZoneKind::Graveyard])
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// TDM 157 — Sagu Wildling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAGU_WILDLING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b72ee8f9-5e79-4f77-ae7e-e4c274f78187"),
    "Sagu Wildling",
    crate::card::CardArt::new("d8b43b00-f4d1-436c-bf3f-6d414cd4ce38", "Gaboleps"),
    crate::card::CardSet::TarkirDragonstorm,
    crate::card::CardRules::unsupported(),
);

// TDM 343 — Cori-Steel Cutter
pub(in crate::card::sets) static CORI_STEEL_CUTTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("470dd3c8-07c9-42ef-aa9e-3c73b23607ff"),
    "Cori-Steel Cutter",
    CardArt::new("470dd3c8-07c9-42ef-aa9e-3c73b23607ff", "Tomas Duchek"),
    CardSet::TarkirDragonstorm,
    // Two mana that turns every second spell into a hasty attacker, and
    // moves itself onto the new one for free every time.
    CardRules::new_artifact(mana_cost!("{1}{R}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 and has trample and haste.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                },
            ),
            AbilityDef::triggered_if(
                "Flurry — Whenever you cast your second spell each turn, create a 1/1 white Monk \
                 creature token with prowess. You may attach this Equipment to it.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)),
                // Exactly the second, not the second or later: the spell that caused the
                // trigger has already been counted by the time this is read.
                &TriggerConditionDef::SpellsCastThisTurn {
                    quantifier: QuantifierDef::Any,
                    player: PlayerRelation::You,
                    comparison: ComparisonDef::Equal,
                    amount: 2,
                },
                EffectDef::create_creature_token(&["Monk"], &[ManaColor::White], 1, 1)
                    .with_abilities(&[abilities::prowess()])
                    .with_art(CardArt::new(
                        "633d2d10-def7-426f-8496-ed6b45684299",
                        "Elizabeth Peiró",
                    ))
                    .with_created_tokens(CreatedTokensDef {
                        binding: ParentBinding,
                        // "You may attach this Equipment to it": the Monk is named rather than
                        // targeted, so the token the trigger just made is the one it moves onto --
                        // and declining leaves the Equipment where it was.
                        then: &EffectDef::May {
                            player: EffectRecipientDef::Controller,
                            effect: &EffectDef::Attach {
                                object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    ParentBinding,
                                )),
                            },
                        },
                    }),
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{1}{R}"))],
                "Equip {1}{R}",
            ),
        ]),
);

static ELSPETH_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// TDM 398 — Elspeth, Storm Slayer
pub(in crate::card::sets) static ELSPETH_STORM_SLAYER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1fdf9438-fd5f-4638-8f41-dae35ae8f257"),
    "Elspeth, Storm Slayer",
    CardArt::new("1fdf9438-fd5f-4638-8f41-dae35ae8f257", "Jeremy Wilson"),
    CardSet::TarkirDragonstorm,
    // Five mana whose first line is worth more than the three below it: in a
    // deck that makes tokens at all, everything it was already doing happens
    // twice.
    CardRules::new_planeswalker(mana_cost!("{3}{W}{W}"), &["Elspeth"], 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            // The doubling is what every other line on the card is written against:
            // her plus makes two Soldiers, and so does anything else you were
            // already doing.
            AbilityDef::static_ability(
                "If one or more tokens would be created under your control, twice that many of those \
                 tokens are created instead.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoublesTokensCreated),
                },
            ),
            AbilityDef::activated(
                "+1: Create a 1/1 white Soldier creature token.",
                &[AbilityCostDef::Loyalty(1)],
                EffectDef::create_creature_token(&["Soldier"], &[ManaColor::White], 1, 1),
            ),
            AbilityDef::activated(
                "0: Put a +1/+1 counter on each creature you control. Those creatures gain flying until \
                 your next turn.",
                &[AbilityCostDef::Loyalty(0)],
                // "Those creatures" is the set the counters went on. Nothing can join or
                // leave the battlefield between the two halves of one resolution, so
                // naming the same query twice names the same creatures -- and unlike a
                // binding it says outright that they are on the battlefield.
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::objects(ObjectSetDef::Query(ELSPETH_CREATURES)),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::objects(ObjectSetDef::Query(ELSPETH_CREATURES)),
                        effect: AppliedEffectDef::add_ability(&abilities::flying()),
                        duration: ResolvedEffectDurationDef::UntilYourNextTurn,
                    },
                ]),
            ),
            AbilityDef::activated_with_targets(
                "−3: Destroy target creature an opponent controls with mana value 3 or greater.",
                &[AbilityCostDef::Loyalty(-3)],
                // "Mana value 3 or greater", which for a whole number is everything that is
                // not two or less.
                &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(2)),
                            ]),
                            zones: &[ZoneKind::Battlefield],
                            controller: Some(PlayerRelation::Opponent),
                            owner: None,
                        },
                    )],
                EffectDef::destroy_target(TargetIndex::PRIMARY, true),
            ),
        ]),
);

// TDM 409 — Ugin, Eye of the Storms (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &UGIN_EYE_OF_THE_STORMS,
    &DESCENDANT_OF_STORMS,
    &FORTRESS_KIN_GUARD,
    &RILING_DAWNBREAKER,
    &SALT_ROAD_PACKBEAST,
    &VOICE_OF_VICTORY,
    &SEIZE_OPPORTUNITY,
    &SHOCK_BRIGADE,
    &TERSA_LIGHTSHATTER,
    &AINOK_WAYFARER,
    &CHAMPION_OF_DUSAN,
    &SAGU_WILDLING,
    &CORI_STEEL_CUTTER,
    &ELSPETH_STORM_SLAYER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&UGIN_EYE_OF_THE_STORMS, 1), // TDM 409
];
