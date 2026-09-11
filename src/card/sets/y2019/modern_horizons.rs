//! Modern Horizons cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityPredicateDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ColorChoiceOperationDef;
use crate::card::ComparisonDef;
use crate::card::ControlDurationDef;
use crate::card::CostDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DiscardFollowUpDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::EmblemCharacteristics;
use crate::card::ExilePlayDurationDef;
use crate::card::ManaColor;
use crate::card::MoveObjectsDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PayOrDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealAndClassifyCardsDef;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::ids::ParentBinding;
use crate::mana_cost;

/// "If it's not your turn" gates only the free cast. The printed cost is
/// always available, which is why this is a condition on the alternative
/// rather than a restriction on the card.
static NOT_YOUR_TURN: TriggerConditionDef =
    TriggerConditionDef::ActivePlayer(PlayerRelation::Opponent);

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "MH1",
    slug: "modern-horizons-1",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// MH1 7 — Ephemerate
pub(in crate::card::sets) static EPHEMERATE: CardRecord = CardRecord::new(
    "Ephemerate",
    "2da5f3f8-5eef-498f-ba2c-2f3fbc3745aa",
    "Bastien L. Deharme",
    // One white mana for two enter triggers, a turn apart. What it costs is
    // that the creature has to survive until the second one.
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Exile target creature you control, then return it to the battlefield under \
                     its owner's control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            // Exiling links the creature to the spell, which is what lets the return name
            // the card it just made.
            EffectDef::Sequence(&[
                EffectDef::ExileLinkedToSource {
                    until_source_leaves: false,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    face_down: false,
                    then: None,
                },
                EffectDef::ReturnLinkedExiles {
                    object: ObjectPredicateDef::Any,
                    counters: None,
                    zone: ZoneKind::Battlefield,
                    grant: None,
                    controller: None,
                    transformed: false,
                },
            ]),
        ),
        abilities::rebound(),
    ]),
);

// MH1 13 — Giver of Runes
pub(in crate::card::sets) static GIVER_OF_RUNES: CardRecord = CardRecord::new(
    "Giver of Runes",
    "4e117771-5a8b-4812-b487-32ba34b7f724",
    "Seb McKinnon",
    // Mother of Runes who cannot save herself, and in exchange answers the
    // colourless removal her mother could not.
    CardRules::new_creature(mana_cost!("{W}"), &["Kor", "Cleric"], 1, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Another target creature you control gains protection from colorless or from \
             the color of your choice until end of turn.",
            &[CostDef::TapSource],
            // "Another target creature you control": she may not protect herself, which
            // is the whole difference between her and her mother.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::ChooseColor {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                operation: ColorChoiceOperationDef::ProtectionFromChosenColorOrColorless,
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// MH1 21 — Ranger-Captain of Eos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RANGER_CAPTAIN_OF_EOS_21: CardRecord = CardRecord::new(
    "Ranger-Captain of Eos",
    "af3928b4-813a-4120-8799-de34235d60ac",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// MH1 24 — Rhox Veteran
pub(in crate::card::sets) static RHOX_VETERAN: CardRecord = CardRecord::new(
    "Rhox Veteran",
    "6384e266-d0dc-4af1-b3ab-ecaf9be2553c",
    "Milivoj Ćeran",
    // A 2/4 that attacks profitably because everything beside it gets
    // bigger and the best blocker is tapped out of the way first.
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Rhino", "Soldier"], 2, 4).with_abilities(&[
        crate::card::sets::y2011::mirrodin_besieged::battle_cry(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, tap target creature an opponent controls.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// MH1 27 — Settle Beyond Reality
pub(in crate::card::sets) static SETTLE_BEYOND_REALITY: CardRecord = CardRecord::new(
    "Settle Beyond Reality",
    "72ed8e57-61bb-4e89-9484-ff2be800a449",
    "Anthony Palumbo",
    // Five mana for exile plus a blink is why both modes are on one card:
    // the answer and the value are the same spell in a limited deck.
    CardRules::new_sorcery(mana_cost!("{4}{W}")).with_ability(
        AbilityDef::modal_spell(
            "Choose one or both —",
            &[
                AbilityDef::spell_with_targets(
                    "Exile target creature you don't control.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &[ZoneKind::Battlefield],
                            controller: Some(PlayerRelation::NotYou),
                            owner: None,
                        },
                    )],
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                ),
                AbilityDef::spell_with_targets(
                    "Exile target creature you control, then return it to the battlefield under \
                     its owner's control.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &[ZoneKind::Battlefield],
                            controller: Some(PlayerRelation::You),
                            owner: None,
                        },
                    )],
                    // Exiled linked to this spell and returned by the same
                    // resolution, so it comes back as a new object with its
                    // enters triggers armed.
                    EffectDef::Sequence(&[
                        EffectDef::ExileLinkedToSource {
                            until_source_leaves: false,
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            face_down: false,
                            then: None,
                        },
                        EffectDef::ReturnLinkedExiles {
                            object: ObjectPredicateDef::Any,
                            counters: None,
                            zone: ZoneKind::Battlefield,
                            grant: None,
                            controller: None,
                            transformed: false,
                        },
                    ]),
                ),
            ],
        )
        .with_mode_selection(1, 2, false),
    ),
);

// MH1 29 — Sisay, Weatherlight Captain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SISAY_WEATHERLIGHT_CAPTAIN_29: CardRecord = CardRecord::new(
    "Sisay, Weatherlight Captain",
    "5a293c45-1e73-4527-be2f-2dcd5c47b610",
    "Anna Steinbauer",
    crate::card::CardRules::unsupported(),
);

