// Saga chapters: the trigger shape that reads one, and the constructors a
// card names them with.
//
// Included textually into `abilities.rs`, so the imports here are that
// module's. What makes a chapter a chapter is this shape -- the rules read
// the number back off it to know when a Saga has been read through.

/// The event one chapter watches: a lore counter arriving that brings the
/// count to this chapter's number.
const fn saga_chapter_event(chapter: u8) -> TriggerEventDef {
    TriggerEventDef::CountersCross {
        object: ObjectPredicateDef::Source,
        kind: CounterKind::Lore,
        thresholds: crate::card::CounterThresholdsDef::One(chapter),
    }
}

const fn saga_chapters_event(chapters: &'static [u8]) -> TriggerEventDef {
    TriggerEventDef::CountersCross {
        object: ObjectPredicateDef::Source,
        kind: CounterKind::Lore,
        thresholds: crate::card::CounterThresholdsDef::Several(chapters),
    }
}

/// One printed ability that triggers separately for every indicated chapter.
#[must_use]
pub const fn saga_chapters_with_targets(
    chapters: &'static [u8],
    text: &'static str,
    targets: &'static [AbilityTargetDef],
    effect: EffectDef,
) -> AbilityDef {
    AbilityDef::triggered_with_targets(text, saga_chapters_event(chapters), targets, effect)
}

/// One printed clause shared by several chapter abilities (CR 714.2c).
#[must_use]
pub const fn saga_chapters(
    chapters: &'static [u8],
    text: &'static str,
    effect: EffectDef,
) -> AbilityDef {
    AbilityDef::triggered(text, saga_chapters_event(chapters), effect)
}

/// One chapter of a Saga (CR 714.2c): the ability that triggers when the
/// lore counter placed makes the count reach `chapter`.
///
/// Crossing the threshold is part of the event, not an intervening if:
/// proliferating in response does not undo an already-triggered chapter.
#[must_use]
pub const fn saga_chapter(chapter: u8, text: &'static str, effect: EffectDef) -> AbilityDef {
    AbilityDef::triggered(text, saga_chapter_event(chapter), effect)
}

/// The same, for a chapter that names a target.
#[must_use]
pub const fn saga_chapter_with_targets(
    chapter: u8,
    text: &'static str,
    targets: &'static [AbilityTargetDef],
    effect: EffectDef,
) -> AbilityDef {
    AbilityDef::triggered_with_targets(text, saga_chapter_event(chapter), targets, effect)
}
