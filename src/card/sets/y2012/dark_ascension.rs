//! Dark Ascension card records used by the built-in ISD–M14 Standard deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::sets::y2011::innistrad::{
    WEREWOLF_BACK_TRANSFORM, WEREWOLF_FRONT_TRANSFORM, morbid_entry_counters,
};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, BattlefieldArrivalDef, BattlefieldEntryModificationDef,
    CardArt, CardRules, CardSet, CardSupertype, CardType, ComparisonDef, ConditionalValueDef,
    ControlDurationDef, CopyStackObjectDef, CostModificationDef, CostQuantityDef, CounterKind,
    DamageEventMatcherDef, DamageKindDef, DamagePreventionDef, DamageRecipientMatcherDef,
    DamageSourceMatcherDef, DestroyFollowUpDef, DiscardFollowUpDef, DiscardSelectionDef,
    EffectChoiceDef, EffectDef, EffectRecipientDef, KeywordAbility, LifeConditionDef, ManaColor,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetCountConditionDef, ObjectSetDef,
    ObjectSetPredicateDef, PlayActionMatcherDef, PlayRestrictionDef, PlayerAttachmentQueryDef,
    PlayerRefDef, PlayerRelation, QuantifierDef, ReplacementEffectDef, ResolvedEffectDurationDef,
    SacrificedAmountDef, ScaledValueDef, SpellAdditionalCostDef, SumValueDef, TargetConditionDef,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::ids::{AdditionalCostObjectIndex, TargetIndex};
use crate::{ParentBinding, mana_cost, value_if_condition};

static FATEFUL_HOUR: TriggerConditionDef = TriggerConditionDef::ControllerLifeAtMost(5);

/// "As an additional cost to cast this spell, exile a creature card from your
/// graveyard."
static EXILE_A_CREATURE_CARD: SpellAdditionalCostDef = SpellAdditionalCostDef::exile(
    ObjectPredicateDef::HasType(CardType::Creature),
    ZoneKind::Graveyard,
    CostQuantityDef::Fixed(1),
);

static MORBID_A_CREATURE_DIED: TriggerConditionDef = TriggerConditionDef::CreatureDiedThisTurn;

static CAST_FROM_GRAVEYARD: TriggerConditionDef =
    TriggerConditionDef::SourceCastFrom(ZoneKind::Graveyard);

/// "Search your library for a basic land card, put it onto the battlefield
/// tapped, then shuffle."
static FETCH_A_BASIC_TAPPED: EffectDef = EffectDef::SearchZone {
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
};

// DKA 1 — Archangel's Light
pub(in crate::card::sets) static ARCHANGELS_LIGHT: CardRecord = CardRecord::new_with_legacy_id(
    1878,
    "Archangel's Light",
    CardArt::new("f99837b3-b487-43bb-846b-7a0e8afb6eef", "Volkan Baǵa"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{7}{W}")).with_ability(AbilityDef::spell(
        "You gain 2 life for each card in your graveyard, then shuffle your graveyard into \
         your library.",
        // The gain is counted before the shuffle empties the graveyard, which is
        // the only order that gains anything at all.
        EffectDef::Sequence(&[
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                // One life-gain event of twice the count, not two events of the count: a
                // card watching for life gain should see this happen once.
                amount: ValueDef::Scaled(&ScaledValueDef::new(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                    2,
                )),
            },
            EffectDef::MoveToZone {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ),
                zone: ZoneKind::Library,
                placement: ZonePlacement::Top,
            },
            EffectDef::ShuffleLibrary {
                player: EffectRecipientDef::Controller,
            },
        ]),
    )),
);

// DKA 2 — Bar the Door
pub(in crate::card::sets) static BAR_THE_DOOR: CardRecord = CardRecord::new_with_legacy_id(
    680,
    "Bar the Door",
    CardArt::new("b593f544-2d82-4237-b9a9-88503b5036cc", "Ryan Pancoast"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell(
        "Creatures you control get +0/+4 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(0),
                ValueDef::Constant(4),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// DKA 3 — Break of Day
static BREAK_OF_DAY_CREATURES: EffectRecipientDef = EffectRecipientDef::matching_objects(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static BREAK_OF_DAY: CardRecord = CardRecord::new_with_legacy_id(
    1880,
    "Break of Day",
    CardArt::new("9e39da2a-814a-46bb-a1ca-fc5532ece842", "Karl Kopinski"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell(
        "Creatures you control get +1/+1 until end of turn. Fateful hour — If you have 5 or \
         less life, those creatures gain indestructible until end of turn.",
        // The pump happens either way; the fateful-hour clause only adds to it, and
        // the life total is read once as the spell resolves.
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: BREAK_OF_DAY_CREATURES,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::IfCondition {
                condition: &FATEFUL_HOUR,
                then: &EffectDef::Apply {
                    recipient: BREAK_OF_DAY_CREATURES,
                    effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            },
        ]),
    )),
);

// DKA 4 — Burden of Guilt
pub(in crate::card::sets) static BURDEN_OF_GUILT: CardRecord = CardRecord::new_with_legacy_id(
    681,
    "Burden of Guilt",
    CardArt::new("d7440288-6c55-4502-bf20-3c5b50a2de5a", "John Stanko"),
    CardSet::DarkAscension,
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::activated(
                "{1}: Tap enchanted creature.",
                &[AbilityCostDef::Mana(mana_cost!("{1}"))],
                EffectDef::Tap {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
        ]),
);

// DKA 5 — Curse of Exhaustion
pub(in crate::card::sets) static CURSE_OF_EXHAUSTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b737a959-e974-4b2a-8dca-a257da6084b0"),
    "Curse of Exhaustion",
    crate::card::CardArt::new("b737a959-e974-4b2a-8dca-a257da6084b0", "Slawomir Maniak"),
    crate::card::CardSet::DarkAscension,
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}"))
        .with_subtypes(&["Aura", "Curse"])
        .with_abilities(&[
            abilities::enchant_player(),
            AbilityDef::static_ability(
                "Enchanted player can't cast more than one spell each turn.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::EnchantedPlayer,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                        PlayRestrictionDef::new(
                            PlayActionMatcherDef::CastSpell,
                            ObjectPredicateDef::Any,
                        )
                        .after_spells_cast(1),
                    )),
                },
            ),
        ]),
);

// DKA 6 — Elgaud Inquisitor
pub(in crate::card::sets) static ELGAUD_INQUISITOR: CardRecord = CardRecord::new_with_legacy_id(
    682,
    "Elgaud Inquisitor",
    CardArt::new("c342e1da-7ab9-4e29-96e6-77d820a45ede", "Slawomir Maniak"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Cleric"], 2, 2).with_abilities(&[
        abilities::lifelink(),
        abilities::dies_trigger(
            "When this creature dies, create a 1/1 white Spirit creature token with flying.",
            EffectDef::create_creature_token(&["Spirit"], &[ManaColor::White], 1, 1)
                .with_abilities(&[abilities::flying()])
                .with_art(CardArt::new(
                    "59e79ba0-33c8-46c8-8694-8bf854345fe7",
                    "Ryan Yee",
                )),
        ),
    ]),
);

// DKA 7 — Faith's Shield
// Audit: unsupported — Needs a recorded color choice, temporary protection from that choice, and the fateful-hour controller-life branch.
pub(in crate::card::sets) static FAITH_S_SHIELD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("65eb5920-3b03-4300-bc77-0fba5e6abe69"),
    "Faith's Shield",
    crate::card::CardArt::new("65eb5920-3b03-4300-bc77-0fba5e6abe69", "Svetlin Velinov"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 8 — Gather the Townsfolk
pub(in crate::card::sets) static GATHER_THE_TOWNSFOLK: CardRecord = CardRecord::new_with_legacy_id(
    1882,
    "Gather the Townsfolk",
    CardArt::new("9cfa554b-ee6d-4d4e-aabc-fe7bc6b25236", "Dan Murayama Scott"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell(
        "Create two 1/1 white Human creature tokens. Fateful hour — If you have 5 or less \
         life, create five of those tokens instead.",
        EffectDef::create_creature_token(&["Human"], &[ManaColor::White], 1, 1)
            .with_art(CardArt::new(
                "8894949b-f190-461e-996a-cf2b39f08a5d",
                "Michael C. Hayes",
            ))
            .with_count(ValueDef::IfControllerLifeAtMost(
                // "Instead", so this is one token-creation of a chosen size rather than two
                // creations one of which is skipped.
                &LifeConditionDef::new(5, ValueDef::Constant(5), ValueDef::Constant(2)),
            )),
    )),
);

// DKA 9 — Gavony Ironwright
pub(in crate::card::sets) static GAVONY_IRONWRIGHT: CardRecord = CardRecord::new_with_legacy_id(
    1881,
    "Gavony Ironwright",
    CardArt::new("05d8de75-b169-4426-94a4-b19cdfdffd89", "Karl Kopinski"),
    CardSet::DarkAscension,
    // "As long as", so the condition is continuous rather than checked once:
    // gaining life back above five turns the anthem off again.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 1, 4).with_ability(
        AbilityDef::static_ability(
            "Fateful hour — As long as you have 5 or less life, other creatures you control get \
             +1/+4.",
            EffectDef::IfCondition {
                condition: &FATEFUL_HOUR,
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(4),
                    ),
                },
            },
        ),
    ),
);

// DKA 10 — Hollowhenge Spirit
// Audit: unsupported — Needs an effect that removes a chosen attacking or blocking creature from combat.
pub(in crate::card::sets) static HOLLOWHENGE_SPIRIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b1be91a-cdec-4933-b834-0a7838abb9b8"),
    "Hollowhenge Spirit",
    crate::card::CardArt::new("0b1be91a-cdec-4933-b834-0a7838abb9b8", "Lars Grant-West"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 11 — Increasing Devotion
pub(in crate::card::sets) static INCREASING_DEVOTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("87b5de81-65a6-4a74-8a71-767b92e89e91"),
    "Increasing Devotion",
    crate::card::CardArt::new("87b5de81-65a6-4a74-8a71-767b92e89e91", "Daniel Ljunggren"),
    crate::card::CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{3}{W}{W}")).with_abilities(&[
        AbilityDef::spell(
            "Create five 1/1 white Human creature tokens. If this spell was cast from a graveyard, create ten of those tokens instead.",
            EffectDef::create_creature_token(&["Human"], &[ManaColor::White], 1, 1)
                .with_art(CardArt::new(
                    "8894949b-f190-461e-996a-cf2b39f08a5d",
                    "Michael C. Hayes",
                ))
                .with_count(value_if_condition!(
                    &CAST_FROM_GRAVEYARD,
                    ValueDef::Constant(10),
                    ValueDef::Constant(5)
                )),
        ),
        abilities::flashback(mana_cost!("{7}{W}{W}")),
    ]),
);

// DKA 12 — Lingering Souls
pub(in crate::card::sets) static LINGERING_SOULS: CardRecord = CardRecord::new_with_legacy_id(
    683,
    "Lingering Souls",
    CardArt::new("891a92d7-9ccf-4de1-8286-aa5254f27ba9", "Bud Cook"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{2}{W}")).with_abilities(&[
        AbilityDef::spell(
            "Create two 1/1 white Spirit creature tokens with flying.",
            EffectDef::create_creature_token(&["Spirit"], &[ManaColor::White], 1, 1)
                .with_abilities(&[abilities::flying()])
                .with_art(CardArt::new(
                    "59e79ba0-33c8-46c8-8694-8bf854345fe7",
                    "Ryan Yee",
                ))
                .with_amount(2),
        ),
        abilities::flashback(mana_cost!("{1}{B}")),
    ]),
);

// DKA 13 — Loyal Cathar // Unhallowed Cathar
// Audit: unsupported — Needs a delayed end-step return that brings the dead card back transformed and tapped.
pub(in crate::card::sets) static LOYAL_CATHAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cb09041b-4d09-4cae-9e85-b859edae885b"),
    "Loyal Cathar",
    crate::card::CardArt::new("cb09041b-4d09-4cae-9e85-b859edae885b", "Ryan Pancoast"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 14 — Midnight Guard
pub(in crate::card::sets) static MIDNIGHT_GUARD: CardRecord = CardRecord::new_with_legacy_id(
    684,
    "Midnight Guard",
    CardArt::new("2264b760-c527-470d-bad0-d8baaf543631", "Jason A. Engle"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 2, 3).with_ability(
        AbilityDef::triggered(
            "Whenever another creature enters, untap this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ),
);

// DKA 15 — Niblis of the Mist
pub(in crate::card::sets) static NIBLIS_OF_THE_MIST: CardRecord = CardRecord::new_with_legacy_id(
    685,
    "Niblis of the Mist",
    CardArt::new("08aea6e3-c8a8-4964-b95d-4c639da55de1", "Igor Kieryluk"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Spirit"], 2, 1).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, you may tap target creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            },
        ),
    ]),
);

// DKA 16 — Niblis of the Urn
pub(in crate::card::sets) static NIBLIS_OF_THE_URN: CardRecord = CardRecord::new_with_legacy_id(
    686,
    "Niblis of the Urn",
    CardArt::new("11bf2ff7-0f8d-47ea-adfd-af299e793a37", "Igor Kieryluk"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Spirit"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, you may tap target creature.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            },
        ),
    ]),
);

// DKA 17 — Ray of Revelation
pub(in crate::card::sets) static RAY_OF_REVELATION: CardRecord = CardRecord::new_with_legacy_id(
    201,
    "Ray of Revelation",
    CardArt::new("d7e2c5a4-cf92-46bd-9033-8036436488cb", "Cliff Childs"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target enchantment.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Enchantment),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
        abilities::flashback(mana_cost!("{G}")),
    ]),
);

// DKA 18 — Requiem Angel
pub(in crate::card::sets) static REQUIEM_ANGEL: CardRecord = CardRecord::new_with_legacy_id(
    687,
    "Requiem Angel",
    CardArt::new("5385925d-05ad-4f2e-bd2c-8de6c088ed05", "Eric Deschamps"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{5}{W}"), &["Angel"], 5, 5).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever another non-Spirit creature you control dies, create a 1/1 white Spirit creature token with flying.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Spirit")),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]), Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)),
            EffectDef::create_creature_token(&["Spirit"], &[ManaColor::White], 1, 1).with_abilities(&[abilities::flying()]).with_art(CardArt::new("59e79ba0-33c8-46c8-8694-8bf854345fe7", "Ryan Yee")),
        ),
    ]),
);

