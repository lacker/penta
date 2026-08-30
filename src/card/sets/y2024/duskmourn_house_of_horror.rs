//! Duskmourn: House of Horror cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::CardComposition;
use crate::card::CostQuantityDef;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef,
    AddManaEffectDef, AlternativeCastKindDef, AppliedEffectDef, AppliedRuleDef, BasicLandType,
    BattlefieldEntryModificationDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    CardTypeSet, CharacteristicOperationDef, ChoiceVisibilityDef, ChooseDef, ComparisonDef,
    CounterKind, CreatureTypeSetDef, DamageEventMatcherDef, DamageKindDef,
    DamageRecipientMatcherDef, DamageSourceMatcherDef, DiscardSelectionDef, EffectDef,
    EffectRecipientDef, EmblemCharacteristics, GraveyardPlayPermissionDef, ManaColor,
    ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectSetDef, PlayActionMatcherDef,
    PlayRestrictionDef, PlayerRefDef, PlayerRelation, PlayerSetDef, ReplacementConditionDef,
    ReplacementEffectDef, ReplacementEventDef, ResolvedEffectDurationDef, SetOperationDef,
    SpellAdditionalCostDef, TokenCountersDef, TriggerConditionDef, TriggerEventDef, TurnPhaseDef,
    TurnStepDef, ValueComparisonDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::ObjectSetBindingIndex;
use crate::{TargetIndex, mana_cost};

// DSK 6 — Enduring Innocence
pub(in crate::card::sets) static ENDURING_INNOCENCE: CardRecord = CardRecord::new_with_legacy_id(
    2222,
    "Enduring Innocence",
    CardArt::new("6d908299-aac0-46a6-8fa5-780d5b3e0386", "Liiga Smilshkalne"),
    CardSet::DuskmournHouseOfHorror,
    // Answering it costs two cards: one to kill the creature and one for the
    // enchantment that gets up afterwards and keeps drawing.
    CardRules::new_enchantment_creature(mana_cost!("{1}{W}{W}"), &["Sheep", "Glimmer"], 2, 1)
        .with_abilities(&[
        abilities::lifelink(),
        AbilityDef::triggered(
            "Whenever one or more other creatures you control with power 2 or less enter, draw a \
                 card. This ability triggers only once each turn.",
            TriggerEventDef::zone_changed(
                // "Other creatures you control with power 2 or less", read as each one
                // enters. The cap below is what makes a batch of them draw one card.
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )
        .triggering_at_most(1),
        AbilityDef::triggered_if(
            "When this creature dies, if it was a creature, return it to the battlefield under its \
                 owner's control. It's an enchantment. (It's not a creature.)",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            &TriggerConditionDef::SourceMatches {
                object: ObjectPredicateDef::HasType(CardType::Creature),
            },
            EffectDef::WithZoneMoveResult {
                effect: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::TriggeringZoneChangeResult,
                    zone: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                },
                binding: ObjectSetBindingIndex::PRIMARY,
                then: &EffectDef::Apply {
                    recipient: EffectRecipientDef::binding_zone_change_successors(
                        ObjectSetBindingIndex::PRIMARY,
                    ),
                    // What it comes back as. Setting the type line rather than adding to it is
                    // what takes the creature away, and the effect lasts as long as the
                    // permanent does -- so the next time it dies the clause below finds an
                    // enchantment and leaves it in the graveyard.
                    effect: AppliedEffectDef::set_card_types(CardTypeSet::single(
                        CardType::Enchantment,
                    )),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            },
        ),
    ]),
);

// DSK 18 — Leyline of Hope
// Audit: partial — The opening-hand action is declarative; its life-gain replacement and starting-life-relative anthem need shared static effects.
pub(in crate::card::sets) static LEYLINE_OF_HOPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("40960e47-3065-485e-aede-29a62411034e"),
    "Leyline of Hope",
    CardArt::new("40960e47-3065-485e-aede-29a62411034e", "Sergey Glushakov"),
    CardSet::DuskmournHouseOfHorror,
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_abilities(&[
        abilities::begin_game_on_battlefield("If this card is in your opening hand, you may begin the game with it on the battlefield."),
        AbilityDef::not_implemented("If you would gain life, you gain that much life plus 1 instead.\nAs long as you have at least 7 life more than your starting life total, creatures you control get +2/+2.", "Needs a controller-scoped life-gain replacement and a static condition relative to the format's starting life."),
    ]),
);

