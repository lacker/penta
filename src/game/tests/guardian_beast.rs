use super::*;
use crate::card::PlayerRefDef;

static ENCHANT_PERMANENT_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Any,
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];
static ENCHANT_PERMANENT_ABILITIES: [AbilityDef; 1] = [AbilityDef::spell_with_targets(
    "Enchant permanent",
    &ENCHANT_PERMANENT_TARGETS,
    EffectDef::Attach {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
)];

#[test]
fn guardian_beast_protects_only_controlled_noncreature_artifacts_while_untapped() {
    let mut game = ready_game();
    let guardian = GameObjectId(10_000);
    let orb = GameObjectId(10_001);
    let artifact_creature = GameObjectId(10_002);
    let opposing_orb = GameObjectId(10_003);
    game.battlefield.extend([
        creature(guardian.0, cards::GUARDIAN_BEAST, PlayerId::One),
        creature(orb.0, cards::CHAOS_ORB, PlayerId::One),
        creature(artifact_creature.0, cards::SU_CHI, PlayerId::One),
        creature(opposing_orb.0, cards::CHAOS_ORB, PlayerId::Two),
    ]);

    let has_indestructible = |game: &Game, id| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("the test permanent is on the battlefield");
        game.permanent_has_executable_keyword(permanent, KeywordAbility::Indestructible)
    };
    assert!(has_indestructible(&game, orb));
    assert!(!has_indestructible(&game, artifact_creature));
    assert!(!has_indestructible(&game, opposing_orb));

    game.destroy_permanent(orb);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == orb),
        "the untapped Beast makes its controller's Chaos Orb indestructible",
    );

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == guardian)
        .expect("Guardian Beast is on the battlefield")
        .tapped = true;
    game.destroy_permanent(orb);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != orb),
        "tapping Guardian Beast immediately ends its protection",
    );
}

#[test]
fn guardian_beast_stops_an_opponent_from_gaining_control_of_an_artifact() {
    let mut game = ready_game();
    let guardian = GameObjectId(10_000);
    let orb = GameObjectId(10_001);
    game.battlefield.extend([
        creature(guardian.0, cards::GUARDIAN_BEAST, PlayerId::One),
        creature(orb.0, cards::CHAOS_ORB, PlayerId::One),
    ]);
    let steal = EffectDef::GainControl {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        duration: crate::card::ControlDurationDef::UntilEndOfTurn,
        controller: PlayerRefDef::EffectController,
    };

    let object = spell_with_targets(
        20_000,
        cards::ZEALOUS_CONSCRIPTS,
        PlayerId::Two,
        vec![Target::Permanent(orb)],
        0,
    );
    game.resolve_effect_def(
        ScopedEffect::primary(steal),
        &object,
        TriggerContext::empty(),
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == orb)
            .expect("Chaos Orb remains on the battlefield")
            .controller,
        PlayerId::One,
    );

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == guardian)
        .expect("Guardian Beast is on the battlefield")
        .tapped = true;
    let object = spell_with_targets(
        20_001,
        cards::ZEALOUS_CONSCRIPTS,
        PlayerId::Two,
        vec![Target::Permanent(orb)],
        0,
    );
    game.resolve_effect_def(
        ScopedEffect::primary(steal),
        &object,
        TriggerContext::empty(),
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == orb)
            .expect("Chaos Orb remains on the battlefield")
            .controller,
        PlayerId::Two,
        "the same control effect works once Guardian Beast is tapped",
    );
}

#[test]
fn guardian_beast_keeps_an_existing_aura_but_blocks_a_new_one() {
    let aura_definition = CardDefinitionId::new(10_100);
    let mut definition = CardDefinition::new(
        aura_definition,
        "Enchant permanent test Aura",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_enchantment(ManaCost::default())
        .with_subtypes(&["Aura"])
        .with_abilities(&ENCHANT_PERMANENT_ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();

    let guardian = GameObjectId(10_000);
    let orb = GameObjectId(10_001);
    let other_host = GameObjectId(10_004);
    let mut existing_aura = creature(10_002, aura_definition, PlayerId::One);
    existing_aura.attached_to = Some(orb);
    let mut moving_aura = creature(10_005, aura_definition, PlayerId::One);
    moving_aura.attached_to = Some(other_host);
    game.battlefield.extend([
        creature(guardian.0, cards::GUARDIAN_BEAST, PlayerId::One),
        creature(orb.0, cards::CHAOS_ORB, PlayerId::One),
        creature(other_host.0, cards::GRIZZLY_BEARS, PlayerId::One),
        existing_aura,
        moving_aura,
    ]);

    let aura = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_002))
        .expect("the existing Aura is on the battlefield");
    assert!(
        game.is_legal_aura_host(aura, orb),
        "Guardian Beast does not remove an Aura already attached",
    );
    assert!(
        !game.try_attach(GameObjectId(10_005), orb),
        "a nontargeted effect cannot move a new Aura onto the protected artifact",
    );

    let new_aura = card(10_003, aura_definition, PlayerId::One);
    game.players[PlayerId::One.index()]
        .hand
        .push(new_aura.clone());
    let aura_targets = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .filter_map(|action| match action {
                Action::CastSpell { card, choices, .. } if card == new_aura.id => {
                    choices.iter_targets().copied().next()
                }
                _ => None,
            })
            .collect::<std::collections::HashSet<_>>()
    };
    assert!(
        !aura_targets(&game).contains(&Target::Permanent(orb)),
        "a new Aura cannot target the protected artifact",
    );

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == guardian)
        .expect("Guardian Beast is on the battlefield")
        .tapped = true;
    assert!(
        aura_targets(&game).contains(&Target::Permanent(orb)),
        "the artifact becomes a legal Aura target when Guardian Beast is tapped",
    );
    assert!(
        game.try_attach(GameObjectId(10_005), orb),
        "nontargeted attachment becomes legal at the same time",
    );
}
