use std::io::Write;
use std::path::Path;

pub(super) fn write(
    config: &super::Config,
    report: &penta::engine_profiling::Report,
    wins: [u64; 2],
    draws: u64,
    failures: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(
        config
            .engine_profile
            .as_ref()
            .expect("capture has an output path"),
    );
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)?;
    }
    let document = serde_json::json!({
        "workload": {
            "binary": "penta-match",
            "format": "old-school-93-94",
            "p1": config.p1.label(), "p2": config.p2.label(),
            "deck1": config.deck1, "deck2": config.deck2,
            "games": config.games, "seed": config.seed,
            "match_mode": config.match_mode.slug(),
            "prepared_engine": config.prepared_engine,
        },
        "outcomes": { "wins": wins, "draws": draws, "unfinished": failures },
        "engine_profile": report,
    });
    let mut output = std::io::BufWriter::new(std::fs::File::create(path)?);
    serde_json::to_writer_pretty(&mut output, &document)?;
    writeln!(output)?;
    output.flush()?;
    eprintln!("Engine coverage written to {}", path.display());
    Ok(())
}
