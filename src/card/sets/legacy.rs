use super::{
    CardBehavior, CardRules, PileChosen, PilesSeparated, UNSUPPORTED_RULES, y1993, y1994, y2011,
    y2012,
};

// This is deliberately only an index: special hooks and the legacy
// CardDefinition::new compatibility keys resolve to card-local rules.
pub(in crate::card) const fn rules(behavior: CardBehavior) -> &'static CardRules {
    match behavior {
        CardBehavior::AugurOfBolas => &y2012::magic_2013::AUGUR_OF_BOLAS.rules,
        CardBehavior::Fireball => &y1993::alpha::FIREBALL.rules,
        CardBehavior::GoblinGrenade => &y1994::fallen_empires::GOBLIN_GRENADE.rules,
        CardBehavior::Balance => &y1993::alpha::BALANCE.rules,
        CardBehavior::LibraryOfAlexandria => &y1993::arabian_nights::LIBRARY_OF_ALEXANDRIA.rules,
        CardBehavior::Recall => &y1994::legends::RECALL.rules,
        CardBehavior::DustToDust => &y1994::the_dark::DUST_TO_DUST.rules,
        CardBehavior::GrislySalvage => &y2012::return_to_ravnica::GRISLY_SALVAGE.rules,
        CardBehavior::Mulch => &y2011::innistrad::MULCH.rules,
        CardBehavior::TetravusDetach | CardBehavior::TetravusAssemble => {
            &y1994::antiquities::TETRAVUS.rules
        }
        CardBehavior::Mountain => &y1993::alpha::MOUNTAIN.rules,
        CardBehavior::Plains => &y1993::alpha::PLAINS.rules,
        CardBehavior::Unsupported => &UNSUPPORTED_RULES,
    }
}

pub(crate) fn piles_separated_resolver(key: &str) -> Option<PilesSeparated> {
    match key {
        "lilianaOfTheVeil.pilesSeparated" => Some(y2011::innistrad::LILIANA_PILES_SEPARATED),
        _ => None,
    }
}

pub(crate) fn pile_chosen_resolver(key: &str) -> Option<PileChosen> {
    match key {
        "lilianaOfTheVeil.pileChosen" => Some(y2011::innistrad::LILIANA_PILE_CHOSEN),
        _ => None,
    }
}
