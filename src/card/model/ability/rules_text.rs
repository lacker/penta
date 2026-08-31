use std::borrow::Cow;
use std::fmt::Write as _;

use super::AbilityDef;
use crate::card::{DeclarativeAbilityDef, ModalModeListDef, SpellAbilityDef};

impl AbilityDef {
    /// Renders the complete printed clause. Most abilities borrow their
    /// canonical static text; structured alternative-casting keywords insert
    /// their owned mana cost into canonical reminder text, and Escalate
    /// appends its ordered modes to the keyword clause.
    #[must_use]
    pub fn rules_text(&self) -> Cow<'static, str> {
        match self.definition {
            DeclarativeAbilityDef::AlternativeCast(definition) => {
                Cow::Owned(definition.rules_text())
            }
            DeclarativeAbilityDef::OptionalAdditionalCost(definition)
                if definition.mana_cost.is_some() && self.text == definition.kind.label() =>
            {
                Cow::Owned(definition.rules_text())
            }
            DeclarativeAbilityDef::Spell(SpellAbilityDef::Modal(modal))
                if self.text == "Spree" && modal.modes.has_additional_mana_costs() =>
            {
                let mut text = String::from("Spree (Choose one or more additional costs.)");
                if let ModalModeListDef::WithAdditionalManaCosts(modes) = modal.modes {
                    for (cost, mode) in modes {
                        write!(text, "\n+ {cost} — {}", mode.text)
                            .expect("writing to a string cannot fail");
                    }
                }
                Cow::Owned(text)
            }
            DeclarativeAbilityDef::Spell(SpellAbilityDef::Modal(modal))
                if modal.escalate_cost.is_some() =>
            {
                Cow::Owned(format!(
                    "{}\nChoose one or more —\n{}",
                    self.text,
                    modal
                        .modes
                        .iter()
                        .map(|mode| format!("• {}", mode.rules_text()))
                        .collect::<Vec<_>>()
                        .join("\n")
                ))
            }
            _ => Cow::Borrowed(self.text),
        }
    }
}
