//! March of the Machine cards cataloged for the Vintage Cube pool.

use crate::card::AddManaEffectDef;
use crate::card::AggregateOperationDef;
use crate::card::BattlefieldArrivalDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseCardsFromCollectionDef;
use crate::card::ChooseDef;
use crate::card::CollectionInspectionDef;
use crate::card::ComparisonDef;
use crate::card::ConditionalStaticEffectDef;
use crate::card::FreePlayDef;
use crate::card::FreePlayDurationDef;
use crate::card::MoveObjectsDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetCountConditionDef;
use crate::card::ObjectSetPredicateDef;
use crate::card::ObjectValueAggregateDef;
use crate::card::ObjectValueDef;
use crate::card::PlayerRefDef;
use crate::card::RandomizeObjectOrderDef;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
use crate::card::StaticApplyDef;
use crate::card::SubtypeDef;
use crate::card::TokenStatsDef;
use crate::card::TriggerConditionDef;
use crate::card::ZoneMoveCauseDef;
use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BasicLandType;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::DrawEventMatcherDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ExiledCastPermissionDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenCountersDef;
use crate::card::TokenDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::tokens;
use crate::ids::ParentBinding;
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "MOM",
    slug: "march-of-the-machine",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const INCUBATOR_TOKEN: TokenCharacteristics = tokens::incubator().with_art(CardArt::new(
    "2c5ed737-657b-43bf-b222-941da7579a4a",
    "Johann Bodin",
));

// MOM 3 — Alabaster Host Intercessor
pub(in crate::card::sets) static ALABASTER_HOST_INTERCESSOR: CardRecord = CardRecord::new(
    "Alabaster Host Intercessor",
    "165357cc-ec74-490f-aec3-7048bb43c8f9",
    "Konstantin Porubov",
// Six mana for removal on a body, or two for a land: the cycling half is
    // what keeps it from being a dead card in the early game.
    CardRules::new_creature(mana_cost!("{5}{W}"), &["Phyrexian", "Samurai"], 3, 4).with_abilities(
        &[
            abilities::enters_trigger_with_targets(
                "When this creature enters, exile target creature an opponent controls until this creature leaves the battlefield.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                    ]),
                )],
                // The modern one-clause "until" wording, so the return is
                // installed by this same resolution.
                abilities::exile_until_source_leaves(EffectRecipientDef::Target(
                    TargetIndex::PRIMARY,
                )),
            ),
            abilities::typecycling!(
                "Plainscycling {2} ({2}, Discard this card: Search your library for a Plains card, reveal it, put it into your hand, then shuffle.)",
                &[CostDef::Mana(mana_cost!("{2}"))],
                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
            ),
        ],
    ),
);

// MOM 13 — Elspeth's Smite
pub(in crate::card::sets) static ELSPETH_S_SMITE: CardRecord = CardRecord::new(
    "Elspeth's Smite",
    "f03a480f-de67-4611-9db7-c0c3d020f597",
    "Livia Prima",
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Elspeth's Smite deals 3 damage to target attacking or \
         blocking creature. If that creature would die this turn, \
         exile it instead.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Attacking,
                    ObjectPredicateDef::Blocking,
                ]),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::ExileInsteadOfDying),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )]),
);

// MOM 31 — Phyrexian Censor
// Audit: unsupported — PlayRestrictionDef::after_spells_cast counts every prior spell, not only prior non-Phyrexian spells; the required filtered cast-history quota is absent.
pub(in crate::card::sets) static PHYREXIAN_CENSOR_31: CardRecord = CardRecord::new(
    "Phyrexian Censor",
    "150e17b1-b9fd-4ec4-b305-19596fed14d1",
    "Alexey Kruglov",
    crate::card::CardRules::unsupported(),
);

