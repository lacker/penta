//! Static prohibitions on activating artifact abilities.

use super::*;

const NEW_ARTIFACT_LOCKS: [CardDefinitionId; 2] = [cards::NULL_ROD, cards::COLLECTOR_OUPHE];

#[derive(Clone, Copy)]
struct ActivationSources {
    player: PlayerId,
    artifact_ordinary: GameObjectId,
    artifact_mana: GameObjectId,
    nonartifact_ordinary: GameObjectId,
    nonartifact_mana: GameObjectId,
}

fn activation_board(lock: CardDefinitionId) -> (Game, GameObjectId, [ActivationSources; 2]) {
    let mut game = ready_game();
    game.turns_started = [1, 1];

    let lock = creature(10_000, lock, PlayerId::One);
    let lock_id = lock.card.id;
    game.battlefield.push(lock);

    let sources = [
        ActivationSources {
            player: PlayerId::One,
            artifact_ordinary: GameObjectId(10_010),
            artifact_mana: GameObjectId(10_011),
            nonartifact_ordinary: GameObjectId(10_012),
            nonartifact_mana: GameObjectId(10_013),
        },
        ActivationSources {
            player: PlayerId::Two,
            artifact_ordinary: GameObjectId(10_020),
            artifact_mana: GameObjectId(10_021),
            nonartifact_ordinary: GameObjectId(10_022),
            nonartifact_mana: GameObjectId(10_023),
        },
    ];
    for source in sources {
        game.battlefield.extend([
            creature(
                source.artifact_ordinary.0,
                cards::NEVINYRRALS_DISK,
                source.player,
            ),
            creature(source.artifact_mana.0, cards::MOX_RUBY, source.player),
            creature(
                source.nonartifact_ordinary.0,
                cards::SEDGE_TROLL,
                source.player,
            ),
            creature(
                source.nonartifact_mana.0,
                cards::ELVISH_MYSTIC,
                source.player,
            ),
        ]);
        game.players[source.player.index()].mana_pool.black = 1;
        game.players[source.player.index()].mana_pool.colorless = 1;
    }

    (game, lock_id, sources)
}

fn has_ordinary_activation(actions: &[Action], source: GameObjectId) -> bool {
    actions.iter().any(
        |action| matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source),
    )
}

fn has_mana_activation(actions: &[Action], source: GameObjectId) -> bool {
    actions.iter().any(
        |action| matches!(action, Action::ActivateManaAbility { source: actual, .. } if *actual == source),
    )
}

fn assert_activation_state(
    game: &mut Game,
    sources: [ActivationSources; 2],
    artifacts_can_activate: bool,
) {
    for source in sources {
        game.priority = source.player;
        let actions = game.legal_actions(source.player);
        assert_eq!(
            has_ordinary_activation(&actions, source.artifact_ordinary),
            artifacts_can_activate,
            "the artifact's ordinary activation has the expected permission",
        );
        assert_eq!(
            has_mana_activation(&actions, source.artifact_mana),
            artifacts_can_activate,
            "the artifact's mana activation has the expected permission",
        );
        assert!(
            has_ordinary_activation(&actions, source.nonartifact_ordinary),
            "a nonartifact's ordinary activation remains available",
        );
        assert!(
            has_mana_activation(&actions, source.nonartifact_mana),
            "a nonartifact's mana activation remains available",
        );
    }
}

#[test]
fn each_new_artifact_lock_is_global_and_live() {
    for lock in NEW_ARTIFACT_LOCKS {
        let (mut game, lock_id, sources) = activation_board(lock);
        assert_activation_state(&mut game, sources, false);

        game.battlefield
            .retain(|permanent| permanent.card.id != lock_id);
        assert_activation_state(&mut game, sources, true);
    }
}

