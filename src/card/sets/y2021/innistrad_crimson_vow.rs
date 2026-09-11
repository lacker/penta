//! Innistrad: Crimson Vow cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::BattlefieldArrivalDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardArt;
use crate::card::CardNameDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ComparisonDef;
use crate::card::ConditionDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ExilePlayDurationDef;
use crate::card::ManaColor;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCountConditionDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::ScaledValueDef;
use crate::card::SubtypeDef;
use crate::card::SumValueDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenCountersDef;
use crate::card::TokenDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::tokens;
use crate::ids::ParentBinding;
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "VOW",
    slug: "innistrad-crimson-vow",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const BLOOD_TOKEN: TokenCharacteristics = tokens::blood().with_art(CardArt::new(
    "a6f374bc-cd29-469f-808a-6a6c004ee8aa",
    "Miranda Meeks",
));

// VOW 46 — Welcoming Vampire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WELCOMING_VAMPIRE_46: CardRecord = CardRecord::new(
    "Welcoming Vampire",
    "d8f69cea-823c-482b-a605-8138b3d950e6",
    "Lorenzo Mastroianni",
    crate::card::CardRules::unsupported(),
);

// VOW 55 — Cruel Witness
pub(in crate::card::sets) static CRUEL_WITNESS: CardRecord = CardRecord::new(
    "Cruel Witness",
    "5bf2c686-efb0-46c7-b34e-c77987914b96",
    "Vincent Proce",
    // A four-mana flier that also fixes every draw afterwards, in a deck
    // already casting the spells that turn it on.
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Bird", "Horror"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever you cast a noncreature spell, surveil 1.",
            // On the cast rather than the resolution, so a countered spell
            // has already paid for its surveil.
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::NoncreatureSpell,
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

// VOW 87 — Wash Away
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WASH_AWAY_87: CardRecord = CardRecord::new(
    "Wash Away",
    "43411ade-be80-4535-8baa-7055e78496df",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// VOW 95 — Blood Fountain
pub(in crate::card::sets) static BLOOD_FOUNTAIN: CardRecord = CardRecord::new(
    "Blood Fountain",
    "dd03651e-ada0-41dc-8722-0eba476943e3",
    "Evyn Fong",
    // One mana smooths the draw now; the same card buys back two creatures
    // later, which is why a graveyard deck runs it over a plain rummage.
    CardRules::new_artifact(mana_cost!("{B}")).with_abilities(&[
        abilities::enters_trigger(
            "When this artifact enters, create a Blood token. (It's an artifact with \"{1}, {T}, \
             Discard a card, Sacrifice this token: Draw a card.\")",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(BLOOD_TOKEN))),
        ),
        AbilityDef::activated_with_targets(
            "{3}{B}, {T}, Sacrifice this artifact: Return up to two target creature cards from \
             your graveyard to your hand.",
            &[
                CostDef::Mana(mana_cost!("{3}{B}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            // One slot holding up to two, so a graveyard with a single
            // creature in it still activates.
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
                2,
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// VOW 101 — Concealing Curtains // Revealing Eye
pub(in crate::card::sets) static CONCEALING_CURTAINS: CardRecord = CardRecord::new_dfc(
    "Concealing Curtains // Revealing Eye",
    "612b2e6e-fe8d-49ad-b845-6fa7fa59ffd1",
    "Brian Valeza",
    &[
        (
            "Concealing Curtains",
            const {
                CardRules::new_creature(mana_cost!("{B}"), &["Wall"], 0, 4).with_abilities(
                    &const {
                        [
                            abilities::defender(),
                            AbilityDef::activated(
                                "{2}{B}: Transform this creature. Activate only as a sorcery.",
                                &[CostDef::Mana(mana_cost!("{2}{B}"))],
                                EffectDef::Transform {
                                    object: EffectRecipientDef::Source,
                                },
                            )
                            .with_activation_timing(ActivationTimingDef::SorcerySpeed),
                        ]
                    },
                )
            },
        ),
        (
            "Revealing Eye",
            const {
                CardRules::new_creature_without_mana_cost(&["Eye", "Horror"], 3, 4)
                    // A back face has no mana cost to read a colour off; the
                    // colour indicator is what says she is still black.
                    .printed_colors(&[ManaColor::Black])
                .with_abilities(&const { [
                    abilities::menace(),
                    AbilityDef::triggered_with_targets(
                        "When this creature transforms into Revealing Eye, target opponent reveals their hand. \
                         You may choose a nonland card from it. If you do, that player discards that card, then \
                         draws a card.",
                        TriggerEventDef::transforms(ObjectPredicateDef::Source),
                        &const { [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                        )] },
                        // "You may choose a nonland card from it": a choice of none is a legal
                        // answer, which is why the minimum is zero rather than one.
                        EffectDef::Sequence(&const { [
                            EffectDef::RevealHand {
                                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            },
                            EffectDef::Choose(ChooseDef {
                                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                                unchosen: None,
                                chooser: PlayerRefDef::EffectController,
                                candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                                    &[ZoneKind::Hand],
                                    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                                )),
                                exclude: None,
                                minimum: 0,
                                maximum: 1,
                                visibility: ChoiceVisibilityDef::Public,
                                then: &EffectDef::ForEachInBinding {
                                    objects: ParentBinding,
                                    binding: ParentBinding,
                                    // What the Eye does with the card it picked. Written as a walk over the
                                    // chosen set rather than a plain sequence, because "if you do" gates the
                                    // draw as well as the discard: an Eye that looked and took nothing leaves
                                    // the opponent with the hand they had.
                                    effect: &EffectDef::Sequence(&const { [
                                        EffectDef::discard_cards(EffectRecipientDef::object(
                                                ObjectRefDef::Binding(ParentBinding),
                                            )),
                                        EffectDef::DrawCards {
                                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                            amount: ValueDef::Constant(1),
                                        },
                                    ] }),
                                },
                            }),
                        ] }),
                    ),
                ] })
            },
        ),
    ],
);

// VOW 134 — Undying Malice
pub(in crate::card::sets) static UNDYING_MALICE: CardRecord = CardRecord::new(
    "Undying Malice",
    "8eb38041-043a-4b18-9d9a-f1283684e8f1",
    "Igor Kieryluk",
    // One mana that answers removal, wins a combat, and re-triggers an
    // arrival, all by making the creature's death a profit.
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Until end of turn, target creature gains \"When this creature dies, return it to the \
         battlefield tapped under its owner's control with a +1/+1 counter on it.\"",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            // Granted to the creature rather than kept on this spell, which
            // is what the printed quotation marks mean: the ability leaves
            // with the creature and comes back with the new object.
            effect: AppliedEffectDef::add_ability(&AbilityDef::triggered(
                "When this creature dies, return it to the battlefield tapped under its \
                 owner's control with a +1/+1 counter on it.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                // Tapped and countered on arrival rather than afterwards:
                // the permanent is never briefly untapped.
                EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Source,
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: BattlefieldArrivalDef {
                        modifications: &[BattlefieldEntryModificationDef::Tapped],
                        counters: Some(TokenCountersDef {
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        }),
                        // "Under its owner's control", which the default
                        // already is.
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// VOW 142 — Ancestral Anger
pub(in crate::card::sets) static ANCESTRAL_ANGER: CardRecord = CardRecord::new(
    "Ancestral Anger",
    "5dee47ab-d603-4346-97f4-a25dc3f47765",
    "Randy Vargas",
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gains trample and gets +X/+0 until end of \
         turn, where X is 1 plus the number of cards named Ancestral \
         Anger in your graveyard.\nDraw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Sum(&SumValueDef {
                            left: ValueDef::Constant(1),
                            right: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::NameEquals(CardNameDef::Literal(
                                    "Ancestral Anger",
                                )),
                                &[ZoneKind::Graveyard],
                                PlayerRelation::You,
                            )),
                        }),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            abilities::draw_cards(ValueDef::Constant(1)),
        ]),
    )]),
);

// VOW 164 — Kessig Flamebreather
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KESSIG_FLAMEBREATHER_164: CardRecord = CardRecord::new(
    "Kessig Flamebreather",
    "303ad78a-b02a-44dc-afe6-7f95781a5062",
    "Lius Lasahido",
    crate::card::CardRules::unsupported(),
);

// VOW 174 — Reckless Impulse
pub(in crate::card::sets) static RECKLESS_IMPULSE: CardRecord = CardRecord::new(
    "Reckless Impulse",
    "6943c07f-ab0d-4f5a-bbe9-c0a83dc98546",
    "Mathias Kollros",
    // Two cards for two mana in a colour that does not draw them. The extra
    // turn is what makes it a real two-for-one: a red deck casting this on
    // three still has the mana to spend both halves before they lapse.
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell(
        "Exile the top two cards of your library. Until the end of your next turn, you may play \
         those cards.",
        EffectDef::ExileTopOfLibraryToPlay {
            player: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
            // "You may play those cards", not "without paying their mana
            // costs": the cards are still bought at full price.
            free: false,
            face_down: false,
            duration: ExilePlayDurationDef::UntilEndOfYourNextTurn,
            spend_any_color: false,
            play_condition: None,
            cast_only: false,
        },
    )),
);

