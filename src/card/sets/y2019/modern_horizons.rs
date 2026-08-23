//! Modern Horizons cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef,
    AddManaEffectDef, AlternativeCastKindDef, AppliedEffectDef, AppliedRuleDef, CardArt, CardRules,
    CardSet, CardSupertype, CardType, CounterKind, EffectDef, EffectRecipientDef,
    ExilePlayDurationDef, ManaColor, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef,
    ObjectSetDef, PlayerRefDef, PlayerRelation, SpellAdditionalCostDef, SpendModeDef,
    TokenCharacteristics, TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::ids::ObjectSetBindingIndex;
use crate::{TargetIndex, mana_cost};

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
        zone: ZoneKind::Exile,
        controller: None,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
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
        zone: ZoneKind::Exile,
        controller: None,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
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
        binding: None,
        then: None,
    },
];

static WINDS_OVERLOADED: EffectDef = EffectDef::BindMatching {
    objects: ObjectSetDef::Query(WINDS_OVERLOADED_CREATURES),
    binding: ObjectSetBindingIndex::PRIMARY,
    then: &EffectDef::Sequence(&WINDS_OVERLOADED_STEPS),
};

// MH1 7 — Ephemerate
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EPHEMERATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2da5f3f8-5eef-498f-ba2c-2f3fbc3745aa"),
    "Ephemerate",
    crate::card::CardArt::new("2da5f3f8-5eef-498f-ba2c-2f3fbc3745aa", "Bastien L. Deharme"),
    crate::card::CardSet::ModernHorizons1,
    crate::card::CardRules::unsupported(),
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

/// Exiled rather than discarded: the card is spent without ever becoming a
/// graveyard card, which is what "exile a green card" means.
static EXILE_A_GREEN_CARD: SpellAdditionalCostDef = SpellAdditionalCostDef::new(
    ObjectPredicateDef::Color(ManaColor::Green),
    ZoneKind::Hand,
    1,
)
.spent(SpendModeDef::Exile);

/// "If it's not your turn" gates only the free cast. The printed cost is
/// always available, which is why this is a condition on the alternative
/// rather than a restriction on the card.
static NOT_YOUR_TURN: TriggerConditionDef =
    TriggerConditionDef::ActivePlayer(PlayerRelation::Opponent);

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
            },
        ),
    ]),
);

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
    },
];

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
            AbilityDef::triggered(
                "When this creature enters, create a 0/0 colorless Construct artifact creature \
                 token with \"This token gets +1/+1 for each artifact you control.\"",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
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
            },
        ),
    ]),
);

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
        },
    ),
];

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

// MH1 222 — Farmstead Gleaner
pub(in crate::card::sets) static FARMSTEAD_GLEANER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("edafd52f-2dda-4981-baee-404f47ee8969"),
    "Farmstead Gleaner",
    CardArt::new("edafd52f-2dda-4981-baee-404f47ee8969", "Josh Hass"),
    CardSet::ModernHorizons1,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Scarecrow"], 2, 2)
        .with_abilities(&FARMSTEAD_GLEANER_ABILITIES),
);

static SUNBAKED_CANYON_COLORS: [ManaColor; 2] = [ManaColor::Red, ManaColor::White];

static SUNBAKED_CANYON_ABILITIES: [AbilityDef; 2] =
    abilities::horizon_land("{T}, Pay 1 life: Add {R} or {W}.", &SUNBAKED_CANYON_COLORS);

// MH1 247 — Sunbaked Canyon
pub(in crate::card::sets) static SUNBAKED_CANYON: CardRecord = CardRecord::new_with_legacy_id(
    2230,
    "Sunbaked Canyon",
    CardArt::new("c36820fa-ee86-4206-9a0d-737a67cf5208", "Yeong-Hao Han"),
    CardSet::ModernHorizons1,
    CardRules::new_land(&[]).with_abilities(&SUNBAKED_CANYON_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &EPHEMERATE,
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
    &COLLECTOR_OUPHE,
    &FORCE_OF_VIGOR,
    &KROSAN_TUSKER,
    &MOTHER_BEAR,
    &TRUMPETING_HERD,
    &WINDING_WAY,
    &FALLEN_SHINOBI,
    &FARMSTEAD_GLEANER,
    &SUNBAKED_CANYON,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
