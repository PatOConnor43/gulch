use rocket_contrib::json::Json;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Debug)]
pub struct SearchRequest {
    query: String,
}

#[derive(Serialize, Debug)]
pub struct SearchResponse {}

#[get("/search", format = "json")]
pub fn search() -> Json<SearchResponse> {
    Json(SearchResponse {})
}
