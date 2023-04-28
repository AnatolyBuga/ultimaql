pub mod routers;

use crate::marketdata::models::{Spot, Curve, MarketData, CompoundingFrequency};
use utoipa::OpenApi;
use yearfrac::DayCountConvention;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Ultima QL"
    ),
    paths(
        routers::health_check,
        routers::upload
    ),
    components(
        schemas(Spot, Curve, MarketData, CompoundingFrequency, DayCountConvention)
    ),
    tags(
        (name = "Ultima QL", description = "Ultimate Business Intellegence endpoints.")
    ),
)]
pub struct ApiDoc;