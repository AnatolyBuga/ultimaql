pub mod routers;

use crate::marketdata::models::{Spot, Curve, MarketData, CompoundingFrequency, DCC};
use utoipa::OpenApi;

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
        schemas(Spot, Curve, MarketData, CompoundingFrequency, DCC)
    ),
    tags(
        (name = "Ultima QL", description = "Ultimate Business Intellegence endpoints.")
    ),
)]
pub struct ApiDoc;