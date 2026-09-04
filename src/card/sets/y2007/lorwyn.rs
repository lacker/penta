//! Lorwyn cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::CounterKind;
use crate::TriggerEventDef;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    CardRules, CardSet, CardType, ComparisonDef, EffectDef, EffectRecipientDef, FreePlayDef,
    FreePlayDurationDef, ManaColor, ObjectPredicateDef, ObjectSetDef, PlayerRefDef, PlayerRelation,
    TriggerConditionDef, ValueComparisonDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// LRW 34 — Oblivion Ring
pub(in crate::card::sets) static OBLIVION_RING: CardRecord = CardRecord::new(
    CardSet::Lorwyn,
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
    CardSet::Lorwyn,
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
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                    },
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MULLDRIFTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Lorwyn,
    "Mulldrifter",
    "a97cfefa-ade7-49f6-b2aa-1118b9db4935",
    "Eric Fortune",
    crate::card::CardRules::unsupported(),
);

// LRW 78 — Pestermite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PESTERMITE: CardRecord = CardRecord::new(
    crate::card::CardSet::Lorwyn,
    "Pestermite",
    "f252ae53-443c-4a27-b8f0-639a9a2b8598",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// LRW 79 — Ponder
pub(in crate::card::sets) static PONDER: CardRecord = CardRecord::new(
    CardSet::Lorwyn,
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

// LRW 145 — Thoughtseize
pub(in crate::card::sets) static THOUGHTSEIZE: CardRecord = CardRecord::new(
    CardSet::Lorwyn,
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

// LRW 173 — Hamletback Goliath
pub(in crate::card::sets) static HAMLETBACK_GOLIATH: CardRecord = CardRecord::new(
    CardSet::Lorwyn,
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

// LRW 194 — Tarfire
/// A Shock that is also a Goblin card. Kindred is what carries the subtype
/// onto a noncreature card (CR 205.2a) and does nothing else: the spell is
/// still an instant, and it is a Goblin in every zone -- in the library
/// where a Ringleader looks for one, and in the graveyard afterwards.
pub(in crate::card::sets) static TARFIRE: CardRecord = CardRecord::new(
    CardSet::Lorwyn,
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
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        )),
);

// LRW 196 — Wild Ricochet
pub(in crate::card::sets) static WILD_RICOCHET: CardRecord = CardRecord::new(
    crate::card::CardSet::Lorwyn,
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

// LRW 262 — Thorn of Amethyst
pub(in crate::card::sets) static THORN_OF_AMETHYST: CardRecord = CardRecord::new(
    CardSet::Lorwyn,
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

// LRW 272 — Shelldock Isle
pub(in crate::card::sets) static SHELLDOCK_ISLE: CardRecord = CardRecord::new(
    CardSet::Lorwyn,
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
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
        ),
        AbilityDef::activated(
            "{U}, {T}: You may play the exiled card without paying its mana cost if a library \
             has twenty or fewer cards in it.",
            &[
                AbilityCostDef::Mana(mana_cost!("{U}")),
                AbilityCostDef::TapSource,
            ],
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIMMERING_GROTTO: CardRecord = CardRecord::new(
    crate::card::CardSet::Lorwyn,
    "Shimmering Grotto",
    "b5e85acc-ed12-4036-8193-739721c3e178",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &OBLIVION_RING,
    &CRYPTIC_COMMAND,
    &MULLDRIFTER,
    &PESTERMITE,
    &PONDER,
    &THOUGHTSEIZE,
    &HAMLETBACK_GOLIATH,
    &TARFIRE,
    &WILD_RICOCHET,
    &THORN_OF_AMETHYST,
    &SHELLDOCK_ISLE,
    &SHIMMERING_GROTTO,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