// MH1 37 — Winds of Abandon
pub(in crate::card::sets) static WINDS_OF_ABANDON: CardRecord = CardRecord::new(
    "Winds of Abandon",
    "3bb17913-fe4d-4acd-9b75-71f5a90f898b",
    "Noah Bradley",
// Two mana answers one creature and six answers the board, and neither
    // half leaves anything behind to rebuild from -- exile rather than
    // destruction is the whole reason the card ends games.
    CardRules::new_sorcery(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Exile target creature you don't control. For each creature exiled this way, its controller searches their library for a basic land card. Those players put those cards onto the battlefield tapped, then shuffle.",
            // "You don't control" is a constraint on the slot rather than on the object:
            // a spell being cast is not a permanent, so a predicate that compares
            // controllers has nothing to compare against yet.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
                // The searcher is the creature's controller, read from the announced
                // target: by now the creature is in exile and cannot be asked.
                EffectDef::SearchZone {
                    player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                        TargetIndex::PRIMARY,
                    ))),
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
            ]),
        ),
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{4}{W}{W}"))],
            AlternativeCastKindDef::Overload,
            Some("Exile each creature you don't control. For each creature exiled this way, its controller searches their library for a basic land card. Those players put those cards onto the battlefield tapped, then shuffle."),
            abilities::bind_objects_then(
                crate::card::ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    ),
                )),
                // "For each creature exiled this way" counts what the exile actually took,
                // so the set is bound before it is emptied and the search reads the count
                // off that binding rather than off a board the creatures have left.
                &EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                    EffectDef::SearchZone {
                        player: EffectRecipientDef::Opponent,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ]),
                        minimum: 0,
                        maximum: ValueDef::BoundObjectCount(ParentBinding),
                        reveal: false,
                        destination: ZoneKind::Battlefield,
                        placement: ZonePlacement::Top,
                        shuffle: true,
                        enters_tapped: true,
                        attachment: None,
                        binding: None,
                        then: None,
                    },
                ]),
            ),
        ),
    ]),
);

// MH1 46 — Echo of Eons
pub(in crate::card::sets) static ECHO_OF_EONS: CardRecord = CardRecord::new(
    "Echo of Eons",
    "ff590af2-2d6c-4f16-a9b8-1a6dab6e9ad5",
    "Terese Nielsen",
    // Six mana nobody pays: the card is here for the flashback, which turns a
    // graveyard full of rituals into a fresh seven for three.
    CardRules::new_sorcery(mana_cost!("{4}{U}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Each player shuffles their hand and graveyard into their library, then draws seven \
             cards.",
            abilities::shuffle_back_and_draw_seven(),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{2}{U}"))]),
    ]),
);

// MH1 51 — Faerie Seer
pub(in crate::card::sets) static FAERIE_SEER: CardRecord = CardRecord::new(
    "Faerie Seer",
    "d1fcfeb4-1818-4e08-be4c-27b8a9dc12e6",
    "Colin Boyer",
    // One mana for a flier that fixes the next two draws; the body is
    // incidental to the scry in every deck that plays it.
    CardRules::new_creature(mana_cost!("{U}"), &["Faerie", "Wizard"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, scry 2.",
            abilities::scry(ValueDef::Constant(2)),
        ),
    ]),
);

// MH1 52 — Force of Negation
pub(in crate::card::sets) static FORCE_OF_NEGATION: CardRecord = CardRecord::new(
    "Force of Negation",
    "e9be371c-c688-44ad-ab71-bd4c9f242d58",
    "Paul Scott Canavan",
    // Free interaction that only answers the half of the format worth
    // answering for free, and only on the turn somebody else is using it.
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::exile(
                ObjectPredicateDef::Color(ManaColor::Blue),
                ZoneKind::Hand,
                CostQuantityDef::Fixed(1),
            )],
            AlternativeCastKindDef::AlternativeCost,
            Some(
                "If it's not your turn, you may exile a blue card from your hand rather than pay \
                 this spell's mana cost.",
            ),
            EffectDef::None,
        )
        // Exiled rather than discarded, the same way the green half of the cycle
        // spends its card: what pays is gone without ever becoming a graveyard
        // card.
        .with_alternative_condition(&NOT_YOUR_TURN),
        AbilityDef::spell_with_targets(
            "Counter target noncreature spell. If that spell is countered this way, exile it \
             instead of putting it into its owner's graveyard.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Spell,
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            // The destination is part of the counter rather than a second
            // clause: a spell countered this way never reaches a graveyard,
            // so nothing watching one sees it arrive.
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// MH1 55 — Man-o'-War (reprint)
const MAN_O_WAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::visions::MAN_O_WAR,
    "5eaa4199-df9b-494a-af7a-2491e8b0ef70",
    "Jon J Muth",
);