// DSK 36 — Trapped in the Screen
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TRAPPED_IN_THE_SCREEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1fe95bfb-8ca7-434f-a2e7-a6b2e699584e"),
    "Trapped in the Screen",
    crate::card::CardArt::new("1fe95bfb-8ca7-434f-a2e7-a6b2e699584e", "Michael Phillippi"),
    crate::card::CardSet::DuskmournHouseOfHorror,
    crate::card::CardRules::unsupported(),
);

// DSK 42 — Abhorrent Oculus
pub(in crate::card::sets) static ABHORRENT_OCULUS: CardRecord = CardRecord::new_with_legacy_id(
    2270,
    "Abhorrent Oculus",
    CardArt::new("d2705b43-a94a-44c0-8740-82e0b296820c", "Bryan Sola"),
    CardSet::DuskmournHouseOfHorror,
    // A three-mana 5/5 flier for a deck that filled its own graveyard on
    // purpose, and a body every turn afterwards for nothing.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Eye"], 5, 5).with_abilities(&[
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, exile six cards from your graveyard.",
            &[],
            // Six cards out of your own graveyard, exiled to pay. Nothing is chosen
            // after the fact: the additional cost travels with the cast.
            SpellAdditionalCostDef::exile(
                ObjectPredicateDef::Any,
                ZoneKind::Graveyard,
                CostQuantityDef::Fixed(6),
            ),
            EffectDef::None,
        ),
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of each opponent's upkeep, manifest dread. (Look at the top two cards \
             of your library. Put one onto the battlefield face down as a 2/2 creature and the other \
             into your graveyard. Turn it face up any time for its mana cost if it's a creature \
             card.)",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::Opponent,
            },
            abilities::manifest_dread(),
        ),
    ]),
);

// DSK 78 — Unable to Scream
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static UNABLE_TO_SCREAM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7c59e0cd-10a8-4a32-9c0a-a2c6ef1ed9a6"),
    "Unable to Scream",
    crate::card::CardArt::new("7c59e0cd-10a8-4a32-9c0a-a2c6ef1ed9a6", "Fariba Khamseh"),
    crate::card::CardSet::DuskmournHouseOfHorror,
    crate::card::CardRules::unsupported(),
);

// DSK 113 — Overlord of the Balemurk
pub(in crate::card::sets) static OVERLORD_OF_THE_BALEMURK: CardRecord =
    CardRecord::new_with_legacy_id(
        2234,
        "Overlord of the Balemurk",
        CardArt::new("9b911653-7b96-4cf3-a907-13c5c53a14f7", "Babs Webb"),
        CardSet::DuskmournHouseOfHorror,
        // Two mana for the trigger now and a 5/5 five turns later, which is the
        // whole appeal: the enchantment does the work while the body waits.
        CardRules::new_enchantment_creature(mana_cost!("{3}{B}{B}"), &["Avatar", "Horror"], 5, 5)
            .with_abilities(&[
                AbilityDef::alternative_cast(
                    mana_cost!("{1}{B}"),
                    AlternativeCastKindDef::Impending,
                    Some(
                        "Impending 5—{1}{B} (If you cast this spell for its impending cost, it enters with \
                         five time counters and isn't a creature until the last is removed. At the beginning \
                         of your end step, remove a time counter from it.)",
                    ),
                    EffectDef::None,
                ),
                AbilityDef::as_enters_if(
                    "If you cast this spell for its impending cost, it enters with five time counters.",
                    ReplacementConditionDef::SourceCastWith(AlternativeCastKindDef::Impending),
                    ReplacementEffectDef::ModifyBattlefieldEntry(
                        BattlefieldEntryModificationDef::AddCounters {
                            kind: CounterKind::named("time"),
                            amount: 5,
                        },
                    ),
                ),
                AbilityDef::triggered(
                    "At the beginning of your end step, remove a time counter from this permanent.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::End,
                        player: PlayerRelation::You,
                    },
                    EffectDef::RemoveCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::named("time"),
                        amount: ValueDef::Constant(1),
                    },
                ),
                AbilityDef::triggered(
                    "Whenever this permanent enters or attacks, mill four cards, then you may return a \
                     non-Avatar creature card or a planeswalker card from your graveyard to your hand.",
                    TriggerEventDef::AnyOf(&[
                        TriggerEventDef::zone_changed(
                            ObjectPredicateDef::Source,
                            None,
                            Some(ZoneKind::Battlefield),
                        ),
                        TriggerEventDef::attacks(ObjectPredicateDef::Source),
                    ]),
                    EffectDef::Sequence(&[
                        EffectDef::Mill {
                            player: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(4),
                            binding: None,
                        },
                        // The whole graveyard, not only what the mill just put there: the clause
                        // says "from your graveyard" and means it.
                        EffectDef::Choose(ChooseDef {
                            binding: ObjectChoiceBindingDef::Objects(ObjectSetBindingIndex::PRIMARY),
                            unchosen: None,
                            chooser: PlayerRefDef::EffectController,
                            candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                                // "A non-Avatar creature card or a planeswalker card." The Overlord itself
                                // is an Avatar, which is what the exclusion is there for: it cannot buy
                                // itself back.
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::All(&[
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Avatar")),
                                    ]),
                                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                                ]),
                                &[ZoneKind::Graveyard],
                                PlayerSetDef::Related(PlayerRelation::You),
                            )),
                            exclude: None,
                            minimum: 0,
                            maximum: 1,
                            visibility: ChoiceVisibilityDef::Public,
                            then: &EffectDef::MoveToZone {
                                object: EffectRecipientDef::objects(ObjectSetDef::Binding(ObjectSetBindingIndex::PRIMARY)),
                                zone: ZoneKind::Hand,
                                placement: ZonePlacement::Top,
                            },
                        }),
                    ]),
                ),
            ]),
    );

