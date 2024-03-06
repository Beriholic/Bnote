use color_eyre::eyre::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::{
    db::init::db_connect,
    entity::total_info::{ActiveModel as TotalActiveModel, Entity as TotalInfo},
};

pub async fn get_total_number(db: &DatabaseConnection) -> Result<i32> {
    let total_info = TotalInfo::find_by_id(1).one(db).await?;

    return match total_info {
        None => Ok(0),
        Some(total_info) => Ok(total_info.total.unwrap()),
    };
}
pub async fn add_total_number(db: &DatabaseConnection, add_number: i32) -> Result<Option<i32>> {
    let db_total_number = get_total_number(db).await?;

    let db = db_connect().await?;
    let total_info = TotalInfo::find_by_id(1).one(&db).await?;
    let mut total_info: TotalActiveModel = total_info.unwrap().into();

    total_info.total = Set(Some(db_total_number + add_number));

    let total_info = total_info.update(&db).await?;
    Ok(total_info.total)
}

#[cfg(test)]
mod test {
    use crate::entity::total_info::{self, Model};

    #[allow(unused_imports)]
    use super::*;
    use sea_orm::{DatabaseBackend, MockDatabase};

    #[tokio::test]
    async fn test_get_total_number() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![Model {
                id: 1,
                total: Some(10),
            }]])
            .into_connection();

        let total = get_total_number(&db).await.unwrap();

        assert_eq!(total, 10);
        eprintln!("total: {}", total);
    }

    #[tokio::test]
    async fn test_add_total_number() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![total_info::Model {
                id: 1,
                total: Some(0),
            }]])
            .into_connection();

        let res = add_total_number(&db, 10).await.unwrap().unwrap();

        assert_eq!(res, 10);
    }
}
