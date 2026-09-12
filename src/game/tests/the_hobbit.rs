//! The Hobbit compositions, source lifetimes, and Adventure regressions.

use super::*;

fn board(library: &[CardDefinitionId]) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    for player in &mut game.players {
        player.hand.clear();
        player.graveyard.clear();
    }
    game.players[0].library = game.build_zone(PlayerId::One, library).unwrap();
    game.players[0].library.reverse();
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.turns_started = [3, 3];
    game.format = Format::IsdM14Standard;
    game.cards_drawn_this_turn = [0, 0];
    game
}

fn settle(game: &mut Game) {
    for _ in 0..64 {
        if let Some(pending) = game.pending_decisions.first() {
            let decision = pending.observation.clone();
            let options = if matches!(
                pending.continuation,
                DecisionContinuation::OptionalEffect { .. } | DecisionContinuation::PayOr { .. }
            ) {
                vec![decision.options.last().unwrap().id]
            } else {
                decision
                    .options
                    .iter()
                    .map(|o| o.id)
                    .take(decision.maximum)
                    .collect()
            };
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .unwrap();
        } else if game.stack.is_empty() && game.pending_triggers.is_empty() {
            return;
        } else {
            game.apply(game.priority, Action::PassPriority).unwrap();
        }
    }
    panic!("resolution did not settle");
}

fn held(game: &mut Game, definition: CardDefinitionId) -> GameObjectId {
    let cards = game.build_zone(PlayerId::One, &[definition]).unwrap();
    let id = cards[0].id;
    game.players[0].hand.extend(cards);
    for color in ManaColor::COLORS {
        game.add_unrestricted_mana(PlayerId::One, color, 10);
    }
    id
}

fn permanent(game: &Game, definition: CardDefinitionId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|p| p.card.definition == definition)
        .unwrap()
}

fn activate(game: &mut Game, id: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::ActivateAbility {source,..} if *source == id))
        .expect("ability is available");
    game.apply(PlayerId::One, action).unwrap();
    settle(game);
}

fn cast(game: &mut Game, definition: CardDefinitionId) {
    let id = held(game, definition);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a,Action::CastSpell{card,..} if *card==id))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(game);
}

#[test]
fn recruit_creates_a_soldier_only_for_a_discarded_nonland_card() {
    for (drawn, expected) in [(cards::FOREST, 0), (cards::ORDINARY_BEAR, 1)] {
        let mut game = board(&[drawn]);
        game.put_onto_battlefield(PlayerId::One, cards::LONG_LAKE_NUISANCE)
            .unwrap();
        settle(&mut game);
        assert_eq!(game.players[0].graveyard.len(), 1);
        assert_eq!(
            game.battlefield
                .iter()
                .filter(|p| p.card.definition.is_token())
                .count(),
            expected
        );
    }
}

#[test]
fn amass_reuses_one_army_and_plate_mail_attaches_to_that_army() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::GOBLIN_TOWN_FLUNKIES)
        .unwrap();
    settle(&mut game);
    let army = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap()
        .card
        .id;
    game.put_onto_battlefield(PlayerId::One, cards::GOBLIN_PLATE_MAIL)
        .unwrap();
    settle(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        1
    );
    let army = game.battlefield.iter().find(|p| p.card.id == army).unwrap();
    assert_eq!(army.counters(CounterKind::PlusOnePlusOne), 2);
    assert_eq!(game.power(army), Some(3));
    assert!(game.permanent_has_executable_keyword(army, KeywordAbility::Menace));
}

#[test]
fn ponies_split_two_found_lands_between_battlefield_and_hand() {
    let mut game = board(&[cards::FOREST, cards::ISLAND]);
    let ponies = game
        .put_onto_battlefield(PlayerId::One, cards::TROOP_OF_PONIES)
        .unwrap();
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == ponies)
        .unwrap()
        .entered_controller_turn = 0;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    activate(&mut game, ponies);
    assert!(game.players[0].library.is_empty());
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.battlefield.len(), 1);
    assert!(game.battlefield[0].tapped);
}

