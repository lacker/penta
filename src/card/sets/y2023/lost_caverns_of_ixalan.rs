//! Lost Caverns of Ixalan cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AlternativeCastKindDef, AppliedEffectDef,
    CardArt, CardRules, CardSet, CardSupertype, CardType, ChoiceVisibilityDef, ChooseDef,
    ComparisonDef, CounterKind, DiscardFollowUpDef, DiscardSelectionDef, EffectDef,
    EffectPaymentCostDef, EffectPaymentDef, EffectRecipientDef, ExilePlayDurationDef,
    InstalledTriggerDef, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef,
    ObjectSetDef, PayOrDef, PlayerRefDef, PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef,
    SpellAdditionalCostDef, TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, abilities,
    tokens,
};
use crate::ids::{ObjectBindingIndex, ObjectSetBindingIndex};
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
pub(in crate::card::sets) static GET_LOST: CardRecord = CardRecord::new_with_legacy_id(
    2294,
    "Get Lost",
    CardArt::new("522aa72b-2b8c-484c-872b-f082101cee35", "Eli Minaya"),
    CardSet::LostCavernsOfIxalan,
    // Two mana that answers three card types at instant speed, and the two
    // Maps are what it pays for that: real but slow ones.
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature, enchantment, or planeswalker. Its controller creates two Map \
         tokens.",
        &A_CREATURE_ENCHANTMENT_OR_PLANESWALKER,
        EffectDef::Sequence(&GET_LOST_EFFECT),
    )),
);

/// One cost with two ways to pay it. The life is the way a deck with an
/// empty hand still casts this, which is what keeps it playable late.
static DISCARD_A_CARD_OR_PAY_THREE_LIFE: SpellAdditionalCostDef =
    SpellAdditionalCostDef::new(ObjectPredicateDef::Any, ZoneKind::Hand, 1).or_pay_life(3);

static A_CREATURE_OR_PLANESWALKER: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::HasType(CardType::Planeswalker),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

/// What the fourth connection is worth: the card you just threw away, cast
/// for nothing. The kind says both halves at once -- no mana, and an
/// ordinary trip to the graveyard afterwards.
static MALCOLM_FREE_CAST: AbilityDef = AbilityDef::alternative_cast(
    mana_cost!("{0}"),
    AlternativeCastKindDef::WithoutPayingManaCost,
    Some("Cast without paying its mana cost."),
    EffectDef::None,
);

static MALCOLM_CAST_THE_DISCARD: EffectDef = EffectDef::MayCastTargetWithoutPaying {
    object: EffectRecipientDef::objects(ObjectSetDef::Binding(ObjectSetBindingIndex::PRIMARY)),
    ability: &MALCOLM_FREE_CAST,
};

/// Read after the counter has been added, so the connection that makes it
/// four is itself the one that pays.
static MALCOLM_IS_A_CHORUS: TriggerConditionDef = TriggerConditionDef::SourceCounters {
    kind: CounterKind::Chorus,
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 4,
};

static MALCOLM_MAYBE_CAST: EffectDef = EffectDef::IfCondition {
    condition: &MALCOLM_IS_A_CHORUS,
    then: &MALCOLM_CAST_THE_DISCARD,
};

static MALCOLM_TRIGGER: [EffectDef; 3] = [
    EffectDef::AddCounters {
        object: EffectRecipientDef::Source,
        kind: CounterKind::Chorus,
        amount: ValueDef::Constant(1),
    },
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
    EffectDef::Discard {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
        selection: DiscardSelectionDef::RecipientChooses,
        then: Some(DiscardFollowUpDef {
            counted: ObjectPredicateDef::Any,
            bound: Some(ObjectSetBindingIndex::PRIMARY),
            effect: &MALCOLM_MAYBE_CAST,
        }),
    },
];

