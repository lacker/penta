//! Throne of Eldraine cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::PlayOptionDef;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityPredicateDef, AbilityTargetDef, AbilityTargetPredicate,
    ActivationTimingDef, AlternateSpellKind, AppliedEffectDef, BattlefieldEntryModificationDef,
    CardArt, CardComposition, CardEffectStatus, CardPart, CardRules, CardSet, CardStructure,
    CardSupertype, CardType, CardTypeSet, ColorSet, ControlDurationDef, CounterKind,
    CreatureTypeSetDef, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef, PlayerRefDef,
    PlayerRelation, ReplacementEffectDef, ResolvedEffectDurationDef, SpellForm,
    SpellResolutionDestinationDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, tokens,
};
use crate::{CardPartId, PlayOptionId, TargetIndex, mana_cost};

static WISHCLAW_COSTS: [AbilityCostDef; 3] = [
    AbilityCostDef::Mana(mana_cost!("{1}")),
    AbilityCostDef::TapSource,
    AbilityCostDef::RemoveCountersFromSource {
        kind: CounterKind::Wish,
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
                    kind: CounterKind::Wish,
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

// ELD 115 — Bonecrusher Giant
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

// ELD 122 — Embereth Shieldbreaker
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
// Audit: blocked — Needs three things. An intervening-if that compares two players' hand sizes rather than a count against a printed number; a permission to cast one exiled card that survives its source leaving the battlefield and is gated on having attacked with a Rogue that turn; and spending mana as though it were mana of any color, which already blocks North Star in Legends.

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
        },
    ),
];

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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARDENVALE_TACTICIAN,
    &FAERIE_GUIDEMOTHER,
    &WISHCLAW_TALISMAN,
    &BONECRUSHER_GIANT,
    &EMBERETH_SHIELDBREAKER,
    &RIMROCK_KNIGHT,
    &OKO_THIEF_OF_CROWNS,
    &GINGERBRUTE,
    &MYSTIC_SANCTUARY,
    &WITCH_S_COTTAGE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
