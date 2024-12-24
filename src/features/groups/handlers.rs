use {
    super::{
        dto::{
            CreateGroup,
            GroupResponse,
        },
        models::{
            Group,
            NewGroup,
            NewUserGroup,
            UserGroup,
        },
    }, crate::{
        database::Database,
        enums::{
            errors::{
                Error,
                Result,
            },
            types::{
                DataResponse,
                GenericResponse,
            },
        },
        features::users::model::User,
        schema::{
            groups,
            users,
            users_groups,
        },
    }, axum::{
        extract::Path,
        http::StatusCode,
        response::IntoResponse,
        Extension,
        Json,
    }, diesel::{
        insert_into,
        prelude::*,
    }, diesel_async::RunQueryDsl, std::sync::Arc, uuid::Uuid
};

pub async fn create_group(
    Extension(db): Extension<Arc<Database>>,
    Json(payload): Json<CreateGroup>,
) -> Result<impl IntoResponse> {
    let CreateGroup { name, user_ids } = payload;

    let mut conn = db.get_connection().await;
    let new_group: Group = insert_into(groups::table)
        .values(NewGroup { name: &name })
        .returning(Group::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| Error::InsertFailed(e))?;

    let records = user_ids
        .iter()
        .map(|user_id| {
            NewUserGroup {
                user_id,
                group_id: &new_group.id,
            }
        })
        .collect::<Vec<_>>();

    insert_into(users_groups::table)
        .values(records)
        .execute(&mut conn)
        .await
        .map_err(|e| Error::InsertFailed(e))?;

    Ok((
        StatusCode::CREATED,
        Json(GenericResponse {
            status: StatusCode::CREATED.to_string(),
            result: DataResponse::<String> {
                msg: "success".into(),
                data: None,
            },
        }),
    ))
}

pub async fn get_group_by_id(
    Extension(db): Extension<Arc<Database>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let mut conn = db.get_connection().await;

    let group = groups::table
        .find(id)
        .select(Group::as_select())
        .get_result::<Group>(&mut conn)
        .await
        .map_err(|_| Error::RecordNotFound)?;

    let users: Vec<User> = UserGroup::belonging_to(&group)
        .inner_join(users::table)
        .select(User::as_select())
        .load(&mut conn)
        .await
        .map_err(|e| Error::QueryFailed(e))?;

    let result = GroupResponse {
        id: group.id,
        users: users.into_iter().map(|u| u.into()).collect::<Vec<_>>(),
    };

    Ok((
        StatusCode::OK,
        Json(GenericResponse {
            status: StatusCode::OK.to_string(),
            result: DataResponse {
                msg: "success".into(),
                data: Some(result),
            },
        }),
    ))
}
