use rocket::{
    http::RawStr,
    request::FromParam,
};
use rocket_contrib::json::Json;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Debug)]
pub struct SearchRequest<'a> {
    query: &'a str,
}
impl<'a> FromParam<'a> for SearchRequest<'a> {
    type Error = &'a RawStr;

    fn from_param(param: &'a rocket::http::RawStr) -> Result<Self, Self::Error> {
        Ok(SearchRequest {
            query: param.as_str(),
        })
    }
}

#[derive(Serialize, Debug)]
pub struct SearchResponse {}

#[get("/search/<query>", format = "json")]
pub fn search(query: SearchRequest) -> Json<SearchResponse> {
    Json(SearchResponse {})
}
