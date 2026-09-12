//! Guilds of Ravnica cards used as cross-format rules-engine test cases.

use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseOneOfEachDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreatedTokensDef;
use crate::card::DrawEventMatcherDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealObjectsDef;
use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::CreateTokenDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::SpellCastQueryDef;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

static ARCLIGHT_PHOENIX_INSTANT_OR_SORCERY: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Instant),
    ObjectPredicateDef::HasType(CardType::Sorcery),
]);

static ARCLIGHT_PHOENIX_CAST_QUERY: SpellCastQueryDef = SpellCastQueryDef {
    player: PlayerRelation::You,
    spell: ARCLIGHT_PHOENIX_INSTANT_OR_SORCERY,
};

static ARCLIGHT_PHOENIX_CAST_COUNT: ValueComparisonDef = ValueComparisonDef {
    left: ValueDef::CountSpellsCastThisTurn(&ARCLIGHT_PHOENIX_CAST_QUERY),
    comparison: ComparisonDef::GreaterOrEqual,
    right: ValueDef::Constant(3),
};

static ARCLIGHT_PHOENIX_RETURN_CONDITION: TriggerConditionDef = TriggerConditionDef::All(&[
    TriggerConditionDef::SourceInZone(ZoneKind::Graveyard),
    TriggerConditionDef::ValueComparison(&ARCLIGHT_PHOENIX_CAST_COUNT),
]);

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "GRN",
    slug: "guilds-of-ravnica",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// GRN 14 — Healer's Hawk
pub(in crate::card::sets) static HEALER_S_HAWK: CardRecord = CardRecord::new(
    "Healer's Hawk",
    "3313bd5c-b657-47a3-822a-dd0d9165492a",
    "Milivoj Ćeran",
    CardRules::new_creature(mana_cost!("{W}"), &["Bird"], 1, 1)
        .with_abilities(&[abilities::flying(), abilities::lifelink()]),
);

// GRN 45 — Murmuring Mystic
pub(in crate::card::sets) static MURMURING_MYSTIC: CardRecord = CardRecord::new(
    "Murmuring Mystic",
    "5fc6adff-dcb3-456d-a8c2-0e77b784ff89",
    "Mark Winters",
// A 1/5 body that turns every cantrip into a blocker, so the deck that
    // was already casting spells stops needing creatures of its own.
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Human", "Wizard"], 1, 5).with_ability(
        AbilityDef::triggered(
            "Whenever you cast an instant or sorcery spell, create a 1/1 blue Bird Illusion creature token with flying.",
            // On the cast rather than the resolution, so a countered spell
            // has already paid for its Bird.
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Bird", "Illusion"], &[ManaColor::Blue], 1, 1)
                    .with_abilities(&[abilities::flying()]),
            ))),
        ),
    ),
);

// GRN 64 — Burglar Rat
pub(in crate::card::sets) static BURGLAR_RAT: CardRecord = CardRecord::new(
    "Burglar Rat",
    "e9f7f218-fd2a-4233-8753-065ddc314f2d",
    "Tyler Walpole",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Rat"], 1, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, each opponent discards a card.",
            EffectDef::Discard {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ]),
);

// GRN 75 — Mausoleum Secrets
pub(in crate::card::sets) static MAUSOLEUM_SECRETS_75: CardRecord = CardRecord::new(
    "Mausoleum Secrets",
    "26f7cf38-78cc-4139-9f2a-4dd0be7d9da8",
    "Adam Paquette",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[
AbilityDef::spell("Undergrowth — Search your library for a black card with mana value less than or equal to the number of creature cards in your graveyard, reveal it, put it into your hand, then shuffle.", EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::All(&[ObjectPredicateDef::Color(ManaColor::Black), ObjectPredicateDef::ManaValueAtMostValue(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Graveyard], PlayerRelation::You)))]), minimum: 0, maximum: ValueDef::Constant(1), reveal: true, destination: ZoneKind::Hand, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None })
]),
);

// GRN 77 — Midnight Reaper
pub(in crate::card::sets) static MIDNIGHT_REAPER: CardRecord = CardRecord::new(
    "Midnight Reaper",
    "5e122fe0-51c5-404d-a7b9-3d161a426c35",
    "Sidharth Chaturvedi",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie", "Knight"], 3, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever a nontoken creature you control dies, this creature \
             deals 1 damage to you and you draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::Sequence(&[
                EffectDef::damage(EffectRecipientDef::Controller, ValueDef::Constant(1)),
                abilities::draw_cards(ValueDef::Constant(1)),
            ]),
        ),
    ]),
);

