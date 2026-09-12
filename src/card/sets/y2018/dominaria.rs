//! Dominaria cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ComparisonDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatedTokensDef;
use crate::card::DrawEventMatcherDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::MoveObjectsDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealObjectsDef;
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
use crate::ids::Binding;
use crate::ids::ParentBinding;
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "DOM",
    slug: "dominaria",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// DOM 1 — Karn, Scion of Urza
/// The opponent chooses which of the two you keep, so what Karn draws is
/// always the worse half -- and the better one waits in exile for his minus.
const KARN_CHOSEN: Binding = Binding!("karn_chosen");
const KARN_REST: Binding = Binding!("karn_rest");
/// "This token gets +1/+1 for each artifact you control", which counts the
/// token itself: a lone Construct is a 1/1, and every artifact beside it is
/// another point in both directions.
static ARTIFACTS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Artifact),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static KARN_SCION_OF_URZA: CardRecord = CardRecord::new(
    "Karn, Scion of Urza",
    "07a3d9e8-8597-498b-869c-cff79e0df516",
    "Chase Stone",
// Colorless, so every deck can play him: a card every turn that the
    // other player picks, the pile of leftovers he can cash in later, and a
    // body that grows with the artifacts the deck is made of.
    CardRules::new_planeswalker(mana_cost!("{4}"), &["Karn"], 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "+1: Reveal the top two cards of your library. An opponent chooses one of them. Put that \
                 card into your hand and exile the other with a silver counter on it.",
                &[CostDef::Loyalty(1)],
                abilities::bind_top_cards_then(
                    PlayerRefDef::EffectController,
                    ValueDef::Constant(2),
                    &EffectDef::Sequence(&[
                        EffectDef::RevealObjects(RevealObjectsDef {
                            input: ObjectSetDef::Binding(ParentBinding),
                            then: &EffectDef::None,
                        }),
                            EffectDef::Choose(ChooseDef {
                                binding: ObjectChoiceBindingDef::Objects(KARN_CHOSEN),
                                unchosen: Some(KARN_REST),
                                chooser: PlayerRefDef::Opponent,
                                candidates: ObjectSetDef::Binding(ParentBinding),
                                exclude: None,
                                minimum: 1,
                                maximum: 1,
                                visibility: ChoiceVisibilityDef::Public,
                                then: &EffectDef::Sequence(&[
                                        EffectDef::MoveObjects(MoveObjectsDef {
                                            input: ObjectSetDef::Binding(KARN_CHOSEN),
                                            from: Some(ZoneKind::Library),
                                            zone: ZoneKind::Hand,
                                            placement: ZonePlacement::Top,
                                            moved: None,
                                            then: &EffectDef::None,
                                        }),
                                            EffectDef::MoveObjects(MoveObjectsDef {
                                                input: ObjectSetDef::Binding(KARN_REST),
                                                from: Some(ZoneKind::Library),
                                                zone: ZoneKind::Exile,
                                                placement: ZonePlacement::Top,
                                                moved: Some(ParentBinding),
                                                then: &EffectDef::AddCounters {
                                                    object: EffectRecipientDef::objects(
                                                        ObjectSetDef::Binding(ParentBinding),
                                                    ),
                                                    kind: CounterKind::named("silver"),
                                                    amount: ValueDef::Constant(1),
                                                },
                                            }),
                                    ]),
                            }),
                    ]),
                ),
            ),
            AbilityDef::activated(
                "\u{2212}1: Put a card you own with a silver counter on it from exile into your hand.",
                &[CostDef::Loyalty(-1)],
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Object(ParentBinding),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    // "A card you own with a silver counter on it from exile": the counter is
                    // what makes the pile nameable at all, since exile holds everything anybody
                    // has ever put there.
                    candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                        ObjectPredicateDef::HasCounter(CounterKind::named("silver")),
                        &[ZoneKind::Exile],
                        PlayerSetDef::Related(PlayerRelation::You),
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::move_to_zone(
                        EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                }),
            ),
            AbilityDef::activated(
                "\u{2212}2: Create a 0/0 colorless Construct artifact creature token with \"This token \
                 gets +1/+1 for each artifact you control.\"",
                &[CostDef::Loyalty(-2)],
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::artifact_creature(&["Construct"], &[], 0, 0).with_abilities(&[
                        AbilityDef::static_ability(
                            "This token gets +1/+1 for each artifact you control.",
                            EffectDef::StaticApply {
                                recipient: EffectRecipientDef::Source,
                                effect: AppliedEffectDef::modify_power_toughness(
                                    ValueDef::CountMatchingObjects(&ARTIFACTS_YOU_CONTROL),
                                    ValueDef::CountMatchingObjects(&ARTIFACTS_YOU_CONTROL),
                                ),
                            },
                        ),
                    ]),
                ))),
            ),
        ]),
);

