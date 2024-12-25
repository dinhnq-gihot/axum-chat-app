use {
    diesel::prelude::*,
    uuid::Uuid,
};

#[derive(Debug, Identifiable, AsChangeset, Selectable, Queryable, PartialEq, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar: Option<String>,
    pub is_online: Option<bool>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            name: Default::default(),
            email: Default::default(),
            password: Default::default(),
            avatar: Default::default(),
            is_online: Default::default(),
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub avatar: Option<&'a str>,
}
