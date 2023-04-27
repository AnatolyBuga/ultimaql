//! TODO move to a separate crate

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, utoipa::ToSchema)]
#[serde(untagged)]
pub enum MarketData{
    Spot(Spot),
    YieldCurve(YieldCurve),
    ImplVolCurve(ImplVolCurve)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, utoipa::ToSchema)]
pub struct Spot {
    name: String,
    as_of: DateTime<Utc>,
    value: f64
}

#[derive(Debug, Clone, Serialize, Deserialize,Hash, PartialEq, Eq, utoipa::ToSchema)]
pub struct YieldCurve {}

#[derive(Debug, Clone, Serialize, Deserialize,Hash, PartialEq, Eq, utoipa::ToSchema)]
pub struct ImplVolCurve {}