// MOM 40 — Sunfall
pub(in crate::card::sets) static SUNFALL: CardRecord = CardRecord::new(
    "Sunfall",
    "32e29c7d-ed4b-4eff-b3c2-d99e5b63ef8d",
    "Kasia 'Kafis' Zielińska",
    // A wrath that exiles rather than destroys, and hands the caster the
    // biggest thing on the empty board it just made.
    CardRules::new_sorcery(mana_cost!("{3}{W}{W}")).with_ability(AbilityDef::spell(
        "Exile all creatures. Incubate X, where X is the number of creatures exiled this way. \
         (Create an Incubator token with X +1/+1 counters on it and \"{2}: Transform this \
         token.\" It transforms into a 0/0 Phyrexian artifact creature.)",
        abilities::bind_objects_then(
            // Everyone's, which is what "all creatures" means.
            crate::card::ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Query(
                ObjectQueryDef::new(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                ),
            )),
            // The creatures are bound before they move, because "X, where X is the
            // number of creatures exiled this way" asks about a set the board no longer
            // holds by the time the token is made.
            &EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
                // Incubate X. One token however large X is, and X of zero still makes
                // one: the keyword creates the token unconditionally.
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(INCUBATOR_TOKEN)).with_counters(
                        TokenCountersDef {
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::BoundObjectCount(ParentBinding),
                        },
                    ),
                ),
            ]),
        ),
    )),
);

// MOM 41 — Surge of Salvation
// Audit: unsupported — Resolving prevention shields cannot retain a dynamic creature-recipient predicate, so they cannot protect creatures that enter or change controller later this turn.
pub(in crate::card::sets) static SURGE_OF_SALVATION_41: CardRecord = CardRecord::new(
    "Surge of Salvation",
    "41d25ee5-0348-4206-bb6a-ccb0a599ac87",
    "Dominik Mayer",
    CardRules::unsupported(),
);

// MOM 58 — Faerie Mastermind
pub(in crate::card::sets) static FAERIE_MASTERMIND: CardRecord = CardRecord::new(
    "Faerie Mastermind",
    "52d3005f-a1c7-4ef5-911f-ccc0752f4181",
    "Joshua Raphael",
    // A two-mana flash flier that is never a dead card: it taxes every
    // cantrip the other deck was going to cast anyway, and turns into a
    // draw engine once there is nothing else to spend mana on.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Faerie", "Rogue"], 2, 1).with_abilities(&[
        abilities::flash(),
        abilities::flying(),
        // The ordinal is the whole clause: their first card each turn is the one
        // the rules hand them, so this catches the extra one and nothing else.
        AbilityDef::triggered(
            "Whenever an opponent draws their second card each turn, you draw a card.",
            TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(
                PlayerRelation::Opponent,
                2,
            )),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        // Symmetrical on purpose: with the trigger above out, the copy they draw
        // is the one that draws you another.
        AbilityDef::activated(
            "{3}{U}: Each player draws a card.",
            &[CostDef::Mana(mana_cost!("{3}{U}"))],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::players(PlayerSetDef::All),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// MOM 66 — Meeting of Minds
pub(in crate::card::sets) static MEETING_OF_MINDS: CardRecord = CardRecord::new(
    "Meeting of Minds",
    "508b8650-c283-4e54-abdc-32ec2fb1ee34",
    "Milivoj Ćeran",
    // Convoke is doing all the work: a board that has already committed
    // draws two for free at instant speed, and pays four otherwise.
    CardRules::new_instant(mana_cost!("{3}{U}")).with_abilities(&[
        abilities::convoke(),
        AbilityDef::spell(
            "Draw two cards.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// MOM 73 — Preening Champion
pub(in crate::card::sets) static PREENING_CHAMPION: CardRecord = CardRecord::new(
    "Preening Champion",
    "44178ece-af31-4a94-88bc-c9ce43bb4573",
    "Alix Branwyn",
    // Three mana for three power across two bodies, one of them in the air,
    // which is the rate a limited deck plays it at.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Bird", "Knight"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 blue and red Elemental creature token.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(
                    &["Elemental"],
                    &[ManaColor::Blue, ManaColor::Red],
                    1,
                    1,
                ),
            ))),
        ),
    ]),
);

// MOM 80 — Temporal Cleansing
// Audit: unsupported — Needs a library insertion at the second position from the top; ZonePlacement supports top and bottom but not an indexed position.
pub(in crate::card::sets) static TEMPORAL_CLEANSING: CardRecord = CardRecord::new(
    "Temporal Cleansing",
    "6e67031a-8216-4c66-b6fb-6628bd02d279",
    "Dominik Mayer",
    CardRules::unsupported(),
);