// DOM 2 — Adamant Will
pub(in crate::card::sets) static ADAMANT_WILL: CardRecord = CardRecord::new(
    "Adamant Will",
    "3dfb8817-ca3c-44ba-92f2-e9d6294cd25d",
    "Alex Konstad",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +2/+2 and gains indestructible until end \
         of turn. (Damage and effects that say \"destroy\" don't \
         destroy it.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                AppliedEffectDef::add_ability(&abilities::indestructible()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// DOM 23 — Knight of Grace
// Audit: unsupported — Needs hexproof filtered by an opposing spell or ability source being black; protection from black would incorrectly also prevent damage, blocking, and attachments.
pub(in crate::card::sets) static KNIGHT_OF_GRACE: CardRecord = CardRecord::new(
    "Knight of Grace",
    "7bbbddc0-f8b3-4255-bd82-d50f829ca009",
    "Sidharth Chaturvedi",
    CardRules::unsupported(),
);

// DOM 26 — Lyra Dawnbringer
pub(in crate::card::sets) static LYRA_DAWNBRINGER: CardRecord = CardRecord::new(
    "Lyra Dawnbringer",
    "93be6799-7b9d-44d4-84dc-2961692b5a85",
    "Chris Rahn",
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Angel"], 5, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::first_strike(),
            abilities::lifelink(),
            AbilityDef::static_ability(
                "Other Angels you control get +1/+1 and have lifelink.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Angel")),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::lifelink()),
                    ]),
                },
            ),
        ]),
);

// DOM 68 — Tempest Djinn
pub(in crate::card::sets) static TEMPEST_DJINN: CardRecord = CardRecord::new(
    "Tempest Djinn",
    "3acc883b-3aea-4d0b-ae0f-00d4a08c47c1",
    "Zezhou Chen",
    CardRules::new_creature(mana_cost!("{U}{U}{U}"), &["Djinn"], 0, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature gets +1/+0 for each basic Island you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Supertype(CardSupertype::Basic),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Island")),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    ValueDef::Constant(0),
                ),
            },
        ),
    ]),
);

// DOM 81 — Cast Down
pub(in crate::card::sets) static CAST_DOWN: CardRecord = CardRecord::new(
    "Cast Down",
    "116ce944-6871-4f51-a889-d9c4a5d7cff2",
    "Bastien L. Deharme",
    // Two mana for unconditional removal, priced by the one exception it
    // makes -- which is exactly the thing the opponent built around.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(AbilityDef::destroy_target(
        "Destroy target nonlegendary creature.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(CardSupertype::Legendary)),
        ])),
    )),
);

// DOM 97 — Knight of Malice
// Audit: unsupported — Needs hexproof filtered by an opposing spell or ability source being white; protection from white would incorrectly also prevent damage, blocking, and attachments.
pub(in crate::card::sets) static KNIGHT_OF_MALICE: CardRecord = CardRecord::new(
    "Knight of Malice",
    "b45266f0-eb4f-4a06-bc64-8c2d774b4cc5",
    "Sidharth Chaturvedi",
    CardRules::unsupported(),
);

