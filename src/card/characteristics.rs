//! Selection of the printed card parts that supply an object's characteristics.
//!
//! This module deliberately resolves catalog structure only. Continuous
//! effects, copy effects, face-down characteristics, and physical backing are
//! separate layers. In particular, transforming or flipping a permanent
//! changes the `presented` part stored by that game object; it does not change
//! the identity of a physical card.

use std::error::Error;
use std::fmt;

use super::{CardDefinition, CardStructure, CardTypeSet, PlayActionKind, SpellForm};
use crate::{CardDefinitionId, CardPartId};

/// The zone-sensitive information needed to select printed characteristics.
///
/// A stack context always carries the spell form locked in while the object
/// was played (and retained by a copy). A battlefield context always carries
/// the part currently presented by the permanent. Phased-out permanents still
/// use `Battlefield` because phasing does not move an object to another zone.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CharacteristicContext {
    Library,
    Hand,
    Graveyard,
    Exile,
    Command,
    Stack { form: SpellForm },
    Battlefield { presented: CardPartId },
}

impl CharacteristicContext {
    /// The zone this context is a view of, when it is a zone at all. The
    /// stack and the battlefield answer `None`: what a card is there is a
    /// spell or a permanent, and both have their own views.
    #[must_use]
    pub const fn zone(&self) -> Option<super::ZoneKind> {
        match self {
            Self::Library => Some(super::ZoneKind::Library),
            Self::Hand => Some(super::ZoneKind::Hand),
            Self::Graveyard => Some(super::ZoneKind::Graveyard),
            Self::Exile => Some(super::ZoneKind::Exile),
            Self::Command => Some(super::ZoneKind::Command),
            Self::Stack { .. } | Self::Battlefield { .. } => None,
        }
    }

    const fn uses_canonical_outside_stack_parts(&self) -> bool {
        matches!(
            self,
            Self::Library | Self::Hand | Self::Graveyard | Self::Exile | Self::Command
        )
    }
}

/// Resolves the ordered printed parts that apply to `definition` in `context`.
///
/// Split cards return every half, in printed order, outside the stack. A spell
/// returns the part or ordered combination named by its legal cast play option.
/// A permanent returns its explicitly presented part. The resolver does not
/// infer presentation from a printing or physical backing.
///
/// # Errors
///
/// Returns [`CharacteristicError`] if the definition's applicable structure
/// refers to an absent part, a stack form is not one of the definition's spell
/// play options, or a battlefield presentation is not a permanent part in the
/// definition's structure.
pub fn applicable_part_ids(
    definition: &CardDefinition,
    context: &CharacteristicContext,
) -> Result<Vec<CardPartId>, CharacteristicError> {
    applicable_part_ids_ref(definition, context).map(<[CardPartId]>::to_vec)
}

/// Borrowed form for runtime characteristic queries. Every applicable part is
/// already stored in either the definition or the context, so engine hot paths
/// do not need to allocate a temporary vector merely to iterate those IDs.
pub(crate) fn applicable_part_ids_ref<'a>(
    definition: &'a CardDefinition,
    context: &'a CharacteristicContext,
) -> Result<&'a [CardPartId], CharacteristicError> {
    let parts = if context.uses_canonical_outside_stack_parts() {
        outside_stack_parts(&definition.structure)
    } else {
        match context {
            CharacteristicContext::Stack { form } => {
                if !definition.play_options.iter().any(|option| {
                    option.action == PlayActionKind::CastSpell && &option.form == form
                }) {
                    return Err(CharacteristicError::UnavailableSpellForm {
                        definition: definition.id,
                        form: form.clone(),
                    });
                }
                spell_form_parts(form)
            }
            CharacteristicContext::Battlefield { presented } => {
                if !structure_contains(&definition.structure, *presented) {
                    return Err(CharacteristicError::PartNotInStructure {
                        definition: definition.id,
                        part: *presented,
                    });
                }
                let part =
                    definition
                        .part(*presented)
                        .ok_or(CharacteristicError::UndefinedPart {
                            definition: definition.id,
                            part: *presented,
                        })?;
                if !part.rules.types().is_permanent() {
                    return Err(CharacteristicError::NonpermanentPresentation {
                        definition: definition.id,
                        part: *presented,
                        types: part.rules.types(),
                    });
                }
                std::slice::from_ref(presented)
            }
            CharacteristicContext::Library
            | CharacteristicContext::Hand
            | CharacteristicContext::Graveyard
            | CharacteristicContext::Exile
            | CharacteristicContext::Command => {
                unreachable!("outside-stack contexts were handled above")
            }
        }
    };

    if parts.is_empty() {
        return Err(CharacteristicError::NoApplicableParts {
            definition: definition.id,
        });
    }
    for part in parts {
        if !structure_contains(&definition.structure, *part) {
            return Err(CharacteristicError::PartNotInStructure {
                definition: definition.id,
                part: *part,
            });
        }
        if definition.part(*part).is_none() {
            return Err(CharacteristicError::UndefinedPart {
                definition: definition.id,
                part: *part,
            });
        }
    }
    Ok(parts)
}