#[test]
fn bilbos_adventure_exiles_then_allows_the_creature_half() {
    let mut game = board(&[cards::FOREST; 3]);
    let id = held(&mut game, cards::BILBO_BAGGINS_BURGLAR);
    let action = game.legal_actions(PlayerId::One).into_iter().find(|a| matches!(a,
        Action::CastSpell {card,choices,..} if *card == id && choices.play_option() == PlayOptionId(1)
    )).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    let exiled = game.players[0].exile[0].id;
    let action = game.legal_actions(PlayerId::One).into_iter().find(|a| matches!(a,
        Action::CastSpell {card,choices,..} if *card == exiled && choices.play_option() == PlayOptionId::DEFAULT
    )).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert!(game.players[0].exile.is_empty());
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.battlefield.len(), 1);
}

#[test]
fn misty_mountains_sacrifices_itself_at_four_treasures_and_creates_a_dragon() {
    let mut game = board(&[]);
    for _ in 0..3 {
        game.put_onto_battlefield(PlayerId::One, cards::LONG_BODIED_GREY_DOG)
            .unwrap();
        settle(&mut game);
    }
    game.put_onto_battlefield(PlayerId::One, cards::THE_MISTY_MOUNTAINS_COLD)
        .unwrap();
    settle(&mut game);
    assert!(
        !game
            .battlefield
            .iter()
            .any(|p| p.card.definition == cards::THE_MISTY_MOUNTAINS_COLD)
    );
    let dragon = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token() && game.power(p) == Some(6))
        .unwrap();
    assert!(game.permanent_has_executable_keyword(dragon, KeywordAbility::Flying));
}

#[test]
fn notary_copies_are_nonlegendary_and_all_three_count_for_mana() {
    let mut game = board(&[]);
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::THE_NOTARY_HOBBITS)
        .unwrap();
    settle(&mut game);
    assert_eq!(game.battlefield.len(), 3);
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == source)
        .unwrap()
        .entered_controller_turn = 0;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| {
            matches!(a,
                Action::ActivateManaAbility {source:id,..} if *id == source
            )
        })
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[0].mana_pool.colorless, 3);
}

#[test]
fn silvan_reveler_returns_its_discarded_land_and_triggers_from_graveyard() {
    let mut game = board(&[cards::FOREST]);
    let elf = game
        .put_onto_battlefield(PlayerId::One, cards::SILVAN_REVELER)
        .unwrap();
    settle(&mut game);
    assert!(permanent(&game, cards::FOREST).tapped);
    game.move_target_to_zone(
        Target::Permanent(elf),
        ZoneKind::Graveyard,
        ZoneMoveCause::Effect {
            controller: PlayerId::One,
        },
        None,
        ZonePlacement::Top,
    );
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    game.put_onto_battlefield(PlayerId::One, cards::ISLAND)
        .unwrap();
    settle(&mut game);
    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|c| c.definition == cards::SILVAN_REVELER)
    );
}

#[test]
fn bard_doubles_extra_draws_and_recruit_tokens() {
    let mut game = board(&[cards::ORDINARY_BEAR; 6]);
    game.put_onto_battlefield(PlayerId::One, cards::BARD_KING_OF_DALE)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::LONG_LAKE_NUISANCE)
        .unwrap();
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.players[0].graveyard.len(), 1);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        2
    );
}

#[test]
fn queen_remembers_the_first_noncreature_cast_before_later_spells_resolve() {
    let mut game = board(&[cards::ORDINARY_BEAR; 5]);
    game.put_onto_battlefield(PlayerId::Two, cards::THE_QUEEN_OF_DALE)
        .unwrap();
    for _ in 0..2 {
        cast(&mut game, cards::SMAUG_S_FURY);
    }
    assert_eq!(game.players[1].graveyard.len(), 1);
}

