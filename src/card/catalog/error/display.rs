use std::error::Error;
use std::fmt;

use super::{CatalogError, MismatchedAdditionalCost, MismatchedAlternativeCost};

impl fmt::Display for CatalogError {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateId(id) => write!(formatter, "duplicate card definition ID {id:?}"),
            Self::DuplicateName(name) => write!(formatter, "duplicate card name {name:?}"),
            Self::DuplicatePrintingId(id) => write!(formatter, "duplicate card printing ID {id:?}"),
            Self::MismatchedPrintingDefinition {
                definition,
                printing,
            } => write!(
                formatter,
                "card printing {printing:?} was supplied by definition {definition:?}"
            ),
            Self::OrphanPrinting(id) => write!(
                formatter,
                "card printing {id:?} references an unknown definition"
            ),
            Self::EmptyAbilityText {
                definition,
                part,
                ability,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} has empty rules text"
            ),
            Self::MissingImplementationExplanation {
                definition,
                part,
                ability,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} has a non-declarative implementation without an explanation"
            ),
            Self::LegacyProcedureRequiresCustomExecution {
                definition,
                part,
                ability,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} uses the legacy rules procedure without a custom effect executor"
            ),
            Self::DuplicatePartId { definition, part } => write!(
                formatter,
                "card definition {definition:?} defines part {part:?} more than once"
            ),
            Self::IncoherentCardRules {
                definition,
                part,
                explanation,
            } => write!(
                formatter,
                "part {part:?} of card definition {definition:?} has incoherent rules: {explanation}"
            ),
            Self::MismatchedPrimaryRules { definition, part } => write!(
                formatter,
                "card definition {definition:?} has compatibility rules that differ from primary part {part:?}"
            ),
            Self::TooManyAbilities {
                definition,
                part,
                count,
            } => write!(
                formatter,
                "part {part:?} of card definition {definition:?} defines {count} abilities, but positional ability IDs support at most 256"
            ),
            Self::MultipleSpellAbilities {
                definition,
                part,
                count,
            } => write!(
                formatter,
                "part {part:?} of card definition {definition:?} defines {count} spell abilities, but one castable card part must have at most one"
            ),
            Self::InvalidModalSpellParent {
                definition,
                part,
                ability,
            } => write!(
                formatter,
                "modal spell ability {ability:?} on part {part:?} of card definition {definition:?} must be a targetless declarative wrapper with no effect of its own"
            ),
            Self::TooManySpellModes {
                definition,
                part,
                ability,
                count,
            } => write!(
                formatter,
                "spell ability {ability:?} on part {part:?} of card definition {definition:?} defines {count} modes, but positional mode IDs support at most 256"
            ),
            Self::InvalidModalSpellSelection {
                definition,
                part,
                ability,
                minimum,
                maximum,
                may_repeat,
                available,
            } => write!(
                formatter,
                "spell ability {ability:?} on part {part:?} of card definition {definition:?} declares {available} modes with selection bounds {minimum}..={maximum} (repeat={may_repeat}), which cannot produce a legal selection"
            ),
            Self::NonSpellMode {
                definition,
                part,
                ability,
                mode,
            } => write!(
                formatter,
                "mode {mode:?} of spell ability {ability:?} on part {part:?} of card definition {definition:?} is not an ordinary spell ability"
            ),
            Self::NestedModalSpellMode {
                definition,
                part,
                ability,
                mode,
            } => write!(
                formatter,
                "mode {mode:?} of spell ability {ability:?} on part {part:?} of card definition {definition:?} is itself modal"
            ),
            Self::CustomSpellModeImplementation {
                definition,
                part,
                ability,
                mode,
            } => write!(
                formatter,
                "mode {mode:?} of spell ability {ability:?} on part {part:?} of card definition {definition:?} uses a custom implementation, but modal branches currently require declarative effects"
            ),
            Self::InvalidSpellMode {
                definition,
                part,
                ability,
                mode,
                problem,
            } => write!(
                formatter,
                "mode {mode:?} of spell ability {ability:?} on part {part:?} of card definition {definition:?} {problem}"
            ),
            Self::TooManyAbilityGrantSites {
                definition,
                part,
                ability,
                count,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} defines {count} grant sites, but grant IDs support at most 256"
            ),
            Self::InvalidGrantedAbility {
                definition,
                part,
                ability,
                grant_path,
                problem,
            } => write!(
                formatter,
                "granted ability at path {grant_path:?} from ability {ability:?} on part {part:?} of card definition {definition:?} {problem}"
            ),
            Self::AbilityHasNoSourceZone {
                definition,
                part,
                ability,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} has no source zone"
            ),
            Self::ManaAbilityHasTargets {
                definition,
                part,
                ability,
            } => write!(
                formatter,
                "mana ability {ability:?} on part {part:?} of card definition {definition:?} declares targets"
            ),
            Self::ReplacementAbilityRequiresReplacementProgram {
                definition,
                part,
                ability,
            } => write!(
                formatter,
                "replacement ability {ability:?} on part {part:?} of card definition {definition:?} does not define a replacement program"
            ),
            Self::ReplacementProgramRequiresReplacementAbility {
                definition,
                part,
                ability,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} defines a replacement program but is not a replacement ability"
            ),
            Self::UnsupportedReplacementProgram {
                definition,
                part,
                ability,
                event,
                operation,
            } => write!(
                formatter,
                "replacement ability {ability:?} on part {part:?} of card definition {definition:?} uses unsupported operation {operation} for event {event:?}"
            ),
            Self::UnsupportedInstalledTriggerAbility {
                definition,
                part,
                ability,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} installs an ability that is not a targetless shared declarative triggered ability"
            ),
            Self::UnsupportedTriggerEvent {
                definition,
                part,
                ability,
                event,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} uses unsupported trigger event {event:?}"
            ),
            Self::UnsupportedTriggeredManaProgram {
                definition,
                part,
                ability,
            } => write!(
                formatter,
                "triggered mana ability {ability:?} on part {part:?} of card definition {definition:?} cannot resolve its program immediately"
            ),
            Self::UnsupportedResolvingAppliedEffect {
                definition,
                part,
                ability,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} uses a resolving applied effect that cannot be stored on its recipient"
            ),
            Self::UnsupportedAbilityEffectProgramContext {
                definition,
                part,
                ability,
                context,
                operation,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} uses {operation} in a {context} effect program, where that operation is not interpreted"
            ),
            Self::TooManyAbilityTargets {
                definition,
                part,
                ability,
                count,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} defines {count} targets, but positional target indices support at most 256"
            ),
            Self::InvalidAbilityTargetBounds {
                definition,
                part,
                ability,
                target,
                minimum,
                maximum,
            } => write!(
                formatter,
                "target {target:?} of ability {ability:?} on part {part:?} of card definition {definition:?} requires at least {minimum} targets but allows at most {maximum}"
            ),
            Self::UnsupportedActivatedAbilityTargetChoice {
                definition,
                part,
                ability,
                target,
            } => write!(
                formatter,
                "target {target:?} of activated ability {ability:?} on part {part:?} of card definition {definition:?} has an alternate chooser, but only one undivided final target chosen by the opponent is supported"
            ),
            Self::AbilityTargetReferenceOutOfBounds {
                definition,
                part,
                ability,
                target,
                target_count,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} references target {target:?}, but defines only {target_count} target slots"
            ),
            Self::AbilityTargetReferenceKindMismatch {
                definition,
                part,
                ability,
                target,
                predicate,
                expected,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} references target {target:?} as an {expected:?}, but its predicate is {predicate:?}"
            ),
            Self::AbilityTargetReferenceRequiresSingular {
                definition,
                part,
                ability,
                target,
                maximum,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} reads one value from target {target:?}, but that slot allows up to {maximum} targets"
            ),
            Self::AbilityEffectRecipientKindMismatch {
                definition,
                part,
                ability,
                recipient,
                expected,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} uses {recipient:?} where the effect requires an {expected:?} recipient"
            ),
            Self::InvalidAbilityScalarChoice {
                definition,
                part,
                ability,
                list,
                destination,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} stores a {list:?} choice in the incompatible {destination:?} destination"
            ),
            Self::UnsupportedStaticAbilityPlayerRecipient {
                definition,
                part,
                ability,
                recipient,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} uses {recipient:?} for a static player rule, but it cannot be resolved from the static source"
            ),
            Self::InvalidAbilityObjectChoiceBounds {
                definition,
                part,
                ability,
                binding,
                minimum,
                maximum,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} binds {binding:?} from a choice requiring at least {minimum} objects and allowing at most {maximum}"
            ),
            Self::InvalidAbilityPaymentPayer {
                definition,
                part,
                ability,
                players,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} uses {players:?} for an effect payment, but a payment must select at most one player"
            ),
            Self::AbilityObjectBindingReferenceOutOfScope {
                definition,
                part,
                ability,
                binding,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} references object binding {binding:?} outside its scope"
            ),
            Self::AbilityObjectBindingAlreadyInScope {
                definition,
                part,
                ability,
                binding,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} binds object slot {binding:?} more than once in the same scope"
            ),
            Self::AbilityObjectSetBindingReferenceOutOfScope {
                definition,
                part,
                ability,
                binding,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} references object-set binding {binding:?} outside its scope"
            ),
            Self::AbilityObjectSetBindingAlreadyInScope {
                definition,
                part,
                ability,
                binding,
            } => write!(
                formatter,
                "ability {ability:?} on part {part:?} of card definition {definition:?} binds object-set slot {binding:?} more than once in the same scope"
            ),
            Self::DuplicateStructurePart { definition, part } => write!(
                formatter,
                "card definition {definition:?}'s structure references part {part:?} more than once"
            ),
            Self::InvalidSplitPartCount { definition, actual } => write!(
                formatter,
                "split card definition {definition:?} must contain at least two ordered parts, but contains {actual}"
            ),
            Self::UndefinedStructurePart { definition, part } => write!(
                formatter,
                "card definition {definition:?}'s structure references undefined part {part:?}"
            ),
            Self::PartOutsideStructure { definition, part } => write!(
                formatter,
                "card definition {definition:?} defines part {part:?}, but its structure does not contain that part"
            ),
            Self::DuplicatePlayOptionId { definition, option } => write!(
                formatter,
                "card definition {definition:?} defines play option {option:?} more than once"
            ),
            Self::EmptySpellForm { definition, option } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} has an empty combined spell form"
            ),
            Self::DuplicateSpellFormPart {
                definition,
                option,
                part,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} references part {part:?} more than once in its spell form"
            ),
            Self::UndefinedSpellFormPart {
                definition,
                option,
                part,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} references undefined spell-form part {part:?}"
            ),
            Self::SpellFormPartOutsideStructure {
                definition,
                option,
                part,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} uses part {part:?}, which is not in the card's structure"
            ),
            Self::MissingFusedPlayOption { definition, option } => write!(
                formatter,
                "split card definition {definition:?} names missing fused play option {option:?}"
            ),
            Self::InvalidFusedPlayOption {
                definition,
                option,
                expected,
                actual,
                actual_action,
            } => write!(
                formatter,
                "fused play option {option:?} of card definition {definition:?} must cast combined parts {expected:?} in printed order, but has action {actual_action:?} and form {actual:?}"
            ),
            Self::UnexpectedCombinedSpellForm { definition, option } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} has a combined spell form but is not its declared fused split option"
            ),
            Self::CombinedModalSpellUnsupported {
                definition,
                option,
                part,
            } => write!(
                formatter,
                "combined play option {option:?} of card definition {definition:?} includes modal part {part:?}, but combined mode selections are not part-scoped"
            ),
            Self::DuplicateModeId {
                definition,
                option,
                mode,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} defines mode {mode:?} more than once"
            ),
            Self::NonPositionalModeId {
                definition,
                option,
                expected,
                actual,
            } => write!(
                formatter,
                "mode position {expected:?} in play option {option:?} of card definition {definition:?} uses ID {actual:?}; mode IDs must match printed position"
            ),
            Self::EmptyModeSet { definition, option } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} has a mode set with no modes"
            ),
            Self::TooManyModes {
                definition,
                option,
                count,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} defines {count} modes, but positional mode IDs support at most 256"
            ),
            Self::InvalidModeBounds {
                definition,
                option,
                minimum,
                maximum,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} requires at least {minimum} modes but allows at most {maximum}"
            ),
            Self::ZeroModeMaximum { definition, option } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} has modes but allows none to be selected"
            ),
            Self::TooManyModesWithoutRepetition {
                definition,
                option,
                maximum,
                available,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} allows {maximum} modes without repetition but defines only {available}"
            ),
            Self::UnexpectedPresentationSpellModes { definition, option } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} presents mode choices for an executable nonmodal spell"
            ),
            Self::MissingPresentationSpellTarget {
                definition,
                option,
                target,
            } => write!(
                formatter,
                "semantic target {target:?} has no presentation counterpart in play option {option:?} of card definition {definition:?}"
            ),
            Self::MissingSemanticSpellTarget {
                definition,
                option,
                target,
            } => write!(
                formatter,
                "presentation target {target:?} has no semantic counterpart in play option {option:?} of card definition {definition:?}"
            ),
            Self::MismatchedSpellTargetCardinality {
                definition,
                option,
                target,
                presentation_minimum,
                presentation_maximum,
                semantic_minimum,
                semantic_maximum,
            } => write!(
                formatter,
                "target {target:?} in play option {option:?} of card definition {definition:?} has presentation cardinality {presentation_minimum}..={presentation_maximum} but semantic cardinality {semantic_minimum}..={semantic_maximum}"
            ),
            Self::UnpresentableSpellTarget {
                definition,
                option,
                target,
            } => write!(
                formatter,
                "semantic target {target:?} in play option {option:?} of card definition {definition:?} cannot be represented by the presentation target vocabulary"
            ),
            Self::MismatchedSpellTargetPresentation {
                definition,
                option,
                position,
                presentation,
                semantic,
            } => write!(
                formatter,
                "target at position {position} in play option {option:?} of card definition {definition:?} presents {presentation:?} but its semantic target projects to {semantic:?}"
            ),
            Self::UnexpectedModalSpellTargets {
                definition,
                option,
                count,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} presents {count} top-level targets for a semantic modal spell; targets must belong to its mode branches"
            ),
            Self::MissingPresentationSpellMode {
                definition,
                option,
                mode,
            } => write!(
                formatter,
                "semantic spell mode {mode:?} has no presentation counterpart in play option {option:?} of card definition {definition:?}"
            ),
            Self::MissingSemanticSpellMode {
                definition,
                option,
                mode,
            } => write!(
                formatter,
                "presentation mode {mode:?} has no semantic spell counterpart in play option {option:?} of card definition {definition:?}"
            ),
            Self::MissingPresentationSpellModeTarget {
                definition,
                option,
                mode,
                target,
            } => write!(
                formatter,
                "semantic target {target:?} of spell mode {mode:?} has no presentation counterpart in play option {option:?} of card definition {definition:?}"
            ),
            Self::MissingSemanticSpellModeTarget {
                definition,
                option,
                mode,
                target,
            } => write!(
                formatter,
                "presentation target {target:?} of spell mode {mode:?} has no semantic counterpart in play option {option:?} of card definition {definition:?}"
            ),
            Self::MismatchedSpellModeTargetCardinality {
                definition,
                option,
                mode,
                target,
                presentation_minimum,
                presentation_maximum,
                semantic_minimum,
                semantic_maximum,
            } => write!(
                formatter,
                "target {target:?} of spell mode {mode:?} in play option {option:?} of card definition {definition:?} has presentation cardinality {presentation_minimum}..={presentation_maximum} but semantic cardinality {semantic_minimum}..={semantic_maximum}"
            ),
            Self::UnpresentableSpellModeTarget {
                definition,
                option,
                mode,
                target,
            } => write!(
                formatter,
                "semantic target {target:?} of spell mode {mode:?} in play option {option:?} of card definition {definition:?} cannot be represented by the presentation target vocabulary"
            ),
            Self::MismatchedSpellModeTargetPresentation {
                definition,
                option,
                mode,
                position,
                presentation,
                semantic,
            } => write!(
                formatter,
                "target at position {position} of spell mode {mode:?} in play option {option:?} of card definition {definition:?} presents {presentation:?} but its semantic target projects to {semantic:?}"
            ),
            Self::MismatchedSpellModeSelection {
                definition,
                option,
                presentation_minimum,
                presentation_maximum,
                presentation_may_repeat,
                semantic_minimum,
                semantic_maximum,
                semantic_may_repeat,
            } => write!(
                formatter,
                "spell modes in play option {option:?} of card definition {definition:?} present {presentation_minimum}..={presentation_maximum} (repeat={presentation_may_repeat}) but declare {semantic_minimum}..={semantic_maximum} (repeat={semantic_may_repeat})"
            ),
            Self::MismatchedSpellModeImplementation {
                definition,
                option,
                mode,
                presentation,
                semantic,
            } => write!(
                formatter,
                "spell mode {mode:?} in play option {option:?} of card definition {definition:?} presents {presentation:?} but its semantic branch is {semantic:?}"
            ),
            Self::MismatchedSpellModeLabel {
                definition,
                option,
                mode,
                presentation,
                semantic,
            } => write!(
                formatter,
                "spell mode {mode:?} in play option {option:?} of card definition {definition:?} is labeled {presentation:?} but its semantic branch is labeled {semantic:?}"
            ),
            Self::DuplicateAlternativeCostId {
                definition,
                option,
                cost,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} defines alternative cost {cost:?} more than once"
            ),
            Self::MissingAlternativeCostForAbility {
                definition,
                part,
                ability,
                cost,
            } => write!(
                formatter,
                "alternative-cast ability {ability:?} on part {part:?} of card definition {definition:?} references missing cost {cost:?}"
            ),
            Self::MismatchedAlternativeCostForAbility(mismatch) => {
                let MismatchedAlternativeCost {
                    definition,
                    part,
                    ability,
                    option,
                    cost,
                    expected_label,
                    actual_label,
                    expected_mana_cost,
                    actual_mana_cost,
                } = mismatch.as_ref();
                write!(
                    formatter,
                    "alternative cost {cost:?} on play option {option:?}, projected from ability {ability:?} on part {part:?} of card definition {definition:?}, must be labeled {expected_label:?} with mana cost {expected_mana_cost}, but is labeled {actual_label:?} with mana cost {actual_mana_cost}"
                )
            }
            Self::MissingAdditionalCostForAbility {
                definition,
                part,
                ability,
                cost,
            } => write!(
                formatter,
                "optional additional-cost ability {ability:?} on part {part:?} of card definition {definition:?} references missing cost {cost:?}"
            ),
            Self::MismatchedAdditionalCostForAbility(mismatch) => {
                let MismatchedAdditionalCost {
                    definition,
                    part,
                    ability,
                    option,
                    cost,
                    expected_label,
                    actual_label,
                    expected_mana_cost,
                    actual_mana_cost,
                } = mismatch.as_ref();
                write!(
                    formatter,
                    "additional cost {cost:?} on play option {option:?}, projected from ability {ability:?} on part {part:?} of card definition {definition:?}, must be labeled {expected_label:?} with mana cost {expected_mana_cost:?}, but is labeled {actual_label:?} with mana cost {actual_mana_cost:?}"
                )
            }
            Self::DuplicateAdditionalCostId { definition, cost } => write!(
                formatter,
                "card definition {definition:?} defines additional cost {cost:?} more than once"
            ),
            Self::InvalidTargetBounds {
                definition,
                option,
                mode,
                slot,
                minimum,
                maximum,
            } => {
                if let Some(mode) = mode {
                    write!(
                        formatter,
                        "target slot {slot:?} in mode {mode:?} of play option {option:?} on card definition {definition:?} requires at least {minimum} targets but allows at most {maximum}"
                    )
                } else {
                    write!(
                        formatter,
                        "target slot {slot:?} in play option {option:?} of card definition {definition:?} requires at least {minimum} targets but allows at most {maximum}"
                    )
                }
            }
            Self::TooManyTargetSlots {
                definition,
                option,
                mode,
                count,
            } => write!(
                formatter,
                "{count} target slots are declared for mode {mode:?} of play option {option:?} on card definition {definition:?}, but positional target IDs support at most 256"
            ),
            Self::NonPositionalTargetSlot {
                definition,
                option,
                mode,
                expected,
                actual,
            } => write!(
                formatter,
                "target position {expected:?} in mode {mode:?} of play option {option:?} on card definition {definition:?} uses ID {actual:?}; target slot IDs must match instantiated order"
            ),
            Self::TooManyInstantiatedTargets {
                definition,
                option,
                count,
            } => write!(
                formatter,
                "play option {option:?} of card definition {definition:?} can instantiate {count} targets, but runtime target slot IDs support at most 256"
            ),
        }
    }
}

impl Error for CatalogError {}
