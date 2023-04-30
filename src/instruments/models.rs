use std::ops::Div;

use anyhow::Context;
use chrono::NaiveDate;
use mongodb::{
    bson::{Bson, Document},
    Collection,
};
use serde::{Deserialize, Serialize};

use crate::marketdata::models::{Curve, MarketData, Spot};
use statrs::distribution::{ContinuousCDF, Normal};

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(tag = "type")]
pub enum Instrument {
    EurpoeanOption(EurpoeanOption),
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct EurpoeanOption {
    /// MarketData of type Spot must exist with same name
    pub underlying: String,
    /// MarketData of type Curve must exist with same name
    pub yield_curve: String,
    /// Currently only supports implied vol
    /// MarketData of type Curve must exist with same name
    pub implied_vol: String,
    pub strike: f64,
    /// Call or Put
    pub direction: OptionDirection,
    /// date
    /// eg "2023-01-01"
    #[schema(format=Date, example="2021-12-01")]
    pub exp: NaiveDate,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, utoipa::ToSchema)]
pub enum OptionDirection {
    Call,
    Put,
}

impl Instrument {
    pub async fn pv(&self, dt: &str, md: &Collection<MarketData>) -> Result<f64, anyhow::Error> {
        match self {
            Instrument::EurpoeanOption(eo) => {
                let underlying = md
                    .find_one(
                        Document::from_iter([
                            ("name".to_string(), Bson::String(eo.underlying.to_string())),
                            ("as_of".to_string(), Bson::String(dt.to_string())),
                        ]),
                        None,
                    )
                    .await
                    .unwrap()
                    .ok_or(QuantLibError::MissingMarketData(format!(
                        "Missing Market Data name: {}, date: {}",
                        eo.underlying, dt
                    )))?;
                let yc = md
                    .find_one(
                        Document::from_iter([
                            ("name".to_string(), Bson::String(eo.yield_curve.to_string())),
                            ("as_of".to_string(), Bson::String(dt.to_string())),
                        ]),
                        None,
                    )
                    .await
                    .unwrap()
                    .ok_or(QuantLibError::MissingMarketData(format!(
                        "Missing Market Data name: {}, date: {}",
                        eo.underlying, dt
                    )))?;
                let iv = md
                    .find_one(
                        Document::from_iter([
                            ("name".to_string(), Bson::String(eo.implied_vol.to_string())),
                            ("as_of".to_string(), Bson::String(dt.to_string())),
                        ]),
                        None,
                    )
                    .await
                    .unwrap()
                    .ok_or(QuantLibError::MissingMarketData(format!(
                        "Missing Market Data name: {}, date: {}",
                        eo.underlying, dt
                    )))?;

                let MarketData::Spot(underlying) = underlying else{
                        return Err(QuantLibError::WrongMarketDataFormat(format!(
                            "Wrong Market Data name: {}, date: {}; is not a Curve",
                            eo.underlying, dt
                        )).into())
                    };

                let MarketData::Curve(yc) = yc else{
                        return Err(QuantLibError::WrongMarketDataFormat(format!(
                            "Wrong Market Data name: {}, date: {}; is not a Curve",
                            eo.underlying, dt
                        )).into())
                    };

                let MarketData::Curve(iv) = iv else{
                    return Err(QuantLibError::WrongMarketDataFormat(format!(
                        "Wrong Market Data name: {}, date: {}; is not a Curve",
                        eo.underlying, dt
                    )).into())
                };

                let prod = eo.clone();
                let dt: NaiveDate = serde_json::from_str(dt)?;

                tokio::task::spawn_blocking(move || prod.pv(dt, underlying, yc, iv))
                    .await
                    .context("Failed to spawn blocking task.")
            }
        }
    }
}

impl EurpoeanOption {
    pub fn pv(&self, dt: NaiveDate, s: Spot, yc: Curve, iv: Curve) -> f64 {
        let dc = yc.day_count_conv;
        let exp = self.exp;
        let k = self.strike;
        let f = s.value;
        let r = interp_curve(exp, yc);
        let v = interp_curve(exp, iv);

        let sk = (f / k).ln();
        let ttm = dc.yearfrac(dt, exp);
        let ttm_sqrt = ttm.sqrt();
        let a = v.powi(2).div(2.0) * ttm;
        let b = v * ttm_sqrt;
        let d1 = sk + a + b;
        let d2 = d1 - v * ttm_sqrt;

        let dist = Normal::new(0.0, 1.0).unwrap();
        match self.direction {
            OptionDirection::Call => (-r * ttm).exp() * (f * dist.cdf(d1) - k * dist.cdf(d2)),
            OptionDirection::Put => (-r * ttm).exp() * (-f * dist.cdf(-d1) + k * dist.cdf(-d2)),
        }
    }
}

/// dt is same as as_of
pub fn interp_curve(exp: NaiveDate, curve: Curve) -> f64 {
    let dcc = curve.day_count_conv;
    let as_of = curve.as_of;
    let mut curve: Vec<(f64, f64)> = curve
        .values
        .into_iter()
        .map(|(x, y)| (dcc.yearfrac(as_of, x), y))
        .collect();
    curve.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let look_up = dcc.yearfrac(as_of, exp);
    let first = curve.first().unwrap(); //TODO validate curve value is non empty
    let last = curve.last().unwrap();
    if look_up < first.0 {
        first.1
    } else if look_up > last.0 {
        last.1
    } else {
        let mut i = 0;
        for (j, ab) in curve.iter().enumerate() {
            if look_up < ab.0 {
                continue;
            } else {
                i = j;
                break;
            }
        }
        let (x0, y0) = curve[i];
        let (x1, y1) = curve[i + 1];

        y0 + (look_up - x0) * (y1 - y0) / (x1 - x0)
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuantLibError {
    #[error("{0}")]
    MissingMarketData(String),
    #[error("{0}")]
    WrongMarketDataFormat(String),
}
