//! Scourge cards used by the staged Premodern deck tranche.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::ArrivalAttachmentDef;
use crate::card::AttackDefenderScopeDef;
use crate::card::AttackRestrictionDef;
use crate::card::BasicLandType;
use crate::card::BlockRestrictionDef;
use crate::card::BlockRestrictionMatchDef;
use crate::card::BlockRestrictionSubjectDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::DamageEventMatcherDef;
use crate::card::DamagePreventionDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::PayOrDef;
use crate::card::PlayerRelation;
use crate::card::PlayerRuleDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SubtypeDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::sets::y1998::portal_second_age as catalog_p02;
use crate::ids::TargetIndex;
use crate::mana_cost;

static A_PLAYER: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "SCG",
    slug: "scourge",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// SCG 1 — Ageless Sentinels
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGELESS_SENTINELS: CardRecord = CardRecord::new(
    "Ageless Sentinels",
    "ccaa4a19-8eba-4c43-8a9a-636e234df751",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// SCG 2 — Astral Steel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASTRAL_STEEL: CardRecord = CardRecord::new(
    "Astral Steel",
    "64f836d3-52c8-4628-b18a-8c8fb67969c9",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// SCG 3 — Aven Farseer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVEN_FARSEER: CardRecord = CardRecord::new(
    "Aven Farseer",
    "47854e89-4d22-4eb6-a77d-6f04407bd2e5",
    "Luca Zontini",
    crate::card::CardRules::unsupported(),
);

// SCG 4 — Aven Liberator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVEN_LIBERATOR: CardRecord = CardRecord::new(
    "Aven Liberator",
    "b2804006-2a60-400c-be0b-8aa042469372",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// SCG 5 — Daru Spiritualist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARU_SPIRITUALIST: CardRecord = CardRecord::new(
    "Daru Spiritualist",
    "18f26b88-cffc-47ed-a70a-7d704a32c8bb",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// SCG 6 — Daru Warchief
pub(in crate::card::sets) static DARU_WARCHIEF: CardRecord = CardRecord::new(
    "Daru Warchief",
    "2630d3b5-8f3a-4aad-a45e-22a7979429f3",
    "Tim Hildebrandt",
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Human", "Soldier"], 1, 1).with_abilities(
        &[
            abilities::spell_cost_reduction(
                "Soldier spells you cast cost {1} less to cast.",
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Soldier")),
                PlayerRelation::You,
                ValueDef::Constant(1),
            ),
            AbilityDef::static_ability(
                "Soldier creatures you control get +1/+2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Soldier")),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(2),
                    ),
                },
            ),
        ],
    ),
);

// SCG 7 — Dawn Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAWN_ELEMENTAL: CardRecord = CardRecord::new(
    "Dawn Elemental",
    "fd90a303-25fb-460b-bd55-6249f61c361c",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// SCG 8 — Decree of Justice
pub(in crate::card::sets) static DECREE_OF_JUSTICE: CardRecord = CardRecord::new(
    "Decree of Justice",
    "5e8a7e5c-f252-4de8-94d7-e7327210bf26",
    "Adam Rex",
// Cast for Angels when the game went long, cycled for Soldiers at the end
    // of an opponent's turn when it did not. Landstill wants the second mode
    // far more often than the first.
    CardRules::new_sorcery(mana_cost!("{X}{X}{2}{W}{W}")).with_abilities(&[
        AbilityDef::spell(
            "Create X 4/4 white Angel creature tokens with flying.",
            EffectDef::create_creature_token(&["Angel"], &[ManaColor::White], 4, 4)
                .with_count(ValueDef::ChosenX)
                .with_abilities(&[abilities::flying()])
                .with_art(CardArt::new(
                    "bb6d0a6a-3007-47fc-a42c-3db311c9c41f",
                    "Magali Villeneuve",
                )),
        ),
        abilities::cycling!(
            "Cycling {2}{W} ({2}{W}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}{W}"))],
        ),
        AbilityDef::triggered(
            "When you cycle this card, you may pay {X}. If you do, create X 1/1 white Soldier creature tokens.",
            TriggerEventDef::DiscardedToActivate(crate::card::AbilityLabel::CYCLING),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::ChosenGenericMana], // The cycling half: X is settled by the payment rather than by a cast, so
                // the branch that makes the tokens reads back what was actually paid.
                &EffectDef::create_creature_token(&["Soldier"], &[ManaColor::White], 1, 1)
                    .with_count(ValueDef::PaidAmount)
                    .with_art(CardArt::new(
                        "70205fb6-7722-4974-a8c6-8909dbb1c96d",
                        "Bachzim",
                    )),
            )),
        ),
    ]),
);

// SCG 9 — Dimensional Breach
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIMENSIONAL_BREACH: CardRecord = CardRecord::new(
    "Dimensional Breach",
    "f18f2832-07c5-47be-8966-b250fb997f78",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// SCG 10 — Dragon Scales
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGON_SCALES: CardRecord = CardRecord::new(
    "Dragon Scales",
    "8e78b364-015d-4074-ad9e-55c973ce2f4b",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// SCG 11 — Dragonstalker
pub(in crate::card::sets) static DRAGONSTALKER: CardRecord = CardRecord::new(
    "Dragonstalker",
    "58017ff1-74d2-4be2-976a-8dff53e16150",
    "Ron Spencer",
    // Protection aimed at exactly one tribe, which is what a set about
    // Dragons made worth printing.
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Bird", "Soldier"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::keyword(
            "Protection from Dragons",
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                "Dragon",
            ))),
        ),
    ]),
);

// SCG 12 — Eternal Dragon
pub(in crate::card::sets) static ETERNAL_DRAGON: CardRecord = CardRecord::new(
    "Eternal Dragon",
    "0596928c-2b20-4dbb-aa78-3ab6c3ce0d72",
    "Justin Sweet",
// Three cards in one: a land early, a threat late, and a threat again
    // every turn after that. Control decks play it as a one-of because it
    // never runs out.
    CardRules::new_creature(mana_cost!("{5}{W}{W}"), &["Dragon", "Spirit"], 5, 5).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{3}{W}{W}: Return this card from your graveyard to your hand. Activate only during your upkeep.",
            &[CostDef::Mana(mana_cost!("{3}{W}{W}"))],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
},
        )
        .with_source_zones(&[ZoneKind::Graveyard])
        .with_activation_timing(ActivationTimingDef::YourUpkeep),
        abilities::typecycling!(
            "Plainscycling {2} ({2}, Discard this card: Search your library for a Plains card, \
                reveal it, put it into your hand, then shuffle.)",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
        ),
    ]),
);

// SCG 13 — Exiled Doomsayer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXILED_DOOMSAYER: CardRecord = CardRecord::new(
    "Exiled Doomsayer",
    "74aaf095-143a-43fc-a858-b1e82a4b906e",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// SCG 14 — Force Bubble
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORCE_BUBBLE: CardRecord = CardRecord::new(
    "Force Bubble",
    "742ac116-86ed-4ce6-9805-76f47a41c4c4",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// SCG 15 — Frontline Strategist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FRONTLINE_STRATEGIST: CardRecord = CardRecord::new(
    "Frontline Strategist",
    "1c43fac2-62fb-4924-848d-a8d739773d6e",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// SCG 16 — Gilded Light
pub(in crate::card::sets) static GILDED_LIGHT: CardRecord = CardRecord::new(
    "Gilded Light",
    "01b92597-cb1e-4b8f-9ee9-07b48cf1a5c6",
    "John Avon",
    // It answers the discard spell and the burn spell with the same card, so
    // long as either one targets.
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::spell(
            "You gain shroud until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(PlayerRuleDef::Shroud)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))],
        ),
    ]),
);

