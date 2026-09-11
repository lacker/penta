//! Guilds of Ravnica cards used as cross-format rules-engine test cases.

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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAUSOLEUM_SECRETS_75: CardRecord = CardRecord::new(
    "Mausoleum Secrets",
    "26f7cf38-78cc-4139-9f2a-4dd0be7d9da8",
    "Adam Paquette",
    crate::card::CardRules::unsupported(),
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXPERIMENTAL_FRENZY_99: CardRecord = CardRecord::new(
    "Experimental Frenzy",
    "4b8f32e2-5dc8-4f1b-8a69-d3ae06378ed8",
    "Simon Dominic",
    crate::card::CardRules::unsupported(),
);

// GRN 103 — Goblin Cratermaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_CRATERMAKER_103: CardRecord = CardRecord::new(
    "Goblin Cratermaker",
    "86ecaedc-08f1-4de7-aae8-056df57940e0",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

// GRN 109 — Legion Warboss
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEGION_WARBOSS_109: CardRecord = CardRecord::new(
    "Legion Warboss",
    "5e84cb9c-9876-47a4-aea4-78574321bc36",
    "Alex Konstad",
    crate::card::CardRules::unsupported(),
);

// GRN 115 — Runaway Steam-Kin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUNAWAY_STEAM_KIN_115: CardRecord = CardRecord::new(
    "Runaway Steam-Kin",
    "d8c9c111-fbc7-44e1-94bd-1ca164370623",
    "Jason Felix",
    crate::card::CardRules::unsupported(),
);

// GRN 119 — Torch Courier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TORCH_COURIER_119: CardRecord = CardRecord::new(
    "Torch Courier",
    "d4c9fc8c-e68f-4636-84b8-877f6ec04b09",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MNEMONIC_BETRAYAL_189: CardRecord = CardRecord::new(
    "Mnemonic Betrayal",
    "a5cf45aa-ed34-4add-a2ec-fc11f8c15ffa",
    "Clint Cearley",
    crate::card::CardRules::unsupported(),
);

// GRN 192 — Niv-Mizzet, Parun
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIV_MIZZET_PARUN_192: CardRecord = CardRecord::new(
    "Niv-Mizzet, Parun",
    "6f3d2dc5-7b9d-4af6-9f3b-4de90fbf63c9",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INVERT_INVENT_228: CardRecord = CardRecord::new(
    "Invert // Invent",
    "054a4e4f-8baa-41cf-b24c-d068e8b9a070",
    "Mathias Kollros",
    crate::card::CardRules::unsupported(),
);

// GRN 242 — Wand of Vertebrae
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAND_OF_VERTEBRAE_242: CardRecord = CardRecord::new(
    "Wand of Vertebrae",
    "87f208bc-e4dc-4d3a-8906-dccde3cc251b",
    "Volkan Baǵa",
    crate::card::CardRules::unsupported(),
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
