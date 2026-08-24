//! Tarkir: Dragonstorm cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    ChoiceVisibilityDef, ChooseDef, ComparisonDef, CounterKind, CreatedTokensDef, EffectDef,
    EffectPaymentDef, EffectRecipientDef, InstalledTriggerDef, ManaColor, ObjectChoiceBindingDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectSetDef, PayOrDef, PlayActionMatcherDef,
    PlayRestrictionDef, PlayerRefDef, PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::ids::{ObjectSetBindingIndex, TargetIndex};
use crate::mana_cost;

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

static UGIN_EXILES_IT: EffectDef = EffectDef::MoveToZone {
    counters: None,
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    zone: ZoneKind::Exile,
    placement: ZonePlacement::Top,
    controller: None,
    arrival_effect: None,
    attachment: None,
};

/// A colorless spell you cast, which is every spell the deck around him is
/// made of. His own cast is not one of these: he is still on the stack, and
/// this clause is read off the battlefield.
static A_COLORLESS_SPELL_YOU_CAST: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::ColorCount(0),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

static UGIN_GAINS_AND_DRAWS: [EffectDef; 2] = [
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(3),
    },
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
];

/// "Until end of turn, you may cast those cards without paying their mana
/// costs": the cards the search just exiled, named by what it bound rather
/// than by anything about exile, since a card that was already there is not
/// one of them.
static UGIN_MAY_CAST_THEM: EffectDef = EffectDef::MayPlayWithoutPaying {
    objects: ObjectSetDef::Binding(ObjectSetBindingIndex::PRIMARY),
};

/// "Any number": the bound is the library, so the search offers everything
/// that matches and takes as many as its controller wants.
static UGIN_SEARCH: EffectDef = EffectDef::SearchZone {
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
    binding: Some(ObjectSetBindingIndex::PRIMARY),
    then: Some(&UGIN_MAY_CAST_THEM),
};

static UGIN_ABILITIES: [AbilityDef; 5] = [
    AbilityDef::triggered_with_targets(
        "When you cast this spell, exile up to one target permanent that's one or more colors.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::Source),
        &UP_TO_ONE_COLORED_PERMANENT,
        UGIN_EXILES_IT,
    ),
    AbilityDef::triggered_with_targets(
        "Whenever you cast a colorless spell, exile up to one target permanent that's one or \
         more colors.",
        TriggerEventDef::SpellCast(A_COLORLESS_SPELL_YOU_CAST),
        &UP_TO_ONE_COLORED_PERMANENT,
        UGIN_EXILES_IT,
    ),
    AbilityDef::activated(
        "+2: You gain 3 life and draw a card.",
        &[AbilityCostDef::Loyalty(2)],
        EffectDef::Sequence(&UGIN_GAINS_AND_DRAWS),
    ),
    // A loyalty ability that makes mana is still a mana ability: it never
    // uses the stack, and it is still the one loyalty ability he may use
    // this turn.
    AbilityDef::activated_mana(
        "0: Add {C}{C}{C}.",
        &[AbilityCostDef::Loyalty(0)],
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(3)),
    ),
    AbilityDef::activated(
        "\u{2212}11: Search your library for any number of colorless nonland cards, exile them, \
         then shuffle. Until end of turn, you may cast those cards without paying their mana \
         costs.",
        &[AbilityCostDef::Loyalty(-11)],
        UGIN_SEARCH,
    ),
];

pub(in crate::card::sets) static UGIN_EYE_OF_THE_STORMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("64a5d494-efa1-446b-bebe-2ad36e154376"),
    "Ugin, Eye of the Storms",
    CardArt::new("64a5d494-efa1-446b-bebe-2ad36e154376", "Joshua Raphael"),
    CardSet::TarkirDragonstorm,
    // Seven mana that answers something the moment it is cast and again for
    // every colorless spell after it, pays for the next one itself, and
    // eventually empties the library onto the table for free.
    CardRules::new_planeswalker(mana_cost!("{7}"), &["Ugin"], 7)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&UGIN_ABILITIES),
);

// TDM 8 — Descendant of Storms
/// "It endures 1": the counter or the Spirit, and the attacking body is
/// what either one is about.
static DESCENDANT_ENDURES: EffectDef = EffectDef::Endure {
    object: EffectRecipientDef::Source,
    amount: ValueDef::Constant(1),
};