// VOW 182 — Voldaren Epicure
pub(in crate::card::sets) static VOLDAREN_EPICURE: CardRecord = CardRecord::new(
    "Voldaren Epicure",
    "ae154e64-f626-45fb-bd52-840c1c27b2d3",
    "Martina Fačková",
    // One mana for a body, a point of damage, and a card the Blood turns a
    // dead draw into later.
    CardRules::new_creature(mana_cost!("{R}"), &["Vampire"], 1, 1).with_ability(
        abilities::enters_trigger(
            "When this creature enters, it deals 1 damage to each opponent. Create a Blood token. \
             (It's an artifact with \"{1}, {T}, Discard a card, Sacrifice this token: Draw a \
             card.\")",
            // One clause with two instructions in the order it prints them: the damage
            // is the reason the one-drop is played and the Blood is what it leaves
            // behind.
            EffectDef::Sequence(&[
                EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)),
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(BLOOD_TOKEN))),
            ]),
        ),
    ),
);

// VOW 189 — Bramble Wurm
pub(in crate::card::sets) static BRAMBLE_WURM: CardRecord = CardRecord::new(
    "Bramble Wurm",
    "8f16f137-4ceb-469c-a381-e575d58f456b",
    "Lars Grant-West",
    // Seven mana is more than most decks reach, so the graveyard half is
    // what the card usually does: five life for three, from the bin.
    CardRules::new_creature(mana_cost!("{6}{G}"), &["Wurm"], 7, 6).with_abilities(&[
        abilities::reach(),
        abilities::trample(),
        abilities::enters_trigger(
            "When this creature enters, you gain 5 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(5),
            },
        ),
        AbilityDef::activated(
            "{2}{G}, Exile this card from your graveyard: You gain 5 life.",
            &[CostDef::Mana(mana_cost!("{2}{G}")), CostDef::ExileSource],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(5),
            },
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// VOW 225 — Ulvenwald Oddity // Ulvenwald Behemoth
pub(in crate::card::sets) static ULVENWALD_ODDITY: CardRecord = CardRecord::new_dfc(
    "Ulvenwald Oddity // Ulvenwald Behemoth",
    "5fdf5fc4-69c8-4a59-9095-c2feefb64371",
    "Brent Hollowell",
    &[
        (
            "Ulvenwald Oddity",
            const {
                CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Beast"], 4, 4).with_abilities(
                    &const {
                        [
                            abilities::trample(),
                            abilities::haste(),
                            AbilityDef::activated(
                                "{5}{G}{G}: Transform this creature.",
                                &[CostDef::Mana(mana_cost!("{5}{G}{G}"))],
                                EffectDef::Transform {
                                    object: EffectRecipientDef::Source,
                                },
                            ),
                        ]
                    },
                )
            },
        ),
        (
            "Ulvenwald Behemoth",
            const {
                CardRules::new_creature_without_mana_cost(&["Beast", "Horror"], 8, 8)
                    // Same again: the indicator keeps the Behemoth green.
                    .printed_colors(&[ManaColor::Green])
                .with_abilities(&const { [
                    abilities::trample(),
                    abilities::haste(),
                    AbilityDef::static_ability(
                        "Other creatures you control get +1/+1 and have trample and haste.",
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::matching_objects(
                                // "Other creatures you control", which excludes the Behemoth itself: it
                                // already has both keywords and does not need the counters.
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                            // What the back face hands the rest of the board. The keywords are the ones
                            // it already has, which is the joke: the 8/8 makes everything else look
                            // like a smaller version of itself.
                            effect: AppliedEffectDef::Composite(&const { [
                                AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
                                AppliedEffectDef::add_ability(&const { abilities::trample() }),
                                AppliedEffectDef::add_ability(&const { abilities::haste() }),
                            ] }),
                        },
                    ),
                ] })
            },
        ),
    ],
);

// VOW 239 — Halana and Alena, Partners
pub(in crate::card::sets) static HALANA_AND_ALENA_PARTNERS: CardRecord = CardRecord::new(
    "Halana and Alena, Partners",
    "608fa232-f5fe-4c58-9efe-fb780f454b19",
    "Jason Rainville",
    CardRules::new_creature(mana_cost!("{2}{R}{G}"), &["Human", "Ranger"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::reach(),
            abilities::first_strike(),
            AbilityDef::triggered_with_targets(
                "At the beginning of combat on your turn, put X +1/+1 counters \
                 on another target creature you control, where X is Halana and \
                 Alena's power. That creature gains haste until end of turn.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::BeginningOfCombat,
                    player: PlayerRelation::You,
                },
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::SourcePower,
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_ability(&abilities::haste()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
        ]),
);

// VOW 259 — Lantern of the Lost
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LANTERN_OF_THE_LOST_259: CardRecord = CardRecord::new(
    "Lantern of the Lost",
    "c2303f11-2c82-44d5-893a-8e71dece7746",
    "Chris Cold",
    crate::card::CardRules::unsupported(),
);

// VOW 261 — Deathcap Glade
pub(in crate::card::sets) static DEATHCAP_GLADE: CardRecord = CardRecord::new(
    "Deathcap Glade",
    "d5523659-98a8-4ae2-9ec7-d77e7352c374",
    "Sam Burley",
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped unless you control two or more other \
             lands.",
            ReplacementEffectDef::Conditional {
                condition: ConditionDef::ObjectCount(&ObjectCountConditionDef {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 2,
                }),
                if_true: &[],
                if_false: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                )],
            },
        ),
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

