//! Dragons of Tarkir cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingRecord};
use crate::TargetIndex;
use crate::card::{
    AbilityCoverageDef, AbilityDef, AbilityTargetDef, AppliedEffectDef, CardArt, CardRules,
    CardSet, CardType, EffectDef, EffectRecipientDef, ObjectPredicateDef,
    ResolvedEffectDurationDef, SpellResolutionDestinationDef, ValueDef,
};
use crate::mana_cost;

// DTK 4 — Artful Maneuver
// Audit: partial — Rebound's self-exile is executable, but its next-upkeep free cast from exile needs the shared exile-casting lifecycle.
pub(in crate::card::sets) static ARTFUL_MANEUVER: CardRecord = CardRecord::new_with_legacy_id(
    1710,
    "Artful Maneuver",
    CardArt::new("7fcaf67e-ba97-4af9-8c47-dbca703cba35", "Lars Grant-West"),
    CardSet::DragonsOfTarkir,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(
        AbilityDef::spell_with_targets(
            "Target creature gets +2/+2 until end of turn.\n\nRebound (If you cast this spell from your hand, exile it as it resolves. At the beginning of your next upkeep, you may cast this card from exile without paying its mana cost.)",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .with_resolution_destination(SpellResolutionDestinationDef::Exile)
        .with_coverage(AbilityCoverageDef::partial(
            "Rebound's next-upkeep free cast from exile needs the shared exile-casting lifecycle.",
        )),
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&ARTFUL_MANEUVER];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
