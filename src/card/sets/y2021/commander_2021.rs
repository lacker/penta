//! Commander 2021 cards cataloged for the Vintage Cube pool.

use crate::card::AppliedEffectDef;
use crate::card::BasicLandType;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::CreatedTokensDef;
use crate::card::InstalledTriggerDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SpellCastQueryDef;
use crate::card::SumValueDef;
use crate::card::TokenCopyDef;
use crate::card::TurnStepDef;
use crate::card::ZonePlacement;
use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ExilePlayDurationDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::ScaledValueDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "C21",
    slug: "commander-2021",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const PEST_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Pest"], &[ManaColor::Black, ManaColor::Green], 1, 1)
        .with_abilities(&[abilities::dies_trigger(
            "When this token dies, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )])
        .with_art(CardArt::new(
            "d0ddbe3e-4a66-494d-9304-7471232549bf",
            "Ilse Gort",
        ));

// C21 44 — Stinging Study
pub(in crate::card::sets) static STINGING_STUDY_44: CardRecord = CardRecord::new(
    "Stinging Study",
    "b8840226-1693-44bc-a067-e50198c5e17e",
    "Kieran Yanner",
    CardRules::new_instant(mana_cost!("{4}{B}")).with_abilities(&[
AbilityDef::spell("You draw X cards and you lose X life, where X is the mana value of a commander you own on the battlefield or in the command zone.", EffectDef::Choose(ChooseDef { chooser: PlayerRefDef::EffectController, candidates: ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::Commander, ObjectPredicateDef::OwnedBy(PlayerRelation::You)]), &[ZoneKind::Battlefield, ZoneKind::Command], PlayerRelation::Any)), exclude: None, minimum: 1, maximum: 1, binding: ObjectChoiceBindingDef::Objects(Binding!("study_commander")), unchosen: None, visibility: ChoiceVisibilityDef::Private, then: &EffectDef::ForEachInBinding { objects: Binding!("study_commander"), binding: Binding!("studied_commander"), effect: &EffectDef::Sequence(&[EffectDef::DrawCards { recipient: EffectRecipientDef::Controller, amount: ValueDef::ObjectManaValue(ObjectRefDef::Binding(Binding!("studied_commander"))) }, EffectDef::LoseLife { recipient: EffectRecipientDef::Controller, amount: ValueDef::ObjectManaValue(ObjectRefDef::Binding(Binding!("studied_commander"))) }]) } }))
]),
);

// C21 53 — Laelia, the Blade Reforged
pub(in crate::card::sets) static LAELIA_THE_BLADE_REFORGED: CardRecord =
    CardRecord::new(
    "Laelia, the Blade Reforged",
    "a3bb2881-e8fb-4fba-a9f9-d93e6ca24378",
    "Wisnu Tan",
// Three mana with haste that attacks as a 3/3 on the turn it lands, and
        // grows every attack after because her own trigger feeds the other one.
        CardRules::new_creature(mana_cost!("{2}{R}"), &["Spirit", "Warrior"], 2, 2)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                abilities::haste(),
                AbilityDef::triggered(
                    "Whenever Laelia attacks, exile the top card of your library. You may play that card this \
                     turn.",
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                    EffectDef::ExileTopOfLibraryToPlay {
                        player: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        // "You may play that card this turn", which is not the same as
                        // playing it for nothing: Laelia still pays for what she finds.
                        free: false,
                        face_down: false,
                        duration: ExilePlayDurationDef::ThisTurn,
                        spend_any_color: false,
                        play_condition: None,
                        cast_only: false,
                    },
                ),
                // One counter for the move rather than one per card, which is what "one
                // or more" means: her own attack trigger gives one, and a Breach exiling
                // three still gives one.
                AbilityDef::triggered(
                    "Whenever one or more cards are put into exile from your library and/or your graveyard, \
                     put a +1/+1 counter on Laelia.",
                    TriggerEventDef::CardsExiled {
                        // "From your library and/or your graveyard": one clause naming two zones,
                        // so either answers it and a move that takes cards from both is still one
                        // trigger.
                        zones: &[ZoneKind::Library, ZoneKind::Graveyard],
                        owner: PlayerRelation::You,
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                ),
            ]),
);