// DKA 19 — Sanctuary Cat
pub(in crate::card::sets) static SANCTUARY_CAT: CardRecord = CardRecord::new_with_legacy_id(
    688,
    "Sanctuary Cat",
    CardArt::new("96865440-01ad-40f2-90d7-9ecd0b4efecc", "David Palumbo"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{W}"), &["Cat"], 1, 2),
);

// DKA 20 — Séance
// Audit: unsupported — Needs temporary copy tokens that add the Spirit subtype and a delayed end-step exile linked to each token.
pub(in crate::card::sets) static SEANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e00ae92c-af6d-4a00-b102-c6d3bcc394b4"),
    "Séance",
    crate::card::CardArt::new("e00ae92c-af6d-4a00-b102-c6d3bcc394b4", "David Rapoza"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 21 — Silverclaw Griffin
pub(in crate::card::sets) static SILVERCLAW_GRIFFIN: CardRecord = CardRecord::new_with_legacy_id(
    689,
    "Silverclaw Griffin",
    CardArt::new("54528722-a6aa-4567-9cd1-e4a97adec7d0", "Daniel Ljunggren"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Griffin"], 3, 2)
        .with_abilities(&[abilities::flying(), abilities::first_strike()]),
);

// DKA 22 — Skillful Lunge
pub(in crate::card::sets) static SKILLFUL_LUNGE: CardRecord = CardRecord::new_with_legacy_id(
    690,
    "Skillful Lunge",
    CardArt::new("2a28abc1-3e75-4db4-baa1-b47abdb7453b", "Jason Felix"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +2/+0 and gains first strike until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// DKA 23 — Sudden Disappearance
// Audit: unsupported — Needs one delayed trigger to return an arbitrary mass-exiled group at the next end step after the spell source has left the stack.
pub(in crate::card::sets) static SUDDEN_DISAPPEARANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a51b792c-b987-49b6-9cc6-80d613c7d065"),
    "Sudden Disappearance",
    crate::card::CardArt::new("a51b792c-b987-49b6-9cc6-80d613c7d065", "Cliff Childs"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 24 — Thalia, Guardian of Thraben
pub(in crate::card::sets) static THALIA_GUARDIAN_OF_THRABEN: CardRecord =
    CardRecord::new_with_legacy_id(
        1879,
        "Thalia, Guardian of Thraben",
        CardArt::new(
            "824423ff-6441-4be6-b754-810adf9ca6a2",
            "Jana Schirmer & Johannes Voss",
        ),
        CardSet::DarkAscension,
        // No "you cast" clause, so the tax reaches both seats -- Thalia's own
        // controller included, which is the cost of playing her.
        CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 2, 1)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                abilities::first_strike(),
                AbilityDef::static_ability(
                    "Noncreature spells cost {1} more to cast.",
                    EffectDef::ModifyCost(CostModificationDef::increase_spell(
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                        PlayerRelation::Any,
                        mana_cost!("{1}"),
                    )),
                ),
            ]),
    );

// DKA 25 — Thraben Doomsayer
pub(in crate::card::sets) static THRABEN_DOOMSAYER: CardRecord = CardRecord::new_with_legacy_id(
    1883,
    "Thraben Doomsayer",
    CardArt::new("066a9312-a5b2-4fc5-b46d-e0c9020583a5", "John Stanko"),
    CardSet::DarkAscension,
    // The tokens it makes are among the "other creatures" the anthem pumps,
    // so a low life total turns each of them into a 3/3.
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Cleric"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{T}: Create a 1/1 white Human creature token.",
            &[AbilityCostDef::TapSource],
            EffectDef::create_creature_token(&["Human"], &[ManaColor::White], 1, 1).with_art(
                CardArt::new("8894949b-f190-461e-996a-cf2b39f08a5d", "Michael C. Hayes"),
            ),
        ),
        AbilityDef::static_ability(
            "Fateful hour — As long as you have 5 or less life, other creatures you control get \
             +2/+2.",
            EffectDef::IfCondition {
                condition: &FATEFUL_HOUR,
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                },
            },
        ),
    ]),
);

// DKA 26 — Thraben Heretic
pub(in crate::card::sets) static THRABEN_HERETIC: CardRecord = CardRecord::new_with_legacy_id(
    691,
    "Thraben Heretic",
    CardArt::new("f8cc36df-040b-4f29-bcc1-f5600803f71d", "James Ryman"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Wizard"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Exile target creature card from a graveyard.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// DKA 27 — Artful Dodge
pub(in crate::card::sets) static ARTFUL_DODGE: CardRecord = CardRecord::new_with_legacy_id(
    692,
    "Artful Dodge",
    CardArt::new("de6ce6aa-e19f-4299-9807-e68920e63c73", "Tomasz Jedruszek"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature can't be blocked this turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Any,
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::flashback(mana_cost!("{U}")),
    ]),
);

// DKA 28 — Beguiler of Wills
pub(in crate::card::sets) static BEGUILER_OF_WILLS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e21ae024-d565-48a1-8004-5aa320a5d24d"),
    "Beguiler of Wills",
    crate::card::CardArt::new("e21ae024-d565-48a1-8004-5aa320a5d24d", "Eric Deschamps"),
    crate::card::CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Human", "Wizard"], 1, 1)
        .with_ability(AbilityDef::activated_with_targets(
            "{T}: Gain control of target creature with power less than or equal to the number of creatures you control.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::PowerLessThan(ValueDef::Sum(&SumValueDef {
                    left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    right: ValueDef::Constant(1),
                })),
            ]))],
            EffectDef::GainControl {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                controller: PlayerRefDef::EffectController,
                duration: ControlDurationDef::Indefinitely,
            },
        )),
);

// DKA 29 — Bone to Ash
pub(in crate::card::sets) static BONE_TO_ASH: CardRecord = CardRecord::new_with_legacy_id(
    693,
    "Bone to Ash",
    CardArt::new("c4a75cef-9551-45e2-b1ff-80662c76ec20", "Clint Cearley"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{2}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target creature spell.\nDraw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Spell,
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// DKA 30 — Call to the Kindred
// Audit: unsupported — Needs a top-five selection predicate that dynamically shares a creature type with the enchanted creature, plus arbitrary bottom ordering.
pub(in crate::card::sets) static CALL_TO_THE_KINDRED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee35f96c-6060-4456-897e-27b74c8c2137"),
    "Call to the Kindred",
    crate::card::CardArt::new("ee35f96c-6060-4456-897e-27b74c8c2137", "Jason A. Engle"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 31 — Chant of the Skifsang
pub(in crate::card::sets) static CHANT_OF_THE_SKIFSANG: CardRecord = CardRecord::new_with_legacy_id(
    694,
    "Chant of the Skifsang",
    CardArt::new("6e604b2e-f257-465d-9342-6eb55b2334c5", "Nils Hamm"),
    CardSet::DarkAscension,
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets -13/-0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-13),
                        ValueDef::Constant(0),
                    ),
                },
            ),
        ]),
);

// DKA 32 — Chill of Foreboding
pub(in crate::card::sets) static CHILL_OF_FOREBODING: CardRecord = CardRecord::new_with_legacy_id(
    695,
    "Chill of Foreboding",
    CardArt::new("0abd6534-92bb-44e3-88c2-6709f1a4f29c", "Terese Nielsen"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Each player mills five cards.",
            EffectDef::Mill {
                player: EffectRecipientDef::EachPlayer,
                amount: ValueDef::Constant(5),
            },
        ),
        abilities::flashback(mana_cost!("{7}{U}")),
    ]),
);

// DKA 33 — Counterlash
// Audit: unsupported — Needs a post-counter optional cast from hand without paying mana, filtered by a card type shared with the countered spell.
pub(in crate::card::sets) static COUNTERLASH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d3ec2c57-8e67-472d-8f2e-0492d311f130"),
    "Counterlash",
    crate::card::CardArt::new("d3ec2c57-8e67-472d-8f2e-0492d311f130", "Austin Hsu"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 34 — Curse of Echoes
// Audit: unsupported — Needs optional copies of a triggering spell with independently reselectable targets for every other player.
pub(in crate::card::sets) static CURSE_OF_ECHOES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("147dbe42-665a-4e21-b405-d17554d5efcf"),
    "Curse of Echoes",
    crate::card::CardArt::new("147dbe42-665a-4e21-b405-d17554d5efcf", "Slawomir Maniak"),
    crate::card::CardSet::DarkAscension,
    CardRules::unsupported(),
);

// DKA 35 — Divination
pub(in crate::card::sets) static DIVINATION: CardRecord = CardRecord::new_with_legacy_id(
    696,
    "Divination",
    CardArt::new("4a1340f1-85a4-4551-9871-bb00db6d97a8", "Scott Chou"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell(
        "Draw two cards.",
        abilities::draw_cards(ValueDef::Constant(2)),
    )),
);

// DKA 36 — Dungeon Geists
pub(in crate::card::sets) static DUNGEON_GEISTS: CardRecord = CardRecord::new_with_legacy_id(
    697,
    "Dungeon Geists",
    CardArt::new("b715da2e-c816-4c14-8522-811c97c66fed", "Nils Hamm"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Spirit"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets("When this creature enters, tap target creature an opponent controls. That creature doesn't untap during its controller's untap step for as long as you control this creature.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            })], EffectDef::Sequence(&[
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Rule(
                        AppliedRuleDef::DoesNotUntapDuringUntapStep,
                    ),
                    duration: ResolvedEffectDurationDef::WhileSourceRemains,
                },
            ])),
    ]),
);

// DKA 37 — Geralf's Mindcrusher
pub(in crate::card::sets) static GERALFS_MINDCRUSHER: CardRecord = CardRecord::new_with_legacy_id(
    698,
    "Geralf's Mindcrusher",
    CardArt::new("68ac8b5f-4d95-43fc-bf23-10247986a746", "Steven Belledin"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Zombie", "Horror"], 5, 5).with_abilities(
        &[
            abilities::enters_trigger_with_targets(
                "When this creature enters, target player mills five cards.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Mill {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(5),
                },
            ),
            abilities::undying(),
        ],
    ),
);

// DKA 38 — Griptide
pub(in crate::card::sets) static GRIPTIDE: CardRecord = CardRecord::new_with_legacy_id(
    699,
    "Griptide",
    CardArt::new("27f92b74-86bb-4bb3-8f78-640984698f28", "Igor Kieryluk"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Put target creature on top of its owner's library.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Library,
            placement: ZonePlacement::Top,
        },
    )),
);

// DKA 39 — Havengul Runebinder
pub(in crate::card::sets) static HAVENGUL_RUNEBINDER: CardRecord = CardRecord::new_with_legacy_id(
    700,
    "Havengul Runebinder",
    CardArt::new("de766c12-eb2c-466a-8630-8242a153eb1f", "Bud Cook"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Human", "Wizard"], 2, 2).with_ability(
        AbilityDef::activated(
            "{2}{U}, {T}, Exile a creature card from your graveyard: Create a 2/2 black Zombie creature token, then put a +1/+1 counter on each Zombie creature you control.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{U}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::MoveToZone(crate::card::MoveToZoneCostDef::new(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ZoneKind::Graveyard,
                    ZoneKind::Exile,
                    1,
                )),
            ],
            EffectDef::Sequence(&[
                EffectDef::create_creature_token(&["Zombie"], &[ManaColor::Black], 2, 2).with_art(CardArt::new("b877c19d-6022-4377-92e7-4511e24eb98e", "Lucas Graciano")),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Zombie"),
                        ]), &[ZoneKind::Battlefield], PlayerRelation::You),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ),
);

// DKA 40 — Headless Skaab
pub(in crate::card::sets) static HEADLESS_SKAAB: CardRecord = CardRecord::new_with_legacy_id(
    1605,
    "Headless Skaab",
    CardArt::new("ca63f9a2-381e-4c84-b0bb-3acd9445b4db", "Johann Bodin"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Zombie", "Warrior"], 3, 6).with_abilities(&[
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, exile a creature card from your \
             graveyard.",
            &[],
            EXILE_A_CREATURE_CARD,
            EffectDef::None,
        ),
        AbilityDef::as_enters(
            "This creature enters tapped.",
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
        ),
    ]),
);

// DKA 41 — Increasing Confusion
pub(in crate::card::sets) static INCREASING_CONFUSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("13f5bcdc-70bb-4d67-99e1-282f166ee4bf"),
    "Increasing Confusion",
    crate::card::CardArt::new("13f5bcdc-70bb-4d67-99e1-282f166ee4bf", "Dan Murayama Scott"),
    crate::card::CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{X}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target player mills X cards. If this spell was cast from a graveyard, that player mills twice that many cards instead.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                PlayerRelation::Any,
            ))],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: value_if_condition!(
                    &CAST_FROM_GRAVEYARD,
                    ValueDef::Scaled(&ScaledValueDef::new(ValueDef::ChosenX, 2)),
                    ValueDef::ChosenX
                ),
            },
        ),
        abilities::flashback(mana_cost!("{X}{U}")),
    ]),
);

// DKA 42 — Mystic Retrieval
pub(in crate::card::sets) static MYSTIC_RETRIEVAL: CardRecord = CardRecord::new_with_legacy_id(
    701,
    "Mystic Retrieval",
    CardArt::new("281a685a-bd02-43bf-8700-2207c65bbbb1", "Scott Chou"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{3}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return target instant or sorcery card from your graveyard to your hand.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
        abilities::flashback(mana_cost!("{2}{R}")),
    ]),
);

