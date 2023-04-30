//! TODO move to a separate crate

use chrono::{NaiveDate};
use serde::{Deserialize, Serialize};
use yearfrac::DayCountConvention;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(untagged)]
pub enum MarketData{
    Spot(Spot),
    Curve(Curve),
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Spot {
    name: String,
    /// date
    /// eg "2023-01-01"
    #[schema(format=Date)]
    as_of: NaiveDate,
    value: f64
}

/// General Curve 
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Curve {
    name: String,
    /// date
    /// eg "2023-01-01"
    #[schema(format=Date, example="2021-12-01")]
    as_of: NaiveDate,
    day_count_conv: DayCountConvention,
    compounding: CompoundingFrequency,
    /// (Tenor, Value)
    /// eg [("2023-01-01", 0.999), ("2023-01-22", 0.81)]
    values: Vec<(NaiveDate, f64)>
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub enum CompoundingFrequency{
    Continuous
}
