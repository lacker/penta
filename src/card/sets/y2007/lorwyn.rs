//! Lorwyn cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::CounterKind;
use crate::ParentBinding;
use crate::TriggerEventDef;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AggregateOperationDef;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::CopyAbilityDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::CreateTokenDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DamageEventMatcherDef;
use crate::card::DamagePreventionDef;
use crate::card::EffectChoiceDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::FreePlayDef;
use crate::card::FreePlayDurationDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectValueAggregateDef;
use crate::card::ObjectValueDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenCopyDef;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "LRW",
    slug: "lorwyn",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// LRW 7 — Burrenton Forge-Tender
pub(in crate::card::sets) static BURRENTON_FORGE_TENDER_7: CardRecord = CardRecord::new(
    "Burrenton Forge-Tender",
    "c000c3e4-d71a-43c8-8ded-f3da54bc088d",
    "Chuck Lukacs",
    CardRules::new_creature(mana_cost!("{W}"), &["Kithkin", "Wizard"], 1, 1).with_abilities(&[
        abilities::protection_from_color(ManaColor::Red),
        AbilityDef::activated(
            "Sacrifice this creature: Prevent all damage a red source of your choice \
             would deal this turn.",
            &[CostDef::SacrificeSource],
            abilities::shield_against_a_chosen_source(
                ObjectPredicateDef::Color(ManaColor::Red),
                &EffectDef::PreventDamage {
                    prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::from(
                        ObjectRefDef::Binding(ParentBinding),
                    )),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ),
    ]),
);

// LRW 11 — Crib Swap
// Audit: unsupported — Needs an all-zone creature-type characteristic-defining ability whose all-types value is copiable; battlefield all-type modifiers do not implement changeling.
pub(in crate::card::sets) static CRIB_SWAP: CardRecord = CardRecord::new(
    "Crib Swap",
    "a9044585-4d44-42fb-ad7b-e0e224fbc502",
    "Brandon Dorman",
    CardRules::unsupported(),
);

// LRW 34 — Oblivion Ring
pub(in crate::card::sets) static OBLIVION_RING: CardRecord = CardRecord::new(
    "Oblivion Ring",
    "1c7fffe8-709c-4cb4-bbad-e4a0c35b616a",
    "Wayne England",
CardRules::new_enchantment(mana_cost!("{2}{W}")).with_abilities(&[
        abilities::enters_trigger_with_targets("When this enchantment enters, exile another target nonland permanent.", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )], EffectDef::ExileLinkedToSource {
                until_source_leaves: false,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
face_down: false,
then: None,
}),
        AbilityDef::triggered(
            "When this enchantment leaves the battlefield, return the exiled card to the battlefield under its owner's control.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, Some(ZoneKind::Battlefield), None),
            EffectDef::ReturnLinkedExiles {
                object: ObjectPredicateDef::Any,
                counters: None,
                zone: ZoneKind::Battlefield,
                grant: None,
                controller: None,
                transformed: false,
            },
        ),
    ]),
);

// LRW 56 — Cryptic Command
pub(in crate::card::sets) static CRYPTIC_COMMAND: CardRecord = CardRecord::new(
    "Cryptic Command",
    "829e3d6e-5d7c-4cc4-a7a6-7cbf5a7442ba",
    "Wayne England",
    // Four mana of triple blue that is never the wrong card: counter and
    // draw when they act, bounce and draw when they do not.
    CardRules::new_instant(mana_cost!("{1}{U}{U}{U}")).with_ability(
        AbilityDef::modal_spell(
            "Choose two —",
            // Two of four, and never the same one twice. Each targeting mode carries
            // its own slot, so a Command that counters and bounces declares a spell and
            // a permanent, and one that taps and draws declares nothing at all.
            &[
                AbilityDef::counter_target(
                    "Counter target spell.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::Spell,
                            zones: &[ZoneKind::Stack],
                            controller: None,
                            owner: None,
                        },
                    )][0],
                ),
                AbilityDef::spell_with_targets(
                    "Return target permanent to its owner's hand.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::Any,
                    )],
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                ),
                // Their creatures, not everyone's: the Command is a Fog you get to keep
                // the draw off, and tapping your own would defeat the point.
                AbilityDef::spell(
                    "Tap all creatures your opponents control.",
                    EffectDef::Tap {
                        object: EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Opponent,
                        ),
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
        .with_mode_selection(2, 2, false),
    ),
);

