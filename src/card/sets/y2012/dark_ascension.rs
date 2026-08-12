//! Dark Ascension card records used by the built-in ISD–RTR Standard deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AppliedEffectDef, CardArt, CardComposition, CardEffectStatus, CardPart, CardRules, CardSet,
    CardStructure, CardType, ComparisonDef, ConditionalValueDef, DoubleFacedKind, EffectDef,
    EffectDurationDef, EffectRecipientDef, ManaColor, ObjectPredicateDef, PlayOptionDef,
    PlayerRelation, QuantifierDef, SpellForm, TriggerConditionDef, TriggerEventDef, TurnStepDef,
    ValueDef, ZoneKind, abilities, cards,
};
use crate::ids::{CardPartId, PlayOptionId, TargetIndex};
use crate::mana_cost;

// DKA 17 — Ray of Revelation
pub(in crate::card::sets) static RAY_OF_REVELATION: CardRecord = CardRecord::new(
    cards::RAY_OF_REVELATION,
    "Ray of Revelation",
    CardArt::new("d7e2c5a4-cf92-46bd-9033-8036436488cb", "Cliff Childs"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target enchantment.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Enchantment),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        ),
        abilities::flashback(mana_cost!("{G}")),
    ]),
);

/// Morbid replaces the amount rather than adding a second effect, so both
/// printed clauses come down to which number this picks.
const TRAGIC_SLIP_AMOUNT: ValueDef = ValueDef::IfCreatureDiedThisTurn(&ConditionalValueDef {
    then: ValueDef::Constant(-13),
    otherwise: ValueDef::Constant(-1),
});

// DKA 76 — Tragic Slip
pub(in crate::card::sets) static TRAGIC_SLIP: CardRecord = CardRecord::new(
    cards::TRAGIC_SLIP,
    "Tragic Slip",
    CardArt::new("09666671-601e-4fca-bdfb-fb288bf2672c", "Christopher Moeller"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{B}")).with_abilities(&[
        AbilityDef::spell_with_targets("Target creature gets -1/-1 until end of turn.", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )], EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: TRAGIC_SLIP_AMOUNT,
                    toughness: TRAGIC_SLIP_AMOUNT,
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            }),
        AbilityDef::static_ability(
            "Morbid — That creature gets -13/-13 until end of turn instead if a creature died this turn.",
            // The conditional value on the spell clause above already
            // carries this modifier; this clause has no second effect to run.
            EffectDef::None,
        )
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The morbid amount is chosen by the value on the preceding clause.",
        )),
    ]),
);

// DKA 93 — Hellrider
pub(in crate::card::sets) static HELLRIDER: CardRecord = CardRecord::new(
    cards::HELLRIDER,
    "Hellrider",
    CardArt::new("0ec8d800-7f06-44e0-b22d-cdff0a9b153d", "Svetlin Velinov"),
    CardSet::DarkAscension,
    CardRules::new_creature(
        mana_cost!("{2}{R}{R}"),
        &["Devil"],
        3,
        3,
    )
    .with_abilities(&[
        abilities::haste(),
        AbilityDef::triggered(
            "Whenever a creature you control attacks, this creature deals 1 damage to the player or planeswalker it's attacking.",
            TriggerEventDef::Attacks(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::DealDamage {
                // With no planeswalkers in the game, the player an attacker is
                // attacking is always the defending player.
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// DKA 127 — Strangleroot Geist
pub(in crate::card::sets) static STRANGLEROOT_GEIST: CardRecord = CardRecord::new(
    cards::STRANGLEROOT_GEIST,
    "Strangleroot Geist",
    CardArt::new("bf1fb137-205c-480f-b6dc-dfa137793ae3", "Jason Chan"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Spirit"], 2, 1)
        .with_abilities(&[abilities::haste(), abilities::undying()]),
);

const fn huntmaster_front_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{2}{R}{G}"), &["Human", "Werewolf"], 2, 2)
        .with_abilities(&HUNTMASTER_FRONT_ABILITIES)
}

/// Entering and transforming into this face do the same thing, so the printed
/// sentence is two triggers watching two different events.
static HUNTMASTER_FRONT_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::triggered(
        "Whenever this creature enters, create a 2/2 green Wolf creature token and you gain 2 life.",
        TriggerEventDef::ZoneChanged {
            object: ObjectPredicateDef::Source,
            from: None,
            to: Some(ZoneKind::Battlefield),
        },
        HUNTMASTER_WOLF_AND_LIFE,
    ),
    AbilityDef::triggered(
        "Whenever this creature transforms into Huntmaster of the Fells, create a 2/2 green Wolf creature token and you gain 2 life.",
        TriggerEventDef::TransformsIntoThisFace,
        HUNTMASTER_WOLF_AND_LIFE,
    ),
    AbilityDef::triggered_if(
        "At the beginning of each upkeep, if no spells were cast last turn, transform this creature.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::Any,
        },
        &NO_SPELLS_LAST_TURN,
        EffectDef::Transform {
            object: EffectRecipientDef::Source,
        },
    ),
];

static HUNTMASTER_WOLF_AND_LIFE: EffectDef = EffectDef::Sequence(&[
    EffectDef::CreateToken {
        token: cards::WOLF_TOKEN_2_2_GREEN,
        count: ValueDef::Constant(1),
    },
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    },
]);

/// Nobody cast anything, so every player has to be at zero.
static NO_SPELLS_LAST_TURN: TriggerConditionDef = TriggerConditionDef::SpellsCastLastTurn {
    quantifier: QuantifierDef::Every,
    player: PlayerRelation::Any,
    comparison: ComparisonDef::LessOrEqual,
    amount: 0,
};

/// One player is enough, which is why this side turns back sooner than the
/// other side turns over.
static TWO_SPELLS_LAST_TURN: TriggerConditionDef = TriggerConditionDef::SpellsCastLastTurn {
    quantifier: QuantifierDef::Any,
    player: PlayerRelation::Any,
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 2,
};

static HUNTMASTER_BACK_ABILITIES: [AbilityDef; 3] = [
    abilities::trample(),
    AbilityDef::triggered_with_targets(
        "Whenever this creature transforms into Ravager of the Fells, it deals 2 damage to target opponent or planeswalker and 2 damage to up to one target creature that player or that planeswalker's controller controls.",
        TriggerEventDef::TransformsIntoThisFace,
        &RAVAGER_TARGETS,
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex(1)),
                amount: ValueDef::Constant(2),
            },
        ]),
    ),
    AbilityDef::triggered_if(
        "At the beginning of each upkeep, if a player cast two or more spells last turn, transform this creature.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::Any,
        },
        &TWO_SPELLS_LAST_TURN,
        EffectDef::Transform {
            object: EffectRecipientDef::Source,
        },
    ),
];

