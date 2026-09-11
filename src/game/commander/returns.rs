//! Optional replacement of a move to hand or library (CR 903.9b).
use super::{
    DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone,
    Game, GameObjectId, PlayerId, ZoneKind, ZoneMoveCause, ZonePlacement,
};
use crate::game::{BattlefieldExitCompletion, PendingBattlefieldExitBatch, Target};

#[derive(Clone, Debug)]
pub(in crate::game) enum CommanderMove {
    Battlefield {
        batch: PendingBattlefieldExitBatch,
        index: usize,
    },
    Card {
        card: GameObjectId,
        destination: ZoneKind,
        cause: ZoneMoveCause,
        placement: ZonePlacement,
    },
    Spell {
        card: GameObjectId,
        destination: ZoneKind,
        placement: ZonePlacement,
    },
}

impl Game {
    pub(in crate::game) fn queue_commander_move(
        &mut self,
        owner: PlayerId,
        movement: CommanderMove,
    ) {
        let object = match &movement {
            CommanderMove::Battlefield { batch, index } => batch.moves[*index].object,
            CommanderMove::Card { card, .. } | CommanderMove::Spell { card, .. } => *card,
        };
        let name = self
            .commander_index(object)
            .and_then(|index| self.catalog.get(self.commanders[index].definition))
            .map_or("your commander", |card| card.name.as_str());
        let prompt = format!("Return {name} to the command zone instead?");
        let options = [
            (0, "Move to the command zone"),
            (1, "Keep the original destination"),
        ]
        .into_iter()
        .map(|(id, label)| DecisionOption {
            id,
            label: label.into(),
            card: None,
            members: Vec::new(),
            ability_text: None,
            zone: DecisionZone::None,
        })
        .collect();
        self.queue_decision(
            owner,
            prompt,
            DecisionVisibility::Public,
            DecisionPreference::PreferOption(0),
            1..=1,
            false,
            options,
            DecisionContinuation::CommanderMove {
                movement: Box::new(movement),
                completion: None,
            },
        );
    }

    pub(in crate::game) fn finish_commander_move(
        &mut self,
        movement: CommanderMove,
        command: bool,
        completion: Option<Box<BattlefieldExitCompletion>>,
    ) {
        match movement {
            CommanderMove::Battlefield { mut batch, index } => {
                if command {
                    batch.moves[index].destination = ZoneKind::Command;
                }
                if let Some(completion) = completion {
                    batch.completion = Some(Box::new(match batch.completion.take() {
                        None => *completion,
                        Some(earlier) => {
                            BattlefieldExitCompletion::Completions(vec![*earlier, *completion])
                        }
                    }));
                }
                self.continue_battlefield_exit_replacements(batch);
                return;
            }
            CommanderMove::Card {
                card,
                destination,
                cause,
                placement,
            } => {
                self.commander_move_answer = Some((card, command));
                self.move_card_target_to_zone(card, destination, cause, None, placement);
                self.commander_move_answer = None;
            }
            CommanderMove::Spell {
                card,
                destination,
                placement,
            } => {
                self.commander_move_answer = Some((card, command));
                if destination == ZoneKind::Hand {
                    self.return_spell_to_hand(card);
                } else {
                    self.put_spell_into_library(card, placement);
                }
                self.commander_move_answer = None;
            }
        }
        if let Some(completion) = completion {
            self.resume_battlefield_exit_completion(*completion, &[]);
        }
    }

    pub(in crate::game) fn commander_hidden_move_destination(
        &mut self,
        target: Target,
        destination: ZoneKind,
        cause: ZoneMoveCause,
        placement: ZonePlacement,
    ) -> Option<ZoneKind> {
        let (Target::Card(id) | Target::Spell(id)) = target else {
            return Some(destination);
        };
        if !matches!(destination, ZoneKind::Hand | ZoneKind::Library) || !self.is_commander(id) {
            return Some(destination);
        }
        if let Some((answered, command)) = self.commander_move_answer
            && answered == id
        {
            return Some(if command {
                ZoneKind::Command
            } else {
                destination
            });
        }
        let owner = self.commanders[self.commander_index(id)?].owner;
        let movement = match target {
            Target::Card(card) => CommanderMove::Card {
                card,
                destination,
                cause,
                placement,
            },
            Target::Spell(card) => CommanderMove::Spell {
                card,
                destination,
                placement,
            },
            _ => unreachable!(),
        };
        self.queue_commander_move(owner, movement);
        None
    }
}
