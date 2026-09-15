use super::*;

#[test]
fn object_zone_filters_preserve_live_identity_and_ignore_players_and_retired_objects() {
    const ZONES: &[ZoneKind] = &[
        ZoneKind::Battlefield,
        ZoneKind::Stack,
        ZoneKind::Hand,
        ZoneKind::Graveyard,
        ZoneKind::Exile,
        ZoneKind::Library,
        ZoneKind::Command,
    ];
    let mut game = ready_game();
    game.players[0].library.clear();
    game.players[1].library.clear();
    game.players[0]
        .library
        .push(card(191_006, cards::FOREST, PlayerId::One));
    let spell = card(191_000, cards::LIGHTNING_BOLT, PlayerId::One);
    let spell_id = spell.id;
    game.players[0].hand.push(spell);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.apply(
        PlayerId::One,
        cast_action(spell_id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .unwrap();
    let object = game.stack[0].clone();
    let permanent = creature(191_001, cards::GRIZZLY_BEARS, PlayerId::Two);
    let permanent_id = permanent.card.id;
    game.battlefield.push(permanent);
    game.players[0]
        .hand
        .push(card(191_002, cards::FOREST, PlayerId::One));
    game.players[0]
        .graveyard
        .push(card(191_003, cards::FOREST, PlayerId::One));
    game.players[0]
        .exile
        .push(card(191_004, cards::FOREST, PlayerId::One));
    game.players[0].command.push(card(
        191_005,
        cards::ROFELLOS_LLANOWAR_EMISSARY,
        PlayerId::One,
    ));
    let all = ObjectSetDef::Query(ObjectQueryDef::new(ObjectPredicateDef::Any, ZONES));
    let scoped = ScopedEffect::primary(EffectDef::None);
    let mut context = EffectResolutionContext::new(TriggerContext::empty());
    let mut targets = game.effect_objects(all, &object, &context, scoped);
    targets.extend([Target::Player(PlayerId::One), Target::Card(spell_id)]);
    context.bind_object_group(Binding!("mixed"), targets);
    for (zone, expected) in [
        (ZoneKind::Battlefield, Target::Permanent(permanent_id)),
        (ZoneKind::Stack, Target::Spell(object.id)),
        (ZoneKind::Hand, Target::Card(CardInstanceId(191_002))),
        (ZoneKind::Graveyard, Target::Card(CardInstanceId(191_003))),
        (ZoneKind::Exile, Target::Card(CardInstanceId(191_004))),
        (ZoneKind::Command, Target::Card(CardInstanceId(191_005))),
        (ZoneKind::Library, Target::Card(CardInstanceId(191_006))),
    ] {
        let actual = game.effect_objects(
            ObjectSetDef::InZone {
                objects: &const { ObjectSetDef::Binding(Binding!("mixed")) },
                zone,
            },
            &object,
            &context,
            scoped,
        );
        assert_eq!(actual, [expected], "{zone:?}");
        assert_eq!(
            game.source_object_set_targets(
                ObjectSetDef::InZone {
                    objects: &const {
                        ObjectSetDef::Query(ObjectQueryDef::new(ObjectPredicateDef::Any, ZONES))
                    },
                    zone,
                },
                object.id
            ),
            [expected],
            "source-relative {zone:?}",
        );
    }
    game.move_permanents_to_graveyard(&[permanent_id]);
    let remaining = game.effect_objects(
        ObjectSetDef::InZone {
            objects: &const { ObjectSetDef::Binding(Binding!("mixed")) },
            zone: ZoneKind::Graveyard,
        },
        &object,
        &context,
        scoped,
    );
    assert_eq!(
        remaining,
        [Target::Card(CardInstanceId(191_003))],
        "the binding does not follow a retired permanent"
    );
}
