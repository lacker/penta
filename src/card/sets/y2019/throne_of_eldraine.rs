//! Throne of Eldraine cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::PlayOptionDef;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef,
    AlternateSpellKind, BattlefieldEntryModificationDef, CardArt, CardComposition,
    CardEffectStatus, CardPart, CardRules, CardSet, CardStructure, CardType, ControlDurationDef,
    CounterKind, EffectDef, EffectRecipientDef, ObjectPredicateDef, PlayerRefDef,
    ReplacementEffectDef, SpellForm, SpellResolutionDestinationDef, TriggerEventDef, ValueDef,
    ZoneKind, ZonePlacement,
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

// ELD 138 — Robber of the Rich
// Audit: blocked — Needs three things. An intervening-if that compares two players' hand sizes rather than a count against a printed number; a permission to cast one exiled card that survives its source leaving the battlefield and is gated on having attacked with a Rogue that turn; and spending mana as though it were mana of any color, which already blocks North Star in Legends.

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &WISHCLAW_TALISMAN,
    &BONECRUSHER_GIANT,
    &EMBERETH_SHIELDBREAKER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
