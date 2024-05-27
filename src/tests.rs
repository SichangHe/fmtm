use insta::assert_snapshot;
use tracing::Level;
use tracing_subscriber::EnvFilter;

use super::*;

mod format;

fn init_tracing() {
    _ = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(Level::INFO.into())
                .from_env_lossy(),
        )
        .try_init();
}
