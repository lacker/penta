//! Rise of the Eldrazi cards cataloged for the Vintage Cube pool.

use crate::card::AbilityKindDef;
use crate::card::AbilityPredicateDef;
use crate::card::CostModificationDef;
use super::CardRecord;
use super::PrintingRecord;
use crate::AppliedEffectDef;
use crate::BasicLandType;
use crate::ControlDurationDef;
use crate::ResolvedEffectDurationDef;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "ROE",
    slug: "rise-of-the-eldrazi",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// ROE 1 — All Is Dust
pub(in crate::card::sets) static ALL_IS_DUST_1: CardRecord = CardRecord::new(
    "All Is Dust",
    "62dba377-7446-4517-a504-ee04568fd6cf",
    "Jason Felix",
    CardRules::new_sorcery(mana_cost!("{7}"))
        .with_subtypes(&["Eldrazi"])
        .with_abilities(&[AbilityDef::spell(
            "Each player sacrifices all permanents they control that are one or more colors.",
            EffectDef::sacrifice(EffectRecipientDef::matching_objects(
                ObjectPredicateDef::Not(&ObjectPredicateDef::ColorCount(0)),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            )),
        )])
        .with_type(crate::card::CardType::Kindred),
);

// ROE 4 — Emrakul, the Aeons Torn
pub(in crate::card::sets) static EMRAKUL_THE_AEONS_TORN: CardRecord = CardRecord::new(
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
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::owned_by(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Graveyard],
                            PlayerSetDef::One(PlayerRefDef::OwnerOf(ObjectRefDef::Source)),
                        ))),
                        ZoneKind::Library,
                        ZonePlacement::Top,
                    ),
                    EffectDef::ShuffleLibrary {
                        player: EffectRecipientDef::player(PlayerRefDef::OwnerOf(ObjectRefDef::Source)),
                    },
                ]),
            )
            .with_source_zones(&[ZoneKind::Graveyard]),
        ]),
);

// ROE 6 — Kozilek, Butcher of Truth
pub(in crate::card::sets) static KOZILEK_BUTCHER_OF_TRUTH_6: CardRecord = CardRecord::new(
    "Kozilek, Butcher of Truth",
    "067fac91-2483-4678-b86a-2c54a3a480cf",
    "Michael Komarck",
    CardRules::new_creature(mana_cost!("{10}"), &["Eldrazi"], 12, 12).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::triggered("When you cast this spell, draw four cards.", TriggerEventDef::spell_cast(ObjectPredicateDef::Source), abilities::draw_cards(ValueDef::Constant(4))),
abilities::annihilator(4),
AbilityDef::triggered("When Kozilek is put into a graveyard from anywhere, its owner shuffles their graveyard into their library.", TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Graveyard)), EffectDef::Sequence(&[EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::owned_by(ObjectPredicateDef::Any, &[ZoneKind::Graveyard], PlayerSetDef::One(PlayerRefDef::OwnerOf(ObjectRefDef::Source))))), ZoneKind::Library, ZonePlacement::Top), EffectDef::ShuffleLibrary { player: EffectRecipientDef::player(PlayerRefDef::OwnerOf(ObjectRefDef::Source)) }])).with_source_zones(&[ZoneKind::Graveyard])
]),
);

// ROE 8 — Not of This World
// Audit: unsupported — The spell cost evaluator cannot condition a source-card discount on the already chosen target's own creature targets. TargetsObjectMatching supports the counter target restriction but is not a supported cast-context value for the seven-mana discount.
pub(in crate::card::sets) static NOT_OF_THIS_WORLD_8: CardRecord = CardRecord::new(
    "Not of This World",
    "569e2c39-7a49-4a3b-afe5-1862a7da8026",
    "Izzy",
    crate::card::CardRules::unsupported(),
);

// ROE 13 — Ulamog's Crusher
pub(in crate::card::sets) static ULAMOG_S_CRUSHER: CardRecord = CardRecord::new(
    "Ulamog's Crusher",
    "76bacedb-9fa8-4a21-b0eb-e7ead64360b4",
    "Todd Lockwood",
    CardRules::new_creature(mana_cost!("{8}"), &["Eldrazi"], 8, 8).with_abilities(&[
        abilities::annihilator(2),
        abilities::attacks_each_combat_if_able(),
    ]),
);

// ROE 21 — Gideon Jura
// Audit: unsupported — Needs forced attacks against a chosen planeswalker controller and a turn-long planeswalker animation with damage prevention.
pub(in crate::card::sets) static GIDEON_JURA: CardRecord = CardRecord::new(
    "Gideon Jura",
    "e0440668-1b0e-437c-9e42-7166dd14dfe5",
    "Aleksi Briclot",
    crate::card::CardRules::unsupported(),
);

