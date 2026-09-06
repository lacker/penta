//! Lost Caverns of Ixalan cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::CostQuantityDef;
use crate::card::{
    AbilityDef, AbilityPredicateDef, AbilityTargetDef, AbilityTargetPredicate,
    AlternativeCastKindDef, AppliedEffectDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    ChoiceVisibilityDef, ChooseDef, ComparisonDef, CostDef, CounterKind, DiscardFollowUpDef,
    DiscardSelectionDef, EffectDef, EffectPaymentDef, EffectRecipientDef, ExilePlayDurationDef,
    InstalledTriggerDef, ManaColor, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef,
    ObjectRefDef, ObjectSetDef, PayOrDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    ResolvedEffectDurationDef, TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind,
    ZonePlacement, abilities, tokens,
};
use crate::ids::ParentBinding;
use crate::{TargetIndex, mana_cost};

// LCI 14 — Get Lost
pub(in crate::card::sets) static GET_LOST: CardRecord = CardRecord::new_with_legacy_id(
    2294,
    "Get Lost",
    CardArt::new("522aa72b-2b8c-484c-872b-f082101cee35", "Eli Minaya"),
    CardSet::LostCavernsOfIxalan,
    // Two mana that answers three card types at instant speed, and the two
    // Maps are what it pays for that: real but slow ones.
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature, enchantment, or planeswalker. Its controller creates two Map \
         tokens.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Enchantment),
                ObjectPredicateDef::HasType(CardType::Planeswalker),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            // "Its controller creates two Map tokens." The Maps are theirs, not yours,
            // and the permanent is already destroyed by the time they arrive -- so the
            // player is read from what the target was rather than from where it is.
            EffectDef::create_token(tokens::map())
                .with_art(CardArt::new(
                    "64839118-09d2-4645-9d3c-f80755ac781f",
                    "Francesca Baerald",
                ))
                .with_controller(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                    TargetIndex::PRIMARY,
                )))
                .with_amount(2),
        ]),
    )),
);

// LCI 24 — Miner's Guidewing
pub(in crate::card::sets) static MINER_S_GUIDEWING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9048cd9d-df3f-4705-a5f4-e5b09760c631"),
    "Miner's Guidewing",
    CardArt::new("9048cd9d-df3f-4705-a5f4-e5b09760c631", "Allen Douglas"),
    CardSet::LostCavernsOfIxalan,
    // A one-drop flier that pays again when it trades. Vigilance is what
    // makes the trade happen on their turn as well as yours, so the explore
    // is rarely far away.
    CardRules::new_creature(mana_cost!("{W}"), &["Bird"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::vigilance(),
        abilities::dies_trigger_with_targets(
            "When this creature dies, target creature you control explores.",
            // The Bird is already in the graveyard when this resolves, so
            // "creature you control" never includes it: the target is chosen
            // from whatever is left.
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
            )],
            EffectDef::Explore {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// LCI 30 — Petrify
pub(in crate::card::sets) static PETRIFY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bbc5f28f-6361-455f-ac82-260a70e59316"),
    "Petrify",
    CardArt::new("bbc5f28f-6361-455f-ac82-260a70e59316", "Samuel Araya"),
    CardSet::LostCavernsOfIxalan,
    // Two mana that answers a creature or a mana rock, and unlike Pacifism
    // it also turns off the activated ability the creature was played for.
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell(
                "Enchant artifact or creature",
                &const {
                    [AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                        ]),
                    )]
                },
            ),
            abilities::enchanted_permanent_subdued(),
        ]),
);

