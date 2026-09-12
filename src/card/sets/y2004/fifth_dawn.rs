//! Fifth Dawn cards cataloged for the Vintage Cube.

use crate::card::ObjectQueryDef;
use super::CardRecord;
use super::PrintingRecord;
use crate::ResolvedEffectDurationDef;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreatureTypeSetDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::GraveyardPlayPermissionDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementEffectDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "5DN",
    slug: "fifth-dawn",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// 5DN 4 — Auriok Salvagers
pub(in crate::card::sets) static AURIOK_SALVAGERS_4: CardRecord = CardRecord::new(
    "Auriok Salvagers",
    "09c9cd1b-9260-4f98-ac7a-25bb5ae3e06d",
    "Randy Gallegos",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Soldier"], 2, 4).with_abilities(&[
AbilityDef::activated_with_targets("{1}{W}: Return target artifact card with mana value 1 or less from your graveyard to your hand.", &[CostDef::Mana(mana_cost!("{1}{W}"))], &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::ManaValueAtMost(1)]), zones: &[ZoneKind::Graveyard], controller: None, owner: Some(PlayerRelation::You) })], EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Hand, ZonePlacement::Top))
]),
);

// 5DN 27 — Condescend
pub(in crate::card::sets) static CONDESCEND: CardRecord = CardRecord::new(
    "Condescend",
    "e8303b80-e29a-46b8-90b0-c0cfe551b435",
    "Ron Spears",
    // The scry is what keeps this live once X is too small to counter
    // anything, which is why a tempo deck can cast it for one.
    CardRules::new_instant(mana_cost!("{X}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell unless its controller pays {X}. Scry 2. (Look at the top two cards \
         of your library, then put any number of them on the bottom and the rest on top in any \
         order.)",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            // The demand is the same X this was cast for, so paying more for
            // it raises what the other player has to find.
            abilities::counter_target_unless_paid(&[CostDef::GenericMana(ValueDef::ChosenX)]),
            // Scrying happens either way: the spell resolving through does
            // not stop the second half.
            abilities::scry(ValueDef::Constant(2)),
        ]),
    )),
);

// 5DN 36 — Serum Visions
pub(in crate::card::sets) static SERUM_VISIONS: CardRecord = CardRecord::new(
    "Serum Visions",
    "4bc61952-88ba-447a-835a-f1e9643fcd0d",
    "Ben Thompson",
    // The draw comes first and the scry second, which is the whole
    // difference from Preordain: this fixes the next two draws, not this one.
    CardRules::new_sorcery(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Draw a card. Scry 2. (Look at the top two cards of your library, then put any number of \
         them on the bottom and the rest on top in any order.)",
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
            abilities::scry(ValueDef::Constant(2)),
        ]),
    )),
);

// 5DN 39 — Trinket Mage
pub(in crate::card::sets) static TRINKET_MAGE: CardRecord = CardRecord::new(
    "Trinket Mage",
    "4c5a41ab-1840-4abb-a8bb-f0b1e7d1b450",
    "Mark A. Nelson",
CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 2, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, you may search your library for an artifact card with mana value 1 or less, reveal that card, put it into your hand, then shuffle.",
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::ManaValueAtMost(1),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
    ),
);

// 5DN 55 — Night's Whisper
pub(in crate::card::sets) static NIGHTS_WHISPER: CardRecord = CardRecord::new(
    "Night's Whisper",
    "61f0c6f6-b90d-4eb1-a5db-86e0a3997501",
    "David Martin",
    // Two mana and two life for two which is the rate every black
    // deck in the cube is happy to pay and no other colour is offered.
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell(
        "You draw two cards and lose 2 life.",
        // "You draw two cards and lose 2 life" is one sentence about you, so the
        // life is not a cost and nothing stops it: a player at 2 who casts this
        // draws the two cards and loses the game.
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )),
);

// 5DN 65 — Furnace Whelp
pub(in crate::card::sets) static FURNACE_WHELP: CardRecord = CardRecord::new(
    "Furnace Whelp",
    "a1726eba-c471-40bd-a487-40d910b75d64",
    "Matt Cavotta",
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Dragon"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// 5DN 75 — Mana Geyser
pub(in crate::card::sets) static MANA_GEYSER_75: CardRecord = CardRecord::new(
    "Mana Geyser",
    "3929662e-99d7-48e9-afac-1852af8be722",
    "Martina Pilcerova",
    CardRules::new_sorcery(mana_cost!("{3}{R}{R}")).with_abilities(&[AbilityDef::spell(
        "Add {R} for each tapped land your opponents control.",
        EffectDef::AddManaEqualTo {
            color: ManaColor::Red,
            amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Tapped,
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Opponent,
            )),
        },
    )]),
);

