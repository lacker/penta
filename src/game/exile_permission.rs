//! Permission to play a card from exile.
//!
//! Two shapes reach this: a card on an adventure, which its owner may cast
//! later as the creature half and which never lapses; and a card somebody
//! else's effect exiled and handed to a player for a while, which is played
//! for free and expires.

use super::{CardDefinition, Game, GameObjectId, PlayOptionDef, PlayerId};
use crate::card::{ManaCost, ZoneKind};

/// What a card in exile costs the player who may play it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ExilePlayCost {
    /// Its own cost, as printed.
    Printed,
    /// Waived entirely (CR 118.5): "you may play those cards without paying
    /// their mana costs".
    Free,
    /// "By paying an amount of {E} equal to its mana value rather than paying
    /// its mana cost." The mana cost goes away and the energy takes its
    /// place, so a card nobody has the energy for is not castable at all.
    EnergyEqualToManaValue,
    /// Foretell: the card's own foretell cost, which the card prints as an
    /// alternative cast. A foretold card lies face down, so until it is cast
    /// only its owner knows what it is.
    Foretell,
}

/// One card in exile somebody may play from there.
///
/// Several of the flags below are independent facts about one permission --
/// what it costs, whether the card lies face down, whether it may be played
/// at all yet -- so they stay separate rather than collapsing into a kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub(super) struct ExilePlayPermission {
    pub(super) card: GameObjectId,
    /// Who may play it. An adventure returns to its owner; a card taken off
    /// the top of somebody's library is played by whoever took it.
    pub(super) player: PlayerId,
    /// What playing it costs, which need not be what the card prints.
    pub(super) cost: ExilePlayCost,
    /// The turn this permission belongs to, as the turn count of the player
    /// whose turn it was. `None` never lapses, which is what an adventure
    /// means; anything else is gone once that turn is over.
    pub(super) until_end_of_turn: Option<(PlayerId, u32)>,
    /// Whether only the main half of an Adventure card may be played, which
    /// is what "as the creature, never as the adventure again" means
    /// (CR 715.3d).
    pub(super) adventure_return_only: bool,
    /// What a spell played under this permission costs on top of whatever
    /// [`Self::cost`] already says. Empty for every permission that adds
    /// nothing, which is all of them but Elite Spellbinder's.
    pub(super) surcharge: ManaCost,
    /// The earliest turn this permission may be used, as the turn count of
    /// the player whose turn it will be. Foretell is the only thing that
    /// sets it: a card exiled this turn is castable on a later one, and
    /// "later" is the whole cost of the two mana.
    pub(super) not_before_turn: Option<(PlayerId, u32)>,
    /// Whether the card lies face down in exile. Everybody may count how
    /// many there are; who may see what they are is
    /// [`Self::hidden_from_owner`].
    pub(super) face_down: bool,
    /// Whether this permission reaches playing a land. "You may play that
    /// card" does; "you may cast that card" does not, and a land exiled
    /// under a cast permission stays where it is (CR 305.1 -- playing a land
    /// is not casting a spell).
    pub(super) lands_may_be_played: bool,
    /// Whether the card is hidden from its own owner too. A card exiled
    /// face down may not be looked at unless something says otherwise
    /// (CR 713.2), and most things that exile face down do say so -- a
    /// foretold card or a hideaway land is its owner's to look at. Memory
    /// Jar is the other kind: nobody sees those hands until they come back.
    pub(super) hidden_from_owner: bool,
    /// Whether mana spent on this card may be of any colour, which is a
    /// property of the permission rather than of the card.
    pub(super) spend_any_color: bool,
    /// What has to be true where the card is played, asked then rather than
    /// where the permission was granted.
    pub(super) condition: Option<crate::card::ExilePlayConditionDef>,
    /// A permission to look and nothing more. Hideaway hides a card its
    /// controller may see and nobody may play until the land's own second
    /// ability says so, so the two halves are separate: this one records
    /// that the card is theirs to look at.
    pub(super) hidden_only: bool,
    /// The holder's turn whose end step this permission runs to, as their
    /// turn count. Unlike [`Self::until_end_of_turn`] this survives the turn
    /// it was granted on when that turn was somebody else's: "until your
    /// next end step" reaches across to the holder's own.
    pub(super) until_holder_end_step: Option<(PlayerId, u32)>,
    /// Where the card this permission names is. Almost every one of them
    /// is about a card in exile, which is what the zone defaults to; Emry
    /// hands out one about a card in a graveyard, and the two are told
    /// apart here rather than by two lists.
    pub(super) zone: ZoneKind,
    /// The pile this permission belongs to, named by the object whose
    /// resolution made it. "You may cast a spell from among cards exiled
    /// this way" is one permission over several cards: casting any of them
    /// spends it, so the rest go with it.
    pub(super) group: Option<GameObjectId>,
    /// A creature cast through this permission receives suspend's haste.
    pub(super) grants_haste: bool,
}