// LCI 63 — Malcolm, Alluring Scoundrel
pub(in crate::card::sets) static MALCOLM_ALLURING_SCOUNDREL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("19d6834d-afa3-4747-a62d-0654f4d9729f"),
    "Malcolm, Alluring Scoundrel",
    CardArt::new("19d6834d-afa3-4747-a62d-0654f4d9729f", "Fesbra"),
    CardSet::LostCavernsOfIxalan,
    // Two mana for an evasive body that loots every time it connects, and
    // that turns the loot into a free spell once it has connected four
    // times.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Siren", "Pirate"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flash(),
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever this creature deals combat damage to a player, put a chorus counter on \
                 it. Draw a card, then discard a card. If there are four or more chorus counters \
                 on it, you may cast the discarded card without paying its mana cost.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::named("chorus"),
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: Some(DiscardFollowUpDef {
                            counted: ObjectPredicateDef::Any,
                            bound: Some(ParentBinding),
                            effect: &EffectDef::IfCondition {
                                // Read after the counter has been added, so the connection that makes it
                                // four is itself the one that pays.
                                condition: &TriggerConditionDef::SourceCounters {
                                    kind: CounterKind::named("chorus"),
                                    comparison: ComparisonDef::GreaterOrEqual,
                                    amount: 4,
                                },
                                then: &EffectDef::MayCastTargetWithoutPaying {
                                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        ParentBinding,
                                    )),
                                    // What the fourth connection is worth: the card you just threw away, cast
                                    // for nothing. The kind says both halves at once -- no mana, and an
                                    // ordinary trip to the graveyard afterwards.
                                    ability: &AbilityDef::alternative_cast(
                                        mana_cost!("{0}"),
                                        AlternativeCastKindDef::WithoutPayingManaCost,
                                        Some("Cast without paying its mana cost."),
                                        EffectDef::None,
                                    ),
                                },
                            },
                        }),
                    },
                ]),
            ),
        ]),
);

// LCI 91 — Bitter Triumph
pub(in crate::card::sets) static BITTER_TRIUMPH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("05bdd22c-3e11-4c29-bdfa-d3dfc0e90a9f"),
    "Bitter Triumph",
    CardArt::new("05bdd22c-3e11-4c29-bdfa-d3dfc0e90a9f", "Donato Giancola"),
    CardSet::LostCavernsOfIxalan,
    // Two mana for unconditional removal at instant speed, and the card or
    // the three life is the whole restriction: it answers anything, and it
    // never answers it for free.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, discard a card or pay 3 life.\nDestroy \
             target creature or planeswalker.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            // One cost with two ways to pay it. The life is the way a deck with an
            // empty hand still casts this, which is what keeps it playable late.
            CostDef::choice(&[
                CostDef::discard(ObjectPredicateDef::Any, CostQuantityDef::Fixed(1)),
                CostDef::pay_life(CostQuantityDef::Fixed(3)),
            ]),
            EffectDef::destroy_target(TargetIndex::PRIMARY),
        ),
    ),
);

// LCI 102 — Deep-Cavern Bat
pub(in crate::card::sets) static DEEP_CAVERN_BAT: CardRecord = CardRecord::new_with_legacy_id(
    2161,
    "Deep-Cavern Bat",
    CardArt::new("69c68c95-b788-43b1-9f22-1b22c5a00b25", "Campbell White"),
    CardSet::LostCavernsOfIxalan,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Bat"], 1, 1)
        .with_abilities(&[
            abilities::flying(),
            abilities::lifelink(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, look at target opponent's hand. You may exile a nonland card from it until this creature leaves the battlefield.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::Sequence(&[
                    EffectDef::LookAtHand {
                        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    // "You may exile" -- a minimum of none, so looking and taking nothing is
                    // a legal answer. The Sculler and the Freebooter both must take one.
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Object(ParentBinding),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                            &[ZoneKind::Hand],
                            PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                        )),
                        exclude: None,
                        minimum: 0,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::Sequence(&[
                            EffectDef::ExileLinkedToSource {
                                until_source_leaves: true,
                                object: EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
                                face_down: false,
                                then: None,
                            },
                            // "Until this creature leaves the battlefield" is one printed ability, so
                            // the return rides on the same resolution as a delayed trigger rather than
                            // appearing as a second clause the card does not print.
                            EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
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
                            ))),
                        ]),
                    }),
                ]),
            ),
        ]),
);

