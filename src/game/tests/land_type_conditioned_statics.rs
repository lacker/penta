//! Static abilities gated by control of a particular basic land type.

use super::*;

fn stats(game: &Game, id: CardInstanceId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the creature is still on the battlefield");
    (game.power(permanent), game.toughness(permanent))
}

#[test]
fn land_type_bonuses_follow_the_controller_s_board() {
    for (definition, land, printed, with_land) in [
        (
            cards::KIRD_APE,
            cards::FOREST,
            (Some(1), Some(1)),
            (Some(2), Some(3)),
        ),
        (
            cards::SEDGE_TROLL,
            cards::SWAMP,
            (Some(2), Some(2)),
            (Some(3), Some(3)),
        ),
        (
            cards::MIRE_KAVU,
            cards::SWAMP,
            (Some(3), Some(2)),
            (Some(4), Some(3)),
        ),
    ] {
        let mut game = ready_game();
        let subject = CardInstanceId(10_000);
        game.battlefield
            .push(creature(subject.0, definition, PlayerId::One));
        game.battlefield.push(creature(10_001, land, PlayerId::Two));
        assert_eq!(
            stats(&game, subject),
            printed,
            "an opponent's land does not satisfy the condition",
        );

        game.battlefield.push(creature(10_002, land, PlayerId::One));
        assert_eq!(stats(&game, subject), with_land);

        game.battlefield.push(creature(10_003, land, PlayerId::One));
        assert_eq!(
            stats(&game, subject),
            with_land,
            "presence is a predicate, not a count",
        );
    }
}

#[test]
fn dire_wolves_gains_banding_only_while_its_controller_has_a_plains() {
    let mut game = ready_game();
    let wolves = CardInstanceId(10_000);
    game.battlefield
        .push(creature(wolves.0, cards::DIRE_WOLVES, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::PLAINS, PlayerId::Two));

    let has_banding = |game: &Game| {
        let wolves = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == wolves)
            .expect("Dire Wolves remains on the battlefield");
        game.permanent_has_executable_keyword(wolves, KeywordAbility::Banding)
    };
    assert!(!has_banding(&game));

    game.battlefield
        .push(creature(10_002, cards::PLAINS, PlayerId::One));
    assert!(has_banding(&game));

    game.battlefield[2].text_changes.push(BasicLandTypeChange {
        from: BasicLandType::Plains,
        to: BasicLandType::Island,
    });
    assert!(
        !has_banding(&game),
        "the condition reads the land's effective type",
    );
}

#[test]
fn tek_reads_each_basic_land_type_independently() {
    let mut game = ready_game();
    let tek = CardInstanceId(10_000);
    game.battlefield
        .push(creature(tek.0, cards::TEK, PlayerId::One));
    for (offset, land) in [
        cards::PLAINS,
        cards::ISLAND,
        cards::SWAMP,
        cards::MOUNTAIN,
        cards::FOREST,
    ]
    .into_iter()
    .enumerate()
    {
        game.battlefield.push(creature(
            10_001 + u32::try_from(offset).expect("five land types fit in u32"),
            land,
            PlayerId::One,
        ));
    }

    assert_eq!(stats(&game, tek), (Some(4), Some(4)));
    let tek = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == tek)
        .expect("Tek remains on the battlefield");
    for keyword in [
        KeywordAbility::Flying,
        KeywordAbility::FirstStrike,
        KeywordAbility::Trample,
    ] {
        assert!(
            game.permanent_has_executable_keyword(tek, keyword),
            "Tek gains {keyword:?}",
        );
    }
}

#[test]
fn stat_bonuses_are_conditionals_with_constant_modifiers() {
    let catalog = poc::catalog().expect("the catalog builds");
    for (definition, expected) in [
        (
            cards::KIRD_APE,
            (ValueDef::Constant(1), ValueDef::Constant(2)),
        ),
        (
            cards::SEDGE_TROLL,
            (ValueDef::Constant(1), ValueDef::Constant(1)),
        ),
        (
            cards::MIRE_KAVU,
            (ValueDef::Constant(1), ValueDef::Constant(1)),
        ),
    ] {
        let card = catalog.get(definition).expect("the card is cataloged");
        let effect = card.rules.ability_clauses()[0]
            .declarative_effect()
            .expect("the static ability has a declarative effect");
        let EffectDef::IfCondition { condition, then } = effect else {
            panic!("{} should express its land check as a condition", card.name);
        };
        assert!(matches!(condition, TriggerConditionDef::ObjectCount { .. }));
        let EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect:
                AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                    PowerToughnessOperationDef::Modify { power, toughness },
                )),
        } = *then
        else {
            panic!("{} should conditionally apply a stat modifier", card.name);
        };
        assert_eq!((power, toughness), expected);
    }
}
