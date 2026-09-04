//! Rise of the Eldrazi cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedRuleDef, CardArt, CardRules, CardSet, CardSupertype, CardType, EffectDef,
    EffectRecipientDef, KeywordAbility, ManaColor, ObjectPredicateDef, ObjectQueryDef,
    ObjectRefDef, ObjectSetDef, PlayerRefDef, PlayerRelation, PlayerSetDef, TriggerEventDef,
    ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// ROE 4 — Emrakul, the Aeons Torn
pub(in crate::card::sets) static EMRAKUL_THE_AEONS_TORN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("67600383-bbb8-411c-b8e6-2296650bc747"),
    "Emrakul, the Aeons Torn",
    CardArt::new("67600383-bbb8-411c-b8e6-2296650bc747", "Mark Tedin"),
    CardSet::RiseOfTheEldrazi,
    CardRules::new_creature(mana_cost!("{15}"), &["Eldrazi"], 15, 15)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::cannot_be_countered(),
            AbilityDef::triggered(
                "When you cast this spell, take an extra turn after this one.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::Source),
                EffectDef::TakeExtraTurn {
                    player: EffectRecipientDef::Controller,
                },
            ),
            abilities::flying(),
            AbilityDef::keyword(
                "Protection from spells that are one or more colors",
                KeywordAbility::ProtectionFrom(&ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Spell,
                    ObjectPredicateDef::Not(&ObjectPredicateDef::ColorCount(0)),
                ])),
            ),
            abilities::annihilator(6),
            AbilityDef::triggered(
                "When Emrakul is put into a graveyard from anywhere, its owner shuffles their graveyard into their library.",
                TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Graveyard)),
                EffectDef::Sequence(&[
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::owned_by(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Graveyard],
                            PlayerSetDef::One(PlayerRefDef::OwnerOf(ObjectRefDef::Source)),
                        ))),
                        zone: ZoneKind::Library,
                        placement: ZonePlacement::Top,
                    },
                    EffectDef::ShuffleLibrary {
                        player: EffectRecipientDef::player(PlayerRefDef::OwnerOf(ObjectRefDef::Source)),
                    },
                ]),
            )
            .with_source_zones(&[ZoneKind::Graveyard]),
        ]),
);

// ROE 13 — Ulamog's Crusher
pub(in crate::card::sets) static ULAMOG_S_CRUSHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("76bacedb-9fa8-4a21-b0eb-e7ead64360b4"),
    "Ulamog's Crusher",
    crate::card::CardArt::new("76bacedb-9fa8-4a21-b0eb-e7ead64360b4", "Todd Lockwood"),
    crate::card::CardSet::RiseOfTheEldrazi,
    CardRules::new_creature(mana_cost!("{8}"), &["Eldrazi"], 8, 8).with_abilities(&[
        abilities::annihilator(2),
        abilities::attacks_each_combat_if_able(),
    ]),
);

// ROE 40 — Oust
pub(in crate::card::sets) static OUST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07313dd3-d0dc-40ca-98a3-fa4d39e5bcae"),
    "Oust",
    crate::card::CardArt::new("07313dd3-d0dc-40ca-98a3-fa4d39e5bcae", "Mike Bierek"),
    crate::card::CardSet::RiseOfTheEldrazi,
    // One white mana answers anything, and pays for it with three life and a
    // card the other player draws again in two turns.
    CardRules::new_sorcery(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Put target creature into its owner's library second from the top. Its controller gains \
         3 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        // One card down is the whole card: the creature is gone, and its owner's
        // next draw is the card that was already on top rather than the thing that
        // just left. Second from the top is beneath the top one.
        EffectDef::Sequence(&[
            EffectDef::PutIntoLibraryBeneathTop {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                depth: ValueDef::Constant(1),
            },
            // "Its controller", read after the creature has left: the player who
            // controlled it is the one paid for losing it, whoever owns the card.
            EffectDef::GainLife {
                recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                    ObjectRefDef::Target(TargetIndex::PRIMARY),
                )),
                amount: ValueDef::Constant(3),
            },
        ]),
    )),
);

// ROE 61 — Domestication
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOMESTICATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e1f15831-8dfd-4232-875c-efa6744c9a12"),
    "Domestication",
    crate::card::CardArt::new("e1f15831-8dfd-4232-875c-efa6744c9a12", "Jesper Ejsing"),
    crate::card::CardSet::RiseOfTheEldrazi,
    crate::card::CardRules::unsupported(),
);