// VOW 262 — Dreamroot Cascade
pub(in crate::card::sets) static DREAMROOT_CASCADE: CardRecord = CardRecord::new(
    "Dreamroot Cascade",
    "eb604455-c411-414d-a2ef-e7567ee86a4d",
    "Sam Burley",
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped unless you control two or more other \
             lands.",
            ReplacementEffectDef::Conditional {
                condition: ConditionDef::ObjectCount(&ObjectCountConditionDef {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 2,
                }),
                if_true: &[],
                if_false: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                )],
            },
        ),
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

// VOW 264 — Shattered Sanctum
pub(in crate::card::sets) static SHATTERED_SANCTUM: CardRecord = CardRecord::new(
    "Shattered Sanctum",
    "ad44c9aa-eb8f-4200-8dfe-2af728d80083",
    "Muhammad Firdaus",
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped unless you control two or more other \
             lands.",
            ReplacementEffectDef::Conditional {
                condition: ConditionDef::ObjectCount(&ObjectCountConditionDef {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 2,
                }),
                if_true: &[],
                if_false: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                )],
            },
        ),
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

// VOW 265 — Stormcarved Coast
pub(in crate::card::sets) static STORMCARVED_COAST: CardRecord = CardRecord::new(
    "Stormcarved Coast",
    "299f1dee-b3d7-472b-aa0b-2f9b46a96da5",
    "Sarah Finnigan",
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped unless you control two or more other \
             lands.",
            ReplacementEffectDef::Conditional {
                condition: ConditionDef::ObjectCount(&ObjectCountConditionDef {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 2,
                }),
                if_true: &[],
                if_false: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                )],
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Red])),
        ),
    ]),
);

