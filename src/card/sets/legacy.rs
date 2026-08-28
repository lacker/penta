use super::{CardBehavior, CardRules, UNSUPPORTED_RULES, y1993, y1994, y1997, y2012, y2013};

// This is deliberately only an index: special hooks and the legacy
// CardDefinition::new compatibility keys resolve to card-local rules.
pub(in crate::card) const fn rules(behavior: CardBehavior) -> &'static CardRules {
    match behavior {
        CardBehavior::BloodBaronOfVizkopa => &y2013::dragons_maze::BLOOD_BARON_OF_VIZKOPA.rules,
        CardBehavior::Fireball => &y1993::alpha::FIREBALL.rules,
        CardBehavior::GoblinGrenade => &y1994::fallen_empires::GOBLIN_GRENADE.rules,
        CardBehavior::FellwarStone => &y1994::the_dark::FELLWAR_STONE.rules,
        CardBehavior::ReflectingPool => &y1997::tempest::REFLECTING_POOL.rules,
        CardBehavior::Balance => &y1993::alpha::BALANCE.rules,
        CardBehavior::LibraryOfAlexandria => &y1993::arabian_nights::LIBRARY_OF_ALEXANDRIA.rules,
        CardBehavior::Recall => &y1994::legends::RECALL.rules,
        CardBehavior::DustToDust => &y1994::the_dark::DUST_TO_DUST.rules,
        CardBehavior::EssenceScatter => &y2012::magic_2013::ESSENCE_SCATTER.rules,
        CardBehavior::Negate => &y2012::magic_2013::NEGATE.rules,
        CardBehavior::PillarOfFlame => &y2012::avacyn_restored::PILLAR_OF_FLAME.rules,
        CardBehavior::SphinxsRevelation => &y2012::return_to_ravnica::SPHINXS_REVELATION.rules,
        CardBehavior::TetravusDetach | CardBehavior::TetravusAssemble => {
            &y1994::antiquities::TETRAVUS.rules
        }
        CardBehavior::Mountain => &y1993::alpha::MOUNTAIN.rules,
        CardBehavior::Plains => &y1993::alpha::PLAINS.rules,
        CardBehavior::Unsupported => &UNSUPPORTED_RULES,
    }
}
