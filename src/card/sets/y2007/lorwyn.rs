//! Lorwyn cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AddManaEffectDef, CardArt, CardRules, CardSet, CardType, ChoiceVisibilityDef, ChooseDef,
    ComparisonDef, EffectDef, EffectRecipientDef, ManaColor, ObjectChoiceBindingDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, PlayerRefDef, PlayerRelation,
    PlayerSetDef, TopCardSelectionDef, TriggerConditionDef, ValueComparisonDef, ValueDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::ids::{ObjectBindingIndex, TargetIndex};
use crate::mana_cost;

// LRW 56 — Cryptic Command
static A_SPELL: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Any)];

static A_PERMANENT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::Any,
)];

/// Two of four, and never the same one twice. Each targeting mode carries
/// its own slot, so a Command that counters and bounces declares a spell and
/// a permanent, and one that taps and draws declares nothing at all.
static CRYPTIC_COMMAND_MODES: [AbilityDef; 4] = [
    AbilityDef::counter_target("Counter target spell.", &A_SPELL[0]),
    AbilityDef::spell_with_targets(
        "Return target permanent to its owner's hand.",
        &A_PERMANENT,
        EffectDef::MoveToZone {
            counters: None,
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            from: None,
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            controller: None,
            arrival_effect: None,
            attachment: None,
            tapped: false,
        },
    ),
    // Their creatures, not everyone's: the Command is a Fog you get to keep
    // the draw off, and tapping your own would defeat the point.
    AbilityDef::spell(
        "Tap all creatures your opponents control.",
        EffectDef::Tap {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Opponent,
            ),
        },
    ),
    AbilityDef::spell(
        "Draw a card.",
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    ),
];

pub(in crate::card::sets) static CRYPTIC_COMMAND: CardRecord = CardRecord::new_with_legacy_id(
    2272,
    "Cryptic Command",
    CardArt::new("829e3d6e-5d7c-4cc4-a7a6-7cbf5a7442ba", "Wayne England"),
    CardSet::Lorwyn,
    // Four mana of triple blue that is never the wrong card: counter and
    // draw when they act, bounce and draw when they do not.
    CardRules::new_instant(mana_cost!("{1}{U}{U}{U}")).with_ability(AbilityDef::modal_spell(
        "Choose two \u{2014}\n\u{2022} Counter target spell.\n\u{2022} Return target permanent \
         to its owner's hand.\n\u{2022} Tap all creatures your opponents control.\n\u{2022} \
         Draw a card.",
        &CRYPTIC_COMMAND_MODES,
        2,
        2,
        false,
    )),
);

// LRW 76 — Mulldrifter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MULLDRIFTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a97cfefa-ade7-49f6-b2aa-1118b9db4935"),
    "Mulldrifter",
    crate::card::CardArt::new("a97cfefa-ade7-49f6-b2aa-1118b9db4935", "Eric Fortune"),
    crate::card::CardSet::Lorwyn,
    crate::card::CardRules::unsupported(),
);

// LRW 79 — Ponder
/// The shuffle is the caster's call and comes after the look: having seen
/// the three, you decide whether to keep the arrangement or wash it away.
/// The draw is last either way, so a shuffled Ponder still finds a card.
static PONDER_SHUFFLE_AND_DRAW: EffectDef = EffectDef::Sequence(&[
    EffectDef::May {
        player: EffectRecipientDef::Controller,
        effect: &EffectDef::ShuffleLibrary {
            player: EffectRecipientDef::Controller,
        },
    },
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
]);

/// Every card looked at is selected, which is what makes the choice an
/// ordering rather than a filter: all three go back on top, in the order
/// they were named.
static PONDER_LOOK: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(3),
    object: None,
    minimum: 3,
    maximum: 3,
    select_all_matching: false,
    select_one_of_each_type: false,
    reveal_inspected: false,
    reveal_selected: false,
    counted: None,
    selected_zone: ZoneKind::Library,
    selected_placement: ZonePlacement::Top,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Top,
    rest_random_order: false,
    rest_counters: None,
    selected_order_follows_choice: true,
    then: Some(&PONDER_SHUFFLE_AND_DRAW),
    selected_hidden: false,
    selected_linked_to_source: false,
    selected_face_down: None,
};