impl ExilePlayCost {
    /// The stable wire label for this cost. Written out rather than derived,
    /// because a `Debug` rendering is not a wire contract.
    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::Printed => "printed",
            Self::Free => "free",
            Self::EnergyEqualToManaValue => "energyEqualToManaValue",
            Self::Foretell => "foretell",
        }
    }

    /// The cost a label names. An unknown one is a refusal rather than a
    /// guess: reading it wrong would let a card be cast for nothing.
    pub(super) fn from_label(label: &str) -> Option<Self> {
        match label {
            "printed" => Some(Self::Printed),
            "free" => Some(Self::Free),
            "energyEqualToManaValue" => Some(Self::EnergyEqualToManaValue),
            "foretell" => Some(Self::Foretell),
            _ => None,
        }
    }
}

impl Game {
    /// Whether `player` may presently play `card` from exile with `option`.
    pub(super) fn exile_play_is_permitted(
        &self,
        definition: &CardDefinition,
        option: &PlayOptionDef,
        card: GameObjectId,
        player: PlayerId,
    ) -> bool {
        self.exile_play_permission(card, player)
            .is_some_and(|permission| {
                !permission.adventure_return_only
                    || Self::is_adventure_return_option(definition, option)
            })
    }

    /// The live permission `player` holds over `card`, if any.
    pub(super) fn exile_play_permission(
        &self,
        card: GameObjectId,
        player: PlayerId,
    ) -> Option<ExilePlayPermission> {
        self.exile_play_permissions
            .iter()
            .copied()
            .find(|permission| {
                permission.card == card
                    && permission.player == player
                    && permission.zone == ZoneKind::Exile
                    // A look is not a permission to play.
                    && !permission.hidden_only
                    // "During any turn you attacked with a Rogue": asked
                    // here, because the permission outlives the turn it was
                    // granted on.
                    && permission
                        .condition
                        .is_none_or(|condition| self.exile_play_condition_holds(condition, player))
                    && permission.until_end_of_turn.is_none_or(|(owner, turn)| {
                        self.turns_started[owner.index()] == turn && self.active_player == owner
                    })
                    // "Cast it on a later turn": the turn it was exiled on
                    // is not one of them, however long that turn runs.
                    && permission.not_before_turn.is_none_or(|(owner, turn)| {
                        self.turns_started[owner.index()] > turn || self.active_player != owner
                    })
                    // Live until the holder's own turn `turn` is over, which
                    // is what "until your next end step" reaches.
                    && permission
                        .until_holder_end_step
                        .is_none_or(|(holder, turn)| self.turns_started[holder.index()] <= turn)
            })
    }

    /// "You may cast that card", with nothing about the turn bounding it:
    /// what limits the permission is the condition the clause attaches
    /// below, asked again every time the card could be played.
    pub(super) fn permit_conditional_cast_while_exiled(
        &mut self,
        card: GameObjectId,
        player: PlayerId,
    ) {
        self.exile_play_permissions.push(ExilePlayPermission {
            card,
            player,
            cost: ExilePlayCost::Printed,
            until_end_of_turn: None,
            adventure_return_only: false,
            surcharge: ManaCost::default(),
            not_before_turn: None,
            face_down: false,
            hidden_only: false,
            spend_any_color: false,
            condition: None,
            until_holder_end_step: None,
            zone: ZoneKind::Exile,
            group: None,
            hidden_from_owner: false,
            lands_may_be_played: true,
            grants_haste: false,
        });
    }

    /// Records what a permission just granted asks for and allows: the
    /// colours its mana may be spent as, and what has to be true when the
    /// card is played.
    pub(super) fn qualify_exile_permission(
        &mut self,
        card: GameObjectId,
        spend_any_color: bool,
        condition: Option<crate::card::ExilePlayConditionDef>,
    ) {
        if let Some(permission) = self
            .exile_play_permissions
            .iter_mut()
            .rev()
            .find(|permission| permission.card == card)
        {
            permission.spend_any_color = spend_any_color;
            permission.condition = condition;
        }
    }

