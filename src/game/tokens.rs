//! Creating tokens.
//!
//! A token is an ordinary permanent with no backing card, so everything about
//! it -- entry replacements, triggers, arriving tapped -- goes through the
//! same battlefield entry path as a card. What lives here is only the part
//! that differs: minting the object.

use super::{
    CardPartId, CharacteristicSource, CopiableCharacteristics, CounterKind,
    DoubleFacedCopiableCharacteristics, EntryCompletion, Game, GameObjectId, ObjectBacking,
    ObjectCharacteristics, ObjectInstance, ObjectKind, PendingBattlefieldEntry, Permanent,
    PlayerId, TokenCharacteristics, ZoneKind,
};

impl Game {
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
            counters: [0; CounterKind::COUNT],
        }
    }

    /// Test/setup shorthand for creating an ordinary unlinked token.
    #[cfg(test)]
    pub(super) fn create_token(&mut self, controller: PlayerId, token: TokenCharacteristics) {
        self.create_token_from(controller, token, None);
    }

    /// Puts one token onto the battlefield, remembering which permanent's
    /// ability made it. Only the cards that later refer to their own tokens
    /// pass a creator; for everything else the link is dead weight.
    pub(super) fn create_token_from(
        &mut self,
        controller: PlayerId,
        token: TokenCharacteristics,
        creator: Option<GameObjectId>,
    ) {
        self.create_token_arriving(controller, token, creator, false, None, None);
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
    ) {
        let card = self.unbacked_token(controller, CharacteristicSource::Token(token));
        let permanent = Permanent::entering_token(
            card,
            token,
            controller,
            self.turns_started[controller.index()],
        );
        self.enqueue_battlefield_entry(PendingBattlefieldEntry {
            permanent,
            from: ZoneKind::Stack,
            completion: EntryCompletion::AttachSource { source },
            redirected_to: None,
        });
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
    pub(super) fn create_token_copy(
        &mut self,
        controller: PlayerId,
        copy: CopiableCharacteristics,
        double_faced: Option<DoubleFacedCopiableCharacteristics>,
        presented: CardPartId,
    ) {
        let source = match copy.base {
            ObjectCharacteristics::Card { definition, .. } => {
                CharacteristicSource::Copy(definition)
            }
            ObjectCharacteristics::Token { token, .. } => CharacteristicSource::Token(token),
            ObjectCharacteristics::Emblem { .. } => {
                unreachable!("an emblem cannot supply copiable permanent characteristics")
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
        );
        permanent.copied_from = Some(copy.base);
        if let Some(double_faced) = double_faced {
            permanent.double_faced_token_copy = Some(double_faced);
        } else {
            permanent.copy_effect = Some(copy);
        }
        self.enqueue_battlefield_entry(PendingBattlefieldEntry {
            permanent,
            from: ZoneKind::Stack,
            completion: EntryCompletion::None,
            redirected_to: None,
        });
    }
}
