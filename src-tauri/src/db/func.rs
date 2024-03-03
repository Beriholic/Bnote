use color_eyre::eyre::Result;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, QueryOrder, SelectColumns, Set};

use super::{init::db_connect, models};
use crate::entity::note::{self, ActiveModel, Entity as DbNote};
use crate::entity::total_info::{ActiveModel as TotalActiveModel, Entity as TotalInfo};

pub async fn get_note_by_id(id: i32) -> Result<Option<models::Note>> {
    let db = db_connect().await?;
    let note = DbNote::find_by_id(id).one(&db).await;

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
pub async fn get_note_list() -> Result<Option<Vec<models::Note>>> {
    let db = db_connect().await.unwrap();
    let notes = DbNote::find()
        .select_column(note::Column::Id)
        .select_column(note::Column::Title)
        .select_column(note::Column::CreatedAt)
        .order_by_desc(note::Column::CreatedAt)
        .all(&db)
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
pub async fn create_note(title: &str, content: &str) -> Result<()> {
    let db = db_connect().await?;
    let res = ActiveModel {
        title: Set(title.to_string()),
        content: Set(content.to_string()),
        ..Default::default()
    }
    .save(&db)
    .await;

    return match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    };
}

pub async fn delete_note_by_id(id: i32) -> Result<()> {
    let db = db_connect().await?;

    let db_note = DbNote::find_by_id(id).one(&db).await?;

    let mut db_note: ActiveModel = db_note.unwrap().into();
    db_note.is_delete = Set(1);

    let res = db_note.update(&db).await;

    return match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    };
}
pub async fn confirm_delete_note_by_id(id: i32) -> Result<()> {
    let db = db_connect().await?;

    let db_note = DbNote::find_by_id(id).one(&db).await?;
    let db_note: ActiveModel = db_note.unwrap().into();
    let res = db_note.delete(&db).await;

    return match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    };
}
pub async fn update_note_by_id(id: i32, title: &str, content: &str) -> Result<()> {
    let db = db_connect().await?;

    let db_note = DbNote::find_by_id(id).one(&db).await?;

    let mut db_note: ActiveModel = db_note.unwrap().into();

    db_note.title = Set(title.to_string());
    db_note.content = Set(content.to_string());

    let res = db_note.update(&db).await;

    return match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    };
}
pub async fn get_total_number() -> Result<i32> {
    let db = db_connect().await?;
    let total_info = TotalInfo::find_by_id(1).one(&db).await?;

    return match total_info {
        None => Ok(0),
        Some(total_info) => Ok(total_info.total),
    };
}

#[tokio::test]
async fn test_get_note_by_id() {
    let note = get_note_by_id(1).await.unwrap();
    eprintln!("{:#?}", note);
}

#[tokio::test]
async fn test_get_note_list() {
    let notes = get_note_list().await.unwrap();
    eprintln!("{:#?}", notes);
}
#[tokio::test]
async fn test_create_note() {
    create_note("test", "test").await.unwrap();
}
#[tokio::test]
async fn test_delete_note_by_id() {
    delete_note_by_id(1).await.unwrap();
}
#[tokio::test]
async fn test_confirm_delete_note_by_id() {
    confirm_delete_note_by_id(1).await.unwrap();
}

#[tokio::test]
async fn test_update_note_by_id() {
    update_note_by_id(1, "test", "test test test test")
        .await
        .unwrap();
}