// DSK 136 — Fear of Missing Out
pub(in crate::card::sets) static FEAR_OF_MISSING_OUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d48aaff-46ab-411b-9456-171d4709f951"),
    "Fear of Missing Out",
    CardArt::new("9d48aaff-46ab-411b-9456-171d4709f951", "John Stanko"),
    CardSet::DuskmournHouseOfHorror,
    // Two mana for a body that fills its own graveyard on the way in and
    // then, once the graveyard is deep enough, hands the whole team a second
    // attack.
    CardRules::new_enchantment_creature(mana_cost!("{1}{R}"), &["Nightmare"], 2, 3).with_abilities(
        &[
            abilities::enters_trigger(
                "When this creature enters, discard a card, then draw a card.",
                EffectDef::Sequence(&[
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
            AbilityDef::triggered_if_with_targets(
                "Delirium — Whenever this creature attacks for the first time each turn, if there \
                 are four or more card types among cards in your graveyard, untap target \
                 creature. After this phase, there is an additional combat phase.",
                TriggerEventDef::attacks_first_time_this_turn(ObjectPredicateDef::Source),
                &// Delirium: four or more card types among the cards in your graveyard,
                    // counted as the trigger is placed and again as it resolves. The discard
                    // his own arrival asks for is often what turns it on.
                    TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                        comparison: ComparisonDef::GreaterOrEqual,
                        right: ValueDef::Constant(4),
                    }),
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                // Untapping is what makes the extra combat worth having: the creature that
                // just attacked can attack again.
                EffectDef::Sequence(&[
                    EffectDef::Untap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::ScheduleTurnPhases(&[TurnPhaseDef::Combat]),
                ]),
            ),
        ],
    ),
);

// DSK 143 — Leyline of Resonance
// Audit: partial — The opening-hand action is declarative; its cast trigger needs an exact single-controlled-creature targeting shape.
pub(in crate::card::sets) static LEYLINE_OF_RESONANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("92c5f0e3-345a-40a8-9cda-565a62156692"),
    "Leyline of Resonance",
    CardArt::new("92c5f0e3-345a-40a8-9cda-565a62156692", "Sergey Glushakov"),
    CardSet::DuskmournHouseOfHorror,
    CardRules::new_enchantment(mana_cost!("{2}{R}{R}")).with_abilities(&[
        abilities::begin_game_on_battlefield("If this card is in your opening hand, you may begin the game with it on the battlefield."),
        AbilityDef::not_implemented("Whenever you cast an instant or sorcery spell that targets only a single creature you control, copy that spell. You may choose new targets for the copy.", "Needs a spell-cast matcher that verifies the complete target set contains only one controlled creature."),
    ]),
);

// DSK 178 — Flesh Burrower
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FLESH_BURROWER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("60499c90-a512-4abb-98eb-0735a7138421"),
    "Flesh Burrower",
    crate::card::CardArt::new("60499c90-a512-4abb-98eb-0735a7138421", "Maxime Minard"),
    crate::card::CardSet::DuskmournHouseOfHorror,
    crate::card::CardRules::unsupported(),
);

