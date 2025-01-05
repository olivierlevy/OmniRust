use omnirust::networking::graphql::build_schema;

#[tokio::test]
async fn test_graphql_query() {
    let schema = build_schema();

    let query = r#"
    {
        books {
            title
            author
        }
    }
    "#;

    let response = schema.execute(query).await;
    assert!(response.is_ok());
}
