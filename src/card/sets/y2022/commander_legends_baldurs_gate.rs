//! Commander Legends: Battle for Baldur's Gate cards cataloged for the
//! Vintage Cube.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt, CardRules,
    CardSet, CardSupertype, CardType, ChoiceVisibilityDef, ChooseDef, CounterKind,
    DeckConstructionDef, EffectDef, EffectRecipientDef, KeywordAbility, ManaColor,
    ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    PlayerRefDef, PlayerRelation, SacrificedAmountDef, TokenCharacteristics, TriggerConditionDef,
    TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, abilities,
};
use crate::ids::ObjectBindingIndex;
use crate::{TargetIndex, mana_cost};

// CLB 11 — Blessed Hippogriff
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLESSED_HIPPOGRIFF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b4590e53-ca8d-4896-a8cf-6af1e4bc456f"),
    "Blessed Hippogriff",
    crate::card::CardArt::new("b4590e53-ca8d-4896-a8cf-6af1e4bc456f", "Leanna Crossan"),
    crate::card::CardSet::CommanderLegendsBattleForBaldursGate,
    crate::card::CardRules::unsupported(),
);

// CLB 22 — Greatsword of Tyr
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GREATSWORD_OF_TYR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("50088a60-642b-47ed-a289-ef0b617b688f"),
    "Greatsword of Tyr",
    crate::card::CardArt::new("50088a60-642b-47ed-a289-ef0b617b688f", "Titus Lunter"),
    crate::card::CardSet::CommanderLegendsBattleForBaldursGate,
    crate::card::CardRules::unsupported(),
);

// CLB 99 — Sword Coast Serpent
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SWORD_COAST_SERPENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0bbfb7ae-9a32-428d-903c-99d0d8669b8d"),
    "Sword Coast Serpent",
    crate::card::CardArt::new("0bbfb7ae-9a32-428d-903c-99d0d8669b8d", "Caio Monteiro"),
    crate::card::CardSet::CommanderLegendsBattleForBaldursGate,
    crate::card::CardRules::unsupported(),
);

// CLB 106 — Young Blue Dragon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static YOUNG_BLUE_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("56b0f66b-dca9-4a01-9394-20a513c2b225"),
    "Young Blue Dragon",
    crate::card::CardArt::new("56b0f66b-dca9-4a01-9394-20a513c2b225", "Tuan Duong Chu"),
    crate::card::CardSet::CommanderLegendsBattleForBaldursGate,
    crate::card::CardRules::unsupported(),
);

// CLB 113 — Arms of Hadar
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARMS_OF_HADAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("db1fd431-8f6d-4ca5-bc0c-53881c500da1"),
    "Arms of Hadar",
    crate::card::CardArt::new("db1fd431-8f6d-4ca5-bc0c-53881c500da1", "Mirko Failoni"),
    crate::card::CardSet::CommanderLegendsBattleForBaldursGate,
    crate::card::CardRules::unsupported(),
);

// CLB 119 — Cast Down
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CAST_DOWN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("116ce944-6871-4f51-a889-d9c4a5d7cff2"),
    "Cast Down",
    crate::card::CardArt::new("aba79021-39af-4e74-beb5-f2f508c865b2", "Tyler Walpole"),
    crate::card::CardSet::CommanderLegendsBattleForBaldursGate,
    crate::card::CardRules::unsupported(),
);

// CLB 130 — Guildsworn Prowler
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GUILDSWORN_PROWLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d7efb10f-c760-431c-8ac6-904965d850dc"),
    "Guildsworn Prowler",
    crate::card::CardArt::new("d7efb10f-c760-431c-8ac6-904965d850dc", "Fariba Khamseh"),
    crate::card::CardSet::CommanderLegendsBattleForBaldursGate,
    crate::card::CardRules::unsupported(),
);

// CLB 180 — Gut, True Soul Zealot
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
                    count: ValueDef::Constant(1),
                    player: EffectRecipientDef::Controller,
                    object: ANOTHER_CREATURE_OR_AN_ARTIFACT,
                    then: Some(&GUT_MAKES_A_SKELETON),
                    amount: SacrificedAmountDef::Power,
                    otherwise: None,
                    optional: true,
                },
            ),
            AbilityDef::deck_construction(
                "Choose a Background (You can have a Background as a second commander.)",
                DeckConstructionDef::ChooseABackground,
                "The parenthesis is the whole sentence: it is a deck-construction \
                 permission, checked where a Commander list is assembled and silent \
                 once the game starts.",
            ),
        ]),
);