// DSK 188 — Leyline of Mutation
// Audit: partial — The opening-hand action is declarative; its five-color alternative cost must be offered for every spell.
pub(in crate::card::sets) static LEYLINE_OF_MUTATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2359b670-41f0-4ec7-8db9-3f87f7577bc3"),
    "Leyline of Mutation",
    CardArt::new("2359b670-41f0-4ec7-8db9-3f87f7577bc3", "Sergey Glushakov"),
    CardSet::DuskmournHouseOfHorror,
    CardRules::new_enchantment(mana_cost!("{2}{G}{G}")).with_abilities(&[
        abilities::begin_game_on_battlefield("If this card is in your opening hand, you may begin the game with it on the battlefield."),
        AbilityDef::not_implemented("You may pay {W}{U}{B}{R}{G} rather than pay the mana cost for spells you cast.", "Needs a battlefield-granted alternative casting cost over every spell."),
    ]),
);

// DSK 191 — Monstrous Emergence
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MONSTROUS_EMERGENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b999eb47-b842-47f1-be91-c79fc46e1896"),
    "Monstrous Emergence",
    crate::card::CardArt::new("b999eb47-b842-47f1-be91-c79fc46e1896", "Loïc Canavaggia"),
    crate::card::CardSet::DuskmournHouseOfHorror,
    crate::card::CardRules::unsupported(),
);

// DSK 205 — Walk-In Closet // Forgotten Cellar
const WALK_IN_CLOSET_DOOR: AbilityDef = AbilityDef::static_ability(
    "You may play lands from your graveyard.",
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::Controller,
        effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromGraveyard(
            // Crucible of Worlds' line, and the door of the pair that asks nothing of
            // you afterwards: a permission in the same vocabulary a prohibition uses.
            GraveyardPlayPermissionDef::unlimited(PlayRestrictionDef::new(
                PlayActionMatcherDef::PlayLand,
                ObjectPredicateDef::HasType(CardType::Land),
            )),
        )),
    },
);

