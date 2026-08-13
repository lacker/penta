use penta::{Deck, Format, Game, GameResult, HandcraftedPolicy, PlayerId, RandomPolicy, play_game};
use penta::{card, decks};

const GAMES_PER_SEAT: u64 = 25;
const ACTION_LIMIT: usize = 50_000;

/// Every built-in deck, grouped by the format it is legal in. A deck that is
/// only validated and never played can hide a stall behind a legal decklist,
/// so each format's whole tranche runs here.
fn tranches() -> [(Format, Vec<(&'static str, Deck)>); 2] {
    [
        (
            Format::OldSchool9394,
            vec![
                ("Goblins", decks::goblins()),
                ("Sligh", decks::sligh()),
                ("Artifacts", decks::artifacts()),
                ("Robots", decks::robots()),
                ("The Deck", decks::the_deck()),
                ("Mono Black", decks::mono_black()),
                ("White Weenie", decks::white_weenie()),
                ("Erhnamgeddon", decks::erhnamgeddon()),
                ("Counterburn", decks::counterburn()),
                ("Lions/Dib", decks::lions_dib()),
                ("BWR Aggro", decks::bwr_aggro()),
                ("GR Aggro", decks::gr_aggro()),
                ("Troll Disk", decks::troll_disk()),
                ("Jeskai Aggro", decks::jeskai_aggro()),
                ("Lion Dib Bolt", decks::lions_dib_bolt()),
            ],
        ),
        (
            Format::IsdDgmStandard,
            vec![
                (
                    "Briksza Naya",
                    decks::isd_dgm_standard::naya_midrange_rudy_briksza(),
                ),
                (
                    "Greer G/R",
                    decks::isd_dgm_standard::gr_aggro_joseph_greer(),
                ),
                (
                    "Fyrberg B/G",
                    decks::isd_dgm_standard::bg_midrange_mike_fyrberg(),
                ),
                (
                    "Smith Naya",
                    decks::isd_dgm_standard::naya_midrange_jimmie_smith(),
                ),
                (
                    "McDuffie UWR",
                    decks::isd_dgm_standard::uwr_flash_korey_mcduffie(),
                ),
                (
                    "Lorren U/W",
                    decks::isd_dgm_standard::uw_flash_phillip_lorren(),
                ),
                ("Arch U/W", decks::isd_dgm_standard::uw_flash_clayton_arch()),
                (
                    "Kuenzinger Junk",
                    decks::isd_dgm_standard::junk_reanimator_drew_kuenzinger(),
                ),
                (
                    "Anderson Omnidoor",
                    decks::isd_dgm_standard::omnidoor_thragfire_todd_anderson(),
                ),
                (
                    "Braun-Duin Naya",
                    decks::isd_dgm_standard::naya_midrange_brian_braun_duin(),
                ),
            ],
        ),
    ]
}

struct Tally {
    wins: u64,
    losses: u64,
    draws: u64,
}

impl Tally {
    const fn new() -> Self {
        Self {
            wins: 0,
            losses: 0,
            draws: 0,
        }
    }

    fn add(&mut self, other: &Self) {
        self.wins += other.wins;
        self.losses += other.losses;
        self.draws += other.draws;
    }

    /// The handcrafted policy's share of decided games, in tenths of a
    /// percent, or `None` when every game was a draw.
    const fn win_rate_tenths(&self) -> Option<u64> {
        match self.wins + self.losses {
            0 => None,
            decided => Some(self.wins * 1_000 / decided),
        }
    }
}

fn report(label: &str, tally: &Tally) {
    let Tally {
        wins,
        losses,
        draws,
    } = *tally;
    match tally.win_rate_tenths() {
        Some(tenths) => println!(
            "{label:16}: {wins:>4} wins, {losses:>4} losses, {draws:>4} draws \
             ({}.{}% of decided games)",
            tenths / 10,
            tenths % 10
        ),
        None => println!("{label:16}: {wins:>4} wins, {losses:>4} losses, {draws:>4} draws"),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let catalog = card::catalog()?;
    let mut overall = Tally::new();

    for (format, decks) in tranches() {
        println!("\n{}", format.display_name());
        let mut format_tally = Tally::new();

        for (deck_name, deck) in decks {
            let mut deck_tally = Tally::new();
            for seed in 0..GAMES_PER_SEAT {
                for handcrafted_seat in [PlayerId::One, PlayerId::Two] {
                    let mut game = Game::new_with_format(
                        format,
                        catalog.clone(),
                        [deck.clone(), deck.clone()],
                        seed,
                    )?;
                    let mut handcrafted = HandcraftedPolicy::new(catalog.clone());
                    let mut random = RandomPolicy::new(seed ^ 0xa11c_e5ed);
                    let result = match handcrafted_seat {
                        PlayerId::One => {
                            play_game(&mut game, &mut handcrafted, &mut random, ACTION_LIMIT)?
                        }
                        PlayerId::Two => {
                            play_game(&mut game, &mut random, &mut handcrafted, ACTION_LIMIT)?
                        }
                    };
                    match result {
                        GameResult::Winner { winner, .. } if winner == handcrafted_seat => {
                            deck_tally.wins += 1;
                        }
                        GameResult::Winner { .. } => deck_tally.losses += 1,
                        GameResult::Draw => deck_tally.draws += 1,
                    }
                }
            }
            report(deck_name, &deck_tally);
            format_tally.add(&deck_tally);
        }

        report("Format total", &format_tally);
        overall.add(&format_tally);
    }

    println!();
    report("Overall", &overall);
    Ok(())
}
