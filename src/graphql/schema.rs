extern crate serde;
use crate::graphql::resolvers::*;
use crate::graphql::{AppContext, RequestContext};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};

pub type TestSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn test<'a>(&self, ctx: &'a Context<'a>) -> Test<'a> {
        let AppContext {
            logger,
            settings: _,
        } = ctx.data_unchecked::<AppContext>();

        Test {
            logger,
            request_context: ctx.data_unchecked::<RequestContext>(),
        }
    }
}

pub fn build_schema(app_context: AppContext) -> TestSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(app_context)
        .finish()
}
