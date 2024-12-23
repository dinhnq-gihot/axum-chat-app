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
        features::users::model::User,
        schema::{
            groups,
            users,
            users_groups,
        },
    },
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
    serde_json::json,
    uuid::Uuid,
};

pub async fn create_group(
    Extension(db): Extension<Database>,
    Json(payload): Json<CreateGroup>,
) -> impl IntoResponse {
    let CreateGroup { name, user_ids } = payload;

    let mut conn = db.get_connection().await;
    let new_group: Group = insert_into(groups::table)
        .values(NewGroup { name: &name })
        .returning(Group::as_returning())
        .get_result(&mut conn)
        .await
        .unwrap();

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
        .unwrap();

    (
        StatusCode::CREATED,
        Json(json!({"result": "created group successfully"})),
    )
}

pub async fn get_group_by_id(
    Extension(db): Extension<Database>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let mut conn = db.get_connection().await;

    let group: Group = groups::table
        .find(id)
        .select(Group::as_select())
        .get_result(&mut conn)
        .await
        .unwrap();

    let users: Vec<User> = UserGroup::belonging_to(&group)
        .inner_join(users::table)
        .select(User::as_select())
        .load(&mut conn)
        .await
        .unwrap();

    let result = GroupResponse {
        id: group.id,
        users: users.into_iter().map(|u| u.into()).collect::<Vec<_>>(),
    };

    (StatusCode::OK, Json(json!({"result": result})))
}
