use std::sync::Arc;

use async_trait::async_trait;
use shaku::Interface;

#[async_trait]
pub trait Cron: Interface {
    async fn run(self: Arc<Self>) -> ();
}
