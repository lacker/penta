use crate::action::{AbilityOrigin, ManaColor};
use crate::card::{AbilityCostList, AddManaEffectDef, AppliedEffectDef, SpellForm};
use crate::ids::{CardDefinitionId, GameObjectId, PlayerId};

use super::{ManaPool, ManaSource};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AppliedStackEffect {
    pub(super) source: Option<ManaSource>,
    pub(super) effect: AppliedEffectDef,
}

/// The object or procedure a mana payment is paying for. Restrictions are
/// evaluated against this frozen purpose both while planning mana abilities
/// and when selecting the exact mana units to spend.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum ManaPaymentPurpose {
    Spell {
        object: GameObjectId,
        definition: CardDefinitionId,
        controller: PlayerId,
        form: SpellForm,
    },
    Ability {
        source: GameObjectId,
        /// Whether the ability taps its source to pay for itself. When it
        /// does, that source cannot also be tapped for mana, so it is barred
        /// from the payment rather than merely deprioritised.
        taps_source: bool,
        /// Whether the source must still be on the battlefield after mana is
        /// raised so it can be sacrificed or exiled for the main ability.
        /// Mana abilities of that same source which leave the battlefield are
        /// not legal ways to pay this cost.
        leaves_source: bool,
    },
    Other,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ManaAbilityActivation {
    pub(super) source: GameObjectId,
    pub(super) ability: AbilityOrigin,
    pub(super) color: ManaColor,
    pub(super) costs: AbilityCostList,
    pub(super) effect: AddManaEffectDef,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct PlannedManaActivation {
    pub(super) source: GameObjectId,
    pub(super) ability: AbilityOrigin,
    pub(super) color: ManaColor,
    pub(super) production: ManaPool,
    pub(super) benefits_payment: bool,
    pub(super) flexibility: usize,
    pub(super) order: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct FlexibleManaSource {
    pub(super) source: GameObjectId,
    pub(super) outputs: Vec<(AbilityOrigin, ManaColor, ManaPool, bool)>,
    pub(super) order: usize,
}
