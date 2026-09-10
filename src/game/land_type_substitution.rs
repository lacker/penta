//! "Each land of the first chosen type becomes the second."
//!
//! Both types are chosen as the spell resolves, so neither the lands it
//! affects nor the type it gives them can be written down in advance. That
//! is why this cannot go through the ordinary applied-effect path: the
//! recipient and the operation are both answers rather than declarations.

use super::{
    AppliedEffectDef, BasicLandType, CharacteristicOperationDef, DecisionContinuation,
    EffectResolutionContext, Game, ResolvedEffectDurationDef, ScopedEffect, SetOperationDef,
    StackObject, Target,
};

/// One slice per basic land type, so a chosen type can name the result of a
/// layer-4 set operation, which takes a borrowed list.
const AS_PLAINS: &[BasicLandType] = &[BasicLandType::Plains];
const AS_ISLAND: &[BasicLandType] = &[BasicLandType::Island];
const AS_SWAMP: &[BasicLandType] = &[BasicLandType::Swamp];
const AS_MOUNTAIN: &[BasicLandType] = &[BasicLandType::Mountain];
const AS_FOREST: &[BasicLandType] = &[BasicLandType::Forest];

const fn as_only_type(land_type: BasicLandType) -> &'static [BasicLandType] {
    match land_type {
        BasicLandType::Plains => AS_PLAINS,
        BasicLandType::Island => AS_ISLAND,
        BasicLandType::Swamp => AS_SWAMP,
        BasicLandType::Mountain => AS_MOUNTAIN,
        BasicLandType::Forest => AS_FOREST,
    }
}

impl Game {
    pub(super) fn queue_basic_land_type_substitution(
        &mut self,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
        chooser: crate::card::PlayerRefDef,
    ) {
        let Some(player) = self.player_reference(chooser, object, context, scoped) else {
            return;
        };
        self.queue_decision(
            player,
            "Each land of the first type becomes the second until end of turn",
            super::DecisionVisibility::PublicNotice,
            super::DecisionPreference::Neutral,
            1..=1,
            false,
            Self::basic_land_type_pair_options(),
            DecisionContinuation::BasicLandTypeSubstitution {
                object: Box::new(object.clone()),
                context: context.clone(),
                effect: scoped,
            },
        );
    }

    /// Applies the answer: every land presently carrying the first type
    /// becomes the second, and nothing else changes. `Set` rather than `Add`
    /// because "becomes" carries CR 305.7 with it.
    pub(super) fn resolve_basic_land_type_substitution(
        &mut self,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
        option: u32,
    ) {
        let Some((from, to)) = Self::basic_land_type_pair(option) else {
            return;
        };
        let affected: Vec<Target> = self
            .battlefield
            .iter()
            .filter(|permanent| self.effective_land_types(permanent)[from.index()])
            .map(|permanent| Target::Permanent(permanent.card.id))
            .collect();
        if affected.is_empty() {
            return;
        }
        let effect = AppliedEffectDef::Characteristic(CharacteristicOperationDef::BasicLandTypes(
            SetOperationDef::Set(as_only_type(to)),
        ));
        self.apply_effect_to_targets(
            &affected,
            effect,
            ResolvedEffectDurationDef::UntilEndOfTurn,
            object,
            context,
            scoped,
        );
    }
}

impl Game {
    /// Every ordered pair of distinct basic land types, as one option list.
    /// Magical Hack asks the same question for its text change, so the two
    /// share this encoding.
    pub(super) fn basic_land_type_pair_options() -> Vec<super::DecisionOption> {
        BasicLandType::ALL
            .into_iter()
            .flat_map(|from| {
                BasicLandType::ALL
                    .into_iter()
                    .filter(move |to| from != *to)
                    .map(move |to| super::DecisionOption {
                        id: u32::try_from(from.index() * BasicLandType::ALL.len() + to.index())
                            .expect("the basic-land-type choice id fits u32"),
                        label: format!("{} → {}", from.subtype(), to.subtype()),
                        card: None,
                        members: Vec::new(),
                        ability_text: None,
                        zone: super::DecisionZone::None,
                    })
            })
            .collect()
    }

    /// The pair an answer names, or nothing when it names one type twice.
    pub(super) fn basic_land_type_pair(option: u32) -> Option<(BasicLandType, BasicLandType)> {
        let width = u32::try_from(BasicLandType::ALL.len()).ok()?;
        let from = usize::try_from(option / width)
            .ok()
            .and_then(BasicLandType::from_index)?;
        let to = usize::try_from(option % width)
            .ok()
            .and_then(BasicLandType::from_index)?;
        (from != to).then_some((from, to))
    }
}
