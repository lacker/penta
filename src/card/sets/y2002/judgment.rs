//! Judgment cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AlternativeCastKindDef,
    AppliedEffectDef, CardArt, CardRules, CardSet, CardType, CharacteristicOperationDef,
    ChoiceVisibilityDef, ChooseDef, EffectDef, EffectRecipientDef, ManaColor,
    ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectSetDef, PlayerRefDef,
    PlayerRelation, PlayerSetDef, PowerToughnessOperationDef, ReplacementChoiceDef,
    ReplacementEffectDef, ResolvedEffectDurationDef, SpellAdditionalCostDef, SpendModeDef,
    TopCardSelectionDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::ObjectSetBindingIndex;
use crate::{TargetIndex, mana_cost};

static SAFEKEEPER_SHROUD: AbilityDef = abilities::shroud();

/// X blue cards from your own graveyard, exiled to pay. The count is the same
/// X the spell is cast for, which is what makes the flashback expensive
/// exactly when it is worth casting big.
static EXILE_X_BLUE_CARDS: SpellAdditionalCostDef = SpellAdditionalCostDef::new(
    ObjectPredicateDef::Color(ManaColor::Blue),
    ZoneKind::Graveyard,
    0,
)
.counted_in_x()
.spent(SpendModeDef::Exile);

static FLASH_OF_INSIGHT_LOOK: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::ChosenX,
    object: None,
    minimum: 1,
    maximum: 1,
    select_all_matching: false,
    reveal_selected: false,
    selected_zone: ZoneKind::Hand,
    selected_placement: ZonePlacement::Top,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Bottom,
    rest_random_order: false,
    selected_order_follows_choice: false,
    then: None,
    selected_face_down: None,
};

// JUD 3 — Battle Screech
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BATTLE_SCREECH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c3c38264-0d79-47d4-bca2-a20a991bbac9"),
    "Battle Screech",
    crate::card::CardArt::new("c3c38264-0d79-47d4-bca2-a20a991bbac9", "Randy Gallegos"),
    crate::card::CardSet::Judgment,
    crate::card::CardRules::unsupported(),
);

// JUD 17 — Phantom Nomad
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHANTOM_NOMAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6c5309f5-8b32-4a57-99f2-dcf7a8341898"),
    "Phantom Nomad",
    crate::card::CardArt::new("6c5309f5-8b32-4a57-99f2-dcf7a8341898", "Jim Nelson"),
    crate::card::CardSet::Judgment,
    crate::card::CardRules::unsupported(),
);

// JUD 18 — Prismatic Strands
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PRISMATIC_STRANDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3454ef42-2e0b-4ce4-945f-e4ec3e83c39d"),
    "Prismatic Strands",
    crate::card::CardArt::new("3454ef42-2e0b-4ce4-945f-e4ec3e83c39d", "Eric Peterson"),
    crate::card::CardSet::Judgment,
    crate::card::CardRules::unsupported(),
);

// JUD 40 — Flash of Insight
pub(in crate::card::sets) static FLASH_OF_INSIGHT: CardRecord = CardRecord::new_with_legacy_id(
    2064,
    "Flash of Insight",
    CardArt::new("ffaab905-8b2f-4a5c-9b1f-3c8e5d2b7a41", "Ben Thompson"),
    CardSet::Judgment,
    // Cast small early, flashed back huge late: the graveyard a control deck
    // fills is the second casting's mana.
    CardRules::new_instant(mana_cost!("{X}{1}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Look at the top X cards of your library. Put one of them into your hand and the rest on the bottom of your library in any order.",
            EffectDef::LookAtTopAndSelect {
                player: EffectRecipientDef::Controller,
                looker: EffectRecipientDef::Controller,
                selection: &FLASH_OF_INSIGHT_LOOK,
            },
        ),
        AbilityDef::alternative_cast(
            mana_cost!("{1}{U}"),
            AlternativeCastKindDef::Flashback,
            Some("Flashback—{1}{U}, Exile X blue cards from your graveyard."),
            EffectDef::None,
        )
        .with_alternative_additional_cost(&EXILE_X_BLUE_CARDS),
    ]),
);

/// Everything of the named card in the target's hand, revealed first so the
/// choice is answered honestly and then taken all at once.
static THERAPY_TAKE: EffectDef = EffectDef::Sequence(&[
    EffectDef::RevealHand {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::DiscardCards {
        object: EffectRecipientDef::objects(ObjectSetDef::Binding(ObjectSetBindingIndex::PRIMARY)),
    },
]);

static THERAPY_SACRIFICE: SpellAdditionalCostDef = SpellAdditionalCostDef::new(
    ObjectPredicateDef::HasType(CardType::Creature),
    ZoneKind::Battlefield,
    1,
);

static THERAPY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

// JUD 62 — Cabal Therapy
pub(in crate::card::sets) static CABAL_THERAPY: CardRecord = CardRecord::new_with_legacy_id(
    2068,
    "Cabal Therapy",
    CardArt::new("0a5df970-6c2b-4e7f-9a3d-1b8e5c2f4d6a", "Ron Spencer"),
    CardSet::Judgment,
    // A guess for one mana, and the same guess again later for a creature
    // that has already attacked.
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Choose a nonland card name. Target player reveals their hand and discards all cards with that name.",
            &THERAPY_TARGET,
            EffectDef::ChooseCardName {
                chooser: PlayerRefDef::EffectController,
                nonland_only: true,
                matched_in: PlayerRefDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                binding: ObjectSetBindingIndex::PRIMARY,
                then: &THERAPY_TAKE,
            },
        ),
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::Flashback,
            Some("Flashback—Sacrifice a creature."),
            EffectDef::None,
        )
        .with_alternative_additional_cost(&THERAPY_SACRIFICE),
    ]),
);