pub(in crate::card::sets) static PONDER: CardRecord = CardRecord::new_with_legacy_id(
    2241,
    "Ponder",
    CardArt::new("ba6b6fc5-5077-4812-b8e9-906783dbaf67", "Mark Tedin"),
    CardSet::Lorwyn,
    // One mana to see four cards deep and keep the best of them, which is
    // why the format has never been able to leave it legal for long.
    CardRules::new_sorcery(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Look at the top three cards of your library, then put them back in any order. You may \
         shuffle.\nDraw a card.",
        EffectDef::LookAtTopAndSelect {
            player: EffectRecipientDef::Controller,
            looker: EffectRecipientDef::Controller,
            selection: &PONDER_LOOK,
        },
    )),
);

// LRW 145 — Thoughtseize
static A_PLAYER: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

static SEIZE_IT: EffectDef = EffectDef::DiscardCards {
    object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
};

/// The hand is revealed rather than looked at: everybody sees it, which is
/// what makes the choice checkable and what the card prints.
static THOUGHTSEIZE_EFFECT: [EffectDef; 3] = [
    EffectDef::RevealHand {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::Choose(ChooseDef {
        binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
        unchosen: None,
        chooser: PlayerRefDef::EffectController,
        candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            &[ZoneKind::Hand],
            PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
        )),
        exclude: None,
        minimum: 1,
        maximum: 1,
        visibility: ChoiceVisibilityDef::Public,
        then: &SEIZE_IT,
    }),
    // Unconditional: a hand of nothing but lands still costs you two.
    EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    },
];

pub(in crate::card::sets) static THOUGHTSEIZE: CardRecord = CardRecord::new_with_legacy_id(
    2240,
    "Thoughtseize",
    CardArt::new("3df8c148-e87d-4043-9d8b-ec72bf8b6d5d", "Aleksi Briclot"),
    CardSet::Lorwyn,
    // One mana, any card, two life. The life is what keeps it honest and it
    // has never been enough.
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player reveals their hand. You choose a nonland card from it. That player \
         discards that card. You lose 2 life.",
        &A_PLAYER,
        EffectDef::Sequence(&THOUGHTSEIZE_EFFECT),
    )),
);

// LRW 194 — Tarfire
/// A Shock that is also a Goblin card. Kindred is what carries the subtype
/// onto a noncreature card (CR 205.2a) and does nothing else: the spell is
/// still an instant, and it is a Goblin in every zone -- in the library
/// where a Ringleader looks for one, and in the graveyard afterwards.
pub(in crate::card::sets) static TARFIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d13a898e-6a97-4fd9-980e-3bfd8d755386"),
    "Tarfire",
    CardArt::new("d13a898e-6a97-4fd9-980e-3bfd8d755386", "Omar Rayyan"),
    CardSet::Lorwyn,
    // Two damage for one mana is a fine rate and not why it is played: the
    // Goblin deck plays it because Ringleader draws it and Matron finds it.
    CardRules::new_instant(mana_cost!("{R}"))
        .with_type(CardType::Kindred)
        .with_subtypes(&["Goblin"])
        .with_ability(AbilityDef::spell_with_targets(
            "This spell deals 2 damage to any target.",
            &TARFIRE_TARGET,
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        )),
);

static TARFIRE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

// LRW 196 — Wild Ricochet
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WILD_RICOCHET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d76f09bc-b49a-4ad2-be2d-2a191d41b86d"),
    "Wild Ricochet",
    crate::card::CardArt::new("d76f09bc-b49a-4ad2-be2d-2a191d41b86d", "Dan Murayama Scott"),
    crate::card::CardSet::Lorwyn,
    crate::card::CardRules::unsupported(),
);

// LRW 262 — Thorn of Amethyst
pub(in crate::card::sets) static THORN_OF_AMETHYST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e472d4f5-add4-4de3-8718-31a47a35277c"),
    "Thorn of Amethyst",
    CardArt::new("e472d4f5-add4-4de3-8718-31a47a35277c", "Chuck Lukacs"),
    CardSet::Lorwyn,
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(abilities::spell_cost_increase(
        "Noncreature spells cost {1} more to cast.",
        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
        PlayerRelation::Any,
        mana_cost!("{1}"),
    )),
);

