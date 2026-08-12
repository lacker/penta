//! Dragon's Maze card records used by the built-in ISD–RTR Standard decks.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AnimationDef, AppliedEffectDef, CardArt, CardBehavior, CardComposition, CardEffectStatus,
    CardPart, CardRules, CardSet, CardStructure, CardSupertype, CardType, ColorSet, EffectDef,
    EffectDurationDef, EffectExecutionDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    PlayOptionDef, PlayerRelation, SpellForm, TriggerConditionDef, TriggerEventDef, TurnStepDef,
    ValueDef, ZoneKind, abilities, cards,
};
use crate::ids::{CardPartId, PlayOptionId, TargetIndex};
use crate::mana_cost;

// DGM 11 — Aetherling
pub(in crate::card::sets) static AETHERLING: CardRecord = CardRecord::new(
    cards::AETHERLING,
    "Aetherling",
    CardArt::new("9c93313b-cf43-47e9-a911-717b4d14b0b5", "Tyler Jacobson"),
    CardSet::DragonsMaze,
    CardRules::new_creature(
        mana_cost!("{4}{U}{U}"),
        &["Shapeshifter"],
        4,
        5,
    )
    .with_abilities(&[
        AbilityDef::activated(
            "{U}: Exile this creature. Return it to the battlefield under its owner's control at the beginning of the next end step.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Sequence(&[
                EffectDef::ExileLinkedToSource {
                    object: EffectRecipientDef::Source,
                },
                // The next end step belongs to whoever's turn it is, which
                // may well be the opponent.
                EffectDef::AtNextStep {
                    step: TurnStepDef::End,
                    player: PlayerRelation::Any,
                    effect: &EffectDef::ReturnLinkedExiles {
                        zone: ZoneKind::Battlefield,
                        grant: None,
                    },
                },
            ]),
        ),
        AbilityDef::activated(
            "{U}: This creature can't be blocked this turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::MakeUnblockableThisTurn {
                object: EffectRecipientDef::Source,
            },
        ),
        AbilityDef::activated(
            "{1}: This creature gets +1/-1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(-1),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{1}: This creature gets -1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(-1),
                    toughness: ValueDef::Constant(1),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DGM 57 — Blood Baron of Vizkopa
pub(in crate::card::sets) static BLOOD_BARON_OF_VIZKOPA: CardRecord = CardRecord::new(
    cards::BLOOD_BARON_OF_VIZKOPA,
    "Blood Baron of Vizkopa",
    CardArt::new("e4edad09-bf7b-40e9-ac2a-100da8a43274", "Anthony Palumbo"),
    CardSet::DragonsMaze,
    CardRules::new_creature(
        mana_cost!("{3}{W}{B}"),
        &["Vampire"],
        4,
        4,
    )
    .with_abilities(&[
        abilities::lifelink(),
        abilities::protection_from(ManaColor::White),
        abilities::protection_from(ManaColor::Black),
        AbilityDef::custom_full(
            "As long as you have 30 or more life and an opponent has 10 or less life, this creature gets +6/+6 and has flying.",
            CardBehavior::BloodBaronOfVizkopa,
            "The conditional power, toughness, and flying effect is implemented by the card-local static-effect hook.",
        ),
    ]),
);

// DGM 72 — Gaze of Granite
pub(in crate::card::sets) static GAZE_OF_GRANITE: CardRecord = CardRecord::new(
    cards::GAZE_OF_GRANITE,
    "Gaze of Granite",
    CardArt::new("96c9ac10-d114-4aa5-87ac-f1069cde8e40", "Nils Hamm"),
    CardSet::DragonsMaze,
    CardRules::new_sorcery(mana_cost!("{X}{B}{B}{G}")).with_ability(AbilityDef::spell(
        "Destroy each nonland permanent with mana value X or less.",
        EffectDef::Destroy {
            object: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    ObjectPredicateDef::ManaValueAtMostValue(ValueDef::ChosenX),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            can_regenerate: true,
        },
    )),
);

// DGM 93 — Putrefy
pub(in crate::card::sets) static PUTREFY: CardRecord = CardRecord::new(
    cards::PUTREFY,
    "Putrefy",
    CardArt::new("0d43a0b6-2a5c-4959-96ee-6e570949dfed", "Igor Kieryluk"),
    CardSet::DragonsMaze,
    CardRules::new_instant(mana_cost!("{1}{B}{G}")).with_ability(AbilityDef::destroy_target(
        "Destroy target artifact or creature. It can't be regenerated.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Creature),
        ])),
        false,
    )),
);