    /// Whether a permission's own condition is satisfied right now.
    fn exile_play_condition_holds(
        &self,
        condition: crate::card::ExilePlayConditionDef,
        player: PlayerId,
    ) -> bool {
        match condition {
            // What the turn was attacked with, not what is still standing:
            // a Rogue that attacked and then died still made this a turn
            // you attacked with a Rogue.
            crate::card::ExilePlayConditionDef::AttackedWithSubtypeThisTurn(subtype) => {
                self.attacked_subtypes_this_turn[player.index()].contains(&subtype)
            }
        }
    }

    /// "Look at the top four cards of your library, exile one face down."
    /// What it buys is the looking: playing it waits for whatever clause
    /// hid it to say so.
    pub(super) fn permit_look_while_exiled(&mut self, card: GameObjectId, player: PlayerId) {
        self.exile_play_permissions.push(ExilePlayPermission {
            card,
            player,
            cost: ExilePlayCost::Printed,
            until_end_of_turn: None,
            adventure_return_only: false,
            surcharge: ManaCost::default(),
            not_before_turn: None,
            face_down: true,
            hidden_only: true,
            spend_any_color: false,
            condition: None,
            until_holder_end_step: None,
            zone: ZoneKind::Exile,
            group: None,
            hidden_from_owner: false,
            lands_may_be_played: true,
            grants_haste: false,
        });
    }

    /// Narrows the permission just granted over `card` to casting alone:
    /// "you may cast that card" does not reach a land, where "you may play
    /// that card" does.
    pub(super) fn restrict_exile_permission_to_casting(&mut self, card: GameObjectId) {
        if let Some(permission) = self
            .exile_play_permissions
            .iter_mut()
            .rev()
            .find(|permission| permission.card == card)
        {
            permission.lands_may_be_played = false;
        }
    }

    /// Records that a card lies face down in exile and that nobody may look
    /// at it, which is what a card exiled face down means unless something
    /// says otherwise (CR 713.2). Both players may still count them.
    pub(super) fn hide_from_everyone_while_exiled(&mut self, card: GameObjectId, owner: PlayerId) {
        self.exile_play_permissions.push(ExilePlayPermission {
            card,
            player: owner,
            cost: ExilePlayCost::Printed,
            until_end_of_turn: None,
            adventure_return_only: false,
            surcharge: ManaCost::default(),
            not_before_turn: None,
            face_down: true,
            hidden_from_owner: true,
            hidden_only: true,
            spend_any_color: false,
            condition: None,
            until_holder_end_step: None,
            zone: ZoneKind::Exile,
            group: None,
            lands_may_be_played: true,
            grants_haste: false,
        });
    }

    /// Records that a card on an adventure may come back as the creature it
    /// is. The permission never lapses: a crown that never moves keeps it,
    /// and so does an adventure nobody takes.
    pub(super) fn permit_adventure_return(&mut self, card: GameObjectId, player: PlayerId) {
        self.exile_play_permissions.push(ExilePlayPermission {
            card,
            player,
            cost: ExilePlayCost::Printed,
            until_end_of_turn: None,
            adventure_return_only: true,
            surcharge: ManaCost::default(),
            not_before_turn: None,
            face_down: false,
            hidden_only: false,
            spend_any_color: false,
            condition: None,
            until_holder_end_step: None,
            zone: ZoneKind::Exile,
            group: None,
            hidden_from_owner: false,
            lands_may_be_played: true,
            grants_haste: false,
        });
    }

    /// "Until end of turn, you may play those cards without paying their
    /// mana costs."
    pub(super) fn permit_free_play_this_turn(&mut self, card: GameObjectId, player: PlayerId) {
        let active = self.active_player;
        self.exile_play_permissions.push(ExilePlayPermission {
            card,
            player,
            cost: ExilePlayCost::Free,
            until_end_of_turn: Some((active, self.turns_started[active.index()])),
            adventure_return_only: false,
            surcharge: ManaCost::default(),
            not_before_turn: None,
            face_down: false,
            hidden_only: false,
            spend_any_color: false,
            condition: None,
            until_holder_end_step: None,
            zone: ZoneKind::Exile,
            group: None,
            hidden_from_owner: false,
            lands_may_be_played: true,
            grants_haste: false,
        });
    }

