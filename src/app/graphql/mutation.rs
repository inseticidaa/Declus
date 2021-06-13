use super::get_addr_from_ctx;
use super::User;
use crate::database::actions;
use async_graphql::*;
use validator::Validate;

use super::NewUser;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_user(&self, ctx: &Context<'_>, new_user: NewUser) -> Result<User, String> {

        if let Err(e) = new_user.validate() {
            return Err(e.to_string());
        }

        let db = get_addr_from_ctx(ctx);

        let register_user = actions::RegisterUser {
            first_name: new_user.first_name,
            last_name: new_user.last_name,
            email: new_user.email,
            username: new_user.username,
            password: new_user.password,
        };

        let response = db.send(register_user).await.expect("error on send message to database executor");

        match response {
            Ok(user) => {
                Ok(User::from(user))
            },
            Err(e) => {
                Err(e)
            }          
        }
    }
}
