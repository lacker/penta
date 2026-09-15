use super::{Game, GameObjectId, RetiredObject};
use crate::card::{ColorSet, ColorSetDef, ObjectRefDef};

impl Game {
    pub(super) fn color_set_value(
        &self,
        value: ColorSetDef,
        resolve: impl Fn(ObjectRefDef) -> Option<GameObjectId>,
    ) -> ColorSet {
        match value {
            ColorSetDef::Fixed(colors) => colors,
            ColorSetDef::OfObject(reference) => resolve(reference)
                .map_or(ColorSet::empty(), |id| {
                    ColorSet::from_flags(self.object_colors(id))
                }),
            ColorSetDef::ChosenBy(reference) => resolve(reference)
                .and_then(|id| {
                    self.battlefield
                        .iter()
                        .find(|p| p.card.id == id)
                        .or_else(|| match self.retired_objects.get(&id) {
                            Some(RetiredObject::Permanent { permanent, .. }) => {
                                Some(permanent.as_ref())
                            }
                            _ => None,
                        })
                })
                .map_or(ColorSet::empty(), |p| {
                    if p.chosen_colors.is_colorless() {
                        p.chosen_color
                            .filter(|color| color.color_index().is_some())
                            .map_or(ColorSet::empty(), |color| ColorSet::from_colors(&[color]))
                    } else {
                        p.chosen_colors
                    }
                }),
        }
    }

    pub(super) fn color_intersection(
        sets: &[ColorSetDef],
        mut evaluate: impl FnMut(ColorSetDef) -> ColorSet,
    ) -> ColorSet {
        let mut sets = sets.iter().copied();
        let Some(first) = sets.next() else {
            return ColorSet::empty();
        };
        sets.fold(evaluate(first), |colors, set| {
            colors.intersection(evaluate(set))
        })
    }
}
