// Reading retired objects, pending events, and entry completions back.
//
// Split out of `wire.rs` for the source-size budget: the file next door reads
// zones and permanents, and these read the bookkeeping that travels beside
// them. Included textually, so the imports here are that module's.

pub(super) fn parse_detached_card(
    snapshot: &DetachedCardSnapshot,
    catalog: &CardCatalog,
) -> Result<CardInstance, String> {
    let mut card = card(
        GameObjectId(snapshot.object_id),
        snapshot.definition,
        player_from_index(snapshot.owner)?,
        catalog,
    )?;
    if let Some(part) = snapshot.copy_part_id {
        if !catalog.get(snapshot.definition).is_some_and(|definition| {
            matches!(
                definition.structure,
                crate::card::CardStructure::AlternateSpell {
                    alternate, kind: crate::card::AlternateSpellKind::Prepare, ..
                } if alternate.0 == part
            )
        }) {
            return Err("invalid detached prepare spell frame".into());
        }
        card.characteristics = CharacteristicSource::PartCopy {
            definition: snapshot.definition,
            part: CardPartId(part),
        };
    }
    Ok(card)
}

pub(super) fn parse_retired_objects(
    snapshots: &[RetiredObjectSnapshot],
    game: &Game,
) -> Result<BTreeMap<GameObjectId, RetiredObject>, String> {
    snapshots
        .iter()
        .map(|snapshot| match snapshot {
            RetiredObjectSnapshot::Card {
                card: snapshot,
                power,
                toughness,
                colors,
            } => {
                let parsed = parse_detached_card(snapshot, &game.catalog)?;
                Ok((
                    parsed.id,
                    RetiredObject::Card(crate::game::RetiredCard {
                        card: parsed,
                        stats: power
                            .zip(*toughness)
                            .map(|(power, toughness)| crate::CreatureStats { power, toughness }),
                        colors: *colors,
                    }),
                ))
            }
            RetiredObjectSnapshot::Stack { object } => {
                let parsed = parse_detached_stack(object, game)?;
                Ok((parsed.id, RetiredObject::Stack(Box::new(parsed))))
            }
            RetiredObjectSnapshot::Permanent {
                permanent,
                attachments,
                colors,
                power,
                toughness,
                mana_value,
                keywords,
            } => {
                let parsed = parse_detached_permanent(permanent, &game.catalog)?;
                Ok((
                    parsed.card.id,
                    RetiredObject::Permanent {
                        permanent: Box::new(parsed),
                        attachments: attachments.iter().copied().map(GameObjectId).collect(),
                        colors: *colors,
                        power: *power,
                        toughness: *toughness,
                        mana_value: *mana_value,
                        keywords: keywords.iter().copied().map(parse_keyword).collect(),
                    },
                ))
            }
        })
        .collect()
}

pub(super) fn parse_pending_events(
    snapshots: &[PendingEventSnapshot],
    catalog: &CardCatalog,
) -> Result<VecDeque<PendingEvent>, String> {
    snapshots
        .iter()
        .map(|snapshot| {
            Ok(PendingEvent {
                event: ReplaceableEvent::BattlefieldEntry(PendingBattlefieldEntry {
                    permanent: parse_detached_permanent(&snapshot.entry.permanent, catalog)?,
                    from: parse_zone_kind(snapshot.entry.from),
                    completion: parse_completion(snapshot.entry.completion)?,
                    redirected_to: snapshot.entry.redirected_to.map(parse_zone_kind),
                }),
                applied: snapshot
                    .applied
                    .iter()
                    .copied()
                    .map(|source| AbilitySourceRef {
                        object: GameObjectId(source.object),
                        ability: ability_origin_from_snapshot(source.ability),
                    })
                    .collect(),
                effects: snapshot
                    .effects
                    .iter()
                    .map(|effect| {
                        let context = parse_replacement_context_snapshot(effect.context)?;
                        if !replacement_effect_locator_matches_source(
                            &effect.effect,
                            context.source,
                        ) {
                            return Err(
                                "pending entry replacement locator disagrees with its source"
                                    .into(),
                            );
                        }
                        Ok(PendingReplacementEffect {
                            context,
                            effect: catalog_entry_replacement_effect(catalog, &effect.effect)?,
                        })
                    })
                    .collect::<Result<Vec<_>, String>>()?,
            })
        })
        .collect()
}

pub(super) const fn parse_zone_kind(zone: ZoneKindSnapshot) -> ZoneKind {
    match zone {
        ZoneKindSnapshot::Library => ZoneKind::Library,
        ZoneKindSnapshot::Hand => ZoneKind::Hand,
        ZoneKindSnapshot::Battlefield => ZoneKind::Battlefield,
        ZoneKindSnapshot::Graveyard => ZoneKind::Graveyard,
        ZoneKindSnapshot::Stack => ZoneKind::Stack,
        ZoneKindSnapshot::Exile => ZoneKind::Exile,
        ZoneKindSnapshot::Command => ZoneKind::Command,
    }
}

pub(super) fn parse_completion(
    completion: EntryCompletionSnapshot,
) -> Result<EntryCompletion, String> {
    match completion {
        EntryCompletionSnapshot::LandPlayed { seat } => Ok(EntryCompletion::LandPlayed {
            player: player_from_index(seat)?,
        }),
        EntryCompletionSnapshot::SpellResolved { card, definition } => {
            Ok(EntryCompletion::SpellResolved {
                card: GameObjectId(card),
                definition,
            })
        }
        EntryCompletionSnapshot::AttachSource { source } => Ok(EntryCompletion::AttachSource {
            source: GameObjectId(source),
        }),
        EntryCompletionSnapshot::AttachToHost { host } => Ok(EntryCompletion::AttachToHost {
            host: GameObjectId(host),
        }),
        EntryCompletionSnapshot::Attacking { defender } => Ok(EntryCompletion::Attacking {
            defender: match defender {
                AttackDefenderSnapshot::Player { seat } => {
                    AttackDefender::Player(player_from_index(seat)?)
                }
                AttackDefenderSnapshot::Planeswalker { object_id } => {
                    AttackDefender::Planeswalker(GameObjectId(object_id))
                }
            },
        }),
        EntryCompletionSnapshot::Setup => Ok(EntryCompletion::Setup),
        EntryCompletionSnapshot::None => Ok(EntryCompletion::None),
    }
}
