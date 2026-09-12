//! Commander 2016 card records required by the cEDH corpus.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::BattlefieldArrivalDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BindObjectsDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::ComparisonDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::DeckConstructionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::QuantifierDef;
use crate::card::RevealObjectsDef;
use crate::card::TokenCopyDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "C16",
    slug: "commander-2016",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// C16 8 — Faerie Artisans
pub(in crate::card::sets) static FAERIE_ARTISANS_8: CardRecord = CardRecord::new(
    "Faerie Artisans",
    "ff05c503-2536-48f5-a639-480614a2e5f8",
    "Tony Foti",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Faerie", "Artificer"], 2, 2).with_abilities(&[
abilities::flying(),
AbilityDef::triggered("Whenever a nontoken creature an opponent controls enters, create a token that's a copy of that creature except it's an artifact in addition to its other types. Then exile all other tokens created with this creature.", TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Not(&ObjectPredicateDef::Token), ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent)]), None, Some(ZoneKind::Battlefield)), EffectDef::BindObjects(BindObjectsDef { source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::TokensCreatedBy(ObjectRefDef::Source)), binding: Binding!("artisans_previous"), then: &EffectDef::Sequence(&[EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Copy(&TokenCopyDef { object: &EffectRecipientDef::TriggeringObject, exceptions: CopyExceptionsDef::NONE.with_added_types(CardTypeSet::single(CardType::Artifact)) }))), EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("artisans_previous"))), ZoneKind::Exile, ZonePlacement::Top)]) }))
]),
);

// C16 34 — Kraum, Ludevic's Opus
pub(in crate::card::sets) static KRAUM_LUDEVIC_S_OPUS_34: CardRecord = CardRecord::new(
    "Kraum, Ludevic's Opus",
    "557fcd17-6cb3-414a-b2b1-ea9ae32e5aec",
    "Aaron Miller",
    CardRules::new_creature(mana_cost!("{3}{U}{R}"), &["Zombie", "Horror"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::haste(),
            AbilityDef::triggered_if(
                "Whenever an opponent casts their second spell each turn, draw a card.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(
                    PlayerRelation::Opponent,
                )),
                &TriggerConditionDef::SpellsCastThisTurn {
                    quantifier: QuantifierDef::Any,
                    player: PlayerRelation::Opponent,
                    comparison: ComparisonDef::Equal,
                    amount: 2,
                },
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
            AbilityDef::deck_construction(
                "Partner (You can have two commanders if both have partner.)",
                DeckConstructionDef::Partner,
                "Both commanders are designated before the game.",
            ),
        ]),
);

// C16 43 — Silas Renn, Seeker Adept
pub(in crate::card::sets) static SILAS_RENN_SEEKER_ADEPT_43: CardRecord = CardRecord::new(
    "Silas Renn, Seeker Adept",
    "4e3fe912-1374-47c7-b73f-89ef55c479c1",
    "Joseph Meehan",
    CardRules::new_artifact_creature(mana_cost!("{1}{U}{B}"), &["Human"], 2, 2).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::deathtouch(),
AbilityDef::triggered_with_targets("Whenever Silas Renn deals combat damage to a player, choose target artifact card in your graveyard. You may cast that card this turn.", TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source), &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::HasType(CardType::Artifact), zones: &[ZoneKind::Graveyard], controller: None, owner: Some(PlayerRelation::You) })], EffectDef::PermitCastFromGraveyardThisTurn { object: EffectRecipientDef::Target(TargetIndex::PRIMARY) }),
AbilityDef::deck_construction("Partner", DeckConstructionDef::Partner, "Commander pairing is chosen before the game.")
]),
);

// C16 45 — Tana, the Bloodsower
pub(in crate::card::sets) static TANA_THE_BLOODSOWER_45: CardRecord = CardRecord::new(
    "Tana, the Bloodsower",
    "a3d8d64f-a403-42a7-881b-4f70e9fe15a2",
    "Magali Villeneuve",
    CardRules::new_creature(mana_cost!("{2}{R}{G}"), &["Elf", "Druid"], 2, 2).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::trample(),
AbilityDef::triggered("Whenever Tana deals combat damage to a player, create that many 1/1 green Saproling creature tokens.", TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source), EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::TokenCharacteristics::creature(&["Saproling"], &[ManaColor::Green], 1, 1))).with_count(ValueDef::TriggerEventAmount))),
AbilityDef::deck_construction("Partner (You can have two commanders if both have partner.)", DeckConstructionDef::Partner, "Both commanders are designated before the game.")
]),
);