pub(in crate::card::sets) static DESCENDANT_OF_STORMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f632be90-9e7f-41f8-a52e-a2952354d730"),
    "Descendant of Storms",
    CardArt::new("f632be90-9e7f-41f8-a52e-a2952354d730", "Lie Setiawan"),
    CardSet::TarkirDragonstorm,
    // A one-mana 2/1 that attacks well early and has somewhere to put mana
    // late. Which half of endure you want changes with the board: the
    // counter makes the attack bigger, the Spirit makes the next one wider.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature attacks, you may pay {1}{W}. If you do, it endures 1.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef::mana(
                    PlayerSetDef::Related(PlayerRelation::You),
                    mana_cost!("{1}{W}"),
                ),
                &DESCENDANT_ENDURES,
            )),
        ),
    ),
);

// TDM 12 — Fortress Kin-Guard
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FORTRESS_KIN_GUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b647a018-1d70-43a1-a265-928bcd863689"),
    "Fortress Kin-Guard",
    crate::card::CardArt::new("b647a018-1d70-43a1-a265-928bcd863689", "Daneen Wilkerson"),
    crate::card::CardSet::TarkirDragonstorm,
    crate::card::CardRules::unsupported(),
);

// TDM 21 — Riling Dawnbreaker
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RILING_DAWNBREAKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("312f7072-3bf8-449f-bfb7-93727ef26c66"),
    "Riling Dawnbreaker",
    crate::card::CardArt::new("312f7072-3bf8-449f-bfb7-93727ef26c66", "Tuan Duong Chu"),
    crate::card::CardSet::TarkirDragonstorm,
    crate::card::CardRules::unsupported(),
);

// TDM 23 — Salt Road Packbeast
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SALT_ROAD_PACKBEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("98d548c9-42bc-4155-8211-0aea801c3724"),
    "Salt Road Packbeast",
    crate::card::CardArt::new("98d548c9-42bc-4155-8211-0aea801c3724", "Ben Wootten"),
    crate::card::CardSet::TarkirDragonstorm,
    crate::card::CardRules::unsupported(),
);

// TDM 33 — Voice of Victory
/// The tokens go away at the next end step, and it has to be exactly the
/// ones this attack made: by then nothing about the board could tell them
/// apart from the pair the last attack made, or from a Warrior that arrived
/// some other way. So they are bound as they are created and the delayed
/// clause names the binding.
static MOBILIZE_SACRIFICE: EffectDef =
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
        "At the beginning of the next end step, sacrifice those tokens.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::Any,
        },
        EffectDef::Sacrifice {
            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                ObjectSetBindingIndex::PRIMARY,
            )),
        },
    )));

/// Mobilize 2 (CR 702.180a). Written out rather than abbreviated: the
/// keyword is a shorthand for a triggered ability, and this is that ability.
static MOBILIZE_TWO: AbilityDef = AbilityDef::triggered(
    "Mobilize 2 (Whenever this creature attacks, create two tapped and attacking 1/1 red Warrior \
     creature tokens. Sacrifice them at the beginning of the next end step.)",
    TriggerEventDef::attacks(ObjectPredicateDef::Source),
    EffectDef::create_creature_token(&["Warrior"], &[ManaColor::Red], 1, 1)
        .with_art(CardArt::new(
            "7edc0515-a130-45a7-aa09-0e23bba41587",
            "Forrest Imel",
        ))
        .with_amount(2)
        .entering_tapped()
        .entering_attacking()
        .with_created_tokens(CreatedTokensDef {
            binding: ObjectSetBindingIndex::PRIMARY,
            then: &MOBILIZE_SACRIFICE,
        }),
);

static NO_SPELLS: PlayRestrictionDef =
    PlayRestrictionDef::new(PlayActionMatcherDef::CastSpell, ObjectPredicateDef::Any);

/// "During your turn" is the whole of the clause's timing, and it gates the
/// restriction rather than narrowing who it names: on their own turn the
/// same opponents may cast whatever they like.
static SILENCE_ON_YOUR_TURN: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::Opponent)),
    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(NO_SPELLS)),
};

static VOICE_OF_VICTORY_ABILITIES: [AbilityDef; 2] = [
    MOBILIZE_TWO,
    AbilityDef::static_ability(
        "Your opponents can't cast spells during your turn.",
        EffectDef::IfCondition {
            condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
            then: &SILENCE_ON_YOUR_TURN,
        },
    ),
];

pub(in crate::card::sets) static VOICE_OF_VICTORY: CardRecord = CardRecord::new_with_legacy_id(
    2282,
    "Voice of Victory",
    CardArt::new("ec3de5f4-bb55-4ab9-995f-f3e0dc22c1bb", "Joshua Cairos"),
    CardSet::TarkirDragonstorm,
    // Two mana that adds two power to every attack and turns off every
    // instant your opponent was holding for the turn you attack.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Bard"], 1, 3)
        .with_abilities(&VOICE_OF_VICTORY_ABILITIES),
);