// LRW 76 — Mulldrifter
pub(in crate::card::sets) static MULLDRIFTER: CardRecord = CardRecord::new(
    "Mulldrifter",
    "a97cfefa-ade7-49f6-b2aa-1118b9db4935",
    "Eric Fortune",
    // Five mana for a flier and two cards, or three mana for just the two
    // cards. Only choosing the evoke cost triggers the sacrifice; another
    // alternative cost can also leave the creature on the battlefield.
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Elemental"], 2, 2).with_abilities(
        &crate::ability_list![
            [
                abilities::flying(),
                abilities::enters_trigger(
                    "When this creature enters, draw two cards.",
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                    },
                ),
            ],
            abilities::evoke(&[CostDef::Mana(mana_cost!("{2}{U}"))],),
        ],
    ),
);

// LRW 78 — Pestermite
pub(in crate::card::sets) static PESTERMITE: CardRecord = CardRecord::new(
    "Pestermite",
    "f252ae53-443c-4a27-b8f0-639a9a2b8598",
    "Christopher Moeller",
    // Flash plus the untap half is the whole reason the card is remembered:
    // held up on the opponent's end step it is a Time Walk on their land.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Faerie", "Rogue"], 2, 1).with_abilities(&[
        abilities::flash(),
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, you may tap or untap target permanent.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Any,
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::ChooseEffect {
                    player: EffectRecipientDef::Controller,
                    choices: &[
                        EffectChoiceDef {
                            label: "Tap it",
                            effect: EffectDef::Tap {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            },
                        },
                        EffectChoiceDef {
                            label: "Untap it",
                            effect: EffectDef::Untap {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            },
                        },
                    ],
                },
            },
        ),
    ]),
);

// LRW 79 — Ponder
pub(in crate::card::sets) static PONDER: CardRecord = CardRecord::new(
    "Ponder",
    "ba6b6fc5-5077-4812-b8e9-906783dbaf67",
    "Mark Tedin",
    // One mana to see four cards deep and keep the best of them, which is
    // why the format has never been able to leave it legal for long.
    CardRules::new_sorcery(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Look at the top three cards of your library, then put them back in any order. You may \
         shuffle.\nDraw a card.",
        EffectDef::Sequence(&[
            abilities::look_at_top_cards_and_reorder(
                PlayerRefDef::EffectController,
                ValueDef::Constant(3),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::ShuffleLibrary {
                    player: EffectRecipientDef::Controller,
                },
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// LRW 97 — Wings of Velis Vel
pub(in crate::card::sets) static WINGS_OF_VELIS_VEL_97: CardRecord = CardRecord::new(
    "Wings of Velis Vel",
    "fb3c1f39-b6ac-4663-9623-bd573a1117b0",
    "Jim Pavelec",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_subtypes(&["Shapeshifter"]).with_type(CardType::Kindred).with_abilities(&[AbilityDef::static_ability("Changeling (This card is every creature type.)", EffectDef::StaticApply { recipient: EffectRecipientDef::Source, effect: AppliedEffectDef::Characteristic(crate::card::CharacteristicOperationDef::Subtypes(crate::card::SetOperationDef::Add(crate::card::CREATURE_TYPES))) }).with_source_zones(&[ZoneKind::Library, ZoneKind::Hand, ZoneKind::Graveyard, ZoneKind::Stack, ZoneKind::Exile, ZoneKind::Command]),

AbilityDef::spell_with_targets("Until end of turn, target creature has base power and toughness 4/4, gains all creature types, and gains flying.", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))], EffectDef::Apply { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), effect: AppliedEffectDef::Composite(&[AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(4), ValueDef::Constant(4)), AppliedEffectDef::set_creature_types(CreatureTypeSetDef::ALL), AppliedEffectDef::add_ability(&abilities::flying())]), duration: ResolvedEffectDurationDef::UntilEndOfTurn })

]),
);