// MOM 98 — Corrupted Conviction
pub(in crate::card::sets) static CORRUPTED_CONVICTION: CardRecord = CardRecord::new(
    "Corrupted Conviction",
    "ce133ad5-8748-4a3d-ae8c-7b2a5938927d",
    "Joseph Weston",
    CardRules::new_instant(mana_cost!("{B}")).with_abilities(&[AbilityDef::spell(
        "Draw two cards.",
        abilities::draw_cards(ValueDef::Constant(2)),
    )
    .with_spell_additional_cost(&CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(
        CardType::Creature,
    )))]),
);

// MOM 173 — Wrenn's Resolve
// Audit: unsupported — Needs an exile-play permission lasting until the end of your next turn. ExilePlayDurationDef offers ThisTurn, UntilYourNextEndStep and WhileExiled, and FreePlayDurationDef only WhileResolving and UntilEndOfTurn; none of them reaches the end of the following turn, and UntilYourNextEndStep expires a turn early when the spell is cast on your own turn.
pub(in crate::card::sets) static WRENN_S_RESOLVE: CardRecord = CardRecord::new(
    "Wrenn's Resolve",
    "9a47999c-12d5-4e1a-a9c1-40a1757007f1",
    "Viko Menezes",
    crate::card::CardRules::unsupported(),
);

// MOM 190 — Invasion of Ikoria // Zilortha, Apex of Ikoria
// Audit: unsupported — CardType has no Battle/Siege or defense/protector model, so the front face's entry, combat, defeat, and transformed cast lifecycle cannot be represented.
pub(in crate::card::sets) static INVASION_OF_IKORIA_ZILORTHA_APEX_OF_IKORIA_190: CardRecord =
    CardRecord::new(
        "Invasion of Ikoria // Zilortha, Apex of Ikoria",
        "5d59c8f2-f6af-40a6-8dfe-8cc45bf231ce",
        "Antonio José Manzanedo",
        crate::card::CardRules::unsupported(),
    );

// MOM 215 — Wary Thespian
pub(in crate::card::sets) static WARY_THESPIAN: CardRecord = CardRecord::new(
    "Wary Thespian",
    "675b29bf-0b64-410f-9a92-c88e5615c27f",
    "Billy Christian",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Cat", "Druid"], 3, 1).with_abilities(&[
        AbilityDef::triggered(
            "When this creature enters or dies, surveil 1. (Look at the \
             top card of your library. You may put it into your \
             graveyard.)",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
            ]),
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

// MOM 221 — Djeru and Hazoret
pub(in crate::card::sets) static DJERU_AND_HAZORET_221: CardRecord = CardRecord::new(
    "Djeru and Hazoret",
    "1db1ae7b-ed48-409f-8d50-07c7e8c6c128",
    "Matt Stewart",
    CardRules::new_creature(mana_cost!("{2}{R}{R}{W}"), &["Human", "God"], 5, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "As long as you have one or fewer cards in hand, this creature has vigilance and haste.",
                EffectDef::ConditionalStatic(ConditionalStaticEffectDef {
                    condition: ObjectSetCountConditionDef {
                        objects: &ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Hand],
                            PlayerRelation::You,
                        )),
                        predicate: ObjectSetPredicateDef {
                            filter: None,
                            comparison: ComparisonDef::LessOrEqual,
                            amount: 1,
                        },
                    },
                    then: StaticApplyDef {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::add_ability(&abilities::vigilance()),
                            AppliedEffectDef::add_ability(&abilities::haste()),
                        ]),
                    },
                }),
            ),
            AbilityDef::triggered(
                "Whenever this creature attacks, look at the top six cards of your library. You may exile a legendary creature card from among them. Put the rest on the bottom of your library in a random order. Until end of turn, you may cast the exiled card without paying its mana cost.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                    source: ObjectCollectionSourceDef::TopCards {
                        player: crate::card::PlayerRefDef::EffectController,
                        count: ValueDef::Constant(6),
                    },
                    actor: crate::card::PlayerRefDef::EffectController,
                    inspection: CollectionInspectionDef::Look,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    minimum: 0,
                    maximum: 1,
                    chosen: Binding!("djeru_and_hazoret_exiled"),
                    remainder: Binding!("djeru_and_hazoret_remainder"),
                    then: &EffectDef::Sequence(&[
                        EffectDef::MoveObjects(MoveObjectsDef {
                            input: ObjectSetDef::Binding(Binding!("djeru_and_hazoret_exiled")),
                            from: Some(ZoneKind::Library),
                            zone: ZoneKind::Exile,
                            placement: ZonePlacement::Top,
                            moved: Some(Binding!("djeru_and_hazoret_exiled_successors")),
                            then: &EffectDef::None,
                        }),
                        EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef {
                            input: ObjectSetDef::Binding(Binding!("djeru_and_hazoret_remainder")),
                            randomized: ParentBinding,
                            then: &EffectDef::MoveObjects(MoveObjectsDef {
                                input: ObjectSetDef::Binding(ParentBinding),
                                from: Some(ZoneKind::Library),
                                zone: ZoneKind::Library,
                                placement: ZonePlacement::Bottom,
                                moved: None,
                                then: &EffectDef::None,
                            }),
                        }),
                        EffectDef::MayPlayWithoutPaying(FreePlayDef {
                            objects: ObjectSetDef::Binding(Binding!("djeru_and_hazoret_exiled_successors")),
                            duration: FreePlayDurationDef::UntilEndOfTurn,
                            mandatory: false,
                            grants_haste: false,
                        }),
                    ]),
                }),
            ),
        ]),
);

