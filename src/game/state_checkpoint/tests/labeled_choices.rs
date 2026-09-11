use super::*;
use crate::card::cards;
use crate::card::{
    BattlefieldEntryScalarChoiceDef, CostDef, EffectRecipientDef, ReplacementChoiceDef,
    ReplacementEffectDef,
};
use crate::game::tests::{drain_pending, ready_game};
const ARBITRARY_CHOICES: &[crate::card::TokenChoiceDef] = &[
    crate::card::TokenChoiceDef {
        label: "Sunrise",
        token: crate::card::TokenCharacteristics::creature(
            &["Camarid"],
            &[crate::ManaColor::Blue],
            1,
            1,
        ),
    },
    crate::card::TokenChoiceDef {
        label: "Twilight",
        token: crate::card::TokenCharacteristics::creature(
            &["Goblin"],
            &[crate::ManaColor::Red],
            1,
            1,
        )
        .with_abilities(&[crate::card::abilities::flying()]),
    },
];
const ARBITRARY_RULES: CardRules = CardRules::new_artifact(crate::mana_cost!("{3}"))
    .with_abilities(&[
        AbilityDef::replacement(
            "As this enters, choose Sunrise or Twilight.",
            ReplacementEffectDef::BindOutput {
                binding: crate::Binding!("branch_output"),
                effect: &ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                    BattlefieldEntryScalarChoiceDef::tokens(ARBITRARY_CHOICES),
                )),
            },
        ),
        AbilityDef::activated(
            "Create two tapped tokens of the chosen declaration, then gain 16 life.",
            &[CostDef::Mana(crate::mana_cost!("{3}")), CostDef::TapSource],
            EffectDef::Sequence(&[
                EffectDef::CreateToken(
                    crate::card::CreateTokenDef::new(crate::card::TokenDef::Binding(
                        crate::Binding!("branch_output"),
                    ))
                    .with_amount(2)
                    .entering_tapped()
                    .with_created_tokens(crate::card::CreatedTokensDef {
                        binding: crate::ParentBinding,
                        then: &EffectDef::Sequence(&[
                            EffectDef::AddCounters {
                                object: EffectRecipientDef::objects(
                                    crate::card::ObjectSetDef::Binding(crate::ParentBinding),
                                ),
                                kind: crate::CounterKind::PlusOnePlusOne,
                                amount: ValueDef::Constant(1),
                            },
                            EffectDef::GainLife {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::Constant(8),
                            },
                        ]),
                    }),
                ),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(16),
                },
            ]),
        ),
    ]);

fn choose(game: &mut Game, label: &str) {
    let player = game.decision_player().unwrap();
    let decision = game.observe(player).decision.unwrap();
    let option = decision
        .options
        .iter()
        .find(|option| option.label == label)
        .unwrap()
        .id;
    game.apply(
        player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .unwrap();
}

fn activate(game: &mut Game, player: PlayerId, source: GameObjectId) {
    game.priority = player;
    game.players[player.index()].mana_pool.colorless = 3;
    let action = game.legal_actions(player).into_iter().find(|action|
        matches!(action, Action::ActivateAbility { source: candidate, .. } if *candidate == source)
    ).expect("the paid tap activation is offered");
    game.apply(player, action).unwrap();
    assert_eq!(
        game.stack.len(),
        1,
        "the token ability uses the ordinary stack"
    );
    assert_eq!(game.players[player.index()].mana_pool.colorless, 0);
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
            .unwrap()
            .tapped
    );
}

fn assert_token(game: &Game, controller: PlayerId, subtype: &str, color_index: usize) {
    let tokens: Vec<_> = game
        .battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == ObjectKind::Token)
        .collect();
    assert_eq!(tokens.len(), 1);
    let token = tokens[0];
    assert_eq!(token.controller, controller);
    assert_eq!(
        (game.power(token), game.toughness(token)),
        (Some(1), Some(1))
    );
    assert_eq!(game.effective_subtypes(token), vec![subtype]);
    let mut colors = [false; 5];
    colors[color_index] = true;
    assert_eq!(
        game.effective_colors(token, &game.effective_rules(token).unwrap()),
        colors
    );
}

