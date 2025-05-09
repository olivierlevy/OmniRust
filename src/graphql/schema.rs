use async_graphql::{Context, Object, Result, SimpleObject, ID, Schema, Subscription, EmptySubscription, value};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::collections::HashMap; // Using HashMap for easier ID management for now
use tokio::sync::broadcast::{self, Sender, Receiver};
use futures_util::stream::{Stream, StreamExt};


// In-memory store for items
static ITEMS: Lazy<Mutex<HashMap<String, Item>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static NEXT_ID: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(1));

// Channel for broadcasting item events
// The channel will send ItemEvent, which needs to be Clone and Send + Sync.
// For simplicity, let's make ItemEvent an enum that can be cloned.
static ITEM_EVENT_SENDER: Lazy<Sender<ItemEvent>> = Lazy::new(|| {
    let (tx, _rx) = broadcast::channel(16); // Capacity of 16
    tx
});

/// Represents events related to items, used for GraphQL subscriptions.
///
/// This struct is broadcast when items are added, updated, or deleted.
#[derive(Clone, Debug, SimpleObject)]
pub struct ItemEvent {
    /// Describes the type of event that occurred (e.g., "ADDED", "UPDATED", "DELETED").
    pub event_type: String,
    /// The `Item` that is the subject of the event.
    /// For "DELETED" events, this will contain the state of the item just before deletion.
    pub item: Item,
}


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
        
        // Broadcast event
        let event = ItemEvent { event_type: "ADDED".to_string(), item: item.clone() };
        let _ = ITEM_EVENT_SENDER.send(event); // Ignore error if no subscribers

        Ok(item)
    }

    /// Updates an existing item's name.
    ///
    /// Takes the `id` of the item to update and an optional new `name`.
    /// If `name` is provided, the item's name will be updated.
    /// Returns the updated `Item` if found, otherwise `null`.
    async fn update_item(&self, _ctx: &Context<'_>, id: ID, name: Option<String>) -> Result<Option<Item>> {
        let mut items_guard = ITEMS.lock().unwrap();
        if let Some(item_ref) = items_guard.get_mut(id.as_str()) {
            if let Some(n) = name {
                item_ref.name = n;
            }
            let updated_item = item_ref.clone();
            // Broadcast event
            let event = ItemEvent { event_type: "UPDATED".to_string(), item: updated_item.clone() };
            let _ = ITEM_EVENT_SENDER.send(event);

            Ok(Some(updated_item))
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
        if let Some(deleted_item) = items_guard.remove(id.as_str()) {
            // Broadcast event
            let event = ItemEvent { event_type: "DELETED".to_string(), item: deleted_item };
            let _ = ITEM_EVENT_SENDER.send(event);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// The root of all GraphQL subscriptions.
///
/// Provides streams of events that clients can subscribe to.
pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    /// Subscribes to real-time events for items.
    ///
    /// Clients subscribing to this field will receive an `ItemEvent`
    /// whenever an item is added, updated, or deleted.
    async fn item_events(&self) -> impl Stream<Item = ItemEvent> {
        let mut rx = ITEM_EVENT_SENDER.subscribe();
        async_stream::stream! {
            loop {
                match rx.recv().await {
                    Ok(event) => yield event,
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        // Handle lagged receiver if necessary, e.g., log or skip
                        // For this example, we'll just continue
                        eprintln!("GraphQL Subscription: Lagged!"); // TODO: Use proper logger
                        continue;
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        // Sender is dropped, stream ends
                        break;
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    // use async_graphql::{Schema, EmptySubscription, value}; // EmptySubscription will be replaced by SubscriptionRoot

    fn create_schema() -> Schema<QueryRoot, MutationRoot, SubscriptionRoot> { // Updated schema
        Schema::build(QueryRoot {}, MutationRoot {}, SubscriptionRoot {}).finish()
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

    #[tokio::test]
    async fn test_item_events_subscription() {
        ITEMS.lock().unwrap().clear();
        *NEXT_ID.lock().unwrap() = 1;
        let schema = create_schema();

        // Start the subscription
        let sub_query = "subscription { itemEvents { eventType item { id name } } }";
        let mut stream = schema.execute_stream(sub_query).await;

        // Perform addItem mutation
        let add_mutation = r#"mutation { addItem(name: "Sub Item 1") { id name } }"#;
        let add_res = schema.execute(add_mutation).await;
        let add_data = add_res.data.into_json().unwrap();
        let added_item_id = add_data["addItem"]["id"].as_str().unwrap().to_string();
        let added_item_name = add_data["addItem"]["name"].as_str().unwrap().to_string();

        // Check for ADDED event
        let event_res = stream.next().await.unwrap();
        let event_data = event_res.data.into_json().unwrap();
        assert_eq!(event_data["itemEvents"]["eventType"], value!("ADDED"));
        assert_eq!(event_data["itemEvents"]["item"]["id"], value!(added_item_id.clone()));
        assert_eq!(event_data["itemEvents"]["item"]["name"], value!(added_item_name.clone()));

        // Perform updateItem mutation
        let update_mutation = format!(r#"mutation {{ updateItem(id: "{}", name: "Sub Item 1 Updated") {{ id name }} }}"#, added_item_id);
        let _update_res = schema.execute(update_mutation).await;
        
        // Check for UPDATED event
        let event_res_update = stream.next().await.unwrap();
        let event_data_update = event_res_update.data.into_json().unwrap();
        assert_eq!(event_data_update["itemEvents"]["eventType"], value!("UPDATED"));
        assert_eq!(event_data_update["itemEvents"]["item"]["id"], value!(added_item_id.clone()));
        assert_eq!(event_data_update["itemEvents"]["item"]["name"], value!("Sub Item 1 Updated"));

        // Perform deleteItem mutation
        let delete_mutation = format!(r#"mutation {{ deleteItem(id: "{}") }}"#, added_item_id);
        let _delete_res = schema.execute(delete_mutation).await;

        // Check for DELETED event
        let event_res_delete = stream.next().await.unwrap();
        let event_data_delete = event_res_delete.data.into_json().unwrap();
        assert_eq!(event_data_delete["itemEvents"]["eventType"], value!("DELETED"));
        assert_eq!(event_data_delete["itemEvents"]["item"]["id"], value!(added_item_id.clone()));
        // The name of the deleted item might still be the last known name, which is "Sub Item 1 Updated"
        assert_eq!(event_data_delete["itemEvents"]["item"]["name"], value!("Sub Item 1 Updated"));


        ITEMS.lock().unwrap().clear();
        *NEXT_ID.lock().unwrap() = 1;
    }
}