// LRW 128 — Nameless Inversion
// Audit: unsupported — Needs an all-zone creature-type characteristic-defining ability whose all-types value is copiable; battlefield all-type modifiers do not implement changeling.
pub(in crate::card::sets) static NAMELESS_INVERSION: CardRecord = CardRecord::new(
    "Nameless Inversion",
    "94b4e4d2-2358-48d2-9a2a-3d17afea28f5",
    "Jeff Miracola",
    CardRules::unsupported(),
);

// LRW 145 — Thoughtseize
pub(in crate::card::sets) static THOUGHTSEIZE: CardRecord = CardRecord::new(
    "Thoughtseize",
    "3df8c148-e87d-4043-9d8b-ec72bf8b6d5d",
    "Aleksi Briclot",
    // One mana, any card, two life. The life is what keeps it honest and it
    // has never been enough.
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player reveals their hand. You choose a nonland card from it. That player \
         discards that card. You lose 2 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        // The hand is revealed rather than looked at: everybody sees it, which is
        // what makes the choice checkable and what the card prints.
        EffectDef::Sequence(&[
            EffectDef::Sequence(&abilities::reveal_hand_and_discard_chosen_card(
                PlayerRefDef::Target(TargetIndex::PRIMARY),
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            )),
            // Unconditional: a hand of nothing but lands still costs you two.
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )),
);

// LRW 152 — Blades of Velis Vel
pub(in crate::card::sets) static BLADES_OF_VELIS_VEL_152: CardRecord = CardRecord::new(
    "Blades of Velis Vel",
    "5a3ac629-a8c9-4b84-a8ea-b775d7913238",
    "Ron Spencer",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_subtypes(&["Shapeshifter"]).with_abilities(&[
AbilityDef::static_ability("Changeling (This card is every creature type.)", EffectDef::StaticApply { recipient: EffectRecipientDef::Source, effect: AppliedEffectDef::Characteristic(crate::card::CharacteristicOperationDef::Subtypes(crate::card::SetOperationDef::Add(crate::card::CREATURE_TYPES))) }).with_source_zones(&[ZoneKind::Library, ZoneKind::Hand, ZoneKind::Graveyard, ZoneKind::Stack, ZoneKind::Exile, ZoneKind::Command]),
AbilityDef::spell_with_targets("Up to two target creatures each get +2/+0 and gain all creature types until end of turn.", &[AbilityTargetDef::up_to(AbilityTargetPredicate::Object { object: ObjectPredicateDef::HasType(CardType::Creature), zones: &[ZoneKind::Battlefield], controller: None, owner: None }, 2)], EffectDef::Apply { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), effect: AppliedEffectDef::Composite(&[AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(0)), AppliedEffectDef::set_creature_types(CreatureTypeSetDef::ALL)]), duration: ResolvedEffectDurationDef::UntilEndOfTurn })
]).with_type(CardType::Kindred),
);

// LRW 173 — Hamletback Goliath
pub(in crate::card::sets) static HAMLETBACK_GOLIATH: CardRecord = CardRecord::new(
    "Hamletback Goliath",
    "96f71692-6389-462f-933e-b18b5aa7d76b",
    "Paolo Parente & Brian Snõddy",
    // "Another creature", with no controller clause: the opponent's arrivals
    // feed it too, which is what makes it worth its cost.
    CardRules::new_creature(mana_cost!("{6}{R}"), &["Giant", "Warrior"], 6, 6).with_ability(
        AbilityDef::triggered(
            "Whenever another creature enters, you may put X +1/+1 counters on this creature, \
             where X is that creature's power.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::TriggeringObjectPower,
                },
            },
        ),
    ),
);

