use actix_web::{HttpRequest, web::{Data, self}, Responder, HttpResponse, get, Result, post, http::header::ContentType, delete};
use anyhow::Context;
use futures::TryStreamExt;
use mongodb::{Collection, bson::{Document, Bson}};
use serde::Deserialize;
use crate::{marketdata::models::MarketData, instruments::models::Instrument};
//use futures::stream::{StreamExt};

#[utoipa::path]
#[get("/health_check")]
pub async fn health_check(_: HttpRequest, _: Data<Collection<MarketData>>) -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}

#[utoipa::path(
    post,
    request_body(content = Vec<MarketData>, description = "Load Market Data", content_type = "application/json",
        example = json!(r#"
        [
    {"name":"USDRUB","as_of":"2021-12-01","value":95.6},
{"name":"USDOIS","as_of":"2021-12-01","day_count_conv":"Act365","compounding":null,"values":[["2021-12-02",0.001],["2021-12-15",0.02],["2021-12-31",0.03]]}
]
    "#)
    ),
    responses(
        (status = 200, description = "data loaded successfully",)
))]
#[post("/marketdata")]
pub async fn upload(req: web::Json<Vec<MarketData>>, md: Data<Collection<MarketData>>) 
-> Result<HttpResponse> {
    let new = req.into_inner();
    let res = md.insert_many(new, None)
        .await
        .map_err(actix_web::error::ErrorExpectationFailed)?;
    let body = serde_json::to_string(&res).unwrap();
    Ok(
    HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    )
}

#[utoipa::path(
    get,
    params(("name",Query,example="USDJPY"),("as_of",Query,example="2021-12-01")),
    responses(
        (status = 200, description = "data retrieved successfully")
    )
)]
#[get("/marketdata")]
pub async fn get_md(path: web::Query<Search>, md: Data<Collection<MarketData>>) 
-> Result<HttpResponse> {
    //let (name, dt) = path.into_inner();
    let Search {name, as_of} = path.into_inner();
    dbg!(&as_of);
    let doc = Document::from_iter([
        ("name".to_string(), Bson::String(name)),
        ("as_of".to_string(), Bson::String(as_of))
    ]);
    let cursor = md.find(doc, None)
        .await
        .map_err(actix_web::error::ErrorExpectationFailed)?;
    let v: Vec<MarketData> = cursor.try_collect().await
        .map_err(actix_web::error::ErrorExpectationFailed)?;
    let body = serde_json::to_string(&v).unwrap();
    Ok(
    HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    )
}

#[utoipa::path(
    delete,
    params(("name",Query,example="USDJPY"),("as_of",Query,example="2021-12-01")),
    responses(
        (status = 200, description = "data deleted successfully")
    )
)]
#[delete("/marketdata")]
pub async fn delete_md(path: web::Query<Search>, md: Data<Collection<MarketData>>) 
-> Result<HttpResponse> {
    //let (name, dt) = path.into_inner();
    let Search {name, as_of} = path.into_inner();
    let doc = Document::from_iter([
        ("name".to_string(), Bson::String(name)),
        ("as_of".to_string(), Bson::String(as_of))
    ]);
    let res = md.delete_many(doc, None)
        .await
        .map_err(actix_web::error::ErrorExpectationFailed)?;
    let body = serde_json::to_string(&res).unwrap();
    Ok(
    HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    )
}

#[post("/price")]
pub async fn price(prod: web::Json<Instrument>, md: Data<Collection<MarketData>>) 
-> Result<HttpResponse> {
    
    let res = tokio::task::spawn_blocking(move || {
            //let md = md.as_ref();
            prod.pv(&md)
        })
        .await
        .context("Failed to spawn blocking task.")
        .map_err(actix_web::error::ErrorInternalServerError)?;
    
    let res = 100;
    let body = serde_json::to_string(&res).unwrap();
    Ok(
    HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    )
}

#[derive(Deserialize)]
pub struct Search {
    name: String,
    as_of: String,
}