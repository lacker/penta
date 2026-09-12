use std::collections::{BTreeSet, HashMap, HashSet};
use std::error::Error;
use std::fmt;

use crate::CardDefinitionId;
use crate::Format;
use crate::card::{
    CardCatalog, CardDefinition, CardType, CompanionConditionDef, ImplementationStatus, ManaCost,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Deck {
    pub main: Vec<CardDefinitionId>,
    pub sideboard: Vec<CardDefinitionId>,
    /// The physical cards designated to begin in the command zone. They are
    /// separate from both the library and sideboard.
    #[serde(default)]
    pub commanders: Vec<CardDefinitionId>,
}

impl Deck {
    /// Checks implementation coverage for commanders, main-deck cards, and sideboard cards.
    /// This is independent of format legality and reads the supplied catalog
    /// each time, so completing a card automatically updates the result.
    ///
    /// # Errors
    /// Returns [`DeckError::UnknownCard`] for an absent definition, or
    /// [`DeckError::UnsupportedCards`] with sorted, distinct card names.
    pub fn validate_supported_cards(&self, catalog: &CardCatalog) -> Result<(), DeckError> {
        let mut unsupported = BTreeSet::new();
        for &id in self
            .main
            .iter()
            .chain(&self.sideboard)
            .chain(&self.commanders)
        {
            let card = catalog.get(id).ok_or(DeckError::UnknownCard(id))?;
            if card.implementation_status() == ImplementationStatus::Unsupported {
                unsupported.insert(card.name.clone());
            }
        }
        if unsupported.is_empty() {
            Ok(())
        } else {
            Err(DeckError::UnsupportedCards(
                unsupported.into_iter().collect(),
            ))
        }
    }

    /// Checks this deck against the default Eternal Central construction rules.
    ///
    /// # Errors
    ///
    /// Returns [`DeckError`] when the deck size, card identities, banned list,
    /// restricted list, or copy limits are invalid.
    pub fn validate(self, catalog: &CardCatalog) -> Result<ValidatedDeck, DeckError> {
        self.validate_for_format(catalog, Format::OldSchool9394)
    }

    /// Checks this deck against the construction rules and card legality of
    /// `format`.
    ///
    /// # Errors
    ///
    /// Returns [`DeckError`] when the deck size, card identities, format
    /// legality, banned list, restricted list, or copy limits are invalid.
    pub fn validate_for_format(
        self,
        catalog: &CardCatalog,
        format: Format,
    ) -> Result<ValidatedDeck, DeckError> {
        for id in &self.commanders {
            if catalog.get(*id).is_none() {
                return Err(DeckError::UnknownCard(*id));
            }
        }
        // Commander deck construction is intentionally not enforced yet:
        // cEDH can load authored games while the complete shared Commander
        // validator (including colour identity and the command zone) lands.
        // Still reject an ID the catalog does not know, including a designated
        // commander, so a game never starts from an invented card identity.
        if format.defers_deck_legality() {
            for id in self.main.iter().chain(&self.sideboard) {
                if catalog.get(*id).is_none() {
                    return Err(DeckError::UnknownCard(*id));
                }
            }
            return Ok(ValidatedDeck(self));
        }
        let format_rules = format.rules();
        if self.main.len() < format_rules.minimum_main_deck_size {
            return Err(DeckError::MainDeckTooSmall {
                actual: self.main.len(),
                minimum: format_rules.minimum_main_deck_size,
            });
        }
        if self.sideboard.len() > format_rules.maximum_sideboard_size {
            return Err(DeckError::SideboardTooLarge {
                actual: self.sideboard.len(),
                maximum: format_rules.maximum_sideboard_size,
            });
        }

        let mut counts = HashMap::<CardDefinitionId, usize>::new();
        for id in self.main.iter().chain(&self.sideboard) {
            let Some(card) = catalog.get(*id) else {
                return Err(DeckError::UnknownCard(*id));
            };
            if !catalog.is_allowed_in(*id, format) {
                return Err(DeckError::CardNotAllowed {
                    card: card.name.clone(),
                    format,
                });
            }
            if catalog.is_banned_in(*id, format) {
                return Err(DeckError::BannedCard(card.name.clone()));
            }
            *counts.entry(*id).or_default() += 1;
        }

        for (id, count) in counts {
            let Some(card) = catalog.get(id) else {
                return Err(DeckError::UnknownCard(id));
            };
            let limit = if card.is_basic_land() {
                usize::MAX
            } else if catalog.is_restricted_in(id, format) {
                1
            } else {
                format_rules.maximum_copies
            };
            if count > limit {
                return Err(DeckError::TooManyCopies {
                    card: card.name.clone(),
                    count,
                    limit,
                });
            }
        }

        Ok(ValidatedDeck(self))
    }

    /// Whether `companion` may be this deck's companion: it has to print a
    /// companion condition, sit in the sideboard rather than the deck, and
    /// find that condition met by the main deck (CR 702.139a).
    #[must_use]
    pub fn companion_is_legal(&self, catalog: &CardCatalog, companion: CardDefinitionId) -> bool {
        if !self.sideboard.contains(&companion) || self.main.contains(&companion) {
            return false;
        }
        catalog
            .get(companion)
            .and_then(CardDefinition::companion_condition)
            .is_some_and(|condition| companion_condition_is_met(condition, catalog, &self.main))
    }

    /// Checks this deck as a Commander deck led by `commanders`.
    ///
    /// This is deck construction only: the engine plays no format with a
    /// command zone, so a validated commander deck is a legal list rather
    /// than a game that can be started from it. What is checked is what the
    /// singleton rules say -- who may lead (CR 903.3), that a second leader
    /// is one the first is allowed to take, that no leader is also in the
    /// deck, and that nothing else is duplicated (CR 903.5b) -- and
    /// deliberately not colour identity, which nothing here can yet compute.
    ///
    /// # Errors
    ///
    /// Returns [`DeckError`] when a commander is unknown, may not lead, or
    /// may not be paired with the other, when one also appears in the deck,
    /// or when the list is the wrong size or not singleton.
    pub fn validate_as_commander_deck(
        self,
        catalog: &CardCatalog,
        commanders: &[CardDefinitionId],
    ) -> Result<ValidatedDeck, DeckError> {
        let leaders = Self::commander_definitions(catalog, commanders)?;
        Self::commanders_may_lead_together(&leaders)?;
        for leader in &leaders {
            if self.main.contains(&leader.id) {
                return Err(DeckError::CommanderInDeck(leader.name.clone()));
            }
        }
        // The commanders are among the hundred; the list beside them is the
        // rest of it.
        let expected = COMMANDER_DECK_SIZE - leaders.len();
        if self.main.len() != expected {
            return Err(DeckError::MainDeckTooSmall {
                actual: self.main.len(),
                minimum: expected,
            });
        }
        self.main_deck_is_singleton(catalog)?;
        Ok(ValidatedDeck(Deck {
            commanders: commanders.to_vec(),
            ..self
        }))
    }

    fn commander_definitions<'a>(
        catalog: &'a CardCatalog,
        commanders: &[CardDefinitionId],
    ) -> Result<Vec<&'a CardDefinition>, DeckError> {
        if commanders.is_empty() || commanders.len() > MAXIMUM_COMMANDERS {
            return Err(DeckError::WrongNumberOfCommanders(commanders.len()));
        }
        commanders
            .iter()
            .map(|commander| {
                catalog
                    .get(*commander)
                    .ok_or(DeckError::UnknownCard(*commander))
            })
            .collect()
    }

    /// Who may lead, and who may lead beside whom. One commander answers for
    /// itself (CR 903.3); two require a printed pairing permission. Both
    /// Partner and Choose a Background are symmetric deck facts even when
    /// their printed wording appears on only one of the two cards.
    fn commanders_may_lead_together(leaders: &[&CardDefinition]) -> Result<(), DeckError> {
        match leaders {
            [] => Err(DeckError::WrongNumberOfCommanders(0)),
            [commander] if commander.may_be_commander() => Ok(()),
            [commander] => Err(DeckError::NotALegalCommander(commander.name.clone())),
            [first, second] => {
                let background_pair = (first.may_be_commander()
                    && first.may_choose_a_background()
                    && second.is_background())
                    || (second.may_be_commander()
                        && second.may_choose_a_background()
                        && first.is_background());
                let partner_pair = first.may_be_commander()
                    && second.may_be_commander()
                    && first.has_partner()
                    && second.has_partner();
                if background_pair || partner_pair {
                    Ok(())
                } else {
                    Err(DeckError::CommandersDoNotPair {
                        first: first.name.clone(),
                        second: second.name.clone(),
                    })
                }
            }
            _ => Err(DeckError::WrongNumberOfCommanders(leaders.len())),
        }
    }

    fn main_deck_is_singleton(&self, catalog: &CardCatalog) -> Result<(), DeckError> {
        let mut counts = HashMap::<CardDefinitionId, usize>::new();
        for id in &self.main {
            if catalog.get(*id).is_none() {
                return Err(DeckError::UnknownCard(*id));
            }
            *counts.entry(*id).or_default() += 1;
        }
        for (id, count) in counts {
            let Some(card) = catalog.get(id) else {
                return Err(DeckError::UnknownCard(id));
            };
            if count > 1 && !card.is_basic_land() {
                return Err(DeckError::TooManyCopies {
                    card: card.name.clone(),
                    count,
                    limit: 1,
                });
            }
        }
        Ok(())
    }
}

