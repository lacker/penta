//! Innistrad card records used by the built-in ISD–RTR Standard deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityPolicyHint, AbilityTargetDef,
    AbilityTargetPredicate, AddManaEffectDef, AppliedEffectDef, BasicLandType, CardAbilityBinding,
    CardArt, CardBehavior, CardComposition, CardEffectStatus, CardPart, CardRules, CardSet,
    CardStructure, CardSupertype, CardType, ComparisonDef, CounterKind, DoubleFacedKind, EffectDef,
    EffectDurationDef, EffectExecutionDef, EffectRecipientDef, LibraryPlacement, ManaColor,
    ObjectPredicateDef, ObjectQueryDef, PlayOptionDef, PlayerRelation, SpellForm,
    TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, abilities, cards,
};
use crate::game::{CardAbilityResolver, CardRuntime, PileChoice, PileSplit, ResolvedAbility};
use crate::ids::{AbilityId, CardPartId, PlayOptionId, TargetIndex, TargetSlotId};
use crate::mana_cost;

// ISD 40 — Urgent Exorcism
pub(in crate::card::sets) static URGENT_EXORCISM: CardRecord = CardRecord::new(
    cards::URGENT_EXORCISM,
    "Urgent Exorcism",
    CardArt::new("516a437c-a2ee-43c6-876c-1a63a455c97c", "Svetlin Velinov"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target Spirit or enchantment.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Subtype("Spirit"),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
        },
    )]),
);

// ISD 53 — Dissipate
pub(in crate::card::sets) static DISSIPATE: CardRecord = CardRecord::new(
    cards::DISSIPATE,
    "Dissipate",
    CardArt::new("5d778082-bcdb-423a-b16f-57ac0d4dace7", "Tomasz Jedruszek"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_ability(
        AbilityDef::spell_with_targets(
            "Counter target spell. If that spell is countered this way, exile it instead of putting it into its owner's graveyard.",
            &[AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Any)],
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
            },
        ),
    ),
);

// ISD 78 — Snapcaster Mage
pub(in crate::card::sets) static SNAPCASTER_MAGE: CardRecord = CardRecord::new(
    cards::SNAPCASTER_MAGE,
    "Snapcaster Mage",
    CardArt::new("9e5b279e-4670-4a1e-87d0-3cab7e4f9e58", "Volkan Baǵa"),
    CardSet::Innistrad,
    CardRules::new_creature(
        mana_cost!("{1}{U}"),
        &["Human", "Wizard"],
        2,
        1,
    )
    .with_abilities(&[
        abilities::flash(),
        AbilityDef::triggered_with_targets("When this creature enters, target instant or sorcery card in your graveyard gains flashback until end of turn. The flashback cost is equal to its mana cost. (You may cast that card from your graveyard for its flashback cost. Then exile it.)", TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            }, &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )], EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::GrantAbility(
                    &abilities::flashback_for_card_mana_cost(),
                ),
                duration: EffectDurationDef::UntilEndOfTurn,
            }),
    ]),
);

// ISD 83 — Think Twice
pub(in crate::card::sets) static THINK_TWICE: CardRecord = CardRecord::new(
    cards::THINK_TWICE,
    "Think Twice",
    CardArt::new("53e44060-a9a2-4095-9f5b-f60297525315", "Anthony Francisco"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::flashback(mana_cost!("{2}{U}")),
    ]),
);

static LILIANA_ULTIMATE_RESOLVER: CardAbilityResolver = CardAbilityResolver::new(
    "innistrad/liliana-of-the-veil/ultimate",
    resolve_liliana_ultimate,
);

const LILIANA_ULTIMATE_ABILITY: AbilityDef = AbilityDef::activated_with_targets(
    "−6: Separate all permanents target player controls into two piles. That player sacrifices all permanents in the pile of their choice.",
    &[AbilityCostDef::Loyalty(-6)],
    &[AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Player(PlayerRelation::Any),
    )],
    EffectDef::None,
)
.with_effect_execution(EffectExecutionDef::CardOwned)
.with_coverage(AbilityCoverageDef::explained_complete(
    "Pile separation, pile choice, and the chosen-pile sacrifice are composed by Liliana's card-owned resolver from shared runtime primitives.",
));

