use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(untagged)]
pub enum Instrument{
    EurpoeanOption(EurpoeanOption),
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct EurpoeanOption{
    /// MarketData of type Spot must exist with same name
    underlying: String,
    /// MarketData of type Curve must exist with same name
    yield_curve: String,
    /// MarketData of type Curve must exist with same name
    implied_vol: String,
    exp: NaiveDate
}