// MH1 75 — Urza, Lord High Artificer
/// "This token gets +1/+1 for each artifact you control", which counts the
/// token itself: a lone Construct is a 1/1, and every artifact after it is
/// another point in both directions.
static ARTIFACTS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Artifact),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static URZA_LORD_HIGH_ARTIFICER: CardRecord = CardRecord::new(
    "Urza, Lord High Artificer",
    "9e7fb3c0-5159-4d1f-8490-ce4c9a60f567",
    "Grzegorz Rutkowski",
    // Four mana for a body, a blue mana out of every artifact you have, and
    // a mana sink that turns the rest of them into a free card.
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Human", "Artificer"], 1, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When this creature enters, create a 0/0 colorless Construct artifact creature \
                 token with \"This token gets +1/+1 for each artifact you control.\"",
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::artifact_creature(&["Construct"], &[], 0, 0)
                        .with_abilities(&[AbilityDef::static_ability(
                            "This token gets +1/+1 for each artifact you control.",
                            EffectDef::StaticApply {
                                recipient: EffectRecipientDef::Source,
                                effect: AppliedEffectDef::modify_power_toughness(
                                    ValueDef::CountMatchingObjects(&ARTIFACTS_YOU_CONTROL),
                                    ValueDef::CountMatchingObjects(&ARTIFACTS_YOU_CONTROL),
                                ),
                            },
                        )])
                        .with_art(CardArt::new(
                            "85f212cd-4fc6-42fe-b268-22d8e3b2b7eb",
                            "Victor Adame Minguez",
                        )),
                ))),
            ),
            AbilityDef::activated(
                "Tap an untapped artifact you control: Add {U}.",
                // "Tap an untapped artifact you control", which the Construct itself
                // answers -- and so does every Mox, every Lotus, and everything they made.
                &[CostDef::TapPermanents {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    controller: PlayerRelation::You,
                    count: 1,
                }],
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
            ),
            AbilityDef::activated(
                "{5}: Shuffle your library, then exile the top card. Until end of turn, you may \
                 play that card without paying its mana cost.",
                &[CostDef::Mana(mana_cost!("{5}"))],
                EffectDef::Sequence(&[
                    EffectDef::ShuffleLibrary {
                        player: EffectRecipientDef::Controller,
                    },
                    EffectDef::ExileTopOfLibraryToPlay {
                        player: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        free: true,
                        face_down: false,
                        duration: ExilePlayDurationDef::ThisTurn,
                        spend_any_color: false,
                        play_condition: None,
                        cast_only: false,
                    },
                ]),
            ),
        ]),
);

// MH1 81 — Carrion Feeder (reprint)
const CARRION_FEEDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2003::scourge::CARRION_FEEDER,
    "0a19da90-880e-4eca-8cf7-6d7baf090d53",
    "Svetlin Velinov",
);

// MH1 91 — First-Sphere Gargantua
// Audit: unsupported — Needs unearth. Its last clause exiles the creature if it would leave the battlefield for any zone, and ReplacementEventDef::WouldMove names a single destination.
pub(in crate::card::sets) static FIRST_SPHERE_GARGANTUA: CardRecord = CardRecord::new(
    "First-Sphere Gargantua",
    "a59f4e5c-fdc7-485f-aadb-2a71b3701dcc",
    "Randy Vargas",
    crate::card::CardRules::unsupported(),
);

// MH1 92 — Force of Despair
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORCE_OF_DESPAIR_92: CardRecord = CardRecord::new(
    "Force of Despair",
    "8f497b0d-4448-4201-bd55-c147da1a216d",
    "Seb McKinnon",
    crate::card::CardRules::unsupported(),
);

// MH1 94 — Graveshifter
// Audit: unsupported — Needs an all-zone creature-type characteristic-defining ability whose all-types value is copiable; battlefield all-type modifiers do not implement changeling.
pub(in crate::card::sets) static GRAVESHIFTER: CardRecord = CardRecord::new(
    "Graveshifter",
    "128c516b-7eb1-4f81-8b54-428bd0649d92",
    "Jakub Kasper",
    CardRules::unsupported(),
);

// MH1 101 — Putrid Goblin
pub(in crate::card::sets) static PUTRID_GOBLIN: CardRecord = CardRecord::new(
    "Putrid Goblin",
    "333406d5-abcc-4629-a33b-395d0662ba1b",
    "Winona Nelson",
    // A two-drop that has to be killed twice, and the second body is a 1/1 --
    // which is what makes it a sacrifice engine rather than a beater.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie", "Goblin"], 2, 2)
        .with_ability(abilities::persist()),
);

// MH1 116 — Yawgmoth, Thran Physician
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YAWGMOTH_THRAN_PHYSICIAN_116: CardRecord = CardRecord::new(
    "Yawgmoth, Thran Physician",
    "8690cbcc-f8fd-41f7-9e28-e61c12b04014",
    "Mark Winters",
    crate::card::CardRules::unsupported(),
);

// MH1 120 — Bogardan Dragonheart
pub(in crate::card::sets) static BOGARDAN_DRAGONHEART: CardRecord = CardRecord::new(
    "Bogardan Dragonheart",
    "feb81f44-8f22-4d28-a452-a50bef69a3e3",
    "Randy Vargas",
// One spare creature turns a 2/2 into a hasty 4/4 flier, so the card is
    // a finisher in the deck that was already sacrificing things.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Shaman"], 2, 2).with_ability(
        AbilityDef::activated(
            "Sacrifice another creature: Until end of turn, this creature becomes a Dragon with base power and toughness 4/4, flying, and haste.",
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                controller: PlayerRelation::You,
            }],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                // "Becomes a Dragon" repaints the whole creature-type line
                // rather than adding to it, and the 4/4 is a base value, so
                // counters and pumps still apply on top.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Dragon"])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(4),
                        ValueDef::Constant(4),
                    ),
                    AppliedEffectDef::add_ability(&abilities::flying()),
                    AppliedEffectDef::add_ability(&abilities::haste()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// MH1 126 — Goatnap
pub(in crate::card::sets) static GOATNAP: CardRecord = CardRecord::new(
    "Goatnap",
    "709d4928-e976-4c7c-ba09-cce95d1797b2",
    "Mark Zug",
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Gain control of target creature until end of turn. Untap that \
         creature. It gains haste until end of turn. If that creature \
         is a Goat, it also gets +3/+0 until end of turn.",
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
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goat")),
                },
                then: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            },
        ]),
    )]),
);

