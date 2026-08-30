//! Throne of Eldraine cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::PlayOptionDef;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityPredicateDef, AbilityTargetDef, AbilityTargetPredicate,
    ActivationTimingDef, AlternateSpellKind, AlternativeCastKindDef, AppliedEffectDef,
    AppliedRuleDef, BattlefieldEntryModificationDef, CardArt, CardComposition, CardEffectStatus,
    CardPart, CardRules, CardSet, CardStructure, CardSupertype, CardType, CardTypeSet, ColorSet,
    ComparisonDef, ControlDurationDef, CounterKind, CreatureTypeSetDef, EffectDef,
    EffectRecipientDef, ExilePlayConditionDef, ExilePlayDurationDef, KeywordAbility, ManaColor,
    ObjectPredicateDef, ObjectQueryDef, ObjectSetDef, PlayerRefDef, PlayerRelation,
    ReplacementEffectDef, ResolvedEffectDurationDef, SpellForm, SpellResolutionDestinationDef,
    TopCardSelectionDef, TriggerConditionDef, TriggerEventDef, ValueComparisonDef, ValueDef,
    ZoneKind, ZonePlacement, abilities, tokens,
};
use crate::ids::ObjectSetBindingIndex;
use crate::{CardPartId, PlayOptionId, TargetIndex, mana_cost};

// ELD 5 — Ardenvale Tactician
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARDENVALE_TACTICIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bd6ccd0b-5279-431f-b65a-7fdbdffd1a90"),
    "Ardenvale Tactician",
    crate::card::CardArt::new("c7d5e394-8e41-442e-ae97-a478a61e1b9d", "Jason Rainville"),
    crate::card::CardSet::ThroneOfEldraine,
    crate::card::CardRules::unsupported(),
);

// ELD 11 — Faerie Guidemother
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FAERIE_GUIDEMOTHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e8bbece8-9620-44d9-b991-350fe952538a"),
    "Faerie Guidemother",
    crate::card::CardArt::new("e8bbece8-9620-44d9-b991-350fe952538a", "Mila Pesic"),
    crate::card::CardSet::ThroneOfEldraine,
    crate::card::CardRules::unsupported(),
);

// ELD 39 — Brazen Borrower
static A_NONLAND_PERMANENT_THEY_CONTROL: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::Opponent),
        owner: None,
    },
)];

const fn petty_theft_rules() -> CardRules {
    CardRules::new_instant(mana_cost!("{1}{U}"))
        .with_subtypes(&["Adventure"])
        .with_ability(
            AbilityDef::spell_with_targets(
                "Return target nonland permanent an opponent controls to its owner's hand.",
                &A_NONLAND_PERMANENT_THEY_CONTROL,
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            )
            .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
        )
}

/// "Can block only creatures with flying" is the price of a 3/1 flier with
/// flash: it answers what is in the air and nothing on the ground.
static A_FLYER: ObjectPredicateDef = ObjectPredicateDef::HasKeyword(KeywordAbility::Flying);

static BORROWER_BLOCKS_ONLY_FLYERS: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(A_FLYER)),
};

static BORROWER_ABILITIES: [AbilityDef; 3] = [
    abilities::flash(),
    abilities::flying(),
    AbilityDef::static_ability(
        "This creature can block only creatures with flying.",
        BORROWER_BLOCKS_ONLY_FLYERS,
    ),
];

const fn brazen_borrower_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Faerie", "Rogue"], 3, 1)
        .with_abilities(&BORROWER_ABILITIES)
}

fn brazen_borrower_composition() -> CardComposition {
    let borrower = brazen_borrower_rules();
    let theft = petty_theft_rules();
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
static WISHCLAW_COSTS: [AbilityCostDef; 3] = [
    AbilityCostDef::Mana(mana_cost!("{1}")),
    AbilityCostDef::TapSource,
    AbilityCostDef::RemoveCountersFromSource {
        kind: CounterKind::named("wish"),
        amount: 1,
    },
];

/// The tutor and the handover are one clause resolving in order, so the card
/// is in hand before the artifact changes sides -- and the opponent inherits
/// two counters they may spend on their own turn.
static WISHCLAW_GRANTS_A_WISH: [EffectDef; 2] = [
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
];

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
            &WISHCLAW_COSTS,
            EffectDef::Sequence(&WISHCLAW_GRANTS_A_WISH),
        )
        .with_activation_timing(ActivationTimingDef::YourTurn),
    ]),
);

