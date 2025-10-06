use client::Client;

#[cfg(feature = "writable")]
use backlog_api_core::Result;

/// API client for star-related operations.
#[derive(Debug)]
pub struct StarApi(#[allow(dead_code)] pub(crate) Client);

impl StarApi {
    /// Creates a new instance of the StarApi.
    pub fn new(client: impl Into<Client>) -> Self {
        Self(client.into())
    }

    /// Adds a star to a resource.
    /// Corresponds to `POST /api/v2/stars`.
    ///
    /// # Arguments
    /// * `params` - Parameters for adding a star
    ///
    /// # Returns
    /// Returns `Ok(())` on success (204 No Content), or an error if the operation fails.
    ///
    /// # Example
    /// ```no_run
    /// # use backlog_star::{StarApi, api::AddStarParams};
    /// # async fn example(api: StarApi) -> Result<(), Box<dyn std::error::Error>> {
    /// // Add star to an issue
    /// api.add_star(AddStarParams::issue(123u32)).await?;
    ///
    /// // Add star to a wiki page
    /// api.add_star(AddStarParams::wiki(456u32)).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "writable")]
    pub async fn add_star(&self, params: super::AddStarParams) -> Result<()> {
        self.0.execute_no_content(params).await
    }
}
