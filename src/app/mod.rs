// Uses and Crates
use actix::Addr;
use async_graphql::*;
use crate::database::DbExecutor;
use actix_web::{web, HttpResponse};
use async_graphql_actix_web::{Request, Response};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};


mod graphql;
use graphql::*;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

// this function could be located in different module
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/")
        .route(web::get().to(index_playground))
        .route(web::post().to(index))
    );
}

// Processes GraphQL queries and mutations
async fn index(schema: web::Data<AppSchema>, req: Request) -> Response {
    let query = req.into_inner();
    schema.execute(query).await.into()
}

async fn index_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

pub fn create_schema_with_context(
    db: Addr<DbExecutor>,
) -> Schema<Query, Mutation, EmptySubscription> {
    Schema::build(Query, Mutation, EmptySubscription)
        .enable_federation()
        .data(db.clone())
        .finish()
}