// ELD 115 — Bonecrusher Giant
static STOMP_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

/// The two sentences are one clause resolving in order, and the order is what
/// the card is for: prevention is off before the damage arrives, so a
/// protection that would have stopped it does not.
static STOMP_EFFECTS: [EffectDef; 2] = [
    EffectDef::DamageCannotBePreventedThisTurn,
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(2),
    },
];

const fn stomp_rules() -> CardRules {
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Damage can't be prevented this turn.\nStomp deals 2 damage to any target.",
            &STOMP_TARGET,
            EffectDef::Sequence(&STOMP_EFFECTS),
        )
        .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
    )
}

/// The punishment half. Answering the Giant with a removal spell costs two
/// life whether or not the spell works, which is what makes it awkward to
/// answer at all.
static BONECRUSHER_PUNISHES: AbilityDef = AbilityDef::triggered(
    "Whenever this creature becomes the target of a spell, this creature deals 2 damage to that spell's controller.",
    TriggerEventDef::BecomesTargetOfSpell(ObjectPredicateDef::Any),
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::EventPlayer,
        amount: ValueDef::Constant(2),
    },
);

const fn bonecrusher_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Giant"], 4, 3)
        .with_ability(BONECRUSHER_PUNISHES)
}

fn bonecrusher_composition() -> CardComposition {
    let giant = bonecrusher_rules();
    let stomp = stomp_rules();
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
static BATTLE_DISPLAY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Artifact),
)];

/// The adventure half. Answering an artifact for one red leaves the body
/// waiting in exile, which is the whole bargain of the mechanic.
fn battle_display_rules() -> CardRules {
    CardRules::new_sorcery(mana_cost!("{R}"))
        .with_subtypes(&["Adventure"])
        .with_ability(
            AbilityDef::spell_with_targets(
                "Destroy target artifact.",
                &BATTLE_DISPLAY_TARGET,
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
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
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RIMROCK_KNIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a3d13d84-01e4-4429-93db-e5afff811527"),
    "Rimrock Knight",
    crate::card::CardArt::new("a3d13d84-01e4-4429-93db-e5afff811527", "Chris Rallis"),
    crate::card::CardSet::ThroneOfEldraine,
    crate::card::CardRules::unsupported(),
);

// ELD 138 — Robber of the Rich
/// "If defending player has more cards in hand than you", which is two hand
/// sizes compared rather than either measured: a hand above nothing is the
/// whole of it.
static THEY_HAVE_MORE_CARDS: ValueComparisonDef = ValueComparisonDef {
    left: ValueDef::CardsInHandAbove {
        player: PlayerRelation::Opponent,
        threshold: 0,
    },
    comparison: ComparisonDef::Greater,
    right: ValueDef::CardsInHandAbove {
        player: PlayerRelation::You,
        threshold: 0,
    },
};

static ROBBER_STEALS_IF_THEY_ARE_RICHER: TriggerConditionDef =
    TriggerConditionDef::ValueComparison(&THEY_HAVE_MORE_CARDS);

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
                &ROBBER_STEALS_IF_THEY_ARE_RICHER,
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
/// "You may reveal a creature or land card from among them": the two types
/// the deck casting this on turn one is actually short of.
static A_CREATURE_OR_LAND_CARD: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::HasType(CardType::Land),
]);

static ONCE_UPON_A_TIME_DIGS: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(5),
    object: Some(A_CREATURE_OR_LAND_CARD),
    minimum: 0,
    maximum: 1,
    select_all_matching: false,
    select_one_of_each_type: false,
    reveal_inspected: false,
    reveal_selected: true,
    counted: None,
    selected_zone: ZoneKind::Hand,
    selected_placement: ZonePlacement::Top,
    selected_hidden: false,
    selected_linked_to_source: false,
    selected_face_down: None,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Bottom,
    rest_random_order: true,
    rest_counters: None,
    selected_order_follows_choice: false,
    then: None,
};