/// What a companion asks of the deck it sits beside, answered over the cards
/// that deck actually holds.
///
/// The starting deck is what a companion reads (CR 702.139a), so this takes
/// a list rather than a game: the answer is fixed before the first turn and
/// nothing that happens afterwards changes it.
#[must_use]
pub fn companion_condition_is_met(
    condition: CompanionConditionDef,
    catalog: &CardCatalog,
    starting_deck: &[CardDefinitionId],
) -> bool {
    let cards = || starting_deck.iter().filter_map(|id| catalog.get(*id));
    match condition {
        // A card with no mana cost has mana value 0 (CR 202.3b), which is
        // what makes the lands in a Lurrus deck legal rather than fatal.
        CompanionConditionDef::PermanentManaValueAtMost(limit) => cards()
            .filter(|card| card.is_permanent_card())
            .all(|card| card.rules.mana_cost().map_or(0, ManaCost::mana_value) <= limit),
        CompanionConditionDef::EveryPermanentHasAnActivatedAbility => cards()
            .filter(|card| card.is_permanent_card())
            .all(CardDefinition::has_an_activated_ability),
        CompanionConditionDef::NonlandNamesAreDistinct => {
            let mut seen = HashSet::new();
            cards()
                .filter(|card| !card.rules.has_type(CardType::Land))
                .all(|card| seen.insert(card.name.as_str()))
        }
    }
}

