//! Mana-cost characteristics, independent of the resources used to cast a spell.
use super::{Game, GameObjectId, Permanent, RetiredObject, StackObject};
use crate::card::{CharacteristicContext, FlexibleManaSymbol, ManaColor, ManaCost};

fn symbols(cost: ManaCost, color: ManaColor) -> u16 {
    let fixed = match color {
        ManaColor::White => cost.white,
        ManaColor::Blue => cost.blue,
        ManaColor::Black => cost.black,
        ManaColor::Red => cost.red,
        ManaColor::Green => cost.green,
        ManaColor::Colorless => cost.colorless,
    };
    FlexibleManaSymbol::ALL
        .into_iter()
        .filter(|symbol| symbol.mana_options().contains(&color))
        .map(|symbol| cost.flexible_count(symbol))
        .fold(fixed, u16::saturating_add)
}

impl Game {
    pub(in crate::game) fn object_mana_symbol_count(
        &self,
        id: GameObjectId,
        color: ManaColor,
    ) -> u16 {
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
        {
            return self.permanent_mana_symbol_count(permanent, color);
        }
        if let Some(object) = self.stack.iter().find(|object| object.id == id) {
            return self.spell_mana_symbol_count(object, color);
        }
        if let Some((_, card)) = self.card_in_nonbattlefield_zone(id) {
            if self.exiled_card_is_face_down(id) {
                return 0;
            }
            return self.card_mana_symbol_count(
                card.definition,
                &self
                    .part_copy(id)
                    .map_or(CharacteristicContext::Hand, |part| {
                        CharacteristicContext::Stack {
                            form: crate::card::SpellForm::Part(part),
                        }
                    }),
                color,
            );
        }
        match self.retired_objects.get(&id) {
            Some(RetiredObject::Permanent { permanent, .. }) => {
                self.permanent_mana_symbol_count(permanent, color)
            }
            Some(RetiredObject::Stack(object)) => self.spell_mana_symbol_count(object, color),
            Some(RetiredObject::Card(card)) => self.card_mana_symbol_count(
                card.definition,
                &self
                    .part_copy(id)
                    .map_or(CharacteristicContext::Hand, |part| {
                        CharacteristicContext::Stack {
                            form: crate::card::SpellForm::Part(part),
                        }
                    }),
                color,
            ),
            None => 0,
        }
    }

    fn permanent_mana_symbol_count(&self, permanent: &Permanent, color: ManaColor) -> u16 {
        if permanent
            .active_copy_values()
            .is_some_and(|copy| copy.no_mana_cost)
        {
            return 0;
        }
        self.effective_rules(permanent)
            .and_then(|rules| rules.mana_cost())
            .map_or(0, |cost| symbols(cost, color))
    }

    fn spell_mana_symbol_count(&self, object: &StackObject, color: ManaColor) -> u16 {
        if let Some(face_down) = object.face_down {
            return face_down
                .rules()
                .mana_cost()
                .map_or(0, |cost| symbols(cost, color));
        }
        let Some(spell) = self.stack_spell_view(object) else {
            return 0;
        };
        self.card_mana_symbol_count(
            spell.definition,
            &CharacteristicContext::Stack {
                form: spell.form.clone(),
            },
            color,
        )
    }

    fn card_mana_symbol_count(
        &self,
        id: crate::CardDefinitionId,
        context: &CharacteristicContext,
        color: ManaColor,
    ) -> u16 {
        let Some(definition) = self.catalog.get(id) else {
            return 0;
        };
        let Ok(parts) = crate::card::applicable_part_ids_ref(definition, context) else {
            return 0;
        };
        parts
            .iter()
            .filter_map(|part| definition.part(*part)?.mana_cost())
            .map(|cost| symbols(cost, color))
            .fold(0, u16::saturating_add)
    }
}