// SCG 17 — Guilty Conscience
pub(in crate::card::sets) static GUILTY_CONSCIENCE: CardRecord = CardRecord::new(
    "Guilty Conscience",
    "67b8701c-0f03-4ad0-9097-3caf885abd59",
    "Christopher Moeller",
// It kills anything that deals damage equal to its own toughness, which
    // is most of what a big attacker is.
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered(
            "Whenever enchanted creature deals damage, this Aura deals that much damage to that creature.",
            TriggerEventDef::damage_dealt_by(ObjectPredicateDef::AttachedToSource),
            EffectDef::damage(
                EffectRecipientDef::AttachedPermanent,
                ValueDef::TriggerEventAmount,
            ),
        ),
        ]),
);

// SCG 18 — Karona's Zealot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KARONA_S_ZEALOT: CardRecord = CardRecord::new(
    "Karona's Zealot",
    "914a1200-b77c-4a2c-96c6-7cc624ee9a6a",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// SCG 19 — Noble Templar
pub(in crate::card::sets) static NOBLE_TEMPLAR: CardRecord = CardRecord::new(
    "Noble Templar",
    "6a9ede92-e64f-44a5-afb6-c7495077fb0b",
    "Alex Horley-Orlandelli",
    // A six-drop that is a land on the turns you needed a land, which is the
    // whole point of the landcycling cycle.
    CardRules::new_creature(mana_cost!("{5}{W}"), &["Human", "Cleric", "Soldier"], 3, 6)
        .with_abilities(&[
            abilities::vigilance(),
            abilities::typecycling!(
                "Plainscycling {2} ({2}, Discard this card: Search your library for a Plains card, \
                reveal it, put it into your hand, then shuffle.)",
                &[CostDef::Mana(mana_cost!("{2}"))],
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Plains")),
            ),
        ]),
);

// SCG 20 — Rain of Blades
pub(in crate::card::sets) static RAIN_OF_BLADES: CardRecord = CardRecord::new(
    "Rain of Blades",
    "418476cd-94da-47a5-ba77-6bb4771e9c89",
    "Rob Alexander",
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell(
        "Rain of Blades deals 1 damage to each attacking creature.",
        EffectDef::damage(
            EffectRecipientDef::matching_objects(
                ObjectPredicateDef::Attacking,
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            ValueDef::Constant(1),
        ),
    )),
);

// SCG 21 — Recuperate
pub(in crate::card::sets) static RECUPERATE: CardRecord = CardRecord::new(
    "Recuperate",
    "a5945397-0906-48dd-80d1-c65bfa2b31a6",
    "Tim Hildebrandt",
    // Six is a lot of either, and four mana at instant speed is what it costs
    // to decide which after the opponent has committed.
    CardRules::new_instant(mana_cost!("{3}{W}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "You gain 6 life.",
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(6),
                },
            ),
            AbilityDef::spell_with_targets(
                "Prevent the next 6 damage that would be dealt to target creature this turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::PreventDamage {
                    prevention: DamagePreventionDef::amount(
                        DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                        ValueDef::Constant(6),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )),
);

// SCG 22 — Reward the Faithful
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REWARD_THE_FAITHFUL: CardRecord = CardRecord::new(
    "Reward the Faithful",
    "df6e8844-3736-4fb1-bedb-6a6bfa6ccdc8",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// SCG 23 — Silver Knight
pub(in crate::card::sets) static SILVER_KNIGHT: CardRecord = CardRecord::new(
    "Silver Knight",
    "93f559da-08ad-402d-8c6b-3050bce5867b",
    "Eric Peterson",
    // The one-colour version at two mana, which is why every white deck of
    // the era ran it against red.
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        abilities::first_strike(),
        abilities::protection_from_color(ManaColor::Red),
    ]),
);

// SCG 24 — Trap Digger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRAP_DIGGER: CardRecord = CardRecord::new(
    "Trap Digger",
    "05cd76bf-db08-45f8-b3ae-501bcca6df3c",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// SCG 25 — Wing Shards
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WING_SHARDS: CardRecord = CardRecord::new(
    "Wing Shards",
    "65efa443-666a-45c1-8784-e98c510854b5",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// SCG 26 — Wipe Clean
pub(in crate::card::sets) static WIPE_CLEAN: CardRecord = CardRecord::new(
    "Wipe Clean",
    "cf7c14c9-cb5a-49f0-be2c-eb3166f02510",
    "Arnie Swekel",
    // Exile rather than destroy, which matters against exactly the
    // enchantments that come back.
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Exile target enchantment.",
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                )]
            },
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
        abilities::cycling!(
            "Cycling {3} ({3}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{3}"))],
        ),
    ]),
);

// SCG 27 — Zealous Inquisitor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZEALOUS_INQUISITOR: CardRecord = CardRecord::new(
    "Zealous Inquisitor",
    "1fb821b6-4e73-4970-b1ac-b67c93990ec0",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// SCG 28 — Aphetto Runecaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APHETTO_RUNECASTER: CardRecord = CardRecord::new(
    "Aphetto Runecaster",
    "7c5de028-91ce-48d8-8557-ae12542adea2",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// SCG 29 — Brain Freeze
pub(in crate::card::sets) static BRAIN_FREEZE: CardRecord = CardRecord::new(
    "Brain Freeze",
    "59a43ef5-08f0-44fc-802d-b6cfd56b7d1f",
    "Tim Hildebrandt",
    // Three cards a copy, and a Stasis deck casting four cheap spells in a
    // turn mills a dozen: the sideboard plan against another control deck.
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target player mills three cards.",
            &A_PLAYER,
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        ),
        abilities::storm(),
    ]),
);

// SCG 30 — Coast Watcher
pub(in crate::card::sets) static COAST_WATCHER: CardRecord = CardRecord::new(
    "Coast Watcher",
    "6bbbc67d-99d0-4277-a8f2-64509e59ec00",
    "Luca Zontini",
    // The cheap end of the same hoser cycle.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Bird", "Soldier"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::protection_from_color(ManaColor::Green),
    ]),
);

// SCG 31 — Day of the Dragons
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAY_OF_THE_DRAGONS: CardRecord = CardRecord::new(
    "Day of the Dragons",
    "366a934c-eb01-48c6-8393-c2fe0708ff91",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// SCG 32 — Decree of Silence
pub(in crate::card::sets) static DECREE_OF_SILENCE: CardRecord = CardRecord::new(
    "Decree of Silence",
    "f2fc46e2-5e19-4999-a4cd-1e84697066c1",
    "Adam Rex",
// Eight mana is not what the deck pays: it cycles this to counter one
    // spell, and Replenish puts it onto the battlefield afterwards.
    CardRules::new_enchantment(mana_cost!("{6}{U}{U}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever an opponent casts a spell, counter that spell and put a depletion counter on this enchantment. If there are three or more depletion counters on this enchantment, sacrifice it.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(
                PlayerRelation::Opponent,
            )),
            // Counter the spell, mark the enchantment, and go when the third mark
            // lands. The sacrifice is checked in the same resolution rather than as a
            // state trigger, which is what the printed clause says.
            EffectDef::Sequence(&const {
                [
                    EffectDef::Counter {
                        object: EffectRecipientDef::TriggeringObject,
                        zone: ZoneKind::Graveyard,
                        placement: ZonePlacement::Top,
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::named("depletion"),
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::IfCondition {
                        condition: &const {
                            TriggerConditionDef::SourceCounters {
                                kind: CounterKind::named("depletion"),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 3,
                            }
                        },
                        then: &const {
                            EffectDef::sacrifice(EffectRecipientDef::Source)
                        },
                    },
                ]
            }),
        ),
        abilities::cycling!(
            "Cycling {4}{U}{U} ({4}{U}{U}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{4}{U}{U}"))],
        ),
        AbilityDef::triggered_with_targets(
            "When you cycle this card, you may counter target spell.",
            TriggerEventDef::DiscardedToActivate(crate::card::AbilityLabel::CYCLING),
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            })],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Counter {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Graveyard,
                    placement: ZonePlacement::Top,
                },
            },
        ),
    ]),
);