const FORGOTTEN_CELLAR_DOOR: AbilityDef = AbilityDef::triggered(
    "When you unlock this door, you may cast spells from your graveyard this turn, and if a card \
     would be put into your graveyard from anywhere this turn, exile it instead.",
    TriggerEventDef::DoorUnlocked,
    // Both clauses last the turn, and they are one sentence because they need
    // each other: casting from a graveyard that keeps filling up is a worse
    // deal than casting from one that empties into exile behind you.
    //
    // The permission is aimed at the player, where a permission belongs. The
    // replacement is granted to the Room, which is the one place this differs
    // from the printed card: a Room that leaves the battlefield before the turn
    // ends stops exiling, where the printed effect would go on without it.
    EffectDef::Sequence(&[
        EffectDef::Apply {
            recipient: EffectRecipientDef::Controller,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromGraveyard(
                // The other half's permission, for the turn it lasts. Spells rather than
                // lands: the cellar opens onto everything the closet does not.
                GraveyardPlayPermissionDef::unlimited(PlayRestrictionDef::new(
                    PlayActionMatcherDef::CastSpell,
                    ObjectPredicateDef::Any,
                )),
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            // "A card", not "a card or token": a token put into a graveyard goes there
            // and ceases to exist as it always would, which is what keeps this from
            // quietly turning every token into an exiled one.
            effect: AppliedEffectDef::add_ability(&AbilityDef::replacement_for(
                "If a card would be put into your graveyard from anywhere, exile it instead.",
                ReplacementEventDef::AnyObjectWouldMove {
                    to: ZoneKind::Graveyard,
                    owner: PlayerRelation::You,
                    tokens: false,
                },
                ReplacementEffectDef::MoveToZone(ZoneKind::Exile),
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    ]),
);

const fn walk_in_closet_rules() -> CardRules {
    CardRules::new_enchantment(mana_cost!("{2}{G}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[WALK_IN_CLOSET_DOOR])
}

fn walk_in_closet_composition() -> CardComposition {
    CardComposition::room(
        "Walk-In Closet // Forgotten Cellar",
        "Walk-In Closet",
        walk_in_closet_rules(),
        "Forgotten Cellar",
        CardRules::new_enchantment(mana_cost!("{3}{G}{G}"))
            .with_subtypes(&["Room"])
            .with_abilities(&[FORGOTTEN_CELLAR_DOOR]),
        CardRules::new_enchantment(mana_cost!("{5}{G}{G}{G}"))
            .with_subtypes(&["Room"])
            // What the permanent is once both doors are open: the two text boxes at
            // once, for the two costs added up (CR 714.2b).
            .with_abilities(&[WALK_IN_CLOSET_DOOR, FORGOTTEN_CELLAR_DOOR]),
    )
}

pub(in crate::card::sets) static WALK_IN_CLOSET_FORGOTTEN_CELLAR: CardRecord =
    CardRecord::new_with_legacy_id(
        2305,
        "Walk-In Closet // Forgotten Cellar",
        CardArt::new("0adcd4e5-d542-4293-8774-ace2305ef820", "Mikl\u{f3}s Ligeti"),
        CardSet::DuskmournHouseOfHorror,
        // Three mana for Crucible of Worlds, and five more whenever the game
        // gives you nothing better to do -- which is what a Room is for: a card
        // that is cheap early and still has something left late.
        walk_in_closet_rules(),
    )
    .with_composition(walk_in_closet_composition);

// DSK 220 — Kaito, Bane of Nightmares
pub(in crate::card::sets) static KAITO_BANE_OF_NIGHTMARES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("55a14f30-4ff9-4472-90a6-c3139f1c18e5"),
    "Kaito, Bane of Nightmares",
    CardArt::new("55a14f30-4ff9-4472-90a6-c3139f1c18e5", "Joshua Raphael"),
    CardSet::DuskmournHouseOfHorror,
    // Four mana, or a ninjutsu out of a connected attacker: he arrives
    // attacking, is a hexproof 3/4 for as long as it is your turn, and is a
    // planeswalker again the moment it is not.
    CardRules::new_planeswalker(mana_cost!("{2}{U}{B}"), &["Kaito"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "Ninjutsu {1}{U}{B} ({1}{U}{B}, Return an unblocked attacker you control to hand: Put \
                 this card onto the battlefield from your hand tapped and attacking.)",
                &[
                    AbilityCostDef::Mana(mana_cost!("{1}{U}{B}")),
                    AbilityCostDef::ReturnUnblockedAttackerToHand,
                ],
                EffectDef::PutSourceOntoBattlefieldAttacking,
            )
            .with_source_zones(&[ZoneKind::Hand])
            .with_activation_timing(ActivationTimingDef::AfterAttackersDeclared),
            AbilityDef::static_ability(
                "During your turn, as long as Kaito has one or more loyalty counters on him, he's a 3/4 \
                 Ninja creature and has hexproof.",
                EffectDef::IfCondition {
                    // He is a creature only while it is your turn and only while he still has
                    // loyalty: the pair of conditions is what keeps him from being a creature
                    // anyone can answer on their own turn.
                    condition: &TriggerConditionDef::All(&[
                        TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                        TriggerConditionDef::SourceCounters {
                            kind: CounterKind::Loyalty,
                            comparison: ComparisonDef::GreaterOrEqual,
                            amount: 1,
                        },
                    ]),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(SetOperationDef::Add(
                                CardTypeSet::single(CardType::Creature),
                            ))),
                            AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Ninja"])),
                            AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(3), ValueDef::Constant(4)),
                            AppliedEffectDef::add_ability(&abilities::hexproof()),
                        ]),
                    },
                },
            ),
            AbilityDef::activated(
                "+1: You get an emblem with \"Ninjas you control get +1/+1.\"",
                &[AbilityCostDef::Loyalty(1)],
                EffectDef::CreateEmblem {
                    emblem: EmblemCharacteristics::new("Kaito, Bane of Nightmares emblem", &[AbilityDef::static_ability(
                            "Ninjas you control get +1/+1.",
                            EffectDef::StaticApply {
                                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                                    ObjectPredicateDef::Subtype("Ninja"),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                ))),
                                effect: AppliedEffectDef::modify_power_toughness(
                                    ValueDef::Constant(1),
                                    ValueDef::Constant(1),
                                ),
                            },
                        )]),
                },
            ),
            AbilityDef::activated(
                "0: Surveil 2. Then draw a card for each opponent who lost life this turn.",
                &[AbilityCostDef::Loyalty(0)],
                EffectDef::Sequence(&[
                    abilities::surveil(ValueDef::Constant(2)),
                    // "A card for each opponent who lost life this turn" is a count of players
                    // rather than of life, which in a two-player game is one card or none.
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::OpponentsWhoLostLifeThisTurn,
                    },
                ]),
            ),
            AbilityDef::activated_with_targets(
                "\u{2212}2: Tap target creature. Put two stun counters on it.",
                &[AbilityCostDef::Loyalty(-2)],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Sequence(&[
                    EffectDef::Tap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::Stun,
                        amount: ValueDef::Constant(2),
                    },
                ]),
            ),
        ]),
);

