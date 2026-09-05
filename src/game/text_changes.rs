use super::{
    AbilityDef, AbilityOrigin, BasicLandType, CharacteristicSource, ColorSet,
    DeclarativeAbilityDef, Game, GameObjectId, KeywordAbility, ManaColor, ObjectCharacteristics,
    Permanent, RetiredObject, SetOperationDef, TextChange, TextWordChange, TokenCharacteristics,
};

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

    pub(super) fn text_changed_basic_land_type(
        &self,
        source: GameObjectId,
        mut land_type: BasicLandType,
    ) -> BasicLandType {
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
            && let Some(token) = Self::copiable_token_words(permanent)
        {
            land_type = token.basic_land_type_word(land_type);
        }
        for change in self.text_changes_for_source(source) {
            if !self.text_change_is_active(source, change) {
                continue;
            }
            if let TextWordChange::BasicLandType { from, to } = change.word
                && land_type == from
            {
                land_type = to;
            }
        }
        land_type
    }

    pub(super) fn text_changed_color_word(
        &self,
        source: GameObjectId,
        mut color: ManaColor,
    ) -> ManaColor {
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
            && let Some(token) = Self::copiable_token_words(permanent)
        {
            color = token.color_word(color);
        }
        for change in self.text_changes_for_source(source) {
            if !self.text_change_is_active(source, change) {
                continue;
            }
            if let TextWordChange::Color { from, to } = change.word
                && color == from
            {
                color = to;
            }
        }
        color
    }

    pub(super) fn text_changed_color_set(
        &self,
        source: GameObjectId,
        colors: ColorSet,
    ) -> ColorSet {
        ManaColor::COLORS
            .into_iter()
            .filter(|color| colors.contains(*color))
            .map(|color| self.text_changed_color_word(source, color))
            .fold(ColorSet::empty(), ColorSet::with)
    }

    pub(super) fn text_changed_color_operation(
        &self,
        source: GameObjectId,
        operation: SetOperationDef<ColorSet>,
    ) -> SetOperationDef<ColorSet> {
        match operation {
            SetOperationDef::Add(colors) => {
                SetOperationDef::Add(self.text_changed_color_set(source, colors))
            }
            SetOperationDef::Remove(colors) => {
                SetOperationDef::Remove(self.text_changed_color_set(source, colors))
            }
            SetOperationDef::Set(colors) => {
                SetOperationDef::Set(self.text_changed_color_set(source, colors))
            }
        }
    }

    pub(super) fn text_changed_token(
        &self,
        source: GameObjectId,
        token: TokenCharacteristics,
    ) -> TokenCharacteristics {
        let basic_land_type_words = token
            .basic_land_type_word_map()
            .map(|word| self.text_changed_basic_land_type(source, word));
        let color_words = token
            .color_word_map()
            .map(|word| self.text_changed_color_word(source, word));
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
        let mut ability = *ability;
        if let DeclarativeAbilityDef::Keyword(KeywordAbility::Landwalk(land_type)) =
            ability.definition
        {
            ability.definition = DeclarativeAbilityDef::Keyword(KeywordAbility::Landwalk(
                self.text_changed_basic_land_type(source, land_type),
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