// MH1 128 — Goblin Engineer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_ENGINEER_128: CardRecord = CardRecord::new(
    "Goblin Engineer",
    "a55c4d47-5252-40af-961d-c08bc688028a",
    "Jehan Choo",
    crate::card::CardRules::unsupported(),
);

// MH1 130 — Goblin Oriflamme
pub(in crate::card::sets) static GOBLIN_ORIFLAMME: CardRecord = CardRecord::new(
    "Goblin Oriflamme",
    "33ec7cbe-16a0-4dbb-91fe-7e445a5268c8",
    "David Palumbo",
    CardRules::new_enchantment(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::static_ability(
        "Attacking creatures you control get +1/+0.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ))),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(0),
            ),
        },
    )]),
);

// MH1 138 — Pashalik Mons
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PASHALIK_MONS_138: CardRecord = CardRecord::new(
    "Pashalik Mons",
    "11616853-34b1-4bb1-9590-461e12970ec3",
    "Even Amundsen",
    crate::card::CardRules::unsupported(),
);

// MH1 143 — Ravenous Giant
pub(in crate::card::sets) static RAVENOUS_GIANT: CardRecord = CardRecord::new(
    "Ravenous Giant",
    "52337d8d-e0ee-4229-848d-9bbd989e15b7",
    "Milivoj Ćeran",
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Giant"], 5, 5).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, this creature deals 1 damage \
             to you.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::damage(EffectRecipientDef::Controller, ValueDef::Constant(1)),
        ),
    ]),
);

// MH1 144 — Reckless Charge (reprint)
const RECKLESS_CHARGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2001::odyssey::RECKLESS_CHARGE,
    "1754a8db-060e-470f-94c0-37f12d82978a",
    "Steve Argyle",
);

// MH1 145 — Seasoned Pyromancer
const PYROMANCER_ELEMENTAL: TokenCharacteristics =
    TokenCharacteristics::creature(&["Elemental"], &[ManaColor::Red], 1, 1).with_art(CardArt::new(
        "e5b57672-c346-42f5-ac3e-82466a13b957",
        "Winona Nelson",
    ));

pub(in crate::card::sets) static SEASONED_PYROMANCER: CardRecord = CardRecord::new(
    "Seasoned Pyromancer",
    "2e139ad1-1079-49e9-babd-6399c44ad333",
    "Cynthia Sheppard",
// Three mana that turns the two worst cards in your hand into two fresh
    // ones and a body for each of them that was not a land -- and then does
    // it again from the graveyard.
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Human", "Shaman"], 2, 2)
        .with_abilities(&[
            abilities::enters_trigger(
                "When this creature enters, discard two cards, then draw two cards. For each nonland card \
                 discarded this way, create a 1/1 red Elemental creature token.",
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: Some(DiscardFollowUpDef {
                        counted: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        bound: Some(ParentBinding),
                        effect: &// The draw comes before the tokens are counted, which is what the printed
                            // order says: two cards go, two cards come, and only then does the board
                            // pay you back for the ones that were not lands.
                            EffectDef::Sequence(&[
                                EffectDef::DrawCards {
                                    recipient: EffectRecipientDef::Controller,
                                    amount: ValueDef::Constant(2),
                                },
                                EffectDef::CreateToken(
                                    CreateTokenDef::new(TokenDef::Literal(PYROMANCER_ELEMENTAL))
                                        .with_count(ValueDef::BoundObjectCount(ParentBinding)),
                                ),
                            ]),
                    }),
                },
            ),
            // The card is spent from the graveyard, which is why he is never a dead
            // draw late: the body was the first half and this is the second.
            AbilityDef::activated(
                "{3}{R}{R}, Exile this card from your graveyard: Create two 1/1 red Elemental creature \
                 tokens.",
                &[
                    CostDef::Mana(mana_cost!("{3}{R}{R}")),
                    CostDef::ExileSource,
                ],
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(PYROMANCER_ELEMENTAL)).with_count(ValueDef::Constant(2)),
                ),
            )
            .with_source_zones(&[ZoneKind::Graveyard]),
        ]),
);

// MH1 158 — Collector Ouphe
pub(in crate::card::sets) static COLLECTOR_OUPHE: CardRecord = CardRecord::new(
    "Collector Ouphe",
    "085107a2-c1ec-473c-81d8-23e5a7197776",
    "Filip Burburan",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Ouphe"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "Activated abilities of artifacts can't be activated.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::cannot_activate_abilities(AbilityPredicateDef::Any),
            },
        ),
    ),
);

// MH1 164 — Force of Vigor
pub(in crate::card::sets) static FORCE_OF_VIGOR: CardRecord = CardRecord::new(
    "Force of Vigor",
    "017c415b-d635-43c6-92b8-8c95d1c4ff8d",
    "Randy Vargas",
CardRules::new_instant(mana_cost!("{2}{G}{G}")).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::exile(
                ObjectPredicateDef::Color(ManaColor::Green),
                ZoneKind::Hand,
                CostQuantityDef::Fixed(1),
            )],
            AlternativeCastKindDef::AlternativeCost,
            Some(
                "If it's not your turn, you may exile a green card from your hand rather than pay this spell's mana cost.",
            ),
            EffectDef::None,
        )
        // Exiled rather than discarded: the card is spent without ever becoming a
        // graveyard card, which is what "exile a green card" means.

        .with_alternative_condition(&NOT_YOUR_TURN),
        AbilityDef::spell_with_targets(
            "Destroy up to two target artifacts and/or enchantments.",
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
                2,
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ]),
);