// SCG 33 — Dispersal Shield
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISPERSAL_SHIELD: CardRecord = CardRecord::new(
    "Dispersal Shield",
    "0c257df6-f275-40db-bfe3-a9291356cdf7",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// SCG 34 — Dragon Wings
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGON_WINGS: CardRecord = CardRecord::new(
    "Dragon Wings",
    "7674ab4d-9bc0-45c3-88e1-3fd2c947cfaa",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// SCG 35 — Faces of the Past
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FACES_OF_THE_PAST: CardRecord = CardRecord::new(
    "Faces of the Past",
    "0f6dc35b-eb26-498f-ae35-0e860871446e",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// SCG 36 — Frozen Solid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FROZEN_SOLID: CardRecord = CardRecord::new(
    "Frozen Solid",
    "9b89b98d-0245-4b64-b835-d101ce2bd3fa",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// SCG 37 — Hindering Touch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HINDERING_TOUCH: CardRecord = CardRecord::new(
    "Hindering Touch",
    "db9735d9-4aac-4175-8ec8-fc9bfd8f2c5c",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// SCG 38 — Long-Term Plans
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LONG_TERM_PLANS: CardRecord = CardRecord::new(
    "Long-Term Plans",
    "7e0422d9-9694-45b6-9c2b-2ca31198cebf",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// SCG 39 — Mercurial Kite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERCURIAL_KITE: CardRecord = CardRecord::new(
    "Mercurial Kite",
    "a6bc8655-ae27-40be-8d61-e80a5924e955",
    "Richard Sardinha",
    crate::card::CardRules::unsupported(),
);

// SCG 40 — Metamorphose
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static METAMORPHOSE: CardRecord = CardRecord::new(
    "Metamorphose",
    "a0f0c20c-184e-4d27-ae8b-933abb6fee0c",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// SCG 41 — Mind's Desire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_S_DESIRE: CardRecord = CardRecord::new(
    "Mind's Desire",
    "3c7474e1-cfae-4867-a11a-d5cf9ff7ea5f",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// SCG 42 — Mischievous Quanar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISCHIEVOUS_QUANAR: CardRecord = CardRecord::new(
    "Mischievous Quanar",
    "dc48c2db-f5b4-4c24-a5fa-00750b7ff56f",
    "Lars Grant-West",
    crate::card::CardRules::unsupported(),
);

// SCG 43 — Mistform Warchief
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFORM_WARCHIEF: CardRecord = CardRecord::new(
    "Mistform Warchief",
    "a633d85b-4be1-44a2-8fd8-1ccec4a95ecb",
    "Greg Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// SCG 44 — Parallel Thoughts
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARALLEL_THOUGHTS: CardRecord = CardRecord::new(
    "Parallel Thoughts",
    "d913c541-a8fb-4383-bbab-988be3e0f5d5",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// SCG 45 — Pemmin's Aura
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PEMMIN_S_AURA: CardRecord = CardRecord::new(
    "Pemmin's Aura",
    "9fb3e38b-086e-4fbc-b7b1-8564c18276d7",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// SCG 46 — Raven Guild Initiate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAVEN_GUILD_INITIATE: CardRecord = CardRecord::new(
    "Raven Guild Initiate",
    "c1e11f70-06c3-4dc5-aafe-82d65080085e",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// SCG 47 — Raven Guild Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAVEN_GUILD_MASTER: CardRecord = CardRecord::new(
    "Raven Guild Master",
    "9843f847-6a8f-4042-86b6-f7fe5a47cc57",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// SCG 48 — Riptide Survivor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIPTIDE_SURVIVOR: CardRecord = CardRecord::new(
    "Riptide Survivor",
    "7515187f-4821-400d-b78f-cec173df6b84",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// SCG 49 — Rush of Knowledge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUSH_OF_KNOWLEDGE: CardRecord = CardRecord::new(
    "Rush of Knowledge",
    "65b03b40-671f-4973-8d75-c3fa878ef603",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// SCG 50 — Scornful Egotist
pub(in crate::card::sets) static SCORNFUL_EGOTIST: CardRecord = CardRecord::new(
    "Scornful Egotist",
    "fec6b189-97e7-4627-9785-a9ce2f1ad89f",
    "Jim Nelson",
    // Eight mana for a 1/1 is the joke; one blue to flip a 2/2 face up is
    // the reason anybody ever put it in a deck.
    CardRules::new_creature(mana_cost!("{7}{U}"), &["Human", "Wizard"], 1, 1)
        .with_morph(&[CostDef::Mana(mana_cost!("{U}"))])
        .with_ability(AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{3}"))],
            crate::card::face_down::morph_cast(),
            Some(
                "Morph {U} (You may cast this card face down as a 2/2 creature for {3}. \
                 Turn it face up any time for its morph cost.)",
            ),
            EffectDef::None,
        )),
);

// SCG 51 — Shoreline Ranger
pub(in crate::card::sets) static SHORELINE_RANGER: CardRecord = CardRecord::new(
    "Shoreline Ranger",
    "eed813c4-fff0-43f1-bc62-cbc3a126d600",
    "Michael Sutfin",
    // Blue's copy: a flier late, an Island early, and never a card stuck in
    // hand.
    CardRules::new_creature(mana_cost!("{5}{U}"), &["Bird", "Soldier"], 3, 4).with_abilities(&[
        abilities::flying(),
        abilities::typecycling!(
            "Islandcycling {2} ({2}, Discard this card: Search your library for a Island card, \
            reveal it, put it into your hand, then shuffle.)",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Island")),
        ),
    ]),
);

// SCG 52 — Stifle
pub(in crate::card::sets) static STIFLE: CardRecord = CardRecord::new(
    "Stifle",
    "2d7643c0-b2db-478f-944e-b27b77bad3eb",
    "Dany Orizio",
    // One mana that answers a fetchland, a Dreadnought's own drawback, or
    // whatever the opponent built their turn around.
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target activated or triggered ability.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Ability,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Counter {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Graveyard,
            placement: ZonePlacement::Top,
        },
    )),
);

// SCG 53 — Temporal Fissure
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEMPORAL_FISSURE: CardRecord = CardRecord::new(
    "Temporal Fissure",
    "97949c53-aef7-4c0c-b846-8d4003193ced",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// SCG 54 — Thundercloud Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDERCLOUD_ELEMENTAL: CardRecord = CardRecord::new(
    "Thundercloud Elemental",
    "597aea42-43e0-41ed-bfe7-fc92b6b8e680",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// SCG 55 — Bladewing's Thrall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLADEWING_S_THRALL: CardRecord = CardRecord::new(
    "Bladewing's Thrall",
    "f07e0d28-6383-4846-89d3-72910a7bbdcd",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// SCG 56 — Cabal Conditioning
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CABAL_CONDITIONING: CardRecord = CardRecord::new(
    "Cabal Conditioning",
    "eb81c6e6-fded-4cd3-a6fa-486419a5408a",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// SCG 57 — Cabal Interrogator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CABAL_INTERROGATOR: CardRecord = CardRecord::new(
    "Cabal Interrogator",
    "256a7a37-6f47-47a3-b149-5692aee8b34a",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// SCG 58 — Call to the Grave
// Audit: unsupported — Needs each active player to make an APNAP-ordered non-Zombie sacrifice choice before any chosen creature moves.
pub(in crate::card::sets) static CALL_TO_THE_GRAVE: CardRecord = CardRecord::new(
    "Call to the Grave",
    "2a346b4a-ac8a-4f99-9ed7-dd41102e56ce",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// SCG 59 — Carrion Feeder
pub(in crate::card::sets) static CARRION_FEEDER: CardRecord = CardRecord::new(
    "Carrion Feeder",
    "88042031-64af-4f84-85d5-95992b43aa6c",
    "Brian Snõddy",
    // A free sacrifice outlet with no mana in the cost, which is what makes
    // it an engine piece rather than a creature; not blocking is the price.
    CardRules::new_creature(mana_cost!("{B}"), &["Zombie"], 1, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            },
        ),
        AbilityDef::activated(
            "Sacrifice a creature: Put a +1/+1 counter on this creature.",
            // Any creature you control, the Feeder included -- which is the
            // out when it is the last thing on the board.
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
            }],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// SCG 60 — Chill Haunting
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHILL_HAUNTING: CardRecord = CardRecord::new(
    "Chill Haunting",
    "91035d03-2bf8-4e6b-945b-4dfbed0873ec",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// SCG 61 — Clutch of Undeath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLUTCH_OF_UNDEATH: CardRecord = CardRecord::new(
    "Clutch of Undeath",
    "7301fdec-ca17-47ae-9a0a-84ea8665ece1",
    "Greg Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// SCG 62 — Consumptive Goo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONSUMPTIVE_GOO: CardRecord = CardRecord::new(
    "Consumptive Goo",
    "0f0f549f-6607-483a-9d89-2019ca9ef571",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// SCG 63 — Death's-Head Buzzard
pub(in crate::card::sets) static DEATH_S_HEAD_BUZZARD: CardRecord = CardRecord::new(
    "Death's-Head Buzzard",
    "8a8d4fd9-1f9e-41f0-9114-d1a698506ad9",
    "Marcelo Vignali",
    // The small version: a 2/1 flier that takes the X/1s with it, which in
    // a token format is most of a board.
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Bird"], 2, 1).with_abilities(&[
        abilities::flying(),
        abilities::dies_trigger(
            "When this creature dies, all creatures get -1/-1 until end of turn.",
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
        ),
    ]),
);

// SCG 64 — Decree of Pain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DECREE_OF_PAIN: CardRecord = CardRecord::new(
    "Decree of Pain",
    "e1958a07-fc75-41cd-ac45-d92d49587754",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// SCG 65 — Dragon Shadow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGON_SHADOW: CardRecord = CardRecord::new(
    "Dragon Shadow",
    "0ec35e03-022b-417c-9987-7379cf3956f9",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// SCG 66 — Fatal Mutation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FATAL_MUTATION: CardRecord = CardRecord::new(
    "Fatal Mutation",
    "57cf9f50-8858-44a6-8bd5-0ce1e281a584",
    "Erica Gassalasca-Jape",
    crate::card::CardRules::unsupported(),
);

// SCG 67 — Final Punishment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FINAL_PUNISHMENT: CardRecord = CardRecord::new(
    "Final Punishment",
    "097dbfae-1a18-4c10-8d1f-b2c20971c75e",
    "Matt Thompson",
    crate::card::CardRules::unsupported(),
);

// SCG 68 — Lethal Vapors
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LETHAL_VAPORS: CardRecord = CardRecord::new(
    "Lethal Vapors",
    "f96acfea-009a-4ac9-8746-64f65199024f",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// SCG 69 — Lingering Death
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LINGERING_DEATH: CardRecord = CardRecord::new(
    "Lingering Death",
    "f174fd76-f28d-4272-8cb0-7f66cd60579e",
    "Matt Thompson",
    crate::card::CardRules::unsupported(),
);

// SCG 70 — Nefashu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NEFASHU: CardRecord = CardRecord::new(
    "Nefashu",
    "7046acc2-e2fd-43e6-9d46-a729d48ba562",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// SCG 71 — Putrid Raptor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PUTRID_RAPTOR: CardRecord = CardRecord::new(
    "Putrid Raptor",
    "9127942b-d73d-42a9-9f97-6a39fa798a8b",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// SCG 72 — Reaping the Graves
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REAPING_THE_GRAVES: CardRecord = CardRecord::new(
    "Reaping the Graves",
    "760a66bd-2821-4710-8f02-3c30772dd884",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// SCG 73 — Skulltap
pub(in crate::card::sets) static SKULLTAP: CardRecord = CardRecord::new(
    "Skulltap",
    "48a90779-008e-401f-9877-be0a935d2ccd",
    "Adam Rex",
    // Two cards for two mana and a body, which is the rate a deck already
    // sacrificing creatures for other reasons gets to draw at for free.
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a creature.\nDraw two cards.",
            &[],
            CostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Creature),
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// SCG 74 — Soul Collector
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_COLLECTOR: CardRecord = CardRecord::new(
    "Soul Collector",
    "ec78c0e8-a354-46d2-95ad-012f120c3df8",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// SCG 75 — Tendrils of Agony
pub(in crate::card::sets) static TENDRILS_OF_AGONY: CardRecord = CardRecord::new(
    "Tendrils of Agony",
    "0559352e-95c1-403b-bd8f-d0679717cfa2",
    "Pete Venters",
    // Four life is nothing; ten copies of it is the whole game, which is why
    // every ritual in the format is really a Tendrils card.
    CardRules::new_sorcery(mana_cost!("{2}{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target player loses 2 life and you gain 2 life.",
            &A_PLAYER,
            // Life loss rather than damage: nothing prevents it, nothing watching for
            // damage sees it, and the two life you gain is a flat two however little
            // they had left to lose.
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(2),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
        abilities::storm(),
    ]),
);

// SCG 76 — Twisted Abomination
pub(in crate::card::sets) static TWISTED_ABOMINATION: CardRecord = CardRecord::new(
    "Twisted Abomination",
    "446e672f-87aa-4308-98bb-d00548c5bcef",
    "Daren Bader",
    // The best of the cycle: a Swamp on turn two and a regenerating 5/3 on
    // turn six, out of the same card.
    CardRules::new_creature(mana_cost!("{5}{B}"), &["Zombie", "Mutant"], 5, 3).with_abilities(&[
        abilities::regenerate_self(
            "{B}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{B}"))],
        ),
        abilities::typecycling!(
            "Swampcycling {2} ({2}, Discard this card: Search your library for a Swamp card, \
            reveal it, put it into your hand, then shuffle.)",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Swamp")),
        ),
    ]),
);

// SCG 77 — Unburden
pub(in crate::card::sets) static UNBURDEN: CardRecord = CardRecord::new(
    "Unburden",
    "bd5fc0e0-4ee5-40eb-a9f0-9b1fff2adefc",
    "Wayne England",
    // Two cards for three mana is a fair rate only on the turn the opponent
    // still has two, which cycling lets you wait for.
    CardRules::new_sorcery(mana_cost!("{1}{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target player discards two cards.",
            &const {
                [AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )]
            },
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))],
        ),
    ]),
);

