//! Dominaria cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AppliedEffectDef, CardArt, CardRules, CardSet,
    CardSupertype, CardType, ChoiceVisibilityDef, ChooseDef, CopyExceptionsDef, CounterKind,
    CreatedTokensDef, DrawEventMatcherDef, EffectDef, EffectRecipientDef, InstalledTriggerDef,
    MoveObjectsDef, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef,
    ObjectSetDef, PlayerRefDef, PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef,
    RevealObjectsDef, TokenCharacteristics, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::ids::{Binding, ParentBinding, TargetIndex};
use crate::mana_cost;

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
    PrintingAnchor::scryfall("07a3d9e8-8597-498b-869c-cff79e0df516"),
    "Karn, Scion of Urza",
    CardArt::new("07a3d9e8-8597-498b-869c-cff79e0df516", "Chase Stone"),
    CardSet::Dominaria,
    // Colorless, so every deck can play him: a card every turn that the
    // other player picks, the pile of leftovers he can cash in later, and a
    // body that grows with the artifacts the deck is made of.
    CardRules::new_planeswalker(mana_cost!("{4}"), &["Karn"], 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "+1: Reveal the top two cards of your library. An opponent chooses one of them. Put that \
                 card into your hand and exile the other with a silver counter on it.",
                &[AbilityCostDef::Loyalty(1)],
                abilities::bind_top_cards_then(
                    PlayerRefDef::EffectController,
                    ValueDef::Constant(2),
                    &const {
                        EffectDef::Sequence(&[
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
                                    then: &const {
                                        EffectDef::Sequence(&[
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
                                        ])
                                    },
                                }),
                        ])
                    },
                ),
            ),
            AbilityDef::activated(
                "\u{2212}1: Put a card you own with a silver counter on it from exile into your hand.",
                &[AbilityCostDef::Loyalty(-1)],
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
                    then: &EffectDef::MoveToZone {
                        object: EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
                        zone: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                    },
                }),
            ),
            AbilityDef::activated(
                "\u{2212}2: Create a 0/0 colorless Construct artifact creature token with \"This token \
                 gets +1/+1 for each artifact you control.\"",
                &[AbilityCostDef::Loyalty(-2)],
                EffectDef::create_token(TokenCharacteristics::artifact_creature(&["Construct"], &[], 0, 0)
                        .with_abilities(&[AbilityDef::static_ability(
                            "This token gets +1/+1 for each artifact you control.",
                            EffectDef::StaticApply {
                                recipient: EffectRecipientDef::Source,
                                effect: AppliedEffectDef::modify_power_toughness(
                                    ValueDef::CountMatchingObjects(&ARTIFACTS_YOU_CONTROL),
                                    ValueDef::CountMatchingObjects(&ARTIFACTS_YOU_CONTROL),
                                ),
                            },
                        )])),
            ),
        ]),
);

// DOM 207 — Teferi, Hero of Dominaria
pub(in crate::card::sets) static TEFERI_HERO_OF_DOMINARIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5d10b752-d9cb-419d-a5c4-d4ee1acb655e"),
    "Teferi, Hero of Dominaria",
    crate::card::CardArt::new("5d10b752-d9cb-419d-a5c4-d4ee1acb655e", "Chris Rallis"),
    crate::card::CardSet::Dominaria,
    // Five mana that draws a card and leaves two lands up, so the turn he
    // lands is not the turn he costs you: the plus pays for the counterspell
    // held behind him.
    CardRules::new_planeswalker(mana_cost!("{3}{W}{U}"), &["Teferi"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "+1: Draw a card. At the beginning of the next end step, untap up to two lands.",
                &[AbilityCostDef::Loyalty(1)],
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
                &[AbilityCostDef::Loyalty(-3)],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                )],
                // Third from the top, so two cards have to be drawn before it comes back --
                // and unlike a bounce it answers a permanent that would rather be in a hand
                // or a graveyard.
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Library,
                    placement: ZonePlacement::FromTop(3),
                },
            ),
            AbilityDef::activated(
                "\u{2212}8: You get an emblem with \"Whenever you draw a card, exile target permanent an \
                 opponent controls.\"",
                &[AbilityCostDef::Loyalty(-8)],
                // One trigger per card drawn, which is what makes the emblem and the plus
                // the same card: every draw for the rest of the game eats a permanent.
                EffectDef::create_emblem("Teferi, Hero of Dominaria emblem", &[AbilityDef::triggered_with_targets(
                    "Whenever you draw a card, exile target permanent an opponent controls.",
                    TriggerEventDef::DrewCard(DrawEventMatcherDef::any(PlayerRelation::You)),
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                    )],
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Exile,
                        placement: ZonePlacement::Top,
                    },
                )]),
            ),
        ]),
);

// DOM 213 — Damping Sphere
// Audit: unsupported — Needs a static replacement changing a land ability producing two or more mana into exactly {C}.
pub(in crate::card::sets) static DAMPING_SPHERE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a5c7d16b-8f4e-42b9-be24-3cb091932d7c"),
    "Damping Sphere",
    CardArt::new("a5c7d16b-8f4e-42b9-be24-3cb091932d7c", "Adam Paquette"),
    CardSet::Dominaria,
    CardRules::unsupported(),
);

// DOM 217 — Helm of the Host
pub(in crate::card::sets) static HELM_OF_THE_HOST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1d65d20c-09e5-4139-838b-7e0e48eb2b2b"),
    "Helm of the Host",
    CardArt::new("1d65d20c-09e5-4139-838b-7e0e48eb2b2b", "Igor Kieryluk"),
    CardSet::Dominaria,
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
                EffectDef::create_token_from_copy(&crate::card::TokenCopyDef {
                    object: &EffectRecipientDef::AttachedPermanent,
                    exceptions: CopyExceptionsDef::NONE
                        .without_supertypes(&[CardSupertype::Legendary]),
                })
                .with_created_tokens(CreatedTokensDef {
                    binding: ParentBinding,
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                            ParentBinding,
                        )),
                        effect: AppliedEffectDef::add_ability(&abilities::haste()),
                        duration: ResolvedEffectDurationDef::Permanent,
                    },
                }),
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{5}"))], "Equip {5}"),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &KARN_SCION_OF_URZA,
    &TEFERI_HERO_OF_DOMINARIA,
    &DAMPING_SPHERE,
    &HELM_OF_THE_HOST,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