// DKA 43 — Nephalia Seakite
pub(in crate::card::sets) static NEPHALIA_SEAKITE: CardRecord = CardRecord::new_with_legacy_id(
    702,
    "Nephalia Seakite",
    CardArt::new("174a1d08-cd79-43d6-897f-3ee9a682d15e", "Wayne England"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Bird"], 2, 3)
        .with_abilities(&[abilities::flash(), abilities::flying()]),
);

// DKA 44 — Niblis of the Breath
pub(in crate::card::sets) static NIBLIS_OF_THE_BREATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0686843d-6d1e-4488-8c17-7c986a154195"),
    "Niblis of the Breath",
    crate::card::CardArt::new("0686843d-6d1e-4488-8c17-7c986a154195", "Igor Kieryluk"),
    crate::card::CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Spirit"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{U}, {T}: You may tap or untap target creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{U}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::ChooseEffect {
                    player: EffectRecipientDef::Controller,
                    choices: &[
                        EffectChoiceDef {
                            label: "Tap the target creature",
                            effect: EffectDef::Tap {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            },
                        },
                        EffectChoiceDef {
                            label: "Untap the target creature",
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

// DKA 45 — Relentless Skaabs
pub(in crate::card::sets) static RELENTLESS_SKAABS: CardRecord = CardRecord::new_with_legacy_id(
    1606,
    "Relentless Skaabs",
    CardArt::new("b3304cab-0dc9-47e4-ac68-00974b64f5a0", "Karl Kopinski"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Zombie"], 4, 4).with_abilities(&[
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, exile a creature card from your \
             graveyard.",
            &[],
            EXILE_A_CREATURE_CARD,
            EffectDef::None,
        ),
        abilities::undying(),
    ]),
);

// DKA 46 — Saving Grasp
pub(in crate::card::sets) static SAVING_GRASP: CardRecord = CardRecord::new_with_legacy_id(
    703,
    "Saving Grasp",
    CardArt::new("914837df-c255-4cea-9255-b05f218fd9f8", "Matt Stewart"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return target creature you own to your hand.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
        abilities::flashback(mana_cost!("{W}")),
    ]),
);

// DKA 47 — Screeching Skaab
pub(in crate::card::sets) static SCREECHING_SKAAB: CardRecord = CardRecord::new_with_legacy_id(
    704,
    "Screeching Skaab",
    CardArt::new("3c40a2c7-df7a-41a6-a49e-5f7db808b810", "Clint Cearley"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Zombie"], 2, 1).with_ability(
        abilities::enters_trigger(
            "When this creature enters, mill two cards.",
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// DKA 48 — Secrets of the Dead
pub(in crate::card::sets) static SECRETS_OF_THE_DEAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c982d679-581d-43e1-acd0-db77eb0987c2"),
    "Secrets of the Dead",
    crate::card::CardArt::new("c982d679-581d-43e1-acd0-db77eb0987c2", "Eytan Zana"),
    crate::card::CardSet::DarkAscension,
    CardRules::new_enchantment(mana_cost!("{2}{U}")).with_ability(AbilityDef::triggered(
        "Whenever you cast a spell from your graveyard, draw a card.",
        TriggerEventDef::spell_cast_from(
            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ZoneKind::Graveyard,
        ),
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )),
);

// DKA 49 — Shriekgeist
pub(in crate::card::sets) static SHRIEKGEIST: CardRecord = CardRecord::new_with_legacy_id(
    705,
    "Shriekgeist",
    CardArt::new("435c5218-46b3-456a-aedf-d9586a4bd0a3", "Raymond Swanland"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Spirit"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, that player mills two cards.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::Mill {
                player: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// DKA 50 — Soul Seizer // Ghastly Haunting
// Audit: unsupported — Needs transforming a creature into an Aura, attaching the transformed permanent to the damaged player, and granting permanent control of that player's creature.
pub(in crate::card::sets) static SOUL_SEIZER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f500cb95-d5ea-4cf2-920a-f1df45a9059b"),
    "Soul Seizer",
    crate::card::CardArt::new("f500cb95-d5ea-4cf2-920a-f1df45a9059b", "Lucas Graciano"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 51 — Stormbound Geist
pub(in crate::card::sets) static STORMBOUND_GEIST: CardRecord = CardRecord::new_with_legacy_id(
    1598,
    "Stormbound Geist",
    CardArt::new("040eddb0-fca2-41eb-ab07-c48d49385973", "Dan Murayama Scott"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Spirit"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature can block only creatures with flying.",
            // "This creature can block only creatures with flying."
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                )),
            },
        ),
        abilities::undying(),
    ]),
);

// DKA 52 — Thought Scour
pub(in crate::card::sets) static THOUGHT_SCOUR: CardRecord = CardRecord::new_with_legacy_id(
    706,
    "Thought Scour",
    CardArt::new("88bf1ebb-9d85-4b9b-a614-c7f965c0893d", "David Rapoza"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target player mills two cards.\nDraw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// DKA 53 — Tower Geist
pub(in crate::card::sets) static TOWER_GEIST: CardRecord = CardRecord::new_with_legacy_id(
    707,
    "Tower Geist",
    CardArt::new("c9e9f552-34b6-43a5-8ef8-9d5208f4cae0", "Izzy"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Spirit"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, look at the top two cards of your library. Put one of them into your hand and the other into your graveyard.",
            abilities::look_at_top_cards_choose_to_hand_rest_graveyard(
                ValueDef::Constant(2),
                ObjectPredicateDef::Any,
                1,
                1,
            ),
        ),
    ]),
);

// DKA 54 — Black Cat
pub(in crate::card::sets) static BLACK_CAT: CardRecord = CardRecord::new_with_legacy_id(
    708,
    "Black Cat",
    CardArt::new("bb1c6379-69d5-48aa-8d06-257c0592794e", "David Palumbo"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie", "Cat"], 1, 1).with_ability(
        abilities::dies_trigger_with_targets(
            "When this creature dies, target opponent discards a card at random.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::Random,
                then: None,
            },
        ),
    ),
);

// DKA 55 — Chosen of Markov // Markov's Servant
pub(in crate::card::sets) static CHOSEN_OF_MARKOV: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("7c5a3c09-5656-4975-ba03-2d809903ed18"),
    "Chosen of Markov // Markov's Servant",
    crate::card::CardArt::new("7c5a3c09-5656-4975-ba03-2d809903ed18", "Steve Argyle"),
    crate::card::CardSet::DarkAscension,
    &[
        (
            "Chosen of Markov",
            CardRules::new_creature(mana_cost!("{2}{B}"), &["Human"], 2, 2).with_ability(
                AbilityDef::activated(
                    "{T}, Tap an untapped Vampire you control: Transform this creature.",
                    &[
                        AbilityCostDef::TapSource,
                        AbilityCostDef::TapPermanents {
                            object: ObjectPredicateDef::Subtype("Vampire"),
                            controller: PlayerRelation::You,
                            count: 1,
                        },
                    ],
                    EffectDef::Transform {
                        object: EffectRecipientDef::Source,
                    },
                ),
            ),
        ),
        (
            "Markov's Servant",
            CardRules::new_creature_without_mana_cost(&["Vampire"], 4, 4)
                .printed_colors(&[ManaColor::Black]),
        ),
    ],
);

// DKA 56 — Curse of Misfortunes
// Audit: unsupported — Needs a library search excluding names already attached to the enchanted player and an attached arrival.
pub(in crate::card::sets) static CURSE_OF_MISFORTUNES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c531d218-ff1c-4333-a19d-446d709b1e28"),
    "Curse of Misfortunes",
    crate::card::CardArt::new("c531d218-ff1c-4333-a19d-446d709b1e28", "Terese Nielsen"),
    crate::card::CardSet::DarkAscension,
    CardRules::unsupported(),
);

// DKA 57 — Curse of Thirst
pub(in crate::card::sets) static CURSE_OF_THIRST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2fa2b4ac-a62f-45c6-88fb-6ad44c6af28c"),
    "Curse of Thirst",
    crate::card::CardArt::new(
        "a23ed5d1-44dc-4733-9e01-65fbc5dc02f2",
        "Christopher Moeller",
    ),
    crate::card::CardSet::DarkAscension,
    CardRules::new_enchantment(mana_cost!("{4}{B}"))
        .with_subtypes(&["Aura", "Curse"])
        .with_abilities(&[
            abilities::enchant_player(),
            abilities::enchanted_player_upkeep(
                "At the beginning of enchanted player's upkeep, this Aura deals damage to that player equal to the number of Curses attached to them.",
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::EnchantedPlayer,
                    amount: ValueDef::CountMatchingPlayerAttachments(
                        &PlayerAttachmentQueryDef::new(
                            PlayerRelation::EnchantedPlayer,
                            ObjectPredicateDef::Subtype("Curse"),
                        ),
                    ),
                },
            ),
        ]),
);

// DKA 58 — Deadly Allure
pub(in crate::card::sets) static DEADLY_ALLURE: CardRecord = CardRecord::new_with_legacy_id(
    1739,
    "Deadly Allure",
    CardArt::new("268e4582-7674-4565-8ef4-00be1a90f410", "Steve Argyle"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature gains deathtouch until end of turn and must be blocked this turn if \
             able.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                // Deathtouch and the requirement are the point of each other: anything
                // forced to block it dies for having done so.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_ability(&abilities::deathtouch()),
                    AppliedEffectDef::Rule(AppliedRuleDef::MustBeBlockedBy(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::flashback(mana_cost!("{G}")),
    ]),
);

// DKA 59 — Death's Caress
pub(in crate::card::sets) static DEATHS_CARESS: CardRecord = CardRecord::new_with_legacy_id(
    1975,
    "Death's Caress",
    CardArt::new("0643fb9a-8284-4dfc-836a-c2c69ef09f32", "James Ryman"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{3}{B}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature. If that creature was a Human, you gain life equal to its toughness.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                // "If that creature was a Human" is read after the destruction, so both the
                // subtype and the toughness are last-known -- which is exactly what the
                // target slot still remembers.
                amount: ValueDef::IfTargetMatches(&TargetConditionDef {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::Subtype("Human"),
                    then: ValueDef::TargetToughness(TargetIndex::PRIMARY),
                    otherwise: ValueDef::Constant(0),
                }),
            },
        ]),
    )),
);

// DKA 60 — Falkenrath Torturer
// Audit: unsupported — Needs an activated sacrifice cost to expose whether the chosen creature was Human so the conditional counter can follow the flying grant.
pub(in crate::card::sets) static FALKENRATH_TORTURER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5e81d6ed-2141-4177-9ded-680fff65b39e"),
    "Falkenrath Torturer",
    crate::card::CardArt::new("5e81d6ed-2141-4177-9ded-680fff65b39e", "Steve Argyle"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 61 — Farbog Boneflinger
pub(in crate::card::sets) static FARBOG_BONEFLINGER: CardRecord = CardRecord::new_with_legacy_id(
    709,
    "Farbog Boneflinger",
    CardArt::new("98d45316-b44a-4cf6-8cbe-b02fe6545141", "Tomasz Jedruszek"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Zombie"], 2, 2).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, target creature gets -2/-2 until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-2),
                    ValueDef::Constant(-2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// DKA 62 — Fiend of the Shadows
// Audit: unsupported — Needs play permission for a card exiled from an opponent's hand and a sacrifice-regeneration procedure that preserves source identity.
pub(in crate::card::sets) static FIEND_OF_THE_SHADOWS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("38167118-f5b3-4e07-8060-b170b49cff9e"),
    "Fiend of the Shadows",
    crate::card::CardArt::new("38167118-f5b3-4e07-8060-b170b49cff9e", "Igor Kieryluk"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 63 — Geralf's Messenger
pub(in crate::card::sets) static GERALFS_MESSENGER: CardRecord = CardRecord::new_with_legacy_id(
    710,
    "Geralf's Messenger",
    CardArt::new("bffaad78-97ff-431f-bfb0-e96c7558f974", "Kev Walker"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{B}{B}{B}"), &["Zombie"], 3, 2).with_abilities(&[
        abilities::enters_tapped(CardType::Creature),
        abilities::enters_trigger_with_targets(
            "When this creature enters, target opponent loses 2 life.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
        abilities::undying(),
    ]),
);

// DKA 64 — Gravecrawler
// Audit: unsupported — Needs conditional graveyard casting permission tied to controlling a Zombie and a static prohibition on blocking.
pub(in crate::card::sets) static GRAVECRAWLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("33df275d-bdad-4e4b-9f92-ec2bf98fcba7"),
    "Gravecrawler",
    crate::card::CardArt::new("48d73cb5-22ac-43df-9c4b-0c860bb80b3e", "Steven Belledin"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 65 — Gravepurge
// Audit: unsupported — Needs an unbounded “any number” target group; the fixed-cardinality target representation cannot express every legal graveyard size.
pub(in crate::card::sets) static GRAVEPURGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3c0c266b-9ef5-4de2-a358-65739de41491"),
    "Gravepurge",
    crate::card::CardArt::new("3c0c266b-9ef5-4de2-a358-65739de41491", "Zoltan Boros"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 66 — Gruesome Discovery
// Audit: unsupported — Needs the morbid replacement branch to reveal a hand and let the spell's controller choose two cards from it.
pub(in crate::card::sets) static GRUESOME_DISCOVERY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d0a9d733-24b6-49ed-a15a-c00285eea4b2"),
    "Gruesome Discovery",
    crate::card::CardArt::new("d0a9d733-24b6-49ed-a15a-c00285eea4b2", "John Stanko"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 67 — Harrowing Journey
pub(in crate::card::sets) static HARROWING_JOURNEY: CardRecord = CardRecord::new_with_legacy_id(
    711,
    "Harrowing Journey",
    CardArt::new("9cf96a6c-8481-4954-b149-7153b80480be", "James Paick"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{4}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player draws three cards and loses 3 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        ]),
    )),
);

// DKA 68 — Highborn Ghoul
pub(in crate::card::sets) static HIGHBORN_GHOUL: CardRecord = CardRecord::new_with_legacy_id(
    712,
    "Highborn Ghoul",
    CardArt::new("fbe999ed-b419-440c-9189-1046f43d7b87", "Volkan Baǵa"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Zombie"], 2, 1)
        .with_ability(abilities::intimidate()),
);

// DKA 69 — Increasing Ambition
pub(in crate::card::sets) static INCREASING_AMBITION: CardRecord = CardRecord::new_with_legacy_id(
    1692,
    "Increasing Ambition",
    CardArt::new("c8f508dc-7c7d-47e8-a4ef-0e8fd99cbd74", "Volkan Baǵa"),
    CardSet::DarkAscension,
    const {
        CardRules::new_sorcery(mana_cost!("{4}{B}")).with_abilities(&const { [
            AbilityDef::spell(
                "Search your library for a card and put that card into your hand. If this spell was cast from a graveyard, instead search your library for two cards and put those cards into your hand. Then shuffle.",
                EffectDef::IfElseCondition {
                    condition: &CAST_FROM_GRAVEYARD,
                    then: &EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::Any,
                        minimum: 2,
                        maximum: ValueDef::Constant(2),
                        reveal: false,
                        destination: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                        shuffle: true,
                        enters_tapped: false,
                        attachment: None,
                        binding: None,
                        then: None,
                    },
                    otherwise: &EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::Any,
                        minimum: 1,
                        maximum: ValueDef::Constant(1),
                        reveal: false,
                        destination: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                        shuffle: true,
                        enters_tapped: false,
                        attachment: None,
                        binding: None,
                        then: None,
                    },
                },
            ),
            abilities::flashback(mana_cost!("{7}{B}")),
        ] })
    },
);

// DKA 70 — Mikaeus, the Unhallowed
pub(in crate::card::sets) static MIKAEUS_THE_UNHALLOWED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("801b124d-3546-4882-a6e1-c9c353628a18"),
    "Mikaeus, the Unhallowed",
    crate::card::CardArt::new("801b124d-3546-4882-a6e1-c9c353628a18", "Chris Rahn"),
    crate::card::CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{3}{B}{B}{B}"), &["Zombie", "Cleric"], 5, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::intimidate(),
            AbilityDef::triggered(
                "Whenever a Human deals damage to you, destroy it.",
                TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                    kind: DamageKindDef::Any,
                    source: DamageSourceMatcherDef::Matching(ObjectPredicateDef::Subtype("Human")),
                    recipient: DamageRecipientMatcherDef::Recipients(
                        EffectRecipientDef::Controller,
                    ),
                }),
                EffectDef::Destroy {
                    object: EffectRecipientDef::TriggeringObject,
                    can_regenerate: true,
                    then: None,
                },
            ),
            AbilityDef::static_ability(
                "Other non-Human creatures you control get +1/+1 and have undying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Human")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::undying()),
                    ]),
                },
            ),
        ]),
);