// C21 65 — Pest Infestation
pub(in crate::card::sets) static PEST_INFESTATION: CardRecord = CardRecord::new(
    "Pest Infestation",
    "4720b4f2-e6af-4223-9250-a0ed21ed5693",
    "Brian Valeza",
    // Two mana per artifact answered, and the two Pests that come with each
    // are what makes paying it twice over worth doing.
    CardRules::new_sorcery(mana_cost!("{X}{X}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy up to X target artifacts and/or enchantments. Create twice X 1/1 black and \
         green Pest creature tokens with \"When this token dies, you gain 1 life.\"",
        // "Up to X", so a board with nothing worth destroying is no reason not to
        // cast it: the Pests come either way.
        &[AbilityTargetDef::up_to_chosen_x(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(PEST_TOKEN))
                    // Twice X, and X is paid twice over in the cost, so every Pest costs a
                    // mana and every artifact destroyed comes with two of them.
                    .with_count(ValueDef::Scaled(&ScaledValueDef::new(ValueDef::ChosenX, 2))),
            ),
        ]),
    )),
);

// C21 338 — Angel of the Ruins
pub(in crate::card::sets) static ANGEL_OF_THE_RUINS_338: CardRecord = CardRecord::new(
    "Angel of the Ruins",
    "b2babb93-6b30-4446-bf51-b0303e2e9a27",
    "Viko Menezes",
    CardRules::new_creature(mana_cost!("{5}{W}{W}"), &["Angel"], 5, 7).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, exile up to two target artifacts and/or enchantments.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                    zones: &[ZoneKind::Battlefield], controller: None, owner: None,
                },
                2,
            )],
            EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Exile, crate::card::ZonePlacement::Top),
        ),
        abilities::typecycling!(
            "Plainscycling {2} ({2}, Discard this card: Search your library for a Plains card, reveal it, put it into your hand, then shuffle.)",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
        ),
    ])
.with_type(crate::card::CardType::Artifact),
);

// C21 377 — Cursed Mirror
// Audit: unsupported — CopyEntering has no duration. A resolving BecomeCopyOf until end of turn would occur after entry and would miss the copied creature's entry replacement and triggered abilities.
pub(in crate::card::sets) static CURSED_MIRROR_377: CardRecord = CardRecord::new(
    "Cursed Mirror",
    "0c3bbdd4-146c-40ea-99e8-6cef0d04e953",
    "David Gaillet",
    crate::card::CardRules::unsupported(),
);

// C21 382 — Rionya, Fire Dancer
pub(in crate::card::sets) static RIONYA_FIRE_DANCER_382: CardRecord = CardRecord::new(
    "Rionya, Fire Dancer",
    "536c2936-2fdc-4664-97fd-bd580fc2d90e",
    "Heonhwa",
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Human", "Wizard"], 3, 4).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::triggered_with_targets("At the beginning of combat on your turn, create X tokens that are copies of another target creature you control, where X is one plus the number of instant and sorcery spells you've cast this turn. They gain haste. Exile them at the beginning of the next end step.", TriggerEventDef::StepBegins { step: TurnStepDef::BeginningOfCombat, player: PlayerRelation::You }, &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), zones: &[ZoneKind::Battlefield], controller: Some(PlayerRelation::You), owner: None })], EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Copy(&TokenCopyDef { object: &EffectRecipientDef::Target(TargetIndex::PRIMARY), exceptions: CopyExceptionsDef::NONE })).with_count(ValueDef::Sum(&SumValueDef { left: ValueDef::Constant(1), right: ValueDef::CountSpellsCastThisTurn(&SpellCastQueryDef { spell: ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Instant), ObjectPredicateDef::HasType(CardType::Sorcery)]), player: PlayerRelation::You }) })).with_created_tokens(CreatedTokensDef { binding: Binding!("rionya_tokens"), then: &EffectDef::Sequence(&[EffectDef::Apply { recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("rionya_tokens"))), effect: AppliedEffectDef::add_ability(&abilities::haste()), duration: ResolvedEffectDurationDef::Permanent }, EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered("At the beginning of the next end step, exile those tokens.", TriggerEventDef::StepBegins { step: TurnStepDef::End, player: PlayerRelation::Any }, EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("rionya_tokens"))), ZoneKind::Exile, ZonePlacement::Top))))]) })))
]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[
    &STINGING_STUDY_44,
    &LAELIA_THE_BLADE_REFORGED,
    &PEST_INFESTATION,
    &ANGEL_OF_THE_RUINS_338,
    &CURSED_MIRROR_377,
    &RIONYA_FIRE_DANCER_382,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
