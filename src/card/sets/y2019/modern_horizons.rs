//! Modern Horizons cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef,
    AddManaEffectDef, AlternativeCastKindDef, AppliedEffectDef, AppliedRuleDef, CardArt, CardRules,
    CardSet, CardSupertype, CardType, ColorChoiceOperationDef, ComparisonDef, CounterKind,
    DiscardFollowUpDef, DiscardSelectionDef, EffectDef, EffectRecipientDef, EmblemCharacteristics,
    ExilePlayDurationDef, ManaColor, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef,
    ObjectSetDef, PlayerRefDef, PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef,
    SpellAdditionalCostDef, SpellResolutionDestinationDef, SpendModeDef, TokenCharacteristics,
    TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities, tokens,
};
use crate::ids::ObjectSetBindingIndex;
use crate::{TargetIndex, mana_cost};

/// "If it's not your turn" gates only the free cast. The printed cost is
/// always available, which is why this is a condition on the alternative
/// rather than a restriction on the card.
static NOT_YOUR_TURN: TriggerConditionDef =
    TriggerConditionDef::ActivePlayer(PlayerRelation::Opponent);

// MH1 7 — Ephemerate
/// The blink, and then rebound's own clause. Exiling links the creature to
/// the spell, which is what lets the return name the card it just made.
static EPHEMERATE_BLINKS: [EffectDef; 3] = [
    EffectDef::ExileLinkedToSource {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        face_down: false,
        then: None,
    },
    EffectDef::ReturnLinkedExiles {
        object: ObjectPredicateDef::Any,
        counters: None,
        arrival_effect: None,
        zone: ZoneKind::Battlefield,
        grant: None,
        controller: None,
        transformed: false,
    },
    abilities::rebound_offer(),
];

static A_CREATURE_YOU_CONTROL: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

pub(in crate::card::sets) static EPHEMERATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2da5f3f8-5eef-498f-ba2c-2f3fbc3745aa"),
    "Ephemerate",
    CardArt::new("2da5f3f8-5eef-498f-ba2c-2f3fbc3745aa", "Bastien L. Deharme"),
    CardSet::ModernHorizons1,
    // One white mana for two enter triggers, a turn apart. What it costs is
    // that the creature has to survive until the second one.
    CardRules::new_instant(mana_cost!("{W}")).with_ability(
        AbilityDef::spell_with_targets(
            "Exile target creature you control, then return it to the battlefield under its \
             owner's control.",
            &A_CREATURE_YOU_CONTROL,
            EffectDef::Sequence(&EPHEMERATE_BLINKS),
        )
        .with_resolution_destination(SpellResolutionDestinationDef::ExileIfCastFromHand),
    ),
);

// MH1 13 — Giver of Runes
/// "Another target creature you control": she may not protect herself, which
/// is the whole difference between her and her mother.
static ANOTHER_CREATURE_YOU_CONTROL: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

static GIVER_OF_RUNES_COST: [AbilityCostDef; 1] = [AbilityCostDef::TapSource];

pub(in crate::card::sets) static GIVER_OF_RUNES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4e117771-5a8b-4812-b487-32ba34b7f724"),
    "Giver of Runes",
    CardArt::new("4e117771-5a8b-4812-b487-32ba34b7f724", "Seb McKinnon"),
    CardSet::ModernHorizons1,
    // Mother of Runes who cannot save herself, and in exchange answers the
    // colourless removal her mother could not.
    CardRules::new_creature(mana_cost!("{W}"), &["Kor", "Cleric"], 1, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Another target creature you control gains protection from colorless or from \
             the color of your choice until end of turn.",
            &GIVER_OF_RUNES_COST,
            &ANOTHER_CREATURE_YOU_CONTROL,
            EffectDef::ChooseColor {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                operation: ColorChoiceOperationDef::ProtectionFromChosenColorOrColorless,
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// MH1 24 — Rhox Veteran
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RHOX_VETERAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6384e266-d0dc-4af1-b3ab-ecaf9be2553c"),
    "Rhox Veteran",
    crate::card::CardArt::new("6384e266-d0dc-4af1-b3ab-ecaf9be2553c", "Milivoj Ćeran"),
    crate::card::CardSet::ModernHorizons1,
    crate::card::CardRules::unsupported(),
);

// MH1 27 — Settle Beyond Reality
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SETTLE_BEYOND_REALITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("72ed8e57-61bb-4e89-9484-ff2be800a449"),
    "Settle Beyond Reality",
    crate::card::CardArt::new("72ed8e57-61bb-4e89-9484-ff2be800a449", "Anthony Palumbo"),
    crate::card::CardSet::ModernHorizons1,
    crate::card::CardRules::unsupported(),
);

// MH1 37 — Winds of Abandon
/// "You don't control" is a constraint on the slot rather than on the object:
/// a spell being cast is not a permanent, so a predicate that compares
/// controllers has nothing to compare against yet.
static WINDS_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::Opponent),
        owner: None,
    },
)];

