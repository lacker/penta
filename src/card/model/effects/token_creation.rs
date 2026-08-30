use crate::card::{
    AbilityDef, CardArt, CardTypeSet, ManaColor, TokenCharacteristics, TokenCopyDef, TokenStatsDef,
};

use super::{CreatedTokensDef, EffectDef, PlayerRefDef, TokenCountersDef, ValueDef};

impl EffectDef {
    /// Creates one ordinary creature token. Its name is derived from all of
    /// `subtypes` in order; use [`Self::with_name`] only when the effect gives
    /// it a different name.
    #[must_use]
    pub const fn create_creature_token(
        subtypes: &'static [&'static str],
        colors: &'static [ManaColor],
        power: i16,
        toughness: i16,
    ) -> Self {
        Self::create_token(TokenCharacteristics::creature(
            subtypes, colors, power, toughness,
        ))
    }

    /// Creates one ordinary artifact creature token.
    #[must_use]
    pub const fn create_artifact_creature_token(
        subtypes: &'static [&'static str],
        colors: &'static [ManaColor],
        power: i16,
        toughness: i16,
    ) -> Self {
        Self::create_token(TokenCharacteristics::artifact_creature(
            subtypes, colors, power, toughness,
        ))
    }

    /// Creates one ordinary noncreature artifact token.
    #[must_use]
    pub const fn create_artifact_token(
        subtypes: &'static [&'static str],
        colors: &'static [ManaColor],
    ) -> Self {
        Self::create_token(TokenCharacteristics::artifact(subtypes, colors))
    }

    /// Creates one token with explicit characteristics and ordinary entry
    /// defaults.
    #[must_use]
    pub const fn create_token(token: TokenCharacteristics) -> Self {
        Self::CreateToken {
            token,
            copy: None,
            controller: None,
            count: ValueDef::Constant(1),
            tapped: false,
            attacking: false,
            counters: None,
            linked_to_source: false,
            created: None,
        }
    }

    /// Creates one token whose base characteristics are copied from the
    /// named object before the token enters the battlefield.
    #[must_use]
    pub const fn create_token_from_copy(copy: &'static TokenCopyDef) -> Self {
        Self::CreateToken {
            token: TokenCharacteristics::new(CardTypeSet::EMPTY, &[], &[], None),
            copy: Some(copy),
            controller: None,
            count: ValueDef::Constant(1),
            tapped: false,
            attacking: false,
            counters: None,
            linked_to_source: false,
            created: None,
        }
    }

    /// Changes a literal token amount on a token-creation effect.
    ///
    /// # Panics
    ///
    /// Panics when called on an effect other than [`Self::CreateToken`].
    #[must_use]
    pub const fn with_amount(mut self, amount: u16) -> Self {
        let Self::CreateToken { count, .. } = &mut self else {
            panic!("with_amount() requires a token-creation effect");
        };
        *count = ValueDef::Constant(amount as i32);
        self
    }

    /// Changes a computed token count on a token-creation effect.
    ///
    /// # Panics
    ///
    /// Panics when called on an effect other than [`Self::CreateToken`].
    #[must_use]
    pub const fn with_count(mut self, value: ValueDef) -> Self {
        let Self::CreateToken { count, .. } = &mut self else {
            panic!("with_count() requires a token-creation effect");
        };
        *count = value;
        self
    }

    /// Overrides the ordinary subtype-derived token name.
    ///
    /// # Panics
    ///
    /// Panics when called on an effect that does not carry authored token
    /// characteristics.
    #[must_use]
    pub const fn with_name(mut self, name: &'static str) -> Self {
        let token = self.authored_token_mut();
        *token = (*token).with_name(name);
        self
    }

    /// "Create an X/X blue Illusion creature token." The size is a pair of
    /// amounts the creating effect works out rather than anything printed on
    /// the token.
    ///
    /// # Panics
    ///
    /// Panics when called on an effect that does not carry authored token
    /// characteristics.
    #[must_use]
    pub const fn with_variable_token_stats(mut self, stats: &'static TokenStatsDef) -> Self {
        let token = self.authored_token_mut();
        *token = (*token).with_variable_stats(stats);
        self
    }

    /// Adds creator-selected token art.
    ///
    /// # Panics
    ///
    /// Panics when called on an effect that does not carry authored token
    /// characteristics.
    #[must_use]
    pub const fn with_art(mut self, art: CardArt) -> Self {
        let token = self.authored_token_mut();
        *token = (*token).with_art(art);
        self
    }

    /// Supplies the token's ordered printed abilities.
    ///
    /// # Panics
    ///
    /// Panics when called on an effect that does not carry authored token
    /// characteristics.
    #[must_use]
    pub const fn with_abilities(mut self, abilities: &'static [AbilityDef]) -> Self {
        let token = self.authored_token_mut();
        *token = (*token).with_abilities(abilities);
        self
    }

    /// Gives the created tokens to an explicitly named controller.
    ///
    /// # Panics
    ///
    /// Panics when called on an effect other than [`Self::CreateToken`].
    #[must_use]
    pub const fn with_controller(mut self, player: PlayerRefDef) -> Self {
        let Self::CreateToken { controller, .. } = &mut self else {
            panic!("with_controller() requires a token-creation effect");
        };
        *controller = Some(player);
        self
    }

    /// Makes the created tokens enter tapped.
    ///
    /// # Panics
    ///
    /// Panics when called on an effect other than [`Self::CreateToken`].
    #[must_use]
    pub const fn entering_tapped(mut self) -> Self {
        let Self::CreateToken { tapped, .. } = &mut self else {
            panic!("entering_tapped() requires a token-creation effect");
        };
        *tapped = true;
        self
    }

    /// Makes the created tokens enter attacking.
    ///
    /// # Panics
    ///
    /// Panics when called on an effect other than [`Self::CreateToken`].
    #[must_use]
    pub const fn entering_attacking(mut self) -> Self {
        let Self::CreateToken { attacking, .. } = &mut self else {
            panic!("entering_attacking() requires a token-creation effect");
        };
        *attacking = true;
        self
    }

    /// Makes each token enter carrying the described counters.
    ///
    /// # Panics
    ///
    /// Panics when called on an effect other than [`Self::CreateToken`].
    #[must_use]
    pub const fn with_counters(mut self, value: TokenCountersDef) -> Self {
        let Self::CreateToken { counters, .. } = &mut self else {
            panic!("with_counters() requires a token-creation effect");
        };
        *counters = Some(value);
        self
    }

    /// Links each created token to the source of the resolving spell or
    /// ability. The token may then use source-linked values such as
    /// [`ValueDef::CountersOnCreator`] in its own rules.
    ///
    /// # Panics
    ///
    /// Panics when called on an effect other than [`Self::CreateToken`].
    #[must_use]
    pub const fn linked_to_source(mut self) -> Self {
        let Self::CreateToken {
            linked_to_source, ..
        } = &mut self
        else {
            panic!("linked_to_source() requires a token-creation effect");
        };
        *linked_to_source = true;
        self
    }

    /// Records the created objects for a token-linked follow-up.
    ///
    /// # Panics
    ///
    /// Panics when called on an effect other than [`Self::CreateToken`].
    #[must_use]
    pub const fn with_created_tokens(mut self, value: CreatedTokensDef) -> Self {
        let Self::CreateToken { created, .. } = &mut self else {
            panic!("with_created_tokens() requires a token-creation effect");
        };
        *created = Some(value);
        self
    }

    const fn authored_token_mut(&mut self) -> &mut TokenCharacteristics {
        match self {
            Self::CreateToken {
                token, copy: None, ..
            }
            | Self::CreateAttachedToken { token, .. } => token,
            _ => panic!("token modifier requires authored token characteristics"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::abilities;

    const KRENKOS_COMMAND: EffectDef =
        EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1).with_amount(2);
    const LINKED_COMMAND: EffectDef = KRENKOS_COMMAND.linked_to_source();

    const WURMCOIL_TOKENS: [EffectDef; 2] = [
        EffectDef::create_artifact_creature_token(&["Phyrexian", "Wurm"], &[], 3, 3)
            .with_abilities(&[abilities::deathtouch()]),
        EffectDef::create_artifact_creature_token(&["Phyrexian", "Wurm"], &[], 3, 3)
            .with_abilities(&[abilities::lifelink()]),
    ];

    #[test]
    fn common_token_factory_supplies_defaults_and_modifiers() {
        let EffectDef::CreateToken {
            token,
            controller,
            count,
            tapped,
            attacking,
            counters,
            linked_to_source,
            created,
            copy: _,
        } = KRENKOS_COMMAND
        else {
            panic!("the creature-token factory should create a token effect");
        };
        assert_eq!(token.name(), "Goblin");
        assert_eq!(
            token.rules().creature_stats().map(|stats| stats.power),
            Some(1)
        );
        assert!(token.rules().ability_clauses().is_empty());
        assert_eq!(controller, None);
        assert_eq!(count, ValueDef::Constant(2));
        assert!(!tapped);
        assert!(!attacking);
        assert_eq!(counters, None);
        assert!(!linked_to_source);
        assert_eq!(created, None);

        let EffectDef::CreateToken {
            linked_to_source, ..
        } = LINKED_COMMAND
        else {
            unreachable!()
        };
        assert!(linked_to_source);
    }

    #[test]
    fn artifact_creature_factory_covers_wurmcoil_tokens_without_named_globals() {
        let tokens = WURMCOIL_TOKENS.map(|effect| {
            let EffectDef::CreateToken { token, .. } = effect else {
                panic!("the artifact-creature factory should create a token effect");
            };
            token
        });

        for token in tokens {
            assert_eq!(token.name(), "Phyrexian Wurm");
            assert!(token.rules().has_type(crate::card::CardType::Artifact));
            assert!(token.rules().has_type(crate::card::CardType::Creature));
        }
        assert_eq!(
            tokens[0].rules().ability_clauses(),
            &[abilities::deathtouch()],
        );
        assert_eq!(
            tokens[1].rules().ability_clauses(),
            &[abilities::lifelink()],
        );
        assert_ne!(
            tokens[0].rules().ability_clauses(),
            tokens[1].rules().ability_clauses(),
        );
    }
}
