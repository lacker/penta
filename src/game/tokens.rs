//! Creating tokens.
//!
//! A token is an ordinary permanent with no backing card, so everything about
//! it -- entry replacements, triggers, arriving tapped -- goes through the
//! same battlefield entry path as a card. What lives here is only the part
//! that differs: minting the object.

use super::{
    AppliedRuleDef, CardPartId, CharacteristicSource, CopiableCharacteristics, CounterKind,
    DoubleFacedCopiableCharacteristics, EntryCompletion, Game, GameObjectId, ObjectBacking,
    ObjectCharacteristics, ObjectInstance, ObjectKind, PendingBattlefieldEntry, Permanent,
    PlayerId, RetiredObject, TokenCharacteristics, ZoneKind,
};

impl Game {
    /// The exact source captured when a token was created, including from its
    /// last known permanent state after the token itself has left.
    pub(super) fn creating_source_of(&self, token: GameObjectId) -> Option<GameObjectId> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == token)
            .and_then(|permanent| permanent.created_by)
            .or_else(|| match self.retired_objects.get(&token) {
                Some(RetiredObject::Permanent { permanent, .. }) => permanent.created_by,
                Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => None,
            })
    }

    /// Mints the object shell shared by an authored token and a token copy.
    fn unbacked_token(
        &mut self,
        owner: PlayerId,
        characteristics: CharacteristicSource,
    ) -> ObjectInstance {
        ObjectInstance {
            id: self.allocate_object_id(),
            definition: ObjectKind::Token,
            owner,
            backing: ObjectBacking::None,
            characteristics,
            counters: crate::game::counters::Counters::new(),
        }
    }

    /// How many tokens one instruction actually makes for this player.
    ///
    /// "Twice that many of those tokens are created instead" is a
    /// replacement on the creation, so it applies wherever tokens are made
    /// and to copies as much as to fresh ones. Several doublers multiply,
    /// which is what each of them says on its own terms (CR 616.1).
    pub(super) fn tokens_created(&self, controller: PlayerId, count: usize) -> usize {
        let mut doublers = 0_u32;
        self.visit_player_static_rules(controller, |rule| {
            if rule == AppliedRuleDef::DoublesTokensCreated {
                doublers = doublers.saturating_add(1);
            }
        });
        // Bounded rather than trusting the board: a runaway multiplier must
        // not be able to ask for more tokens than a game could hold.
        count.saturating_mul(1_usize << doublers.min(8))
    }

    /// Publishes one instruction's worth of token creation, for the clauses
    /// that read "whenever you create one or more tokens". A token is
    /// created as it enters (CR 111.11), so an entry that was replaced or is
    /// still waiting on a decision is not in the batch.
    pub(super) fn capture_tokens_created(&mut self, controller: PlayerId, tokens: &[GameObjectId]) {
        let created = tokens
            .iter()
            .filter_map(|id| {
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == *id)
            })
            .map(|permanent| self.trigger_event_object(permanent))
            .collect::<Vec<_>>();
        if created.is_empty() {
            return;
        }
        self.capture_battlefield_triggers(&super::CommittedTriggerEvent::TokensCreated {
            tokens: created,
            controller,
        });
    }

    /// Test/setup shorthand for creating an ordinary unlinked token.
    #[cfg(test)]
    pub(super) fn create_token(&mut self, controller: PlayerId, token: TokenCharacteristics) {
        let created = self.create_token_from(controller, token, None);
        self.capture_tokens_created(controller, &[created]);
    }

    /// Puts one token onto the battlefield, remembering which permanent's
    /// ability made it. Only the cards that later refer to their own tokens
    /// pass a creator; for everything else the link is dead weight.
    pub(super) fn create_token_from(
        &mut self,
        controller: PlayerId,
        token: TokenCharacteristics,
        creator: Option<GameObjectId>,
    ) -> GameObjectId {
        self.create_token_arriving(controller, token, creator, false, None, None)
    }

    /// Creates one token whose committed battlefield incarnation becomes the
    /// host of `source`. The binding sits on the entry event rather than this
    /// resolving instruction because replacement effects can replace or
    /// otherwise delay the prospective token entry.
    pub(super) fn create_attached_token(
        &mut self,
        controller: PlayerId,
        token: TokenCharacteristics,
        source: GameObjectId,
        creator: GameObjectId,
    ) -> GameObjectId {
        let card = self.unbacked_token(controller, CharacteristicSource::Token(token));
        let mut permanent = Permanent::entering_token(
            card,
            token,
            controller,
            self.turns_started[controller.index()],
            self.turn,
        );
        permanent.created_by = Some(creator);
        let prospective = permanent.card.id;
        self.enqueue_battlefield_entry(PendingBattlefieldEntry {
            permanent,
            from: ZoneKind::Stack,
            completion: EntryCompletion::AttachSource { source },
            redirected_to: None,
        });
        self.successors
            .get(&prospective)
            .copied()
            .unwrap_or(prospective)
    }

    /// A token that arrives already attached to a permanent that is
    /// there. The mirror of [`Self::create_attached_token`], where the
    /// resolving permanent is the one that moves.
    pub(super) fn create_token_attached_to(
        &mut self,
        controller: PlayerId,
        token: TokenCharacteristics,
        host: GameObjectId,
        creator: GameObjectId,
    ) -> GameObjectId {
        let card = self.unbacked_token(controller, CharacteristicSource::Token(token));
        let mut permanent = Permanent::entering_token(
            card,
            token,
            controller,
            self.turns_started[controller.index()],
            self.turn,
        );
        permanent.created_by = Some(creator);
        let prospective = permanent.card.id;
        self.enqueue_battlefield_entry(PendingBattlefieldEntry {
            permanent,
            from: ZoneKind::Stack,
            completion: EntryCompletion::AttachToHost { host },
            redirected_to: None,
        });
        self.successors
            .get(&prospective)
            .copied()
            .unwrap_or(prospective)
    }

    /// The same, for a token whose card says it arrives tapped.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn create_token_arriving(
        &mut self,
        controller: PlayerId,
        token: TokenCharacteristics,
        creator: Option<GameObjectId>,
        tapped: bool,
        attacking: Option<crate::AttackDefender>,
        counters: Option<(CounterKind, u16)>,
    ) -> GameObjectId {
        let card = self.unbacked_token(controller, CharacteristicSource::Token(token));
        let mut permanent = Permanent::entering_token(
            card,
            token,
            controller,
            self.turns_started[controller.index()],
            self.turn,
        );
        permanent.created_by = creator;
        // Set before entry replacements run, the same point an as-enters
        // clause would set it.
        permanent.tapped = tapped;
        // "Create an Incubator token with X +1/+1 counters on it" is one
        // instruction: the counters are on it as it arrives rather than put
        // there afterwards, so an enters trigger reading its power sees them.
        if let Some((kind, amount)) = counters {
            permanent.add_counters(kind, amount);
        }
        let prospective = permanent.card.id;
        self.enqueue_battlefield_entry(PendingBattlefieldEntry {
            permanent,
            from: ZoneKind::Stack,
            completion: match attacking {
                Some(defender) => EntryCompletion::Attacking { defender },
                None => EntryCompletion::None,
            },
            redirected_to: None,
        });
        // A permanent takes a fresh identity as it actually arrives, so what
        // the caller gets back is the object that ended up on the
        // battlefield rather than the prospective one it was handed. An
        // entry still waiting on a decision has no successor yet, and gives
        // back the only id there is.
        self.successors
            .get(&prospective)
            .copied()
            .unwrap_or(prospective)
    }

    /// Creates a token with the complete copiable values of another permanent.
    /// Token nature belongs to the new object shell, independently of whether
    /// those values came from a printed card or another token.
    #[cfg(test)]
    pub(super) fn create_token_copy(
        &mut self,
        controller: PlayerId,
        copy: CopiableCharacteristics,
        double_faced: Option<DoubleFacedCopiableCharacteristics>,
        presented: CardPartId,
    ) -> GameObjectId {
        self.create_token_copy_with_completion(
            controller,
            copy,
            double_faced,
            presented,
            EntryCompletion::None,
            |_| {},
        )
    }

    /// Creates a token copy while remembering the exact source of the
    /// instruction that made it.
    pub(super) fn create_token_copy_from(
        &mut self,
        controller: PlayerId,
        copy: CopiableCharacteristics,
        double_faced: Option<DoubleFacedCopiableCharacteristics>,
        presented: CardPartId,
        creator: GameObjectId,
    ) -> GameObjectId {
        self.create_token_copy_with_completion(
            controller,
            copy,
            double_faced,
            presented,
            EntryCompletion::None,
            |permanent| permanent.created_by = Some(creator),
        )
    }

    /// Creates a token copy through the normal battlefield-entry pipeline,
    /// with an optional completion for the effect that is moving it there.
    /// A copied permanent spell uses this path because it becomes a token as
    /// it resolves rather than becoming an unbacked physical card.
    pub(super) fn create_token_copy_with_completion(
        &mut self,
        controller: PlayerId,
        copy: CopiableCharacteristics,
        double_faced: Option<DoubleFacedCopiableCharacteristics>,
        presented: CardPartId,
        completion: EntryCompletion,
        configure: impl FnOnce(&mut Permanent),
    ) -> GameObjectId {
        let source = match copy.base {
            ObjectCharacteristics::Card { definition, .. } => {
                CharacteristicSource::Copy(definition)
            }
            ObjectCharacteristics::Token { token, .. } => CharacteristicSource::Token(token),
            ObjectCharacteristics::Emblem { .. } => {
                unreachable!("an emblem cannot supply copiable permanent characteristics")
            }
            ObjectCharacteristics::FaceDown { face_down } => {
                CharacteristicSource::FaceDown(face_down)
            }
        };
        let card = self.unbacked_token(controller, source);
        let physical_part = double_faced
            .as_ref()
            .map_or(CardPartId::PRIMARY, |_| presented);
        let mut permanent = Permanent::entering(
            card,
            physical_part,
            controller,
            self.turns_started[controller.index()],
            self.turn,
        );
        permanent.copied_from = Some(copy.base);
        if let Some(double_faced) = double_faced {
            permanent.double_faced_token_copy = Some(double_faced);
        } else {
            permanent.copy_effect = Some(copy);
        }
        configure(&mut permanent);
        let prospective = permanent.card.id;
        self.enqueue_battlefield_entry(PendingBattlefieldEntry {
            permanent,
            from: ZoneKind::Stack,
            completion,
            redirected_to: None,
        });
        // The same fresh identity every arrival takes, for the same reason:
        // a clause naming what it just made has to name what is there.
        self.successors
            .get(&prospective)
            .copied()
            .unwrap_or(prospective)
    }
}