static WINDS_SINGLE: [EffectDef; 2] = [
    EffectDef::MoveToZone {
        counters: None,
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        from: None,
        zone: ZoneKind::Exile,
        controller: None,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        tapped: false,
    },
    // The searcher is the creature's controller, read from the announced
    // target: by now the creature is in exile and cannot be asked.
    EffectDef::SearchZone {
        player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
            TargetIndex::PRIMARY,
        ))),
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
        binding: None,
        then: None,
    },
];

static WINDS_OVERLOADED_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::Opponent,
);

/// "For each creature exiled this way" counts what the exile actually took,
/// so the set is bound before it is emptied and the search reads the count
/// off that binding rather than off a board the creatures have left.
static WINDS_OVERLOADED_STEPS: [EffectDef; 2] = [
    EffectDef::MoveToZone {
        counters: None,
        object: EffectRecipientDef::objects(ObjectSetDef::Binding(ObjectSetBindingIndex::PRIMARY)),
        from: None,
        zone: ZoneKind::Exile,
        controller: None,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        tapped: false,
    },
    EffectDef::SearchZone {
        player: EffectRecipientDef::Opponent,
        source: ZoneKind::Library,
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Land),
            ObjectPredicateDef::Supertype(CardSupertype::Basic),
        ]),
        minimum: 0,
        maximum: ValueDef::BoundObjectCount(ObjectSetBindingIndex::PRIMARY),
        reveal: false,
        destination: ZoneKind::Battlefield,
        placement: ZonePlacement::Top,
        shuffle: true,
        enters_tapped: true,
        attachment: None,
        binding: None,
        then: None,
    },
];

static WINDS_OVERLOADED: EffectDef = EffectDef::BindMatching {
    objects: ObjectSetDef::Query(WINDS_OVERLOADED_CREATURES),
    binding: ObjectSetBindingIndex::PRIMARY,
    then: &EffectDef::Sequence(&WINDS_OVERLOADED_STEPS),
};

pub(in crate::card::sets) static WINDS_OF_ABANDON: CardRecord = CardRecord::new_with_legacy_id(
    2181,
    "Winds of Abandon",
    CardArt::new("3bb17913-fe4d-4acd-9b75-71f5a90f898b", "Noah Bradley"),
    CardSet::ModernHorizons1,
    // Two mana answers one creature and six answers the board, and neither
    // half leaves anything behind to rebuild from -- exile rather than
    // destruction is the whole reason the card ends games.
    CardRules::new_sorcery(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Exile target creature you don't control. For each creature exiled this way, its controller searches their library for a basic land card. Those players put those cards onto the battlefield tapped, then shuffle.",
            &WINDS_TARGET,
            EffectDef::Sequence(&WINDS_SINGLE),
        ),
        AbilityDef::alternative_cast(
            mana_cost!("{4}{W}{W}"),
            AlternativeCastKindDef::Overload,
            Some("Exile each creature you don't control. For each creature exiled this way, its controller searches their library for a basic land card. Those players put those cards onto the battlefield tapped, then shuffle."),
            WINDS_OVERLOADED,
        ),
    ]),
);

// MH1 46 — Echo of Eons
pub(in crate::card::sets) static ECHO_OF_EONS: CardRecord = CardRecord::new_with_legacy_id(
    2278,
    "Echo of Eons",
    CardArt::new("ff590af2-2d6c-4f16-a9b8-1a6dab6e9ad5", "Terese Nielsen"),
    CardSet::ModernHorizons1,
    // Six mana nobody pays: the card is here for the flashback, which turns a
    // graveyard full of rituals into a fresh seven for three.
    CardRules::new_sorcery(mana_cost!("{4}{U}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Each player shuffles their hand and graveyard into their library, then draws seven \
             cards.",
            abilities::shuffle_back_and_draw_seven(),
        ),
        abilities::flashback(mana_cost!("{2}{U}")),
    ]),
);

// MH1 51 — Faerie Seer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FAERIE_SEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d1fcfeb4-1818-4e08-be4c-27b8a9dc12e6"),
    "Faerie Seer",
    crate::card::CardArt::new("d1fcfeb4-1818-4e08-be4c-27b8a9dc12e6", "Colin Boyer"),
    crate::card::CardSet::ModernHorizons1,
    crate::card::CardRules::unsupported(),
);

// MH1 52 — Force of Negation
/// Exiled rather than discarded, the same way the green half of the cycle
/// spends its card: what pays is gone without ever becoming a graveyard
/// card.
static EXILE_A_BLUE_CARD: SpellAdditionalCostDef = SpellAdditionalCostDef::new(
    ObjectPredicateDef::Color(ManaColor::Blue),
    ZoneKind::Hand,
    1,
)
.spent(SpendModeDef::Exile);

static A_NONCREATURE_SPELL: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_spell(
    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
)];