// DSK 248 — Ghost Vacuum
pub(in crate::card::sets) static GHOST_VACUUM: CardRecord = CardRecord::new_with_legacy_id(
    2289,
    "Ghost Vacuum",
    CardArt::new("8ac39c01-127f-4471-bc74-11a90c48e306", "David Szabo"),
    CardSet::DuskmournHouseOfHorror,
    // One mana of graveyard hate that the deck playing it can cash in for a
    // board, which is what keeps it in a cube where dead cards are the cost
    // of every sideboard card.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Exile target card from a graveyard.",
            &[AbilityCostDef::TapSource],
            // Either graveyard: the Vacuum is as happy eating your own escape targets
            // as theirs, and the second ability does not care whose card it was.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            )],
            // Linked rather than exiled outright: the second ability names what
            // this one took, and by then nothing else could tell those cards
            // apart from anything else in exile.
            EffectDef::ExileLinkedToSource {
                until_source_leaves: false,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                face_down: false,
                then: None,
            },
        ),
        AbilityDef::activated(
            "{6}, {T}, Sacrifice this artifact: Put each creature card exiled with this artifact onto \
             the battlefield under your control with a flying counter on it. Each of them is a 1/1 \
             Spirit in addition to its other types. Activate only as a sorcery.",
            &[
                AbilityCostDef::Mana(mana_cost!("{6}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            EffectDef::WithZoneMoveResult {
                effect: &EffectDef::ReturnLinkedExiles {
                    // Only the creature cards: a Brainstorm the Vacuum ate stays
                    // exiled, still linked to a source that is no longer there.
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zone: ZoneKind::Battlefield,
                    grant: None,
                    counters: Some(TokenCountersDef {
                        kind: CounterKind::Flying,
                        amount: ValueDef::Constant(1),
                    }),
                    transformed: false,
                    controller: Some(PlayerRelation::You),
                },
                binding: ObjectSetBindingIndex::PRIMARY,
                then: &EffectDef::Apply {
                    recipient: EffectRecipientDef::binding_zone_change_successors(
                        ObjectSetBindingIndex::PRIMARY,
                    ),
                    // "Each of them is a 1/1 Spirit in addition to its other types." Adding the
                    // subtype rather than setting it is what "in addition" means: a Griselbrand
                    // that comes back this way is a Demon Spirit, and a 1/1 one.
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::Characteristic(
                            CharacteristicOperationDef::CreatureTypes(SetOperationDef::Add(
                                CreatureTypeSetDef::named(&["Spirit"]),
                            )),
                        ),
                    ]),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// DSK 249 — Glimmerlight
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GLIMMERLIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1071691c-5c65-42d4-ac96-d302185ca678"),
    "Glimmerlight",
    crate::card::CardArt::new("1071691c-5c65-42d4-ac96-d302185ca678", "Wero Gallo"),
    crate::card::CardSet::DuskmournHouseOfHorror,
    crate::card::CardRules::unsupported(),
);

// DSK 256 — Blazemire Verge
pub(in crate::card::sets) static BLAZEMIRE_VERGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d151c8e2-d715-470d-868a-f45191db9fa0"),
    "Blazemire Verge",
    CardArt::new("d151c8e2-d715-470d-868a-f45191db9fa0", "Andrew Mar"),
    CardSet::DuskmournHouseOfHorror,
    // Untapped and free either way: the black is unconditional, and the red
    // is what the rest of the mana base is for.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {B}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
        AbilityDef::activated_mana_if(
            "{T}: Add {R}. Activate only if you control a Swamp or a Mountain.",
            &[AbilityCostDef::TapSource],
            &TriggerConditionDef::ObjectCount {
                // The same condition in this cycle's Rakdos colours. Either type answers
                // it, so a Badlands is both halves at once.
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[
                        BasicLandType::Swamp,
                        BasicLandType::Mountain,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
    ]),
);

// DSK 270 — Thornspire Verge
pub(in crate::card::sets) static THORNSPIRE_VERGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7e1cdc03-6faa-4138-9a52-caafbe34fb59"),
    "Thornspire Verge",
    CardArt::new(
        "7e1cdc03-6faa-4138-9a52-caafbe34fb59",
        "Kasia 'Kafis' Zielińska",
    ),
    CardSet::DuskmournHouseOfHorror,
    // Untapped and free either way: the red is unconditional, and the green
    // is what the rest of the mana base is for.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
        AbilityDef::activated_mana_if(
            "{T}: Add {G}. Activate only if you control a Mountain or a Forest.",
            &[AbilityCostDef::TapSource],
            &TriggerConditionDef::ObjectCount {
                // The verge condition in this cycle's Gruul colours. Either type answers
                // it, so a Taiga is both halves at once.
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[
                        BasicLandType::Mountain,
                        BasicLandType::Forest,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ),
    ]),
);

// DSK 295 — Clockwork Percussionist
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CLOCKWORK_PERCUSSIONIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("10986e5a-9fc6-41e2-8352-289328245171"),
    "Clockwork Percussionist",
    crate::card::CardArt::new("e44340c7-d3bb-4cf9-a105-ebbf6ce3ace1", "Eric Wilkerson"),
    crate::card::CardSet::DuskmournHouseOfHorror,
    crate::card::CardRules::unsupported(),
);

