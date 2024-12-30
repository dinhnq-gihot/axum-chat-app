#[allow(unused_imports)]
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
    axum_chat_app::only_role,
    diesel::{
        insert_into,
        prelude::*,
    },
    diesel_async::RunQueryDsl,
    std::sync::Arc,
    tracing::debug,
    uuid::Uuid,
};

#[utoipa::path(
    post,
    context_path = "/api",
    path = "/groups",
    request_body = CreateGroup,
    responses(
        (status = 201, description = "Group created successfully", body = GenericResponse<GroupResponse>),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Groups"
)]
#[only_role("admin", "user")]
pub async fn create_group(
    Extension(db): Extension<Arc<Database>>,
    Extension(sender): Extension<User>,
    Json(payload): Json<CreateGroup>,
) -> Result<impl IntoResponse> {
    debug!("create_group: sender {sender:?}, payload {payload:?}");

    let CreateGroup {
        group_name,
        user_emails,
        user_names,
    } = payload;

    let mut conn = db.get_connection().await;

    // get user ids from emails or usernames
    let mut user_ids: Vec<Uuid> = if let Some(user_emails) = user_emails {
        if user_emails.contains(&sender.email) {
            return Err(Error::NotSelfAssign);
        }

        users::table
            .filter(users::email.eq_any(user_emails))
            .select(users::id)
            .load(&mut conn)
            .await
            .map_err(|e| Error::QueryFailed(e))?
    } else if let Some(user_names) = user_names {
        if user_names.contains(&sender.name) {
            return Err(Error::NotSelfAssign);
        }

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
            result: DataResponse {
                msg: "Group created successfully".into(),
                data: Some(GroupResponse::from(new_group)),
            },
        }),
    ))
}

#[utoipa::path(
    get,
    context_path = "/api",
    path = "/groups/{id}",
    responses(
        (status = 200, description = "Group found", body = GenericResponse<GroupResponse>),
        (status = 404, description = "Group not found"),
        (status = 500, description = "Internal Server Error"),
    ),
    params(
        ("id" = Uuid, Path, description = "Group ID")
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Groups"
)]
#[only_role("user", "admin")]
pub async fn get_group_by_id(
    Extension(db): Extension<Arc<Database>>,
    Extension(sender): Extension<User>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    debug!("get_group_by_id: sender: {sender:?}, id: {id}");

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
        name: group.name,
        users: Some(users.into_iter().map(|u| u.into()).collect::<Vec<_>>()),
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

#[utoipa::path(
    get,
    context_path = "/api",
    path = "/groups/user/{user_id}",
    responses(
        (status = 200, description = "Groups retrieved successfully", body = GenericResponse<Vec<GroupResponse>>),
        (status = 404, description = "Record not found"),
        (status = 500, description = "Internal Server Error")
    ),
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Groups"
)]
#[only_role("user", "admin")]
pub async fn get_all_groups_of_user(
    Extension(db): Extension<Arc<Database>>,
    Extension(sender): Extension<User>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    debug!("get_all_groups_of_user: sender {sender:?}, user_id {user_id}");

    let mut conn = db.get_connection().await;

    let user = users::table
        .find(user_id)
        .select(User::as_select())
        .first::<User>(&mut conn)
        .await
        .map_err(|_| Error::RecordNotFound)?;

    let groups = UserGroup::belonging_to(&user)
        .inner_join(groups::table)
        .select(Group::as_select())
        .load(&mut conn)
        .await
        .map_err(|e| Error::QueryFailed(e))?
        .into_iter()
        .map(|g| GroupResponse::from(g))
        .collect::<Vec<_>>();

    Ok((
        StatusCode::OK,
        Json(GenericResponse {
            status: StatusCode::OK.to_string(),
            result: DataResponse {
                msg: "success".into(),
                data: Some(groups),
            },
        }),
    ))
}