fn outside_stack_parts(structure: &CardStructure) -> &[CardPartId] {
    match structure {
        CardStructure::Single { main } | CardStructure::AlternateSpell { main, .. } => {
            std::slice::from_ref(main)
        }
        CardStructure::Split { parts, .. } => parts,
        // A Room's doors, and only its doors: outside the battlefield a Room
        // card is the combination of the two halves as printed, which is why
        // Walk-In Closet // Forgotten Cellar has mana value 8 in a library.
        // The combined and locked parts describe a permanent's state rather
        // than anything printed, so nothing outside the battlefield uses
        // them.
        CardStructure::Room { doors, .. } => doors,
        CardStructure::Flip { normal, .. } => std::slice::from_ref(normal),
        CardStructure::DoubleFaced { front, .. } | CardStructure::MeldPart { front, .. } => {
            std::slice::from_ref(front)
        }
    }
}

fn spell_form_parts(form: &SpellForm) -> &[CardPartId] {
    match form {
        SpellForm::Part(part) => std::slice::from_ref(part),
        SpellForm::Combined(parts) => parts,
    }
}

fn structure_contains(structure: &CardStructure, wanted: CardPartId) -> bool {
    match structure {
        CardStructure::Single { main } => *main == wanted,
        CardStructure::Split { parts, .. } => parts.contains(&wanted),
        CardStructure::Room {
            doors,
            combined,
            locked,
        } => doors.contains(&wanted) || *combined == wanted || *locked == wanted,
        CardStructure::Flip { normal, flipped } => *normal == wanted || *flipped == wanted,
        CardStructure::DoubleFaced { front, back, .. } => *front == wanted || *back == wanted,
        CardStructure::AlternateSpell {
            main, alternate, ..
        } => *main == wanted || *alternate == wanted,
        CardStructure::MeldPart { front, .. } => *front == wanted,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CharacteristicError {
    NoApplicableParts {
        definition: CardDefinitionId,
    },
    UndefinedPart {
        definition: CardDefinitionId,
        part: CardPartId,
    },
    PartNotInStructure {
        definition: CardDefinitionId,
        part: CardPartId,
    },
    UnavailableSpellForm {
        definition: CardDefinitionId,
        form: SpellForm,
    },
    NonpermanentPresentation {
        definition: CardDefinitionId,
        part: CardPartId,
        types: CardTypeSet,
    },
}

impl fmt::Display for CharacteristicError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoApplicableParts { definition } => {
                write!(
                    formatter,
                    "card definition {definition:?} has no applicable parts"
                )
            }
            Self::UndefinedPart { definition, part } => write!(
                formatter,
                "card definition {definition:?} refers to undefined part {part:?}"
            ),
            Self::PartNotInStructure { definition, part } => write!(
                formatter,
                "part {part:?} is not in card definition {definition:?}'s structure"
            ),
            Self::UnavailableSpellForm { definition, form } => write!(
                formatter,
                "spell form {form:?} is not a cast option for card definition {definition:?}"
            ),
            Self::NonpermanentPresentation {
                definition,
                part,
                types,
            } => write!(
                formatter,
                "part {part:?} of card definition {definition:?} has types {types:?}, not a permanent"
            ),
        }
    }
}

