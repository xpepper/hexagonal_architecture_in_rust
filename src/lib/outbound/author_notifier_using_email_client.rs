use crate::domain::blog::models::author::Author;
use crate::domain::blog::ports::AuthorNotifier;

/// An unimplemented example of an adapter to [AuthorNotifier].
#[derive(Debug, Clone)]
pub struct AuthorNotifierUsingEmailClient;

impl AuthorNotifierUsingEmailClient {
    pub fn new() -> Self {
        Self
    }
}

impl AuthorNotifier for AuthorNotifierUsingEmailClient {
    async fn author_created(&self, _: &Author) {}
}
