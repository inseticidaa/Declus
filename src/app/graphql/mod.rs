use actix::Addr;
use async_graphql::*;
use crate::database::{DbExecutor, actions::UserResponse};
use validator::{Validate, ValidationError};
use async_graphql::Context;

mod mutation;
pub use mutation::*;
mod query;
pub use query::*;

pub fn get_addr_from_ctx(ctx: &Context<'_>) -> Addr<DbExecutor> {
    ctx.data::<Addr<DbExecutor>>()
        .expect("Can't get pool")
        .clone()
}

#[derive(SimpleObject)]
pub struct User {
    id: String,
    username: String,
    email: String,
    first_name: String,
    last_name:String,
    token: Option<String>
}

impl From<UserResponse> for User {
    fn from(user: UserResponse) -> Self {
        User {
            id: user.id.to_string(),
            email: user.email,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            token: user.token
        }
    }
}

#[derive(InputObject, Validate)]
pub struct NewUser {
    #[validate(length(min = 1), custom = "validate_unique_username")]
    username: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 8), custom = "validate_password")]
    password: String,
    #[validate(length(min = 1), custom = "validate_name")]
    first_name: String,
    #[validate(length(min = 1), custom = "validate_name")]
    last_name:String,
}

fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
    if username == "xXxShad0wxXx" {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("terrible_username"));
    }

    Ok(())
}

fn validate_password(_username: &str) -> Result<(), ValidationError> {
    Ok(())
}

fn validate_name(_username: &str) -> Result<(), ValidationError> {
    Ok(())
}