use crate::{global, region, sdk::{DbContext, IntoQueries, Result}};
use crate::sdk::__codegen::{SpacetimeModule, SubscriptionBuilder};
use tokio::sync::mpsc::unbounded_channel as channel;

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


pub trait SubscribeAsync<M: SpacetimeModule> {
    fn subscribe_async<Queries: IntoQueries>(self, queries: Queries) -> impl Future<Output = Result<M::SubscriptionHandle>>;
}

impl<M: SpacetimeModule> SubscribeAsync<M> for SubscriptionBuilder<M> {
    async fn subscribe_async<Queries: IntoQueries>(mut self, queries: Queries) -> Result<M::SubscriptionHandle> {
        let (tx, mut rx) = channel();
        {
            let tx = tx.clone();
            self = self.on_error(move |_, e| { let _ = tx.send(Err(e)); });
        }
        {
            let tx = tx.clone();
            self = self.on_applied(move |_| { let _ = tx.send(Ok(())); });
        }
        let handle = self.subscribe(queries);
        rx.recv().await.unwrap()?;
        Ok(handle)
    }
}