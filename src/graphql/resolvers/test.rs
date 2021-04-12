extern crate serde;
use crate::graphql::RequestContext;
use async_graphql::{Object, SimpleObject};
use slog::Logger;

#[derive(SimpleObject)]
pub struct NameValue {
    pub name: String,
    pub value: String,
}

pub struct Test<'a> {
    pub logger: &'a Logger,
    pub request_context: &'a RequestContext,
}

#[Object]
impl<'a> Test<'a> {
    async fn message(&self) -> String {
        "hi!".into()
    }

    async fn headers(&self) -> Vec<NameValue> {
        self.request_context
            .headers
            .iter()
            .map(|(header_name, header_value)| NameValue {
                name: header_name.to_string(),
                value: header_value.to_str().unwrap_or("").into(),
            })
            .collect()
    }
}
