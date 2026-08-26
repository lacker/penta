//! Future Sight cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AddManaEffectDef,
    AppliedEffectDef, ArrivalAttachmentDef, CardArt, CardRules, CardSet, CardType, CounterKind,
    CreatureStats, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef, ObjectRefDef,
    PlayerRelation, ResolvedEffectDurationDef, SpellResolutionDestinationDef, TriggerConditionDef,
    TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// FUT 43 — Reality Strobe
// Audit: partial — Its spell effect and self-exile with time counters are executable, but suspend's upkeep counter removal and free cast from exile need the shared exile-casting lifecycle.
static REALITY_STROBE_TIME_COUNTERS: [(CounterKind, u16); 1] = [(CounterKind::named("time"), 3)];

pub(in crate::card::sets) static REALITY_STROBE: CardRecord = CardRecord::new_with_legacy_id(
    1709,
    "Reality Strobe",
    CardArt::new("8e6d881a-f7b1-471f-bc0b-64a79bb491c9", "Dan Murayama Scott"),
    CardSet::FutureSight,
    CardRules::new_sorcery(mana_cost!("{4}{U}{U}")).with_ability(
        AbilityDef::spell_with_targets(
            "Return target permanent to its owner's hand. Exile Reality Strobe with three time counters on it.\n\nSuspend 3—{2}{U} (Rather than cast this card from your hand, you may pay {2}{U} and exile it with three time counters on it. At the beginning of your upkeep, remove a time counter. When the last is removed, you may cast it without paying its mana cost.)",
            &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Any)],
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                from: None,
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                controller: None,
                            tapped: false,
},
        )
        .with_resolution_destination(SpellResolutionDestinationDef::ExileWithCounters(
            &REALITY_STROBE_TIME_COUNTERS,
        ))
        .with_coverage(AbilityCoverageDef::partial(
            "Suspend's upkeep counter removal and free cast from exile need the shared exile-casting lifecycle.",
        )),
    ),
);

// FUT 54 — Narcomoeba
static NARCOMOEBA_ENTERS: EffectDef = EffectDef::MoveToZone {
    object: EffectRecipientDef::Source,
    from: Some(ZoneKind::Graveyard),
    zone: ZoneKind::Battlefield,
    placement: ZonePlacement::Top,
    controller: None,
    arrival_effect: None,
    attachment: None,
    counters: None,
    tapped: false,
};

pub(in crate::card::sets) static NARCOMOEBA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f76b3746-2e2c-4560-a2d2-e7b5b92833b2"),
    "Narcomoeba",
    CardArt::new("f76b3746-2e2c-4560-a2d2-e7b5b92833b2", "Matt Stewart"),
    CardSet::FutureSight,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Illusion"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "When this card is put into your graveyard from your library, you may put it onto the battlefield.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Library),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &NARCOMOEBA_ENTERS,
            },
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// FUT 76 — Shimian Specter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHIMIAN_SPECTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e6faa406-aa7a-49ce-a42e-00e98f3fb74e"),
    "Shimian Specter",
    crate::card::CardArt::new("e6faa406-aa7a-49ce-a42e-00e98f3fb74e", "Anthony S. Waters"),
    crate::card::CardSet::FutureSight,
    crate::card::CardRules::unsupported(),
);

static BRIDGE_FROM_BELOW_IS_IN_GRAVEYARD: TriggerConditionDef =
    TriggerConditionDef::SourceInZone(ZoneKind::Graveyard);

static BRIDGE_FROM_BELOW_OWN_NONTOKEN_CREATURE: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
    ObjectPredicateDef::OwnedBy(PlayerRelation::You),
]);

static BRIDGE_FROM_BELOW_OPPONENT_CREATURE: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::OwnedBy(PlayerRelation::Opponent),
]);