/// A deck may be led by one commander, or by two where a printed permission
/// pairs them (CR 903.3, CR 702.124a).
const MAXIMUM_COMMANDERS: usize = 2;

/// A Commander deck is a hundred cards counting the commander (CR 903.5a).
const COMMANDER_DECK_SIZE: usize = 100;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedDeck(Deck);

impl ValidatedDeck {
    pub(crate) fn into_parts(
        self,
    ) -> (
        Vec<CardDefinitionId>,
        Vec<CardDefinitionId>,
        Vec<CardDefinitionId>,
    ) {
        (self.0.main, self.0.sideboard, self.0.commanders)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeckError {
    MainDeckTooSmall {
        actual: usize,
        minimum: usize,
    },
    SideboardTooLarge {
        actual: usize,
        maximum: usize,
    },
    UnknownCard(CardDefinitionId),
    /// Distinct unsupported card names across the main deck and sideboard.
    UnsupportedCards(Vec<String>),
    CardNotAllowed {
        card: String,
        format: Format,
    },
    BannedCard(String),
    /// The named card is neither a legendary creature nor a card that prints
    /// permission to lead a deck.
    NotALegalCommander(String),
    /// The commander was also listed among the ninety-nine.
    CommanderInDeck(String),
    /// Nothing printed on either lets these two lead the same deck.
    CommandersDoNotPair {
        first: String,
        second: String,
    },
    /// A deck is led by one commander, or by two that pair.
    WrongNumberOfCommanders(usize),
    TooManyCopies {
        card: String,
        count: usize,
        limit: usize,
    },
}

impl fmt::Display for DeckError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MainDeckTooSmall { actual, minimum } => write!(
                formatter,
                "main deck has {actual} cards; at least {minimum} are required"
            ),
            Self::SideboardTooLarge { actual, maximum } => write!(
                formatter,
                "sideboard has {actual} cards; at most {maximum} are allowed"
            ),
            Self::UnknownCard(id) => write!(formatter, "unknown card definition ID {id:?}"),
            Self::UnsupportedCards(cards) => {
                write!(
                    formatter,
                    "deck contains unsupported cards: {}",
                    cards.join(", ")
                )
            }
            Self::CardNotAllowed { card, format } => {
                write!(formatter, "{card} is not legal in {format}")
            }
            Self::BannedCard(card) => write!(formatter, "{card} is banned"),
            Self::NotALegalCommander(card) => {
                write!(formatter, "{card} cannot be your commander")
            }
            Self::CommanderInDeck(card) => write!(
                formatter,
                "{card} is the commander and cannot also be in the deck"
            ),
            Self::CommandersDoNotPair { first, second } => write!(
                formatter,
                "{first} and {second} cannot lead the same deck together"
            ),
            Self::WrongNumberOfCommanders(count) => {
                write!(
                    formatter,
                    "a deck is led by one or two commanders, not {count}"
                )
            }
            Self::TooManyCopies { card, count, limit } => {
                write!(
                    formatter,
                    "{card} appears {count} times; the limit is {limit}"
                )
            }
        }
    }
}