// DKA 71 — Ravenous Demon // Archdemon of Greed
// Audit: unsupported — Needs a transformed-face upkeep procedure that offers a Human sacrifice and otherwise transforms the source and makes its controller lose 9 life.
pub(in crate::card::sets) static RAVENOUS_DEMON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("abbbb466-31ef-42dd-9993-124f7ec3d8ae"),
    "Ravenous Demon",
    crate::card::CardArt::new("6aef77b3-4b38-4902-9f7a-dc18b5bb9da9", "Igor Kieryluk"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 72 — Reap the Seagraf
pub(in crate::card::sets) static REAP_THE_SEAGRAF: CardRecord = CardRecord::new_with_legacy_id(
    713,
    "Reap the Seagraf",
    CardArt::new("4defdead-19fa-4535-9f71-8808388b0332", "James Ryman"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_abilities(&[
        AbilityDef::spell(
            "Create a 2/2 black Zombie creature token.",
            EffectDef::create_creature_token(&["Zombie"], &[ManaColor::Black], 2, 2).with_art(
                CardArt::new("b877c19d-6022-4377-92e7-4511e24eb98e", "Lucas Graciano"),
            ),
        ),
        abilities::flashback(mana_cost!("{4}{U}")),
    ]),
);

// DKA 73 — Sightless Ghoul
pub(in crate::card::sets) static SIGHTLESS_GHOUL: CardRecord = CardRecord::new_with_legacy_id(
    1515,
    "Sightless Ghoul",
    CardArt::new("018bd4ae-cdea-410d-9ce6-6a70f12de966", "Svetlin Velinov"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Zombie", "Soldier"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            },
        ),
        abilities::undying(),
    ]),
);

// DKA 74 — Skirsdag Flayer
pub(in crate::card::sets) static SKIRSDAG_FLAYER: CardRecord = CardRecord::new_with_legacy_id(
    714,
    "Skirsdag Flayer",
    CardArt::new("274976b0-2bb5-46e6-b62e-b50d80a77e28", "Austin Hsu"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Cleric"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{3}{B}, {T}, Sacrifice a Human: Destroy target creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}{B}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Human"),
                    ]),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ),
);

// DKA 75 — Spiteful Shadows
pub(in crate::card::sets) static SPITEFUL_SHADOWS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ea0a94fe-11d6-48a7-9195-2cb5eff4b962"),
    "Spiteful Shadows",
    crate::card::CardArt::new("ea0a94fe-11d6-48a7-9195-2cb5eff4b962", "John Stanko"),
    crate::card::CardSet::DarkAscension,
    CardRules::new_enchantment(mana_cost!("{1}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered(
                "Whenever enchanted creature is dealt damage, it deals that much damage to its controller.",
                TriggerEventDef::DamageDealt(DamageEventMatcherDef::to(
                    EffectRecipientDef::AttachedPermanent,
                )),
                EffectDef::DealDamageFrom {
                    source: ObjectRefDef::DamagedObject,
                    recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::DamagedObject,
                    )),
                    amount: ValueDef::TriggerEventAmount,
                },
            ),
        ]),
);

// DKA 76 — Tragic Slip
/// Morbid replaces the amount rather than adding a second effect, so both
/// printed clauses come down to which number this picks.
const TRAGIC_SLIP_AMOUNT: ValueDef = ValueDef::IfCreatureDiedThisTurn(&ConditionalValueDef {
    then: ValueDef::Constant(-13),
    otherwise: ValueDef::Constant(-1),
});

pub(in crate::card::sets) static TRAGIC_SLIP: CardRecord = CardRecord::new_with_legacy_id(
    229,
    "Tragic Slip",
    CardArt::new("09666671-601e-4fca-bdfb-fb288bf2672c", "Christopher Moeller"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{B}")).with_abilities(&[
        AbilityDef::spell_with_targets("Target creature gets -1/-1 until end of turn.", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )], EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(TRAGIC_SLIP_AMOUNT, TRAGIC_SLIP_AMOUNT),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            }),
        AbilityDef::static_ability(
            "Morbid — That creature gets -13/-13 until end of turn instead if a creature died this turn.",
            // The conditional value on the spell clause above already
            // carries this modifier; this clause has no second effect to run.
            EffectDef::None,
        ),
    ]),
);

// DKA 77 — Undying Evil
pub(in crate::card::sets) static UNDYING_EVIL: CardRecord = CardRecord::new_with_legacy_id(
    715,
    "Undying Evil",
    CardArt::new("325f2243-54fd-484b-a742-166cea7ec179", "Kev Walker"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gains undying until end of turn. (When it dies, if it had no +1/+1 counters on it, return it to the battlefield under its owner's control with a +1/+1 counter on it.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::add_ability(&abilities::undying()),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// DKA 78 — Vengeful Vampire
pub(in crate::card::sets) static VENGEFUL_VAMPIRE: CardRecord = CardRecord::new_with_legacy_id(
    716,
    "Vengeful Vampire",
    CardArt::new("d03c64a7-37d2-4d8f-bd7a-9435bc2f4101", "Lucas Graciano"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Vampire"], 3, 2)
        .with_abilities(&[abilities::flying(), abilities::undying()]),
);

// DKA 79 — Wakedancer
pub(in crate::card::sets) static WAKEDANCER: CardRecord = CardRecord::new_with_legacy_id(
    1851,
    "Wakedancer",
    CardArt::new("f533fbfa-42ae-4e27-92a4-9936bcd2a5f4", "Austin Hsu"),
    CardSet::DarkAscension,
    // Morbid is an intervening if: with nothing dead the trigger is never
    // created at all, rather than created and then doing nothing.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Shaman"], 2, 2).with_ability(
        AbilityDef::triggered_if(
            "Morbid — When this creature enters, if a creature died this turn, create a 2/2 \
             black Zombie creature token.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &MORBID_A_CREATURE_DIED,
            EffectDef::create_creature_token(&["Zombie"], &[ManaColor::Black], 2, 2).with_art(
                CardArt::new("b877c19d-6022-4377-92e7-4511e24eb98e", "Lucas Graciano"),
            ),
        ),
    ),
);

// DKA 80 — Zombie Apocalypse
pub(in crate::card::sets) static ZOMBIE_APOCALYPSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("756119c5-f10e-431b-9f8b-5da5850a72fd"),
    "Zombie Apocalypse",
    crate::card::CardArt::new("fe662a08-a8b1-4f25-b7c0-dca1c7ad7271", "Peter Mohrbacher"),
    crate::card::CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{3}{B}{B}{B}")).with_ability(AbilityDef::spell(
        "Return all Zombie creature cards from your graveyard to the battlefield tapped, then destroy all Humans.",
        EffectDef::Sequence(&[
            EffectDef::WithBattlefieldArrival {
                effect: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Zombie"),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                    zone: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                },
                arrival: BattlefieldArrivalDef {
                    modifications: &[BattlefieldEntryModificationDef::Tapped],
                    ..BattlefieldArrivalDef::DEFAULT
                },
            },
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Subtype("Human"),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                can_regenerate: true,
                then: None,
            },
        ]),
    )),
);

// DKA 81 — Afflicted Deserter // Werewolf Ransacker
pub(in crate::card::sets) static AFFLICTED_DESERTER: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("a2c044c0-3625-4bdf-9445-b462394cecae"),
    "Afflicted Deserter // Werewolf Ransacker",
    crate::card::CardArt::new("a2c044c0-3625-4bdf-9445-b462394cecae", "David Palumbo"),
    crate::card::CardSet::DarkAscension,
    &[
        (
            "Afflicted Deserter",
            CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Werewolf"], 3, 2)
                .with_ability(WEREWOLF_FRONT_TRANSFORM),
        ),
        (
            "Werewolf Ransacker",
            CardRules::new_creature_without_mana_cost(&["Werewolf"], 5, 4)
                .printed_colors(&[ManaColor::Red])
                .with_abilities(&[
                    AbilityDef::triggered_with_targets(
                        "Whenever this creature transforms into Werewolf Ransacker, you may destroy target artifact. If that artifact is put into a graveyard this way, this creature deals 3 damage to that artifact's controller.",
                        TriggerEventDef::transforms(ObjectPredicateDef::Source),
                        &[AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Artifact),
                        )],
                        EffectDef::May {
                            player: EffectRecipientDef::Controller,
                            effect: &EffectDef::Destroy {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                can_regenerate: true,
                                then: Some(DestroyFollowUpDef {
                                    binding: ParentBinding,
                                    effect: &EffectDef::IfCondition {
                                        condition: &TriggerConditionDef::ObjectSetCount(
                                            &ObjectSetCountConditionDef {
                                                objects: &ObjectSetDef::Binding(
                                                    ParentBinding,
                                                ),
                                                predicate: ObjectSetPredicateDef {
                                                    filter: None,
                                                    comparison: ComparisonDef::GreaterOrEqual,
                                                    amount: 1,
                                                },
                                            },
                                        ),
                                        then: &EffectDef::DealDamage {
                                            recipient: EffectRecipientDef::ControllerOfTarget(
                                                TargetIndex::PRIMARY,
                                            ),
                                            amount: ValueDef::Constant(3),
                                        },
                                    },
                                }),
                            },
                        },
                    ),
                    WEREWOLF_BACK_TRANSFORM,
                ]),
        ),
    ],
);

// DKA 82 — Alpha Brawl
// Audit: unsupported — Needs damage sourced by the targeted creature to every other creature its opponent controls, followed by reciprocal damage from each survivor.
pub(in crate::card::sets) static ALPHA_BRAWL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e2ec168a-3e4f-4527-901a-bc28cc28d125"),
    "Alpha Brawl",
    crate::card::CardArt::new("e2ec168a-3e4f-4527-901a-bc28cc28d125", "Randy Gallegos"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 83 — Blood Feud
pub(in crate::card::sets) static BLOOD_FEUD: CardRecord = CardRecord::new_with_legacy_id(
    717,
    "Blood Feud",
    CardArt::new("634d59b8-6046-4796-95c5-eec75a239986", "Winona Nelson"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{4}{R}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Target creature fights another target creature. (Each deals damage equal to its power to the other.)",
            &[
                AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
                AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
            ],
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex(1)),
                    amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::TargetPower(TargetIndex(1)),
                },
            ]),
        ),
    ),
);

// DKA 84 — Burning Oil
pub(in crate::card::sets) static BURNING_OIL: CardRecord = CardRecord::new_with_legacy_id(
    718,
    "Burning Oil",
    CardArt::new("47773da8-afe4-43e1-8355-6ab51451ee00", "Trevor Claxton"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Burning Oil deals 3 damage to target attacking or blocking creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AttackingOrBlocking,
                ]),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        ),
        abilities::flashback(mana_cost!("{3}{W}")),
    ]),
);

// DKA 85 — Curse of Bloodletting
// Audit: unsupported — Needs a damage-event replacement that doubles damage to the enchanted player with replacement-order choices.
pub(in crate::card::sets) static CURSE_OF_BLOODLETTING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9dc4ac6f-0005-47f8-bee9-10429cc542e4"),
    "Curse of Bloodletting",
    crate::card::CardArt::new("9dc4ac6f-0005-47f8-bee9-10429cc542e4", "Michael C. Hayes"),
    crate::card::CardSet::DarkAscension,
    CardRules::unsupported(),
);

// DKA 86 — Erdwal Ripper
pub(in crate::card::sets) static ERDWAL_RIPPER: CardRecord = CardRecord::new_with_legacy_id(
    719,
    "Erdwal Ripper",
    CardArt::new("769ea5e9-6d05-4bc6-8f14-00eb2532c8b5", "Kev Walker"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Vampire"], 2, 1).with_abilities(&[
        abilities::haste(),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, put a +1/+1 counter on it.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// DKA 87 — Faithless Looting
pub(in crate::card::sets) static FAITHLESS_LOOTING: CardRecord = CardRecord::new_with_legacy_id(
    720,
    "Faithless Looting",
    CardArt::new("a1b0da17-d595-441d-811c-a2d28d2bb232", "Gabor Szikszai"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell(
            "Draw two cards, then discard two cards.",
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ]),
        ),
        abilities::flashback(mana_cost!("{2}{R}")),
    ]),
);