    /// "Exile the top card of your library face down. You may look at and
    /// play that card this turn." Lying face down is the whole difference
    /// from the permission below: the cost is still owed either way, and
    /// only its owner knows what the card is.
    pub(super) fn permit_face_down_play_this_turn(&mut self, card: GameObjectId, player: PlayerId) {
        let active = self.active_player;
        self.exile_play_permissions.push(ExilePlayPermission {
            card,
            player,
            cost: ExilePlayCost::Printed,
            until_end_of_turn: Some((active, self.turns_started[active.index()])),
            adventure_return_only: false,
            surcharge: ManaCost::default(),
            not_before_turn: None,
            face_down: true,
            hidden_only: false,
            spend_any_color: false,
            condition: None,
            until_holder_end_step: None,
            zone: ZoneKind::Exile,
            group: None,
            hidden_from_owner: false,
            lands_may_be_played: true,
            grants_haste: false,
        });
    }

    /// "You may play that card until your next end step."
    ///
    /// Longer than a turn when the card was exiled on somebody else's: the
    /// end step it runs to is the holder's own, so a discard on their turn
    /// buys the whole of yours. Recorded as the holder's turn count at which
    /// it lapses, which is this turn when it is already theirs.
    pub(super) fn permit_play_until_your_next_end_step(
        &mut self,
        card: GameObjectId,
        player: PlayerId,
    ) {
        self.exile_play_permissions.push(ExilePlayPermission {
            card,
            player,
            cost: ExilePlayCost::Printed,
            until_end_of_turn: None,
            adventure_return_only: false,
            surcharge: ManaCost::default(),
            not_before_turn: None,
            face_down: false,
            hidden_only: false,
            spend_any_color: false,
            condition: None,
            until_holder_end_step: Some((
                player,
                if self.active_player == player {
                    self.turns_started[player.index()]
                } else {
                    // Their turn: "your next end step" is the one in the
                    // turn after this, so the permission outlives it.
                    self.turns_started[player.index()].saturating_add(1)
                },
            )),
            zone: ZoneKind::Exile,
            group: None,
            hidden_from_owner: false,
            lands_may_be_played: true,
            grants_haste: false,
        });
    }

    /// "You may play that card until the end of your next turn."
    ///
    /// Always the holder's following turn, whoever is active: on their own
    /// turn "your next turn" is the one after this, and on somebody else's
    /// it is the one about to start. Both are one more turn than they have
    /// begun, so the count is the same either way -- which is exactly what
    /// separates this from [`Self::permit_play_until_your_next_end_step`],
    /// where being active shortens the grant to tonight.
    pub(super) fn permit_play_until_end_of_your_next_turn(
        &mut self,
        card: GameObjectId,
        player: PlayerId,
    ) {
        self.exile_play_permissions.push(ExilePlayPermission {
            card,
            player,
            cost: ExilePlayCost::Printed,
            until_end_of_turn: None,
            adventure_return_only: false,
            surcharge: ManaCost::default(),
            not_before_turn: None,
            face_down: false,
            hidden_only: false,
            spend_any_color: false,
            condition: None,
            until_holder_end_step: Some((
                player,
                self.turns_started[player.index()].saturating_add(1),
            )),
            zone: ZoneKind::Exile,
            group: None,
            hidden_from_owner: false,
            lands_may_be_played: true,
            grants_haste: false,
        });
    }

    /// "You may cast that card." Unlike the free play above, the cost is
    /// still owed; what the permission grants is only that exile is a legal
    /// place to cast it from.
    pub(super) fn permit_cast_this_turn(&mut self, card: GameObjectId, player: PlayerId) {
        let active = self.active_player;
        self.exile_play_permissions.push(ExilePlayPermission {
            card,
            player,
            cost: ExilePlayCost::Printed,
            until_end_of_turn: Some((active, self.turns_started[active.index()])),
            adventure_return_only: false,
            surcharge: ManaCost::default(),
            not_before_turn: None,
            face_down: false,
            hidden_only: false,
            spend_any_color: false,
            condition: None,
            until_holder_end_step: None,
            zone: ZoneKind::Exile,
            group: None,
            hidden_from_owner: false,
            lands_may_be_played: true,
            grants_haste: false,
        });
    }

    /// "You may cast that card by paying an amount of {E} equal to its mana
    /// value rather than paying its mana cost." Nothing states a duration, so
    /// the permission lasts as long as the card sits in exile.
    pub(super) fn permit_energy_cast(&mut self, card: GameObjectId, player: PlayerId) {
        self.exile_play_permissions.push(ExilePlayPermission {
            card,
            player,
            cost: ExilePlayCost::EnergyEqualToManaValue,
            until_end_of_turn: None,
            adventure_return_only: false,
            surcharge: ManaCost::default(),
            not_before_turn: None,
            face_down: false,
            hidden_only: false,
            spend_any_color: false,
            condition: None,
            until_holder_end_step: None,
            zone: ZoneKind::Exile,
            group: None,
            hidden_from_owner: false,
            lands_may_be_played: true,
            grants_haste: false,
        });
    }