#[test]
fn a_permanent_is_locked_as_soon_as_it_becomes_an_artifact() {
    let mut game = ready_game();
    game.turns_started = [1, 1];
    game.battlefield
        .push(creature(10_000, cards::NULL_ROD, PlayerId::One));
    let factory = creature(10_001, cards::MISHRA_S_FACTORY, PlayerId::Two);
    let factory_id = factory.card.id;
    game.battlefield.push(factory);
    game.players[PlayerId::Two.index()].mana_pool.colorless = 1;
    game.priority = PlayerId::Two;

    let actions = game.legal_actions(PlayerId::Two);
    assert!(has_mana_activation(&actions, factory_id));
    let animate = actions
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == factory_id)
        })
        .expect("the nonartifact Factory can animate");
    game.apply(PlayerId::Two, animate)
        .expect("the animation activates");
    drain_pending(&mut game);

    let factory = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == factory_id)
        .expect("the Factory remains on the battlefield");
    assert!(
        game.permanent_types(factory)
            .is_some_and(|types| types.contains(CardType::Artifact)),
    );
    game.priority = PlayerId::Two;
    let actions = game.legal_actions(PlayerId::Two);
    assert!(!has_ordinary_activation(&actions, factory_id));
    assert!(!has_mana_activation(&actions, factory_id));
}

#[test]
fn activation_prohibitions_match_each_effective_ability() {
    let mut game = ready_game();
    game.turns_started = [1, 1];
    game.battlefield.clear();

    let assassin = creature(10_000, cards::ROYAL_ASSASSIN, PlayerId::One);
    let assassin_id = assassin.card.id;
    let mox = creature(10_001, cards::MOX_RUBY, PlayerId::One);
    let mox_id = mox.card.id;
    let mut target = creature(10_002, cards::SAVANNAH_LIONS, PlayerId::Two);
    target.tapped = true;
    game.battlefield.extend([assassin, mox, target]);
    game.priority = PlayerId::One;

    attach_resolved_rule(
        &mut game,
        assassin_id,
        AppliedRuleDef::CannotActivateAbilities(AbilityPredicateDef::Is(
            crate::card::AbilityKindDef::ActivatedMana,
        )),
        ContinuousEffectExpiration::Never,
    );
    attach_resolved_rule(
        &mut game,
        mox_id,
        AppliedRuleDef::CannotActivateAbilities(AbilityPredicateDef::Is(
            crate::card::AbilityKindDef::NonManaActivated,
        )),
        ContinuousEffectExpiration::Never,
    );

    let actions = game.legal_actions(PlayerId::One);
    assert!(
        has_ordinary_activation(&actions, assassin_id),
        "a mana-only prohibition does not suppress an ordinary activation",
    );
    assert!(
        has_mana_activation(&actions, mox_id),
        "a nonmana prohibition does not suppress a mana activation",
    );

    attach_resolved_rule(
        &mut game,
        assassin_id,
        AppliedRuleDef::CannotActivateAbilities(AbilityPredicateDef::Is(
            crate::card::AbilityKindDef::NonManaActivated,
        )),
        ContinuousEffectExpiration::Never,
    );
    attach_resolved_rule(
        &mut game,
        mox_id,
        AppliedRuleDef::CannotActivateAbilities(AbilityPredicateDef::Is(
            crate::card::AbilityKindDef::ActivatedMana,
        )),
        ContinuousEffectExpiration::Never,
    );

    let actions = game.legal_actions(PlayerId::One);
    assert!(!has_ordinary_activation(&actions, assassin_id));
    assert!(!has_mana_activation(&actions, mox_id));
}

#[test]
fn arrest_has_both_printings() {
    let catalog = poc::catalog().expect("catalog builds");
    let arrest = catalog.get(cards::ARREST).expect("Arrest is cataloged");
    for set in [CardSet::MercadianMasques, CardSet::ReturnToRavnica] {
        assert!(
            arrest
                .printings
                .iter()
                .any(|printing| printing.id.set == set),
            "Arrest includes its {set:?} printing",
        );
    }
}