/// The spell asking is counted as it goes on the stack, so a spell that is
/// the first one asks about a tally still standing at zero.
static NOTHING_CAST_YET: ValueComparisonDef = ValueComparisonDef {
    left: ValueDef::SpellsCastThisGame(PlayerRelation::You),
    comparison: ComparisonDef::Equal,
    right: ValueDef::Constant(0),
};

static IT_IS_YOUR_FIRST_SPELL: TriggerConditionDef =
    TriggerConditionDef::ValueComparison(&NOTHING_CAST_YET);

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
        .with_alternative_condition(&IT_IS_YOUR_FIRST_SPELL),
        AbilityDef::spell(
            "Look at the top five cards of your library. You may reveal a creature or land card \
             from among them and put it into your hand. Put the rest on the bottom of your \
             library in a random order.",
            EffectDef::LookAtTopAndSelect {
                player: EffectRecipientDef::Controller,
                looker: EffectRecipientDef::Controller,
                selection: &ONCE_UPON_A_TIME_DIGS,
            },
        ),
    ]),
);

// ELD 197 — Oko, Thief of Crowns
/// "Loses all abilities and becomes a green Elk creature with base power and
/// toughness 3/3." Five operations in one clause, and no duration at all:
/// what Oko does to a Mox is permanent.
static OKO_ELK: [AppliedEffectDef; 5] = [
    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
    AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Elk"])),
    AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
    AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Green])),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(3), ValueDef::Constant(3)),
];

static AN_ARTIFACT_OR_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Creature),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

/// The exchange names one of each: something of yours, and something small
/// of theirs. An Elk the +1 just made is exactly the kind of thing the
/// first slot is for.
static OKO_EXCHANGE_TARGETS: [AbilityTargetDef; 2] = [
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
];

static OKO_ABILITIES: [AbilityDef; 3] = [
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
        &AN_ARTIFACT_OR_CREATURE,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&OKO_ELK),
            duration: ResolvedEffectDurationDef::Permanent,
        },
    ),
    AbilityDef::activated_with_targets(
        "−5: Exchange control of target artifact or creature you control and target creature an \
         opponent controls with power 3 or less.",
        &[AbilityCostDef::Loyalty(-5)],
        &OKO_EXCHANGE_TARGETS,
        EffectDef::ExchangeControl {
            first: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            second: EffectRecipientDef::Target(TargetIndex(1)),
            otherwise: None,
        },
    ),
];

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
        .with_abilities(&OKO_ABILITIES),
);

// ELD 219 — Gingerbrute
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GINGERBRUTE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f55fe038-c903-4d92-b689-72dd6d041a91"),
    "Gingerbrute",
    crate::card::CardArt::new("f55fe038-c903-4d92-b689-72dd6d041a91", "Vincent Proce"),
    crate::card::CardSet::ThroneOfEldraine,
    crate::card::CardRules::unsupported(),
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
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTIC_SANCTUARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("170e792c-80d5-4775-ad95-37614574ab84"),
    "Mystic Sanctuary",
    crate::card::CardArt::new("170e792c-80d5-4775-ad95-37614574ab84", "Randy Vargas"),
    crate::card::CardSet::ThroneOfEldraine,
    crate::card::CardRules::unsupported(),
);

// ELD 249 — Witch's Cottage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WITCH_S_COTTAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b87891cd-b457-4dff-8d18-a7eaf6748fc6"),
    "Witch's Cottage",
    crate::card::CardArt::new("b87891cd-b457-4dff-8d18-a7eaf6748fc6", "Gabor Szikszai"),
    crate::card::CardSet::ThroneOfEldraine,
    crate::card::CardRules::unsupported(),
);

// ELD 342 — Emry, Lurker of the Loch
static ARTIFACTS_YOU_CONTROL_EMRY: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Artifact),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

static AN_ARTIFACT_CARD_IN_YOUR_GRAVEYARD: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Artifact),
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: Some(PlayerRelation::You),
    },
)];

static EMRY_TAP_COST: [AbilityCostDef; 1] = [AbilityCostDef::TapSource];