// 5DN 85 — Dawn's Reflection
pub(in crate::card::sets) static DAWNS_REFLECTION: CardRecord = CardRecord::new(
    "Dawn's Reflection",
    "131a124f-f11e-4ea1-a7b2-b94eea988d4e",
    "John Avon",
CardRules::new_enchantment(mana_cost!("{3}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_land(),
            AbilityDef::triggered_mana(
                "Whenever enchanted land is tapped for mana, its controller adds an additional two mana in any combination of colors.",
                TriggerEventDef::tapped_for_mana(ObjectPredicateDef::AttachedToSource),
                EffectDef::AddMana(
                    AddManaEffectDef::combination(&ManaColor::COLORS, 2)
                        .to_triggering_objects_controller(),
                ),
            ),
        ]),
);

// 5DN 86 — Eternal Witness
pub(in crate::card::sets) static ETERNAL_WITNESS: CardRecord = CardRecord::new(
    "Eternal Witness",
    "c7e10ca7-1e5d-4224-82cf-798a4d436d72",
    "Terese Nielsen",
    // A 2/1 body nobody plays it for. What it is worth is the card, and
    // every way of making it enter again is worth another one.
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Human", "Shaman"], 2, 1).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, you may return target card from your graveyard to your \
             hand.",
            // Your own graveyard, and any card in it: a land comes back as readily as
            // the spell that killed the Witness.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            // The target is chosen as the trigger goes on the stack; the "may" is
            // answered as it resolves. A Witness whose card was exiled in response
            // still asks, and taking it back is what the answer refuses.
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            },
        ),
    ),
);

// 5DN 104 — Avarice Totem
pub(in crate::card::sets) static AVARICE_TOTEM_104: CardRecord = CardRecord::new(
    "Avarice Totem",
    "53a5cfa8-4091-445c-8641-64402cca7d2d",
    "Ben Thompson",
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{5}: Exchange control of this artifact and target nonland permanent.",
            &[CostDef::Mana(mana_cost!("{5}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            )],
            EffectDef::ExchangeControl {
                first: EffectRecipientDef::Source,
                second: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                otherwise: None,
            },
        ),
    ]),
);

// 5DN 106 — Battered Golem
pub(in crate::card::sets) static BATTERED_GOLEM_106: CardRecord = CardRecord::new(
    "Battered Golem",
    "f69add35-c529-4b30-8e64-f09b8308432f",
    "Carl Critchlow",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Golem"], 3, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::triggered(
            "Whenever an artifact enters, you may untap this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::HasType(CardType::Artifact),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Untap {
                    object: EffectRecipientDef::Source,
                },
            },
        ),
    ]),
);

// 5DN 107 — Blasting Station
pub(in crate::card::sets) static BLASTING_STATION_107: CardRecord = CardRecord::new(
    "Blasting Station",
    "71e2f832-6601-4232-b250-fd1c88538fbd",
    "Stephen Tappin",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice a creature: This artifact deals 1 damage to any target.",
            &[
                CostDef::TapSource,
                CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(CardType::Creature)),
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
        AbilityDef::triggered(
            "Whenever a creature enters, you may untap this artifact.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::HasType(CardType::Creature),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Untap {
                    object: EffectRecipientDef::Source,
                },
            },
        ),
    ]),
);

// 5DN 110 — Clock of Omens
pub(in crate::card::sets) static CLOCK_OF_OMENS: CardRecord = CardRecord::new(
    "Clock of Omens",
    "0ffce71b-eb60-4649-a62b-a1b4acaa9d2d",
    "Alex Horley-Orlandelli",
    // Two artifacts tapped to untap one, which is only a gain when the one
    // being untapped is worth more than the two that paid for it.
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(AbilityDef::activated_with_targets(
        "Tap two untapped artifacts you control: Untap target artifact.",
        &[CostDef::TapPermanents {
            object: ObjectPredicateDef::HasType(CardType::Artifact),
            controller: PlayerRelation::You,
            count: 2,
        }],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Artifact),
        )],
        EffectDef::Untap {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )),
);

// 5DN 112 — Conjurer's Bauble
pub(in crate::card::sets) static CONJURER_S_BAUBLE_112: CardRecord = CardRecord::new(
    "Conjurer's Bauble",
    "2d32960e-d182-455f-8e74-eb11b10050da",
    "Darrell Riche",
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
AbilityDef::activated_with_targets("{T}, Sacrifice this artifact: Put up to one target card from your graveyard on the bottom of your library. Draw a card.", &[CostDef::TapSource, CostDef::SacrificeSource], &[AbilityTargetDef::up_to(AbilityTargetPredicate::Object { object: ObjectPredicateDef::Any, zones: &[ZoneKind::Graveyard], controller: None, owner: Some(PlayerRelation::You) }, 1)], EffectDef::Sequence(&[EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Library, ZonePlacement::Bottom), abilities::draw_cards(ValueDef::Constant(1))]))
]),
);

