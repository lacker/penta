//! Lost Caverns of Ixalan cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt, CardRules, CardSet, CardType,
    ChoiceVisibilityDef, ChooseDef, EffectDef, EffectRecipientDef, InstalledTriggerDef,
    ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef, TriggerEventDef, ZoneKind, abilities, cards,
    tokens,
};
use crate::ids::ObjectBindingIndex;
use crate::{TargetIndex, mana_cost};

/// "Until this creature leaves the battlefield" is one printed ability, so
/// the return rides on the same resolution as a delayed trigger rather than
/// appearing as a second clause the card does not print.
static BAT_RETURNS_IT: AbilityDef = AbilityDef::triggered(
    "When this creature leaves the battlefield, return the exiled card to its owner's hand.",
    TriggerEventDef::zone_changed(
        ObjectPredicateDef::Source,
        Some(ZoneKind::Battlefield),
        None,
    ),
    EffectDef::ReturnLinkedExiles {
        object: ObjectPredicateDef::Any,
        counters: None,
        arrival_effect: None,
        zone: ZoneKind::Hand,
        grant: None,
        controller: None,
        transformed: false,
    },
);

static BAT_EXILE: [EffectDef; 2] = [
    EffectDef::ExileLinkedToSource {
        object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    },
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&BAT_RETURNS_IT)),
];

static BAT_LOOKS_AND_MAY_TAKE: [EffectDef; 2] = [
    EffectDef::LookAtHand {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    // "You may exile" -- a minimum of none, so looking and taking nothing is
    // a legal answer. The Sculler and the Freebooter both must take one.
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
        minimum: 0,
        maximum: 1,
        visibility: ChoiceVisibilityDef::Public,
        then: &EffectDef::Sequence(&BAT_EXILE),
    }),
];

static BAT_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
)];

static DEEP_CAVERN_BAT_ABILITIES: [AbilityDef; 3] = [
    abilities::flying(),
    abilities::lifelink(),
    AbilityDef::triggered_with_targets(
        "When this creature enters, look at target opponent's hand. You may exile a nonland card from it until this creature leaves the battlefield.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        &BAT_TARGET,
        EffectDef::Sequence(&BAT_LOOKS_AND_MAY_TAKE),
    ),
];

static A_CREATURE_ENCHANTMENT_OR_PLANESWALKER: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::HasType(CardType::Enchantment),
            ObjectPredicateDef::HasType(CardType::Planeswalker),
        ]),
    )];

/// "Its controller creates two Map tokens." The Maps are theirs, not yours,
/// and the permanent is already destroyed by the time they arrive -- so the
/// player is read from what the target was rather than from where it is.
static TWO_MAPS_FOR_ITS_CONTROLLER: EffectDef = EffectDef::create_token(tokens::map())
    .with_art(CardArt::new(
        "64839118-09d2-4645-9d3c-f80755ac781f",
        "Francesca Baerald",
    ))
    .with_controller(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
        TargetIndex::PRIMARY,
    )))
    .with_amount(2);

static GET_LOST_EFFECT: [EffectDef; 2] = [
    EffectDef::Destroy {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        can_regenerate: true,
    },
    TWO_MAPS_FOR_ITS_CONTROLLER,
];

// LCI 14 — Get Lost
pub(in crate::card::sets) static GET_LOST: CardRecord = CardRecord::new(
    cards::GET_LOST,
    "Get Lost",
    CardArt::new("522aa72b-2b8c-484c-872b-f082101cee35", "Eli Minaya"),
    CardSet::LostCavernsOfIxalan,
    // Two mana that answers three card types at instant speed, and the two
    // Maps are what it pays for that: real cards, but slow ones.
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature, enchantment, or planeswalker. Its controller creates two Map \
         tokens.",
        &A_CREATURE_ENCHANTMENT_OR_PLANESWALKER,
        EffectDef::Sequence(&GET_LOST_EFFECT),
    )),
);

// LCI 102 — Deep-Cavern Bat
pub(in crate::card::sets) static DEEP_CAVERN_BAT: CardRecord = CardRecord::new(
    cards::DEEP_CAVERN_BAT,
    "Deep-Cavern Bat",
    CardArt::new("69c68c95-b788-43b1-9f22-1b22c5a00b25", "Campbell White"),
    CardSet::LostCavernsOfIxalan,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Bat"], 1, 1)
        .with_abilities(&DEEP_CAVERN_BAT_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&GET_LOST, &DEEP_CAVERN_BAT];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
