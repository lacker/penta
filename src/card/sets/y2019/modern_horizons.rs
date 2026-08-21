//! Modern Horizons cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef,
    AddManaEffectDef, AlternativeCastKindDef, AppliedEffectDef, AppliedRuleDef, CardArt, CardRules,
    CardSet, CardSupertype, CardType, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    ObjectQueryDef, ObjectRefDef, ObjectSetDef, PlayerRefDef, PlayerRelation,
    SpellAdditionalCostDef, SpendModeDef, TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind,
    ZonePlacement, abilities, cards,
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

// MH1 37 — Winds of Abandon
pub(in crate::card::sets) static WINDS_OF_ABANDON: CardRecord = CardRecord::new(
    cards::WINDS_OF_ABANDON,
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

/// Timetwister's effect, one word at a time: everything from both hidden
/// zones goes back, both libraries are shuffled, and both players draw. The
/// Echo itself is on the stack while this resolves, so it is not among the
/// cards that go back.
static ECHO_OF_EONS_EFFECT: [EffectDef; 3] = [
    EffectDef::MoveToZone {
        object: EffectRecipientDef::matching_objects(
            ObjectPredicateDef::Any,
            &[ZoneKind::Hand, ZoneKind::Graveyard],
            PlayerRelation::Any,
        ),
        zone: ZoneKind::Library,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        controller: None,
    },
    EffectDef::ShuffleLibrary {
        player: EffectRecipientDef::EachPlayer,
    },
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::EachPlayer,
        amount: ValueDef::Constant(7),
    },
];

// MH1 46 — Echo of Eons
pub(in crate::card::sets) static ECHO_OF_EONS: CardRecord = CardRecord::new(
    cards::ECHO_OF_EONS,
    "Echo of Eons",
    CardArt::new("ff590af2-2d6c-4f16-a9b8-1a6dab6e9ad5", "Terese Nielsen"),
    CardSet::ModernHorizons1,
    // Six mana nobody pays: the card is here for the flashback, which turns a
    // graveyard full of rituals into a fresh seven for three.
    CardRules::new_sorcery(mana_cost!("{4}{U}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Each player shuffles their hand and graveyard into their library, then draws seven \
             cards.",
            EffectDef::Sequence(&ECHO_OF_EONS_EFFECT),
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

// MH1 52 — Force of Negation
pub(in crate::card::sets) static FORCE_OF_NEGATION: CardRecord = CardRecord::new(
    cards::FORCE_OF_NEGATION,
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

// MH1 158 — Collector Ouphe
pub(in crate::card::sets) static COLLECTOR_OUPHE: CardRecord = CardRecord::new(
    cards::COLLECTOR_OUPHE,
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
pub(in crate::card::sets) static FORCE_OF_VIGOR: CardRecord = CardRecord::new(
    cards::FORCE_OF_VIGOR,
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
        },
    ),
];

// MH1 199 — Fallen Shinobi
pub(in crate::card::sets) static FALLEN_SHINOBI: CardRecord = CardRecord::new(
    cards::FALLEN_SHINOBI,
    "Fallen Shinobi",
    CardArt::new("900c9dfd-ece1-4b09-a801-0fa05e1994b9", "Tomasz Jedruszek"),
    CardSet::ModernHorizons1,
    // Ninjutsu is what makes a five-mana 5/4 connect on turn three, and
    // connecting is the whole card: two cards off the top of their deck,
    // free, every time.
    CardRules::new_creature(mana_cost!("{3}{U}{B}"), &["Zombie", "Ninja"], 5, 4)
        .with_abilities(&SHINOBI_ABILITIES),
);

/// The horizon-land cycle: two colours for a life apiece, and when the game
/// has gone long enough that the land is dead weight it cashes itself in for
/// a card. What it never does is make colourless mana for free, which is the
/// whole of the tradeoff.
const fn horizon_mana(mana_text: &'static str, colors: &'static [ManaColor]) -> AbilityDef {
    AbilityDef::activated_mana(
        mana_text,
        &HORIZON_MANA_COST,
        EffectDef::AddMana(AddManaEffectDef::choice(colors)),
    )
}

/// The same on every land in the cycle, down to the printed text.
static HORIZON_CASH_IN: AbilityDef = AbilityDef::activated(
    "{1}, {T}, Sacrifice this land: Draw a card.",
    &HORIZON_CASH_IN_COST,
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
);

static HORIZON_MANA_COST: [AbilityCostDef; 2] =
    [AbilityCostDef::TapSource, AbilityCostDef::PayLife(1)];

static HORIZON_CASH_IN_COST: [AbilityCostDef; 3] = [
    AbilityCostDef::Mana(mana_cost!("{1}")),
    AbilityCostDef::TapSource,
    AbilityCostDef::SacrificeSource,
];

static SUNBAKED_CANYON_COLORS: [ManaColor; 2] = [ManaColor::Red, ManaColor::White];

static SUNBAKED_CANYON_ABILITIES: [AbilityDef; 2] = [
    horizon_mana("{T}, Pay 1 life: Add {R} or {W}.", &SUNBAKED_CANYON_COLORS),
    HORIZON_CASH_IN,
];

// MH1 247 — Sunbaked Canyon
pub(in crate::card::sets) static SUNBAKED_CANYON: CardRecord = CardRecord::new(
    cards::SUNBAKED_CANYON,
    "Sunbaked Canyon",
    CardArt::new("c36820fa-ee86-4206-9a0d-737a67cf5208", "Yeong-Hao Han"),
    CardSet::ModernHorizons1,
    CardRules::new_land(&[]).with_abilities(&SUNBAKED_CANYON_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &WINDS_OF_ABANDON,
    &ECHO_OF_EONS,
    &FORCE_OF_NEGATION,
    &COLLECTOR_OUPHE,
    &FORCE_OF_VIGOR,
    &FALLEN_SHINOBI,
    &SUNBAKED_CANYON,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
