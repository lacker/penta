//! Behavior checks for cards implemented while seeding Duel Commander.
use super::*;

fn cast_named(game: &mut Game, definition: CardDefinitionId) {
    let spell = card(880_000, definition, PlayerId::One);
    let id = spell.id;
    game.players[0].hand.push(spell);
    for color in [
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
    ] {
        game.add_unrestricted_mana(PlayerId::One, color, 10);
    }
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::CastSpell { card, .. } if *card == id))
        .expect("spell offered");
    game.apply(PlayerId::One, action).unwrap();
}

#[test]
fn duel_cards_living_death_returns_only_the_exiled_graveyard_group() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        game.battlefield.clear();
        game.players[0]
            .graveyard
            .push(card(880_001, cards::SERRA_ANGEL, PlayerId::One));
        game.players[1]
            .graveyard
            .push(card(880_002, cards::AIR_ELEMENTAL, PlayerId::Two));
        game.battlefield
            .push(creature(880_003, cards::GRIZZLY_BEARS, PlayerId::One));
        game.battlefield
            .push(creature(880_004, cards::SAVANNAH_LIONS, PlayerId::Two));
        cast_named(&mut game, cards::LIVING_DEATH);
        drain_pending(&mut game);
        assert!(
            game.battlefield
                .iter()
                .any(|p| p.card.definition == cards::SERRA_ANGEL && p.controller == PlayerId::One)
        );
        assert!(game.battlefield.iter().any(|p| p.card.definition == cards::AIR_ELEMENTAL && p.controller == PlayerId::Two));
        assert!(
            !game
                .battlefield
                .iter()
                .any(|p| p.card.definition == cards::GRIZZLY_BEARS
                    || p.card.definition == cards::SAVANNAH_LIONS)
        );
        assert!(
            game.players[0]
                .graveyard
                .iter()
                .any(|c| c.definition == cards::GRIZZLY_BEARS)
        );
    }
}

#[test]
fn duel_cards_consultation_exiles_six_then_stops_at_the_named_card() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        game.players[0].library = vec![
            card(881_001, cards::ISLAND, PlayerId::One),
            card(881_002, cards::FOREST, PlayerId::One),
            card(881_003, cards::MOUNTAIN, PlayerId::One),
        ];
        game.players[0]
            .library
            .extend((0..6).map(|i| card(881_010 + i, cards::PLAINS, PlayerId::One)));
        cast_named(&mut game, cards::DEMONIC_CONSULTATION);
        pass_priority_pair(&mut game);
        let decision = game.pending_decisions[0].observation.clone();
        let forest = decision
            .options
            .iter()
            .find(|o| o.label == "Forest")
            .unwrap()
            .id;
        game.apply(
            decision.player,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![forest],
            },
        )
        .unwrap();
        drain_pending(&mut game);
        assert_eq!(game.players[0].library.len(), 1);
        assert_eq!(game.players[0].library[0].definition, cards::ISLAND);
        assert!(
            game.players[0]
                .hand
                .iter()
                .any(|c| c.id == GameObjectId(881_002) || c.definition == cards::FOREST)
        );
        assert_eq!(game.players[0].exile.len(), 7);
        assert!(
            game.players[0]
                .exile
                .iter()
                .any(|c| c.definition == cards::MOUNTAIN)
        );
    }
}

#[test]
fn duel_cards_oven_reads_sacrificed_toughness_before_zone_change() {
    for prepared in [false, true] {
        for counters in [0, 2] {
            let mut game = ready_game();
            game.set_prepared_engine_enabled(prepared);
            game.battlefield.clear();
            let oven = game
                .put_onto_battlefield(PlayerId::One, cards::WITCH_S_OVEN_237)
                .unwrap();
            let mut bear = creature(882_001, cards::GRIZZLY_BEARS, PlayerId::One);
            bear.counters.set(CounterKind::PlusOnePlusOne, counters);
            game.battlefield.push(bear);
            let action = game
                .legal_actions(PlayerId::One)
                .into_iter()
                .find(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == oven))
                .expect("Oven activation");
            game.apply(PlayerId::One, action).unwrap();
            assert!(
                !game
                    .battlefield
                    .iter()
                    .any(|p| p.card.id == GameObjectId(882_001))
            );
            drain_pending(&mut game);
            let foods = game
                .battlefield
                .iter()
                .filter(|p| p.card.id != oven)
                .count();
            assert_eq!(foods, if counters == 2 { 2 } else { 1 });
        }
    }
}

#[test]
fn duel_cards_pre_war_formalwear_attaches_to_returned_incarnation() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        game.battlefield.clear();
        game.players[0]
            .graveyard
            .push(card(883_001, cards::GRIZZLY_BEARS, PlayerId::One));
        game.put_onto_battlefield(PlayerId::One, cards::PRE_WAR_FORMALWEAR_21)
            .unwrap();
        drain_pending(&mut game);
        let bear = game
            .battlefield
            .iter()
            .find(|p| p.card.definition == cards::GRIZZLY_BEARS)
            .unwrap();
        let equipment = game
            .battlefield
            .iter()
            .find(|p| p.card.definition == cards::PRE_WAR_FORMALWEAR_21)
            .unwrap();
        assert_ne!(bear.card.id, GameObjectId(883_001));
        assert_eq!(equipment.attached_to, Some(bear.card.id));
        assert_eq!((game.power(bear), game.toughness(bear)), (Some(4), Some(4)));
    }
}

#[test]
fn duel_cards_cactus_preserve_uses_owned_commanders_in_every_zone() {
    for prepared in [false, true] {
        let deck = crate::Deck {
            commanders: vec![cards::GRIZZLY_BEARS, cards::SERRA_ANGEL],
            main: vec![cards::FOREST; 98],
            sideboard: Vec::new(),
        };
        let mut game = Game::new_with_format(
            crate::Format::DuelCommander,
            poc::catalog().unwrap(),
            [deck.clone(), deck],
            1,
        )
        .unwrap();
        game.set_prepared_engine_enabled(prepared);
        game.pregame = None;
        game.step = Step::PrecombatMain;
        let preserve = game
            .put_onto_battlefield(PlayerId::One, cards::CACTUS_PRESERVE_40)
            .unwrap();
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
        // The larger commander is hidden in hand, but remains a designation.
        let angel = game.players[0].command.pop().unwrap();
        game.players[0].hand.push(angel);
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == preserve))
            .unwrap();
        game.apply(PlayerId::One, action).unwrap();
        drain_pending(&mut game);
        let land = game
            .battlefield
            .iter()
            .find(|p| p.card.id == preserve)
            .unwrap();
        assert_eq!((game.power(land), game.toughness(land)), (Some(5), Some(5)));
    }
}
