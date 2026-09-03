use super::{
    AbilityDef, AbilityOrigin, BasicLandType, CharacteristicSource, ColorSet,
    DeclarativeAbilityDef, Game, GameObjectId, KeywordAbility, ManaColor, ObjectCharacteristics,
    Permanent, RetiredObject, SetOperationDef, TextChange, TextWordChange, TokenCharacteristics,
};

/// The effective meaning of every replaceable word in one object's rules
/// text. The declarative tree remains immutable; consumers interpret its
/// typed leaves through this small value instead of manufacturing a second,
/// owned copy of the tree.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct TextWordMap {
    basic_land_types: [BasicLandType; BasicLandType::ALL.len()],
    colors: [ManaColor; ManaColor::COLORS.len()],
}

/// How a predicate obtains its effective words. General reference matching
/// keeps the source identity lazy, while a prepared/static walk can carry the
/// compact map it already resolved for the source.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum TextWordView {
    Source(GameObjectId),
    Resolved(TextWordMap),
}

impl TextWordView {
    pub(super) fn basic_land_type(self, game: &Game, word: BasicLandType) -> BasicLandType {
        match self {
            Self::Source(source) => game.text_changed_basic_land_type(source, word),
            Self::Resolved(words) => words.basic_land_type(word),
        }
    }

    pub(super) fn color(self, game: &Game, word: ManaColor) -> ManaColor {
        match self {
            Self::Source(source) => game.text_changed_color_word(source, word),
            Self::Resolved(words) => words.color(word),
        }
    }
}

impl TextWordMap {
    const IDENTITY: Self = Self {
        basic_land_types: BasicLandType::ALL,
        colors: ManaColor::COLORS,
    };

    const fn from_token(token: TokenCharacteristics) -> Self {
        Self {
            basic_land_types: token.basic_land_type_word_map(),
            colors: token.color_word_map(),
        }
    }

    fn apply(&mut self, change: TextWordChange) {
        match change {
            TextWordChange::BasicLandType { from, to } => {
                for word in &mut self.basic_land_types {
                    if *word == from {
                        *word = to;
                    }
                }
            }
            TextWordChange::Color { from, to } => {
                for word in &mut self.colors {
                    if *word == from {
                        *word = to;
                    }
                }
            }
        }
    }

    pub(super) const fn basic_land_type(self, word: BasicLandType) -> BasicLandType {
        self.basic_land_types[word.index()]
    }

    pub(super) fn basic_land_types_are_identity(self) -> bool {
        self.basic_land_types == BasicLandType::ALL
    }

    pub(super) const fn color(self, word: ManaColor) -> ManaColor {
        let Some(index) = word.color_index() else {
            return word;
        };
        self.colors[index]
    }

    pub(super) fn color_set(self, colors: ColorSet) -> ColorSet {
        ManaColor::COLORS
            .into_iter()
            .filter(|color| colors.contains(*color))
            .map(|color| self.color(color))
            .fold(ColorSet::empty(), ColorSet::with)
    }

    pub(super) fn color_operation(
        self,
        operation: SetOperationDef<ColorSet>,
    ) -> SetOperationDef<ColorSet> {
        match operation {
            SetOperationDef::Add(colors) => SetOperationDef::Add(self.color_set(colors)),
            SetOperationDef::Remove(colors) => SetOperationDef::Remove(self.color_set(colors)),
            SetOperationDef::Set(colors) => SetOperationDef::Set(self.color_set(colors)),
        }
    }
}

impl Game {
    pub(super) const fn text_source_for_ability_origin(
        host: GameObjectId,
        origin: AbilityOrigin,
    ) -> GameObjectId {
        match origin {
            AbilityOrigin::Granted { source, .. }
            | AbilityOrigin::TokenGranted { source, .. }
            | AbilityOrigin::EmblemGranted { source, .. }
            | AbilityOrigin::FaceDownGranted { source, .. } => source,
            AbilityOrigin::Printed { .. }
            | AbilityOrigin::Token { .. }
            | AbilityOrigin::Emblem { .. }
            | AbilityOrigin::FaceDown { .. }
            | AbilityOrigin::IntrinsicBasicLand(_)
            | AbilityOrigin::IntrinsicCounter(_) => host,
        }
    }

    pub(super) fn copiable_token_words(permanent: &Permanent) -> Option<TokenCharacteristics> {
        if let Some(copy) = permanent.active_copy_values()
            && let ObjectCharacteristics::Token { token, .. } = copy.base
        {
            return Some(token);
        }
        match permanent.card.characteristics {
            CharacteristicSource::Token(token) => Some(token),
            CharacteristicSource::Card(_)
            | CharacteristicSource::Emblem(_)
            | CharacteristicSource::FaceDown(_)
            | CharacteristicSource::Copy(_)
            | CharacteristicSource::Ability(_)
            | CharacteristicSource::Meld(_) => None,
        }
    }

