//! Future Sight cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, ArrivalAttachmentDef, BattlefieldEntryModificationDef, CardRules, CardSet,
    CardSupertype, CardType, CounterKind, CreatureStats, EffectDef, EffectRecipientDef, ManaColor,
    ObjectPredicateDef, ObjectRefDef, PlayerRelation, ReplacementConditionDef,
    ReplacementEffectDef, ResolvedEffectDurationDef, SpellResolutionDestinationDef,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::{TargetIndex, mana_cost};

// FUT 43 — Reality Strobe
pub(in crate::card::sets) static REALITY_STROBE: CardRecord = CardRecord::new(
    CardSet::FutureSight,
    "Reality Strobe",
    "8e6d881a-f7b1-471f-bc0b-64a79bb491c9",
    "Dan Murayama Scott",
    CardRules::new_sorcery(mana_cost!("{4}{U}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return target permanent to its owner's hand. Exile Reality Strobe with three time counters on it.",
            &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Any)],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        )
        .with_resolution_destination(SpellResolutionDestinationDef::ExileWithCounters(
            &[(CounterKind::named("time"), 3)],
        )),
        abilities::suspend("Suspend 3—{2}{U}", 3, &mana_cost!("{2}{U}")),
    ]),
);

// FUT 46 — Venser, Shaper Savant
pub(in crate::card::sets) static VENSER_SHAPER_SAVANT: CardRecord = CardRecord::new(
    CardSet::FutureSight,
    "Venser, Shaper Savant",
    "0e84fc99-4045-4518-b588-512a675f2933",
    "Aleksi Briclot",
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Human", "Wizard"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flash(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, return target spell or permanent to its owner's hand.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyOf(&[
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::Spell,
                            zones: &[ZoneKind::Stack],
                            controller: None,
                            owner: None,
                        },
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::Any,
                            zones: &[ZoneKind::Battlefield],
                            controller: None,
                            owner: None,
                        },
                    ]),
                )],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            ),
        ]),
);

// FUT 47 — Venser's Diffusion
pub(in crate::card::sets) static VENSERS_DIFFUSION: CardRecord = CardRecord::new(
    CardSet::FutureSight,
    "Venser's Diffusion",
    "fbedfc40-7c2f-4a6e-8157-219cafca3548",
    "Hideaki Takamura",
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target nonland permanent or suspended card to its owner's hand.",
        &[AbilityTargetDef::exactly_one(
            crate::card::AbilityTargetPredicate::AnyOf(&[
                crate::card::AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                crate::card::AbilityTargetPredicate::Object {
                    object: abilities::SUSPENDED_CARD,
                    zones: &[ZoneKind::Exile],
                    controller: None,
                    owner: None,
                },
            ]),
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
        },
    )),
);

// FUT 54 — Narcomoeba
pub(in crate::card::sets) static NARCOMOEBA: CardRecord = CardRecord::new(
    CardSet::FutureSight,
    "Narcomoeba",
    "f76b3746-2e2c-4560-a2d2-e7b5b92833b2",
    "Matt Stewart",
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
                effect: &const {
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Source,
                        zone: ZoneKind::Battlefield,
                        placement: ZonePlacement::Top,
                    }
                },
            },
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// FUT 76 — Shimian Specter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIMIAN_SPECTER: CardRecord = CardRecord::new(
    crate::card::CardSet::FutureSight,
    "Shimian Specter",
    "e6faa406-aa7a-49ce-a42e-00e98f3fb74e",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

static BRIDGE_FROM_BELOW_IS_IN_GRAVEYARD: TriggerConditionDef =
    TriggerConditionDef::SourceInZone(ZoneKind::Graveyard);

// FUT 81 — Bridge from Below
pub(in crate::card::sets) static BRIDGE_FROM_BELOW: CardRecord = CardRecord::new(
    CardSet::FutureSight,
    "Bridge from Below",
    "52c44610-6d4b-4c14-839f-2c085badec90",
    "Greg Hildebrandt & Tim Hildebrandt",
    CardRules::new_enchantment(mana_cost!("{B}{B}{B}")).with_abilities(&[
        AbilityDef::triggered_if(
            "Whenever a nontoken creature is put into your graveyard from the battlefield, if this card is in your graveyard, create a 2/2 black Zombie creature token.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                    ObjectPredicateDef::OwnedBy(PlayerRelation::You),
                ]),
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
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::OwnedBy(PlayerRelation::Opponent),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            &BRIDGE_FROM_BELOW_IS_IN_GRAVEYARD,
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// FUT 94 — Arc Blade
pub(in crate::card::sets) static ARC_BLADE: CardRecord = CardRecord::new(
    CardSet::FutureSight,
    "Arc Blade",
    "4d1c04fb-213f-4be1-9bba-94c737826bf8",
    "Shishizaru",
    CardRules::new_sorcery(mana_cost!("{3}{R}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Arc Blade deals 2 damage to any target. Exile Arc Blade with three time counters on it.",
            &[AbilityTargetDef::exactly_one(
                crate::card::AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        )
        .with_resolution_destination(SpellResolutionDestinationDef::ExileWithCounters(
            &[(CounterKind::named("time"), 3)],
        )),
        abilities::suspend("Suspend 3—{2}{R}", 3, &mana_cost!("{2}{R}")),
    ]),
);

// FUT 110 — Bloodshot Trainee
pub(in crate::card::sets) static BLOODSHOT_TRAINEE: CardRecord = CardRecord::new(
    crate::card::CardSet::FutureSight,
    "Bloodshot Trainee",
    "b930b146-d132-454f-b35d-4a247c14c054",
    "Lucio Parrillo",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Goblin", "Warrior"], 2, 3).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 4 damage to target creature. Activate only if this creature's power is 4 or greater.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
        )
        .with_activation_condition(&TriggerConditionDef::SourceMatches {
            object: ObjectPredicateDef::PowerAtLeast(4),
        }),
    ),
);

// FUT 137 — Rites of Flourishing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RITES_OF_FLOURISHING: CardRecord = CardRecord::new(
    crate::card::CardSet::FutureSight,
    "Rites of Flourishing",
    "811458c7-dcdc-43ef-8c3e-a90e21ce315e",
    "Brandon Kitkouski",
    crate::card::CardRules::unsupported(),
);

