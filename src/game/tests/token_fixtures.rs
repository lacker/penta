//! Compact token fixtures whose printed abilities are material to a test.

use super::*;

static ASSASSIN_ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered(
    "Whenever this token deals combat damage to a player, that player loses the game.",
    TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
    EffectDef::LoseTheGame {
        player: EffectRecipientDef::EventPlayer,
    },
)];

pub(in crate::game) const fn assassin_token() -> TokenCharacteristics {
    tokens::creature(&["Assassin"], &[ManaColor::Black], 1, 1).with_abilities(&ASSASSIN_ABILITIES)
}

static VOICE_CREATURES_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);
static VOICE_ELEMENTAL_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
    "This token's power and toughness are each equal to the number of creatures you control.",
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::modify_power_toughness(
            ValueDef::CountMatchingObjects(&VOICE_CREATURES_YOU_CONTROL),
            ValueDef::CountMatchingObjects(&VOICE_CREATURES_YOU_CONTROL),
        ),
    },
)];

pub(in crate::game) const fn voice_elemental_token() -> TokenCharacteristics {
    tokens::creature(&["Elemental"], &[ManaColor::Green, ManaColor::White], 0, 0)
        .with_abilities(&VOICE_ELEMENTAL_ABILITIES)
}

static WOLVES_OF_THE_HUNT_ABILITIES: [AbilityDef; 1] =
    [abilities::bands_with_other(BandingQuality::WolvesOfTheHunt)];

pub(in crate::game) const fn wolves_of_the_hunt_token() -> TokenCharacteristics {
    tokens::creature(&["Wolf"], &[ManaColor::Green], 1, 1)
        .with_name("Wolves of the Hunt")
        .with_abilities(&WOLVES_OF_THE_HUNT_ABILITIES)
}

static POISONOUS_SNAKE_ABILITIES: [AbilityDef; 1] = [abilities::poisonous_damage(
    1,
    "Whenever this creature deals damage to a player, that player gets a poison counter.",
)];

pub(in crate::game) const fn poisonous_snake_token() -> TokenCharacteristics {
    tokens::artifact_creature(&["Snake"], &[], 1, 1).with_abilities(&POISONOUS_SNAKE_ABILITIES)
}

static FLYING_TOKEN_ABILITIES: [AbilityDef; 1] = [abilities::flying()];
static HASTE_TOKEN_ABILITIES: [AbilityDef; 1] = [abilities::haste()];
static MENACE_TOKEN_ABILITIES: [AbilityDef; 1] = [abilities::menace()];
static PROWESS_TOKEN_ABILITIES: [AbilityDef; 1] = [abilities::prowess()];
static TRAMPLE_TOKEN_ABILITIES: [AbilityDef; 1] = [abilities::trample()];
static VIGILANCE_TOKEN_ABILITIES: [AbilityDef; 1] = [abilities::vigilance()];

pub(in crate::game) const fn token_with_flying(
    token: TokenCharacteristics,
) -> TokenCharacteristics {
    token.with_abilities(&FLYING_TOKEN_ABILITIES)
}

pub(in crate::game) const fn token_with_haste(token: TokenCharacteristics) -> TokenCharacteristics {
    token.with_abilities(&HASTE_TOKEN_ABILITIES)
}

pub(in crate::game) const fn token_with_menace(
    token: TokenCharacteristics,
) -> TokenCharacteristics {
    token.with_abilities(&MENACE_TOKEN_ABILITIES)
}

pub(in crate::game) const fn token_with_prowess(
    token: TokenCharacteristics,
) -> TokenCharacteristics {
    token.with_abilities(&PROWESS_TOKEN_ABILITIES)
}

pub(in crate::game) const fn token_with_trample(
    token: TokenCharacteristics,
) -> TokenCharacteristics {
    token.with_abilities(&TRAMPLE_TOKEN_ABILITIES)
}

pub(in crate::game) const fn token_with_vigilance(
    token: TokenCharacteristics,
) -> TokenCharacteristics {
    token.with_abilities(&VIGILANCE_TOKEN_ABILITIES)
}
