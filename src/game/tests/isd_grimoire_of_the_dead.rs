//! Grimoire of the Dead's discard-and-study setup and graveyard-wide return.

use super::*;

const STUDY_COUNTER: CounterKind = CounterKind::named("study");

fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    game.players[1].graveyard.clear();
    let grimoire = game
        .put_onto_battlefield(PlayerId::One, cards::GRIMOIRE_OF_THE_DEAD)
        .expect("Grimoire of the Dead is cataloged");
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, grimoire)
}

fn activation(game: &Game, source: GameObjectId, ability: usize) -> Option<Action> {
    let origin = activated_ability_for(game, source, ability);
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility {
                    source: actual,
                    ability: actual_origin,
                    ..
                } if *actual == source && *actual_origin == origin
            )
        })
}

fn permanent(game: &Game, definition: CardDefinitionId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == definition)
        .expect("the creature returned to the battlefield")
}

#[test]
fn discarding_and_tapping_are_costs_before_the_study_counter_resolves() {
    let (mut game, grimoire) = staged();
    let discarded = card(90_001, cards::GRIZZLY_BEARS, PlayerId::One);
    let discarded_id = discarded.id;
    game.players[0].hand.push(discarded);
    game.players[0].mana_pool.colorless = 1;

    let action = activation(&game, grimoire, 0)
        .expect("one mana, an untapped Grimoire, and a card can pay the first ability");
    game.apply(PlayerId::One, action)
        .expect("the first ability activates");

    let source = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == grimoire)
        .expect("the first ability does not sacrifice Grimoire");
    assert!(source.tapped, "tapping is paid before resolution");
    assert_eq!(source.counters(STUDY_COUNTER), 0);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.id != discarded_id && card.definition == cards::GRIZZLY_BEARS),
        "discarding creates the graveyard-zone successor before resolution",
    );

    pass_priority_pair(&mut game);
    let source = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == grimoire)
        .expect("Grimoire remains");
    assert_eq!(source.counters(STUDY_COUNTER), 1);
}

#[test]
fn three_studies_return_creatures_from_every_graveyard_as_black_zombies() {
    let (mut game, grimoire) = staged();
    let ours = card(90_010, cards::GRIZZLY_BEARS, PlayerId::One);
    let theirs = card(90_011, cards::AIR_ELEMENTAL, PlayerId::Two);
    let noncreature = card(90_012, cards::LIGHTNING_BOLT, PlayerId::Two);
    game.players[0].graveyard.push(ours);
    game.players[1].graveyard.extend([theirs, noncreature]);

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == grimoire)
        .expect("Grimoire is on the battlefield")
        .set_counters(STUDY_COUNTER, 2);
    assert!(
        activation(&game, grimoire, 1).is_none(),
        "fewer than three study counters cannot pay the activation cost",
    );

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == grimoire)
        .expect("Grimoire is on the battlefield")
        .set_counters(STUDY_COUNTER, 3);
    let action = activation(&game, grimoire, 1)
        .expect("three study counters make the second ability payable");
    game.apply(PlayerId::One, action)
        .expect("the second ability activates");
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != grimoire),
        "Grimoire is sacrificed as a cost",
    );

    pass_priority_pair(&mut game);

    let bear = permanent(&game, cards::GRIZZLY_BEARS);
    let elemental = permanent(&game, cards::AIR_ELEMENTAL);
    for returned in [bear, elemental] {
        assert_eq!(returned.controller, PlayerId::One);
        assert!(game.effective_subtypes(returned).contains(&"Zombie"));
        let colors = game.effective_colors(returned, &game.effective_rules(returned).unwrap());
        assert!(colors[ManaColor::Black.index()], "the creature is black");
    }
    assert!(game.effective_subtypes(bear).contains(&"Bear"));
    assert!(game.effective_subtypes(elemental).contains(&"Elemental"));
    assert!(
        game.effective_colors(bear, &game.effective_rules(bear).unwrap())[ManaColor::Green.index()],
        "the returned Bear keeps green",
    );
    assert!(
        game.effective_colors(elemental, &game.effective_rules(elemental).unwrap())
            [ManaColor::Blue.index()],
        "the returned Elemental keeps blue",
    );
    assert!(
        game.players[1]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "noncreature cards stay in their graveyards",
    );
}