impl Error for CharacteristicError {}

#[cfg(test)]
mod tests {
    use super::{CharacteristicContext, CharacteristicError, applicable_part_ids};
    use crate::card::{CardCatalog, cards};
    use crate::{
        AlternateSpellKind, CardBehavior, CardDefinition, CardDefinitionId, CardPart, CardPartId,
        CardRules, CardSet, CardStructure, ManaCost, PlayOptionDef, PlayOptionId, SpellForm,
    };

    fn definition(catalog: &CardCatalog, id: CardDefinitionId) -> crate::CardDefinition {
        catalog
            .get(id)
            .unwrap_or_else(|| panic!("built-in definition {id:?} is present"))
            .clone()
    }

    #[test]
    fn single_card_uses_its_primary_part_in_every_applicable_zone() {
        let catalog = crate::card::catalog().expect("built-in catalog is valid");
        let card = definition(&catalog, cards::GOBLIN_BALLOON_BRIGADE);
        let expected = vec![CardPartId::PRIMARY];

        for context in [
            CharacteristicContext::Library,
            CharacteristicContext::Hand,
            CharacteristicContext::Graveyard,
            CharacteristicContext::Exile,
            CharacteristicContext::Command,
            CharacteristicContext::Stack {
                form: SpellForm::Part(CardPartId::PRIMARY),
            },
            CharacteristicContext::Battlefield {
                presented: CardPartId::PRIMARY,
            },
        ] {
            assert_eq!(applicable_part_ids(&card, &context), Ok(expected.clone()));
        }
    }

    #[test]
    fn split_card_combines_parts_outside_stack_and_uses_locked_stack_form() {
        let catalog = crate::card::catalog().expect("built-in catalog is valid");
        let card = definition(&catalog, cards::TURN_BURN);
        let turn = CardPartId::PRIMARY;
        let burn = CardPartId(1);

        assert_eq!(
            applicable_part_ids(&card, &CharacteristicContext::Hand),
            Ok(vec![turn, burn])
        );
        assert_eq!(
            applicable_part_ids(
                &card,
                &CharacteristicContext::Stack {
                    form: SpellForm::Part(turn),
                },
            ),
            Ok(vec![turn])
        );
        assert_eq!(
            applicable_part_ids(
                &card,
                &CharacteristicContext::Stack {
                    form: SpellForm::Part(burn),
                },
            ),
            Ok(vec![burn])
        );
        assert_eq!(
            applicable_part_ids(
                &card,
                &CharacteristicContext::Stack {
                    form: SpellForm::Combined(vec![turn, burn]),
                },
            ),
            Ok(vec![turn, burn])
        );

        let reversed = SpellForm::Combined(vec![burn, turn]);
        assert_eq!(
            applicable_part_ids(
                &card,
                &CharacteristicContext::Stack {
                    form: reversed.clone(),
                },
            ),
            Err(CharacteristicError::UnavailableSpellForm {
                definition: cards::TURN_BURN,
                form: reversed,
            })
        );
        assert!(matches!(
            applicable_part_ids(
                &card,
                &CharacteristicContext::Battlefield { presented: turn },
            ),
            Err(CharacteristicError::NonpermanentPresentation { .. })
        ));
    }

