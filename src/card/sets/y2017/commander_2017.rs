//! Commander 2017 card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, CardArt, CardRules, CardSet, CardType, CopyExceptionsDef,
    EffectDef, EffectRecipientDef, ObjectPredicateDef, ObjectRefDef, PlayerRefDef, ZoneKind,
    ZonePlacement,
};
use crate::{TargetIndex, mana_cost};

// C17 37 — Fractured Identity
static A_NONLAND_PERMANENT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
)];

/// The copy is made before the exile rather than after it, which is the one
/// place this differs from the printed order. A target that has already left
/// the battlefield is no longer a legal target, so the copy has to be taken
/// while the permanent is still there; what it copies -- the permanent's
/// copiable values -- is the same either way, and no player receives
/// priority in between.
///
/// "Each player other than its controller" is one player here, and it is
/// read off the target rather than off the spell: a Fractured Identity
/// pointed at your own permanent hands the copy to your opponent.
static FRACTURED_IDENTITY_EFFECTS: [EffectDef; 2] = [
    EffectDef::create_token_from_copy(&crate::card::TokenCopyDef {
        object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
        exceptions: CopyExceptionsDef::NONE,
    })
    .with_controller(PlayerRefDef::OpponentOf(ObjectRefDef::Target(
        TargetIndex::PRIMARY,
    ))),
    EffectDef::MoveToZone {
        counters: None,
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        from: None,
        zone: ZoneKind::Exile,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        controller: None,
        tapped: false,
    },
];

pub(in crate::card::sets) static FRACTURED_IDENTITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b2f73f5d-1aad-48c2-9e74-5f7bdd87900f"),
    "Fractured Identity",
    CardArt::new("b2f73f5d-1aad-48c2-9e74-5f7bdd87900f", "Yongjae Choi"),
    CardSet::Commander2017,
    // Five mana that answers anything and keeps it: what leaves their board
    // arrives on yours, which is why the card is played over the cheaper
    // exile effects beside it.
    CardRules::new_sorcery(mana_cost!("{3}{W}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target nonland permanent. Each player other than its controller creates a token \
         that's a copy of it.",
        &A_NONLAND_PERMANENT,
        EffectDef::Sequence(&FRACTURED_IDENTITY_EFFECTS),
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&FRACTURED_IDENTITY];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
