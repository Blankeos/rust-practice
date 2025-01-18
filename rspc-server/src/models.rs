use diesel::prelude::*;

#[derive(Queryable, Identifiable, Selectable, PartialEq)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: String,
    pub username: String,
    pub hashed_password: String,
    pub email: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::user)]
pub struct NewUser {
    pub id: String,
    pub username: String,
    pub hashed_password: String,
    pub email: Option<String>,
}