impl Error for DeckError {}

#[cfg(test)]
mod tests {
    use super::{Deck, DeckError};
    use crate::CardDefinitionId;
    use crate::card::sets;
    use crate::card::{
        AbilityDef, CardCatalog, CardComposition, CardDefinition, CardRules, CardSupertype,
        DeckConstructionDef, ManaCost, cards,
    };

    fn catalog() -> CardCatalog {
        crate::card::catalog().expect("catalog builds")
    }

    fn support_catalog(main_complete: bool, sideboard_complete: bool) -> (CardCatalog, Deck) {
        let main = CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000015f92");
        let sideboard = CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000015f93");
        let rules = |complete| {
            if complete {
                CardRules::new_creature(crate::mana_cost!("{1}"), &["Construct"], 1, 1)
            } else {
                CardRules::unsupported()
            }
        };
        let catalog = CardCatalog::new([
            CardDefinition::new(main, "Main Card", sets::alpha::SET, rules(main_complete)),
            CardDefinition::new(
                sideboard,
                "Sideboard Card",
                sets::alpha::SET,
                rules(sideboard_complete),
            ),
        ])
        .unwrap();
        let deck = Deck {
            main: vec![main, main],
            sideboard: vec![sideboard, main],
            commanders: Vec::new(),
        };
        (catalog, deck)
    }

    #[test]
    fn supported_card_validation_reports_both_zones_and_tracks_catalog_changes() {
        let (catalog, deck) = support_catalog(false, false);
        let error = deck.validate_supported_cards(&catalog).unwrap_err();
        assert_eq!(
            error,
            DeckError::UnsupportedCards(vec!["Main Card".into(), "Sideboard Card".into()])
        );
        assert_eq!(
            error.to_string(),
            "deck contains unsupported cards: Main Card, Sideboard Card"
        );

        let (main_implemented, _) = support_catalog(true, false);
        assert_eq!(
            deck.validate_supported_cards(&main_implemented),
            Err(DeckError::UnsupportedCards(vec!["Sideboard Card".into()]))
        );
        let (both_implemented, _) = support_catalog(true, true);
        assert_eq!(deck.validate_supported_cards(&both_implemented), Ok(()));
    }

    #[test]
    fn supported_card_validation_includes_designated_commanders() {
        let (catalog, source) = support_catalog(false, true);
        let commander = source.main[0];
        let mut deck = Deck {
            commanders: vec![commander],
            main: Vec::new(),
            sideboard: Vec::new(),
        };
        assert_eq!(
            deck.validate_supported_cards(&catalog),
            Err(DeckError::UnsupportedCards(vec!["Main Card".into()]))
        );
        let (complete, _) = support_catalog(true, true);
        assert_eq!(deck.validate_supported_cards(&complete), Ok(()));
        let unknown = CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000099995");
        deck.commanders = vec![unknown];
        assert_eq!(
            deck.validate_supported_cards(&catalog),
            Err(DeckError::UnknownCard(unknown))
        );
    }

    #[test]
    fn supported_card_validation_rejects_unknown_cards_in_either_zone() {
        let (catalog, deck) = support_catalog(true, true);
        let unknown = CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000015f94");
        for in_sideboard in [false, true] {
            let mut deck = deck.clone();
            if in_sideboard {
                deck.sideboard.push(unknown);
            } else {
                deck.main.push(unknown);
            }
            assert_eq!(
                deck.validate_supported_cards(&catalog),
                Err(DeckError::UnknownCard(unknown))
            );
        }
    }