// DKA 88 — Fires of Undeath
pub(in crate::card::sets) static FIRES_OF_UNDEATH: CardRecord = CardRecord::new_with_legacy_id(
    721,
    "Fires of Undeath",
    CardArt::new("6d94aaa4-c2fd-4714-9198-8415158b9c4d", "Jason Chan"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{2}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Fires of Undeath deals 2 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
        abilities::flashback(mana_cost!("{5}{B}")),
    ]),
);

// DKA 89 — Flayer of the Hatebound
pub(in crate::card::sets) static FLAYER_OF_THE_HATEBOUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ffb17c3f-0154-49ee-bb5f-cd1df8546871"),
    "Flayer of the Hatebound",
    crate::card::CardArt::new(
        "ffb17c3f-0154-49ee-bb5f-cd1df8546871",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{5}{R}"), &["Devil"], 4, 2).with_abilities(&[
        abilities::undying(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature or another creature enters from your graveyard, that creature deals damage equal to its power to any target.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::OwnedBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Graveyard),
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
            EffectDef::DealDamageFrom {
                source: ObjectRefDef::TriggeringObject,
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::TriggeringObjectPower,
            },
        ),
    ]),
);

// DKA 90 — Fling
pub(in crate::card::sets) static FLING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6b144452-2e91-4e46-abe9-ed76b39f8314"),
    "Fling",
    crate::card::CardArt::new("cf1ab466-44bb-45d5-a94f-21b8924f0d89", "Izzy"),
    crate::card::CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a creature. Fling deals damage equal to the sacrificed creature's power to any target.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
            SpellAdditionalCostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Creature),
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::ObjectPower(ObjectRefDef::AdditionalCostObject(
                    AdditionalCostObjectIndex::PRIMARY,
                )),
            },
        ),
    ),
);

// DKA 91 — Forge Devil
pub(in crate::card::sets) static FORGE_DEVIL: CardRecord = CardRecord::new_with_legacy_id(
    722,
    "Forge Devil",
    CardArt::new("63b565a5-d706-47b4-bfa2-deebcc0e2e60", "Austin Hsu"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{R}"), &["Devil"], 1, 1).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, it deals 1 damage to target creature and 1 damage to you.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ),
);

// DKA 92 — Heckling Fiends
pub(in crate::card::sets) static HECKLING_FIENDS: CardRecord = CardRecord::new_with_legacy_id(
    723,
    "Heckling Fiends",
    CardArt::new("e9fd8895-9282-44d3-969f-b0529eb3bc07", "Clint Cearley"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Devil"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{R}: Target creature attacks this turn if able.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{R}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::attacks_each_combat_if_able()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// DKA 93 — Hellrider
pub(in crate::card::sets) static HELLRIDER: CardRecord = CardRecord::new_with_legacy_id(
    175,
    "Hellrider",
    CardArt::new("0ec8d800-7f06-44e0-b22d-cdff0a9b153d", "Svetlin Velinov"),
    CardSet::DarkAscension,
    CardRules::new_creature(
        mana_cost!("{2}{R}{R}"),
        &["Devil"],
        3,
        3,
    )
    .with_abilities(&[
        abilities::haste(),
        AbilityDef::triggered(
            "Whenever a creature you control attacks, this creature deals 1 damage to the player or planeswalker it's attacking.",
            TriggerEventDef::attacks(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::DealDamage {
                // With no planeswalkers in the game, the player an attacker is
                // attacking is always the defending player.
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// DKA 94 — Hinterland Hermit // Hinterland Scourge
pub(in crate::card::sets) static HINTERLAND_HERMIT: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("b6edac85-78e7-4e90-b538-b67c88bb5c62"),
    "Hinterland Hermit // Hinterland Scourge",
    crate::card::CardArt::new("b6edac85-78e7-4e90-b538-b67c88bb5c62", "Steven Belledin"),
    crate::card::CardSet::DarkAscension,
    &[
        (
            "Hinterland Hermit",
            CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Werewolf"], 2, 1)
                .with_ability(WEREWOLF_FRONT_TRANSFORM),
        ),
        (
            "Hinterland Scourge",
            CardRules::new_creature_without_mana_cost(&["Werewolf"], 3, 2)
                .printed_colors(&[ManaColor::Red])
                .with_abilities(&[
                    AbilityDef::static_ability(
                        "This creature must be blocked if able.",
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::Rule(AppliedRuleDef::MustBeBlockedBy(
                                ObjectPredicateDef::Any,
                            )),
                        },
                    ),
                    WEREWOLF_BACK_TRANSFORM,
                ]),
        ),
    ],
);

// DKA 95 — Increasing Vengeance
pub(in crate::card::sets) static INCREASING_VENGEANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d13afe4a-4a3d-42ae-ac0a-b789364c7e7e"),
    "Increasing Vengeance",
    crate::card::CardArt::new("d13afe4a-4a3d-42ae-ac0a-b789364c7e7e", "Anthony Francisco"),
    crate::card::CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{R}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Copy target instant or sorcery spell you control. If this spell was cast from a graveyard, copy that spell twice instead. You may choose new targets for the copies.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                zones: &[ZoneKind::Stack],
                controller: Some(PlayerRelation::You),
                owner: None,
            })],
            EffectDef::CopyStackObject(&CopyStackObjectDef {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                controller: PlayerRefDef::EffectController,
                count: value_if_condition!(
                    &CAST_FROM_GRAVEYARD,
                    ValueDef::Constant(2),
                    ValueDef::Constant(1)
                ),
                retarget: true,
                colors: None,
            }),
        ),
        abilities::flashback(mana_cost!("{3}{R}{R}")),
    ]),
);

// DKA 96 — Markov Blademaster
pub(in crate::card::sets) static MARKOV_BLADEMASTER: CardRecord = CardRecord::new_with_legacy_id(
    724,
    "Markov Blademaster",
    CardArt::new(
        "122163dd-e070-48af-8036-e9850541d138",
        "Jana Schirmer & Johannes Voss",
    ),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Vampire", "Warrior"], 1, 1)
        .with_abilities(&[
            abilities::double_strike(),
            AbilityDef::triggered(
                "Whenever this creature deals combat damage to a player, put a +1/+1 counter on it.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// DKA 97 — Markov Warlord
pub(in crate::card::sets) static MARKOV_WARLORD: CardRecord = CardRecord::new_with_legacy_id(
    1516,
    "Markov Warlord",
    CardArt::new("5035276f-31b9-4dd3-9ec8-42a664bdbd5c", "Cynthia Sheppard"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{5}{R}"), &["Vampire", "Warrior"], 4, 4).with_abilities(&[
        abilities::haste(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, up to two target creatures can't block this turn.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                2,
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DKA 98 — Mondronen Shaman // Tovolar's Magehunter
pub(in crate::card::sets) static MONDRONEN_SHAMAN: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("98fc475a-809d-4278-be03-86eab62b46ce"),
    "Mondronen Shaman // Tovolar's Magehunter",
    crate::card::CardArt::new("b150d71f-11c9-40d6-a461-4967ef437315", "Mike Sass"),
    crate::card::CardSet::DarkAscension,
    &[
        (
            "Mondronen Shaman",
            CardRules::new_creature(
                mana_cost!("{3}{R}"),
                &["Human", "Shaman", "Werewolf"],
                3,
                2,
            )
            .with_ability(WEREWOLF_FRONT_TRANSFORM),
        ),
        (
            "Tovolar's Magehunter",
            CardRules::new_creature_without_mana_cost(&["Werewolf"], 5, 5)
                .printed_colors(&[ManaColor::Red])
                .with_abilities(&[
                    AbilityDef::triggered(
                        "Whenever an opponent casts a spell, this creature deals 2 damage to that player.",
                        TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(
                            PlayerRelation::Opponent,
                        )),
                        EffectDef::DealDamage {
                            recipient: EffectRecipientDef::ControllerOfTriggeringObject,
                            amount: ValueDef::Constant(2),
                        },
                    ),
                    WEREWOLF_BACK_TRANSFORM,
                ]),
        ),
    ],
);

// DKA 99 — Moonveil Dragon
pub(in crate::card::sets) static MOONVEIL_DRAGON: CardRecord = CardRecord::new_with_legacy_id(
    725,
    "Moonveil Dragon",
    CardArt::new("92503118-b37b-4c52-b40a-487f6ad695ef", "Todd Lockwood"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{3}{R}{R}{R}"), &["Dragon"], 5, 5).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{R}: Each creature you control gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DKA 100 — Nearheath Stalker
pub(in crate::card::sets) static NEARHEATH_STALKER: CardRecord = CardRecord::new_with_legacy_id(
    726,
    "Nearheath Stalker",
    CardArt::new("7d4cdf4a-2d55-4769-8c51-bc86c13000ef", "Michael C. Hayes"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Vampire", "Rogue"], 4, 1)
        .with_ability(abilities::undying()),
);

// DKA 101 — Pyreheart Wolf
pub(in crate::card::sets) static PYREHEART_WOLF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9722f20c-e0d9-4165-8cd5-4abadc5378eb"),
    "Pyreheart Wolf",
    crate::card::CardArt::new("9722f20c-e0d9-4165-8cd5-4abadc5378eb", "Lars Grant-West"),
    crate::card::CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Wolf"], 1, 1).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks, creatures you control gain menace until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::menace()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::undying(),
    ]),
);

// DKA 102 — Russet Wolves
pub(in crate::card::sets) static RUSSET_WOLVES: CardRecord = CardRecord::new_with_legacy_id(
    727,
    "Russet Wolves",
    CardArt::new(
        "b3c7c972-5a11-4709-b3ef-e2acb3b51dd9",
        "Christopher Moeller",
    ),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Wolf"], 3, 3),
);

// DKA 103 — Scorch the Fields
pub(in crate::card::sets) static SCORCH_THE_FIELDS: CardRecord = CardRecord::new_with_legacy_id(
    728,
    "Scorch the Fields",
    CardArt::new("05c4338d-e5c0-46b4-ab16-1f9aa97b4026", "Jaime Jones"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{4}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target land. Scorch the Fields deals 1 damage to each Human creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Land),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Human"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// DKA 104 — Shattered Perception
pub(in crate::card::sets) static SHATTERED_PERCEPTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0f5223b4-e0d1-4fc6-a363-ebcdfeee56d1"),
    "Shattered Perception",
    crate::card::CardArt::new("0f5223b4-e0d1-4fc6-a363-ebcdfeee56d1", "Terese Nielsen"),
    crate::card::CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[
        AbilityDef::spell(
            "Discard all the cards in your hand, then draw that many cards.",
            EffectDef::Discard {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::CardsInHandAbove {
                    player: PlayerRelation::You,
                    threshold: 0,
                },
                selection: DiscardSelectionDef::RecipientChooses,
                then: Some(DiscardFollowUpDef {
                    counted: ObjectPredicateDef::Any,
                    bound: Some(ParentBinding),
                    effect: &EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::BoundObjectCount(ParentBinding),
                    },
                }),
            },
        ),
        abilities::flashback(mana_cost!("{5}{R}")),
    ]),
);

// DKA 105 — Talons of Falkenrath
pub(in crate::card::sets) static TALONS_OF_FALKENRATH: CardRecord = CardRecord::new_with_legacy_id(
    729,
    "Talons of Falkenrath",
    CardArt::new("f8e38239-a9ec-4149-9c90-74dcd46ed95d", "Svetlin Velinov"),
    CardSet::DarkAscension,
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature has \"{1}{R}: This creature gets +2/+0 until end of turn.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated(
                        "{1}{R}: This creature gets +2/+0 until end of turn.",
                        &[AbilityCostDef::Mana(mana_cost!("{1}{R}"))],
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(2),
                                ValueDef::Constant(0),
                            ),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    )),
                },
            ),
        ]),
);

// DKA 106 — Torch Fiend
pub(in crate::card::sets) static TORCH_FIEND: CardRecord = CardRecord::new_with_legacy_id(
    730,
    "Torch Fiend",
    CardArt::new("d596feee-6ccc-4648-884b-ed2eeb1cffc0", "Winona Nelson"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Devil"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{R}, Sacrifice this creature: Destroy target artifact.",
            &[
                AbilityCostDef::Mana(mana_cost!("{R}")),
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ),
);

// DKA 107 — Wrack with Madness
pub(in crate::card::sets) static WRACK_WITH_MADNESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4f27ee20-b5c5-4867-b832-9cdade0eda03"),
    "Wrack with Madness",
    crate::card::CardArt::new("4f27ee20-b5c5-4867-b832-9cdade0eda03", "Todd Lockwood"),
    crate::card::CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature deals damage to itself equal to its power.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::DealDamageFrom {
            source: ObjectRefDef::Target(TargetIndex::PRIMARY),
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
        },
    )),
);

// DKA 108 — Briarpack Alpha
pub(in crate::card::sets) static BRIARPACK_ALPHA: CardRecord = CardRecord::new_with_legacy_id(
    731,
    "Briarpack Alpha",
    CardArt::new("a052e945-7535-4b0a-b580-cf76377633f3", "Daarken"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Wolf"], 3, 3).with_abilities(&[
        abilities::flash(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, target creature gets +2/+2 until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DKA 109 — Clinging Mists
static CLINGING_MISTS_ATTACKERS: EffectRecipientDef = EffectRecipientDef::matching_objects(
    ObjectPredicateDef::Attacking,
    &[ZoneKind::Battlefield],
    PlayerRelation::Any,
);

pub(in crate::card::sets) static CLINGING_MISTS: CardRecord = CardRecord::new_with_legacy_id(
    1884,
    "Clinging Mists",
    CardArt::new("e0152975-790a-40e4-993e-a970676a2d32", "Anthony Francisco"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell(
        "Prevent all combat damage that would be dealt this turn. Fateful hour — If you have 5 \
         or less life, tap all attacking creatures. Those creatures don't untap during their \
         controller's next untap step.",
        // The Fog happens either way; only the tap is behind the threshold.
        EffectDef::Sequence(&[
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::COMBAT),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::IfCondition {
                condition: &FATEFUL_HOUR,
                // The tap and the skip land on the same set, and the skip is counted per
                // permanent so each creature misses its own controller's step.
                then: &EffectDef::Sequence(&[
                    EffectDef::Tap {
                        object: CLINGING_MISTS_ATTACKERS,
                    },
                    EffectDef::SkipNextUntapSteps {
                        object: CLINGING_MISTS_ATTACKERS,
                        count: 1,
                    },
                ]),
            },
        ]),
    )),
);

// DKA 110 — Crushing Vines
pub(in crate::card::sets) static CRUSHING_VINES: CardRecord = CardRecord::new_with_legacy_id(
    732,
    "Crushing Vines",
    CardArt::new("c59b3653-5a50-48f2-bcf1-ab305ef30902", "Scott Chou"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{2}{G}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Destroy target creature with flying.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
                    ]),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
            ),
            AbilityDef::spell_with_targets(
                "Destroy target artifact.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
            ),
        ],
    )),
);

