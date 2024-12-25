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
    },
    crate::{
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
        features::users::models::User,
        schema::{
            groups,
            users,
            users_groups,
        },
    },
    anyhow::anyhow,
    axum::{
        extract::Path,
        http::StatusCode,
        response::IntoResponse,
        Extension,
        Json,
    },
    diesel::{
        insert_into,
        prelude::*,
    },
    diesel_async::RunQueryDsl,
    std::sync::Arc,
    uuid::Uuid,
};

pub async fn create_group(
    Extension(db): Extension<Arc<Database>>,
    Extension(sender): Extension<User>,
    Json(payload): Json<CreateGroup>,
) -> Result<impl IntoResponse> {
    let CreateGroup {
        group_name,
        user_emails,
        user_names,
    } = payload;

    let mut conn = db.get_connection().await;

    let mut user_ids: Vec<Uuid> = if let Some(user_emails) = user_emails {
        users::table
            .filter(users::email.eq_any(user_emails))
            .select(users::id)
            .load(&mut conn)
            .await
            .map_err(|e| Error::QueryFailed(e))?
    } else if let Some(user_names) = user_names {
        users::table
            .filter(users::name.eq_any(user_names))
            .select(users::id)
            .load(&mut conn)
            .await
            .map_err(|e| Error::QueryFailed(e))?
    } else {
        vec![]
    };

    if user_ids.is_empty() {
        return Err(Error::Anyhow(anyhow!("Not found input")));
    }

    let new_group: Group = insert_into(groups::table)
        .values(NewGroup { name: &group_name })
        .returning(Group::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| Error::InsertFailed(e))?;

    user_ids.push(sender.id);
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
