use async_graphql::{Context, Object, Result, SimpleObject, ID};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::collections::HashMap; // Using HashMap for easier ID management for now

// In-memory store for items
static ITEMS: Lazy<Mutex<HashMap<String, Item>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static NEXT_ID: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(1));

/// Represents a generic item in the system.
#[derive(SimpleObject, Clone, Debug)]
pub struct Item {
    /// The unique identifier of the item.
    pub id: ID,
    /// The name of the item.
    pub name: String,
    // Add other fields as needed, e.g., description, price
}

/// The root of all GraphQL queries.
pub struct QueryRoot {
    // This field will be set during QueryRoot instantiation
    // and resolved by a method in the #[Object] impl block.
    // Or, it can be accessed via ctx.data if QueryRoot is passed as data.
    // For simplicity, let's assume it's passed during construction.
    // If QueryRoot is default-constructible, system_status needs a default value or to be fetched.
    // Let's make QueryRoot default-constructible for now and fetch system_status dynamically.
}

#[Object]
impl QueryRoot {
    /// Retrieves the current operational status of the OmniRust system.
    /// This can be used as a health check.
    async fn system_status(&self, _ctx: &Context<'_>) -> Result<String> {
        // In a real app, this might involve checking database connections, service health, etc.
        Ok("OmniRust System Nominal".to_string())
    }

    /// Retrieves a list of all available items.
    async fn items(&self, _ctx: &Context<'_>) -> Result<Vec<Item>> {
        let items_guard = ITEMS.lock().unwrap();
        Ok(items_guard.values().cloned().collect())
    }

    /// Retrieves a specific item by its unique ID.
    /// Returns `null` if no item with the given ID is found.
    async fn item(&self, _ctx: &Context<'_>, id: ID) -> Result<Option<Item>> {
        let items_guard = ITEMS.lock().unwrap();
        Ok(items_guard.get(id.as_str()).cloned())
    }
}

