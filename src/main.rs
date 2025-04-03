mod catalog_search;

use dotenv::dotenv;
use rocket::{get, http::Status, launch, routes, serde::json::Json, State};
use catalog_search::catalog_search_types::{CatalogSearchResponse,Language};

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_rocket::{GraphQLRequest, GraphQLResponse};

struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn search(&self, _ctx: &async_graphql::Context<'_>, term: String, language: Language) -> Result<Vec<CatalogSearchResponse>, String> {
        let language = handle_language(Some(language));
        match catalog_search::catalog_search_service::search_term(&term, language).await {
            Ok(catalog_search_result) => Ok(catalog_search_result),
            Err(e) => {
                eprintln!("Error fetching term: {:?}", e);
                Err("Internal Server Error".to_string())
            }
        }
    }
}

type CatalogsSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(
    schema: &State<CatalogsSchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    request.execute(schema.inner()).await
}

fn handle_language(lang: Option<Language>) -> Language {
    match lang {
        Some(lang) => lang,
        None => Language::PtBr,
    }
}

#[get("/")]
async fn index() -> &'static str {
    "hello, world!"
}

#[get("/?<term>&<language>")]
async fn search_route(term: Option<String>, language: Option<Language>) -> Result<Json<Vec<CatalogSearchResponse>>, Status> {

    let language = handle_language(language);

    match term {
        Some(term) => match catalog_search::catalog_search_service::search_term(&term, language).await {
            Ok(catalog_search_result) => Ok(Json(catalog_search_result)),
            Err(e) => {
                eprintln!("Error fetching term: {:?}", e);
                Err(Status::InternalServerError)
            }
        },
        None => Err(Status::BadRequest),
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    // rocket::build()
    //     .mount("/", routes![index])
    //     .mount("/search", routes![search_route])

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .finish();

    rocket::build()
        .manage(schema)
        .mount("/", routes![index, graphql_request])
        .mount("/search", routes![search_route])
        // .mount("/", routes![index, search_route, graphql_query, graphql_request])
}