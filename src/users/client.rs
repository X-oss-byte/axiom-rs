use crate::{error::Result, http, users::model::*};
use maybe_async::maybe_async;
use tracing::instrument;

/// Provides methods to work with Axiom datasets.
#[derive(Debug, Clone)]
pub struct Client {
    http_client: http::Client,
}

impl Client {
    pub(crate) fn new(http_client: http::Client) -> Self {
        Self { http_client }
    }

    /// Retrieve the authenticated user.
    #[maybe_async]
    #[instrument(skip(self))]
    pub async fn current(&self) -> Result<User> {
        self.http_client.get("/v1/user").await?.json().await
    }
}
