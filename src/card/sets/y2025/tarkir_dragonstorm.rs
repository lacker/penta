//! Tarkir: Dragonstorm cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AlternateSpellKind;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardArt;
use crate::card::CardComposition;
use crate::card::CardEffectStatus;
use crate::card::CardPart;
use crate::card::CardRules;
use crate::card::CardStructure;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatedTokensDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ExilePlayDurationDef;
use crate::card::FreePlayDef;
use crate::card::FreePlayDurationDef;
use crate::card::ManaColor;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::PayOrDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayOptionDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::QuantifierDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SpellForm;
use crate::card::SpellResolutionDestinationDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::ids::CardPartId;
use crate::ids::ParentBinding;
use crate::ids::PlayOptionId;
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "TDM",
    slug: "tarkir-dragonstorm",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

/// The delayed half of mobilize: the tokens this attack made go away at the
/// next end step, and it has to be exactly those. By then nothing about the
/// board could tell them from the ones the last attack made, or from a
/// Warrior that arrived some other way, so they are bound as they are
/// created and this names the binding.
static MOBILIZE_SACRIFICE: EffectDef = EffectDef::InstallTrigger(
    crate::card::InstalledTriggerDef::once(&AbilityDef::triggered(
        "At the beginning of the next end step, sacrifice those tokens.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::Any,
        },
        EffectDef::sacrifice(EffectRecipientDef::objects(ObjectSetDef::Binding(
            ParentBinding,
        ))),
    )),
);

/// Mobilize N (CR 702.180a): "Whenever this creature attacks, create N tapped
/// and attacking 1/1 red Warrior creature tokens. Sacrifice them at the
/// beginning of the next end step."
///
/// Written out as the triggered ability it abbreviates. The caller supplies
/// the printed text because the reminder spells the number out in words.
#[must_use]
pub(in crate::card::sets) const fn mobilize(count: u16, text: &'static str) -> AbilityDef {
    AbilityDef::triggered(
        text,
        TriggerEventDef::attacks(ObjectPredicateDef::Source),
        EffectDef::CreateToken(
            CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Warrior"], &[ManaColor::Red], 1, 1).with_art(
                    crate::card::CardArt::new(
                        "7edc0515-a130-45a7-aa09-0e23bba41587",
                        "Forrest Imel",
                    ),
                ),
            ))
            .with_amount(count)
            .entering_tapped()
            .entering_attacking()
            .with_created_tokens(CreatedTokensDef {
                binding: ParentBinding,
                then: &MOBILIZE_SACRIFICE,
            }),
        ),
    )
}

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

static UGIN_EXILES_IT: EffectDef = EffectDef::move_to_zone(
    EffectRecipientDef::Target(TargetIndex::PRIMARY),
    ZoneKind::Exile,
    ZonePlacement::Top,
);

pub(in crate::card::sets) static UGIN_EYE_OF_THE_STORMS: CardRecord = CardRecord::new(
    "Ugin, Eye of the Storms",
    "64a5d494-efa1-446b-bebe-2ad36e154376",
    "Joshua Raphael",
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
                &[CostDef::Loyalty(2)],
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
                &[CostDef::Loyalty(0)],
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(3)),
            ),
            AbilityDef::activated(
                "\u{2212}11: Search your library for any number of colorless nonland cards, exile them, \
                 then shuffle. Until end of turn, you may cast those cards without paying their mana \
                 costs.",
                &[CostDef::Loyalty(-11)],
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
    "Descendant of Storms",
    "f632be90-9e7f-41f8-a52e-a2952354d730",
    "Lie Setiawan",
    // A one-mana 2/1 that attacks well early and has somewhere to put mana
    // late. Which half of endure you want changes with the board: the
    // counter makes the attack bigger, the Spirit makes the next one wider.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature attacks, you may pay {1}{W}. If you do, it endures 1.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::Mana(mana_cost!("{1}{W}"))], // "It endures 1": the counter or the Spirit, and the attacking body is
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
pub(in crate::card::sets) static FORTRESS_KIN_GUARD: CardRecord = CardRecord::new(
    "Fortress Kin-Guard",
    "b647a018-1d70-43a1-a265-928bcd863689",
    "Daneen Wilkerson",
    // Two mana for two bodies or one bigger one, and the choice is made
    // where it matters: a board that wants a blocker takes the Spirit.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Dog", "Soldier"], 1, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, it endures 1. (Put a +1/+1 counter on it or create a 1/1 \
             white Spirit creature token.)",
            EffectDef::Endure {
                object: EffectRecipientDef::Source,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// TDM 21 — Riling Dawnbreaker
const fn riling_dawnbreaker_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Dragon"], 3, 4).with_abilities(
        &const {
            [
                abilities::flying(),
                abilities::vigilance(),
                AbilityDef::triggered_with_targets(
                    "At the beginning of combat on your turn, another target creature you \
                     control gets +1/+0 until end of turn.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::BeginningOfCombat,
                        player: PlayerRelation::You,
                    },
                    // "Another": the Dragon is already the biggest attacker,
                    // so the bonus always goes to something beside it.
                    &const {
                        [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                ]),
                                zones: &[ZoneKind::Battlefield],
                                controller: Some(PlayerRelation::You),
                                owner: None,
                            },
                        )]
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
            ]
        },
    )
}

