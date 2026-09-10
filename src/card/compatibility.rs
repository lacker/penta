//! Historical card identities retained independently of current declarations.
//!
//! New definitions derive their IDs from exact debut-art Scryfall UUIDs. The
//! generated lookup below preserves IDs that predate that rule without making
//! compatibility metadata part of each [`super::record::CardRecord`].

include!(concat!(
    env!("OUT_DIR"),
    "/card_definition_compatibility.rs"
));

pub(super) fn historical_definition_id(scryfall_id: &str) -> Option<crate::CardDefinitionId> {
    let index = HISTORICAL_DEFINITION_IDS
        .binary_search_by(|(candidate, _)| candidate.as_bytes().cmp(scryfall_id.as_bytes()))
        .ok()?;
    Some(crate::CardDefinitionId::new(
        HISTORICAL_DEFINITION_IDS[index].1,
    ))
}
