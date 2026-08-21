//! Lorwyn cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt, CardRules, CardSet, CardType,
    ChoiceVisibilityDef, ChooseDef, EffectDef, EffectRecipientDef, ObjectChoiceBindingDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, PlayerRefDef, PlayerRelation,
    PlayerSetDef, TopCardSelectionDef, ValueDef, ZoneKind, ZonePlacement, cards,
};
use crate::ids::{ObjectBindingIndex, TargetIndex};
use crate::mana_cost;

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
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            controller: None,
            arrival_effect: None,
            attachment: None,
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

// LRW 56 — Cryptic Command
pub(in crate::card::sets) static CRYPTIC_COMMAND: CardRecord = CardRecord::new(
    cards::CRYPTIC_COMMAND,
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
    reveal_selected: false,
    selected_zone: ZoneKind::Library,
    selected_placement: ZonePlacement::Top,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Top,
    selected_order_follows_choice: true,
    then: Some(&PONDER_SHUFFLE_AND_DRAW),
    selected_face_down: None,
};

// LRW 79 — Ponder
pub(in crate::card::sets) static PONDER: CardRecord = CardRecord::new(
    cards::PONDER,
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

// LRW 145 — Thoughtseize
pub(in crate::card::sets) static THOUGHTSEIZE: CardRecord = CardRecord::new(
    cards::THOUGHTSEIZE,
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&CRYPTIC_COMMAND, &PONDER, &THOUGHTSEIZE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
