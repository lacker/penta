//! Dragons of Tarkir cards cataloged as cross-format rules-engine test cases.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ManaRestrictionDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SubtypeDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "DTK",
    slug: "dragons-of-tarkir",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// DTK 4 — Artful Maneuver
pub(in crate::card::sets) static ARTFUL_MANEUVER: CardRecord = CardRecord::new(
    "Artful Maneuver",
    "7fcaf67e-ba97-4af9-8c47-dbca703cba35",
    "Lars Grant-West",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature gets +2/+2 until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::rebound(),
    ]),
);

// DTK 120 — Sidisi, Undead Vizier
// Audit: unsupported — Exploit must emit a distinct sacrifice-mechanic event and trigger only while Sidisi is still present. A sacrifice follow-up would incorrectly resolve the tutor in the entry trigger and even after sacrificing Sidisi herself; there is no exploit event matcher.
pub(in crate::card::sets) static SIDISI_UNDEAD_VIZIER_120: CardRecord = CardRecord::new(
    "Sidisi, Undead Vizier",
    "6ea5dbba-6114-4d97-9363-817ab9e896d3",
    "Min Yum",
    crate::card::CardRules::unsupported(),
);

// DTK 138 — Dragonlord's Servant
pub(in crate::card::sets) static DRAGONLORD_S_SERVANT: CardRecord = CardRecord::new(
    "Dragonlord's Servant",
    "0ffcdd54-b6be-4d42-82c0-ae927037e859",
    "Steve Prescott",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Shaman"], 1, 3).with_abilities(&[
        abilities::spell_cost_reduction(
            "Dragon spells you cast cost {1} less to cast.",
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dragon")),
            PlayerRelation::You,
            ValueDef::Constant(1),
        ),
    ]),
);

// DTK 140 — Impact Tremors
pub(in crate::card::sets) static IMPACT_TREMORS: CardRecord = CardRecord::new(
    "Impact Tremors",
    "56fb4035-197b-4d28-9bf7-bb62c304067e",
    "Lake Hurwitz",
    CardRules::new_enchantment(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a creature you control enters, this enchantment \
         deals 1 damage to each opponent.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ]),
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)),
    )]),
);

// DTK 150 — Rending Volley
pub(in crate::card::sets) static RENDING_VOLLEY_150: CardRecord = CardRecord::new(
    "Rending Volley",
    "8234090e-9df1-4915-90ef-8a4bc6212655",
    "Lucas Graciano",
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[
        abilities::cannot_be_countered(),
        AbilityDef::spell_with_targets(
            "Rending Volley deals 4 damage to target white or blue creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Color(ManaColor::White),
                        ObjectPredicateDef::Color(ManaColor::Blue),
                    ]),
                ]),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(4),
            ),
        ),
    ]),
);

// DTK 156 — Seismic Rupture
pub(in crate::card::sets) static SEISMIC_RUPTURE: CardRecord = CardRecord::new(
    "Seismic Rupture",
    "9b952e4e-c1ed-4455-90d5-46b56478e6b0",
    "Jason A. Engle",
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::spell(
        "Seismic Rupture deals 2 damage to each creature without flying.",
        EffectDef::damage(
            EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                        KeywordAbility::Flying,
                    )),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ))),
            ValueDef::Constant(2),
        ),
    )]),
);

// DTK 164 — Twin Bolt
pub(in crate::card::sets) static TWIN_BOLT: CardRecord = CardRecord::new(
    "Twin Bolt",
    "5bd58ec4-34a9-4fc2-b057-438492e2e06e",
    "Svetlin Velinov",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Twin Bolt deals 2 damage divided as you choose among one or \
         two targets.",
        &[AbilityTargetDef {
            minimum: 1,
            maximum: 2,
            divided_total: Some(crate::card::DividedTotal::Fixed(2)),
            ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)
        }],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::DividedAmongTargets,
        ),
    )]),
);