// VOW 266 — Sundown Pass
pub(in crate::card::sets) static SUNDOWN_PASS: CardRecord = CardRecord::new(
    "Sundown Pass",
    "8f3fddd7-ede4-41c7-a645-a6af298a3d35",
    "Muhammad Firdaus",
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped unless you control two or more other \
             lands.",
            ReplacementEffectDef::Conditional {
                condition: ConditionDef::ObjectCount(&ObjectCountConditionDef {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 2,
                }),
                if_true: &[],
                if_false: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                )],
            },
        ),
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

// VOW 310 — Bloodtithe Harvester
/// Twice the count, and downward. Two Blood is -4/-4, which is what makes
/// the second token worth keeping around rather than cashing in.
static HARVESTER_PENALTY: ValueDef = ValueDef::Scaled(&ScaledValueDef::new(
    // Nothing but a token carries the Blood artifact type, so naming it is
    // enough: the count is read as the ability resolves, and the token the
    // Harvester's own arrival made is one of them.
    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Blood")),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    )),
    -2,
));

pub(in crate::card::sets) static BLOODTITHE_HARVESTER: CardRecord = CardRecord::new(
    "Bloodtithe Harvester",
    "01182501-2b50-4b87-835a-fea3c5e6e330",
    "Sami Makkonen",
// Two mana for a 3/2 that replaces itself with a card later, and can
    // instead be spent as removal the turn it stops attacking.
    CardRules::new_creature(mana_cost!("{B}{R}"), &["Vampire"], 3, 2)
        .with_abilities(&[
            abilities::enters_trigger(
                "When this creature enters, create a Blood token.",
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    BLOOD_TOKEN,
                ))),
            ),
            // Sacrificing the Harvester is what pays for the removal, so the body
            // and the answer are the same card twice rather than both at once.
            AbilityDef::activated_with_targets(
                "{T}, Sacrifice this creature: Target creature gets -X/-X until end of turn, where X is \
                 twice the number of Blood tokens you control. Activate only as a sorcery.",
                &[CostDef::TapSource, CostDef::SacrificeSource],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(HARVESTER_PENALTY, HARVESTER_PENALTY),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed),
        ]),
);

