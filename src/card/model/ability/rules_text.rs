use std::borrow::Cow;
use std::fmt::Write as _;

use super::AbilityDef;
use crate::card::{DeclarativeAbilityDef, ModalModeListDef, SpellAbilityDef};

impl AbilityDef {
    /// Renders the complete printed clause. Most abilities borrow their
    /// canonical static text; structured alternative-casting keywords insert
    /// their owned mana cost into canonical reminder text. Modal clauses
    /// append their selection instructions, costs, and ordered modes.
    #[must_use]
    pub fn rules_text(&self) -> Cow<'static, str> {
        match self.definition {
            DeclarativeAbilityDef::AlternativeCast(definition) => {
                Cow::Owned(definition.rules_text())
            }
            DeclarativeAbilityDef::OptionalAdditionalCost(definition)
                if self.text == definition.kind.label() =>
            {
                Cow::Owned(definition.rules_text())
            }
            DeclarativeAbilityDef::Spell(SpellAbilityDef::Modal(modal)) => {
                let mut text = self.text.to_owned();
                if let Some(selection) = modal.selection_text {
                    write!(text, "\n{selection}").expect("writing to a string cannot fail");
                }
                match modal.modes {
                    ModalModeListDef::Ordinary(modes) => {
                        for mode in modes {
                            write!(text, "\n• {}", mode.rules_text())
                                .expect("writing to a string cannot fail");
                        }
                    }
                    ModalModeListDef::WithAdditionalCosts(modes) => {
                        for (cost, mode) in modes {
                            write!(
                                text,
                                "\n+ {} — {}",
                                crate::card::costs::rules_text(cost)
                                    .unwrap_or_else(|| "Pay the additional cost".into()),
                                mode.rules_text()
                            )
                            .expect("writing to a string cannot fail");
                        }
                    }
                }
                Cow::Owned(text)
            }
            _ => Cow::Borrowed(self.text),
        }
    }
}
