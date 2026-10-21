//! Action identity and clause metadata, without expanding target assignments.
use crate::card::{ModalSpellDef, SpellAbilityDef};
use crate::{AbilityDef, AbilityOrigin, AbilityTargetDef, DeclarativeAbilityDef, DividedTotal};
use serde_json::{Value, json};

/// Describe a visible object's effective clause and its local selection IDs.
#[must_use]
pub fn proposal_ability_json(origin: AbilityOrigin, ability: AbilityDef) -> Value {
    let (kind, targets, modes) = match ability.definition {
        DeclarativeAbilityDef::Spell(spell) => match spell {
            SpellAbilityDef::Nonmodal { targets, .. } => ("spell", targets, None),
            SpellAbilityDef::Modal(modes) => ("spell", &[][..], Some(modes)),
        },
        DeclarativeAbilityDef::Activated(def) => ("activated", def.targets, def.modes),
        DeclarativeAbilityDef::ActivatedMana(def) => ("mana", def.targets, def.modes),
        DeclarativeAbilityDef::Triggered(def) => ("triggered", def.targets, None),
        DeclarativeAbilityDef::TriggeredMana(def) => ("triggeredMana", def.targets, None),
        DeclarativeAbilityDef::Static(_) => ("static", &[][..], None),
        DeclarativeAbilityDef::Replacement(_) => ("replacement", &[][..], None),
        DeclarativeAbilityDef::AlternativeCast(_) => ("alternativeCast", &[][..], None),
        DeclarativeAbilityDef::OptionalAdditionalCost(_) => ("additionalCost", &[][..], None),
        DeclarativeAbilityDef::SpecialAction(_) => ("special", &[][..], None),
        DeclarativeAbilityDef::Pregame(_) => ("pregame", &[][..], None),
        DeclarativeAbilityDef::Keyword(_) => ("keyword", &[][..], None),
        DeclarativeAbilityDef::DeckConstruction(_) => ("deckConstruction", &[][..], None),
        DeclarativeAbilityDef::Companion(_) => ("companion", &[][..], None),
    };
    json!({"ability":super::ability_origin_json(origin),"kind":kind,"text":ability.rules_text(),
        "targetSlots":target_slots(targets),"modes":modes.map(mode_schema)})
}

fn target_slots(targets: &[AbilityTargetDef]) -> Vec<Value> {
    targets.iter().enumerate().map(|(index, slot)| json!({
        "slotId":index,"minimum":slot.minimum,"maximum":slot.maximum,
        "countFromRules":slot.exact_count.is_some() || slot.maximum == AbilityTargetDef::UNLIMITED,
        "another":slot.another,"excludesSource":slot.excludes_source,
        "dividedTotal":match slot.divided_total {
            None=>Value::Null,Some(DividedTotal::Fixed(n))=>json!({"kind":"fixed","amount":n}),
            Some(DividedTotal::ChosenX)=>json!({"kind":"chosenX"}),
        },
    })).collect()
}

fn mode_schema(modes: ModalSpellDef) -> Value {
    json!({"minimum":modes.minimum,"maximum":modes.maximum,"mayRepeat":modes.may_repeat,
        "choices":modes.modes.iter().enumerate().map(|(id,mode)| {
            let targets=match mode.definition {DeclarativeAbilityDef::Spell(spell)=>spell.targets(),_=>&[]};
            json!({"modeId":id,"text":mode.rules_text(),"targetSlots":target_slots(targets)})
        }).collect::<Vec<_>>()})
}