// GRN 91 — Arclight Phoenix
pub(in crate::card::sets) static ARCLIGHT_PHOENIX: CardRecord = CardRecord::new(
    "Arclight Phoenix",
    "787de9ce-02c5-4a17-a88b-d38e83dbeb0b",
    "Slawomir Maniak",
CardRules::new_creature(mana_cost!("{3}{R}"), &["Phoenix"], 3, 2).with_abilities(&[
        abilities::flying(),
        abilities::haste(),
        AbilityDef::triggered_if(
            "At the beginning of combat on your turn, if you've cast three or more instant and sorcery spells this turn, return this card from your graveyard to the battlefield.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &ARCLIGHT_PHOENIX_RETURN_CONDITION,
            EffectDef::move_to_zone(
                EffectRecipientDef::Source,
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// GRN 99 — Experimental Frenzy
// Audit: unsupported — PlayRestrictionDef has no source-zone selector, so it cannot prohibit plays from hand while preserving the separate permission to play the top card.
pub(in crate::card::sets) static EXPERIMENTAL_FRENZY_99: CardRecord = CardRecord::new(
    "Experimental Frenzy",
    "4b8f32e2-5dc8-4f1b-8a69-d3ae06378ed8",
    "Simon Dominic",
    crate::card::CardRules::unsupported(),
);

// GRN 103 — Goblin Cratermaker
pub(in crate::card::sets) static GOBLIN_CRATERMAKER_103: CardRecord = CardRecord::new(
    "Goblin Cratermaker",
    "86ecaedc-08f1-4de7-aae8-056df57940e0",
    "Svetlin Velinov",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Warrior"], 2, 2).with_abilities(&[
AbilityDef::modal_activated("{1}, Sacrifice this creature: Choose one —\n• This creature deals 2 damage to target creature.\n• Destroy target colorless nonland permanent.", &[CostDef::Mana(mana_cost!("{1}")), CostDef::SacrificeSource], &[AbilityDef::spell_with_targets("This creature deals 2 damage to target creature.", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))], EffectDef::damage(EffectRecipientDef::Target(TargetIndex::PRIMARY), ValueDef::Constant(2))), AbilityDef::spell_with_targets("Destroy target colorless nonland permanent.", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[ObjectPredicateDef::ColorCount(0), ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land))]))], EffectDef::destroy_target(TargetIndex::PRIMARY))], 1, 1, false)
])
);

// GRN 109 — Legion Warboss
pub(in crate::card::sets) static LEGION_WARBOSS_109: CardRecord = CardRecord::new(
    "Legion Warboss",
    "5e84cb9c-9876-47a4-aea4-78574321bc36",
    "Alex Konstad",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Soldier"], 2, 2).with_abilities(&[
AbilityDef::triggered_with_targets("Mentor (Whenever this creature attacks, put a +1/+1 counter on target attacking creature with lesser power.)", TriggerEventDef::attacks(ObjectPredicateDef::Source), &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Attacking, ObjectPredicateDef::PowerLessThan(ValueDef::SourcePower)]))], EffectDef::AddCounters { object: EffectRecipientDef::Target(TargetIndex::PRIMARY), kind: CounterKind::PlusOnePlusOne, amount: ValueDef::Constant(1) }),
AbilityDef::triggered("At the beginning of combat on your turn, create a 1/1 red Goblin creature token. That token gains haste until end of turn and attacks this combat if able.", TriggerEventDef::StepBegins { step: TurnStepDef::BeginningOfCombat, player: PlayerRelation::You }, EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1).with_created_tokens(CreatedTokensDef { binding: Binding!("warboss_goblin"), then: &EffectDef::Sequence(&[EffectDef::Apply { recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("warboss_goblin"))), effect: AppliedEffectDef::add_ability(&abilities::haste()), duration: ResolvedEffectDurationDef::UntilEndOfTurn }, EffectDef::Apply { recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("warboss_goblin"))), effect: AppliedEffectDef::add_ability(&abilities::attacks_each_combat_if_able()), duration: ResolvedEffectDurationDef::UntilEndOfCombat }]) }))
]),
);

// GRN 115 — Runaway Steam-Kin
pub(in crate::card::sets) static RUNAWAY_STEAM_KIN_115: CardRecord = CardRecord::new(
    "Runaway Steam-Kin",
    "d8c9c111-fbc7-44e1-94bd-1ca164370623",
    "Jason Felix",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Elemental"], 1, 1).with_abilities(&[
AbilityDef::triggered_if("Whenever you cast a red spell, if this creature has fewer than three +1/+1 counters on it, put a +1/+1 counter on this creature.", TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[ObjectPredicateDef::Color(ManaColor::Red), ObjectPredicateDef::ControlledBy(PlayerRelation::You)])), &TriggerConditionDef::SourceCounters { kind: CounterKind::PlusOnePlusOne, comparison: ComparisonDef::Less, amount: 3 }, EffectDef::AddCounters { object: EffectRecipientDef::Source, kind: CounterKind::PlusOnePlusOne, amount: ValueDef::Constant(1) }),
AbilityDef::activated_mana("Remove three +1/+1 counters from this creature: Add {R}{R}{R}.", &[CostDef::RemoveCountersFromSource { kind: CounterKind::PlusOnePlusOne, amount: 3 }], EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red).with_amount(3)))
]),
);

