//! Commander 2018 cards cataloged for the Vintage Cube pool.

use crate::ParentBinding;
use crate::card::AppliedEffectDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardSupertype;
use crate::card::CardTypeSet;
use crate::card::CounterKind;
use crate::card::CreatedTokensDef;
use crate::card::ManaColor;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TurnStepDef;
use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AddManaEffectDef;
use crate::card::CardNameSetDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::ControlDurationDef;
use crate::card::CostDef;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetCountConditionDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::ObjectSetPredicateDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "C18",
    slug: "commander-2018",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// C18 23 — Loyal Apprentice
pub(in crate::card::sets) static LOYAL_APPRENTICE_23: CardRecord = CardRecord::new(
    "Loyal Apprentice",
    "b5fef5fe-3e59-4fd6-8a88-7c1cee56892f",
    "Joe Slucher",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Artificer"], 2, 1).with_abilities(&[
abilities::haste(),
AbilityDef::triggered_if("Lieutenant — At the beginning of combat on your turn, if you control your commander, create a 1/1 colorless Thopter artifact creature token with flying. That token gains haste until end of turn.", TriggerEventDef::StepBegins { step: TurnStepDef::BeginningOfCombat, player: PlayerRelation::You }, &TriggerConditionDef::ObjectCount { query: ObjectQueryDef::matching(ObjectPredicateDef::Commander, &[ZoneKind::Battlefield], PlayerRelation::You), comparison: ComparisonDef::GreaterOrEqual, amount: 1 }, EffectDef::create_artifact_creature_token(&["Thopter"], &[], 1, 1).with_abilities(&[abilities::flying()]).with_created_tokens(CreatedTokensDef { binding: ParentBinding, then: &EffectDef::Apply { recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)), effect: AppliedEffectDef::add_ability(&abilities::haste()), duration: ResolvedEffectDurationDef::UntilEndOfTurn } }))
]),
);

// C18 38 — Arixmethes, Slumbering Isle
pub(in crate::card::sets) static ARIXMETHES_SLUMBERING_ISLE_38: CardRecord = CardRecord::new(
    "Arixmethes, Slumbering Isle",
    "c723c7dc-a452-49ec-a4e5-9e516fe530e9",
    "Dimitar Marinski",
    CardRules::new_creature(mana_cost!("{2}{G}{U}"), &["Kraken"], 12, 12).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::as_enters("Arixmethes enters tapped with five slumber counters on it.", ReplacementEffectDef::Sequence(&[ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped), ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::AddCounters { kind: CounterKind::named("slumber"), amount: 5 })])),
AbilityDef::static_ability("As long as Arixmethes has a slumber counter on it, it's a land. (It's not a creature.)", EffectDef::IfCondition { condition: &TriggerConditionDef::SourceCounters { kind: CounterKind::named("slumber"), comparison: ComparisonDef::Greater, amount: 0 }, then: &EffectDef::StaticApply { recipient: EffectRecipientDef::Source, effect: AppliedEffectDef::set_card_types(CardTypeSet::single(CardType::Land)) } }),
AbilityDef::triggered("Whenever you cast a spell, you may remove a slumber counter from Arixmethes.", TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)), EffectDef::May { player: EffectRecipientDef::Controller, effect: &EffectDef::RemoveCounters { object: EffectRecipientDef::Source, kind: CounterKind::named("slumber"), amount: ValueDef::Constant(1) } }),
abilities::tap_for_mana("{T}: Add {G}{U}.", AddManaEffectDef::one_of_each(ManaColor::Green, ManaColor::Blue))
]),
);