// SCG 78 — Undead Warchief
pub(in crate::card::sets) static UNDEAD_WARCHIEF: CardRecord = CardRecord::new(
    "Undead Warchief",
    "e6b3bcfe-be82-458b-ba59-ecb84436d747",
    "Greg Hildebrandt",
    // The discount and the anthem compound: every Zombie after this one is
    // cheaper and bigger than the card says.
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Zombie"], 1, 1).with_abilities(&[
        AbilityDef::static_ability(
            "Zombie spells you cast cost {1} less to cast.",
            EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Zombie")),
                PlayerRelation::You,
                ValueDef::Constant(1),
            )),
        ),
        AbilityDef::static_ability(
            "Zombie creatures you control get +2/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Zombie")),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(1),
                ),
            },
        ),
    ]),
);

// SCG 79 — Unspeakable Symbol
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNSPEAKABLE_SYMBOL: CardRecord = CardRecord::new(
    "Unspeakable Symbol",
    "2cc4601b-5f34-4733-8c32-9779de4c502c",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// SCG 80 — Vengeful Dead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENGEFUL_DEAD: CardRecord = CardRecord::new(
    "Vengeful Dead",
    "7c11c11d-9809-4031-8cbc-21aef07d7f1f",
    "Alex Horley-Orlandelli",
    crate::card::CardRules::unsupported(),
);