// DKA 111 — Dawntreader Elk
pub(in crate::card::sets) static DAWNTREADER_ELK: CardRecord = CardRecord::new_with_legacy_id(
    1601,
    "Dawntreader Elk",
    CardArt::new("127c969b-1c9a-4265-af0e-5b9dbe136064", "John Avon"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elk"], 2, 2).with_ability(
        AbilityDef::activated(
            "{G}, Sacrifice this creature: Search your library for a basic land card, put it \
             onto the battlefield tapped, then shuffle.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}")),
                AbilityCostDef::SacrificeSource,
            ],
            FETCH_A_BASIC_TAPPED,
        ),
    ),
);

// DKA 112 — Deranged Outcast
pub(in crate::card::sets) static DERANGED_OUTCAST: CardRecord = CardRecord::new_with_legacy_id(
    733,
    "Deranged Outcast",
    CardArt::new("e2b35fee-8e24-4d89-ad77-d55d06bb1d7f", "Nils Hamm"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Rogue"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{G}, Sacrifice a Human: Put two +1/+1 counters on target creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{G}")),
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Human"),
                    ]),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// DKA 113 — Favor of the Woods
// Audit: unsupported — Trigger capture has no event for the attached creature becoming a blocker.
pub(in crate::card::sets) static FAVOR_OF_THE_WOODS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("da0e760a-241b-40c5-b201-bfc03effed2e"),
    "Favor of the Woods",
    crate::card::CardArt::new("da0e760a-241b-40c5-b201-bfc03effed2e", "Tomasz Jedruszek"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 114 — Feed the Pack
pub(in crate::card::sets) static FEED_THE_PACK: CardRecord = CardRecord::new_with_legacy_id(
    1971,
    "Feed the Pack",
    CardArt::new("9831e3cc-659b-4408-b5d8-a27ae2738680", "Steve Prescott"),
    CardSet::DarkAscension,
    CardRules::new_enchantment(mana_cost!("{5}{B}{B}")).with_ability(AbilityDef::triggered(
        "At the beginning of your end step, you may sacrifice a nontoken creature. If you do, create X 2/2 green Wolf creature tokens, where X is the sacrificed creature's toughness.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::You,
        },
        EffectDef::SacrificeOfChoice {
            count: ValueDef::Constant(1),
            player: EffectRecipientDef::Controller,
            // Nontoken, so the Wolves it makes cannot feed it back.
            // Nontoken, so the Wolves it makes cannot feed it back.
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
            ]),
            // One Wolf per point of toughness, which is why the food of choice is a
            // Wall: the pack it makes is worth far more than what fed it.
            then: Some(&EffectDef::create_creature_token(&["Wolf"], &[ManaColor::Green], 2, 2)
                    .with_art(CardArt::new(
                        "a53f8031-aaa8-424c-929a-5478538a8cc6",
                        "David Palumbo",
                    ))
                    .with_count(ValueDef::TriggerEventAmount)),
            amount: SacrificedAmountDef::Toughness,
            otherwise: None,
            optional: true,
        },
    )),
);

// DKA 115 — Ghoultree
pub(in crate::card::sets) static GHOULTREE: CardRecord = CardRecord::new_with_legacy_id(
    734,
    "Ghoultree",
    CardArt::new("a413c65e-5965-429b-8c25-11f8b73cba03", "Volkan Baǵa"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{7}{G}"), &["Zombie", "Treefolk"], 10, 10).with_ability(
        AbilityDef::static_ability(
            "This spell costs {1} less to cast for each creature card in your graveyard.",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                &ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ),
            )),
        )
        .with_source_zones(&[ZoneKind::Hand]),
    ),
);

// DKA 116 — Gravetiller Wurm
pub(in crate::card::sets) static GRAVETILLER_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("66d9fe36-2eac-49e3-8f89-810009ba8a4b"),
    "Gravetiller Wurm",
    crate::card::CardArt::new("66d9fe36-2eac-49e3-8f89-810009ba8a4b", "Slawomir Maniak"),
    crate::card::CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Wurm"], 4, 4).with_abilities(&[
        abilities::trample(),
        morbid_entry_counters(
            "Morbid — This creature enters with four +1/+1 counters on it if a creature died this turn.",
            4,
        ),
    ]),
);

// DKA 117 — Grim Flowering
pub(in crate::card::sets) static GRIM_FLOWERING: CardRecord = CardRecord::new_with_legacy_id(
    735,
    "Grim Flowering",
    CardArt::new("e5f3e2ad-7a04-4735-ba73-576e32249ba3", "Adam Paquette"),
    CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{5}{G}")).with_ability(AbilityDef::spell(
        "Draw a card for each creature card in your graveyard.",
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Graveyard],
                PlayerRelation::You,
            )),
        },
    )),
);

// DKA 118 — Hollowhenge Beast
pub(in crate::card::sets) static HOLLOWHENGE_BEAST: CardRecord = CardRecord::new_with_legacy_id(
    736,
    "Hollowhenge Beast",
    CardArt::new("052ab91f-ac01-43f4-9276-9af35dbfbf71", "Dave Kendall"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Beast"], 5, 5),
);

// DKA 119 — Hunger of the Howlpack
pub(in crate::card::sets) static HUNGER_OF_THE_HOWLPACK: CardRecord = CardRecord::new_with_legacy_id(
    737,
    "Hunger of the Howlpack",
    CardArt::new("b38a0dbc-3ebd-4f87-a5fb-bc2ee8a48a8d", "Nils Hamm"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Put a +1/+1 counter on target creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::IfCreatureDiedThisTurn(&ConditionalValueDef {
                        then: ValueDef::Constant(3),
                        otherwise: ValueDef::Constant(1),
                    }),
            },
        ),
        AbilityDef::static_ability(
            "Morbid — Put three +1/+1 counters on that creature instead if a creature died this turn.",
            EffectDef::None,
        ),
    ]),
);

// DKA 120 — Increasing Savagery
pub(in crate::card::sets) static INCREASING_SAVAGERY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29ab8737-151f-4702-a95d-7f7b60a5ee8a"),
    "Increasing Savagery",
    crate::card::CardArt::new("29ab8737-151f-4702-a95d-7f7b60a5ee8a", "Steve Prescott"),
    crate::card::CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{2}{G}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Put five +1/+1 counters on target creature. If this spell was cast from a graveyard, put ten +1/+1 counters on that creature instead.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: value_if_condition!(
                    &CAST_FROM_GRAVEYARD,
                    ValueDef::Constant(10),
                    ValueDef::Constant(5)
                ),
            },
        ),
        abilities::flashback(mana_cost!("{5}{G}{G}")),
    ]),
);

// DKA 121 — Kessig Recluse
pub(in crate::card::sets) static KESSIG_RECLUSE: CardRecord = CardRecord::new_with_legacy_id(
    738,
    "Kessig Recluse",
    CardArt::new("695b8abe-796e-4d9b-aad3-4e03e925d2a7", "Vincent Proce"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Spider"], 2, 3)
        .with_abilities(&[abilities::reach(), abilities::deathtouch()]),
);

// DKA 122 — Lambholt Elder // Silverpelt Werewolf
pub(in crate::card::sets) static LAMBHOLT_ELDER: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("932d753d-9584-4ad8-9a5e-a3524184f961"),
    "Lambholt Elder // Silverpelt Werewolf",
    crate::card::CardArt::new("932d753d-9584-4ad8-9a5e-a3524184f961", "Matt Stewart"),
    crate::card::CardSet::DarkAscension,
    &[
        (
            "Lambholt Elder",
            CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Werewolf"], 1, 2)
                .with_ability(WEREWOLF_FRONT_TRANSFORM),
        ),
        (
            "Silverpelt Werewolf",
            CardRules::new_creature_without_mana_cost(&["Werewolf"], 4, 5)
                .printed_colors(&[ManaColor::Green])
                .with_abilities(&[
                    AbilityDef::triggered(
                        "Whenever this creature deals combat damage to a player, draw a card.",
                        TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(1),
                        },
                    ),
                    WEREWOLF_BACK_TRANSFORM,
                ]),
        ),
    ],
);

// DKA 123 — Lost in the Woods
// Audit: unsupported — Needs a per-attacker top-card reveal, a Forest-card branch that removes that attacker from combat, and bottom placement.
pub(in crate::card::sets) static LOST_IN_THE_WOODS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5865603c-0a5e-45c3-84e3-2dc3b4cf0cf7"),
    "Lost in the Woods",
    crate::card::CardArt::new("5865603c-0a5e-45c3-84e3-2dc3b4cf0cf7", "Matt Stewart"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 124 — Predator Ooze
pub(in crate::card::sets) static PREDATOR_OOZE: CardRecord = CardRecord::new_with_legacy_id(
    739,
    "Predator Ooze",
    CardArt::new("73c71457-f7c9-4ab4-b89d-e235e3f15e16", "Ryan Yee"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{G}{G}{G}"), &["Ooze"], 1, 1).with_abilities(&[
        abilities::indestructible(),
        AbilityDef::triggered(
            "Whenever this creature attacks, put a +1/+1 counter on it.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::creature_damaged_by_source_dies_trigger(
            "Whenever a creature dealt damage by this creature this turn dies, put a +1/+1 counter on this creature.",
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// DKA 125 — Scorned Villager // Moonscarred Werewolf
pub(in crate::card::sets) static SCORNED_VILLAGER: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("6f35e364-81d9-4888-993b-acc7a53d963c"),
    "Scorned Villager // Moonscarred Werewolf",
    crate::card::CardArt::new("6f35e364-81d9-4888-993b-acc7a53d963c", "Cynthia Sheppard"),
    crate::card::CardSet::DarkAscension,
    &[
        (
            "Scorned Villager",
            CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Werewolf"], 1, 1)
                .with_abilities(&[
                    abilities::tap_for(ManaColor::Green),
                    WEREWOLF_FRONT_TRANSFORM,
                ]),
        ),
        (
            "Moonscarred Werewolf",
            CardRules::new_creature_without_mana_cost(&["Werewolf"], 2, 2)
                .printed_colors(&[ManaColor::Green])
                .with_abilities(&[
                    abilities::vigilance(),
                    AbilityDef::activated_mana(
                        "{T}: Add {G}{G}.",
                        &[AbilityCostDef::TapSource],
                        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green).with_amount(2)),
                    ),
                    WEREWOLF_BACK_TRANSFORM,
                ]),
        ),
    ],
);

// DKA 126 — Somberwald Dryad
pub(in crate::card::sets) static SOMBERWALD_DRYAD: CardRecord = CardRecord::new_with_legacy_id(
    740,
    "Somberwald Dryad",
    CardArt::new("307edca0-769d-4071-9654-3537341e96bd", "Jaime Jones"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Dryad"], 2, 2)
        .with_ability(abilities::forestwalk()),
);

// DKA 127 — Strangleroot Geist
pub(in crate::card::sets) static STRANGLEROOT_GEIST: CardRecord = CardRecord::new_with_legacy_id(
    219,
    "Strangleroot Geist",
    CardArt::new("bf1fb137-205c-480f-b6dc-dfa137793ae3", "Jason Chan"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Spirit"], 2, 1)
        .with_abilities(&[abilities::haste(), abilities::undying()]),
);

// DKA 128 — Tracker's Instincts
pub(in crate::card::sets) static TRACKER_S_INSTINCTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("59960387-3adf-4b9a-b0e6-c441579f7388"),
    "Tracker's Instincts",
    crate::card::CardArt::new("59960387-3adf-4b9a-b0e6-c441579f7388", "Jung Park"),
    crate::card::CardSet::DarkAscension,
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[
        AbilityDef::spell(
            "Reveal the top four cards of your library. Put a creature card from among them into your hand and the rest into your graveyard.",
            abilities::reveal_top_cards_choose_to_hand_rest_graveyard(
                ValueDef::Constant(4),
                ObjectPredicateDef::HasType(CardType::Creature),
                0,
                1,
            ),
        ),
        abilities::flashback(mana_cost!("{2}{U}")),
    ]),
);

// DKA 129 — Ulvenwald Bear
pub(in crate::card::sets) static ULVENWALD_BEAR: CardRecord = CardRecord::new_with_legacy_id(
    1852,
    "Ulvenwald Bear",
    CardArt::new("9e3837a7-854a-440d-93d7-d36f50149346", "Jason A. Engle"),
    CardSet::DarkAscension,
    // The same intervening if, and it matters more here: an uncreated
    // trigger asks for no target, so nothing is put on the stack pointing at
    // a creature.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Bear"], 2, 2).with_ability(
        AbilityDef::triggered_if_with_targets(
            "Morbid — When this creature enters, if a creature died this turn, put two +1/+1 \
             counters on target creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &MORBID_A_CREATURE_DIED,
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// DKA 130 — Village Survivors
pub(in crate::card::sets) static VILLAGE_SURVIVORS: CardRecord = CardRecord::new_with_legacy_id(
    1885,
    "Village Survivors",
    CardArt::new("13c0e852-9966-4145-b7c3-ac957f729376", "David Rapoza"),
    CardSet::DarkAscension,
    // It has vigilance outright; the fateful-hour clause hands that out to
    // everything else, so the printed keyword is not what the branch reads.
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Human"], 4, 5).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::static_ability(
            "Fateful hour — As long as you have 5 or less life, other creatures you control \
             have vigilance.",
            EffectDef::IfCondition {
                condition: &FATEFUL_HOUR,
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::vigilance()),
                },
            },
        ),
    ]),
);