pub(in crate::card::sets) static FORCE_OF_NEGATION: CardRecord = CardRecord::new_with_legacy_id(
    2268,
    "Force of Negation",
    CardArt::new("e9be371c-c688-44ad-ab71-bd4c9f242d58", "Paul Scott Canavan"),
    CardSet::ModernHorizons1,
    // Free interaction that only answers the half of the format worth
    // answering for free, and only on the turn somebody else is using it.
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some(
                "If it's not your turn, you may exile a blue card from your hand rather than pay \
                 this spell's mana cost.",
            ),
            EffectDef::None,
        )
        .with_alternative_additional_cost(&EXILE_A_BLUE_CARD)
        .with_alternative_condition(&NOT_YOUR_TURN),
        AbilityDef::spell_with_targets(
            "Counter target noncreature spell. If that spell is countered this way, exile it \
             instead of putting it into its owner's graveyard.",
            &A_NONCREATURE_SPELL,
            // The destination is part of the counter rather than a second
            // clause: a spell countered this way never reaches a graveyard,
            // so nothing watching one sees it arrive.
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// MH1 55 — Man-o'-War
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MAN_O_WAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4dbf9bf9-75cd-4b25-a3a1-43b7e029700b"),
    "Man-o'-War",
    crate::card::CardArt::new("5eaa4199-df9b-494a-af7a-2491e8b0ef70", "Jon J Muth"),
    crate::card::CardSet::ModernHorizons1,
    crate::card::CardRules::unsupported(),
);

// MH1 75 — Urza, Lord High Artificer
/// "This token gets +1/+1 for each artifact you control", which counts the
/// token itself: a lone Construct is a 1/1, and every artifact after it is
/// another point in both directions.
static ARTIFACTS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Artifact),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

static CONSTRUCT_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
    "This token gets +1/+1 for each artifact you control.",
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::modify_power_toughness(
            ValueDef::CountMatchingObjects(&ARTIFACTS_YOU_CONTROL),
            ValueDef::CountMatchingObjects(&ARTIFACTS_YOU_CONTROL),
        ),
    },
)];

static URZA_CONSTRUCT: TokenCharacteristics =
    TokenCharacteristics::artifact_creature(&["Construct"], &[], 0, 0)
        .with_abilities(&CONSTRUCT_ABILITIES);

/// "Tap an untapped artifact you control", which the Construct itself
/// answers -- and so does every Mox, every Lotus, and everything they made.
static TAP_AN_ARTIFACT: AbilityCostDef = AbilityCostDef::TapPermanent {
    object: ObjectPredicateDef::HasType(CardType::Artifact),
    controller: PlayerRelation::You,
};

static URZA_DIG: [EffectDef; 2] = [
    EffectDef::ShuffleLibrary {
        player: EffectRecipientDef::Controller,
    },
    EffectDef::ExileTopOfLibraryToPlay {
        player: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
        free: true,
        face_down: false,
        duration: ExilePlayDurationDef::ThisTurn,
        spend_any_color: false,
        play_condition: None,
    },
];

pub(in crate::card::sets) static URZA_LORD_HIGH_ARTIFICER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e7fb3c0-5159-4d1f-8490-ce4c9a60f567"),
    "Urza, Lord High Artificer",
    CardArt::new("9e7fb3c0-5159-4d1f-8490-ce4c9a60f567", "Grzegorz Rutkowski"),
    CardSet::ModernHorizons1,
    // Four mana for a body, a blue mana out of every artifact you have, and
    // a mana sink that turns the rest of them into a free card.
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Human", "Artificer"], 1, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When this creature enters, create a 0/0 colorless Construct artifact creature \
                 token with \"This token gets +1/+1 for each artifact you control.\"",
                EffectDef::create_token(URZA_CONSTRUCT).with_art(CardArt::new(
                    "85f212cd-4fc6-42fe-b268-22d8e3b2b7eb",
                    "Victor Adame Minguez",
                )),
            ),
            AbilityDef::activated(
                "Tap an untapped artifact you control: Add {U}.",
                &[TAP_AN_ARTIFACT],
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
            ),
            AbilityDef::activated(
                "{5}: Shuffle your library, then exile the top card. Until end of turn, you may \
                 play that card without paying its mana cost.",
                &[AbilityCostDef::Mana(mana_cost!("{5}"))],
                EffectDef::Sequence(&URZA_DIG),
            ),
        ]),
);

// MH1 81 — Carrion Feeder
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CARRION_FEEDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("88042031-64af-4f84-85d5-95992b43aa6c"),
    "Carrion Feeder",
    crate::card::CardArt::new("0a19da90-880e-4eca-8cf7-6d7baf090d53", "Svetlin Velinov"),
    crate::card::CardSet::ModernHorizons1,
    crate::card::CardRules::unsupported(),
);

// MH1 91 — First-Sphere Gargantua
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FIRST_SPHERE_GARGANTUA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a59f4e5c-fdc7-485f-aadb-2a71b3701dcc"),
    "First-Sphere Gargantua",
    crate::card::CardArt::new("a59f4e5c-fdc7-485f-aadb-2a71b3701dcc", "Randy Vargas"),
    crate::card::CardSet::ModernHorizons1,
    crate::card::CardRules::unsupported(),
);

