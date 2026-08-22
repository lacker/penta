//! Apocalypse cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef, CardArt,
    CardComposition, CardEffectStatus, CardPart, CardRules, CardSet, CardStructure, CardType,
    DiscardFollowUpDef, DiscardSelectionDef, DividedTotal, EffectDef, EffectRecipientDef,
    ManaColor, ObjectPredicateDef, PlayOptionDef, PlayerRelation, ResolvedEffectDurationDef,
    ScaledValueDef, SpellForm, TopCardSelectionDef, TriggerEventDef, TurnStepDef, ValueDef,
    ZoneKind, ZonePlacement, abilities,
};
use crate::{CardPartId, PlayOptionId, TargetIndex, mana_cost};

// APC 47 — Phyrexian Arena
pub(in crate::card::sets) static PHYREXIAN_ARENA: CardRecord = CardRecord::new_with_legacy_id(
    289,
    "Phyrexian Arena",
    CardArt::new("84e19975-e3e1-453b-b902-a1b1fc1d8504", "Pete Venters"),
    CardSet::Apocalypse,
    CardRules::new_enchantment(mana_cost!("{1}{B}{B}")).with_ability(AbilityDef::triggered(
        "At the beginning of your upkeep, you draw a card and you lose 1 life.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

/// Four cards deep, every Goblin among them taken, and no question asked:
/// the clause is mandatory and unbounded, so the selection takes all matches
/// rather than offering a bounded choice.
static RINGLEADER_DIG: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(4),
    object: Some(ObjectPredicateDef::Subtype("Goblin")),
    minimum: 0,
    maximum: 4,
    select_all_matching: true,
    reveal_selected: true,
    selected_zone: ZoneKind::Hand,
    selected_placement: ZonePlacement::Top,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Bottom,
    selected_order_follows_choice: false,
    then: None,
    selected_face_down: None,
};

// APC 62 — Goblin Ringleader
pub(in crate::card::sets) static GOBLIN_RINGLEADER: CardRecord = CardRecord::new_with_legacy_id(
    2027,
    "Goblin Ringleader",
    CardArt::new("b6b2cd77-9552-48b1-80cb-26966323c1ea", "Mark Romanoski"),
    CardSet::Apocalypse,
    // Haste plus a refill is what keeps the deck from running out: each
    // Ringleader tends to find the next one.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Goblin"], 2, 2).with_abilities(&[
        abilities::haste(),
        AbilityDef::triggered(
            "When this creature enters, reveal the top four cards of your library. Put all Goblin cards revealed this way into your hand and the rest on the bottom of your library in any order.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::LookAtTopAndSelect {
                player: EffectRecipientDef::Controller,
                looker: EffectRecipientDef::Controller,
                selection: &RINGLEADER_DIG,
            },
        ),
    ]),
);

/// Three life a land, counted among the two cards that actually went. The
/// discard is the opponent's choice, so the payoff cannot be known until
/// they have made it.
static VERDICT_LIFE: EffectDef = EffectDef::GainLife {
    recipient: EffectRecipientDef::Controller,
    amount: ValueDef::Scaled(&VERDICT_PER_LAND),
};

static VERDICT_PER_LAND: ScaledValueDef = ScaledValueDef {
    value: ValueDef::MatchedCount,
    factor: 3,
};

static VERDICT_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

// APC 102 — Gerrard's Verdict
pub(in crate::card::sets) static GERRARDS_VERDICT: CardRecord = CardRecord::new_with_legacy_id(
    2067,
    "Gerrard's Verdict",
    CardArt::new("583740c0-8b3d-4f2a-9e1c-6b5d8a3f2c7e", "Carl Critchlow"),
    CardSet::Apocalypse,
    // Two cards for two mana, and the life is what makes it a fine turn-two
    // play against a deck full of lands.
    CardRules::new_sorcery(mana_cost!("{W}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player discards two cards. You gain 3 life for each land card discarded this way.",
        &VERDICT_TARGET,
        EffectDef::Discard {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
            selection: DiscardSelectionDef::RecipientChooses,
            then: Some(DiscardFollowUpDef {
                counted: ObjectPredicateDef::HasType(CardType::Land),
                effect: &VERDICT_LIFE,
            }),
        },
    )),
);

// APC 126 — Vindicate
pub(in crate::card::sets) static VINDICATE: CardRecord = CardRecord::new_with_legacy_id(
    278,
    "Vindicate",
    CardArt::new("2a1bfefd-dae8-49e9-9d56-cc852e3dc93b", "Brian Snõddy"),
    CardSet::Apocalypse,
    CardRules::new_sorcery(mana_cost!("{1}{W}{B}")).with_ability(AbilityDef::destroy_target(
        "Destroy target permanent.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Any),
        true,
    )),
);

static FIRE_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef {
    predicate: AbilityTargetPredicate::AnyTarget,
    minimum: 1,
    maximum: 2,
    divided_total: Some(DividedTotal::Fixed(2)),
    another: false,
}];

const fn fire_rules() -> CardRules {
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Fire deals 2 damage divided as you choose among one or two targets.",
        &FIRE_TARGETS,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::DividedAmongTargets,
        },
    ))
}

static ICE_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::Any,
)];