static EMRY_ABILITIES: [AbilityDef; 3] = [
    // Affinity is a discount the card prints about itself, read from hand
    // where the spell is being paid for rather than off the battlefield.
    AbilityDef::static_ability(
        "Affinity for artifacts (This spell costs {1} less to cast for each artifact you \
         control.)",
        EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(&ARTIFACTS_YOU_CONTROL_EMRY)),
    )
    .with_source_zones(&[ZoneKind::Hand]),
    abilities::enters_trigger(
        "When Emry enters, mill four cards.",
        EffectDef::Mill {
            player: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(4),
            binding: None,
            then: None,
        },
    ),
    // The cost is still owed and the timing rules still apply: what the
    // permission buys is that the graveyard is a legal place to cast the
    // named card from, and only until the turn is over.
    AbilityDef::activated_with_targets(
        "{T}: Choose target artifact card in your graveyard. You may cast that card this turn. \
         (You still pay its costs. Timing rules still apply.)",
        &EMRY_TAP_COST,
        &AN_ARTIFACT_CARD_IN_YOUR_GRAVEYARD,
        EffectDef::PermitCastFromGraveyardThisTurn {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    ),
];

pub(in crate::card::sets) static EMRY_LURKER_OF_THE_LOCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("157f343d-8583-4827-a77d-d916e6a5caa1"),
    "Emry, Lurker of the Loch",
    CardArt::new("157f343d-8583-4827-a77d-d916e6a5caa1", "Livia Prima"),
    CardSet::ThroneOfEldraine,
    // A one-mana 1/2 on any board with two artifacts, and the mill she
    // arrives with is where she finds what to recast.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk", "Wizard"], 1, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&EMRY_ABILITIES),
);

// ELD 372 — Questing Beast
/// "Power 2 or less" is strictly-less-than-three, which is the comparison
/// the engine has and the same set of creatures.
static A_SMALL_BLOCKER: ObjectPredicateDef =
    ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3));

static CREATURES_YOU_CONTROL_QB: EffectRecipientDef = EffectRecipientDef::matching_objects(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

/// "That player controls": with two players the player just dealt combat
/// damage by the Beast is the opponent, so the relation says it exactly.
static THEIR_PLANESWALKER: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Planeswalker),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::Opponent),
        owner: None,
    },
)];

static QUESTING_BEAST_ABILITIES: [AbilityDef; 6] = [
    abilities::vigilance(),
    abilities::deathtouch(),
    abilities::haste(),
    AbilityDef::static_ability(
        "This creature can't be blocked by creatures with power 2 or less.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(A_SMALL_BLOCKER)),
        },
    ),
    AbilityDef::static_ability(
        "Combat damage that would be dealt by creatures you control can't be prevented.",
        EffectDef::StaticApply {
            recipient: CREATURES_YOU_CONTROL_QB,
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
        &THEIR_PLANESWALKER,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::TriggerEventAmount,
        },
    ),
];

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
        .with_abilities(&QUESTING_BEAST_ABILITIES),
);

// ELD 391 — Fabled Passage
/// Counted after the search, so the land that just arrived is one of the
/// four -- and the Passage itself is not, having sacrificed itself to pay.
/// Three lands beside it is the threshold in practice.
static FABLED_FOUR_LANDS: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::HasType(CardType::Land),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 4,
};

/// "Untap that land": the one this search found rather than any land, which
/// is why the search binds what it took.
static FABLED_UNTAPS_IT: EffectDef = EffectDef::Untap {
    object: EffectRecipientDef::objects(ObjectSetDef::Binding(ObjectSetBindingIndex::PRIMARY)),
};

static FABLED_MAY_UNTAP: EffectDef = EffectDef::IfCondition {
    condition: &FABLED_FOUR_LANDS,
    then: &FABLED_UNTAPS_IT,
};

static FABLED_PASSAGE_FETCH: EffectDef = EffectDef::SearchZone {
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
    binding: Some(ObjectSetBindingIndex::PRIMARY),
    then: Some(&FABLED_MAY_UNTAP),
};

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
        FABLED_PASSAGE_FETCH,
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