// MH1 101 — Putrid Goblin
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PUTRID_GOBLIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("333406d5-abcc-4629-a33b-395d0662ba1b"),
    "Putrid Goblin",
    crate::card::CardArt::new("333406d5-abcc-4629-a33b-395d0662ba1b", "Winona Nelson"),
    crate::card::CardSet::ModernHorizons1,
    crate::card::CardRules::unsupported(),
);

// MH1 120 — Bogardan Dragonheart
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BOGARDAN_DRAGONHEART: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("feb81f44-8f22-4d28-a452-a50bef69a3e3"),
    "Bogardan Dragonheart",
    crate::card::CardArt::new("feb81f44-8f22-4d28-a452-a50bef69a3e3", "Randy Vargas"),
    crate::card::CardSet::ModernHorizons1,
    crate::card::CardRules::unsupported(),
);

// MH1 144 — Reckless Charge
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RECKLESS_CHARGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0938e686-345e-4411-b564-cf9324ec6b9d"),
    "Reckless Charge",
    crate::card::CardArt::new("1754a8db-060e-470f-94c0-37f12d82978a", "Steve Argyle"),
    crate::card::CardSet::ModernHorizons1,
    crate::card::CardRules::unsupported(),
);

// MH1 145 — Seasoned Pyromancer
/// A 1/1 red Elemental, which is what both halves of him make: the arrival
/// pays one per nonland card it threw away, and the graveyard ability pays
/// two flat.
static PYROMANCER_ELEMENTAL: TokenCharacteristics =
    tokens::creature(&["Elemental"], &[ManaColor::Red], 1, 1);

static PYROMANCER_MAKES_TOKENS: EffectDef = EffectDef::CreateToken {
    token: PYROMANCER_ELEMENTAL,
    copy: None,
    controller: None,
    count: ValueDef::MatchedCount,
    tapped: false,
    attacking: false,
    counters: None,
    created: None,
};

/// The draw comes before the tokens are counted, which is what the printed
/// order says: two cards go, two cards come, and only then does the board
/// pay you back for the ones that were not lands.
static PYROMANCER_DRAWS_THEN_PAYS: [EffectDef; 2] = [
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    },
    PYROMANCER_MAKES_TOKENS,
];

static PYROMANCER_DRAWS_THEN_PAYS_SEQUENCE: EffectDef =
    EffectDef::Sequence(&PYROMANCER_DRAWS_THEN_PAYS);

static PYROMANCER_GRAVEYARD_COST: [AbilityCostDef; 2] = [
    AbilityCostDef::Mana(mana_cost!("{3}{R}{R}")),
    AbilityCostDef::ExileSource,
];

static PYROMANCER_MAKES_TWO: EffectDef = EffectDef::CreateToken {
    token: PYROMANCER_ELEMENTAL,
    copy: None,
    controller: None,
    count: ValueDef::Constant(2),
    tapped: false,
    attacking: false,
    counters: None,
    created: None,
};

static SEASONED_PYROMANCER_ABILITIES: [AbilityDef; 2] = [
    abilities::enters_trigger(
        "When this creature enters, discard two cards, then draw two cards. For each nonland card \
         discarded this way, create a 1/1 red Elemental creature token.",
        EffectDef::Discard {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
            selection: DiscardSelectionDef::RecipientChooses,
            then: Some(DiscardFollowUpDef {
                counted: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                bound: None,
                effect: &PYROMANCER_DRAWS_THEN_PAYS_SEQUENCE,
            }),
        },
    ),
    // The card is spent from the graveyard, which is why he is never a dead
    // draw late: the body was the first half and this is the second.
    AbilityDef::activated(
        "{3}{R}{R}, Exile this card from your graveyard: Create two 1/1 red Elemental creature \
         tokens.",
        &PYROMANCER_GRAVEYARD_COST,
        PYROMANCER_MAKES_TWO,
    )
    .with_source_zones(&[ZoneKind::Graveyard]),
];

pub(in crate::card::sets) static SEASONED_PYROMANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2e139ad1-1079-49e9-babd-6399c44ad333"),
    "Seasoned Pyromancer",
    CardArt::new("2e139ad1-1079-49e9-babd-6399c44ad333", "Cynthia Sheppard"),
    CardSet::ModernHorizons1,
    // Three mana that turns the two worst cards in your hand into two fresh
    // ones and a body for each of them that was not a land -- and then does
    // it again from the graveyard.
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Human", "Shaman"], 2, 2)
        .with_abilities(&SEASONED_PYROMANCER_ABILITIES),
);

// MH1 158 — Collector Ouphe
pub(in crate::card::sets) static COLLECTOR_OUPHE: CardRecord = CardRecord::new_with_legacy_id(
    2284,
    "Collector Ouphe",
    CardArt::new("085107a2-c1ec-473c-81d8-23e5a7197776", "Filip Burburan"),
    CardSet::ModernHorizons1,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Ouphe"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "Activated abilities of artifacts can't be activated.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotActivateAbilities),
            },
        ),
    ),
);

// MH1 164 — Force of Vigor
/// Exiled rather than discarded: the card is spent without ever becoming a
/// graveyard card, which is what "exile a green card" means.
static EXILE_A_GREEN_CARD: SpellAdditionalCostDef = SpellAdditionalCostDef::new(
    ObjectPredicateDef::Color(ManaColor::Green),
    ZoneKind::Hand,
    1,
)
.spent(SpendModeDef::Exile);

