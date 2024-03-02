use color_eyre::eyre::Result;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;

pub async fn db_connect() -> Result<DatabaseConnection, DbErr> {
    let db_url = format!("sqlite:///home/beri/.config/Bnote/bnote.db",);

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    let db = Database::connect(opt).await?;
    Ok(db)
}

#[tokio::test]
async fn test_db_connect() {
    let db = db_connect().await.unwrap();
    assert!(db.ping().await.is_ok());
}