// SCG 81 — Zombie Cutthroat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZOMBIE_CUTTHROAT: CardRecord = CardRecord::new(
    "Zombie Cutthroat",
    "fe491cba-6ec7-4c44-ad1e-832d936986a0",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// SCG 82 — Bonethorn Valesk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BONETHORN_VALESK: CardRecord = CardRecord::new(
    "Bonethorn Valesk",
    "297d7326-ad03-464d-97e2-443042d48f92",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// SCG 83 — Carbonize
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CARBONIZE: CardRecord = CardRecord::new(
    "Carbonize",
    "6f565fa1-a1a0-4dd0-b7f4-df65a807d156",
    "Alex Horley-Orlandelli",
    crate::card::CardRules::unsupported(),
);

// SCG 84 — Chartooth Cougar
pub(in crate::card::sets) static CHARTOOTH_COUGAR: CardRecord = CardRecord::new(
    "Chartooth Cougar",
    "6b2c9c07-c3db-46ca-a204-b710c3a34ae9",
    "Tony Szczudlo",
    // Red's copy, with a mana sink attached for the game where the Mountain
    // was not the half you needed.
    CardRules::new_creature(mana_cost!("{5}{R}"), &["Cat", "Beast"], 4, 4).with_abilities(&[
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
        abilities::typecycling!(
            "Mountaincycling {2} ({2}, Discard this card: Search your library for a Mountain card, \
            reveal it, put it into your hand, then shuffle.)",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mountain")),
        ),
    ]),
);

// SCG 85 — Decree of Annihilation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DECREE_OF_ANNIHILATION: CardRecord = CardRecord::new(
    "Decree of Annihilation",
    "73744717-518c-478e-9da9-201c49124f37",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// SCG 86 — Dragon Breath
pub(in crate::card::sets) static DRAGON_BREATH: CardRecord = CardRecord::new(
    "Dragon Breath",
    "1832aaed-e164-4f78-9bc9-ec6c015835f5",
    "Greg Staples",
// Nobody casts it. It is discarded on the way to filling a graveyard and
    // comes back for free the turn something enormous arrives.
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has haste.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                },
            ),
            AbilityDef::activated(
                "{R}: Enchanted creature gets +1/+0 until end of turn.",
                &[CostDef::Mana(mana_cost!("{R}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::triggered(
                "When a creature with mana value 6 or greater enters, you may return this card from your graveyard to the battlefield attached to that creature.",
                TriggerEventDef::zone_changed(
                    // Six or more, which the deck reaches by assembling a creature rather than
                    // by paying for one: the Ghoul arrives enormous and the Breath comes back
                    // attached to give it haste.
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMostValue(
                            ValueDef::Constant(5),
                        )),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::WithBattlefieldArrival {
                        effect: &EffectDef::MoveToZone {
                            object: EffectRecipientDef::Source,
                            zone: ZoneKind::Battlefield,
                            placement: ZonePlacement::Top,
                        },
                        arrival: crate::card::BattlefieldArrivalDef {
                            controller: Some(PlayerRelation::You),
                            attachment: Some(ArrivalAttachmentDef::ArrivalToHost(
                                ObjectRefDef::TriggeringObject,
                            )),
                            ..crate::card::BattlefieldArrivalDef::DEFAULT
                        },
                    },
                },
            )
            .with_source_zones(&[ZoneKind::Graveyard]),
        ]),
);

// SCG 87 — Dragon Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGON_MAGE: CardRecord = CardRecord::new(
    "Dragon Mage",
    "7687a201-0ecc-4739-86e3-3b4090d345a8",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// SCG 88 — Dragon Tyrant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGON_TYRANT: CardRecord = CardRecord::new(
    "Dragon Tyrant",
    "04d1a29b-af80-4f9a-881b-ef7374ecbce1",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// SCG 89 — Dragonspeaker Shaman
pub(in crate::card::sets) static DRAGONSPEAKER_SHAMAN: CardRecord = CardRecord::new(
    "Dragonspeaker Shaman",
    "49f5fa96-dcfb-4d29-bea9-7dd99e8c43d8",
    "Kev Walker",
    // Two mana off every Dragon turns a seven-drop into a five-drop, which is
    // the difference between a card and a deck.
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Human", "Barbarian"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "Dragon spells you cast cost {2} less to cast.",
            EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dragon")),
                PlayerRelation::You,
                ValueDef::Constant(2),
            )),
        ),
    ),
);

// SCG 90 — Dragonstorm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGONSTORM: CardRecord = CardRecord::new(
    "Dragonstorm",
    "4b9aa594-39e6-4824-aed9-75d1a301ac51",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// SCG 91 — Enrage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENRAGE: CardRecord = CardRecord::new(
    "Enrage",
    "d6ed7866-9eef-49c3-9b9e-4247b6e71a6c",
    "Justin Sweet",
    crate::card::CardRules::unsupported(),
);

// SCG 92 — Extra Arms
pub(in crate::card::sets) static EXTRA_ARMS: CardRecord = CardRecord::new(
    "Extra Arms",
    "28efa11c-6aeb-4c22-bbb3-b41f26d65c65",
    "Greg Staples",
    // Two damage every attack, which the opponent has to answer even when
    // the creature itself is not the problem.
    CardRules::new_enchantment(mana_cost!("{4}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered_with_targets(
                "Whenever enchanted creature attacks, it deals 2 damage to any target.",
                TriggerEventDef::attacks(ObjectPredicateDef::AttachedToSource),
                &const {
                    [AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::AnyTarget,
                    )]
                },
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(2),
                ),
            ),
        ]),
);

// SCG 93 — Form of the Dragon
pub(in crate::card::sets) static FORM_OF_THE_DRAGON: CardRecord = CardRecord::new(
    "Form of the Dragon",
    "2058bcb4-50ac-4323-ab49-3b80a5891894",
    "Carl Critchlow",
    CardRules::new_enchantment(mana_cost!("{4}{R}{R}{R}")).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "At the beginning of your upkeep, this enchantment deals 5 damage to any target.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(5),
            ),
        ),
        AbilityDef::triggered(
            "At the beginning of each end step, your life total becomes 5.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            EffectDef::SetLifeTotal {
                recipient: EffectRecipientDef::Controller,
                total: ValueDef::Constant(5),
            },
        ),
        AbilityDef::static_ability(
            "Creatures without flying can't attack you.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::AttackRestriction(
                    AttackRestrictionDef::prohibit(
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                            crate::card::KeywordAbility::Flying,
                        )),
                        AttackDefenderScopeDef::AffectedPlayer,
                    ),
                )),
            },
        ),
    ]),
);

// SCG 94 — Goblin Brigand
pub(in crate::card::sets) static GOBLIN_BRIGAND: CardRecord = CardRecord::new(
    "Goblin Brigand",
    "4b024afe-7a28-4e3b-afbd-b42f1c45f338",
    "Arnie Swekel",
    // Two power for two with the usual goblin catch: the opponent always
    // knows exactly what is attacking.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Warrior"], 2, 2)
        .with_ability(abilities::attacks_each_combat_if_able()),
);

// SCG 95 — Goblin Psychopath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_PSYCHOPATH: CardRecord = CardRecord::new(
    "Goblin Psychopath",
    "52287036-00f1-4b6d-8cd8-b8cbc70c5135",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// SCG 96 — Goblin War Strike (reprint)
