use super::schema::users;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "users"]
pub struct UserEntity {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub hash: String
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUserEntity {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub hash: String
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserEntityChange {
    pub username: Option<String>,
    pub email: Option<String>,
    pub hash: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}