mod mutation;
mod resolver;
mod routes;

use mutation::MutationRoot;
use resolver::ResolverRoot;

use async_graphql::EmptySubscription;

pub use routes::*;

pub type Schema = async_graphql::Schema<ResolverRoot, MutationRoot, EmptySubscription>;