const GOBLIN_WAR_STRIKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_p02::GOBLIN_WAR_STRIKE,
    "dce59945-37a2-4f09-8831-9d44b4a59ea7",
    "Pete Venters",
);

// SCG 97 — Goblin Warchief
pub(in crate::card::sets) static GOBLIN_WARCHIEF: CardRecord = CardRecord::new(
    "Goblin Warchief",
    "66864a4b-8924-40ef-a337-15b12413a158",
    "Greg Hildebrandt & Tim Hildebrandt",
    // The haste is what makes the discount matter: a Goblin cast for one
    // less that also attacks the turn it lands.
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Goblin", "Warrior"], 2, 2).with_abilities(
        &[
            AbilityDef::static_ability(
                "Goblin spells you cast cost {1} less to cast.",
                EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")),
                    PlayerRelation::You,
                    ValueDef::Constant(1),
                )),
            ),
            AbilityDef::static_ability(
                "Goblins you control have haste.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                },
            ),
        ],
    ),
);

// SCG 98 — Grip of Chaos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRIP_OF_CHAOS: CardRecord = CardRecord::new(
    "Grip of Chaos",
    "defbbd3a-0e7d-4af2-b25f-9003ddad0bf5",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// SCG 99 — Misguided Rage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISGUIDED_RAGE: CardRecord = CardRecord::new(
    "Misguided Rage",
    "74b5e00a-fef0-4711-9112-2fd067321890",
    "Michael Sutfin",
    crate::card::CardRules::unsupported(),
);

// SCG 100 — Pyrostatic Pillar
pub(in crate::card::sets) static PYROSTATIC_PILLAR: CardRecord = CardRecord::new(
    "Pyrostatic Pillar",
    "5973cd53-f6cd-4edc-b952-f6d3eef97988",
    "Pete Venters",
    // It hurts the cheap deck and leaves the expensive one alone, so the two
    // mana it costs are the last cheap thing its controller wants to cast.
    CardRules::new_enchantment(mana_cost!("{1}{R}")).with_ability(AbilityDef::triggered(
        "Whenever a player casts a spell with mana value 3 or less, this enchantment deals 2 \
         damage to that player.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::ManaValueAtMost(3)),
        EffectDef::damage(
            EffectRecipientDef::ControllerOfTriggeringObject,
            ValueDef::Constant(2),
        ),
    )),
);

// SCG 101 — Rock Jockey
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROCK_JOCKEY: CardRecord = CardRecord::new(
    "Rock Jockey",
    "8f836e90-3255-48bd-a174-6a127528551e",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// SCG 102 — Scattershot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCATTERSHOT: CardRecord = CardRecord::new(
    "Scattershot",
    "cf22f3e7-1626-4bab-9f62-7d4774704395",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// SCG 103 — Siege-Gang Commander
pub(in crate::card::sets) static SIEGE_GANG_COMMANDER: CardRecord = CardRecord::new(
    "Siege-Gang Commander",
    "92e78cec-aaf9-4fe8-887b-b7e356d63315",
    "Christopher Moeller",
    // Four bodies for five mana, and the ability turns any of them --
    // including itself -- into two damage anywhere.
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Goblin"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create three 1/1 red Goblin creature tokens.",
            EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1)
                .with_amount(3)
                .with_art(CardArt::new(
                    "09faad62-42ff-4e37-b8a5-d8e8a0f6d096",
                    "Wizard of Barge",
                )),
        ),
        AbilityDef::activated_with_targets(
            "{1}{R}, Sacrifice a Goblin: This creature deals 2 damage to any target.",
            &[
                CostDef::Mana(mana_cost!("{1}{R}")),
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
        ),
    ]),
);

// SCG 104 — Skirk Volcanist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKIRK_VOLCANIST: CardRecord = CardRecord::new(
    "Skirk Volcanist",
    "8cdfb7e3-e077-400a-868d-3f3811e7a35c",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// SCG 105 — Spark Spray
pub(in crate::card::sets) static SPARK_SPRAY: CardRecord = CardRecord::new(
    "Spark Spray",
    "f60d8716-4297-484c-8e02-c30ce2773a65",
    "Pete Venters",
    // One damage is barely a spell; being a cantrip against every deck that
    // has no one-toughness creature is what earns the slot.
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Spark Spray deals 1 damage to any target.",
            &const {
                [AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )]
            },
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
        abilities::cycling!(
            "Cycling {R} ({R}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{R}"))],
        ),
    ]),
);

// SCG 106 — Sulfuric Vortex
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SULFURIC_VORTEX: CardRecord = CardRecord::new(
    "Sulfuric Vortex",
    "79955e27-eef7-43bd-9895-e9209ed1537f",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// SCG 107 — Torrent of Fire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TORRENT_OF_FIRE: CardRecord = CardRecord::new(
    "Torrent of Fire",
    "feeee859-f64a-4cd8-be0b-ad60cff8812e",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// SCG 108 — Uncontrolled Infestation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNCONTROLLED_INFESTATION: CardRecord = CardRecord::new(
    "Uncontrolled Infestation",
    "d9ead6c3-a4e9-43e0-ae2a-6eb73033bc49",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// SCG 109 — Accelerated Mutation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ACCELERATED_MUTATION: CardRecord = CardRecord::new(
    "Accelerated Mutation",
    "282f808c-0b58-4b98-aeda-f606a10d1a4b",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// SCG 110 — Alpha Status
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALPHA_STATUS: CardRecord = CardRecord::new(
    "Alpha Status",
    "fd210c45-57f3-4d7d-93ba-81fe4298ade3",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// SCG 111 — Ambush Commander
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AMBUSH_COMMANDER: CardRecord = CardRecord::new(
    "Ambush Commander",
    "7485da91-a051-4680-8a25-c81fdaa77130",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// SCG 112 — Ancient Ooze
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANCIENT_OOZE: CardRecord = CardRecord::new(
    "Ancient Ooze",
    "3b57b41c-f99c-4525-8541-f025b7e31974",
    "Erica Gassalasca-Jape",
    crate::card::CardRules::unsupported(),
);

// SCG 113 — Break Asunder
pub(in crate::card::sets) static BREAK_ASUNDER: CardRecord = CardRecord::new(
    "Break Asunder",
    "fb989895-a5c7-4151-8620-ab277d826303",
    "Jim Nelson",
    // Four mana is a bad rate for the effect and a fine rate for a card that
    // is never stuck in your hand.
    CardRules::new_sorcery(mana_cost!("{2}{G}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target artifact or enchantment.",
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                )]
            },
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))],
        ),
    ]),
);

// SCG 114 — Claws of Wirewood
pub(in crate::card::sets) static CLAWS_OF_WIREWOOD: CardRecord = CardRecord::new(
    "Claws of Wirewood",
    "b94cd33f-40b6-4b11-97a4-8676ef27631e",
    "Tony Szczudlo",
    // Green's answer to fliers, with three damage to your own face as the
    // price -- and a cantrip when there are none.
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_abilities(&[
        AbilityDef::spell(
            "Claws of Wirewood deals 3 damage to each creature with flying and each player.",
            EffectDef::Sequence(
                &const {
                    [
                        EffectDef::damage(
                            EffectRecipientDef::matching_objects(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::Any,
                            ),
                            ValueDef::Constant(3),
                        ),
                        EffectDef::damage(EffectRecipientDef::EachPlayer, ValueDef::Constant(3)),
                    ]
                },
            ),
        ),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))],
        ),
    ]),
);

// SCG 115 — Decree of Savagery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DECREE_OF_SAVAGERY: CardRecord = CardRecord::new(
    "Decree of Savagery",
    "e643fbf1-74d5-412b-beba-ab3c712edb3b",
    "Alex Horley-Orlandelli",
    crate::card::CardRules::unsupported(),
);

