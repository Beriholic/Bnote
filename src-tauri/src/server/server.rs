use sea_orm::DatabaseConnection;

use crate::db::init::db_connect;

pub async fn new_connection() -> DatabaseConnection {
    return db_connect()
        .await
        .expect("Failed to connect to the database");
}
