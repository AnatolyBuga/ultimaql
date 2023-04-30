pub mod routers;

use crate::{
    instruments::models::{EurpoeanOption, Instrument, OptionDirection},
    marketdata::models::{CompoundingFrequency, Curve, MarketData, Spot},
};
use utoipa::OpenApi;
use yearfrac::DayCountConvention;

use self::routers::PriceRequest;

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
            Instrument,EurpoeanOption, OptionDirection, PriceRequest)
    ),
    tags(
        (name = "Ultima QL", description = "Ultimate Business Intellegence endpoints.")
    ),
)]
pub struct ApiDoc;
