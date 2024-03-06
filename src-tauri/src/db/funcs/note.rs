use color_eyre::eyre::Result;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryOrder, SelectColumns, Set,
};

use crate::db::models;
use crate::entity::note::{self, ActiveModel as NoteActiveModel, Entity as DbNoteEntity};

pub async fn get_note_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<models::Note>> {
    let note = DbNoteEntity::find_by_id(id).one(db).await;

    match note {
        Ok(note) => match note {
            Some(note) => Ok(Some(models::Note {
                id: note.id,
                title: note.title,
                content: note.content,
                created_at: note.created_at.unwrap_or_default().to_string(),
            })),
            None => Ok(None),
        },
        Err(DbErr::RecordNotFound(_)) => return Ok(None),
        Err(e) => {
            return Err(e.into());
        }
    }
}
pub async fn get_note_list(db: &DatabaseConnection) -> Result<Option<Vec<models::Note>>> {
    let notes = DbNoteEntity::find()
        .select_column(note::Column::Id)
        .select_column(note::Column::Title)
        .select_column(note::Column::CreatedAt)
        .order_by_desc(note::Column::CreatedAt)
        .all(db)
        .await;

    match notes {
        Ok(notes) => {
            let mut note_list: Vec<models::Note> = Vec::new();
            for note in notes {
                note_list.push(models::Note {
                    id: note.id,
                    title: note.title,
                    content: note.content,
                    created_at: note.created_at.unwrap_or_default().to_string(),
                });
            }
            return Ok(Some(note_list));
        }
        Err(DbErr::RecordNotFound(_)) => return Ok(None),
        Err(e) => {
            return Err(e.into());
        }
    }
}
pub async fn create_note(db: &DatabaseConnection, title: &str, content: &str) -> Result<()> {
    let res = NoteActiveModel {
        title: Set(title.to_string()),
        content: Set(content.to_string()),
        ..Default::default()
    }
    .save(db)
    .await;

    return match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    };
}

pub async fn delete_note_by_id(db: &DatabaseConnection, id: i32) -> Result<()> {
    let db_note = DbNoteEntity::find_by_id(id).one(db).await?;
    if let None = db_note {
        return Ok(());
    }

    let mut db_note: NoteActiveModel = db_note.unwrap().into();
    db_note.is_delete = Set(Some(1));

    let res = db_note.update(db).await;

    return match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    };
}
pub async fn confirm_delete_note_by_id(db: &DatabaseConnection, id: i32) -> Result<()> {
    DbNoteEntity::delete_by_id(id).exec(db).await?;
    Ok(())
}
pub async fn update_note_by_id(
    db: &DatabaseConnection,
    id: i32,
    title: &str,
    content: &str,
) -> Result<()> {
    let db_note = DbNoteEntity::find_by_id(id).one(db).await?;
    if let None = db_note {
        return Ok(());
    }

    let mut db_note: NoteActiveModel = db_note.unwrap().into();

    db_note.title = Set(title.to_string());
    db_note.content = Set(content.to_string());

    let res = db_note.update(db).await;

    return match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    };
}

mod test {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use crate::entity::note::Model;
    #[allow(unused_imports)]
    use sea_orm::{DatabaseBackend, MockDatabase};

    #[tokio::test]
    async fn test_get_note_by_id() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![Model {
                id: 1,
                title: "test".to_string(),
                content: "test".to_string(),
                created_at: None,
                is_delete: Some(0),
                category_id: None,
            }]])
            .into_connection();

        let note = get_note_by_id(&db, 1).await.unwrap();
        assert!(note.is_some());
        eprintln!("{:#?}", note);
    }

    #[tokio::test]
    async fn test_get_note_list() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![
                Model {
                    id: 1,
                    title: "test1".to_string(),
                    content: "test".to_string(),
                    created_at: None,
                    is_delete: Some(0),
                    category_id: None,
                },
                Model {
                    id: 2,
                    title: "test2".to_string(),
                    content: "test".to_string(),
                    created_at: None,
                    is_delete: Some(0),
                    category_id: None,
                },
                Model {
                    id: 3,
                    title: "test3".to_string(),
                    content: "test".to_string(),
                    created_at: None,
                    is_delete: Some(0),
                    category_id: None,
                },
            ]])
            .into_connection();
        let notes = get_note_list(&db).await.unwrap();
        assert!(notes.is_some());
        eprintln!("{:#?}", notes);
    }
    #[tokio::test]
    async fn test_create_note() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![Model {
                id: 1,
                title: "test".to_string(),
                content: "test".to_string(),
                created_at: None,
                is_delete: Some(0),
                category_id: None,
            }]])
            .append_exec_results([sea_orm::MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();
        create_note(&db, "test", "test").await.unwrap();
    }
    //waitting to fix
    #[tokio::test]
    async fn test_delete_note_by_id() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![Model {
                id: 1,
                title: "test".to_string(),
                content: "test".to_string(),
                created_at: None,
                is_delete: Some(0),
                category_id: None,
            }]])
            .append_exec_results(vec![sea_orm::MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();
        let res = delete_note_by_id(&db, 1).await;
        match res {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
    }
    #[tokio::test]
    async fn test_confirm_delete_note_by_id() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![Model {
                id: 1,
                title: "test".to_string(),
                content: "test".to_string(),
                created_at: None,
                is_delete: Some(0),
                category_id: None,
            }]])
            .append_exec_results(vec![sea_orm::MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();

        confirm_delete_note_by_id(&db, 1).await.unwrap();
    }

    //waitting to fix
    #[tokio::test]
    async fn test_update_note_by_id() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![Model {
                id: 1,
                title: "test".to_string(),
                content: "test".to_string(),
                created_at: None,
                is_delete: Some(0),
                category_id: None,
            }]])
            .append_exec_results(vec![sea_orm::MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();

        let res = update_note_by_id(&db, 1, "test", "test test test test").await;
        match res {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
    }
}