// DGM 99 — Ruric Thar, the Unbowed
pub(in crate::card::sets) static RURIC_THAR_THE_UNBOWED: CardRecord = CardRecord::new(
    cards::RURIC_THAR_THE_UNBOWED,
    "Ruric Thar, the Unbowed",
    CardArt::new("84dd3586-7c3b-4f9c-a1eb-7745b75339b0", "Tyler Jacobson"),
    CardSet::DragonsMaze,
    CardRules::new_creature(
        mana_cost!("{4}{R}{G}"),
        &["Ogre", "Warrior"],
        6,
        6,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::vigilance(),
        abilities::reach(),
        abilities::attacks_each_combat_if_able("Ruric Thar attacks each combat if able."),
        AbilityDef::triggered(
            "Whenever a player casts a noncreature spell, Ruric Thar deals 6 damage to that player.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::NoncreatureSpell),
            EffectDef::DealDamage {
                // Whoever cast it, which is what the event names; this hits
                // its own controller too.
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(6),
            },
        ),
    ]),
);

// DGM 103 — Sin Collector
pub(in crate::card::sets) static SIN_COLLECTOR: CardRecord = CardRecord::new(
    cards::SIN_COLLECTOR,
    "Sin Collector",
    CardArt::new("305a3feb-df49-486c-a3b4-ff2721d60019", "Mike Bierek"),
    CardSet::DragonsMaze,
    CardRules::new_creature(
        mana_cost!("{1}{W}{B}"),
        &["Human", "Cleric"],
        2,
        1,
    )
    .with_abilities(&[AbilityDef::triggered_with_targets("When this creature enters, target opponent reveals their hand. You choose an instant or sorcery card from it and exile that card.", TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            }, &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
        )], EffectDef::None)
        .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::SinCollector))
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The targeted trigger uses the shared stack and a card-local hand-reveal and exile resolver.",
        )),
    ]),
);

// DGM 111 — Unflinching Courage
pub(in crate::card::sets) static UNFLINCHING_COURAGE: CardRecord = CardRecord::new(
    cards::UNFLINCHING_COURAGE,
    "Unflinching Courage",
    CardArt::new("35952c24-d728-4ec6-b0d1-b8183a18554a", "Mike Bierek"),
    CardSet::DragonsMaze,
    CardRules::new_enchantment(mana_cost!("{1}{G}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
        AbilityDef::spell_with_targets("Enchant creature", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )], EffectDef::Attach {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            }),
        AbilityDef::static_ability(
            "Enchanted creature gets +2/+2 and has trample and lifelink. (Damage dealt by the creature also causes its controller to gain that much life.)",
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(2),
                        toughness: ValueDef::Constant(2),
                    },
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::GrantAbility(&abilities::trample()),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::GrantAbility(&abilities::lifelink()),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ]),
        ),
    ]),
);

static VOICE_OF_RESURGENCE_DURING_YOUR_TURN: TriggerConditionDef =
    TriggerConditionDef::ActivePlayer(PlayerRelation::You);

static VOICE_OF_RESURGENCE_TOKEN: EffectDef = EffectDef::CreateToken {
    token: cards::ELEMENTAL_TOKEN_GREEN_WHITE,
    count: ValueDef::Constant(1),
};

