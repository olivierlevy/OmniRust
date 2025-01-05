use async_graphql::*;
use std::error::Error;
use crate::graphql::schema::QueryRoot;

pub async fn start_server(_schema: Schema<QueryRoot, EmptyMutation, EmptySubscription>) -> Result<(), Box<dyn Error>> {
    // Implémentez le démarrage du serveur GraphQL ici
    Ok(())
}