// MH1 168 — Hexdrinker
pub(in crate::card::sets) static HEXDRINKER: CardRecord = CardRecord::new(
    "Hexdrinker",
    "89f5cc05-5d9d-4709-b3c5-a6249c294acc",
    "Forrest Imel",
    // One mana for a 2/1, and every spare mana afterwards buys a step toward
    // a creature nothing in the deck can answer.
    CardRules::new_creature(mana_cost!("{G}"), &["Snake"], 2, 1).with_abilities(&[
        AbilityDef::activated(
            "Level up {1} ({1}: Put a level counter on this. Level up only as a sorcery.)",
            &[CostDef::Mana(mana_cost!("{1}"))],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("level"),
                amount: ValueDef::Constant(1),
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
        AbilityDef::static_ability(
            "LEVEL 3-7: 4/4, protection from instants",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    // A level band is a continuous effect that applies while the permanent's
                    // level is inside it (CR 711.4a), so each band is a static ability whose
                    // subject is its own source and whose condition is the count of level
                    // counters on it. The bands do not overlap: the first ends where the second
                    // begins, which is why the lower one names a top as well as a bottom.
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Source,
                        ObjectPredicateDef::CounterCount {
                            kind: CounterKind::named("level"),
                            comparison: ComparisonDef::GreaterOrEqual,
                            amount: 3,
                        },
                        ObjectPredicateDef::CounterCount {
                            kind: CounterKind::named("level"),
                            comparison: ComparisonDef::Less,
                            amount: 8,
                        },
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(4),
                        ValueDef::Constant(4),
                    ),
                    AppliedEffectDef::add_ability(&AbilityDef::keyword(
                        "Protection from instants",
                        crate::card::KeywordAbility::ProtectionFrom(&ObjectPredicateDef::HasType(
                            CardType::Instant,
                        )),
                    )),
                ]),
            },
        ),
        AbilityDef::static_ability(
            "LEVEL 8+: 6/6, protection from everything",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Source,
                        ObjectPredicateDef::CounterCount {
                            kind: CounterKind::named("level"),
                            comparison: ComparisonDef::GreaterOrEqual,
                            amount: 8,
                        },
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(6),
                        ValueDef::Constant(6),
                    ),
                    // The reward for eight activations: nothing may block it, target it, damage
                    // it, or enchant it.
                    AppliedEffectDef::add_ability(&AbilityDef::keyword(
                        "Protection from everything",
                        crate::card::KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Any),
                    )),
                ]),
            },
        ),
    ]),
);

// MH1 169 — Krosan Tusker (reprint)
const KROSAN_TUSKER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2002::onslaught::KROSAN_TUSKER,
    "6391ba8b-7d9a-4077-8eeb-1b2ced14d973",
    "Kev Walker",
);

// MH1 171 — Mother Bear
pub(in crate::card::sets) static MOTHER_BEAR: CardRecord = CardRecord::new(
    "Mother Bear",
    "efae4d84-8134-461a-a352-a5bdff7259a7",
    "Winona Nelson",
// Two mana now and five later out of the same card, which is why the
    // graveyard half is worth holding rather than a consolation prize.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Bear"], 2, 2).with_ability(
        AbilityDef::activated(
            "{3}{G}{G}, Exile this card from your graveyard: Create two 2/2 green Bear creature tokens. Activate only as a sorcery.",
            &[
                CostDef::Mana(mana_cost!("{3}{G}{G}")),
                CostDef::ExileSource,
            ],
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                    &["Bear"],
                    &[ManaColor::Green],
                    2,
                    2,
                )))
                .with_amount(2),
            ),
        )
        // Activated from the graveyard rather than the battlefield, the same
        // way scavenge is, and at sorcery speed.
        .with_source_zones(&[ZoneKind::Graveyard])
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ),
);

// MH1 181 — Springbloom Druid
pub(in crate::card::sets) static SPRINGBLOOM_DRUID: CardRecord = CardRecord::new(
    "Springbloom Druid",
    "6161d2ed-7cff-4c90-9e74-1d179a6c1498",
    "Randy Gallegos",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Druid"], 1, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, you may sacrifice a land. If you \
             do, search your library for up to two basic land cards, put \
             them onto the battlefield tapped, then shuffle.",
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(
                    CardType::Land,
                ))],
                &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(2),
                    reveal: true,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: true,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            )),
        ),
    ]),
);

// MH1 187 — Trumpeting Herd
pub(in crate::card::sets) static TRUMPETING_HERD: CardRecord = CardRecord::new(
    "Trumpeting Herd",
    "b0f3b68e-f616-4687-bc2d-075165162cd1",
    "Lars Grant-West",
    // Six power over two turns for four mana, which is why the rebound is
    // the card rather than a rider on it.
    CardRules::new_sorcery(mana_cost!("{2}{G}{G}")).with_abilities(&[
        AbilityDef::spell(
            "Create a 3/3 green Elephant creature token.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Elephant"], &[ManaColor::Green], 3, 3),
            ))),
        ),
        abilities::rebound(),
    ]),
);

