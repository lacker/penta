//! Dragons of Tarkir cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::TargetIndex;
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef, CardArt, CardRules,
    CardSet, CardType, DiscardSelectionDef, EffectDef, EffectRecipientDef, ObjectPredicateDef,
    PlayerRelation, ResolvedEffectDurationDef, SpellResolutionDestinationDef, ValueDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::mana_cost;

// DTK 4 — Artful Maneuver
static ARTFUL_MANEUVER_STEPS: [EffectDef; 2] = [
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(2),
            ValueDef::Constant(2),
        ),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
    abilities::rebound_offer(),
];

pub(in crate::card::sets) static ARTFUL_MANEUVER: CardRecord = CardRecord::new_with_legacy_id(
    1710,
    "Artful Maneuver",
    CardArt::new("7fcaf67e-ba97-4af9-8c47-dbca703cba35", "Lars Grant-West"),
    CardSet::DragonsOfTarkir,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(
        AbilityDef::spell_with_targets(
            "Target creature gets +2/+2 until end of turn.\n\nRebound (If you cast this spell from your hand, exile it as it resolves. At the beginning of your next upkeep, you may cast this card from exile without paying its mana cost.)",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&ARTFUL_MANEUVER_STEPS),
        )
        .with_resolution_destination(SpellResolutionDestinationDef::ExileIfCastFromHand),
    ),
);

// DTK 224 — Kolaghan's Command
/// Your own graveyard, and a creature card while it is still a card: what
/// comes back goes to hand rather than to the battlefield.
static A_CREATURE_CARD_IN_YOUR_GRAVEYARD: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: Some(PlayerRelation::You),
    },
)];

static A_PLAYER: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

static AN_ARTIFACT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Artifact),
)];

static ANY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

/// Two of four, and never the same one twice. Each mode carries its own
/// slot, so what a Command declares depends on which two it is.
static KOLAGHAN_COMMAND_MODES: [AbilityDef; 4] = [
    AbilityDef::spell_with_targets(
        "Return target creature card from your graveyard to your hand.",
        &A_CREATURE_CARD_IN_YOUR_GRAVEYARD,
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
        },
    ),
    AbilityDef::spell_with_targets(
        "Target player discards a card.",
        &A_PLAYER,
        EffectDef::Discard {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(1),
            selection: DiscardSelectionDef::RecipientChooses,
            then: None,
        },
    ),
    AbilityDef::spell_with_targets(
        "Destroy target artifact.",
        &AN_ARTIFACT,
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
            then: None,
        },
    ),
    AbilityDef::spell_with_targets(
        "Kolaghan's Command deals 2 damage to any target.",
        &ANY_TARGET,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
        },
    ),
];

pub(in crate::card::sets) static KOLAGHAN_S_COMMAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7c884e1e-fecb-4330-b3de-5fc2a60f7173"),
    "Kolaghan's Command",
    CardArt::new("7c884e1e-fecb-4330-b3de-5fc2a60f7173", "Daarken"),
    CardSet::DragonsOfTarkir,
    // Three mana that is two cards on every board: something always comes
    // back, and something of theirs always goes.
    CardRules::new_instant(mana_cost!("{1}{B}{R}")).with_ability(AbilityDef::modal_spell(
        "Choose two \u{2014}\n\u{2022} Return target creature card from your graveyard to your \
         hand.\n\u{2022} Target player discards a card.\n\u{2022} Destroy target artifact.\n\
         \u{2022} Kolaghan's Command deals 2 damage to any target.",
        &KOLAGHAN_COMMAND_MODES,
        2,
        2,
        false,
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&ARTFUL_MANEUVER, &KOLAGHAN_S_COMMAND];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
