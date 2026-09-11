use crate::card::{CreatedTokensDef, PlayerRefDef, TokenCountersDef, TokenDef, ValueDef};

/// A token-creation instruction. Characteristics belong to its token source;
/// this operation owns quantity, recipient, entry conditions, and follow-ups.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CreateTokenDef {
    pub token: TokenDef,
    /// `None` means the resolving object's controller.
    pub controller: Option<PlayerRefDef>,
    pub count: ValueDef,
    /// Entry modifiers currently require a literal or bound source. The catalog
    /// rejects them for copy sources until the copy-entry path supports them.
    pub tapped: bool,
    pub attacking: bool,
    /// Counters received on entry are distinct from the token's characteristics.
    pub counters: Option<TokenCountersDef>,
    /// The actual created objects after replacements, bound as one batch.
    pub created: Option<CreatedTokensDef>,
}

impl CreateTokenDef {
    /// Creates one token under the resolving object's controller, with ordinary
    /// entry conditions and no follow-up.
    #[must_use]
    pub const fn new(token: TokenDef) -> Self {
        Self {
            token,
            controller: None,
            count: ValueDef::Constant(1),
            tapped: false,
            attacking: false,
            counters: None,
            created: None,
        }
    }

    #[must_use]
    pub const fn with_amount(self, amount: u16) -> Self {
        self.with_count(ValueDef::Constant(amount as i32))
    }

    #[must_use]
    pub const fn with_count(mut self, count: ValueDef) -> Self {
        self.count = count;
        self
    }

    #[must_use]
    pub const fn with_controller(mut self, controller: PlayerRefDef) -> Self {
        self.controller = Some(controller);
        self
    }

    #[must_use]
    pub const fn entering_tapped(mut self) -> Self {
        self.tapped = true;
        self
    }

    #[must_use]
    pub const fn entering_attacking(mut self) -> Self {
        self.attacking = true;
        self
    }

    #[must_use]
    pub const fn with_counters(mut self, counters: TokenCountersDef) -> Self {
        self.counters = Some(counters);
        self
    }

    #[must_use]
    pub const fn with_created_tokens(mut self, created: CreatedTokensDef) -> Self {
        self.created = Some(created);
        self
    }
}