// LCI 24 — Miner's Guidewing
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MINER_S_GUIDEWING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9048cd9d-df3f-4705-a5f4-e5b09760c631"),
    "Miner's Guidewing",
    crate::card::CardArt::new("9048cd9d-df3f-4705-a5f4-e5b09760c631", "Allen Douglas"),
    crate::card::CardSet::LostCavernsOfIxalan,
    crate::card::CardRules::unsupported(),
);

// LCI 30 — Petrify
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PETRIFY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bbc5f28f-6361-455f-ac82-260a70e59316"),
    "Petrify",
    crate::card::CardArt::new("bbc5f28f-6361-455f-ac82-260a70e59316", "Samuel Araya"),
    crate::card::CardSet::LostCavernsOfIxalan,
    crate::card::CardRules::unsupported(),
);

// LCI 63 — Malcolm, Alluring Scoundrel
pub(in crate::card::sets) static MALCOLM_ALLURING_SCOUNDREL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("19d6834d-afa3-4747-a62d-0654f4d9729f"),
    "Malcolm, Alluring Scoundrel",
    CardArt::new("19d6834d-afa3-4747-a62d-0654f4d9729f", "Fesbra"),
    CardSet::LostCavernsOfIxalan,
    // Two mana for an evasive body that loots every time it connects, and
    // that turns the loot into a free spell once it has connected four
    // times.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Siren", "Pirate"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flash(),
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever this creature deals combat damage to a player, put a chorus counter on \
                 it. Draw a card, then discard a card. If there are four or more chorus counters \
                 on it, you may cast the discarded card without paying its mana cost.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                EffectDef::Sequence(&MALCOLM_TRIGGER),
            ),
        ]),
);

// LCI 91 — Bitter Triumph
pub(in crate::card::sets) static BITTER_TRIUMPH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("05bdd22c-3e11-4c29-bdfa-d3dfc0e90a9f"),
    "Bitter Triumph",
    CardArt::new("05bdd22c-3e11-4c29-bdfa-d3dfc0e90a9f", "Donato Giancola"),
    CardSet::LostCavernsOfIxalan,
    // Two mana for unconditional removal at instant speed, and the card or
    // the three life is the whole restriction: it answers anything, and it
    // never answers it for free.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, discard a card or pay 3 life.\nDestroy \
             target creature or planeswalker.",
            &A_CREATURE_OR_PLANESWALKER,
            DISCARD_A_CARD_OR_PAY_THREE_LIFE,
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ),
);

// LCI 102 — Deep-Cavern Bat
pub(in crate::card::sets) static DEEP_CAVERN_BAT: CardRecord = CardRecord::new_with_legacy_id(
    2161,
    "Deep-Cavern Bat",
    CardArt::new("69c68c95-b788-43b1-9f22-1b22c5a00b25", "Campbell White"),
    CardSet::LostCavernsOfIxalan,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Bat"], 1, 1)
        .with_abilities(&DEEP_CAVERN_BAT_ABILITIES),
);

static INTI_TRAMPLE: AbilityDef = abilities::trample();

/// "It gains trample until end of turn" -- the creature that took the
/// counter, which is the one the trigger targeted.
static INTI_PUMP: [EffectDef; 2] = [
    EffectDef::AddCounters {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        kind: CounterKind::PlusOnePlusOne,
        amount: ValueDef::Constant(1),
    },
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::add_ability(&INTI_TRAMPLE),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
];

static AN_ATTACKING_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Attacking,
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