static LILIANA_ABILITY_BINDINGS: [CardAbilityBinding; 1] = [CardAbilityBinding::new(
    CardPartId::PRIMARY,
    AbilityId(2),
    LILIANA_ULTIMATE_ABILITY,
    &LILIANA_ULTIMATE_RESOLVER,
)
.with_policy_hint(
    AbilityPolicyHint::TargetPlayerSacrificesOneOfTwoPermanentPiles {
        target: TargetSlotId(0),
    },
)];

fn resolve_liliana_ultimate(runtime: &mut CardRuntime<'_>, ability: &ResolvedAbility) {
    let Some(victim) = ability.target_player(TargetIndex::PRIMARY) else {
        return;
    };
    let permanents = runtime.controlled_permanents(victim);
    runtime.queue_permanent_partition(
        ability.controller(),
        ability.controller(),
        victim,
        &permanents,
        liliana_piles_separated,
    );
}

fn liliana_piles_separated(runtime: &mut CardRuntime<'_>, piles: PileSplit) {
    let victim = piles.subject();
    runtime.queue_pile_choice(
        victim,
        piles,
        "Choose a pile to sacrifice",
        "Sacrifice pile",
        liliana_pile_chosen,
    );
}

fn liliana_pile_chosen(runtime: &mut CardRuntime<'_>, choice: PileChoice) {
    let victim = choice.subject();
    let resolving_controller = choice.resolving_controller();
    let (chosen, _unchosen) = choice.into_parts();
    runtime.sacrifice_permanents_simultaneously(&chosen, victim, resolving_controller);
}

// ISD 105 — Liliana of the Veil
pub(in crate::card::sets) static LILIANA_OF_THE_VEIL: CardRecord = CardRecord::new(
    cards::LILIANA_OF_THE_VEIL,
    "Liliana of the Veil",
    CardArt::new("ac506c17-adc8-49c6-9d8d-43db7cb1ec9d", "Steve Argyle"),
    CardSet::Innistrad,
    CardRules::new_planeswalker(mana_cost!("{1}{B}{B}"), &["Liliana"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "+1: Each player discards a card.",
                &[AbilityCostDef::Loyalty(1)],
                EffectDef::DiscardCards {
                    recipient: EffectRecipientDef::EachPlayer,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::activated_with_targets(
                "−2: Target player sacrifices a creature.",
                &[AbilityCostDef::Loyalty(-2)],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::SacrificeOfChoice {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    then: None,
                    optional: false,
                },
            ),
            LILIANA_ULTIMATE_ABILITY,
        ]),
)
.with_ability_bindings(&LILIANA_ABILITY_BINDINGS);

// ISD 122 — Unburial Rites
pub(in crate::card::sets) static UNBURIAL_RITES: CardRecord = CardRecord::new(
    cards::UNBURIAL_RITES,
    "Unburial Rites",
    CardArt::new("2794c82b-e5ce-4369-894e-bf56c6402ae1", "Ryan Pancoast"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{4}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return target creature card from your graveyard to the battlefield.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Battlefield,
                controller: None,
                placement: LibraryPlacement::Top,
            },
        ),
        abilities::flashback(mana_cost!("{3}{W}")),
    ]),
);

/// Every creature anyone controls, which is what the reduction counts.
static EVERY_CREATURE: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::HasType(CardType::Creature),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::Any,
};

