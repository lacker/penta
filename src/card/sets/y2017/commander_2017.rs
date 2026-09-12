//! Commander 2017 card records required by supported formats.

use crate::card::AbilityTargetPredicate;
use crate::card::BattlefieldEntryChoiceDestinationDef;
use crate::card::BattlefieldEntryScalarChoiceDef;
use crate::card::BindObjectsDef;
use crate::card::CardTypeSet;
use crate::card::ChoiceVisibilityDef;
use crate::card::CostAdjustmentDef;
use crate::card::CostAmountDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::LookAtObjectsDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectSetDef;
use crate::card::ReplacementChoiceDef;
use crate::card::ReplacementEffectDef;
use crate::card::RevealObjectsDef;
use crate::card::SpellCostConditionDef;
use crate::card::SpellCostModificationDef;
use crate::card::SubtypeDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::abilities;
use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CopyExceptionsDef;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerRuleDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SpellResolutionDestinationDef;
use crate::card::TokenDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "C17",
    slug: "commander-2017",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// C17 8 — Teferi's Protection
pub(in crate::card::sets) static TEFERIS_PROTECTION: CardRecord = CardRecord::new(
    "Teferi's Protection",
    "77f130c7-0138-4a1a-9f67-62d2c302dc48",
    "Chase Stone",
CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(
        AbilityDef::spell(
            "Until your next turn, your life total can't change and you gain protection from everything. All permanents you control phase out. (While they're phased out, they're treated as though they don't exist. They phase in before you untap during your untap step.)\nExile Teferi's Protection.",
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                            PlayerRuleDef::LifeTotalCannotChange,
                        )),
                        AppliedEffectDef::Rule(AppliedRuleDef::PlayerProtectionFrom(
                            ObjectPredicateDef::Any,
                        )),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilYourNextTurn,
                },
                EffectDef::PhaseOut {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                },
            ]),
        )
        .with_resolution_destination(SpellResolutionDestinationDef::Exile),
    ),
);

// C17 14 — Bloodline Necromancer
pub(in crate::card::sets) static BLOODLINE_NECROMANCER_14: CardRecord = CardRecord::new(
    "Bloodline Necromancer",
    "42bffd03-3821-4b0f-9535-2eb455154587",
    "Joe Slucher",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Vampire", "Wizard"], 3, 2).with_abilities(&[
abilities::lifelink(),
AbilityDef::triggered_with_targets("When this creature enters, you may return target Vampire or Wizard creature card from your graveyard to the battlefield.", TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)), &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vampire")), ObjectPredicateDef::Subtype(SubtypeDef::Literal("Wizard"))])]), zones: &[ZoneKind::Graveyard], controller: None, owner: Some(PlayerRelation::You) })], EffectDef::May { player: EffectRecipientDef::Controller, effect: &EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Battlefield, ZonePlacement::Top) })
]),
);

// C17 24 — Curse of Opulence
// Audit: unsupported — Attack-declaration predicates do not identify the player being attacked relative to the Aura’s enchanted player; triggering once per attacker would overproduce Gold.
pub(in crate::card::sets) static CURSE_OF_OPULENCE_24: CardRecord = CardRecord::new(
    "Curse of Opulence",
    "e23db9d3-d11f-4b2c-8349-687bc0e9d4c2",
    "Kieran Yanner",
    crate::card::CardRules::unsupported(),
);

// C17 37 — Fractured Identity
pub(in crate::card::sets) static FRACTURED_IDENTITY: CardRecord = CardRecord::new(
    "Fractured Identity",
    "b2f73f5d-1aad-48c2-9e74-5f7bdd87900f",
    "Yongjae Choi",
    // Five mana that answers anything and keeps it: what leaves their board
    // arrives on yours, which is why the card is played over the cheaper
    // exile effects beside it.
    CardRules::new_sorcery(mana_cost!("{3}{W}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target nonland permanent. Each player other than its controller creates a token \
         that's a copy of it.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        )],
        // The copy is made before the exile rather than after it, which is the one
        // place this differs from the printed order. A target that has already left
        // the battlefield is no longer a legal target, so the copy has to be taken
        // while the permanent is still there; what it copies -- the permanent's
        // copiable values -- is the same either way, and no player receives
        // priority in between.
        //
        // "Each player other than its controller" is one player here, and it is
        // read off the target rather than off the spell: a Fractured Identity
        // pointed at your own permanent hands the copy to your opponent.
        EffectDef::Sequence(&[
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Copy(&crate::card::TokenCopyDef {
                    object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    exceptions: CopyExceptionsDef::NONE,
                }))
                .with_controller(PlayerRefDef::OpponentOf(ObjectRefDef::Target(
                    TargetIndex::PRIMARY,
                ))),
            ),
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
        ]),
    )),
);

// C17 38 — Inalla, Archmage Ritualist
// Audit: unsupported — The shared battlefield-event listener does not register abilities of command-zone cards, so eminence cannot observe another Wizard entering while Inalla is in the command zone.
pub(in crate::card::sets) static INALLA_ARCHMAGE_RITUALIST_38: CardRecord = CardRecord::new(
    "Inalla, Archmage Ritualist",
    "7c6e803a-451c-4aa6-97a2-400077f32c47",
    "Yongjae Choi",
    crate::card::CardRules::unsupported(),
);