static FORCE_OF_VIGOR_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Enchantment),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
    2,
)];

pub(in crate::card::sets) static FORCE_OF_VIGOR: CardRecord = CardRecord::new_with_legacy_id(
    2127,
    "Force of Vigor",
    CardArt::new("017c415b-d635-43c6-92b8-8c95d1c4ff8d", "Randy Vargas"),
    CardSet::ModernHorizons1,
    CardRules::new_instant(mana_cost!("{2}{G}{G}")).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some(
                "If it's not your turn, you may exile a green card from your hand rather than pay this spell's mana cost.",
            ),
            EffectDef::None,
        )
        .with_alternative_additional_cost(&EXILE_A_GREEN_CARD)
        .with_alternative_condition(&NOT_YOUR_TURN),
        AbilityDef::spell_with_targets(
            "Destroy up to two target artifacts and/or enchantments.",
            &FORCE_OF_VIGOR_TARGETS,
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ]),
);

// MH1 168 — Hexdrinker
/// A level band is a continuous effect that applies while the permanent's
/// level is inside it (CR 711.4a), so each band is a static ability whose
/// subject is its own source and whose condition is the count of level
/// counters on it. The bands do not overlap: the first ends where the second
/// begins, which is why the lower one names a top as well as a bottom.
static HEXDRINKER_AT_LEAST_THREE: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::Source,
    ObjectPredicateDef::CounterCount {
        kind: CounterKind::Level,
        comparison: ComparisonDef::GreaterOrEqual,
        amount: 3,
    },
    ObjectPredicateDef::CounterCount {
        kind: CounterKind::Level,
        comparison: ComparisonDef::Less,
        amount: 8,
    },
]);

static HEXDRINKER_AT_LEAST_EIGHT: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::Source,
    ObjectPredicateDef::CounterCount {
        kind: CounterKind::Level,
        comparison: ComparisonDef::GreaterOrEqual,
        amount: 8,
    },
]);

static PROTECTION_FROM_INSTANTS: AbilityDef = AbilityDef::keyword(
    "Protection from instants",
    crate::card::KeywordAbility::ProtectionFrom(&ObjectPredicateDef::HasType(CardType::Instant)),
);

/// The reward for eight activations: nothing may block it, target it, damage
/// it, or enchant it.
static PROTECTION_FROM_EVERYTHING: AbilityDef = AbilityDef::keyword(
    "Protection from everything",
    crate::card::KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Any),
);

static HEXDRINKER_LEVEL_THREE: [AppliedEffectDef; 2] = [
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(4), ValueDef::Constant(4)),
    AppliedEffectDef::add_ability(&PROTECTION_FROM_INSTANTS),
];

static HEXDRINKER_LEVEL_EIGHT: [AppliedEffectDef; 2] = [
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(6), ValueDef::Constant(6)),
    AppliedEffectDef::add_ability(&PROTECTION_FROM_EVERYTHING),
];

static HEXDRINKER_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::activated(
        "Level up {1} ({1}: Put a level counter on this. Level up only as a sorcery.)",
        &[AbilityCostDef::Mana(mana_cost!("{1}"))],
        EffectDef::AddCounters {
            object: EffectRecipientDef::Source,
            kind: CounterKind::Level,
            amount: ValueDef::Constant(1),
        },
    )
    .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    AbilityDef::static_ability(
        "LEVEL 3-7: 4/4, protection from instants",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                HEXDRINKER_AT_LEAST_THREE,
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Composite(&HEXDRINKER_LEVEL_THREE),
        },
    ),
    AbilityDef::static_ability(
        "LEVEL 8+: 6/6, protection from everything",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                HEXDRINKER_AT_LEAST_EIGHT,
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Composite(&HEXDRINKER_LEVEL_EIGHT),
        },
    ),
];

pub(in crate::card::sets) static HEXDRINKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("89f5cc05-5d9d-4709-b3c5-a6249c294acc"),
    "Hexdrinker",
    crate::card::CardArt::new("89f5cc05-5d9d-4709-b3c5-a6249c294acc", "Forrest Imel"),
    crate::card::CardSet::ModernHorizons1,
    // One mana for a 2/1, and every spare mana afterwards buys a step toward
    // a creature nothing in the deck can answer.
    CardRules::new_creature(mana_cost!("{G}"), &["Snake"], 2, 1)
        .with_abilities(&HEXDRINKER_ABILITIES),
);

// MH1 169 — Krosan Tusker
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KROSAN_TUSKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b872f85-60c5-44c4-956d-a8aa8132908b"),
    "Krosan Tusker",
    crate::card::CardArt::new("6391ba8b-7d9a-4077-8eeb-1b2ced14d973", "Kev Walker"),
    crate::card::CardSet::ModernHorizons1,
    crate::card::CardRules::unsupported(),
);