// MOM 225 — Ghalta and Mavren
pub(in crate::card::sets) static GHALTA_AND_MAVREN_225: CardRecord = CardRecord::new(
    "Ghalta and Mavren",
    "a9ec900f-1e31-4440-a75a-20b256734d5b",
    "Zezhou Chen",
    CardRules::new_creature(mana_cost!("{3}{G}{G}{W}{W}"), &["Dinosaur", "Vampire"], 12, 12)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::trample(),
            AbilityDef::modal_triggered(
                "Whenever you attack, choose one —\n• Create a tapped and attacking X/X green Dinosaur creature token with trample, where X is the greatest power among other attacking creatures.\n• Create X 1/1 white Vampire creature tokens with lifelink, where X is the number of other attacking creatures.",
                TriggerEventDef::attack_declared(
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    1,
                    None,
                ),
                &[
                    AbilityDef::spell(
                        "Create a tapped and attacking X/X green Dinosaur creature token with trample, where X is the greatest power among other attacking creatures.",
                        EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::TokenCharacteristics::creature_with_stats(
                            &["Dinosaur"],
                            &[ManaColor::Green],
                            &TokenStatsDef {
                                power: ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                                    objects: ObjectSetDef::Query(ObjectQueryDef::matching(
                                        ObjectPredicateDef::All(&[
                                            ObjectPredicateDef::HasType(CardType::Creature),
                                            ObjectPredicateDef::Attacking,
                                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                        ]),
                                        &[ZoneKind::Battlefield],
                                        PlayerRelation::You,
                                    )),
                                    select: ObjectValueDef::Power,
                                    operation: AggregateOperationDef::Maximum,
                                }),
                                toughness: ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                                    objects: ObjectSetDef::Query(ObjectQueryDef::matching(
                                        ObjectPredicateDef::All(&[
                                            ObjectPredicateDef::HasType(CardType::Creature),
                                            ObjectPredicateDef::Attacking,
                                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                        ]),
                                        &[ZoneKind::Battlefield],
                                        PlayerRelation::You,
                                    )),
                                    select: ObjectValueDef::Power,
                                    operation: AggregateOperationDef::Maximum,
                                }),
                            },
                        ).with_abilities(&[abilities::trample()]))).entering_tapped().entering_attacking()),
                    ),
                    AbilityDef::spell(
                        "Create X 1/1 white Vampire creature tokens with lifelink, where X is the number of other attacking creatures.",
                        EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::TokenCharacteristics::creature(
                            &["Vampire"],
                            &[ManaColor::White],
                            1,
                            1,
                        ).with_abilities(&[abilities::lifelink()]))).with_count(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Attacking,
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )))),
                    ),
                ],
            ),
        ]),
);

// MOM 259 — Flywheel Racer
pub(in crate::card::sets) static FLYWHEEL_RACER_259: CardRecord = CardRecord::new(
    "Flywheel Racer",
    "c694485d-a753-4e55-929c-d8e4a53c7d08",
    "Joshua Cairos",
    CardRules::new_vehicle(mana_cost!("{2}"), 3, 3).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::activated_mana_if(
            "{T}: Add one mana of any color. Activate only if this permanent is a creature.",
            &[CostDef::TapSource],
            &TriggerConditionDef::SourceMatches {
                object: ObjectPredicateDef::HasType(CardType::Creature),
            },
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
        abilities::crew("Crew 1 (Tap any number of creatures you control with total power 1 or more: This Vehicle becomes an artifact creature until end of turn.)", 1),
    ]),
);

