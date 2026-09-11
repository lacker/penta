//! Lorwyn cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::CounterKind;
use crate::TriggerEventDef;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::EffectChoiceDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::FreePlayDef;
use crate::card::FreePlayDurationDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::TriggerConditionDef;
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

// LRW 11 — Crib Swap
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRIB_SWAP: CardRecord = CardRecord::new(
    "Crib Swap",
    "a9044585-4d44-42fb-ad7b-e0e224fbc502",
    "Brandon Dorman",
    crate::card::CardRules::unsupported(),
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

// LRW 128 — Nameless Inversion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NAMELESS_INVERSION: CardRecord = CardRecord::new(
    "Nameless Inversion",
    "94b4e4d2-2358-48d2-9a2a-3d17afea28f5",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
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

// LRW 220 — Imperious Perfect
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IMPERIOUS_PERFECT: CardRecord = CardRecord::new(
    "Imperious Perfect",
    "706fce74-fed9-4bf7-949d-7df6bef29238",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// LRW 261 — Springleaf Drum
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPRINGLEAF_DRUM: CardRecord = CardRecord::new(
    "Springleaf Drum",
    "fa8b09d0-fbd2-4441-9d87-02450412e0db",
    "Cyril Van Der Haegen",
    crate::card::CardRules::unsupported(),
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
    &CRIB_SWAP,
    &OBLIVION_RING,
    &CRYPTIC_COMMAND,
    &MULLDRIFTER,
    &PESTERMITE,
    &PONDER,
    &NAMELESS_INVERSION,
    &THOUGHTSEIZE,
    &HAMLETBACK_GOLIATH,
    &TARFIRE,
    &WILD_RICOCHET,
    &IMPERIOUS_PERFECT,
    &SPRINGLEAF_DRUM,
    &THORN_OF_AMETHYST,
    &SHELLDOCK_ISLE,
    &SHIMMERING_GROTTO,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