// MH1 171 — Mother Bear
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOTHER_BEAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("efae4d84-8134-461a-a352-a5bdff7259a7"),
    "Mother Bear",
    crate::card::CardArt::new("efae4d84-8134-461a-a352-a5bdff7259a7", "Winona Nelson"),
    crate::card::CardSet::ModernHorizons1,
    crate::card::CardRules::unsupported(),
);

// MH1 187 — Trumpeting Herd
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TRUMPETING_HERD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b0f3b68e-f616-4687-bc2d-075165162cd1"),
    "Trumpeting Herd",
    crate::card::CardArt::new("b0f3b68e-f616-4687-bc2d-075165162cd1", "Lars Grant-West"),
    crate::card::CardSet::ModernHorizons1,
    crate::card::CardRules::unsupported(),
);

// MH1 193 — Winding Way
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WINDING_WAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4e5d9776-b6ce-4ad6-8acc-69115ba5de76"),
    "Winding Way",
    crate::card::CardArt::new("4e5d9776-b6ce-4ad6-8acc-69115ba5de76", "Adam Paquette"),
    crate::card::CardSet::ModernHorizons1,
    crate::card::CardRules::unsupported(),
);

// MH1 199 — Fallen Shinobi
static SHINOBI_NINJUTSU_COST: [AbilityCostDef; 2] = [
    AbilityCostDef::Mana(mana_cost!("{2}{U}{B}")),
    AbilityCostDef::ReturnUnblockedAttackerToHand,
];

static SHINOBI_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::activated(
        "Ninjutsu {2}{U}{B} ({2}{U}{B}, Return an unblocked attacker you control to hand: Put this card onto the battlefield from your hand tapped and attacking.)",
        &SHINOBI_NINJUTSU_COST,
        EffectDef::PutSourceOntoBattlefieldAttacking,
    )
    .with_source_zones(&[ZoneKind::Hand])
    .with_activation_timing(ActivationTimingDef::AfterAttackersDeclared),
    AbilityDef::triggered(
        "Whenever this creature deals combat damage to a player, that player exiles the top two cards of their library. Until end of turn, you may play those cards without paying their mana costs.",
        TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
        EffectDef::ExileTopOfLibraryToPlay {
            player: EffectRecipientDef::EventPlayer,
            amount: ValueDef::Constant(2),
            free: true,
            face_down: false,
            duration: ExilePlayDurationDef::ThisTurn,
            spend_any_color: false,
            play_condition: None,
        },
    ),
];

pub(in crate::card::sets) static FALLEN_SHINOBI: CardRecord = CardRecord::new_with_legacy_id(
    2178,
    "Fallen Shinobi",
    CardArt::new("900c9dfd-ece1-4b09-a801-0fa05e1994b9", "Tomasz Jedruszek"),
    CardSet::ModernHorizons1,
    // Ninjutsu is what makes a five-mana 5/4 connect on turn three, and
    // connecting is the whole card: two cards off the top of their deck,
    // free, every time.
    CardRules::new_creature(mana_cost!("{3}{U}{B}"), &["Zombie", "Ninja"], 5, 4)
        .with_abilities(&SHINOBI_ABILITIES),
);

// MH1 217 — Wrenn and Six
/// Retrace's own cost: the card's mana cost, plus a land out of your hand.
/// Discarding is what an ordinary hand cost does, so nothing else has to be
/// said about how the land is spent.
static DISCARD_A_LAND: SpellAdditionalCostDef = SpellAdditionalCostDef::new(
    ObjectPredicateDef::HasType(CardType::Land),
    ZoneKind::Hand,
    1,
);

static RETRACE: AbilityDef = AbilityDef::alternative_cast_for_card_mana_cost(
    AlternativeCastKindDef::Retrace,
    Some(
        "Retrace (You may cast this card from your graveyard by discarding a land card in \
         addition to paying its other costs.)",
    ),
    EffectDef::None,
)
.with_alternative_additional_cost(&DISCARD_A_LAND);

static AN_INSTANT_OR_SORCERY_CARD: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Instant),
    ObjectPredicateDef::HasType(CardType::Sorcery),
]);

static WRENN_EMBLEM_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
    "Instant and sorcery cards in your graveyard have retrace.",
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
        effect: AppliedEffectDef::Rule(AppliedRuleDef::GrantsAlternativeCastFromGraveyard {
            object: AN_INSTANT_OR_SORCERY_CARD,
            ability: &RETRACE,
        }),
    },
)];

static WRENN_EMBLEM: EmblemCharacteristics =
    EmblemCharacteristics::new("Wrenn and Six emblem", &WRENN_EMBLEM_ABILITIES);

/// "Up to one target land card from your graveyard": a Wrenn with an empty
/// graveyard still ticks up.
static UP_TO_ONE_LAND_IN_YOUR_GRAVEYARD: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Land),
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: Some(PlayerRelation::You),
    },
    1,
)];

