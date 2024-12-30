use {
    super::models::User,
    crate::{
        database::Database,
        enums::errors::*,
        features::groups::models::{
            Group,
            UserGroup,
        },
        schema::{
            groups,
            users,
        },
    },
    diesel::{
        prelude::*,
        QueryDsl,
        SelectableHelper,
    },
    diesel_async::RunQueryDsl,
    std::sync::Arc,
    uuid::Uuid,
};

pub async fn get_all_user_in_group(db: Arc<Database>, group_id: Uuid) -> Result<Vec<User>> {
    let mut conn = db.get_connection().await;

    let group = groups::table
        .find(group_id)
        .select(Group::as_select())
        .get_result::<Group>(&mut conn)
        .await
        .map_err(|_| Error::RecordNotFound)?;

    let users: Vec<User> = UserGroup::belonging_to(&group)
        .inner_join(users::table)
        .select(User::as_select())
        .load(&mut conn)
        .await
        .map_err(Error::QueryFailed)?;

    Ok(users)
}