// GRN 119 — Torch Courier
pub(in crate::card::sets) static TORCH_COURIER_119: CardRecord = CardRecord::new(
    "Torch Courier",
    "d4c9fc8c-e68f-4636-84b8-877f6ec04b09",
    "Mark Zug",
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1).with_abilities(&[
        abilities::haste(),
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Another target creature gains haste until end of turn.",
            &[CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// GRN 121 — Affectionate Indrik
pub(in crate::card::sets) static AFFECTIONATE_INDRIK: CardRecord = CardRecord::new(
    "Affectionate Indrik",
    "b4c8ddc1-d95c-499f-b1d1-f608f8f07b02",
    "Steve Prescott",
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Beast"], 4, 4).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, you may have it fight target \
             creature you don't control. (Each deals damage equal to its \
             power to the other.)",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Fight {
                    first: ObjectRefDef::Source,
                    second: ObjectRefDef::Target(TargetIndex::PRIMARY),
                    excess: None,
                },
            },
        ),
    ]),
);

// GRN 125 — Circuitous Route
pub(in crate::card::sets) static CIRCUITOUS_ROUTE: CardRecord = CardRecord::new(
    "Circuitous Route",
    "5a970429-6369-4422-a343-00b30267f09d",
    "Milivoj Ćeran",
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_abilities(&[AbilityDef::spell(
        "Search your library for up to two basic land cards and/or \
         Gate cards, put them onto the battlefield tapped, then \
         shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ]),
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Gate")),
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
    )]),
);

// GRN 152 — Assassin's Trophy
pub(in crate::card::sets) static ASSASSIN_S_TROPHY: CardRecord = CardRecord::new(
    "Assassin's Trophy",
    "906b6e99-128f-4c11-8daf-16099d35b0d4",
    "Seb McKinnon",
    CardRules::new_instant(mana_cost!("{B}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target permanent an opponent controls. Its controller \
         may search their library for a basic land card, put it onto \
         the battlefield, then shuffle.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            EffectDef::May {
                player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                    ObjectRefDef::Target(TargetIndex::PRIMARY),
                )),
                effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::Target(TargetIndex::PRIMARY),
                    )),
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            },
        ]),
    )]),
);

// GRN 189 — Mnemonic Betrayal
// Audit: unsupported — Arbitrary-card exile grants cannot attach a spend-mana-as-any-type permission to the exiled graveyard group. That modifier is exposed only on top-of-library exile operations, which cannot select these cards.
pub(in crate::card::sets) static MNEMONIC_BETRAYAL_189: CardRecord = CardRecord::new(
    "Mnemonic Betrayal",
    "a5cf45aa-ed34-4add-a2ec-fc11f8c15ffa",
    "Clint Cearley",
    crate::card::CardRules::unsupported(),
);

// GRN 192 — Niv-Mizzet, Parun
pub(in crate::card::sets) static NIV_MIZZET_PARUN_192: CardRecord = CardRecord::new(
    "Niv-Mizzet, Parun",
    "6f3d2dc5-7b9d-4af6-9f3b-4de90fbf63c9",
    "Svetlin Velinov",
    CardRules::new_creature(
        mana_cost!("{U}{U}{U}{R}{R}{R}"),
        &["Dragon", "Wizard"],
        5,
        5,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::cannot_be_countered(),
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "Whenever you draw a card, Niv-Mizzet deals 1 damage to any target.",
            TriggerEventDef::DrewCard(DrawEventMatcherDef::any(PlayerRelation::You)),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
        AbilityDef::triggered(
            "Whenever a player casts an instant or sorcery spell, you draw a card.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::HasType(CardType::Sorcery),
            ])),
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// GRN 203 — Swiftblade Vindicator
pub(in crate::card::sets) static SWIFTBLADE_VINDICATOR: CardRecord = CardRecord::new(
    "Swiftblade Vindicator",
    "285c4d9e-0f22-49a8-b68c-150fd0d4b617",
    "Viktor Titov",
    CardRules::new_creature(mana_cost!("{R}{W}"), &["Human", "Soldier"], 1, 1).with_abilities(&[
        abilities::double_strike(),
        abilities::vigilance(),
        abilities::trample(),
    ]),
);

