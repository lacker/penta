//! Throne of Eldraine cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::PlayOptionDef;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityPredicateDef, AbilityTargetDef, AbilityTargetPredicate,
    ActivationTimingDef, AlternateSpellKind, AlternativeCastKindDef, AppliedEffectDef,
    AppliedRuleDef, BasicLandType, BattlefieldEntryModificationDef, BlockRestrictionDef,
    BlockRestrictionMatchDef, BlockRestrictionSubjectDef, CardArt, CardComposition,
    CardEffectStatus, CardPart, CardRules, CardSet, CardStructure, CardSupertype, CardType,
    CardTypeSet, ColorSet, ComparisonDef, ConditionDef, ControlDurationDef, CounterKind,
    CreatureTypeSetDef, EffectDef, EffectRecipientDef, ExilePlayConditionDef, ExilePlayDurationDef,
    KeywordAbility, ManaColor, ObjectCountConditionDef, ObjectPredicateDef, ObjectQueryDef,
    ObjectSetDef, PlayerRefDef, PlayerRelation, ReplacementEffectDef, ResolvedEffectDurationDef,
    SpellForm, SpellResolutionDestinationDef, TriggerConditionDef, TriggerEventDef,
    ValueComparisonDef, ValueDef, ZoneKind, ZonePlacement, abilities, tokens,
};
use crate::ids::ParentBinding;
use crate::{CardPartId, PlayOptionId, TargetIndex, mana_cost};

// ELD 5 — Ardenvale Tactician
const fn ardenvale_tactician_rules() -> CardRules {
    CardRules::new_creature(
        mana_cost!("{1}{W}{W}"),
        &const { ["Human", "Knight"] },
        2,
        3,
    )
    .with_ability(abilities::flying())
}

fn ardenvale_tactician_composition() -> CardComposition {
    let knight = ardenvale_tactician_rules();
    let swoop = const {
        CardRules::new_instant(mana_cost!("{1}{W}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Tap up to two target creatures.",
                    // One slot holding up to two, so the Adventure is still
                    // castable with a single creature on the board.
                    &const {
                        [AbilityTargetDef::up_to(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::HasType(CardType::Creature),
                                zones: &const { [ZoneKind::Battlefield] },
                                controller: None,
                                owner: None,
                            },
                            2,
                        )]
                    },
                    EffectDef::Tap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            )
    };
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Ardenvale Tactician", knight),
            CardPart::new(CardPartId(1), "Dizzying Swoop", swoop),
        ],
        structure: CardStructure::AlternateSpell {
            main: CardPartId::PRIMARY,
            alternate: CardPartId(1),
            kind: AlternateSpellKind::Adventure,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Ardenvale Tactician",
                SpellForm::Part(CardPartId::PRIMARY),
                knight
                    .mana_cost()
                    .expect("the Knight has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Dizzying Swoop",
                SpellForm::Part(CardPartId(1)),
                swoop
                    .mana_cost()
                    .expect("Dizzying Swoop has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

pub(in crate::card::sets) static ARDENVALE_TACTICIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c7d5e394-8e41-442e-ae97-a478a61e1b9d"),
    "Ardenvale Tactician",
    CardArt::new("c7d5e394-8e41-442e-ae97-a478a61e1b9d", "Jason Rainville"),
    CardSet::ThroneOfEldraine,
    // Clear two blockers now and cast the flier later: one card that buys a
    // turn of tempo and then a body.
    ardenvale_tactician_rules(),
)
.with_composition(ardenvale_tactician_composition);

// ELD 11 — Faerie Guidemother
const fn faerie_guidemother_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{W}"), &const { ["Faerie"] }, 1, 1)
        .with_ability(abilities::flying())
}

fn faerie_guidemother_composition() -> CardComposition {
    let faerie = faerie_guidemother_rules();
    let gift = const {
        CardRules::new_sorcery(mana_cost!("{1}{W}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Target creature gets +2/+1 and gains flying until end of turn.",
                    &const {
                        [AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )]
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::Composite(
                            &const {
                                [
                                    AppliedEffectDef::modify_power_toughness(
                                        ValueDef::Constant(2),
                                        ValueDef::Constant(1),
                                    ),
                                    AppliedEffectDef::add_ability(&const { abilities::flying() }),
                                ]
                            },
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                )
                // Exiled on resolution rather than put in the graveyard,
                // which is what makes the Faerie castable afterwards.
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            )
    };
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Faerie Guidemother", faerie),
            CardPart::new(CardPartId(1), "Gift of the Fae", gift),
        ],
        structure: CardStructure::AlternateSpell {
            main: CardPartId::PRIMARY,
            alternate: CardPartId(1),
            kind: AlternateSpellKind::Adventure,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Faerie Guidemother",
                SpellForm::Part(CardPartId::PRIMARY),
                faerie
                    .mana_cost()
                    .expect("the Faerie has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Gift of the Fae",
                SpellForm::Part(CardPartId(1)),
                gift.mana_cost()
                    .expect("Gift of the Fae has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

pub(in crate::card::sets) static FAERIE_GUIDEMOTHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e8bbece8-9620-44d9-b991-350fe952538a"),
    "Faerie Guidemother",
    CardArt::new("e8bbece8-9620-44d9-b991-350fe952538a", "Mila Pesic"),
    CardSet::ThroneOfEldraine,
    // A combat trick that leaves a flier behind, which is the whole appeal
    // of the cheap end of the Adventure cycle.
    faerie_guidemother_rules(),
)
.with_composition(faerie_guidemother_composition);

// ELD 39 — Brazen Borrower
const fn brazen_borrower_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Faerie", "Rogue"], 3, 1).with_abilities(
        &const {
            [
                abilities::flash(),
                abilities::flying(),
                AbilityDef::static_ability(
                    "This creature can block only creatures with flying.",
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        // "Can block only creatures with flying" is the price of a 3/1 flier with
                        // flash: it answers what is in the air and nothing on the ground.
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                            ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                        )),
                    },
                ),
            ]
        },
    )
}