// MOM 298 — Etali, Primal Conqueror // Etali, Primal Sickness
pub(in crate::card::sets) static ETALI_PRIMAL_CONQUEROR: CardRecord = CardRecord::new_dfc(
    "Etali, Primal Conqueror // Etali, Primal Sickness",
    "3e97c609-3932-4428-96d4-1c97e61f0abb",
    "Yeong-Hao Han",
    // Seven mana that casts the two best cards on the table, and a back face
    // nobody in the cube ever pays for.
    &[
        (
            "Etali, Primal Conqueror",
            const {
                CardRules::new_creature(mana_cost!("{5}{R}{R}"), &["Elder", "Dinosaur"], 7, 7)
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&const { [
                    abilities::trample(),
                    abilities::enters_trigger(
                        "When this creature enters, each player exiles cards from the top of their library until \
                         they exile a nonland card. You may cast any number of spells from among the nonland \
                         cards exiled this way without paying their mana costs.",
                        // Both libraries, and the permission is always Etali's controller's: what
                        // their library turned up is yours to cast.
                        //
                        // The printed clause states no duration, which means the casting happens as
                        // the ability resolves: a card left uncast stays in exile uncastable rather
                        // than waiting for later in the turn. Every card it turned up is offered at
                        // once, so the order they are cast in is the caster's.
                        EffectDef::ExileFromTopUntil {
                            player: EffectRecipientDef::EachPlayer,
                            object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                            permission: ExiledCastPermissionDef::FreeWhileResolving,
                        },
                    ),
                    AbilityDef::activated(
                        "{9}{G/P}: Transform this creature. Activate only as a sorcery.",
                        &[CostDef::Mana(mana_cost!("{9}{G/P}"))],
                        EffectDef::Transform {
                            object: EffectRecipientDef::Source,
                        },
                    )
                    .with_activation_timing(ActivationTimingDef::SorcerySpeed),
                ] })
            },
        ),
        (
            "Etali, Primal Sickness",
            const {
                CardRules::new_creature_without_mana_cost(&["Phyrexian", "Elder", "Dinosaur"], 11, 11)
                .with_supertype(CardSupertype::Legendary)
                .printed_colors(&[ManaColor::Green])
                .with_abilities(&const { [
                    abilities::trample(),
                    abilities::indestructible(),
                    AbilityDef::triggered(
                        "Whenever this creature deals combat damage to a player, they get that many poison \
                         counters.",
                        TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                        // "They get that many poison counters": the amount is the damage that was
                        // dealt rather than the creature's power, which is what makes a blocked
                        // trampler give exactly what got through.
                        EffectDef::AddPlayerCounters {
                            recipient: EffectRecipientDef::EventPlayer,
                            kind: CounterKind::Poison,
                            amount: ValueDef::TriggerEventAmount,
                        },
                    ),
                ] })
            },
        ),
    ],
);

// MOM 299 — Urabrask // The Great Work
// Audit: unsupported — The Great Work chapter III needs one controller permission to cast instant and sorcery cards from any player's graveyard, plus the cast-this-way graveyard-to-exile replacement; existing graveyard permissions are owner-scoped or one named card.
pub(in crate::card::sets) static URABRASK_THE_GREAT_WORK_299: CardRecord = CardRecord::new(
    "Urabrask // The Great Work",
    "52173f36-19d2-48be-a8e8-8cbe946759c0",
    "Kekai Kotaki",
    crate::card::CardRules::unsupported(),
);

// MOM 305 — Drana and Linvala
// Audit: unsupported — The engine has no declarative operation that acquires the open-ended activated-ability set from matching permanents (and preserves their costs and source references) onto this creature.
pub(in crate::card::sets) static DRANA_AND_LINVALA_305: CardRecord = CardRecord::new(
    "Drana and Linvala",
    "29a5fe67-beaa-4ee3-b22c-0b8aed6f6f4d",
    "Anato Finnstark",
    crate::card::CardRules::unsupported(),
);