// LRW 175 — Heat Shimmer
pub(in crate::card::sets) static HEAT_SHIMMER_175: CardRecord = CardRecord::new(
    "Heat Shimmer",
    "a432470c-7f68-4429-970a-3da8eabcf0b8",
    "Franz Vohwinkel",
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[
AbilityDef::spell_with_targets("Create a token that's a copy of target creature, except it has haste and \"At the beginning of the end step, exile this token.\"", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))], EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Copy(&TokenCopyDef { object: &EffectRecipientDef::Target(TargetIndex::PRIMARY), exceptions: CopyExceptionsDef::NONE.with_abilities(&[CopyAbilityDef::Ability(&abilities::haste()), CopyAbilityDef::Ability(&AbilityDef::triggered("At the beginning of the end step, exile this token.", TriggerEventDef::StepBegins { step: TurnStepDef::End, player: PlayerRelation::Any }, EffectDef::move_to_zone(EffectRecipientDef::Source, ZoneKind::Exile, ZonePlacement::Top)))]) }))))
]),
);

// LRW 186 — Needle Drop
// Audit: unsupported — Object predicates record damage received this turn, but target-player predicates cannot require that history for the player branch of any target.
pub(in crate::card::sets) static NEEDLE_DROP_186: CardRecord = CardRecord::new(
    "Needle Drop",
    "d3f89bcf-46f8-4598-a949-7f10134606aa",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// LRW 194 — Tarfire
/// A Shock that is also a Goblin card. Kindred is what carries the subtype
/// onto a noncreature card (CR 205.2a) and does nothing else: the spell is
/// still an instant, and it is a Goblin in every zone -- in the library
/// where a Ringleader looks for one, and in the graveyard afterwards.
pub(in crate::card::sets) static TARFIRE: CardRecord = CardRecord::new(
    "Tarfire",
    "d13a898e-6a97-4fd9-980e-3bfd8d755386",
    "Omar Rayyan",
    // Two damage for one mana is a fine rate and not why it is played: the
    // Goblin deck plays it because Ringleader draws it and Matron finds it.
    CardRules::new_instant(mana_cost!("{R}"))
        .with_type(CardType::Kindred)
        .with_subtypes(&["Goblin"])
        .with_ability(AbilityDef::spell_with_targets(
            "This spell deals 2 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
        )),
);

// LRW 196 — Wild Ricochet
pub(in crate::card::sets) static WILD_RICOCHET: CardRecord = CardRecord::new(
    "Wild Ricochet",
    "d76f09bc-b49a-4ad2-be2d-2a191d41b86d",
    "Dan Murayama Scott",
CardRules::new_instant(mana_cost!("{2}{R}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "You may choose new targets for target instant or sorcery spell. Then copy that spell. You may choose new targets for the copy.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Spell,
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::ChangeStackTargets(&crate::card::ChangeStackTargetsDef {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    chooser: PlayerRefDef::EffectController,
                    change: crate::card::StackTargetChangeDef::ChooseNew {
                        optional: true,
                        restriction: None,
                    },
                }),
                EffectDef::CopyStackObject(&crate::card::CopyStackObjectDef {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    controller: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(1),
                    retarget: true,
                    colors: None,
                }),
            ]),
        ),
    ),
);

// LRW 207 — Elvish Harbinger
pub(in crate::card::sets) static ELVISH_HARBINGER_207: CardRecord = CardRecord::new(
    "Elvish Harbinger",
    "de789231-8358-4cbd-b8eb-1da4ce5b34c0",
    "Larry MacDougall",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Druid"], 1, 2).with_abilities(&[
abilities::enters_trigger("When this creature enters, you may search your library for an Elf card, reveal it, then shuffle and put that card on top.", EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")), minimum: 0, maximum: ValueDef::Constant(1), reveal: true, destination: ZoneKind::Library, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None }),
abilities::tap_for_mana("{T}: Add one mana of any color.", AddManaEffectDef::any_color())
]),
);

// LRW 220 — Imperious Perfect
pub(in crate::card::sets) static IMPERIOUS_PERFECT: CardRecord = CardRecord::new(
    "Imperious Perfect",
    "706fce74-fed9-4bf7-949d-7df6bef29238",
    "Scott M. Fischer",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Warrior"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "Other Elves you control get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
            },
        ),
        AbilityDef::activated(
            "{G}, {T}: Create a 1/1 green Elf Warrior creature token.",
            &[CostDef::Mana(mana_cost!("{G}")), CostDef::TapSource],
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Elf", "Warrior"], &[ManaColor::Green], 1, 1),
            ))),
        ),
    ]),
);

