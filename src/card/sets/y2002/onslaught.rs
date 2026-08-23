//! Onslaught cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, BasicLandType, CardArt, CardRules, CardSet, CardType,
    DiscardSelectionDef, EffectDef, EffectPaymentCostDef, EffectPaymentDef, EffectRecipientDef,
    ManaColor, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, PayOrDef, PlayerRefDef,
    PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef, ScaledValueDef, TriggerEventDef,
    ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, TurnStepDef, mana_cost};

// ONS 2 — Akroma's Vengeance
pub(in crate::card::sets) static AKROMAS_VENGEANCE: CardRecord = CardRecord::new_with_legacy_id(
    2023,
    "Akroma's Vengeance",
    CardArt::new(
        "5e33aaf7-7490-4b64-a966-82fbf7ca8686",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    CardSet::Onslaught,
    // Six mana is a lot for a sweeper, and the cycling is what makes it
    // maindeckable anyway: the card is never dead.
    CardRules::new_sorcery(mana_cost!("{4}{W}{W}")).with_abilities(&[
        AbilityDef::spell(
            "Destroy all artifacts, creatures, and enchantments.",
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                can_regenerate: true,
            },
        ),
        abilities::cycling(
            "Cycling {3} ({3}, Discard this card: Draw a card.)",
            mana_cost!("{3}"),
        ),
    ]),
);

/// "Each other attacking Goblin", so the Piledriver never counts itself and
/// a lone one gets nothing.
static OTHER_ATTACKING_GOBLINS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::Subtype("Goblin"),
        ObjectPredicateDef::Attacking,
        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::Any,
);

static GOBLIN_PILEDRIVER_BONUS: ScaledValueDef = ScaledValueDef {
    value: ValueDef::CountMatchingObjects(&OTHER_ATTACKING_GOBLINS),
    factor: 2,
};

/// A land of their choice, sacrificed by whoever just had a permanent
/// bounced. Paying buys the copy, which is what turns one Chain of Vapor into
/// a board sweep in a deck holding the lands to spend.
static CHAIN_OF_VAPOR_REBOUND: EffectDef = EffectDef::PayOr(PayOrDef::optional(
    EffectPaymentDef {
        payer: PlayerSetDef::One(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
            TargetIndex::PRIMARY,
        ))),
        cost: EffectPaymentCostDef::SacrificePermanentMatching(ObjectPredicateDef::HasType(
            CardType::Land,
        )),
    },
    &EffectDef::CopyResolvingSpell {
        chooser: PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
        count: ValueDef::Constant(1),
    },
));

static A_NONLAND_PERMANENT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
)];

// ONS 28 — Exalted Angel
pub(in crate::card::sets) static EXALTED_ANGEL: CardRecord = CardRecord::new_with_legacy_id(
    2076,
    "Exalted Angel",
    CardArt::new("d75cc975-0f7e-48e7-a693-453306e5a907", "Michael Sutfin"),
    CardSet::Onslaught,
    // Six mana is more than a control deck wants to pay on turn four, so it
    // comes down face down on three and stands up on the next turn instead.
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Angel"], 4, 5)
        .with_morph(mana_cost!("{2}{W}{W}"))
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever this creature deals damage, you gain that much life.",
                TriggerEventDef::damage_dealt_by(ObjectPredicateDef::Source),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::TriggerEventAmount,
                },
            ),
            AbilityDef::alternative_cast(
                mana_cost!("{3}"),
                crate::card::face_down::morph_cast(),
                Some(
                    "Morph {2}{W}{W} (You may cast this card face down as a 2/2 creature for {3}. Turn it face up any time for its morph cost.)",
                ),
                EffectDef::None,
            ),
        ]),
);

// ONS 73 — Chain of Vapor
pub(in crate::card::sets) static CHAIN_OF_VAPOR: CardRecord = CardRecord::new_with_legacy_id(
    2062,
    "Chain of Vapor",
    CardArt::new("30f6b4a2-4e64-4d0e-9dbb-2b6a5b8f5b1f", "Carl Critchlow"),
    CardSet::Onslaught,
    // One mana to undo anything, and the chain is the opponent's to continue
    // or stop.
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target nonland permanent to its owner's hand. Then that permanent's controller may sacrifice a land of their choice. If the player does, they may copy this spell and may choose a new target for that copy.",
        &A_NONLAND_PERMANENT,
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
            },
            CHAIN_OF_VAPOR_REBOUND,
        ]),
    )),
);