fn brazen_borrower_composition() -> CardComposition {
    let borrower = brazen_borrower_rules();
    let theft = const {
        CardRules::new_instant(mana_cost!("{1}{U}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Return target nonland permanent an opponent controls to its owner's hand.",
                    &const {
                        [AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                    CardType::Land,
                                )),
                                zones: &const { [ZoneKind::Battlefield] },
                                controller: Some(PlayerRelation::Opponent),
                                owner: None,
                            },
                        )]
                    },
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                    },
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            )
    };
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Brazen Borrower", borrower),
            CardPart::new(CardPartId(1), "Petty Theft", theft),
        ],
        structure: CardStructure::AlternateSpell {
            main: CardPartId::PRIMARY,
            alternate: CardPartId(1),
            kind: AlternateSpellKind::Adventure,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Brazen Borrower",
                SpellForm::Part(CardPartId::PRIMARY),
                borrower
                    .mana_cost()
                    .expect("the Faerie has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Petty Theft",
                SpellForm::Part(CardPartId(1)),
                theft
                    .mana_cost()
                    .expect("Petty Theft has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

pub(in crate::card::sets) static BRAZEN_BORROWER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c2089ec9-0665-448f-bfe9-d181de127814"),
    "Brazen Borrower",
    CardArt::new("c2089ec9-0665-448f-bfe9-d181de127814", "Eric Deschamps"),
    CardSet::ThroneOfEldraine,
    // Bounce something at the end of their turn, then flash in the body it
    // came back on: one card that answers a threat and becomes one.
    brazen_borrower_rules(),
)
.with_composition(brazen_borrower_composition);

// ELD 110 — Wishclaw Talisman
pub(in crate::card::sets) static WISHCLAW_TALISMAN: CardRecord = CardRecord::new_with_legacy_id(
    2166,
    "Wishclaw Talisman",
    CardArt::new("07c17b01-ee5d-491a-8403-b3f819b778c4", "Daarken"),
    CardSet::ThroneOfEldraine,
    // Two mana for any card in the deck, and the price is handing the rest of
    // the artifact to the person it will be used against. The decks that play
    // it intend to win before that matters.
    CardRules::new_artifact(mana_cost!("{1}{B}")).with_abilities(&[
        AbilityDef::as_enters(
            "This artifact enters with three wish counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::named("wish"),
                    amount: 3,
                },
            ),
        ),
        AbilityDef::activated(
            "{1}, {T}, Remove a wish counter from this artifact: Search your library for a card, put it into your hand, then shuffle. An opponent gains control of this artifact. Activate only during your turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("wish"),
                    amount: 1,
                },
            ],
            // The tutor and the handover are one clause resolving in order, so the card
            // is in hand before the artifact changes sides -- and the opponent inherits
            // two counters they may spend on their own turn.
            EffectDef::Sequence(&[
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::Any,
                    minimum: 1,
                    maximum: ValueDef::Constant(1),
                    reveal: false,
                    destination: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
                EffectDef::GainControl {
                    object: EffectRecipientDef::Source,
                    controller: PlayerRefDef::Opponent,
                    // Nothing holds the change and no cleanup ends it: the artifact is
                    // theirs from here (CR 611.2b).
                    duration: ControlDurationDef::Indefinitely,
                },
            ]),
        )
        .with_activation_timing(ActivationTimingDef::YourTurn),
    ]),
);

