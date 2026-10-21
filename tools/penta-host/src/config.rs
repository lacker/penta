use penta::{Format, match_play::MatchMode, session::ExactSession};
use serde::{Deserialize, Serialize};

/// Public match setup. The host generates and retains its private seed.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Config {
    pub format: String,
    pub p1_deck: String,
    pub p2_deck: String,
    #[serde(default = "default_mode")]
    pub match_mode: String,
}

fn default_mode() -> String {
    "first-to-two-wins".into()
}

impl Config {
    pub(crate) fn session(&self, seed: u64) -> Result<ExactSession, String> {
        self.session_version(seed, true)
    }

    pub(crate) fn session_version(
        &self,
        seed: u64,
        proposals: bool,
    ) -> Result<ExactSession, String> {
        let format = Format::ALL
            .iter()
            .find(|format| format.slug() == self.format)
            .copied()
            .ok_or_else(|| format!("unknown format: {}", self.format))?;
        let mut session = ExactSession::new(
            format,
            &self.p1_deck,
            &self.p2_deck,
            seed,
            MatchMode::parse(&self.match_mode)?,
        )?;
        if proposals {
            session = session.with_action_proposals();
        }
        session.advance_forced()?;
        Ok(session)
    }
}