static CHAIN_OF_SMOG_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

/// The copy costs nothing here, unlike Chain of Vapor's land. Whoever was
/// just hit decides whether to pass it on, and picks the next target -- which
/// is why the chain usually stops at whoever cannot afford to keep it going.
static CHAIN_OF_SMOG_REBOUND: EffectDef = EffectDef::May {
    player: EffectRecipientDef::player(PlayerRefDef::Target(TargetIndex::PRIMARY)),
    effect: &EffectDef::CopyResolvingSpell {
        chooser: PlayerRefDef::Target(TargetIndex::PRIMARY),
        count: ValueDef::Constant(1),
    },
};

// ONS 132 — Chain of Smog
pub(in crate::card::sets) static CHAIN_OF_SMOG: CardRecord = CardRecord::new_with_legacy_id(
    2155,
    "Chain of Smog",
    CardArt::new("6bfe64f9-8b03-41f6-a47b-fade397ad9d1", "Greg Staples"),
    CardSet::Onslaught,
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player discards two cards. That player may copy this spell and may choose a new target for that copy.",
        &CHAIN_OF_SMOG_TARGET,
        EffectDef::Sequence(&[
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
            CHAIN_OF_SMOG_REBOUND,
        ]),
    )),
);

// ONS 205 — Goblin Piledriver
pub(in crate::card::sets) static GOBLIN_PILEDRIVER: CardRecord = CardRecord::new_with_legacy_id(
    2019,
    "Goblin Piledriver",
    CardArt::new("f6c4df1f-f148-42ec-8e22-e7114216927d", "Matt Cavotta"),
    CardSet::Onslaught,
    // Protection from blue is half the card: it walks past the format's
    // blue blockers while the rest of the team makes it enormous.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Warrior"], 1, 2).with_abilities(&[
        abilities::protection_from(ManaColor::Blue),
        AbilityDef::triggered(
            "Whenever this creature attacks, it gets +2/+0 until end of turn for each other attacking Goblin.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Scaled(&GOBLIN_PILEDRIVER_BONUS),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ONS 206 — Goblin Pyromancer
pub(in crate::card::sets) static GOBLIN_PYROMANCER: CardRecord = CardRecord::new_with_legacy_id(
    307,
    "Goblin Pyromancer",
    CardArt::new(
        "bb4815b7-fc20-44a4-ad1c-66d92993557f",
        "Edward P. Beard, Jr.",
    ),
    CardSet::Onslaught,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Goblin", "Wizard"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "When this creature enters, Goblin creatures get +3/+0 until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Goblin"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::triggered(
            "At the beginning of the end step, destroy all Goblins.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Subtype("Goblin"),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                can_regenerate: true,
            },
        ),
    ]),
);

// ONS 207 — Goblin Sharpshooter
pub(in crate::card::sets) static GOBLIN_SHARPSHOOTER: CardRecord = CardRecord::new_with_legacy_id(
    292,
    "Goblin Sharpshooter",
    CardArt::new("7e689df7-b85d-4346-bee8-5e978b5cbbbc", "Greg Staples"),
    CardSet::Onslaught,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin"], 1, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This creature doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::triggered(
            "Whenever a creature dies, untap this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::HasType(CardType::Creature),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 1 damage to any target.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ONS 230 — Skirk Prospector
pub(in crate::card::sets) static SKIRK_PROSPECTOR: CardRecord = CardRecord::new_with_legacy_id(
    2028,
    "Skirk Prospector",
    CardArt::new("eb545dcd-3a7a-46a7-9c35-d28faebc6d17", "Doug Chaffee"),
    CardSet::Onslaught,
    // A one-drop that turns the rest of the board into mana, including
    // itself: the sacrifice names any Goblin, and the Prospector is one.
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "Sacrifice a Goblin: Add {R}.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::Subtype("Goblin"),
                controller: PlayerRelation::You,
            }],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
    ),
);