// CLB 263 — You Meet in a Tavern
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static YOU_MEET_IN_A_TAVERN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("593aa59a-4025-4df8-9f27-188fc7712fde"),
    "You Meet in a Tavern",
    crate::card::CardArt::new("9fddbd7a-799c-4432-810c-d839c5c354b9", "Zoltan Boros"),
    crate::card::CardSet::CommanderLegendsBattleForBaldursGate,
    crate::card::CardRules::unsupported(),
);

// CLB 285 — Minsc & Boo, Timeless Heroes
// Audit: partial — The minus names its damage target on activation instead of through a reflexive trigger, so an answered target counters the sacrifice too.
static BOO_ABILITIES: [AbilityDef; 2] = [abilities::trample(), abilities::haste()];

static BOO: TokenCharacteristics =
    TokenCharacteristics::creature(&["Hamster"], &[ManaColor::Red], 1, 1)
        .with_name("Boo")
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&BOO_ABILITIES);

static MINSC_MAKES_BOO: EffectDef = EffectDef::create_token(BOO);

/// One ability with two events rather than two abilities: the card prints
/// one, and Boo arrives once per event either way.
static MINSC_ENTERS_OR_UPKEEP: [TriggerEventDef; 2] = [
    TriggerEventDef::zone_changed(
        ObjectPredicateDef::Source,
        None,
        Some(ZoneKind::Battlefield),
    ),
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
];

static A_TRAMPLER_OR_A_HASTY_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasKeyword(KeywordAbility::Trample),
                ObjectPredicateDef::HasKeyword(KeywordAbility::Haste),
            ]),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
    1,
)];

/// "Where X is that creature's power": read off the creature that was
/// sacrificed, from last-known information, since paying is what put it in
/// the graveyard.
static SACRIFICED_POWER: ValueDef =
    ValueDef::ObjectPower(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY));

static THE_SACRIFICED_WAS_A_HAMSTER: TriggerConditionDef =
    TriggerConditionDef::BoundObjectMatches {
        binding: ObjectBindingIndex::PRIMARY,
        object: ObjectPredicateDef::Subtype("Hamster"),
    };

static MINSC_DRAWS: EffectDef = EffectDef::DrawCards {
    recipient: EffectRecipientDef::Controller,
    amount: SACRIFICED_POWER,
};

static MINSC_THROWS_IT: [EffectDef; 3] = [
    EffectDef::Sacrifice {
        object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    },
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: SACRIFICED_POWER,
    },
    EffectDef::IfCondition {
        condition: &THE_SACRIFICED_WAS_A_HAMSTER,
        then: &MINSC_DRAWS,
    },
];

static MINSC_THROWS_IT_SEQUENCE: EffectDef = EffectDef::Sequence(&MINSC_THROWS_IT);

/// The creature is named as the ability resolves rather than as a cost, so
/// it is still on the battlefield while the ability is on the stack -- and
/// naming it is what lets the damage read its power afterwards.
static MINSC_CHOOSES_A_CREATURE: ChooseDef = ChooseDef {
    binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
    unchosen: None,
    chooser: PlayerRefDef::EffectController,
    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
        ObjectPredicateDef::HasType(CardType::Creature),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    )),
    exclude: None,
    minimum: 1,
    maximum: 1,
    visibility: ChoiceVisibilityDef::Public,
    then: &MINSC_THROWS_IT_SEQUENCE,
};

static MINSC_ABILITIES: [AbilityDef; 4] = [
    AbilityDef::triggered(
        "When Minsc & Boo enters and at the beginning of your upkeep, you may create Boo, a \
         legendary 1/1 red Hamster creature token with trample and haste.",
        TriggerEventDef::AnyOf(&MINSC_ENTERS_OR_UPKEEP),
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &MINSC_MAKES_BOO,
        },
    ),
    AbilityDef::activated_with_targets(
        "+1: Put three +1/+1 counters on up to one target creature with trample or haste.",
        &[AbilityCostDef::Loyalty(1)],
        &A_TRAMPLER_OR_A_HASTY_CREATURE,
        EffectDef::AddCounters {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            kind: CounterKind::PlusOnePlusOne,
            amount: ValueDef::Constant(3),
        },
    ),
    // The target is declared as the ability is activated rather than when
    // the sacrifice is actually made, which is the one place this differs
    // from the printed reflexive trigger -- the same deviation Inti and
    // Guide of Souls carry. A board with nothing to throw does not offer
    // the ability at all. It follows that an answer to the target counters
    // the whole ability rather than only the reflexive trigger, so the
    // creature that would have been thrown survives; the printed card has
    // already sacrificed it by then and loses only the damage and the cards.
    AbilityDef::activated_with_targets(
        "\u{2212}2: Sacrifice a creature. When you do, Minsc & Boo deals X damage to any target, \
         where X is that creature's power. If the sacrificed creature was a Hamster, draw X cards.",
        &[AbilityCostDef::Loyalty(-2)],
        &ANY_TARGET,
        EffectDef::Choose(MINSC_CHOOSES_A_CREATURE),
    ),
    AbilityDef::deck_construction(
        "Minsc & Boo, Timeless Heroes can be your commander.",
        DeckConstructionDef::MayBeCommander,
        "The whole sentence is a deck-construction permission: a planeswalker \
         may lead a Commander deck, which the deck layer checks and the game \
         never revisits.",
    ),
];