// ROE 115 — Inquisition of Kozilek
/// A choice of one with nothing on offer simply does not ask: a hand with
/// nothing cheap enough in it loses nothing.
pub(in crate::card::sets) static INQUISITION_OF_KOZILEK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6a3ff5c3-0fdb-4d54-b4e5-ce7bad9953f0"),
    "Inquisition of Kozilek",
    CardArt::new("6a3ff5c3-0fdb-4d54-b4e5-ce7bad9953f0", "Tomasz Jedruszek"),
    CardSet::RiseOfTheEldrazi,
    // One mana and no life, for everything the format actually casts on the
    // first three turns.
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player reveals their hand. You choose a nonland card from it with mana value 3 \
         or less. That player discards that card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&abilities::reveal_hand_and_discard_chosen_card(
            PlayerRefDef::Target(TargetIndex::PRIMARY),
            // The bound is the whole difference from Thoughtseize: the expensive half
            // of their hand is safe, and what it costs you instead of two life is that
            // the card you wanted may not be a legal choice at all.
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                ObjectPredicateDef::ManaValueAtMost(3),
            ]),
        )),
    )),
);

// ROE 130 — Vendetta
pub(in crate::card::sets) static VENDETTA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("67ced38e-0f33-4bda-8e18-09f6ac03a3d7"),
    "Vendetta",
    CardArt::new("039fc76d-3b7e-4329-a997-07c25509e421", "Karl Kopinski"),
    CardSet::RiseOfTheEldrazi,
    // One mana kills almost anything; the life is what makes killing the big
    // thing a real decision rather than a free one.
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target nonblack creature. It can't be regenerated. You lose life equal to that creature's toughness.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::WithRule {
                rule: AppliedRuleDef::CannotRegenerate,
                effect: &EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            },
            // Read off the target rather than off the board, so the
            // toughness charged is the one it had as it died.
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::TargetToughness(TargetIndex::PRIMARY),
            },
        ]),
    )),
);

// ROE 145 — Flame Slash
pub(in crate::card::sets) static FLAME_SLASH: CardRecord = CardRecord::new_with_legacy_id(
    2184,
    "Flame Slash",
    CardArt::new("006d2bf1-20f7-4b09-8d98-8233d91682bd", "Raymond Swanland"),
    CardSet::RiseOfTheEldrazi,
    // One mana for four damage is the best rate in the format; the sorcery
    // speed is the whole price, and it cannot go upstairs.
    CardRules::new_sorcery(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Flame Slash deals 4 damage to target creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(4),
        },
    )),
);

// ROE 161 — Raid Bombardment
pub(in crate::card::sets) static RAID_BOMBARDMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c2d1a48-efde-4134-95f0-b23f6cf85259"),
    "Raid Bombardment",
    CardArt::new("9c2d1a48-efde-4134-95f0-b23f6cf85259", "Matt Cavotta"),
    CardSet::RiseOfTheEldrazi,
    // The power cap is the deckbuilding cost: this pays a token deck and
    // nothing else, and it turns chump attackers into reach.
    CardRules::new_enchantment(mana_cost!("{2}{R}")).with_ability(AbilityDef::triggered(
        "Whenever a creature you control with power 2 or less attacks, this enchantment deals 1 \
         damage to the player or planeswalker that creature is attacking.",
        TriggerEventDef::attacks(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
        ])),
        EffectDef::DealDamage {
            // Read off the attacker rather than off this enchantment, which
            // is not in combat and defends nothing.
            recipient: EffectRecipientDef::DefenderOfTriggeringObject,
            amount: ValueDef::Constant(1),
        },
    )),
);

// ROE 201 — Nest Invader
pub(in crate::card::sets) static NEST_INVADER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("24517d9c-6cde-41e8-9e82-ee73f069379a"),
    "Nest Invader",
    CardArt::new("24517d9c-6cde-41e8-9e82-ee73f069379a", "Trevor Claxton"),
    CardSet::RiseOfTheEldrazi,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Eldrazi", "Drone"], 2, 2).with_ability(
        abilities::enters_trigger("When this creature enters, create a 0/1 colorless Eldrazi Spawn creature token. It has \"Sacrifice this token: Add {C}.\"", EffectDef::create_creature_token(&["Eldrazi", "Spawn"], &[], 0, 1)
                .with_abilities(&[AbilityDef::activated_mana(
                    "Sacrifice this creature: Add {C}.",
                    &[AbilityCostDef::SacrificeSource],
                    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
                )])
                .with_art(CardArt::new(
                    "d0da4f8d-cce9-4d08-8d11-792e0b2af7d0",
                    "Véronique Meignaud",
                ))),
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &EMRAKUL_THE_AEONS_TORN,
    &ULAMOG_S_CRUSHER,
    &OUST,
    &DOMESTICATION,
    &INQUISITION_OF_KOZILEK,
    &VENDETTA,
    &FLAME_SLASH,
    &RAID_BOMBARDMENT,
    &NEST_INVADER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