// ONS 235 — Sparksmith
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPARKSMITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("15a4460d-3fe8-4b1f-9990-0a19c3345367"),
    "Sparksmith",
    crate::card::CardArt::new("15a4460d-3fe8-4b1f-9990-0a19c3345367", "Jim Nelson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 275 — Naturalize
pub(in crate::card::sets) static NATURALIZE: CardRecord = CardRecord::new_with_legacy_id(
    270,
    "Naturalize",
    CardArt::new("c0acc41f-b55b-47cb-8803-d39d72788799", "Ron Spears"),
    CardSet::Onslaught,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::destroy_target(
        "Destroy target artifact or enchantment.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Enchantment),
        ])),
        true,
    )),
);

const fn fetch_land(text: &'static str, land_types: &'static [BasicLandType]) -> CardRules {
    CardRules::new_land(&[]).with_ability(AbilityDef::activated(
        text,
        &[
            AbilityCostDef::TapSource,
            AbilityCostDef::PayLife(1),
            AbilityCostDef::SacrificeSource,
        ],
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasAnyBasicLandType(land_types),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            binding: None,
            then: None,
        },
    ))
}

// ONS 313 — Bloodstained Mire
pub(in crate::card::sets) static BLOODSTAINED_MIRE: CardRecord = CardRecord::new_with_legacy_id(
    1363,
    "Bloodstained Mire",
    CardArt::new("68c72226-6f52-4322-8b14-18737293dfa0", "Rob Alexander"),
    CardSet::Onslaught,
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Swamp or Mountain card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Swamp, BasicLandType::Mountain],
    ),
);

// ONS 316 — Flooded Strand
pub(in crate::card::sets) static FLOODED_STRAND: CardRecord = CardRecord::new_with_legacy_id(
    283,
    "Flooded Strand",
    CardArt::new("b4e3d844-d3b4-41d8-921d-c1cb3af343f8", "Rob Alexander"),
    CardSet::Onslaught,
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Plains or Island card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Plains, BasicLandType::Island],
    ),
);

// ONS 321 — Polluted Delta
pub(in crate::card::sets) static POLLUTED_DELTA: CardRecord = CardRecord::new_with_legacy_id(
    1364,
    "Polluted Delta",
    CardArt::new("0f7585c8-9e21-4eef-afc1-2852de23db2f", "Rob Alexander"),
    CardSet::Onslaught,
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for an Island or Swamp card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Island, BasicLandType::Swamp],
    ),
);

// ONS 324 — Secluded Steppe
pub(in crate::card::sets) static SECLUDED_STEPPE: CardRecord = CardRecord::new_with_legacy_id(
    2024,
    "Secluded Steppe",
    CardArt::new("ea454280-f7f4-4315-bb46-b56050c02c97", "Heather Hudson"),
    CardSet::Onslaught,
    // The tapped land you play on a turn you had nothing to do, or the card
    // you cycle away on a turn you did.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
        ),
        abilities::cycling(
            "Cycling {W} ({W}, Discard this card: Draw a card.)",
            mana_cost!("{W}"),
        ),
    ]),
);

// ONS 328 — Windswept Heath
pub(in crate::card::sets) static WINDSWEPT_HEATH: CardRecord = CardRecord::new_with_legacy_id(
    1365,
    "Windswept Heath",
    CardArt::new("7a7c5941-9c8a-4a40-9efb-a84f05c58e53", "Anthony S. Waters"),
    CardSet::Onslaught,
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Forest or Plains card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Forest, BasicLandType::Plains],
    ),
);

// ONS 330 — Wooded Foothills
pub(in crate::card::sets) static WOODED_FOOTHILLS: CardRecord = CardRecord::new_with_legacy_id(
    284,
    "Wooded Foothills",
    CardArt::new("cdad38f7-9dfa-4f1b-9fac-41ab2b253f53", "Rob Alexander"),
    CardSet::Onslaught,
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Mountain or Forest card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Mountain, BasicLandType::Forest],
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AKROMAS_VENGEANCE,
    &EXALTED_ANGEL,
    &CHAIN_OF_VAPOR,
    &CHAIN_OF_SMOG,
    &GOBLIN_PILEDRIVER,
    &GOBLIN_PYROMANCER,
    &GOBLIN_SHARPSHOOTER,
    &SKIRK_PROSPECTOR,
    &SPARKSMITH,
    &NATURALIZE,
    &BLOODSTAINED_MIRE,
    &FLOODED_STRAND,
    &POLLUTED_DELTA,
    &SECLUDED_STEPPE,
    &WINDSWEPT_HEATH,
    &WOODED_FOOTHILLS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