fn riling_dawnbreaker_composition() -> CardComposition {
    let dragon = riling_dawnbreaker_rules();
    let roar = const {
        CardRules::new_sorcery(mana_cost!("{1}{W}"))
            .with_subtypes(&["Omen"])
            .with_ability(
                AbilityDef::spell(
                    "Create a 2/2 white Soldier creature token.",
                    EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::creature(&["Soldier"], &[ManaColor::White], 2, 2),
                    ))),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::LibraryShuffled),
            )
    };
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Riling Dawnbreaker", dragon),
            CardPart::new(CardPartId(1), "Signaling Roar", roar),
        ],
        structure: CardStructure::AlternateSpell {
            main: CardPartId::PRIMARY,
            alternate: CardPartId(1),
            kind: AlternateSpellKind::Omen,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Riling Dawnbreaker",
                SpellForm::Part(CardPartId::PRIMARY),
                dragon
                    .mana_cost()
                    .expect("the Dragon has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Signaling Roar",
                SpellForm::Part(CardPartId(1)),
                roar.mana_cost()
                    .expect("Signaling Roar has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

pub(in crate::card::sets) static RILING_DAWNBREAKER: CardRecord = CardRecord::new(
    "Riling Dawnbreaker",
    "312f7072-3bf8-449f-bfb7-93727ef26c66",
    "Tuan Duong Chu",
    // A body early and a bigger one later out of one card, and the Dragon
    // pushes whatever the Omen left behind.
    riling_dawnbreaker_rules(),
)
.with_composition(riling_dawnbreaker_composition);

// TDM 23 — Salt Road Packbeast
pub(in crate::card::sets) static SALT_ROAD_PACKBEAST: CardRecord = CardRecord::new(
    "Salt Road Packbeast",
    "98d548c9-42bc-4155-8211-0aea801c3724",
    "Ben Wootten",
    // Six mana printed, but a board that has already gone wide pays a
    // fraction of it, and the card it draws makes the turn no worse.
    CardRules::new_creature(mana_cost!("{5}{W}"), &["Beast"], 4, 3).with_abilities(&[
        AbilityDef::static_ability(
            "Affinity for creatures (This spell costs {1} less to cast for each creature you \
             control.)",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                &ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
            )),
        )
        // Read while the card is in hand: this prices the spell, so it has
        // to apply from the zone the spell is cast out of.
        .with_source_zones(&[ZoneKind::Hand]),
        abilities::enters_trigger(
            "When this creature enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// TDM 33 — Voice of Victory
pub(in crate::card::sets) static VOICE_OF_VICTORY: CardRecord = CardRecord::new(
    "Voice of Victory",
    "ec3de5f4-bb55-4ab9-995f-f3e0dc22c1bb",
    "Joshua Cairos",
// Two mana that adds two power to every attack and turns off every
    // instant your opponent was holding for the turn you attack.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Bard"], 1, 3)
        .with_abilities(&[
            mobilize(
                2,
                "Mobilize 2 (Whenever this creature attacks, create two tapped and attacking 1/1 red Warrior \
                 creature tokens. Sacrifice them at the beginning of the next end step.)",
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
pub(in crate::card::sets) static SEIZE_OPPORTUNITY: CardRecord = CardRecord::new(
    "Seize Opportunity",
    "f7818d28-b9a5-4341-9adc-666070b8878d",
    "Josiah \"Jo\" Cameron",
    // Cards when the board is empty, reach when it is not. Neither half is
    // worth three mana alone; being able to pick at instant speed is.
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Exile the top two cards of your library. Until the end of your next turn, you \
                 may play those cards.",
                EffectDef::ExileTopOfLibraryToPlay {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                    free: false,
                    face_down: false,
                    duration: ExilePlayDurationDef::UntilEndOfYourNextTurn,
                    spend_any_color: false,
                    play_condition: None,
                    cast_only: false,
                },
            ),
            AbilityDef::spell_with_targets(
                "Up to two target creatures each get +2/+1 until end of turn.",
                // "Up to two" and not "two": cast for this half with a single
                // creature on the board, it still resolves on that one.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    2,
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )),
);

// TDM 120 — Shock Brigade
pub(in crate::card::sets) static SHOCK_BRIGADE: CardRecord = CardRecord::new(
    "Shock Brigade",
    "66940466-8e9d-4a85-bfb0-e92189b7a121",
    "Fajareka Setiawan",
    // A 1/3 body nobody blocks profitably, attacking as two creatures. The
    // Warrior is gone by the end step, so what mobilize buys is damage on
    // this attack rather than a board.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Soldier"], 1, 3).with_abilities(&[
        abilities::menace(),
        mobilize(
            1,
            "Mobilize 1 (Whenever this creature attacks, create a tapped and attacking 1/1 red \
             Warrior creature token. Sacrifice it at the beginning of the next end step.)",
        ),
    ]),
);

// TDM 127 — Tersa Lightshatter
pub(in crate::card::sets) static TERSA_LIGHTSHATTER: CardRecord = CardRecord::new(
    "Tersa Lightshatter",
    "99e96b34-b1c4-4647-a38e-2cf1aedaaace",
    "Olivier Bernard",
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
    "Ainok Wayfarer",
    "57695a9b-8f72-4ccc-a946-5d5037b09b8f",
    "Filipe Pagliuso",
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
                        EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                            ZoneKind::Hand,
                            ZonePlacement::Top,
                        ),
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
    "Champion of Dusan",
    "c51dcdab-38ee-4804-8859-09adc353c182",
    "Bastien L. Deharme",
// A 4/2 trades early and then hands its trample to something better
    // from the graveyard, which is the whole arc of the card.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Warrior"], 4, 2).with_abilities(&[
        abilities::trample(),
        AbilityDef::activated_with_targets(
            "Renew — {1}{G}, Exile this card from your graveyard: Put a +1/+1 counter and a trample counter on target creature. Activate only as a sorcery.",
            &[
                CostDef::Mana(mana_cost!("{1}{G}")),
                CostDef::ExileSource,
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
const fn sagu_wildling_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Dragon"], 3, 3).with_abilities(
        &const {
            [
                abilities::flying(),
                abilities::enters_trigger(
                    "When this creature enters, you gain 3 life.",
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                ),
            ]
        },
    )
}

fn sagu_wildling_composition() -> CardComposition {
    let wildling = sagu_wildling_rules();
    let seek = const {
        CardRules::new_sorcery(mana_cost!("{G}"))
            .with_subtypes(&["Omen"])
            .with_ability(
                AbilityDef::spell(
                    "Search your library for a basic land card, reveal it, put it into your hand, \
                     then shuffle.",
                    EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ]),
                        minimum: 0,
                        maximum: ValueDef::Constant(1),
                        reveal: true,
                        destination: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                        shuffle: true,
                        enters_tapped: false,
                        attachment: None,
                        binding: None,
                        then: None,
                    },
                )
                // An Omen shuffles itself back in rather than exiling, which
                // is the whole difference from an Adventure: the creature is
                // drawn again later instead of waiting in exile.
                .with_resolution_destination(SpellResolutionDestinationDef::LibraryShuffled),
            )
    };
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Sagu Wildling", wildling),
            CardPart::new(CardPartId(1), "Roost Seek", seek),
        ],
        structure: CardStructure::AlternateSpell {
            main: CardPartId::PRIMARY,
            alternate: CardPartId(1),
            kind: AlternateSpellKind::Omen,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Sagu Wildling",
                SpellForm::Part(CardPartId::PRIMARY),
                wildling
                    .mana_cost()
                    .expect("the Dragon has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Roost Seek",
                SpellForm::Part(CardPartId(1)),
                seek.mana_cost()
                    .expect("Roost Seek has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

pub(in crate::card::sets) static SAGU_WILDLING: CardRecord = CardRecord::new(
    "Sagu Wildling",
    "d8b43b00-f4d1-436c-bf3f-6d414cd4ce38",
    "Gaboleps",
    // A land on turn one and a five-drop later out of the same card, which
    // is what an Omen buys over a plain fetch spell.
    sagu_wildling_rules(),
)
.with_composition(sagu_wildling_composition);

// TDM 343 — Cori-Steel Cutter
pub(in crate::card::sets) static CORI_STEEL_CUTTER: CardRecord = CardRecord::new(
    "Cori-Steel Cutter",
    "470dd3c8-07c9-42ef-aa9e-3c73b23607ff",
    "Tomas Duchek",
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
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::creature(&["Monk"], &[ManaColor::White], 1, 1)
                            .with_abilities(&[abilities::prowess()])
                            .with_art(CardArt::new(
                                "633d2d10-def7-426f-8496-ed6b45684299",
                                "Elizabeth Peiró",
                            )),
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
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{1}{R}"))], "Equip {1}{R}"),
        ]),
);

static ELSPETH_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// TDM 398 — Elspeth, Storm Slayer
pub(in crate::card::sets) static ELSPETH_STORM_SLAYER: CardRecord = CardRecord::new(
    "Elspeth, Storm Slayer",
    "1fdf9438-fd5f-4638-8f41-dae35ae8f257",
    "Jeremy Wilson",
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
                &[CostDef::Loyalty(1)],
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Soldier"], &[ManaColor::White], 1, 1),
                ))),
            ),
            AbilityDef::activated(
                "0: Put a +1/+1 counter on each creature you control. Those creatures gain flying until \
                 your next turn.",
                &[CostDef::Loyalty(0)],
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
                &[CostDef::Loyalty(-3)],
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
                EffectDef::destroy_target(TargetIndex::PRIMARY),
            ),
        ]),
);

// TDM 409 — Ugin, Eye of the Storms (alternate printing)
const UGIN_EYE_OF_THE_STORMS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UGIN_EYE_OF_THE_STORMS,
    1,
    "2e7cb37b-3ab5-42d0-860a-0c0760924850",
    "Joshua Raphael",
);

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

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[UGIN_EYE_OF_THE_STORMS_ALTERNATE_1];