// SCG 116 — Divergent Growth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIVERGENT_GROWTH: CardRecord = CardRecord::new(
    "Divergent Growth",
    "0e609448-7868-4e28-b399-3750556a693c",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// SCG 117 — Dragon Fangs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGON_FANGS: CardRecord = CardRecord::new(
    "Dragon Fangs",
    "9754f52f-8937-4402-8956-2c18b520898a",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// SCG 118 — Elvish Aberration
pub(in crate::card::sets) static ELVISH_ABERRATION: CardRecord = CardRecord::new(
    "Elvish Aberration",
    "137d326f-83e1-449a-b934-71c7986c64e7",
    "Matt Cavotta",
    // Three green mana out of a body, or a Forest out of the same card on
    // the turn there was no Forest.
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Elf", "Mutant"], 4, 5).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {G}{G}{G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green).with_amount(3)),
        ),
        abilities::typecycling!(
            "Forestcycling {2} ({2}, Discard this card: Search your library for a Forest card, \
            reveal it, put it into your hand, then shuffle.)",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Forest")),
        ),
    ]),
);

// SCG 119 — Fierce Empath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIERCE_EMPATH: CardRecord = CardRecord::new(
    "Fierce Empath",
    "d237e169-f152-4ddf-a5a1-32ca46cfa16d",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// SCG 120 — Forgotten Ancient
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORGOTTEN_ANCIENT: CardRecord = CardRecord::new(
    "Forgotten Ancient",
    "49d3b91d-2e4f-4574-89f8-7b804f1d21bf",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// SCG 121 — Hunting Pack
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HUNTING_PACK: CardRecord = CardRecord::new(
    "Hunting Pack",
    "8b0f5d29-5342-4591-bdc9-c2bc9289ed41",
    "Jim Nelson",
    crate::card::CardRules::unsupported(),
);

// SCG 122 — Krosan Drover
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROSAN_DROVER: CardRecord = CardRecord::new(
    "Krosan Drover",
    "e92a7141-119f-4bf8-a82d-eb7c0c37185c",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// SCG 123 — Krosan Warchief
pub(in crate::card::sets) static KROSAN_WARCHIEF: CardRecord = CardRecord::new(
    "Krosan Warchief",
    "435b700b-2072-47c0-9725-ad04414d2474",
    "Greg Hildebrandt",
    // A discount and a regeneration shield for the tribe, on a body the tribe
    // was going to play anyway.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Beast"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "Beast spells you cast cost {1} less to cast.",
            EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Beast")),
                PlayerRelation::You,
                ValueDef::Constant(1),
            )),
        ),
        AbilityDef::activated_with_targets(
            "{1}{G}: Regenerate target Beast.",
            &[CostDef::Mana(mana_cost!("{1}{G}"))],
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Beast")),
                )]
            },
            EffectDef::Regenerate {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// SCG 124 — Kurgadon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KURGADON: CardRecord = CardRecord::new(
    "Kurgadon",
    "52a1758c-849a-4de3-b674-857c3c9bf399",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// SCG 125 — One with Nature
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ONE_WITH_NATURE: CardRecord = CardRecord::new(
    "One with Nature",
    "2321b01c-7eef-48cc-a86b-4074dfa5b86b",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// SCG 126 — Primitive Etchings
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRIMITIVE_ETCHINGS: CardRecord = CardRecord::new(
    "Primitive Etchings",
    "eae26b8d-c3af-42d1-94f4-56950ceac1c7",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// SCG 127 — Root Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROOT_ELEMENTAL: CardRecord = CardRecord::new(
    "Root Elemental",
    "5161968e-b757-45b8-826f-98415291024d",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// SCG 128 — Sprouting Vines
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPROUTING_VINES: CardRecord = CardRecord::new(
    "Sprouting Vines",
    "3a3246a6-b604-4f9f-adb9-3692e0fa8638",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// SCG 129 — Titanic Bulvox
pub(in crate::card::sets) static TITANIC_BULVOX: CardRecord = CardRecord::new(
    "Titanic Bulvox",
    "3f42c4d7-b555-449c-a539-119c1ae62232",
    "Wayne England",
// Seven trampling power either way, and the morph cost buys the turn it
    // arrives rather than the mana it costs.
    CardRules::new_creature(mana_cost!("{6}{G}{G}"), &["Beast"], 7, 4)
        .with_morph(&[CostDef::Mana(mana_cost!("{4}{G}{G}{G}"))])
        .with_abilities(&[
            AbilityDef::alternative_cast(
                &[CostDef::Mana(mana_cost!("{3}"))],
                crate::card::face_down::morph_cast(),
                Some(
                    "Morph {4}{G}{G}{G} (You may cast this card face down as a 2/2 creature for {3}. \
                     Turn it face up any time for its morph cost.)",
                ),
                EffectDef::None,
            ),abilities::trample()]),
);

// SCG 130 — Treetop Scout
pub(in crate::card::sets) static TREETOP_SCOUT: CardRecord = CardRecord::new(
    "Treetop Scout",
    "2fa39646-a609-4b37-b8de-97893ae43c49",
    "Alan Pollack",
    // The same evasion for one mana on a body too small to use it,
    // which is why it was printed as a one-drop and not a threat.
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Scout"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked except by creatures with flying.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                    BlockRestrictionDef::prohibit(
                        BlockRestrictionSubjectDef::Attacker,
                        BlockRestrictionMatchDef::Except(ObjectPredicateDef::HasKeyword(
                            KeywordAbility::Flying,
                        )),
                    ),
                )),
            },
        ),
    ),
);

// SCG 131 — Upwelling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UPWELLING: CardRecord = CardRecord::new(
    "Upwelling",
    "21ab4600-1f71-48fa-a291-f5c5628c7395",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// SCG 132 — Wirewood Guardian
pub(in crate::card::sets) static WIREWOOD_GUARDIAN: CardRecord = CardRecord::new(
    "Wirewood Guardian",
    "e8676b1f-e37c-4ae1-9dbe-d000369fa422",
    "Mark Tedin",
    // Seven mana for a 6/6 is filler; being a Forest on turn two is what puts
    // it in the deck.
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Giant"], 6, 6).with_ability(
        abilities::typecycling!(
            "Forestcycling {2} ({2}, Discard this card: Search your library for a Forest card, \
            reveal it, put it into your hand, then shuffle.)",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Forest")),
        ),
    ),
);

// SCG 133 — Wirewood Symbiote
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WIREWOOD_SYMBIOTE: CardRecord = CardRecord::new(
    "Wirewood Symbiote",
    "49488b76-abaf-4dba-b01f-7b418e4ff295",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// SCG 134 — Woodcloaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WOODCLOAKER: CardRecord = CardRecord::new(
    "Woodcloaker",
    "4aa85e09-a1cd-473d-98cd-c6a7c7aff949",
    "Jim Nelson",
    crate::card::CardRules::unsupported(),
);

// SCG 135 — Xantid Swarm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static XANTID_SWARM: CardRecord = CardRecord::new(
    "Xantid Swarm",
    "6a87911a-3931-46aa-9348-2728c4b73b96",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// SCG 136 — Bladewing the Risen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLADEWING_THE_RISEN: CardRecord = CardRecord::new(
    "Bladewing the Risen",
    "2bd3e13d-53f8-42bf-aa83-09a9ca94a9f0",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// SCG 137 — Edgewalker
pub(in crate::card::sets) static EDGEWALKER: CardRecord = CardRecord::new(
    "Edgewalker",
    "c8b477c2-2cd5-41f2-8754-d4d5000df58d",
    "Ben Thompson",
CardRules::new_creature(mana_cost!("{1}{W}{B}"), &["Human", "Cleric"], 2, 2).with_ability(
        abilities::spell_colored_cost_reduction(
            "Cleric spells you cast cost {W}{B} less to cast. This effect reduces only the amount of colored mana you pay. (For example, if you cast a Cleric spell with mana cost {1}{W}, it costs {1} to cast.)",
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cleric")),
            PlayerRelation::You,
            mana_cost!("{W}{B}"),
        ),
    ),
);

