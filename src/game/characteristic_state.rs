use crate::action::AbilityOrigin;
use crate::card::{AbilityDef, BasicLandType, CardTypeSet, KeywordAbility};
use crate::ids::{CardDefinitionId, MeldRecipeId};
use crate::{EmblemCharacteristics, ObjectCharacteristics, TokenCharacteristics};

use super::TriggerEventObject;

/// Where this object's copiable characteristics come from. This deliberately
/// does not follow physical backing: a copy can have characteristics with no
/// card, while a future meld result can be backed by two cards without being
/// the printed definition of either one.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub(super) enum CharacteristicSource {
    Card(CardDefinitionId),
    Token(TokenCharacteristics),
    Emblem(EmblemCharacteristics),
    Copy(CardDefinitionId),
    Ability(CardDefinitionId),
    Meld(MeldRecipeId),
}

/// One indefinite text-changing effect in layer 3. These effects belong to
/// the object, are applied in timestamp order, and are deliberately excluded
/// from its copiable values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct BasicLandTypeChange {
    pub(super) from: BasicLandType,
    pub(super) to: BasicLandType,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum LandTypeOperation {
    SetTo(&'static [BasicLandType]),
    Add(&'static [BasicLandType]),
    Remove(&'static [BasicLandType]),
}

/// An ability added as an exception while copying an object. Unlike an
/// ordinary granted ability, this becomes part of the resulting object's
/// copiable values and can therefore be copied again.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct CopiableAbility {
    pub(super) origin: AbilityOrigin,
    pub(super) definition: AbilityDef,
}

/// The compact copiable-value snapshot needed by the copy effects currently
/// supported by the engine. The catalog source supplies all ordinary printed
/// characteristics; copy-process exceptions are frozen beside it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct CopiableCharacteristics {
    pub(super) base: ObjectCharacteristics,
    pub(super) added_types: CardTypeSet,
    pub(super) added_abilities: Vec<CopiableAbility>,
    /// Whether the copying card's own printed subtypes stand beside the ones
    /// it copied, which is what "except it's an Illusion in addition to its
    /// other types" says.
    pub(super) retain_printed_subtypes: bool,
}

/// The permanent, two-face copiable values of a double-faced token created as
/// a copy. The physical face identifiers stay separate from either face's
/// effective base because an intervening copy effect can make both faces copy
/// the same single-faced object without making the token single-faced.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct DoubleFacedCopiableCharacteristics {
    pub(super) kind: crate::card::DoubleFacedKind,
    pub(super) front_part: crate::CardPartId,
    pub(super) back_part: crate::CardPartId,
    pub(super) front: CopiableCharacteristics,
    pub(super) back: CopiableCharacteristics,
}

impl DoubleFacedCopiableCharacteristics {
    pub(super) fn face(&self, presented: crate::CardPartId) -> Option<&CopiableCharacteristics> {
        if presented == self.front_part {
            Some(&self.front)
        } else if presented == self.back_part {
            Some(&self.back)
        } else {
            None
        }
    }

    pub(super) const fn other_face(
        &self,
        presented: crate::CardPartId,
    ) -> Option<crate::CardPartId> {
        if presented.0 == self.front_part.0 {
            Some(self.back_part)
        } else if presented.0 == self.back_part.0 {
            Some(self.front_part)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct EffectiveAbility {
    pub(super) origin: AbilityOrigin,
    pub(super) ability: AbilityDef,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PermanentLastKnownInformation {
    pub(super) power: Option<i16>,
    pub(super) toughness: Option<i16>,
    pub(super) mana_value: u16,
    pub(super) keywords: Vec<KeywordAbility>,
}

/// Characteristics and abilities frozen immediately before a permanent exits
/// the battlefield. Every member of a simultaneous exit batch is snapshotted
/// before any member is removed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct BattlefieldExitSnapshot {
    pub(super) object: TriggerEventObject,
    pub(super) abilities: Vec<EffectiveAbility>,
    pub(super) last_known: PermanentLastKnownInformation,
}
