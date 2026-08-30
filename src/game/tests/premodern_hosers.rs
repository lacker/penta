//! Cards that tax or switch off what an opponent is doing.
//!
//! Each is symmetrical or unconditional in a way worth pinning down: Chill
//! taxes every red spell whoever casts it, Cursed Totem stops creature
//! abilities on both sides, and the Atog eats from either of two places.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game.players[PlayerId::One.index()].graveyard.clear();
    game
}

fn castable(game: &Game, id: GameObjectId) -> bool {
    game.legal_actions(PlayerId::One)
        .iter()
        .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
}

fn settle(game: &mut Game) {
    for _ in 0..8 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
}

/// Two more mana for a red spell, and nothing for anything else.
#[test]
fn chill_taxes_red_spells_only() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::CHILL, PlayerId::One));
    let bolt = card(20_000, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[PlayerId::One.index()].hand.push(bolt);
    let recall = card(20_001, cards::ANCESTRAL_RECALL, PlayerId::One);
    let recall_id = recall.id;
    game.players[PlayerId::One.index()].hand.push(recall);
    // One red and one blue: enough for either spell unless the tax applies.
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    game.players[PlayerId::One.index()].mana_pool.blue = 1;

    assert!(!castable(&game, bolt_id), "a Bolt now costs {{2}}{{R}}");
    assert!(castable(&game, recall_id), "and a blue spell is untouched");
}

/// Three more mana pays the tax off.
#[test]
fn chill_is_paid_off_with_the_extra_two() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::CHILL, PlayerId::One));
    let bolt = card(20_000, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[PlayerId::One.index()].hand.push(bolt);
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    assert!(castable(&game, bolt_id), "{{2}}{{R}} is payable");
}

/// The Totem stops creature abilities on both sides of the table, and leaves
/// noncreature abilities alone.
#[test]
fn the_totem_silences_creatures_on_both_sides() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::CURSED_TOTEM, PlayerId::One));
    let mine = creature(10_001, cards::GOBLIN_SHARPSHOOTER, PlayerId::One);
    let mine_id = mine.card.id;
    game.battlefield.push(mine);
    let theirs = creature(10_002, cards::GOBLIN_SHARPSHOOTER, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);
    // A noncreature source with an activated ability, as the control.
    let factory = creature(10_003, cards::MISHRA_S_FACTORY, PlayerId::One);
    let factory_id = factory.card.id;
    game.battlefield.push(factory);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;

    let activates = |game: &Game, player: PlayerId, id: GameObjectId| {
        game.legal_actions(player)
            .iter()
            .any(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == id))
    };
    assert!(
        !activates(&game, PlayerId::One, mine_id),
        "your own creature is silenced too",
    );
    assert!(
        !activates(&game, PlayerId::Two, theirs_id),
        "and so is theirs",
    );
    assert!(
        activates(&game, PlayerId::One, factory_id),
        "a land is not a creature, so its ability still works",
    );
}

/// The Atog grows from hand or from graveyard, and the graveyard half spends
/// two cards for the same +1/+1.
#[test]
fn the_atog_eats_from_hand_and_from_graveyard() {
    let mut game = ready();
    let atog = creature(10_000, cards::PSYCHATOG, PlayerId::One);
    let atog_id = atog.card.id;
    game.battlefield.push(atog);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.players[PlayerId::One.index()].hand.push(card(
        20_000,
        cards::GRIZZLY_BEARS,
        PlayerId::One,
    ));
    for index in 0..2 {
        game.players[PlayerId::One.index()].graveyard.push(card(
            30_000 + index,
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }

    let feeds: Vec<_> = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == atog_id),
        )
        .collect();
    assert_eq!(
        feeds.len(),
        2,
        "one card in hand and one pair in the graveyard is one of each",
    );

    let stats = |game: &Game| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == atog_id)
            .expect("still there");
        (game.power(permanent), game.toughness(permanent))
    };
    assert_eq!(stats(&game), (Some(1), Some(2)), "a 1/2 to start");
    let feed = feeds.into_iter().next().expect("an activation is offered");
    game.apply(PlayerId::One, feed).expect("it is activated");
    settle(&mut game);
    assert_eq!(stats(&game), (Some(2), Some(3)), "and 2/3 after one meal");
}

