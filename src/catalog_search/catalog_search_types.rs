use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SearchError {
    #[error("Error configuring transport: {0}")]
    TransportError(String),
    #[error("Error sending search query: {0}")]
    SearchError(String),
    #[error("Error processing search response: {0}")]
    ResponseError(String),
    #[error("Error deserializing CatalogSearch: {0}")]
    DeserializationError(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CatalogSearch {
    pub id: i32,
    #[serde(rename = "catalogType", default)]
    pub catalog_type: String,
    pub name: String,
    #[serde(rename = "originalTitle")]
    pub original_name: Option<String>,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    #[serde(rename = "imagePath")]
    pub image_path: Option<String>,
    pub popularity: f64,
    #[serde(default)] // Adiciona um valor padrão para locations
    pub locations: Option<Vec<Location>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub language: String,
    pub title: String,
    #[serde(rename = "posterPath")]
    pub poster_path: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CatalogSearchResponse {
    pub id: i32,
    pub language: String, // Substitua por um tipo apropriado se você tiver um tipo `Language` definido
    pub name: String,
    #[serde(rename = "catalogType")]
    pub catalog_type: String,
    pub popularity: f64,
    #[serde(rename = "imagePath")]
    pub image_path: String,
    #[serde(rename = "releaseDate")]
pub release_date: String,
}

#[derive(Debug)]
pub enum Language {
    En,
    PtBr,
}

impl Language {
    pub fn as_str(&self) -> &str {
        match self {
            Language::En => "en",
            Language::PtBr => "pt-BR",
        }
    }
}