#[test]
fn part_in_friendship_uses_the_revealed_creatures_value_and_randomizes_the_rest() {
    for (lands, battlefield) in [(2, false), (4, true)] {
        let mut game = board(&[cards::ISLAND, cards::ORDINARY_BEAR, cards::MOUNTAIN]);
        for _ in 0..lands {
            game.put_onto_battlefield(PlayerId::One, cards::FOREST)
                .unwrap();
        }
        game.put_onto_battlefield(PlayerId::One, cards::PART_IN_FRIENDSHIP)
            .unwrap();
        let dying = game
            .put_onto_battlefield(PlayerId::One, cards::DUSKWATCH_HUNTER)
            .unwrap();
        settle(&mut game);
        game.move_target_to_zone(
            Target::Permanent(dying),
            ZoneKind::Graveyard,
            ZoneMoveCause::Effect {
                controller: PlayerId::One,
            },
            None,
            ZonePlacement::Top,
        );
        settle(&mut game);
        assert_eq!(
            game.battlefield
                .iter()
                .any(|p| p.card.definition == cards::ORDINARY_BEAR),
            battlefield
        );
        assert_eq!(game.players[0].hand.len(), usize::from(!battlefield));
        assert_eq!(game.players[0].library.len(), 2);
    }
}

#[test]
fn rampager_uses_sacrificed_power_and_its_last_known_power_on_death() {
    let mut game = board(&[]);
    let rampager = game
        .put_onto_battlefield(PlayerId::One, cards::RHOVANION_RAMPAGER)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::ORDINARY_BEAR)
        .unwrap();
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == rampager)
        .unwrap()
        .entered_controller_turn = 0;
    game.step = Step::DeclareAttackers;
    game.declare_attacker(rampager, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    settle(&mut game);
    assert_eq!(
        permanent(&game, cards::RHOVANION_RAMPAGER).counters(CounterKind::PlusOnePlusOne),
        4
    );
    game.move_target_to_zone(
        Target::Permanent(rampager),
        ZoneKind::Graveyard,
        ZoneMoveCause::Effect {
            controller: PlayerId::One,
        },
        None,
        ZonePlacement::Top,
    );
    settle(&mut game);
    let army = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap();
    assert_eq!(army.counters(CounterKind::PlusOnePlusOne), 7);
}

#[test]
fn eagles_rescue_returns_from_graveyard_already_attached() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::GUARDIAN_OF_THE_HALLS)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::OLD_THRUSH)
        .unwrap();
    settle(&mut game);
    let aura = game
        .build_zone(PlayerId::One, &[cards::EAGLE_S_RESCUE])
        .unwrap();
    let id = aura[0].id;
    game.players[0].graveyard = aura;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 4);
    activate(&mut game, id);
    assert!(game.players[0].graveyard.is_empty());
    assert_eq!(game.power(permanent(&game, cards::OLD_THRUSH)), Some(3));
    assert_eq!(
        game.power(permanent(&game, cards::GUARDIAN_OF_THE_HALLS)),
        Some(2)
    );
}

#[test]
fn great_fierce_bee_excludes_its_own_death_but_observes_other_simultaneous_deaths() {
    for with_other in [false, true] {
        let mut game = board(&[cards::FOREST]);
        let bee = game
            .put_onto_battlefield(PlayerId::One, cards::GREAT_FIERCE_BEE)
            .unwrap();
        let mut dying = vec![bee];
        if with_other {
            dying.push(
                game.put_onto_battlefield(PlayerId::One, cards::ORDINARY_BEAR)
                    .unwrap(),
            );
        }
        game.destroy_permanents(&dying, false);
        assert_eq!(
            game.pending_triggers.len() + game.stack.len(),
            usize::from(with_other)
        );
        settle(&mut game);
    }
}