// ISD 130 — Blasphemous Act
pub(in crate::card::sets) static BLASPHEMOUS_ACT: CardRecord = CardRecord::new(
    cards::BLASPHEMOUS_ACT,
    "Blasphemous Act",
    CardArt::new("509ce648-fb76-486d-8b39-183e368b7cb7", "Daarken"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{8}{R}")).with_abilities(&[
        AbilityDef::static_ability(
            "This spell costs {1} less to cast for each creature on the battlefield.",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(&EVERY_CREATURE)),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        AbilityDef::spell(
            "Blasphemous Act deals 13 damage to each creature.",
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                amount: ValueDef::Constant(13),
            },
        ),
    ]),
);

// ISD 170 — Avacyn's Pilgrim
pub(in crate::card::sets) static AVACYNS_PILGRIM: CardRecord = CardRecord::new(
    cards::AVACYNS_PILGRIM,
    "Avacyn's Pilgrim",
    CardArt::new(
        "7eb39e97-53c2-4df0-9fb3-a3d6a24ec41f",
        "Jana Schirmer & Johannes Voss",
    ),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{G}"), &["Human", "Monk"], 1, 1)
        .with_abilities(&[abilities::tap_for(ManaColor::White)]),
);

static GARRUK_FRONT_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::triggered_if(
        "When Garruk has two or fewer loyalty counters on him, transform him.",
        TriggerEventDef::StateCondition,
        &GARRUK_LOW_LOYALTY,
        EffectDef::Transform {
            object: EffectRecipientDef::Source,
        },
    ),
    AbilityDef::activated_with_targets(
        "0: Garruk deals 3 damage to target creature. That creature deals damage equal to its power to him.",
        &[AbilityCostDef::Loyalty(0)],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        // The creature hits back with the power it had when the ability
        // resolved, which is why the loyalty it costs Garruk is read off
        // the target rather than printed.
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Source,
                amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
            },
        ]),
    ),
    AbilityDef::activated(
        "0: Create a 2/2 green Wolf creature token.",
        &[AbilityCostDef::Loyalty(0)],
        EffectDef::CreateToken {
            token: cards::WOLF_TOKEN_2_2_GREEN,
            count: ValueDef::Constant(1),
        },
    ),
];

const fn garruk_front_rules() -> CardRules {
    CardRules::new_planeswalker(mana_cost!("{3}{G}"), &["Garruk"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&GARRUK_FRONT_ABILITIES)
}
/// Two or fewer is at most two, checked as a state trigger so it turns the
/// moment the damage lands rather than waiting for anything.
static GARRUK_LOW_LOYALTY: TriggerConditionDef = TriggerConditionDef::SourceLoyalty {
    comparison: ComparisonDef::AtMost,
    amount: 2,
};

static GARRUK_TUTOR: EffectDef = EffectDef::SearchLibrary {
    player: EffectRecipientDef::Controller,
    object: ObjectPredicateDef::HasType(CardType::Creature),
    destination: ZoneKind::Hand,
};

static GARRUK_TRAMPLE: AbilityDef = abilities::trample();

static GARRUK_GRAVEYARD_CREATURES: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::HasType(CardType::Creature),
    zones: &[ZoneKind::Graveyard],
    controller: PlayerRelation::You,
};

static GARRUK_BACK_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::activated(
        "+1: Create a 1/1 black Wolf creature token with deathtouch.",
        &[AbilityCostDef::Loyalty(1)],
        EffectDef::CreateToken {
            token: cards::WOLF_TOKEN_1_1_BLACK,
            count: ValueDef::Constant(1),
        },
    ),
    AbilityDef::activated(
        "−1: Sacrifice a creature. If you do, search your library for a creature card, reveal it, put it into your hand, then shuffle.",
        &[AbilityCostDef::Loyalty(-1)],
        EffectDef::SacrificeOfChoice {
            player: EffectRecipientDef::Controller,
            object: ObjectPredicateDef::HasType(CardType::Creature),
            then: Some(&GARRUK_TUTOR),
            optional: false,
        },
    ),
    AbilityDef::activated(
        "−3: Creatures you control gain trample and get +X/+X until end of turn, where X is the number of creature cards in your graveyard.",
        &[AbilityCostDef::Loyalty(-3)],
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::You,
            },
            effect: GARRUK_PUMP,
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    ),
];