/// The root of all GraphQL mutations.
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Adds a new item to the in-memory store.
    ///
    /// Takes a `name` for the new item and returns the created `Item`
    /// with a server-generated ID.
    async fn add_item(&self, _ctx: &Context<'_>, name: String) -> Result<Item> {
        let mut items_guard = ITEMS.lock().unwrap();
        let mut id_guard = NEXT_ID.lock().unwrap();

        let new_id = *id_guard;
        *id_guard += 1;

        let item = Item {
            id: ID(new_id.to_string()),
            name,
        };
        items_guard.insert(item.id.to_string(), item.clone());
        Ok(item)
    }

    /// Updates an existing item's name.
    ///
    /// Takes the `id` of the item to update and an optional new `name`.
    /// If `name` is provided, the item's name will be updated.
    /// Returns the updated `Item` if found, otherwise `null`.
    async fn update_item(&self, _ctx: &Context<'_>, id: ID, name: Option<String>) -> Result<Option<Item>> {
        let mut items_guard = ITEMS.lock().unwrap();
        if let Some(item) = items_guard.get_mut(id.as_str()) {
            if let Some(n) = name {
                item.name = n;
            }
            Ok(Some(item.clone()))
        } else {
            Ok(None)
        }
    }

    /// Deletes an item from the in-memory store.
    ///
    /// Takes the `id` of the item to delete.
    /// Returns `true` if the item was found and deleted, `false` otherwise.
    async fn delete_item(&self, _ctx: &Context<'_>, id: ID) -> Result<bool> {
        let mut items_guard = ITEMS.lock().unwrap();
        Ok(items_guard.remove(id.as_str()).is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_graphql::{Schema, EmptySubscription, value};

    fn create_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
        Schema::build(QueryRoot {}, MutationRoot {}, EmptySubscription).finish()
    }

    #[tokio::test]
    async fn test_add_item_mutation() {
        let schema = create_schema();
        let query = r#"
            mutation {
                addItem(name: "Test Item 1") {
                    id
                    name
                }
            }
        "#;
        let res = schema.execute(query).await;
        let data = res.data.into_json().unwrap();
        
        assert_eq!(data["addItem"]["name"], value!("Test Item 1"));
        assert!(data["addItem"]["id"].as_str().is_some()); // Check if ID is a non-empty string

        // Optionally, query for the item to ensure it's in the list
        let item_id = data["addItem"]["id"].as_str().unwrap();
        let query_item = format!(r#"
            query {{
                item(id: "{}") {{
                    id
                    name
                }}
            }}
        "#, item_id);
        let res_item = schema.execute(&query_item).await;
        let data_item = res_item.data.into_json().unwrap();

        assert_eq!(data_item["item"]["name"], value!("Test Item 1"));
        assert_eq!(data_item["item"]["id"], value!(item_id));

        // Clean up static storage for other tests if necessary, though for this simple test it's okay.
        // For more complex scenarios, consider dependency injection for the store.
        ITEMS.lock().unwrap().clear();
        *NEXT_ID.lock().unwrap() = 1;
    }

    #[tokio::test]
    async fn test_items_query_empty() {
        ITEMS.lock().unwrap().clear(); // Ensure store is empty
        *NEXT_ID.lock().unwrap() = 1;

        let schema = create_schema();
        let query = r#"
            query {
                items {
                    id
                    name
                }
            }
        "#;
        let res = schema.execute(query).await;
        let data = res.data.into_json().unwrap();
        assert!(data["items"].as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_system_status_query() {
        let schema = create_schema();
        let query = r#"
            query {
                systemStatus
            }
        "#;
        let res = schema.execute(query).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(data["systemStatus"], value!("OmniRust System Nominal"));
    }

    #[tokio::test]
    async fn test_update_item_mutation() {
        ITEMS.lock().unwrap().clear();
        *NEXT_ID.lock().unwrap() = 1;
        let schema = create_schema();

        // Add an item first
        let add_query = r#"mutation { addItem(name: "Original Name") { id name } }"#;
        let add_res = schema.execute(add_query).await;
        let add_data = add_res.data.into_json().unwrap();
        let item_id = add_data["addItem"]["id"].as_str().unwrap().to_string();

        // Update the item
        let update_query = format!(r#"
            mutation {{
                updateItem(id: "{}", name: "Updated Name") {{
                    id
                    name
                }}
            }}
        "#, item_id);
        let update_res = schema.execute(&update_query).await;
        let update_data = update_res.data.into_json().unwrap();

        assert_eq!(update_data["updateItem"]["id"], value!(item_id));
        assert_eq!(update_data["updateItem"]["name"], value!("Updated Name"));

        // Verify with a query
        let query_item = format!(r#"query {{ item(id: "{}") {{ name }} }}"#, item_id);
        let res_item = schema.execute(&query_item).await;
        let data_item = res_item.data.into_json().unwrap();
        assert_eq!(data_item["item"]["name"], value!("Updated Name"));

        // Test updating non-existent item
        let update_non_existent_query = r#"
            mutation {
                updateItem(id: "nonexistent", name: "New Name") {
                    id name
                }
            }
        "#;
        let res_non_existent = schema.execute(update_non_existent_query).await;
        assert!(res_non_existent.data.into_json().unwrap()["updateItem"].is_null());
        
        ITEMS.lock().unwrap().clear();
        *NEXT_ID.lock().unwrap() = 1;
    }

    #[tokio::test]
    async fn test_delete_item_mutation() {
        ITEMS.lock().unwrap().clear();
        *NEXT_ID.lock().unwrap() = 1;
        let schema = create_schema();

        // Add an item first
        let add_query = r#"mutation { addItem(name: "To Be Deleted") { id } }"#;
        let add_res = schema.execute(add_query).await;
        let add_data = add_res.data.into_json().unwrap();
        let item_id = add_data["addItem"]["id"].as_str().unwrap().to_string();

        // Delete the item
        let delete_query = format!(r#"mutation {{ deleteItem(id: "{}") }}"#, item_id);
        let delete_res = schema.execute(&delete_query).await;
        let delete_data = delete_res.data.into_json().unwrap();
        assert_eq!(delete_data["deleteItem"], value!(true));

        // Verify it's gone
        let query_item = format!(r#"query {{ item(id: "{}") {{ id }} }}"#, item_id);
        let res_item = schema.execute(&query_item).await;
        assert!(res_item.data.into_json().unwrap()["item"].is_null());

        // Test deleting non-existent item
        let delete_non_existent_query = r#"mutation { deleteItem(id: "nonexistent") }"#;
        let res_non_existent = schema.execute(delete_non_existent_query).await;
        let data_non_existent = res_non_existent.data.into_json().unwrap();
        assert_eq!(data_non_existent["deleteItem"], value!(false));
        
        ITEMS.lock().unwrap().clear();
        *NEXT_ID.lock().unwrap() = 1;
    }
}