// SCG 138 — Karona, False God
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KARONA_FALSE_GOD: CardRecord = CardRecord::new(
    "Karona, False God",
    "de53d083-251e-42a4-9e2e-c2978c80615b",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// SCG 139 — Sliver Overlord
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLIVER_OVERLORD: CardRecord = CardRecord::new(
    "Sliver Overlord",
    "3c16915b-c50d-4fb5-830f-9ca4597a9c0f",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// SCG 140 — Ark of Blight
pub(in crate::card::sets) static ARK_OF_BLIGHT: CardRecord = CardRecord::new(
    "Ark of Blight",
    "f3b09956-cc34-4472-8b9f-ae355522bd5a",
    "David Martin",
    // Land destruction the opponent can see coming for a whole turn, which
    // is why it costs two mana less than doing it at instant speed.
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::activated_with_targets(
        "{3}, {T}, Sacrifice this artifact: Destroy target land.",
        &[
            CostDef::Mana(mana_cost!("{3}")),
            CostDef::TapSource,
            CostDef::SacrificeSource,
        ],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Land),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    )),
);

// SCG 141 — Proteus Machine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROTEUS_MACHINE: CardRecord = CardRecord::new(
    "Proteus Machine",
    "d1c8cff1-b289-41a4-9fa3-cc5e7ba70802",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// SCG 142 — Stabilizer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STABILIZER: CardRecord = CardRecord::new(
    "Stabilizer",
    "b72dbe81-96d0-4b0d-97a7-c59345f081e8",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// SCG 143 — Temple of the False God
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEMPLE_OF_THE_FALSE_GOD: CardRecord = CardRecord::new(
    "Temple of the False God",
    "d7036f51-10a6-4036-8650-9bc12d2a55cb",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AGELESS_SENTINELS,
    &ASTRAL_STEEL,
    &AVEN_FARSEER,
    &AVEN_LIBERATOR,
    &DARU_SPIRITUALIST,
    &DARU_WARCHIEF,
    &DAWN_ELEMENTAL,
    &DECREE_OF_JUSTICE,
    &DIMENSIONAL_BREACH,
    &DRAGON_SCALES,
    &DRAGONSTALKER,
    &ETERNAL_DRAGON,
    &EXILED_DOOMSAYER,
    &FORCE_BUBBLE,
    &FRONTLINE_STRATEGIST,
    &GILDED_LIGHT,
    &GUILTY_CONSCIENCE,
    &KARONA_S_ZEALOT,
    &NOBLE_TEMPLAR,
    &RAIN_OF_BLADES,
    &RECUPERATE,
    &REWARD_THE_FAITHFUL,
    &SILVER_KNIGHT,
    &TRAP_DIGGER,
    &WING_SHARDS,
    &WIPE_CLEAN,
    &ZEALOUS_INQUISITOR,
    &APHETTO_RUNECASTER,
    &BRAIN_FREEZE,
    &COAST_WATCHER,
    &DAY_OF_THE_DRAGONS,
    &DECREE_OF_SILENCE,
    &DISPERSAL_SHIELD,
    &DRAGON_WINGS,
    &FACES_OF_THE_PAST,
    &FROZEN_SOLID,
    &HINDERING_TOUCH,
    &LONG_TERM_PLANS,
    &MERCURIAL_KITE,
    &METAMORPHOSE,
    &MIND_S_DESIRE,
    &MISCHIEVOUS_QUANAR,
    &MISTFORM_WARCHIEF,
    &PARALLEL_THOUGHTS,
    &PEMMIN_S_AURA,
    &RAVEN_GUILD_INITIATE,
    &RAVEN_GUILD_MASTER,
    &RIPTIDE_SURVIVOR,
    &RUSH_OF_KNOWLEDGE,
    &SCORNFUL_EGOTIST,
    &SHORELINE_RANGER,
    &STIFLE,
    &TEMPORAL_FISSURE,
    &THUNDERCLOUD_ELEMENTAL,
    &BLADEWING_S_THRALL,
    &CABAL_CONDITIONING,
    &CABAL_INTERROGATOR,
    &CALL_TO_THE_GRAVE,
    &CARRION_FEEDER,
    &CHILL_HAUNTING,
    &CLUTCH_OF_UNDEATH,
    &CONSUMPTIVE_GOO,
    &DEATH_S_HEAD_BUZZARD,
    &DECREE_OF_PAIN,
    &DRAGON_SHADOW,
    &FATAL_MUTATION,
    &FINAL_PUNISHMENT,
    &LETHAL_VAPORS,
    &LINGERING_DEATH,
    &NEFASHU,
    &PUTRID_RAPTOR,
    &REAPING_THE_GRAVES,
    &SKULLTAP,
    &SOUL_COLLECTOR,
    &TENDRILS_OF_AGONY,
    &TWISTED_ABOMINATION,
    &UNBURDEN,
    &UNDEAD_WARCHIEF,
    &UNSPEAKABLE_SYMBOL,
    &VENGEFUL_DEAD,
    &ZOMBIE_CUTTHROAT,
    &BONETHORN_VALESK,
    &CARBONIZE,
    &CHARTOOTH_COUGAR,
    &DECREE_OF_ANNIHILATION,
    &DRAGON_BREATH,
    &DRAGON_MAGE,
    &DRAGON_TYRANT,
    &DRAGONSPEAKER_SHAMAN,
    &DRAGONSTORM,
    &ENRAGE,
    &EXTRA_ARMS,
    &FORM_OF_THE_DRAGON,
    &GOBLIN_BRIGAND,
    &GOBLIN_PSYCHOPATH,
    &GOBLIN_WARCHIEF,
    &GRIP_OF_CHAOS,
    &MISGUIDED_RAGE,
    &PYROSTATIC_PILLAR,
    &ROCK_JOCKEY,
    &SCATTERSHOT,
    &SIEGE_GANG_COMMANDER,
    &SKIRK_VOLCANIST,
    &SPARK_SPRAY,
    &SULFURIC_VORTEX,
    &TORRENT_OF_FIRE,
    &UNCONTROLLED_INFESTATION,
    &ACCELERATED_MUTATION,
    &ALPHA_STATUS,
    &AMBUSH_COMMANDER,
    &ANCIENT_OOZE,
    &BREAK_ASUNDER,
    &CLAWS_OF_WIREWOOD,
    &DECREE_OF_SAVAGERY,
    &DIVERGENT_GROWTH,
    &DRAGON_FANGS,
    &ELVISH_ABERRATION,
    &FIERCE_EMPATH,
    &FORGOTTEN_ANCIENT,
    &HUNTING_PACK,
    &KROSAN_DROVER,
    &KROSAN_WARCHIEF,
    &KURGADON,
    &ONE_WITH_NATURE,
    &PRIMITIVE_ETCHINGS,
    &ROOT_ELEMENTAL,
    &SPROUTING_VINES,
    &TITANIC_BULVOX,
    &TREETOP_SCOUT,
    &UPWELLING,
    &WIREWOOD_GUARDIAN,
    &WIREWOOD_SYMBIOTE,
    &WOODCLOAKER,
    &XANTID_SWARM,
    &BLADEWING_THE_RISEN,
    &EDGEWALKER,
    &KARONA_FALSE_GOD,
    &SLIVER_OVERLORD,
    &ARK_OF_BLIGHT,
    &PROTEUS_MACHINE,
    &STABILIZER,
    &TEMPLE_OF_THE_FALSE_GOD,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[GOBLIN_WAR_STRIKE_REPRINT];