// LRW 257 — Herbal Poultice
pub(in crate::card::sets) static HERBAL_POULTICE_257: CardRecord = CardRecord::new(
    "Herbal Poultice",
    "b20925a3-dd4f-477c-806a-a3ec0fd2e00d",
    "Scott Hampton",
    CardRules::new_artifact(mana_cost!("{0}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{3}, Sacrifice this artifact: Regenerate target creature.",
            &[CostDef::Mana(mana_cost!("{3}")), CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Regenerate {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// LRW 259 — Rings of Brighthearth
// Audit: unsupported — TriggerEventDef has no event for activating a nonmana ability; copying stack abilities alone cannot create the printed activation trigger.
pub(in crate::card::sets) static RINGS_OF_BRIGHTHEARTH_259: CardRecord = CardRecord::new(
    "Rings of Brighthearth",
    "fbfd3898-cb06-4bb9-9d52-b319e1fa2217",
    "Howard Lyon",
    crate::card::CardRules::unsupported(),
);

// LRW 260 — Runed Stalactite
pub(in crate::card::sets) static RUNED_STALACTITE_260: CardRecord = CardRecord::new(
    "Runed Stalactite",
    "9be88336-83c7-422d-8826-13ceb8db5534",
    "Jim Pavelec",
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 and is every creature type.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::object(ObjectRefDef::AttachedToSource),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::ALL),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// LRW 261 — Springleaf Drum
// Audit: unsupported — Needs an immediate mana-ability payment combining TapSource with a chosen untapped creature; ordinary nonmana activations support that tap payment but the immediate mana path does not.
pub(in crate::card::sets) static SPRINGLEAF_DRUM: CardRecord = CardRecord::new(
    "Springleaf Drum",
    "fa8b09d0-fbd2-4441-9d87-02450412e0db",
    "Cyril Van Der Haegen",
    CardRules::unsupported(),
);

// LRW 262 — Thorn of Amethyst
pub(in crate::card::sets) static THORN_OF_AMETHYST: CardRecord = CardRecord::new(
    "Thorn of Amethyst",
    "e472d4f5-add4-4de3-8718-31a47a35277c",
    "Chuck Lukacs",
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(abilities::spell_cost_increase(
        "Noncreature spells cost {1} more to cast.",
        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
        PlayerRelation::Any,
        mana_cost!("{1}"),
    )),
);

// LRW 263 — Thousand-Year Elixir
// Audit: unsupported — No continuous rule waives summoning sickness only for creature activated abilities while leaving their attack restriction intact.
pub(in crate::card::sets) static THOUSAND_YEAR_ELIXIR_263: CardRecord = CardRecord::new(
    "Thousand-Year Elixir",
    "18743fd4-2a15-40a2-ac90-e3f0fef07e37",
    "Richard Sardinha",
    crate::card::CardRules::unsupported(),
);

// LRW 265 — Wanderer's Twig
pub(in crate::card::sets) static WANDERER_S_TWIG_265: CardRecord = CardRecord::new(
    "Wanderer's Twig",
    "8ea7b2c0-c641-478f-b8d9-17aa17fa1cbe",
    "Dave Dorman",
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
AbilityDef::activated("{1}, Sacrifice this artifact: Search your library for a basic land card, reveal it, put it into your hand, then shuffle.", &[CostDef::Mana(mana_cost!("{1}")), CostDef::SacrificeSource], EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Land), ObjectPredicateDef::Supertype(CardSupertype::Basic)]), minimum: 0, maximum: ValueDef::Constant(1), reveal: true, destination: ZoneKind::Hand, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None })
]),
);

