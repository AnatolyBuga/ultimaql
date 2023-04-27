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
#[serde(untagged)]
pub enum CompoundingFrequency{
    Continuous
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DCC{
    DCC(yearfrac::DayCountConvention)
}

impl<'__s> utoipa::ToSchema<'__s> for DCC {
    fn schema() -> (&'__s str, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>) {
         ("DayCountConvention",
         utoipa::openapi::ObjectBuilder::new()
         //.to_array_builder()
         //.items([
         //   "US30360",
         //])
         //.enum_values(Some(["US30360"]))
         .into()
        )
    }
}