static GARRUK_PUMP_PARTS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::GrantAbility(&GARRUK_TRAMPLE),
    AppliedEffectDef::ModifyPowerToughness {
        power: ValueDef::CountMatchingObjects(&GARRUK_GRAVEYARD_CREATURES),
        toughness: ValueDef::CountMatchingObjects(&GARRUK_GRAVEYARD_CREATURES),
    },
];

static GARRUK_PUMP: AppliedEffectDef = AppliedEffectDef::Composite(&GARRUK_PUMP_PARTS);

fn garruk_composition() -> CardComposition {
    let front = garruk_front_rules();
    let back = CardRules::new_planeswalker_without_mana_cost(&["Garruk"])
        .with_supertype(CardSupertype::Legendary)
        .printed_colors(&[ManaColor::Black, ManaColor::Green])
        .with_abilities(&GARRUK_BACK_ABILITIES);
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Garruk Relentless", front),
            CardPart::new(CardPartId(1), "Garruk, the Veil-Cursed", back),
        ],
        structure: CardStructure::DoubleFaced {
            front: CardPartId::PRIMARY,
            back: CardPartId(1),
            kind: DoubleFacedKind::Transforming,
        },
        play_options: vec![PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Garruk Relentless",
            SpellForm::Part(CardPartId::PRIMARY),
            front
                .mana_cost()
                .expect("Garruk Relentless has a printed mana cost"),
            CardEffectStatus::MetadataOnly,
        )],
    }
}

// ISD 181 — Garruk Relentless
pub(in crate::card::sets) static GARRUK_RELENTLESS: CardRecord = CardRecord::new(
    cards::GARRUK_RELENTLESS,
    "Garruk Relentless",
    CardArt::new("b4160322-ff40-41a4-887a-73cd6b85ae45", "Eric Deschamps"),
    CardSet::Innistrad,
    garruk_front_rules(),
)
.with_composition(garruk_composition);

// ISD 196 — Mulch
pub(in crate::card::sets) static MULCH: CardRecord = CardRecord::new(
    cards::MULCH,
    "Mulch",
    CardArt::new("52a1dabd-82df-4814-9d64-bf7bf9c1018d", "Christopher Moeller"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_ability(
        AbilityDef::custom_full(
            "Reveal the top four cards of your library. Put all land cards revealed this way into your hand and the rest into your graveyard.",
            CardBehavior::Mulch,
            "Implemented by the named card-local special behavior.",
        ),
    ),
);

// ISD 238 — Clifftop Retreat
pub(in crate::card::sets) static CLIFFTOP_RETREAT: CardRecord = CardRecord::new(
    cards::CLIFFTOP_RETREAT,
    "Clifftop Retreat",
    CardArt::new("fd7e1bf9-bd6a-48e3-9331-178e5142c06a", "John Avon"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Mountain or a Plains.",
            &[BasicLandType::Mountain, BasicLandType::Plains],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::White,
            ])),
        ),
    ]),
);

// ISD 239 — Gavony Township
pub(in crate::card::sets) static GAVONY_TOWNSHIP: CardRecord = CardRecord::new(
    cards::GAVONY_TOWNSHIP,
    "Gavony Township",
    CardArt::new("b5f73443-2fe8-424f-8e71-fc7ce1f3a3eb", "Peter Mohrbacher"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{2}{G}{W}, {T}: Put a +1/+1 counter on each creature you control.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{G}{W}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::AddCounters {
                kind: CounterKind::PlusOnePlusOne,
                object: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::You,
                },
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ISD 240 — Ghost Quarter
pub(in crate::card::sets) static GHOST_QUARTER: CardRecord = CardRecord::new(
    cards::GHOST_QUARTER,
    "Ghost Quarter",
    CardArt::new("1c6456ed-0ffb-4d22-b252-5775076030ce", "Peter Mohrbacher"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets("{T}, Sacrifice this land: Destroy target land. Its controller may search their library for a basic land card, put it onto the battlefield, then shuffle.", &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource], &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Land),
        )], EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                },
                // The printed "may" adds nothing: a search of a hidden zone
                // never obliges the searcher to find, so declining is already
                // one of the choices. The controller is read after the
                // destruction from last-known information.
                EffectDef::SearchLibrary {
                    player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    destination: ZoneKind::Battlefield,
                },
            ])),
    ]),
);

