pub mod models;

use mongodb::{ Collection,};
use self::models::MarketData;

pub struct MongoRepo {
    _col: Collection<MarketData>,
}