static INTI_ABILITIES: [AbilityDef; 2] = [
    // The target is declared as the attack trigger goes on the stack rather
    // than when the discard is made, which is the one place this differs
    // from the printed reflexive trigger. "Whenever you attack" guarantees
    // an attacking creature, so there is always something to name.
    AbilityDef::triggered_with_targets(
        "Whenever you attack, you may discard a card. When you do, put a +1/+1 counter on target \
         attacking creature. It gains trample until end of turn.",
        TriggerEventDef::attack_declared(ObjectPredicateDef::Any, 1, None),
        &AN_ATTACKING_CREATURE,
        EffectDef::PayOr(PayOrDef::optional(
            EffectPaymentDef {
                payer: PlayerSetDef::Related(PlayerRelation::You),
                cost: EffectPaymentCostDef::Discard(1),
            },
            &EffectDef::Sequence(&INTI_PUMP),
        )),
    ),
    // One trigger for the whole discard however many cards it took, and the
    // card it finds is playable into your own turn when the discard
    // happened on somebody else's.
    AbilityDef::triggered(
        "Whenever you discard one or more cards, exile the top card of your library. You may play \
         that card until your next end step.",
        TriggerEventDef::DiscardedCards(PlayerRelation::You),
        EffectDef::ExileTopOfLibraryToPlay {
            player: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
            free: false,
            face_down: false,
            duration: ExilePlayDurationDef::UntilYourNextEndStep,
        },
    ),
];

// LCI 128 — Tithing Blade
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TITHING_BLADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dbaa9a2d-e9fd-4746-a26c-f99ae731f024"),
    "Tithing Blade",
    crate::card::CardArt::new("dbaa9a2d-e9fd-4746-a26c-f99ae731f024", "Michael Walsh"),
    crate::card::CardSet::LostCavernsOfIxalan,
    crate::card::CardRules::unsupported(),
);

// LCI 156 — Inti, Seneschal of the Sun
pub(in crate::card::sets) static INTI_SENESCHAL_OF_THE_SUN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fa7a55aa-ae61-4933-b7a4-dcc55dac6fcd"),
    "Inti, Seneschal of the Sun",
    CardArt::new(
        "fa7a55aa-ae61-4933-b7a4-dcc55dac6fcd",
        "Victor Adame Minguez",
    ),
    CardSet::LostCavernsOfIxalan,
    // Two mana that turns every spare card into a bigger attack and a new
    // card, and the two halves feed each other: the discard he asks for is
    // the discard the second clause is watching for.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Knight"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&INTI_ABILITIES),
);

static SENTINEL_ENTERS_OR_ATTACKS: [TriggerEventDef; 2] = [
    TriggerEventDef::zone_changed(
        ObjectPredicateDef::Source,
        None,
        Some(ZoneKind::Battlefield),
    ),
    TriggerEventDef::attacks(ObjectPredicateDef::Source),
];

static SENTINEL_MAP: EffectDef = EffectDef::create_token(tokens::map()).with_art(CardArt::new(
    "64839118-09d2-4645-9d3c-f80755ac781f",
    "Francesca Baerald",
));

// LCI 211 — Sentinel of the Nameless City
pub(in crate::card::sets) static SENTINEL_OF_THE_NAMELESS_CITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eeeffc0b-dc92-458e-ad58-86ff6077a508"),
    "Sentinel of the Nameless City",
    CardArt::new("eeeffc0b-dc92-458e-ad58-86ff6077a508", "Josu Hernaiz"),
    CardSet::LostCavernsOfIxalan,
    // A 3/4 that blocks and attacks in the same turn, and hands you a Map
    // for doing either.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Merfolk", "Warrior", "Scout"], 3, 4)
        .with_abilities(&[
            abilities::vigilance(),
            AbilityDef::triggered(
                "Whenever this creature enters or attacks, create a Map token.",
                TriggerEventDef::AnyOf(&SENTINEL_ENTERS_OR_ATTACKS),
                SENTINEL_MAP,
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &GET_LOST,
    &MINER_S_GUIDEWING,
    &PETRIFY,
    &MALCOLM_ALLURING_SCOUNDREL,
    &BITTER_TRIUMPH,
    &DEEP_CAVERN_BAT,
    &TITHING_BLADE,
    &INTI_SENESCHAL_OF_THE_SUN,
    &SENTINEL_OF_THE_NAMELESS_CITY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