// LRW 270 — Mosswort Bridge
pub(in crate::card::sets) static MOSSWORT_BRIDGE_270: CardRecord = CardRecord::new(
    "Mosswort Bridge",
    "38234590-812c-4d29-80c1-32b9e1282580",
    "Jeremy Jarvis",
    CardRules::new_land(&[]).with_abilities(&[
abilities::enters_trigger("Hideaway 4 (When this land enters, look at the top four cards of your library, exile one face down, then put the rest on the bottom in a random order.)", abilities::hideaway(ValueDef::Constant(4))),
abilities::enters_tapped(CardType::Land),
abilities::tap_for(ManaColor::Green),
AbilityDef::activated("{G}, {T}: You may play the exiled card without paying its mana cost if creatures you control have total power 10 or greater.", &[CostDef::Mana(mana_cost!("{G}")), CostDef::TapSource], EffectDef::IfCondition { condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::AggregateObjectValues(&ObjectValueAggregateDef { objects: ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You)), select: ObjectValueDef::Power, operation: AggregateOperationDef::Sum }), comparison: ComparisonDef::GreaterOrEqual, right: ValueDef::Constant(10) }), then: &EffectDef::MayPlayWithoutPaying(FreePlayDef { objects: ObjectSetDef::LinkedExiles, duration: FreePlayDurationDef::WhileResolving, mandatory: false, grants_haste: false }) })
]),
);

// LRW 272 — Shelldock Isle
pub(in crate::card::sets) static SHELLDOCK_ISLE: CardRecord = CardRecord::new(
    "Shelldock Isle",
    "4216656e-90e8-45fc-a0f6-0d0d79d0a021",
    "Mark Tedin",
    // A tapped Island that hides your best card until the game is nearly
    // over, and then plays it for nothing.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_trigger(
            "Hideaway 4 (When this land enters, look at the top four cards of your library, \
             exile one face down, then put the rest on the bottom in a random order.)",
            abilities::hideaway(ValueDef::Constant(4)),
        ),
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
        ),
        AbilityDef::activated(
            "{U}, {T}: You may play the exiled card without paying its mana cost if a library \
             has twenty or fewer cards in it.",
            &[CostDef::Mana(mana_cost!("{U}")), CostDef::TapSource],
            // "You may play the exiled card": the offer stands while this
            // ability resolves and no longer, so a player who declines has
            // to pay the {U} and the tap again to be asked twice.
            EffectDef::MayPlayWithoutPaying(FreePlayDef {
                objects: ObjectSetDef::LinkedExiles,
                duration: FreePlayDurationDef::WhileResolving,
                mandatory: false,
                grants_haste: false,
            }),
        )
        // "If a library has twenty or fewer cards in it" -- either library, which
        // is why the two are asked separately rather than counted together.
        .with_activation_condition(&TriggerConditionDef::AnyOf(&[
            TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::LibrarySize(PlayerRelation::You),
                comparison: ComparisonDef::LessOrEqual,
                right: ValueDef::Constant(20),
            }),
            TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::LibrarySize(PlayerRelation::Opponent),
                comparison: ComparisonDef::LessOrEqual,
                right: ValueDef::Constant(20),
            }),
        ])),
    ]),
);

// LRW 273 — Shimmering Grotto
pub(in crate::card::sets) static SHIMMERING_GROTTO: CardRecord = CardRecord::new(
    "Shimmering Grotto",
    "b5e85acc-ed12-4036-8193-739721c3e178",
    "Alan Pollack",
    // The same land a mana cheaper, printed into a block where five-colour
    // decks were the point.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BURRENTON_FORGE_TENDER_7,
    &CRIB_SWAP,
    &OBLIVION_RING,
    &CRYPTIC_COMMAND,
    &MULLDRIFTER,
    &PESTERMITE,
    &PONDER,
    &WINGS_OF_VELIS_VEL_97,
    &NAMELESS_INVERSION,
    &THOUGHTSEIZE,
    &BLADES_OF_VELIS_VEL_152,
    &HAMLETBACK_GOLIATH,
    &HEAT_SHIMMER_175,
    &NEEDLE_DROP_186,
    &TARFIRE,
    &WILD_RICOCHET,
    &ELVISH_HARBINGER_207,
    &IMPERIOUS_PERFECT,
    &HERBAL_POULTICE_257,
    &RINGS_OF_BRIGHTHEARTH_259,
    &RUNED_STALACTITE_260,
    &SPRINGLEAF_DRUM,
    &THORN_OF_AMETHYST,
    &THOUSAND_YEAR_ELIXIR_263,
    &WANDERER_S_TWIG_265,
    &MOSSWORT_BRIDGE_270,
    &SHELLDOCK_ISLE,
    &SHIMMERING_GROTTO,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