// MH1 193 — Winding Way
/// One mode of Winding Way: reveal four and sort them by the named type,
/// keeping the matches and burying the rest. A classification rather than a
/// choice -- the printed clause takes all of them, so nothing is asked.
const fn winding_way_mode(text: &'static str, object: ObjectPredicateDef) -> AbilityDef {
    AbilityDef::spell(
        text,
        EffectDef::RevealAndClassifyCards(RevealAndClassifyCardsDef {
            source: ObjectCollectionSourceDef::TopCards {
                player: PlayerRefDef::EffectController,
                count: ValueDef::Constant(4),
            },
            object,
            matching: Binding!("winding_way_chosen"),
            remainder: Binding!("winding_way_rest"),
            then: &const {
                EffectDef::Sequence(&[
                    EffectDef::MoveObjects(MoveObjectsDef {
                        input: ObjectSetDef::Binding(Binding!("winding_way_chosen")),
                        from: Some(ZoneKind::Library),
                        zone: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                        moved: None,
                        then: &EffectDef::None,
                    }),
                    EffectDef::MoveObjects(MoveObjectsDef {
                        input: ObjectSetDef::Binding(Binding!("winding_way_rest")),
                        from: Some(ZoneKind::Library),
                        zone: ZoneKind::Graveyard,
                        placement: ZonePlacement::Top,
                        moved: None,
                        then: &EffectDef::None,
                    }),
                ])
            },
        }),
    )
}

pub(in crate::card::sets) static WINDING_WAY: CardRecord = CardRecord::new(
    "Winding Way",
    "4e5d9776-b6ce-4ad6-8acc-69115ba5de76",
    "Adam Paquette",
    // The choice is made before anything is revealed, so this is a read on
    // the deck rather than a free pick of the better half.
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_ability(AbilityDef::modal_spell(
        "Choose creature or land.",
        &[
            winding_way_mode(
                "Creature — Reveal the top four cards of your library. Put all creature cards \
                 revealed this way into your hand and the rest into your graveyard.",
                ObjectPredicateDef::HasType(CardType::Creature),
            ),
            winding_way_mode(
                "Land — Reveal the top four cards of your library. Put all land cards revealed \
                 this way into your hand and the rest into your graveyard.",
                ObjectPredicateDef::HasType(CardType::Land),
            ),
        ],
    )),
);

// MH1 199 — Fallen Shinobi
pub(in crate::card::sets) static FALLEN_SHINOBI: CardRecord = CardRecord::new(
    "Fallen Shinobi",
    "900c9dfd-ece1-4b09-a801-0fa05e1994b9",
    "Tomasz Jedruszek",
// Ninjutsu is what makes a five-mana 5/4 connect on turn three, and
    // connecting is the whole card: two cards off the top of their deck,
    // free, every time.
    CardRules::new_creature(mana_cost!("{3}{U}{B}"), &["Zombie", "Ninja"], 5, 4)
        .with_abilities(&[
            abilities::ninjutsu!(
                "Ninjutsu {2}{U}{B} ({2}{U}{B}, Return an unblocked attacker you control to hand: Put this card onto the battlefield from your hand tapped and attacking.)",
                &[CostDef::Mana(mana_cost!("{2}{U}{B}"))],
            ),
            AbilityDef::triggered(
                "Whenever this creature deals combat damage to a player, that player exiles the top two cards of their library. Until end of turn, you may play those cards without paying their mana costs.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                EffectDef::ExileTopOfLibraryToPlay {
                    player: EffectRecipientDef::EventPlayer,
                    amount: ValueDef::Constant(2),
                    free: true,
                    face_down: false,
                    duration: ExilePlayDurationDef::ThisTurn,
                    spend_any_color: false,
                    play_condition: None,
                    cast_only: false,
                },
            ),
        ]),
);