// DSK 314 — Chainsaw
pub(in crate::card::sets) static CHAINSAW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c8d0f4e-6b1e-4444-8851-adf857273964"),
    "Chainsaw",
    CardArt::new("1c8d0f4e-6b1e-4444-8851-adf857273964", "Alexis Ziritt"),
    CardSet::DuskmournHouseOfHorror,
    // Two mana that shoots something on the way in and then grows for the
    // rest of the game, on a board where creatures keep dying anyway.
    CardRules::new_artifact(mana_cost!("{1}{R}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When this Equipment enters, it deals 3 damage to up to one target creature.",
                // "Up to one target creature": the Equipment arrives whether or not there
                // is anything worth shooting.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(3),
                },
            ),
            // One counter for the batch rather than one per creature, which is what
            // "one or more" means: a board wipe revs it once.
            AbilityDef::triggered(
                "Whenever one or more creatures die, put a rev counter on this Equipment.",
                TriggerEventDef::ObjectsDied {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("rev"),
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +X/+0, where X is the number of rev counters on this Equipment.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountersOnSource(CounterKind::named("rev")),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{3}"))], "Equip {3}"),
        ]),
);

// DSK 316 — Fear of Missing Out (alternate printing)

// DSK 329 — Blazemire Verge (alternate printing)

// DSK 348 — Screaming Nemesis
pub(in crate::card::sets) static SCREAMING_NEMESIS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ad3f4c72-ff6e-4d7f-8eb8-45a0a9605fc0"),
    "Screaming Nemesis",
    CardArt::new("ad3f4c72-ff6e-4d7f-8eb8-45a0a9605fc0", "Inkognit"),
    CardSet::DuskmournHouseOfHorror,
    // Three mana that attacks into anything: blocking it, burning it, or
    // fighting it all send the damage somewhere else, and a player who takes
    // it is out of lifegain for good.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Spirit"], 3, 3).with_abilities(&[
        abilities::haste(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature is dealt damage, it deals that much damage to any other \
             target. If a player is dealt damage this way, they can't gain life for the rest of \
             the game.",
            TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                kind: DamageKindDef::Any,
                source: DamageSourceMatcherDef::Any,
                recipient: DamageRecipientMatcherDef::MatchingObject(ObjectPredicateDef::Source),
            }),
            // "Any other target": everything an ordinary any-target slot offers, minus
            // the Spirit itself. Without the exclusion it could answer its own trigger
            // and hit itself, which would trigger it again.
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget).excluding_source()],
            // The damage and the rider are one effect rather than a sequence, because
            // the rider is about what actually took the damage: prevented damage stops
            // nothing from being gained.
            EffectDef::DealDamageAndApply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::TriggerEventAmount,
                applied: AppliedEffectDef::Rule(AppliedRuleDef::CannotGainLife),
                duration: ResolvedEffectDurationDef::Permanent,
            },
        ),
    ]),
);