/// The chosen shuffled back in. The shuffle follows the move so the
/// library the cards join is the one that gets randomized.
static RECLAMATION_SHUFFLE: EffectDef = EffectDef::Sequence(&[
    EffectDef::MoveToZone {
        counters: None,
        object: EffectRecipientDef::objects(ObjectSetDef::Binding(ObjectSetBindingIndex::PRIMARY)),
        zone: ZoneKind::Library,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        controller: None,
    },
    EffectDef::ShuffleLibrary {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
]);

/// The graveyard the cards come out of belongs to the targeted player, which
/// is what makes the choice a resolution choice here rather than a second
/// target: the constraint is "from their graveyard", and choosing on
/// resolution states it exactly.
static RECLAMATION_CANDIDATES: ObjectQueryDef = ObjectQueryDef::owned_by(
    ObjectPredicateDef::Any,
    &[ZoneKind::Graveyard],
    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
);

static RECLAMATION_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

/// A body assembled from the graveyard, read live off the pile rather than
/// fixed as it entered: a characteristic-defining ability keeps answering.
/// A body assembled from the graveyard, read live off the pile rather than
/// fixed as it entered: a characteristic-defining ability keeps answering.
/// This sets the base rather than adding to it, which is what a printed
/// */* says.
static GHOUL_BODY: AppliedEffectDef = AppliedEffectDef::Characteristic(
    CharacteristicOperationDef::PowerToughness(PowerToughnessOperationDef::SetBase {
        power: ValueDef::TotalPowerOfLinkedExiles,
        toughness: ValueDef::TotalToughnessOfLinkedExiles,
    }),
);

// JUD 73 — Sutured Ghoul
pub(in crate::card::sets) static SUTURED_GHOUL: CardRecord = CardRecord::new_with_legacy_id(
    2089,
    "Sutured Ghoul",
    CardArt::new("754a167b-19ac-4100-91b8-4c605efa5ff7", "Carl Critchlow"),
    CardSet::Judgment,
    // Seven mana for a creature the deck never pays for: it is reanimated
    // onto a graveyard the Druid has already filled, and eats all of it.
    CardRules::new_creature(mana_cost!("{4}{B}{B}{B}"), &["Zombie"], 0, 0).with_abilities(&[
        abilities::trample(),
        AbilityDef::as_enters(
            "As this creature enters, exile any number of creature cards from your graveyard.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::ExileMatchingFromGraveyard(
                ObjectPredicateDef::HasType(CardType::Creature),
            )),
        ),
        AbilityDef::static_ability(
            "Sutured Ghoul's power is equal to the total power of the exiled cards and its toughness is equal to their total toughness.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: GHOUL_BODY,
            },
        ),
    ]),
);

// JUD 122 — Krosan Reclamation
pub(in crate::card::sets) static KROSAN_RECLAMATION: CardRecord = CardRecord::new_with_legacy_id(
    2074,
    "Krosan Reclamation",
    CardArt::new("2aa77608-8f0e-4b12-80e2-d1feabf7787d", "Gary Ruddell"),
    CardSet::Judgment,
    // Graveyard hate that answers a single card twice, which is what a
    // combo deck holding one Sutured Ghoul actually needs.
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target player shuffles up to two target cards from their graveyard into their library.",
            &RECLAMATION_TARGET,
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(ObjectSetBindingIndex::PRIMARY),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(RECLAMATION_CANDIDATES),
                exclude: None,
                minimum: 0,
                maximum: 2,
                visibility: ChoiceVisibilityDef::Public,
                then: &RECLAMATION_SHUFFLE,
            }),
        ),
        AbilityDef::alternative_cast(
            mana_cost!("{1}{G}"),
            AlternativeCastKindDef::Flashback,
            Some("Flashback {1}{G}"),
            EffectDef::None,
        ),
    ]),
);

// JUD 129 — Phantom Tiger
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHANTOM_TIGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("32839296-e583-4f71-aa44-dbe16408665e"),
    "Phantom Tiger",
    crate::card::CardArt::new("32839296-e583-4f71-aa44-dbe16408665e", "Brian Snõddy"),
    crate::card::CardSet::Judgment,
    crate::card::CardRules::unsupported(),
);

// JUD 133 — Sylvan Safekeeper
pub(in crate::card::sets) static SYLVAN_SAFEKEEPER: CardRecord = CardRecord::new_with_legacy_id(
    293,
    "Sylvan Safekeeper",
    CardArt::new("f1b8413f-c9fc-4cea-b416-a1fcf651b009", "Pete Venters"),
    CardSet::Judgment,
    CardRules::new_creature(mana_cost!("{G}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice a land: Target creature you control gains shroud until end of turn.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Land),
                controller: PlayerRelation::You,
            }],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&SAFEKEEPER_SHROUD),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BATTLE_SCREECH,
    &PHANTOM_NOMAD,
    &PRISMATIC_STRANDS,
    &FLASH_OF_INSIGHT,
    &CABAL_THERAPY,
    &SUTURED_GHOUL,
    &KROSAN_RECLAMATION,
    &PHANTOM_TIGER,
    &SYLVAN_SAFEKEEPER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&crate::card::sets::y2012::dark_ascension::RAY_OF_REVELATION), // JUD 20
];