/// The second slot reads the first: the creature has to belong to whoever the
/// damage was aimed at.
static RAVAGER_TARGETS: [AbilityTargetDef; 2] = [
    AbilityTargetDef::exactly_one(AbilityTargetPredicate::PlayerOrPlaneswalker(
        PlayerRelation::Opponent,
    )),
    AbilityTargetDef::up_to(
        AbilityTargetPredicate::ControlledByTargetOf {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            slot: TargetIndex::PRIMARY,
        },
        1,
    ),
];

const fn huntmaster_back_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 4, 4)
        .printed_colors(&[ManaColor::Red, ManaColor::Green])
        .with_abilities(&HUNTMASTER_BACK_ABILITIES)
}

fn huntmaster_composition() -> CardComposition {
    let front = huntmaster_front_rules();
    let back = huntmaster_back_rules();
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Huntmaster of the Fells", front),
            CardPart::new(CardPartId(1), "Ravager of the Fells", back),
        ],
        structure: CardStructure::DoubleFaced {
            front: CardPartId::PRIMARY,
            back: CardPartId(1),
            kind: DoubleFacedKind::Transforming,
        },
        play_options: vec![PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Huntmaster of the Fells",
            SpellForm::Part(CardPartId::PRIMARY),
            front
                .mana_cost()
                .expect("Huntmaster of the Fells has a printed mana cost"),
            CardEffectStatus::MetadataOnly,
        )],
    }
}

// DKA 140 — Huntmaster of the Fells
pub(in crate::card::sets) static HUNTMASTER_OF_THE_FELLS: CardRecord = CardRecord::new(
    cards::HUNTMASTER_OF_THE_FELLS,
    "Huntmaster of the Fells",
    CardArt::new("aae6fb12-b252-453b-bca7-1ea2a0d6c8dc", "Chris Rahn"),
    CardSet::DarkAscension,
    huntmaster_front_rules(),
)
.with_composition(huntmaster_composition);

// DKA 158 — Vault of the Archangel
pub(in crate::card::sets) static VAULT_OF_THE_ARCHANGEL: CardRecord = CardRecord::new(
    cards::VAULT_OF_THE_ARCHANGEL,
    "Vault of the Archangel",
    CardArt::new("35a65437-430a-42ef-854f-6e66f8e1a04a", "John Avon"),
    CardSet::DarkAscension,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{2}{W}{B}, {T}: Creatures you control gain deathtouch and lifelink until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{W}{B}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::MatchingObjects {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: PlayerRelation::You,
                    },
                    effect: AppliedEffectDef::GrantAbility(&abilities::deathtouch()),
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::MatchingObjects {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: PlayerRelation::You,
                    },
                    effect: AppliedEffectDef::GrantAbility(&abilities::lifelink()),
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);
pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &RAY_OF_REVELATION,
    &TRAGIC_SLIP,
    &HELLRIDER,
    &STRANGLEROOT_GEIST,
    &HUNTMASTER_OF_THE_FELLS,
    &VAULT_OF_THE_ARCHANGEL,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