// MH1 201 — Good-Fortune Unicorn
pub(in crate::card::sets) static GOOD_FORTUNE_UNICORN: CardRecord = CardRecord::new(
    "Good-Fortune Unicorn",
    "49d68905-e13e-4751-b028-90c795c11cd5",
    "Kee Lo",
    CardRules::new_creature(mana_cost!("{1}{G}{W}"), &["Unicorn"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever another creature you control enters, put a +1/+1 \
             counter on that creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::TriggeringObject,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// MH1 216 — Unsettled Mariner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNSETTLED_MARINER_216: CardRecord = CardRecord::new(
    "Unsettled Mariner",
    "eaea2e54-ee50-47b9-a2a5-e3353831248c",
    "John Stanko",
    crate::card::CardRules::unsupported(),
);

// MH1 217 — Wrenn and Six
pub(in crate::card::sets) static WRENN_AND_SIX: CardRecord = CardRecord::new(
    "Wrenn and Six",
    "4a706ecf-3277-40e3-871c-4ba4ead16e20",
    "Chase Stone",
// Two mana that buys back a fetchland every turn, pings something on the
    // way, and eventually turns the graveyard into a second hand.
    CardRules::new_planeswalker(mana_cost!("{R}{G}"), &["Wrenn"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "+1: Return up to one target land card from your graveyard to your hand.",
                &[CostDef::Loyalty(1)],
                // "Up to one target land card from your graveyard": a Wrenn with an empty
                // graveyard still ticks up.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Land),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                    1,
                )],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
            AbilityDef::activated_with_targets(
                "−1: This planeswalker deals 1 damage to any target.",
                &[CostDef::Loyalty(-1)],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(1),
                ),
            ),
            AbilityDef::activated(
                "−7: You get an emblem with \"Instant and sorcery cards in your graveyard have retrace.\" \
                 (You may cast instant and sorcery cards from your graveyard by discarding a land card in \
                 addition to paying their other costs.)",
                &[CostDef::Loyalty(-7)],
                EffectDef::CreateEmblem {
                    emblem: EmblemCharacteristics::new("Wrenn and Six emblem", &[AbilityDef::static_ability(
                            "Instant and sorcery cards in your graveyard have retrace.",
                            EffectDef::StaticApply {
                                recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
                                effect: AppliedEffectDef::Rule(AppliedRuleDef::GrantsAlternativeCastFromGraveyard {
                                    object: ObjectPredicateDef::AnyOf(&[
                                        ObjectPredicateDef::HasType(CardType::Instant),
                                        ObjectPredicateDef::HasType(CardType::Sorcery),
                                    ]),
                                    ability: &AbilityDef::alternative_cast(
                                        &[CostDef::ManaCostOf(crate::ObjectRefDef::Source), CostDef::discard(ObjectPredicateDef::HasType(CardType::Land))],
                                        AlternativeCastKindDef::Retrace,
                                        Some(
                                            "Retrace (You may cast this card from your graveyard by discarding a land card in \
                                             addition to paying its other costs.)",
                                        ),
                                        EffectDef::None,
                                    )
                                    // Retrace's own cost: the card's mana cost, plus a land out of your hand.
                                    // Discarding is what an ordinary hand cost does, so nothing else has to be
                                    // said about how the land is spent.
                                    ,
                                }),
                            },
                        )]),
                },
            ),
        ]),
);

// MH1 222 — Farmstead Gleaner
pub(in crate::card::sets) static FARMSTEAD_GLEANER: CardRecord = CardRecord::new(
    "Farmstead Gleaner",
    "edafd52f-2dda-4981-baee-404f47ee8969",
    "Josh Hass",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Scarecrow"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::activated(
            "{2}, {Q}: Put a +1/+1 counter on this creature. ({Q} is the untap symbol.)",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::UntapSource],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// MH1 225 — Lesser Masticore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LESSER_MASTICORE_225: CardRecord = CardRecord::new(
    "Lesser Masticore",
    "c4c7cba5-6111-40ce-828a-e811301bb283",
    "Wisnu Tan",
    crate::card::CardRules::unsupported(),
);

// MH1 230 — Talisman of Conviction
static TALISMAN_TAP: [CostDef; 1] = [CostDef::TapSource];

pub(in crate::card::sets) static TALISMAN_OF_CONVICTION: CardRecord = CardRecord::new(
    "Talisman of Conviction",
    "71148fd3-0c2c-459e-b8f5-735a0a8dd87f",
    "Lindsey Look",
    // Two mana that fixes for a life a turn, or for nothing at all when
    // colorless is what the next spell wants.
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &TALISMAN_TAP,
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {W}. This artifact deals 1 damage to you.",
            &TALISMAN_TAP,
            // The Talisman cycle's two halves: colorless for nothing, or the pair of
            // colours the card is for at a life apiece. Which colour is chosen belongs
            // to the activation, so the two are one printed ability.
            EffectDef::AddMana(
                AddManaEffectDef::choice(&[ManaColor::Red, ManaColor::White])
                    .with_damage_to_controller(1),
            ),
        ),
    ]),
);

// MH1 231 — Talisman of Creativity
pub(in crate::card::sets) static TALISMAN_OF_CREATIVITY: CardRecord = CardRecord::new(
    "Talisman of Creativity",
    "4d9dbadd-c1b6-44fe-92ac-6f69d7178342",
    "Lindsey Look",
    // Two mana that fixes for a life a turn, or for nothing at all when
    // colorless is what the next spell wants.
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &TALISMAN_TAP,
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {R}. This artifact deals 1 damage to you.",
            &TALISMAN_TAP,
            EffectDef::AddMana(
                AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Red])
                    .with_damage_to_controller(1),
            ),
        ),
    ]),
);

// MH1 232 — Talisman of Curiosity
pub(in crate::card::sets) static TALISMAN_OF_CURIOSITY: CardRecord = CardRecord::new(
    "Talisman of Curiosity",
    "fd52688a-39fd-430f-b950-cb56e0004396",
    "Lindsey Look",
    // The Simic half of the cycle: the damage is what pays for a colour, and
    // the colorless mode is what makes it free when colour is not the point.
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &TALISMAN_TAP,
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {G} or {U}. This artifact deals 1 damage to you.",
            &TALISMAN_TAP,
            EffectDef::AddMana(
                AddManaEffectDef::choice(&[ManaColor::Green, ManaColor::Blue])
                    .with_damage_to_controller(1),
            ),
        ),
    ]),
);

// MH1 233 — Talisman of Hierarchy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TALISMAN_OF_HIERARCHY_233: CardRecord = CardRecord::new(
    "Talisman of Hierarchy",
    "826f99c7-f534-4183-8f0d-efe1609808ac",
    "Lindsey Look",
    crate::card::CardRules::unsupported(),
);

// MH1 235 — Universal Automaton
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNIVERSAL_AUTOMATON_235: CardRecord = CardRecord::new(
    "Universal Automaton",
    "53c682e2-c90f-4f4b-9010-00b099e85518",
    "Ben Maier",
    crate::card::CardRules::unsupported(),
);

