use orion::pwhash;
use actix::{Handler, Message};
use diesel::{EqAll, QueryDsl, RunQueryDsl};

use super::DbExecutor;
use super::models::{NewUserEntity, UserEntity};
use super::actions::{GetUser, RegisterUser, UserResponse};

impl Message for GetUser {
    type Result = Result<UserResponse, String>;
}

// Handler to Get a user on database
impl Handler<GetUser> for DbExecutor {
    type Result = Result<UserResponse, String>;

    fn handle(&mut self, msg: GetUser, _ctx: &mut Self::Context) -> Self::Result {
        use crate::database::schema::users;

        // Use this to get connection from Self DbExecutor
        let conn = &self.0.get().expect("Cant get connection");

        // Use this to build a query
        let mut query = users::table.into_boxed();

        // Add username to query filter if provided a username on msg
        if let Some(provided_username) = msg.username {
            query = query.filter(users::username.eq_all(provided_username));
        };

        // Add email to query filter if provided a email on msg
        if let Some(provided_email) = msg.email {
            query = query.filter(users::email.eq_all(provided_email));
        };

        // Results is response of query execution
        let results = query.load::<UserEntity>(conn).expect("Error on load user");

        if results.len() == 0 {
            return Err(String::from("User not found"));
        } else if results.len() == 1 {
            let entity = results.get(0).unwrap();
            return Ok(UserResponse {
                token: None,
                username: entity.username.clone(),
                first_name: entity.first_name.clone(),
                last_name: entity.last_name.clone(),
                id: entity.id.clone(),
                email: entity.email.clone(),
            });
        } else {
            return Err(
                String::from(
                    "This is a bug! When looking for a user in the database, it returned more than one value."
                ));
        }
    }
}

impl Message for RegisterUser {
    type Result = Result<UserResponse, String>;
}

impl Handler<RegisterUser> for DbExecutor {
    type Result = Result<UserResponse, String>;

    fn handle(&mut self, msg: RegisterUser, _ctx: &mut Self::Context) -> Self::Result {
        use crate::database::schema::users::dsl::*;

        let password = pwhash::Password::from_slice(msg.password.as_bytes()).unwrap();
        let temp_hash = pwhash::hash_password(&password, 3, 1 << 16).unwrap();

        let new_user = NewUserEntity {
            first_name: msg.first_name,
            last_name: msg.last_name,
            username: msg.username,
            email: msg.email,
            hash: temp_hash.unprotected_as_encoded().to_string(),
        };

        let conn = &self.0.get().unwrap();

        match diesel::insert_into(users)
            .values(new_user)
            .get_result::<UserEntity>(conn)
        {
            Ok(entity) => Ok(UserResponse::from(entity)),
            Err(e) => Err(e.to_string()),
        }
    }
}

// impl Message for LoginUser {
//     type Result = Result<UserResponse, ()>;
// }

// impl Handler<LoginUser> for DbExecutor {
//     type Result = Result<UserResponse, String>;

//     fn handle(&mut self, msg: LoginUser, ctx: &mut Self::Context) -> Self::Result {
//         use crate::database::schema::users::dsl::*;

//         let provided_password = pwhash::Password::from_slice(msg.password.as_bytes()).unwrap();
//     }
// }