    /// "For as long as that card remains exiled, its owner may play it. A
    /// spell cast this way costs `surcharge` more to cast."
    ///
    /// The permission is the owner's rather than the exiler's, and it has no
    /// duration: nothing takes it back, so it lapses only when the card
    /// leaves exile by being played.
    pub(super) fn permit_owner_play_while_exiled(
        &mut self,
        card: GameObjectId,
        owner: PlayerId,
        surcharge: ManaCost,
    ) {
        self.exile_play_permissions.push(ExilePlayPermission {
            card,
            player: owner,
            cost: ExilePlayCost::Printed,
            until_end_of_turn: None,
            adventure_return_only: false,
            surcharge,
            not_before_turn: None,
            face_down: false,
            hidden_only: false,
            spend_any_color: false,
            condition: None,
            until_holder_end_step: None,
            zone: ZoneKind::Exile,
            group: None,
            hidden_from_owner: false,
            lands_may_be_played: true,
            grants_haste: false,
        });
    }

    /// "Exile this card from your hand face down. Cast it on a later turn
    /// for its foretell cost."
    pub(super) fn permit_foretold_cast(&mut self, card: GameObjectId, owner: PlayerId) {
        let turn = self.turns_started[owner.index()];
        self.exile_play_permissions.push(ExilePlayPermission {
            card,
            player: owner,
            cost: ExilePlayCost::Foretell,
            until_end_of_turn: None,
            adventure_return_only: false,
            surcharge: ManaCost::default(),
            not_before_turn: Some((owner, turn)),
            face_down: true,
            hidden_only: false,
            spend_any_color: false,
            condition: None,
            until_holder_end_step: None,
            zone: ZoneKind::Exile,
            group: None,
            hidden_from_owner: false,
            lands_may_be_played: true,
            grants_haste: false,
        });
    }

    /// "Exile this card from your hand. Cast it as a sorcery on a later turn
    /// without paying its mana cost." The plot cost was paid to get it here,
    /// so what remains is a free cast that has to wait for another turn. The
    /// card lies face up: everybody can see what is coming.
    pub(super) fn permit_plotted_cast(&mut self, card: GameObjectId, owner: PlayerId) {
        let turn = self.turns_started[owner.index()];
        self.exile_play_permissions.push(ExilePlayPermission {
            card,
            player: owner,
            cost: ExilePlayCost::Free,
            until_end_of_turn: None,
            adventure_return_only: false,
            surcharge: ManaCost::default(),
            not_before_turn: Some((owner, turn)),
            face_down: false,
            hidden_only: false,
            spend_any_color: false,
            condition: None,
            until_holder_end_step: None,
            zone: ZoneKind::Exile,
            group: None,
            hidden_from_owner: false,
            lands_may_be_played: true,
            grants_haste: false,
        });
    }

    /// Whether this exiled card is lying face down, which today means it was
    /// foretold. Its owner knows what it is; nobody else does.
    pub(super) fn exiled_card_is_face_down(&self, card: GameObjectId) -> bool {
        self.exile_play_permissions
            .iter()
            .any(|permission| permission.card == card && permission.face_down)
    }

    /// Whether nobody at all may look at this exiled card, its owner
    /// included. False for every face-down exile that hands its owner a
    /// look, which is most of them.
    pub(super) fn exiled_card_is_hidden_from_owner(&self, card: GameObjectId) -> bool {
        self.exile_play_permissions
            .iter()
            .any(|permission| permission.card == card && permission.hidden_from_owner)
    }

    /// One player's exile as another sees it. A card lying face down is
    /// absent from the list rather than shown, unless the viewer is the one
    /// who put it there and the clause that hid it left them a look.
    pub(super) fn observed_exile(
        &self,
        owner: PlayerId,
        viewer: PlayerId,
    ) -> Vec<super::PublicCard> {
        self.players[owner.index()]
            .exile
            .iter()
            .filter(|card| {
                !self.exiled_card_is_face_down(card.id)
                    || (viewer == owner && !self.exiled_card_is_hidden_from_owner(card.id))
            })
            .map(|card| (card.id, card.definition))
            .collect()
    }