static WRENN_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::activated_with_targets(
        "+1: Return up to one target land card from your graveyard to your hand.",
        &[AbilityCostDef::Loyalty(1)],
        &UP_TO_ONE_LAND_IN_YOUR_GRAVEYARD,
        EffectDef::MoveToZone {
            counters: None,
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            from: None,
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
            controller: None,
            tapped: false,
        },
    ),
    AbilityDef::activated_with_targets(
        "−1: This planeswalker deals 1 damage to any target.",
        &[AbilityCostDef::Loyalty(-1)],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(1),
        },
    ),
    AbilityDef::activated(
        "−7: You get an emblem with \"Instant and sorcery cards in your graveyard have retrace.\" \
         (You may cast instant and sorcery cards from your graveyard by discarding a land card in \
         addition to paying their other costs.)",
        &[AbilityCostDef::Loyalty(-7)],
        EffectDef::CreateEmblem {
            emblem: WRENN_EMBLEM,
        },
    ),
];

pub(in crate::card::sets) static WRENN_AND_SIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a706ecf-3277-40e3-871c-4ba4ead16e20"),
    "Wrenn and Six",
    CardArt::new("4a706ecf-3277-40e3-871c-4ba4ead16e20", "Chase Stone"),
    CardSet::ModernHorizons1,
    // Two mana that buys back a fetchland every turn, pings something on the
    // way, and eventually turns the graveyard into a second hand.
    CardRules::new_planeswalker(mana_cost!("{R}{G}"), &["Wrenn"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&WRENN_ABILITIES),
);

// MH1 222 — Farmstead Gleaner
static FARMSTEAD_GLEANER_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::static_ability(
        "This creature doesn't untap during your untap step.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
        },
    ),
    AbilityDef::activated(
        "{2}, {Q}: Put a +1/+1 counter on this creature. ({Q} is the untap symbol.)",
        &[
            AbilityCostDef::Mana(mana_cost!("{2}")),
            AbilityCostDef::UntapSource,
        ],
        EffectDef::AddCounters {
            object: EffectRecipientDef::Source,
            kind: CounterKind::PlusOnePlusOne,
            amount: ValueDef::Constant(1),
        },
    ),
];

pub(in crate::card::sets) static FARMSTEAD_GLEANER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("edafd52f-2dda-4981-baee-404f47ee8969"),
    "Farmstead Gleaner",
    CardArt::new("edafd52f-2dda-4981-baee-404f47ee8969", "Josh Hass"),
    CardSet::ModernHorizons1,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Scarecrow"], 2, 2)
        .with_abilities(&FARMSTEAD_GLEANER_ABILITIES),
);

// MH1 230 — Talisman of Conviction
/// The Talisman cycle's two halves: colorless for nothing, or the pair of
/// colours the card is for at a life apiece. Which colour is chosen belongs
/// to the activation, so the two are one printed ability.
static TALISMAN_COLORS: [ManaColor; 2] = [ManaColor::Red, ManaColor::White];

static TALISMAN_OF_CONVICTION_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::activated_mana(
        "{T}: Add {C}.",
        &TALISMAN_TAP,
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
    ),
    AbilityDef::activated_mana(
        "{T}: Add {R} or {W}. This artifact deals 1 damage to you.",
        &TALISMAN_TAP,
        EffectDef::AddMana(AddManaEffectDef::choice(&TALISMAN_COLORS).with_damage_to_controller(1)),
    ),
];

static TALISMAN_TAP: [AbilityCostDef; 1] = [AbilityCostDef::TapSource];

pub(in crate::card::sets) static TALISMAN_OF_CONVICTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("71148fd3-0c2c-459e-b8f5-735a0a8dd87f"),
    "Talisman of Conviction",
    CardArt::new("71148fd3-0c2c-459e-b8f5-735a0a8dd87f", "Lindsey Look"),
    CardSet::ModernHorizons1,
    // Two mana that fixes for a life a turn, or for nothing at all when
    // colorless is what the next spell wants.
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&TALISMAN_OF_CONVICTION_ABILITIES),
);

// MH1 231 — Talisman of Creativity
static CREATIVITY_COLORS: [ManaColor; 2] = [ManaColor::Blue, ManaColor::Red];

static TALISMAN_OF_CREATIVITY_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::activated_mana(
        "{T}: Add {C}.",
        &TALISMAN_TAP,
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
    ),
    AbilityDef::activated_mana(
        "{T}: Add {U} or {R}. This artifact deals 1 damage to you.",
        &TALISMAN_TAP,
        EffectDef::AddMana(
            AddManaEffectDef::choice(&CREATIVITY_COLORS).with_damage_to_controller(1),
        ),
    ),
];

pub(in crate::card::sets) static TALISMAN_OF_CREATIVITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4d9dbadd-c1b6-44fe-92ac-6f69d7178342"),
    "Talisman of Creativity",
    CardArt::new("4d9dbadd-c1b6-44fe-92ac-6f69d7178342", "Lindsey Look"),
    CardSet::ModernHorizons1,
    // Two mana that fixes for a life a turn, or for nothing at all when
    // colorless is what the next spell wants.
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&TALISMAN_OF_CREATIVITY_ABILITIES),
);

// MH1 232 — Talisman of Curiosity
static CURIOSITY_COLORS: [ManaColor; 2] = [ManaColor::Green, ManaColor::Blue];