// MOM 311 — Kogla and Yidaro
pub(in crate::card::sets) static KOGLA_AND_YIDARO_311: CardRecord = CardRecord::new(
    "Kogla and Yidaro",
    "a32dbdc6-3321-4d77-8d2f-acbcb1a29090",
    "Daniel Warren Johnson",
    CardRules::new_creature(mana_cost!("{2}{R}{R}{G}{G}"), &["Ape", "Dinosaur", "Turtle"], 7, 7)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::modal_triggered(
                "When this creature enters, choose one —\n• It gains trample and haste until end of turn.\n• It fights target creature you don't control.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                &[
                    AbilityDef::spell(
                        "This creature gains trample and haste until end of turn.",
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::Composite(&[
                                AppliedEffectDef::add_ability(&abilities::trample()),
                                AppliedEffectDef::add_ability(&abilities::haste()),
                            ]),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ),
                    AbilityDef::spell_with_targets(
                        "This creature fights target creature you don't control.",
                        &[AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                            ]),
                        )],
                        EffectDef::Fight {
                            first: ObjectRefDef::Source,
                            second: ObjectRefDef::Target(TargetIndex::PRIMARY),
                            excess: None,
                        },
                    ),
                ],
            ),
            AbilityDef::activated_with_targets(
                "{2}{R}{G}, Discard this card: Destroy up to one target artifact or enchantment. Shuffle this card into your library from your graveyard, then draw a card.",
                &[CostDef::Mana(mana_cost!("{2}{R}{G}")), CostDef::DiscardSource],
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::Sequence(&[
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        then: None,
                    },
                    EffectDef::move_to_zone(
                        EffectRecipientDef::SourceZoneChangeSuccessor,
                        ZoneKind::Library,
                        ZonePlacement::Top,
                    ),
                    EffectDef::ShuffleLibrary {
                        player: EffectRecipientDef::Controller,
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            )
            .with_source_zones(&[ZoneKind::Hand]),
        ]),
);