// DOM 127 — Ghitu Lavarunner
pub(in crate::card::sets) static GHITU_LAVARUNNER: CardRecord = CardRecord::new(
    "Ghitu Lavarunner",
    "c448ba82-a502-459f-9ebc-fc9e85674e6c",
    "Jesper Ejsing",
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Wizard"], 1, 2).with_abilities(&[
        AbilityDef::static_ability(
            "As long as there are two or more instant and/or sorcery cards \
             in your graveyard, this creature gets +1/+0 and has haste. \
             (It can attack and {T} as soon as it comes under your \
             control.)",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 2,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                },
            },
        ),
    ]),
);

// DOM 164 — Grow from the Ashes
pub(in crate::card::sets) static GROW_FROM_THE_ASHES: CardRecord = CardRecord::new(
    "Grow from the Ashes",
    "51d4d1c2-671c-498c-a232-7d076e3dc3bb",
    "Richard Wright",
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_abilities(&[
        abilities::kicker(&[CostDef::Mana(mana_cost!("{2}"))]),
        AbilityDef::spell(
            "Search your library for a basic land card, put it onto the \
             battlefield, then shuffle. If this spell was kicked, instead \
             search your library for two basic land cards, put them onto \
             the battlefield, then shuffle.",
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourcePaidAdditionalCost(
                    crate::AdditionalCostIndex::PRIMARY,
                ),
                then: &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(2),
                    reveal: true,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
                otherwise: &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
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
            },
        ),
    ]),
);

// DOM 199 — Muldrotha, the Gravetide
// Audit: unsupported — Needs a graveyard play allowance consumed by a player-chosen permanent type for each cast, with separate per-turn allowances for every permanent type and lands.
pub(in crate::card::sets) static MULDROTHA_THE_GRAVETIDE: CardRecord = CardRecord::new(
    "Muldrotha, the Gravetide",
    "c654737d-34ac-42ff-ae27-3a3bbb930fc1",
    "Jason Rainville",
    CardRules::unsupported(),
);

// DOM 206 — Tatyova, Benthic Druid
pub(in crate::card::sets) static TATYOVA_BENTHIC_DRUID: CardRecord = CardRecord::new(
    "Tatyova, Benthic Druid",
    "93657aaa-7a0f-49ad-b026-6f79b3bd6768",
    "Mathias Kollros",
    CardRules::new_creature(mana_cost!("{3}{G}{U}"), &["Merfolk", "Druid"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, you gain 1 \
             life and draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::Sequence(&[
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                abilities::draw_cards(ValueDef::Constant(1)),
            ]),
        )]),
);