// ISD 242 — Isolated Chapel
pub(in crate::card::sets) static ISOLATED_CHAPEL: CardRecord = CardRecord::new(
    cards::ISOLATED_CHAPEL,
    "Isolated Chapel",
    CardArt::new("b3c1a371-5ded-4a3a-bf96-503c4f1a665d", "Cliff Childs"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Plains or a Swamp.",
            &[BasicLandType::Plains, BasicLandType::Swamp],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {B}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Black,
            ])),
        ),
    ]),
);

// ISD 243 — Kessig Wolf Run
pub(in crate::card::sets) static KESSIG_WOLF_RUN: CardRecord = CardRecord::new(
    cards::KESSIG_WOLF_RUN,
    "Kessig Wolf Run",
    CardArt::new("4a8447fe-7368-470a-911a-1083ec6cc831", "Eytan Zana"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{X}{R}{G}, {T}: Target creature gets +X/+0 and gains trample until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{X}{R}{G}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::ChosenX,
                        toughness: ValueDef::Constant(0),
                    },
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::GrantAbility(&abilities::trample()),
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// ISD 244 — Moorland Haunt
pub(in crate::card::sets) static MOORLAND_HAUNT: CardRecord = CardRecord::new(
    cards::MOORLAND_HAUNT,
    "Moorland Haunt",
    CardArt::new("1d5569e3-278c-4cf3-860e-712010333fe6", "James Paick"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{W}{U}, {T}, Exile a creature card from your graveyard: Create a 1/1 white Spirit creature token with flying.",
            &[
                AbilityCostDef::Mana(mana_cost!("{W}{U}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::ExileCardFromGraveyard(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
            ],
            EffectDef::CreateToken {
                token: cards::SPIRIT_TOKEN_1_1_WHITE,
                count: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ISD 248 — Sulfur Falls
pub(in crate::card::sets) static SULFUR_FALLS: CardRecord = CardRecord::new(
    cards::SULFUR_FALLS,
    "Sulfur Falls",
    CardArt::new("4968b65d-50e5-4d7e-b78b-cdada1cbf7a7", "Cliff Childs"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control an Island or a Mountain.",
            &[BasicLandType::Island, BasicLandType::Mountain],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Red])),
        ),
    ]),
);

// ISD 249 — Woodland Cemetery
pub(in crate::card::sets) static WOODLAND_CEMETERY: CardRecord = CardRecord::new(
    cards::WOODLAND_CEMETERY,
    "Woodland Cemetery",
    CardArt::new("67139101-ec5e-434b-be3a-21338cc33840", "Lars Grant-West"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Swamp or a Forest.",
            &[BasicLandType::Swamp, BasicLandType::Forest],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
            ])),
        ),
    ]),
);
pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &URGENT_EXORCISM,
    &DISSIPATE,
    &SNAPCASTER_MAGE,
    &THINK_TWICE,
    &LILIANA_OF_THE_VEIL,
    &UNBURIAL_RITES,
    &BLASPHEMOUS_ACT,
    &AVACYNS_PILGRIM,
    &GARRUK_RELENTLESS,
    &MULCH,
    &CLIFFTOP_RETREAT,
    &GAVONY_TOWNSHIP,
    &GHOST_QUARTER,
    &ISOLATED_CHAPEL,
    &KESSIG_WOLF_RUN,
    &MOORLAND_HAUNT,
    &SULFUR_FALLS,
    &WOODLAND_CEMETERY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
