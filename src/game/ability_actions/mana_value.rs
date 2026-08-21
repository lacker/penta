use super::{
    CardPart, CardStructure, DoubleFacedKind, Game, GameObjectId, ObjectCharacteristics, Permanent,
    RetiredObject, StackObject, mana_cost_value,
};

impl Game {
    pub(in crate::game) fn permanent_mana_value(&self, permanent: &Permanent) -> u16 {
        // A transforming double-faced permanent keeps the mana value of its
        // front face while its back face is up. A permanent merely copying a
        // back face is not itself that transforming double-faced card, so its
        // copied characteristics continue through the ordinary path below.
        if permanent.copy_effect.is_none() {
            if let Some(faces) = &permanent.double_faced_token_copy
                && faces.kind == DoubleFacedKind::Transforming
            {
                return self.object_characteristics_mana_value(faces.front.base);
            }
            if let Some(token) = permanent.token_characteristics
                && token.other_face(token.primary_part_id()).is_some()
            {
                return token.rules().printed_mana_cost().mana_value();
            }
            if permanent.copied_from.is_none()
                && let Some(card_definition) = permanent.card.definition.card_definition()
                && let Some(definition) = self.catalog.get(card_definition)
                && let CardStructure::DoubleFaced {
                    front,
                    kind: DoubleFacedKind::Transforming,
                    ..
                } = &definition.structure
            {
                return definition
                    .part(*front)
                    .map_or(0, |part| part.rules.printed_mana_cost().mana_value());
            }
        }
        self.effective_rules(permanent)
            .map_or(0, |rules| rules.printed_mana_cost().mana_value())
    }

    fn object_characteristics_mana_value(&self, characteristics: ObjectCharacteristics) -> u16 {
        match characteristics {
            ObjectCharacteristics::Card { definition, part } => self
                .catalog
                .get(definition)
                .and_then(|definition| definition.part(part))
                .map_or(0, |part| part.rules.printed_mana_cost().mana_value()),
            ObjectCharacteristics::Token { token, part } => token
                .part(part)
                .map_or(0, |part| part.rules.printed_mana_cost().mana_value()),
            ObjectCharacteristics::Emblem { .. } => 0,
        }
    }

    /// A permanent or spell's mana value, still readable after it has left
    /// its zone so a later effect in the same sequence can measure it.
    pub(in crate::game) fn current_or_last_known_mana_value(
        &self,
        id: GameObjectId,
    ) -> Option<u16> {
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
        {
            return Some(self.permanent_mana_value(permanent));
        }
        if let Some(object) = self.stack.iter().find(|object| object.id == id) {
            return Some(self.stack_spell_mana_value(object));
        }
        if let Some((_, card)) = self.card_in_nonbattlefield_zone(id) {
            return self
                .catalog
                .get(card.definition)
                .map(|definition| definition.rules.printed_mana_cost().mana_value());
        }
        match self.retired_objects.get(&id) {
            Some(RetiredObject::Permanent { mana_value, .. }) => Some(*mana_value),
            Some(RetiredObject::Stack(object)) => Some(self.stack_spell_mana_value(object)),
            Some(RetiredObject::Card(card)) => self
                .catalog
                .get(card.definition)
                .map(|definition| definition.rules.printed_mana_cost().mana_value()),
            None => None,
        }
    }

    pub(in crate::game) fn stack_spell_mana_value(&self, object: &StackObject) -> u16 {
        let Some(card_definition) = object.card.definition.card_definition() else {
            return 0;
        };
        let Some(definition) = self.catalog.get(card_definition) else {
            return 0;
        };
        let Some(signature) = &object.signature else {
            return 0;
        };
        match signature.form() {
            crate::card::SpellForm::Part(part) => definition
                .part(*part)
                .and_then(CardPart::mana_cost)
                .map_or(0, mana_cost_value),
            crate::card::SpellForm::Combined(parts) => parts
                .iter()
                .filter_map(|part| definition.part(*part).and_then(CardPart::mana_cost))
                .map(mana_cost_value)
                .fold(0, u16::saturating_add),
        }
    }
}
