use {
    crate::features::users::model::User,
    diesel::prelude::*,
    uuid::Uuid,
};

#[derive(Debug, Identifiable, AsChangeset, Selectable, Queryable, PartialEq, Clone)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Group {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewGroup<'a> {
    pub name: &'a str,
}

#[derive(Debug, Identifiable, Selectable, Associations, Clone)]
#[diesel(table_name = crate::schema::users_groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Group))]
#[diesel(primary_key(user_id, group_id))]
pub struct UserGroup {
    pub user_id: uuid::Uuid,
    pub group_id: uuid::Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::users_groups)]
pub struct NewUserGroup<'a> {
    pub user_id: &'a uuid::Uuid,
    pub group_id: &'a uuid::Uuid,
}
