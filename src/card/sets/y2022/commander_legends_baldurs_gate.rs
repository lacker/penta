//! Commander Legends: Battle for Baldur's Gate cards cataloged for the
//! Vintage Cube.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCoverageDef, AbilityDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef, PlayerRelation,
    SacrificedAmountDef, TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, abilities,
};
use crate::mana_cost;

/// "Another creature or an artifact." Gut is neither an artifact nor another
/// creature, so the exclusion covers both halves without saying so twice.
static ANOTHER_CREATURE_OR_AN_ARTIFACT: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::HasType(CardType::Artifact),
    ]),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
]);

/// The token arrives already attacking, which is the whole point: it was
/// never declared, so nothing that watches a declaration sees it, and it
/// still connects this combat.
static GUT_MAKES_A_SKELETON: EffectDef =
    EffectDef::create_creature_token(&["Skeleton"], &[ManaColor::Black], 4, 1)
        .with_abilities(&[abilities::menace()])
        .with_art(CardArt::new(
            "cf4c245f-af2f-46a7-81f3-670a04940901",
            "David Astruga",
        ))
        .entering_tapped()
        .entering_attacking();

/// "Whenever you attack" is one or more creatures you control attacking,
/// counted once for the declaration rather than once per attacker.
static WHENEVER_YOU_ATTACK: TriggerEventDef = TriggerEventDef::attack_declared(
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
    1,
    None,
);

// CLB 180 — Gut, True Soul Zealot
pub(in crate::card::sets) static GUT_TRUE_SOUL_ZEALOT: CardRecord = CardRecord::new_with_legacy_id(
    2211,
    "Gut, True Soul Zealot",
    CardArt::new("3d8ca18d-9099-4f1e-95c1-f04da58a26bd", "Wayne Reynolds"),
    CardSet::CommanderLegendsBattleForBaldursGate,
    // Every spent artifact and every creature that has done its work turns
    // into four attacking power that two blockers cannot answer alone.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Shaman"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever you attack, you may sacrifice another creature or an artifact. If you do, create a 4/1 black Skeleton creature token with menace that's tapped and attacking.",
                WHENEVER_YOU_ATTACK,
                EffectDef::SacrificeOfChoice {
                    player: EffectRecipientDef::Controller,
                    object: ANOTHER_CREATURE_OR_AN_ARTIFACT,
                    then: Some(&GUT_MAKES_A_SKELETON),
                    amount: SacrificedAmountDef::Power,
                    otherwise: None,
                    optional: true,
                },
            ),
            AbilityDef::static_ability(
                "Choose a Background (You can have a Background as a second commander.)",
                EffectDef::Special("Choose a Background"),
            )
            .with_coverage(AbilityCoverageDef::metadata_only(
                "Backgrounds are a Commander deck-construction rule. This engine plays no \
                 format that has a command zone, so the clause names nothing a game can do.",
            )),
        ]),
);

/// Two damage as the baseline and five when it was foretold, which is the
/// whole of the card: the two mana spent a turn earlier buy three damage and
/// one mana off the price.
static FIREBALL_FOR_TWO: EffectDef = EffectDef::DealDamage {
    recipient: EffectRecipientDef::EachOpponentAndTheirCreatures,
    amount: ValueDef::Constant(2),
};

static FIREBALL_FOR_FIVE: EffectDef = EffectDef::DealDamage {
    recipient: EffectRecipientDef::EachOpponentAndTheirCreatures,
    amount: ValueDef::Constant(5),
};

static CAST_FROM_EXILE: TriggerConditionDef = TriggerConditionDef::SourceCastFrom(ZoneKind::Exile);

/// "Instead": the two branches are exclusive, so each names the condition
/// and the smaller one names its negation. Written this way rather than as
/// one conditional with an else because that is what the card says -- a
/// baseline, and a replacement for it.
static DELAYED_BLAST_FIREBALL_EFFECT: [EffectDef; 2] = [
    EffectDef::IfCondition {
        condition: &CAST_FROM_EXILE,
        then: &FIREBALL_FOR_FIVE,
    },
    EffectDef::IfCondition {
        condition: &TriggerConditionDef::Not(&CAST_FROM_EXILE),
        then: &FIREBALL_FOR_TWO,
    },
];

// CLB 630 — Delayed Blast Fireball
pub(in crate::card::sets) static DELAYED_BLAST_FIREBALL: CardRecord =
    CardRecord::new_with_legacy_id(
        2299,
        "Delayed Blast Fireball",
        CardArt::new("400c76c6-f677-4e7e-87ad-2e526d4b498a", "Andreas Zafiratos"),
        CardSet::CommanderLegendsBattleForBaldursGate,
        // A one-sided sweeper that costs a turn of setup, which is the trade the
        // cube's aggressive decks are least able to make and the slow ones most.
        CardRules::new_instant(mana_cost!("{1}{R}{R}")).with_abilities(&[
            AbilityDef::spell(
                "Delayed Blast Fireball deals 2 damage to each opponent and each creature they \
             control. If this spell was cast from exile, it deals 5 damage to each opponent and \
             each creature they control instead.",
                EffectDef::Sequence(&DELAYED_BLAST_FIREBALL_EFFECT),
            ),
            abilities::foretell(mana_cost!("{4}{R}{R}")),
        ]),
    );

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&GUT_TRUE_SOUL_ZEALOT, &DELAYED_BLAST_FIREBALL];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
