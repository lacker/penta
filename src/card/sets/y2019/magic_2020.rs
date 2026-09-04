//! Core Set 2020 cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AddManaEffectDef, AppliedEffectDef,
    AppliedRuleDef, CardArt, CardRules, CardSet, CardType, ComparisonDef, CounterKind, EffectDef,
    EffectRecipientDef, ManaColor, ObjectPredicateDef, ObjectQueryDef, PlayerRelation,
    PlayerSetDef, ResolvedEffectDurationDef, TriggerConditionDef, TriggerEventDef,
    ValueComparisonDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// M20 3 — Ancestral Blade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANCESTRAL_BLADE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2020,
    "Ancestral Blade",
    "2ba18114-af6c-48cd-82c9-eb6541d566bf",
    "Scott Murphy",
    crate::card::CardRules::unsupported(),
);

// M20 34 — Raise the Alarm (reprint)
const RAISE_THE_ALARM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2003::mirrodin::RAISE_THE_ALARM,
    "764a7a53-314e-4b1f-aa33-0f312d06df71",
    "Zoltan Boros",
);

// M20 54 — Cloudkin Seer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOUDKIN_SEER: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2020,
    "Cloudkin Seer",
    "e2111753-a930-403f-9d94-a86dfcb069da",
    "Anastasia Ovchinnikova",
    crate::card::CardRules::unsupported(),
);

// M20 148 — Leyline of Combustion
// Audit: unsupported — Needs one grouped trigger for a spell or ability targeting the player and/or any controlled permanents.
pub(in crate::card::sets) static LEYLINE_OF_COMBUSTION: CardRecord = CardRecord::new(
    CardSet::Magic2020,
    "Leyline of Combustion",
    "3a93c8e2-fb27-43af-83a7-2bd4d40e0eff",
    "Noah Bradley",
    CardRules::unsupported(),
);

// M20 169 — Elvish Reclaimer
pub(in crate::card::sets) static ELVISH_RECLAIMER: CardRecord = CardRecord::new(
    CardSet::Magic2020,
    "Elvish Reclaimer",
    "39c431d7-d94b-46c4-bb89-f3db56214ab4",
    "Victor Adame Minguez",
    // One mana for a body that turns a spent fetchland into whatever land
    // the deck is built around, and is a 3/4 by the time it has done it
    // twice.
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Warrior"], 1, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature gets +2/+2 as long as there are three or more land cards in your \
             graveyard.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    // "Three or more land cards in your graveyard": the fetchlands that made
                    // him a 3/4 are the same ones his own ability puts there, which is why he
                    // grows on the turn he is used.
                    query: ObjectQueryDef::owned_by(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Graveyard],
                        PlayerSetDef::Related(PlayerRelation::You),
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 3,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                },
            },
        ),
        AbilityDef::activated(
            "{2}, {T}, Sacrifice a land: Search your library for a land card, put it onto the \
             battlefield tapped, then shuffle.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::HasType(CardType::Land),
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
        ),
    ]),
);

// M20 179 — Leyline of Abundance
pub(in crate::card::sets) static LEYLINE_OF_ABUNDANCE: CardRecord = CardRecord::new(
    CardSet::Magic2020,
    "Leyline of Abundance",
    "c68e8342-78d2-4826-a287-64c371b97d19",
    "Noah Bradley",
    CardRules::new_enchantment(mana_cost!("{2}{G}{G}")).with_abilities(&[
        abilities::begin_game_on_battlefield(
            "If this card is in your opening hand, you may begin the game with it on the battlefield.",
        ),
        AbilityDef::triggered_mana(
            "Whenever you tap a creature for mana, add an additional {G}.",
            TriggerEventDef::tapped_for_mana(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ),
        AbilityDef::activated(
            "{6}{G}{G}: Put a +1/+1 counter on each creature you control.",
            &[AbilityCostDef::Mana(mana_cost!("{6}{G}{G}"))],
            EffectDef::AddCounters {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// M20 230 — Manifold Key
pub(in crate::card::sets) static MANIFOLD_KEY: CardRecord = CardRecord::new(
    CardSet::Magic2020,
    "Manifold Key",
    "715e637a-dfd8-45a0-b1ea-53e4abd29307",
    "Lake Hurwitz",
    // One mana that untaps a Mox for profit and, when there is nothing to
    // untap, pushes a creature through instead.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}, {T}: Untap another target artifact.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
            ],
            // "Another" excludes the Key itself, which is what stops it untapping
            // itself for free every turn.
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
        AbilityDef::activated_with_targets(
            "{3}, {T}: Target creature can't be blocked this turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M20 247 — Field of the Dead
pub(in crate::card::sets) static FIELD_OF_THE_DEAD: CardRecord = CardRecord::new(
    CardSet::Magic2020,
    "Field of the Dead",
    "470ca3f4-29aa-4c4c-8ff2-8cdd70c69943",
    "Kev Walker",
    // A land that makes colourless and comes in tapped, which is what a deck
    // pays for turning every land drop after the seventh into a 2/2.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::triggered_if(
            "Whenever this land or another land you control enters, if you control seven or more \
             lands with different names, create a 2/2 black Zombie creature token.",
            // "This land or another land you control": the Field's own arrival counts,
            // which is what makes the seventh land the one that starts it.
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            &// The Field itself is one of the seven, and so is every other land you
                // control -- what is counted is names rather than lands, which is why a
                // deck built for this plays one of each dual rather than four of one.
                TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::DistinctNamesAmong(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(7),
                }),
            EffectDef::create_creature_token(&["Zombie"], &[ManaColor::Black], 2, 2).with_art(
                CardArt::new("18f0436e-9328-4266-9cf8-80b557a0c17c", "Anna Steinbauer"),
            ),
        ),
    ]),
);

// M20 297 — Wildfire Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILDFIRE_ELEMENTAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2020,
    "Wildfire Elemental",
    "272e317c-55c4-43b2-91aa-3e0009cfd7d5",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANCESTRAL_BLADE,
    &CLOUDKIN_SEER,
    &LEYLINE_OF_COMBUSTION,
    &ELVISH_RECLAIMER,
    &LEYLINE_OF_ABUNDANCE,
    &MANIFOLD_KEY,
    &FIELD_OF_THE_DEAD,
    &WILDFIRE_ELEMENTAL,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[RAISE_THE_ALARM_REPRINT];