static ANY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

pub(in crate::card::sets) static MINSC_BOO_TIMELESS_HEROES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("928036c9-11b8-493e-b9f2-8fbd3487cd19"),
    "Minsc & Boo, Timeless Heroes",
    CardArt::new("928036c9-11b8-493e-b9f2-8fbd3487cd19", "Andreas Zafiratos"),
    CardSet::CommanderLegendsBattleForBaldursGate,
    // Four mana that leaves a hamster behind every turn, and the hamster is
    // both the thing the plus grows and the thing the minus throws.
    CardRules::new_planeswalker(mana_cost!("{2}{R}{G}"), &["Minsc"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&MINSC_ABILITIES),
);

// CLB 560 — Displacer Kitten
/// A noncreature spell you cast. What it does is no part of the condition:
/// the Kitten reads the type line and nothing else.
static A_NONCREATURE_SPELL_YOU_CAST: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

/// "Up to one target nonland permanent you control": the trigger goes on the
/// stack whether or not there is anything worth blinking.
static UP_TO_ONE_NONLAND_PERMANENT_YOU_CONTROL: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
    1,
)];

/// Exiling links the permanent to the Kitten, which is what lets the return
/// name the card the exile just made.
static KITTEN_BLINKS: [EffectDef; 2] = [
    EffectDef::ExileLinkedToSource {
        until_source_leaves: false,
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        face_down: false,
        then: None,
    },
    EffectDef::ReturnLinkedExiles {
        object: ObjectPredicateDef::Any,
        counters: None,
        zone: ZoneKind::Battlefield,
        grant: None,
        controller: None,
        transformed: false,
    },
];

pub(in crate::card::sets) static DISPLACER_KITTEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9a53e8fc-bfd2-4866-a61c-f3204b0a98bf"),
    "Displacer Kitten",
    CardArt::new("9a53e8fc-bfd2-4866-a61c-f3204b0a98bf", "Campbell White"),
    CardSet::CommanderLegendsBattleForBaldursGate,
    // Four mana for a 2/2 that does nothing on its own and everything in a
    // deck built to cast noncreature spells: every one of them is another
    // enter trigger off whatever is already on the battlefield.
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Cat", "Beast"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "Avoidance — Whenever you cast a noncreature spell, exile up to one target nonland \
             permanent you control, then return that card to the battlefield under its owner's \
             control.",
            TriggerEventDef::SpellCast(A_NONCREATURE_SPELL_YOU_CAST),
            &UP_TO_ONE_NONLAND_PERMANENT_YOU_CONTROL,
            EffectDef::Sequence(&KITTEN_BLINKS),
        ),
    ),
);

// CLB 630 — Delayed Blast Fireball
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

// CLB 748 — Dauthi Horror
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DAUTHI_HORROR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c5a8bb3a-3a84-442f-8e31-8af2f04408ab"),
    "Dauthi Horror",
    crate::card::CardArt::new("7c41afe6-7eed-4cf5-9bbb-ccc9f82cb4fa", "Jeff Laubenstein"),
    crate::card::CardSet::CommanderLegendsBattleForBaldursGate,
    crate::card::CardRules::unsupported(),
);

// CLB 897 — Izzet Boilerworks
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static IZZET_BOILERWORKS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("666f455e-3a3d-475d-b67a-a1fdd74820eb"),
    "Izzet Boilerworks",
    crate::card::CardArt::new("c86e42c6-342b-443f-9b99-a68cf536ff45", "John Avon"),
    crate::card::CardSet::CommanderLegendsBattleForBaldursGate,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BLESSED_HIPPOGRIFF,
    &GREATSWORD_OF_TYR,
    &SWORD_COAST_SERPENT,
    &YOUNG_BLUE_DRAGON,
    &ARMS_OF_HADAR,
    &CAST_DOWN,
    &GUILDSWORN_PROWLER,
    &GUT_TRUE_SOUL_ZEALOT,
    &YOU_MEET_IN_A_TAVERN,
    &MINSC_BOO_TIMELESS_HEROES,
    &DISPLACER_KITTEN,
    &DELAYED_BLAST_FIREBALL,
    &DAUTHI_HORROR,
    &IZZET_BOILERWORKS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