// FUT 81 — Bridge from Below
pub(in crate::card::sets) static BRIDGE_FROM_BELOW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("52c44610-6d4b-4c14-839f-2c085badec90"),
    "Bridge from Below",
    CardArt::new(
        "52c44610-6d4b-4c14-839f-2c085badec90",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    CardSet::FutureSight,
    CardRules::new_enchantment(mana_cost!("{B}{B}{B}")).with_abilities(&[
        AbilityDef::triggered_if(
            "Whenever a nontoken creature is put into your graveyard from the battlefield, if this card is in your graveyard, create a 2/2 black Zombie creature token.",
            TriggerEventDef::zone_changed(
                BRIDGE_FROM_BELOW_OWN_NONTOKEN_CREATURE,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            &BRIDGE_FROM_BELOW_IS_IN_GRAVEYARD,
            EffectDef::create_creature_token(&["Zombie"], &[ManaColor::Black], 2, 2),
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
        AbilityDef::triggered_if(
            "When a creature is put into an opponent's graveyard from the battlefield, if this card is in your graveyard, exile this card.",
            TriggerEventDef::zone_changed(
                BRIDGE_FROM_BELOW_OPPONENT_CREATURE,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            &BRIDGE_FROM_BELOW_IS_IN_GRAVEYARD,
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                from: None,
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
                controller: None,
                arrival_effect: None,
                attachment: None,
                counters: None,
                            tapped: false,
},
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// FUT 138 — Sprout Swarm
pub(in crate::card::sets) static SPROUT_SWARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b915355-4e98-44df-81bd-961a3d3c86b8"),
    "Sprout Swarm",
    CardArt::new("0b915355-4e98-44df-81bd-961a3d3c86b8", "Chippy"),
    CardSet::FutureSight,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[
        abilities::convoke(),
        abilities::buyback(mana_cost!("{3}")),
        AbilityDef::spell(
            "Create a 1/1 green Saproling creature token.",
            EffectDef::create_creature_token(&["Saproling"], &[ManaColor::Green], 1, 1),
        ),
    ]),
);

// FUT 161 — Coalition Relic
/// The printed clause removes the counters and then adds the mana, but the
/// amount is read off the counters, so the two steps are written the other
/// way round. One resolution, no priority in between, and nothing else in
/// the pool watches a charge counter leave: what is observable is that the
/// counters are gone and that many mana arrived.
static RELIC_CASHES_IN: [EffectDef; 2] = [
    EffectDef::AddMana(
        AddManaEffectDef::any_color()
            .with_variable_amount(ValueDef::CountersOnSource(CounterKind::named("charge"))),
    ),
    EffectDef::RemoveAllCounters {
        object: EffectRecipientDef::Source,
        kind: Some(CounterKind::named("charge")),
    },
];

pub(in crate::card::sets) static COALITION_RELIC: CardRecord = CardRecord::new_with_legacy_id(
    2197,
    "Coalition Relic",
    CardArt::new("7a7c98b0-d64d-4d0a-b284-1187a8e7095e", "Donato Giancola"),
    CardSet::FutureSight,
    // Three mana that fixes on the turn it lands and ramps on every one
    // after, provided nothing needs the Relic tapped for mana that turn.
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
        AbilityDef::activated(
            "{T}: Put a charge counter on this artifact.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("charge"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::triggered(
            "At the beginning of your first main phase, remove all charge counters from this artifact. Add one mana of any color for each charge counter removed this way.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::PrecombatMain,
                player: PlayerRelation::You,
            },
            EffectDef::Sequence(&RELIC_CASHES_IN),
        ),
    ]),
);

// FUT 165 — Sword of the Meek
/// Read as the creature enters, so a 1/1 that is only a 1/1 because of what
/// is already on the battlefield still counts, and a 2/2 shrunk to 1/1 does
/// too.
static A_ONE_ONE_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::PowerExactly(1),
    ObjectPredicateDef::ToughnessExactly(1),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

/// The attachment rides the return rather than following it: what comes back
/// from the graveyard is a new object, so a later effect would have nothing
/// left to name.
static SWORD_RETURNS_AND_EQUIPS: EffectDef = EffectDef::MoveToZone {
    counters: None,
    object: EffectRecipientDef::Source,
    from: None,
    zone: ZoneKind::Battlefield,
    placement: ZonePlacement::Top,
    controller: None,
    arrival_effect: None,
    attachment: Some(ArrivalAttachmentDef::ArrivalToHost(
        ObjectRefDef::TriggeringObject,
    )),
    tapped: false,
};

static SWORD_OF_THE_MEEK_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::static_ability(
        "Equipped creature gets +1/+2.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::AttachedPermanent,
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(2),
            ),
        },
    ),
    abilities::equip(
        &[AbilityCostDef::Mana(mana_cost!("{2}"))],
        "Equip {2} ({2}: Attach to target creature you control. Equip only as a sorcery.)",
    ),
    AbilityDef::triggered(
        "Whenever a 1/1 creature you control enters, you may return this card from your graveyard \
         to the battlefield, then attach it to that creature.",
        TriggerEventDef::zone_changed(A_ONE_ONE_YOU_CONTROL, None, Some(ZoneKind::Battlefield)),
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &SWORD_RETURNS_AND_EQUIPS,
        },
    )
    .with_source_zones(&[ZoneKind::Graveyard]),
];