// ELD 115 — Bonecrusher Giant
const fn bonecrusher_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Giant"], 4, 3)
        // The punishment half. Answering the Giant with a removal spell costs two
        // life whether or not the spell works, which is what makes it awkward to
        // answer at all.
        .with_ability(AbilityDef::triggered(
            "Whenever this creature becomes the target of a spell, this creature deals 2 damage to that spell's controller.",
            TriggerEventDef::becomes_targeted(ObjectPredicateDef::Spell),
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(2),
            },
        ))
}

fn bonecrusher_composition() -> CardComposition {
    let giant = bonecrusher_rules();
    let stomp = const {
        CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(
            AbilityDef::spell_with_targets(
                "Damage can't be prevented this turn.\nStomp deals 2 damage to any target.",
                &const {
                    [AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::AnyTarget,
                    )]
                },
                // The two sentences are one clause resolving in order, and the order is what
                // the card is for: prevention is off before the damage arrives, so a
                // protection that would have stopped it does not.
                EffectDef::Sequence(
                    &const {
                        [
                            EffectDef::DamageCannotBePreventedThisTurn,
                            EffectDef::DealDamage {
                                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                amount: ValueDef::Constant(2),
                            },
                        ]
                    },
                ),
            )
            .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
        )
    };
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Bonecrusher Giant", giant),
            CardPart::new(CardPartId(1), "Stomp", stomp),
        ],
        structure: CardStructure::AlternateSpell {
            main: CardPartId::PRIMARY,
            alternate: CardPartId(1),
            kind: AlternateSpellKind::Adventure,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Bonecrusher Giant",
                SpellForm::Part(CardPartId::PRIMARY),
                giant
                    .mana_cost()
                    .expect("the Giant has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Stomp",
                SpellForm::Part(CardPartId(1)),
                stomp.mana_cost().expect("Stomp has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

pub(in crate::card::sets) static BONECRUSHER_GIANT: CardRecord = CardRecord::new_with_legacy_id(
    2167,
    "Bonecrusher Giant",
    CardArt::new(
        "09fd2d9c-1793-4beb-a3fb-7a869f660cd4",
        "Victor Adame Minguez",
    ),
    CardSet::ThroneOfEldraine,
    bonecrusher_rules(),
)
.with_composition(bonecrusher_composition);

// ELD 122 — Embereth Shieldbreaker
/// The adventure half. Answering an artifact for one red leaves the body
/// waiting in exile, which is the whole bargain of the mechanic.
fn battle_display_rules() -> CardRules {
    CardRules::new_sorcery(mana_cost!("{R}"))
        .with_subtypes(&["Adventure"])
        .with_ability(
            AbilityDef::spell_with_targets(
                "Destroy target artifact.",
                &const {
                    [AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Artifact),
                    )]
                },
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            )
            .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
        )
}

const fn embereth_shieldbreaker_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Knight"], 2, 1)
}