// MH1 238 — Fiery Islet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIERY_ISLET_238: CardRecord = CardRecord::new(
    "Fiery Islet",
    "a3aab13c-9d9d-4507-ae5d-da979990ae1b",
    "Richard Wright",
    crate::card::CardRules::unsupported(),
);

// MH1 241 — Hall of Heliod's Generosity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HALL_OF_HELIOD_S_GENEROSITY_241: CardRecord = CardRecord::new(
    "Hall of Heliod's Generosity",
    "b5cbd10a-b9a6-4c00-8280-72bb4add4390",
    "Daniel Ljunggren",
    crate::card::CardRules::unsupported(),
);

// MH1 243 — Nurturing Peatland
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NURTURING_PEATLAND_243: CardRecord = CardRecord::new(
    "Nurturing Peatland",
    "2744ac83-a79f-4042-8720-688b5adda382",
    "Noah Bradley",
    crate::card::CardRules::unsupported(),
);

// MH1 244 — Prismatic Vista
pub(in crate::card::sets) static PRISMATIC_VISTA: CardRecord = CardRecord::new(
    "Prismatic Vista",
    "e37da81e-be12-45a2-9128-376f1ad7b3e8",
    "Sam Burley",
    // A fetchland for every basic at once, which costs it the fetchland's
    // other half: nothing it finds is a dual, so it fixes colour without
    // paying anybody's land types.
    CardRules::new_land(&[]).with_ability(abilities::fetch_land_ability(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a basic land card, put it \
         onto the battlefield, then shuffle.",
        // "A basic land card", which is the supertype rather than the land types:
        // a dual with two basic types printed on it is not a basic land, and the
        // Vista cannot find one.
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Land),
            ObjectPredicateDef::Supertype(CardSupertype::Basic),
        ]),
    )),
);

// MH1 246 — Silent Clearing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILENT_CLEARING_246: CardRecord = CardRecord::new(
    "Silent Clearing",
    "ac07e230-0297-4e1d-bdfe-119010e0ad8e",
    "Seb McKinnon",
    crate::card::CardRules::unsupported(),
);

// MH1 247 — Sunbaked Canyon
pub(in crate::card::sets) static SUNBAKED_CANYON: CardRecord = CardRecord::new(
    "Sunbaked Canyon",
    "c36820fa-ee86-4206-9a0d-737a67cf5208",
    "Yeong-Hao Han",
    CardRules::new_land(&[]).with_abilities(&abilities::horizon_land(
        "{T}, Pay 1 life: Add {R} or {W}.",
        &[ManaColor::Red, ManaColor::White],
    )),
);

// MH1 249 — Waterlogged Grove
pub(in crate::card::sets) static WATERLOGGED_GROVE: CardRecord = CardRecord::new(
    "Waterlogged Grove",
    "0ab6bfbd-d2e1-4c4c-9f91-6f69c5b8e3bb",
    "John Avon",
    CardRules::new_land(&[]).with_abilities(&abilities::horizon_land(
        "{T}, Pay 1 life: Add {G} or {U}.",
        &[ManaColor::Green, ManaColor::Blue],
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &EPHEMERATE,
    &GIVER_OF_RUNES,
    &RANGER_CAPTAIN_OF_EOS_21,
    &RHOX_VETERAN,
    &SETTLE_BEYOND_REALITY,
    &SISAY_WEATHERLIGHT_CAPTAIN_29,
    &WINDS_OF_ABANDON,
    &ECHO_OF_EONS,
    &FAERIE_SEER,
    &FORCE_OF_NEGATION,
    &URZA_LORD_HIGH_ARTIFICER,
    &FIRST_SPHERE_GARGANTUA,
    &FORCE_OF_DESPAIR_92,
    &GRAVESHIFTER,
    &PUTRID_GOBLIN,
    &YAWGMOTH_THRAN_PHYSICIAN_116,
    &BOGARDAN_DRAGONHEART,
    &GOATNAP,
    &GOBLIN_ENGINEER_128,
    &GOBLIN_ORIFLAMME,
    &PASHALIK_MONS_138,
    &RAVENOUS_GIANT,
    &SEASONED_PYROMANCER,
    &COLLECTOR_OUPHE,
    &FORCE_OF_VIGOR,
    &HEXDRINKER,
    &MOTHER_BEAR,
    &SPRINGBLOOM_DRUID,
    &TRUMPETING_HERD,
    &WINDING_WAY,
    &FALLEN_SHINOBI,
    &GOOD_FORTUNE_UNICORN,
    &UNSETTLED_MARINER_216,
    &WRENN_AND_SIX,
    &FARMSTEAD_GLEANER,
    &LESSER_MASTICORE_225,
    &TALISMAN_OF_CONVICTION,
    &TALISMAN_OF_CREATIVITY,
    &TALISMAN_OF_CURIOSITY,
    &TALISMAN_OF_HIERARCHY_233,
    &UNIVERSAL_AUTOMATON_235,
    &FIERY_ISLET_238,
    &HALL_OF_HELIOD_S_GENEROSITY_241,
    &NURTURING_PEATLAND_243,
    &PRISMATIC_VISTA,
    &SILENT_CLEARING_246,
    &SUNBAKED_CANYON,
    &WATERLOGGED_GROVE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    MAN_O_WAR_REPRINT,
    CARRION_FEEDER_REPRINT,
    RECKLESS_CHARGE_REPRINT,
    KROSAN_TUSKER_REPRINT,
];