// C16 46 — Thrasios, Triton Hero
pub(in crate::card::sets) static THRASIOS_TRITON_HERO_46: CardRecord = CardRecord::new(
    "Thrasios, Triton Hero",
    "21e27b91-c7f1-4709-aa0d-8b5d81b22a0a",
    "Josu Hernaiz",
    CardRules::new_creature(mana_cost!("{G}{U}"), &["Merfolk", "Wizard"], 1, 3).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::activated("{4}: Scry 1, then reveal the top card of your library. If it's a land card, put it onto the battlefield tapped. Otherwise, draw a card.", &[CostDef::Mana(mana_cost!("{4}"))], EffectDef::Sequence(&[abilities::scry(ValueDef::Constant(1)), EffectDef::BindObjects(BindObjectsDef { source: ObjectCollectionSourceDef::TopCards { player: PlayerRefDef::EffectController, count: ValueDef::Constant(1) }, binding: Binding!("thrasios_top"), then: &EffectDef::Sequence(&[EffectDef::RevealObjects(RevealObjectsDef { input: ObjectSetDef::Binding(Binding!("thrasios_top")), then: &EffectDef::None }), EffectDef::ForEachInBinding { objects: Binding!("thrasios_top"), binding: Binding!("thrasios_card"), effect: &EffectDef::IfElseCondition { condition: &TriggerConditionDef::BoundObjectMatches { binding: Binding!("thrasios_card"), object: ObjectPredicateDef::HasType(CardType::Land) }, then: &EffectDef::WithBattlefieldArrival { effect: &EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::One(ObjectRefDef::Binding(Binding!("thrasios_card")))), ZoneKind::Battlefield, ZonePlacement::Top), arrival: BattlefieldArrivalDef { controller: None, modifications: &[BattlefieldEntryModificationDef::Tapped], attachment: None, counters: None } }, otherwise: &abilities::draw_cards(ValueDef::Constant(1)) } }]) })])),
AbilityDef::deck_construction("Partner", DeckConstructionDef::Partner, "Commander pairing is chosen before the game.")
]),
);

// C16 48 — Tymna the Weaver
// Audit: unsupported — Turn history does not distinguish players dealt combat damage from players dealt noncombat damage; querying current attackers would lose earlier combat history.
pub(in crate::card::sets) static TYMNA_THE_WEAVER_48: CardRecord = CardRecord::new(
    "Tymna the Weaver",
    "bc7cbe9b-324e-42b8-94e2-36e91cb32163",
    "Winona Nelson",
    crate::card::CardRules::unsupported(),
);

// C16 53 — Conqueror's Flail
// Audit: unsupported — No value counts the distinct colors represented among controlled permanents for the Equipment’s continuous power/toughness bonus.
pub(in crate::card::sets) static CONQUEROR_S_FLAIL_53: CardRecord = CardRecord::new(
    "Conqueror's Flail",
    "1644f535-9281-4a88-8fac-916bc0021d1d",
    "Franz Vohwinkel",
    crate::card::CardRules::unsupported(),
);

// C16 56 — Ash Barrens
pub(in crate::card::sets) static ASH_BARRENS_56: CardRecord = CardRecord::new(
    "Ash Barrens",
    "9d7ac112-bb20-4ea4-b797-5d21f6b7c121",
    "Jonas De Ro",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        abilities::typecycling_with_costs(
            "Basic landcycling {1}",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::DiscardSource],
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Land),
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
            ]),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &FAERIE_ARTISANS_8,
    &KRAUM_LUDEVIC_S_OPUS_34,
    &SILAS_RENN_SEEKER_ADEPT_43,
    &TANA_THE_BLOODSOWER_45,
    &THRASIOS_TRITON_HERO_46,
    &TYMNA_THE_WEAVER_48,
    &CONQUEROR_S_FLAIL_53,
    &ASH_BARRENS_56,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
