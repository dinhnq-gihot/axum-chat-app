use {
    super::models::Group,
    crate::{
        database::Database,
        enums::errors::*,
        schema::groups,
    },
    diesel::prelude::*,
    diesel_async::RunQueryDsl,
    std::sync::Arc,
    uuid::Uuid,
};

pub async fn get_group_by_id(db: Arc<Database>, group_id: Uuid) -> Result<Group> {
    let mut conn = db.get_connection().await;

    let group = groups::table
        .find(group_id)
        .select(Group::as_select())
        .first(&mut conn)
        .await
        .map_err(|_| Error::RecordNotFound)?;

    Ok(group)
}
