pub mod routers;

use crate::{marketdata::models::{Spot, Curve, MarketData, CompoundingFrequency}, instruments::models::{Instrument, EurpoeanOption}};
use utoipa::OpenApi;
use yearfrac::DayCountConvention;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Ultima QL"
    ),
    paths(
        routers::health_check,
        routers::upload,
        routers::get_md,
        routers::delete_md,
        routers::price,
    ),
    components(
        schemas(Spot, Curve, MarketData, CompoundingFrequency, DayCountConvention,
            Instrument,EurpoeanOption)
    ),
    tags(
        (name = "Ultima QL", description = "Ultimate Business Intellegence endpoints.")
    ),
)]
pub struct ApiDoc;