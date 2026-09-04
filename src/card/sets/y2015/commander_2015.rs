//! Commander 2015 cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityKindDef, AbilityPredicateDef, AbilityTargetDef, AbilityTargetPredicate,
    AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet, CardType, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, PlayerRelation, TriggerEventDef, ValueDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// C15 14 — Mystic Confluence
pub(in crate::card::sets) static MYSTIC_CONFLUENCE: CardRecord = CardRecord::new_with_legacy_id(
    2229,
    "Mystic Confluence",
    CardArt::new("81bbffc2-6f58-4baa-8f95-168eab106b15", "Kieran Yanner"),
    CardSet::Commander2015,
    // Five mana that is never dead: three cards when nothing is happening, a
    // hard counter plus a card when something is.
    CardRules::new_instant(mana_cost!("{3}{U}{U}")).with_ability(
        AbilityDef::modal_spell(
            "Choose three. You may choose the same mode more than once.",
            // Each mode declares its own target slot, and a mode chosen twice gets two
            // of them -- which is what "you may choose the same mode more than once"
            // means for a clause that targets.
            &[
                AbilityDef::spell_with_targets(
                    "Counter target spell unless its controller pays {3}.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::Spell,
                            zones: &[ZoneKind::Stack],
                            controller: None,
                            owner: None,
                        },
                    )],
                    abilities::counter_target_unless_paid(ValueDef::Constant(3)),
                ),
                AbilityDef::spell_with_targets(
                    "Return target creature to its owner's hand.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                    },
                ),
                AbilityDef::spell(
                    "Draw a card.",
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ),
            ],
        )
        .with_mode_selection(3, 3, true),
    ),
);

// C15 26 — Fiery Confluence
pub(in crate::card::sets) static FIERY_CONFLUENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7b61c9bc-16e8-417f-99e7-8bd83d4666c5"),
    "Fiery Confluence",
    CardArt::new("7b61c9bc-16e8-417f-99e7-8bd83d4666c5", "Kieran Yanner"),
    CardSet::Commander2015,
    // Four mana that is a burn spell, a sweeper, or artifact removal, and
    // usually two of the three at once.
    CardRules::new_sorcery(mana_cost!("{2}{R}{R}")).with_ability(
        AbilityDef::modal_spell(
            "Choose three. You may choose the same mode more than once.",
            // Three modes chosen three times between them: six damage to the other
            // player, three damage to every creature, three artifacts destroyed, or any
            // mixture of the three.
            &[
                AbilityDef::spell(
                    "Fiery Confluence deals 1 damage to each creature.",
                    // "Deals 1 damage to each creature": everything on the battlefield, yours
                    // included, which is what makes the sweeper half a cost as well as an
                    // answer.
                    EffectDef::DealDamage {
                        recipient: EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ),
                        amount: ValueDef::Constant(1),
                    },
                ),
                AbilityDef::spell(
                    "Fiery Confluence deals 2 damage to each opponent.",
                    EffectDef::DealDamage {
                        recipient: EffectRecipientDef::Opponent,
                        amount: ValueDef::Constant(2),
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
            ],
        )
        .with_mode_selection(3, 3, true),
    ),
);

// C15 34 — Caller of the Pack
pub(in crate::card::sets) static CALLER_OF_THE_PACK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1286208b-896b-4f41-a837-1c8a2b199a0f"),
    "Caller of the Pack",
    CardArt::new("1286208b-896b-4f41-a837-1c8a2b199a0f", "Ryan Yee"),
    CardSet::Commander2015,
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Beast"], 8, 6)
        .with_abilities(&[abilities::trample(), abilities::myriad()]),
);

// C15 69 — Faith's Fetters
pub(in crate::card::sets) static FAITH_S_FETTERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5b8ffba3-44a9-41ce-a5a1-37413346db2f"),
    "Faith's Fetters",
    CardArt::new("fe653236-c5c1-4dcd-95cd-3c53f1e256ef", "Brian Despain"),
    CardSet::Commander2015,
    // Four mana and four life for an answer that reaches anything, which is
    // what a slow deck pays for not having to guess what it will face.
    CardRules::new_enchantment(mana_cost!("{3}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell(
                "Enchant permanent",
                &const {
                    [AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::Any,
                    )]
                },
            ),
            abilities::enters_trigger(
                "When this Aura enters, you gain 4 life.",
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(4),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted permanent can't attack or block, and its activated abilities can't be \
                 activated unless they're mana abilities.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    // The mana exception is why this is NonManaActivated
                    // rather than Any: a land or rock it lands on still taps
                    // for mana.
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                        AppliedEffectDef::cannot_activate_abilities(AbilityPredicateDef::Is(
                            AbilityKindDef::NonManaActivated,
                        )),
                    ]),
                },
            ),
        ]),
);

// C15 99 — Ninja of the Deep Hours
pub(in crate::card::sets) static NINJA_OF_THE_DEEP_HOURS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("367a67c7-54db-4336-b55a-3fa27625172a"),
    "Ninja of the Deep Hours",
    CardArt::new("26184ff2-3b8c-419a-9b28-95d6e4e996bb", "Dan Murayama Scott"),
    CardSet::Commander2015,
    // Nobody casts this for four. Two mana off an unblocked one-drop is the
    // card: the attacker that got through goes back to be replayed, and the
    // 2/2 that replaced it is already connecting for a card a turn.
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Human", "Ninja"], 2, 2).with_abilities(&[
        abilities::ninjutsu(
            "Ninjutsu {1}{U} ({1}{U}, Return an unblocked attacker you control to hand: Put this \
             card onto the battlefield from your hand tapped and attacking.)",
            mana_cost!("{1}{U}"),
        ),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, you may draw a card.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &MYSTIC_CONFLUENCE,
    &FIERY_CONFLUENCE,
    &CALLER_OF_THE_PACK,
    &FAITH_S_FETTERS,
    &NINJA_OF_THE_DEEP_HOURS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
