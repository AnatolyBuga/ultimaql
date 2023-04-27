use std::{env, net::SocketAddr};

use actix_web::{HttpServer, App, web::{Data, self}, middleware::Logger};
use clap::Parser;
use mongodb::{Client, Collection, IndexModel, bson::{Document, Bson}, options::IndexOptions};
use ultima_quantlib::{marketdata::models::MarketData, api::routers::health_check};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // Read .env
    dotenv::dotenv().ok();
    // Allow pretty logs
    pretty_env_logger::init();
    
    let mongo_uri = env::var("MONGO_URI")
        .ok()
        .or_else(||Some("mongodb://quantlib_db:27017/".to_string()))
        .unwrap();

    let mongo_db = env::var("MONGO_DB").ok()
        .or_else(||Some("marketdata".to_string()))
        .unwrap();

    let client = Client::with_uri_str(mongo_uri).await.unwrap();
    for db_name in client.list_database_names(None, None).await? {
        dbg!("{}", db_name);
    }
    let db = client.database(&mongo_db);
    let md: Collection<MarketData> = db.collection("marketdata");
    let mut  index = IndexModel::builder()
        .keys(Document::from_iter([("name".to_string(), Bson::Int32(1)),
        ("as_of".to_string(), Bson::Int32(1))
        ]))
        .build();
    let options = IndexOptions::builder()
        .unique(true)
        .build();
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

    HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
        .service(
            web::scope("/api")
            .service(health_check)
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

