use rocket::{
    http::RawStr,
    request::FromParam,
    response::status::NotFound,
    State,
};
use rocket_contrib::json::Json;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    error::ApiError,
    resolver::Resolver,
    youtube::Youtube,
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
pub async fn search(
    query: String,
    resolver: State<'_, Resolver>,
) -> Result<Json<SearchResponse>, NotFound<String>> {
    resolver.youtube().query(&query).await.map_err(|_| NotFound("whoops"));
    Ok(Json(SearchResponse {}))
}