    fn text_changes_for_source(&self, source: GameObjectId) -> &[TextChange] {
        if let Some(object) = self.stack.iter().find(|object| object.id == source) {
            return &object.text_changes;
        }
        if let Some(RetiredObject::Stack(object)) = self.retired_objects.get(&source) {
            return &object.text_changes;
        }
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
        {
            return &permanent.text_changes;
        }
        self.stack
            .iter()
            .rev()
            .find(|object| object.source == Some(source))
            .map_or(&[], |object| object.text_changes.as_slice())
    }

    pub(super) fn frozen_text_changes_for_source(&self, source: GameObjectId) -> Vec<TextChange> {
        self.text_changes_for_source(source)
            .iter()
            .copied()
            .filter(|change| self.text_change_is_active(source, change))
            .collect()
    }

    fn text_change_is_active(&self, source: GameObjectId, change: &TextChange) -> bool {
        self.continuous_effect_expiration_is_active(change.expiration, source)
    }

    fn text_word_map(
        &self,
        source: GameObjectId,
        token: Option<TokenCharacteristics>,
        changes: &[TextChange],
    ) -> TextWordMap {
        let mut words = token.map_or(TextWordMap::IDENTITY, TextWordMap::from_token);
        for change in changes {
            if self.text_change_is_active(source, change) {
                words.apply(change.word);
            }
        }
        words
    }

    pub(super) fn text_word_map_for_permanent(&self, source: &Permanent) -> TextWordMap {
        self.text_word_map(
            source.card.id,
            Self::copiable_token_words(source),
            &source.text_changes,
        )
    }

    pub(super) fn text_word_map_for_source(&self, source: GameObjectId) -> TextWordMap {
        let token = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
            .and_then(Self::copiable_token_words);
        self.text_word_map(source, token, self.text_changes_for_source(source))
    }

    pub(super) fn text_changed_basic_land_type(
        &self,
        source: GameObjectId,
        land_type: BasicLandType,
    ) -> BasicLandType {
        self.text_word_map_for_source(source)
            .basic_land_type(land_type)
    }

    pub(super) fn text_changed_color_word(
        &self,
        source: GameObjectId,
        color: ManaColor,
    ) -> ManaColor {
        self.text_word_map_for_source(source).color(color)
    }

    pub(super) fn text_changed_color_set(
        &self,
        source: GameObjectId,
        colors: ColorSet,
    ) -> ColorSet {
        self.text_word_map_for_source(source).color_set(colors)
    }

    pub(super) fn text_changed_color_operation(
        &self,
        source: GameObjectId,
        operation: SetOperationDef<ColorSet>,
    ) -> SetOperationDef<ColorSet> {
        self.text_word_map_for_source(source)
            .color_operation(operation)
    }

    pub(super) fn text_changed_token(
        &self,
        source: GameObjectId,
        token: TokenCharacteristics,
    ) -> TokenCharacteristics {
        let words = self.text_word_map_for_source(source);
        let basic_land_type_words = token
            .basic_land_type_word_map()
            .map(|word| words.basic_land_type(word));
        let color_words = token.color_word_map().map(|word| words.color(word));
        let token = token.with_word_maps(basic_land_type_words, color_words);
        let colors = ManaColor::COLORS
            .into_iter()
            .filter(|color| token.rules().color_set().contains(*color))
            .map(|color| token.color_word(color))
            .fold(ColorSet::empty(), ColorSet::with);
        token.with_color_set(colors)
    }

    /// Applies layer 3 only to text that belongs to the affected object.
    /// Later grants are added in layer 6 and deliberately bypass this helper.
    pub(super) fn text_changed_ability(
        &self,
        source: GameObjectId,
        ability: &AbilityDef,
    ) -> AbilityDef {
        Self::text_changed_ability_with_words(self.text_word_map_for_source(source), ability)
    }

    pub(super) fn text_changed_ability_with_words(
        words: TextWordMap,
        ability: &AbilityDef,
    ) -> AbilityDef {
        let mut ability = *ability;
        if let DeclarativeAbilityDef::Keyword(KeywordAbility::Landwalk(land_type)) =
            ability.definition
        {
            ability.definition = DeclarativeAbilityDef::Keyword(KeywordAbility::Landwalk(
                words.basic_land_type(land_type),
            ));
        }
        ability
    }

    pub(super) fn text_changed_base_ability(
        &self,
        permanent: &Permanent,
        ability: &AbilityDef,
    ) -> AbilityDef {
        self.text_changed_ability(permanent.card.id, ability)
    }
}
