use async_graphql::Schema;
use async_graphql::{EmptyMutation, EmptySubscription};

use crate::model::query_root::QueryRoot;

pub(crate) type ServiceSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
