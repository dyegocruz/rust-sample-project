use std::env;
use opensearch::{http::transport::Transport, OpenSearch, SearchParts};
use serde_json::{json, Value};

use super::catalog_search_types::{SearchError, CatalogSearch};


pub async fn search_by_term(term: String) -> Result<Vec<CatalogSearch>, SearchError> {
  let mut catalog_search: Vec<CatalogSearch> = Vec::new();

  // Configurar o transporte com autenticação básica
  let opensearch_host = env::var("OPENSEARCH_HOST").map_err(|e| SearchError::TransportError(e.to_string()))?;
  let transport = Transport::single_node(&opensearch_host)
    .map_err(|e| SearchError::TransportError(e.to_string()))?;
  // .auth(Credentials::Basic("username".into(), "password".into()));
  let client = OpenSearch::new(transport);
  
  let query = json!({
      "query": {
          "bool": {
              "must": [
                  {
                      "simple_query_string": {
                          "query": term,
                          "fields": [
                              "name^5",
                              "locations.title^4",
                              "originalName^5",
                              "originalTitle^4",
                              "search_field^2",
                              "*"
                          ],
                          "default_operator": "AND",
                          "analyze_wildcard": true
                      }
                  }
              ]
          }
      },
      "sort": [
          {
              "popularity": {
                  "order": "desc"
              }
          }
      ]
  });

  // Fazer uma consulta de busca
  let search_response = client.search(SearchParts::Index(&["catalog_search"]))
      .body(query)
      .send()
      .await
      .map_err(|e| SearchError::SearchError(e.to_string()))?;
      

  let search_response_body: Value = search_response.json().await.map_err(|e| {
      SearchError::ResponseError(e.to_string())
  })?;

  if let Some(hits) = search_response_body["hits"]["hits"].as_array() {
    for hit in hits {
        if let Some(source) = hit["_source"].as_object() {
            let catalog_search_item: CatalogSearch = serde_json::from_value(Value::Object(source.clone()))
            .map_err(|e| {
              SearchError::DeserializationError(e.to_string())
            })?;

            catalog_search.push(catalog_search_item);
        }
    }
  }

	// Placeholder implementation
	Ok(catalog_search)
}