// LCI 128 — Tithing Blade // Consuming Sepulcher
// Audit: unsupported — Needs craft (CR 726). Nothing in the model expresses an activation that exiles the artifact along with a creature from the battlefield or graveyard and returns the card transformed; the transforming two-face record exists, but the ability that flips it does not.
pub(in crate::card::sets) static TITHING_BLADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dbaa9a2d-e9fd-4746-a26c-f99ae731f024"),
    "Tithing Blade",
    crate::card::CardArt::new("dbaa9a2d-e9fd-4746-a26c-f99ae731f024", "Michael Walsh"),
    crate::card::CardSet::LostCavernsOfIxalan,
    crate::card::CardRules::unsupported(),
);

// LCI 156 — Inti, Seneschal of the Sun
pub(in crate::card::sets) static INTI_SENESCHAL_OF_THE_SUN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fa7a55aa-ae61-4933-b7a4-dcc55dac6fcd"),
    "Inti, Seneschal of the Sun",
    CardArt::new(
        "fa7a55aa-ae61-4933-b7a4-dcc55dac6fcd",
        "Victor Adame Minguez",
    ),
    CardSet::LostCavernsOfIxalan,
    // Two mana that turns every spare card into a bigger attack and a new
    // card, and the two halves feed each other: the discard he asks for is
    // the discard the second clause is watching for.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Knight"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            // The target is declared as the attack trigger goes on the stack rather
            // than when the discard is made, which is the one place this differs
            // from the printed reflexive trigger. "Whenever you attack" guarantees
            // an attacking creature, so there is always something to name.
            AbilityDef::triggered_with_targets(
                "Whenever you attack, you may discard a card. When you do, put a +1/+1 counter on target \
                 attacking creature. It gains trample until end of turn.",
                TriggerEventDef::attack_declared(ObjectPredicateDef::Any, 1, None),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Attacking,
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::PayOr(PayOrDef::optional(
                    EffectPaymentDef {
                        payer: PlayerSetDef::Related(PlayerRelation::You),
                        cost: CostDef::DiscardCards(1),
                    },
                    // "It gains trample until end of turn" -- the creature that took the
                    // counter, which is the one the trigger targeted.
                    &EffectDef::Sequence(&[
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::add_ability(&abilities::trample()),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ]),
                )),
            ),
            // One trigger for the whole discard however many cards it took, and the
            // card it finds is playable into your own turn when the discard
            // happened on somebody else's.
            AbilityDef::triggered(
                "Whenever you discard one or more cards, exile the top card of your library. You may play \
                 that card until your next end step.",
                TriggerEventDef::DiscardedCards(PlayerRelation::You),
                EffectDef::ExileTopOfLibraryToPlay {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    free: false,
                    face_down: false,
                    duration: ExilePlayDurationDef::UntilYourNextEndStep,
                    spend_any_color: false,
                    play_condition: None,
                    cast_only: false,
                },
            ),
        ]),
);

// LCI 211 — Sentinel of the Nameless City
pub(in crate::card::sets) static SENTINEL_OF_THE_NAMELESS_CITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eeeffc0b-dc92-458e-ad58-86ff6077a508"),
    "Sentinel of the Nameless City",
    CardArt::new("eeeffc0b-dc92-458e-ad58-86ff6077a508", "Josu Hernaiz"),
    CardSet::LostCavernsOfIxalan,
    // A 3/4 that blocks and attacks in the same turn, and hands you a Map
    // for doing either.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Merfolk", "Warrior", "Scout"], 3, 4)
        .with_abilities(&[
            abilities::vigilance(),
            AbilityDef::triggered(
                "Whenever this creature enters or attacks, create a Map token.",
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                ]),
                EffectDef::create_token(tokens::map()).with_art(CardArt::new(
                    "64839118-09d2-4645-9d3c-f80755ac781f",
                    "Francesca Baerald",
                )),
            ),
        ]),
);