// FUT 138 — Sprout Swarm
pub(in crate::card::sets) static SPROUT_SWARM: CardRecord = CardRecord::new(
    CardSet::FutureSight,
    "Sprout Swarm",
    "0b915355-4e98-44df-81bd-961a3d3c86b8",
    "Chippy",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[
        abilities::convoke(),
        abilities::buyback(mana_cost!("{3}")),
        AbilityDef::spell(
            "Create a 1/1 green Saproling creature token.",
            EffectDef::create_creature_token(&["Saproling"], &[ManaColor::Green], 1, 1),
        ),
    ]),
);

// FUT 157 — Jhoira of the Ghitu
pub(in crate::card::sets) static JHOIRA_OF_THE_GHITU: CardRecord = CardRecord::new(
    CardSet::FutureSight,
    "Jhoira of the Ghitu",
    "1f437128-3a87-4958-97d0-3940d8761cba",
    "Kev Walker",
    CardRules::new_creature(mana_cost!("{1}{U}{R}"), &["Human", "Wizard"], 2, 2)
        .with_supertype(crate::card::CardSupertype::Legendary)
        .with_ability(AbilityDef::activated(
            "{2}, Exile a nonland card from your hand: Put four time counters on the exiled card. If it doesn't have suspend, it gains suspend.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::MoveToZone(
                    crate::card::MoveToZoneCostDef::new(
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        ZoneKind::Hand,
                        ZoneKind::Exile,
                        1,
                    )
                    .binding(Binding!("suspended_card")),
                ),
            ],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::binding_zone_change_successor(Binding!("suspended_card")),
                    kind: CounterKind::named("time"),
                    amount: ValueDef::Constant(4),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::binding_zone_change_successor(Binding!("suspended_card")),
                    effect: AppliedEffectDef::add_ability(&abilities::GRANTED_SUSPEND),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            ]),
        )),
);

// FUT 159 — Akroma's Memorial
pub(in crate::card::sets) static AKROMAS_MEMORIAL: CardRecord = CardRecord::new(
    CardSet::FutureSight,
    "Akroma's Memorial",
    "d2cb3bdf-dff5-4285-ad67-ae7ca84c9fbc",
    "Dan Murayama Scott",
    CardRules::new_artifact(mana_cost!("{7}"))
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::static_ability(
            "Creatures you control have flying, first strike, vigilance, trample, haste, and protection from black and from red.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_ability(&abilities::flying()),
                    AppliedEffectDef::add_ability(&abilities::first_strike()),
                    AppliedEffectDef::add_ability(&abilities::vigilance()),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                    AppliedEffectDef::add_ability(&abilities::haste()),
                    AppliedEffectDef::add_ability(&abilities::protection_from_color(ManaColor::Black)),
                    AppliedEffectDef::add_ability(&abilities::protection_from_color(ManaColor::Red)),
                ]),
            },
        )),
);

// FUT 161 — Coalition Relic
pub(in crate::card::sets) static COALITION_RELIC: CardRecord = CardRecord::new(
    CardSet::FutureSight,
    "Coalition Relic",
    "7a7c98b0-d64d-4d0a-b284-1187a8e7095e",
    "Donato Giancola",
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
            // The printed clause removes the counters and then adds the mana, but the
            // amount is read off the counters, so the two steps are written the other
            // way round. One resolution, no priority in between, and nothing else in
            // the pool watches a charge counter leave: what is observable is that the
            // counters are gone and that many mana arrived.
            EffectDef::Sequence(&[
                EffectDef::AddMana(
                    AddManaEffectDef::any_color()
                        .with_variable_amount(ValueDef::CountersOnSource(CounterKind::named("charge"))),
                ),
                EffectDef::RemoveAllCounters {
                    object: EffectRecipientDef::Source,
                    kind: Some(CounterKind::named("charge")),
                },
            ]),
        ),
    ]),
);

