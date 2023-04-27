//! TODO move to a separate crate

use chrono::{NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(untagged)]
pub enum MarketData{
    Spot(Spot),
    YieldCurve(Curve),
    ImplVolCurve(Curve)
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Spot {
    name: String,
    as_of: NaiveDate,
    value: f64
}

/// General Curve 
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Curve {
    name: String,
    as_of: NaiveDate,
    day_count_conv: yearfrac::DayCountConvention,
    compounding: CompoundingFrequency,
    /// (Tenor, Value)
    values: Vec<(NaiveDate, f64)>
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub enum CompoundingFrequency{
    Continuous
}