fn embereth_shieldbreaker_composition() -> CardComposition {
    let knight = embereth_shieldbreaker_rules();
    let display = battle_display_rules();
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Embereth Shieldbreaker", knight),
            CardPart::new(CardPartId(1), "Battle Display", display),
        ],
        structure: CardStructure::AlternateSpell {
            main: CardPartId::PRIMARY,
            alternate: CardPartId(1),
            kind: AlternateSpellKind::Adventure,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Embereth Shieldbreaker",
                SpellForm::Part(CardPartId::PRIMARY),
                knight
                    .mana_cost()
                    .expect("the Knight has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Battle Display",
                SpellForm::Part(CardPartId(1)),
                display
                    .mana_cost()
                    .expect("Battle Display has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

pub(in crate::card::sets) static EMBERETH_SHIELDBREAKER: CardRecord =
    CardRecord::new_with_legacy_id(
        2208,
        "Embereth Shieldbreaker",
        CardArt::new("6cc73d16-5ed7-4104-91f6-0997a2080e2e", "Randy Vargas"),
        CardSet::ThroneOfEldraine,
        embereth_shieldbreaker_rules(),
    )
    .with_composition(embereth_shieldbreaker_composition);

// ELD 137 — Rimrock Knight
const fn rimrock_knight_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{1}{R}"), &const { ["Dwarf", "Knight"] }, 3, 1)
        .with_ability(AbilityDef::static_ability(
            "This creature can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            },
        ))
}

fn rimrock_knight_composition() -> CardComposition {
    let knight = rimrock_knight_rules();
    let rush = const {
        CardRules::new_instant(mana_cost!("{R}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Target creature gets +2/+0 until end of turn.",
                    &const {
                        [AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )]
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(0),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            )
    };
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Rimrock Knight", knight),
            CardPart::new(CardPartId(1), "Boulder Rush", rush),
        ],
        structure: CardStructure::AlternateSpell {
            main: CardPartId::PRIMARY,
            alternate: CardPartId(1),
            kind: AlternateSpellKind::Adventure,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Rimrock Knight",
                SpellForm::Part(CardPartId::PRIMARY),
                knight
                    .mana_cost()
                    .expect("the Knight has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Boulder Rush",
                SpellForm::Part(CardPartId(1)),
                rush.mana_cost()
                    .expect("Boulder Rush has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

pub(in crate::card::sets) static RIMROCK_KNIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a3d13d84-01e4-4429-93db-e5afff811527"),
    "Rimrock Knight",
    CardArt::new("a3d13d84-01e4-4429-93db-e5afff811527", "Chris Rallis"),
    CardSet::ThroneOfEldraine,
    // Three power for two that only ever attacks, and a trick that turns a
    // stalled board into damage: an aggressive deck wants both halves.
    rimrock_knight_rules(),
)
.with_composition(rimrock_knight_composition);

// ELD 138 — Robber of the Rich
pub(in crate::card::sets) static ROBBER_OF_THE_RICH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0ecbe097-ba51-42e5-957c-382eb66c08f0"),
    "Robber of the Rich",
    CardArt::new("0ecbe097-ba51-42e5-957c-382eb66c08f0", "Paul Scott Canavan"),
    CardSet::ThroneOfEldraine,
    // Two mana for a hasty reaching body that also takes a card off the top
    // of whoever is holding more, and hands it back to you on any turn your
    // Rogues have been out attacking.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Archer", "Rogue"], 2, 2)
        .with_abilities(&[
            abilities::reach(),
            abilities::haste(),
            AbilityDef::triggered_if(
                "Whenever this creature attacks, if defending player has more cards in hand than \
                 you, exile the top card of their library. During any turn you attacked with a \
                 Rogue, you may cast that card and you may spend mana as though it were mana of \
                 any color to cast that spell.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                &// "If defending player has more cards in hand than you", which is two hand
                    // sizes compared rather than either measured: a hand above nothing is the
                    // whole of it.
                    TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CardsInHandAbove {
                            player: PlayerRelation::Opponent,
                            threshold: 0,
                        },
                        comparison: ComparisonDef::Greater,
                        right: ValueDef::CardsInHandAbove {
                            player: PlayerRelation::You,
                            threshold: 0,
                        },
                    }),
                EffectDef::ExileTopOfLibraryToPlay {
                    player: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(1),
                    free: false,
                    face_down: false,
                    duration: ExilePlayDurationDef::WhileExiled,
                    spend_any_color: true,
                    play_condition: Some(ExilePlayConditionDef::AttackedWithSubtypeThisTurn(
                        "Rogue",
                    )),
                    cast_only: true,
                },
            ),
        ]),
);

// ELD 169 — Once Upon a Time
pub(in crate::card::sets) static ONCE_UPON_A_TIME: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4034e5ba-9974-43e3-bde7-8d9b4586c3a4"),
    "Once Upon a Time",
    CardArt::new("4034e5ba-9974-43e3-bde7-8d9b4586c3a4", "Matt Stewart"),
    CardSet::ThroneOfEldraine,
    // A free spell that finds a land or a creature, which is why every green
    // deck played it and why it is banned in the format it was printed for.
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some(
                "If this spell is the first spell you've cast this game, you may cast it without \
                 paying its mana cost.",
            ),
            EffectDef::None,
        )
        .with_alternative_condition(
            &// The spell asking is counted as it goes on the stack, so a spell that is
            // the first one asks about a tally still standing at zero.
            TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::SpellsCastThisGame(PlayerRelation::You),
                comparison: ComparisonDef::Equal,
                right: ValueDef::Constant(0),
            }),
        ),
        AbilityDef::spell(
            "Look at the top five cards of your library. You may reveal a creature or land card \
             from among them and put it into your hand. Put the rest on the bottom of your \
             library in a random order.",
            abilities::look_at_top_cards_reveal_choice_to_hand_rest_random_bottom(
                ValueDef::Constant(5),
                // "You may reveal a creature or land card from among them": the two types
                // the deck casting this on turn one is actually short of.
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
                0,
                1,
            ),
        ),
    ]),
);

