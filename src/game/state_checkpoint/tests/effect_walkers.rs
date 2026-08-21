use super::*;

#[test]
fn emperor_of_bones_end_step_trigger_reconstructs_through_its_follow_up() {
    let mut game = crate::game::tests::ready_game();
    game.battlefield.clear();
    let definition = crate::card::cards::EMPEROR_OF_BONES;
    let source_object = game
        .put_onto_battlefield(PlayerId::One, definition)
        .expect("Emperor of Bones enters");
    let source_ability = AbilityId(2);
    let parent = game
        .catalog
        .get(definition)
        .and_then(|card| card.part(CardPartId::PRIMARY))
        .and_then(|part| part.rules.ability(source_ability))
        .copied()
        .expect("the counter trigger is cataloged");
    let nested = semantics::child_abilities(&parent)
        .into_iter()
        .find(|ability| {
            ability
                .text
                .starts_with("At the beginning of the next end step")
        })
        .expect("the reanimation follow-up installs its sacrifice trigger");
    let DeclarativeAbilityDef::Triggered(triggered) = nested.definition else {
        panic!("the installed ability is triggered");
    };
    let effect = nested
        .declarative_effect()
        .expect("the installed sacrifice trigger is declarative");
    let origin = AbilityOrigin::Printed {
        definition,
        part: CardPartId::PRIMARY,
        ability: source_ability,
    };
    game.installed_triggers.push(InstalledTrigger {
        id: 0,
        event: triggered.event,
        capture: TriggerCapture {
            source: AbilitySourceRef {
                object: source_object,
                ability: origin,
            },
            presentation: ObjectCharacteristics::card(definition, CardPartId::PRIMARY),
            owner: PlayerId::One,
            controller: PlayerId::One,
            text: nested.text,
            target_defs: Vec::new(),
            targets: Vec::new(),
            effect,
            resolver: Game::ability_resolver(origin, nested),
            context: EffectResolutionContext::empty(),
            condition: triggered.condition,
            x: 0,
        },
        lifetime: InstalledTriggerLifetime::Once,
    });
    game.next_installed_trigger_id = 1;

    let (wire, rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 81_009);
    assert_eq!(wire["checkpoint"]["hasDeferredState"], json!(false));
    assert_eq!(rebuilt.installed_triggers, game.installed_triggers);
    assert_eq!(rebuilt.next_installed_trigger_id, 1);
}
