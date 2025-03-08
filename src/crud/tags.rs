use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use crate::models::{NewTag, Tag, NewTaskTag};
use crate::schema::tags::dsl::*;
use crate::schema::task_tags::dsl::{task_tags, tag_id as task_tag_id, task_id as task_tag_task_id};
use log::info;

pub fn create_tag(conn: &mut SqliteConnection, tag_name: &str, new_user_id: i32) -> QueryResult<Tag> {
    info!("Creating Tag ...");

    let new_tag = NewTag {
        name: tag_name.to_string(),
        user_id: new_user_id,
        icon_url: None 
    };

    diesel::insert_into(tags)
        .values(&new_tag)
        .execute(conn)
        .map_err(|err| {
            log::error!("Error inserting new tag: {}", err);
            err
        })?;

    tags.order(id.desc()).select(Tag::as_select()).first(conn).map_err(|err| {
        log::error!("Error retrieving the newly created tag: {}", err);
        err
    })
}

pub fn add_tags_by_task(conn: &mut SqliteConnection, task_id: i32, tag_list: Vec<i32>) -> QueryResult<Vec<i32>> {
    let new_task_tags: Vec<NewTaskTag> = tag_list.into_iter().map(|tag_id| NewTaskTag {
        task_id,
        tag_id,
    }).collect();

    log::debug!("Adding Tags: {:?}", new_task_tags );

    let result = diesel::insert_into(task_tags)
        .values(&new_task_tags)
        .execute(conn);

    match result {
        Ok(_) => Ok(new_task_tags.into_iter().map(|task_tag| task_tag.tag_id).collect()),
        Err(err) => {
            log::error!("Error inserting task tags: {}", err);
            Err(err)
        }
    }
}

pub fn get_tags(conn: &mut SqliteConnection, new_user_id: i32) -> QueryResult<Vec<Tag>> {
    tags.filter(user_id.eq(new_user_id))
        .select(Tag::as_select()).load::<Tag>(conn)
}

pub fn get_tags_by_task(conn: &mut SqliteConnection, new_task_id: i32) -> QueryResult<Vec<Tag>> {
    tags.inner_join(task_tags.on(task_tag_id.eq(id)))
        .filter(task_tag_task_id.eq(new_task_id))
        .select(Tag::as_select())
        .load::<Tag>(conn)
}

pub fn get_tag(conn: &mut SqliteConnection, tag_id: i32) -> QueryResult<Tag> {
    tags.filter(id.eq(tag_id)).select(Tag::as_select()).first(conn)
}

pub fn update_tag(conn: &mut SqliteConnection, tag_id: i32, new_name: &str) -> QueryResult<Tag> {
    diesel::update(tags.find(tag_id))
        .set(name.eq(new_name))
        .execute(conn)?;

    tags.find(tag_id).select(Tag::as_select()).first(conn)
}

pub fn delete_tag(conn: &mut SqliteConnection, tag_id: i32) -> QueryResult<usize> {
    diesel::delete(tags.find(tag_id)).execute(conn)
}