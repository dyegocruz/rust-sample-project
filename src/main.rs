mod catalog_search;

use dotenv::dotenv;
use rocket::{get, http::Status, launch, routes, serde::json::Json};
use catalog_search::catalog_search_types::{CatalogSearchResponse,Language};

fn handle_language(lang: Option<String>) -> Language {
    match lang {
        Some(lang) => match lang.as_str() {
            "en" => Language::En,
            "pt-br" => Language::PtBr,
            _ => Language::PtBr,
        },
        None => Language::PtBr,
    }
}

#[get("/")]
async fn index() -> &'static str {
    "hello, world!"
}

#[get("/?<term>&<lang>")]
async fn search_route(term: Option<String>, lang: Option<String>) -> Result<Json<Vec<CatalogSearchResponse>>, Status> {

    let language = handle_language(lang);

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
    rocket::build()
        .mount("/", routes![index])
        .mount("/search", routes![search_route])
}