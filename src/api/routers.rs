use crate::{instruments::models::Instrument, marketdata::models::MarketData};
use actix_web::{
    delete, get,
    http::header::ContentType,
    post,
    web::{self, Data},
    HttpRequest, HttpResponse, Responder, Result,
};

use chrono::NaiveDate;
use futures::TryStreamExt;
use mongodb::{
    bson::{Bson, Document},
    Collection,
};
use serde::{Deserialize, Serialize};
//use futures::stream::{StreamExt};

#[utoipa::path]
#[get("/health_check")]
pub async fn health_check(
    _: HttpRequest,
    _: Data<Collection<MarketData>>,
) -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}

#[utoipa::path(
    post,
    request_body(content = Vec<MarketData>, description = "Load Market Data", content_type = "application/json",
        example = json!(r#"
        [
    {"name":"Brent","as_of":"2021-12-01","value":95.6},
{"name":"OIS","as_of":"2021-12-01","day_count_conv":"Act365","compounding":"Continuous","values":[["2021-12-02",0.001],["2021-12-15",0.02],["2021-12-31",0.03]]},
{"name":"ImplVol_Brent_100","as_of":"2021-12-01","day_count_conv":"Act365","compounding":"Continuous","values":[["2021-12-02",0.05],["2021-12-15",0.1],["2021-12-31",0.2]]}

]
    "#)
    ),
    responses(
        (status = 200, description = "data loaded successfully",)
))]
#[post("/marketdata")]
pub async fn upload(
    req: web::Json<Vec<MarketData>>,
    md: Data<Collection<MarketData>>,
) -> Result<HttpResponse> {
    let new = req.into_inner();
    let res = md
        .insert_many(new, None)
        .await
        .map_err(actix_web::error::ErrorExpectationFailed)?;
    let body = serde_json::to_string(&res).unwrap();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body))
}

#[utoipa::path(
    get,
    params(("name",Query,example="Brent"),("as_of",Query,example="2021-12-01")),
    responses(
        (status = 200, description = "data retrieved successfully")
    )
)]
#[get("/marketdata")]
pub async fn get_md(
    path: web::Query<Search>,
    md: Data<Collection<MarketData>>,
) -> Result<HttpResponse> {
    //let (name, dt) = path.into_inner();
    let Search { name, as_of } = path.into_inner();

    let mut search = vec![];

    if let Some(name) = name {
        search.push(("name".to_string(), Bson::String(name)))
    };

    if let Some(as_of) = as_of {
        search.push(("as_of".to_string(), Bson::String(as_of)))
    };
    
    let doc = Document::from_iter(search);
    let cursor = md
        .find(doc, None)
        .await
        .map_err(actix_web::error::ErrorExpectationFailed)?;
    let v: Vec<MarketData> = cursor
        .try_collect()
        .await
        .map_err(actix_web::error::ErrorExpectationFailed)?;
    let body = serde_json::to_string(&v).unwrap();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body))
}

#[utoipa::path(
    delete,
    params(("name",Query,example="USDJPY"),("as_of",Query,example="2021-12-01")),
    responses(
        (status = 200, description = "data deleted successfully")
    )
)]
#[delete("/marketdata")]
pub async fn delete_md(
    path: web::Query<SearchDelete>,
    md: Data<Collection<MarketData>>,
) -> Result<HttpResponse> {
    //let (name, dt) = path.into_inner();
    let SearchDelete { name, as_of } = path.into_inner();

    let doc = Document::from_iter([
        ("name".to_string(), Bson::String(name)),
        ("as_of".to_string(), Bson::String(as_of)),
    ]);
    let res = md
        .delete_many(doc, None)
        .await
        .map_err(actix_web::error::ErrorExpectationFailed)?;
    let body = serde_json::to_string(&res).unwrap();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body))
}

#[utoipa::path(
    post,
    request_body(content = PriceRequest, description = "Compute Present Value", content_type = "application/json",
        example = json!(r#"
        {
            "type": "PriceRequest",
            "instrument": {"type":"EurpoeanOption","underlying":"Brent","yield_curve":"OIS","implied_vol":"ImplVol_Brent_100","strike":100.0,"direction":"Call","exp":"2023-11-30"},
            "date": "2021-12-01"
        }
    "#)
    ),
    responses(
        (status = 200, description = "Priced successfully",)
))]
#[post("/price")]
pub async fn price(
    req: web::Json<PriceRequest>,
    md: Data<Collection<MarketData>>,
) -> Result<HttpResponse> {
    let pr = req.into_inner();
    let PriceRequest { instrument, date } = pr;

    let res = instrument
        .pv(date, &md)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let body = serde_json::to_string(&res).unwrap();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body))
}

#[derive(Deserialize)]
pub struct Search {
    name: Option<String>,
    as_of: Option<String>,
}

#[derive(Deserialize)]
pub struct SearchDelete {
    name: String,
    as_of: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(tag = "type")]
pub struct PriceRequest {
    /// Instrument to be priced
    instrument: Instrument,
    /// Pricing As Of date
    /// Market Data for this date must exis.
    /// If not provided defaults to Naive local date
    /// eg "2023-01-01"
    #[schema(format=Date, example="2021-12-01")]
    #[serde(default = "today")]
    date: NaiveDate,
}

fn today() -> NaiveDate {
    //serde_json::to_string(&chrono::Utc::now().date_naive()).unwrap()
    chrono::Utc::now().date_naive()
}
