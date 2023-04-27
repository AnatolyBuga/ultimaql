use actix_web::{HttpRequest, web::{Data, self}, Responder, HttpResponse, get, Result, post, http::header::ContentType};
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

#[utoipa::path]
#[post("/upload")]
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