// MOM 328 — Zephyr Winder
pub(in crate::card::sets) static ZEPHYR_WINDER: CardRecord = CardRecord::new(
    "Zephyr Winder",
    "14456a8e-016c-4407-8410-c490db3f5ea9",
    "Jana Schirmer",
// The untap is usually its own blocker coming back, which is what lets a
    // 2/1 flier attack into a board it could not otherwise race.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Elemental"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature deals combat damage to a player, untap up to one target creature.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            // "Up to one", so connecting with an empty board is still a
            // legal trigger rather than one that is removed for no targets.
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// MOM 338 — Elesh Norn // The Argent Etchings
// Audit: unsupported — Its front-face damage trigger matches damage to you or any permanent you control; DamageRecipientMatcherDef has PlayerAndCreaturesControlledBy but no player-plus-all-permanents matcher, so it would omit noncreature permanents.
pub(in crate::card::sets) static ELESH_NORN_THE_ARGENT_ETCHINGS_338: CardRecord = CardRecord::new(
    "Elesh Norn // The Argent Etchings",
    "40307bcf-199c-4487-bfab-cb5fb841dee8",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// MOM 360 — Hoarding Broodlord
// Audit: unsupported — No effect combines a searched arbitrary card's face-down linked exile with a controller play permission lasting while it remains exiled; ExileGrantingOwnerPlay cannot preserve the required face-down linked result. The engine also cannot grant convoke at cast-cost time to every spell cast from exile.
pub(in crate::card::sets) static HOARDING_BROODLORD_360: CardRecord = CardRecord::new(
    "Hoarding Broodlord",
    "2cbfb78e-cbab-4511-a887-60dad1a6cb6b",
    "Filip Burburan",
    crate::card::CardRules::unsupported(),
);

// MOM 374 — Realmbreaker, the Invasion Tree
pub(in crate::card::sets) static REALMBREAKER_THE_INVASION_TREE_374: CardRecord = CardRecord::new(
    "Realmbreaker, the Invasion Tree",
    "182520ae-d98f-4ef0-ad32-252f932f5d51",
    "Kekai Kotaki",
    CardRules::new_artifact(mana_cost!("{3}")).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::activated_with_targets("{2}, {T}: Target opponent mills three cards. Put a land card from their graveyard onto the battlefield tapped under your control. It gains \"If this land would leave the battlefield, exile it instead of putting it anywhere else.\"", &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource], &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Opponent))], EffectDef::Sequence(&[EffectDef::Mill { player: EffectRecipientDef::Target(TargetIndex::PRIMARY), amount: ValueDef::Constant(3) }, EffectDef::Choose(ChooseDef { chooser: PlayerRefDef::EffectController, candidates: ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Land), &[ZoneKind::Graveyard], PlayerRelation::Opponent)), exclude: None, minimum: 1, maximum: 1, binding: ObjectChoiceBindingDef::Objects(Binding!("realmbreaker_land")), unchosen: None, visibility: ChoiceVisibilityDef::Private, then: &EffectDef::WithZoneMoveResult { effect: &EffectDef::WithBattlefieldArrival { effect: &EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("realmbreaker_land"))), ZoneKind::Battlefield, ZonePlacement::Top), arrival: BattlefieldArrivalDef { controller: Some(PlayerRelation::You), modifications: &[BattlefieldEntryModificationDef::Tapped], ..BattlefieldArrivalDef::DEFAULT } }, binding: Binding!("realmbreaker_arrival"), then: &EffectDef::Apply { recipient: EffectRecipientDef::binding_zone_change_successors(Binding!("realmbreaker_arrival")), effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(&AbilityDef::replacement_for("If this land would be put into a graveyard from the battlefield, exile it instead.", ReplacementEventDef::WouldMove { from: Some(ZoneKind::Battlefield), to: ZoneKind::Graveyard, cause: ZoneMoveCauseDef::Any }, ReplacementEffectDef::MoveToZone(ZoneKind::Exile))), AppliedEffectDef::add_ability(&AbilityDef::replacement_for("If this land would be put into a hand from the battlefield, exile it instead.", ReplacementEventDef::WouldMove { from: Some(ZoneKind::Battlefield), to: ZoneKind::Hand, cause: ZoneMoveCauseDef::Any }, ReplacementEffectDef::MoveToZone(ZoneKind::Exile))), AppliedEffectDef::add_ability(&AbilityDef::replacement_for("If this land would be put into a library from the battlefield, exile it instead.", ReplacementEventDef::WouldMove { from: Some(ZoneKind::Battlefield), to: ZoneKind::Library, cause: ZoneMoveCauseDef::Any }, ReplacementEffectDef::MoveToZone(ZoneKind::Exile))), AppliedEffectDef::add_ability(&AbilityDef::replacement_for("If this land would be put into the command zone from the battlefield, exile it instead.", ReplacementEventDef::WouldMove { from: Some(ZoneKind::Battlefield), to: ZoneKind::Command, cause: ZoneMoveCauseDef::Any }, ReplacementEffectDef::MoveToZone(ZoneKind::Exile)))]), duration: ResolvedEffectDurationDef::Permanent } } })])),
AbilityDef::activated("{10}, {T}, Sacrifice Realmbreaker: Search your library for any number of Praetor cards, put them onto the battlefield, then shuffle.", &[CostDef::Mana(mana_cost!("{10}")), CostDef::TapSource, CostDef::SacrificeSource], EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Praetor")), minimum: 0, maximum: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::Subtype(SubtypeDef::Literal("Praetor")), &[ZoneKind::Library], PlayerRelation::You)), reveal: false, destination: ZoneKind::Battlefield, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None })
]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ALABASTER_HOST_INTERCESSOR,
    &ELSPETH_S_SMITE,
    &PHYREXIAN_CENSOR_31,
    &SUNFALL,
    &SURGE_OF_SALVATION_41,
    &FAERIE_MASTERMIND,
    &MEETING_OF_MINDS,
    &PREENING_CHAMPION,
    &TEMPORAL_CLEANSING,
    &CORRUPTED_CONVICTION,
    &WRENN_S_RESOLVE,
    &INVASION_OF_IKORIA_ZILORTHA_APEX_OF_IKORIA_190,
    &WARY_THESPIAN,
    &DJERU_AND_HAZORET_221,
    &GHALTA_AND_MAVREN_225,
    &FLYWHEEL_RACER_259,
    &ETALI_PRIMAL_CONQUEROR,
    &URABRASK_THE_GREAT_WORK_299,
    &DRANA_AND_LINVALA_305,
    &KOGLA_AND_YIDARO_311,
    &ZEPHYR_WINDER,
    &ELESH_NORN_THE_ARGENT_ETCHINGS_338,
    &HOARDING_BROODLORD_360,
    &REALMBREAKER_THE_INVASION_TREE_374,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
