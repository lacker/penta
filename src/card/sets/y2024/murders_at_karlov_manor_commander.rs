//! Murders at Karlov Manor Commander card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AppliedEffectDef;
use crate::card::BindObjectsDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::PutObjectsOntoBattlefieldFaceDownDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::ValueDef;
use crate::card::abilities;
use crate::mana_cost;

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "MKC",
    slug: "murders-at-karlov-manor-commander",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());





// MKC 326 — Trouble in Pairs
// Audit: unsupported — There is no replacement for an opponent beginning an extra turn; an ordinary extra-turn scheduling effect cannot skip that turn instead.
pub(in crate::card::sets) static TROUBLE_IN_PAIRS_326: CardRecord = CardRecord::new(
    "Trouble in Pairs",
    "0dd4d070-38cf-4517-8152-84c9fcf2c984",
    "Fay Dalton",
    crate::card::CardRules::unsupported(),
);

// MKC 358 — Ransom Note
// With one opponent, goad imposes only the attack requirement until your next turn.
pub(in crate::card::sets) static RANSOM_NOTE_358: CardRecord = CardRecord::new(
    "Ransom Note",
    "05f9437a-50c2-415f-afa9-39f64f3aa3da",
    "Gaboleps",
    CardRules::new_artifact(mana_cost!("{1}")).with_subtypes(&["Clue"]).with_abilities(&[
abilities::enters_trigger("When this artifact enters, surveil 1.", abilities::surveil(ValueDef::Constant(1))),
AbilityDef::modal_activated("{2}, Sacrifice this artifact: Choose one —\n• Cloak the top card of your library.\n• Goad target creature.\n• Draw a card.", &[CostDef::Mana(mana_cost!("{2}")), CostDef::SacrificeSource], &[AbilityDef::spell("Cloak the top card of your library.", EffectDef::BindObjects(BindObjectsDef { source: ObjectCollectionSourceDef::TopCards { player: PlayerRefDef::EffectController, count: ValueDef::Constant(1) }, binding: Binding!("ransom_top"), then: &EffectDef::PutObjectsOntoBattlefieldFaceDown(PutObjectsOntoBattlefieldFaceDownDef { input: ObjectSetDef::Binding(Binding!("ransom_top")), controller: PlayerRefDef::EffectController, characteristics: crate::card::face_down::cloak(), turn_up_for_mana_cost: true, moved: None, then: &EffectDef::None }) })), AbilityDef::spell_with_targets("Goad target creature.", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))], EffectDef::Apply { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), effect: AppliedEffectDef::add_ability(&abilities::attacks_each_combat_if_able()), duration: ResolvedEffectDurationDef::UntilYourNextTurn }), AbilityDef::spell("Draw a card.", abilities::draw_cards(ValueDef::Constant(1)))], 1, 1, false)
]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &TROUBLE_IN_PAIRS_326,
    &RANSOM_NOTE_358,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