// LCI 335 — Tishana's Tidebinder
pub(in crate::card::sets) static TISHANA_S_TIDEBINDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("604e2bfc-655d-4d3e-98aa-374780ca4016"),
    "Tishana's Tidebinder",
    CardArt::new("604e2bfc-655d-4d3e-98aa-374780ca4016", "LeDania"),
    CardSet::LostCavernsOfIxalan,
    // Three mana at instant speed for a body, an answer, and a permanent
    // that never does anything again.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk", "Wizard"], 3, 2)
        .with_abilities(&[
            abilities::flash(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, counter up to one target activated or triggered ability. If \
                 an ability of an artifact, creature, or planeswalker is countered this way, that \
                 permanent loses all abilities for as long as this creature remains on the battlefield.",
                // An ability and not a spell, and up to one of them: a Tidebinder flashed
                // in with nothing on the stack is still a 3/2. Mana abilities never use the
                // stack, so nothing has to exclude them.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Ability,
                        zones: &[ZoneKind::Stack],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::Sequence(&[
                    EffectDef::Counter {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Graveyard,
                        placement: ZonePlacement::Top,
                    },
                    // The permanent the countered ability came from, read after the counter has
                    // retired it, then narrowed to the types the rider names. A countered
                    // ability whose source was an enchantment binds nothing here, which is the
                    // "if" doing its work.
                    abilities::bind_objects_then(
                        crate::card::ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::One(
                            ObjectRefDef::SourceOfTargetedStackObject(TargetIndex::PRIMARY),
                        )),
                        &EffectDef::Apply {
                            recipient: EffectRecipientDef::objects(ObjectSetDef::MatchingBinding {
                                binding: ParentBinding,
                                // The rider names three permanent types and not the other two: an
                                // enchantment or a land whose ability is countered keeps everything it has.
                                object: ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                                ]),
                            }),
                            effect: AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
                            // Not a turn and not forever: the silence lasts exactly as long as
                            // the Tidebinder is standing there.
                            duration: ResolvedEffectDurationDef::WhileSourceRemains,
                        },
                    ),
                ]),
            ),
        ]),
);

// LCI 367 — Preacher of the Schism
pub(in crate::card::sets) static PREACHER_OF_THE_SCHISM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a0db433-7ca2-48d6-b60c-0a9a9149378a"),
    "Preacher of the Schism",
    CardArt::new("3a0db433-7ca2-48d6-b60c-0a9a9149378a", "Donato Giancola"),
    CardSet::LostCavernsOfIxalan,
    // A 2/4 deathtouch body that punishes whoever is ahead on life, and
    // draws while she is the one ahead.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Vampire", "Cleric"], 2, 4).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::triggered(
            "Whenever this creature attacks the player with the most life or tied for most life, \
             create a 1/1 white Vampire creature token with lifelink.",
            // "Attacks the player with the most life": the condition belongs to the
            // attack rather than being an intervening if, and the player it asks about
            // is the one the attack was aimed at, which the event names. The player
            // themselves -- a planeswalker of theirs is a different thing to attack,
            // whoever ends up being attacked by it.
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks_a_player(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::PlayerHasMostLife(PlayerRelation::EventPlayer),
            },
            EffectDef::create_creature_token(&["Vampire"], &[ManaColor::White], 1, 1)
                .with_abilities(&[abilities::lifelink()]),
        ),
        AbilityDef::triggered(
            "Whenever this creature attacks while you have the most life or are tied for most \
             life, you draw a card and you lose 1 life.",
            // The same attack, asked about his own controller instead. Both clauses
            // read one attack, so a creature attacking the player who is ahead while
            // its controller is also tied for the lead triggers both.
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::PlayerHasMostLife(PlayerRelation::You),
            },
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &GET_LOST,
    &MINER_S_GUIDEWING,
    &PETRIFY,
    &MALCOLM_ALLURING_SCOUNDREL,
    &BITTER_TRIUMPH,
    &DEEP_CAVERN_BAT,
    &TITHING_BLADE,
    &INTI_SENESCHAL_OF_THE_SUN,
    &SENTINEL_OF_THE_NAMELESS_CITY,
    &TISHANA_S_TIDEBINDER,
    &PREACHER_OF_THE_SCHISM,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
