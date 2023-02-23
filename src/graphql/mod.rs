mod context;
mod routes;

use juniper::RootNode;
use juniper::EmptySubscription;

pub use context::Context;
pub use routes::*;

pub struct Query;

pub struct Mutation;

pub type Schema =
    RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

