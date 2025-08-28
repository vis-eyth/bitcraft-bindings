use crate::{global, region, sdk::{DbContext, Result}};

async fn run_until(
    ctx: &impl DbContext,
    exec: impl Future<Output=Result<()>>,
    signal: impl Future
) -> Result<()> {
    tokio::pin!(exec);
    tokio::select! {
        biased;
        _ = signal => { let _ = ctx.disconnect(); exec.await }
        v = &mut exec => v
    }
}

pub trait RunUntil {
    fn run_until(self, signal: impl Future) -> impl Future<Output = Result<()>>;
}

impl RunUntil for global::DbConnection {
    async fn run_until(self, signal: impl Future) -> Result<()> {
        run_until(&self, self.run_async(), signal).await
    }
}

impl RunUntil for region::DbConnection {
    async fn run_until(self, signal: impl Future) -> Result<()> {
        run_until(&self, self.run_async(), signal).await
    }
}