// C18 54 — Coveted Jewel
pub(in crate::card::sets) static COVETED_JEWEL: CardRecord = CardRecord::new(
    "Coveted Jewel",
    "f83ed433-fae3-4fa5-acad-bb8a5b535ce3",
    "Jason A. Engle",
// Six mana for three cards and a Gilded Lotus, held only as long as you
    // can stop them getting through -- and they untap it on the way out.
    CardRules::new_artifact(mana_cost!("{6}")).with_abilities(&[
        abilities::enters_trigger(
            "When this artifact enters, draw three cards.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add three mana of any one color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color().with_amount(3)),
        ),
        AbilityDef::triggered(
            "Whenever one or more creatures an opponent controls attack you and aren't blocked, that \
             player draws three cards and gains control of this artifact. Untap it.",
            // "One or more creatures an opponent controls attack you and aren't
            // blocked": one trigger for the whole unblocked crew, not one apiece, and
            // only for an attack aimed at you rather than at something you control.
            TriggerEventDef::UnblockedAttackersDeclared {
                attacker: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                ]),
                defender: PlayerRelation::You,
            },
            // The reward, the theft, and the untap, in printed order: the attacker has
            // the cards before the artifact changes hands, and it arrives ready to be
            // tapped again on their own turn.
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(3),
                },
                EffectDef::gain_control(
                    EffectRecipientDef::Source,
                    PlayerRefDef::Opponent,
                    ControlDurationDef::Indefinitely,
                ),
                EffectDef::Untap {
                    object: EffectRecipientDef::Source,
                },
            ]),
        ),
    ]),
);

// C18 55 — Endless Atlas
pub(in crate::card::sets) static ENDLESS_ATLAS: CardRecord = CardRecord::new(
    "Endless Atlas",
    "d2547a42-b2b9-4887-8671-4cf63a7b0eff",
    "Titus Lunter",
CardRules::new_artifact(mana_cost!("{2}")).with_ability(
        AbilityDef::activated(
            "{2}, {T}: Draw a card. Activate only if you control three or more lands with the same name.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::TapSource,
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )
        .with_activation_condition(&TriggerConditionDef::ObjectSetCount(
            &ObjectSetCountConditionDef {
                objects: &ObjectSetDef::Matching {
                    objects: &ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::NameIn(&
                        CardNameSetDef::NamesAppearingAtLeast {
                            objects: &ObjectSetDef::Query(ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Land),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                            count: 3,
                        },
                    )),
                },
                predicate: ObjectSetPredicateDef {
                    filter: None,
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
            },
        )),
    ),
);

// C18 57 — Retrofitter Foundry
pub(in crate::card::sets) static RETROFITTER_FOUNDRY: CardRecord = CardRecord::new(
    "Retrofitter Foundry",
    "5da578b8-19e6-4068-9336-e7cd33c585f1",
    "Dmitry Burmak",
    // One mana on turn one and a mana sink for the rest of the game, which
    // is why it is played in decks with no other artifacts at all.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        // No tap in the cost, so the untap is what makes every other ability
        // usable more than once a turn -- and usable on their turn.
        AbilityDef::activated(
            "{3}: Untap this artifact.",
            &[CostDef::Mana(mana_cost!("{3}"))],
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
        AbilityDef::activated(
            "{2}, {T}: Create a 1/1 colorless Servo artifact creature token.",
            // The line the card is played for: with four mana up on their turn it
            // untaps and makes a Servo, and the Servo becomes a Thopter and the Thopter
            // a 4/4, one tap at a time.
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::artifact_creature(&["Servo"], &[], 1, 1),
            ))),
        ),
        AbilityDef::activated(
            "{1}, {T}, Sacrifice a Servo: Create a 1/1 colorless Thopter artifact creature token \
             with flying.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::TapSource,
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Servo")),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::artifact_creature(&["Thopter"], &[], 1, 1)
                    .with_abilities(&[abilities::flying()]),
            ))),
        ),
        AbilityDef::activated(
            "{T}, Sacrifice a Thopter: Create a 4/4 colorless Construct artifact creature token.",
            &[
                CostDef::TapSource,
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Thopter")),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::artifact_creature(&["Construct"], &[], 4, 4),
            ))),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[
    &LOYAL_APPRENTICE_23,
    &ARIXMETHES_SLUMBERING_ISLE_38,
    &COVETED_JEWEL,
    &ENDLESS_ATLAS,
    &RETROFITTER_FOUNDRY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