// DGM 114 — Voice of Resurgence
pub(in crate::card::sets) static VOICE_OF_RESURGENCE: CardRecord = CardRecord::new(
    cards::VOICE_OF_RESURGENCE,
    "Voice of Resurgence",
    CardArt::new("07246783-d475-4f61-99ac-e2b574072349", "Winona Nelson"),
    CardSet::DragonsMaze,
    CardRules::new_creature(
        mana_cost!("{G}{W}"),
        &["Elemental"],
        2,
        2,
    )
    // One printed sentence, two separate triggers: the cast one only during
    // your turn, and the death one whenever it happens.
    .with_abilities(&[
        AbilityDef::triggered_if(
            "Whenever an opponent casts a spell during your turn, create a green and white Elemental creature token with \"This token's power and toughness are each equal to the number of creatures you control.\"",
            TriggerEventDef::SpellCast(ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent)),
            &VOICE_OF_RESURGENCE_DURING_YOUR_TURN,
            VOICE_OF_RESURGENCE_TOKEN,
        ),
        AbilityDef::triggered(
            "When this creature dies, create a green and white Elemental creature token with \"This token's power and toughness are each equal to the number of creatures you control.\"",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            VOICE_OF_RESURGENCE_TOKEN,
        ),
    ]),
);

// DGM 116 — Warleader's Helix
pub(in crate::card::sets) static WARLEADERS_HELIX: CardRecord = CardRecord::new(
    cards::WARLEADERS_HELIX,
    "Warleader's Helix",
    CardArt::new("81e474ac-54f7-43f9-8af9-2f1adf258b15", "Greg Staples"),
    CardSet::DragonsMaze,
    CardRules::new_instant(mana_cost!("{2}{R}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Warleader's Helix deals 4 damage to any target and you gain 4 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
        ]),
    )),
);

/// Turn repaints its target rather than adding to it: the printed subtypes,
/// abilities, and colours all give way.
static TURN_ANIMATION: AnimationDef =
    AnimationDef::new(0, 1).becoming(&["Weird"], ColorSet::from_colors(&[ManaColor::Red]));

static TURN_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];
static BURN_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

const fn turn_rules() -> CardRules {
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Until end of turn, target creature loses all abilities and becomes a red Weird with base power and toughness 0/1.\nFuse (You may cast one or both halves of this card from your hand.)",
        &TURN_TARGETS,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Animate(&TURN_ANIMATION),
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    ))
}

fn turn_burn_composition() -> CardComposition {
    let turn = turn_rules();
    let burn = CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Burn deals 2 damage to any target.\nFuse (You may cast one or both halves of this card from your hand.)",
            &BURN_TARGETS,
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
    );
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Turn", turn),
            CardPart::new(CardPartId(1), "Burn", burn),
        ],
        structure: CardStructure::Split {
            parts: vec![CardPartId::PRIMARY, CardPartId(1)],
            fused: Some(PlayOptionId(2)),
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Turn",
                SpellForm::Part(CardPartId::PRIMARY),
                turn.mana_cost().expect("Turn has a printed mana cost"),
                CardEffectStatus::MetadataOnly,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Burn",
                SpellForm::Part(CardPartId(1)),
                burn.mana_cost().expect("Burn has a printed mana cost"),
                CardEffectStatus::MetadataOnly,
            ),
            PlayOptionDef::cast(
                PlayOptionId(2),
                "Turn // Burn",
                SpellForm::Combined(vec![CardPartId::PRIMARY, CardPartId(1)]),
                mana_cost!("{3}{U}{R}"),
                CardEffectStatus::MetadataOnly,
            )
            .restricted_to_hand(),
        ],
    }
    .with_derived_spell_targets()
}

// DGM 134 — Turn // Burn
pub(in crate::card::sets) static TURN_BURN: CardRecord = CardRecord::new(
    cards::TURN_BURN,
    "Turn // Burn",
    CardArt::new("8d7fdd59-6d76-4a0c-ac75-816345ef4a39", "Ryan Barger"),
    CardSet::DragonsMaze,
    turn_rules(),
)
.with_composition(turn_burn_composition);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AETHERLING,
    &BLOOD_BARON_OF_VIZKOPA,
    &GAZE_OF_GRANITE,
    &PUTREFY,
    &RURIC_THAR_THE_UNBOWED,
    &SIN_COLLECTOR,
    &UNFLINCHING_COURAGE,
    &VOICE_OF_RESURGENCE,
    &WARLEADERS_HELIX,
    &TURN_BURN,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
