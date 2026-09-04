//! Rise of the Eldrazi cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::AppliedEffectDef;
use crate::AppliedRuleDef;
use crate::BasicLandType;
use crate::ControlDurationDef;
use crate::ResolvedEffectDurationDef;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    CardArt, CardRules, CardSet, CardSupertype, CardType, EffectDef, EffectRecipientDef,
    KeywordAbility, ManaColor, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::{TargetIndex, mana_cost};

// ROE 4 — Emrakul, the Aeons Torn
pub(in crate::card::sets) static EMRAKUL_THE_AEONS_TORN: CardRecord = CardRecord::new(
    CardSet::RiseOfTheEldrazi,
    "Emrakul, the Aeons Torn",
    "67600383-bbb8-411c-b8e6-2296650bc747",
    "Mark Tedin",
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
    crate::card::CardSet::RiseOfTheEldrazi,
    "Ulamog's Crusher",
    "76bacedb-9fa8-4a21-b0eb-e7ead64360b4",
    "Todd Lockwood",
    CardRules::new_creature(mana_cost!("{8}"), &["Eldrazi"], 8, 8).with_abilities(&[
        abilities::annihilator(2),
        abilities::attacks_each_combat_if_able("This creature attacks each combat if able."),
    ]),
);

// ROE 21 — Gideon Jura
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIDEON_JURA: CardRecord = CardRecord::new(
    crate::card::CardSet::RiseOfTheEldrazi,
    "Gideon Jura",
    "e0440668-1b0e-437c-9e42-7166dd14dfe5",
    "Aleksi Briclot",
    crate::card::CardRules::unsupported(),
);

// ROE 40 — Oust
pub(in crate::card::sets) static OUST: CardRecord = CardRecord::new(
    crate::card::CardSet::RiseOfTheEldrazi,
    "Oust",
    "07313dd3-d0dc-40ca-98a3-fa4d39e5bcae",
    "Mike Bierek",
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
    crate::card::CardSet::RiseOfTheEldrazi,
    "Domestication",
    "e1f15831-8dfd-4232-875c-efa6744c9a12",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// ROE 67 — Fleeting Distraction
pub(in crate::card::sets) static FLEETING_DISTRACTION: CardRecord = CardRecord::new(
    CardSet::RiseOfTheEldrazi,
    "Fleeting Distraction",
    "ed843c4d-28b5-4a4c-8bae-8f03f329bf2b",
    "Kieran Yanner",
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets -1/-0 until end of turn. Draw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// ROE 98 — Bloodthrone Vampire
pub(in crate::card::sets) static BLOODTHRONE_VAMPIRE: CardRecord = CardRecord::new(
    CardSet::RiseOfTheEldrazi,
    "Bloodthrone Vampire",
    "48bf0233-1d2e-40cb-9a69-8eeeeb2959ca",
    "Steve Argyle",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire"], 1, 1).with_ability(
        AbilityDef::activated(
            "Sacrifice a creature: This creature gets +2/+2 until end of turn.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
            }],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ROE 102 — Contaminated Ground
pub(in crate::card::sets) static CONTAMINATED_GROUND: CardRecord = CardRecord::new(
    CardSet::RiseOfTheEldrazi,
    "Contaminated Ground",
    "3d2ba8f3-58f5-43e5-9201-974ba58f56f8",
    "Rob Alexander",
    CardRules::new_enchantment(mana_cost!("{1}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant land",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Land),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted land is a Swamp.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::set_basic_land_types(&[BasicLandType::Swamp]),
                },
            ),
            AbilityDef::static_ability(
                "Whenever enchanted land becomes tapped, its controller loses 2 life.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::triggered(
                        "Whenever enchanted land becomes tapped, its controller loses 2 life.",
                        TriggerEventDef::tapped(ObjectPredicateDef::Source),
                        EffectDef::LoseLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(2),
                        },
                    )),
                },
            ),
        ]),
);

