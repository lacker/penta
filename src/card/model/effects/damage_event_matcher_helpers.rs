impl DamageEventMatcherDef {
    pub const ANY: Self = Self {
        kind: DamageKindDef::Any,
        source: DamageSourceMatcherDef::Any,
        recipient: DamageRecipientMatcherDef::Any,
    };

    pub const COMBAT: Self = Self {
        kind: DamageKindDef::Combat,
        source: DamageSourceMatcherDef::Any,
        recipient: DamageRecipientMatcherDef::Any,
    };

    #[must_use]
    pub const fn to(recipients: EffectRecipientDef) -> Self {
        Self {
            recipient: DamageRecipientMatcherDef::Recipients(recipients),
            ..Self::ANY
        }
    }

    #[must_use]
    pub const fn from(source: ObjectRefDef) -> Self {
        Self {
            source: DamageSourceMatcherDef::Object(source),
            ..Self::ANY
        }
    }

    #[must_use]
    pub const fn from_group_to(
        source: DamageSourceGroupDef,
        recipients: EffectRecipientDef,
    ) -> Self {
        Self {
            source: DamageSourceMatcherDef::Group(source),
            recipient: DamageRecipientMatcherDef::Recipients(recipients),
            ..Self::ANY
        }
    }

    #[must_use]
    pub const fn combat_to(recipients: EffectRecipientDef) -> Self {
        Self {
            recipient: DamageRecipientMatcherDef::Recipients(recipients),
            ..Self::COMBAT
        }
    }

    #[must_use]
    pub const fn combat_from(source: ObjectRefDef) -> Self {
        Self {
            source: DamageSourceMatcherDef::Object(source),
            ..Self::COMBAT
        }
    }

    #[must_use]
    pub const fn combat_except(source: ObjectRefDef) -> Self {
        Self {
            source: DamageSourceMatcherDef::Except(source),
            ..Self::COMBAT
        }
    }

    #[must_use]
    pub const fn to_player_and_creatures_controlled_by(player: PlayerRefDef) -> Self {
        Self {
            recipient: DamageRecipientMatcherDef::PlayerAndCreaturesControlledBy(player),
            ..Self::ANY
        }
    }

    #[must_use]
    pub const fn from_matching_to_affected(source: ObjectPredicateDef) -> Self {
        Self {
            kind: DamageKindDef::Any,
            source: DamageSourceMatcherDef::Matching(source),
            recipient: DamageRecipientMatcherDef::AffectedObject,
        }
    }

    pub const COMBAT_FROM_AFFECTED: Self = Self {
        kind: DamageKindDef::Combat,
        source: DamageSourceMatcherDef::AffectedObject,
        recipient: DamageRecipientMatcherDef::Any,
    };

    pub const COMBAT_TO_AFFECTED: Self = Self {
        kind: DamageKindDef::Combat,
        source: DamageSourceMatcherDef::Any,
        recipient: DamageRecipientMatcherDef::AffectedObject,
    };
}