// 5DN 114 — Crucible of Worlds
pub(in crate::card::sets) static CRUCIBLE_OF_WORLDS: CardRecord = CardRecord::new(
    "Crucible of Worlds",
    "312a6058-de08-487d-95bd-b3c56807fdd6",
    "Ron Spencer",
    // One line, and it turns every fetchland, every Wasteland, and every
    // land anything made you discard back into a land drop.
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::static_ability(
        "You may play lands from your graveyard.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Controller,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromGraveyard(
                // A permission rather than a prohibition, in the same vocabulary: which
                // action it opens, and which cards it opens it for.
                GraveyardPlayPermissionDef::unlimited(PlayRestrictionDef::new(
                    PlayActionMatcherDef::PlayLand,
                    ObjectPredicateDef::HasType(CardType::Land),
                )),
            )),
        },
    )),
);

// 5DN 115 — Door to Nothingness
pub(in crate::card::sets) static DOOR_TO_NOTHINGNESS: CardRecord = CardRecord::new(
    "Door to Nothingness",
    "c92ffeae-6b51-4426-a080-b1b065b1290d",
    "Puddnhead",
CardRules::new_artifact(mana_cost!("{5}")).with_abilities(&[
        abilities::enters_tapped(CardType::Artifact),
        AbilityDef::activated_with_targets(
            "{W}{W}{U}{U}{B}{B}{R}{R}{G}{G}, {T}, Sacrifice this artifact: Target player loses the game.",
            &[
                CostDef::Mana(mana_cost!("{W}{W}{U}{U}{B}{B}{R}{R}{G}{G}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::LoseTheGame {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// 5DN 118 — Engineered Explosives
pub(in crate::card::sets) static ENGINEERED_EXPLOSIVES: CardRecord = CardRecord::new(
    "Engineered Explosives",
    "8492a272-e595-4f94-a6eb-08d29f211fd6",
    "Ron Spears",
CardRules::new_artifact(mana_cost!("{X}")).with_abilities(&[
            AbilityDef::as_enters(
                "Sunburst (This artifact enters with a charge counter on it for each color of mana spent to cast it.)",
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCountersValue {
                        kind: CounterKind::named("charge"),
                        amount: ValueDef::ColorsOfManaSpent,
                    },
                ),
            ),
            AbilityDef::activated(
                "{2}, Sacrifice this artifact: Destroy each nonland permanent with mana value equal to the number of charge counters on this artifact.",
                &[
                    CostDef::Mana(mana_cost!("{2}")),
                    CostDef::SacrificeSource,
                ],
                EffectDef::Destroy {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                            ObjectPredicateDef::ManaValueEqualTo(ValueDef::CountersOnSource(
                                CounterKind::named("charge"),
                            )),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    then: None,
                },
            ),
        ]),
);

// 5DN 127 — Grinding Station
pub(in crate::card::sets) static GRINDING_STATION_127: CardRecord = CardRecord::new(
    "Grinding Station",
    "df1df511-b52c-45cd-9503-ffce4271a802",
    "Greg Staples",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice an artifact: Target player mills three cards.",
            &[
                CostDef::TapSource,
                CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(CardType::Artifact)),
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        ),
        AbilityDef::triggered(
            "Whenever an artifact enters, you may untap this artifact.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::HasType(CardType::Artifact),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Untap {
                    object: EffectRecipientDef::Source,
                },
            },
        ),
    ]),
);

// 5DN 128 — Guardian Idol
pub(in crate::card::sets) static GUARDIAN_IDOL: CardRecord = CardRecord::new(
    "Guardian Idol",
    "a6a62a73-b7db-47ec-9b68-65dd7c1a06a5",
    "Edward P. Beard, Jr.",
    // A mana rock that stops being a dead draw late, which is what the two
    // mana of animation buys -- and entering tapped is what it costs.
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::as_enters(
            "This artifact enters tapped.",
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated(
            "{2}: This artifact becomes a 2/2 Golem artifact creature until end of turn.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                // It is already an artifact, so adding the type again is
                // harmless and keeps the clause reading as printed.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_card_types(
                        CardTypeSet::single(CardType::Creature).with(CardType::Artifact),
                    ),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Golem"])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// 5DN 134 — Krark-Clan Ironworks
pub(in crate::card::sets) static KRARK_CLAN_IRONWORKS_134: CardRecord = CardRecord::new(
    "Krark-Clan Ironworks",
    "c60174d6-1f9d-4870-b3db-34d6fcb3f6ab",
    "Greg Hildebrandt",
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[AbilityDef::activated_mana(
        "Sacrifice an artifact: Add {C}{C}.",
        &[CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(
            CardType::Artifact,
        ))],
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(2)),
    )]),
);