    #[test]
    fn transforming_card_defaults_front_but_battlefield_presentation_is_explicit() {
        let catalog = crate::card::catalog().expect("built-in catalog is valid");
        let card = definition(&catalog, cards::HUNTMASTER_OF_THE_FELLS);
        let front = CardPartId::PRIMARY;
        let back = CardPartId(1);

        assert_eq!(
            applicable_part_ids(&card, &CharacteristicContext::Graveyard),
            Ok(vec![front])
        );
        assert_eq!(
            applicable_part_ids(
                &card,
                &CharacteristicContext::Stack {
                    form: SpellForm::Part(front),
                },
            ),
            Ok(vec![front])
        );
        assert_eq!(
            applicable_part_ids(
                &card,
                &CharacteristicContext::Battlefield { presented: front },
            ),
            Ok(vec![front])
        );
        assert_eq!(
            applicable_part_ids(
                &card,
                &CharacteristicContext::Battlefield { presented: back },
            ),
            Ok(vec![back])
        );

        assert_eq!(
            applicable_part_ids(
                &card,
                &CharacteristicContext::Stack {
                    form: SpellForm::Part(back),
                },
            ),
            Err(CharacteristicError::UnavailableSpellForm {
                definition: cards::HUNTMASTER_OF_THE_FELLS,
                form: SpellForm::Part(back),
            })
        );
        assert_eq!(
            applicable_part_ids(
                &card,
                &CharacteristicContext::Battlefield {
                    presented: CardPartId(99),
                },
            ),
            Err(CharacteristicError::PartNotInStructure {
                definition: cards::HUNTMASTER_OF_THE_FELLS,
                part: CardPartId(99),
            })
        );
    }

    #[test]
    fn flip_and_alternate_spell_parts_follow_zone_context() {
        let normal = CardPartId::PRIMARY;
        let flipped = CardPartId(1);
        let creature_rules = CardRules::new_creature(ManaCost::new(2, 0), &[], 2, 2);
        let flipped_rules = CardRules::new_creature_without_mana_cost(&[], 4, 4);
        let mut flip = CardDefinition::new(
            CardDefinitionId::new(20_000),
            "Test flip card",
            CardSet::Innistrad,
            false,
            CardBehavior::Unsupported,
        );
        flip.parts = vec![
            CardPart::new(normal, "Normal", creature_rules),
            CardPart::new(flipped, "Flipped", flipped_rules),
        ];
        flip.structure = CardStructure::Flip { normal, flipped };
        flip.play_options = vec![PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Normal",
            SpellForm::Part(normal),
            creature_rules
                .mana_cost()
                .expect("the test creature has a printed mana cost"),
            crate::CardEffectStatus::MetadataOnly,
        )];

        assert_eq!(
            applicable_part_ids(&flip, &CharacteristicContext::Hand),
            Ok(vec![normal])
        );
        assert_eq!(
            applicable_part_ids(
                &flip,
                &CharacteristicContext::Battlefield { presented: flipped },
            ),
            Ok(vec![flipped])
        );

        let main = CardPartId::PRIMARY;
        let adventure = CardPartId(1);
        let adventure_rules = CardRules::new_instant(ManaCost::new(1, 0));
        let mut alternate = CardDefinition::new(
            CardDefinitionId::new(20_001),
            "Test adventurer",
            CardSet::Innistrad,
            false,
            CardBehavior::Unsupported,
        );
        alternate.parts = vec![
            CardPart::new(main, "Test adventurer", creature_rules),
            CardPart::new(adventure, "Test adventure", adventure_rules),
        ];
        alternate.structure = CardStructure::AlternateSpell {
            main,
            alternate: adventure,
            kind: AlternateSpellKind::Adventure,
        };
        alternate.play_options = vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Test adventurer",
                SpellForm::Part(main),
                creature_rules
                    .mana_cost()
                    .expect("the test creature has a printed mana cost"),
                crate::CardEffectStatus::MetadataOnly,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Test adventure",
                SpellForm::Part(adventure),
                adventure_rules
                    .mana_cost()
                    .expect("the test adventure has a printed mana cost"),
                crate::CardEffectStatus::MetadataOnly,
            ),
        ];

        assert_eq!(
            applicable_part_ids(&alternate, &CharacteristicContext::Graveyard),
            Ok(vec![main])
        );
        assert_eq!(
            applicable_part_ids(
                &alternate,
                &CharacteristicContext::Stack {
                    form: SpellForm::Part(adventure),
                },
            ),
            Ok(vec![adventure])
        );
    }
}