    /// The catalog plus one Background, which the real one has none of: the
    /// cube prints no Legendary Enchantment -- Background, so the only way to
    /// exercise the pairing is to build one and hand it over.
    fn catalog_with_a_background() -> (CardCatalog, CardDefinitionId) {
        let id = CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000015f91");
        let mut background = CardDefinition::new(
            id,
            "Test Background",
            sets::commander_legends_baldurs_gate::SET,
            crate::card::CardRules::unsupported(),
        );
        background.rules = CardRules::new_enchantment(ManaCost::new(0, 0))
            .with_supertype(CardSupertype::Legendary)
            .with_subtypes(&["Background"]);
        let composition = CardComposition::single(background.name.clone(), background.rules);
        background.parts = composition.parts;
        background.structure = composition.structure;
        background.play_options = composition.play_options;
        let mut definitions: Vec<CardDefinition> =
            catalog().definitions().into_iter().cloned().collect();
        definitions.push(background);
        (
            CardCatalog::new(definitions).expect("the catalog still builds"),
            id,
        )
    }

    fn catalog_with_partners() -> (CardCatalog, CardDefinitionId, CardDefinitionId) {
        let first = CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000090002");
        let second = CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000090003");
        let partner = |id, name| {
            CardDefinition::new(
                id,
                name,
                sets::commander_legends_baldurs_gate::SET,
                CardRules::new_creature(ManaCost::new(0, 0), &["Test"], 1, 1)
                    .with_supertype(CardSupertype::Legendary)
                    .with_ability(AbilityDef::deck_construction(
                        "Partner",
                        DeckConstructionDef::Partner,
                        "A symmetric commander pairing permission.",
                    )),
            )
        };
        let mut definitions: Vec<CardDefinition> =
            catalog().definitions().into_iter().cloned().collect();
        definitions.extend([
            partner(first, "Test Partner One"),
            partner(second, "Test Partner Two"),
        ]);
        (
            CardCatalog::new(definitions).expect("the catalog still builds"),
            first,
            second,
        )
    }

    /// Ninety-nine distinct cards, which is a legal Commander list's size
    /// whatever those cards happen to be. Anything named is skipped so the
    /// caller can lead with it.
    fn ninety_nine(catalog: &CardCatalog, skip: &[CardDefinitionId]) -> Vec<CardDefinitionId> {
        let main: Vec<CardDefinitionId> = catalog
            .definitions()
            .iter()
            .map(|definition| definition.id)
            .filter(|id| !skip.contains(id))
            .take(99)
            .collect();
        assert_eq!(main.len(), 99, "the catalog is large enough to fill a list");
        main
    }

    /// The ordinary permission is the type line, and a creature without the
    /// supertype has neither.
    #[test]
    fn a_legendary_creature_can_and_an_ordinary_one_cannot() {
        let catalog = catalog();

        assert!(
            catalog
                .get(cards::EMRY_LURKER_OF_THE_LOCH)
                .expect("cataloged")
                .may_be_commander(),
        );
        assert!(
            !catalog
                .get(cards::GRIZZLY_BEARS)
                .expect("cataloged")
                .may_be_commander(),
        );
    }

    #[test]
    fn a_hundred_singleton_cards_led_by_emry_are_legal() {
        let catalog = catalog();
        let deck = Deck {
            main: ninety_nine(&catalog, &[cards::EMRY_LURKER_OF_THE_LOCH]),
            sideboard: Vec::new(),
            commanders: Vec::new(),
        };

        deck.validate_as_commander_deck(&catalog, &[cards::EMRY_LURKER_OF_THE_LOCH])
            .expect("ninety-nine distinct cards and a legal leader");
    }

    #[test]
    fn commander_validation_preserves_the_designated_physical_card() {
        let catalog = catalog();
        let deck = Deck {
            main: ninety_nine(&catalog, &[cards::EMRY_LURKER_OF_THE_LOCH]),
            sideboard: Vec::new(),
            commanders: Vec::new(),
        };

        let (_, _, commanders) = deck
            .validate_as_commander_deck(&catalog, &[cards::EMRY_LURKER_OF_THE_LOCH])
            .expect("the list validates")
            .into_parts();
        assert_eq!(commanders, vec![cards::EMRY_LURKER_OF_THE_LOCH]);
    }