// ROE 33 — Linvala, Keeper of Silence
pub(in crate::card::sets) static LINVALA_KEEPER_OF_SILENCE_33: CardRecord = CardRecord::new(
    "Linvala, Keeper of Silence",
    "82b80a09-7e75-4091-a60e-04aff79339a3",
    "Igor Kieryluk",
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Angel"], 3, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::static_ability(
                "Activated abilities of creatures your opponents control can't be activated.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    ),
                    effect: AppliedEffectDef::cannot_activate_abilities(AbilityPredicateDef::Is(
                        AbilityKindDef::Activated,
                    )),
                },
            ),
        ]),
);

// ROE 40 — Oust
pub(in crate::card::sets) static OUST: CardRecord = CardRecord::new(
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
    "Domestication",
    "e1f15831-8dfd-4232-875c-efa6744c9a12",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// ROE 67 — Fleeting Distraction
pub(in crate::card::sets) static FLEETING_DISTRACTION: CardRecord = CardRecord::new(
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

// ROE 91 — Training Grounds
pub(in crate::card::sets) static TRAINING_GROUNDS_91: CardRecord = CardRecord::new(
    "Training Grounds",
    "e2cf16f8-6e69-46b3-8453-1d1a2a5670e2",
    "James Ryman",
    CardRules::new_enchantment(mana_cost!("{U}")).with_abilities(&[
AbilityDef::static_ability("Activated abilities of creatures you control cost {2} less to activate. This effect can't reduce the mana in that cost to less than one mana.", EffectDef::ModifyCost(CostModificationDef::AbilityReduction { permanent: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ControlledBy(PlayerRelation::You)]), amount: ValueDef::Constant(2), minimum: 1 }))
]),
);

// ROE 98 — Bloodthrone Vampire
pub(in crate::card::sets) static BLOODTHRONE_VAMPIRE: CardRecord = CardRecord::new(
    "Bloodthrone Vampire",
    "48bf0233-1d2e-40cb-9a69-8eeeeb2959ca",
    "Steve Argyle",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire"], 1, 1).with_ability(
        AbilityDef::activated(
            "Sacrifice a creature: This creature gets +2/+2 until end of turn.",
            &[CostDef::SacrificePermanent {
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

// ROE 136 — Battle-Rattle Shaman
pub(in crate::card::sets) static BATTLE_RATTLE_SHAMAN: CardRecord = CardRecord::new(
    "Battle-Rattle Shaman",
    "aa1df08a-ccef-44cf-936a-838e238c27c1",
    "Warren Mahy",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Goblin", "Shaman"], 2, 2).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "At the beginning of combat on your turn, you may have target \
             creature get +2/+0 until end of turn.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            },
        ),
    ]),
);

// ROE 145 — Flame Slash
pub(in crate::card::sets) static FLAME_SLASH: CardRecord = CardRecord::new(
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
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::Constant(4),
        ),
    )),
);

// ROE 147 — Goblin Arsonist
pub(in crate::card::sets) static GOBLIN_ARSONIST: CardRecord = CardRecord::new(
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
                effect: &EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(1),
                ),
            },
        ),
    ),
);

// ROE 148 — Goblin Tunneler
pub(in crate::card::sets) static GOBLIN_TUNNELER: CardRecord = CardRecord::new(
    "Goblin Tunneler",
    "0b2e4a34-6255-4f89-a62d-941996c573e1",
    "Jesper Ejsing",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Rogue"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target creature with power 2 or less can't be blocked this turn.",
            &[CostDef::TapSource],
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
pub(in crate::card::sets) static RAID_BOMBARDMENT: CardRecord = CardRecord::new(
    "Raid Bombardment",
    "9c2d1a48-efde-4134-95f0-b23f6cf85259",
    "Matt Cavotta",
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
        EffectDef::damage(
            // Read off the attacker rather than off this enchantment, which
            // is not in combat and defends nothing.
            EffectRecipientDef::DefenderOfTriggeringObject,
            ValueDef::Constant(1),
        ),
    )),
);