// DTK 191 — Inspiring Call
// Audit: unsupported — Needs a battlefield-typed saved object group that the ability-grant validator can accept after drawing cards; plain object bindings are treated as possibly nonbattlefield and cannot receive temporary indestructible.
pub(in crate::card::sets) static INSPIRING_CALL: CardRecord = CardRecord::new(
    "Inspiring Call",
    "c565b991-7021-4b81-b9c0-f7231daae360",
    "Dan Murayama Scott",
    CardRules::unsupported(),
);

// DTK 210 — Surrak, the Hunt Caller
// Audit: unsupported — Needs the intervening-if evaluator to aggregate the total power of controlled creatures at both trigger creation and resolution; AggregateObjectValues currently works only during effect resolution.
pub(in crate::card::sets) static SURRAK_THE_HUNT_CALLER: CardRecord = CardRecord::new(
    "Surrak, the Hunt Caller",
    "b374446d-44bc-4ac5-9829-8c49f0cca173",
    "Wesley Burt",
    CardRules::unsupported(),
);

// DTK 224 — Kolaghan's Command
pub(in crate::card::sets) static KOLAGHAN_S_COMMAND: CardRecord = CardRecord::new(
    "Kolaghan's Command",
    "7c884e1e-fecb-4330-b3de-5fc2a60f7173",
    "Daarken",
    // Three mana that is two cards on every board: something always comes
    // back, and something of theirs always goes.
    CardRules::new_instant(mana_cost!("{1}{B}{R}")).with_ability(
        AbilityDef::modal_spell(
            "Choose two —",
            // Two of four, and never the same one twice. Each mode carries its own
            // slot, so what a Command declares depends on which two it is.
            &[
                AbilityDef::spell_with_targets(
                    "Return target creature card from your graveyard to your hand.",
                    // Your own graveyard, and a creature card while it is still a card: what
                    // comes back goes to hand rather than to the battlefield.
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &[ZoneKind::Graveyard],
                            controller: None,
                            owner: Some(PlayerRelation::You),
                        },
                    )],
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                ),
                AbilityDef::spell_with_targets(
                    "Target player discards a card.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Player(PlayerRelation::Any),
                    )],
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Destroy target artifact.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Artifact),
                    )],
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        then: None,
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Kolaghan's Command deals 2 damage to any target.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::AnyTarget,
                    )],
                    EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(2),
                    ),
                ),
            ],
        )
        .with_mode_selection(2, 2, false),
    ),
);

// DTK 231 — Savage Ventmaw
// Audit: unsupported — Needs mana-retention expiry attached to the six mana units generated by this trigger; retention permissions currently apply to a player's pool rather than only those particular units.
pub(in crate::card::sets) static SAVAGE_VENTMAW: CardRecord = CardRecord::new(
    "Savage Ventmaw",
    "690008d1-d1fe-49ad-810c-84be57cecc6c",
    "Slawomir Maniak",
    CardRules::unsupported(),
);

// DTK 249 — Haven of the Spirit Dragon
pub(in crate::card::sets) static HAVEN_OF_THE_SPIRIT_DRAGON: CardRecord = CardRecord::new(
    "Haven of the Spirit Dragon",
    "9910042e-091f-47da-ad29-8f76c9d3b8c1",
    "Raymond Swanland",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color. Spend this mana only to cast a Dragon creature \
             spell.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color().with_restrictions(&[
                ManaRestrictionDef::CastSpell(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dragon")),
                ])),
            ])),
        ),
        AbilityDef::activated_with_targets(
            "{2}, {T}, Sacrifice this land: Return target Dragon creature card or Ugin \
             planeswalker card from your graveyard to your hand.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dragon")),
                        ]),
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Ugin")),
                        ]),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARTFUL_MANEUVER,
    &SIDISI_UNDEAD_VIZIER_120,
    &DRAGONLORD_S_SERVANT,
    &IMPACT_TREMORS,
    &RENDING_VOLLEY_150,
    &SEISMIC_RUPTURE,
    &TWIN_BOLT,
    &INSPIRING_CALL,
    &SURRAK_THE_HUNT_CALLER,
    &KOLAGHAN_S_COMMAND,
    &SAVAGE_VENTMAW,
    &HAVEN_OF_THE_SPIRIT_DRAGON,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
