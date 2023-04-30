use std::{env, net::SocketAddr};

use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use clap::Parser;
use mongodb::{
    bson::{Bson, Document},
    options::IndexOptions,
    Client, Collection, IndexModel,
};
use std::net::TcpListener;
use ultima_quantlib::{
    api::routers::{delete_md, get_md, health_check, price, upload},
    api::ApiDoc,
    marketdata::models::MarketData,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read .env
    dotenv::dotenv().ok();
    // Allow pretty logs
    pretty_env_logger::init();

    let mongo_uri = env::var("MONGO_URI")
        .ok()
        .or_else(|| Some("mongodb://quantlib_db:27017/".to_string()))
        .unwrap();

    let mongo_db = env::var("MONGO_DB")
        .ok()
        .or_else(|| Some("marketdata".to_string()))
        .unwrap();

    let client = Client::with_uri_str(mongo_uri).await.unwrap();

    let db = client.database(&mongo_db);
    let md: Collection<MarketData> = db.collection("marketdata");
    let mut index = IndexModel::builder()
        .keys(Document::from_iter([
            ("name".to_string(), Bson::Int32(1)),
            ("as_of".to_string(), Bson::Int32(1)),
        ]))
        .build();
    let options = IndexOptions::builder().unique(true).build();
    index.options = Some(options);

    let _ = md.create_index(index, None).await;

    let data_md = Data::new(md);

    let cli = CliServer::parse();
    let addr: SocketAddr = cli
        .address // command line arg first
        .or_else(|| env::var("ADDRESS").ok()) // OR use .env
        .and_then(|addr| addr.parse().ok())
        .or_else(|| Some(([0, 0, 0, 0], 8000).into())) // Finaly, this default
        .expect("can't parse ADDRES variable");

    let listener = TcpListener::bind(addr).expect("Failed to bind the port");
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(health_check)
                    .service(upload)
                    .service(get_md)
                    .service(delete_md)
                    .service(price),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi.clone()),
            )
            .app_data(data_md.clone())
    })
    .listen(listener)?
    .run()
    .await?;

    Ok(())
}

/// Cli for the server run
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliServer {
    #[arg(short, long, value_name = "SOCKET_ADDRESS")]
    pub address: Option<String>,
}
