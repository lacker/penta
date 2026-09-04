//! Through the Omenpaths reprints cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::sets::y2025::marvels_spider_man::{
    MULTIVERSAL_PASSAGE, OMINOUS_ASYLUM, SAVAGE_MANSION, SINISTER_HIDEOUT, SUBURBAN_SANCTUARY,
    UNIVERSITY_CAMPUS,
};

// OM1 181 — Multiversal Passage (reprint)
const MULTIVERSAL_PASSAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &MULTIVERSAL_PASSAGE,
    "21502958-a8e3-494a-9be9-bebbbb1dd9dc",
    "Daren Bader",
);
// OM1 182 — Ominous Asylum (reprint)
const OMINOUS_ASYLUM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &OMINOUS_ASYLUM,
    "371b03a1-7707-4a8a-8c0e-0272418c801f",
    "Daniel Ljunggren",
);
// OM1 183 — Savage Mansion (reprint)
const SAVAGE_MANSION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &SAVAGE_MANSION,
    "c172cdb5-aa2c-419d-b8ab-4795f4b7e160",
    "Vincent Proce",
);
// OM1 184 — Sinister Hideout (reprint)
const SINISTER_HIDEOUT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &SINISTER_HIDEOUT,
    "c417f8ce-e156-4c9a-af30-792606d861bd",
    "Julian Kok Joon Wen",
);
// OM1 185 — Suburban Sanctuary (reprint)
const SUBURBAN_SANCTUARY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &SUBURBAN_SANCTUARY,
    "cabf021b-23e9-404d-90c6-eef629e1283e",
    "Victor Sales",
);
// OM1 186 — University Campus (reprint)
const UNIVERSITY_CAMPUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &UNIVERSITY_CAMPUS,
    "cd4b9fc5-fe3d-41d9-9d0e-77f1aebef618",
    "Randy Gallegos",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    MULTIVERSAL_PASSAGE_REPRINT,
    OMINOUS_ASYLUM_REPRINT,
    SAVAGE_MANSION_REPRINT,
    SINISTER_HIDEOUT_REPRINT,
    SUBURBAN_SANCTUARY_REPRINT,
    UNIVERSITY_CAMPUS_REPRINT,
];
