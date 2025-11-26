use loco_rs::prelude::*;
use oauth2::TokenResponse;
use sea_orm::DatabaseConnection;

/// Trait for OAuth2 sessions.
/// # Generic
/// * `T` - Should implement `OAuth2UserTrait` to be able to upsert a session with OAuth2.
#[async_trait]
pub trait OAuth2SessionsTrait<T>: Clone {
    /// Check if a session is expired from the database
    ///
    /// # Arguments
    /// db: &`DatabaseConnection` - Database connection
    /// session_id: &str - Session id
    /// # Returns
    /// A boolean
    /// # Errors
    /// Returns a `ModelError` if the session is not found
    async fn is_expired(db: &DatabaseConnection, cookie: &str) -> ModelResult<bool>;
    /// Upsert a session with OAuth
    ///
    /// # Arguments
    /// db: &`DatabaseConnection` - Database connection
    /// token: &impl `TokenResponse` - OAuth token (any type implementing TokenResponse)
    /// user: &`users::Model` - User
    /// # Returns
    /// A session
    /// # Errors
    /// Returns a `ModelError` if the session cannot be upserted
    async fn upsert_with_oauth2<R: TokenResponse + Sync>(
        db: &DatabaseConnection,
        token: &R,
        user: &T,
    ) -> ModelResult<Self>;
}
