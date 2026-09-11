//! Source-faithful lists from the September 8, 2026 MTGO Standard Challenge 32.
//!
//! These inventories include unsupported cards. A WOE–HOB Standard gameplay
//! profile and playable-deck registration are separate follow-up work.

deck!(
    izzet_spellementals_fazparte,
    "woe_hob_standard",
    "izzet_spellementals_fazparte.yaml",
    "Returns `fazparte`'s Izzet Spellementals list from the September 8, 2026 MTGO Challenge."
);

deck!(
    izzet_spellementals_darth_vaner,
    "woe_hob_standard",
    "izzet_spellementals_darth_vaner.yaml",
    "Returns `Darth_Vaner`'s Izzet Spellementals list from the September 8, 2026 MTGO Challenge."
);

deck!(
    boros_dwarves_l1x0,
    "woe_hob_standard",
    "boros_dwarves_l1x0.yaml",
    "Returns `L1X0`'s Boros Dwarves list from the September 8, 2026 MTGO Challenge."
);

deck!(
    jund_aggro_univerce,
    "woe_hob_standard",
    "jund_aggro_univerce.yaml",
    "Returns `Univerce`'s Jund Aggro list from the September 8, 2026 MTGO Challenge."
);

deck!(
    four_color_legends_burnet7,
    "woe_hob_standard",
    "4c_legends_burnet7.yaml",
    "Returns `burnet7`'s 4c Legends list from the September 8, 2026 MTGO Challenge."
);

deck!(
    dimir_aggro_franbenitez91,
    "woe_hob_standard",
    "dimir_aggro_franbenitez91.yaml",
    "Returns `FranBenitez91`'s Dimir Aggro list from the September 8, 2026 MTGO Challenge."
);

deck!(
    dimir_aggro_makaaaa,
    "woe_hob_standard",
    "dimir_aggro_makaaaa.yaml",
    "Returns `makaaaa`'s Dimir Aggro list from the September 8, 2026 MTGO Challenge."
);

deck!(
    jund_aggro_capitano_cl,
    "woe_hob_standard",
    "jund_aggro_capitano_cl.yaml",
    "Returns `Capitano_CL`'s Jund Aggro list from the September 8, 2026 MTGO Challenge."
);

deck!(
    selesnya_landfall_sorryimsotilted,
    "woe_hob_standard",
    "selesnya_landfall_sorryimsotilted.yaml",
    "Returns `SorryImSoTilted`'s Selesnya Landfall list from the September 8, 2026 MTGO Challenge."
);

deck!(
    bant_airbending_protopulse,
    "woe_hob_standard",
    "bant_airbending_protopulse.yaml",
    "Returns `ProtoPulse`'s Bant Airbending list from the September 8, 2026 MTGO Challenge."
);

deck!(
    azorius_momo_sbantwin,
    "woe_hob_standard",
    "azorius_momo_sbantwin.yaml",
    "Returns `SbanTwin`'s Azorius Momo list from the September 8, 2026 MTGO Challenge."
);

deck!(
    boros_tokens_whensmarvel,
    "woe_hob_standard",
    "boros_tokens_whensmarvel.yaml",
    "Returns `WhensMarvel`'s Boros Tokens list from the September 8, 2026 MTGO Challenge."
);

deck!(
    jund_aggro_komattaman,
    "woe_hob_standard",
    "jund_aggro_komattaman.yaml",
    "Returns `komattaman`'s Jund Aggro list from the September 8, 2026 MTGO Challenge."
);

deck!(
    four_five_color_control_noob,
    "woe_hob_standard",
    "4_5c_control_noob.yaml",
    "Returns `__Noob__`'s 4/5C Control list from the September 8, 2026 MTGO Challenge."
);

deck!(
    izzet_spellementals_peter780108,
    "woe_hob_standard",
    "izzet_spellementals_peter780108.yaml",
    "Returns `peter780108`'s Izzet Spellementals list from the September 8, 2026 MTGO Challenge."
);

deck!(
    izzet_spellementals_ale_mtg,
    "woe_hob_standard",
    "izzet_spellementals_ale_mtg.yaml",
    "Returns `Ale_Mtg`'s Izzet Spellementals list from the September 8, 2026 MTGO Challenge."
);

#[cfg(test)]
mod tests {
    use crate::Deck;

    type DeckBuilder = fn() -> Deck;

    #[test]
    fn event_90673_decks_resolve_every_card_and_preserve_published_sizes() {
        let decks: &[(DeckBuilder, usize)] = &[
            (super::izzet_spellementals_fazparte, 61),
            (super::izzet_spellementals_darth_vaner, 61),
            (super::boros_dwarves_l1x0, 60),
            (super::jund_aggro_univerce, 60),
            (super::four_color_legends_burnet7, 60),
            (super::dimir_aggro_franbenitez91, 60),
            (super::dimir_aggro_makaaaa, 60),
            (super::jund_aggro_capitano_cl, 60),
            (super::selesnya_landfall_sorryimsotilted, 60),
            (super::bant_airbending_protopulse, 60),
            (super::azorius_momo_sbantwin, 60),
            (super::boros_tokens_whensmarvel, 60),
            (super::jund_aggro_komattaman, 60),
            (super::four_five_color_control_noob, 60),
            (super::izzet_spellementals_peter780108, 60),
            (super::izzet_spellementals_ale_mtg, 60),
        ];
        for (build, main_count) in decks {
            let deck = build();
            assert_eq!(deck.main.len(), *main_count);
            assert_eq!(deck.sideboard.len(), 15);
        }
    }
}