// GRN 207 — Thousand-Year Storm
// Audit: unsupported — Needs a cast-event snapshot counting prior instant and sorcery spells this turn; the current matching-spell count reads live history at resolution and can include spells cast after this trigger.
pub(in crate::card::sets) static THOUSAND_YEAR_STORM: CardRecord = CardRecord::new(
    "Thousand-Year Storm",
    "270a0863-7d07-43f0-925d-a8ce0383a1cb",
    "Dimitar Marinski",
    CardRules::unsupported(),
);

// GRN 228 — Invert // Invent
pub(in crate::card::sets) static INVERT_INVENT_228: CardRecord = CardRecord::new_split(
    "Invert // Invent",
    "054a4e4f-8baa-41cf-b24c-d068e8b9a070",
    "Mathias Kollros",
    &[("Invert", CardRules::new_instant(mana_cost!("{U/R}")).with_ability(AbilityDef::spell_with_targets("Switch the power and toughness of each of up to two target creatures until end of turn.", &[AbilityTargetDef::up_to(AbilityTargetPredicate::Object { object: ObjectPredicateDef::HasType(CardType::Creature), zones: &[ZoneKind::Battlefield], controller: None, owner: None }, 2)], EffectDef::Apply { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), effect: AppliedEffectDef::switch_power_toughness(), duration: ResolvedEffectDurationDef::UntilEndOfTurn }))), ("Invent", CardRules::new_instant(mana_cost!("{4}{U}{R}")).with_ability(AbilityDef::spell("Search your library for an instant card and/or a sorcery card, reveal them, put them into your hand, then shuffle.", EffectDef::Sequence(&[EffectDef::ChooseOneOfEach(ChooseOneOfEachDef { actor: PlayerRefDef::EffectController, input: ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::Any, &[ZoneKind::Library], PlayerRelation::You)), predicates: &[ObjectPredicateDef::HasType(CardType::Instant), ObjectPredicateDef::HasType(CardType::Sorcery)], chosen: Binding!("invent_found"), remainder: Binding!("invent_rest"), visibility: ChoiceVisibilityDef::Private, then: &EffectDef::Sequence(&[EffectDef::RevealObjects(RevealObjectsDef { input: ObjectSetDef::Binding(Binding!("invent_found")), then: &EffectDef::None }), EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("invent_found"))), ZoneKind::Hand, ZonePlacement::Top)]) }), EffectDef::ShuffleLibrary { player: EffectRecipientDef::Controller }]))))]);

// GRN 242 — Wand of Vertebrae
pub(in crate::card::sets) static WAND_OF_VERTEBRAE_242: CardRecord = CardRecord::new(
    "Wand of Vertebrae",
    "87f208bc-e4dc-4d3a-8906-dccde3cc251b",
    "Volkan Baǵa",
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
AbilityDef::activated("{T}: Mill a card.", &[CostDef::TapSource], EffectDef::Mill { player: EffectRecipientDef::Controller, amount: ValueDef::Constant(1) }),
AbilityDef::activated_with_targets("{2}, {T}, Exile this artifact: Shuffle up to five target cards from your graveyard into your library.", &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource, CostDef::ExileSource], &[AbilityTargetDef::up_to(AbilityTargetPredicate::Object { object: ObjectPredicateDef::Any, zones: &[ZoneKind::Graveyard], controller: None, owner: Some(PlayerRelation::You) }, 5)], EffectDef::Sequence(&[EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Library, ZonePlacement::Top), EffectDef::ShuffleLibrary { player: EffectRecipientDef::Controller }]))
]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &HEALER_S_HAWK,
    &MURMURING_MYSTIC,
    &BURGLAR_RAT,
    &MAUSOLEUM_SECRETS_75,
    &MIDNIGHT_REAPER,
    &ARCLIGHT_PHOENIX,
    &EXPERIMENTAL_FRENZY_99,
    &GOBLIN_CRATERMAKER_103,
    &LEGION_WARBOSS_109,
    &RUNAWAY_STEAM_KIN_115,
    &TORCH_COURIER_119,
    &AFFECTIONATE_INDRIK,
    &CIRCUITOUS_ROUTE,
    &ASSASSIN_S_TROPHY,
    &MNEMONIC_BETRAYAL_189,
    &NIV_MIZZET_PARUN_192,
    &SWIFTBLADE_VINDICATOR,
    &THOUSAND_YEAR_STORM,
    &INVERT_INVENT_228,
    &WAND_OF_VERTEBRAE_242,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
