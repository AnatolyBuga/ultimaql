use actix_web::{HttpRequest, web::Data, Responder, HttpResponse, get, Result};
use mongodb::Collection;
use crate::marketdata::models::MarketData;
//use futures::stream::{StreamExt};

#[utoipa::path]
#[get("/health_check")]
pub async fn health_check(_: HttpRequest, _: Data<Collection<MarketData>>) -> Result<impl Responder> {
    //let mut cursor = data.find(None, None).await.unwrap();
    // regular Stream uses next() and iterates over Option<Result<T>>
    //while let Some(doc) = cursor.next().await {
    //  println!("{:?}", doc.map_err(actix_web::error::ErrorExpectationFailed)?)
    //} 
    Ok(HttpResponse::Ok())
}