/// The Coliseum's dig is gated by the same threshold the Ring uses, and its
/// mana ability stays available either way.
#[test]
fn the_coliseum_digs_only_past_threshold() {
    let offered = |graveyard: usize| {
        let mut game = ready();
        let land = creature(10_000, cards::CEPHALID_COLISEUM, PlayerId::One);
        let land_id = land.card.id;
        game.battlefield.push(land);
        for index in 0..graveyard {
            game.players[PlayerId::One.index()].graveyard.push(card(
                30_000 + u32::try_from(index).expect("small"),
                cards::GRIZZLY_BEARS,
                PlayerId::One,
            ));
        }
        game.players[PlayerId::One.index()].mana_pool.blue = 1;
        game.legal_actions(PlayerId::One)
            .iter()
            .filter(|action| {
                matches!(action, Action::ActivateAbility { source, .. } if *source == land_id)
            })
            .count()
    };
    assert_eq!(offered(6), 0, "six cards is short of threshold");
    assert!(offered(7) > 0, "seven turns it into a draw spell");
}

/// The Aura taxes only what an opponent casts.
#[test]
fn the_aura_taxes_the_opponent_and_not_you() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::AURA_OF_SILENCE, PlayerId::One));
    let yours = card(20_000, cards::BLACK_VISE, PlayerId::One);
    let yours_id = yours.id;
    game.players[PlayerId::One.index()].hand.push(yours);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    assert!(
        castable(&game, yours_id),
        "your own artifact is not taxed, so one mana still casts a Vise",
    );
}

/// And it answers something on the way out.
#[test]
fn the_aura_trades_itself_for_an_artifact() {
    let mut game = ready();
    let aura = creature(10_000, cards::AURA_OF_SILENCE, PlayerId::One);
    let aura_id = aura.card.id;
    game.battlefield.push(aura);
    let vise = creature(10_001, cards::BLACK_VISE, PlayerId::Two);
    let vise_id = vise.card.id;
    game.battlefield.push(vise);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == aura_id
                    && targets
                        .iter()
                        .any(|slot| slot.targets().contains(&Target::Permanent(vise_id)))
            }
            _ => false,
        })
        .expect("the Vise can be named");
    game.apply(PlayerId::One, action).expect("it is activated");
    settle(&mut game);

    assert!(
        game.battlefield.is_empty(),
        "the Aura and the Vise both went"
    );
}

/// Standstill pays out to the caster's opponents, so casting into your own
/// Standstill hands the cards across the table.
#[test]
fn standstill_refills_the_other_player() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::STANDSTILL, PlayerId::One));
    game.players[PlayerId::Two.index()].library.clear();
    for index in 0..5 {
        game.players[PlayerId::Two.index()].library.push(card(
            31_000 + index,
            cards::GRIZZLY_BEARS,
            PlayerId::Two,
        ));
    }
    let before = game.players[PlayerId::Two.index()].hand.len();

    let bolt = spell(21_000, cards::LIGHTNING_BOLT, PlayerId::One, 0);
    let cast = game
        .stack_trigger_event_object(&bolt)
        .expect("a cast spell");
    game.capture_battlefield_triggers(&CommittedTriggerEvent::SpellCast {
        object: cast,
        from: CastSourceZone::Hand,
    });
    settle(&mut game);

    assert_eq!(
        game.players[PlayerId::Two.index()].hand.len(),
        before + 3,
        "the caster's opponent drew three",
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::STANDSTILL),
        "and the enchantment sacrificed itself",
    );
}