// DSK 372 — Leyline of Transformation
// Audit: partial — The opening-hand action is declarative; the as-enters creature-type choice and all-zone type layer remain unsupported.
pub(in crate::card::sets) static LEYLINE_OF_TRANSFORMATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd545d86-9a3e-4e4f-b0fe-9363a85b9290"),
    "Leyline of Transformation",
    CardArt::new("fd545d86-9a3e-4e4f-b0fe-9363a85b9290", "Sergey Glushakov"),
    CardSet::DuskmournHouseOfHorror,
    CardRules::new_enchantment(mana_cost!("{2}{U}{U}")).with_abilities(&[
        abilities::begin_game_on_battlefield("If this card is in your opening hand, you may begin the game with it on the battlefield."),
        AbilityDef::not_implemented("As this enchantment enters, choose a creature type.\nCreatures you control are the chosen type in addition to their other types. The same is true for creature spells you control and creature cards you own that aren't on the battlefield.", "Needs an as-enters creature-type choice whose result feeds type layers in every relevant zone."),
    ]),
);

// DSK 387 — Overlord of the Mistmoors
pub(in crate::card::sets) static OVERLORD_OF_THE_MISTMOORS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1951ed76-16a1-4639-b824-08dfc3d6d098"),
    "Overlord of the Mistmoors",
    CardArt::new("1951ed76-16a1-4639-b824-08dfc3d6d098", "Takeuchi Moto"),
    CardSet::DuskmournHouseOfHorror,
    // Four mana for four power of fliers now and a 6/6 four turns later,
    // which is the whole appeal: the enchantment does the work while the
    // body waits.
    CardRules::new_enchantment_creature(mana_cost!("{5}{W}{W}"), &["Avatar", "Horror"], 6, 6)
        .with_abilities(&[
            AbilityDef::alternative_cast(
                mana_cost!("{2}{W}{W}"),
                AlternativeCastKindDef::Impending,
                Some(
                    "Impending 4—{2}{W}{W} (If you cast this spell for its impending cost, it enters \
                     with four time counters and isn't a creature until the last is removed. At the \
                     beginning of your end step, remove a time counter from it.)",
                ),
                EffectDef::None,
            ),
            AbilityDef::as_enters_if(
                "If you cast this spell for its impending cost, it enters with four time counters.",
                ReplacementConditionDef::SourceCastWith(AlternativeCastKindDef::Impending),
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCounters {
                        kind: CounterKind::named("time"),
                        amount: 4,
                    },
                ),
            ),
            AbilityDef::triggered(
                "At the beginning of your end step, remove a time counter from this permanent.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::You,
                },
                EffectDef::RemoveCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("time"),
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::triggered(
                "Whenever this permanent enters or attacks, create two 2/1 white Insect creature tokens \
                 with flying.",
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                ]),
                // Two at a time, which is one instruction rather than two: what watches
                // for tokens being created sees one batch of two.
                EffectDef::create_creature_token(&["Insect"], &[ManaColor::White], 2, 1)
                        .with_abilities(&[abilities::flying()])
                        .with_amount(2),
            ),
        ]),
);

// DSK 409 — Kaito, Bane of Nightmares (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ENDURING_INNOCENCE,
    &LEYLINE_OF_HOPE,
    &TRAPPED_IN_THE_SCREEN,
    &ABHORRENT_OCULUS,
    &UNABLE_TO_SCREAM,
    &OVERLORD_OF_THE_BALEMURK,
    &FEAR_OF_MISSING_OUT,
    &LEYLINE_OF_RESONANCE,
    &FLESH_BURROWER,
    &LEYLINE_OF_MUTATION,
    &MONSTROUS_EMERGENCE,
    &WALK_IN_CLOSET_FORGOTTEN_CELLAR,
    &KAITO_BANE_OF_NIGHTMARES,
    &GHOST_VACUUM,
    &GLIMMERLIGHT,
    &BLAZEMIRE_VERGE,
    &THORNSPIRE_VERGE,
    &CLOCKWORK_PERCUSSIONIST,
    &CHAINSAW,
    &SCREAMING_NEMESIS,
    &LEYLINE_OF_TRANSFORMATION,
    &OVERLORD_OF_THE_MISTMOORS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&FEAR_OF_MISSING_OUT, 1), // DSK 316
    PrintingRecord::alternate(&BLAZEMIRE_VERGE, 1),     // DSK 329
    PrintingRecord::alternate(&KAITO_BANE_OF_NIGHTMARES, 1), // DSK 409
];
