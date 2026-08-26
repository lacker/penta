//! Ikoria: Lair of Behemoths cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    CompanionConditionDef, CostModificationDef, DeckConstructionDef, EffectDef, EffectRecipientDef,
    GraveyardPlayPermissionDef, ObjectPredicateDef, PlayActionMatcherDef, PlayRestrictionDef,
    PlayerRefDef, PlayerRelation, ResolvedEffectDurationDef, TriggerConditionDef, TriggerEventDef,
    ValueDef, ZoneKind, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

/// A triome is a tapped land with three basic land types and cycling, and
/// nothing else. Its printed mana ability is reminder text for what the
/// subtypes already grant, so it is not restated as a clause.
const TRIOME_ABILITIES: &[AbilityDef] = &[
    abilities::enters_tapped("This land enters tapped."),
    abilities::cycling(
        "Cycling {3} ({3}, Discard this card: Draw a card.)",
        mana_cost!("{3}"),
    ),
];

const fn triome(types: &'static [&'static str]) -> CardRules {
    CardRules::new_land(types).with_abilities(TRIOME_ABILITIES)
}

// IKO 137 — Spelleater Wolverine
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPELLEATER_WOLVERINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a5f03ffd-dcdb-441c-8dfc-4fe06a289b22"),
    "Spelleater Wolverine",
    crate::card::CardArt::new("a5f03ffd-dcdb-441c-8dfc-4fe06a289b22", "Uriah Voth"),
    crate::card::CardSet::Ikoria,
    crate::card::CardRules::unsupported(),
);

// IKO 170 — Ram Through
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAM_THROUGH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ac0b24e7-14e7-45ee-b5d8-bdb8674b669c"),
    "Ram Through",
    crate::card::CardArt::new("ac0b24e7-14e7-45ee-b5d8-bdb8674b669c", "Zoltan Boros"),
    crate::card::CardSet::Ikoria,
    crate::card::CardRules::unsupported(),
);

// IKO 226 — Lurrus of the Dream-Den
/// "A permanent spell with mana value 2 or less." The action is a cast, so a
/// land card in the graveyard is not among them: lands are played rather
/// than cast, which is what keeps this from being a Crucible.
static LURRUS_PERMISSION: PlayRestrictionDef = PlayRestrictionDef::new(
    PlayActionMatcherDef::CastSpell,
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::HasType(CardType::Enchantment),
            ObjectPredicateDef::HasType(CardType::Planeswalker),
        ]),
        ObjectPredicateDef::ManaValueAtMost(2),
    ]),
);

pub(in crate::card::sets) static LURRUS_OF_THE_DREAM_DEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5ad36fb2-c44e-4085-ba0d-54277841ad3a"),
    "Lurrus of the Dream-Den",
    CardArt::new("5ad36fb2-c44e-4085-ba0d-54277841ad3a", "Slawomir Maniak"),
    CardSet::Ikoria,
    // Three mana for a lifelinking body that turns every cheap permanent in
    // the graveyard back into a card, one a turn -- which is why the decks
    // that play him keep their curve at two.
    CardRules::new_creature(mana_cost!("{1}{W/B}{W/B}"), &["Cat", "Nightmare"], 3, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::deck_construction(
                "Companion — Each permanent card in your starting deck has mana value 2 or less. \
                 (If this card is your chosen companion, you may put it into your hand from \
                 outside the game for {3} as a sorcery.)",
                DeckConstructionDef::Companion(CompanionConditionDef::PermanentManaValueAtMost(2)),
                "Both halves are here: the deck-construction condition the deck layer checks, \
                 and the special action that takes it from outside the game for {3}.",
            ),
            abilities::lifelink(),
            AbilityDef::static_ability(
                "Once during each of your turns, you may cast a permanent spell with mana value 2 \
                 or less from your graveyard.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromGraveyard(
                        GraveyardPlayPermissionDef::once_each_of_your_turns(LURRUS_PERMISSION),
                    )),
                },
            ),
        ]),
);

// IKO 233 — Zirda, the Dawnwaker
/// "Abilities you activate", which the shared vocabulary reaches as the
/// abilities of permanents you control. Cycling and the rest of the
/// abilities a card in hand or a graveyard prints are outside it.
static PERMANENTS_YOU_CONTROL: ObjectPredicateDef =
    ObjectPredicateDef::ControlledBy(PlayerRelation::You);

static ZIRDA_BLOCK_COST: [AbilityCostDef; 2] = [
    AbilityCostDef::Mana(mana_cost!("{1}")),
    AbilityCostDef::TapSource,
];

static A_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

