//! Eternal Masters cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};

// EMA 6 — Coalition Honor Guard (reprint)
const COALITION_HONOR_GUARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2001::apocalypse::COALITION_HONOR_GUARD,
    "2c7c2b5c-634a-4d83-81bc-c6128e3ac339",
    "Eric Peterson",
);

// EMA 45 — Deep Analysis (reprint)
const DEEP_ANALYSIS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2002::torment::DEEP_ANALYSIS,
    "821cc8b6-eb2e-4441-8d88-c54cb44ab024",
    "Jesper Ejsing",
);

// EMA 119 — Beetleback Chief (reprint)
const BEETLEBACK_CHIEF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2012::planechase_2012::BEETLEBACK_CHIEF,
    "779d4745-ff14-4c79-b2c8-8e273faf7375",
    "Wayne England",
);

// EMA 139 — Mogg War Marshal (reprint)
const MOGG_WAR_MARSHAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2006::time_spiral::MOGG_WAR_MARSHAL,
    "deed0a5a-6662-460c-bd78-e3d95e8bc83e",
    "Jesper Ejsing",
);

// EMA 191 — Werebear (reprint)
const WEREBEAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2001::odyssey::WEREBEAR,
    "224ea635-b95b-4803-8716-edd4cb655923",
    "Filip Burburan",
);

// EMA 225 — Mana Crypt (reprint)
const MANA_CRYPT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1994::harper_prism_book_promos::MANA_CRYPT,
    "0cb33b46-4d1b-4f97-bfdc-d815aee111da",
    "Matt Stewart",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    COALITION_HONOR_GUARD_REPRINT,
    DEEP_ANALYSIS_REPRINT,
    BEETLEBACK_CHIEF_REPRINT,
    MOGG_WAR_MARSHAL_REPRINT,
    WEREBEAR_REPRINT,
    MANA_CRYPT_REPRINT,
];
