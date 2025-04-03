use regex::Regex;
use super::catalog_search_repository::search_by_term;
use super::catalog_search_types::{SearchError, CatalogSearchResponse, Language};

pub async fn search_term(term: &str, language: Language) -> Result<Vec<CatalogSearchResponse>, SearchError> {
    let mut catalog_search_response_list: Vec<CatalogSearchResponse> = Vec::new();
    let processed_term = process_term(term); // Substitua pelo valor real

    let catalog_search_result = search_by_term(processed_term).await?;

    for catalog in catalog_search_result {
        let mut catalog_search_response = CatalogSearchResponse {
            id: catalog.id,
            name: catalog.name,
            catalog_type: catalog.catalog_type,
            release_date: catalog.release_date.unwrap_or_default(),
            popularity: catalog.popularity,
            image_path: catalog.image_path.unwrap_or_default(),
            language: language.as_str().to_string(),
        };

        for location in catalog.locations.iter().flatten() {
            if location.language == language.as_str() {
                catalog_search_response.name = location.title.clone();
                catalog_search_response.image_path = location.poster_path.clone().unwrap_or_default();
            }
        }

        catalog_search_response_list.push(catalog_search_response);
    }

    Ok(catalog_search_response_list)
}

fn process_term(term: &str) -> String {
    let mut words: Vec<String> = term.replace("-", " ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let re = Regex::new(r"(\w+)([\)|\]])?$").unwrap();

    for word in &mut words {
        if word.len() > 1 {
            let string_replace = if contains_fuzzy_chars(word) {
                "$1~$2"
            } else {
                "$1*$2"
            };

            *word = format!("\\\"{}", re.replace(word, string_replace).to_string());
        } else {
            if is_vowel(word.chars().next().unwrap()) {
                *word = "".to_string();
            }
        }
    }

  format!("({})", words.join(" ").trim())
}

fn contains_fuzzy_chars(s: &str) -> bool {
    let fuzzy_chars = Regex::new(r"[a-zA-Z0-9].{1}[eèéáàòìcçEÈÉAÁÀÒÌCÇ]").unwrap();
    fuzzy_chars.is_match(s)
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}