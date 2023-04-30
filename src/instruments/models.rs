use chrono::NaiveDate;
use mongodb::Collection;
use serde::{Serialize, Deserialize};

use crate::marketdata::models::MarketData;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(tag = "type")]
pub enum Instrument{
    EurpoeanOption(EurpoeanOption),
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct EurpoeanOption{
    /// MarketData of type Spot must exist with same name
    underlying: String,
    /// MarketData of type Curve must exist with same name
    yield_curve: String,
    /// Currently only supports implied vol
    /// MarketData of type Curve must exist with same name
    implied_vol: String,
    strike: f64,
    /// date
    /// eg "2023-01-01"
    #[schema(format=Date, example="2021-12-01")]
    exp: NaiveDate
}

impl Instrument {
    pub fn pv(&self, md: &Collection<MarketData>){
        
    }
}