// 5DN 135 — Lantern of Insight
pub(in crate::card::sets) static LANTERN_OF_INSIGHT_135: CardRecord = CardRecord::new(
    "Lantern of Insight",
    "cb0e4c78-75fe-4692-b177-974b148f0614",
    "Greg Hildebrandt",
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::static_ability(
            "Players play with the top card of their libraries revealed.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::EachPlayer,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::PlaysWithTopOfLibraryRevealed),
            },
        ),
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this artifact: Target player shuffles.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::ShuffleLibrary {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// 5DN 142 — Paradise Mantle
pub(in crate::card::sets) static PARADISE_MANTLE_142: CardRecord = CardRecord::new(
    "Paradise Mantle",
    "1252e9e2-2dd5-4bd6-aa56-f0a0ba056a77",
    "Greg Hildebrandt",
    CardRules::new_artifact(mana_cost!("{0}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has \"{T}: Add one mana of any color.\".",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::object(
                        crate::card::ObjectRefDef::AttachedToSource,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::tap_for_mana(
                        "{T}: Add one mana of any color.",
                        AddManaEffectDef::any_color(),
                    )),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// 5DN 143 — Pentad Prism
pub(in crate::card::sets) static PENTAD_PRISM: CardRecord = CardRecord::new(
    "Pentad Prism",
    "672b9b16-daef-44e6-9a3a-cfd9f3c78bc7",
    "David Martin",
// Two mana of two colours for two mana of any colours, later: a ritual
    // that waits, which is why it wants a deck already casting things in
    // more than one colour on turn two.
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
            AbilityDef::as_enters(
                "Sunburst (This artifact enters with a charge counter on it for each color of mana spent \
                 to cast it.)",
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCountersValue {
                        kind: CounterKind::named("charge"),
                        amount: ValueDef::ColorsOfManaSpent,
                    },
                ),
            ),
            AbilityDef::activated_mana(
                "Remove a charge counter from this artifact: Add one mana of any color.",
                &[CostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("charge"),
                    amount: 1,
                }],
                EffectDef::AddMana(AddManaEffectDef::any_color()),
            ),
        ]),
);

// 5DN 156 — Staff of Domination
pub(in crate::card::sets) static STAFF_OF_DOMINATION_156: CardRecord = CardRecord::new(
    "Staff of Domination",
    "7980fc3b-71d5-427d-bd42-087256fd2059",
    "Ben Thompson",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated(
            "{1}: Untap this artifact.",
            &[CostDef::Mana(mana_cost!("{1}"))],
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
        AbilityDef::activated(
            "{2}, {T}: You gain 1 life.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_with_targets(
            "{3}, {T}: Untap target creature.",
            &[CostDef::Mana(mana_cost!("{3}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
        AbilityDef::activated_with_targets(
            "{4}, {T}: Tap target creature.",
            &[CostDef::Mana(mana_cost!("{4}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
        AbilityDef::activated(
            "{5}, {T}: Draw a card.",
            &[CostDef::Mana(mana_cost!("{5}")), CostDef::TapSource],
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AURIOK_SALVAGERS_4,
    &CONDESCEND,
    &SERUM_VISIONS,
    &TRINKET_MAGE,
    &NIGHTS_WHISPER,
    &FURNACE_WHELP,
    &MANA_GEYSER_75,
    &DAWNS_REFLECTION,
    &ETERNAL_WITNESS,
    &AVARICE_TOTEM_104,
    &BATTERED_GOLEM_106,
    &BLASTING_STATION_107,
    &CLOCK_OF_OMENS,
    &CONJURER_S_BAUBLE_112,
    &CRUCIBLE_OF_WORLDS,
    &DOOR_TO_NOTHINGNESS,
    &ENGINEERED_EXPLOSIVES,
    &GRINDING_STATION_127,
    &GUARDIAN_IDOL,
    &KRARK_CLAN_IRONWORKS_134,
    &LANTERN_OF_INSIGHT_135,
    &PARADISE_MANTLE_142,
    &PENTAD_PRISM,
    &STAFF_OF_DOMINATION_156,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