// DOM 207 — Teferi, Hero of Dominaria
pub(in crate::card::sets) static TEFERI_HERO_OF_DOMINARIA: CardRecord = CardRecord::new(
    "Teferi, Hero of Dominaria",
    "5d10b752-d9cb-419d-a5c4-d4ee1acb655e",
    "Chris Rallis",
// Five mana that draws a card and leaves two lands up, so the turn he
    // lands is not the turn he costs you: the plus pays for the counterspell
    // held behind him.
    CardRules::new_planeswalker(mana_cost!("{3}{W}{U}"), &["Teferi"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "+1: Draw a card. At the beginning of the next end step, untap up to two lands.",
                &[CostDef::Loyalty(1)],
                EffectDef::Sequence(&[
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                    // "The next end step" is whichever one comes first, which on Teferi's own
                    // turn is his: the two lands come back before the other player untaps, and
                    // that is the whole trick.
                    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                        "At the beginning of the next end step, untap up to two lands.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::End,
                            player: PlayerRelation::Any,
                        },
                        // Chosen as the delayed trigger resolves rather than targeted, and nothing
                        // says whose lands they are -- the same shape Time Spiral's six use, with
                        // "up to" meaning a minimum of none.
                        EffectDef::Choose(ChooseDef {
                            binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                            unchosen: None,
                            chooser: PlayerRefDef::EffectController,
                            candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Land),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::Any,
                            )),
                            exclude: None,
                            minimum: 0,
                            maximum: 2,
                            visibility: ChoiceVisibilityDef::Public,
                            then: &EffectDef::Untap {
                                object: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                            },
                        }),
                    ))),
                ]),
            ),
            AbilityDef::activated_with_targets(
                "\u{2212}3: Put target nonland permanent into its owner's library third from the top.",
                &[CostDef::Loyalty(-3)],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                )],
                // Third from the top, so two cards have to be drawn before it comes back --
                // and unlike a bounce it answers a permanent that would rather be in a hand
                // or a graveyard.
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Library,
                    ZonePlacement::FromTop(3),
                ),
            ),
            AbilityDef::activated(
                "\u{2212}8: You get an emblem with \"Whenever you draw a card, exile target permanent an \
                 opponent controls.\"",
                &[CostDef::Loyalty(-8)],
                // One trigger per card drawn, which is what makes the emblem and the plus
                // the same card: every draw for the rest of the game eats a permanent.
                EffectDef::create_emblem("Teferi, Hero of Dominaria emblem", &[AbilityDef::triggered_with_targets(
                    "Whenever you draw a card, exile target permanent an opponent controls.",
                    TriggerEventDef::DrewCard(DrawEventMatcherDef::any(PlayerRelation::You)),
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                    )],
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                )]),
            ),
        ]),
);

// DOM 213 — Damping Sphere
// Audit: unsupported — Needs a static replacement changing a land ability producing two or more mana into exactly {C}.
pub(in crate::card::sets) static DAMPING_SPHERE: CardRecord = CardRecord::new(
    "Damping Sphere",
    "a5c7d16b-8f4e-42b9-be24-3cb091932d7c",
    "Adam Paquette",
    CardRules::unsupported(),
);

// DOM 217 — Helm of the Host
pub(in crate::card::sets) static HELM_OF_THE_HOST: CardRecord = CardRecord::new(
    "Helm of the Host",
    "1d65d20c-09e5-4139-838b-7e0e48eb2b2b",
    "Igor Kieryluk",
CardRules::new_artifact(mana_cost!("{4}"))
        .with_supertype(CardSupertype::Legendary)
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::triggered(
                "At the beginning of combat on your turn, create a token that's a copy of equipped creature, except the token isn't legendary. That token gains haste.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::BeginningOfCombat,
                    player: PlayerRelation::You,
                },
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Copy(&crate::card::TokenCopyDef {
                        object: &EffectRecipientDef::AttachedPermanent,
                        exceptions: CopyExceptionsDef::NONE.without_supertypes(&[CardSupertype::Legendary]),
                    }))
                    .with_created_tokens(CreatedTokensDef {
                        binding: ParentBinding,
                        then: &EffectDef::Apply {
                            recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                            effect: AppliedEffectDef::add_ability(&abilities::haste()),
                            duration: ResolvedEffectDurationDef::Permanent,
                        },
                    }),
                ),
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{5}"))], "Equip {5}"),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &KARN_SCION_OF_URZA,
    &ADAMANT_WILL,
    &KNIGHT_OF_GRACE,
    &LYRA_DAWNBRINGER,
    &TEMPEST_DJINN,
    &CAST_DOWN,
    &KNIGHT_OF_MALICE,
    &GHITU_LAVARUNNER,
    &GROW_FROM_THE_ASHES,
    &MULDROTHA_THE_GRAVETIDE,
    &TATYOVA_BENTHIC_DRUID,
    &TEFERI_HERO_OF_DOMINARIA,
    &DAMPING_SPHERE,
    &HELM_OF_THE_HOST,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
