use std::time::Instant;

use crate::error::AnyResult;

#[cfg(feature = "bin")]
pub fn init(_module: &str, level: i8, filter: Option<String>) -> AnyResult<()> {
    use anyhow::Context;
    use tracing::metadata::LevelFilter;

    let level = match level {
        ..=-3 => LevelFilter::OFF,
        -2 => LevelFilter::ERROR,
        -1 => LevelFilter::WARN,
        0 => LevelFilter::INFO,
        1 => LevelFilter::DEBUG,
        2.. => LevelFilter::TRACE,
    };

    let directive = match filter {
        Some(filter) => filter
            .parse()
            .with_context(|| format!("failed to parse filter: {filter}"))?,
        None => level.to_string().parse().unwrap(),
    };

    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(directive)
        .from_env_lossy();

    let formatter = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_timer(Uptime {
            start: Instant::now(),
        })
        .finish();

    tracing::subscriber::set_global_default(formatter)
        .context("failed to set global tracing subscriber")
}

struct Uptime {
    start: Instant,
}

impl tracing_subscriber::fmt::time::FormatTime for Uptime {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        write!(
            w,
            "{:3}.{:03}",
            self.start.elapsed().as_secs(),
            self.start.elapsed().subsec_millis()
        )
    }
}
