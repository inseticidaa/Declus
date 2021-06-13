use async_graphql::*;

use crate::database::actions::GetUser;

use super::get_addr_from_ctx;
use super::User;

pub struct Query;

#[Object]
impl Query {
    async fn get_user(
        &self,
        ctx: &Context<'_>,
        username: Option<String>,
        email: Option<String>,
    ) -> Result<User, String> {
        let db = get_addr_from_ctx(ctx);

        let data = db.send(GetUser { email, username }).await.unwrap();

        match data {
            Ok(user) => Ok(User::from(user)),
            Err(e) => Err(e),
        }
    }
}