// VOW 359 — Hullbreaker Horror
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HULLBREAKER_HORROR_359: CardRecord = CardRecord::new(
    "Hullbreaker Horror",
    "2e073047-fba8-41bd-b260-1eefb084fc80",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

// VOW 374 — Alchemist's Gambit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALCHEMIST_S_GAMBIT_374: CardRecord = CardRecord::new(
    "Alchemist's Gambit",
    "9eab8938-57c9-4e08-b808-09eb02b040a0",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &WELCOMING_VAMPIRE_46,
    &CRUEL_WITNESS,
    &WASH_AWAY_87,
    &BLOOD_FOUNTAIN,
    &CONCEALING_CURTAINS,
    &UNDYING_MALICE,
    &ANCESTRAL_ANGER,
    &KESSIG_FLAMEBREATHER_164,
    &RECKLESS_IMPULSE,
    &VOLDAREN_EPICURE,
    &BRAMBLE_WURM,
    &ULVENWALD_ODDITY,
    &HALANA_AND_ALENA_PARTNERS,
    &LANTERN_OF_THE_LOST_259,
    &DEATHCAP_GLADE,
    &DREAMROOT_CASCADE,
    &SHATTERED_SANCTUM,
    &STORMCARVED_COAST,
    &SUNDOWN_PASS,
    &BLOODTITHE_HARVESTER,
    &HULLBREAKER_HORROR_359,
    &ALCHEMIST_S_GAMBIT_374,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
