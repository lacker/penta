//! What one payment is actually asked for, once the purpose behind it is
//! taken into account. A printed cost is the starting point rather than the
//! final word: a spell may demand a colour for its X, and a permission may
//! excuse the colours a creature's activation prints.

use super::super::{CardTypeSet, Game, ManaColor, ManaCost, ManaPaymentPurpose, fold_restricted_x};
use crate::card::AppliedRuleDef;

impl Game {
    /// The colour a spell's printed cost demands for X, if it prints such a
    /// restriction. Read from the compatibility primary-part view: no
    /// multi-part card prints one today.
    pub(in crate::game) fn x_spend_restriction(
        &self,
        purpose: &ManaPaymentPurpose,
    ) -> Option<ManaColor> {
        let ManaPaymentPurpose::Spell { definition, .. } = purpose else {
            return None;
        };
        self.catalog.get(*definition)?.rules.x_spend_restriction()
    }

    pub(in crate::game) fn restrict_x(
        &self,
        player: super::super::PlayerId,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
    ) -> (ManaCost, u16) {
        // The colour permission comes first so that a cost carrying both it
        // and "spend only black mana on X" still has to find black for the X
        // portion: the permission loosens what is printed, and the
        // restriction is then applied to the result.
        let _ = player;
        self.x_spend_restriction(purpose)
            .map_or((cost, x), |color| fold_restricted_x(cost, x, color))
    }

    pub(in crate::game) fn may_spend_any_type(purpose: &ManaPaymentPurpose) -> bool {
        matches!(
            purpose,
            ManaPaymentPurpose::Spell {
                spend_any_type: true,
                ..
            }
        )
    }

    /// Color-spending permissions change which units can satisfy a symbol,
    /// without changing a colored symbol into a generic cost.
    pub(in crate::game) fn may_spend_any_color(
        &self,
        player: super::super::PlayerId,
        purpose: &ManaPaymentPurpose,
    ) -> bool {
        if self.player_rule_applies(player, AppliedRuleDef::MaySpendManaAsAnyColor) {
            return true;
        }
        if matches!(
            purpose,
            ManaPaymentPurpose::Spell {
                spend_any_color: true,
                ..
            }
        ) {
            return true;
        }
        let ManaPaymentPurpose::Ability { source, .. } = purpose else {
            return false;
        };
        let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == *source)
        else {
            return false;
        };
        if !self
            .permanent_types(permanent)
            .is_some_and(CardTypeSet::is_creature)
        {
            return false;
        }
        self.player_rule_applies(
            permanent.controller,
            AppliedRuleDef::MaySpendManaAsAnyColorForCreatureAbilities,
        )
    }
}
