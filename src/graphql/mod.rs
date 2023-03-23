mod mutation;
mod resolver;
mod routes;
mod guards;

pub use async_graphql::EmptySubscription as SubscriptionRoot;
pub use mutation::MutationRoot;
pub use resolver::ResolverRoot;

pub use routes::*;

pub type Schema = async_graphql::Schema<ResolverRoot, MutationRoot, SubscriptionRoot>;