    #[test]
    fn cedh_defers_construction_but_rejects_unknown_card_identities() {
        let catalog = catalog();
        Deck {
            main: Vec::new(),
            sideboard: Vec::new(),
            commanders: vec![cards::EMRY_LURKER_OF_THE_LOCH],
        }
        .validate_for_format(&catalog, crate::Format::Cedh)
        .expect("cEDH construction validation is deliberately deferred");

        let error = Deck {
            main: vec![CardDefinitionId::from_uuid(
                "00000000-0000-0000-0000-000000999999",
            )],
            sideboard: Vec::new(),
            commanders: Vec::new(),
        }
        .validate_for_format(&catalog, crate::Format::Cedh)
        .expect_err("an unknown identity still cannot start a game");
        assert!(matches!(error, DeckError::UnknownCard(_)));
    }

    #[test]
    fn partner_commanders_pair_in_either_designation_order() {
        let (catalog, first, second) = catalog_with_partners();
        let mut main = ninety_nine(&catalog, &[first, second]);
        main.pop();
        let deck = Deck {
            main,
            sideboard: Vec::new(),
            commanders: Vec::new(),
        };

        deck.clone()
            .validate_as_commander_deck(&catalog, &[first, second])
            .expect("two Partner cards lead together");
        deck.validate_as_commander_deck(&catalog, &[second, first])
            .expect("Partner has no primary commander");
    }

    #[test]
    fn a_card_without_the_permission_cannot_lead() {
        let catalog = catalog();
        let deck = Deck {
            main: ninety_nine(&catalog, &[cards::GRIZZLY_BEARS]),
            sideboard: Vec::new(),
            commanders: Vec::new(),
        };

        let error = deck
            .validate_as_commander_deck(&catalog, &[cards::GRIZZLY_BEARS])
            .expect_err("a Grizzly Bears leads nothing");

        assert!(matches!(error, DeckError::NotALegalCommander(_)));
    }

    #[test]
    fn the_commander_may_not_also_be_one_of_the_ninety_nine() {
        let catalog = catalog();
        let mut main = ninety_nine(&catalog, &[cards::EMRY_LURKER_OF_THE_LOCH]);
        main[0] = cards::EMRY_LURKER_OF_THE_LOCH;
        let deck = Deck {
            main,
            sideboard: Vec::new(),
            commanders: Vec::new(),
        };

        let error = deck
            .validate_as_commander_deck(&catalog, &[cards::EMRY_LURKER_OF_THE_LOCH])
            .expect_err("the leader is not also in the deck");

        assert!(matches!(error, DeckError::CommanderInDeck(_)));
    }

    /// Singleton is the point of the format: a second copy of anything but a
    /// basic land is illegal, and the size check must not mask it.
    #[test]
    fn a_second_copy_of_a_nonbasic_is_illegal() {
        let catalog = catalog();
        let mut main = ninety_nine(
            &catalog,
            &[cards::EMRY_LURKER_OF_THE_LOCH, cards::GRIZZLY_BEARS],
        );
        main[0] = cards::GRIZZLY_BEARS;
        main[1] = cards::GRIZZLY_BEARS;
        let deck = Deck {
            main,
            sideboard: Vec::new(),
            commanders: Vec::new(),
        };

        let error = deck
            .validate_as_commander_deck(&catalog, &[cards::EMRY_LURKER_OF_THE_LOCH])
            .expect_err("two of something is not singleton");

        assert!(matches!(error, DeckError::TooManyCopies { count: 2, .. }));
    }

    #[test]
    fn basic_lands_are_exempt_from_singleton() {
        let catalog = catalog();
        let mut main = ninety_nine(&catalog, &[cards::EMRY_LURKER_OF_THE_LOCH, cards::FOREST]);
        main[0] = cards::FOREST;
        main[1] = cards::FOREST;
        let deck = Deck {
            main,
            sideboard: Vec::new(),
            commanders: Vec::new(),
        };

        deck.validate_as_commander_deck(&catalog, &[cards::EMRY_LURKER_OF_THE_LOCH])
            .expect("any number of basics is legal");
    }

    #[test]
    fn ninety_eight_cards_are_not_a_deck() {
        let catalog = catalog();
        let mut main = ninety_nine(&catalog, &[cards::EMRY_LURKER_OF_THE_LOCH]);
        main.pop();
        let deck = Deck {
            main,
            sideboard: Vec::new(),
            commanders: Vec::new(),
        };

        let error = deck
            .validate_as_commander_deck(&catalog, &[cards::EMRY_LURKER_OF_THE_LOCH])
            .expect_err("a hundred counts the commander");

        assert!(matches!(
            error,
            DeckError::MainDeckTooSmall {
                actual: 98,
                minimum: 99
            }
        ));
    }