pub(in crate::card::sets) static SWORD_OF_THE_MEEK: CardRecord = CardRecord::new_with_legacy_id(
    2220,
    "Sword of the Meek",
    CardArt::new("e9f13705-6ede-4c29-a2b4-a082bf69e9c5", "Franz Vohwinkel"),
    CardSet::FutureSight,
    // On its own it is a bad Equipment. Beside anything that makes 1/1s for
    // free it is an engine that never runs out of Swords.
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&SWORD_OF_THE_MEEK_ABILITIES),
);

// FUT 167 — Darksteel Garrison
pub(in crate::card::sets) static DARKSTEEL_GARRISON: CardRecord = CardRecord::new_with_legacy_id(
    1702,
    "Darksteel Garrison",
    CardArt::new("e77eaaa0-40f9-40e4-b0ba-5a8addd764d3", "David Martin"),
    CardSet::FutureSight,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Fortification"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Fortified land has indestructible.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                },
            ),
            AbilityDef::triggered_with_targets(
                "Whenever fortified land becomes tapped, target creature gets +1/+1 until end of turn.",
                TriggerEventDef::tapped(ObjectPredicateDef::AttachedToSource),
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            abilities::fortify(
                mana_cost!("{3}"),
                "Fortify {3} ({3}: Attach to target land you control. Fortify only as a sorcery. This card enters unattached and stays on the battlefield if the land leaves.)",
            ),
        ]),
);

// FUT 174 — Dryad Arbor
pub(in crate::card::sets) static DRYAD_ARBOR: CardRecord = CardRecord::new_with_legacy_id(
    252,
    "Dryad Arbor",
    CardArt::new("8cee476d-42e1-4997-87af-73e18f542167", "Eric Fortune"),
    CardSet::FutureSight,
    CardRules::new_land(&[])
        .with_type(CardType::Creature)
        .with_subtypes(&["Forest", "Dryad"])
        .with_creature_stats(CreatureStats {
            power: 1,
            toughness: 1,
        })
        .printed_colors(&[ManaColor::Green]),
);

// FUT 177 — Horizon Canopy
static HORIZON_CANOPY_COLORS: [ManaColor; 2] = [ManaColor::Green, ManaColor::White];

static HORIZON_CANOPY_ABILITIES: [AbilityDef; 2] =
    abilities::horizon_land("{T}, Pay 1 life: Add {G} or {W}.", &HORIZON_CANOPY_COLORS);

pub(in crate::card::sets) static HORIZON_CANOPY: CardRecord = CardRecord::new_with_legacy_id(
    2285,
    "Horizon Canopy",
    CardArt::new("d5dfc25d-a17b-4ead-9484-e8a18b8fa176", "Michael Komarck"),
    CardSet::FutureSight,
    // The original of the cycle Modern Horizons finished twelve years later,
    // and still the one the cube wants: a dual that costs life to use and a
    // card when there is nothing left to use it on.
    CardRules::new_land(&[]).with_abilities(&HORIZON_CANOPY_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &REALITY_STROBE,
    &NARCOMOEBA,
    &SHIMIAN_SPECTER,
    &BRIDGE_FROM_BELOW,
    &SPROUT_SWARM,
    &COALITION_RELIC,
    &SWORD_OF_THE_MEEK,
    &DARKSTEEL_GARRISON,
    &DRYAD_ARBOR,
    &HORIZON_CANOPY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