// LRW 272 — Shelldock Isle
/// Hideaway's look: four cards, one of them exiled face down and linked to
/// the land that took it, and the rest back under the library in an order
/// nobody knows.
static HIDEAWAY_FOUR: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(4),
    object: None,
    minimum: 1,
    maximum: 1,
    select_all_matching: false,
    select_one_of_each_type: false,
    reveal_inspected: false,
    reveal_selected: false,
    counted: None,
    selected_zone: ZoneKind::Exile,
    selected_placement: ZonePlacement::Top,
    selected_hidden: true,
    selected_linked_to_source: true,
    selected_face_down: None,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Bottom,
    rest_random_order: true,
    rest_counters: None,
    selected_order_follows_choice: false,
    then: None,
};

/// "If a library has twenty or fewer cards in it" -- either library, which
/// is why the two are asked separately rather than counted together.
static A_LIBRARY_IS_NEARLY_EMPTY: TriggerConditionDef = TriggerConditionDef::AnyOf(&[
    TriggerConditionDef::ValueComparison(&YOUR_LIBRARY_IS_NEARLY_EMPTY),
    TriggerConditionDef::ValueComparison(&THEIR_LIBRARY_IS_NEARLY_EMPTY),
]);

static YOUR_LIBRARY_IS_NEARLY_EMPTY: ValueComparisonDef = ValueComparisonDef {
    left: ValueDef::LibrarySize(PlayerRelation::You),
    comparison: ComparisonDef::LessOrEqual,
    right: ValueDef::Constant(20),
};

static THEIR_LIBRARY_IS_NEARLY_EMPTY: ValueComparisonDef = ValueComparisonDef {
    left: ValueDef::LibrarySize(PlayerRelation::Opponent),
    comparison: ComparisonDef::LessOrEqual,
    right: ValueDef::Constant(20),
};

static SHELLDOCK_UNLOCK_COST: [AbilityCostDef; 2] = [
    AbilityCostDef::Mana(mana_cost!("{U}")),
    AbilityCostDef::TapSource,
];

pub(in crate::card::sets) static SHELLDOCK_ISLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4216656e-90e8-45fc-a0f6-0d0d79d0a021"),
    "Shelldock Isle",
    CardArt::new("4216656e-90e8-45fc-a0f6-0d0d79d0a021", "Mark Tedin"),
    CardSet::Lorwyn,
    // A tapped Island that hides your best card until the game is nearly
    // over, and then plays it for nothing.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_trigger(
            "Hideaway 4 (When this land enters, look at the top four cards of your library, \
             exile one face down, then put the rest on the bottom in a random order.)",
            EffectDef::LookAtTopAndSelect {
                player: EffectRecipientDef::Controller,
                looker: EffectRecipientDef::Controller,
                selection: &HIDEAWAY_FOUR,
            },
        ),
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
        ),
        AbilityDef::activated(
            "{U}, {T}: You may play the exiled card without paying its mana cost if a library \
             has twenty or fewer cards in it.",
            &SHELLDOCK_UNLOCK_COST,
            EffectDef::MayPlayWithoutPaying {
                objects: ObjectSetDef::LinkedExiles(ObjectPredicateDef::Any),
            },
        )
        .with_activation_condition(&A_LIBRARY_IS_NEARLY_EMPTY)
        .with_coverage(AbilityCoverageDef::partial(
            "The card is playable for the rest of the turn rather than only while this ability \
             resolves, so a player who waits keeps the option a little longer than the printed \
             clause allows.",
        )),
    ]),
);

// LRW 273 — Shimmering Grotto
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHIMMERING_GROTTO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b5e85acc-ed12-4036-8193-739721c3e178"),
    "Shimmering Grotto",
    crate::card::CardArt::new("b5e85acc-ed12-4036-8193-739721c3e178", "Alan Pollack"),
    crate::card::CardSet::Lorwyn,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &CRYPTIC_COMMAND,
    &MULLDRIFTER,
    &PONDER,
    &THOUGHTSEIZE,
    &TARFIRE,
    &WILD_RICOCHET,
    &THORN_OF_AMETHYST,
    &SHELLDOCK_ISLE,
    &SHIMMERING_GROTTO,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