static ICE_EFFECTS: [EffectDef; 2] = [
    EffectDef::Tap {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
];

const fn ice_rules() -> CardRules {
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Tap target permanent.\nDraw a card.",
        &ICE_TARGETS,
        EffectDef::Sequence(&ICE_EFFECTS),
    ))
}

fn fire_ice_composition() -> CardComposition {
    let fire = fire_rules();
    let ice = ice_rules();
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Fire", fire),
            CardPart::new(CardPartId(1), "Ice", ice),
        ],
        structure: CardStructure::Split {
            parts: vec![CardPartId::PRIMARY, CardPartId(1)],
            fused: None,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Fire",
                SpellForm::Part(CardPartId::PRIMARY),
                fire.mana_cost().expect("Fire has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Ice",
                SpellForm::Part(CardPartId(1)),
                ice.mana_cost().expect("Ice has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

// APC 128 — Fire // Ice
pub(in crate::card::sets) static FIRE_ICE: CardRecord = CardRecord::new_with_legacy_id(
    306,
    "Fire // Ice",
    CardArt::new(
        "f98f4538-5b5b-475d-b98f-49d01dae6f04",
        "David Martin & Franz Vohwinkel",
    ),
    CardSet::Apocalypse,
    fire_rules(),
)
.with_composition(fire_ice_composition);

/// "They're still lands" is not flavour: adding the creature type rather
/// than replacing the land one is what keeps them tapping for mana, and what
/// makes a board wipe answer the whole mana base.
static LIFE_ANIMATION: [AppliedEffectDef; 2] = [
    AppliedEffectDef::add_card_types(crate::card::CardTypeSet::single(CardType::Creature)),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
];

const fn life_rules() -> CardRules {
    CardRules::new_sorcery(mana_cost!("{G}")).with_ability(AbilityDef::spell(
        "All lands you control become 1/1 creatures until end of turn. They're still lands.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Land),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::Composite(&LIFE_ANIMATION),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    ))
}

static DEATH_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Graveyard],
        controller: None,
        // Reanimate takes one from any graveyard; this half is narrower.
        owner: Some(PlayerRelation::You),
    },
)];

static DEATH_EFFECTS: [EffectDef; 2] = [
    EffectDef::MoveToZone {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        zone: ZoneKind::Battlefield,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        controller: Some(PlayerRelation::You),
    },
    EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::TargetManaValue(TargetIndex::PRIMARY),
    },
];

const fn death_rules() -> CardRules {
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Return target creature card from your graveyard to the battlefield. You lose life equal to its mana value.",
        &DEATH_TARGETS,
        EffectDef::Sequence(&DEATH_EFFECTS),
    ))
}

fn life_death_composition() -> CardComposition {
    let life = life_rules();
    let death = death_rules();
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Life", life),
            CardPart::new(CardPartId(1), "Death", death),
        ],
        structure: CardStructure::Split {
            parts: vec![CardPartId::PRIMARY, CardPartId(1)],
            fused: None,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Life",
                SpellForm::Part(CardPartId::PRIMARY),
                life.mana_cost().expect("Life has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Death",
                SpellForm::Part(CardPartId(1)),
                death.mana_cost().expect("Death has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

// APC 130 — Life // Death
pub(in crate::card::sets) static LIFE_DEATH: CardRecord = CardRecord::new_with_legacy_id(
    2123,
    "Life // Death",
    CardArt::new(
        "7ab75cdb-93a1-4f78-b404-37566295c321",
        "Anthony S. Waters & Edward P. Beard, Jr.",
    ),
    CardSet::Apocalypse,
    life_rules(),
)
.with_composition(life_death_composition);

// APC 140 — Caves of Koilos
pub(in crate::card::sets) static CAVES_OF_KOILOS: CardRecord = CardRecord::new_with_legacy_id(
    297,
    "Caves of Koilos",
    CardArt::new("144dd08e-451e-4438-b572-7a138e1a15f3", "Jim Nelson"),
    CardSet::Apocalypse,
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {W} or {B}. This land deals 1 damage to you.",
        &[ManaColor::White, ManaColor::Black],
    )),
);

// APC 141 — Llanowar Wastes
pub(in crate::card::sets) static LLANOWAR_WASTES: CardRecord = CardRecord::new_with_legacy_id(
    298,
    "Llanowar Wastes",
    CardArt::new("610b7cd5-5532-45a9-acfe-24a818034d1c", "Rob Alexander"),
    CardSet::Apocalypse,
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {B} or {G}. This land deals 1 damage to you.",
        &[ManaColor::Black, ManaColor::Green],
    )),
);

// APC 143 — Yavimaya Coast
pub(in crate::card::sets) static YAVIMAYA_COAST: CardRecord = CardRecord::new_with_legacy_id(
    299,
    "Yavimaya Coast",
    CardArt::new("177ee102-d981-4fc3-9f09-9dd07755f22c", "Anthony S. Waters"),
    CardSet::Apocalypse,
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {G} or {U}. This land deals 1 damage to you.",
        &[ManaColor::Green, ManaColor::Blue],
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &PHYREXIAN_ARENA,
    &GOBLIN_RINGLEADER,
    &GERRARDS_VERDICT,
    &VINDICATE,
    &FIRE_ICE,
    &LIFE_DEATH,
    &CAVES_OF_KOILOS,
    &LLANOWAR_WASTES,
    &YAVIMAYA_COAST,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
