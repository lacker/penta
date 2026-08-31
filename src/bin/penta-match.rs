//! Runs a head-to-head series between built-in policies.
//!
//! ```text
//! penta-match --p1 random --p2 handcrafted --deck1 Sligh --deck2 "The Deck" \
//!     --games 100 --seed 1
//! ```
//!
//! Seats swap every game so neither policy always plays first. A deck of
//! `Random` rotates deterministically through the built-in list, so a series
//! samples the whole pool.

use std::process::ExitCode;

use penta::protocol::{deck_by_name, deck_names};
use penta::{
    Deck, Game, GameResult, HandcraftedPolicy, PlayerId, Policy, RandomPolicy, WinReason, poc,
};

const ACTION_LIMIT: usize = 100_000;

#[derive(Clone, Copy, PartialEq)]
enum PolicyKind {
    Random,
    Handcrafted,
}

impl PolicyKind {
    fn parse(name: &str) -> Option<Self> {
        match name.to_ascii_lowercase().as_str() {
            "random" => Some(Self::Random),
            "handcrafted" => Some(Self::Handcrafted),
            _ => None,
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Random => "random",
            Self::Handcrafted => "handcrafted",
        }
    }

    fn build(self, seed: u64) -> Box<dyn Policy> {
        match self {
            Self::Random => Box::new(RandomPolicy::new(seed)),
            Self::Handcrafted => Box::new(HandcraftedPolicy::new(
                poc::catalog().expect("catalog builds"),
            )),
        }
    }
}

struct Config {
    p1: PolicyKind,
    p2: PolicyKind,
    deck1: String,
    deck2: String,
    games: u64,
    seed: u64,
    prepared_engine: bool,
}

fn parse_args() -> Result<Config, String> {
    let mut config = Config {
        p1: PolicyKind::Random,
        p2: PolicyKind::Handcrafted,
        deck1: "Random".to_string(),
        deck2: "Random".to_string(),
        games: 100,
        seed: 1,
        prepared_engine: true,
    };
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        let mut value = |flag: &str| args.next().ok_or_else(|| format!("{flag} needs a value"));
        match flag.as_str() {
            "--p1" => {
                let name = value("--p1")?;
                config.p1 =
                    PolicyKind::parse(&name).ok_or_else(|| format!("unknown policy: {name}"))?;
            }
            "--p2" => {
                let name = value("--p2")?;
                config.p2 =
                    PolicyKind::parse(&name).ok_or_else(|| format!("unknown policy: {name}"))?;
            }
            "--deck1" => config.deck1 = value("--deck1")?,
            "--deck2" => config.deck2 = value("--deck2")?,
            "--games" => {
                let count = value("--games")?;
                config.games = count
                    .parse()
                    .map_err(|_| format!("--games must be a number, got {count}"))?;
            }
            "--seed" => {
                let count = value("--seed")?;
                config.seed = count
                    .parse()
                    .map_err(|_| format!("--seed must be a number, got {count}"))?;
            }
            // Diagnostic escape hatch intentionally omitted from --help: the
            // prepared engine is the normal runtime, while this preserves a
            // same-binary reference path for differential tests and timing.
            "--reference-engine" => config.prepared_engine = false,
            "--help" | "-h" => {
                return Err(
                    "usage: penta-match [--p1 random|handcrafted] [--p2 random|handcrafted] \
                     [--deck1 NAME|Random] [--deck2 NAME|Random] [--games N] [--seed N]"
                        .to_string(),
                );
            }
            other => return Err(format!("unknown flag: {other}")),
        }
    }
    Ok(config)
}

/// Resolves a deck request for one game, rotating `Random` through the pool.
fn pick_deck(request: &str, rotation: u64) -> Result<Deck, String> {
    if request.eq_ignore_ascii_case("random") {
        let names = deck_names();
        let index = usize::try_from(rotation).unwrap_or(0) % names.len();
        return Ok(deck_by_name(names[index]).expect("built-in names resolve"));
    }
    deck_by_name(request).ok_or_else(|| format!("unknown deck: {request}"))
}

fn main() -> ExitCode {
    let config = match parse_args() {
        Ok(config) => config,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::FAILURE;
        }
    };

    let catalog = match poc::catalog() {
        Ok(catalog) => catalog,
        Err(error) => {
            eprintln!("catalog failed to build: {error}");
            return ExitCode::FAILURE;
        }
    };

    let mut wins = [0_u64, 0_u64];
    let mut draws = 0_u64;
    let mut failures = 0_u64;
    let mut by_reason: Vec<(WinReason, u64)> = Vec::new();

    for game_index in 0..config.games {
        let seed = config.seed.wrapping_add(game_index);
        // Swap seats every game so neither contestant always plays first.
        // Contestant 1 sits in seat One on even games.
        let swapped = game_index % 2 == 1;
        let decks = match (
            pick_deck(&config.deck1, game_index),
            pick_deck(&config.deck2, game_index.wrapping_mul(7).wrapping_add(3)),
        ) {
            (Ok(deck1), Ok(deck2)) => {
                if swapped {
                    [deck2, deck1]
                } else {
                    [deck1, deck2]
                }
            }
            (Err(message), _) | (_, Err(message)) => {
                eprintln!("{message}");
                return ExitCode::FAILURE;
            }
        };
        let mut game = match Game::new(catalog.clone(), decks, seed) {
            Ok(game) => game,
            Err(error) => {
                eprintln!("game {game_index} failed to start: {error}");
                return ExitCode::FAILURE;
            }
        };
        game.set_prepared_engine_enabled(config.prepared_engine);
        let (first, second) = if swapped {
            (config.p2, config.p1)
        } else {
            (config.p1, config.p2)
        };
        let mut seat_one = first.build(seed ^ 0x517c_c1b7_2722_0a95);
        let mut seat_two = second.build(seed ^ 0x00b0_7b07);
        match penta::play_game(
            &mut game,
            seat_one.as_mut(),
            seat_two.as_mut(),
            ACTION_LIMIT,
        ) {
            Ok(GameResult::Draw) => draws += 1,
            Ok(GameResult::Winner { winner, reason }) => {
                let contestant = match (winner, swapped) {
                    (PlayerId::One, false) | (PlayerId::Two, true) => 0,
                    (PlayerId::One, true) | (PlayerId::Two, false) => 1,
                };
                wins[contestant] += 1;
                match by_reason.iter_mut().find(|(seen, _)| *seen == reason) {
                    Some((_, count)) => *count += 1,
                    None => by_reason.push((reason, 1)),
                }
            }
            Err(error) => {
                eprintln!("game {game_index} (seed {seed}) did not finish: {error}");
                failures += 1;
            }
        }
    }

    println!(
        "{} games: {} = {} wins, {} = {} wins, {} draws{}",
        config.games,
        config.p1.label(),
        wins[0],
        config.p2.label(),
        wins[1],
        draws,
        if failures > 0 {
            format!(", {failures} DID NOT FINISH")
        } else {
            String::new()
        },
    );
    for (reason, count) in by_reason {
        println!("  {reason:?}: {count}");
    }
    if failures > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