#[test]
fn labeled_choices_sarpadian_options_restore_and_create_their_tokens() {
    let labels = [
        "white Citizen",
        "blue Camarid",
        "black Thrull",
        "red Goblin",
        "green Saproling",
    ];
    for (index, (label, subtype)) in labels
        .iter()
        .zip(["Citizen", "Camarid", "Thrull", "Goblin", "Saproling"])
        .enumerate()
    {
        let mut game = ready_game();
        let source = game
            .put_onto_battlefield(PlayerId::One, cards::SARPADIAN_EMPIRES_VOL_VII)
            .unwrap();
        assert!(
            game.battlefield
                .iter()
                .all(|permanent| permanent.card.id != source),
            "the choice happens before entry, not in an ETB trigger"
        );
        let decision = game.observe(PlayerId::One).decision.unwrap();
        assert_eq!(
            decision
                .options
                .iter()
                .map(|option| option.label.as_str())
                .collect::<Vec<_>>(),
            labels
        );
        let (_, mut game) = rebuild_current_checkpoint(&game, PlayerId::One, 100);
        choose(&mut game, label);
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
            .unwrap();
        assert_eq!(
            permanent
                .chosen_tokens
                .get("sarpadian_empires_choice")
                .map(|declaration| declaration.label.as_str()),
            Some(*label)
        );
        assert_eq!(
            permanent.chosen_tokens["sarpadian_empires_choice"]
                .token
                .name(),
            subtype
        );
        assert!(permanent.chosen_color.is_none());
        assert!(permanent.chosen_creature_type.is_none());
        let (wire, mut game) = rebuild_current_checkpoint(&game, PlayerId::Two, 101);
        assert_eq!(
            wire["battlefield"][0]["chosenLabels"]["sarpadian_empires_choice"],
            *label
        );
        assert_eq!(wire["battlefield"][0]["hasIndividualState"], true);
        activate(&mut game, PlayerId::One, source);
        let (_, mut game) = rebuild_current_checkpoint(&game, PlayerId::One, 102);
        drain_pending(&mut game);
        assert!(
            game.pending_decisions.is_empty(),
            "activation does not ask for another choice"
        );
        assert_token(&game, PlayerId::One, subtype, index);
        let (_, rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 103);
        assert_token(&rebuilt, PlayerId::One, subtype, index);
    }
}

#[test]
fn labeled_choices_follow_control_and_survive_source_removal_without_leaking_to_new_objects() {
    let mut game = ready_game();
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::SARPADIAN_EMPIRES_VOL_VII)
        .unwrap();
    choose(&mut game, "blue Camarid");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == source)
        .unwrap()
        .controller = PlayerId::Two;
    activate(&mut game, PlayerId::Two, source);
    game.destroy_permanent(source);
    let (_, mut game) = rebuild_current_checkpoint(&game, PlayerId::Two, 104);
    let second = game
        .put_onto_battlefield(PlayerId::One, cards::SARPADIAN_EMPIRES_VOL_VII)
        .unwrap();
    choose(&mut game, "red Goblin");
    drain_pending(&mut game);
    assert_token(&game, PlayerId::Two, "Camarid", 1);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == second)
            .unwrap()
            .chosen_tokens["sarpadian_empires_choice"]
            .label,
        "red Goblin"
    );
}

#[test]
fn labeled_choices_bind_declarations_and_compose_with_token_modifiers_and_sequence_tails() {
    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    let definition = definitions
        .iter_mut()
        .find(|definition| definition.id == cards::SARPADIAN_EMPIRES_VOL_VII)
        .unwrap();
    definition.rules = ARBITRARY_RULES;
    crate::game::tests::synchronize_single_part_definition(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::SARPADIAN_EMPIRES_VOL_VII)
        .unwrap();
    choose(&mut game, "Twilight");
    let initial = game.players[0].life;
    activate(&mut game, PlayerId::One, source);
    let (_, mut game) = rebuild_current_checkpoint(&game, PlayerId::One, 105);
    drain_pending(&mut game);
    assert!(game.pending_decisions.is_empty());
    assert_eq!(game.players[0].life, initial + 24);
    let tokens = game
        .battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == ObjectKind::Token)
        .collect::<Vec<_>>();
    assert_eq!(tokens.len(), 2);
    for token in tokens {
        assert!(token.tapped);
        assert_eq!(
            (game.power(token), game.toughness(token)),
            (Some(2), Some(2))
        );
        assert_eq!(game.effective_subtypes(token), vec!["Goblin"]);
        assert_eq!(
            token
                .token_characteristics
                .unwrap()
                .rules()
                .ability_clauses(),
            &[crate::card::abilities::flying()]
        );
    }
    let (_, mut game) = rebuild_current_checkpoint(&game, PlayerId::One, 106);
    let permanent = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == source)
        .unwrap();
    // A label is display metadata: changing it must not change the bound token.
    permanent
        .chosen_tokens
        .get_mut("branch_output")
        .unwrap()
        .label = "Sunrise".into();
    permanent.tapped = false;
    activate(&mut game, PlayerId::One, source);
    drain_pending(&mut game);
    assert!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == ObjectKind::Token)
            .all(|token| game.effective_subtypes(token) == vec!["Goblin"])
    );
    let permanent = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == source)
        .unwrap();
    permanent.chosen_tokens.clear();
    permanent.tapped = false;
    activate(&mut game, PlayerId::One, source);
    drain_pending(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == ObjectKind::Token)
            .count(),
        4
    );
    assert_eq!(
        game.players[0].life,
        initial + 72,
        "a missing binding still resumes the sequence tail"
    );
}
