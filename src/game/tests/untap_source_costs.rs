//! The untap symbol as an activated-ability cost.

use super::*;

static UNTAP_COSTS: [CostDef; 2] = [CostDef::Mana(ManaCost::new(2, 0)), CostDef::UntapSource];
static UNTAP_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated(
    "{2}, {Q}: Put a +1/+1 counter on this creature.",
    &UNTAP_COSTS,
    EffectDef::AddCounters {
        object: EffectRecipientDef::Source,
        kind: CounterKind::PlusOnePlusOne,
        amount: ValueDef::Constant(1),
    },
)];

fn untap_source_game() -> (Game, GameObjectId, Action) {
    let definition_id = CardDefinitionId::new(10_103);
    let mut definition = CardDefinition::new(
        definition_id,
        "Untap source cost test",
        CardSet::Magic2014,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = CardRules::new_artifact_creature(ManaCost::new(3, 0), &["Scarecrow"], 2, 2)
        .with_abilities(&UNTAP_ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).expect("the untap-cost fixture is valid");

    let mut source = creature(10_000, definition_id, PlayerId::One);
    source.tapped = true;
    source.entered_controller_turn = 0;
    let source_id = source.card.id;
    game.battlefield.push(source);
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    let action = Action::ActivateAbility {
        source: source_id,
        ability: activated_ability_for(&game, source_id, 0),
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
        mana_payment: None,
    };
    (game, source_id, action)
}

#[test]
fn untap_source_is_paid_before_the_ability_resolves() {
    let (mut game, source, action) = untap_source_game();
    assert!(game.legal_actions(PlayerId::One).contains(&action));

    game.apply(PlayerId::One, action)
        .expect("the offered untap-cost activation is legal");
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == source)
        .expect("the source remains on the battlefield");
    assert!(!permanent.tapped, "{{Q}} is paid during activation");
    assert_eq!(
        permanent.counters(CounterKind::PlusOnePlusOne),
        0,
        "the effect has not resolved yet",
    );
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.colorless, 0);
    assert_eq!(game.stack.len(), 1);

    pass_priority_pair(&mut game);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == source)
        .expect("the source remains after resolution");
    assert_eq!(permanent.counters(CounterKind::PlusOnePlusOne), 1);
}

#[test]
fn untap_source_requires_the_source_to_be_tapped() {
    let (mut game, source, action) = untap_source_game();
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == source)
        .expect("the source is on the battlefield")
        .tapped = false;

    assert!(!game.legal_actions(PlayerId::One).contains(&action));
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == source)
        .expect("the source is on the battlefield")
        .tapped = true;
    assert!(game.legal_actions(PlayerId::One).contains(&action));
}

#[test]
fn untap_symbol_observes_creature_summoning_sickness() {
    let (mut game, source, action) = untap_source_game();
    let entered_this_turn = game.turns_started[PlayerId::One.index()];
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == source)
        .expect("the source is on the battlefield")
        .entered_controller_turn = entered_this_turn;

    assert!(!game.legal_actions(PlayerId::One).contains(&action));
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == source)
        .expect("the source is on the battlefield")
        .entered_controller_turn = entered_this_turn.saturating_sub(1);
    assert!(game.legal_actions(PlayerId::One).contains(&action));
}