pub(in crate::card::sets) static ZIRDA_THE_DAWNWAKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1bd8e61c-2ee8-4243-a848-7008810db8a0"),
    "Zirda, the Dawnwaker",
    CardArt::new("1bd8e61c-2ee8-4243-a848-7008810db8a0", "Jesper Ejsing"),
    CardSet::Ikoria,
    // Three mana for a 3/3 that makes every activated ability on the board
    // two cheaper, which is what a deck full of equipment and pingers is
    // waiting for.
    CardRules::new_creature(mana_cost!("{1}{R/W}{R/W}"), &["Elemental", "Fox"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::deck_construction(
                "Companion — Each permanent card in your starting deck has an activated ability. \
                 (If this card is your chosen companion, you may put it into your hand from \
                 outside the game for {3} as a sorcery.)",
                DeckConstructionDef::Companion(
                    CompanionConditionDef::EveryPermanentHasAnActivatedAbility,
                ),
                "Both halves are here: the deck-construction condition the deck layer checks, \
                 and the special action that takes it from outside the game for {3}.",
            ),
            AbilityDef::static_ability(
                "Abilities you activate that aren't mana abilities cost {2} less to activate. \
                 This effect can't reduce the mana in that cost to less than one mana.",
                EffectDef::ModifyCost(CostModificationDef::AbilityReduction {
                    permanent: PERMANENTS_YOU_CONTROL,
                    amount: ValueDef::Constant(2),
                    minimum: 1,
                }),
            )
            .with_coverage(AbilityCoverageDef::partial(
                "The discount reaches the activated abilities of permanents you control. An \
                 ability you activate from somewhere else -- cycling a card in hand, or a \
                 graveyard ability -- is outside what the shared cost vocabulary can name.",
            )),
            AbilityDef::activated_with_targets(
                "{1}, {T}: Target creature can't block this turn.",
                &ZIRDA_BLOCK_COST,
                &A_CREATURE,
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// IKO 248 — Indatha Triome
pub(in crate::card::sets) static INDATHA_TRIOME: CardRecord = CardRecord::new_with_legacy_id(
    2096,
    "Indatha Triome",
    CardArt::new("2b74bb81-fb9a-40e5-a941-e517430b52f5", "Noah Bradley"),
    CardSet::Ikoria,
    triome(&["Plains", "Swamp", "Forest"]),
);

// IKO 250 — Ketria Triome
pub(in crate::card::sets) static KETRIA_TRIOME: CardRecord = CardRecord::new_with_legacy_id(
    2097,
    "Ketria Triome",
    CardArt::new("a249b1f4-2b22-4b67-a207-e0c4ae95d2e1", "Sam Burley"),
    CardSet::Ikoria,
    triome(&["Forest", "Island", "Mountain"]),
);

// IKO 251 — Raugrin Triome
pub(in crate::card::sets) static RAUGRIN_TRIOME: CardRecord = CardRecord::new_with_legacy_id(
    2098,
    "Raugrin Triome",
    CardArt::new("02138fbb-3962-4348-8d31-faaefba0b8b2", "Jonas De Ro"),
    CardSet::Ikoria,
    triome(&["Island", "Mountain", "Plains"]),
);

// IKO 253 — Savai Triome
pub(in crate::card::sets) static SAVAI_TRIOME: CardRecord = CardRecord::new_with_legacy_id(
    2099,
    "Savai Triome",
    CardArt::new("748e6a61-9c1f-4225-9f04-e54002f63ac3", "Titus Lunter"),
    CardSet::Ikoria,
    triome(&["Mountain", "Plains", "Swamp"]),
);

// IKO 259 — Zagoth Triome
pub(in crate::card::sets) static ZAGOTH_TRIOME: CardRecord = CardRecord::new_with_legacy_id(
    2100,
    "Zagoth Triome",
    CardArt::new("cc520518-2063-4b57-a0d4-10cf62a7175e", "Eytan Zana"),
    CardSet::Ikoria,
    triome(&["Swamp", "Forest", "Island"]),
);

// IKO 355 — Lurrus of the Dream-Den (alternate printing)

// IKO 356 — Lutri, the Spellchaser
/// Yours rather than anybody's: Lutri copies what you are casting, not what
/// is being cast at you.
static YOUR_INSTANT_OR_SORCERY_SPELL: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Instant),
            ObjectPredicateDef::HasType(CardType::Sorcery),
        ]),
        zones: &[ZoneKind::Stack],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

/// "If you cast it": a Lutri put onto the battlefield some other way is a
/// 3/2 and nothing else, which is what keeps the trigger honest about being
/// half of a spell rather than half of a creature.
static LUTRI_WAS_CAST: TriggerConditionDef = TriggerConditionDef::SourceWasCast;

static LUTRI_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::deck_construction(
        "Companion — Each nonland card in your starting deck has a different name. (If this card \
         is your chosen companion, you may put it into your hand from outside the game for {3} \
         as a sorcery.)",
        DeckConstructionDef::Companion(CompanionConditionDef::NonlandNamesAreDistinct),
        "Both halves are here: the deck-construction condition the deck layer checks, and the \
         special action that takes it from outside the game for {3}.",
    ),
    abilities::flash(),
    AbilityDef::triggered_if_with_targets(
        "When Lutri enters, if you cast it, copy target instant or sorcery spell you control. \
         You may choose new targets for the copy.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        &LUTRI_WAS_CAST,
        &YOUR_INSTANT_OR_SORCERY_SPELL,
        EffectDef::CopyStackObject(&crate::card::CopyStackObjectDef {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            controller: PlayerRefDef::EffectController,
            count: ValueDef::Constant(1),
            retarget: true,
            colors: None,
        }),
    ),
];

pub(in crate::card::sets) static LUTRI_THE_SPELLCHASER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("12c01a00-2128-4b6c-874f-a206eca3a756"),
    "Lutri, the Spellchaser",
    CardArt::new("12c01a00-2128-4b6c-874f-a206eca3a756", "Lie Setiawan"),
    CardSet::Ikoria,
    // Three mana at instant speed for a body and a copy of whatever you were
    // already casting -- and in a singleton cube the companion clause costs
    // the deck nothing it was not already paying.
    CardRules::new_creature(mana_cost!("{1}{U/R}{U/R}"), &["Elemental", "Otter"], 3, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&LUTRI_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SPELLEATER_WOLVERINE,
    &RAM_THROUGH,
    &LURRUS_OF_THE_DREAM_DEN,
    &ZIRDA_THE_DAWNWAKER,
    &INDATHA_TRIOME,
    &KETRIA_TRIOME,
    &RAUGRIN_TRIOME,
    &SAVAI_TRIOME,
    &ZAGOTH_TRIOME,
    &LUTRI_THE_SPELLCHASER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&LURRUS_OF_THE_DREAM_DEN, 1), // IKO 355
];