// DKA 131 — Vorapede
pub(in crate::card::sets) static VORAPEDE: CardRecord = CardRecord::new_with_legacy_id(
    741,
    "Vorapede",
    CardArt::new("1348aa65-85e7-4ac7-bcdb-a83f2c3aa1a6", "Slawomir Maniak"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{2}{G}{G}{G}"), &["Insect"], 5, 4).with_abilities(&[
        abilities::vigilance(),
        abilities::trample(),
        abilities::undying(),
    ]),
);

// DKA 132 — Wild Hunger
pub(in crate::card::sets) static WILD_HUNGER: CardRecord = CardRecord::new_with_legacy_id(
    742,
    "Wild Hunger",
    CardArt::new("a564e8d4-4111-4d8e-897d-523bc4cfef94", "Karl Kopinski"),
    CardSet::DarkAscension,
    CardRules::new_instant(mana_cost!("{2}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature gets +3/+1 and gains trample until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
        abilities::flashback(mana_cost!("{3}{R}")),
    ]),
);

// DKA 133 — Wolfbitten Captive // Krallenhorde Killer
// Audit: unsupported — Needs a once-per-turn activation limit shared with each face's distinct self-pump ability in a transforming Werewolf composition.
pub(in crate::card::sets) static WOLFBITTEN_CAPTIVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1303e02a-ef69-4817-bca5-02c74774b811"),
    "Wolfbitten Captive",
    crate::card::CardArt::new("1303e02a-ef69-4817-bca5-02c74774b811", "Zoltan Boros"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 134 — Young Wolf
pub(in crate::card::sets) static YOUNG_WOLF: CardRecord = CardRecord::new_with_legacy_id(
    743,
    "Young Wolf",
    CardArt::new("0c39aa40-ef5f-40f1-a6dd-fbce91172c50", "Ryan Pancoast"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{G}"), &["Wolf"], 1, 1).with_ability(abilities::undying()),
);

// DKA 135 — Diregraf Captain
pub(in crate::card::sets) static DIREGRAF_CAPTAIN: CardRecord = CardRecord::new_with_legacy_id(
    744,
    "Diregraf Captain",
    CardArt::new("0e5f41eb-609b-4882-af9e-904daa717484", "Slawomir Maniak"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{U}{B}"), &["Zombie", "Soldier"], 2, 2).with_abilities(
        &[
            abilities::deathtouch(),
            AbilityDef::static_ability(
                "Other Zombie creatures you control get +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Zombie"),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::triggered_with_targets(
                "Whenever another Zombie you control dies, target opponent loses 1 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Zombie"),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            ),
        ],
    ),
);

// DKA 136 — Drogskol Captain
pub(in crate::card::sets) static DROGSKOL_CAPTAIN: CardRecord = CardRecord::new_with_legacy_id(
    745,
    "Drogskol Captain",
    CardArt::new("b8238e36-625f-460d-9e39-fd501e65490c", "Peter Mohrbacher"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{W}{U}"), &["Spirit", "Soldier"], 2, 2)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::static_ability(
                "Other Spirit creatures you control get +1/+1 and have hexproof. (They can't be the targets of spells or abilities your opponents control.)",
                EffectDef::Sequence(&[
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Subtype("Spirit"),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]), &[ZoneKind::Battlefield], PlayerRelation::You),
                        effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
                    },
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Subtype("Spirit"),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]), &[ZoneKind::Battlefield], PlayerRelation::You),
                        effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                    },
                ]),
            ),
        ]),
);

// DKA 137 — Drogskol Reaver
pub(in crate::card::sets) static DROGSKOL_REAVER: CardRecord = CardRecord::new_with_legacy_id(
    746,
    "Drogskol Reaver",
    CardArt::new("af2d9e0b-6433-40a2-9847-9fa4e3c008c4", "Vincent Proce"),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{5}{W}{U}"), &["Spirit"], 3, 5).with_abilities(&[
        abilities::flying(),
        abilities::double_strike(),
        abilities::lifelink(),
        AbilityDef::triggered(
            "Whenever you gain life, draw a card.",
            TriggerEventDef::LifeGained(PlayerRelation::You),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// DKA 138 — Falkenrath Aristocrat
// Audit: unsupported — Needs an activated sacrifice cost to expose whether the chosen creature was Human for the conditional +1/+1 counter.
pub(in crate::card::sets) static FALKENRATH_ARISTOCRAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c397388e-ebe2-4034-83b7-a7c0df1af78f"),
    "Falkenrath Aristocrat",
    crate::card::CardArt::new("c397388e-ebe2-4034-83b7-a7c0df1af78f", "Igor Kieryluk"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 139 — Havengul Lich
// Audit: unsupported — Needs temporary graveyard-casting permission for a targeted creature card and a later cast trigger that copies all of that card's activated abilities.
pub(in crate::card::sets) static HAVENGUL_LICH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("321077a4-e468-4e74-94f7-80c83790e0d9"),
    "Havengul Lich",
    crate::card::CardArt::new("321077a4-e468-4e74-94f7-80c83790e0d9", "James Ryman"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 140 — Huntmaster of the Fells // Ravager of the Fells
static HUNTMASTER_WOLF_AND_LIFE: EffectDef = EffectDef::Sequence(&[
    EffectDef::create_creature_token(&["Wolf"], &[ManaColor::Green], 2, 2).with_art(CardArt::new(
        "a53f8031-aaa8-424c-929a-5478538a8cc6",
        "David Palumbo",
    )),
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    },
]);

pub(in crate::card::sets) static HUNTMASTER_OF_THE_FELLS: CardRecord =
    CardRecord::new_dfc_with_legacy_id(
        176,
        "Huntmaster of the Fells // Ravager of the Fells",
        CardArt::new("aae6fb12-b252-453b-bca7-1ea2a0d6c8dc", "Chris Rahn"),
        CardSet::DarkAscension,
        &[
            (
                "Huntmaster of the Fells",
                const {
                    CardRules::new_creature(mana_cost!("{2}{R}{G}"), &const { ["Human", "Werewolf"] }, 2, 2)
                    // Entering and transforming into this face do the same thing, so the printed
                    // sentence is two triggers watching two different events.
                    .with_abilities(&const { [
                        abilities::enters_trigger(
                            "Whenever this creature enters, create a 2/2 green Wolf creature token and you gain 2 life.",
                            HUNTMASTER_WOLF_AND_LIFE,
                        ),
                        AbilityDef::triggered(
                            "Whenever this creature transforms into Huntmaster of the Fells, create a 2/2 green Wolf creature token and you gain 2 life.",
                            TriggerEventDef::transforms(ObjectPredicateDef::Source),
                            HUNTMASTER_WOLF_AND_LIFE,
                        ),
                        AbilityDef::triggered_if(
                            "At the beginning of each upkeep, if no spells were cast last turn, transform this creature.",
                            TriggerEventDef::StepBegins {
                                step: TurnStepDef::Upkeep,
                                player: PlayerRelation::Any,
                            },
                            // Nobody cast anything, so every player has to be at zero.
                            &TriggerConditionDef::SpellsCastLastTurn {
                                quantifier: QuantifierDef::Every,
                                player: PlayerRelation::Any,
                                comparison: ComparisonDef::LessOrEqual,
                                amount: 0,
                            },
                            EffectDef::Transform {
                                object: EffectRecipientDef::Source,
                            },
                        ),
                    ] })
                },
            ),
            (
                "Ravager of the Fells",
                const {
                    CardRules::new_creature_without_mana_cost(&const { ["Werewolf"] }, 4, 4)
                    .printed_colors(&const { [ManaColor::Red, ManaColor::Green] })
                    .with_abilities(&const { [
                        abilities::trample(),
                        AbilityDef::triggered_with_targets(
                            "Whenever this creature transforms into Ravager of the Fells, it deals 2 damage to target opponent or planeswalker and 2 damage to up to one target creature that player or that planeswalker's controller controls.",
                            TriggerEventDef::transforms(ObjectPredicateDef::Source),
                            // The second slot reads the first: the creature has to belong to whoever the
                            // damage was aimed at.
                            &const { [
                                AbilityTargetDef::exactly_one(AbilityTargetPredicate::PlayerOrPlaneswalker(
                                    PlayerRelation::Opponent,
                                )),
                                AbilityTargetDef::up_to(
                                    AbilityTargetPredicate::ControlledByTargetOf {
                                        object: ObjectPredicateDef::HasType(CardType::Creature),
                                        slot: TargetIndex::PRIMARY,
                                    },
                                    1,
                                ),
                            ] },
                            EffectDef::Sequence(&const { [
                                EffectDef::DealDamage {
                                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                    amount: ValueDef::Constant(2),
                                },
                                EffectDef::DealDamage {
                                    recipient: EffectRecipientDef::Target(TargetIndex(1)),
                                    amount: ValueDef::Constant(2),
                                },
                            ] }),
                        ),
                        AbilityDef::triggered_if(
                            "At the beginning of each upkeep, if a player cast two or more spells last turn, transform this creature.",
                            TriggerEventDef::StepBegins {
                                step: TurnStepDef::Upkeep,
                                player: PlayerRelation::Any,
                            },
                            // One player is enough, which is why this side turns back sooner than the
                            // other side turns over.
                            &TriggerConditionDef::SpellsCastLastTurn {
                                quantifier: QuantifierDef::Any,
                                player: PlayerRelation::Any,
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 2,
                            },
                            EffectDef::Transform {
                                object: EffectRecipientDef::Source,
                            },
                        ),
                    ] })
                },
            ),
        ],
    );

// DKA 141 — Immerwolf
// Audit: unsupported — Needs a continuous prohibition preventing non-Human Werewolves you control from transforming.
pub(in crate::card::sets) static IMMERWOLF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9326061f-ea76-4be7-a06f-aefb63454777"),
    "Immerwolf",
    crate::card::CardArt::new("9326061f-ea76-4be7-a06f-aefb63454777", "Terese Nielsen"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 142 — Sorin, Lord of Innistrad
// Audit: unsupported — Needs its creator-owned emblem ability, Vampire token effect, and an ultimate continuation that returns only the permanents destroyed this way under your control.
pub(in crate::card::sets) static SORIN_LORD_OF_INNISTRAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("27bb371f-d49f-41bd-bbe0-d5e1e2067e36"),
    "Sorin, Lord of Innistrad",
    crate::card::CardArt::new("27bb371f-d49f-41bd-bbe0-d5e1e2067e36", "Michael Komarck"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 143 — Stromkirk Captain
pub(in crate::card::sets) static STROMKIRK_CAPTAIN: CardRecord = CardRecord::new_with_legacy_id(
    747,
    "Stromkirk Captain",
    CardArt::new(
        "5bfcca87-04f8-480a-bae6-ae87f7afb7e1",
        "Jana Schirmer & Johannes Voss",
    ),
    CardSet::DarkAscension,
    CardRules::new_creature(mana_cost!("{1}{B}{R}"), &["Vampire", "Soldier"], 2, 2).with_abilities(
        &[
            abilities::first_strike(),
            AbilityDef::static_ability(
                "Other Vampire creatures you control get +1/+1 and have first strike.",
                EffectDef::Sequence(&[
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Subtype("Vampire"),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                    },
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Subtype("Vampire"),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                    },
                ]),
            ),
        ],
    ),
);

// DKA 144 — Altar of the Lost
// Audit: unsupported — Needs two-mana any-color combination choice plus spending provenance restricted to flashback spells cast from graveyards.
pub(in crate::card::sets) static ALTAR_OF_THE_LOST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("774e5322-1b41-488d-94b1-7742fbd983d4"),
    "Altar of the Lost",
    crate::card::CardArt::new("774e5322-1b41-488d-94b1-7742fbd983d4", "Daarken"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 145 — Avacyn's Collar
pub(in crate::card::sets) static AVACYNS_COLLAR: CardRecord = CardRecord::new_with_legacy_id(
    2307,
    "Avacyn's Collar",
    CardArt::new("972e9a78-204b-4012-b394-b40fd0edac4c", "James Paick"),
    CardSet::DarkAscension,
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+0 and has vigilance.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(0)),
                        AppliedEffectDef::add_ability(&abilities::vigilance()),
                    ]),
                },
            ),
            AbilityDef::triggered(
                "Whenever equipped creature dies, if it was a Human, create a 1/1 white Spirit creature token with flying.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AttachedToSource,
                        ObjectPredicateDef::Subtype("Human"),
                    ]),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::create_creature_token(&["Spirit"], &[ManaColor::White], 1, 1).with_abilities(&[abilities::flying()]).with_art(CardArt::new(
                        "59e79ba0-33c8-46c8-8694-8bf854345fe7",
                        "Ryan Yee",
                    )),
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// DKA 146 — Chalice of Life // Chalice of Death
// Audit: unsupported — Needs an activation-resolution life-total threshold that transforms the source, plus the complete back-face mana ability.
pub(in crate::card::sets) static CHALICE_OF_LIFE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d9c1c46-7aa7-464c-87b0-b29b9663daef"),
    "Chalice of Life",
    crate::card::CardArt::new("9d9c1c46-7aa7-464c-87b0-b29b9663daef", "Ryan Yee"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 147 — Elbrus, the Binding Blade // Withengar Unbound
// Audit: unsupported — Penta's two-player game terminates before Withengar's player-loses trigger could resolve.
pub(in crate::card::sets) static ELBRUS_THE_BINDING_BLADE: CardRecord =
    CardRecord::new_dfc_with_legacy_id(
        2313,
        "Elbrus, the Binding Blade // Withengar Unbound",
        CardArt::new("683af377-c491-4f62-900c-6b83d75c33c9", "Eric Deschamps"),
        CardSet::DarkAscension,
        &[
            (
                "Elbrus, the Binding Blade",
                const { CardRules::unsupported() },
            ),
            ("Withengar Unbound", const { CardRules::unsupported() }),
        ],
    );

// DKA 148 — Executioner's Hood
pub(in crate::card::sets) static EXECUTIONERS_HOOD: CardRecord = CardRecord::new_with_legacy_id(
    1924,
    "Executioner's Hood",
    CardArt::new("7447d115-9b29-4086-8435-40c7c957f242", "Anthony Palumbo"),
    CardSet::DarkAscension,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has intimidate.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::intimidate()),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{2}"))],
                "Equip {2} ({2}: Attach to target creature you control. Equip only as a \
                 sorcery.)",
            ),
        ]),
);