// FUT 162 — Epochrasite
pub(in crate::card::sets) static EPOCHRASITE: CardRecord = CardRecord::new(
    CardSet::FutureSight,
    "Epochrasite",
    "7971f6a6-c26c-4f8f-8de7-afc40563967d",
    "Michael Bruinsma",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Construct"], 1, 1).with_abilities(&[
        AbilityDef::as_enters_if(
            "This creature enters with three +1/+1 counters on it if you didn't cast it from your hand.",
            ReplacementConditionDef::SourceNotCastFrom(ZoneKind::Hand),
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 3,
                },
            ),
        ),
        AbilityDef::triggered(
            "When this creature dies, exile it with three time counters on it and it gains suspend.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::Sequence(&const {
                [
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::TriggeringZoneChangeResult,
                        zone: ZoneKind::Exile,
                        placement: ZonePlacement::Top,
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::TriggeringZoneChangeResultSuccessor,
                        kind: CounterKind::named("time"),
                        amount: ValueDef::Constant(3),
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::TriggeringZoneChangeResultSuccessor,
                        effect: AppliedEffectDef::add_ability(&abilities::GRANTED_SUSPEND),
                        duration: ResolvedEffectDurationDef::Permanent,
                    },
                ]
            }),
        ),
    ]),
);

// FUT 165 — Sword of the Meek
pub(in crate::card::sets) static SWORD_OF_THE_MEEK: CardRecord = CardRecord::new(
    CardSet::FutureSight,
    "Sword of the Meek",
    "e9f13705-6ede-4c29-a2b4-a082bf69e9c5",
    "Franz Vohwinkel",
    // On its own it is a bad Equipment. Beside anything that makes 1/1s for
    // free it is an engine that never runs out of Swords.
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
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
                // Read as the creature enters, so a 1/1 that is only a 1/1 because of what
                // is already on the battlefield still counts, and a 2/2 shrunk to 1/1 does
                // too.
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&const {
                        [
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::PowerExactly(1),
                            ObjectPredicateDef::ToughnessExactly(1),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]
                    }),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    // The attachment rides the return rather than following it: what comes back
                    // from the graveyard is a new object, so a later effect would have nothing
                    // left to name.
                    effect: &const {
                        EffectDef::WithBattlefieldArrival {
                            effect: &const {
                                EffectDef::MoveToZone {
                                    object: EffectRecipientDef::Source,
                                    zone: ZoneKind::Battlefield,
                                    placement: ZonePlacement::Top,
                                }
                            },
                            arrival: crate::card::BattlefieldArrivalDef {
                                attachment: Some(ArrivalAttachmentDef::ArrivalToHost(
                                    ObjectRefDef::TriggeringObject,
                                )),
                                ..crate::card::BattlefieldArrivalDef::DEFAULT
                            },
                        }
                    },
                },
            )
            .with_source_zones(&[ZoneKind::Graveyard]),
        ]),
);

// FUT 167 — Darksteel Garrison
pub(in crate::card::sets) static DARKSTEEL_GARRISON: CardRecord = CardRecord::new(
    CardSet::FutureSight,
    "Darksteel Garrison",
    "e77eaaa0-40f9-40e4-b0ba-5a8addd764d3",
    "David Martin",
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
pub(in crate::card::sets) static DRYAD_ARBOR: CardRecord = CardRecord::new(
    CardSet::FutureSight,
    "Dryad Arbor",
    "8cee476d-42e1-4997-87af-73e18f542167",
    "Eric Fortune",
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
pub(in crate::card::sets) static HORIZON_CANOPY: CardRecord = CardRecord::new(
    CardSet::FutureSight,
    "Horizon Canopy",
    "d5dfc25d-a17b-4ead-9484-e8a18b8fa176",
    "Michael Komarck",
    // The original of the cycle Modern Horizons finished twelve years later,
    // and still the one the cube wants: a dual that costs life to use and a
    // card when there is nothing left to use it on.
    CardRules::new_land(&[]).with_abilities(&abilities::horizon_land(
        "{T}, Pay 1 life: Add {G} or {W}.",
        &[ManaColor::Green, ManaColor::White],
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &REALITY_STROBE,
    &VENSER_SHAPER_SAVANT,
    &VENSERS_DIFFUSION,
    &NARCOMOEBA,
    &SHIMIAN_SPECTER,
    &BRIDGE_FROM_BELOW,
    &ARC_BLADE,
    &BLOODSHOT_TRAINEE,
    &RITES_OF_FLOURISHING,
    &SPROUT_SWARM,
    &JHOIRA_OF_THE_GHITU,
    &AKROMAS_MEMORIAL,
    &COALITION_RELIC,
    &EPOCHRASITE,
    &SWORD_OF_THE_MEEK,
    &DARKSTEEL_GARRISON,
    &DRYAD_ARBOR,
    &HORIZON_CANOPY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
