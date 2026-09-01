// What a clause claims about its own implementation.
//
// Split out of `ability_kinds.rs` for the source-size budget: everything next
// door describes what an ability does, and this describes how completely the
// engine does it. Included textually, so the imports here are that module's.

/// Clause-level implementation coverage, independent of effect dispatch.
///
/// An explanation is optional for a complete clause. Partial and
/// metadata-only clauses explain the remaining gap.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AbilityCoverageDef {
    pub status: ImplementationStatus,
    pub explanation: Option<&'static str>,
}

impl AbilityCoverageDef {
    #[must_use]
    pub const fn complete() -> Self {
        Self {
            status: ImplementationStatus::Complete,
            explanation: None,
        }
    }

    #[must_use]
    pub const fn explained_complete(explanation: &'static str) -> Self {
        Self {
            status: ImplementationStatus::Complete,
            explanation: Some(explanation),
        }
    }

    #[must_use]
    pub const fn partial(explanation: &'static str) -> Self {
        Self {
            status: ImplementationStatus::Partial,
            explanation: Some(explanation),
        }
    }

    #[must_use]
    pub const fn metadata_only(explanation: &'static str) -> Self {
        Self {
            status: ImplementationStatus::MetadataOnly,
            explanation: Some(explanation),
        }
    }

    #[must_use]
    pub const fn is_executable(self) -> bool {
        !matches!(self.status, ImplementationStatus::MetadataOnly)
    }
}