// DKA 149 — Grafdigger's Cage
// Audit: unsupported — Needs zone-origin-sensitive casting prohibitions and a replacement that stops creature cards in graveyards or libraries entering the battlefield.
pub(in crate::card::sets) static GRAFDIGGER_S_CAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2a6240e7-d3aa-40e9-a627-58e7bf62525c"),
    "Grafdigger's Cage",
    crate::card::CardArt::new("2a6240e7-d3aa-40e9-a627-58e7bf62525c", "Daniel Ljunggren"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 150 — Heavy Mattock
pub(in crate::card::sets) static HEAVY_MATTOCK: CardRecord = CardRecord::new_with_legacy_id(
    1925,
    "Heavy Mattock",
    CardArt::new("8b09df01-0b3e-463e-bf00-0a9f1822261a", "Winona Nelson"),
    CardSet::DarkAscension,
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::static_ability(
                "As long as equipped creature is a Human, it gets an additional +1/+1.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::AttachedPermanentMatches {
                        object: ObjectPredicateDef::Subtype("Human"),
                    },
                    // "An additional +1/+1", so this is a second modifier on top of the printed
                    // one rather than a replacement for it.
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                    },
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{2}"))],
                "Equip {2} ({2}: Attach to target creature you control. Equip only as a \
                 sorcery.)",
            ),
        ]),
);

// DKA 151 — Helvault
pub(in crate::card::sets) static HELVAULT: CardRecord = CardRecord::new_with_legacy_id(
    748,
    "Helvault",
    CardArt::new("16d2448c-1b2e-466a-a0ab-e20ba1de6bc9", "Jaime Jones"),
    CardSet::DarkAscension,
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "{1}, {T}: Exile target creature you control.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{1}")),
                    AbilityCostDef::TapSource,
                ],
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                })],
                EffectDef::ExileLinkedToSource {
                    until_source_leaves: false,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
face_down: false,
then: None,
},
            ),
            AbilityDef::activated_with_targets(
                "{7}, {T}: Exile target creature you don't control.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{7}")),
                    AbilityCostDef::TapSource,
                ],
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                })],
                EffectDef::ExileLinkedToSource {
                    until_source_leaves: false,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
face_down: false,
then: None,
},
            ),
            abilities::dies_trigger("When Helvault is put into a graveyard from the battlefield, return all cards exiled with it to the battlefield under their owners' control.", EffectDef::ReturnLinkedExiles {
                    object: ObjectPredicateDef::Any,
                    counters: None,
                    zone: ZoneKind::Battlefield,
                    grant: None,
                    controller: None,
                    transformed: false,
                }),
        ]),
);

// DKA 152 — Jar of Eyeballs
// Audit: unsupported — Needs a “remove all” counter cost whose removed count is retained as X for a later top-card selection.
pub(in crate::card::sets) static JAR_OF_EYEBALLS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("72825270-d5c1-4ab1-903a-e2868ade17f2"),
    "Jar of Eyeballs",
    crate::card::CardArt::new("72825270-d5c1-4ab1-903a-e2868ade17f2", "Jaime Jones"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 153 — Warden of the Wall
// Audit: unsupported — Needs a continuous animation active only during turns other than its controller's, while preserving the tapped entry and mana ability.
pub(in crate::card::sets) static WARDEN_OF_THE_WALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd1a1f48-d46b-4b8e-a642-fc70fd9ef7df"),
    "Warden of the Wall",
    crate::card::CardArt::new("fd1a1f48-d46b-4b8e-a642-fc70fd9ef7df", "Daniel Ljunggren"),
    crate::card::CardSet::DarkAscension,
    crate::card::CardRules::unsupported(),
);

// DKA 154 — Wolfhunter's Quiver
pub(in crate::card::sets) static WOLFHUNTERS_QUIVER: CardRecord = CardRecord::new_with_legacy_id(
    1930,
    "Wolfhunter's Quiver",
    CardArt::new("d84c9b19-9b4d-4a60-984f-636b749c8bcc", "Daniel Ljunggren"),
    CardSet::DarkAscension,
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has \"{T}: This creature deals 1 damage to any target\" and \
                 \"{T}: This creature deals 3 damage to target Werewolf creature.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    // Two abilities, not one with a choice: each costs the creature's own tap,
                    // so only one of them can be used per untap.
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets(
                            "{T}: This creature deals 1 damage to any target.",
                            &[AbilityCostDef::TapSource],
                            &[AbilityTargetDef::exactly_one(
                                AbilityTargetPredicate::AnyTarget,
                            )],
                            EffectDef::DealDamage {
                                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                amount: ValueDef::Constant(1),
                            },
                        )),
                        AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets(
                            "{T}: This creature deals 3 damage to target Werewolf creature.",
                            &[AbilityCostDef::TapSource],
                            &[AbilityTargetDef::exactly_one_permanent(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::Subtype("Werewolf"),
                                ]),
                            )],
                            EffectDef::DealDamage {
                                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                amount: ValueDef::Constant(3),
                            },
                        )),
                    ]),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{5}"))],
                "Equip {5} ({5}: Attach to target creature you control. Equip only as a \
                 sorcery.)",
            ),
        ]),
);

// DKA 155 — Evolving Wilds
pub(in crate::card::sets) static EVOLVING_WILDS: CardRecord = CardRecord::new_with_legacy_id(
    1602,
    "Evolving Wilds",
    CardArt::new("30066306-f943-44c1-8814-b8b60388c26d", "Cliff Childs"),
    CardSet::DarkAscension,
    CardRules::new_land(&[]).with_ability(AbilityDef::activated(
        "{T}, Sacrifice this land: Search your library for a basic land card, put it onto the \
         battlefield tapped, then shuffle.",
        &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
        FETCH_A_BASIC_TAPPED,
    )),
);

// DKA 156 — Grim Backwoods
pub(in crate::card::sets) static GRIM_BACKWOODS: CardRecord = CardRecord::new_with_legacy_id(
    749,
    "Grim Backwoods",
    CardArt::new("045abeeb-f5e5-4f3f-9836-5b1553e03f11", "Vincent Proce"),
    CardSet::DarkAscension,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{2}{B}{G}, {T}, Sacrifice a creature: Draw a card.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{B}{G}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// DKA 157 — Haunted Fengraf
pub(in crate::card::sets) static HAUNTED_FENGRAF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("84bca9e1-c0b7-4dce-9ee4-370db2c322b6"),
    "Haunted Fengraf",
    crate::card::CardArt::new("84bca9e1-c0b7-4dce-9ee4-370db2c322b6", "Adam Paquette"),
    crate::card::CardSet::DarkAscension,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{3}, {T}, Sacrifice this land: Return a creature card at random from your graveyard to your hand.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            EffectDef::Sequence(&[
                EffectDef::BindOutput {
                    effect: &EffectDef::SelectAtRandomFromZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Graveyard,
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        amount: ValueDef::Constant(1),
                    },
                    binding: Binding!("haunted_fengraf_card"),
                },
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                        Binding!("haunted_fengraf_card"),
                    )),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            ]),
        ),
    ]),
);

// DKA 158 — Vault of the Archangel
pub(in crate::card::sets) static VAULT_OF_THE_ARCHANGEL: CardRecord = CardRecord::new_with_legacy_id(
    237,
    "Vault of the Archangel",
    CardArt::new("35a65437-430a-42ef-854f-6e66f8e1a04a", "John Avon"),
    CardSet::DarkAscension,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{2}{W}{B}, {T}: Creatures you control gain deathtouch and lifelink until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{W}{B}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::deathtouch()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);
pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARCHANGELS_LIGHT,
    &BAR_THE_DOOR,
    &BREAK_OF_DAY,
    &BURDEN_OF_GUILT,
    &CURSE_OF_EXHAUSTION,
    &ELGAUD_INQUISITOR,
    &FAITH_S_SHIELD,
    &GATHER_THE_TOWNSFOLK,
    &GAVONY_IRONWRIGHT,
    &HOLLOWHENGE_SPIRIT,
    &INCREASING_DEVOTION,
    &LINGERING_SOULS,
    &LOYAL_CATHAR,
    &MIDNIGHT_GUARD,
    &NIBLIS_OF_THE_MIST,
    &NIBLIS_OF_THE_URN,
    &RAY_OF_REVELATION,
    &REQUIEM_ANGEL,
    &SANCTUARY_CAT,
    &SEANCE,
    &SILVERCLAW_GRIFFIN,
    &SKILLFUL_LUNGE,
    &SUDDEN_DISAPPEARANCE,
    &THALIA_GUARDIAN_OF_THRABEN,
    &THRABEN_DOOMSAYER,
    &THRABEN_HERETIC,
    &ARTFUL_DODGE,
    &BEGUILER_OF_WILLS,
    &BONE_TO_ASH,
    &CALL_TO_THE_KINDRED,
    &CHANT_OF_THE_SKIFSANG,
    &CHILL_OF_FOREBODING,
    &COUNTERLASH,
    &CURSE_OF_ECHOES,
    &DIVINATION,
    &DUNGEON_GEISTS,
    &GERALFS_MINDCRUSHER,
    &GRIPTIDE,
    &HAVENGUL_RUNEBINDER,
    &HEADLESS_SKAAB,
    &INCREASING_CONFUSION,
    &MYSTIC_RETRIEVAL,
    &NEPHALIA_SEAKITE,
    &NIBLIS_OF_THE_BREATH,
    &RELENTLESS_SKAABS,
    &SAVING_GRASP,
    &SCREECHING_SKAAB,
    &SECRETS_OF_THE_DEAD,
    &SHRIEKGEIST,
    &SOUL_SEIZER,
    &STORMBOUND_GEIST,
    &THOUGHT_SCOUR,
    &TOWER_GEIST,
    &BLACK_CAT,
    &CHOSEN_OF_MARKOV,
    &CURSE_OF_MISFORTUNES,
    &CURSE_OF_THIRST,
    &DEADLY_ALLURE,
    &DEATHS_CARESS,
    &FALKENRATH_TORTURER,
    &FARBOG_BONEFLINGER,
    &FIEND_OF_THE_SHADOWS,
    &GERALFS_MESSENGER,
    &GRAVECRAWLER,
    &GRAVEPURGE,
    &GRUESOME_DISCOVERY,
    &HARROWING_JOURNEY,
    &HIGHBORN_GHOUL,
    &INCREASING_AMBITION,
    &MIKAEUS_THE_UNHALLOWED,
    &RAVENOUS_DEMON,
    &REAP_THE_SEAGRAF,
    &SIGHTLESS_GHOUL,
    &SKIRSDAG_FLAYER,
    &SPITEFUL_SHADOWS,
    &TRAGIC_SLIP,
    &UNDYING_EVIL,
    &VENGEFUL_VAMPIRE,
    &WAKEDANCER,
    &ZOMBIE_APOCALYPSE,
    &AFFLICTED_DESERTER,
    &ALPHA_BRAWL,
    &BLOOD_FEUD,
    &BURNING_OIL,
    &CURSE_OF_BLOODLETTING,
    &ERDWAL_RIPPER,
    &FAITHLESS_LOOTING,
    &FIRES_OF_UNDEATH,
    &FLAYER_OF_THE_HATEBOUND,
    &FLING,
    &FORGE_DEVIL,
    &HECKLING_FIENDS,
    &HELLRIDER,
    &HINTERLAND_HERMIT,
    &INCREASING_VENGEANCE,
    &MARKOV_BLADEMASTER,
    &MARKOV_WARLORD,
    &MONDRONEN_SHAMAN,
    &MOONVEIL_DRAGON,
    &NEARHEATH_STALKER,
    &PYREHEART_WOLF,
    &RUSSET_WOLVES,
    &SCORCH_THE_FIELDS,
    &SHATTERED_PERCEPTION,
    &TALONS_OF_FALKENRATH,
    &TORCH_FIEND,
    &WRACK_WITH_MADNESS,
    &BRIARPACK_ALPHA,
    &CLINGING_MISTS,
    &CRUSHING_VINES,
    &DAWNTREADER_ELK,
    &DERANGED_OUTCAST,
    &FAVOR_OF_THE_WOODS,
    &FEED_THE_PACK,
    &GHOULTREE,
    &GRAVETILLER_WURM,
    &GRIM_FLOWERING,
    &HOLLOWHENGE_BEAST,
    &HUNGER_OF_THE_HOWLPACK,
    &INCREASING_SAVAGERY,
    &KESSIG_RECLUSE,
    &LAMBHOLT_ELDER,
    &LOST_IN_THE_WOODS,
    &PREDATOR_OOZE,
    &SCORNED_VILLAGER,
    &SOMBERWALD_DRYAD,
    &STRANGLEROOT_GEIST,
    &TRACKER_S_INSTINCTS,
    &ULVENWALD_BEAR,
    &VILLAGE_SURVIVORS,
    &VORAPEDE,
    &WILD_HUNGER,
    &WOLFBITTEN_CAPTIVE,
    &YOUNG_WOLF,
    &DIREGRAF_CAPTAIN,
    &DROGSKOL_CAPTAIN,
    &DROGSKOL_REAVER,
    &FALKENRATH_ARISTOCRAT,
    &HAVENGUL_LICH,
    &HUNTMASTER_OF_THE_FELLS,
    &IMMERWOLF,
    &SORIN_LORD_OF_INNISTRAD,
    &STROMKIRK_CAPTAIN,
    &ALTAR_OF_THE_LOST,
    &AVACYNS_COLLAR,
    &CHALICE_OF_LIFE,
    &ELBRUS_THE_BINDING_BLADE,
    &EXECUTIONERS_HOOD,
    &GRAFDIGGER_S_CAGE,
    &HEAVY_MATTOCK,
    &HELVAULT,
    &JAR_OF_EYEBALLS,
    &WARDEN_OF_THE_WALL,
    &WOLFHUNTERS_QUIVER,
    &EVOLVING_WILDS,
    &GRIM_BACKWOODS,
    &HAUNTED_FENGRAF,
    &VAULT_OF_THE_ARCHANGEL,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
