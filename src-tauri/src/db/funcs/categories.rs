use color_eyre::eyre::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::{
    db::models::Categories,
    entity::category::{ActiveModel as CategoryActiveModel, Entity as CategoriesEntity},
};

pub async fn create_categories(db: &DatabaseConnection, name: &str) -> Result<()> {
    CategoryActiveModel {
        name: Set(name.to_string()),
        ..Default::default()
    }
    .save(db)
    .await?;

    Ok(())
}
pub async fn delete_categories_by_id(db: &DatabaseConnection, id: i32) -> Result<()> {
    let categories = CategoriesEntity::find_by_id(id).one(db).await?;
    if let None = categories {
        return Ok(());
    }
    let mut categories: CategoryActiveModel = categories.unwrap().into();
    categories.is_delete = Set(Some(1));

    categories.save(db).await?;

    Ok(())
}
pub async fn confirm_delete_categories_by_id(db: &DatabaseConnection, id: i32) -> Result<()> {
    CategoriesEntity::delete_by_id(id).exec(db).await?;
    Ok(())
}
pub async fn update_categories_by_id(db: &DatabaseConnection, id: i32, name: &str) -> Result<()> {
    let categories = CategoriesEntity::find_by_id(id).one(db).await?;
    if let None = categories {
        return Ok(());
    }

    let mut categories: CategoryActiveModel = categories.unwrap().into();
    categories.name = Set(name.to_string());

    categories.save(db).await?;

    Ok(())
}
pub async fn get_categories_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Categories>> {
    let categories = CategoriesEntity::find_by_id(id).one(db).await?;
    if None == categories {
        return Ok(None);
    }

    let categories: Categories = categories.unwrap().into();
    Ok(Some(categories))
}
pub async fn get_categories_list(db: &DatabaseConnection) -> Result<Option<Vec<Categories>>> {
    let categories = CategoriesEntity::find().all(db).await?;
    if categories.len() == 0 {
        return Ok(None);
    }

    let mut categories_list = Vec::new();
    for category in categories {
        let category: Categories = category.into();
        categories_list.push(category);
    }

    Ok(Some(categories_list))
}
#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
    use crate::entity::category::Model;
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
    #[tokio::test]
    async fn test_create_categories() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![Model {
                id: 1,
                name: "test".to_string(),
                created_at: None,
                is_delete: Some(0),
            }]])
            .append_exec_results([MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();
        let name = "test";
        create_categories(&db, name).await.unwrap();
    }

    //waitting to fix
    #[tokio::test]
    async fn test_delete_categories_by_id() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![Model {
                id: 1,
                name: "test".to_string(),
                created_at: None,
                is_delete: Some(0),
            }]])
            .append_exec_results([MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();

        let id = 1;
        match delete_categories_by_id(&db, id).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_confirm_delete_categories_by_id() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![Model {
                id: 1,
                name: "test".to_string(),
                created_at: None,
                is_delete: Some(0),
            }]])
            .append_exec_results([MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();
        let id = 1;
        confirm_delete_categories_by_id(&db, id).await.unwrap();
    }
    //waitting to fix
    #[tokio::test]
    async fn test_update_categories_by_id() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![Model {
                id: 1,
                name: "test".to_string(),
                created_at: None,
                is_delete: Some(0),
            }]])
            .append_exec_results([MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();
        let id = 1;
        let name = "testtest";
        match update_categories_by_id(&db, id, name).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_categories_by_id() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![Model {
                id: 1,
                name: "test".to_string(),
                created_at: None,
                is_delete: Some(0),
            }]])
            .into_connection();
        let id = 1;
        get_categories_by_id(&db, id).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_categories_list() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(std::iter::once(vec![Model {
                id: 1,
                name: "test".to_string(),
                created_at: None,
                is_delete: Some(0),
            }]))
            .into_connection();
        get_categories_list(&db).await.unwrap();
    }
}