// ROE 168 — Traitorous Instinct
pub(in crate::card::sets) static TRAITOROUS_INSTINCT: CardRecord = CardRecord::new(
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
                EffectDef::gain_control(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    PlayerRefDef::EffectController,
                    ControlDurationDef::UntilEndOfTurn,
                ),
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

// ROE 172 — World at War
// Audit: unsupported — Phase scheduling inserts phases after the current phase; it cannot schedule after this turn’s second main phase with an untap trigger attached to that particular combat.
pub(in crate::card::sets) static WORLD_AT_WAR_172: CardRecord = CardRecord::new(
    "World at War",
    "a47a05ad-fe86-481e-b770-e1760be4f852",
    "Igor Kieryluk",
    crate::card::CardRules::unsupported(),
);

// ROE 201 — Nest Invader
pub(in crate::card::sets) static NEST_INVADER: CardRecord = CardRecord::new(
    "Nest Invader",
    "24517d9c-6cde-41e8-9e82-ee73f069379a",
    "Trevor Claxton",
CardRules::new_creature(mana_cost!("{1}{G}"), &["Eldrazi", "Drone"], 2, 2).with_ability(
        abilities::enters_trigger("When this creature enters, create a 0/1 colorless Eldrazi Spawn creature token. It has \"Sacrifice this token: Add {C}.\"", EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
            TokenCharacteristics::creature(&["Eldrazi", "Spawn"], &[], 0, 1)
                .with_abilities(&[AbilityDef::activated_mana(
                    "Sacrifice this creature: Add {C}.",
                    &[CostDef::SacrificeSource],
                    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
                )])
                .with_art(CardArt::new(
                    "d0da4f8d-cce9-4d08-8d11-792e0b2af7d0",
                    "Véronique Meignaud",
                )),
        )))),
    ),
);

// ROE 204 — Pelakka Wurm
pub(in crate::card::sets) static PELAKKA_WURM: CardRecord = CardRecord::new(
    "Pelakka Wurm",
    "8e732593-0bdc-4dd4-9b07-9aa1a780e6e8",
    "Daniel Ljunggren",
    CardRules::new_creature(mana_cost!("{4}{G}{G}{G}"), &["Wurm"], 7, 7).with_abilities(&[
        abilities::trample(),
        abilities::enters_trigger(
            "When this creature enters, you gain 7 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(7),
            },
        ),
        abilities::dies_trigger(
            "When this creature dies, draw a card.",
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// ROE 213 — Wildheart Invoker
pub(in crate::card::sets) static WILDHEART_INVOKER: CardRecord = CardRecord::new(
    "Wildheart Invoker",
    "dc8315bf-03af-4f19-92c7-556e486cb099",
    "Erica Yang",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Elf", "Shaman"], 4, 3).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{8}: Target creature gets +5/+5 and gains trample until end \
             of turn. (It can deal excess combat damage to the player or \
             planeswalker it's attacking.)",
            &[CostDef::Mana(mana_cost!("{8}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(5),
                        ValueDef::Constant(5),
                    ),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ROE 222 — Prophetic Prism
pub(in crate::card::sets) static PROPHETIC_PRISM: CardRecord = CardRecord::new(
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
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// ROE 227 — Eldrazi Temple
// Audit: unsupported — Mana restrictions are conjunctive; they cannot allow either casting a colorless Eldrazi or activating a colorless Eldrazi ability.
pub(in crate::card::sets) static ELDRAZI_TEMPLE_227: CardRecord = CardRecord::new(
    "Eldrazi Temple",
    "315924c9-77e3-405b-9bbf-852ed563c6e3",
    "James Paick",
    crate::card::CardRules::unsupported(),
);

// ROE 228 — Evolving Wilds
pub(in crate::card::sets) static EVOLVING_WILDS: CardRecord = CardRecord::new(
    "Evolving Wilds",
    "bc7e0407-fea1-43ef-8580-82271e440bb3",
    "Steven Belledin",
    CardRules::new_land(&[]).with_ability(AbilityDef::activated(
        "{T}, Sacrifice this land: Search your library for a basic land card, put it onto the \
         battlefield tapped, then shuffle.",
        &[CostDef::TapSource, CostDef::SacrificeSource],
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
    &ALL_IS_DUST_1,
    &EMRAKUL_THE_AEONS_TORN,
    &KOZILEK_BUTCHER_OF_TRUTH_6,
    &NOT_OF_THIS_WORLD_8,
    &ULAMOG_S_CRUSHER,
    &GIDEON_JURA,
    &LINVALA_KEEPER_OF_SILENCE_33,
    &OUST,
    &DOMESTICATION,
    &FLEETING_DISTRACTION,
    &TRAINING_GROUNDS_91,
    &BLOODTHRONE_VAMPIRE,
    &CONTAMINATED_GROUND,
    &INQUISITION_OF_KOZILEK,
    &SHRIVEL,
    &BATTLE_RATTLE_SHAMAN,
    &FLAME_SLASH,
    &GOBLIN_ARSONIST,
    &GOBLIN_TUNNELER,
    &RAID_BOMBARDMENT,
    &TRAITOROUS_INSTINCT,
    &WORLD_AT_WAR_172,
    &NEST_INVADER,
    &PELAKKA_WURM,
    &WILDHEART_INVOKER,
    &PROPHETIC_PRISM,
    &ELDRAZI_TEMPLE_227,
    &EVOLVING_WILDS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[VENDETTA_REPRINT];