// ELD 197 — Oko, Thief of Crowns
pub(in crate::card::sets) static OKO_THIEF_OF_CROWNS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3462a3d0-5552-49fa-9eb7-100960c55891"),
    "Oko, Thief of Crowns",
    CardArt::new("3462a3d0-5552-49fa-9eb7-100960c55891", "Yongjae Choi"),
    CardSet::ThroneOfEldraine,
    // Three mana that answers a permanent every turn and gains loyalty for
    // doing it. What it answers with is a 3/3 Elk, which is the joke and the
    // reason it was banned everywhere.
    CardRules::new_planeswalker(mana_cost!("{1}{G}{U}"), &["Oko"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "+2: Create a Food token.",
                &[AbilityCostDef::Loyalty(2)],
                EffectDef::create_token(tokens::food()).with_art(CardArt::new(
                    "4a029bdc-92e3-4d85-8af5-e33429a5f017",
                    "L J Koh",
                )),
            ),
            AbilityDef::activated_with_targets(
                "+1: Target artifact or creature loses all abilities and becomes a green Elk creature \
                 with base power and toughness 3/3.",
                &[AbilityCostDef::Loyalty(1)],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    // "Loses all abilities and becomes a green Elk creature with base power and
                    // toughness 3/3." Five operations in one clause, and no duration at all:
                    // what Oko does to a Mox is permanent.
                    //
                    // "Becomes a green Elk creature" replaces the type line rather than
                    // adding to it: the Elk "loses any other card types it has (such as
                    // artifact)", which is why a Mox that has been elked stops answering
                    // to anything that reads artifacts. Supertypes are untouched.
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::set_card_types(CardTypeSet::single(CardType::Creature)),
                        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Elk"])),
                        AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
                        AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Green])),
                        AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(3), ValueDef::Constant(3)),
                    ]),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            ),
            AbilityDef::activated_with_targets(
                "−5: Exchange control of target artifact or creature you control and target creature an \
                 opponent controls with power 3 or less.",
                &[AbilityCostDef::Loyalty(-5)],
                // The exchange names one of each: something of yours, and something small
                // of theirs. An Elk the +1 just made is exactly the kind of thing the
                // first slot is for.
                &[
                    AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    }),
                    AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                        // "Power 3 or less" said the way the vocabulary has it: a creature
                        // always has a power, so failing to reach four is having at most
                        // three.
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::PowerAtLeast(4)),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    }),
                ],
                EffectDef::ExchangeControl {
                    first: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    second: EffectRecipientDef::Target(TargetIndex(1)),
                    otherwise: None,
                },
            ),
        ]),
);

