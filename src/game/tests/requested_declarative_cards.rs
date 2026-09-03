//! Focused execution checks for the unsupported-card completion pass.

use super::*;

fn choose_keep_original_targets(game: &mut Game, player: PlayerId) {
    let decision = game
        .observe(player)
        .decision
        .expect("copy target selection is pending");
    let option = decision
        .options
        .iter()
        .find(|option| option.label == "Keep original targets")
        .expect("the copy may retain its targets")
        .id;
    game.apply(
        player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("keeping the original targets is legal");
}

#[test]
fn leyline_of_the_void_exiles_an_opponents_discard() {
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::One, cards::LEYLINE_OF_THE_VOID)
        .expect("Leyline of the Void is cataloged");
    drain_pending(&mut game);
    let discarded = card(190_000, cards::LIGHTNING_BOLT, PlayerId::Two);
    let discarded_id = discarded.id;
    game.players[PlayerId::Two.index()].hand.push(discarded);

    game.discard_cards(PlayerId::Two, &[discarded_id]);
    drain_pending(&mut game);

    assert!(
        game.players[PlayerId::Two.index()].graveyard.is_empty(),
        "the replacement keeps the card out of its owner's graveyard",
    );
    assert_eq!(
        game.players[PlayerId::Two.index()].exile[0].definition,
        cards::LIGHTNING_BOLT,
    );
}

#[test]
fn path_to_exile_gives_the_exiled_creatures_controller_the_search_choice() {
    let mut game = ready_game();
    let creature = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("Grizzly Bears is cataloged");
    drain_pending(&mut game);
    let path = card(190_005, cards::PATH_TO_EXILE, PlayerId::One);
    let path_id = path.id;
    game.players[PlayerId::One.index()].hand.push(path);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == path_id
                        && choices
                            .iter_targets()
                            .any(|target| *target == Target::Permanent(creature))
            )
        })
        .expect("Path can target the opposing creature");
    game.apply(PlayerId::One, cast).expect("Path is cast");
    pass_until_decision(&mut game);

    let decision = game
        .observe(PlayerId::Two)
        .decision
        .expect("the exiled creature's controller may search");
    assert_eq!(decision.player, PlayerId::Two);
    let decline = decision
        .options
        .iter()
        .find(|option| option.label == "Decline")
        .expect("the search may be declined")
        .id;
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decline],
        },
    )
    .expect("declining the search is legal");
    assert!(
        game.players[PlayerId::Two.index()]
            .exile
            .iter()
            .any(|card| card.definition == cards::GRIZZLY_BEARS),
    );
}

#[test]
fn leyline_of_lifeforce_protects_only_creature_spells() {
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::One, cards::LEYLINE_OF_LIFEFORCE)
        .expect("Leyline of Lifeforce is cataloged");
    drain_pending(&mut game);

    game.stack
        .push(spell(190_010, cards::GRIZZLY_BEARS, PlayerId::Two, 0));
    assert!(!game.can_be_countered(&game.stack[0]));

    game.stack.clear();
    game.stack
        .push(spell(190_011, cards::LIGHTNING_BOLT, PlayerId::Two, 0));
    assert!(game.can_be_countered(&game.stack[0]));
}

#[test]
fn leyline_of_abundance_adds_green_when_a_creature_is_tapped_for_mana() {
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::One, cards::LEYLINE_OF_ABUNDANCE)
        .expect("Leyline of Abundance is cataloged");
    let elf = game
        .put_onto_battlefield(PlayerId::One, cards::LLANOWAR_ELVES)
        .expect("Llanowar Elves is cataloged");
    drain_pending(&mut game);
    game.turns_started[PlayerId::One.index()] = 1;
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateManaAbility {
                    source,
                    color: ManaColor::Green,
                    ..
                } if *source == elf
            )
        })
        .expect("the Elf can tap for green");
    game.apply(PlayerId::One, activation)
        .expect("the mana ability is legal");

    assert_eq!(game.players[PlayerId::One.index()].mana_pool.green, 2);
}

#[test]
fn leyline_of_resonance_copies_the_exact_single_target_shape() {
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::One, cards::LEYLINE_OF_RESONANCE)
        .expect("Leyline of Resonance is cataloged");
    let creature = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("Grizzly Bears is cataloged");
    drain_pending(&mut game);
    let bolt = card(190_020, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[PlayerId::One.index()].hand.push(bolt);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == bolt_id
                        && choices
                            .iter_targets()
                            .any(|target| *target == Target::Permanent(creature))
            )
        })
        .expect("the Bolt can target its controller's creature");
    game.apply(PlayerId::One, cast).expect("the Bolt is cast");
    pass_until_decision(&mut game);
    choose_keep_original_targets(&mut game, PlayerId::One);

    assert_eq!(game.stack.iter().filter(|object| object.is_copy).count(), 1);
}

#[test]
fn barkshell_blessings_conspire_cost_taps_two_creatures_and_copies_the_spell() {
    let mut game = ready_game();
    let green = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("Grizzly Bears is cataloged");
    let white = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("Savannah Lions is cataloged");
    drain_pending(&mut game);
    let blessing = card(190_030, cards::BARKSHELL_BLESSING, PlayerId::One);
    let blessing_id = blessing.id;
    game.players[PlayerId::One.index()].hand.push(blessing);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell {
                    card,
                    choices,
                    sacrifices,
                } if *card == blessing_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Permanent(green))
                    && sacrifices.contains(&green)
                    && sacrifices.contains(&white)
            )
        })
        .expect("both creatures can pay the conspire cost");
    game.apply(PlayerId::One, cast)
        .expect("the conspired spell is cast");
    assert!(
        game.battlefield
            .iter()
            .filter(|permanent| [green, white].contains(&permanent.card.id))
            .all(|permanent| permanent.tapped),
    );
    pass_until_decision(&mut game);
    choose_keep_original_targets(&mut game, PlayerId::One);

    assert_eq!(game.stack.iter().filter(|object| object.is_copy).count(), 1);
}

#[test]
fn burn_trail_reuses_conspire_with_its_red_color_restriction() {
    let mut game = ready_game();
    let first_red = game
        .put_onto_battlefield(PlayerId::One, cards::GOBLIN_PIKER)
        .expect("Goblin Piker is cataloged");
    let second_red = game
        .put_onto_battlefield(PlayerId::One, cards::GOBLIN_PIKER)
        .expect("Goblin Piker is cataloged");
    let green = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("Grizzly Bears is cataloged");
    drain_pending(&mut game);
    let trail = card(190_031, cards::BURN_TRAIL, PlayerId::One);
    let trail_id = trail.id;
    game.players[PlayerId::One.index()].hand.push(trail);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 4);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell {
                    card,
                    choices,
                    sacrifices,
                } if *card == trail_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Player(PlayerId::Two))
                    && sacrifices.contains(&first_red)
                    && sacrifices.contains(&second_red)
            )
        })
        .expect("two red creatures can pay Burn Trail's conspire cost");
    game.apply(PlayerId::One, cast)
        .expect("the conspired Burn Trail is cast");
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == green)
            .is_some_and(|permanent| !permanent.tapped),
        "a creature that shares no color with Burn Trail is not tapped",
    );
    pass_until_decision(&mut game);
    choose_keep_original_targets(&mut game, PlayerId::One);

    assert_eq!(game.stack.iter().filter(|object| object.is_copy).count(), 1);
}