// ROE 115 — Inquisition of Kozilek
/// A choice of one with nothing on offer simply does not ask: a hand with
/// nothing cheap enough in it loses nothing.
pub(in crate::card::sets) static INQUISITION_OF_KOZILEK: CardRecord = CardRecord::new(
    CardSet::RiseOfTheEldrazi,
    "Inquisition of Kozilek",
    "6a3ff5c3-0fdb-4d54-b4e5-ce7bad9953f0",
    "Tomasz Jedruszek",
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

// ROE 126 — Shrivel
pub(in crate::card::sets) static SHRIVEL: CardRecord = CardRecord::new(
    CardSet::RiseOfTheEldrazi,
    "Shrivel",
    "a87c80a1-5818-45fd-9a37-a2ee3396626e",
    "Jung Park",
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell(
        "All creatures get -1/-1 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-1),
                ValueDef::Constant(-1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ROE 130 — Vendetta (reprint)
const VENDETTA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1999::mercadian_masques::VENDETTA,
    "039fc76d-3b7e-4329-a997-07c25509e421",
    "Karl Kopinski",
);

// ROE 145 — Flame Slash
pub(in crate::card::sets) static FLAME_SLASH: CardRecord = CardRecord::new(
    CardSet::RiseOfTheEldrazi,
    "Flame Slash",
    "006d2bf1-20f7-4b09-8d98-8233d91682bd",
    "Raymond Swanland",
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

// ROE 147 — Goblin Arsonist
pub(in crate::card::sets) static GOBLIN_ARSONIST: CardRecord = CardRecord::new(
    CardSet::RiseOfTheEldrazi,
    "Goblin Arsonist",
    "707d396d-950b-4ab8-9db2-f40c8f7db062",
    "Wayne Reynolds",
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Shaman"], 1, 1).with_ability(
        abilities::dies_trigger_with_targets(
            "When this creature dies, you may have it deal 1 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            },
        ),
    ),
);

// ROE 148 — Goblin Tunneler
pub(in crate::card::sets) static GOBLIN_TUNNELER: CardRecord = CardRecord::new(
    crate::card::CardSet::RiseOfTheEldrazi,
    "Goblin Tunneler",
    "0b2e4a34-6255-4f89-a62d-941996c573e1",
    "Jesper Ejsing",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Rogue"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target creature with power 2 or less can't be blocked this turn.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::PowerAtLeast(3)),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Any,
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ROE 161 — Raid Bombardment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAID_BOMBARDMENT: CardRecord = CardRecord::new(
    crate::card::CardSet::RiseOfTheEldrazi,
    "Raid Bombardment",
    "9c2d1a48-efde-4134-95f0-b23f6cf85259",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// ROE 168 — Traitorous Instinct
pub(in crate::card::sets) static TRAITOROUS_INSTINCT: CardRecord = CardRecord::new(
    CardSet::RiseOfTheEldrazi,
    "Traitorous Instinct",
    "d65b63ea-e3c3-465d-8cd9-7251cda9cc63",
    "Scott Chou",
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Gain control of target creature until end of turn. Untap that creature. Until end of turn, it gets +2/+0 and gains haste.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::GainControl {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    duration: ControlDurationDef::UntilEndOfTurn,
                    controller: PlayerRefDef::EffectController,
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(0)),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ),
);

// ROE 201 — Nest Invader
pub(in crate::card::sets) static NEST_INVADER: CardRecord = CardRecord::new(
    CardSet::RiseOfTheEldrazi,
    "Nest Invader",
    "24517d9c-6cde-41e8-9e82-ee73f069379a",
    "Trevor Claxton",
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

// ROE 222 — Prophetic Prism
pub(in crate::card::sets) static PROPHETIC_PRISM: CardRecord = CardRecord::new(
    CardSet::RiseOfTheEldrazi,
    "Prophetic Prism",
    "cfb90d44-8cb1-4b83-b2f2-92c19d6304fb",
    "John Avon",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        abilities::enters_trigger(
            "When this artifact enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// ROE 228 — Evolving Wilds
pub(in crate::card::sets) static EVOLVING_WILDS: CardRecord = CardRecord::new(
    CardSet::RiseOfTheEldrazi,
    "Evolving Wilds",
    "bc7e0407-fea1-43ef-8580-82271e440bb3",
    "Steven Belledin",
    CardRules::new_land(&[]).with_ability(AbilityDef::activated(
        "{T}, Sacrifice this land: Search your library for a basic land card, put it onto the \
         battlefield tapped, then shuffle.",
        &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
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
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &EMRAKUL_THE_AEONS_TORN,
    &ULAMOG_S_CRUSHER,
    &GIDEON_JURA,
    &OUST,
    &DOMESTICATION,
    &FLEETING_DISTRACTION,
    &BLOODTHRONE_VAMPIRE,
    &CONTAMINATED_GROUND,
    &INQUISITION_OF_KOZILEK,
    &SHRIVEL,
    &FLAME_SLASH,
    &GOBLIN_ARSONIST,
    &GOBLIN_TUNNELER,
    &RAID_BOMBARDMENT,
    &TRAITOROUS_INSTINCT,
    &NEST_INVADER,
    &PROPHETIC_PRISM,
    &EVOLVING_WILDS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[VENDETTA_REPRINT];
