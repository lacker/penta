//! Shards of Alara cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef,
    CardArt, CardRules, CardSet, CardSupertype, CardType, DiscardSelectionDef, EffectDef,
    EffectRecipientDef, KeywordAbility, ManaColor, ObjectPredicateDef, ObjectRefDef, PlayerRefDef,
    PlayerRelation, ResolvedEffectDurationDef, TriggerEventDef, ValueDef, ZoneKind, abilities,
    tokens,
};
use crate::ids::ParentBinding;
use crate::{TargetIndex, mana_cost};

// ALA 9 — Elspeth, Knight-Errant
pub(in crate::card::sets) static ELSPETH_KNIGHT_ERRANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("44c52e52-2b1c-4ca8-ab6d-20d97a342704"),
    "Elspeth, Knight-Errant",
    CardArt::new("44c52e52-2b1c-4ca8-ab6d-20d97a342704", "Volkan Ba\u{11f}a"),
    CardSet::ShardsOfAlara,
    // Four mana, two plus abilities, and neither of them is the safe one:
    // she makes a blocker or she makes an attacker, and the ultimate ends
    // the game against anything that answers permanents.
    CardRules::new_planeswalker(mana_cost!("{2}{W}{W}"), &["Elspeth"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "+1: Create a 1/1 white Soldier creature token.",
                &[AbilityCostDef::Loyalty(1)],
                EffectDef::CreateToken {
                    token: tokens::creature(&["Soldier"], &[ManaColor::White], 1, 1),
                    copy: None,
                    controller: None,
                    count: ValueDef::Constant(1),
                    tapped: false,
                    attacking: false,
                    counters: None,
                    created: None,
                },
            ),
            // The second plus is what makes her a threat rather than a hedge: any
            // creature, so the token she made last turn is a 4/4 flier this one.
            AbilityDef::activated_with_targets(
                "+1: Target creature gets +3/+3 and gains flying until end of turn.",
                &[AbilityCostDef::Loyalty(1)],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(ValueDef::Constant(3), ValueDef::Constant(3)),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::activated(
                "\u{2212}8: You get an emblem with \"Artifacts, creatures, enchantments, and lands you \
                 control have indestructible.\"",
                &[AbilityCostDef::Loyalty(-8)],
                EffectDef::create_emblem("Elspeth, Knight-Errant emblem", &[AbilityDef::static_ability(
                    "Artifacts, creatures, enchantments, and lands you control have indestructible.",
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::matching_objects(
                            // The four types the emblem names, which between them are every permanent
                            // a white deck is likely to control. Written as one alternation because the
                            // emblem grants one thing to all of them.
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                ObjectPredicateDef::HasType(CardType::Land),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                    },
                )]),
            ),
        ]),
);

// ALA 104 — Hissing Iguanar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HISSING_IGUANAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4b8b8b90-cb6e-4910-bc40-d96b78b0d70c"),
    "Hissing Iguanar",
    crate::card::CardArt::new("4b8b8b90-cb6e-4910-bc40-d96b78b0d70c", "Brandon Kitkouski"),
    crate::card::CardSet::ShardsOfAlara,
    crate::card::CardRules::unsupported(),
);

// ALA 156 — Blightning
pub(in crate::card::sets) static BLIGHTNING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3c05e8a2-b7d0-4f24-b2ae-8e4db30e5842"),
    "Blightning",
    CardArt::new("3c05e8a2-b7d0-4f24-b2ae-8e4db30e5842", "Thomas M. Baxa"),
    CardSet::ShardsOfAlara,
    // Three damage and two cards for three mana, which is why it was the
    // aggressive deck's answer to a control opponent rather than to a board.
    CardRules::new_sorcery(mana_cost!("{1}{B}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Blightning deals 3 damage to target player or planeswalker. That player or that planeswalker's controller discards two cards.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
            // A target naming a player resolves to that player, so one
            // recipient says both halves of "that player or that
            // planeswalker's controller".
            EffectDef::Discard {
                recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ]),
    )),
);

// ALA 158 — Branching Bolt
pub(in crate::card::sets) static BRANCHING_BOLT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e7468876-f401-4a75-81c0-bed09cdda3e1"),
    "Branching Bolt",
    CardArt::new("e7468876-f401-4a75-81c0-bed09cdda3e1", "Vance Kovacs"),
    CardSet::ShardsOfAlara,
    // Three mana for three damage is a poor rate until both modes are live,
    // which is the whole design: it is a two-for-one or it is overpriced.
    CardRules::new_instant(mana_cost!("{1}{R}{G}")).with_ability(
        AbilityDef::modal_spell(
            "Choose one or both —",
            &[
                AbilityDef::spell_with_targets(
                    "Branching Bolt deals 3 damage to target creature with flying.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                        ]),
                    )],
                    EffectDef::DealDamage {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(3),
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Branching Bolt deals 3 damage to target creature without flying.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                                KeywordAbility::Flying,
                            )),
                        ]),
                    )],
                    EffectDef::DealDamage {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(3),
                    },
                ),
            ],
        )
        // Both modes may be chosen, and each carries its own target, so the
        // two halves never land on the same creature.
        .with_mode_selection(1, 2, false),
    ),
);

// ALA 202 — Tidehollow Sculler
pub(in crate::card::sets) static TIDEHOLLOW_SCULLER: CardRecord = CardRecord::new_with_legacy_id(
    2145,
    "Tidehollow Sculler",
    CardArt::new("1abecc77-07f2-43e4-8585-0a8199cdcf01", "rk post"),
    CardSet::ShardsOfAlara,
    CardRules::new_artifact_creature(mana_cost!("{W}{B}"), &["Zombie"], 2, 2)
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When this creature enters, target opponent reveals their hand and you choose a nonland card from it. Exile that card.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::Sequence(&abilities::reveal_hand_and_choose_card(
                    PlayerRefDef::Target(TargetIndex::PRIMARY),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    // Linked to the Sculler rather than exiled outright, which is the whole
                    // bargain: the card is gone only for as long as the body survives.
                    &EffectDef::ExileLinkedToSource {
                        until_source_leaves: false,
                        object: EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
                        face_down: false,
                        then: None,
                    },
                )),
            ),
            // Leaves, not dies: bouncing or exiling the Sculler gives the card back
            // just as killing it does.
            AbilityDef::triggered(
                "When this creature leaves the battlefield, return the exiled card to its owner's hand.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    None,
                ),
                EffectDef::ReturnLinkedExiles {
                    object: ObjectPredicateDef::Any,
                    counters: None,
                    zone: ZoneKind::Hand,
                    grant: None,
                    controller: None,
                    transformed: false,
                },
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ELSPETH_KNIGHT_ERRANT,
    &HISSING_IGUANAR,
    &BLIGHTNING,
    &BRANCHING_BOLT,
    &TIDEHOLLOW_SCULLER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
