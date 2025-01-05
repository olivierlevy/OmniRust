use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct QueryRoot {
    pub system_status: String,
}