static TALISMAN_OF_CURIOSITY_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::activated_mana(
        "{T}: Add {C}.",
        &TALISMAN_TAP,
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
    ),
    AbilityDef::activated_mana(
        "{T}: Add {G} or {U}. This artifact deals 1 damage to you.",
        &TALISMAN_TAP,
        EffectDef::AddMana(
            AddManaEffectDef::choice(&CURIOSITY_COLORS).with_damage_to_controller(1),
        ),
    ),
];

pub(in crate::card::sets) static TALISMAN_OF_CURIOSITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd52688a-39fd-430f-b950-cb56e0004396"),
    "Talisman of Curiosity",
    CardArt::new("fd52688a-39fd-430f-b950-cb56e0004396", "Lindsey Look"),
    CardSet::ModernHorizons1,
    // The Simic half of the cycle: the damage is what pays for a colour, and
    // the colorless mode is what makes it free when colour is not the point.
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&TALISMAN_OF_CURIOSITY_ABILITIES),
);

// MH1 244 — Prismatic Vista
/// "A basic land card", which is the supertype rather than the land types:
/// a dual with two basic types printed on it is not a basic land, and the
/// Vista cannot find one.
static ANY_BASIC_LAND: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Land),
    ObjectPredicateDef::Supertype(CardSupertype::Basic),
]);

pub(in crate::card::sets) static PRISMATIC_VISTA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e37da81e-be12-45a2-9128-376f1ad7b3e8"),
    "Prismatic Vista",
    CardArt::new("e37da81e-be12-45a2-9128-376f1ad7b3e8", "Sam Burley"),
    CardSet::ModernHorizons1,
    // A fetchland for every basic at once, which costs it the fetchland's
    // other half: nothing it finds is a dual, so it fixes colour without
    // paying anybody's land types.
    CardRules::new_land(&[]).with_ability(abilities::fetch_land_ability(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a basic land card, put it \
         onto the battlefield, then shuffle.",
        ANY_BASIC_LAND,
    )),
);

// MH1 247 — Sunbaked Canyon
static SUNBAKED_CANYON_COLORS: [ManaColor; 2] = [ManaColor::Red, ManaColor::White];

static SUNBAKED_CANYON_ABILITIES: [AbilityDef; 2] =
    abilities::horizon_land("{T}, Pay 1 life: Add {R} or {W}.", &SUNBAKED_CANYON_COLORS);

pub(in crate::card::sets) static SUNBAKED_CANYON: CardRecord = CardRecord::new_with_legacy_id(
    2230,
    "Sunbaked Canyon",
    CardArt::new("c36820fa-ee86-4206-9a0d-737a67cf5208", "Yeong-Hao Han"),
    CardSet::ModernHorizons1,
    CardRules::new_land(&[]).with_abilities(&SUNBAKED_CANYON_ABILITIES),
);

// MH1 249 — Waterlogged Grove
static WATERLOGGED_GROVE_COLORS: [ManaColor; 2] = [ManaColor::Green, ManaColor::Blue];

static WATERLOGGED_GROVE_ABILITIES: [AbilityDef; 2] = abilities::horizon_land(
    "{T}, Pay 1 life: Add {G} or {U}.",
    &WATERLOGGED_GROVE_COLORS,
);

pub(in crate::card::sets) static WATERLOGGED_GROVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0ab6bfbd-d2e1-4c4c-9f91-6f69c5b8e3bb"),
    "Waterlogged Grove",
    crate::card::CardArt::new("0ab6bfbd-d2e1-4c4c-9f91-6f69c5b8e3bb", "John Avon"),
    crate::card::CardSet::ModernHorizons1,
    CardRules::new_land(&[]).with_abilities(&WATERLOGGED_GROVE_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &EPHEMERATE,
    &GIVER_OF_RUNES,
    &RHOX_VETERAN,
    &SETTLE_BEYOND_REALITY,
    &WINDS_OF_ABANDON,
    &ECHO_OF_EONS,
    &FAERIE_SEER,
    &FORCE_OF_NEGATION,
    &MAN_O_WAR,
    &URZA_LORD_HIGH_ARTIFICER,
    &CARRION_FEEDER,
    &FIRST_SPHERE_GARGANTUA,
    &PUTRID_GOBLIN,
    &BOGARDAN_DRAGONHEART,
    &RECKLESS_CHARGE,
    &SEASONED_PYROMANCER,
    &COLLECTOR_OUPHE,
    &FORCE_OF_VIGOR,
    &HEXDRINKER,
    &KROSAN_TUSKER,
    &MOTHER_BEAR,
    &TRUMPETING_HERD,
    &WINDING_WAY,
    &FALLEN_SHINOBI,
    &WRENN_AND_SIX,
    &FARMSTEAD_GLEANER,
    &TALISMAN_OF_CONVICTION,
    &TALISMAN_OF_CREATIVITY,
    &TALISMAN_OF_CURIOSITY,
    &PRISMATIC_VISTA,
    &SUNBAKED_CANYON,
    &WATERLOGGED_GROVE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