// ELD 219 — Gingerbrute
pub(in crate::card::sets) static GINGERBRUTE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f55fe038-c903-4d92-b689-72dd6d041a91"),
    "Gingerbrute",
    CardArt::new("f55fe038-c903-4d92-b689-72dd6d041a91", "Andrea Radeck"),
    CardSet::ThroneOfEldraine,
    // One mana for a hasty evasive body that is also a Food, which is why
    // an artifact deck plays it over a bigger one-drop.
    CardRules::new_artifact_creature(mana_cost!("{1}"), &["Food", "Golem"], 1, 1).with_abilities(
        &[
            abilities::haste(),
            AbilityDef::activated(
                "{1}: This creature can't be blocked this turn except by creatures with haste.",
                &[AbilityCostDef::Mana(mana_cost!("{1}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    // A pairing restriction on the attacker: every prospective
                    // blocker outside the predicate is barred, which is what
                    // "except by" says.
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                        BlockRestrictionDef::prohibit(
                            BlockRestrictionSubjectDef::Attacker,
                            BlockRestrictionMatchDef::Except(ObjectPredicateDef::HasKeyword(
                                KeywordAbility::Haste,
                            )),
                        ),
                    )),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::activated(
                "{2}, {T}, Sacrifice this creature: You gain 3 life.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{2}")),
                    AbilityCostDef::TapSource,
                    AbilityCostDef::SacrificeSource,
                ],
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            ),
        ],
    ),
);

// ELD 235 — Stonecoil Serpent
pub(in crate::card::sets) static STONECOIL_SERPENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b34bf7fd-9fe3-43e2-8cfe-7ce7cff08afe"),
    "Stonecoil Serpent",
    CardArt::new("b34bf7fd-9fe3-43e2-8cfe-7ce7cff08afe", "Mark Poole"),
    CardSet::ThroneOfEldraine,
    CardRules::new_artifact_creature(mana_cost!("{X}"), &["Snake"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with X +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCastXCounters {
                    kind: CounterKind::PlusOnePlusOne,
                },
            ),
        ),
        abilities::reach(),
        abilities::trample(),
        abilities::protection_from_multicolored(),
    ]),
);