// TDM 119 — Seize Opportunity
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SEIZE_OPPORTUNITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f7818d28-b9a5-4341-9adc-666070b8878d"),
    "Seize Opportunity",
    crate::card::CardArt::new(
        "f7818d28-b9a5-4341-9adc-666070b8878d",
        "Josiah \"Jo\" Cameron",
    ),
    crate::card::CardSet::TarkirDragonstorm,
    crate::card::CardRules::unsupported(),
);

// TDM 120 — Shock Brigade
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHOCK_BRIGADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("66940466-8e9d-4a85-bfb0-e92189b7a121"),
    "Shock Brigade",
    crate::card::CardArt::new("66940466-8e9d-4a85-bfb0-e92189b7a121", "Fajareka Setiawan"),
    crate::card::CardSet::TarkirDragonstorm,
    crate::card::CardRules::unsupported(),
);

// TDM 127 — Tersa Lightshatter
/// "Discard up to two cards, then draw that many." The size is the player's
/// to choose, so the discard is a choice with a floor of none rather than a
/// fixed number, and what is drawn is however many that turned out to be.
static TERSA_REFILL: [EffectDef; 2] = [
    EffectDef::DiscardCards {
        object: EffectRecipientDef::objects(ObjectSetDef::Binding(ObjectSetBindingIndex::PRIMARY)),
    },
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::BoundObjectCount(ObjectSetBindingIndex::PRIMARY),
    },
];

static TERSA_LOOT: EffectDef = EffectDef::Choose(ChooseDef {
    binding: ObjectChoiceBindingDef::Objects(ObjectSetBindingIndex::PRIMARY),
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
    then: &EffectDef::Sequence(&TERSA_REFILL),
});

/// Seven cards is a real threshold rather than a formality: the attack that
/// turns it on is the one that has already spent a hand.
static SEVEN_IN_YOUR_GRAVEYARD: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::Any,
        &[ZoneKind::Graveyard],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 7,
};

static TERSA_EXILE_AND_PLAY: EffectDef = EffectDef::ExileGrantingControllerPlayThisTurn {
    object: EffectRecipientDef::objects(ObjectSetDef::Binding(ObjectSetBindingIndex::PRIMARY)),
};

static TERSA_ABILITIES: [AbilityDef; 3] = [
    abilities::haste(),
    abilities::enters_trigger(
        "When Tersa Lightshatter enters, discard up to two cards, then draw that many cards.",
        TERSA_LOOT,
    ),
    AbilityDef::triggered_if(
        "Whenever Tersa Lightshatter attacks, if there are seven or more cards in your graveyard, \
         exile a card at random from your graveyard. You may play that card this turn.",
        TriggerEventDef::attacks(ObjectPredicateDef::Source),
        &SEVEN_IN_YOUR_GRAVEYARD,
        EffectDef::SelectAtRandomFromZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Graveyard,
            object: ObjectPredicateDef::Any,
            binding: ObjectSetBindingIndex::PRIMARY,
            then: &TERSA_EXILE_AND_PLAY,
        },
    ),
];

pub(in crate::card::sets) static TERSA_LIGHTSHATTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("39f07b5b-d764-4c88-920b-36b0ba1c62b0"),
    "Tersa Lightshatter",
    CardArt::new("39f07b5b-d764-4c88-920b-36b0ba1c62b0", "Olivier Bernard"),
    CardSet::TarkirDragonstorm,
    // Three mana for a 3/3 that attacks immediately and turns a spent hand
    // into a card a turn. What she asks for is the graveyard the deck was
    // filling anyway.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Orc", "Wizard"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&TERSA_ABILITIES),
);

// TDM 134 — Ainok Wayfarer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AINOK_WAYFARER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57695a9b-8f72-4ccc-a946-5d5037b09b8f"),
    "Ainok Wayfarer",
    crate::card::CardArt::new("57695a9b-8f72-4ccc-a946-5d5037b09b8f", "Filipe Pagliuso"),
    crate::card::CardSet::TarkirDragonstorm,
    crate::card::CardRules::unsupported(),
);

// TDM 137 — Champion of Dusan
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHAMPION_OF_DUSAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c51dcdab-38ee-4804-8859-09adc353c182"),
    "Champion of Dusan",
    crate::card::CardArt::new("c51dcdab-38ee-4804-8859-09adc353c182", "Bastien L. Deharme"),
    crate::card::CardSet::TarkirDragonstorm,
    crate::card::CardRules::unsupported(),
);

