pub mod models;

use self::models::MarketData;
use mongodb::Collection;

pub struct MongoRepo {
    _col: Collection<MarketData>,
}