// ELD 247 — Mystic Sanctuary
pub(in crate::card::sets) static MYSTIC_SANCTUARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("170e792c-80d5-4775-ad95-37614574ab84"),
    "Mystic Sanctuary",
    CardArt::new("170e792c-80d5-4775-ad95-37614574ab84", "Randy Vargas"),
    CardSet::ThroneOfEldraine,
    // An Island, so its mana ability is the subtype's rather than a printed
    // clause -- which is why the card prints that line in parentheses.
    //
    // Written out rather than shared with Witch's Cottage below: the count
    // it asks about names its own basic type, and a condition holding that
    // as a parameter behind a reference cannot be promoted to `'static`.
    CardRules::new_land(&["Island"]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped unless you control three or more other Islands.",
            ReplacementEffectDef::Conditional {
                condition: ConditionDef::ObjectCount(
                    &const {
                        ObjectCountConditionDef {
                            query: ObjectQueryDef::matching(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasAnyBasicLandType(&[
                                        BasicLandType::Island,
                                    ]),
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                            comparison: ComparisonDef::GreaterOrEqual,
                            amount: 3,
                        }
                    },
                ),
                if_true: &[],
                if_false: &const {
                    [ReplacementEffectDef::ModifyBattlefieldEntry(
                        BattlefieldEntryModificationDef::Tapped,
                    )]
                },
            },
        ),
        AbilityDef::triggered_if_with_targets(
            "When this land enters untapped, you may put target instant or sorcery card from \
             your graveyard on top of your library.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            // The same entry the replacement above just decided about: it
            // fires only on the turns the Island count let it come in ready.
            &TriggerConditionDef::SourceUntapped,
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
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &const {
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Library,
                        placement: ZonePlacement::Top,
                    }
                },
            },
        ),
    ]),
);

// ELD 249 — Witch's Cottage
pub(in crate::card::sets) static WITCH_S_COTTAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b87891cd-b457-4dff-8d18-a7eaf6748fc6"),
    "Witch's Cottage",
    CardArt::new("b87891cd-b457-4dff-8d18-a7eaf6748fc6", "Gabor Szikszai"),
    CardSet::ThroneOfEldraine,
    // A Swamp, so its mana ability is the subtype's rather than a printed
    // clause -- which is why the card prints that line in parentheses.
    CardRules::new_land(&["Swamp"]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped unless you control three or more other Swamps.",
            ReplacementEffectDef::Conditional {
                condition: ConditionDef::ObjectCount(&const {
                    ObjectCountConditionDef {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 3,
                    }
                }),
                if_true: &[],
                if_false: &const {
                    [ReplacementEffectDef::ModifyBattlefieldEntry(
                        BattlefieldEntryModificationDef::Tapped,
                    )]
                },
            },
        ),
        AbilityDef::triggered_if_with_targets(
            "When this land enters untapped, you may put target creature card from your graveyard on top of your library.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            // The same entry the replacement above just decided about: it
            // fires only on the turns the Swamp count let it come in ready.
            &TriggerConditionDef::SourceUntapped,
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &const {
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Library,
                        placement: ZonePlacement::Top,
                    }
                },
            },
        ),
    ]),
);

// ELD 342 — Emry, Lurker of the Loch
pub(in crate::card::sets) static EMRY_LURKER_OF_THE_LOCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("157f343d-8583-4827-a77d-d916e6a5caa1"),
    "Emry, Lurker of the Loch",
    CardArt::new("157f343d-8583-4827-a77d-d916e6a5caa1", "Livia Prima"),
    CardSet::ThroneOfEldraine,
    // A one-mana 1/2 on any board with two artifacts, and the mill she
    // arrives with is where she finds what to recast.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk", "Wizard"], 1, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            // Affinity is a discount the card prints about itself, read from hand
            // where the spell is being paid for rather than off the battlefield.
            AbilityDef::static_ability(
                "Affinity for artifacts (This spell costs {1} less to cast for each artifact you \
                 control.)",
                EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ))),
            )
            .with_source_zones(&[ZoneKind::Hand]),
            abilities::enters_trigger(
                "When Emry enters, mill four cards.",
                EffectDef::Mill {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(4),
                },
            ),
            // The cost is still owed and the timing rules still apply: what the
            // permission buys is that the graveyard is a legal place to cast the
            // named card from, and only until the turn is over.
            AbilityDef::activated_with_targets(
                "{T}: Choose target artifact card in your graveyard. You may cast that card this turn. \
                 (You still pay its costs. Timing rules still apply.)",
                &[AbilityCostDef::TapSource],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Artifact),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                )],
                EffectDef::PermitCastFromGraveyardThisTurn {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
        ]),
);