// TDM 157 — Sagu Wildling
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SAGU_WILDLING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b72ee8f9-5e79-4f77-ae7e-e4c274f78187"),
    "Sagu Wildling",
    crate::card::CardArt::new("d8b43b00-f4d1-436c-bf3f-6d414cd4ce38", "Gaboleps"),
    crate::card::CardSet::TarkirDragonstorm,
    crate::card::CardRules::unsupported(),
);

// TDM 343 — Cori-Steel Cutter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CORI_STEEL_CUTTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("470dd3c8-07c9-42ef-aa9e-3c73b23607ff"),
    "Cori-Steel Cutter",
    crate::card::CardArt::new("470dd3c8-07c9-42ef-aa9e-3c73b23607ff", "Tomas Duchek"),
    crate::card::CardSet::TarkirDragonstorm,
    crate::card::CardRules::unsupported(),
);

static ELSPETH_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

static ELSPETH_FLYING: AbilityDef = abilities::flying();

/// "Those creatures" is the set the counters went on. Nothing can join or
/// leave the battlefield between the two halves of one resolution, so
/// naming the same query twice names the same creatures -- and unlike a
/// binding it says outright that they are on the battlefield.
static ELSPETH_ANTHEM_STEPS: [EffectDef; 2] = [
    EffectDef::AddCounters {
        object: EffectRecipientDef::objects(ObjectSetDef::Query(ELSPETH_CREATURES)),
        kind: CounterKind::PlusOnePlusOne,
        amount: ValueDef::Constant(1),
    },
    EffectDef::Apply {
        recipient: EffectRecipientDef::objects(ObjectSetDef::Query(ELSPETH_CREATURES)),
        effect: AppliedEffectDef::add_ability(&ELSPETH_FLYING),
        duration: ResolvedEffectDurationDef::UntilYourNextTurn,
    },
];

static ELSPETH_ANTHEM: EffectDef = EffectDef::Sequence(&ELSPETH_ANTHEM_STEPS);

/// "Mana value 3 or greater", which for a whole number is everything that is
/// not two or less.
static A_BIG_CREATURE_AN_OPPONENT_CONTROLS: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(2)),
            ]),
            zones: &[ZoneKind::Battlefield],
            controller: Some(PlayerRelation::Opponent),
            owner: None,
        },
    )];

static ELSPETH_PLUS_ONE_COST: [AbilityCostDef; 1] = [AbilityCostDef::Loyalty(1)];
static ELSPETH_ZERO_COST: [AbilityCostDef; 1] = [AbilityCostDef::Loyalty(0)];
static ELSPETH_MINUS_THREE_COST: [AbilityCostDef; 1] = [AbilityCostDef::Loyalty(-3)];

static ELSPETH_ABILITIES: [AbilityDef; 4] = [
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
        &ELSPETH_PLUS_ONE_COST,
        EffectDef::create_creature_token(&["Soldier"], &[ManaColor::White], 1, 1),
    ),
    AbilityDef::activated(
        "0: Put a +1/+1 counter on each creature you control. Those creatures gain flying until \
         your next turn.",
        &ELSPETH_ZERO_COST,
        ELSPETH_ANTHEM,
    ),
    AbilityDef::activated_with_targets(
        "−3: Destroy target creature an opponent controls with mana value 3 or greater.",
        &ELSPETH_MINUS_THREE_COST,
        &A_BIG_CREATURE_AN_OPPONENT_CONTROLS,
        EffectDef::destroy_target(TargetIndex::PRIMARY, true),
    ),
];

// TDM 398 — Elspeth, Storm Slayer
pub(in crate::card::sets) static ELSPETH_STORM_SLAYER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1fdf9438-fd5f-4638-8f41-dae35ae8f257"),
    "Elspeth, Storm Slayer",
    CardArt::new("1fdf9438-fd5f-4638-8f41-dae35ae8f257", "Jeremy Wilson"),
    CardSet::TarkirDragonstorm,
    // Five mana whose first line is worth more than the three below it: in a
    // deck that makes tokens at all, everything it was already doing happens
    // twice.
    CardRules::new_planeswalker(mana_cost!("{3}{W}{W}"), &["Elspeth"], 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&ELSPETH_ABILITIES),
);

// TDM 409 — Ugin, Eye of the Storms (alternate printing)

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

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&UGIN_EYE_OF_THE_STORMS, 1), // TDM 409
];
