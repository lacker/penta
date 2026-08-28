use super::{Game, PlayerId, Target, TargetIndex, TargetSelection, fmt};

/// One card-owned stack-ability entry point. Equality and hashing use the
/// stable key rather than a function address so frozen rules remain
/// deterministic across builds and platforms.
#[derive(Clone, Copy)]
pub(crate) struct CardAbilityResolver {
    key: &'static str,
    start: for<'game> fn(&mut CardRuntime<'game>, &ResolvedAbility),
}

impl CardAbilityResolver {
    #[must_use]
    #[allow(dead_code)] // Card-owned resolution remains an extension boundary.
    pub(crate) const fn new(
        key: &'static str,
        start: for<'game> fn(&mut CardRuntime<'game>, &ResolvedAbility),
    ) -> Self {
        Self { key, start }
    }

    pub(super) fn resolve(self, runtime: &mut CardRuntime<'_>, ability: &ResolvedAbility) {
        (self.start)(runtime, ability);
    }
}

impl fmt::Debug for CardAbilityResolver {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CardAbilityResolver")
            .field("key", &self.key)
            .finish_non_exhaustive()
    }
}

impl PartialEq for CardAbilityResolver {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for CardAbilityResolver {}

impl std::hash::Hash for CardAbilityResolver {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.key, state);
    }
}

/// Frozen public facts supplied to a card-owned resolver after target fizzle
/// checking has completed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ResolvedAbility {
    pub(super) controller: PlayerId,
    pub(super) targets: Vec<TargetSelection>,
}

impl ResolvedAbility {
    #[must_use]
    #[allow(dead_code)] // Card-owned resolution remains an extension boundary.
    pub(crate) const fn controller(&self) -> PlayerId {
        self.controller
    }

    #[must_use]
    #[allow(dead_code)] // Card-owned resolution remains an extension boundary.
    pub(crate) fn target_player(&self, index: TargetIndex) -> Option<PlayerId> {
        self.targets.get(index.index()).and_then(|selection| {
            selection.targets().iter().find_map(|target| match target {
                Target::Player(player) => Some(*player),
                Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
            })
        })
    }
}

/// Narrow capability surface available to card-owned resolution callbacks.
/// Its game reference is private so set modules cannot mutate unrelated state.
pub(crate) struct CardRuntime<'game> {
    #[allow(dead_code)] // Available to card-owned resolvers when that extension boundary is used.
    pub(super) game: &'game mut Game,
}