// ELD 372 — Questing Beast
pub(in crate::card::sets) static QUESTING_BEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5357e802-2d25-48d3-a188-101c142787b7"),
    "Questing Beast",
    CardArt::new("5357e802-2d25-48d3-a188-101c142787b7", "Igor Kieryluk"),
    CardSet::ThroneOfEldraine,
    // Four mana for a 4/4 that attacks the turn it lands, kills whatever
    // blocks it, cannot be chump-blocked, and takes a planeswalker down
    // with the player.
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Beast"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::vigilance(),
            abilities::deathtouch(),
            abilities::haste(),
            AbilityDef::static_ability(
                "This creature can't be blocked by creatures with power 2 or less.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    // "Power 2 or less" is strictly-less-than-three, which is the comparison
                    // the engine has and the same set of creatures.
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)))),
                },
            ),
            AbilityDef::static_ability(
                "Combat damage that would be dealt by creatures you control can't be prevented.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CombatDamageCannotBePrevented),
                },
            ),
            // "That much damage" is the combat damage that was actually dealt, so a
            // Beast whose damage was reduced deals the reduced amount here too.
            AbilityDef::triggered_with_targets(
                "Whenever this creature deals combat damage to an opponent, it deals that much damage to \
                 target planeswalker that player controls.",
                TriggerEventDef::combat_damage_to_related_player(
                    ObjectPredicateDef::Source,
                    PlayerRelation::Opponent,
                ),
                // "That player controls": with two players the player just dealt combat
                // damage by the Beast is the opponent, so the relation says it exactly.
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Planeswalker),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    },
                )],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::TriggerEventAmount,
                },
            ),
        ]),
);

// ELD 391 — Fabled Passage
pub(in crate::card::sets) static FABLED_PASSAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57645743-27fa-4a75-9511-acfc32dd349a"),
    "Fabled Passage",
    crate::card::CardArt::new("57645743-27fa-4a75-9511-acfc32dd349a", "Howard Lyon"),
    crate::card::CardSet::ThroneOfEldraine,
    // Evolving Wilds that stops costing you the turn once the game is old
    // enough: the tapped land is only tapped while you are still behind.
    CardRules::new_land(&[]).with_ability(AbilityDef::activated(
        "{T}, Sacrifice this land: Search your library for a basic land card, put it onto the \
         battlefield tapped, then shuffle. Then if you control four or more lands, untap that \
         land.",
        &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Land),
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
            ]),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: true,
            attachment: None,
            binding: Some(ParentBinding),
            then: Some(&EffectDef::IfCondition {
                // Counted after the search, so the land that just arrived is one of the
                // four -- and the Passage itself is not, having sacrificed itself to pay.
                // Three lands beside it is the threshold in practice.
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 4,
                },
                // "Untap that land": the one this search found rather than any land, which
                // is why the search binds what it took.
                then: &EffectDef::Untap {
                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                },
            }),
        },
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARDENVALE_TACTICIAN,
    &FAERIE_GUIDEMOTHER,
    &BRAZEN_BORROWER,
    &WISHCLAW_TALISMAN,
    &BONECRUSHER_GIANT,
    &EMBERETH_SHIELDBREAKER,
    &RIMROCK_KNIGHT,
    &ROBBER_OF_THE_RICH,
    &ONCE_UPON_A_TIME,
    &OKO_THIEF_OF_CROWNS,
    &GINGERBRUTE,
    &STONECOIL_SERPENT,
    &MYSTIC_SANCTUARY,
    &WITCH_S_COTTAGE,
    &EMRY_LURKER_OF_THE_LOCH,
    &QUESTING_BEAST,
    &FABLED_PASSAGE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