// C17 53 — Herald's Horn
pub(in crate::card::sets) static HERALD_S_HORN_53: CardRecord = CardRecord::new(
    "Herald's Horn",
    "07b06421-778a-4d23-862b-30fc5fa25928",
    "Jason Felix",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
AbilityDef::as_enters("As this artifact enters, choose a creature type.", ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(BattlefieldEntryScalarChoiceDef::CREATURE_TYPE))),
AbilityDef::static_ability("Creature spells you cast of the chosen type cost {1} less to cast.", EffectDef::ModifyCost(CostModificationDef::Spell(SpellCostModificationDef { spell: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::HasSourcesChosenScalar(BattlefieldEntryChoiceDestinationDef::CreatureType)]), caster: PlayerRelation::You, condition: SpellCostConditionDef::Always, adjustment: CostAdjustmentDef::Subtract(CostAmountDef::Generic(ValueDef::Constant(1))) }))),
AbilityDef::triggered("At the beginning of your upkeep, look at the top card of your library. If it's a creature card of the chosen type, you may reveal it and put it into your hand.", TriggerEventDef::StepBegins { step: TurnStepDef::Upkeep, player: PlayerRelation::You }, EffectDef::BindObjects(BindObjectsDef { source: ObjectCollectionSourceDef::TopCards { player: PlayerRefDef::EffectController, count: ValueDef::Constant(1) }, binding: Binding!("horn_top"), then: &EffectDef::Sequence(&[EffectDef::LookAtObjects(LookAtObjectsDef { actor: PlayerRefDef::EffectController, source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Binding(Binding!("horn_top"))), visibility: ChoiceVisibilityDef::Private, then: &EffectDef::None }), EffectDef::ForEachInBinding { objects: Binding!("horn_top"), binding: Binding!("horn_card"), effect: &EffectDef::IfCondition { condition: &TriggerConditionDef::BoundObjectMatches { binding: Binding!("horn_card"), object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::HasSourcesChosenScalar(BattlefieldEntryChoiceDestinationDef::CreatureType)]) }, then: &EffectDef::May { player: EffectRecipientDef::Controller, effect: &EffectDef::Sequence(&[EffectDef::RevealObjects(RevealObjectsDef { input: ObjectSetDef::One(ObjectRefDef::Binding(Binding!("horn_card"))), then: &EffectDef::None }), EffectDef::move_to_zone(EffectRecipientDef::object(ObjectRefDef::Binding(Binding!("horn_card"))), ZoneKind::Hand, ZonePlacement::Top)]) } } }]) }))
]),
);

// C17 54 — Mirror of the Forebears
pub(in crate::card::sets) static MIRROR_OF_THE_FOREBEARS_54: CardRecord = CardRecord::new(
    "Mirror of the Forebears",
    "82e96f29-ce98-4e2d-8035-da3994ab66db",
    "Kieran Yanner",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
AbilityDef::as_enters("As this permanent enters, choose a creature type.", ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(BattlefieldEntryScalarChoiceDef::CREATURE_TYPE))),
AbilityDef::activated_with_targets("{1}: Until end of turn, this artifact becomes a copy of target creature you control of the chosen type, except it's an artifact in addition to its other types.", &[CostDef::Mana(mana_cost!("{1}"))], &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::HasSourcesChosenScalar(BattlefieldEntryChoiceDestinationDef::CreatureType)]), zones: &[ZoneKind::Battlefield], controller: Some(PlayerRelation::You), owner: None })], EffectDef::BecomeCopyOf { object: EffectRecipientDef::Target(TargetIndex::PRIMARY), copier: None, exceptions: CopyExceptionsDef::NONE.with_added_types(CardTypeSet::single(CardType::Artifact)), duration: Some(ResolvedEffectDurationDef::UntilEndOfTurn) })
]),
);

// C17 55 — Ramos, Dragon Engine
// Audit: unsupported — Needs a value expression for the number of colors in the triggering cast spell's captured characteristics; AffectedColorCount reads a static effect recipient rather than the cast event.
pub(in crate::card::sets) static RAMOS_DRAGON_ENGINE: CardRecord = CardRecord::new(
    "Ramos, Dragon Engine",
    "2e747ef1-a1ad-4859-a70c-3f935f017310",
    "Joseph Meehan",
    CardRules::unsupported(),
);

// C17 56 — Path of Ancestry
// Audit: unsupported — Mana types cannot be derived from designated commanders’ color identities, and spend effects cannot compare the spell’s creature types with a commander.
pub(in crate::card::sets) static PATH_OF_ANCESTRY_56: CardRecord = CardRecord::new(
    "Path of Ancestry",
    "70e70720-f0b9-4ad7-9366-927d6798d31e",
    "Alayna Danner",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &TEFERIS_PROTECTION,
    &BLOODLINE_NECROMANCER_14,
    &CURSE_OF_OPULENCE_24,
    &FRACTURED_IDENTITY,
    &INALLA_ARCHMAGE_RITUALIST_38,
    &HERALD_S_HORN_53,
    &MIRROR_OF_THE_FOREBEARS_54,
    &RAMOS_DRAGON_ENGINE,
    &PATH_OF_ANCESTRY_56,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