    /// Choosing a Background: the one pairing the deck layer knows, and the
    /// three ways of getting it wrong.
    mod backgrounds {
        use super::{Deck, DeckError, cards, catalog, catalog_with_a_background, ninety_nine};

        /// Gut prints "Choose a Background", so a Background may lead beside
        /// them -- and the deck beside two commanders is ninety-eight.
        #[test]
        fn a_background_may_lead_beside_a_commander_that_chose_one() {
            let (catalog, background) = catalog_with_a_background();
            let mut main = ninety_nine(&catalog, &[cards::GUT_TRUE_SOUL_ZEALOT, background]);
            main.pop();
            let deck = Deck {
                main,
                sideboard: Vec::new(),
                commanders: Vec::new(),
            };

            deck.clone()
                .validate_as_commander_deck(&catalog, &[cards::GUT_TRUE_SOUL_ZEALOT, background])
                .expect("Gut chose a Background and this is one");
            deck.validate_as_commander_deck(&catalog, &[background, cards::GUT_TRUE_SOUL_ZEALOT])
                .expect("a Background has no primary commander designation");
        }

        /// Emry may lead, but they print no Background clause, so nothing
        /// leads beside them.
        #[test]
        fn a_commander_that_chose_nothing_leads_alone() {
            let (catalog, background) = catalog_with_a_background();
            let mut main = ninety_nine(&catalog, &[cards::EMRY_LURKER_OF_THE_LOCH, background]);
            main.pop();
            let deck = Deck {
                main,
                sideboard: Vec::new(),
                commanders: Vec::new(),
            };

            let error = deck
                .validate_as_commander_deck(&catalog, &[cards::EMRY_LURKER_OF_THE_LOCH, background])
                .expect_err("they never chose a Background");

            assert!(matches!(error, DeckError::CommandersDoNotPair { .. }));
        }

        /// And what Gut chose has to actually be a Background: another legendary
        /// creature is not one, however legendary it is.
        #[test]
        fn the_second_commander_has_to_be_a_background() {
            let catalog = catalog();
            let mut main = ninety_nine(
                &catalog,
                &[cards::GUT_TRUE_SOUL_ZEALOT, cards::EMRY_LURKER_OF_THE_LOCH],
            );
            main.pop();
            let deck = Deck {
                main,
                sideboard: Vec::new(),
                commanders: Vec::new(),
            };

            let error = deck
                .validate_as_commander_deck(
                    &catalog,
                    &[cards::GUT_TRUE_SOUL_ZEALOT, cards::EMRY_LURKER_OF_THE_LOCH],
                )
                .expect_err("Emry is a commander, but she is not a Background");

            assert!(matches!(error, DeckError::CommandersDoNotPair { .. }));
        }

        /// A Background cannot lead by itself: it is a second commander or it is
        /// nothing.
        #[test]
        fn a_background_cannot_lead_alone() {
            let (catalog, background) = catalog_with_a_background();
            let deck = Deck {
                main: ninety_nine(&catalog, &[background]),
                sideboard: Vec::new(),
                commanders: Vec::new(),
            };

            let error = deck
                .validate_as_commander_deck(&catalog, &[background])
                .expect_err("nothing on it says it can be your commander");

            assert!(matches!(error, DeckError::NotALegalCommander(_)));
        }

        /// Two is the most a deck is led by, and none is not a deck.
        #[test]
        fn a_deck_is_led_by_one_or_two() {
            let (catalog, background) = catalog_with_a_background();
            let deck = Deck {
                main: ninety_nine(&catalog, &[]),
                sideboard: Vec::new(),
                commanders: Vec::new(),
            };

            assert!(matches!(
                deck.clone()
                    .validate_as_commander_deck(&catalog, &[])
                    .expect_err("a deck has a commander"),
                DeckError::WrongNumberOfCommanders(0),
            ));
            assert!(matches!(
                deck.validate_as_commander_deck(
                    &catalog,
                    &[
                        cards::GUT_TRUE_SOUL_ZEALOT,
                        background,
                        cards::EMRY_LURKER_OF_THE_LOCH
                    ],
                )
                .expect_err("three is more than any pairing allows"),
                DeckError::WrongNumberOfCommanders(3),
            ));
        }
    }
}
