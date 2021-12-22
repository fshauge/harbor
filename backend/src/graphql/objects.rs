use std::sync::Arc;

use async_graphql::{Object, Result};
use bollard::models;

pub struct Docker(pub Arc<bollard::Docker>);

#[Object]
impl Docker {
    async fn info(&self) -> Result<SystemInfo> {
        Ok(SystemInfo(self.0.info().await.unwrap()))
    }
}

pub struct SystemInfo(pub models::SystemInfo);

#[Object]
impl SystemInfo {
    async fn id(&self) -> Option<&String> {
        self.0.id.as_ref()
    }
}
