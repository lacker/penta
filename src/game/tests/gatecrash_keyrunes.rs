//! The four Gatecrash Keyrunes.
//!
//! Each is a mana rock that can turn itself into a creature for a turn. The
//! animation was never the gap: the Return to Ravnica half of the cycle has
//! shipped with this exact shape all along, and these four audit lines had
//! simply not caught up with it.

use super::*;

/// Definition, its two colours, the body it animates into, its creature
/// type, and the keyword the animation grants.
struct Keyrune {
    definition: CardDefinitionId,
    colors: [ManaColor; 2],
    body: (i16, i16),
    subtype: &'static str,
    keyword: KeywordAbility,
}

const fn keyrune(
    definition: CardDefinitionId,
    colors: [ManaColor; 2],
    body: (i16, i16),
    subtype: &'static str,
    keyword: KeywordAbility,
) -> Keyrune {
    Keyrune {
        definition,
        colors,
        body,
        subtype,
        keyword,
    }
}

const KEYRUNES: [Keyrune; 4] = [
    keyrune(
        cards::BOROS_KEYRUNE,
        [ManaColor::Red, ManaColor::White],
        (1, 1),
        "Soldier",
        KeywordAbility::DoubleStrike,
    ),
    keyrune(
        cards::GRUUL_KEYRUNE,
        [ManaColor::Red, ManaColor::Green],
        (3, 2),
        "Beast",
        KeywordAbility::Trample,
    ),
    keyrune(
        cards::ORZHOV_KEYRUNE,
        [ManaColor::White, ManaColor::Black],
        (1, 4),
        "Thrull",
        KeywordAbility::Lifelink,
    ),
    keyrune(
        cards::SIMIC_KEYRUNE,
        [ManaColor::Green, ManaColor::Blue],
        (2, 3),
        "Crab",
        KeywordAbility::Hexproof,
    ),
];

fn board(definition: CardDefinitionId) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    let mut permanent = creature(10_000, definition, PlayerId::One);
    permanent.entered_controller_turn = 0;
    let id = permanent.card.id;
    game.battlefield.push(permanent);
    (game, id)
}

/// Each taps for either of its two colours and nothing else.
#[test]
fn each_keyrune_taps_for_its_own_two_colours() {
    for Keyrune {
        definition, colors, ..
    } in KEYRUNES
    {
        let (game, id) = board(definition);
        let offered = game
            .mana_ability_activations(
                game.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == id)
                    .expect("still there"),
            )
            .into_iter()
            .map(|activation| activation.color)
            .collect::<Vec<_>>();
        assert_eq!(
            offered,
            colors.to_vec(),
            "{definition:?} taps for its guild"
        );
    }
}

/// Before animating it is an artifact and not a creature; afterwards it is
/// both, with the printed body, type, colours, and keyword.
#[test]
fn animating_gives_the_printed_body_type_and_keyword() {
    for Keyrune {
        definition,
        colors,
        body: (power, toughness),
        subtype,
        keyword,
    } in KEYRUNES
    {
        let (mut game, id) = board(definition);
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("still there");
        assert_eq!(
            game.power(permanent),
            None,
            "{definition:?} is no creature to begin with",
        );

        for color in colors {
            game.players[PlayerId::One.index()]
                .mana_pool
                .add_color(color, 1);
        }
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(
                |action| matches!(action, Action::ActivateAbility { source, .. } if *source == id),
            )
            .expect("the animation is affordable");
        game.apply(PlayerId::One, action).expect("it is activated");
        drain_pending(&mut game);

        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("still there");
        assert_eq!(
            (game.power(permanent), game.toughness(permanent)),
            (Some(power), Some(toughness)),
            "{definition:?} takes its printed body",
        );
        assert!(
            game.permanent_types(permanent)
                .is_some_and(|types| types.contains(CardType::Artifact)),
            "{definition:?} is still an artifact",
        );
        assert!(
            game.object_subtypes(id).contains(&subtype),
            "{definition:?} becomes a {subtype}",
        );
        assert!(
            game.permanent_has_executable_keyword(permanent, keyword),
            "{definition:?} gains {keyword:?}",
        );
        for color in colors {
            assert!(
                color
                    .color_index()
                    .is_some_and(|index| game.permanent_colors(permanent)[index]),
                "{definition:?} takes its guild's colours",
            );
        }
    }
}
