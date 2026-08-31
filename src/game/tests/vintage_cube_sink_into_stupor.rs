//! Sink into Stupor // Soporific Springs: an Unsummon that also answers the
//! spell before it lands, on the back of a land the hand can fall back on.

use super::*;

/// Player One holding the card, with a land drop still available and their
/// turn's main phase in progress.
fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    let card = card(96_000, cards::SINK_INTO_STUPOR, PlayerId::One);
    let held = card.id;
    game.players[0].hand.push(card);
    game.players[0].lands_played_this_turn = 0;
    game.players[0].life = 20;
    game.turns_started = [1, 1];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, held)
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if game.observe(PlayerId::One).decision.is_some()
            || game.observe(PlayerId::Two).decision.is_some()
        {
            return;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

/// Every way Player One could cast the front face at `target`.
fn answers(game: &Game, held: GameObjectId, target: Target) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == held
                    && choices
                        .targets()
                        .iter()
                        .any(|selection| selection.targets().contains(&target))
            }
            _ => false,
        })
        .collect()
}

/// Puts `definition` on the stack for Player Two and hands priority back.
fn they_cast(game: &mut Game, definition: CardDefinitionId) -> GameObjectId {
    let card = game
        .build_zone(PlayerId::Two, &[definition])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = card.id;
    game.players[1].hand.push(card);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Green, 4);
    let action = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
        .expect("they can cast it");
    game.apply(PlayerId::Two, action).expect("it casts");
    game.priority = PlayerId::One;
    game.stack.last().expect("it is on the stack").id
}

/// Plays the land face, paying or declining the three life.
fn play_land(game: &mut Game, held: GameObjectId, pay: bool) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == held))
        .expect("the back face is playable as a land");
    game.apply(PlayerId::One, action).expect("it is played");

    let payment = game
        .observe(PlayerId::One)
        .decision
        .expect("it offers the three life");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: payment.id,
            options: vec![u32::from(pay)],
        },
    )
    .expect("answering the payment is legal");
    drain_pending(game);
}

fn springs(game: &Game) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SINK_INTO_STUPOR)
        .expect("it is on the battlefield")
}

/// Both faces are offered from hand: one is a spell, the other a land drop.
#[test]
fn it_offers_a_cast_and_a_land_drop() {
    let (mut game, held) = staged();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 3);
    let theirs = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);

    let actions = game.legal_actions(PlayerId::One);
    assert!(
        actions.iter().any(|action| matches!(
            action,
            Action::CastSpell { card, choices, .. }
                if *card == held
                    && choices
                        .targets()
                        .iter()
                        .any(|selection| selection.targets().contains(&Target::Permanent(theirs)))
        )),
        "the front is castable at their creature",
    );
    assert!(
        actions
            .iter()
            .any(|action| matches!(action, Action::PlayLand { card, .. } if *card == held)),
        "and the back is a land drop for the same card",
    );
}

/// The ordinary half: a creature they control goes back to hand.
#[test]
fn the_front_bounces_their_creature() {
    let (mut game, held) = staged();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 3);
    let theirs = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);

    let offers = answers(&game, held, Target::Permanent(theirs));
    game.apply(PlayerId::One, offers[0].clone())
        .expect("it is cast");
    settle(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == theirs),
        "their creature left the battlefield",
    );
    assert!(
        game.players[1]
            .hand
            .iter()
            .any(|card| card.definition == cards::GRIZZLY_BEARS),
        "and it is in its owner's hand, not the graveyard",
    );
}

/// The half that makes it playable on their turn: the spell itself is
/// returned before it ever resolves.
#[test]
fn the_front_answers_a_spell_on_the_stack() {
    let (mut game, held) = staged();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 3);
    // Their main phase, so the creature spell is castable at all.
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    let bears = they_cast(&mut game, cards::GRIZZLY_BEARS);

    let offers = answers(&game, held, Target::Spell(bears));
    assert!(!offers.is_empty(), "the spell on the stack is a target");
    game.apply(PlayerId::One, offers[0].clone())
        .expect("it is cast");
    settle(&mut game);

    assert!(
        game.battlefield.is_empty(),
        "the creature never reached the battlefield",
    );
    assert!(
        game.players[1]
            .hand
            .iter()
            .any(|card| card.definition == cards::GRIZZLY_BEARS),
        "their card came back rather than being countered into the graveyard",
    );
    assert!(
        game.players[1].graveyard.is_empty(),
        "nothing was countered",
    );
}

/// Sharing a zone with spells does not make an activated ability a spell.
#[test]
fn the_front_does_not_target_an_ability_on_the_stack() {
    let (mut game, held) = staged();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 3);
    let mut ability = spell(96_001, cards::PRODIGAL_SORCERER, PlayerId::Two, 0);
    ability.kind = StackObjectKind::ActivatedAbility;
    ability.source = Some(GameObjectId(96_002));
    ability.signature = None;
    let ability_id = ability.id;
    game.stack.push(ability);

    assert!(
        answers(&game, held, Target::Spell(ability_id)).is_empty(),
        "the stack branch is restricted to spells",
    );
}

/// "Nonland permanent an opponent controls": not your own creature, and not
/// a land on either side.
#[test]
fn it_points_at_neither_your_creatures_nor_any_land() {
    let (mut game, held) = staged();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 3);
    let yours = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    let their_land = game
        .put_onto_battlefield(PlayerId::Two, cards::FOREST)
        .expect("cataloged");
    drain_pending(&mut game);

    assert!(
        answers(&game, held, Target::Permanent(yours)).is_empty(),
        "your own creature is not a legal target",
    );
    assert!(
        answers(&game, held, Target::Permanent(their_land)).is_empty(),
        "and a land is not a nonland permanent",
    );
}

/// Pay the three life and the land arrives ready to use.
#[test]
fn paying_three_life_leaves_it_untapped() {
    let (mut game, held) = staged();

    play_land(&mut game, held, true);

    assert_eq!(game.players[0].life, 17);
    assert!(!springs(&game).tapped, "paid, so it is ready now");
    assert_eq!(
        springs(&game).presented,
        CardPartId(1),
        "and it is the land face that is on the battlefield",
    );
    assert_eq!(
        game.players[0].lands_played_this_turn, 1,
        "it cost the land drop",
    );
}

/// Decline and it enters tapped instead.
#[test]
fn declining_leaves_it_tapped() {
    let (mut game, held) = staged();

    play_land(&mut game, held, false);

    assert_eq!(game.players[0].life, 20, "nothing was paid");
    assert!(springs(&game).tapped);
}

/// Untapped, it taps for blue.
#[test]
fn the_springs_tap_for_blue() {
    let (mut game, held) = staged();
    play_land(&mut game, held, true);
    let land = springs(&game).card.id;

    let add_blue = Action::ActivateManaAbility {
        source: land,
        ability: mana_ability_for(&game, land, ManaColor::Blue),
        color: ManaColor::Blue,
        counters_removed: None,
        cost_object: None,
        combination: None,
        triggered_mana: None,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&add_blue));
    game.apply(PlayerId::One, add_blue).expect("it taps");

    assert_eq!(game.players[0].mana_pool.blue, 1);
}
