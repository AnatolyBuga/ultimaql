pub mod routers;

use crate::marketdata::models::{Spot, Curve, MarketData};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Ultima QL"
    ),
    paths(
        routers::health_check
    ),
    components(
        schemas(Spot, Curve, MarketData)
    ),
    tags(
        (name = "Ultima QL", description = "Ultimate Business Intellegence endpoints.")
    ),
)]
pub(crate) struct ApiDoc;