    /// How many cards are lying face down in one player's exile. Both
    /// players may count them; only their owner knows what they are.
    pub(super) fn face_down_exile_size(&self, owner: PlayerId) -> usize {
        self.players[owner.index()]
            .exile
            .iter()
            .filter(|card| self.exiled_card_is_face_down(card.id))
            .count()
    }

    /// What this player owes on top of a card's own cost to play it out of
    /// exile, which is nothing unless a permission says otherwise.
    pub(super) fn exile_play_surcharge(&self, card: GameObjectId, player: PlayerId) -> ManaCost {
        self.exile_play_permission(card, player)
            .map_or_else(ManaCost::default, |permission| permission.surcharge)
    }

    pub(super) fn exile_cast_grants_haste(&self, card: GameObjectId, player: PlayerId) -> bool {
        self.exile_play_permission(card, player)
            .is_some_and(|permission| permission.grants_haste)
    }

    /// The energy `player` owes to cast `card` from exile, if that is how the
    /// permission they hold over it is paid.
    pub(super) fn exile_energy_cost(&self, card: GameObjectId, player: PlayerId) -> Option<u16> {
        let permission = self.exile_play_permission(card, player)?;
        if permission.cost != ExilePlayCost::EnergyEqualToManaValue {
            return None;
        }
        let (_, instance) = self.card_in_nonbattlefield_zone(card)?;
        Some(
            self.catalog
                .get(instance.definition)?
                .rules
                .printed_mana_cost()
                .mana_value(),
        )
    }

    /// Drops the permission a play has just consumed.
    /// The live permission `player` holds over a card in a graveyard, which
    /// is what Emry's ability hands out.
    pub(super) fn graveyard_cast_permission(
        &self,
        card: GameObjectId,
        player: PlayerId,
    ) -> Option<ExilePlayPermission> {
        self.exile_play_permissions
            .iter()
            .copied()
            .find(|permission| {
                permission.card == card
                    && permission.player == player
                    && permission.zone == ZoneKind::Graveyard
                    && permission.until_end_of_turn.is_none_or(|(owner, turn)| {
                        self.turns_started[owner.index()] == turn && self.active_player == owner
                    })
            })
    }

    /// "Choose target artifact card in your graveyard. You may cast that
    /// card this turn." The cost is still owed; what the permission grants
    /// is that the graveyard is a legal place to cast it from.
    pub(super) fn permit_graveyard_cast_this_turn(&mut self, card: GameObjectId, player: PlayerId) {
        let active = self.active_player;
        self.exile_play_permissions.push(ExilePlayPermission {
            card,
            player,
            cost: ExilePlayCost::Printed,
            until_end_of_turn: Some((active, self.turns_started[active.index()])),
            adventure_return_only: false,
            surcharge: ManaCost::default(),
            not_before_turn: None,
            face_down: false,
            hidden_only: false,
            spend_any_color: false,
            condition: None,
            until_holder_end_step: None,
            zone: ZoneKind::Graveyard,
            group: None,
            hidden_from_owner: false,
            lands_may_be_played: true,
            grants_haste: false,
        });
    }

    /// Puts the permission just granted over `card` into `group`'s pile.
    pub(super) fn group_last_exile_permission(&mut self, card: GameObjectId, group: GameObjectId) {
        if let Some(permission) = self
            .exile_play_permissions
            .iter_mut()
            .rev()
            .find(|permission| permission.card == card)
        {
            permission.group = Some(group);
        }
    }

    /// Drops just this card's permission, leaving the rest of its pile
    /// alone. A card the offer could not use -- a land, where the permission
    /// is to cast, or one that has already left the zone -- was never the
    /// spell the pile's one permission was spent on, so the cards beside it
    /// keep theirs.
    pub(super) fn drop_exile_play_permission(&mut self, card: GameObjectId) {
        self.exile_play_permissions
            .retain(|permission| permission.card != card);
    }

    pub(super) fn consume_exile_play_permission(&mut self, card: GameObjectId) {
        // A permission over a pile is one permission however many cards it
        // covers, so using it takes the whole pile's with it.
        let groups = self
            .exile_play_permissions
            .iter()
            .filter(|permission| permission.card == card)
            .filter_map(|permission| permission.group)
            .collect::<Vec<_>>();
        self.exile_play_permissions.retain(|permission| {
            permission.card != card
                && !permission
                    .group
                    .is_some_and(|group| groups.contains(&group))
        });
    }
}
