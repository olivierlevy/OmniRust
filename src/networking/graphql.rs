use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};

#[derive(SimpleObject)]
struct Book {
    title: String,
    author: String,
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn books(&self) -> Vec<Book> {
        vec![
            Book {
                title: "OmniRust Guide".to_string(),
                author: "The OmniRust Team".to_string(),
            },
            Book {
                title: "Rust for Beginners".to_string(),
                author: "Rustacean".to_string(),
            },
        ]
    }
}

pub fn build_schema() -> Schema<QueryRoot, EmptyMutation, EmptySubscription> {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}
