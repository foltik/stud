use crate::error::{AnyResult, Context};

use futures::{Future, FutureExt};
use tokio::runtime::{Builder, Runtime};

pub fn run(f: impl Future<Output = AnyResult<()>>) -> AnyResult<()> {
    Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("failed to create tokio runtime")?;

    Runtime::new().unwrap().block_on(async move {
        futures::select! {
            result = f.fuse() => result,
            _ = tokio::signal::ctrl_c().fuse() => {
                tracing::debug!("exiting: ctrl-c received");
                Ok(())
            }
        }
    })
}
